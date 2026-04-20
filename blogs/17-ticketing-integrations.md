# Automated Ticketing: From Error to Jira in 3 Seconds

**Stop manually copying errors into tickets. Let Fluxx Agent do it for you.**

---

## The Copy-Paste Tedium

It's 2 AM. You're fighting a production incident. The error is complex—multiple services involved, stack traces, metrics, logs. You need to create a ticket for the post-mortem.

So you:
1. Open Jira
2. Click "Create Issue"
3. Type the title (probably missing key details)
4. Copy-paste the error message
5. Try to remember the context
6. Format it (poorly)
7. Set the priority (guesswork)
8. Add labels (inconsistently)
9. Submit

10 minutes later, you've created a ticket that's missing half the context. Your future self will thank you (not).

## Automated Ticketing

Fluxx Agent's diagnostic system automatically creates tickets with:

- ✅ **Full error context** - message, type, source, severity
- ✅ **Root cause analysis** - AI-generated explanation
- ✅ **Suggested fixes** - with confidence scores
- ✅ **Code samples** - syntax-highlighted, ready to use
- ✅ **Proper formatting** - native to each platform
- ✅ **Smart prioritization** - maps severity to ticket priority
- ✅ **Consistent labeling** - every time

All in under 3 seconds.

## Supported Platforms

### Jira

The gold standard for enterprise ticketing.

**What you get:**
- Wiki-style formatting (Atlassian markup)
- Structured sections (Error Details, Root Cause, Fixes)
- Code blocks with syntax highlighting
- Priority mapping (Critical → Highest)
- Label consistency
- Assignee auto-assignment

**Example Issue:**

```
Title: [CRITICAL] api-service - Connection refused: postgres:5432

h2. Error Details
*Message:* Connection refused: postgres:5432
*Type:* ConnectionError  
*Source:* api-service
*Severity:* Critical
*Diagnostic ID:* diag-7f3a9b2c

h2. Root Cause Analysis
*Primary Cause:* PostgreSQL connection pool exhaustion
*Explanation:* The application is trying to open more database 
connections than the pool allows. Each request holds a connection, 
and under high load, the pool saturates.

h2. Suggested Fixes
# *Increase connection pool size* (confidence: 94%, risk: low)
Increase max connections from 10 to 50
Steps:
#* Edit config/database.yml
#* Change pool: 10 to pool: 50
#* Restart application

h2. Code Samples
*Updated database configuration* (yaml):
{code:yaml}
production:
  adapter: postgresql
  pool: 50
  timeout: 5000
{code}

h2. Context
*database:* postgres
*host:* postgres:5432
*retry_count:* 5
```

**Configuration:**
```yaml
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
```

### ServiceNow Issues

Perfect for enterprise-grade projects or ServiceNow-native teams.

**What you get:**
- Markdown formatting
- Syntax-highlighted code blocks
- Labels for categorization
- Mention support (@user)
- Issue templates respected
- Project board integration

**Example Issue:**

```markdown
## Error Details
- **Message:** Connection refused: postgres:5432
- **Type:** ConnectionError
- **Source:** api-service
- **Diagnostic ID:** diag-7f3a9b2c

## Root Cause Analysis
**Primary Cause:** PostgreSQL connection pool exhaustion

**Explanation:** The application is trying to open more database 
connections than the pool allows...

## Suggested Fixes
### 1. Increase connection pool size (confidence: 94%)
Increase max connections from 10 to 50

**Steps:**
- Edit config/database.yml
- Change pool: 10 to pool: 50
- Restart application

## Code Samples
### Updated database configuration
```yaml
production:
  adapter: postgresql
  pool: 50
  timeout: 5000
```

Increase pool size to handle more concurrent connections...
```

**Configuration:**
```yaml
ticketing:
  servicenow:
    enabled: true
    token: "${GITHUB_TOKEN}"
    owner: "myorg"
    repo: "myapp"
    auto_create: true
    labels: ["bug", "ai-diagnosed", "production"]
    assignees: ["sre-oncall"]
```

### Linear

Modern issue tracking for high-performance teams.

**What you get:**
- Clean, minimal formatting
- Priority support (1-4 scale)
- Cycle/roadmap integration
- Team auto-assignment
- Label support
- Fast creation via GraphQL

