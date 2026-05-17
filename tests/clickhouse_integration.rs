//! End-to-end integration tests for the ClickHouse error-event sink.
//!
//! Spawns a real `clickhouse/clickhouse-server` container via
//! testcontainers, applies the embedded migrations, and exercises the
//! ingest + replay round-trip.
//!
//! Every test is `#[ignore]` so the default `cargo test` runs stay
//! fast and don't require Docker. Run with:
//!
//!   cargo test --test clickhouse_integration -- --ignored
//!
//! Or one at a time:
//!
//!   cargo test --test clickhouse_integration -- --ignored sink_ingests_and_persists_rows

use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use fluxx_agent::storage::{
    fetch_recent_groups, ClickHouseConfig, ClickHouseErrorEventSink, ClickHouseErrorReadStore,
    ErrorEventRow, ErrorEventSink, ErrorReadStore,
};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage,
};

/// Start a fresh ClickHouse container. The container handle is returned
/// alongside the config so callers can keep it alive for the test
/// duration (drop = stop). One container per test for isolation.
async fn start_clickhouse() -> (ClickHouseConfig, ContainerAsync<GenericImage>) {
    let container = GenericImage::new("clickhouse/clickhouse-server", "24.8")
        .with_exposed_port(8123.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready for connections"))
        .start()
        .await
        .expect("clickhouse container should start (is Docker running?)");

    let port = container
        .get_host_port_ipv4(8123.tcp())
        .await
        .expect("host port mapping");

    let cfg = ClickHouseConfig {
        host: "127.0.0.1".into(),
        port,
        database: "fluxx".into(),
        username: None,
        password: None,
        flush_interval: Duration::from_millis(500),
        flush_batch_size: 100,
        queue_capacity: 10_000,
        http_timeout: Duration::from_secs(10),
    };

    // The "Ready for connections" log line precedes the HTTP listener
    // by a hair on some CH versions. Confirm the HTTP endpoint really
    // answers before handing back to the test.
    poll_until_http_ready(&cfg.host, cfg.port).await;

    (cfg, container)
}

async fn poll_until_http_ready(host: &str, port: u16) {
    let url = format!("http://{}:{}/?query=SELECT%201", host, port);
    let client = reqwest::Client::new();
    for _ in 0..40 {
        let ok = client
            .get(&url)
            .timeout(Duration::from_millis(500))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);
        if ok {
            return;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    panic!("clickhouse never became HTTP-ready at {}:{}", host, port);
}

/// Issue a raw count query against ClickHouse and parse the single-cell
/// TSV response back as `u64`. Used to verify row counts independent of
/// the sink's own stats counters.
async fn ch_count(cfg: &ClickHouseConfig, sql: &str) -> u64 {
    let url = format!(
        "http://{}:{}/?database={}&query={}",
        cfg.host,
        cfg.port,
        cfg.database,
        urlencoding::encode(&format!("{} FORMAT TSV", sql))
    );
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
    body.trim()
        .parse()
        .unwrap_or_else(|e| panic!("ch_count expected u64 from `{}`, got `{}`: {}", sql, body, e))
}

fn sample_row(fingerprint: &str, msg: &str, release: &str, service: &str) -> ErrorEventRow {
    ErrorEventRow {
        event_id: uuid::Uuid::new_v4().to_string(),
        org_id: "org_test".into(),
        project_id: "proj_test".into(),
        service: service.into(),
        ts: Utc::now(),
        fingerprint: fingerprint.into(),
        error_type: "TypeError".into(),
        message: msg.into(),
        release: release.into(),
        environment: "test".into(),
        stack_raw: String::new(),
        frames_function: vec![],
        frames_file: vec![],
        frames_line: vec![],
        frames_column: vec![],
        frames_resolved: 0,
        frames_context: vec![],
        breadcrumbs: String::new(),
        session_id: String::new(),
        user_id: String::new(),
        url: String::new(),
        user_agent: String::new(),
        tags: HashMap::new(),
    }
}

#[tokio::test]
#[ignore]
async fn migrations_create_expected_tables() {
    let (cfg, _ch) = start_clickhouse().await;
    let _sink = ClickHouseErrorEventSink::open(cfg.clone())
        .await
        .expect("sink should open against a fresh CH");

    let n = ch_count(
        &cfg,
        "SELECT count() FROM system.tables WHERE database='fluxx'",
    )
    .await;
    // 3 base tables (error_events, error_groups, error_group_by_release)
    // + 2 materialized views = 5
    assert!(n >= 5, "expected at least 5 fluxx.* tables, got {}", n);
}

#[tokio::test]
#[ignore]
async fn idempotent_migrations_can_run_twice() {
    let (cfg, _ch) = start_clickhouse().await;
    let _s1 = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();
    // Second open re-runs DDL — `IF NOT EXISTS` makes it a no-op.
    let _s2 = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    let n = ch_count(
        &cfg,
        "SELECT count() FROM system.tables WHERE database='fluxx'",
    )
    .await;
    assert!(n >= 5);
}

#[tokio::test]
#[ignore]
async fn sink_ingests_and_persists_rows() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    // Two fingerprints, 250 rows total. Spans multiple batches given
    // flush_batch_size=100, exercising the drain loop's batch logic.
    for i in 0..200 {
        sink.enqueue(sample_row(
            "aaaaaaaaaaaaaaaa",
            &format!("boom {}", i),
            "1.0.0",
            "api",
        ));
    }
    for i in 0..50 {
        sink.enqueue(sample_row(
            "bbbbbbbbbbbbbbbb",
            &format!("crash {}", i),
            "1.0.0",
            "worker",
        ));
    }

    let flushed = sink.flush().await.expect("explicit flush should succeed");
    // Some rows may have been flushed by the drain loop's batch trigger
    // before we called flush(); the explicit flush handles the remainder.
    // Together with previously-flushed batches, total should be 250.
    assert!(flushed <= 250, "flushed count cannot exceed total enqueued");

    let count = ch_count(&cfg, "SELECT count() FROM fluxx.error_events").await;
    assert_eq!(count, 250, "ClickHouse should hold exactly 250 rows");

    let stats = sink.stats();
    assert_eq!(stats.enqueued, 250);
    assert_eq!(stats.dropped, 0, "no rows dropped at this volume");
    assert_eq!(stats.flush_errors, 0);
}

#[tokio::test]
#[ignore]
async fn replay_returns_groups_seeded_from_ingest() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    sink.enqueue(sample_row("cccccccccccccccc", "A boom", "1.0.0", "api"));
    sink.enqueue(sample_row("cccccccccccccccc", "A boom", "1.0.0", "api"));
    sink.enqueue(sample_row("dddddddddddddddd", "B boom", "1.0.1", "worker"));
    sink.flush().await.unwrap();

    let groups = fetch_recent_groups(&cfg, 1)
        .await
        .expect("replay query should succeed");
    assert_eq!(
        groups.len(),
        2,
        "two distinct fingerprints expected, got {:?}",
        groups.iter().map(|g| &g.fingerprint).collect::<Vec<_>>()
    );

    let g_c = groups
        .iter()
        .find(|g| g.fingerprint == "cccccccccccccccc")
        .expect("c group present");
    assert_eq!(g_c.count, 2);
    assert_eq!(g_c.error_type, "TypeError");
    assert!(g_c.services.iter().any(|s| s == "api"));

    let g_d = groups
        .iter()
        .find(|g| g.fingerprint == "dddddddddddddddd")
        .expect("d group present");
    assert_eq!(g_d.count, 1);
}

