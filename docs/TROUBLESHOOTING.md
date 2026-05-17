# Troubleshooting Guide

Common issues and solutions for Fluxx Agent.

## Quick Diagnostics

```bash
# Check if Fluxx is running
fluxx config

# Check health
curl http://localhost:8080/health

# View logs
fluxx logs --follow

# Check version
fluxx --version
```

---

## Installation Issues

### "command not found: fluxx"

**Problem:** Binary not in PATH

**Solutions:**

```bash
# Check if installed
which fluxx
ls -la /usr/local/bin/fluxx

# Add to PATH
export PATH="/usr/local/bin:$PATH"
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc

# Or reinstall
curl -sSL https://fluxx.dev/install.sh | bash
```

### "Permission denied"

**Problem:** Insufficient permissions

**Solutions:**

```bash
# Fix permissions
sudo chmod +x /usr/local/bin/fluxx

# Or reinstall with sudo
curl -sSL https://fluxx.dev/install.sh | sudo bash
```

### "Cannot execute binary file"

**Problem:** Wrong architecture

**Solutions:**

```bash
# Check your architecture
uname -m  # x86_64, arm64, etc.

# Download correct binary
curl -LO https://servicenow.com/fluxx/agent/releases/download/v0.1.0/fluxx-$(uname -s)-$(uname -m).tar.gz
```

---

## Startup Issues

### "Address already in use"

**Problem:** Port 8080 is already in use

**Solutions:**

```bash
# Find what's using port 8080
lsof -i :8080
netstat -tlnp | grep 8080

# Kill the process
kill -9 <PID>

# Or use a different port
fluxx start --config <(echo 'server: { bind: "0.0.0.0:8081" }')
```

### "Cannot bind to address"

**Problem:** Permission denied for low ports (< 1024)

**Solutions:**

```bash
# Use high port (recommended)
fluxx start --config <(echo 'server: { bind: "0.0.0.0:8080" }')

# Or use sudo (not recommended for production)
sudo fluxx start

# Or grant capability
sudo setcap 'cap_net_bind_service=+ep' /usr/local/bin/fluxx
```

### "Configuration file not found"

**Problem:** Fluxx can't find config file

**Solutions:**

```bash
# Create default config
mkdir -p ~/.config/fluxx
cat > ~/.config/fluxx/config.yaml << 'EOF'
node_id: "default"
server:
  bind: "0.0.0.0:8080"
EOF

# Or specify config path
fluxx start --config ./my-config.yaml
```

---

## Connection Issues

### "Connection refused"

**Problem:** Fluxx not running or wrong port

**Solutions:**

```bash
# Check if running
curl http://localhost:8080/health

# Check process
ps aux | grep fluxx

# Start Fluxx
fluxx start &

# Check firewall
sudo iptables -L | grep 8080
```

### "Unauthorized"

**Problem:** Missing or invalid API key

**Solutions:**

```bash
# Set API key
export FLUXX_API_KEY="your-key-here"

# Or disable auth (dev only)
export FLUXX_DISABLE_AUTH=1

# Include in request
curl -H "X-API-Key: your-key" http://localhost:8080/metrics
```

### "Mesh connection failed"

**Problem:** Cannot connect to mesh peers

**Solutions:**

```bash
# Check network connectivity
nc -zv node-2 7946

# Check firewall rules
sudo iptables -L | grep 7946

# Verify peer addresses are correct
cat fluxx.yaml | grep peers -A 5

# Check mesh status
fluxx mesh status
```

---

## Performance Issues

### High CPU Usage

**Symptoms:**
- CPU usage > 80%
- Slow query response
- Dropped events

**Diagnosis:**

```bash
# Check CPU usage
top -p $(pgrep fluxx)

# Profile the binary
perf top -p $(pgrep fluxx)

# Check event rate
curl http://localhost:8080/metrics | grep event_rate
```

**Solutions:**

