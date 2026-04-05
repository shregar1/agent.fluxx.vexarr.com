use crate::api::{ApiState, StandardResponse};
use axum::{
    body::Body,
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, Path, State, Query},
    http::{HeaderMap, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tracing::{info, error};

/// Security middleware: API Key authentication
async fn auth_middleware(
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // In production, this should validate against a secure key store
    // For now, we require the X-API-Key header to be present
    let api_key = headers.get("X-API-Key")
        .and_then(|v| v.to_str().ok());
    
    // Check if API key is provided and valid (simple validation for now)
    // In production, compare against hashed keys from environment
    let valid_key = std::env::var("FLUXX_API_KEY").ok();
    
    match (api_key, valid_key) {
        (Some(key), Some(valid)) if key == valid => {},
        _ if std::env::var("FLUXX_DISABLE_AUTH").is_ok() => {}, // Allow bypass in dev
        _ => {
            error!("Unauthorized API access attempt");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    
    Ok(next.run(request).await)
}

/// Security middleware: Add security headers
async fn security_headers_middleware(
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    let mut response = next.run(request).await;
    
    // Add security headers
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", header::HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", header::HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", header::HeaderValue::from_static("1; mode=block"));
    headers.insert("Referrer-Policy", header::HeaderValue::from_static("strict-origin-when-cross-origin"));
    
    response
}

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
        // Configure CORS - restrict to specific origins in production
        let cors = if let Ok(allowed_origin) = std::env::var("FLUXX_CORS_ORIGIN") {
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(allowed_origin.parse()?))
                .allow_methods([axum::http::Method::GET])
                .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        } else {
            // Default: restrictive CORS (no cross-origin requests)
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(format!("http://{}", self.addr).parse()?))
                .allow_methods([axum::http::Method::GET])
        };

        let app = Router::new()
            .route("/v1/sessions", get(list_sessions))
            .route("/v1/sessions/:id", get(get_session))
            .route("/v1/anomalies", get(list_anomalies))
            .route("/v1/events", post(ingest_event))
            .route("/v1/stream", get(stream_ws))
            .layer(middleware::from_fn(security_headers_middleware))
            .layer(middleware::from_fn(auth_middleware))
            .layer(cors)
            .with_state(self.state.clone());

        info!("API: Binding Fluxx UI layer to {} (auth required)", self.addr);
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// REST Handlers with standardized response wrapping
async fn list_sessions(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let sessions = state.session_manager.list_sessions().await;
    StandardResponse::ok(sessions, "Sessions list retrieved successfully.")
}

async fn get_session(Path(id): Path<String>, State(state): State<Arc<ApiState>>) -> axum::response::Response {
    match state.session_manager.get_session(&id).await {
        Some(session) => StandardResponse::ok(session, "Session detail retrieved.").into_response(),
        None => StandardResponse::error(
            serde_json::json!({}), 
            "Session not found.", 
            "NOT_FOUND", 
            vec!["The requested session might have been evicted.".to_string()]
        ).into_response(),
    }
}

async fn list_anomalies() -> impl IntoResponse {
    // Note: Future feature to integrate AlertEngine history retrieval.
    StandardResponse::ok(Vec::<String>::new(), "Anomalies search results.")
}

async fn ingest_event(
    State(state): State<Arc<ApiState>>,
    Json(event): Json<Event>,
) -> impl IntoResponse {
    let event_arc = Arc::new(event);
    
    // Broadcast for real-time visualization
    let _ = state.event_bus.send(event_arc.clone());
    
    // Persist for historical search
    let _ = state.event_store.store_event(event_arc).await;

    StandardResponse::ok(serde_json::json!({}), "Event ingested successfully.")
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
