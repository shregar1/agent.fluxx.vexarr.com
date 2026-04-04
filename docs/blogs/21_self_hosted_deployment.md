# Self-Hosted Deployment: Fluxx Behind the Firewall

In many industrial sectors—Defense, Energy, and Critical Infrastructure—data residency is non-negotiable. You cannot send your Modbus registers or PII-laden telemetry to the public cloud. This is why Fluxx is designed to be **Self-Hosted First**.

---

## 🏭 The Deployment Model: "The Local Hub"

When self-hosting Fluxx, you aren't just deploying an agent; you're deploying a **Local Observability Hub**.

1. **Air-Gapped Hub**: Deploy the Fluxx central hub on a hardened Linux server (Ubuntu/RHEL) within your factory's internal network.
2. **Local Identity Manager**: Instead of using our public Fleet Portal, the self-hosted hub manages its own API keys and agent certificates locally.
3. **Database Local**: All telemetry from the edge agents' **Sinks** is routed to your on-premise Time-Series Database (InfluxDB, TimescaleDB, or ClickHouse).

---

## 🛠️ Step-by-Step: The Linux Daemon

Deploying a Fluxx agent on a local Gateway (like an Advantech or Siemens edge box) is simple.

- **Step 1**: Copy the `fluxx` binary to `/usr/local/bin/`.
- **Step 2**: Initialize the local configuration using `fluxx config init --local`.
- **Step 3**: Point the agent to your local Hub's internal IP: `fluxx config set upstream 10.0.1.50`.
- **Step 4**: (Optional) Use `systemd` to ensure the agent runs as a persistent daemon:

```bash
systemctl enable fluxx-agent
systemctl start fluxx-agent
```

---

## 🚀 Why Self-Host? "Zero Latency, Maximum Security"

- **No Ingress Costs**: You're not paying for gigabytes of data leaving your network.
- **Physical Isolation**: Your OT (Operational Technology) network remains completely isolated from the public internet.
- **Data Sovereignty**: You own the data from capture to storage. No third-party data centers involved.

---

## 🏭 Industrial Case: The Nuclear Power Plant

A regional nuclear facility uses **Fluxx** to monitor its reactor secondary-cooling pumps.

- **Deployment**: Local Fluxx agents on every sensor gateway.
- **Connectivity**: Fully air-gapped network with no physical connection to the internet.
- **Storage**: A local Fluxx Hub stores 30 days of high-resolution telemetry for regulatory auditing and safety analysis.

**Summary**: Fluxx's self-hosted mode ensures that even the most sensitive industrial infrastructures can benefit from modern observability without compromising security.
