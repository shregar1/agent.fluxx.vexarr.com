# Azure Deployment Guide: Scaling Fluxx with Azure Data Explorer and AKS

For enterprises already built on Microsoft's **Azure Industrial IoT**, Fluxx is the perfect observability companion, moving data from the edge to the **Azure Data Explorer (ADX)** with zero friction.

---

## 🏭 Architecture: The "Industrial Bridge"

In an Azure-native Fluxx deployment, the central **Hub** runs on **Azure Kubernetes Service (AKS)**.

1. **AKS Hub**: Manages a scalable set of Fluxx pods for high-throughput event processing.
2. **Standard Load Balancer**: Use an **Azure Standard Load Balancer** with IPv4/v6 support to handle incoming **QUIC** and **HTTP/2** connections from the edge agents.
3. **Microsoft Entra ID (Azure AD)**: Securely manage agent identity and role-based access control (RBAC) across your global industrial fleet.

---

## 💾 Storage: Azure Data Explorer (Kusto)

The primary **Sink** for Fluxx on Azure is **Azure Data Explorer (ADX)**.

- **Kusto Ingestion**: Send every Modbus coil state or gRPC response payload directly into an ADX table.

- **In-Memory Cache**: Use ADX’s hot data cache for sub-second analysis of your last 24 hours of industrial failures.

- **KQL Queries**: Use the powerful **Kusto Query Language (KQL)** to find patterns like:

```kusto
EdgeEvents
| where Timestamp > ago(1h)
| where Environment == 'Berlin-Staging'
| where Service == 'Robotic-Arm-01'
| summarize P99 = percentiles(LatencyUs, 99) by bin(Timestamp, 1m)
```

---

## 🛠️ Step-by-Step: Deployment

### 1. Provision the AKS Cluster

Use **Bicep** or a standard **ARM Template** to deploy your AKS and ADX clusters.

### 2. Configure the ADX Sink

Update your `flux_hub.toml` to point to your ADX cluster's data-ingest URI:

```toml
[sink.azure_adx]
cluster_uri = "https://ingest-fluxx.westus.kusto.windows.net"
database = "observability"
table = "EdgeEvents"
```

### 3. Deploy the Edge Agents

Leverage **Azure IoT Edge** to containerize and deploy the Fluxx agent to your Linux-based edge gateways. Point your agents to your AKS's public DNS.

---

## 🚀 Industrial Case: The European Automotive Factory

A major car manufacturer uses **Fluxx** on its final assembly line.

- **Capture**: Fluxx agents on every workstation capture PLC Modbus/TCP traffic.
- **Streaming**: Data is streamed to an **Azure AKS** hub in `germanywestcentral`.
- **Analysis**: Plant managers use **Azure Managed Grafana** with ADX to identify "Bottleneck Signals" in the assembly sequence in real-time.

**Summary**: Azure Data Explorer and AKS provide the performance and KQL-driven insights that leading manufacturers demand.
