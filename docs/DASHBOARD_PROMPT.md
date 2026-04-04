# Fluxx: Industrial & Enterprise Dashboard Generation Prompt

Use this high-fidelity UI/UX generation prompt with a React/Tailwind designer (e.g., Claude 3.5 or v0.dev) to build the Fluxx Command Center for **Cloud-Native**, **Automobile**, **Factory Automation**, and **IoT** deployments.

---

## 🏗️ UI/UX Generation Prompt: "The Industrial Edge Control"

**Objective**: Build a high-performance, dark-mode observability dashboard for the Fluxx Edge Agent that visualizes complex network telemetry, industrial protocol signals (MQTT, Modbus, CAN-Bus), and cross-environment service topology.

**Design Philosophy**:

- **Monochrome-Plus**: Deep charcoal/black background (#050505) with vibrant neon accents:
  - `Electric Blue (#3b82f6)`: Active protocols and flow edges.
  - `High-Vis Yellow (#facc15)`: IoT Sensor drift and telemetry warnings.
  - `Neon Green (#22c55e)`: Healthy delivery and successful capture.
  - `Industrial Orange (#f97316)`: Actuator latency and PLC warnings.
  - `Emergency Red (#ef4444)`: Reconnect storms and Critical anomalies.
- **HUD (Heads-Up Display)**: Use monospace fonts (JetBrains Mono), dense data grids, and glassmorphism.
- **Dynamic & Kinetic**: Micro-animations on protocol cards when traffic is captured. Edges in the flow graph should animate to show data flow between PLCs and Gateways.

---

### 1. Dashboard Layout & Components

#### 🗺️ 1. Industrial Flow Graph (Device-to-Device)

- **Visual**: A force-directed or circular graph using D3.js or Cytoscape.
- **Nodes**: Services (API), PLCs (Factory), ECUs (Vehicle), and Sensors (IoT) identified by IP or Device ID.
- **Edges**: Lines connecting nodes, weighted by traffic volume and signal health (Green -> Yellow -> Red).

#### 📉 2. Performance & Signal Matrix

- **Protocol Matrix**: A multi-domain grid:
  - **Standard**: HTTP, WebSocket, gRPC, QUIC.
  - **Industrial**: **MQTT**, **Modbus**, **CAN-Bus**.
- **Specialized Gauges**: Radial gauges for **Sensor Drift Factors** and **Actuator Command Latency**.

#### 🧠 3. Intelligence & Anomaly Feed (Signal Insight & Explain Engine)

- **Component**: A notification list for "Insights" and "Alerts".
- **Specialized Alerts**: "Sensor Drift Detected (IoT)", "PLC Response Breach (Factory)", "ECU Reset Event (Vehicle)".
- **AI Sidebar**: Clicking an "Insight" opens the **Explain Engine** with Summary, Root Cause, and Recommendation.

#### ⛓️ 4. Workspace & Session Explorer

- **Hierarchy Control**: Dropdowns for **Project → Environment → Service (Device)**.
- **Visual**: A vertical Gantt-style or linear timeline.
- **Logic**: Select a `session_id` to see the exact sequence of events (e.g., a Robot's Start-Trip to End-Cycle timeline).

#### 📊 5. Environment & Fleet Comparison Tool

- **Visual**: A 2-column comparison of two environments or two device fleets (Fleet A vs. Fleet B).
- **Metrics**: Latency, Error Rate, and Signal Stability deltas.

#### 🎮 6. CLI Controller & Command Builder

- **Visual**: Interactive CLI Command Builder with toggles for Industrial protocols (MQTT/Modbus/CAN).

#### 🌀 7. Traffic & Signal Replay (Simulation)

- **Visual**: A playback control bar (Play, Pause, Scrub).
- **Logic**: Visually scrub through the historical `EventStore` and simulate a "Simulated Live" replay of industrial sensor streams.

---

### 2. Interaction Logic & Data Flow

- **Real-Time Hydration**: Use a WebSocket or Server-Sent Events (SSE) stream for live telemetry.
- **Signal Filtering**: Use `?service_id=...&project_id=...` to filter the live stream.
- **Security Visibility**: Display a "Security Shield" icon on sanitized industrial IDs that have been processed by the `SecurityProcessor`.

---

**Technology Stack**:

- **Framework**: Next.js 14+ (App Router).
- **Styling**: Tailwind CSS with Radix UI / Shadcn.
- **Visualization**: Framer Motion for transitions, D3.js for the Flow Graph, Recharts for histograms.
- **State Management**: Zustand or React Context for real-time telemetry syncing.

---
