//! Network Performance Monitoring (NPM)
//! Tracks TCP/UDP flows, DNS queries, and network latency between services.
//! Receives flow data from eBPF or agent-collected network stats.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::{State, Query}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFlow {
    pub id: String,
    pub source_ip: String,
    pub source_port: u16,
    pub dest_ip: String,
    pub dest_port: u16,
    pub protocol: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub retransmits: u64,
    pub rtt_us: u64,
    pub source_service: Option<String>,
    pub dest_service: Option<String>,
    pub timestamp_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuery {
    pub id: String,
    pub query_name: String,
    pub query_type: String,
    pub response_code: String,
    pub latency_us: u64,
    pub source_ip: String,
    pub server_ip: String,
    pub timestamp_us: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSummary {
    pub total_flows: u64,
    pub total_bytes: u64,
    pub total_retransmits: u64,
    pub avg_rtt_us: u64,
    pub dns_queries: u64,
    pub dns_failures: u64,
    pub top_talkers: Vec<(String, u64)>,
}

#[derive(Debug, Default)]
pub struct NpmStore {
    flows: RwLock<Vec<NetworkFlow>>,
    dns: RwLock<Vec<DnsQuery>>,
}

impl NpmStore {
    pub fn new() -> Self { Self::default() }

    pub fn record_flow(&self, flow: NetworkFlow) { self.flows.write().push(flow); }
    pub fn record_dns(&self, query: DnsQuery) { self.dns.write().push(query); }

    pub fn get_flows(&self, source: Option<&str>, dest: Option<&str>, limit: usize) -> Vec<NetworkFlow> {
        let flows = self.flows.read();
        flows.iter().rev()
            .filter(|f| source.map_or(true, |s| f.source_service.as_deref() == Some(s) || f.source_ip == s))
            .filter(|f| dest.map_or(true, |d| f.dest_service.as_deref() == Some(d) || f.dest_ip == d))
            .take(limit).cloned().collect()
    }

    pub fn get_dns_queries(&self, name: Option<&str>, limit: usize) -> Vec<DnsQuery> {
        let dns = self.dns.read();
        dns.iter().rev()
            .filter(|d| name.map_or(true, |n| d.query_name.contains(n)))
            .take(limit).cloned().collect()
    }

    pub fn get_summary(&self) -> NetworkSummary {
        let flows = self.flows.read();
        let dns = self.dns.read();
        let total_bytes: u64 = flows.iter().map(|f| f.bytes_sent + f.bytes_received).sum();
        let total_retransmits: u64 = flows.iter().map(|f| f.retransmits).sum();
        let avg_rtt = if flows.is_empty() { 0 } else { flows.iter().map(|f| f.rtt_us).sum::<u64>() / flows.len() as u64 };
        let dns_failures = dns.iter().filter(|d| d.response_code != "NOERROR").count() as u64;

        // Top talkers by bytes
        let mut talkers: HashMap<String, u64> = HashMap::new();
        for f in flows.iter() {
            *talkers.entry(f.source_ip.clone()).or_default() += f.bytes_sent;
        }
        let mut top: Vec<(String, u64)> = talkers.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1));
        top.truncate(10);

        NetworkSummary {
            total_flows: flows.len() as u64, total_bytes, total_retransmits,
            avg_rtt_us: avg_rtt, dns_queries: dns.len() as u64, dns_failures, top_talkers: top,
        }
    }
}

pub async fn record_flow(State(state): State<Arc<ApiState>>, Json(flow): Json<NetworkFlow>) -> impl IntoResponse {
    state.npm_store.record_flow(flow);
    StandardResponse::ok(serde_json::json!({"recorded": true}), "Flow recorded.")
}

pub async fn list_flows(State(state): State<Arc<ApiState>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let src = params.get("source").map(|s| s.as_str());
    let dst = params.get("dest").map(|s| s.as_str());
    let limit: usize = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(50);
    StandardResponse::ok(serde_json::to_value(&state.npm_store.get_flows(src, dst, limit)).unwrap_or_default(), "Network flows.")
}

pub async fn network_summary(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    StandardResponse::ok(serde_json::to_value(&state.npm_store.get_summary()).unwrap_or_default(), "Network summary.")
}

pub async fn record_dns(State(state): State<Arc<ApiState>>, Json(query): Json<DnsQuery>) -> impl IntoResponse {
    state.npm_store.record_dns(query);
    StandardResponse::ok(serde_json::json!({"recorded": true}), "DNS query recorded.")
}