#[tokio::test]
#[ignore]
async fn sink_open_fails_on_unreachable_clickhouse() {
    // No container — point at a port nothing is listening on. Open must
    // surface the connect failure as an error, not panic or hang.
    let cfg = ClickHouseConfig {
        host: "127.0.0.1".into(),
        port: 1, // privileged + nothing here
        database: "fluxx".into(),
        username: None,
        password: None,
        flush_interval: Duration::from_secs(1),
        flush_batch_size: 100,
        queue_capacity: 100,
        http_timeout: Duration::from_millis(500),
    };
    let err = ClickHouseErrorEventSink::open(cfg).await;
    assert!(err.is_err(), "open against dead CH must fail");
}

#[tokio::test]
#[ignore]
async fn read_store_list_groups_returns_ingested_data() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    for _ in 0..3 {
        sink.enqueue(sample_row("1111111111111111", "x", "1.0.0", "api"));
    }
    sink.enqueue(sample_row("2222222222222222", "y", "1.0.0", "worker"));
    sink.flush().await.unwrap();

    let store = ClickHouseErrorReadStore::new(cfg.clone());
    let groups = store.list_groups(24).await.expect("list_groups ok");

    assert_eq!(groups.len(), 2);
    let g1 = groups.iter().find(|g| g.fingerprint == "1111111111111111").unwrap();
    assert_eq!(g1.count, 3);
    let g2 = groups.iter().find(|g| g.fingerprint == "2222222222222222").unwrap();
    assert_eq!(g2.count, 1);
}

#[tokio::test]
#[ignore]
async fn read_store_get_group_returns_specific_fingerprint() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    sink.enqueue(sample_row("3333333333333333", "needle", "1.0.0", "api"));
    sink.enqueue(sample_row("4444444444444444", "haystack", "1.0.0", "api"));
    sink.flush().await.unwrap();

    let store = ClickHouseErrorReadStore::new(cfg);
    let g = store.get_group("3333333333333333", 24).await.unwrap();
    assert!(g.is_some());
    assert_eq!(g.unwrap().message, "needle");

    let missing = store.get_group("9999999999999999", 24).await.unwrap();
    assert!(missing.is_none(), "absent fingerprint should yield None");
}

