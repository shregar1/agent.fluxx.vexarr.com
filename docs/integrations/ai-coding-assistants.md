# Integrating Fluxx Agent with AI Coding Assistants

This guide explains how to integrate Fluxx Agent with AI coding assistants (Claude, GitHub Copilot, Sourcegraph Cody, Cursor, Continue.dev, Aider, etc.) to create an intelligent observability experience.

## Overview

AI coding assistants can leverage Fluxx Agent to:
- **Query infrastructure state** using natural language
- **Generate monitoring configurations** automatically
- **Debug issues** with real-time diagnostics
- **Create custom integrations** based on requirements
- **Analyze performance** and suggest optimizations

## Integration Patterns

### 1. Context Provider (MCP - Model Context Protocol)

Allow AI assistants to query your infrastructure in real-time.

```typescript
// mcp-server.ts - Model Context Protocol server for Fluxx Agent
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';

const server = new Server({
  name: 'fluxx-agent',
  version: '1.0.0',
}, {
  capabilities: {
    resources: {},
    tools: {},
  },
});

// Define available tools
server.setToolRequestHandlers({
  // Query metrics
  'fluxx/query_metrics': async (args) => {
    const response = await fetch(`http://localhost:8080/api/v1/metrics?q=${args.query}`);
    return response.json();
  },
  
  // Get health status
  'fluxx/health_check': async (args) => {
    const response = await fetch('http://localhost:8080/api/v1/health');
    return response.json();
  },
  
  // Run diagnostics
  'fluxx/diagnose': async (args) => {
    const response = await fetch('http://localhost:8080/api/v1/diagnose', {
      method: 'POST',
      body: JSON.stringify({ error: args.error }),
    });
    return response.json();
  },
  
  // Get events
  'fluxx/get_events': async (args) => {
    const response = await fetch(`http://localhost:8080/api/v1/events?severity=${args.severity || 'all'}`);
    return response.json();
  },
});

const transport = new StdioServerTransport();
await server.connect(transport);
```

**Claude Desktop Configuration:**

```json
// claude_desktop_config.json
{
  "mcpServers": {
    "fluxx-agent": {
      "command": "node",
      "args": ["/path/to/fluxx-mcp-server.js"],
      "env": {
        "FLUXX_API_URL": "http://localhost:8080"
      }
    }
  }
}
```

Now Claude can answer questions like:
- "Why is my PostgreSQL database showing high connection counts?"
- "Are there any critical events in my Kubernetes cluster?"
- "What's the health status of all my AWS resources?"

### 2. Natural Language to PromQL/FluxQL

Convert natural language queries to monitoring queries.

```rust
// src/ai/nlp_query.rs
use fluxx_agent::integrations::IntegrationManager;

pub struct NlpQueryEngine {
    manager: Arc<IntegrationManager>,
    llm_client: LlmClient,
}

impl NlpQueryEngine {
    /// Convert natural language to metric query
    pub async fn natural_language_to_query(&self, question: &str) -> Result<MetricQuery, Error> {
        let prompt = format!(
            r#"Convert this question to a Fluxx Agent metric query.
            
Available integrations: {}
Available metrics: {}

Question: "{}"

Respond with JSON:
{{
  "integration": "integration_name",
  "metric_pattern": "metric.name.pattern",
  "filters": {{"tag": "value"}},
  "aggregation": "sum|avg|max|min|count",
  "time_range": "1h|1d|1w"
}}"#,
            self.list_integrations(),
            self.list_metrics(),
            question
        );

        let response = self.llm_client.complete(&prompt).await?;
        serde_json::from_str(&response)
    }

    /// Execute the query
    pub async fn query(&self, question: &str) -> Result<QueryResult, Error> {
        let query = self.natural_language_to_query(question).await?;
        self.manager.execute_query(query).await
    }
}

// Example usage:
// "Show me average Redis memory usage over the last hour"
// -> Query { integration: "redis", metric: "redis.memory.used", aggregation: "avg", time_range: "1h" }
```

### 3. IDE Extensions

Build extensions for popular IDEs to show infrastructure state inline.

#### VS Code Extension

```typescript
// vscode-extension/src/extension.ts
import * as vscode from 'vscode';
import { FluxxAgentClient } from './client';

