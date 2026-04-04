# Modbus Integration: Bridging IT and OT with Fluxx

If you're in the industrial world, you know **Modbus**. Created in 1979, it's the grandfather of industrial communication. It's used by everything from PLC controllers to SCADA systems and solar inverters. But for most IT observability tools, Modbus is a "Black Box." Fluxx changes that.

---

## 🏭 The "Payload" Gap: IT vs. OT

When an IT tool like Prometheus says a server is "up," it means its HTTP endpoint returned a 200 OK. When a factory manager asks if the **Solar Inverter** is "up," they mean:

- Is the **Voltage** between 230v and 240v?
- Is the **Frequency** stable at 50Hz?
- Are the **Coils** for the automatic switch responding in <50ms?

---

## 🟠 The Fluxx Solution: "The Modbus Processor"

Fluxx's **CaptureChannel** has native support for both **Modbus/TCP** and **Modbus/RTU** (via serial bridges).

1. **Bit-Level Inspection**: Fluxx can deconstruct raw Modbus TCP payloads. Instead of just seeing "100 packets/sec," it sees the **Register Addresses** (like `40001` for temperature) and **Coil States** (like `00001` for the Motor Switch).
2. **Contextual Decoding**: You define the mapping. Fluxx's `config.json` can specify that Register `40023` is a `FLOAT32` representing **Amperage**.
3. **Real-Time Latency Analysis**: Modbus is sensitive. If the **Master** (the PLC) sends a `Read Multiple Registers` request and the **Slave** (the Inverter) takes 200ms to respond, your control loop might fail. Fluxx tracks this **Turn-Around Time** for every single Modbus packet.

---

## 🏭 Industrial Case: The Robotic Assembly Cell

A robotic arm is controlled via Modbus over Ethernet.

- **The PLC** sends commands to the `Arm-Joint-01`.
- **The Fluxx agent** captures these Modbus packets using its **eBPF-powered capture channel**.
- **It decodes the Modbus payload** to find the current position and velocity registers.
- **The AI Health Engine** analyzes the latency distribution. If the Arm-Joint-01's response time shifts from 2ms to 10ms, Fluxx triggers a **Mechanical Friction Anomaly**.

---

## 🚀 Future: Auto-Register Discovery

We're researching an "Active Probing" mode for Fluxx, where the agent can (safely) crawl a Modbus device to discover which registers are active and what data types they likely contain based on their behavior over 24 hours.

**Summary**: Modbus doesn't have to be a black box. With Fluxx, your oldest industrial hardware gets the same level of observability as your newest microservices.
