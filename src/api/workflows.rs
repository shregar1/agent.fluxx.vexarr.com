//! Workflow Automation — if-this-then-that for observability.
//! Trigger actions automatically when alerts fire or conditions are met.
//! Examples: alert fires → create Jira ticket → notify Slack → page on-call.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub trigger: WorkflowTrigger,
    pub steps: Vec<WorkflowStep>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub created_at_us: u64,
    pub last_run_us: Option<u64>,
    pub run_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowTrigger {
    AlertFired { alert_rule: String, severity: Option<String> },
    MetricThreshold { metric: String, operator: String, value: f64 },
    IncidentCreated { severity: Option<String> },
    Schedule { cron: String },
    Webhook { path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowStep {
    Notify { channel: String, message: String },
    CreateTicket { provider: String, project: String, title_template: String },
    Webhook { url: String, method: String, body_template: Option<String> },
    RunQuery { query: String, save_as: Option<String> },
    Wait { seconds: u64 },
    Condition { query: String, operator: String, value: f64, on_true: Vec<WorkflowStep>, on_false: Vec<WorkflowStep> },
    CreateIncident { title_template: String, severity: String },
    UpdateIncident { incident_id: String, status: String, message: String },
}

fn now_us() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_micros() as u64 }

#[derive(Debug, Default)]
pub struct WorkflowStore {
    workflows: RwLock<HashMap<String, Workflow>>,
}

impl WorkflowStore {
    pub fn new() -> Self { Self::default() }
    pub fn create(&self, mut wf: Workflow) -> Workflow {
        if wf.id.is_empty() { wf.id = uuid::Uuid::new_v4().to_string(); }
        wf.created_at_us = now_us(); wf.run_count = 0;
        self.workflows.write().insert(wf.id.clone(), wf.clone()); wf
    }
    pub fn list(&self) -> Vec<Workflow> { self.workflows.read().values().cloned().collect() }
    pub fn get(&self, id: &str) -> Option<Workflow> { self.workflows.read().get(id).cloned() }
    pub fn delete(&self, id: &str) -> bool { self.workflows.write().remove(id).is_some() }
    pub fn toggle(&self, id: &str) -> Option<bool> {
        let mut store = self.workflows.write();
        store.get_mut(id).map(|wf| { wf.enabled = !wf.enabled; wf.enabled })
    }
}

pub async fn create_workflow(State(state): State<Arc<ApiState>>, Json(wf): Json<Workflow>) -> impl IntoResponse {
    let created = state.workflow_store.create(wf);
    StandardResponse::ok(serde_json::to_value(&created).unwrap_or_default(), "Workflow created.")
}
pub async fn list_workflows(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    StandardResponse::ok(serde_json::to_value(&state.workflow_store.list()).unwrap_or_default(), "Workflows.")
}
pub async fn get_workflow(Path(id): Path<String>, State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    match state.workflow_store.get(&id) {
        Some(wf) => StandardResponse::ok(serde_json::to_value(&wf).unwrap_or_default(), "Workflow."),
        None => StandardResponse::error(serde_json::json!({}), "Not found", "NOT_FOUND", vec![]),
    }
}
