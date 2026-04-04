# Fluxx: Comprehensive Frontend Generation Prompt

Use this prompt with a high-fidelity UI/UX generation AI (e.g., Claude 3.5, v0.dev, or specialized React/Tailwind designers) to build the complete Fluxx observability dashboard.

---

## 🏗️ UI/UX Generation Prompt: "The Fluxx Command Center"

**Objective**: Build a high-performance, dark-mode observability dashboard for the Fluxx Edge Agent that visualizes complex network telemetry, protocol anomalies, and service topology.

**Design Philosophy**:

- **Monochrome-Plus**: Deep charcoal/black background (#050505) with vibrant neon accents:
  - `Electric Blue (#3b82f6)`: Active protocols and flow edges.
  - `Neon Green (#22c55e)`: Healthy delivery and successful capture.
  - `Safety Orange (#f97316)`: Latency spikes and Warnings.
  - `Emergency Red (#ef4444)`: Reconnect storms and Critical anomalies.
- **HUD (Heads-Up Display)**: Use monospace fonts (JetBrains Mono, Roboto Mono), dense data grids, and glassmorphism.
- **Dynamic & Kinetic**: Micro-animations on protocol cards when traffic is captured. Edges in the flow graph should animate in the direction of the request flow.

---

### 1. Dashboard Layout & Components

#### 🗺️ 1. Service Flow Graph (Topology Map)

- **Visual**: A force-directed or circular graph using D3.js or Cytoscape.
- **Nodes**: Services/Endpoints identified by IP (e.g., `10.0.0.1`, `api.github.com`).
- **Edges**: Lines connecting nodes, weighted by traffic volume and color-coded by average latency (Green -> Orange -> Red).
- **Interaction**: Clicking a node filters the global view to that service's logs and metrics.

#### 📈 2. Latency & Performance Matrix

- **Categorical Histograms**: Horizontal bar charts showing the distribution of events in `Latency Buckets` (1ms, 5ms, 50ms, 500ms, >1s).
- **Protocol Matrix**: A 2x3 grid of status cards for `HTTP`, `WebSocket`, `QUIC`, `gRPC`, `eBPF`, and `TCP/UDP`. Show real-time throughput (req/s) and sub-millisecond latencies.

#### 🧠 3. Intelligence & Anomaly Feed (Insight Manager)

- **Component**: A notification list or activity stream displaying "Insights".
- **Items**: "Reconnect Storm detected (WS)", "High Latency Endpoint (HTTP)", "Kernel Volume Surge (eBPF)".
- **Visual**: High-contrast labels with severity indicators and timestamps.

#### ⛓️ 4. Session Timeline Explorer

- **Visual**: A vertical Gantt-style or linear timeline.
- **Logic**: Select a `session_id` to see the exact sequence of events (HTTP handshakes followed by WS messages) with their interleaved latency gaps.

#### 🎮 5. CLI Controller & Command Builder

- **Visual**: A split-screen terminal. Left side: Interactive CLI Command Builder (toggles for protocols, checkboxes for processors like Tagging/Masking). Right side: Visual command block (`fluxx start --config ...`).
- **Functionality**: "Copy to Clipboard" and "Direct Deploy" actions.

#### 🌀 6. Traffic Replay & Chaos Simulator

- **Visual**: A playback control bar (Play, Pause, Scrub).
- **Sliders**: `Delay Mutation (ms)`, `Drop Rate (%)`, `Payload Redaction (Toggle)`.
- **Logic**: Visually scrub through the historical `EventStore` and simulate a "Simulated Live" replay into the dashboard metrics.

---

### 2. Interaction Logic & Data Flow

- **Real-Time Hydration**: Use a WebSocket or Server-Sent Events (SSE) stream to push `Event` and `Insight` JSON objects from the Fluxx agent.
- **Global Filter**: Selecting a protocol (e.g., `QUIC`) in the sidebar should globally highlight all related nodes in the Flow Graph and filter the Event Stream.
- **Security Visibility**: Display a "Security Shield" icon on sanitized payloads that have been processed by the `SecurityProcessor`.

---

**Technology Stack**:

- **Framework**: Next.js 14+ (App Router).
- **Styling**: Tailwind CSS with Radix UI / Shadcn.
- **Visualization**: Framer Motion for transitions, D3.js for the Flow Graph, Recharts for histograms.
- **State Management**: Zustand or React Context for real-time telemetry syncing.

---
