# Fluxx Edge Agent: Feature Architecture

Fluxx is a high-performance network observability agent designed for the edge. It provides a modular, middleware-driven pipeline for capturing, processing, and delivering network events at massive scale.

## 1. Multi-Protocol Ingestion (Capture Layer)
The agent leverages specialized **Capturers** for heterogeneous protocol sources:
- **HTTP/Fetch**: Periodic or on-demand capture from REST endpoints using `reqwest`.
- **WebSockets (WS)**: High-performance TCP-backed WebSocket server logic using `tokio-tungstenite`.
- **QUIC**: Native support for bi-directional UDP-based streams using the `quinn` implementation.
- **gRPC**: Low-latency RPC observation using `tonic` and `prost` interceptors.
- **eBPF (TCP/UDP)**: Kernel-level packet observation via XDP/TC probes using the `aya` framework (TCP/UDP ingress/egress).

## 2. Event Processing Pipeline (Middleware Layer)
Fluxx employs a linear, sequential middleware pattern for event enrichment and transformation:
- **Tagging**: Enriches events with environment-specific metadata (e.g., `region`, `env`, `agent_id`).
- **Latency Tracking**: Microsecond-precision calculation of system processing delay.
- **Categorical Bucketization**: Automatic classification of events into latency buckets (e.g., `10ms`, `50ms`, `overflow`).
- **Dynamic Filtering**: Logic-based dropping of noisy or irrelevant events to save downstream bandwidth.

## 3. High-Performance Runtime
Optimized for zero-copy efficiency and resource management:
- **Zero-Copy Payloads**: Payload sharing across multiple delivery sinks using `bytes::Bytes`.
- **Reference Sharing**: Heavy event structures (metadata) are shared via `Arc<Event>` across all transporters.
- **Worker Pooling**: Concurrency-limited processing tasks via `tokio::sync::Semaphore` to prevent thread exhaustion.
- **Bounded Backpressure**: Native buffer management via `mpsc::channel` ensures ingestion scales with downstream throughput.

## 4. Reliable Event Transport (Delivery Layer)
- **Managed Retries**: Automatic delivery retries with exponential backoff for network-bound transporters.
- **JSON Serialization**: Efficient, field-mapped serialization for upstream analytics ingestion.
- **Multiplexed Destinations**: Parallel fan-out support for delivery to multiple sinks (e.g., HTTP Analytics, Local Logs, Stdout).

## 5. Architectural Principles
- **Scalable**: Built for high-throughput edge environments.
- **Decoupled**: Clean separation between protocol handlers, core logic, and CLI.
- **Observable**: Structured logging and granular error diagnostics throughout.
