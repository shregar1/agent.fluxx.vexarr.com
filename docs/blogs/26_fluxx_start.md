# fluxx start: The Observability Engine's Engine Room

Running standard production telemetry requires more than just "opening a socket." It requires orchestration, lifecycle management, and a robust data pipeline. This is the role of the **`fluxx start`** command.

---

## 🏭 Operation: Activating the Fleet

When you run `fluxx start`, the agent doesn't just "begin." It follows a strict **Orchestration Sequence**:

1. **Config Ingestion**: The agent loads its `config.json`, identifying its **Agent Identity Hash** and its list of active **Projects**.
2. **Protocol Attachment**: Fluxx initiates its **CaptureChannels**. This involves attaching **eBPF probes** to kernel hooks, opening **QUIC streams**, and initiating **Modbus/TCP** connections.
3. **Pipeline Initialization**: The **Worker Pool** is initialized with the sequence of **Processors** (PII Masking, Topology tagging, Anomaly detection) as defined in your environment policy.
4. **Sink Handshake**: The agent performs a TLS-secured handshake with its upstream **Fluxx Hub** to verify persistence channels.

---

## 🛠️ Usage Patterns

### The Standard Daemon

Run the agent with its default configuration:

```bash
fluxx start
```

### Environment Overrides

Choose a specific environment configuration (e.g., bypassing cloud sinks for local debugging):

```bash
fluxx start --config env/staging-isolated.json
```

---

## 🚀 Performance: The Worker Pool

At the core of `fluxx start` is our **Lock-Free Worker Pool**.

- Unlike single-threaded listeners, Fluxx uses a set of concurrent workers to process every event.
- If a high-volume **Modbus packet storm** occurs, the worker pool dynamically balances the load across CPU cores, ensuring that your **Actuator Latency** isn't affected by the observability overhead.

---

## 🏭 Industrial Case: The Global Assembly Line

A multinational car manufacturer uses **`fluxx start`** on every Gateway node in its body-shop.

- **The command is managed by systemd**, ensuring it restarts instantly on failure.
- **The `--config` flag** points to a unique local identity file for each station.
- **The result**: A consistent, high-fidelity data stream from 500 different robots, all orchestrated by the same engine room.

**Summary**: `fluxx start` is the foundation of your production reliability. It’s what turns your infrastructure into a living, observable system.