export function activate(context: vscode.ExtensionContext) {
    const client = new FluxxAgentClient('http://localhost:8080');
    
    // Status bar item showing system health
    const statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    
    // Update status every 30 seconds
    setInterval(async () => {
        const health = await client.getOverallHealth();
        statusBarItem.text = health.status === 'healthy' 
            ? '$(check) Fluxx' 
            : '$(alert) Fluxx';
        statusBarItem.tooltip = health.summary;
        statusBarItem.show();
    }, 30000);
    
    // Command: Show Infrastructure Overview
    context.subscriptions.push(
        vscode.commands.registerCommand('fluxx.showOverview', async () => {
            const panel = vscode.window.createWebviewPanel(
                'fluxxOverview',
                'Fluxx Infrastructure Overview',
                vscode.ViewColumn.One,
                { enableScripts: true }
            );
            
            const data = await client.getOverview();
            panel.webview.html = generateOverviewHtml(data);
        })
    );
    
    // Hover provider: Show metric info on hover
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('*', {
            provideHover(document, position) {
                const word = document.getText(document.getWordRangeAtPosition(position));
                
                // Check if word matches an integration name
                if (isIntegrationName(word)) {
                    return createIntegrationHover(word, client);
                }
                
                return null;
            }
        })
    );
    
    // CodeLens: Show metrics above configuration blocks
    context.subscriptions.push(
        vscode.languages.registerCodeLensProvider('yaml', {
            provideCodeLenses(document) {
                const lenses: vscode.CodeLens[] = [];
                const text = document.getText();
                
                // Find integration config blocks
                const matches = text.matchAll(/integration:\s*(\w+)/g);
                for (const match of matches) {
                    const line = document.positionAt(match.index!).line;
                    const range = new vscode.Range(line, 0, line, 100);
                    
                    lenses.push(new vscode.CodeLens(range, {
                        title: '$(graph) View Metrics',
                        command: 'fluxx.showIntegrationMetrics',
                        arguments: [match[1]]
                    }));
                }
                
                return lenses;
            }
        })
    );
}
```

#### Cursor Extension (.cursorrules)

Add Cursor-specific rules to help the AI understand Fluxx Agent:

```yaml
# .cursorrules
context:
  - name: Fluxx Agent Integration
    description: |
      Fluxx Agent is a high-performance observability platform with 45+ integrations.
      
      When working with infrastructure monitoring:
      1. Use the IntegrationManager pattern for unified monitoring
      2. Check health before collecting metrics
      3. Use appropriate timeout values
      4. Handle errors gracefully
      
      Available integrations:
      - Databases: PostgreSQL, MySQL, Redis, MongoDB, Cassandra, etc.
      - Cloud: AWS, Azure, GCP, DigitalOcean, etc.
      - Message Queues: Kafka, RabbitMQ, SQS, etc.
      - CI/CD: GitHub Actions, GitLab CI, Jenkins, etc.
      
    examples:
      - trigger: "monitor database"
        code: |
          use fluxx_agent::integrations::IntegrationManager;
          
          let mut manager = IntegrationManager::new();
          manager.with_postgres("postgres://localhost/mydb");
          
          let metrics = manager.collect_metrics().await;
          let events = manager.collect_events().await;
      
      - trigger: "check health"
        code: |
          let health = manager.health_check().await;
          for (name, status) in health {
              println!("{}: {:?}", name, status);
          }

tools:
  - name: fluxx_query
    description: Query Fluxx Agent metrics
    parameters:
      query: string
    handler: |
      const result = await fetch(`http://localhost:8080/api/v1/query?q=${encodeURIComponent(query)}`);
      return result.json();
```

### 4. GitHub Copilot Integration

Create a Copilot extension for infrastructure queries.

```typescript
// copilot-extension/index.ts
import { createServer } from '@copilot-extensions/preview-sdk';

const server = createServer({
  // Handle Copilot chat requests
  async handleRequest(request, response) {
    const { messages } = request;
    const lastMessage = messages[messages.length - 1].content;
    
    // Check if message is about infrastructure
    if (isInfrastructureQuery(lastMessage)) {
      // Query Fluxx Agent
      const fluxxData = await queryFluxxAgent(lastMessage);
      
      // Stream response with context
      response.write(`Based on your infrastructure data:\n\n`);
      response.write(fluxxData.summary);
      response.write(`\n\n**Recommendations:**\n`);
      response.write(fluxxData.recommendations);
    }
  },
});

