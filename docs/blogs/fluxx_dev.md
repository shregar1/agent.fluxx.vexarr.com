# The Developer's Superpower: Deep-Dive into `fluxx dev`

Local development often feels like flying blind. You have multiple microservices, a web-client, and maybe an IoT gateway running locally, but when a request fails or latency spikes, you're stuck digging through fragmented logs.

Enter **`fluxx dev`**—the zero-config, high-performance command that transforms your terminal into a live network HUD.

---

## 🟢 The Problem: "Why is it slow on my machine?"

Debugging local cross-service communication is tedious. You're constantly:

- Manually checking if `localhost:8080` is up.
- Trying to correlate logs between your Auth service and the DB.
- Seeing strange WebSocket disconnects without any root-cause info.

## 🟢 The Solution: `fluxx dev`

When you run `fluxx dev`, Fluxx stops being a background daemon and becomes your local diagnostic partner. Here’s exactly what happens under the hood:

1. **Auto-Discovery (Port Scan & Detect)**
   Fluxx automatically scans your local stack. It identifies standard ports (`8080`, `50051`, `3000`) and starts listening. **Zero JSON configuration required.**
2. **Immediate Capture**
   It activates the capture layer for HTTP, gRPC, and WebSockets. As soon as your frontend hits your API, Fluxx sees it.
3. **Live HUD Streaming**
   Unlike standard logging, `fluxx dev` streams high-fidelity events directly to your terminal. You don't just see a log line; you see **Inter-Service Latency**, **Protocol Type**, and **Anomaly Insights** (like "Potential Reconnect Storm Detected").

## 🟢 Deep-Dive: The Real-Time Flow

Under the hood, `fluxx dev` initiates a special **DevManager**. This manager bypasses the persistent storage requirements and hooks directly into the **CaptureChannel** to provide the lowest possible latency for terminal display. It leverages our **TopologyProcessor** to automatically tag your local service as `fluxx-local-dev`.

## 🟢 Pro-Tips for Power Users

- **Combine with `--inspect`**: Use `fluxx dev` to see the live flow, and in another tab, use `fluxx inspect` to get a snapshot of a specific protocol's health.
- **Hierarchical Logs**: Even in dev mode, Fluxx organizes your logs by **Service ID**, so you can filter out the noise and focus only on the sensor stream or the API you're currently building.

---

**Summary**: `fluxx dev` is about speed. It’s the command you run the moment you start your local environment to get instant, deep-visibility into the architecture you're building.
