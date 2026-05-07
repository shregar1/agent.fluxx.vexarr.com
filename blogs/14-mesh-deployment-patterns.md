# 4 Deployment Patterns for Distributed Observability: From Edge to Global

**Real-world architectures for startups, enterprises, and everything in between.**

---

## The Deployment Spectrum

Observability needs vary wildly across organizations:

- A 3-person startup in Nairobi needs visibility into 5 microservices
- A Series B SaaS in São Paulo needs multi-region coverage
- A Fortune 500 needs global edge deployment
- A hospital needs air-gapped, high-availability monitoring

One size doesn't fit all. That's why Fluxx Mesh supports diverse deployment patterns through configuration alone.

## Pattern 1: Single Datacenter Ring

**Best for:** Startups, small teams, single-region deployments  
**Node count:** 3-5  
**Cost:** $50-100/month

### The Architecture

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Node 1  │◄───►│ Node 2  │◄───►│ Node 3  │
│ (Zone A)│     │ (Zone B)│     │ (Zone C)│
└────┬────┘     └────┬────┘     └────┬────┘
     │               │               │
     └───────────────┴───────────────┘
            Gossip Ring
```

Nodes arranged in a ring topology. Each node gossips with 3 peers (configurable). Data replicated 2x within the ring.

### Configuration

```yaml
# Node 1
node_id: dc1-node-1
bind: 0.0.0.0:7946
peers:
  - 10.0.1.10:7946  # Node 2
  - 10.0.1.11:7946  # Node 3

gossip:
  interval_ms: 200
  timeout_ms: 1000
  fanout: 3

replication:
  factor: 2
  cross_region: false
  async_replication: true
```

Nodes 2 and 3 have similar configs with appropriate peer lists.

### Why This Works

**Fault Tolerance:**  
- Can lose 1 node without data loss (2x replication)
- Can lose 1 zone without losing quorum
- Automatic recovery when node returns

**Performance:**
- < 1ms gossip latency within DC
- < 5ms query fanout to all nodes
- Local replication = fast writes

**Simplicity:**
- Static IP addresses work fine
- No external dependencies
- Easy to reason about

### Real-World Example

**Company:** FinTrack (fictional, but typical)  
**Location:** Nairobi, Kenya  
**Team:** 5 engineers  
**Infrastructure:** 12 microservices on AWS

**Their Setup:**
- 3 t3.small instances ($0.0208/hour each)
- 3-node Fluxx Mesh
- Total cost: $45/month

**Results:**
- Ingesting 50K events/sec
- P95 query latency: 12ms
- Survived AZ outage without data loss

**Quote from their CTO:**
> "We looked at Datadog but the cost was absurd—$2,000/month for our volume. With Fluxx Mesh, we get better performance for 2% of the cost. The 3-node setup took 20 minutes to deploy."

### When to Scale

Scale to Pattern 2 when:
- You need multi-region presence
- Data volume exceeds 1M events/sec/node
- You need geographic redundancy

---

## Pattern 2: Multi-Region Full Mesh

**Best for:** Global SaaS, distributed teams, compliance requirements  
**Node count:** 9+ (3 per region)  
**Cost:** $300-600/month

### The Architecture

```
        ┌─────────────────────────────────────────┐
        │              US-EAST-1                 │
        │  ┌─────┐   ┌─────┐   ┌─────┐          │
        │  │ 1a  │◄─►│ 1b  │◄─►│ 1c  │          │
        │  └──┬──┘   └──┬──┘   └──┬──┘          │
        │     └─────────┴─────────┘             │
        └──────┬───────────────────┬────────────┘
               │                   │
               ▼                   ▼
┌─────────────────────────┐  ┌─────────────────────────┐
│       US-WEST-2         │  │        EU-WEST-1        │
│  ┌─────┐   ┌─────┐      │  │  ┌─────┐   ┌─────┐      │
│  │ 2a  │◄─►│ 2b  │      │  │  │ 3a  │◄─►│ 3b  │      │
│  └──┬──┘   └──┬──┘      │  │  └──┬──┘   └──┬──┘      │
│     └─────────┘         │  │     └─────────┘         │
└──────────┬──────────────┘  └──────────┬──────────────┘
           │                            │
           └────────────┬───────────────┘
                        ▼
              ┌─────────────────┐
              │  AP-SOUTHEAST-1  │
              │  ┌─────┐┌─────┐  │
              │  │ 4a  ││ 4b  │  │
              │  └──┬──┘└──┬──┘  │
              │     └──────┘     │
              └──────────────────┘
