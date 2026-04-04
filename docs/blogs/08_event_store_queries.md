# The EventStore: High-Performance Historical Queries at the Edge

Industrial observability doesn't just happen in real-time. Often, you need to look back:

- **"What was the latency of `auth-svc` yesterday at 3 AM?"**
- **"How many Modbus anomalies occurred on the `Conveyor-01` over the last 24 hours?"**

This is where the **EventStore** in Fluxx comes in.

---

## 💾 The Architecture: "Memory-First, Persistence-Driven"

At the heart of the Fluxx agent is a **BTreeMap-indexed** event store. Unlike traditional SQL databases that can be slow and heavy for edge hardware, our EventStore is optimized for **Time-Series Lookups**.

1. **Ordered Keys**: Every event is stored with its `u64` timestamp as the primary key. This allows for **Exact Time Range Queries** in `O(log N)` time.
2. **LRU Caching**: We keep the most recent events in memory for instant access by the API and local dashboard. Older events are flushed to structured local storage (using a lightweight WAL — Write-Ahead Log).
3. **Automatic Compaction**: No one wants their industrial gateway's disk to fill up. Fluxx automatically compacts and expires old events based on your `RetentionPolicy` (e.g., "Keep 7 days of raw events").

---

## 🚀 High-Velocity Lookups: The "Retrospective Analyzer"

Because our data is ordered, we can run **Retrospective Analysis** in milliseconds.

- **Step 1**: The user requests a 1-hour window from the EventStore.
- **Step 2**: The **BTreeMap** quickly identifies the range of blocks to read.
- **Step 3**: Fluxx runs its **Analyzer** layer over the historical data to find anomalies that were missed in real-time or to calculate new metrics (like P99 latency) for that specific hour.

---

## 🏭 Why it matters for Maintenance Teams

Imagine a factory line that had a minor "stutter" at 4 AM during a shift change. No alarm went off because it didn't cross a high-severity threshold.

The maintenance team can use `fluxx inspect` or a local dashboard to **Query the EventStore** for that 10-minute window. They'll see a detailed waterfall of every Modbus packet and HTTP response, allowing them to pinpoint a **Slow DNS Resolver** as the root cause of the shift-change lag.

---

## 🔵 Future: Distributed Event Store

We're prototyping a way for the **Fleet Management Portal** to query the EventStores of hundreds of edge nodes simultaneously. This "Federated Query" would allow you to see the cross-node latency of a distributed industrial application without ever moving all that raw data to a central database.

**Summary**: The EventStore ensures your edge nodes don't just "see" the present; they "remember" the past with the same level of granularity.