async function queryFluxxAgent(query: string) {
  // Convert natural language to Fluxx query
  const metrics = await fetch('http://localhost:8080/api/v1/metrics');
  const events = await fetch('http://localhost:8080/api/v1/events');
  
  // Use LLM to analyze and summarize
  const analysis = await analyzeWithLlm(query, { metrics, events });
  return analysis;
}
```

### 5. Continue.dev Configuration

Add Fluxx Agent as a context provider in Continue.dev.

```json
// .continue/config.json
{
  "contextProviders": [
    {
      "name": "fluxx-agent",
      "params": {
        "apiUrl": "http://localhost:8080",
        "autoRefresh": true,
        "refreshInterval": 60
      }
    }
  ],
  "slashCommands": [
    {
      "name": "infrastructure",
      "description": "Query infrastructure status",
      "handler": "fluxx.queryInfrastructure"
    },
    {
      "name": "diagnose",
      "description": "Run diagnostics on a service",
      "handler": "fluxx.diagnoseService"
    }
  ]
}
```

```typescript
// continue-fluxx-provider.ts
export class FluxxContextProvider {
  private client: FluxxAgentClient;
  
  constructor(config: { apiUrl: string }) {
    this.client = new FluxxAgentClient(config.apiUrl);
  }
  
  async getContextItems(query: string): Promise<ContextItem[]> {
    // Parse query to determine what infrastructure info is needed
    if (query.includes('database') || query.includes('postgres') || query.includes('redis')) {
      const dbMetrics = await this.client.getDatabaseMetrics();
      return [{
        name: 'Database Metrics',
        description: 'Current database health and metrics',
        content: JSON.stringify(dbMetrics, null, 2),
      }];
    }
    
    if (query.includes('kubernetes') || query.includes('k8s') || query.includes('pod')) {
      const k8sMetrics = await this.client.getKubernetesMetrics();
      return [{
        name: 'Kubernetes Metrics',
        description: 'Cluster status and pod metrics',
        content: JSON.stringify(k8sMetrics, null, 2),
      }];
    }
    
    // Default: provide overall health
    const health = await this.client.getOverallHealth();
    return [{
      name: 'Infrastructure Health',
      description: 'Overall infrastructure health status',
      content: JSON.stringify(health, null, 2),
    }];
  }
}
```

### 6. Aider Integration

Create a custom command for Aider to query Fluxx Agent.

```python
# .aider/integrations/fluxx.py
import requests
from aider.commands import Commands

class FluxxCommands(Commands):
    def cmd_fluxx(self, args):
        """Query Fluxx Agent for infrastructure information"""
        
        if not args:
            # Show overall health
            response = requests.get('http://localhost:8080/api/v1/health')
            health = response.json()
            return f"Infrastructure Health:\n{self.format_health(health)}"
        
        if args.startswith('metrics'):
            # Query metrics
            query = args[7:].strip()
            response = requests.get(f'http://localhost:8080/api/v1/metrics?q={query}')
            metrics = response.json()
            return f"Metrics:\n{self.format_metrics(metrics)}"
        
        if args.startswith('events'):
            # Get recent events
            response = requests.get('http://localhost:8080/api/v1/events?limit=10')
            events = response.json()
            return f"Recent Events:\n{self.format_events(events)}"
        
        if args.startswith('diagnose'):
            # Run diagnostics
            service = args[8:].strip()
            response = requests.post('http://localhost:8080/api/v1/diagnose', 
                                   json={'service': service})
            diagnosis = response.json()
            return f"Diagnosis for {service}:\n{self.format_diagnosis(diagnosis)}"

# Usage in Aider:
# /fluxx                    - Show overall health
# /fluxx metrics postgres   - Show PostgreSQL metrics
# /fluxx events             - Show recent events
# /fluxx diagnose redis     - Diagnose Redis issues
```

### 7. Sourcegraph Cody Integration

Add Fluxx Agent as a Cody context source.

```typescript
// cody-fluxx-integration.ts
import { SourcegraphCodyClient } from '@sourcegraph/cody';

export class FluxxCodyIntegration {
  constructor(private fluxxUrl: string) {}
  
  async getContext(query: string): Promise<ContextMessage[]> {
    // Determine what infrastructure context is relevant
    const keywords = this.extractKeywords(query);
    
    const contexts: ContextMessage[] = [];
    
    if (keywords.includes('performance') || keywords.includes('slow')) {
      const perfData = await this.getPerformanceMetrics();
      contexts.push({
        speaker: 'human',
        text: `Current performance metrics:\n${JSON.stringify(perfData, null, 2)}`,
      });
    }
    
    if (keywords.includes('error') || keywords.includes('failure')) {
      const events = await this.getRecentEvents('error');
      contexts.push({
        speaker: 'human',
        text: `Recent error events:\n${JSON.stringify(events, null, 2)}`,
      });
    }
    
    return contexts;
  }
  
