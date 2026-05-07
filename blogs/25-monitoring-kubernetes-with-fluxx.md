# Monitoring Kubernetes with Fluxx: A Complete Guide

**Published:** April 5, 2026  
**Reading time:** 15 minutes  
**Tags:** Kubernetes, Monitoring, Containers, DevOps

---

Kubernetes is powerful but complex. With dozens of pods, services, and nodes, how do you make sense of it all?

In this guide, I'll show you how to monitor your entire Kubernetes cluster with Fluxx—from cluster health to individual pod metrics—in under 10 minutes.

## The Kubernetes Monitoring Challenge

Let's face it: Kubernetes monitoring is hard.

- **Ephemeral resources:** Pods come and go constantly
- **Multiple abstraction layers:** Nodes → Pods → Containers → Apps
- **Distributed by design:** Services span multiple nodes
- **High cardinality:** Thousands of unique metric combinations

Traditional tools struggle. Prometheus is powerful but complex. Datadog is easy but expensive. We need something better.

## Why Fluxx for Kubernetes?

### Auto-Discovery

Fluxx automatically discovers your cluster resources:

```yaml
integrations:
  - type: kubernetes
    name: "k8s-production"
    kubeconfig: "~/.kube/config"
    auto_discover: true
    
    # Automatically monitors:
    # - All nodes
    # - All pods
    # - All services
    # - All deployments
```

No manual configuration needed. Deploy Fluxx, and it finds everything.

### Native Understanding

Fluxx understands Kubernetes concepts:

```
Cluster → Nodes → Namespaces → Pods → Containers

Not just raw metrics—contextual understanding:
- "This pod is part of deployment X"
- "This service routes to these pods"
- "This node is at 90% capacity"
```

### Cost Efficiency

| Setup | Cost/Month (100 nodes) |
|-------|------------------------|
| Prometheus + Grafana + Storage | $800 |
| Datadog | $15,000 |
| New Relic | $12,000 |
| **Fluxx** | **$400** |

**Savings: 97% vs Datadog, 50% vs self-hosted**

## Quick Start

### 1. Install Fluxx

```bash
curl -sSL https://fluxx.vexarr.com/install.sh | bash
```

### 2. Configure Kubernetes Integration

```yaml
# k8s-monitoring.yaml
integrations:
  - type: kubernetes
    name: "k8s-cluster"
    # Use in-cluster config when running inside K8s
    in_cluster: true
    
    # Or specify kubeconfig for external monitoring
    # kubeconfig: "~/.kube/config"
    
    # Namespaces to monitor (empty = all)
    namespaces:
      - "default"
      - "production"
      - "monitoring"
    
    # Metrics to collect
    metrics:
      - nodes
      - pods
      - containers
      - services
      - deployments
      - ingresses
      - persistent_volumes
      
    # Collection interval
    interval: "30s"
    
    # Include pod logs
    logs:
      enabled: true
      namespaces: ["production"]
      severity_filter: ["error", "warning"]

ai:
  enabled: true
  auto_remediate: true
```

### 3. Deploy to Kubernetes

```yaml
# fluxx-daemonset.yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: fluxx-agent
  namespace: monitoring
spec:
  selector:
    matchLabels:
      app: fluxx-agent
  template:
    metadata:
      labels:
        app: fluxx-agent
    spec:
      serviceAccountName: fluxx-agent
      containers:
        - name: fluxx
          image: fluxx/agent:latest
          args:
            - "start"
            - "--config"
            - "/etc/fluxx/config.yaml"
          ports:
            - containerPort: 8080
              name: http
            - containerPort: 7946
              name: mesh
          volumeMounts:
            - name: config
              mountPath: /etc/fluxx
            - name: data
              mountPath: /var/lib/fluxx
          env:
            - name: FLUXX_NODE_ID
              valueFrom:
                fieldRef:
                  fieldPath: spec.nodeName
            - name: OPENAI_API_KEY
              valueFrom:
                secretKeyRef:
                  name: fluxx-secrets
                  key: openai-key
      volumes:
        - name: config
          configMap:
            name: fluxx-config
        - name: data
          hostPath:
            path: /var/lib/fluxx
            type: DirectoryOrCreate
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: fluxx-agent
  namespace: monitoring
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: fluxx-agent
rules:
  - apiGroups: [""]
    resources: ["*"]
    verbs: ["get", "list", "watch"]
  - apiGroups: ["apps"]
    resources: ["*"]
    verbs: ["get", "list", "watch"]
  - apiGroups: ["networking.k8s.io"]
    resources: ["*"]
    verbs: ["get", "list", "watch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: fluxx-agent
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: fluxx-agent
subjects:
  - kind: ServiceAccount
    name: fluxx-agent
    namespace: monitoring
```

