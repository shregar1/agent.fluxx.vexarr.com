# CI/CD Monitoring: From ServiceNow Actions to Jenkins

**Date:** February 1, 2024  
**Author:** Fluxx Agent Team  
**Category:** CI/CD Monitoring

Your CI/CD pipeline is the heartbeat of your engineering team. When builds fail or deployments get stuck, development slows down. Today, we'll explore how Fluxx Agent monitors 7 popular CI/CD platforms through a single interface, giving you visibility into build health, pipeline duration, and deployment success rates.

## Why CI/CD Monitoring Matters

Consider these scenarios:
- A build has been running for 3 hours (should take 10 minutes)
- A deployment failed silently last night
- Queue depth is at 50 builds (normally < 5)
- A Jenkins agent has been offline for days

Without unified monitoring, these issues go unnoticed until they impact production.

## Supported CI/CD Platforms

Fluxx Agent supports 7 major CI/CD platforms:

### ServiceNow Actions

```rust
manager.with_servicenow_actions("ghp_token", "myorg", "myrepo");
```

**Metrics:**
- Workflow runs by status
- Average duration
- Success rate
- Queue depth

**Events:**
- Workflow failures
- Long-running builds
- Deployment failures

### GitLab CI

```rust
// For a specific project
manager.with_gitlab_ci_project("https://gitlab.com", "token", "group/project");

// For entire instance
manager.with_gitlab_ci_instance("https://gitlab.com", "token");
```

**Metrics:**
- Pipeline status counts
- Job statistics
- Runner availability
- Queue size

### Jenkins

```rust
manager.with_jenkins("http://jenkins.internal:8080", "user", "api_token");
```

**Metrics:**
- Jobs by status (success/failed/unstable)
- Executor utilization
- Queue depth
- Node availability

### CircleCI

```rust
// Project-level
manager.with_circleci_project("token", "org", "repo");

// Organization-level
manager.with_circleci_org("token", "org");
```

**Metrics:**
- Pipeline state counts
- Workflow duration
- Success rate
- Credit usage (for insights)

### Travis CI

```rust
manager.with_travis_ci("token", "owner", "repo");
```

**Metrics:**
- Build states
- Duration trends
- Repository activity

### TeamCity

```rust
manager.with_teamcity("http://teamcity.internal", "user", "password");
```

**Metrics:**
- Build configurations
- Agent status
- Queue length
- Build duration

### Bamboo

```rust
manager.with_bamboo("http://bamboo.internal", "user", "password");
```

**Metrics:**
- Plan results
- Deployment status
- Agent availability
- Build queue

## Common Metrics Across All Platforms

| Metric | Description |
|--------|-------------|
| `*.builds.total` | Total builds/pipelines |
| `*.builds.success` | Successful builds |
| `*.builds.failed` | Failed builds |
| `*.builds.in_progress` | Currently running |
| `*.success_rate` | Percentage successful |
| `*.avg_duration_seconds` | Average build time |
| `*.queue.size` | Items waiting |
| `*.agents.total` | Available agents/runners |
| `*.agents.busy` | Busy agents/runners |

## Automatic Event Detection

All CI/CD integrations detect important events:

### Build Failures

```rust
Event::new(
    "Jenkins Job Failed",
    "Job 'deploy-production' has failed 3 times in a row",
    EventSeverity::Critical,
    "jenkins",
    "job_failed",
)
.with_metadata("job", "deploy-production")
.with_metadata("failures", "3");
```

### Long-Running Builds

```rust
Event::new(
    "Long Running Build",
    "Build #1234 has been running for 2 hours (expected: 15 min)",
    EventSeverity::Warning,
    "servicenow_actions",
    "long_running_build",
)
.with_metadata("build_id", "1234")
.with_metadata("duration_minutes", "120");
```

### Queue Backlog

```rust
Event::new(
    "CI Queue Backlog",
    "Queue depth is 45 builds (threshold: 20)",
    EventSeverity::Warning,
    "gitlab_ci",
    "queue_backlog",
)
.with_metadata("queue_depth", "45");
```

### Agent/Runner Issues

```rust
Event::new(
    "Jenkins Agent Offline",
    "Agent 'builder-3' has been offline for 2 hours",
    EventSeverity::Warning,
    "jenkins",
    "agent_offline",
)
.with_metadata("agent", "builder-3");
```

## Real-World Setup: Multi-Platform CI/CD

Many organizations use multiple CI/CD platforms:

