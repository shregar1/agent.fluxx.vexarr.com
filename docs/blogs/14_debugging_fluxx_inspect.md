# Debugging with `fluxx inspect`: Real-Time Snapshots for Industrial OT

Industrial observability isn't just about logs and metrics. When a production line stops or a robotic arm enters an "Error" state, the first thing is: "What’s the current health of the system?" You don't want to wait for an hourly summary or a delayed dashboard. That's when you run **`fluxx inspect <target>`**.

---

## 🏭 The "Immediate Health Check": `inspect`

The `fluxx inspect` command is for when you need a high-resolution snapshot of a specific protocol or device's observational health.

1. **Direct Target Inspection**: `inspect` doesn't just return generic health. It interrogates the **CaptureChannel** for a specific protocol (e.g. `http`, `modbus`, `quic`).
2. **Recent Metrics Summary**: In seconds, `inspect` returns a dense table of:
   - **Last Capture Time**: When was the most recent packet captured?
   - **Throughput**: How many events/sec are currently being captured?
   - **Latency Distribution**: What's the current P50/P90 latency?
   - **Primary Alert Counts**: How many `WARNING` or `CRITICAL` anomalies have been triggered in the last 60 seconds?

---

## 🛠️ The Troubleshooting Flow

Imagine a **Modbus-powered conveyor** in a brewery that's experiencing intermittent lag.

- **Step 1**: The maintenance technician connects their laptop to the factory's diagnostic LAN.
- **Step 2**: They run `fluxx inspect modbus` on the gateway and see a sudden increase in **Modbus Retransmissions**.
- **Step 3**: They identify a **Damaged RJ45 Connector** on the conveyor's motor controller.
- **Step 4**: After replacing the cable, they run `inspect` again and see the throughput and latency return to normal.

---

## 🚀 Why it matters: "No Dashboard Required"

On the factory floor, connectivity to the "Cloud" is often unreliable. `fluxx inspect` runs entirely **Local to the Edge Node**. This ensures you have high-resolution diagnostic power even if the factory is offline.

---

## 🏭 Industrial Case: The Self-Driving Forklift

A warehouse uses **Fluxx** to monitor its fleet of autonomous forklifts.

- **When a forklift stalls**, the technician runs `fluxx inspect grpc` to see if the forklift's internal navigation microservice is responding.
- **The `inspect` output** shows that the **P90 latency** has spiked to 500ms, indicating a **CPU Overload** on the forklift's internal ARM computer.
- **The technician** restarts the navigation service and verifies the health with another `inspect` command.

---

## 🔵 Future: "Verbose Inspection"

We're researching a `--verbose` mode for `fluxx inspect`. This would allow you to see the raw, un-processed industrial payloads for the last 10 packets, providing a deep-dive "Protocol Trace" directly in the CLI.

**Summary**: `fluxx inspect` is the "health check" for your industrial OT. It's the high-speed, local diagnostic tool that ensures your maintenance team isn't flying blind.