Deploy:

```bash
kubectl create namespace monitoring
kubectl apply -f fluxx-daemonset.yaml
```

### 4. Access the Dashboard

```bash
# Port forward
kubectl port-forward -n monitoring svc/fluxx-agent 8080:8080

# Open UI
open http://localhost:8080
```

## What You Get

### Cluster Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    CLUSTER OVERVIEW                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Nodes: 12 (11 healthy, 1 degraded)                         │
│  Pods: 348 (340 running, 5 pending, 3 failed)              │
│  CPU: 67% (812 / 1200 cores)                               │
│  Memory: 54% (1.2TB / 2.2TB)                               │
│  Storage: 40% (8TB / 20TB)                                 │
│                                                             │
│  ⚠️  1 node experiencing disk pressure                     │
│  ⚠️  3 pods restarting frequently                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Node Metrics

```bash
$ curl http://localhost:8080/metrics/kubernetes/nodes

{
  "nodes": [
    {
      "name": "worker-1",
      "status": "Ready",
      "capacity": {
        "cpu": "32",
        "memory": "128Gi",
        "pods": "110"
      },
      "usage": {
        "cpu": "45%",
        "memory": "60%",
        "pods": "45/110"
      },
      "conditions": [
        {"type": "Ready", "status": "True"},
        {"type": "DiskPressure", "status": "False"},
        {"type": "MemoryPressure", "status": "False"}
      ]
    }
  ]
}
```

### Pod Intelligence

Not just metrics—understanding:

```yaml
pod: "api-gateway-7d9f4b8c5-x2v4n"
  namespace: "production"
  status: "Running"
  
  # Resource usage
  cpu: "450m / 1000m (45%)"
  memory: "2.1Gi / 4Gi (52%)"
  
  # Relationships
  deployment: "api-gateway"
  service: "api-gateway-svc"
  ingress: "api.example.com"
  
  # Health
  restarts: 0
  ready: true
  
  # AI Insights
  insights:
    - "CPU usage trending up 15% over last hour"
    - "No errors in logs"
    - "Response time: 45ms (healthy)"
```

## Real-World Scenarios

### Scenario 1: Pod CrashLoopBackOff

**Problem:** Pod `payment-service-abc123` is crashing

**Fluxx Detection:**

```
🚨 CRITICAL: Pod payment-service-abc123 in CrashLoopBackOff

AI Analysis:
├── Root Cause: Out of memory (OOMKilled)
├── Evidence:
│   ├── Last state: Terminated (Exit Code: 137)
│   ├── Memory limit: 512Mi
│   ├── Peak usage: 547Mi
│   └── Error log: "Java heap space"
├── Suggested Fix:
│   └── Increase memory limit to 1Gi
└── Prevention:
    ├── Set memory requests = limits
    └── Add horizontal pod autoscaler
```

**Auto-remediation:**

```bash
# Fluxx can automatically apply the fix
kubectl patch deployment payment-service \
  -p '{"spec":{"template":{"spec":{"containers":[{"name":"app","resources":{"limits":{"memory":"1Gi"}}}]}}}}'
```

### Scenario 2: Node Disk Pressure

**Problem:** Node running out of disk space

**Fluxx Alert:**

