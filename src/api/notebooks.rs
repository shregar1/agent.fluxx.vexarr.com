//! Notebooks — Collaborative investigation documents with embedded queries.
//! Like Datadog Notebooks: mix markdown, charts, and live queries in one document.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::{State, Path}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub created_at_us: u64,
    pub updated_at_us: u64,
    pub cells: Vec<NotebookCell>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub shared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotebookCell {
    Markdown { content: String },
    Query { query: String, visualization: String },
    Metric { metric_name: String, time_range: String },
    LogSearch { query: String, service: Option<String> },
    TraceView { trace_id: String },
    Image { url: String, caption: Option<String> },
}

fn now_us() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_micros() as u64 }

#[derive(Debug, Default)]
pub struct NotebookStore {
    notebooks: RwLock<HashMap<String, Notebook>>,
}

impl NotebookStore {
    pub fn new() -> Self { Self::default() }
    pub fn create(&self, mut nb: Notebook) -> Notebook {
        if nb.id.is_empty() { nb.id = uuid::Uuid::new_v4().to_string(); }
        nb.created_at_us = now_us(); nb.updated_at_us = nb.created_at_us;
        self.notebooks.write().insert(nb.id.clone(), nb.clone()); nb
    }
    pub fn update(&self, id: &str, mut nb: Notebook) -> Option<Notebook> {
        let mut store = self.notebooks.write();
        if store.contains_key(id) { nb.id = id.to_string(); nb.updated_at_us = now_us(); store.insert(id.to_string(), nb.clone()); Some(nb) } else { None }
    }
    pub fn list(&self) -> Vec<Notebook> {
        let store = self.notebooks.read();
        let mut r: Vec<Notebook> = store.values().cloned().collect();
        r.sort_by(|a, b| b.updated_at_us.cmp(&a.updated_at_us)); r
    }
    pub fn get(&self, id: &str) -> Option<Notebook> { self.notebooks.read().get(id).cloned() }
    pub fn delete(&self, id: &str) -> bool { self.notebooks.write().remove(id).is_some() }
}

pub async fn create_notebook(State(state): State<Arc<ApiState>>, Json(nb): Json<Notebook>) -> impl IntoResponse {
    let created = state.notebook_store.create(nb);
    StandardResponse::ok(serde_json::to_value(&created).unwrap_or_default(), "Notebook created.")
}
pub async fn list_notebooks(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    StandardResponse::ok(serde_json::to_value(&state.notebook_store.list()).unwrap_or_default(), "Notebooks.")
}
pub async fn get_notebook(Path(id): Path<String>, State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    match state.notebook_store.get(&id) {
        Some(nb) => StandardResponse::ok(serde_json::to_value(&nb).unwrap_or_default(), "Notebook."),
        None => StandardResponse::error(serde_json::json!({}), "Not found", "NOT_FOUND", vec![]),
    }
}
