# local Mesh Testing with Docker

Fluxx Agent provides a fully automated local mesh testing environment using Docker Compose. This allows you to simulate a distributed infrastructure on a single machine, complete with metrics collection and visualization.

## 🚀 Quick Start

1.  **Generate the Environment**:
    ```bash
    fluxx mesh docker --nodes 3 --output docker-compose.yaml
    ```

2.  **Start the Full Stack**:
    ```bash
    docker-compose up --build -d
    ```

## 🏗️ Architecture

The environment consists of several key components:

### 1. Fluxx Mesh (3 Nodes)
*   **Node 1**: [http://localhost:3001](http://localhost:3001)
*   **Node 2**: [http://localhost:3002](http://localhost:3002)
*   **Node 3**: [http://localhost:3003](http://localhost:3003)
*   **Mesh Ports**: 7946, 7947, 7948
*   **Data Persistence**: Root directory `./data/node-x`

### 2. Sidecar Observability Stack
*   **Prometheus**: [http://localhost:9090](http://localhost:9090)
    *   Automatically scrapes telemetry from all nodes every 5 seconds.
*   **Grafana**: [http://localhost:3000](http://localhost:3000)
    *   Pre-configured with a bridge to Prometheus.
    *   Credentials: `admin/admin`

## 📊 Verification Commands

### Check Mesh Topology
```bash
docker exec -it fluxx-node-1 fluxx mesh status
```

### Inspect a Protocol Across Nodes
```bash
fluxx inspect http --endpoint http://localhost:3001
```

### View Telemetry Stream
```bash
docker-compose logs -f
```

## 🛠️ Customization

To change the number of nodes or configuration settings, modify the `docker-compose.yaml` or run the CLI generator with different flags:

```bash
fluxx mesh docker --nodes 5 --output cluster-5.yaml
```

Combined with Fluxx's **High-Performance Memory Mapped Storage**, this environment provides a ultra-low-latency testing bed for distributed observability strategies.