```

Each region has an internal mesh. At least one node per region connects to other regions, forming a full mesh topology.

### Configuration

**Per-region config (us-east-1 example):**
```yaml
node_id: us-east-1a
bind: 0.0.0.0:7946

# Peers include:
# - All nodes in same region (for fast local gossip)
# - One node per other region (for cross-region connectivity)
peers:
  # Same region
  - us-east-1b.internal:7946
  - us-east-1c.internal:7946
  # Cross-region seeds
  - us-west-2a.internal:7946
  - eu-west-1a.internal:7946
  - ap-southeast-1a.internal:7946

gossip:
  interval_ms: 500      # Slower for WAN
  timeout_ms: 5000      # Higher timeout
  fanout: 3

federation:
  enabled: true
  timeout_secs: 60      # Generous for global queries
  max_concurrent: 20

replication:
  factor: 3             # Global redundancy
  cross_region: true    # Data residency
  async_replication: true
```

### Why This Works

**Geographic Resilience:**
- Survives region failure
- Data replicated across 3 regions
- Query routing to nearest region

**Compliance:**
- EU data stays in EU
- US data stays in US
- Configurable per-region retention

**Performance:**
- Local queries are fast
- Cross-region queries eventual
- Bandwidth-efficient gossip

### Real-World Example

**Company:** CloudRetail (fictional)  
**Location:** São Paulo, Brazil (HQ)  
**Team:** 45 engineers  
**Infrastructure:** 200+ services, 4 regions

**Their Setup:**
- 3 nodes per region = 12 total
- m5.large instances ($0.10/hour each)
- Cross-region data replication

**Cost Comparison:**
| System | Monthly Cost |
|--------|--------------|
| Datadog | $28,000 |
| New Relic | $22,000 |
| Dynatrace | $35,000 |
| Fluxx Mesh | $850 |

**Results:**
- Ingesting 2.3M events/sec globally
- P99 cross-region query: 180ms
- Zero data loss during São Paulo region outage

**Their Architecture Decision:**
> "We needed EU data residency for GDPR. With Datadog, that meant Enterprise tier at $35K/month. With Fluxx Mesh, we just set `cross_region: true` and deployed nodes in Frankfurt. Total cost under $1K."

### When to Scale

Scale to Pattern 3 when:
- You have 100+ edge locations
- You need store-and-forward semantics
- Bandwidth to edge is limited

---

## Pattern 3: Edge Computing Star

**Best for:** IoT, retail chains, CDN edges  
**Node count:** 100-10,000+  
**Cost:** $500-5,000/month (varies by edge count)

### The Architecture

```
                    ┌─────────────┐
                    │ Central Hub │
                    │  (3 nodes)  │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
    ┌────┴────┐      ┌────┴────┐      ┌────┴────┐
    │  Edge   │      │  Edge   │      │  Edge   │
    │  Node   │      │  Node   │      │  Node   │
    │ (Store) │      │ (Store) │      │ (Store) │
    └────┬────┘      └────┬────┘      └────┬────┘
         │                │                │
    ┌────┴────┐      ┌────┴────┐      ┌────┴────┐
    │ Sensors │      │ Sensors │      │ Sensors │
    └─────────┘      └─────────┘      └─────────┘

(100-1000s of edge locations)
```

Central hub aggregates from many edge nodes. Edge nodes operate independently, forwarding data when connected.

### Configuration

**Edge node:**
```yaml
node_id: edge-{{ auto }}    # Auto-generated unique ID
bind: 0.0.0.0:7946

# Only know the hub
peers:
  - hub.central.internal:7946

gossip:
  interval_ms: 2000     # Slow gossip for bandwidth
  timeout_ms: 10000     # Patient timeout
  fanout: 1             # Minimal traffic

federation:
  enabled: false        # Edge nodes don't federate

replication:
  factor: 1             # No replication at edge
  cross_region: false

