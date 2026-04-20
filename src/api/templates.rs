//! Dashboard Templates API
//!
//! Activate/deactivate pre-built monitoring templates.
//! When activated, a template registers its metrics for collection
//! and returns the PromQL queries + chart config for the dashboard to render.

use crate::api::{ApiState, StandardResponse};
use axum::{
    extract::{State, Path},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// A template definition with its metric queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub metrics: Vec<String>,
    pub panels: Vec<TemplatePanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePanel {
    pub title: String,
    pub query: String,
    pub chart_type: String,
    pub unit: Option<String>,
    pub position: u32,
}

/// Active template state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTemplate {
    pub template: DashboardTemplate,
    pub activated_at_us: u64,
}

fn now_us() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_micros() as u64
}

#[derive(Debug, Default)]
pub struct TemplateStore {
    available: RwLock<Vec<DashboardTemplate>>,
    active: RwLock<HashMap<String, ActiveTemplate>>,
}

impl TemplateStore {
    pub fn new() -> Self {
        let store = Self::default();
        *store.available.write() = built_in_templates();
        store
    }

    pub fn list_available(&self) -> Vec<DashboardTemplate> {
        self.available.read().clone()
    }

    pub fn list_active(&self) -> Vec<ActiveTemplate> {
        self.active.read().values().cloned().collect()
    }

    pub fn activate(&self, template_id: &str) -> Option<ActiveTemplate> {
        let available = self.available.read();
        let template = available.iter().find(|t| t.id == template_id)?.clone();
        let active = ActiveTemplate { template: template.clone(), activated_at_us: now_us() };
        self.active.write().insert(template_id.to_string(), active.clone());
        Some(active)
    }

    pub fn deactivate(&self, template_id: &str) -> bool {
        self.active.write().remove(template_id).is_some()
    }

    pub fn is_active(&self, template_id: &str) -> bool {
        self.active.read().contains_key(template_id)
    }

    pub fn get_active(&self, template_id: &str) -> Option<ActiveTemplate> {
        self.active.read().get(template_id).cloned()
    }
}

/// GET /v1/templates — List all available templates with active status.
pub async fn list_templates(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let templates = state.template_store.list_available();
    let active_ids: Vec<String> = state.template_store.list_active().iter().map(|a| a.template.id.clone()).collect();

    let result: Vec<serde_json::Value> = templates.iter().map(|t| {
        serde_json::json!({
            "id": t.id,
            "name": t.name,
            "description": t.description,
            "category": t.category,
            "metrics": t.metrics,
            "panels": t.panels,
            "active": active_ids.contains(&t.id),
        })
    }).collect();

    StandardResponse::ok(serde_json::to_value(&result).unwrap_or_default(), "Templates.")
}

/// POST /v1/templates/:id/activate — Activate a template.
pub async fn activate_template(
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    match state.template_store.activate(&id) {
        Some(active) => {
            let data = serde_json::to_value(&active).unwrap_or_default();
            StandardResponse::ok(data, "Template activated. Metrics collection started.")
        }
        None => StandardResponse::error(
            serde_json::json!({}), &format!("Template '{}' not found", id), "NOT_FOUND", vec![],
        ),
    }
}

/// POST /v1/templates/:id/deactivate — Deactivate a template.
pub async fn deactivate_template(
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let removed = state.template_store.deactivate(&id);
    StandardResponse::ok(serde_json::json!({"deactivated": removed}), "Template deactivated.")
}

/// GET /v1/templates/active — List only active templates with their panel configs.
pub async fn list_active_templates(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let active = state.template_store.list_active();
    StandardResponse::ok(serde_json::to_value(&active).unwrap_or_default(), "Active templates.")
}

/// GET /v1/templates/:id/data — Get live metric data for an active template's panels.
pub async fn get_template_data(
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    let active = match state.template_store.get_active(&id) {
        Some(a) => a,
        None => return StandardResponse::error(
            serde_json::json!({}), "Template not active", "NOT_ACTIVE", vec![],
        ),
    };

    let now_us = now_us();
    let lookback_us = 3600 * 1_000_000u64; // 1 hour
    let window = crate::storage::event_store::TimeWindow::new(now_us.saturating_sub(lookback_us), now_us);

    let mut panel_data: Vec<serde_json::Value> = Vec::new();

    for panel in &active.template.panels {
        // Query the metric referenced in the panel
        let metric_name = extract_metric_from_query(&panel.query);
        let metrics = state.metric_store
            .query(Some(&metric_name), &HashMap::new(), window)
            .await
            .unwrap_or_default();

        let data_points: Vec<serde_json::Value> = metrics.iter().map(|m| {
            serde_json::json!({
                "timestamp_us": m.timestamp_us,
                "value": m.value,
            })
        }).collect();

        let latest_value = metrics.last().map(|m| m.value).unwrap_or(0.0);

        panel_data.push(serde_json::json!({
            "title": panel.title,
            "query": panel.query,
            "chart_type": panel.chart_type,
            "unit": panel.unit,
            "current_value": latest_value,
            "data_points": data_points,
            "has_data": !metrics.is_empty(),
        }));
    }

    StandardResponse::ok(serde_json::json!({
        "template_id": id,
        "template_name": active.template.name,
        "panels": panel_data,
    }), "Template data.")
}

