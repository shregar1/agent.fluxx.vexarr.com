# Observability Sinks: Offloading Edge Data at Scale

Your industrial edge nodes are powerful, but they aren't data centers. They have limited storage and processing power. To build a global view of your industrial fleet, you need a high-performance way to move your **Events** and **Anomalies** from the edge to a central hub. This is where **Sinks** come in.

---

## 🏭 The Problem: "The Data Tsunami."

If you have 1,000 edge nodes each producing 100kb/sec of raw observability data, you're sending **100mb/sec** (800mbps) of traffic to your central hub. A simple HTTP POST for every event just won't cut it. Your network will choke, and your edge CPU will spike just from the overhead of thousands of short-lived connections.

---

## 🔵 The Fluxx Solution: "Aggregated Sinks"

The **SinkProcessor** is the final step in the Fluxx pipeline. It doesn't just send one event at a time; it manages a **Buffer**.

1. **Batch Ingestion**: The SinkProcessor collects events into an internal buffer. Every 1 second (or when the buffer hits 1,000 events), it sends a single, compressed **Batch Request**.
2. **Protocol-Aware Transport**: Fluxx Sinks support multiple protocols:
   - **HTTP/2 Multiplexing**: Send thousands of events over a single TCP connection.
   - **QUIC Streaming**: For high-vibration, high-jitter wireless networks.
   - **gRPC/Protobuf**: For binary-encoded efficiency and strict schema validation.
3. **Backpressure Management**: If the central hub is slow or the network is down, the SinkProcessor can automatically slow down the **Capture Channel**, ensuring that the agent's memory isn't overwhelmed.

---

## 🚀 Industrial Intelligence: "Contextual Sinks"

The SinkProcessor is **Context-Aware**.

- **CRITICAL Anomalies** are sent instantly via a high-priority QUIC stream.
- **INFO Telemetry** is batched and sent via a lower-priority HTTP/2 connection.
- **High-Volume Raw Logs** can be filtered out entirely at the edge, only sending a summary (like P99 latency) to the central hub.

---

## 🏭 Real-World Case: The High-Speed Bottling Line

A bottling factory in Munich has 50 Fluxx agents monitoring its industrial PLC network.

- **Each agent** uses a **QUIC Sink** to stream real-time latency anomalies to a factory-local dashboard.
- **The same agent** uses a batched **HTTP Sink** to send summarized hourly metrics to the company's global **Fleet Hub** in London.

This dual-sink strategy ensures the local maintenance team has real-time visibility, while the global operations team has a high-level view without the data-ingress costs of raw telemetry.

---

## 🔵 Future: Data Partitioning

We're researching a "Partitioned Sink" where the Fluxx agent can automatically route different types of industrial data to different backends. For example: **Modbus registers** go to a specialized Industrial Time-Series DB, while **HTTP logs** go to an ElasticSearch cluster.

**Summary**: Your edge nodes shouldn't be islands. Fluxx Sinks are the high-performance bridges that connect your industrial fleet to a global observability platform.
