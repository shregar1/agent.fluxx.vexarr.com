//! Cloud forwarder.
//!
//! Periodically pushes a rolled-up snapshot of this agent's state to the
//! Fluxx cloud control plane (`POST /api/v1/telemetry/snapshot`). The
//! cloud uses these snapshots to render the per-customer global dashboard
//! at `dashboard.fluxx.vexarr.com` — every signed-in customer sees ONLY
//! the agents associated with their API key.
//!
//! What we send:
//!   - agent_id              stable per-host identifier
//!   - agent_version         current Cargo.toml version
//!   - hostname              `gethostname()`
//!   - total_events          ingested event count from EventStore
//!   - data.protocols        breakdown by protocol name
//!   - data.services         service-map node summary
//!   - data.recent_events    last few events for the dashboard sidebar
//!
//! What we deliberately DON'T send:
//!   - raw events / logs / spans — those stay on the agent and are
//!     queried over its local `/v1/*` surface; pushing every event to
//!     the cloud is a separate (much bigger) piece.

use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use crate::storage::event_store::EventStore;

/// How often we push to the cloud. Tuned for the dashboard's "stale > 90s"
/// indicator: at 30s we get 2-3 pushes per visible window.
const SNAPSHOT_INTERVAL: Duration = Duration::from_secs(30);

/// Maximum recent events to attach to each snapshot. Larger payloads get
/// rejected (the cloud caps at 256 KB).
const RECENT_EVENT_CAP: usize = 20;

#[derive(Debug, Serialize)]
struct SnapshotPayload<'a> {
    agent_id: String,
    agent_version: &'a str,
    hostname: String,
    total_events: u64,
    data: SnapshotData,
}

#[derive(Debug, Serialize, Default)]
struct SnapshotData {
    protocols: serde_json::Value,
    services: serde_json::Value,
    recent_events: serde_json::Value,
}

/// Spawn the forwarder loop on the current tokio runtime.
///
/// Silently no-ops if the agent isn't configured to forward — i.e.
/// `FLUXX_FORWARD_TO_CLOUD=false`, or `FLUXX_API_URL` is unset. The
/// network call is best-effort: a failed push logs a warning and the
/// loop tries again next tick.
pub fn spawn(event_store: Arc<dyn EventStore>) {
    let api_url = match std::env::var("FLUXX_API_URL") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            debug!("cloud forwarder: FLUXX_API_URL not set, skipping");
            return;
        }
    };
    let api_key = match std::env::var("FLUXX_API_KEY") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            debug!("cloud forwarder: FLUXX_API_KEY not set, skipping");
            return;
        }
    };
    if std::env::var("FLUXX_FORWARD_TO_CLOUD")
        .map(|v| v.to_lowercase() == "false" || v == "0")
        .unwrap_or(false)
    {
        info!("cloud forwarder: disabled by FLUXX_FORWARD_TO_CLOUD=false");
        return;
    }

    let agent_id = stable_agent_id();
    let hostname = gethostname_string();
    let agent_version = env!("CARGO_PKG_VERSION");
    let url = format!("{}/api/v1/telemetry/snapshot", api_url.trim_end_matches('/'));

    info!(
        agent_id = %agent_id,
        endpoint = %url,
        interval_secs = SNAPSHOT_INTERVAL.as_secs(),
        "cloud forwarder: spawning"
    );

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("cloud forwarder: HTTP client init failed: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        // Stagger the first push so we don't all flush in lockstep on
        // startup if many agents come up together.
        tokio::time::sleep(Duration::from_secs(5)).await;

        loop {
            match build_snapshot(&event_store, &agent_id, agent_version, &hostname).await {
                Ok(payload) => {
                    if let Err(e) = push(&client, &url, &api_key, &payload).await {
                        warn!(error = %e, "cloud forwarder: push failed");
                    }
                }
                Err(e) => {
                    warn!(error = %e, "cloud forwarder: snapshot build failed");
                }
            }
            tokio::time::sleep(SNAPSHOT_INTERVAL).await;
        }
    });
}

async fn build_snapshot(
    event_store: &Arc<dyn EventStore>,
    agent_id: &str,
    agent_version: &str,
    hostname: &str,
) -> Result<SnapshotPayload<'static>, String> {
    // Total in-store. The agent's own /v1/stats also derives from this.
    let total_events = event_store
        .len()
        .await
        .map_err(|e| format!("event_store.len failed: {}", e))? as u64;

    // Pull the most recent events; we trim to RECENT_EVENT_CAP and also
    // synthesise protocol + service breakdowns client-side so the cloud
    // doesn't need a separate stats endpoint.
    let recent = event_store
        .list_events(None, None, None, None)
        .await;

    let mut protocols: std::collections::HashMap<String, u64> =
        std::collections::HashMap::new();
    let mut services: std::collections::HashMap<String, (u64, u64)> =
        std::collections::HashMap::new(); // service_id -> (span_count, error_count)

    for ev in &recent {
        let proto = format!("{:?}", ev.protocol);
        *protocols.entry(proto).or_insert(0) += 1;
        if let Some(svc) = &ev.service_id {
            let entry = services.entry(svc.clone()).or_insert((0, 0));
            entry.0 += 1;
            if matches!(
                ev.severity,
                crate::models::event::EventSeverity::High
                    | crate::models::event::EventSeverity::Critical
            ) {
                entry.1 += 1;
            }
        }
    }

    let services_arr: Vec<serde_json::Value> = services
        .into_iter()
        .map(|(id, (spans, errors))| {
            serde_json::json!({
                "id": id,
                "name": id,
                "span_count": spans,
                "error_count": errors,
            })
        })
        .collect();

    let recent_trimmed: Vec<&_> = recent.iter().take(RECENT_EVENT_CAP).collect();
    let recent_events = serde_json::to_value(&recent_trimmed)
        .unwrap_or_else(|_| serde_json::json!([]));

    Ok(SnapshotPayload {
        agent_id: agent_id.to_string(),
        agent_version: Box::leak(agent_version.to_string().into_boxed_str()),
        hostname: hostname.to_string(),
        total_events,
        data: SnapshotData {
            protocols: serde_json::to_value(&protocols).unwrap_or_default(),
            services: serde_json::Value::Array(services_arr),
            recent_events,
        },
    })
}

async fn push(
    client: &reqwest::Client,
    url: &str,
    api_key: &str,
    payload: &SnapshotPayload<'_>,
) -> Result<(), String> {
    let resp = client
        .post(url)
        .header("X-API-Key", api_key)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".into());
        return Err(format!("HTTP {}: {}", status, body));
    }

    debug!(
        total_events = payload.total_events,
        "cloud forwarder: snapshot pushed"
    );
    Ok(())
}

/// Build a stable per-host identifier so successive starts of the same
/// agent collapse to one row in `agent_snapshots` instead of fanning out.
fn stable_agent_id() -> String {
    if let Ok(s) = std::env::var("FLUXX_AGENT_ID") {
        if !s.trim().is_empty() {
            return s;
        }
    }
    format!("fluxx-{}", gethostname_string())
}

fn gethostname_string() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| {
        // Fallback: read /etc/hostname directly. Works on most Linux hosts
        // even when HOSTNAME isn't exported in the systemd environment.
        std::fs::read_to_string("/etc/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown-host".to_string())
    })
}