fn extract_metric_from_query(query: &str) -> String {
    // Strip function wrappers: avg(X) → X, rate(X[5m]) → X
    let inner = if let Some(paren) = query.find('(') {
        let end = query.rfind(')').unwrap_or(query.len());
        &query[paren + 1..end]
    } else {
        query
    };
    let name = if let Some(bracket) = inner.find('[') {
        &inner[..bracket]
    } else {
        inner
    };
    let name = if let Some(brace) = name.find('{') {
        &name[..brace]
    } else {
        name
    };
    name.trim().to_string()
}

/// Built-in templates with real PromQL queries.
fn built_in_templates() -> Vec<DashboardTemplate> {
    vec![
        DashboardTemplate {
            id: "host-system".into(), name: "Host System".into(), category: "Infrastructure".into(),
            description: "CPU, memory, disk, and network for bare metal or VMs.".into(),
            metrics: vec!["system.cpu.usage".into(), "system.memory.used".into(), "system.disk.usage".into(), "system.network.bytes.in".into()],
            panels: vec![
                TemplatePanel { title: "CPU Usage".into(), query: "avg(system.cpu.usage)".into(), chart_type: "area".into(), unit: Some("%".into()), position: 1 },
                TemplatePanel { title: "Memory Used".into(), query: "avg(system.memory.used)".into(), chart_type: "area".into(), unit: Some("MB".into()), position: 2 },
                TemplatePanel { title: "Disk Usage".into(), query: "avg(system.disk.usage)".into(), chart_type: "area".into(), unit: Some("%".into()), position: 3 },
                TemplatePanel { title: "Network In".into(), query: "rate(system.network.bytes.in[5m])".into(), chart_type: "area".into(), unit: Some("B/s".into()), position: 4 },
            ],
        },
        DashboardTemplate {
            id: "kubernetes".into(), name: "Kubernetes Cluster".into(), category: "Container".into(),
            description: "Pod health, node resources, deployment status, and restarts.".into(),
            metrics: vec!["k8s.pod.cpu".into(), "k8s.pod.memory".into(), "k8s.pod.restarts".into(), "k8s.node.cpu".into()],
            panels: vec![
                TemplatePanel { title: "Node CPU".into(), query: "avg(k8s.node.cpu)".into(), chart_type: "area".into(), unit: Some("%".into()), position: 1 },
                TemplatePanel { title: "Pod Memory".into(), query: "avg(k8s.pod.memory)".into(), chart_type: "area".into(), unit: Some("MB".into()), position: 2 },
                TemplatePanel { title: "Pod Restarts".into(), query: "sum(k8s.pod.restarts)".into(), chart_type: "bar".into(), unit: None, position: 3 },
                TemplatePanel { title: "Pod CPU".into(), query: "avg(k8s.pod.cpu)".into(), chart_type: "area".into(), unit: Some("%".into()), position: 4 },
            ],
        },
        DashboardTemplate {
            id: "postgresql".into(), name: "PostgreSQL".into(), category: "Database".into(),
            description: "Connections, query latency, cache hit ratio, and WAL size.".into(),
            metrics: vec!["pg.connections.active".into(), "pg.query.latency.p99".into(), "pg.cache.hit.ratio".into(), "pg.deadlocks".into()],
            panels: vec![
                TemplatePanel { title: "Active Connections".into(), query: "avg(pg.connections.active)".into(), chart_type: "area".into(), unit: None, position: 1 },
                TemplatePanel { title: "Query Latency P99".into(), query: "avg(pg.query.latency.p99)".into(), chart_type: "area".into(), unit: Some("ms".into()), position: 2 },
                TemplatePanel { title: "Cache Hit Ratio".into(), query: "avg(pg.cache.hit.ratio)".into(), chart_type: "gauge".into(), unit: Some("%".into()), position: 3 },
                TemplatePanel { title: "Deadlocks".into(), query: "sum(pg.deadlocks)".into(), chart_type: "bar".into(), unit: None, position: 4 },
            ],
        },
        DashboardTemplate {
            id: "redis".into(), name: "Redis".into(), category: "Database".into(),
            description: "Hit rate, memory usage, connected clients, and operations per second.".into(),
            metrics: vec!["redis.hit.rate".into(), "redis.memory.used".into(), "redis.connected.clients".into(), "redis.ops.per.sec".into()],
            panels: vec![
                TemplatePanel { title: "Operations/sec".into(), query: "avg(redis.ops.per.sec)".into(), chart_type: "area".into(), unit: Some("ops/s".into()), position: 1 },
                TemplatePanel { title: "Memory Used".into(), query: "avg(redis.memory.used)".into(), chart_type: "area".into(), unit: Some("MB".into()), position: 2 },
                TemplatePanel { title: "Hit Rate".into(), query: "avg(redis.hit.rate)".into(), chart_type: "gauge".into(), unit: Some("%".into()), position: 3 },
                TemplatePanel { title: "Connected Clients".into(), query: "avg(redis.connected.clients)".into(), chart_type: "bar".into(), unit: None, position: 4 },
            ],
        },
        DashboardTemplate {
            id: "nginx".into(), name: "Nginx".into(), category: "Web".into(),
            description: "Request rate, error rate, response time, and active connections.".into(),
            metrics: vec!["nginx.requests.per.sec".into(), "nginx.errors.5xx".into(), "nginx.response.time".into(), "nginx.active.connections".into()],
            panels: vec![
                TemplatePanel { title: "Requests/sec".into(), query: "rate(nginx.requests.per.sec[1m])".into(), chart_type: "area".into(), unit: Some("req/s".into()), position: 1 },
                TemplatePanel { title: "5xx Errors".into(), query: "sum(nginx.errors.5xx)".into(), chart_type: "bar".into(), unit: None, position: 2 },
                TemplatePanel { title: "Response Time".into(), query: "avg(nginx.response.time)".into(), chart_type: "area".into(), unit: Some("ms".into()), position: 3 },
                TemplatePanel { title: "Active Connections".into(), query: "avg(nginx.active.connections)".into(), chart_type: "area".into(), unit: None, position: 4 },
            ],
        },
        DashboardTemplate {
            id: "http-api".into(), name: "HTTP API".into(), category: "Web".into(),
            description: "Latency percentiles, error rates, and throughput by endpoint.".into(),
            metrics: vec!["http.request.duration.p99".into(), "http.error.rate".into(), "http.requests.total".into()],
            panels: vec![
                TemplatePanel { title: "P99 Latency".into(), query: "avg(http.request.duration.p99)".into(), chart_type: "area".into(), unit: Some("ms".into()), position: 1 },
                TemplatePanel { title: "Error Rate".into(), query: "avg(http.error.rate)".into(), chart_type: "area".into(), unit: Some("%".into()), position: 2 },
                TemplatePanel { title: "Throughput".into(), query: "rate(http.requests.total[1m])".into(), chart_type: "area".into(), unit: Some("req/s".into()), position: 3 },
            ],
        },
        DashboardTemplate {
            id: "kafka".into(), name: "Apache Kafka".into(), category: "Queue".into(),
            description: "Consumer lag, throughput, partition count, and ISR status.".into(),
            metrics: vec!["kafka.consumer.lag".into(), "kafka.messages.in".into(), "kafka.messages.out".into(), "kafka.partitions".into()],
            panels: vec![
                TemplatePanel { title: "Consumer Lag".into(), query: "avg(kafka.consumer.lag)".into(), chart_type: "area".into(), unit: None, position: 1 },
                TemplatePanel { title: "Messages In".into(), query: "rate(kafka.messages.in[1m])".into(), chart_type: "area".into(), unit: Some("msg/s".into()), position: 2 },
                TemplatePanel { title: "Messages Out".into(), query: "rate(kafka.messages.out[1m])".into(), chart_type: "area".into(), unit: Some("msg/s".into()), position: 3 },
                TemplatePanel { title: "Partitions".into(), query: "avg(kafka.partitions)".into(), chart_type: "bar".into(), unit: None, position: 4 },
            ],
        },
        DashboardTemplate {
            id: "nodejs".into(), name: "Node.js".into(), category: "Runtime".into(),
            description: "Event loop lag, heap usage, GC pauses, and active handles.".into(),
            metrics: vec!["node.event.loop.lag".into(), "node.heap.used".into(), "node.gc.pause".into(), "node.active.handles".into()],
            panels: vec![
                TemplatePanel { title: "Event Loop Lag".into(), query: "avg(node.event.loop.lag)".into(), chart_type: "area".into(), unit: Some("ms".into()), position: 1 },
                TemplatePanel { title: "Heap Used".into(), query: "avg(node.heap.used)".into(), chart_type: "area".into(), unit: Some("MB".into()), position: 2 },
                TemplatePanel { title: "GC Pause".into(), query: "avg(node.gc.pause)".into(), chart_type: "bar".into(), unit: Some("ms".into()), position: 3 },
                TemplatePanel { title: "Active Handles".into(), query: "avg(node.active.handles)".into(), chart_type: "area".into(), unit: None, position: 4 },
            ],
        },
    ]
}
