//! Real User Monitoring (RUM) API
//!
//! Ingests browser performance data from the Fluxx RUM JavaScript SDK.
//! Tracks: page loads, web vitals (LCP/FID/CLS/FCP/TTFB), JS errors,
//! network requests, user actions, sessions, and custom events.

use crate::api::{ApiState, StandardResponse};
use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::info;

/// A single RUM event from the browser SDK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp_us: u64,
    pub session_id: String,
    pub page_view_id: Option<String>,
    pub application_id: String,
    pub environment: Option<String>,
    pub app_version: Option<String>,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub url: Option<String>,
    pub user_agent: Option<String>,
    pub screen_width: Option<u32>,
    pub screen_height: Option<u32>,
    pub viewport_width: Option<u32>,
    pub viewport_height: Option<u32>,
    // Flattened extra fields from various event types
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

/// Aggregated RUM session summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RumSessionSummary {
    pub session_id: String,
    pub application_id: String,
    pub user_id: Option<String>,
    pub first_url: String,
    pub page_view_count: u32,
    pub error_count: u32,
    pub action_count: u32,
    pub started_at_us: u64,
    pub last_activity_us: u64,
    pub duration_ms: u64,
    pub user_agent: String,
    pub country: Option<String>,
}

/// Web Vitals summary for an application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebVitalsSummary {
    pub application_id: String,
    pub lcp_p75_ms: f64,
    pub fid_p75_ms: f64,
    pub cls_p75: f64,
    pub fcp_p75_ms: f64,
    pub ttfb_p75_ms: f64,
    pub sample_count: u64,
    pub good_percent: f64,
    pub needs_improvement_percent: f64,
    pub poor_percent: f64,
}

#[derive(Debug, Default)]
pub struct RumStore {
    events: RwLock<Vec<RumEvent>>,
    sessions: RwLock<HashMap<String, RumSessionSummary>>,
}

impl RumStore {
    pub fn new() -> Self { Self::default() }

    pub fn ingest(&self, events: Vec<RumEvent>) {
        let mut sessions = self.sessions.write();

        for event in &events {
            // Update session summary
            let session = sessions.entry(event.session_id.clone()).or_insert_with(|| {
                RumSessionSummary {
                    session_id: event.session_id.clone(),
                    application_id: event.application_id.clone(),
                    user_id: event.user_id.clone(),
                    first_url: event.url.clone().unwrap_or_default(),
                    page_view_count: 0,
                    error_count: 0,
                    action_count: 0,
                    started_at_us: event.timestamp_us,
                    last_activity_us: event.timestamp_us,
                    duration_ms: 0,
                    user_agent: event.user_agent.clone().unwrap_or_default(),
                    country: None,
                }
            });

            session.last_activity_us = session.last_activity_us.max(event.timestamp_us);
            session.duration_ms = (session.last_activity_us - session.started_at_us) / 1000;

            match event.event_type.as_str() {
                "view" => session.page_view_count += 1,
                "error" => session.error_count += 1,
                "action" => session.action_count += 1,
                _ => {}
            }
        }

        self.events.write().extend(events);
    }

    pub fn get_sessions(&self, app: Option<&str>, limit: usize) -> Vec<RumSessionSummary> {
        let sessions = self.sessions.read();
        let mut result: Vec<RumSessionSummary> = sessions.values()
            .filter(|s| app.map_or(true, |a| s.application_id == a))
            .cloned()
            .collect();
        result.sort_by(|a, b| b.last_activity_us.cmp(&a.last_activity_us));
        result.truncate(limit);
        result
    }

    pub fn get_vitals(&self, app_id: &str) -> WebVitalsSummary {
        let events = self.events.read();
        let vitals: Vec<&RumEvent> = events.iter()
            .filter(|e| e.event_type == "vital" && e.application_id == app_id)
            .collect();

        let mut lcp_vals = Vec::new();
        let mut fid_vals = Vec::new();
        let mut cls_vals = Vec::new();
        let mut fcp_vals = Vec::new();
        let mut ratings = HashMap::new();

        for v in &vitals {
            let name = v.data.get("vital_name").and_then(|v| v.as_str()).unwrap_or("");
            let value = v.data.get("vital_value").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let rating = v.data.get("vital_rating").and_then(|v| v.as_str()).unwrap_or("unknown");

            match name {
                "LCP" => lcp_vals.push(value),
                "FID" => fid_vals.push(value),
                "CLS" => cls_vals.push(value),
                "FCP" => fcp_vals.push(value),
                _ => {}
            }
            *ratings.entry(rating.to_string()).or_insert(0u64) += 1;
        }

        let total = ratings.values().sum::<u64>().max(1) as f64;

        WebVitalsSummary {
            application_id: app_id.to_string(),
            lcp_p75_ms: percentile_75(&mut lcp_vals),
            fid_p75_ms: percentile_75(&mut fid_vals),
            cls_p75: percentile_75(&mut cls_vals),
            fcp_p75_ms: percentile_75(&mut fcp_vals),
            ttfb_p75_ms: 0.0, // From page_load events
            sample_count: vitals.len() as u64,
            good_percent: *ratings.get("good").unwrap_or(&0) as f64 / total * 100.0,
            needs_improvement_percent: *ratings.get("needs-improvement").unwrap_or(&0) as f64 / total * 100.0,
            poor_percent: *ratings.get("poor").unwrap_or(&0) as f64 / total * 100.0,
        }
    }

    pub fn total_events(&self) -> usize { self.events.read().len() }
    pub fn total_sessions(&self) -> usize { self.sessions.read().len() }
}

fn percentile_75(vals: &mut Vec<f64>) -> f64 {
    if vals.is_empty() { return 0.0; }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = ((vals.len() as f64 * 0.75) as usize).min(vals.len() - 1);
    vals[idx]
}

/// POST /v1/rum — Ingest RUM events from browser SDK.
pub async fn ingest_rum(
    State(state): State<Arc<ApiState>>,
    Json(events): Json<Vec<RumEvent>>,
) -> impl IntoResponse {
    let count = events.len();
    state.rum_store.ingest(events);
    info!("RUM: Ingested {} browser events", count);
    StandardResponse::ok(serde_json::json!({"ingested": count}), "RUM events ingested.")
}

/// GET /v1/rum/sessions — List RUM sessions.
pub async fn list_rum_sessions(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let app = params.get("application_id").map(|s| s.as_str());
    let limit: usize = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(50);
    let sessions = state.rum_store.get_sessions(app, limit);
    StandardResponse::ok(serde_json::to_value(&sessions).unwrap_or_default(), "RUM sessions.")
}

/// GET /v1/rum/vitals — Get Web Vitals summary.
pub async fn get_web_vitals(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let app_id = params.get("application_id").map(|s| s.as_str()).unwrap_or("default");
    let vitals = state.rum_store.get_vitals(app_id);
    StandardResponse::ok(serde_json::to_value(&vitals).unwrap_or_default(), "Web Vitals.")
}

/// GET /v1/rum/stats — Get RUM overview stats.
pub async fn rum_stats(
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    StandardResponse::ok(serde_json::json!({
        "total_events": state.rum_store.total_events(),
        "total_sessions": state.rum_store.total_sessions(),
    }), "RUM stats.")
}
