"""
Fluxx Python SDK

Lightweight client for sending traces, metrics, and logs to a Fluxx agent.

Usage:
    from fluxx import Fluxx

    client = Fluxx("http://localhost:7007")
    client.log("info", "User logged in", service="auth-service")
    client.metric("request.count", 1, metric_type="counter", tags={"env": "prod"})
    client.event("api-gateway", protocol="http", latency_us=1500)
"""

import json
import time
import uuid
import socket
import urllib.request
import urllib.error
from typing import Optional, Dict, Any, List


class Fluxx:
    """Fluxx observability client."""

    def __init__(
        self,
        endpoint: str = "http://localhost:7007",
        statsd_host: str = "localhost",
        statsd_port: int = 8125,
        project_id: Optional[str] = None,
        environment_id: Optional[str] = None,
        service: Optional[str] = None,
        release: Optional[str] = None,
    ):
        """
        :param release: Build/release version of this application.
            Forwarded as ``service.version`` in log resource attributes
            and as a top-level ``release`` field on events, so the agent
            can attribute errors to a specific release.
        """
        self.endpoint = endpoint.rstrip("/")
        self.statsd_host = statsd_host
        self.statsd_port = statsd_port
        self.project_id = project_id
        self.environment_id = environment_id
        self.service = service
        self.release = release
        self._udp_socket: Optional[socket.socket] = None

    # ── Logs ──

    def log(
        self,
        level: str,
        message: str,
        service: Optional[str] = None,
        trace_id: Optional[str] = None,
        span_id: Optional[str] = None,
        attributes: Optional[Dict[str, Any]] = None,
        release: Optional[str] = None,
    ) -> bool:
        """Send a structured log entry."""
        effective_release = release or self.release
        resource: Dict[str, Any] = {}
        if effective_release:
            resource["service.version"] = effective_release
        entry = {
            "id": str(uuid.uuid4()),
            "timestamp_us": _now_us(),
            "level": level,
            "message": message,
            "service": service or self.service,
            "trace_id": trace_id,
            "span_id": span_id,
            "attributes": attributes or {},
            "resource": resource,
            "project_id": self.project_id,
            "environment_id": self.environment_id,
            "release": effective_release,
        }
        return self._post("/v1/logs", [entry])

    def log_batch(self, entries: List[Dict[str, Any]]) -> bool:
        """Send multiple log entries at once."""
        return self._post("/v1/logs", entries)

    # ── Metrics (HTTP) ──

    def metric(
        self,
        name: str,
        value: float,
        metric_type: str = "gauge",
        tags: Optional[Dict[str, str]] = None,
        sample_rate: float = 1.0,
    ) -> bool:
        """Send a metric via HTTP API."""
        # Use StatsD UDP for performance
        return self.statsd(name, value, metric_type, tags, sample_rate)

    # ── StatsD (UDP) ──

    def statsd(
        self,
        name: str,
        value: float,
        metric_type: str = "gauge",
        tags: Optional[Dict[str, str]] = None,
        sample_rate: float = 1.0,
    ) -> bool:
        """Send a metric via StatsD UDP protocol."""
        type_map = {
            "counter": "c", "gauge": "g", "histogram": "h",
            "timing": "ms", "set": "s", "distribution": "d",
        }
        type_code = type_map.get(metric_type, "g")

        line = f"{name}:{value}|{type_code}"
        if sample_rate != 1.0:
            line += f"|@{sample_rate}"
        if tags:
            tag_str = ",".join(f"{k}:{v}" for k, v in tags.items())
            line += f"|#{tag_str}"

        return self._send_udp(line)

    # ── Events ──

    def event(
        self,
        source: str,
        protocol: str = "http",
        latency_us: int = 0,
        severity: str = "info",
        payload: Optional[str] = None,
        metadata: Optional[Dict[str, str]] = None,
        release: Optional[str] = None,
    ) -> bool:
        """Send an event to the agent."""
        evt = {
            "id": str(uuid.uuid4()),
            "timestamp_us": _now_us(),
            "protocol": protocol,
            "latency_us": latency_us,
            "severity": severity,
            "payload": payload,
            "service_id": source,
            "project_id": self.project_id,
            "environment_id": self.environment_id,
            "metadata": metadata or {},
            "release": release or self.release,
        }
        return self._post("/v1/events", evt)

    # ── Internal ──

    def _post(self, path: str, data: Any) -> bool:
        """HTTP POST to the Fluxx agent."""
        try:
            url = f"{self.endpoint}{path}"
            body = json.dumps(data).encode("utf-8")
            req = urllib.request.Request(
                url,
                data=body,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(req, timeout=5) as resp:
                return resp.status == 200
        except Exception:
            return False

    def _send_udp(self, line: str) -> bool:
        """Send a StatsD line via UDP."""
        try:
            if self._udp_socket is None:
                self._udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            self._udp_socket.sendto(
                line.encode("utf-8"),
                (self.statsd_host, self.statsd_port),
            )
            return True
        except Exception:
            return False

    def close(self):
        """Close the UDP socket."""
        if self._udp_socket:
            self._udp_socket.close()
            self._udp_socket = None


def _now_us() -> int:
    return int(time.time() * 1_000_000)


# ── Convenience ──

_default_client: Optional[Fluxx] = None


def init(endpoint: str = "http://localhost:7007", **kwargs) -> Fluxx:
    """Initialize the default Fluxx client."""
    global _default_client
    _default_client = Fluxx(endpoint, **kwargs)
    return _default_client


def log(level: str, message: str, **kwargs) -> bool:
    """Send a log using the default client."""
    assert _default_client, "Call fluxx.init() first"
    return _default_client.log(level, message, **kwargs)


def metric(name: str, value: float, **kwargs) -> bool:
    """Send a metric using the default client."""
    assert _default_client, "Call fluxx.init() first"
    return _default_client.metric(name, value, **kwargs)
