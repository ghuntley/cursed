# Monitoring Module

## Why This Module Exists

The `monitoring` module provides comprehensive production-grade monitoring, metrics collection, and observability infrastructure for CURSED applications. Modern applications require deep insights into performance, errors, resource usage, and business metrics to maintain reliability and optimize performance.

The module exists because:
- **Production Reliability**: Applications need real-time visibility into system health, error rates, and performance characteristics
- **Performance Optimization**: Detailed metrics enable identification of bottlenecks, memory leaks, and inefficient code paths
- **Business Intelligence**: Applications need to track business metrics alongside technical metrics for holistic understanding
- **Incident Response**: Rapid problem detection and diagnosis requires comprehensive monitoring infrastructure
- **Compliance Requirements**: Many industries require detailed logging and monitoring for regulatory compliance

## Why Testing Is Critical

Monitoring system testing is essential because:
- **Mission Critical Nature**: Monitoring systems must continue operating during application failures - they cannot fail when needed most
- **Performance Impact**: Poorly implemented monitoring can significantly degrade application performance through overhead
- **Memory Leaks**: Metric collection systems accumulate data over time and can consume unbounded memory without proper testing
- **Concurrency Safety**: Metrics are collected from multiple goroutines simultaneously and race conditions can corrupt data
- **Integration Complexity**: Monitoring systems integrate with external services (Prometheus, Grafana, etc.) that must be tested

## Implementation Rationale

### Key Design Decisions:

**1. Zero-Overhead Instrumentation**
- Metrics collection uses atomic operations for minimal performance impact
- Sampling techniques reduce overhead for high-frequency events
- Conditional compilation allows disabling monitoring in performance-critical builds

**2. Multi-Backend Architecture**
- Pluggable backends support Prometheus, StatsD, CloudWatch, custom endpoints
- Unified API prevents vendor lock-in while enabling best-of-breed integrations
- Local storage with configurable retention for offline analysis

**3. Hierarchical Metric Organization**
- Namespace-based organization prevents metric name collisions
- Tag-based dimensionality enables rich querying and aggregation
- Automatic system metric collection (memory, CPU, goroutines)

**4. Real-Time Alerting Integration**
- Built-in threshold monitoring with configurable alerting rules
- Integration with PagerDuty, Slack, email for incident response
- Circuit breaker patterns prevent alert storms during outages

## API Reference

### Core Metric Types

#### `Counter`
**Purpose**: Monotonically increasing values (requests, errors, bytes sent)
**Operations**: Increment only, never decreases
**Use Case**: Event counting, cumulative measurements

```cursed
sus request_counter = monitoring.counter("http_requests_total", [
    "method", "status_code"
])

request_counter.inc(["GET", "200"])  # Increment by 1
request_counter.add(5, ["POST", "201"])  # Increment by specific amount
```

#### `Gauge`
**Purpose**: Values that can increase or decrease (memory usage, queue length, temperature)
**Operations**: Set, increment, decrement
**Use Case**: Current state measurements

```cursed
sus memory_gauge = monitoring.gauge("memory_usage_bytes")
memory_gauge.set(1024 * 1024 * 100)  # Set to 100MB
memory_gauge.add(1024)  # Increase by 1KB
memory_gauge.sub(512)   # Decrease by 512B
```

#### `Histogram`
**Purpose**: Distribution of values with configurable buckets (request latency, response sizes)
**Operations**: Observe values, automatic bucket assignment
**Use Case**: Latency percentiles, size distributions

```cursed
sus latency_histogram = monitoring.histogram("request_duration_seconds", [
    0.001, 0.01, 0.1, 1.0, 10.0  # Bucket boundaries
])

sus start_time = timez.now()
# ... handle request ...
sus duration = timez.since(start_time)
latency_histogram.observe(duration)
```

#### `Summary`
**Purpose**: Sliding window quantile calculations (P50, P95, P99 metrics)
**Operations**: Observe values, automatic quantile calculation
**Use Case**: Real-time percentile tracking without predefined buckets

### Metric Collection and Export

#### `Registry`
**Purpose**: Central metric storage and management
**Features**: Namespace isolation, metric discovery, export coordination

```cursed
# Create isolated metric registry
sus app_registry = monitoring.registry("myapp")

# Register metrics
sus errors = app_registry.counter("errors_total", ["component"])
sus latency = app_registry.histogram("latency_seconds")

# Export all metrics
sus metrics_text = app_registry.export_prometheus()
```

