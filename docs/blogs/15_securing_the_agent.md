# Securing the Agent: API Keys and Secret Management in Fluxx

At the industrial edge, **Security** is as important as **High Performance**. Your Fluxx agent is capturing sensitive industrial control data—Modbus registers, HTTP control commands, and PII-laden logs. If the agent's authentication is compromised, your whole industrial fleet is at risk. This is why we built a robust **Secret Management** system into Fluxx.

---

## 🏭 The "Secured Access" Model: "API Keys"

Every interaction between an edge agent and the central **Fluxx Portal** is protected by high-entropy **API Keys**.

1. **Unique Identities**: Every Fluxx node has a unique **Agent Identity Hash**. This hash is derived from the hardware's serial number and a secure TPM-based key (where available).
2. **API Key / Secret Bundling**: To connect an agent to a **Project** in the Fleet Portal, you generate a unique `{api_key, api_secret}` pair.
3. **Encrypted Configurations**: The `config.json` that stores these keys can be encrypted on-disk, only being decrypted by the agent's internal lifecycle manager at startup.

---

## 🔵 Why it matters: "No Metadata Leakage."

Authentication in Fluxx isn't just about "Logging In." It's about protecting the **Metadata** of your industrial operation.

- **Metadata Obfuscation**: The agent uses its API key to authenticate every batch request to a **Sink**.
- **Rotatable Keys**: If a service technician's laptop is stolen or a factory's LAN is compromised, the **Project Manager** can instantly revoke the agent's API keys in the Fleet Portal.
- **No More Default Passwords**: Fluxx and its `config.json` have no default passwords. Every agent is unique.

---

## 🏭 Industrial Case: The Global Food Factory

A food processing company uses **Fluxx** to monitor its production lines across 50 international factories.

- **A technician in Madrid** uses a local API key to connect a new ingredient-monitoring sensor to the "Munich-Line-01" **Project**.
- **The Fleet Management Portal** verifies the Madrid-specific key and allows the agent to start streaming.
- **A month later**, the technician leaves the company. The IT team instantly revokes all API keys associated with his user account, ensuring the Munich edge node remains secure.

---

## 🚀 Future: HSM / TPM Integration

We're currently researching integration with **Hardware Security Modules (HSM)** and **Trusted Platform Modules (TPM)** found in many modern industrial ARM gateways. This would allow the Fluxx agent's private keys to be stored in secure silicon, making them virtually impossible to steal even if the attacker has physical access to the node.

**Summary**: Security shouldn't be an afterthought in industrial observability. With Fluxx, API Keys and Secret Management are the cornerstone of a secure, modular fleet.