**Configuration:**
```yaml
ticketing:
  linear:
    enabled: true
    api_key: "${LINEAR_API_KEY}"
    team_id: "TEAM_ID"  # Your Linear team ID
    auto_create: true
    labels: ["bug", "infrastructure"]
```

## Smart Prioritization

Not all errors are equal. The system maps Fluxx severity to ticket priority:

| Fluxx Severity | Jira Priority | Linear Priority | ServiceNow Label |
|----------------|---------------|-----------------|--------------|
| Critical | Highest | 1 (Urgent) | priority:critical |
| High | High | 2 (High) | priority:high |
| Warning | Medium | 3 (Medium) | priority:medium |
| Info | Low | 4 (Low) | priority:low |

Customize the mapping:

```yaml
jira:
  priority_mapping:
    critical: "Blocker"
    high: "Critical"
    warning: "Major"
    info: "Minor"
```

## Auto-Create vs Manual

### Auto-Create (Recommended for Production)

```yaml
jira:
  auto_create: true
```

- **Pros:** No manual work, immediate visibility, consistent formatting
- **Cons:** Can create noise if error volume is high
- **Best for:** Critical/high severity errors, production incidents

### Manual Creation

```yaml
jira:
  auto_create: false
```

- **Pros:** Human review before creation, no noise
- **Cons:** Requires manual action, may forget
- **Best for:** Development environments, non-critical warnings

### Hybrid Approach

```yaml
jira:
  auto_create: true
  # Only auto-create for critical/high severity
  # Filter by confidence threshold
```

Use the API to selectively create tickets:

```rust
// Only create ticket if confidence > 90%
if response.report.confidence > 0.90 {
    let ticket = engine.create_ticket(&response.report).await?;
}
```

## Real-World Use Cases

### Use Case 1: SRE Team

**Setup:**
```yaml
ticketing:
  jira:
    enabled: true
    project_key: "SRE"
    auto_create: true
    labels: ["sre", "production"]
    priority_mapping:
      critical: "Highest"
      high: "High"
```

**Result:**
- All critical production issues automatically tracked
- Consistent formatting for post-mortems
- SLA tracking enabled
- Reduced manual ticket creation by 90%

### Use Case 2: Enterprise Grade Project

**Setup:**
```yaml
ticketing:
  servicenow:
    enabled: true
    owner: "myorg"
    repo: "myproject"
    auto_create: false  # Manual review first
    labels: ["bug", "needs-triage"]
```

**Result:**
- Community contributors see well-formatted issues
- Maintainers can review before creating
- Consistent labeling helps with filtering
- Code samples help contributors fix issues

### Use Case 3: Multi-Team Organization

**Setup:**
```yaml
ticketing:
  jira:
    enabled: true
    project_key: "OPS"
  
  linear:
    enabled: true
    team_id: "ENGINEERING"
```

**Result:**
- Jira for executive/SLA tracking
- Linear for engineering workflow
- Same error creates both tickets automatically
- Teams can work in their preferred tools

## Ticket Lifecycle

### 1. Creation

```
Error occurs
    ↓
Fluxx Agent analyzes
    ↓
AI generates diagnostic report
    ↓
Ticket auto-created in Jira/ServiceNow/Linear
    ↓
Link added to alert/notification
```

### 2. Updates

As the diagnostic engine learns more:

```rust
// Add comment with additional findings
engine.add_ticket_comment(
    &ticket_id,
    "Additional logs show connection pool recovered after restart"
).await?;
```

### 3. Resolution

When the fix is applied:

```rust
// Resolve ticket with resolution notes
engine.resolve_ticket(
    &ticket_id,
    "Fixed by increasing connection pool size from 10 to 50"
).await?;
```

## Best Practices

### 1. Use Consistent Labels

```yaml
jira:
  labels:
    - auto-generated      # Always know it's from Fluxx
    - ai-diagnostic       # Indicates AI analysis included
    - needs-review        # For manual triage
```

### 2. Set Up Priority Mapping

Match your existing SLA structure:

```yaml
priority_mapping:
  critical: "P1-Immediate"    # 15 min response
  high: "P2-High"              # 1 hour response
  warning: "P3-Medium"         # 4 hour response
  info: "P4-Low"               # 24 hour response
```