#### `Collector`
**Purpose**: Custom metric collection logic
**Interface**: Implement `collect() []Metric` method
**Use Case**: Dynamic metrics, external system integration

### Real-Time Monitoring

#### `AlertManager`
**Purpose**: Threshold-based alerting with notification integration
**Features**: Rule evaluation, notification routing, alert suppression

```cursed
sus alert_manager = monitoring.alert_manager()

# Define alert rule
alert_manager.add_rule({
    name: "high_error_rate",
    query: "rate(errors_total[5m]) > 0.1",
    threshold: 0.1,
    duration: 60,  # seconds
    actions: [
        monitoring.SlackAlert{webhook: "https://hooks.slack.com/..."},
        monitoring.EmailAlert{to: ["ops@company.com"]}
    ]
})

# Start alert evaluation
alert_manager.start()
```

#### `Dashboard`
**Purpose**: Real-time metric visualization and monitoring interface
**Features**: Graph rendering, threshold visualization, export capabilities

## Usage Examples

### Basic Application Monitoring
```cursed
yeet "monitoring"
yeet "networkz"
yeet "timez"

# Initialize monitoring
sus registry = monitoring.default_registry()
sus requests = registry.counter("http_requests", ["endpoint", "method"])
sus latency = registry.histogram("request_latency_seconds")
sus active_connections = registry.gauge("active_connections")

# Instrument HTTP handler
slay handle_request(endpoint tea, method tea) {
    sus start_time = timez.now()
    defer {
        sus duration = timez.since(start_time)
        requests.inc([endpoint, method])
        latency.observe(duration)
    }
    
    active_connections.inc()
    defer active_connections.dec()
    
    # Handle request logic...
}
```

### System Resource Monitoring
```cursed
# Auto-collect system metrics
sus system_collector = monitoring.system_collector([
    "memory.used",
    "memory.available", 
    "cpu.usage",
    "goroutines.count",
    "gc.pause_time"
])

registry.register_collector("system", system_collector)

# Custom business metric
sus orders = registry.counter("orders_processed", ["product_type"])

slay process_order(product_type tea) {
    # Business logic...
    orders.inc([product_type])
}
```

### Performance Profiling Integration
```cursed
# Integrate with CURSED's profiler
sus profiler = monitoring.profiler()

# Profile critical functions
slay critical_algorithm() {
    sus span = profiler.start_span("critical_algorithm")
    defer span.end()
    
    # Algorithm implementation...
    span.add_event("checkpoint_1")
    # More work...
    span.add_event("checkpoint_2")
}

# Export profiling data
profiler.export_pprof("/tmp/profile.pb.gz")
```

### Multi-Backend Configuration
```cursed
# Configure multiple monitoring backends
sus config = monitoring.Config{
    backends: [
        monitoring.PrometheusBackend{
            endpoint: "http://prometheus:9090",
            scrape_interval: 15  # seconds
        },
        monitoring.StatsDBackend{
            address: "statsd:8125",
            prefix: "myapp"
        },
        monitoring.LocalBackend{
            retention: 24 * 60 * 60,  # 24 hours
            storage_path: "/var/lib/metrics"
        }
    ]
}

monitoring.initialize(config)
```

### Custom Metric Collection
```cursed
# Implement custom collector
squad DatabaseCollector {
    db: Database
}

slay (collector DatabaseCollector) collect() []monitoring.Metric {
    sus connection_count = collector.db.connection_count()
    sus query_latency = collector.db.average_query_latency()
    
    damn [
        monitoring.gauge_metric("db_connections", connection_count),
        monitoring.histogram_metric("db_query_latency", query_latency)
    ]
}

# Register custom collector
registry.register_collector("database", DatabaseCollector{db: db})
```

## Performance Considerations

### Metric Collection Overhead

**High-Frequency Metrics**: Use atomic operations and avoid string allocations
```cursed
# Efficient counter increment
counter.inc_fast()  # Atomic increment without label processing

# Inefficient - creates string allocation
counter.inc([generate_dynamic_label()])  # Avoid in hot paths
```

**Sampling Strategies**: Reduce overhead for high-volume events
```cursed
# Sample 1% of requests for detailed metrics
ready (mathz.random() < 0.01) {
    detailed_histogram.observe(value)
}
```

### Memory Management

**Metric Cardinality**: Limit label combinations to prevent memory exhaustion
```cursed
# Dangerous - potentially millions of combinations
counter.inc([user_id, timestamp, request_id])  # High cardinality

# Safe - bounded label space
counter.inc([method, status_code])  # Low cardinality
```