```yaml
# Reduce workers
performance:
  workers: 8  # Reduce from default 16
  
# Enable sampling
integrations:
  - type: postgres
    sampling_rate: 0.1  # Sample 10%
    
# Increase batch size
performance:
  batch:
    size: 500  # Batch more events
```

### High Memory Usage

**Symptoms:**
- Memory usage growing
- OOM kills
- System swapping

**Diagnosis:**

```bash
# Check memory usage
ps aux | grep fluxx
smem -P fluxx

# Check for memory leaks
valgrind --tool=massif fluxx start
```

**Solutions:**

```yaml
# Limit memory
performance:
  max_memory: "2GB"
  buffer_size: "512KB"
  
# Reduce retention
storage:
  retention:
    hot:
      duration: "3d"
      max_size: "50GB"
      
# Disable unnecessary integrations
integrations:
  - type: some_integration
    enabled: false
```

### Slow Queries

**Symptoms:**
- Query timeout errors
- High latency
- Timeouts

**Diagnosis:**

```bash
# Check query latency
curl http://localhost:8080/metrics | grep query_latency

# Enable slow query logging
logging:
  level: "debug"
  slow_query_threshold: "1s"
```

**Solutions:**

```yaml
# Optimize storage
storage:
  type: "memory"  # Use memory for hot data
  
# Add indexes
# (For supported backends)

# Query optimization
# - Use time range filters
# - Limit result size
# - Use aggregation
```

---

## Integration Issues

### Integration shows "unhealthy"

**Problem:** Cannot connect to integrated service

**Solutions:**

```bash
# Test connectivity
nc -zv postgres-host 5432

# Check credentials
fluxx integration test postgres

# Verify network policies
kubectl get networkpolicies

# Check service logs
fluxx logs --integration postgres
```

### "Authentication failed"

**Problem:** Invalid credentials

**Solutions:**

```bash
# Check environment variables
echo $POSTGRES_URL
echo $AWS_ACCESS_KEY_ID

# Test credentials directly
psql $POSTGRES_URL -c "SELECT 1"

# Update credentials
export POSTGRES_URL="postgres://newuser:newpass@host:5432/db"
```

### "Timeout"

**Problem:** Integration taking too long to respond

**Solutions:**

```yaml
# Increase timeout
integrations:
  - type: postgres
    timeout: "30s"
    
# Check service health
# - Restart the integrated service
# - Check for resource constraints
# - Verify network latency
```

---

## AI Diagnostics Issues

### "AI analysis failed"

**Problem:** Cannot connect to LLM API

**Solutions:**

```bash
# Check API key
echo $OPENAI_API_KEY

# Test API connection
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY"

# Check rate limits
# OpenAI: https://platform.openai.com/account/rate-limits
```

### "No AI suggestions"

**Problem:** AI enabled but not providing suggestions

**Solutions:**

```yaml
# Verify AI is enabled
ai:
  enabled: true
  provider: "openai"
  model: "gpt-4"
  
# Check min confidence
ai:
  diagnostics:
    min_confidence: 0.7  # Lower threshold
```

---

## Data Issues

### Missing events

**Problem:** Expected events not appearing

**Diagnosis:**

```bash
# Check if capture is working
fluxx integration status

# Verify filters
# - Not too restrictive
# - Correct event types

# Check for sampling
integrations:
  - type: some_integration
    sampling_rate: 1.0  # No sampling
```

### Duplicate events

**Problem:** Same event appearing multiple times

**Solutions:**

```yaml
# Enable deduplication
processing:
  deduplication:
    enabled: true
    window: "5m"
```

### Wrong timestamps

**Problem:** Event timestamps incorrect

**Solutions:**

```bash
# Check system time
ntpq -p
timedatectl status

# Sync time
sudo ntpdate -s time.nist.gov
```

---

## Alerting Issues

### "Alerts not firing"

**Problem:** Alerts not being triggered

