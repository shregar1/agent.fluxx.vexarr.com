# Fluxx Edge Agent: The Commmand Center Blog

The `fluxx` CLI is the heart of your industrial observability pipeline. Whether you're debugging a single sensor or managing a fleet of edge nodes, these commands are your primary interface for control, diagnostics, and high-performance ingestion.

---

## 🚀 The Production Daemon: `fluxx start`

When you're ready to deploy Fluxx on your production edge nodes or within your factory’s shop floor, you need stability, security, and meticulous organization. You need **`fluxx start`**.

### 🔵 The Problem: "Production traffic is messy."

In complex deployments, you're not just watching one service; you're watching **fleets** across Projects, Environments, and Regions. You have massive telemetry volume and security policies demanding **PII Masking** before anything leaves the node.

### 🔵 The Solution: `fluxx start`

Running `fluxx start` activates the full Fluxx **Agent Engine**—the foundation for long-lived, high-reliability observation.

- **Fully Configurable Lifecycle**: Pass in `--config my_env.json` and Fluxx instantly sets up all required protocols (HTTP, gRPC, QUIC, eBPF, Modbus).
- **The High-Fidelity Pipeline**: Every packet is sent through a **WorkerPool**-managed pipeline where processors like **TopologyProcessor** tag data by Project/Env, and **SecurityProcessor** masks sensitive fields in real-time.
- **Persistence & Time-Travel**: Uses the **EventStore** (BTreeMap-indexed historical storage), allowing teams to query specific time ranges for retrospective analysis (e.g., "What was the latency at 2 PM yesterday?").

---

## 🔍 Instant Snapshots: `fluxx inspect <target>`

Sometimes you don't need a live stream or a historically indexed store. You need a fast, architectural "health check" on a specific protocol or device. That's when you run **`fluxx inspect`**.

### 🟠 The Problem: "Is the PLC communicating?"

When a shop floor robot stops or a gRPC connection hangs, the first thing is: "What’s the current state?" You're often digging through `netstat` or `tcpdump`, manually parsing bytes.

### 🟠 The Solution: `fluxx inspect`

Fluxx provides a human-readable summary of a target’s current observational health without starting the full API stack.

- **Protocol Diagnostics**: Target any protocol (`http`, `grpc`, `modbus`, `ebpf`). Interrogate the **CaptureChannel** for recent latency, throughput, and pending anomalies.
- **Low-Overhead Architecture**: Triggers a brief diagnostic window using the **Analyzer** layer to give you a "Current Anomaly Count" instantly.
- **Real-Time Table Formatting**: Returns Last Capture Time, Avg Latency (us), and Primary Alert counts in a dense, easy-to-read table.

---

## ⚙️ Architectural Core: `fluxx config`

The CLI's brain. Use this to verify that your identity, secrets, and upstream targets are perfectly aligned for the environment you're in.

### 🟢 The Goal: "No more configuration drift."

Ensuring that your Agent ID matches your Fleet Management Portal is critical for data integrity. `fluxx config` allows you to see exactly what the agent is thinking.

- **Identity Management**: Shows your Agent Name and unique Identity hash.
- **Upstream Targets**: Verifies the endpoint and port for your centralized Fluxx Hub.
- **Observability Settings**: Inspect active log levels and tracing configurations to ensure your diagnostics aren't flooding your disk or missing critical events.

---

## 🚀 Fleet Deployment: `fluxx deploy`

When you're ready to scale from a single developer sandbox to a massive industrial fleet, you need a high-velocity, automated way to provision your agents. This is where **`fluxx deploy`** comes in.

### 🔵 The Problem: "Manual provisioning is slow."

Manually configuring 1,000 edge nodes with their own API keys, Project IDs, and Secret tokens is a recipe for error and a massive time drain for your IT team. You need **Infrastructure-as-Code** for your observability agent.

### 🔵 The Solution: `fluxx deploy`

The `deploy` command is the bridge between your **Fleet Management Portal** and your raw **Edge Hardware**.

- **Environment Injection**: `fluxx deploy --env production-berlin` automatically retrieves the latest security policies, masking rules, and protocol configurations from your central hub and applies them to the local node.
- **Provider Aggregation**: `fluxx deploy --provider aws` or `--provider azure` enables cloud-native extensions like **AWS Timestream** or **Azure Data Explorer** directly from the CLI.
- **Automated Provisioning**: Generates a one-time provisioning token and uses it to negotiate a long-lived, unique **Agent Identity Hash** from your fleet portal.
- **Lifecycle Setup**: Automatically creates and enables the local `systemd` or `launchd` daemon, ensuring the agent starts on boot and recovers from crashes.
- **Verification Hook**: After deployment, `fluxx deploy` runs a final health check (similar to `inspect`) to verify that the agent is successfully streaming its first batch of telemetry to your central Sink.

### 🛠️ Common Deployment Examples

```bash
# Deploy to a self-hosted local hub
fluxx deploy --provider local --hub-ip 10.0.1.50

# Deploy to a global AWS fleet
fluxx deploy --provider aws --region us-west-2 --env production

# Deploy to an Azure Data Explorer hub
fluxx deploy --provider azure --cluster-uri "https://ingest-fluxx.westus.kusto.windows.net"

# Deploy to GCP with BigTable storage
fluxx deploy --provider gcp --project fluxx-prod --instance industrial-hub
```

---

**Summary**: The `fluxx` CLI isn't just a set of commands; it's the control plane for your entire modular observability architecture. Use `start` for prod, `inspect` for health checks, `config` to keep the engine tuned, and **`deploy`** to scale your industrial fleet with ease.
