use crate::api::{ApiState, StandardResponse};
use crate::models::event::Event;
use axum::{
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
    let api_key = headers.get("X-API-Key")
        .and_then(|v| v.to_str().ok());
    
    let valid_key = std::env::var("FLUXX_API_KEY").ok();
    
    match (api_key, valid_key) {
        (Some(key), Some(valid)) if key == valid => {},
        (_, None) => {}, 
        _ if std::env::var("FLUXX_DISABLE_AUTH").is_ok() => {}, 
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
    
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", header::HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", header::HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", header::HeaderValue::from_static("1; mode=block"));
    headers.insert("Referrer-Policy", header::HeaderValue::from_static("strict-origin-when-cross-origin"));
    
    response
}

/// ApiServer: High-performance Axum-based web server for Fluxx telemetry.
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

    pub async fn start(&self) -> anyhow::Result<()> {
        let cors = if let Ok(allowed_origin) = std::env::var("FLUXX_CORS_ORIGIN") {
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(allowed_origin.parse()?))
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
                .allow_headers([axum::http::header::AUTHORIZATION, axum::http::header::CONTENT_TYPE])
        } else {
            CorsLayer::permissive()
        };

        let app = Router::new()
            .route("/v1/sessions", get(list_sessions))
            .route("/v1/sessions/:id", get(get_session))
            .route("/v1/anomalies", get(list_anomalies))
            .route("/v1/events", post(ingest_event).get(list_events))
            .route("/v1/stats", get(get_stats))
            .route("/v1/stream", get(stream_ws))
            // Log management endpoints
            .route("/v1/logs", post(crate::api::logs::ingest_logs).get(crate::api::logs::query_logs))
            .route("/v1/logs/stream", get(crate::api::logs::stream_logs_ws))
            // Metric query endpoints
            .route("/v1/metrics", get(crate::api::metrics::query_metrics))
            .route("/v1/metrics/names", get(crate::api::metrics::list_metric_names))
            // Prometheus-compatible scrape endpoint
            .route("/metrics", get(crate::api::metrics::prometheus_endpoint))
            // Service map endpoint
            .route("/v1/service-map", get(crate::api::service_map::get_service_map))
            // Deploy annotations
            .route("/v1/annotations", post(crate::api::annotations::create_annotation).get(crate::api::annotations::list_annotations))
            // PromQL-compatible query engine
            .route("/v1/query", get(crate::api::promql::query))
            // Trace detail API
            .route("/v1/traces", get(crate::api::traces::list_traces))
            .route("/v1/traces/:trace_id", get(crate::api::traces::get_trace))
            // SLO endpoints
            .route("/v1/slos", post(crate::api::slo::create_slo).get(crate::api::slo::list_slos))
            .route("/v1/slos/:id", get(crate::api::slo::get_slo))
            // Incidents
            .route("/v1/incidents", post(crate::api::incidents::create_incident).get(crate::api::incidents::list_incidents))
            .route("/v1/incidents/:id", get(crate::api::incidents::get_incident))
            .route("/v1/incidents/:id/status", post(crate::api::incidents::update_incident_status))
            // Notebooks
            .route("/v1/notebooks", post(crate::api::notebooks::create_notebook).get(crate::api::notebooks::list_notebooks))
            .route("/v1/notebooks/:id", get(crate::api::notebooks::get_notebook))
            // Workflows
            .route("/v1/workflows", post(crate::api::workflows::create_workflow).get(crate::api::workflows::list_workflows))
            .route("/v1/workflows/:id", get(crate::api::workflows::get_workflow))
            // Database Monitoring
            .route("/v1/dbm/queries", post(crate::api::dbm::ingest_slow_query).get(crate::api::dbm::list_slow_queries))
            .route("/v1/dbm/stats", get(crate::api::dbm::get_query_stats))
            // CI/CD Visibility
            .route("/v1/ci/pipelines", post(crate::api::ci::record_pipeline).get(crate::api::ci::list_pipelines))
            .route("/v1/ci/dora", get(crate::api::ci::get_dora_metrics))
            // Network Performance Monitoring
            .route("/v1/network/flows", post(crate::api::npm::record_flow).get(crate::api::npm::list_flows))
            .route("/v1/network/dns", post(crate::api::npm::record_dns))
            .route("/v1/network/summary", get(crate::api::npm::network_summary))
            // Real User Monitoring (RUM)
            .route("/v1/rum", post(crate::api::rum::ingest_rum))
            .route("/v1/rum/sessions", get(crate::api::rum::list_rum_sessions))
            .route("/v1/rum/vitals", get(crate::api::rum::get_web_vitals))
            .route("/v1/rum/stats", get(crate::api::rum::rum_stats))
            // Cloud Security Posture Management (CSPM)
            .route("/v1/cspm/findings", post(crate::api::cspm::report_finding).get(crate::api::cspm::list_findings))
            .route("/v1/cspm/compliance", get(crate::api::cspm::get_compliance))
            .route("/v1/cspm/rules", get(crate::api::cspm::list_security_rules))
            // Continuous Profiling
            .route("/v1/profiling", post(crate::api::profiling::ingest_profile).get(crate::api::profiling::list_profiles))
            .route("/v1/profiling/raw", post(crate::api::profiling::ingest_raw_profile))
            .route("/v1/profiling/:id", get(crate::api::profiling::get_profile))
            // Cluster status
            .route("/v1/cluster", get(crate::api::cluster::cluster_status))
            .route("/v1/cluster/topology", get(crate::api::cluster::cluster_topology))
            // Prometheus remote_write endpoint
            .route("/v1/prom/write", post(crate::protocols::prometheus::remote_write))
            // OTLP HTTP ingestion endpoints
            .route("/v1/otlp/traces", post(crate::protocols::otlp::http_handlers::ingest_traces))
            .route("/v1/otlp/metrics", post(crate::protocols::otlp::http_handlers::ingest_metrics))
            .route("/v1/otlp/logs", post(crate::protocols::otlp::http_handlers::ingest_logs))
            .layer(middleware::from_fn(security_headers_middleware))
            .layer(middleware::from_fn(auth_middleware))
            .layer(cors)
            .with_state(self.state.clone());

        info!("API: Binding Fluxx layer to {} (auth required if configured)", self.addr);
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

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
    StandardResponse::ok(Vec::<String>::new(), "Anomalies search results.")
}

