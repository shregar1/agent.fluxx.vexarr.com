# Fluxx API Reference

Complete reference for the Fluxx REST API and WebSocket streaming.

## Base URL

```
http://localhost:8080
```

## Authentication

All API requests require an API key header:

```bash
curl -H "X-API-Key: your-api-key" http://localhost:8080/health
```

Or set the environment variable:
```bash
export FLUXX_API_KEY="your-api-key"
```

## Response Format

All responses follow a standard format:

```json
{
  "success": true,
  "data": { ... },
  "message": "Optional message",
  "meta": {
    "timestamp": "2026-04-05T10:30:00Z",
    "request_id": "req-abc123"
  }
}
```

Error responses:

```json
{
  "success": false,
  "error": {
    "code": "NOT_FOUND",
    "message": "Resource not found",
    "details": ["Additional context"]
  },
  "meta": {
    "timestamp": "2026-04-05T10:30:00Z",
    "request_id": "req-abc123"
  }
}
```

---

## Core Endpoints

### Health Check

Check if the agent is running and healthy.

```http
GET /health
```

**Response:**

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "0.1.0",
    "uptime_seconds": 86400,
    "integrations": {
      "total": 45,
      "healthy": 43,
      "degraded": 2
    }
  }
}
```

**Status Codes:**
- `200` - Healthy
- `503` - Degraded or unhealthy

---

### Get Configuration

Retrieve current agent configuration.

```http
GET /config
```

**Response:**

```json
{
  "success": true,
  "data": {
    "node_id": "node-1",
    "bind": "0.0.0.0:8080",
    "integrations": [
      {"name": "postgres", "type": "database", "status": "healthy"},
      {"name": "redis", "type": "cache", "status": "healthy"}
    ],
    "mesh": {
      "enabled": true,
      "nodes": 3,
      "peers": ["node-2:7946", "node-3:7946"]
    }
  }
}
```

---

## Metrics Endpoints

### List All Metrics

Get all collected metrics from all integrations.

```http
GET /metrics
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `integration` | string | Filter by integration name |
| `name` | string | Filter by metric name pattern |
| `since` | string | Metrics since timestamp (ISO 8601) |
| `limit` | integer | Maximum metrics to return (default: 1000) |

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  "http://localhost:8080/metrics?integration=postgres&since=2026-04-05T00:00:00Z"
```

**Response:**

```json
{
  "success": true,
  "data": {
    "metrics": [
      {
        "name": "postgres.connections.active",
        "value": 42.0,
        "type": "gauge",
        "timestamp": "2026-04-05T10:30:00Z",
        "source": "postgres",
        "tags": {
          "database": "production"
        }
      },
      {
        "name": "postgres.transactions.commits",
        "value": 15234.0,
        "type": "counter",
        "timestamp": "2026-04-05T10:30:00Z",
        "source": "postgres",
        "tags": {}
      }
    ],
    "meta": {
      "total": 2,
      "returned": 2
    }
  }
}
```

---

### Get Metrics by Integration

Get metrics from a specific integration.

```http
GET /metrics/:integration
```

**Path Parameters:**

| Parameter | Description |
|-----------|-------------|
| `integration` | Integration name (e.g., "postgres", "redis") |

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  http://localhost:8080/metrics/postgres
```

---

### Query Metrics (PromQL-style)

Query metrics using PromQL-style syntax.

```http
GET /query
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `q` | string | Query expression (required) |
| `time` | string | Evaluation timestamp |
| `timeout` | string | Query timeout |

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  "http://localhost:8080/query?q=rate(postgres_transactions_commits[5m])"
```

**Response:**

```json
{
  "success": true,
  "data": {
    "resultType": "vector",
    "result": [
      {
        "metric": {
          "__name__": "rate(postgres_transactions_commits[5m])",
          "source": "postgres"
        },
        "value": [
          1712310600,
          "152.34"
        ]
      }
    ]
  }
}
```

---

## Events Endpoints

### List Events

Get recent infrastructure events and alerts.

```http
GET /events
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `severity` | string | Filter by severity: `info`, `warning`, `error`, `critical` |
| `source` | string | Filter by source integration |
| `since` | string | Events since timestamp |
| `until` | string | Events until timestamp |
| `limit` | integer | Maximum events (default: 100, max: 10000) |

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  "http://localhost:8080/events?severity=critical&limit=10"
```

**Response:**

