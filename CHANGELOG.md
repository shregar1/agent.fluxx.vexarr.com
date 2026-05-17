# Changelog

All notable changes to the Fluxx agent are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/), and the project
adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added — Agent-side runtime grant client

Closes the security loop the control plane has been waiting for. The agent
now actually performs the four steps `ARCHITECTURE.md §1` promised:

1. **Machine fingerprint** (`src/license/fingerprint.rs`) — sha256 of
   machine UUID + primary MAC + hostname + CPU brand. Raw inputs never
   leave the host; only the digest is sent on grant requests.
2. **Runtime grant client** (`src/license/grant.rs`) — `mint`,
   `refresh`, `with_signed_request`, `start_refresher` background task.
   `GrantStore` holds the current `{ grant_token, hmac_key }` in memory
   only; nothing touches disk.
3. **HMAC request signing** (`src/license/signing.rs`) — every premium
   call attaches `Authorization: Bearer flxg.v1.…`, `X-Fluxx-Nonce`,
   `X-Fluxx-Timestamp`, `X-Fluxx-Signature` headers. Body bytes are
   signed once and sent verbatim so re-serialization can't desync.
4. **7-day on-disk license cache** (`src/license/cache.rs`) at
   `~/.fluxx/license.cache.json`. Keyed by sha256 of the API key,
   atomic-write semantics, 16-entry cap.
5. **TLS SPKI pinning** (`src/license/cert_pin.rs`) — opt-in via
   `FLUXX_API_TLS_PIN=<base64-sha256-of-spki>`. Custom `rustls`
   verifier; rejects any leaf cert whose SPKI hash doesn't match.
6. **Worked-example premium handler** (`src/ai/diagnose_remote.rs`) —
   the canonical pattern for `/v1/ai/diagnose` and future premium
   endpoints. Includes a TCP-loopback test that verifies all four
   signing headers reach the wire.

51 new unit tests across these modules, all green.

### Added
- CI: `release.yml` cross-compiles binaries for Linux (amd64/arm64),
  macOS (amd64/arm64), and Windows (amd64); attaches sha256 sums to GH
  Releases.
- CI: `image.yml` builds multi-arch container images
  (`linux/amd64`, `linux/arm64`) via `docker buildx` and publishes to
  `ghcr.io/fluxx/fluxx-agent`, with SBOM and provenance attestations.
- `helm-chart/README.md` documents the chart and explicitly notes that
  it is a chart, not a Kubernetes operator.

### Changed
- Renamed `k8s-operator/` → `helm-chart/` to match what actually ships.
  No CRDs or custom controllers existed; the directory only contained a
  Helm chart, and the misleading name suggested otherwise.
- Removed empty placeholder directories `k8s-operator/controllers/` and
  `k8s-operator/config/`.

### Removed
- Consolidated three blog corpora (`blogs/`, `docs/blogs/`,
  `docs/blog/`) into the canonical `docs.fluxx.dev` blog with consistent
  `YYYY-MM-DD-slug.md` naming and Docusaurus frontmatter. 65 posts in
  total.
- Internal scratch documents archived out of `docs/`:
  - `CLI_BANNER.md` (ASCII art notes)
  - `DASHBOARD_PROMPT.md` (Claude prompt template)
  - `DOCS_COMMANDS.md` (superseded by the CLI reference page on
    docs.fluxx.dev)
  - `DOCS_FEATURES.md` (meta-notes about documentation)
- `SECURITY_AUDIT_REPORT.md` superseded by the structured remediation
  history below.

---

## Security

This section is the authoritative record of vulnerabilities found and
remediated. Every finding includes the CWE, the affected file, the
status, and the verification reference. New findings are appended here
on disclosure.

### 2026-04 audit — internal pre-release scan

11 findings across 4 severity levels: 1 Critical, 3 High, 4 Medium, 3 Low.

#### 🔴 Critical