```
⚠️ WARNING: Node worker-3 experiencing disk pressure

Current Usage:
├── Disk: 92% (920GB / 1TB)
├── Inodes: 87% (8.7M / 10M)
└── Largest consumers:
    ├── /var/log/containers: 300GB
    ├── /var/lib/docker: 450GB
    └── Unused images: 120GB

Actions Taken:
├── Cleaned old container logs (> 7 days)
├── Removed dangling Docker images
└── Freed 180GB

Recommendation:
├── Resize node disk to 2TB
└── Enable log rotation
```

### Scenario 3: Service Latency Spike

**Problem:** API latency jumped from 50ms to 2s

**Fluxx Diagnosis:**

```
📊 Latency Analysis: api-gateway

Before: 50ms p99
After:  2000ms p99

Root Cause Chain:
api-gateway (2s)
  └── payment-service (1.8s)
      └── postgres-primary (1.7s)
          └── Slow query detected

Query: SELECT * FROM transactions WHERE created_at > '...'
Duration: 1.6s
Rows: 15M (full table scan)

Fix: CREATE INDEX CONCURRENTLY idx_transactions_created_at 
     ON transactions(created_at);

Expected improvement: 1.6s → 5ms
```

## Advanced Configuration

### Multi-Cluster Monitoring

Monitor multiple clusters from a single pane:

```yaml
integrations:
  - type: kubernetes
    name: "k8s-prod-us"
    kubeconfig: "/etc/fluxx/kubeconfigs/prod-us"
    
  - type: kubernetes
    name: "k8s-prod-eu"
    kubeconfig: "/etc/fluxx/kubeconfigs/prod-eu"
    
  - type: kubernetes
    name: "k8s-staging"
    kubeconfig: "/etc/fluxx/kubeconfigs/staging"

mesh:
  enabled: true
  # Federate queries across clusters
```

### Custom Metrics

Add application-specific metrics:

```yaml
integrations:
  - type: kubernetes
    name: "k8s-production"
    custom_metrics:
      - name: "payment_latency"
        source: "prometheus"
        query: "histogram_quantile(0.99, rate(payment_duration_seconds_bucket[5m]))"
        
      - name: "checkout_errors"
        source: "logs"
        pattern: "ERROR.*checkout.*failed"
```

### Intelligent Alerting

Only alert on actionable issues:

```yaml
alerting:
  rules:
    # Don't alert on single pod restart
    - name: "Pod Restarting"
      condition: "pod.restart_count > 3 in 10m"
      severity: warning
      
    # But do alert on deployment-wide issues
    - name: "Deployment Unhealthy"
      condition: "deployment.available_replicas < deployment.replicas * 0.8"
      severity: critical
      
    # AI-filtered alerts
    - name: "AI Detected Anomaly"
      condition: "ai.anomaly_confidence > 0.9"
      severity: warning
```

## Cost Optimization

### Right-Sizing Recommendations

Fluxx analyzes actual usage and recommends resource adjustments:

```
Resource Optimization Report

Over-provisioned (potential savings):
├── api-gateway
│   ├── CPU: Request 1000m, Use 120m → Recommend 200m
│   └── Memory: Request 4Gi, Use 1.2Gi → Recommend 2Gi
│   └── Monthly savings: $145
├── worker-queue
│   └── CPU: Request 2000m, Use 300m → Recommend 500m
│   └── Monthly savings: $290
└── Total monthly savings: $435

Under-provisioned (risk of issues):
├── payment-processor
│   └── Memory: Limit 512Mi, Peak 680Mi → Recommend 1Gi
└── Risk: OOMKills during peak traffic
```

### Spot Instance Optimization

```yaml
# Automatically move workloads to spot instances
integrations:
  - type: kubernetes
    name: "k8s-production"
    spot_optimization:
      enabled: true
      
      # Identify fault-tolerant workloads
      selectors:
        - label: "app-tier=worker"
        - label: "fault-tolerant=true"
        
      # Automatically cordon on spot termination
      spot_handlers:
        - drain_on_termination: true
        - migrate_to_ondemand: true
```

