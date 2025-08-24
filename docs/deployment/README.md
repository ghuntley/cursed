# Deployment Guide

## Overview

This guide covers deploying CURSED applications in production environments, including installation, configuration, monitoring, and scaling strategies.

## Production Installation

### System Requirements
- **OS**: Linux, macOS, Windows
- **Architecture**: x86_64, ARM64
- **Memory**: 512MB minimum, 2GB recommended
- **Storage**: 100MB for runtime, additional for applications

### Install from Package
```bash
# Linux (Ubuntu/Debian)
curl -sSf https://install.cursedlang.org | sh

# macOS (Homebrew)
brew install cursed-lang

# Windows (Chocolatey)
choco install cursed-lang
```

### Container Deployment
```dockerfile
# Dockerfile
FROM cursedlang/runtime:latest

WORKDIR /app
COPY your-app.csd .
COPY stdlib/ ./stdlib/

RUN cursed-zig --compile your-app.csd

EXPOSE 8080
CMD ["./your-app"]
```

### Kubernetes Deployment
```yaml
# cursed-app.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cursed-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cursed-app
  template:
    metadata:
      labels:
        app: cursed-app
    spec:
      containers:
      - name: app
        image: your-registry/cursed-app:v1.0
        ports:
        - containerPort: 8080
        resources:
          limits:
            memory: "512Mi"
            cpu: "500m"
          requests:
            memory: "256Mi"
            cpu: "250m"
```

## Configuration

### Environment Variables
```bash
# Production configuration
export CURSED_ENV=production
export CURSED_LOG_LEVEL=info
export CURSED_GC_TARGET=100MB
export CURSED_MAX_GOROUTINES=10000
export CURSED_THREAD_POOL_SIZE=8
```

### Configuration Files
```toml
# cursed.toml
[runtime]
gc_target_mb = 100
max_goroutines = 10000
thread_pool_size = 8

[logging]
level = "info"
format = "json"
output = "stdout"

[performance]
enable_profiling = false
metric_collection = true
```

## Monitoring and Observability

### Health Checks
```cursed
yeet "httpz"

slay health_check_handler(req HttpRequest) HttpResponse {
    sus checks []HealthCheck = [
        check_database_connection(),
        check_memory_usage(),
        check_goroutine_count()
    ]
    
    sus all_healthy lit = checks.all(slay(check HealthCheck) lit {
        damn check.healthy
    })
    
    ready (all_healthy) {
        damn HttpResponse{200, "OK"}
    } otherwise {
        damn HttpResponse{503, "Service Unavailable"}
    }
}
```

### Metrics Collection
```cursed
yeet "metricz"

struct AppMetrics {
    request_count Counter,
    response_time Histogram,
    error_rate Gauge,
    active_connections Gauge
}

slay setup_metrics() AppMetrics {
    damn AppMetrics{
        request_count: metricz.counter("http_requests_total"),
        response_time: metricz.histogram("http_duration_seconds"),
        error_rate: metricz.gauge("error_rate"),
        active_connections: metricz.gauge("active_connections")
    }
}
```

### Logging
```cursed
yeet "loggingz"

slay setup_production_logging() {
    loggingz.configure({
        level: "info",
        format: "json",
        fields: {
            service: "cursed-app",
            version: "1.0.0",
            environment: "production"
        }
    })
}

slay log_request(req HttpRequest, duration drip) {
    loggingz.info("HTTP request processed", {
        method: req.method,
        path: req.path,
        status: req.status,
        duration_ms: duration,
        user_agent: req.headers["User-Agent"]
    })
}
```

## Performance Optimization

### Compilation Optimization
```bash
# Production build with optimizations
./zig-out/bin/cursed-zig --compile --optimize=fast app.csd

# Profile-guided optimization
./zig-out/bin/cursed-zig --compile --optimize=pgo app.csd
```

### Runtime Tuning
```cursed
// Memory management tuning
yeet "memoryz"

slay tune_gc_for_production() {
    memoryz.set_gc_target(100 * 1024 * 1024)  // 100MB
    memoryz.set_gc_max_pause(10)              // 10ms max pause
    memoryz.enable_concurrent_gc(based)        // Concurrent collection
}

// Goroutine pool tuning
yeet "concurrenz"

slay setup_worker_pools() {
    concurrenz.set_max_goroutines(10000)
    concurrenz.set_thread_pool_size(runtime_cpu_count())
    concurrenz.enable_work_stealing(based)
}
```

### Load Balancing
```cursed
yeet "networkz"

struct LoadBalancer {
    servers []Server,
    current_index drip,
    health_checker HealthChecker
}

slay (lb *LoadBalancer) get_next_server() Server yikes<tea> {
    sus available_servers []Server = lb.servers.filter(slay(s Server) lit {
        damn lb.health_checker.is_healthy(s)
    })
    
    ready (available_servers.is_empty()) {
        yikes "no healthy servers available"
    }
    
    sus server Server = available_servers[lb.current_index % available_servers.length()]
    lb.current_index++
    
    damn server
}
```

## Security

