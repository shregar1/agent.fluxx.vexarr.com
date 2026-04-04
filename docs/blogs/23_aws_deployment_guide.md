# AWS Deployment Guide: Scaling Fluxx with Timestream and EKS

AWS is the most popular cloud for industrial data lakes. Fluxx integrates natively with the AWS ecosystem, allowing you to move from edge capture to centralized Timestream analysis in milliseconds.

---

## 🏭 Architecture: The "Cloud Bridge"

In an AWS-native Fluxx deployment, the central **Hub** runs on **Elastic Kubernetes Service (EKS)**.

1. **EKS Hub**: Handles incoming **QUIC** and **HTTP/2** connections from the edge agents.
2. **NLB Ingress**: An **AWS Network Load Balancer** provides a stable entry point for your global fleet.
3. **IAM Roles**: Every Fluxx agent uses **IAM Roles for Service Accounts (IRSA)** to securely authenticate with AWS services without long-lived credentials.

---

## 💾 Storage: AWS Timestream

The primary **Sink** for Fluxx on AWS is **Amazon Timestream**.

- **High-Resolution Telemetry**: Send raw Modbus/TCP registers and gRPC latency metrics directly to Timestream.
- **Magnetic Storage**: Older observability data is automatically moved to low-cost magnetic storage, preserving your budget while maintaining long-term visibility.
- **SQL-Based Analysis**: Use standard SQL to query across your entire factory fleet.

---

## 🛠️ Step-by-Step: Deployment

### 1. Provision the EKS Hub

Use the official Fluxx Terraform module to spin up your EKS cluster and NLB.

### 2. Configure the Timestream Sink

Update your `fluxx-hub.yaml` to point to your Timestream database:

```yaml
sinks:
  - type: aws-timestream
    region: us-west-2
    database: fluxx_observability
    table: edge_telemetry
```

### 3. Deploy the Edge Agents

Use **AWS IoT Greengrass** or a standard **AMI** to deploy the Fluxx binary to your edge gateways. Point them to your NLB's DNS name.

---

## 🚀 Industrial Case: The North American Power Grid

A renewable energy provider uses **Fluxx** to monitor 500 wind turbines.

- **Capture**: Fluxx agents on every turbine gateway capture rotational frequency via Modbus/RTU.
- **Aggregation**: Data is streamed to a centralized **AWS EKS** hub in `us-east-1`.
- **Analysis**: Engineers use **AWS Grafana** integrated with Timestream to identify mechanical vibration patterns across the entire grid.

**Summary**: AWS Timestream and EKS provide the ultimate scale for Fluxx's high-speed industrial capture engine.