## Best Practices

### 1. Resource Requests & Limits

Always set appropriate resources:

```yaml
resources:
  requests:
    cpu: "100m"
    memory: "256Mi"
  limits:
    cpu: "1000m"
    memory: "512Mi"
```

Fluxx monitors and alerts on:
- CPU throttling
- OOM kills
- Resource contention

### 2. Health Checks

Proper health checks enable better monitoring:

```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 5
```

### 3. Labels & Annotations

Use consistent labeling:

```yaml
labels:
  app.kubernetes.io/name: api-gateway
  app.kubernetes.io/component: backend
  app.kubernetes.io/part-of: payment-system
  environment: production
  team: platform
  cost-center: engineering
```

Fluxx uses these for:
- Filtering
- Cost allocation
- Team-based alerting

## Migration from Prometheus

### Side-by-Side Deployment

Run Fluxx alongside Prometheus during migration:

```yaml
integrations:
  - type: prometheus
    name: "existing-prometheus"
    url: "http://prometheus:9090"
    
  - type: kubernetes
    name: "k8s-native"
    # Native K8s monitoring
```

### Gradual Cutover

1. **Week 1:** Deploy Fluxx, compare metrics
2. **Week 2:** Migrate alerts one by one
3. **Week 3:** Switch dashboards
4. **Week 4:** Decommission Prometheus

### Metric Compatibility

Fluxx supports PromQL queries:

```bash
# Same query works in both
curl "http://fluxx:8080/query?q=rate(http_requests_total[5m])"
curl "http://prometheus:9090/api/v1/query?query=rate(http_requests_total[5m])"
```

## Troubleshooting

### "Forbidden: user cannot list resource"

```bash
# Check RBAC
kubectl auth can-i list pods --as=system:serviceaccount:monitoring:fluxx-agent

# If false, apply the ClusterRole from the deployment manifest
kubectl apply -f fluxx-rbac.yaml
```

### "Connection refused" to Kubernetes API

```bash
# Check if running in cluster
kubectl get pod -n monitoring fluxx-agent-xxx -o yaml | grep serviceAccount

# Verify API access
kubectl exec -it -n monitoring deployment/fluxx-agent -- curl -k https://kubernetes.default.svc/healthz
```

### High memory usage

```bash
# Check which namespaces are being monitored
fluxx config | grep namespaces

# Exclude high-volume namespaces temporarily
kubectl edit -n monitoring configmap/fluxx-config
# Add to exclude list: kube-system, monitoring
```

## The Results

**Company:** SaaS platform (200 microservices, 50 nodes)

| Metric | Before (Prometheus) | After (Fluxx) |
|--------|---------------------|---------------|
| Setup time | 2 weeks | 10 minutes |
| Monthly cost | $1,200 | $400 |
| Query latency | 2-5s | 50-200ms |
| MTTR | 45 min | 8 min |
| Alert fatigue | High | Low (AI-filtered) |
| Data retention | 15 days | 90 days |

**Quote:** *"We replaced our entire monitoring stack in a day. The AI diagnostics alone have saved us countless hours."* — Platform Lead

## Getting Started

```bash
# 1. Install Fluxx
curl -sSL https://fluxx.vexarr.com/install.sh | bash

# 2. Create config
cat > k8s-fluxx.yaml << 'EOF'
integrations:
  - type: kubernetes
    name: "my-cluster"
    in_cluster: true
    
ai:
  enabled: true
EOF

# 3. Deploy
kubectl apply -f - << 'EOF'
apiVersion: v1
kind: Namespace
metadata:
  name: monitoring
---
# ... (full manifest from above)
EOF

# 4. Access
kubectl port-forward -n monitoring svc/fluxx-agent 8080:8080
open http://localhost:8080
```

---

**Questions about Kubernetes monitoring?** Join our [Discord](https://discord.gg/fluxx) #kubernetes channel.

*Share your setup with #FluxxK8s on Twitter!*