storage:
  local_retention: 1h   # Keep 1 hour locally
  forward_to: hub.central.internal:8080
  forward_interval: 5m  # Batch forward every 5 min
```

**Hub node:**
```yaml
node_id: hub-1
bind: 0.0.0.0:7946

# Hub accepts connections from edge
# No static peers needed
peers: []

gossip:
  interval_ms: 500
  timeout_ms: 3000
  fanout: 5             # Higher fanout for many edges

replication:
  factor: 3             # Important data replicated
  cross_region: true    # Hub replicated across regions
```

### Why This Works

**Bandwidth Efficiency:**
- Edge nodes gossip minimally
- Data batched for forwarding
- Compression enabled by default

**Intermittent Connectivity:**
- Edge stores data locally
- Forward when connected
- Automatic retry with backoff

**Massive Scale:**
- Hub handles 1000s of edges
- No limit on edge count
- Horizontal scaling of hubs

### Real-World Example

**Company:** RetailChain (fictional)  
**Location:** 500 stores across North America  
**Infrastructure:** IoT sensors, POS systems, cameras

**Their Setup:**
- 500 Raspberry Pi 4 at edge ($75 each, one-time)
- 3-node hub cluster in AWS ($450/month)
- 4G connectivity from stores

**Data Flow:**
1. Sensors generate events at edge
2. Edge Pi buffers 1 hour locally
3. Batches compressed and forwarded every 5 min
4. Hub aggregates and queries

**Results:**
- 50K events/sec across all stores
- 150 MB/day per store (compressed)
- Total monthly cost: $850 (mostly 4G data)

**vs Previous Solution:**
They previously tried Datadog with 500 hosts at $15/host = $7,500/month just for infrastructure monitoring. With Fluxx Edge Mesh, they monitor 500 locations for $850/month—89% savings.

### When to Scale

Scale to Pattern 4 when:
- You need zero data loss guarantees
- You have compliance requirements for durability
- You're a regulated industry (finance, healthcare)

---

## Pattern 4: High Availability Mesh

**Best for:** Critical infrastructure, financial services, healthcare  
**Node count:** 6+ (2 per zone, 3+ zones)  
**Cost:** $600-1,200/month

### The Architecture

```
           Region: US-EAST-1
    ┌───────────────────────────────┐
    │                               │
┌───┴───┐    ┌───────┐    ┌───────┐│
│ Zone  │    │ Zone  │    │ Zone  ││
│  1a   │◄──►│  1b   │◄──►│  1c   ││
│┌─────┐│    │┌─────┐│    │┌─────┐│
││Node1││    ││Node3││    ││Node5││
││Node2││    ││Node4││    ││Node6││
│└─────┘│    │└─────┘│    │└─────┘│
└───────┘    └───────┘    └───────┘
    │             │             │
    └─────────────┴─────────────┘
         Zone Mesh (3 zones)
              │
              ▼
    [Other Regions: us-west-2, eu-west-1]
```

Multiple nodes per zone. Zones connected in mesh. Cross-region replication for disaster recovery.

### Configuration

```yaml
node_id: zone-1a-1
bind: 0.0.0.0:7946

# Full mesh: all nodes know all nodes
peers:
  # Zone 1a
  - zone-1a-2.internal:7946
  # Zone 1b
  - zone-1b-1.internal:7946
  - zone-1b-2.internal:7946
  # Zone 1c
  - zone-1c-1.internal:7946
  - zone-1c-2.internal:7946
  # Cross-region
  - zone-2a-1.us-west-2.internal:7946
  - zone-3a-1.eu-west-1.internal:7946

gossip:
  interval_ms: 200
  timeout_ms: 1000
  fanout: 3

federation:
  enabled: true
  timeout_secs: 30
  max_concurrent: 20

replication:
  factor: 3             # 3 copies = zone+2
  cross_region: true
  async_replication: false  # SYNC for durability

# Health monitoring
health:
  disk_threshold: 80    # Alert at 80% disk
  memory_threshold: 90  # Alert at 90% memory
  min_replicas: 2       # Alert if < 2 replicas
