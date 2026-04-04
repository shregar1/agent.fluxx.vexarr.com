# QUIC and Performance: Why Fluxx uses modern streaming protocols

Industrial observability is a high-speed game. When thousands of Modbus/TCP, Ethernet/IP, and gRPC events are being captured Every Second on the edge, the bottleneck is often the **Transport Layer**. This is why Fluxx is betting big on **QUIC**.

---

## 🐢 The TCP Problem: "Head-of-Line Blocking"

Traditional TCP (used by HTTP and gRPC) is a reliable protocol. But it has a major flaw: **Head-of-Line Blocking (HoLB)**.

- If one packet is lost on a noisy wireless connection at a factory, TCP waits.
- Every other packet in that stream—even if it's perfectly fine—is blocked from reaching the central hub until that one lost packet is re-transmitted.
- In common industrial environments (long-range LoRaWAN, noisy Wi-Fi/5G), this leads to **Jitter** and **Latency Spikes** in your observability data.

---

## 💨 The QUIC Solution: "Multiplexed Streams"

QUIC (HTTP/3) is built on top of UDP. It's reliable, encrypted, and designed for the modern internet.

1. **Independent Streams**: Fluxx can send `Modbus-Actuator` data and `HTTP-Auth` data as independent streams over a single QUIC connection.
2. **No HoLB**: If a packet for the `Auth` stream is lost, the `Actuator` stream continues to deliver data at 2ms latency. No blocking.
3. **Fast Handshakes**: QUIC combines the crypto-handshake (TLS 1.3) with the protocol-handshake. Fluxx initiates a connection to a remote hub in 1 round-trip (RTT), instead of 3 for TCP+TLS.

---

## 🏭 Why it matters for Edge Fleets

Imagine an **Industrial Gateway** on a mobile crane. It uses a cellular connection that occasionally drops packets.

- **With TCP**: The agent's cache fills up as packets are re-transmitted, eventually causing memory pressure or data loss at the edge.
- **With QUIC**: The "Critical" streams (like Emergency Stop alerts) continue to flow, while "Low Priority" telemetry (like ambient temperature) recovers in the background.

---

## 🚀 Future: XDP-Accelerated QUIC

We're prototyping an **XDP-based QUIC** handler in our **CaptureChannel**. This will allow Fluxx to re-assemble QUIC streams directly in the kernel's eDPF layer, achieving ultra-high-speed throughput with zero user-space intervention.

**Summary**: QUIC ensures your critical industrial data isn't blocked by a single lost packet on a noisy factory network. It's the performance backbone of Fluxx.
