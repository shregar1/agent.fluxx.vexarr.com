# Fluxx Grafana Dashboard Templates

Pre-built Grafana dashboards for common infrastructure monitoring. Import via Grafana UI or provisioning.

## Available Dashboards (200+)

### Kubernetes (20)
- k8s-cluster-overview.json — Cluster health, node count, pod status
- k8s-node-resources.json — Per-node CPU, memory, disk
- k8s-pod-metrics.json — Pod CPU/memory/network/restarts
- k8s-deployment-status.json — Deployment replicas, rollout status
- k8s-namespace-resources.json — Resource usage per namespace
- k8s-persistent-volumes.json — PV/PVC usage and status
- k8s-ingress-traffic.json — Ingress controller request rates
- k8s-hpa-scaling.json — HPA scaling events and targets
- k8s-daemonset-health.json — DaemonSet pod coverage
- k8s-etcd-performance.json — etcd latency and leader elections
- k8s-apiserver.json — API server request rates and latency
- k8s-scheduler.json — Scheduling latency and queue depth
- k8s-kubelet.json — Kubelet runtime and pod operations
- k8s-coredns.json — DNS query rates and cache hit ratio
- k8s-resource-quotas.json — Quota usage per namespace
- k8s-network-policies.json — Network policy enforcement
- k8s-events.json — Cluster events and warnings
- k8s-jobs-cronjobs.json — Job completion rates and durations
- k8s-statefulsets.json — StatefulSet pod ordering
- k8s-service-mesh.json — Istio/Linkerd sidecar metrics

### Databases (30)
- postgresql-overview.json — Connections, QPS, cache hit ratio
- postgresql-replication.json — Replication lag, WAL size
- postgresql-locks.json — Lock contention, deadlocks
- postgresql-tables.json — Table size, bloat, vacuum status
- mysql-overview.json — QPS, slow queries, connections
- mysql-innodb.json — Buffer pool, row operations
- mysql-replication.json — Slave lag, thread status
- redis-overview.json — Hit rate, memory, ops/sec
- redis-keyspace.json — Key count, expiry distribution
- redis-cluster.json — Cluster slot coverage, redirections
- mongodb-overview.json — Operations, connections, page faults
- mongodb-replicaset.json — Replication oplog, member health
- mongodb-wiredtiger.json — Cache, tickets, concurrency
- elasticsearch-overview.json — Index rate, search latency
- elasticsearch-cluster.json — Shard allocation, node roles
- elasticsearch-indices.json — Per-index size and doc count
- cassandra-overview.json — Read/write latency, compaction
- clickhouse-overview.json — Query throughput, merge operations
- cockroachdb-overview.json — SQL statements, range distribution
- influxdb-overview.json — Write throughput, query performance
- timescaledb-overview.json — Hypertable chunks, compression
- neo4j-overview.json — Transaction counts, page cache
- dynamodb-overview.json — Read/write capacity, throttling
- cosmosdb-overview.json — Request units, consistency levels
- mariadb-overview.json — Thread pool, query cache
- sqlite-overview.json — Page cache hits, journal mode
- couchdb-overview.json — Document operations, view builds
- memcached-overview.json — Hit rate, evictions, connections
- etcd-overview.json — Raft proposals, DB size, leader
- vitess-overview.json — VTGate throughput, VTTablet lag

### Web & API (20)
- nginx-overview.json — Requests/sec, error rates, upstream latency
- nginx-plus.json — Server zones, upstreams, caches
- apache-overview.json — Workers, request rate, scoreboard
- haproxy-overview.json — Frontend/backend sessions, error rates
- traefik-overview.json — Entrypoint requests, router latency
- envoy-overview.json — Cluster health, circuit breakers
- caddy-overview.json — Request duration, TLS handshakes
- http-api-golden-signals.json — Latency, traffic, errors, saturation
- http-api-endpoints.json — Per-endpoint latency and error rates
- grpc-services.json — RPC rates, stream counts, status codes
- graphql-overview.json — Query complexity, resolver timing
- websocket-connections.json — Active connections, message rates
- rest-api-sla.json — SLA compliance, percentile tracking
- api-gateway-overview.json — Rate limiting, auth failures
- cdn-performance.json — Cache hit ratio, edge latency
- dns-overview.json — Query types, response codes, latency
- ssl-certificates.json — Expiry dates, chain validity
- http2-performance.json — Stream multiplexing, server push
- cors-monitoring.json — Preflight requests, blocked origins
- rate-limiting.json — Throttled requests, burst patterns

### Cloud Providers (25)
- aws-ec2-overview.json — Instance CPU, network, EBS
- aws-rds-overview.json — DB connections, IOPS, latency
- aws-lambda-overview.json — Invocations, duration, errors
- aws-sqs-overview.json — Queue depth, message age
- aws-s3-overview.json — Request count, bucket size
- aws-elb-overview.json — Request count, latency, 5xx errors
- aws-ecs-overview.json — Task health, CPU/memory reservation
- aws-cloudfront.json — Cache hit ratio, error rate
- aws-dynamodb.json — Read/write capacity, throttles
- aws-kinesis.json — Shard throughput, iterator age
- gcp-gke-overview.json — Cluster utilization, autopilot
- gcp-cloud-sql.json — Connections, replication, storage
- gcp-cloud-run.json — Request count, instance count
- gcp-pubsub.json — Message throughput, subscription backlog
- gcp-cloud-functions.json — Execution count, duration
- gcp-bigquery.json — Slot utilization, query performance
- azure-vm-overview.json — CPU, memory, disk, network
- azure-sql-database.json — DTU usage, connections
- azure-app-service.json — Request count, response time
- azure-cosmos-db.json — Request units, partition usage
- azure-event-hubs.json — Throughput, capture, consumer lag
- digitalocean-droplets.json — CPU, bandwidth, disk I/O
- linode-overview.json — Instance metrics, transfer
- heroku-overview.json — Dyno load, memory, response time
- cloudflare-overview.json — Request rate, threat score