**Retention Policies**: Configure appropriate data retention
```cursed
monitoring.configure_retention({
    high_frequency_metrics: 1 * 60 * 60,    # 1 hour
    business_metrics: 7 * 24 * 60 * 60,     # 7 days  
    system_metrics: 24 * 60 * 60            # 24 hours
})
```

### Export Optimization

1. **Batch Exports**: Group metric updates to reduce network overhead
2. **Compression**: Enable gzip compression for large metric payloads
3. **Async Export**: Use background goroutines for metric shipping
4. **Circuit Breakers**: Prevent cascading failures when monitoring backends are down

## Security Considerations

### Sensitive Data Protection

**Threat**: Accidentally exposing sensitive data through metric labels
**Mitigation**: Sanitize metric labels and use allowlists for label values

```cursed
# Vulnerable - could expose PII
counter.inc([user_email, request_path])  # Dangerous

# Secure - hash or categorize sensitive data  
counter.inc([hash(user_email)[0:8], sanitize_path(request_path)])
```

### Monitoring System Access

**Threat**: Unauthorized access to monitoring data reveals system architecture and usage patterns
**Mitigation**: Implement authentication and authorization for monitoring endpoints

```cursed
# Secure monitoring endpoint
sus monitoring_server = networkz.server(":9090")
monitoring_server.add_middleware(auth.require_token("monitoring"))
monitoring_server.handle("/metrics", registry.prometheus_handler())
```

### Resource Exhaustion Attacks

**Threat**: Malicious actors can create excessive metrics to exhaust system resources
**Mitigation**: Rate limiting, cardinality limits, and monitoring resource usage

```cursed
# Protect against cardinality bombs
sus protected_counter = monitoring.counter_with_limits("requests", {
    max_cardinality: 1000,
    rate_limit: 10000  # metrics per second
})
```

### Data Retention Compliance

**Threat**: Regulatory requirements for data retention and deletion
**Mitigation**: Implement automated data lifecycle management

```cursed
monitoring.configure_compliance({
    gdpr_deletion: based,
    retention_periods: {
        personal_metrics: 30 * 24 * 60 * 60,  # 30 days
        business_metrics: 365 * 24 * 60 * 60  # 1 year
    }
})
```

## Error Handling and Reliability

### Monitoring System Resilience
```cursed
# Monitoring must not fail the application
slay resilient_metric_collection() {
    sus counter = monitoring.counter("operations")
    
    # Safely record metric - never panic on monitoring failures
    monitor_safely(slay() {
        counter.inc()
    })
}

slay monitor_safely(operation: slay()) {
    # Recover from monitoring panics
    defer {
        ready (err := recover()) {
            vibez.spill("Monitoring error:", err)
            # Application continues normally
        }
    }
    operation()
}
```

### Circuit Breaker Integration
```cursed
# Prevent monitoring from overwhelming backend systems
sus circuit_breaker = monitoring.circuit_breaker({
    failure_threshold: 5,
    recovery_timeout: 30  # seconds
})

slay export_with_circuit_breaker(metrics []Metric) {
    ready (circuit_breaker.state() == "OPEN") {
        # Skip export when circuit is open
        damn
    }
    
    # Attempt export with circuit breaker protection
    circuit_breaker.call(slay() {
        prometheus_client.export(metrics)
    })
}
```

## Integration with CURSED Ecosystem

### Automatic Goroutine Monitoring
```cursed
# Monitor goroutine lifecycle automatically
monitoring.enable_goroutine_tracking()

go {
    # Goroutine metrics automatically collected:
    # - goroutine.count
    # - goroutine.duration  
    # - goroutine.panic_count
    
    # Application logic...
}
```

### Memory Allocator Integration  
```cursed
# Monitor memory usage through arena allocators
sus arena = memoryz.arena_with_monitoring("request_handler")

# Automatic metrics:
# - arena.allocations
# - arena.peak_usage
# - arena.current_usage
```

### Error System Integration
```cursed
# Automatically track error rates and types
slay api_handler() yikes<tea> {
    # Error metrics automatically recorded when yikes is used
    ready (validate_input()) {
        yikes "validation_failed"  # Increments errors.validation_failed
    }
    
    damn "success"
}
```

The monitoring module provides comprehensive observability while maintaining CURSED's performance characteristics and safety guarantees. It seamlessly integrates with the broader ecosystem to provide automatic insights into application behavior.
