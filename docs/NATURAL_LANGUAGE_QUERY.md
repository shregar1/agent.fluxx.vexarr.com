# Natural Language Query API

Ask questions about your infrastructure in plain English.

## Overview

Fluxx's Natural Language Query API converts questions into database queries, API calls, and analysis - no SQL or Flux required.

## Examples

### Basic Queries

```bash
# "Which services are down?"
curl "http://localhost:8080/query?q=Which%20services%20are%20down%3F"

# "Show me Postgres connections"
curl "http://localhost:8080/query?q=Show%20Postgres%20connections"

# "Redis memory usage last hour"
curl "http://localhost:8080/query?q=Redis%20memory%20usage%20last%20hour"
```

### Response Format

```json
{
  "query": "Which services are down?",
  "interpreted_as": "health_status:unhealthy",
  "results": {
    "type": "health",
    "integrations": [
      {
        "name": "postgres-primary",
        "status": "unhealthy",
        "last_check": "2024-01-15T10:30:00Z",
        "message": "Connection timeout"
      }
    ]
  },
  "confidence": 0.95
}
```

## Query Patterns

| Pattern | Example | Translates To |
|---------|---------|---------------|
| Health status | "Is Postgres healthy?" | Health check for specific integration |
| Metrics | "Show CPU usage" | Metric query with defaults |
| Time ranges | "errors last 5 minutes" | Metrics with time filter |
| Comparisons | "Redis memory vs yesterday" | Time-series comparison |
| Events | "Recent critical alerts" | Event log query with severity filter |

## Supported Time Expressions

- Absolute: "at 3 PM", "on Monday"
- Relative: "5 minutes ago", "last hour", "since yesterday"
- Ranges: "between 9 AM and 5 PM", "last 24 hours"

## Integration with AI Assistants

### Claude Desktop (MCP)

```
@fluxx Which services are unhealthy?
```

### VS Code Extension

Open the command palette (`Cmd/Ctrl+Shift+P`) and run:
```
Fluxx: Ask Question About Infrastructure
```

### Cursor/Copilot

Type in chat:
```
/fluxx show recent errors in production
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Natural Language Query                    │
│                                                              │
│  Input: "Postgres slow queries last hour"                    │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Parser      │→ │  Intent      │→ │  Query       │      │
│  │  (entities)  │  │  Classifier  │  │  Builder     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                              │
│  Output: MetricsRequest {                                    │
│    integration: "postgres",                                  │
│    metrics: ["slow_queries"],                               │
│    time_range: "1h"                                         │
│  }                                                           │
└─────────────────────────────────────────────────────────────┘
```

## Confidence Scoring

Each query returns a confidence score:

- **0.9-1.0**: Exact match, direct translation
- **0.7-0.9**: Good match, may need clarification
- **0.5-0.7**: Ambiguous, multiple interpretations possible
- **<0.5**: Unknown, returned as text search

## Custom Patterns

Add custom patterns in `ai/query_patterns.yaml`:

```yaml
patterns:
  - name: deployment_frequency
    match: 
      - "how often.*deploy"
      - "deployment frequency"
      - "deploy rate"
    query:
      integration: servicenow
      metric: deployments
      aggregation: count_per_day
```

## Privacy

- No data sent to external services
- All parsing done locally
- Query logs stored locally for pattern improvement