```rust
use fluxx_agent::integrations::IntegrationManager;
use std::env;

let mut manager = IntegrationManager::new();

// Primary CI/CD: ServiceNow Actions
manager.with_servicenow_actions(
    &env::var("GITHUB_TOKEN")?,
    "mycompany",
    "main-service"
);

// Legacy systems: Jenkins
manager.with_jenkins(
    "http://legacy-jenkins.internal",
    &env::var("JENKINS_USER")?,
    &env::var("JENKINS_TOKEN")?
);

// Mobile builds: GitLab CI
manager.with_gitlab_ci_project(
    "https://gitlab.com",
    &env::var("GITLAB_TOKEN")?,
    "mobile/ios-app"
);

// Infrastructure: Terraform Cloud via CircleCI
manager.with_circleci_project(
    &env::var("CIRCLECI_TOKEN")?,
    "mycompany",
    "terraform-modules"
);

// Collect metrics from all platforms
let metrics = manager.collect_metrics().await;

// Calculate overall success rate
let total_builds: f64 = metrics
    .iter()
    .filter(|m| m.name.contains("builds.total"))
    .filter_map(|m| m.value.as_f64())
    .sum();

let successful_builds: f64 = metrics
    .iter()
    .filter(|m| m.name.contains("builds.success"))
    .filter_map(|m| m.value.as_f64())
    .sum();

let overall_success_rate = successful_builds / total_builds * 100.0;
println!("Overall CI/CD success rate: {:.1}%", overall_success_rate);
```

## Pipeline Performance Tracking

Track build duration trends:

```rust
// Metrics include average duration for each platform
// Example output:
// servicenow_actions.pipelines.avg_duration_seconds: 245.5
// jenkins.jobs.avg_duration_seconds: 189.2
// gitlab_ci.pipelines.avg_duration_seconds: 312.8
```

Alert on duration increases:

```rust
// If average duration increases by > 50%
Event::new(
    "Build Duration Increase",
    "Average build duration increased from 5min to 12min",
    EventSeverity::Warning,
    "servicenow_actions",
    "duration_increase",
);
```

## Deployment Monitoring

Track deployment frequency and success:

```rust
// ServiceNow Actions deployment workflow
manager.with_servicenow_actions("token", "org", "repo");

// Track deployments to production
// Metric: servicenow_actions.workflows.production_deployments
```

Deployment events:

```rust
Event::new(
    "Production Deployment",
    "Deployment v2.3.1 to production completed successfully",
    EventSeverity::Info,
    "servicenow_actions",
    "deployment_success",
)
.with_metadata("version", "2.3.1")
.with_metadata("environment", "production");
```

## Best Practices

### 1. Monitor All Environments

```rust
// Production
manager.with_servicenow_actions("prod_token", "org", "prod-repo");

// Staging
manager.with_servicenow_actions("staging_token", "org", "staging-repo");

// Development
manager.with_servicenow_actions("dev_token", "org", "dev-repo");
```

### 2. Track Runner/Agent Health

```rust
// Monitor available capacity
let health = manager.health_check().await;
for (name, status) in health {
    if name.contains("servicenow_actions") {
        match status {
            IntegrationStatus::Healthy => {
                println!("ServiceNow Actions runners available");
            }
            IntegrationStatus::Unhealthy(reason) => {
                println!("Runner issue: {}", reason);
            }
            _ => {}
        }
    }
}
```

### 3. Set Up Appropriate Alerts

```rust
// Alert on failure rate > 10%
// Alert on queue depth > 20
// Alert on no successful builds in 24 hours
```

### 4. Monitor Credential Expiration

```rust
// ServiceNow tokens expire
// Jenkins API tokens can be revoked
// Set up alerts before expiration
```

## Troubleshooting

### ServiceNow Actions: Authentication Failed

```
Error: ServiceNow API authentication failed
Solution:
  1. Check token hasn't expired
  2. Verify token has 'repo' scope
  3. Ensure token isn't revoked
```

### Jenkins: 403 Forbidden

```
Error: Cannot access Jenkins API
Solution:
  1. Check user has Overall/Read permission
  2. Verify API token is correct
  3. Check CSRF protection settings
```

### GitLab CI: Project Not Found

```
Error: Project not found or access denied
Solution:
  1. Verify project path (group/project)
  2. Check token has API scope
  3. Ensure token has project access
```

## Performance Considerations

CI/CD monitoring has minimal impact:

- **API Calls**: Respect rate limits with caching
- **Frequency**: Default 5-minute intervals
- **Data Volume**: ~100 metrics per platform
- **Memory**: < 50 MB for 7 platforms

Rate limits:

| Platform | Limit | Our Strategy |
|----------|-------|--------------|
| ServiceNow | 5000/hour | Cache for 5 minutes |
| GitLab | 2000/minute | Batched requests |
| CircleCI | 100/minute | Incremental fetch |
| Jenkins | No limit | Direct API calls |

## What's Next?

Coming soon:
- Build artifact tracking
- Test result aggregation
- Code coverage trends
- Deployment frequency metrics (DORA)
- Lead time for changes tracking

## Conclusion

With Fluxx Agent's CI/CD monitoring, you can:
- Monitor 7 platforms with one interface
- Get unified success rate metrics
- Detect issues before they impact developers
- Track pipeline performance over time

Stop logging into multiple CI/CD dashboards. Monitor them all in one place.

Get started:

```bash
cargo add fluxx-agent
```

---

*Keep your CI/CD pipelines healthy. Monitor them all with Fluxx Agent.*
