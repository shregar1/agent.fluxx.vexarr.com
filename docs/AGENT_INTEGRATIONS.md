# AI Agent Integrations: MCP & OpenClaw

Fluxx Agent is now a first-class citizen in the world of **Agentic Observability**. By implementing the **Model Context Protocol (MCP)** and native **OpenClaw** support, Fluxx allows autonomous AI agents to monitor, diagnose, and repair your infrastructure.

## 🤝 Model Context Protocol (MCP)

MCP is the open standard for connecting AI assistants (like Claude Desktop, Cursor, and Roo Code) to local tools and data.

### How to Start

Run the following command to start the MCP server on stdio:

```bash
fluxx mcp
```

### Claude Desktop Configuration

Add this to your `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "fluxx-agent": {
      "command": "fluxx",
      "args": ["mcp"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Supported Tools

- **`fluxx_query_metrics`**: Query performance data (CPU, Memory, DB latency).
- **`fluxx_health_check`**: Check the status of all configured integrations.
- **`fluxx_get_events`**: Retrieve the latest error and warning events.
- **`fluxx_diagnose`**: Run AI diagnostics on a specific service (e.g., "Postgresql OOM").

---

## 🐙 OpenClaw Integration

OpenClaw is an autonomous agent that runs locally and uses **Skills** and **Heartbeats** to perform proactive tasks.

### Quick Start

Initialize an OpenClaw workspace with Fluxx observability:

```bash
fluxx openclaw init --path ~/.openclaw
```

### What this Command Does

1. **Adds the Fluxx Skill**: Creates `skills/fluxx_observability/SKILL.md`. This "playbook" teaches OpenClaw how to use the Fluxx CLI for investigations.
2. **Sets up Heartbeat**: Creates `HEARTBEAT.md` which schedules a routine check of your system health every 30 minutes.

### Verification

Check the status of your integration:

```bash
fluxx openclaw status
```

---

## 🚀 Proactive Monitoring Workflow

Fluxx Agent and OpenClaw together enable a **Self-Healing Infrastructure**:

1. **Detection**: Fluxx Agent detects a high latency event in your Redis cluster.
2. **Trigger**: OpenClaw's heartbeat wakes it up, or an alert script notifies the agent.
3. **Investigation**: OpenClaw uses the `fluxx_diagnose` tool via the MCP bridge to find the root cause.
4. **Reporting/Fixing**: OpenClaw posts a summary to your Slack/Discord channel or attempts a pre-approved fix (like flushing a specific cache).
