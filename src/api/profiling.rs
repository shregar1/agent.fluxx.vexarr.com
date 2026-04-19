//! Continuous Profiling API
//!
//! Ingests runtime profiling data (CPU, memory, allocations) from language agents.
//! Supports pprof format (Go), JFR (Java), and generic flamegraph JSON.
//! Stores profiles and provides query APIs for analysis.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::{State, Query, Path}, response::IntoResponse, Json, body::Bytes};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::info;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProfileType { Cpu, Heap, Allocs, Goroutines, Threads, WallClock, Mutex, Block }

/// A stored profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub service: String,
    pub profile_type: ProfileType,
    pub format: String,
    pub duration_secs: f64,
    pub timestamp_us: u64,
    pub sample_count: u64,
    pub environment: Option<String>,
    pub version: Option<String>,
    pub runtime: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    /// Top functions by CPU/memory (pre-extracted for fast queries)
    pub top_functions: Vec<ProfileFunction>,
    /// Raw profile data size in bytes
    pub data_size: usize,
}

/// A function entry in a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFunction {
    pub function_name: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub self_value: f64,
    pub total_value: f64,
    pub self_percent: f64,
    pub total_percent: f64,
}

/// Profile search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub id: String,
    pub service: String,
    pub profile_type: ProfileType,
    pub duration_secs: f64,
    pub timestamp_us: u64,
    pub sample_count: u64,
    pub runtime: String,
    pub top_function: Option<String>,
}

fn now_us() -> u64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_micros() as u64 }

#[derive(Debug, Default)]
pub struct ProfilingStore {
    profiles: RwLock<HashMap<String, Profile>>,
    /// Raw profile data (kept separate to avoid cloning large blobs)
    raw_data: RwLock<HashMap<String, Vec<u8>>>,
}

impl ProfilingStore {
    pub fn new() -> Self { Self::default() }

    pub fn store_profile(&self, mut profile: Profile, raw: Option<Vec<u8>>) -> String {
        if profile.id.is_empty() { profile.id = uuid::Uuid::new_v4().to_string(); }
        if profile.timestamp_us == 0 { profile.timestamp_us = now_us(); }
        let id = profile.id.clone();

        if let Some(data) = raw {
            profile.data_size = data.len();
            self.raw_data.write().insert(id.clone(), data);
        }

        self.profiles.write().insert(id.clone(), profile);
        id
    }

    pub fn list_profiles(&self, service: Option<&str>, profile_type: Option<ProfileType>, limit: usize) -> Vec<ProfileSummary> {
        let profiles = self.profiles.read();
        let mut result: Vec<ProfileSummary> = profiles.values()
            .filter(|p| service.map_or(true, |s| p.service == s))
            .filter(|p| profile_type.map_or(true, |t| p.profile_type == t))
            .map(|p| ProfileSummary {
                id: p.id.clone(), service: p.service.clone(), profile_type: p.profile_type,
                duration_secs: p.duration_secs, timestamp_us: p.timestamp_us,
                sample_count: p.sample_count, runtime: p.runtime.clone(),
                top_function: p.top_functions.first().map(|f| f.function_name.clone()),
            })
            .collect();
        result.sort_by(|a, b| b.timestamp_us.cmp(&a.timestamp_us));
        result.truncate(limit);
        result
    }

    pub fn get_profile(&self, id: &str) -> Option<Profile> {
        self.profiles.read().get(id).cloned()
    }

    pub fn get_raw(&self, id: &str) -> Option<Vec<u8>> {
        self.raw_data.read().get(id).cloned()
    }
}

/// POST /v1/profiling — Ingest a profile.
pub async fn ingest_profile(
    State(state): State<Arc<ApiState>>,
    Json(profile): Json<Profile>,
) -> impl IntoResponse {
    let id = state.profiling_store.store_profile(profile, None);
    info!("Profiling: Stored profile {}", id);
    StandardResponse::ok(serde_json::json!({"id": id}), "Profile stored.")
}

/// POST /v1/profiling/raw — Ingest raw profile data (pprof/JFR binary).
pub async fn ingest_raw_profile(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<HashMap<String, String>>,
    body: Bytes,
) -> impl IntoResponse {
    let service = params.get("service").cloned().unwrap_or_else(|| "unknown".to_string());
    let runtime = params.get("runtime").cloned().unwrap_or_else(|| "unknown".to_string());
    let profile_type = match params.get("type").map(|s| s.as_str()) {
        Some("cpu") => ProfileType::Cpu,
        Some("heap") => ProfileType::Heap,
        Some("allocs") => ProfileType::Allocs,
        Some("goroutines") => ProfileType::Goroutines,
        _ => ProfileType::Cpu,
    };

    let profile = Profile {
        id: String::new(), service, profile_type, format: "pprof".to_string(),
        duration_secs: params.get("duration").and_then(|v| v.parse().ok()).unwrap_or(30.0),
        timestamp_us: 0, sample_count: 0, environment: params.get("env").cloned(),
        version: params.get("version").cloned(), runtime,
        labels: HashMap::new(), top_functions: vec![], data_size: body.len(),
    };

    let id = state.profiling_store.store_profile(profile, Some(body.to_vec()));
    info!("Profiling: Stored raw profile {} ({} bytes)", id, body.len());
    StandardResponse::ok(serde_json::json!({"id": id, "size": body.len()}), "Raw profile stored.")
}

/// GET /v1/profiling — List profiles.
pub async fn list_profiles(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let service = params.get("service").map(|s| s.as_str());
    let limit: usize = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(50);
    let profiles = state.profiling_store.list_profiles(service, None, limit);
    StandardResponse::ok(serde_json::to_value(&profiles).unwrap_or_default(), "Profiles.")
}

/// GET /v1/profiling/:id — Get profile detail with top functions.
pub async fn get_profile(
    Path(id): Path<String>,
    State(state): State<Arc<ApiState>>,
) -> impl IntoResponse {
    match state.profiling_store.get_profile(&id) {
        Some(p) => StandardResponse::ok(serde_json::to_value(&p).unwrap_or_default(), "Profile detail."),
        None => StandardResponse::error(serde_json::json!({}), "Profile not found", "NOT_FOUND", vec![]),
    }
}