#[tokio::test]
#[ignore]
async fn read_store_get_group_rejects_invalid_fingerprint() {
    let (cfg, _ch) = start_clickhouse().await;
    let _sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();
    let store = ClickHouseErrorReadStore::new(cfg);

    // SQL-injection-shaped string must be rejected by the validator
    // before reaching ClickHouse. Returns Ok(None), not an error.
    let g = store
        .get_group("'; DROP TABLE error_events;--", 24)
        .await
        .unwrap();
    assert!(g.is_none());

    // Too short
    let g = store.get_group("abcd", 24).await.unwrap();
    assert!(g.is_none());

    // Non-hex char
    let g = store.get_group("zzzzzzzzzzzzzzzz", 24).await.unwrap();
    assert!(g.is_none());
}

#[tokio::test]
#[ignore]
async fn read_store_top_groups_orders_by_count_desc() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    for _ in 0..5 { sink.enqueue(sample_row("5555555555555555", "a", "1.0.0", "svc")); }
    for _ in 0..20 { sink.enqueue(sample_row("6666666666666666", "b", "1.0.0", "svc")); }
    for _ in 0..1 { sink.enqueue(sample_row("7777777777777777", "c", "1.0.0", "svc")); }
    sink.flush().await.unwrap();

    let store = ClickHouseErrorReadStore::new(cfg);
    let top = store.top_groups(2, 24).await.unwrap();
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].fingerprint, "6666666666666666"); // 20
    assert_eq!(top[0].count, 20);
    assert_eq!(top[1].fingerprint, "5555555555555555"); // 5
}

#[tokio::test]
#[ignore]
async fn read_store_trend_returns_24_hour_buckets() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    for _ in 0..7 {
        sink.enqueue(sample_row("8888888888888888", "z", "1.0.0", "svc"));
    }
    sink.flush().await.unwrap();

    let store = ClickHouseErrorReadStore::new(cfg);
    let trend = store.trend(24).await.unwrap();
    let total: u64 = trend.iter().sum();
    assert_eq!(total, 7, "trend bucket sum should equal event count");
    // Exactly one bucket should be non-zero (events all in the same hour-of-day).
    let nonzero = trend.iter().filter(|&&n| n > 0).count();
    assert_eq!(nonzero, 1, "expected events in exactly one hour bucket");
}

#[tokio::test]
#[ignore]
async fn read_store_stats_reports_group_and_occurrence_counts() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    for _ in 0..3 { sink.enqueue(sample_row("a000000000000000", "a", "1.0.0", "svc")); }
    for _ in 0..2 { sink.enqueue(sample_row("b000000000000000", "b", "1.0.0", "svc")); }
    sink.flush().await.unwrap();

    // AggregatingMergeTree merges on background; force-merge so stats
    // queries against the mat view see a consistent state.
    let merge_url = format!(
        "http://{}:{}/?query={}",
        cfg.host,
        cfg.port,
        urlencoding::encode("OPTIMIZE TABLE fluxx.error_groups FINAL")
    );
    let _ = reqwest::Client::new().post(&merge_url).send().await;

    let store = ClickHouseErrorReadStore::new(cfg);
    let s = store.stats().await.unwrap();
    assert!(s.total_groups >= 2);
    assert!(s.total_occurrences >= 5);
}

#[tokio::test]
#[ignore]
async fn materialized_views_aggregate_into_error_groups() {
    let (cfg, _ch) = start_clickhouse().await;
    let sink = ClickHouseErrorEventSink::open(cfg.clone()).await.unwrap();

    for _ in 0..15 {
        sink.enqueue(sample_row("ffffffffffffffff", "x", "2.0.0", "api"));
    }
    for _ in 0..5 {
        sink.enqueue(sample_row("ffffffffffffffff", "x", "2.0.1", "api"));
    }
    sink.flush().await.unwrap();

    // The materialized view is populated as inserts land. Give CH a
    // moment to merge parts so the AggregatingMergeTree query is stable.
    tokio::time::sleep(Duration::from_millis(500)).await;

    // `count` is SimpleAggregateFunction(sum, UInt64) — read with sum(),
    // not countMerge() (which is for AggregateFunction(count, ...)).
    let count_groups = ch_count(
        &cfg,
        "SELECT sum(count) FROM fluxx.error_groups WHERE fingerprint = 'ffffffffffffffff'",
    )
    .await;
    assert_eq!(count_groups, 20, "mv should sum all 20 events for one fingerprint");

    let releases = ch_count(
        &cfg,
        "SELECT uniqExact(release) FROM fluxx.error_group_by_release \
         WHERE fingerprint = 'ffffffffffffffff'",
    )
    .await;
    assert_eq!(releases, 2, "two distinct releases expected");
}
