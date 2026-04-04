# Instant Snapshots: The Power of `fluxx inspect`

Sometimes you don't need a live stream or a historically indexed store. You need a fast, architectural "health check" on a specific protocol or device. That's when you run **`fluxx inspect <target>`**.

---

## 🟠 The Problem: "Is the PLC communicating?"

When a shop floor robot stops or a gRPC connection hangs, the first thing a developer asks is: "What’s the current state?" You're often digging through system-level CLI tools (`netstat`, `tcpdump`) and manually parsing bytes.

## 🟠 The Solution: `fluxx inspect`

Fluxx provides a human-readable summary of a target’s current observational health with **`fluxx inspect`**.

1. **Protocol Diagnostics**
   Target any capture protocol (`http`, `grpc`, `modbus`, `ebpf`). Fluxx will interrogate the **CaptureChannel** and return a snapshot of recent latency, throughput, and any pending anomalies.
2. **Low-Overhead "Architecture Check"**
   Unlike full-agent activation, `inspect` only triggers a brief diagnostic window. It leverages the **Analyzer** layer to give you a "Current Anomaly Count" without starting the whole API or persistence stack.
3. **Real-Time Result Formatting**
   The CLI returns a dense, easy-to-read table. It includes the **Last Capture Time**, **Avg Latency (us)**, and **Primary Alert counts**.

## 🟠 Use Cases

- **Factory Automation**: Run `fluxx inspect modbus` to verify if the PLC-Actuator latency is within the 100ms threshold for the current shift.
- **Microservices**: Run `fluxx inspect http` to see if the recent response distribution has shifted towards the 500ms bucket.

## 🟠 Pro-Tips for Power Users

- **Chain with `grep`**: Since `fluxx inspect` returns a text summary, you can easily pipe it into shell scripts for automated monitoring tasks.
- **Combine with `--verbose`**: Get deeper protocol-level handshake metadata (e.g. WebSocket sub-protocol negotiation) in the summary.

---

**Summary**: `fluxx inspect` is the "health check" command. It’s for those moments when you need a snapshot of architectural reality without the overhead of a full observability session.
