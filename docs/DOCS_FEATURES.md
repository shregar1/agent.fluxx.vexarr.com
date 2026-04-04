# Fluxx Edge Agent: Feature Architecture

Fluxx is a high-performance, modular network observability agent designed for the modern edge. It provides deep visibility into protocol semantics with minimal overhead.

---

## 1. Multi-Protocol Capture Layer

Fluxx captures and normalizes traffic from disparate network sources into a unified event format.

- **Standard**: HTTP/S, WebSocket, gRPC, QUIC (UDP).
- **Industrial & IoT**: **MQTT**, **Modbus**, **CAN-Bus** (via eBPF).
- **eBPF (XDP)**: Kernel-level packet capture with zero-context-switch metadata extraction.

## 2. Asynchronous Event Pipeline

Events flow through a high-concurrency, middleware-driven pipeline.

- **`CaptureChannel`**: Managed MPSC buffers with backpressure support to prevent memory spikes during high-volume sensor surges.
- **`WorkerPool`**: Semaphore-bounded concurrency to ensure predictable CPU utilization on industrial edge nodes.

## 3. Protocol Intelligence (Industrial & Networking)

An anomaly detection layer that identifies patterns without storing entire payloads.

- **`HTTP/WS/GRPC`**: Standard networking anomalies (latency, reconnect storms).
- **`Sensor Analyzer`**: Identifies **Sensor Drift** and telemetry instability in IoT streams.
- **`Actuator Analyzer`**: Measures **PLC/Robotics Latency** and command deviations in factory automation.
- **`Vehicle Analyzer`**: Monitors ECU resets and CAN-bus congestion in Automobile environments.

## 4. Logical Session Tracking

Group related events (e.g., Robot command cycles or a vehicle's start-trip timeline) into logical units.

- **Session Linking**: Automatic correlation via `session_id`.
- **Eviction**: Lifecycle management for thousands of concurrent industrial streams.

## 5. Security & Privacy Layer

Enforces data protection policies before events leave the edge.

- **PII Masking**: Redacting Emails, Tokens, and specialized Industrial IDs.
- **Local-Only Mode**: Prevents sensitive OT (Operational Technology) data from leaking to the cloud.

## 6. Time-Travel & Traffic Replay

Diagnostic tools for reproducing edge conditions (e.g., simulating a factory-floor sensor failure).

- **`EventStore`**: Chronological storage for O(log N) range queries.
- **`Chaos Replay`**: Inject **Latency**, **Drop %**, and **Signal Mutations** during simulation.

## 7. Real-Time Flow Graph

Visualizes service topology (Cloud-to-Edge) and device-to-device flow.

- **Dynamic Mapping**: Identifies PLCs, ECUs, and Gateways as Nodes.
- **Metric Aggregation**: Tracks throughput and signal health per edge flow.

## 8. Plugin Architecture

A trait-based extension system for custom industrial protocols (e.g., OPC-UA, Profinet).

---

## 13. CLI Interface

Control the agent via the terminal:

- `fluxx start`: Standard activation with industrial capture flags.
- **`fluxx dev`**: Rapid-discovery mode for local sensors and services.
- `fluxx inspect <target>`: Real-time protocol diagnostics.
