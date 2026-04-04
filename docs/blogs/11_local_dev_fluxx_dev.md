# Local Dev with `fluxx dev`: Building Your First Observability Sandbox

Developing a production-grade observability pipeline for an industrial factory floor is a high-stakes game. You don't want to test your first **Modbus Filter** on a million-dollar robotic arm. This is where **`fluxx dev`** comes in.

---

## 🏭 The "Sandbox" Model: "No Hardware Required"

The `fluxx dev` command initiates a localized environment specifically for high-velocity developer testing.

1. **Mock Ingestion**: You don't need a real PLC. `fluxx dev` can spin up a **Mock Modbus Server** that generates synthetic traffic for your processors to analyze.
2. **No Persistence Required**: In `dev` mode, the agent uses an **In-Memory EventStore**. This means it's incredibly fast and has zero disk overhead, perfect for iterative testing of new **Processors**.
3. **Real-Time Live Reload**: Change your `config.json` while `fluxx dev` is running, and the agent automatically re-initializes its **Capture Channels** and **Pipeline**.

---

## 🛠️ The Developer's Flow

- **Step 1**: Run `fluxx dev`.
- **Step 2**: Define a new **Custom Processor** (e.g., `PowerUsageProcessor`) in your code.
- **Step 3**: Configure the `fluxx dev` instance to use the mock Modbus stream for that processor.
- **Step 4**: Monitor the results in real-time. Are your PII masking rules working? Is your anomaly detection engine identifying the synthetic "noise"?

---

## 🚀 Why it matters: "Shortening the Loop"

In a traditional industrial project, the "Dev-to-Deploy" loop can take weeks because you need to wait for hardware access.

**With `fluxx dev`**, an entire engineering team can build and test their observability pipelines synchronously, on their own laptops, before ever stepping foot on the factory floor.

---

## 🏭 Real-World Case: The New Security Policy

Your company introduces a new security policy: all **Auth Cookies** must be masked before leaving the edge node.

- **A developer** runs `fluxx dev` and points it at a mock HTTP stream containing auth cookies.
- **They iterate** on a new regex-based **SecurityProcessor**.
- **They verify** that the cookies are [MASKED] in the `fluxx inspect` output.
- **They push the config change** to the staging environment, confident that it will work on the real robotic controller.

---

## 🔵 Future: "Playback" Mode

We're researching a feature for `fluxx dev` that would allow developers to "Play Back" real-world capture files (captured via `tcpdump` or a previous `fluxx inspect` session) through their local agent. This would allow for perfect reproduction of rare edge-case anomalies.

**Summary**: `fluxx dev` is the developer's workshop. It's the safe, fast way to build the future of industrial observability without the hardware headaches.
