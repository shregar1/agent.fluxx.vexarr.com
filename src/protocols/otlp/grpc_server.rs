use crate::models::event::Event;
use crate::models::metric::Metric;
use crate::models::log_entry::LogEntry;
use crate::storage::event_store::EventStore;
use crate::storage::metric_store::MetricStore;
use crate::storage::log_store::LogStore;
use crate::protocols::otlp::convert;
use std::sync::Arc;
use tokio::sync::broadcast;
use tonic::{Request, Response, Status};
use tracing::{info, error};

use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use opentelemetry_proto::tonic::collector::metrics::v1::{
    metrics_service_server::{MetricsService, MetricsServiceServer},
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
};
use opentelemetry_proto::tonic::collector::logs::v1::{
    logs_service_server::{LogsService, LogsServiceServer},
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};

/// OTLP Trace gRPC service.
pub struct OtlpTraceService {
    event_store: Arc<dyn EventStore>,
    event_bus: broadcast::Sender<Arc<Event>>,
}

impl OtlpTraceService {
    pub fn new(event_store: Arc<dyn EventStore>, event_bus: broadcast::Sender<Arc<Event>>) -> Self {
        Self { event_store, event_bus }
    }
}

#[tonic::async_trait]
impl TraceService for OtlpTraceService {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        let req = request.into_inner();
        let events = convert::otlp_spans_to_events(&req.resource_spans);
        let count = events.len();

        for event in events {
            let event_arc = Arc::new(event.clone());
            let _ = self.event_bus.send(event_arc);
            if let Err(e) = self.event_store.insert(event).await {
                error!("OTLP trace store error: {}", e);
            }
        }

        info!("OTLP: Ingested {} spans via gRPC", count);
        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}

/// OTLP Metrics gRPC service.
pub struct OtlpMetricsService {
    metric_store: Arc<dyn MetricStore>,
    metric_bus: broadcast::Sender<Arc<Metric>>,
}

impl OtlpMetricsService {
    pub fn new(metric_store: Arc<dyn MetricStore>, metric_bus: broadcast::Sender<Arc<Metric>>) -> Self {
        Self { metric_store, metric_bus }
    }
}

#[tonic::async_trait]
impl MetricsService for OtlpMetricsService {
    async fn export(
        &self,
        request: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        let req = request.into_inner();
        let metrics = convert::otlp_metrics_to_metrics(&req.resource_metrics);
        let count = metrics.len();

        for metric in metrics {
            let metric_arc = Arc::new(metric.clone());
            let _ = self.metric_bus.send(metric_arc);
            if let Err(e) = self.metric_store.insert(metric).await {
                error!("OTLP metric store error: {}", e);
            }
        }

        info!("OTLP: Ingested {} metric data points via gRPC", count);
        Ok(Response::new(ExportMetricsServiceResponse {
            partial_success: None,
        }))
    }
}

/// OTLP Logs gRPC service.
pub struct OtlpLogsService {
    log_store: Arc<dyn LogStore>,
    #[allow(dead_code)]
    log_bus: broadcast::Sender<Arc<LogEntry>>,
}

impl OtlpLogsService {
    pub fn new(log_store: Arc<dyn LogStore>, log_bus: broadcast::Sender<Arc<LogEntry>>) -> Self {
        Self { log_store, log_bus }
    }
}

#[tonic::async_trait]
impl LogsService for OtlpLogsService {
    async fn export(
        &self,
        request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        let req = request.into_inner();
        let entries = convert::otlp_logs_to_entries(&req.resource_logs);
        let count = entries.len();

        for entry in entries {
            if let Err(e) = self.log_store.insert(entry).await {
                error!("OTLP log store error: {}", e);
            }
        }

        info!("OTLP: Ingested {} log records via gRPC", count);
        Ok(Response::new(ExportLogsServiceResponse {
            partial_success: None,
        }))
    }
}

/// Start the OTLP gRPC server on the given port.
pub async fn start_otlp_grpc_server(
    port: u16,
    event_store: Arc<dyn EventStore>,
    event_bus: broadcast::Sender<Arc<Event>>,
    metric_store: Arc<dyn MetricStore>,
    metric_bus: broadcast::Sender<Arc<Metric>>,
    log_store: Arc<dyn LogStore>,
    log_bus: broadcast::Sender<Arc<LogEntry>>,
) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let trace_svc = OtlpTraceService::new(event_store, event_bus);
    let metrics_svc = OtlpMetricsService::new(metric_store, metric_bus);
    let logs_svc = OtlpLogsService::new(log_store, log_bus);

    info!("OTLP gRPC: Listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(TraceServiceServer::new(trace_svc))
        .add_service(MetricsServiceServer::new(metrics_svc))
        .add_service(LogsServiceServer::new(logs_svc))
        .serve(addr)
        .await?;

    Ok(())
}
