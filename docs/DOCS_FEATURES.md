# Fluxx Edge Agent: Feature Architecture

Fluxx is a high-performance, modular network observability agent designed for the modern edge. It provides deep visibility into protocol semantics with minimal overhead.

---

## 1. Multi-Protocol Capture Layer

Fluxx captures and normalizes traffic from disparate network sources into a unified event format.

- **HTTP/S**: Periodic health and endpoint observation.
- **WebSocket**: Full-duplex stream monitoring with handshake extraction.
- **gRPC**: Low-latency RPC tracing using interceptors.
- **QUIC**: Modern UDP-based transport observation using `quinn`.
- **eBPF (XDP)**: Kernel-level packet capture with zero-context-switch metadata extraction (TCP/UDP, Port, Payload Size).

## 2. Asynchronous Event Pipeline

Events flow through a high-concurrency, middleware-driven pipeline.

- **`CaptureChannel`**: Managed MPSC buffers with backpressure support to prevent memory spikes during traffic surges.
- **`WorkerPool`**: Semaphore-bounded concurrency to ensure predictable CPU utilization.

## 3. Protocol Intelligence (Analysis)

An anomaly detection layer that identifies patterns without storing entire payloads.

- **`HTTPAnalyzer`**: Detects high-latency endpoints and payload anomalies.
- **`WebSocketAnalyzer`**: Specifically detects **Reconnect Storms** and connection lifecycle patterns.
- **`GrpcAnalyzer`**: Identifies RPC spikes and stream resets.
- **`EbpfAnalyzer`**: Monitors kernel-level traffic surges and volume spikes.

## 4. Logical Session Tracking

Group related events into logical units.

- **Session Linking**: Automatic correlation of events via `session_id`.
- **Timeline Management**: Maintains a chronological history of a session's activity.
- **Eviction**: Automatic memory management for stale sessions.

## 5. Security & Privacy Layer

Enforces data protection policies before events leave the agent.

- **PII Masking**: High-performance regex engine for redacting Emails, CCs, and Auth Tokens.
- **Local-Only Mode**: Prevents sensitive data from being transported to remote sinks.
- **Encryption**: (Conceptual) In-pipeline payload protection.

## 6. Time-Travel & Traffic Replay

Diagnostic tools for reproducing edge conditions.

- **`EventStore`**: BTreeMap-indexed historical storage for O(log N) range queries.
- **`ReplayEngine`**: Chronological streaming with deterministic sorting.
- **Chaos Replay**: Pluggable strategies to inject **Latency**, **Drop %**, and **Payload Mutations** during simulation.

## 7. Real-Time Flow Graph

Visualizes service topology and request flow.

- **Dynamic Mapping**: Automatically identifies Nodes (services) and Edges (transactions).
- **Metric Aggregation**: Tracks average latency and call volume per flow.
- **JSON Export**: Real-time topology streaming for HUD-style dashboards.

## 8. Plugin Architecture

A trait-based extension system allowing third-party modules to inject custom `Processors` and `Analyzers`.

---

## 9. CLI Interface

Control the agent via the terminal:

- `fluxx inspect <target>`: Real-time protocol diagnostics.
- `fluxx start`: Activation with configurable flags (`--config`, `--enable-ebpf`).
- `fluxx config`: Manage local deployment settings.
