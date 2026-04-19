//! Cost Analytics Engine
//!
//! Calculates what the user's current observability usage would cost on Datadog,
//! New Relic, Splunk, and Grafana Cloud — then shows the savings with Fluxx.

use crate::api::{ApiState, StandardResponse};
use axum::{extract::State, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;

/// Pricing models for major vendors (per month).
#[derive(Debug, Clone, Serialize)]
pub struct VendorPricing {
    pub vendor: String,
    pub infrastructure_per_host: f64,
    pub apm_per_host: f64,
    pub logs_per_gb: f64,
    pub custom_metrics_per_metric: f64,
    pub synthetics_per_test: f64,
    pub rum_per_1k_sessions: f64,
    pub npm_per_host: f64,
    pub dbm_per_host: f64,
    pub cspm_per_host: f64,
    pub profiling_per_host: f64,
    pub ci_per_committer: f64,
}

fn datadog_pricing() -> VendorPricing {
    VendorPricing {
        vendor: "Datadog".into(), infrastructure_per_host: 15.0, apm_per_host: 31.0,
        logs_per_gb: 0.10, custom_metrics_per_metric: 0.05, synthetics_per_test: 5.0,
        rum_per_1k_sessions: 1.50, npm_per_host: 5.0, dbm_per_host: 70.0,
        cspm_per_host: 7.50, profiling_per_host: 12.0, ci_per_committer: 13.0,
    }
}

fn newrelic_pricing() -> VendorPricing {
    VendorPricing {
        vendor: "New Relic".into(), infrastructure_per_host: 0.0, apm_per_host: 0.0,
        logs_per_gb: 0.30, custom_metrics_per_metric: 0.0,  synthetics_per_test: 0.0,
        rum_per_1k_sessions: 0.0, npm_per_host: 0.0, dbm_per_host: 0.0,
        cspm_per_host: 0.0, profiling_per_host: 0.0, ci_per_committer: 0.0,
        // New Relic charges per user: $49-$549/user/month (simplified to per-GB data)
    }
}

fn splunk_pricing() -> VendorPricing {
    VendorPricing {
        vendor: "Splunk".into(), infrastructure_per_host: 15.0, apm_per_host: 55.0,
        logs_per_gb: 2.0, custom_metrics_per_metric: 0.0, synthetics_per_test: 2.0,
        rum_per_1k_sessions: 0.0, npm_per_host: 0.0, dbm_per_host: 0.0,
        cspm_per_host: 0.0, profiling_per_host: 0.0, ci_per_committer: 0.0,
    }
}

fn grafana_cloud_pricing() -> VendorPricing {
    VendorPricing {
        vendor: "Grafana Cloud".into(), infrastructure_per_host: 0.0, apm_per_host: 0.0,
        logs_per_gb: 0.50, custom_metrics_per_metric: 8.0, // per 1K series
        synthetics_per_test: 3.0, rum_per_1k_sessions: 0.0, npm_per_host: 0.0,
        dbm_per_host: 0.0, cspm_per_host: 0.0, profiling_per_host: 0.0,
        ci_per_committer: 0.0,
    }
}

/// Usage metrics from Fluxx stores.
#[derive(Debug, Clone, Serialize)]
pub struct UsageMetrics {
    pub hosts: u64,
    pub apm_hosts: u64,
    pub log_gb_per_month: f64,
    pub custom_metrics: u64,
    pub synthetic_checks: u64,
    pub rum_sessions_per_month: u64,
    pub npm_hosts: u64,
    pub dbm_hosts: u64,
    pub cspm_hosts: u64,
    pub profiling_hosts: u64,
    pub ci_committers: u64,
}

/// Cost comparison across vendors.
#[derive(Debug, Clone, Serialize)]
pub struct CostComparison {
    pub usage: UsageMetrics,
    pub vendors: Vec<VendorCost>,
    pub fluxx_cost: f64,
    pub max_savings_vendor: String,
    pub max_savings_amount: f64,
    pub max_savings_percent: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct VendorCost {
    pub vendor: String,
    pub monthly_cost: f64,
    pub annual_cost: f64,
    pub breakdown: CostBreakdown,
    pub savings_vs_fluxx: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CostBreakdown {
    pub infrastructure: f64,
    pub apm: f64,
    pub logs: f64,
    pub metrics: f64,
    pub synthetics: f64,
    pub rum: f64,
    pub npm: f64,
    pub dbm: f64,
    pub cspm: f64,
    pub profiling: f64,
    pub ci: f64,
}

fn compute_vendor_cost(usage: &UsageMetrics, pricing: &VendorPricing) -> VendorCost {
    let breakdown = CostBreakdown {
        infrastructure: usage.hosts as f64 * pricing.infrastructure_per_host,
        apm: usage.apm_hosts as f64 * pricing.apm_per_host,
        logs: usage.log_gb_per_month * pricing.logs_per_gb,
        metrics: usage.custom_metrics as f64 * pricing.custom_metrics_per_metric,
        synthetics: usage.synthetic_checks as f64 * pricing.synthetics_per_test,
        rum: (usage.rum_sessions_per_month as f64 / 1000.0) * pricing.rum_per_1k_sessions,
        npm: usage.npm_hosts as f64 * pricing.npm_per_host,
        dbm: usage.dbm_hosts as f64 * pricing.dbm_per_host,
        cspm: usage.cspm_hosts as f64 * pricing.cspm_per_host,
        profiling: usage.profiling_hosts as f64 * pricing.profiling_per_host,
        ci: usage.ci_committers as f64 * pricing.ci_per_committer,
    };
    let monthly = breakdown.infrastructure + breakdown.apm + breakdown.logs + breakdown.metrics
        + breakdown.synthetics + breakdown.rum + breakdown.npm + breakdown.dbm
        + breakdown.cspm + breakdown.profiling + breakdown.ci;

    VendorCost {
        vendor: pricing.vendor.clone(),
        monthly_cost: monthly,
        annual_cost: monthly * 12.0,
        breakdown,
        savings_vs_fluxx: monthly, // Fluxx is $0
    }
}

/// GET /v1/cost — Cost comparison based on current usage.
pub async fn cost_analysis(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    // Gather usage from stores
    let metric_count = state.metric_store.get_names().await.map(|n| n.len() as u64).unwrap_or(0);
    let log_events = state.log_store.len().await.unwrap_or(0) as u64;
    let rum_sessions = state.rum_store.total_sessions() as u64;

    let usage = UsageMetrics {
        hosts: 1, // Self-reported, would come from fleet info
        apm_hosts: 1,
        log_gb_per_month: (log_events as f64 * 500.0) / (1024.0 * 1024.0 * 1024.0), // ~500 bytes per log
        custom_metrics: metric_count,
        synthetic_checks: 0,
        rum_sessions_per_month: rum_sessions,
        npm_hosts: 1,
        dbm_hosts: 1,
        cspm_hosts: 1,
        profiling_hosts: 1,
        ci_committers: 5,
    };

    let vendors = vec![
        compute_vendor_cost(&usage, &datadog_pricing()),
        compute_vendor_cost(&usage, &newrelic_pricing()),
        compute_vendor_cost(&usage, &splunk_pricing()),
        compute_vendor_cost(&usage, &grafana_cloud_pricing()),
    ];

    let max_cost = vendors.iter().map(|v| v.monthly_cost).fold(0.0f64, f64::max);
    let max_vendor = vendors.iter().max_by(|a, b| a.monthly_cost.partial_cmp(&b.monthly_cost).unwrap())
        .map(|v| v.vendor.clone()).unwrap_or_default();

    let comparison = CostComparison {
        usage,
        fluxx_cost: 0.0,
        max_savings_vendor: max_vendor,
        max_savings_amount: max_cost,
        max_savings_percent: 100.0,
        vendors,
    };

    let data = serde_json::to_value(&comparison).unwrap_or_default();
    StandardResponse::ok(data, "Cost comparison.")
}