async fn ingest_event(
    State(state): State<Arc<ApiState>>,
    Json(event): Json<Event>,
) -> impl IntoResponse {
    let event_arc = Arc::new(event);
    let _ = state.event_bus.send(event_arc.clone());
    let _ = state.event_store.insert((*event_arc).clone()).await;
    StandardResponse::ok(serde_json::json!({}), "Event ingested successfully.")
}

async fn list_events(
    State(state): State<Arc<ApiState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let project_id = params.get("project_id").cloned();
    let service_id = params.get("service_id").cloned();
    let protocol = params.get("protocol").cloned();
    let severity = params.get("severity").cloned();
    
    let protocol_enum = protocol.and_then(|p| {
        match p.to_lowercase().as_str() {
            "http" => Some(crate::models::event::ProtocolType::Http),
            "ws" => Some(crate::models::event::ProtocolType::Ws),
            "tcp" => Some(crate::models::event::ProtocolType::Tcp),
            "udp" => Some(crate::models::event::ProtocolType::Udp),
            "grpc" => Some(crate::models::event::ProtocolType::Grpc),
            "quic" => Some(crate::models::event::ProtocolType::Quic),
            "ebpf" => Some(crate::models::event::ProtocolType::Ebpf),
            "mqtt" => Some(crate::models::event::ProtocolType::Mqtt),
            "modbus" => Some(crate::models::event::ProtocolType::Modbus),
            "system" => Some(crate::models::event::ProtocolType::System),
            "otlp" => Some(crate::models::event::ProtocolType::Otlp),
            "statsd" => Some(crate::models::event::ProtocolType::StatsD),
            "log" => Some(crate::models::event::ProtocolType::Log),
            _ => None,
        }
    });

    let severity_enum = severity.and_then(|s| {
        match s.to_lowercase().as_str() {
            "info" => Some(crate::models::event::EventSeverity::Info),
            "low" => Some(crate::models::event::EventSeverity::Low),
            "medium" => Some(crate::models::event::EventSeverity::Medium),
            "high" => Some(crate::models::event::EventSeverity::High),
            "critical" => Some(crate::models::event::EventSeverity::Critical),
            _ => None,
        }
    });

    let events = state.event_store.list_events(project_id, protocol_enum, severity_enum, service_id).await;
    StandardResponse::ok(events, "Events retrieval successful.")
}

async fn get_stats(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let stats = state.event_store.get_stats().await;
    StandardResponse::ok(stats, "Agent operational statistics.")
}

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

    while let Ok(event) = rx.recv().await {
        if let Some(p) = &target_project { if event.project_id.as_ref() != Some(p) { continue; } }
        if let Some(e) = &target_env { if event.environment_id.as_ref() != Some(e) { continue; } }
        if let Some(s) = &target_service { if event.service_id.as_ref() != Some(s) { continue; } }

        let msg = serde_json::to_string(&*event).unwrap_or_default();
        if let Err(_) = socket.send(Message::Text(msg.into())).await {
            break;
        }
    }
}
