use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Project: Top-level organizational unit for a collection of environments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub environments: HashMap<String, Environment>,
}

/// Environment: Scoped deployment context (e.g., Staging, Production).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub services: HashMap<String, Service>,
}

/// Service: Individual application or network component being monitored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub protocol_configs: Vec<String>, // List of associated capturer names
}

/// WorkspaceManager: Unified state for managing the Fluxx project hierarchy.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorkspaceManager {
    pub projects: HashMap<String, Project>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.insert(project.id.clone(), project);
    }
}
