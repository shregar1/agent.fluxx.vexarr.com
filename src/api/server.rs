use crate::api::ApiState;
use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

/// ApiServer: High-performance Axum-based web server for Fluxx UI.
/// Provides a unified REST/WebSocket interface for telemetry observation.
pub struct ApiServer {
    state: Arc<ApiState>,
    addr: String,
}

impl ApiServer {
    pub fn new(state: Arc<ApiState>, addr: &str) -> Self {
        Self {
            state,
            addr: addr.to_string(),
        }
    }

    /// Primary entry point for activating the API management layer.
    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/v1/sessions", get(list_sessions))
            .route("/v1/sessions/:id", get(get_session))
            .route("/v1/anomalies", get(list_anomalies))
            .route("/v1/stream", get(stream_ws))
            .layer(CorsLayer::permissive())
            .with_state(self.state.clone());

        info!("API: Binding Fluxx UI layer to {}", self.addr);
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// REST Handlers
async fn list_sessions(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    Json(state.session_manager.list_sessions().await)
}

async fn get_session(Path(id): Path<String>, State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    match state.session_manager.get_session(&id).await {
        Some(session) => Json(session).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Session not found").into_response(),
    }
}

async fn list_anomalies() -> impl IntoResponse {
    // Note: Future feature to integrate AlertEngine history retrieval.
    Json(Vec::<String>::new())
}

// WebSocket Streaming with dynamic hierarchy filtering
async fn stream_ws(
    ws: WebSocketUpgrade, 
    State(state): State<Arc<ApiState>>,
    Query(params): Query<std::collections::HashMap<String, String>>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, params))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<ApiState>, filters: std::collections::HashMap<String, String>) {
    let mut rx = state.event_bus.subscribe();
    
    let target_project = filters.get("project_id").cloned();
    let target_env = filters.get("environment_id").cloned();
    let target_service = filters.get("service_id").cloned();

    info!("API: Real-time logs session (Filtered={})", target_service.is_some());

    while let Ok(event) = rx.recv().await {
        // Apply dynamic filtering against the organizational hierarchy
        if let Some(p) = &target_project { if event.project_id.as_ref() != Some(p) { continue; } }
        if let Some(e) = &target_env { if event.environment_id.as_ref() != Some(e) { continue; } }
        if let Some(s) = &target_service { if event.service_id.as_ref() != Some(s) { continue; } }

        let msg = serde_json::to_string(&*event).unwrap_or_default();
        if let Err(_) = socket.send(Message::Text(msg.into())).await {
            break;
        }
    }
}