```json
{
  "success": true,
  "data": {
    "events": [
      {
        "id": "evt-abc123",
        "title": "PostgreSQL Connection Pool Exhausted",
        "description": "All 100 connections in use",
        "severity": "critical",
        "source": "postgres",
        "timestamp": "2026-04-05T10:29:30Z",
        "metadata": {
          "connections_total": 100,
          "connections_active": 100
        }
      }
    ],
    "meta": {
      "total": 1,
      "returned": 1
    }
  }
}
```

---

### Get Event by ID

Get details of a specific event.

```http
GET /events/:id
```

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  http://localhost:8080/events/evt-abc123
```

---

## Sessions Endpoints

### List Sessions

Get all active capture sessions.

```http
GET /sessions
```

**Response:**

```json
{
  "success": true,
  "data": {
    "sessions": [
      {
        "id": "sess-xyz789",
        "name": "Production Traffic",
        "status": "active",
        "started_at": "2026-04-05T09:00:00Z",
        "event_count": 15234,
        "filters": {
          "protocol": ["http", "https"],
          "port": [80, 443]
        }
      }
    ]
  }
}
```

---

### Get Session

Get details of a specific session.

```http
GET /sessions/:id
```

---

### Create Session

Start a new capture session.

```http
POST /sessions
```

**Request Body:**

```json
{
  "name": "API Traffic Analysis",
  "filters": {
    "protocol": ["http", "https"],
    "port": [8080],
    "path_prefix": "/api"
  },
  "sampling": {
    "rate": 1.0,
    "strategy": "random"
  },
  "max_duration": "1h",
  "max_events": 100000
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "session": {
      "id": "sess-new123",
      "name": "API Traffic Analysis",
      "status": "active",
      "started_at": "2026-04-05T10:30:00Z"
    }
  },
  "message": "Session created successfully"
}
```

---

### Stop Session

Stop an active session.

```http
DELETE /sessions/:id
```

---

## Integrations Endpoints

### List Integrations

Get all configured integrations.

```http
GET /integrations
```

**Response:**

```json
{
  "success": true,
  "data": {
    "integrations": [
      {
        "name": "postgres",
        "type": "database",
        "status": "healthy",
        "last_check": "2026-04-05T10:29:00Z",
        "metrics_count": 15
      },
      {
        "name": "redis",
        "type": "cache",
        "status": "healthy",
        "last_check": "2026-04-05T10:29:00Z",
        "metrics_count": 8
      }
    ]
  }
}
```

---

### Get Integration Health

Check health status of all integrations.

```http
GET /integrations/health
```

**Response:**

```json
{
  "success": true,
  "data": {
    "overall": "healthy",
    "integrations": {
      "postgres": {
        "status": "healthy",
        "last_check": "2026-04-05T10:30:00Z",
        "latency_ms": 5
      },
      "redis": {
        "status": "healthy",
        "last_check": "2026-04-05T10:30:00Z",
        "latency_ms": 2
      }
    }
  }
}
```

---

## AI & Diagnostics Endpoints

### Natural Language Query

Query your infrastructure using natural language.

```http
GET /query/nlp?q=:question
```

**Example:**

```bash
curl -H "X-API-Key: $FLUXX_API_KEY" \
  "http://localhost:8080/query/nlp?q=Why+is+Postgres+slow"
```

**Response:**

```json
{
  "success": true,
  "data": {
    "question": "Why is Postgres slow?",
    "interpreted_as": "diagnose:postgresql",
    "answer": "PostgreSQL is experiencing slow queries due to missing indexes on the 'orders' table. Recommended action: CREATE INDEX CONCURRENTLY idx_orders_created_at ON orders(created_at);",
    "confidence": 0.87,
    "sources": [
      {
        "type": "metric",
        "name": "postgres.query.latency",
        "value": 2345
      },
      {
        "type": "log",
        "pattern": "slow query",
        "count": 15
      }
    ]
  }
}
```

---

### Diagnose Integration

Run AI diagnostics on a specific service.

```http
POST /diagnose/:integration
```

**Request Body:**

```json
{
  "symptom": "High latency on API requests",
  "context": {
    "duration": "5m",
    "affected_endpoints": ["/api/v1/orders"]
  }
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "diagnosis": {
      "root_cause": "Connection pool exhaustion",
      "confidence": 0.92,
      "evidence": [
        "Active connections: 100/100",
        "Connection wait time: 2.3s",
        "No idle connections available"
      ],
      "suggested_fixes": [
        {
          "description": "Increase connection pool size",
          "command": "ALTER SYSTEM SET max_connections = 200;",
          "risk": "low"
        }
      ],
      "prevention": [
        "Monitor connection pool usage",
        "Set alert at 80% capacity"
      ]
    }
  }
}
```

---

## WebSocket Streaming

Real-time event streaming via WebSocket.

### Connect

```javascript
const ws = new WebSocket('ws://localhost:8080/stream');

