# WebSocket Streaming: The Virtual HMI for Real-Time Industrial Observability

In an industrial factory, real-time is everything. You don't want to wait 10 seconds for a dashboard to refresh. You need to see the **Heartbeat** of your **Modbus** and **gRPC** services as they happen. This is where **WebSockets** in Fluxx excels.

---

## 🏭 The "Streaming" Model: "WebSocket Snapshots"

Fluxx's **CaptureChannel** doesn't just store events in the **EventStore**. It has a dedicated **Broadcast Channel**.

1. **Subscribing to a Stream**: A local dashboard or a remote diagnostic tool can subscribe to it.
2. **Real-Time Visualization**: Once a client connects, Fluxx pushes every captured event (after it's been through the **PII Masking** and **Topology Tags** processors) instantly to the WebSocket client.
3. **Low-Latency Updates**: The local HMI (Human-Machine Interface) on the factory floor can use this stream to build a real-time "Network Health" dashboard that updates at 100fps.

---

## 🚀 Why it matters: "The Moving Part"

In an automated warehouse, the **Autonomous Mobile Robots (AMR)** are constantly in motion.

- **With Standard HTTP Polling**: You see a snapshot of the robot's state every 5 seconds.
- **With Fluxx's WebSocket Collector**: Your warehouse's central control center sees a continuous stream of the robot's **Battery Metrics**, **Position registers** (from its Modbus/CAN bus), and **gRPC Navigation** response times.

If the robot's navigation service starts lagging at 200ms, the control center sees it *before* the robot even realizes there's a problem.

---

## 🏭 Industrial Case: The Brewery Assembly Line

A craft brewery uses **Fluxx** to monitor its automated bottling line.

- **The Maintenance Technician** opens the Fluxx-powered dashboard on their tablet.
- **The Dashboard connects to the WebSocket stream** of the bottling line's Fluxx gateway.
- **They see a live scrolling log** of every Modbus coil state (e.g., `Switch-A: ON`, `Switch-B: OFF`).
- **Suddenly, Switch-C starts oscillating.** They see this in real-time, identifying a mechanical fault on the conveyor belt and stopping the line *before* a bottle breaks.

---

## 🔵 Future: "Stream Interactivity"

We're prototyping an "Interactive WebSocket" where you can send commands *back* to the Fluxx agent via the same stream. For example: "Set the sampling rate to 100% for the next 60 seconds" or "Force a refresh of the eBPF capture probes."

**Summary**: Your industrial observability shouldn't be a static historical record. With WebSockets, Fluxx provides a real-time, living heartbeat for your industrial fleet.