  private async getPerformanceMetrics() {
    const response = await fetch(`${this.fluxxUrl}/api/v1/metrics?type=performance`);
    return response.json();
  }
  
  private async getRecentEvents(severity: string) {
    const response = await fetch(`${this.fluxxUrl}/api/v1/events?severity=${severity}&limit=10`);
    return response.json();
  }
}
```

## AI-Powered Use Cases

### 1. Auto-Generated Monitoring Configurations

```rust
// Generate configuration from natural language description
pub async fn generate_config(description: &str) -> Result<String, Error> {
    let prompt = format!(
        r#"Generate Fluxx Agent integration configuration based on this description.
        
Description: {}

Available integrations and their parameters:
{}

Generate a YAML configuration file that sets up monitoring for this infrastructure.
Include appropriate intervals, timeouts, and tags."#,
        description,
        list_available_integrations()
    );
    
    let config = llm.generate(&prompt).await?;
    validate_config(&config)?;
    Ok(config)
}

// Example:
// "I have a microservices setup with PostgreSQL, Redis, running on AWS with Kubernetes"
// -> Generates complete configuration with postgres, redis, aws, kubernetes integrations
```

### 2. Intelligent Alerting

```rust
// AI analyzes metrics and suggests alert thresholds
pub async fn suggest_alerts(metrics_history: &[Metric]) -> Result<Vec<AlertRule>, Error> {
    let prompt = format!(
        r#"Analyze these metrics and suggest appropriate alert thresholds.
        
Metrics history: {}

For each metric, suggest:
1. Warning threshold (95th percentile + 10%)
2. Critical threshold (99th percentile + 20%)
3. Duration before alerting
4. Recommended notification channels

Respond in JSON format."#,
        serde_json::to_string(metrics_history)?
    );
    
    let suggestions = llm.generate(&prompt).await?;
    serde_json::from_str(&suggestions)
}
```

### 3. Root Cause Analysis

```rust
// AI-assisted root cause analysis
pub async fn ai_root_cause_analysis(error: &ErrorContext) -> Result<AnalysisReport, Error> {
    // Collect relevant context
    let context = collect_diagnostic_context(error).await?;
    
    let prompt = format!(
        r#"Analyze this error and suggest root cause.
        
Error: {}
Stack trace: {}
Recent events: {}
Related metrics: {}
System state: {}

Provide:
1. Most likely root cause
2. Confidence score (0-1)
3. Supporting evidence
4. Recommended fix
5. Prevention strategies"#,
        error.message,
        error.stack_trace,
        context.events,
        context.metrics,
        context.system_state
    );
    
    let analysis = llm.generate_structured::<AnalysisReport>(&prompt).await?;
    Ok(analysis)
}
```

### 4. Natural Language Dashboard Queries

```typescript
// Ask questions about your infrastructure in natural language
app.post('/api/v1/ask', async (req, res) => {
  const { question } = req.body;
  
  // Convert question to Fluxx queries
  const queries = await nlp.toFluxxQueries(question);
  
  // Execute queries
  const results = await Promise.all(
    queries.map(q => fluxx.execute(q))
  );
  
  // Generate natural language response
  const response = await llm.generate(
    `Based on the infrastructure data, answer: ${question}\n\nData: ${JSON.stringify(results)}`
  );
  
  res.json({ answer: response, raw_data: results });
});

// Example interactions:
// Q: "Why is my API slow?"
// A: "Your PostgreSQL database is showing 150 active connections (limit: 100) and average query time has increased to 500ms. Redis cache hit ratio has dropped to 60%. Consider connection pooling and query optimization."

// Q: "Are there any security issues?"
// A: "HashiCorp Vault is currently sealed. 2 Jenkins nodes are offline. No database security events detected in the last hour."

// Q: "How much are we spending on AWS?"
// A: "You have 5 stopped EC2 instances still incurring EBS costs ($50/month). Consider terminating unused instances."
```

## Setting Up the Integration

### Step 1: Enable Fluxx Agent API

```yaml
# fluxx-config.yaml
api:
  enabled: true
  bind_address: "0.0.0.0:8080"
  cors:
    enabled: true
    allowed_origins:
      - "vscode://*"
      - "cursor://*"
      - "http://localhost:*"
  
  endpoints:
    - /api/v1/metrics
    - /api/v1/events
    - /api/v1/health
    - /api/v1/diagnose
    - /api/v1/nlp/query  # Natural language query endpoint

ai:
  enabled: true
  nlp_engine:
    provider: openai  # or anthropic, local
    model: gpt-4
  
  # Context window for AI assistants
  context:
    max_metrics: 100
    max_events: 50
    history_duration: 1h
