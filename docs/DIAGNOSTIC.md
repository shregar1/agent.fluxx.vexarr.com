# Diagnostic System Documentation

Complete guide to Fluxx Agent's AI-powered diagnostic and remediation system.

## Table of Contents

1. [Overview](#overview)
2. [Configuration](#configuration)
3. [API Reference](#api-reference)
4. [Ticketing Integrations](#ticketing-integrations)
5. [Remediation](#remediation)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

---

## Overview

The Diagnostic System uses Large Language Models (GPT-4, Claude) to:

- Analyze errors and identify root causes
- Generate fix recommendations with confidence scores
- Create code samples in your preferred language
- Automatically create tickets in Jira/ServiceNow/Linear
- Apply fixes automatically (with safeguards)

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Diagnostic Engine                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  AI         │  │ Remediation │  │  Ticketing          │ │
│  │  Analyzer   │  │   Engine    │  │  Providers          │ │
│  │             │  │             │  │                     │ │
│  │ • OpenAI    │  │ • Commands  │  │ • Jira              │ │
│  │ • Anthropic │  │ • Code      │  │ • ServiceNow            │ │
│  │ • Local LLM │  │ • Config    │  │ • Linear            │ │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘ │
│         │                │                    │            │
│         └────────────────┼────────────────────┘            │
│                          │                                 │
│              ┌───────────▼────────────┐                   │
│              │    Knowledge Base      │                   │
│              │  (Vector Search + ML)  │                   │
│              └────────────────────────┘                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Configuration

### Complete Configuration Example

```yaml
# diagnostic.yaml
diagnostic:
  # Enable/disable the entire system
  enabled: true
  
  # AI Provider Configuration
  ai:
    provider: openai  # openai, anthropic
    api_key: "${OPENAI_API_KEY}"
    model: gpt-4      # gpt-4, gpt-3.5-turbo, claude-3-opus
    max_tokens: 2000
    temperature: 0.3  # Lower = more deterministic
    timeout_seconds: 60
    enable_code_generation: true
    enable_root_cause_analysis: true
    
    # Custom prompts (optional)
    custom_prompts:
      root_cause: "Analyze this error focusing on database issues..."
      fix_generation: "Suggest fixes prioritizing minimal changes..."
  
  # Remediation Settings
  remediation:
    enabled: true
    auto_apply: false        # Require approval by default
    confidence_threshold: 0.85
    max_retries: 3
    rollback_on_failure: true
    approval_required_for: [critical, high]
  
  # Ticketing Integrations
  ticketing:
    jira:
      enabled: true
      host: "https://company.atlassian.net"
      username: "${JIRA_USER}"
      api_token: "${JIRA_TOKEN}"
      project_key: "OPS"
      issue_type: "Bug"
      auto_create: true
      labels: ["auto-generated", "ai-diagnostic"]
      priority_mapping:
        critical: "Highest"
        high: "High"
        warning: "Medium"
        info: "Low"
    
    servicenow:
      enabled: true
      token: "${GITHUB_TOKEN}"
      owner: "myorg"
      repo: "myrepo"
      auto_create: true
      labels: ["bug", "ai-diagnosed"]
      assignees: ["sre-team"]
    
    linear:
      enabled: false
      api_key: "${LINEAR_API_KEY}"
      team_id: "TEAM_ID"
  
  # Knowledge Base
  knowledge_base:
    enabled: true
    vector_store_path: "./kb_vectors"
    similarity_threshold: 0.8
    max_results: 5
    auto_learn: true
  
  # Learning & Feedback
  learning:
    enabled: true
    collect_feedback: true
    retrain_interval_hours: 24
    min_samples_for_retrain: 100
```

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `OPENAI_API_KEY` | OpenAI API key | For OpenAI provider |
| `ANTHROPIC_API_KEY` | Anthropic API key | For Claude provider |
| `JIRA_HOST` | Jira instance URL | For Jira integration |
| `JIRA_USER` | Jira username/email | For Jira integration |
| `JIRA_TOKEN` | Jira API token | For Jira integration |
| `GITHUB_TOKEN` | ServiceNow personal access token | For ServiceNow integration |
| `LINEAR_API_KEY` | Linear API key | For Linear integration |

---

## API Reference

### POST /diagnose

Submit an error for AI analysis.

**Request:**
```json
{
  "message": "Connection refused: postgres:5432",
  "error_type": "ConnectionError",
  "source": "api-service",
  "severity": "critical",
  "context": {
    "database": "postgres",
    "retry_count": "5"
  },
  "preferred_language": "rust",
  "include_remediation": true
}
```

**Response:**
```json
{
  "report_id": "diag-abc123",
  "root_cause": "PostgreSQL connection pool exhaustion",
  "explanation": "The application is trying to open more database connections than the pool allows...",
  "confidence": 0.92,
  "processing_time_ms": 4200,
  "fixes": [
    {
      "id": "fix-1",
      "title": "Increase connection pool size",
      "description": "Increase max connections from 10 to 50",
      "confidence": 0.94,
      "risk_level": "low",
      "auto_applicable": true,
      "steps": [
        "Edit config/database.yml",
        "Change pool size to 50",
        "Restart application"
      ]
    }
  ],
  "code_samples": [
    {
      "title": "Updated database configuration",
      "language": "yaml",
      "code": "production:\n  pool: 50\n  timeout: 5000",
      "explanation": "Increase pool size to handle more concurrent connections..."
    }
  ],
  "ticket": {
    "id": "OPS-1847",
    "url": "https://jira.company.com/OPS-1847",
    "provider": "jira"
  },
  "next_steps": [
    "1. Review the root cause: PostgreSQL connection pool exhaustion",
    "2. Review 1 suggested fix(es)..."
  ]
}
```

### GET /reports/:report_id

Retrieve a diagnostic report by ID.

**Response:**
```json
{
  "id": "diag-abc123",
  "error": { ... },
  "root_cause": { ... },
  "suggested_fixes": [ ... ],
  "code_samples": [ ... ],
  "confidence": 0.92,
  "timestamp_us": 1704067200000000
}
```

### POST /reports/:report_id/feedback

Submit feedback to improve the diagnostic system.

**Request:**
```json
{
  "helpful": true,
  "fix_successful": true,
  "applied_fix_id": "fix-1",
  "correct_root_cause": null,
  "comments": "The fix worked perfectly!"
}
```

### POST /reports/:report_id/ticket

Manually create a ticket for a diagnostic report.

**Response:**
```json
{
  "id": "OPS-1848",
  "url": "https://jira.company.com/OPS-1848",
  "provider": "jira"
}
```

### GET /knowledge-base

Search the knowledge base for similar issues.

**Response:**
```json
[
  {
    "id": "kb-1",
    "title": "Database Connection Pool Exhaustion",
    "similarity": 0.94,
    "tags": ["database", "postgres", "connection"]
  }
]
```

### GET /health

Check diagnostic system health.

**Response:**
```json
{
  "status": "healthy",
  "ai_enabled": true,
  "ticketing_enabled": true,
  "providers": {
    "openai": "healthy",
    "jira": "healthy",
    "servicenow": "degraded"
  }
}
```

---

## Ticketing Integrations

### Jira

**Setup:**
1. Go to https://id.atlassian.com/manage-profile/security/api-tokens
2. Create an API token
3. Configure in `diagnostic.yaml`

**Auto-Created Issue Format:**
```
Title: [CRITICAL] api-service - Connection refused: postgres:5432

h2. Error Details
*Message:* Connection refused: postgres:5432
*Type:* ConnectionError
*Source:* api-service

h2. Root Cause Analysis
*Primary Cause:* PostgreSQL connection pool exhaustion
*Explanation:* The application is trying to open more database connections...

h2. Suggested Fixes
# *Increase connection pool size* (confidence: 94%)
Increase max connections from 10 to 50
Steps:
#* Edit config/database.yml
#* Change pool size to 50
#* Restart application

h2. Code Samples
*Updated database configuration* (yaml):
{code:yaml}
production:
  pool: 50
  timeout: 5000
{code}
```

### ServiceNow

**Setup:**
1. Go to ServiceNow Settings → Developer settings → Personal access tokens
2. Generate token with `repo` scope
3. Configure in `diagnostic.yaml`

**Auto-Created Issue Format:**
Uses ServiceNow-flavored markdown with syntax highlighting.

### Linear

**Setup:**
1. Go to Linear Settings → API
2. Create a personal API key
3. Get your team ID from the URL
4. Configure in `diagnostic.yaml`

---

## Remediation

### Safe Auto-Remediation

The remediation engine can automatically apply fixes with multiple safeguards:

1. **Confidence Threshold**: Only fixes with confidence > threshold are auto-applied
2. **Approval Gates**: Critical fixes require human approval
3. **Backup Creation**: Always create backups before changes
4. **Verification**: Run verification commands after fixes
5. **Auto-Rollback**: Revert changes if verification fails

### Remediation Actions

#### Command Execution
```yaml
action:
  type: command
  command: "systemctl restart nginx"
  working_dir: "/etc/nginx"
  env_vars:
    ENV: "production"
```

#### Code Changes
```yaml
action:
  type: code_change
  file_path: "src/config.rs"
  search_pattern: "pool_size: 10"
  replacement: "pool_size: 50"
```

#### Config Changes
```yaml
action:
  type: config_change
  file_path: "config/database.yml"
  key: "production.pool"
  new_value: "50"
```

#### Service Restarts
```yaml
action:
  type: restart
  service_name: "api-service"
```

#### Scaling
```yaml
action:
  type: scale
  service_name: "worker"
  replicas: 10
```

### Remediation Plan Example

```yaml
remediation_plan:
  id: "plan-abc123"
  total_estimated_time_minutes: 15
  overall_risk: "low"
  requires_approval: false
  steps:
    - step_number: 1
      title: "Backup current configuration"
      description: "Create backup of database.yml"
      action_type: command
      action:
        type: command
        command: "cp config/database.yml config/database.yml.bak"
      
    - step_number: 2
      title: "Update connection pool"
      description: "Increase pool size from 10 to 50"
      action_type: config_change
      action:
        type: config_change
        file_path: "config/database.yml"
        key: "production.pool"
        new_value: "50"
      verification: "cat config/database.yml | grep pool"
      rollback:
        type: command
        command: "cp config/database.yml.bak config/database.yml"
    
    - step_number: 3
      title: "Restart application"
      description: "Apply configuration changes"
      action_type: restart
      action:
        type: restart
        service_name: "api-service"
      verification: "curl -f http://localhost:8080/health"
```

---

## Best Practices

### 1. Start with Manual Approval

```yaml
remediation:
  auto_apply: false  # Require approval initially
  confidence_threshold: 0.90
```

Monitor success rate, then gradually enable auto-apply for low-risk fixes.

### 2. Set Appropriate Confidence Thresholds

| Fix Type | Confidence Threshold | Auto-Apply |
|----------|---------------------|------------|
| Config changes | 0.90 | Yes |
| Code changes | 0.95 | No |
| Service restarts | 0.85 | Yes |
| Infrastructure | 0.98 | No |

### 3. Use Priority Mapping

```yaml
jira:
  priority_mapping:
    critical: "Highest"
    high: "High"
    warning: "Medium"
    info: "Low"
```

### 4. Label Auto-Generated Issues

```yaml
jira:
  labels:
    - auto-generated
    - ai-diagnostic
    - needs-review
```

### 5. Collect Feedback

Always submit feedback on diagnostic quality:

```rust
engine.submit_feedback(DiagnosticFeedback {
    report_id: report.id,
    helpful: true,
    fix_successful: Some(true),
    applied_fix_id: Some(fix.id),
    ..Default::default()
}).await?;
```

---

## Troubleshooting

### AI Provider Errors

**Error:** "OpenAI API error: 429 Too Many Requests"

**Solution:**
- Check rate limits
- Implement retry logic (built-in)
- Consider upgrading plan

**Error:** "OpenAI API error: 401 Unauthorized"

**Solution:**
- Verify API key is correct
- Check key hasn't expired
- Ensure key has proper permissions

### Ticketing Errors

**Error:** "Jira health check failed: 403"

**Solution:**
- Verify API token is valid
- Check user has create issue permission
- Ensure project key exists

**Error:** "ServiceNow API error: 404"

**Solution:**
- Verify owner/repo name
- Check token has repo scope
- Ensure repo exists and is accessible

### Remediation Failures

**Error:** "Verification failed after fix"

**Solution:**
- Check verification command is correct
- Review rollback was successful
- Apply fix manually and investigate

**Error:** "Rollback failed"

**Solution:**
- Check backup files exist
- Restore from external backup
- Review logs for root cause

---

## Metrics & Monitoring

### Key Metrics

| Metric | Description | Target |
|--------|-------------|--------|
| `diagnostic_duration_ms` | Time to generate report | < 10s |
| `diagnostic_confidence` | Average confidence score | > 0.85 |
| `fix_success_rate` | % of fixes that resolved issue | > 0.90 |
| `ticket_creation_rate` | Tickets created per day | < 50 |
| `escalation_rate` | Issues requiring human escalation | < 10% |

### Health Checks

```bash
# Check diagnostic system health
curl http://localhost:8080/diagnostic/health

# Check AI provider
curl http://localhost:8080/diagnostic/health/ai

# Check ticketing providers
curl http://localhost:8080/diagnostic/health/ticketing
```

---

## Security Considerations

### Data Privacy

- Error messages are logged locally before being sent to AI providers
- PII is automatically scrubbed from stack traces
- API keys are never logged
- Consider using local LLMs for sensitive environments

### Access Control

- Protect the `/diagnose` endpoint with authentication
- Use read-only API keys for ticketing integrations where possible
- Restrict auto-remediation to specific services
- Audit all remediation actions

### Compliance

- Diagnostic reports contain error details - ensure GDPR/HIPAA compliance
- Ticketing integrations create records in external systems
- Consider data retention policies for diagnostic data

---

## FAQ

**Q: Can I use local LLMs instead of OpenAI?**

A: Yes, local LLM support is on the roadmap. For now, you can use Ollama or similar with the API compatibility layer.

**Q: What happens if the AI gives bad advice?**

A: The system includes confidence scoring, requires approval for risky changes, and has automatic rollback. Always review fixes before applying.

**Q: Can I customize the prompts?**

A: Yes, use the `custom_prompts` configuration to override default prompts.

**Q: How much does it cost?**

A: Depends on error volume. Typical usage is $50-200/month in OpenAI API costs for medium-sized teams.

**Q: Does it work offline?**

A: The diagnostic engine requires internet access for AI analysis. Local LLM support is planned.

---

For more information, see:
- [Alert System Documentation](ALERTS.md)
- [Mesh Documentation](MESH.md)
- [API Reference](API.md)
