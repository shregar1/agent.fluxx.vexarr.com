# fluxx deploy: The Fleet's Provisioning Tool

Scaling your industrial observability from one node to 1,000 shouldn't be a manual task for your IT team. You need **Infrastructure-as-Code** for your observability agent. This is the role of the **`fluxx deploy`** command.

---

## 🏭 Operation: Scaling the Fleet at Once

When you run `fluxx deploy`, you're not just "starting" an agent; you're **Provisioning** it:

1. **One-Time Token Handshake**: Generate a single token from your **Fleet Management Portal** and use it to initiate a unique and long-lived **Agent Identity Hash**.
2. **Environment Policy Injection**: `fluxx deploy --env production-berlin` automatically downloads the latest **PII Masking** rules and **Topology Tags** from your central hub.
3. **Daemon Installation**: Automatically configures and enables the local **systemd** (Linux) or **launchd** (macOS) daemon for persistent, long-term monitoring.

---

## 🛠️ Usage Patterns

### Standard Provisioning

Provision a new node with your cloud project settings:

```bash
fluxx deploy --provider aws --region us-west-2
```

### Self-Hosted Deployment

Provision a new agent to your local Hub:

```bash
fluxx deploy --provider local --hub-ip 10.0.1.50
```

---

## 🚀 Performance: Verification Hooks

After deployment, the `deploy` command doesn't just stop. It runs an automated **Verification Hook**:

- **Health Scan**: It performs a brief diagnostic (similar to **`fluxx inspect`**) to ensure its protocol attachments are working.
- **Upstream Verification**: It sends its first "Heartbeat" batch to your chosen **Sink** (e.g., **Azure Data Explorer** or **AWS Timestream**).
- **The Result**: You know the agent is successfully "live" before the IT technician even leaves the station.

---

## 🏭 Industrial Case: The Global Food Factory

A major food company uses **`fluxx deploy`** as part of its automated provisioning scripts.

- **Infrastructure-as-Code**: The company's **Ansible** scripts call `fluxx deploy --provider gcp` on every new warehouse gateway.
- **Zero Touch**: The maintenance technician just plugs the gateway into the network; the script handles the rest.
- **The Result**: 50 factories in 20 countries, all provisioned with the same high-fidelity observability stack.

**Summary**: Your industrial fleet scales as fast as your deployment command. Use `fluxx deploy` to move from a single node to a global fleet with ease.