### TLS Configuration
```cursed
yeet "tlsz"

slay setup_tls_server() TLSServer {
    sus config TLSConfig = TLSConfig{
        cert_file: "/etc/ssl/certs/app.crt",
        key_file: "/etc/ssl/private/app.key",
        protocols: ["TLSv1.2", "TLSv1.3"],
        cipher_suites: [
            "TLS_AES_256_GCM_SHA384",
            "TLS_CHACHA20_POLY1305_SHA256",
            "TLS_AES_128_GCM_SHA256"
        ]
    }
    
    damn tlsz.create_server(config)
}
```

### Authentication
```cursed
yeet "authz"

slay setup_jwt_auth() JWTValidator {
    sus config JWTConfig = JWTConfig{
        secret_key: get_env("JWT_SECRET"),
        algorithm: "HS256",
        expiry_seconds: 3600,
        issuer: "cursed-app",
        audience: "api-users"
    }
    
    damn authz.create_validator(config)
}
```

## Scaling Strategies

### Horizontal Scaling
```yaml
# Auto-scaling configuration
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: cursed-app-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: cursed-app
  minReplicas: 2
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Database Scaling
```cursed
yeet "dbz"

struct DatabasePool {
    primary_db Database,
    read_replicas []Database,
    connection_pool ConnectionPool
}

slay (pool *DatabasePool) read_query(query tea) QueryResult {
    sus replica Database = pool.select_read_replica()
    damn replica.execute(query)
}

slay (pool *DatabasePool) write_query(query tea) QueryResult {
    damn pool.primary_db.execute(query)
}
```

### Caching
```cursed
yeet "cachingz"

struct CacheLayer {
    redis RedisClient,
    local_cache LocalCache,
    ttl_seconds drip
}

slay (cache *CacheLayer) get(key tea) CacheResult {
    // Try local cache first
    sus local_result = cache.local_cache.get(key)
    ready (local_result.found) {
        damn local_result
    }
    
    // Try Redis
    sus redis_result = cache.redis.get(key)
    ready (redis_result.found) {
        cache.local_cache.set(key, redis_result.value, cache.ttl_seconds)
        damn redis_result
    }
    
    damn CacheResult{found: false}
}
```

## Troubleshooting

### Performance Issues
```bash
# Profile CPU usage
./zig-out/bin/cursed-zig --profile cpu app.csd

# Memory profiling
./zig-out/bin/cursed-zig --profile memory app.csd

# Check for memory leaks
valgrind --leak-check=full ./app
```

### High CPU Usage
```cursed
slay diagnose_cpu_usage() {
    sus goroutine_stats = concurrenz.get_goroutine_stats()
    vibez.spill("Active goroutines:", goroutine_stats.total)
    vibez.spill("Running goroutines:", goroutine_stats.running)
    
    ready (goroutine_stats.total > 50000) {
        vibez.spill_error("Warning: High goroutine count detected")
    }
}
```

### Memory Issues
```cursed
slay diagnose_memory() {
    sus memory_stats = memoryz.get_stats()
    vibez.spill("Heap size:", memory_stats.heap_size)
    vibez.spill("GC pressure:", memory_stats.gc_pressure)
    
    ready (memory_stats.heap_size > 500 * 1024 * 1024) {
        vibez.spill_error("Warning: High memory usage detected")
        memoryz.force_gc()
    }
}
```

### Connection Issues
```cursed
slay diagnose_connections() {
    sus connection_stats = networkz.get_connection_stats()
    vibez.spill("Active connections:", connection_stats.active)
    vibez.spill("Failed connections:", connection_stats.failed)
    
    ready (connection_stats.failed_rate > 0.1) {
        vibez.spill_error("High connection failure rate detected")
    }
}
```

## Backup and Recovery

### Database Backups
```cursed
yeet "backupz"

slay setup_automated_backups() {
    sus backup_config BackupConfig = BackupConfig{
        schedule: "0 2 * * *",  // Daily at 2 AM
        retention_days: 30,
        compression: based,
        encryption: based,
        destinations: [
            "s3://backup-bucket/cursed-app/",
            "/local/backups/"
        ]
    }
    
    backupz.schedule_backup(backup_config)
}
```

### Application State
```cursed
slay create_checkpoint() yikes<tea> {
    sus checkpoint Checkpoint = Checkpoint{
        timestamp: timez.now(),
        memory_state: memoryz.get_heap_snapshot(),
        goroutine_state: concurrenz.get_runtime_state(),
        application_state: serialize_app_state()
    }
    
    sus checkpoint_file tea = format("checkpoint_{}.dat", checkpoint.timestamp)
    write_checkpoint_file(checkpoint_file, checkpoint) fam {
        when _ -> yikes "failed to write checkpoint"
    }
}
```

## Deployment Checklist

### Pre-Deployment
- [ ] Code review completed
- [ ] All tests passing
- [ ] Memory safety validation with valgrind
- [ ] Performance benchmarks acceptable
- [ ] Security scan completed
- [ ] Documentation updated

### Deployment
- [ ] Blue-green deployment strategy
- [ ] Health checks configured
- [ ] Monitoring dashboards ready
- [ ] Rollback plan prepared
- [ ] Database migrations tested
- [ ] Configuration validated

### Post-Deployment
- [ ] Health checks passing
- [ ] Metrics flowing correctly
- [ ] Error rates within limits
- [ ] Performance benchmarks met
- [ ] User acceptance testing
- [ ] Documentation updated

---

This deployment guide provides the foundation for running CURSED applications reliably in production environments with proper monitoring, scaling, and security practices.
