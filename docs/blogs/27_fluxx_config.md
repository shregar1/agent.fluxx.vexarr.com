# fluxx config: The Fluxx Agent's Brain

The `fluxx config` command is more than just a "settings" viewer. It's the command center for identity, secrets, and protocol management across your industrial observability fleet.

---

## 🏭 Configuration: Managing the Identity

Every Fluxx agent needs an identity. The `fluxx config` command lets you manage it:

1. **Identity Summary**: View your Agent Name, unique **Identity Hash**, and active **Project**.
2. **Upstream Targets**: Verify where the agent is sending its data. Is it pointed to your **Local Air-Gapped Hub** or your **GCP BigTable** sink?
3. **Secret Verification**: Check associated **API Keys** and **Secret tokens** (securely obfuscated during display).

---

## 🛠️ Usage Patterns

### View Current Summary

Get a detailed dump of every active parameter:

```bash
fluxx config
```

### Initializing a New Agent

Establish a new instance on your edge gateway:

```bash
fluxx config init --name "Factory-A-Gateway-01"
```

### Setting the Upstream Hub

Instruct the agent where to stream its **Sinks**:

```bash
fluxx config set upstream 10.0.1.50:4433
```

---

## 🚀 Architectural Goal: "No Configuration Drift"

The `config` command helps ensure your **Fleet Management Portal** is in sync with the hardware.

- **Environment Consistency**: Use `fluxx config` to verify that your staging environment settings haven't "drifted" from your production baseline.
- **Protocol Management**: See which **CaptureChannels** are enabled (Modbus, HTTP, WebSocket) and their specific target registers/ports.

---

## 🏭 Industrial Case: The Maintenance Checklist

A regional utility uses **`fluxx config`** as part of its monthly maintenance routine.

- **Technicians** run the command to verify that the local **GCP Hub IP** hasn't changed.
- **They check the Identity Hash** to ensure the node is correctly mapped in the utility's central observability portal.
- **The Result**: A perfectly synchronized fleet that never loses connectivity due to silent configuration drift.

**Summary**: Your agent is only as good as its configuration. Use `fluxx config` to keep your observability fleet perfectly aligned.
