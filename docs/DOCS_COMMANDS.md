# Fluxx Edge Agent: CLI Interface & Controls

The `fluxx` CLI provides a unified interface for starting, managing, and inspecting the agent and its pipeline.

## 1. Agent Runtime Logic (`fluxx start`)
Starts the main event ingestion and processing daemon.
- **Usage**: `fluxx start [--config <PATH>]`
- **Options**:
  - `-c, --config <path>`: Specifies an optional JSON or environment-based configuration file.
- **Workflow**:
  - Initializes logging and structured tracing.
  - Loads or generates default configuration (Agent ID, Host/Port).
  - Spawns concurrent protocol tasks (HTTP, WS, QUIC, gRPC, eBPF).
  - Orthogonally starts the middleware pipeline (Tag, Latency, Filter).

## 2. In-Depth Inspection (`fluxx inspect <TARGET>`)
Used for diagnostic queries on the active agent (future: communicates via IPC/RPC).
- **Usage**: `fluxx inspect <target>`
- **Parameters**:
  - `<target>`: The identifier of the protocol or sink to inspect (e.g. `http`, `ws`, `quic`).
- **Functionality**:
  - Returns current health status, active connection counts, and lifecycle metrics.

## 3. Configuration Management (`fluxx config`)
Displays or validates the current agent configuration state.
- **Usage**: `fluxx config`
- **Output**:
  - Agent Name and Identity.
  - Upstream server endpoint and port.
  - Active log level and observability settings.

## 4. Operational Controls
- **Initialize (Auto)**: Automatically configures the modular capture/transport pipeline based on defaults or provide configuration.
- **Graceful Shutdown**: Responds to `Ctrl+C` (SIGINT) by cleanly detaching eBPF probes and flushing WebSocket/HTTP connections before termination.
- **Structured Output**: Prints machine-readable JSON or human-readable structured logs to stdout during execution.
