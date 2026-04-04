# Fluxx Agent Mesh Deployment

Deploy a distributed cluster of Fluxx Agent nodes with zero-code configuration changes.

## Quick Start

### 1. Single Node (Standalone)

```bash
# Just run the agent - no configuration needed
./fluxx-agent
```

### 2. Two-Node Mesh

**Node 1:**
```bash
FLUXX_NODE_ID=node-1 \
FLUXX_MESH_BIND=0.0.0.0:7946 \
FLUXX_MESH_PEERS=192.168.1.2:7946 \
./fluxx-agent
```

**Node 2:**
```bash
FLUXX_NODE_ID=node-2 \
FLUXX_MESH_BIND=0.0.0.0:7946 \
FLUXX_MESH_PEERS=192.168.1.1:7946 \
./fluxx-agent
```

### 3. Using Configuration File

Create `mesh.yaml`:
```yaml
node_id: my-node
bind: 0.0.0.0:7946
peers:
  - peer1.local:7946
  - peer2.local:7946
```

Run:
```bash
FLUXX_MESH_CONFIG=./mesh.yaml ./fluxx-agent
```

## Docker Compose

Start a 3-node mesh locally:

```bash
cd examples/mesh
docker-compose up -d
```

Check status:
```bash
docker-compose logs -f
```

## Kubernetes

Deploy to Kubernetes:

```bash
kubectl apply -f examples/mesh/kubernetes.yaml
```

Check pods:
```bash
kubectl get pods -n fluxx
```

## Configuration Options

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `FLUXX_MESH_CONFIG` | Path to config file | `/etc/fluxx/mesh.yaml` |
| `FLUXX_NODE_ID` | Unique node identifier | `us-east-1a` |
| `FLUXX_MESH_BIND` | Bind address | `0.0.0.0:7946` |
| `FLUXX_MESH_PEERS` | Comma-separated peer list | `node1:7946,node2:7946` |
| `FLUXX_MESH_FEDERATION` | Enable query federation | `true` |

### Configuration File

```yaml
# Node identity
node_id: unique-node-id
bind: 0.0.0.0:7946

# Peers to connect to
peers:
  - node-1.internal:7946
  - node-2.internal:7946

# Gossip protocol settings
gossip:
  interval_ms: 200        # How often to gossip
  timeout_ms: 1000        # Node death timeout
  fanout: 3               # Gossip fanout

# Query federation
federation:
  enabled: true
  timeout_secs: 30
  max_concurrent: 10

# Data replication
replication:
  factor: 2               # Copies to maintain
  cross_region: true      # Replicate across regions
  async_replication: true # Async for performance
```

## Deployment Patterns

### 1. Edge Computing

Many small nodes at edge locations:
```yaml
node_id: edge-{{ auto }}
bind: 0.0.0.0:7946
peers:
  - central-hub.internal:7946
gossip:
  interval_ms: 1000  # Slower for edge
replication:
  factor: 1          # Minimal replication
```

### 2. Multi-Region

Nodes across global regions:
```yaml
peers:
  # At least one peer in each region
  - us-east-1.internal:7946
  - eu-west-1.internal:7946
  - ap-southeast-1.internal:7946
replication:
  cross_region: true
  factor: 3
```

### 3. High Availability

Multiple nodes per zone:
```yaml
peers:
  # All nodes in all zones
  - zone-1a.internal:7946
  - zone-1b.internal:7946
  - zone-1c.internal:7946
replication:
  factor: 3
```

## Monitoring

Check mesh status via API:

```bash
curl http://localhost:8080/mesh/status
```

Response:
```json
{
  "node_id": "node-1",
  "mesh_size": 3,
  "healthy_nodes": 3,
  "members": [
    {"id": "node-1", "status": "healthy"},
    {"id": "node-2", "status": "healthy"},
    {"id": "node-3", "status": "healthy"}
  ]
}
```

## Troubleshooting

### Nodes not connecting

1. Check firewall rules for port 7946/UDP
2. Verify peer addresses are correct
3. Check logs: `RUST_LOG=debug ./fluxx-agent`

### High latency

1. Reduce gossip interval: `gossip.interval_ms: 500`
2. Enable compression
3. Use region-aware peers

### Data not replicating

1. Check replication factor >= 2
2. Verify `cross_region: true` for multi-region
3. Check disk space on nodes
