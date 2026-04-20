import requests
import time
import uuid
import random

# Fluxx Agent Configuration
API_URL = "http://127.0.0.1:8080/v1/events"
# Set FLUXX_API_KEY if auth is enabled, otherwise it will be ignored if FLUXX_DISABLE_AUTH=true
HEADERS = {
    "Content-Type": "application/json",
    "X-API-Key": "fluxx_test_key_123" 
}

def send_event(protocol, severity, message, latency=0, metadata=None):
    event_id = str(uuid.uuid4())
    timestamp = int(time.time() * 1_000_000)
    
    payload = {
        "id": event_id,
        "protocol": protocol,
        "timestamp_us": timestamp,
        "latency_us": latency,
        "severity": severity,
        "payload": message,
        "metadata": metadata or {}
    }
    
    try:
        response = requests.post(API_URL, json=payload, headers=HEADERS)
        if response.status_code == 200:
            print(f"[SUCCESS] Sent {protocol} event: {event_id} ({severity})")
        else:
            print(f"[FAILED] Status {response.status_code}: {response.text}")
    except Exception as e:
        print(f"[ERROR] Could not connect to Fluxx Agent: {e}")

if __name__ == "__main__":
    print("🚀 Starting Fluxx Sample Log Generator...")
    
    # 1. Normal HTTP Traffic
    send_event(
        protocol="http",
        severity="info",
        message="GET /api/v1/users/shregar1",
        latency=45000,
        metadata={"source_ip": "127.0.0.1", "user_agent": "Mozilla/5.0", "status_code": "200"}
    )
    
    # 2. High Latency Anomaly
    send_event(
        protocol="http",
        severity="high",
        message="POST /api/v1/billing/checkout",
        latency=2500000, # 2.5 seconds
        metadata={"source_ip": "45.12.33.1", "request_id": "req_8822", "db_queries": "12"}
    )
    
    # 3. WebSocket Connection Spike
    send_event(
        protocol="ws",
        severity="medium",
        message="WebSocket Handshake Established",
        metadata={"reconnect_count": "12", "client_version": "2.4.0"}
    )
    
    # 4. gRPC Service Error
    send_event(
        protocol="grpc",
        severity="critical",
        message="rpc error: code = Unavailable desc = transport is closing",
        metadata={"service": "auth-engine", "method": "AuthorizeSession"}
    )
    
    # 5. eBPF Network Burst
    send_event(
        protocol="ebpf",
        severity="low",
        message="TCP Segment Burst Detected",
        metadata={"size_bytes": "2097152", "interface": "en0"}
    )

    print("\n✅ Sample logs sent. Check your Fluxx dashboard or use 'curl' to view them!")
