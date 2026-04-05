# AI Assistant Integrations

Fluxx Agent integrates with popular AI coding assistants to provide infrastructure observability directly in your development environment.

## Supported Assistants

| Assistant | Integration | Status |
|-----------|-------------|--------|
| Claude Desktop | MCP Server | ✅ Ready |
| VS Code | Extension | ✅ Ready |
| Cursor | VS Code API | ✅ Compatible |
| GitHub Copilot | VS Code API | ✅ Compatible |
| Continue.dev | VS Code API | ✅ Compatible |

## MCP Server (Claude Desktop)

The Model Context Protocol (MCP) server exposes Fluxx capabilities as callable tools for Claude Desktop.

### Setup

1. Install Claude Desktop
2. Add to Claude config (`~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "fluxx-agent": {
      "command": "fluxx-agent",
      "args": ["mcp"],
      "env": {
        "FLUXX_CONFIG": "/path/to/fluxx.yaml"
      }
    }
  }
}
```

### Available Tools

| Tool | Description |
|------|-------------|
| `fluxx_query_metrics` | Query infrastructure metrics |
| `fluxx_health_check` | Check service health status |
| `fluxx_get_events` | Get recent events and alerts |
| `fluxx_list_integrations` | List all monitored integrations |
| `fluxx_diagnose` | Run diagnostics on services |

### Example Queries

```
@fluxx Are all my services healthy?
@fluxx Show me Postgres connection metrics
@fluxx What errors happened in the last hour?
@fluxx Diagnose the Redis memory issue
```

## VS Code Extension

The Fluxx VS Code extension provides infrastructure observability in your editor.

### Features

- **Status Bar**: Real-time health indicator
- **Integrations Panel**: Browse all integrations in the sidebar
- **Commands**: 
  - `Fluxx: Show Infrastructure Overview`
  - `Fluxx: Run Health Check`
  - `Fluxx: Refresh`

### Installation

```bash
cd extensions/vscode
npm install
npm run compile
```

Then in VS Code:
1. Open the folder in VS Code
2. Press F5 to launch Extension Development Host

### Configuration

```json
{
  "fluxx.apiUrl": "http://localhost:8080",
  "fluxx.showStatusBar": true,
  "fluxx.refreshInterval": 30
}
```

## Natural Language Queries

Ask questions about your infrastructure in plain English.

### Examples

```bash
# Query via API
curl "http://localhost:8080/query?q=Is%20Postgres%20healthy%3F"

# Response
{
  "query": "Is Postgres healthy?",
  "interpreted_as": "health_check:postgresql",
  "answer": "✅ **postgresql**: Healthy",
  "confidence": 0.95
}
```

### Supported Patterns

| Pattern | Example |
|---------|---------|
| Health checks | "Is Redis up?" |
| Metrics | "Show me CPU usage" |
| Events | "Recent errors" |
| Diagnostics | "Why is Postgres slow?" |
| Listings | "What integrations are configured?" |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AI Assistant Layer                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ MCP Server   │  │ VS Code Ext  │  │ NLP Query    │      │
│  │ (Claude)     │  │ (VS Code)    │  │ (API)        │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
┌─────────────────────────────────────────────────────────────┐
│                      AI Coordinator                          │
│         (Unified interface for all AI assistants)            │
└─────────────────────────────────────────────────────────────┘
                            │
┌─────────────────────────────────────────────────────────────┐
│                  Fluxx Agent Core                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Integration  │  │ Diagnostic   │  │ Event        │      │
│  │ Manager      │  │ Engine       │  │ Store        │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## Security

- AI integrations have read-only access by default
- No sensitive credentials exposed to AI assistants
- All queries go through the same RBAC as the main API
- Local-only processing for NLP queries

## Development

### Adding New AI Tools

1. Implement the `Tool` trait in `src/ai/mcp_server.rs`:

```rust
#[async_trait]
impl Tool for MyNewTool {
    fn name(&self) -> &str { "fluxx_my_tool" }
    fn description(&self) -> &str { "What this tool does" }
    fn schema(&self) -> Value { /* JSON Schema */ }
    async fn execute(&self, args: Value) -> Result<Value, ToolError> { /* ... */ }
}
```

2. Register in `FluxxMcpServer::get_tools()`

### Testing

```bash
# Run AI module tests
cargo test ai::

# Test MCP server
cargo test mcp_server

# Test NLP queries  
cargo test nlp_query
```

## Troubleshooting

### MCP Server Not Connecting

1. Check Claude Desktop logs: `~/Library/Logs/Claude/mcp*.log`
2. Verify fluxx-agent is in PATH: `which fluxx-agent`
3. Test MCP manually: `fluxx mcp`

### VS Code Extension Not Working

1. Check Fluxx Agent is running: `curl http://localhost:8080/health`
2. Verify API URL in settings
3. Check VS Code output panel for errors

### NLP Query Returns Low Confidence

1. Check the integration name is recognized
2. Use more specific terminology
3. Check supported patterns in `src/ai/nlp_query.rs`