```

### Step 2: Install IDE Extension

For VS Code:
```bash
code --install-extension fluxx.fluxx-agent
```

For Cursor:
```bash
# Add to .cursor/extensions/
git clone https://github.com/fluxx/cursor-extension ~/.cursor/extensions/fluxx-agent
```

### Step 3: Configure AI Assistant

Add to your AI assistant's context:

```yaml
# fluxx-context.yaml
name: Fluxx Agent Integration
description: |
  You have access to Fluxx Agent, which monitors infrastructure including:
  - Databases (PostgreSQL, MySQL, Redis, MongoDB, etc.)
  - Cloud providers (AWS, Azure, GCP, etc.)
  - Kubernetes and Docker
  - Message queues (Kafka, RabbitMQ, etc.)
  - CI/CD pipelines
  
  When asked about infrastructure:
  1. Query Fluxx Agent for current state
  2. Check for recent events
  3. Analyze metrics trends
  4. Provide specific recommendations
  
commands:
  /infrastructure - Show infrastructure overview
  /health [service] - Check service health
  /metrics [service] - Show service metrics
  /diagnose [service] - Run diagnostics
  /events - Show recent events
```

## Example Conversations

### With Claude (using MCP)

**User:** Why is my application slow?

**Claude:** I'll check your infrastructure metrics to diagnose the performance issue.

*[Claude queries Fluxx Agent via MCP]*

Based on the current metrics:

🔴 **PostgreSQL**: 150 active connections (limit: 100) - Connection pool exhausted
🟡 **Redis**: Cache hit ratio dropped to 60% (normally 85%+)
🟡 **Application**: Average response time 500ms (normally 100ms)

**Root Cause**: Database connection saturation causing query queuing.

**Recommendations**:
1. Increase PostgreSQL max_connections to 200
2. Implement connection pooling (PgBouncer)
3. Review slow queries: 3 queries taking >1s each
4. Consider Redis cluster for better cache performance

Would you like me to generate the configuration changes?

---

### With GitHub Copilot Chat

**User:** @fluxx Check if my Kubernetes deployment is healthy

**Copilot:**
```
Kubernetes Cluster Status:
━━━━━━━━━━━━━━━━━━━━━━━━
Nodes: 5/5 healthy ✅
Pods: 45/48 running (3 pending)
Deployments: 12/12 available ✅

⚠️  Issues Detected:
- Namespace 'production': 2 pods in CrashLoopBackOff
  - api-gateway-7d9f4b8c5-x2v9n: OutOfMemory
  - worker-5c4d7e8f9-y3w8m: ImagePullBackOff

🔧 Recommended Actions:
1. Increase memory limit for api-gateway deployment
2. Check image tag for worker deployment
3. Review recent events: 5 restarts in last hour
```

---

### With Cursor

**User:** [hovering over "postgresql" in config file]

**Cursor Tooltip:**
```
PostgreSQL (localhost:5432)
━━━━━━━━━━━━━━━━━━━━━━━━━━━
Status: 🟢 Healthy
Connections: 12/100 active
QPS: 450 queries/sec
Cache Hit Ratio: 98.5%
Replication Lag: 0ms

Recent Events: None
Last Check: 30s ago
```

---

## Best Practices

1. **Security**: Use API tokens, not admin credentials
2. **Performance**: Cache context to reduce API calls
3. **Privacy**: Don't send sensitive data to external LLMs
4. **Context Limits**: Be mindful of token limits when providing context
5. **Error Handling**: Gracefully handle when Fluxx Agent is unavailable

## Troubleshooting

### Connection Issues
```
Error: Cannot connect to Fluxx Agent
Solution: Check if API is running: curl http://localhost:8080/api/v1/health
```

### Context Too Large
```
Error: Context exceeds token limit
Solution: Reduce context size in configuration or use filtering
```

### Rate Limiting
```
Error: Too many requests
Solution: Implement caching or increase rate limits in Fluxx config
```

## Next Steps

1. Install the Fluxx Agent IDE extension
2. Configure your AI assistant with Fluxx context
3. Try asking natural language questions about your infrastructure
4. Set up auto-generated alerts based on AI recommendations

## Resources

- [Fluxx Agent MCP Server](https://github.com/fluxx/mcp-server)
- [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=fluxx.fluxx-agent)
- [Cursor Extension](https://github.com/fluxx/cursor-extension)
- [Example Configurations](https://github.com/fluxx/examples)