### 3. Filter Auto-Creation

Don't auto-create for every error:

```rust
// Only create tickets for critical errors with high confidence
if error.severity == "critical" && report.confidence > 0.85 {
    engine.create_ticket(&report).await?;
}
```

### 4. Include Runbook Links

```yaml
jira:
  # In custom prompts or description templates
  custom_description: |
    h2. Runbooks
    * [Database Issues|https://wiki/runbooks/database]
    * [Connection Pool|https://wiki/runbooks/connection-pool]
```

### 5. Monitor Ticket Quality

Track metrics:
- Tickets created per day/week
- Average time to resolution
- False positive rate (tickets for transient errors)
- User feedback on ticket usefulness

## Cost Analysis

### Manual Ticketing

| Activity | Time | Cost/hour | Cost/ticket |
|----------|------|-----------|-------------|
| Create ticket | 10 min | $100 | $16.67 |
| Add context | 5 min | $100 | $8.33 |
| Format code | 3 min | $100 | $5.00 |
| **Total** | **18 min** | | **$30.00** |

### Automated Ticketing

| Activity | Time | Cost | Notes |
|----------|------|------|-------|
| AI analysis | 5 sec | $0.02 | OpenAI API |
| Ticket creation | 1 sec | $0.00 | API call |
| **Total** | **6 sec** | **$0.02** | |

**Savings: $29.98 per ticket (99.9% reduction)**

At 50 tickets/week:
- Manual: $1,500/week = $78,000/year
- Automated: $1/week = $52/year
- **Annual savings: $77,948**

## Migration Guide

### From Manual to Auto

1. **Week 1-2:** Enable with `auto_create: false`
   - Review generated tickets manually
   - Adjust formatting as needed
   
2. **Week 3-4:** Enable for non-critical only
   ```yaml
   # Only auto-create for warning/info
   ```

3. **Week 5+:** Enable full auto-create
   ```yaml
   auto_create: true
   ```

### From Existing Tools

If you're using PagerDuty/Opsgenie for incident tracking:

```yaml
# Continue using PagerDuty for alerts
# Use Fluxx for detailed diagnostics + ticketing

alerts:
  pagerduty:
    enabled: true  # For paging

diagnostic:
  ticketing:
    jira:
      enabled: true  # For detailed tracking
```

## Troubleshooting

### Issue: Duplicate Tickets

**Cause:** Multiple errors for same root cause

**Solution:** 
- Implement deduplication by error signature
- Use cooldown periods
- Link related errors instead of creating new tickets

### Issue: Too Many Tickets

**Cause:** High error volume

**Solution:**
```yaml
# Only create for critical
jira:
  auto_create: true
  # In code, filter by severity
```

### Issue: Poor Formatting

**Cause:** Platform-specific formatting issues

**Solution:**
- Check platform's markup documentation
- Use the `custom_description` option
- Test with sample errors

### Issue: Tickets Not Created

**Debug steps:**

1. Check health: `curl /diagnostic/health`
2. Verify credentials: `curl /diagnostic/health/ticketing`
3. Check logs for API errors
4. Test manually via API

## The Future

Coming soon:

- **Slack integration:** Create tickets from Slack with `/fluxx ticket`
- **Two-way sync:** Update tickets when fixes are applied
- **Custom templates:** Fully customizable ticket format
- **Project board automation:** Auto-assign to sprints/cycles
- **Parent/child linking:** Link related tickets automatically

## Conclusion

Manual ticket creation is a waste of engineering time. Automated ticketing ensures:

- ✅ Consistent, complete documentation
- ✅ No forgotten context
- ✅ Proper formatting
- ✅ Smart prioritization
- ✅ Immediate visibility

Spend your time fixing issues, not documenting them.

---

**Get started:**

```bash
# Add to your diagnostic.yaml
ticketing:
  jira:
    enabled: true
    host: "https://company.atlassian.net"
    project_key: "OPS"
    auto_create: true

# Set credentials
export JIRA_USER="ops@company.com"
export JIRA_TOKEN="..."

# Restart Fluxx Agent
fluxx-agent restart
```

[Full Documentation](https://fluxx.dev/docs/diagnostics) | [API Reference](https://fluxx.dev/api) | [ServiceNow](https://servicenow.com/fluxx/agent)
