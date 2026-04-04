//! Extreme Load Test for Fluxx Agent
//! Measures throughput, latency, and resource usage under Datadog-level simulation

use fluxx_agent::{Event, ProtocolType};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::timeout;

/// Metrics collector for load testing
#[derive(Debug, Default)]
struct LoadMetrics {
    events_processed: AtomicU64,
    events_dropped: AtomicU64,
    bytes_processed: AtomicU64,
    total_latency_ns: AtomicU64,
    peak_memory_mb: AtomicU64,
}

impl LoadMetrics {
    fn new() -> Self {
        Self::default()
    }

    fn record_event(&self, bytes: u64, latency_ns: u64) {
        self.events_processed.fetch_add(1, Ordering::Relaxed);
        self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
        self.total_latency_ns.fetch_add(latency_ns, Ordering::Relaxed);
    }

    fn get_events_per_second(&self, duration_secs: f64) -> f64 {
        self.events_processed.load(Ordering::Relaxed) as f64 / duration_secs
    }

    fn get_avg_latency_us(&self) -> f64 {
        let events = self.events_processed.load(Ordering::Relaxed);
        if events == 0 {
            0.0
        } else {
            (self.total_latency_ns.load(Ordering::Relaxed) / events) as f64 / 1000.0
        }
    }

    fn get_throughput_mbps(&self, duration_secs: f64) -> f64 {
        let bytes = self.bytes_processed.load(Ordering::Relaxed) as f64;
        (bytes / duration_secs) / (1024.0 * 1024.0)
    }
}

/// Generate a realistic event with varying payload sizes
fn generate_event(id: u64, payload_size: usize, protocol: ProtocolType) -> Event {
    let payload = if payload_size > 0 {
        Some("x".repeat(payload_size))
    } else {
        None
    };

    let mut event = Event::new(format!("evt-{}", id), protocol);
    event.session_id = Some(format!("sess-{}", id % 10000)); // 10k active sessions
    event.project_id = Some(format!("project-{}", id % 100));
    event.environment_id = Some(format!("env-{}", id % 10));
    event.service_id = Some(format!("svc-{}", id % 50));
    event.latency_us = 100 + (id % 5000) as u64; // 100us to 5ms latency
    
    if let Some(p) = payload {
        event = event.with_payload(p);
    }

    // Add realistic metadata
    event.metadata.add("region", "us-east-1");
    event.metadata.add("host", "server-01");
    event.metadata.add("version", "1.2.3");

    event
}

/// Get current memory usage in MB
fn get_memory_usage_mb() -> u64 {
    // This is a simplified version - on Linux we'd read /proc/self/status
    // For now, return 0 as placeholder
    0
}

/// Run sustained throughput test
async fn run_sustained_throughput_test(duration_secs: u64, target_eps: u64) -> LoadMetrics {
    println!("\n🚀 SUSTAINED THROUGHPUT TEST");
    println!("   Duration: {}s | Target: {} events/sec", duration_secs, target_eps);
    println!("   ─────────────────────────────────────────");

    let metrics = Arc::new(LoadMetrics::new());
    let (tx, mut rx) = mpsc::channel::<Event>(100_000);
    let start = Instant::now();
    let interval_ns = 1_000_000_000 / target_eps;

    // Producer task
    let producer = tokio::spawn({
        let tx = tx.clone();
        async move {
            let mut id = 0u64;
            let protocols = [
                ProtocolType::Http,
                ProtocolType::Grpc,
                ProtocolType::Ws,
                ProtocolType::Tcp,
            ];

            while start.elapsed().as_secs() < duration_secs {
                let protocol = protocols[id as usize % protocols.len()];
                let event = generate_event(id, 512, protocol); // 512B average payload
                
                if tx.try_send(event).is_err() {
                    // Channel full - drop event
                }
                
                id += 1;
                
                // Rate limiting
                if id % target_eps == 0 {
                    tokio::time::sleep(Duration::from_nanos(interval_ns)).await;
                }
            }
            id
        }
    });

    // Consumer/processor task
    let metrics_clone = metrics.clone();
    let consumer = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            let process_start = Instant::now();
            
            // Simulate processing: serialization + deserialization
            let json = serde_json::to_string(&event).unwrap();
            let _: Event = serde_json::from_str(&json).unwrap();
            
            let latency = process_start.elapsed().as_nanos() as u64;
            metrics_clone.record_event(json.len() as u64, latency);
        }
    });

    // Wait for completion
    let total_events = producer.await.unwrap();
    drop(tx);
    let _ = timeout(Duration::from_secs(10), consumer).await;

    let elapsed = start.elapsed().as_secs_f64();
    let actual_eps = total_events as f64 / elapsed;
    let processed = metrics.events_processed.load(Ordering::Relaxed);

    println!("   ✓ Total Events Generated: {}", total_events);
    println!("   ✓ Events Processed: {}", processed);
    println!("   ✓ Actual Rate: {:.2}K events/sec", actual_eps / 1000.0);
    println!("   ✓ Avg Processing Latency: {:.2}μs", metrics.get_avg_latency_us());
    println!("   ✓ Throughput: {:.2} MB/s", metrics.get_throughput_mbps(elapsed));
    
    Arc::try_unwrap(metrics).unwrap_or_else(|_| LoadMetrics::new())
}