ws.onopen = () => {
  // Subscribe to specific event types
  ws.send(JSON.stringify({
    action: 'subscribe',
    filters: {
      severity: ['error', 'critical'],
      source: ['postgres', 'redis']
    }
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('New event:', data);
};
```

### Filter Options

```json
{
  "project_id": "my-project",
  "environment_id": "production",
  "service_id": "api-gateway",
  "severity": ["error", "critical"],
  "source": ["postgres"],
  "event_type": ["database_slow_query"]
}
```

---

## Mesh Endpoints

### Get Mesh Status

Get status of the distributed mesh.

```http
GET /mesh/status
```

**Response:**

```json
{
  "success": true,
  "data": {
    "node_id": "node-1",
    "status": "active",
    "peers": [
      {
        "id": "node-2",
        "address": "10.0.1.2:7946",
        "status": "healthy",
        "latency_ms": 0.8
      },
      {
        "id": "node-3",
        "address": "10.0.1.3:7946",
        "status": "healthy",
        "latency_ms": 1.2
      }
    ],
    "metadata": {
      "nodes_total": 3,
      "nodes_healthy": 3,
      "data_replicated": "3/3"
    }
  }
}
```

---

### Federated Query

Query across all nodes in the mesh.

```http
POST /mesh/query
```

**Request Body:**

```json
{
  "query": "SELECT * FROM events WHERE severity = 'error'",
  "timeout": "30s",
  "consistency": "quorum"
}
```

**Response:**

```json
{
  "success": true,
  "data": {
    "results": [
      // Aggregated results from all nodes
    ],
    "metadata": {
      "nodes_queried": 3,
      "nodes_responded": 3,
      "execution_time_ms": 145
    }
  }
}
```

---

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Invalid or missing API key |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `BAD_REQUEST` | 400 | Invalid request parameters |
| `TIMEOUT` | 504 | Query timeout |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Rate Limiting

Default rate limits:

- **Read operations:** 1000 requests/minute
- **Write operations:** 100 requests/minute
- **WebSocket:** 1000 messages/minute

Rate limit headers:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1712311200
```

---

## SDK Examples

### Python

```python
import requests

class FluxxClient:
    def __init__(self, base_url, api_key):
        self.base_url = base_url
        self.headers = {"X-API-Key": api_key}
    
    def get_metrics(self, integration=None):
        params = {"integration": integration} if integration else {}
        return requests.get(
            f"{self.base_url}/metrics",
            headers=self.headers,
            params=params
        ).json()
    
    def query_nlp(self, question):
        return requests.get(
            f"{self.base_url}/query/nlp",
            headers=self.headers,
            params={"q": question}
        ).json()

# Usage
client = FluxxClient("http://localhost:8080", "your-api-key")
metrics = client.get_metrics("postgres")
answer = client.query_nlp("Why is the database slow?")
```

### JavaScript/TypeScript

```typescript
class FluxxClient {
  constructor(private baseUrl: string, private apiKey: string) {}

  async getMetrics(integration?: string): Promise<MetricsResponse> {
    const params = integration ? `?integration=${integration}` : '';
    const response = await fetch(`${this.baseUrl}/metrics${params}`, {
      headers: { 'X-API-Key': this.apiKey }
    });
    return response.json();
  }

  async *streamEvents(filters?: EventFilters): AsyncGenerator<Event> {
    const ws = new WebSocket(`${this.baseUrl.replace('http', 'ws')}/stream`);
    
    ws.onopen = () => {
      ws.send(JSON.stringify({ action: 'subscribe', filters }));
    };

    while (true) {
      const event = await new Promise((resolve) => {
        ws.onmessage = (e) => resolve(JSON.parse(e.data));
      });
      yield event;
    }
  }
}
```

---

## OpenAPI Spec

Download the complete OpenAPI specification:

```bash
curl http://localhost:8080/openapi.json
```

Import into:
- Postman
- Swagger UI
- Insomnia

---

For more examples, see the [SDK documentation](https://fluxx.vexarr.com/docs/sdks).
