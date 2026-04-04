# Fluxx Edge Agent: Dashboard Generation Prompt

Use this prompt with a high-level UI/UX generation AI (e.g., Claude, v0.dev) to build a specialized observability dashboard based on the existing documentation.

---

## UI/UX Generation Prompt

**Objective**: Build a high-performance, dark-mode observability dashboard for the Fluxx Edge Agent.

**Design Aesthetic**:

- **Monochrome-Plus**: High-contrast black/white/gray with vibrant accent colors (Electric Blue for active protocols, Neon Green for delivery success, Safety Orange for latency spikes).
- **Glassmorphism**: Translucent cards for protocol metrics.
- **HUD-style**: Technical, dense, and "alive" with micro-animations.

**Source Documents**:

- Reference the **Feature Architecture** (`DOCS_FEATURES.md`) for data flow and protocol nodes.
- Reference the **CLI Interface** (`DOCS_COMMANDS.md`) for the management console and command palette.

**Required Components**:

1. **Protocol Health Matrix**: A grid of status cards for HTTP, WS, QUIC, gRPC, and eBPF. Show active connection counts, throughput (req/s), and real-time status (Operational/Degraded).
2. **Latency Bucket Visualization**: A reactive bar chart or heat map showing the distribution of events across the "Categorical Bucketization" middleware (1ms, 10ms, 50ms, 500ms, overflow).
3. **Command Palette**: A persistent, integrated terminal or command bar that supports `fluxx start`, `fluxx inspect`, and `fluxx config`.
4. **Live Event Stream**: A high-density log feed displaying structured metadata (ID, Protocol, Latency, Source IP).
5. **Performance Metrics**: Gauges for Worker Pool saturation and Backpressure.
6. **CLI Command Builder Panel**: A specialized interactive configuration suite that visually builds the deployment command.
   - **Capturer Selection**: Toggle which protocols to activate (HTTP, WS, QUIC, gRPC, eBPF).
   - **Processor Middleware**: Drag-and-drop or checklist for Tagging, Latency, and Filtering.
   - **Transporters**: Select destination sinks (JSON Upstream, Stdout, File).
   - **Config Flags**: Input fields for `--config`, log level (`debug`, `info`), and worker limits.
   - **Live CLI Preview**: A syntax-highlighted code block that dynamically updates with the generated command, e.g., `fluxx start --config prod.json --enable-quic --verbose`.
   - **"Copy to Clipboard" & "Deploy" Buttons**: Quick actions to use the generated command.

**Interaction Logic**:

- Clicking a protocol card should trigger a `fluxx inspect <protocol>` view in the terminal.
- Toggling "Start/Stop" in the UI should animate the state transitions for the whole agent daemon.

**Technology Goal**: Generate clean, responsive React/Tailwind components with Framer Motion for transitions.

---