| ID | CWE | Component | Status | Notes |
|---|---|---|---|---|
| SEC-001 | CWE-78 OS Command Injection | `integrations/cache/memcached.rs` | **Fixed before release** | Shell pipeline replaced with a direct TCP client; host string validated against a hostname/IP regex. Verified at `src/integrations/cache/memcached.rs:68`. |

#### 🟠 High

| ID | CWE | Component | Status | Notes |
|---|---|---|---|---|
| SEC-002 | CWE-942 Permissive Cross-domain Policy | `api/server.rs` | **Fixed** | Default is now **same-origin only**. Cross-origin requests require `FLUXX_CORS_ORIGIN=<exact-origin>` (CSV supported). Wildcard `*` requires the additional opt-in `FLUXX_CORS_ALLOW_ANY=true` so it cannot happen by accident. Verified by 7 unit tests in `api/server::cors_tests`. |
| SEC-003 | CWE-22 Path Traversal | `cli/mod.rs` | **Fixed** | `sanitize_path` canonicalizes the base dir, joins the user input, re-canonicalizes, and rejects anything that escapes the base. Verified at `src/cli/mod.rs:11` and `:265`. |
| SEC-004 | CWE-306 Missing Authentication | `api/server.rs` | **Fixed** | `auth_middleware` requires `X-API-Key` matching `FLUXX_API_KEY`. Verified at `src/api/server.rs:16`. |

#### 🟡 Medium

| ID | CWE | Component | Status | Notes |
|---|---|---|---|---|
| SEC-005 | CWE-1333 ReDoS | `security/masking.rs:30` | Open | Add `RegexBuilder::size_limit` and `dfa_size_limit` constraints. |
| SEC-006 | CWE-209 Information Disclosure via Errors | multiple (`diagnostic/ticketing/*.rs`) | Open | Replace `{:?}` debug formatting with sanitized error types in customer-facing errors. |
| SEC-007 | CWE-190 Integer Overflow | `perf/mmap_storage.rs:49` | Open | Cast `usize → u64` via `try_into()` and surface oversized inputs as a typed error. |
| SEC-008 | CWE-502 Insecure Deserialization | `mesh/config.rs:62` | Open | Validate mesh config schema before applying (size limits, allowed keys). |

#### 🟢 Low

| ID | CWE | Component | Status | Notes |
|---|---|---|---|---|
| SEC-009 | CWE-391 Uncaught Panics | repo-wide (~100 `unwrap()`/`expect()`) | Open | Sweep tagged for cleanup as part of the production-hardening pass; enable `clippy::unwrap_used` in CI as a warn-level lint. |
| SEC-010 | Debug info in release | build config | Open | Audit `#[cfg(debug_assertions)]` and `#[cfg(test)]` gates. |
| SEC-011 | CWE-693 Missing Security Headers | `api/server.rs` | **Partially fixed** | `X-Content-Type-Options: nosniff` and `X-Frame-Options: DENY` shipped at `src/api/server.rs:47-48`. CSP and HSTS still pending. |

### Reporting a new vulnerability

Email <security@fluxx.dev>. We acknowledge within 24 hours, ship
critical fixes within 7 days, and publish CVE-attributed remediations
here. Researchers are credited unless they ask otherwise.

---

## [0.1.0] — 2026-05-09

Initial public-beta release. Highlights:

### Added
- 13 protocol handlers: HTTP, WebSocket, QUIC, gRPC, Modbus, StatsD,
  Prometheus scrape, OTLP HTTP/gRPC, TraceContext, others.
- Tiered storage (hot in-memory → warm mmap → cold).
- PromQL-compatible query engine (selectors, ranges, offsets, all major
  aggregations, `rate`, `increase`, `histogram_quantile`, binary ops).
- Mesh federation with gossip-based peer discovery and topology-aware
  routing.
- Self-telemetry pipeline.
- Three Cargo build profiles: `release`, `dist` (~3 MB stripped), `ci`.
- Multi-stage Dockerfile producing a non-root runtime image.
- Helm chart for Kubernetes DaemonSet deployment.
