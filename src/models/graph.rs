use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node: Represents a service or network endpoint in the Fluxx flow graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub protocol: String,
}

/// Edge: Represents a transactional request or stream between two nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source_id: String,
    pub target_id: String,
    pub protocol: String,
    pub count: u64,
    pub avg_latency_us: u64,
}

/// Graph: A real-time representation of service topology and request flow.
/// Designed for high-performance JSON output to frontend visualization layers.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, Edge>, // Key: "source_id:target_id:protocol"
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.entry(node.id.clone()).or_insert(node);
    }

    pub fn update_edge(&mut self, source_id: &str, target_id: &str, protocol: &str, latency_us: u64) {
        let edge_key = format!("{}:{}:{}", source_id, target_id, protocol);
        let edge = self.edges.entry(edge_key).or_insert(Edge {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            protocol: protocol.to_string(),
            count: 0,
            avg_latency_us: 0,
        });

        // Update moving average (simplified)
        let new_count = edge.count + 1;
        edge.avg_latency_us = ((edge.avg_latency_us * edge.count) + latency_us) / new_count;
        edge.count = new_count;
    }
}
