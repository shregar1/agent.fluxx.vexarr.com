# Fluxx Agent Mesh Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Configuration](#configuration)
4. [Deployment Patterns](#deployment-patterns)
5. [Operations](#operations)
6. [API Reference](#api-reference)
7. [Troubleshooting](#troubleshooting)
8. [Performance Tuning](#performance-tuning)

---

## Overview

The Fluxx Agent Mesh enables distributed observability across multiple nodes with zero-code configuration. Nodes automatically discover each other, share state, and federate queries using a gossip-based protocol.

### Key Features

- **Zero-Code Deployment**: Configure via YAML or environment variables
- **Automatic Discovery**: Nodes find each other without manual coordination
- **Failure Detection**: Automatic detection and handling of node failures
- **Query Federation**: Execute queries across the entire mesh
- **Data Replication**: Configurable replication across regions
- **Horizontal Scaling**: Add nodes to increase capacity

### Use Cases

| Scenario | Topology | Nodes | Replication |
|----------|----------|-------|-------------|
| Single Datacenter | Ring | 3-5 | 2x |
| Multi-Region | Full Mesh | 9+ | 3x |
| Edge Computing | Star | 100+ | 1x |
| High Availability | Mesh per Zone | 6+ | 3x |

---

## Architecture

### Gossip Protocol

The mesh uses a SWIM-like gossip protocol for membership:

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│ Node A  │◄───►│ Node B  │◄───►│ Node C  │
│ (Seed)  │     │ (Peer)  │     │ (Peer)  │
└────┬────┘     └────┬────┘     └────┬────┘
     │               │               │
     └───────────────┴───────────────┘
              Gossip Messages
```

**Message Types:**
- `Ping`: Probe node liveness
- `Ack`: Acknowledge probe
- `StateSync`: Full membership list exchange
- `NodeUpdate`: Propagate node state changes

### Query Federation

```
┌─────────┐    ┌─────────┐    ┌─────────┐
│ Client  │───►│ Node A  │────┤         │
└─────────┘    │ (Proxy) │    │  Mesh   │
               └────┬────┘    │  Nodes  │
                    │         │         │
                    ├────────►│ B C D E │
                    │         │         │
                    │◄────────┤         │
               ┌────┴────┐    └─────────┘
               │ Aggregate│
               │ Results  │
               └─────────┘
```

### Data Replication

Replication strategies based on configuration:

| Strategy | Factor | Latency | Durability |
|----------|--------|---------|------------|
| None | 1 | Low | None |
| Local | 2 | Medium | Rack |
| Cross-Region | 3 | High | Region |

---

## Configuration

### Configuration File

```yaml
# mesh.yaml - Complete configuration reference

# Node Identity
node_id: us-east-1a                    # Unique identifier
bind: 0.0.0.0:7946                     # Mesh communication port

# Peer Discovery
peers:                                 # Initial peers to connect to
  - us-east-1b.internal:7946
  - us-west-2a.internal:7946
  - eu-west-1a.internal:7946

# Gossip Protocol
gossip:
  interval_ms: 200                     # Gossip frequency (default: 200)
  timeout_ms: 1000                     # Node death timeout (default: 1000)
  fanout: 3                            # Nodes to gossip per round (default: 3)

# Query Federation
federation:
  enabled: true                        # Enable cross-node queries (default: true)
  timeout_secs: 30                     # Query timeout (default: 30)
  max_concurrent: 10                   # Max parallel queries (default: 10)

# Data Replication
replication:
  factor: 2                            # Copies to maintain (default: 2)
  cross_region: true                   # Replicate across regions (default: true)
  async_replication: true              # Async replication (default: true)
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `FLUXX_MESH_CONFIG` | Path to config file | - |
| `FLUXX_NODE_ID` | Node identifier | Auto-generated |
| `FLUXX_MESH_BIND` | Bind address | `0.0.0.0:7946` |
| `FLUXX_MESH_PEERS` | Comma-separated peers | - |
| `FLUXX_MESH_FEDERATION` | Enable federation | `true` |
| `FLUXX_MESH_QUERY_TIMEOUT` | Query timeout (secs) | `30` |

### Configuration Precedence

1. Environment variables (highest)
2. Configuration file
3. Default values (lowest)

---

## Deployment Patterns

### Pattern 1: Single Datacenter (3-5 nodes)

```yaml
# Node configuration (per node)
node_id: dc1-node-{1,2,3}
bind: 0.0.0.0:7946
peers:
  - dc1-node-1.internal:7946
  - dc1-node-2.internal:7946
  - dc1-node-3.internal:7946

gossip:
  interval_ms: 200
  timeout_ms: 1000

replication:
  factor: 2
  cross_region: false
```

**Characteristics:**
- Low latency (< 1ms)
- 2x replication within DC
- Fast failure detection

### Pattern 2: Multi-Region (9+ nodes)

```yaml
# Per-region configuration
node_id: {region}-{zone}-{index}
bind: 0.0.0.0:7946
peers:
  # At least one per region
  - us-east-1a:7946
  - us-west-2a:7946
  - eu-west-1a:7946
  - ap-southeast-1a:7946

gossip:
  interval_ms: 500      # Slower for WAN
  timeout_ms: 5000      # Higher timeout

replication:
  factor: 3
  cross_region: true    # Global replication
```

**Characteristics:**
- Cross-region durability
- Higher latency (50-200ms)
- Automatic geo-routing

### Pattern 3: Edge Computing (100+ nodes)

```yaml
# Edge node configuration
node_id: edge-{{ auto }}
bind: 0.0.0.0:7946
peers:
  - central-hub.internal:7946

gossip:
  interval_ms: 2000     # Slow gossip
  timeout_ms: 10000     # Patient timeout
  fanout: 1             # Minimal traffic

replication:
  factor: 1             # No replication at edge
  cross_region: false
```

**Characteristics:**
- Minimal resource usage
- Central aggregation point
- Store-and-forward pattern

### Pattern 4: High Availability (6+ nodes)

```yaml
# Multi-zone deployment
node_id: {zone}-{index}
bind: 0.0.0.0:7946
peers:
  - zone-1a:7946
  - zone-1b:7946
  - zone-1c:7946
  - zone-2a:7946
  - zone-2b:7946
  - zone-2c:7946

gossip:
  interval_ms: 200
  timeout_ms: 1000

replication:
  factor: 3             # Zone + 2 copies
  cross_region: true
```

**Characteristics:**
- Survives zone failure
- Survives region failure
- Automatic failover

---

## Operations

### Starting a Node

```bash
# Using configuration file
FLUXX_MESH_CONFIG=/etc/fluxx/mesh.yaml ./fluxx

# Using environment variables
FLUXX_NODE_ID=node-1 \
FLUXX_MESH_BIND=0.0.0.0:7946 \
FLUXX_MESH_PEERS=peer1:7946,peer2:7946 \
./fluxx

# Using CLI generation
fluxx mesh config --nodes 3 --format yaml --output mesh.yaml
fluxx mesh docker --nodes 3 --output docker-compose.yaml
```

### Monitoring

```bash
# Check mesh status
curl http://localhost:8080/mesh/status

# Response:
{
  "node_id": "node-1",
  "bind_address": "0.0.0.0:7946",
  "mesh_size": 3,
  "healthy_nodes": 3,
  "suspect_nodes": 0,
  "federation": "enabled",
  "members": [
    {
      "id": "node-1",
      "address": "10.0.0.1:7946",
      "region": "us-east-1",
      "status": "healthy",
      "last_seen_ms": 50
    },
    {
      "id": "node-2",
      "address": "10.0.0.2:7946",
      "region": "us-east-1",
      "status": "healthy",
      "last_seen_ms": 120
    }
  ]
}
```

### Adding Nodes

1. **Prepare new node configuration:**
```yaml
node_id: node-4
bind: 0.0.0.0:7946
peers:
  - existing-node-1:7946
  - existing-node-2:7946
```

2. **Start the node:**
```bash
./fluxx
```

3. **Verify join:**
```bash
curl http://localhost:8080/mesh/status | jq '.mesh_size'
```

### Removing Nodes

1. **Graceful shutdown:**
```bash
# Send shutdown signal
kill -TERM <pid>

# Or via API
curl -X POST http://node-to-remove:8080/mesh/leave
```

2. **Force removal (if node is dead):**
```bash
# Remove from other nodes' peer lists
# Node will be marked dead after timeout
```

### Backup and Recovery

**Backup node state:**
```bash
# Copy data directory
tar czf backup-$(date +%Y%m%d).tar.gz /var/lib/fluxx/

# Backup configuration
cp /etc/fluxx/mesh.yaml backup/
```

**Recover node:**
```bash
# Restore data
tar xzf backup-20240101.tar.gz -C /

# Start with same node_id
FLUXX_NODE_ID=restored-node ./fluxx
```

---

## API Reference

### Mesh Module API

```rust
use fluxx_agent::mesh;

// Initialize from configuration
let mesh = mesh::init().await?;

// Get mesh size
let size = mesh.size().await;

// Get healthy nodes
let nodes = mesh.nodes().await;

// Check connection status
let connected = mesh.is_connected().await;

// Wait for mesh formation
let formed = mesh.wait_for_mesh(min_nodes, timeout_secs).await;

// Execute federated query
let results = mesh.query("SELECT * FROM events").await?;
```

### HTTP Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/mesh/status` | GET | Mesh membership status |
| `/mesh/nodes` | GET | List all nodes |
| `/mesh/health` | GET | Health check |
| `/mesh/query` | POST | Federated query |
| `/mesh/leave` | POST | Graceful leave |

---

## Troubleshooting

### Nodes Not Connecting

**Symptoms:** Mesh size remains 1

**Checklist:**
- [ ] Firewall allows UDP port 7946
- [ ] Peers list contains correct addresses
- [ ] Network connectivity between nodes
- [ ] No NAT/port forwarding issues

**Debug:**
```bash
# Enable debug logging
RUST_LOG=debug ./fluxx

# Test connectivity
nc -u -v peer-address 7946

# Check listening ports
netstat -ulnp | grep 7946
```

### High Latency

**Symptoms:** Slow query responses, gossip timeouts

**Solutions:**
1. Increase gossip interval:
```yaml
gossip:
  interval_ms: 500
  timeout_ms: 5000
```

2. Reduce fanout:
```yaml
gossip:
  fanout: 2
```

3. Enable compression:
```yaml
mesh:
  compression: true
```

### Data Loss

**Symptoms:** Missing events after node failure

**Solutions:**
1. Increase replication factor:
```yaml
replication:
  factor: 3
```

2. Enable sync replication (slower but safer):
```yaml
replication:
  async_replication: false
```

3. Use cross-region replication:
```yaml
replication:
  cross_region: true
```

---

## Performance Tuning

### Gossip Tuning

| Parameter | LAN | WAN | Edge |
|-----------|-----|-----|------|
| `interval_ms` | 200 | 500 | 2000 |
| `timeout_ms` | 1000 | 5000 | 10000 |
| `fanout` | 3 | 3 | 1 |

### Memory Usage

Per-node overhead:
- Base: ~50 MB
- Per peer: ~1 KB
- Per 1M events: ~500 MB (with compression)

**Formula:**
```
Total RAM = 50MB + (peers * 1KB) + (events * 500MB/M)
```

### Network Usage

Gossip bandwidth:
```
Bandwidth = nodes * (gossip_size * (1000/interval_ms)) bytes/sec

Where:
- gossip_size ≈ 100 bytes per node
- Example: 10 nodes at 200ms = 10 * 100 * 5 = 5 KB/sec
```

Query federation bandwidth:
```
Bandwidth = query_response_size * query_rate

Example: 10KB response * 100 QPS = 1 MB/sec
```

### Scaling Limits

| Metric | Limit | Notes |
|--------|-------|-------|
| Max Nodes | 1000 | Gossip overhead grows O(n) |
| Max Regions | 10 | Latency bound |
| Max Events/sec | 1M/node | Depends on hardware |
| Query Latency | < 100ms | Within region |

---

## Security

### Network Security

1. **Firewall Rules:**
```bash
# Allow mesh communication
iptables -A INPUT -p udp --dport 7946 -s 10.0.0.0/8 -j ACCEPT
iptables -A INPUT -p udp --dport 7946 -j DROP
```

2. **TLS for Federation:**
```yaml
mesh:
  tls:
    enabled: true
    cert_file: /etc/fluxx/cert.pem
    key_file: /etc/fluxx/key.pem
```

### Authentication

```yaml
mesh:
  auth:
    enabled: true
    token: ${MESH_AUTH_TOKEN}
```

---

## Migration Guide

### From Single Node to Mesh

1. **Prepare configuration:**
```bash
fluxx mesh config --nodes 3 --output mesh.yaml
```

2. **Update first node:**
```bash
# Stop existing node
kill -TERM <pid>

# Start with mesh config
FLUXX_MESH_CONFIG=mesh.yaml ./fluxx
```

3. **Add additional nodes:**
```bash
# Copy config, update node_id
# Start new nodes
```

### From Another System

**Datadog Agent → Fluxx Mesh:**
```yaml
# Replace Datadog agent
# Install Fluxx with mesh config
# Point to existing peers
```

---

## Best Practices

1. **Always use 3+ nodes** for production
2. **Deploy across zones** for HA
3. **Monitor mesh size** - alerts if < expected
4. **Use config files** for complex setups
5. **Test failure scenarios** regularly
6. **Backup configuration** before changes
7. **Use environment variables** for secrets
8. **Enable federation** for distributed queries

---

## Glossary

| Term | Definition |
|------|------------|
| Mesh | Network of connected Fluxx Agent nodes |
| Gossip | Protocol for disseminating membership state |
| Federation | Query execution across multiple nodes |
| Replication | Copying data to multiple nodes |
| Node ID | Unique identifier for a mesh node |
| Seed Node | Initial node for new nodes to connect to |
| Fanout | Number of nodes to gossip to per round |
| Incarnation | Version counter for node state updates |

---

## References

- [SWIM Protocol Paper](https://www.cs.cornell.edu/projects/Quicksilver/public_pdfs/SWIM.pdf)
- [Gossip Protocols](https://en.wikipedia.org/wiki/Gossip_protocol)
- [CAP Theorem](https://en.wikipedia.org/wiki/CAP_theorem)