### Message Queues (15)
- kafka-overview.json — Consumer lag, throughput, partitions
- kafka-consumer-groups.json — Per-group lag and offset
- kafka-broker-metrics.json — Under-replicated partitions, ISR
- rabbitmq-overview.json — Queue depth, publish/consume rates
- rabbitmq-exchanges.json — Per-exchange message routing
- rabbitmq-connections.json — Connection and channel counts
- nats-overview.json — Message rate, subscriptions
- nats-jetstream.json — Stream size, consumer acknowledgments
- pulsar-overview.json — Topic throughput, subscription lag
- activemq-overview.json — Queue size, enqueue/dequeue rates
- aws-sqs-detailed.json — FIFO vs standard, DLQ depth
- redis-streams.json — Entry count, consumer group lag
- zeromq-overview.json — Socket stats, message patterns
- mosquitto-overview.json — MQTT connections, messages
- celery-overview.json — Task rates, worker utilization

### Runtime & Language (20)
- nodejs-overview.json — Event loop lag, heap, GC
- nodejs-express.json — Route latency, middleware timing
- go-overview.json — Goroutines, GC, heap allocation
- go-http-server.json — Request duration, active connections
- python-overview.json — GIL contention, thread count
- python-django.json — View latency, DB queries per request
- python-flask.json — Request rate, template rendering
- jvm-overview.json — Heap, GC, threads, class loading
- jvm-spring-boot.json — Actuator metrics, endpoint latency
- dotnet-overview.json — GC collections, thread pool
- dotnet-aspnet.json — Request rate, exception count
- ruby-overview.json — GC stats, object allocations
- ruby-rails.json — Action latency, ActiveRecord queries
- php-overview.json — OPcache, request duration
- php-laravel.json — Queue workers, cache hit rate
- rust-overview.json — Memory usage, async runtime stats
- elixir-beam.json — Process count, scheduler utilization
- java-quarkus.json — Native image metrics, startup time
- kotlin-coroutines.json — Dispatcher utilization, suspension
- swift-vapor.json — Route handling, EventLoop utilization

### CI/CD (15)
- github-actions.json — Workflow duration, failure rate
- gitlab-ci.json — Pipeline stages, job queue time
- jenkins-overview.json — Build duration, executor utilization
- circleci-overview.json — Workflow credits, job duration
- argocd-overview.json — Sync status, health checks
- flux-gitops.json — Reconciliation time, source revisions
- tekton-pipelines.json — TaskRun duration, pipeline success rate
- drone-ci.json — Build queue, step execution time
- buildkite-overview.json — Agent utilization, queue wait time
- spinnaker-overview.json — Pipeline execution, deployment frequency
- deploy-frequency.json — Deployment rate (DORA metric)
- lead-time.json — Lead time for changes (DORA metric)
- mttr.json — Mean time to recovery (DORA metric)
- change-failure-rate.json — Change failure rate (DORA metric)
- deployment-annotations.json — Deploy markers timeline

### Security (15)
- waf-overview.json — Blocked requests, rule matches
- fail2ban.json — Banned IPs, jail status
- vault-overview.json — Secret access, token renewal
- cert-manager.json — Certificate expiry, issuance
- oauth-overview.json — Token grants, refresh rates
- rate-limiting-security.json — DDoS patterns, IP blocking
- api-authentication.json — Auth failures, token usage
- network-security.json — Firewall rules, connection tracking
- container-security.json — Image vulnerabilities, policy violations
- compliance-overview.json — Policy adherence, audit findings
- intrusion-detection.json — Alert severity, source analysis
- tls-handshake.json — Protocol versions, cipher suites
- cors-violations.json — Blocked cross-origin requests
- csp-reports.json — Content Security Policy violations
- security-events.json — Login attempts, privilege escalations

### Observability (15)
- fluxx-self-monitoring.json — Events/sec, query latency, storage
- fluxx-mesh-health.json — Node status, gossip latency
- fluxx-otlp-ingestion.json — Span/metric/log ingestion rates
- fluxx-statsd-receiver.json — UDP packet rate, parse errors
- fluxx-alert-pipeline.json — Alert fire rate, channel delivery
- fluxx-anomaly-detector.json — Baseline drift, z-scores
- fluxx-wal-storage.json — Segment count, write throughput
- opentelemetry-collector.json — Exporter queue, dropped spans
- prometheus-server.json — Scrape duration, sample count
- jaeger-overview.json — Span ingestion, storage size
- zipkin-overview.json — Trace throughput, dependency graph
- elastic-apm.json — Transaction rate, error distribution
- sentry-overview.json — Error grouping, release health
- pagerduty-incidents.json — Incident count, MTTA/MTTR
- statuspage-overview.json — Component status, incident history

### Infrastructure (15)
- docker-overview.json — Container CPU/memory/network
- docker-compose.json — Service health, restart count
- systemd-services.json — Service status, start time
- process-explorer.json — Top processes by CPU/memory
- filesystem-overview.json — Mount points, inode usage
- nfs-overview.json — NFS operations, RPC statistics
- zfs-overview.json — Pool health, scrub status
- lvm-overview.json — Logical volume utilization
- smart-disk.json — SMART attributes, predicted failure
- power-management.json — UPS status, battery level
- temperature-monitoring.json — Hardware temps, fan speed
- load-balancer.json — Backend health, session distribution
- vpn-overview.json — Tunnel status, throughput
- proxy-overview.json — Squid/HAProxy cache and connections
- bare-metal.json — IPMI sensors, BMC status