**Diagnosis:**

```bash
# Check alert rules
fluxx alert rules list

# Test alert condition
fluxx alert test --rule "High CPU"

# Check alert history
fluxx alert history
```

**Solutions:**

```yaml
# Verify rule syntax
alerting:
  rules:
    - name: "High CPU"
      condition: "cpu_usage > 80"  # Correct syntax
      duration: "5m"
      
# Check notification channels
alerting:
  channels:
    - type: slack
      webhook_url: "${SLACK_WEBHOOK}"  # Must be set
```

### "Too many alerts"

**Problem:** Alert fatigue from noisy alerts

**Solutions:**

```yaml
# Add hysteresis
alerting:
  rules:
    - name: "High CPU"
      condition: "cpu_usage > 80"
      duration: "10m"  # Wait longer
      
# Group related alerts
alerting:
  grouping:
    enabled: true
    window: "5m"
    
# Use AI filtering
ai:
  alerting:
    noise_reduction: true
```

---

## Log Analysis

### Enable debug logging

```bash
# Set log level
export FLUXX_LOG_LEVEL=debug
fluxx start

# Or in config
logging:
  level: "debug"
```

### Common log patterns

**Error starting server:**
```
ERROR server::bind - Address already in use (os error 98)
→ Port conflict, change port or kill existing process
```

**Integration connection failed:**
```
WARN integration::postgres - Connection timeout
→ Check network, credentials, and service health
```

**Mesh connection failed:**
```
ERROR mesh::gossip - Failed to connect to peer node-2:7946
→ Check network connectivity and firewall rules
```

**High memory warning:**
```
WARN memory - Memory usage above 80%
→ Reduce retention, sampling, or add more memory
```

---

## Getting Help

### Collect diagnostic info

```bash
# Generate diagnostic report
fluxx doctor > fluxx-diagnostic.txt

# Include:
# - Version info
# - Configuration
# - Health status
# - Recent logs
# - System info
```

### Report an issue

1. **Search existing issues:** [ServiceNow Issues](https://servicenow.com/fluxx/agent/issues)
2. **Collect diagnostics:** Run `fluxx doctor`
3. **Create issue:** Include:
   - Fluxx version
   - Operating system
   - Configuration (redact secrets)
   - Error messages
   - Steps to reproduce

### Community support

- **Discord:** [discord.gg/fluxx](https://discord.gg/fluxx) (fastest response)
- **ServiceNow Discussions:** [servicenow.com/fluxx/agent/discussions](https://servicenow.com/fluxx/agent/discussions)
- **Stack Overflow:** Tag with `fluxx-agent`

---

## Emergency Procedures

### Complete data loss

```bash
# Restore from backup
fluxx restore --from-backup /path/to/backup

# Or restart with empty state
rm -rf /var/lib/fluxx/data/*
fluxx start
```

### Reset to defaults

```bash
# Remove all config
rm -rf ~/.config/fluxx
rm -f ./fluxx.yaml

# Reinstall
curl -sSL https://fluxx.dev/install.sh | bash
```

### Security incident

```bash
# 1. Revoke all API keys
fluxx admin keys revoke --all

# 2. Rotate to new keys
fluxx admin keys rotate

# 3. Check audit logs
fluxx audit logs --since "24h ago"

# 4. Report incident
# security@fluxx.dev
```

---

## FAQ

**Q: Why is Fluxx using so much memory?**
A: Check retention settings and number of integrations. Reduce `storage.retention.hot.max_size`.

**Q: Can I run multiple instances on the same machine?**
A: Yes, use different ports and node IDs for each instance.

**Q: How do I backup my data?**
A: Use `fluxx backup create` or directly backup `/var/lib/fluxx/data`.

**Q: Can I downgrade to an older version?**
A: Yes, but check migration notes. Some versions may not be compatible.

---

For more help, visit [fluxx.dev/docs](https://fluxx.dev/docs).
