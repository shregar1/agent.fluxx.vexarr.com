//! CI/CD Visibility — Pipeline execution tracing and test analytics.
//! Tracks pipeline runs, stage durations, test results, and deployment frequency.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::{State, Query}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PipelineStatus { Running, Success, Failed, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRun {
    pub id: String,
    pub pipeline_name: String,
    pub provider: String,
    pub branch: String,
    pub commit_sha: String,
    pub status: PipelineStatus,
    pub started_at_us: u64,
    pub finished_at_us: Option<u64>,
    pub duration_secs: Option<f64>,
    pub stages: Vec<PipelineStage>,
    pub test_results: Option<TestResults>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    pub status: PipelineStatus,
    pub duration_secs: f64,
    pub started_at_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub duration_secs: f64,
    pub failures: Vec<TestFailure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    pub name: String,
    pub suite: String,
    pub message: String,
    pub duration_secs: f64,
}

/// DORA metrics computed from pipeline data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoraMetrics {
    pub deployment_frequency_per_day: f64,
    pub lead_time_for_changes_hours: f64,
    pub change_failure_rate_percent: f64,
    pub mean_time_to_recovery_hours: f64,
    pub period_days: u32,
}

#[derive(Debug, Default)]
pub struct CiStore {
    runs: RwLock<Vec<PipelineRun>>,
}

impl CiStore {
    pub fn new() -> Self { Self::default() }

    pub fn record(&self, run: PipelineRun) { self.runs.write().push(run); }

    pub fn list(&self, pipeline: Option<&str>, status: Option<PipelineStatus>, limit: usize) -> Vec<PipelineRun> {
        let runs = self.runs.read();
        runs.iter().rev()
            .filter(|r| pipeline.map_or(true, |p| r.pipeline_name == p))
            .filter(|r| status.map_or(true, |s| r.status == s))
            .take(limit).cloned().collect()
    }

    pub fn compute_dora(&self, period_days: u32) -> DoraMetrics {
        let runs = self.runs.read();
        let successful: Vec<&PipelineRun> = runs.iter().filter(|r| r.status == PipelineStatus::Success).collect();
        let failed: Vec<&PipelineRun> = runs.iter().filter(|r| r.status == PipelineStatus::Failed).collect();
        let total = runs.len() as f64;
        let days = period_days.max(1) as f64;

        DoraMetrics {
            deployment_frequency_per_day: successful.len() as f64 / days,
            lead_time_for_changes_hours: successful.iter()
                .filter_map(|r| r.duration_secs).sum::<f64>() / successful.len().max(1) as f64 / 3600.0,
            change_failure_rate_percent: if total > 0.0 { (failed.len() as f64 / total) * 100.0 } else { 0.0 },
            mean_time_to_recovery_hours: 0.5, // Placeholder — requires incident correlation
            period_days,
        }
    }
}

pub async fn record_pipeline(State(state): State<Arc<ApiState>>, Json(run): Json<PipelineRun>) -> impl IntoResponse {
    state.ci_store.record(run);
    StandardResponse::ok(serde_json::json!({"recorded": true}), "Pipeline run recorded.")
}

pub async fn list_pipelines(State(state): State<Arc<ApiState>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let pipeline = params.get("pipeline").map(|s| s.as_str());
    let limit: usize = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(50);
    let runs = state.ci_store.list(pipeline, None, limit);
    StandardResponse::ok(serde_json::to_value(&runs).unwrap_or_default(), "Pipeline runs.")
}

pub async fn get_dora_metrics(State(state): State<Arc<ApiState>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let days: u32 = params.get("days").and_then(|v| v.parse().ok()).unwrap_or(30);
    let dora = state.ci_store.compute_dora(days);
    StandardResponse::ok(serde_json::to_value(&dora).unwrap_or_default(), "DORA metrics.")
}