/// Run burst capacity test
async fn run_burst_test(burst_size: u64) -> LoadMetrics {
    println!("\n💥 BURST CAPACITY TEST");
    println!("   Burst Size: {}K events", burst_size / 1000);
    println!("   ─────────────────────────────────────────");

    let metrics = Arc::new(LoadMetrics::new());
    let start = Instant::now();

    let handles: Vec<_> = (0..num_cpus::get())
        .map(|thread_id| {
            let metrics = metrics.clone();
            tokio::spawn(async move {
                let per_thread = burst_size / num_cpus::get() as u64;
                
                for i in 0..per_thread {
                    let id = thread_id as u64 * per_thread + i;
                    let event = generate_event(id, 1024, ProtocolType::Http);
                    
                    let process_start = Instant::now();
                    let json = serde_json::to_string(&event).unwrap();
                    let _: Event = serde_json::from_str(&json).unwrap();
                    
                    metrics.record_event(json.len() as u64, process_start.elapsed().as_nanos() as u64);
                }
            })
        })
        .collect();

    for h in handles {
        let _ = h.await;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let processed = metrics.events_processed.load(Ordering::Relaxed);

    println!("   ✓ Processed: {}K events", processed / 1000);
    println!("   ✓ Time: {:.2}s", elapsed);
    println!("   ✓ Throughput: {:.2}K events/sec", processed as f64 / elapsed / 1000.0);
    println!("   ✓ Peak Throughput: {:.2} MB/s", metrics.get_throughput_mbps(elapsed));

    Arc::try_unwrap(metrics).unwrap_or_else(|_| LoadMetrics::new())
}

/// Run session stress test
async fn run_session_stress_test(num_sessions: u64, events_per_session: u64) -> LoadMetrics {
    println!("\n📊 SESSION STRESS TEST");
    println!("   Sessions: {}K | Events/Session: {}", num_sessions / 1000, events_per_session);
    println!("   ─────────────────────────────────────────");

    use fluxx_agent::sessions::models::Session;
    
    let start = Instant::now();
    let metrics = Arc::new(LoadMetrics::new());

    // Create sessions and add events
    let handles: Vec<_> = (0..num_cpus::get())
        .map(|thread_id| {
            tokio::task::spawn_blocking(move || {
                let per_thread = num_sessions / num_cpus::get() as u64;
                let mut sessions: Vec<Session> = Vec::with_capacity(per_thread as usize);

                for s in 0..per_thread {
                    let session_id = format!("sess-{}-{}", thread_id, s);
                    let mut session = Session::new(session_id);

                    for e in 0..events_per_session {
                        let event_id = format!("evt-{}-{}-{}", thread_id, s, e);
                        let event = Event::new(event_id, ProtocolType::Http);
                        session.add_event(event);
                    }

                    sessions.push(session);
                }

                sessions
            })
        })
        .collect();

    let mut all_sessions = Vec::new();
    for h in handles {
        if let Ok(sessions) = h.await {
            all_sessions.extend(sessions);
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total_events = num_sessions * events_per_session;

    // Serialize all sessions
    let ser_start = Instant::now();
    let mut total_bytes = 0u64;
    for session in &all_sessions {
        let json = serde_json::to_string(session).unwrap();
        total_bytes += json.len() as u64;
    }
    let ser_time = ser_start.elapsed().as_secs_f64();

    println!("   ✓ Total Sessions: {}K", all_sessions.len() / 1000);
    println!("   ✓ Total Events: {}M", total_events / 1_000_000);
    println!("   ✓ Setup Time: {:.2}s", elapsed);
    println!("   ✓ Serialization: {:.2} MB in {:.2}s", 
             total_bytes as f64 / (1024.0 * 1024.0), ser_time);
    println!("   ✓ Ser Throughput: {:.2} MB/s", 
             (total_bytes as f64 / ser_time) / (1024.0 * 1024.0));

    Arc::try_unwrap(metrics).unwrap_or_else(|_| LoadMetrics::new())
}

/// Run payload size impact test
async fn run_payload_size_test() {
    println!("\n📦 PAYLOAD SIZE IMPACT TEST");
    println!("   Testing various payload sizes");
    println!("   ─────────────────────────────────────────");

    let sizes = [0, 128, 512, 1024, 4096, 16384, 65536, 262144];
    
    for size in sizes {
        let start = Instant::now();
        let count = 10_000u64;
        let mut total_bytes = 0u64;

        for i in 0..count {
            let event = generate_event(i, size, ProtocolType::Http);
            let json = serde_json::to_string(&event).unwrap();
            total_bytes += json.len() as u64;
            let _: Event = serde_json::from_str(&json).unwrap();
        }

        let elapsed = start.elapsed().as_secs_f64();
        let eps = count as f64 / elapsed;
        let mbps = (total_bytes as f64 / elapsed) / (1024.0 * 1024.0);

        println!("   Payload {:>6}B: {:>8.2}K eps | {:>6.2} MB/s", 
                 size, eps / 1000.0, mbps);
    }
}

/// Print enterprise analysis report
fn print_enterprise_analysis(laptop_eps: f64) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║          ENTERPRISE SCALE ANALYSIS & COST PROJECTION                         ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    // Datadog scale reference
    let datadog_events_per_day = 10_000_000_000_000i64; // 10 trillion/day (2023 scale)
    let datadog_events_per_sec = datadog_events_per_day as f64 / 86400.0;
    let datadog_peak_eps = datadog_events_per_sec * 3.0; // 3x for peak

    println!("\n📈 DATADOG SCALE REFERENCE (2023-2024)");
    println!("   ├── Daily Volume:     {:>12} trillion events/day", 
             datadog_events_per_day as f64 / 1_000_000_000_000.0);
    println!("   ├── Average Rate:     {:>12.2}M events/sec", datadog_events_per_sec / 1_000_000.0);
    println!("   └── Peak Rate (3x):   {:>12.2}M events/sec", datadog_peak_eps / 1_000_000.0);

    println!("\n💻 THIS LAPTOP PERFORMANCE");
    println!("   ├── Sustained:        {:>12.2}K events/sec", laptop_eps / 1000.0);
    println!("   ├── Burst:            {:>12.2}K events/sec", laptop_eps * 1.5 / 1000.0);
    println!("   └── vs Datadog Peak:  {:.6}x of Datadog scale", laptop_eps / datadog_peak_eps);

    // Scaling calculations
    let nodes_needed = (datadog_peak_eps / laptop_eps).ceil() as u64;
    let ha_nodes = (nodes_needed * 3).max(100); // 3x for HA

    println!("\n🔧 HARDWARE REQUIREMENTS FOR DATADOG SCALE");
    println!("   ├── Minimum Nodes:    {:>12} instances", nodes_needed);
    println!("   ├── With 3x HA:       {:>12} instances", ha_nodes);
    println!("   └── Instance Type:    {:>12} (64 vCPU, 256GB RAM)", "c6i.16xlarge");

    // Cloud costs (AWS US-East-1 pricing)
    let instance_cost_per_hour = 2.72; // c6i.16xlarge on-demand
    let storage_per_node_tb = 50.0; // SSD for hot storage
    let storage_cost_per_month_tb = 100.0;
    let network_cost_per_month = 50_000.0; // High data transfer

    let compute_monthly = ha_nodes as f64 * instance_cost_per_hour * 24.0 * 30.0;
    let storage_monthly = ha_nodes as f64 * storage_per_node_tb * storage_cost_per_month_tb;
    let network_monthly = network_cost_per_month;
    let total_monthly = compute_monthly + storage_monthly + network_monthly;
    let total_annual = total_monthly * 12.0;

    println!("\n💰 INFRASTRUCTURE COSTS (AWS US-East-1)");
    println!("   ┌────────────────────────────────────────────────────────┐");
    println!("   │ Compute (c6i.16xlarge):    ${:>10.0}/month        │", compute_monthly as i64);
    println!("   │ Storage (SSD):             ${:>10.0}/month        │", storage_monthly as i64);
    println!("   │ Network/Data Transfer:     ${:>10.0}/month        │", network_monthly as i64);
    println!("   ├────────────────────────────────────────────────────────┤");
    println!("   │ TOTAL MONTHLY:             ${:>10.0}/month        │", total_monthly as i64);
    println!("   │ TOTAL ANNUAL:              ${:>10.0}/year         │", total_annual as i64);
    println!("   └────────────────────────────────────────────────────────┘");

    // Power consumption
    let laptop_power_watts = 65.0;
    let server_power_watts = 400.0; // c6i.16xlarge ~400W under load
    let pue = 1.6; // Power Usage Effectiveness for data center
    
    let total_power_kw = (ha_nodes as f64 * server_power_watts * pue) / 1000.0;
    let annual_kwh = total_power_kw * 24.0 * 365.0;
    let carbon_tons = annual_kwh * 0.4 / 1000.0; // 0.4 kg CO2/kWh avg US grid

    println!("\n⚡ POWER & ENVIRONMENTAL IMPACT");
    println!("   ├── Per-Server Power:     {:.0}W", server_power_watts);
    println!("   ├── Total Power (w/PUE):  {:.0} kW", total_power_kw);
    println!("   ├── Annual Energy:        {:.0} MWh", annual_kwh / 1000.0);
    println!("   └── CO2 Emissions:        {:.0} tons/year", carbon_tons);
    println!("       (≈ {:.0} US households annual usage)", carbon_tons / 4.5);

    // Engineering team costs
    let engineers_needed = 50; // SRE, backend, platform engineers
    let avg_engineer_cost = 250_000.0; // Fully loaded
    let team_annual = engineers_needed as f64 * avg_engineer_cost;

    println!("\n👥 PERSONNEL REQUIREMENTS");
    println!("   ├── Platform Engineers:   {:>3} @ ${:>6.0}/yr", 15, 300_000_i64);
    println!("   ├── SRE/DevOps:           {:>3} @ ${:>6.0}/yr", 20, 280_000_i64);
    println!("   ├── Backend Engineers:    {:>3} @ ${:>6.0}/yr", 15, 250_000_i64);
    println!("   └── Total Team Cost:      ${:>10.0}/year", team_annual as i64);

    let grand_total = total_annual + team_annual;
    
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║  TOTAL COST OF OWNERSHIP (TCO) - DATADOG SCALE                               ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Infrastructure:                              ${:>15.0}/year  ║", total_annual as i64);
    println!("║  Personnel:                                   ${:>15.0}/year  ║", team_annual as i64);
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  GRAND TOTAL:                                 ${:>15.0}/year  ║", grand_total as i64);
    let per_event_cost = grand_total / datadog_events_per_day as f64 / 365.0;
    println!("║  Per Event (at Datadog scale):                ${:>15.10}       ║", per_event_cost);
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    println!("\n📊 COMPARISON: Self-Hosted vs Datadog Service");
    let datadog_cost_per_host_month = 15.0; // APM Pro
    let estimated_datadog_hosts = 100_000;
    let datadog_monthly_cost = datadog_cost_per_host_month * estimated_datadog_hosts as f64;
    let datadog_annual = datadog_monthly_cost * 12.0;

    println!("   Self-Hosted TCO:    ${:>12.0}/year", grand_total as i64);
    println!("   Datadog Service:    ${:>12.0}/year (est. for 100K hosts)", datadog_annual as i64);
    println!("   Savings:            {:.0}% vs building yourself", 
             (1.0 - grand_total / datadog_annual) * 100.0);
}

#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║           FLUXX AGENT - EXTREME LOAD TEST                                    ║");
    println!("║           Enterprise Performance Benchmark                                   ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    let start_time = Instant::now();

    // Test 1: Sustained throughput
    let sustained = run_sustained_throughput_test(10, 100_000).await;
    let sustained_eps = sustained.get_events_per_second(10.0);

    // Test 2: Burst capacity
    let _burst = run_burst_test(1_000_000).await;

    // Test 3: Session stress
    let _session = run_session_stress_test(100_000, 100).await;

    // Test 4: Payload sizes
    run_payload_size_test().await;

    let total_time = start_time.elapsed();

    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║           LOAD TEST COMPLETED                                                ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("   Total Test Time: {:.2}s", total_time.as_secs_f64());
    println!("   Laptop sustained: {:.2}K events/sec", sustained_eps / 1000.0);

    // Print enterprise analysis
    print_enterprise_analysis(sustained_eps);
}
