terraform {
  required_providers {
    fluxx = {
      source  = "fluxx/fluxx"
      version = "~> 1.0"
    }
  }
}

provider "fluxx" {
  endpoint = "http://localhost:7007"
  api_key  = var.fluxx_api_key
}

variable "fluxx_api_key" {
  type      = string
  sensitive = true
  default   = ""
}

# ── Alert Rules ──

resource "fluxx_alert_rule" "high_cpu" {
  name              = "high-cpu-usage"
  condition         = "cpu.usage > 80"
  severity          = "warning"
  channels          = ["slack-ops"]
  cooldown_seconds  = 300
}

resource "fluxx_alert_rule" "high_error_rate" {
  name              = "high-error-rate"
  condition         = "error.rate > 0.05"
  severity          = "critical"
  channels          = ["slack-ops", "pagerduty"]
  cooldown_seconds  = 60
}

resource "fluxx_alert_rule" "high_latency" {
  name              = "high-p99-latency"
  condition         = "latency.p99 > 500"
  severity          = "warning"
  channels          = ["slack-ops"]
  cooldown_seconds  = 300
}

# ── SLOs ──

resource "fluxx_slo" "api_availability" {
  name            = "api-availability"
  description     = "API gateway 99.9% availability"
  target_percent  = 99.9
  window_days     = 30
  total_query     = "sum(http.requests.total)"
  good_query      = "sum(http.requests.total{status!~\"5..\"})}"
  service         = "api-gateway"
}

resource "fluxx_slo" "checkout_latency" {
  name            = "checkout-latency"
  description     = "Checkout P99 under 2 seconds"
  target_percent  = 99.0
  window_days     = 7
  total_query     = "count(checkout.duration)"
  good_query      = "count(checkout.duration < 2000)"
  service         = "checkout-service"
}

# ── Synthetic Checks ──

resource "fluxx_synthetic" "api_health" {
  name            = "api-health-check"
  url             = "https://api.example.com/health"
  method          = "GET"
  expected_status = 200
  interval_secs   = 60
  timeout_secs    = 10
}

resource "fluxx_synthetic" "homepage" {
  name            = "homepage-uptime"
  url             = "https://www.example.com"
  method          = "GET"
  expected_status = 200
  interval_secs   = 300
  timeout_secs    = 30
}

# ── Deploy Annotation (CI/CD integration) ──

resource "fluxx_annotation" "deploy" {
  title       = "Deploy ${var.app_version}"
  description = "Deployed via Terraform"
  service     = "api-gateway"
  environment = "production"
  source      = "terraform"
  reference   = var.commit_sha
}

variable "app_version" {
  type    = string
  default = "v1.0.0"
}

variable "commit_sha" {
  type    = string
  default = ""
}

# ── Workflow ──

resource "fluxx_workflow" "auto_incident" {
  name         = "auto-create-incident-on-sev1"
  description  = "Automatically create incident when Sev1 alert fires"
  enabled      = true
  trigger_type = "alert_fired"
}

# ── Data Sources ──

data "fluxx_metrics" "all" {}

data "fluxx_service_map" "current" {}

output "metric_count" {
  value = length(data.fluxx_metrics.all.names)
}
