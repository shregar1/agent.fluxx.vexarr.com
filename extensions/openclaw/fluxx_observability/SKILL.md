---
name: fluxx_observability
description: Query infrastructure metrics, health, and run diagnostics using Fluxx Agent.
metadata:
  openclaw:
    os: ["darwin", "linux"]
    requires:
      bins: ["fluxx"]
    config:
      FLUXX_CONFIG: "path/to/fluxx.yaml"
---

# Fluxx Observability Skill

This skill teaches the agent how to use **Fluxx Agent** to monitor and diagnose infrastructure.

## Available Actions

### 1. Check Overall Health

When you need to know if the system is up or identify which services are failing:
`fluxx inspect all`

### 2. Query Specific Metrics

To get performance data for a specific service (e.g., "how is postgres memory?"):
`fluxx inspect <service_name>`

### 3. Run AI-Powered Diagnostics

When investigating a specific problem or error:
`fluxx mcp tools/call --name fluxx_diagnose --arguments '{"service": "<service>", "symptom": "<symptom>"}'`

### 4. Get Recent Events

To see a timeline of errors or warnings:
`fluxx mcp tools/call --name fluxx_get_events --arguments '{"severity": "error", "limit": 10}'`

## Guidelines

- Always check overall health first if the user reports a "general slowness" or "outage".
- Use `fluxx_diagnose` for complex troubleshooting.
- If a service is unhealthy, check its specific metrics to find the bottleneck (CPU, Memory, Connections).