```

### Why This Works

**Maximum Durability:**
- 3x replication across zones
- Synchronous replication (acks from 2 nodes)
- Cross-region backup

**Failure Resilience:**
- Survives zone failure (2 other zones)
- Survives region failure (cross-region replicas)
- Survives multiple node failures

**Compliance:**
- Meets SOC2 requirements
- Audit logging enabled
- Encryption at rest and in transit

### Real-World Example

**Company:** MediVault (fictional)  
**Location:** US East and West  
**Industry:** Healthcare data storage  
**Requirements:** HIPAA compliant, 99.999% uptime

**Their Setup:**
- 6 nodes per region × 2 regions = 12 nodes
- m5.xlarge instances ($0.20/hour)
- Encrypted EBS volumes
- VPC peering between regions

**Compliance Features:**
```yaml
security:
  encryption:
    at_rest: true
    in_transit: true
    key_rotation: 30d
  
  audit:
    log_all_queries: true
    log_retention: 7y  # HIPAA requirement
  
  access:
    require_mtls: true
    role_based: true
```

**Results:**
- 500K events/sec ingestion
- RPO: 0 (no data loss)
- RTO: 30 seconds (automatic failover)
- 99.999% uptime (5 minutes downtime/year)

**Cost Comparison:**
| System | Monthly Cost | Notes |
|--------|--------------|-------|
| Splunk Enterprise | $85,000 | HIPAA add-on |
| Datadog Enterprise | $45,000 | Custom contract |
| Elastic Cloud | $28,000 | Managed service |
| Fluxx Mesh HA | $1,800 | Self-hosted |

**Their Security Officer:**
> "We passed our HIPAA audit on the first try. The auditors were impressed by the distributed architecture—no single point of failure, encryption everywhere, and complete audit trails. For 4% of the cost of Splunk."

### When You've Reached the Pinnacle

This is the most resilient pattern. If you need more:
- Add more regions (3+ for global resilience)
- Increase replication factor (4-5 for extreme durability)
- Enable quorum writes (slower but stronger consistency)

---

## Choosing Your Pattern

| Factor | Pattern 1 | Pattern 2 | Pattern 3 | Pattern 4 |
|--------|-----------|-----------|-----------|-----------|
| Team Size | 3-10 | 20-100 | 50-500 | 100+ |
| Budget | $50/mo | $500/mo | $1K/mo | $2K/mo |
| Nodes | 3-5 | 9-20 | 100+ | 6-50 |
| Regions | 1 | 2-5 | 1 (hub) + edges | 2+ |
| RPO | Minutes | Seconds | Hours | Zero |
| RTO | Minutes | Minutes | Hours | Seconds |
| Complexity | Low | Medium | Medium | High |

### Decision Tree

```
How many locations?
├── 1 datacenter
│   └── Pattern 1: Single DC Ring
├── 2-5 regions
│   └── Pattern 2: Multi-Region Mesh
├── 100+ edge locations
│   └── Pattern 3: Edge Star
└── Need zero data loss?
    └── Pattern 4: High Availability Mesh
```

## Migration Between Patterns

The beauty of configuration-driven deployment: you can evolve.

### Pattern 1 → Pattern 2

1. Deploy nodes in new region
2. Add new peers to existing config
3. Restart existing nodes
4. Enable cross_region: true

### Pattern 2 → Pattern 4

1. Add second node per zone
2. Update peer lists
3. Change async_replication: false
4. Increase factor: 3

No code changes. No data migration. Just configuration updates.

---

## The Common Thread

Whether you're a 3-person startup or a Fortune 500, the deployment process is the same:

1. **Generate config:** `fluxx-agent mesh config --nodes N`
2. **Customize:** Edit YAML for your topology
3. **Deploy:** Start agents on each node
4. **Verify:** Check `/mesh/status`

The complexity scales with your requirements, not with your tooling.

That's the Fluxx way.

---

## What's Next

Coming soon:
- **Auto-scaling**: Kubernetes HPA integration
- **Service mesh**: Istio/Linkerd automatic discovery
- **Multi-tenancy**: Namespace isolation within mesh
- **Federation v2**: Query optimization across 1000+ nodes

---

*Ready to deploy? Start with Pattern 1 and grow: [Deployment Guide](https://fluxx.vexarr.com/docs/mesh/deployment)*

*Need help choosing? [Book a free architecture review](https://fluxx.vexarr.com/consulting)*
