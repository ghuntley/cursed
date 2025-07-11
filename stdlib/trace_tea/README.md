# Trace Tea Module

A comprehensive performance tracing and profiling system for CURSED that provides detailed insights into application performance, request flows, and system behavior.

## Features

- **Distributed Tracing**: Track requests across multiple services and components
- **Performance Metrics**: Detailed timing and performance analytics
- **Span Hierarchy**: Nested operation tracking with parent-child relationships
- **Event Logging**: Detailed event tracking within spans
- **Sampling**: Configurable sampling rates for high-volume applications
- **Export Formats**: JSON, CSV, and plain text export options
- **Real-time Monitoring**: Live performance dashboards and alerts
- **Analysis Tools**: Performance bottleneck identification and optimization recommendations

## Core Components

### TraceEvent
Individual events within a trace span with timing and metadata.

### TraceSpan
Represents a unit of work with start/end times, duration, and associated events.

### TraceCollector
Manages trace spans, handles sampling, and provides analytics.

### PerfMetrics
Aggregated performance metrics for analysis and reporting.

## Basic Usage

### Simple Tracing
```cursed
// Create trace collector
sus collector TraceCollector = create_trace_collector()

// Start a span
sus span TraceSpan = start_span(collector, "user_login")

// Add events and tags
span = add_event(span, "validate_credentials", {})
span = add_tag(span, "user_id", "12345")

// End the span
collector = end_span(collector, span)
```

### HTTP Request Tracing
```cursed
// Trace HTTP requests
sus request_span TraceSpan = trace_http_request(collector, "GET", "/api/users")

// Add request details
request_span = add_tag(request_span, "response_code", "200")
request_span = add_tag(request_span, "response_size", "1024")

// End request
collector = end_span(collector, request_span)
```

### Database Query Tracing
```cursed
// Trace database operations
sus db_span TraceSpan = trace_database_query(collector, "SELECT * FROM users")

// Add query metadata
db_span = add_tag(db_span, "rows_affected", "150")
db_span = add_tag(db_span, "query_time", "25ms")

// End query
collector = end_span(collector, db_span)
```

## Performance Analysis

### Generate Metrics
```cursed
// Calculate performance metrics
sus metrics PerfMetrics = calculate_metrics(collector)

// Access metrics
vibez.spill("Total requests: " + string(metrics.total_requests))
vibez.spill("Average time: " + string(metrics.avg_time) + "ms")
vibez.spill("Error rate: " + string(metrics.error_count))
```

### Performance Reports
```cursed
// Generate comprehensive report
sus report tea = generate_trace_report(collector)
vibez.spill(report)

// Analyze performance issues
sus analysis tea = analyze_performance(collector)
vibez.spill(analysis)
```

### Find Bottlenecks
```cursed
// Get slowest operations
sus slow_spans [TraceSpan] = get_slowest_spans(collector, 10)

// Filter by operation
sus login_spans [TraceSpan] = filter_spans(collector, "user_login")
```

## Export and Monitoring

### Export Trace Data
```cursed
// Export to different formats
sus json_export tea = export_traces(collector, "json")
sus csv_export tea = export_traces(collector, "csv")
sus txt_export tea = export_traces(collector, "txt")
```

### Real-time Monitoring
```cursed
// Create performance monitor
sus monitor tea = create_performance_monitor(collector)
vibez.spill(monitor)
```

## Advanced Features

### Sampling Configuration
```cursed
// Configure sampling
collector.sampling_rate = 0.1  // 10% sampling
collector.max_spans = 5000     // Maximum spans to keep
```

### Span Hierarchy
```cursed
// Create parent-child relationships
sus parent_span TraceSpan = start_span(collector, "process_order")
sus child_span TraceSpan = start_span(collector, "validate_payment")
child_span.parent_span_id = parent_span.span_id
```

### Custom Events
```cursed
// Add custom events with metadata
sus metadata map[tea]tea = {}
metadata["user_id"] = "12345"
metadata["action"] = "login_attempt"
metadata["ip_address"] = "192.168.1.100"

span = add_event(span, "security_event", metadata)
```

## Trace Analysis

### Performance Metrics
- **Throughput**: Requests per unit time
- **Latency**: Response time distribution
- **Error Rate**: Percentage of failed requests
- **Resource Usage**: CPU, memory, network utilization

### Bottleneck Identification
- **Slow Queries**: Database operations exceeding thresholds
- **Long API Calls**: External service calls with high latency
- **Memory Leaks**: Operations with increasing memory usage
- **CPU Intensive**: Operations consuming excessive CPU

### Optimization Recommendations
- **Caching**: Identify cacheable operations
- **Database Optimization**: Suggest query improvements
- **Parallelization**: Identify sequential operations that can be parallelized
- **Resource Scaling**: Recommend scaling based on usage patterns

## Integration Examples

### Web Application Tracing
```cursed
// Trace complete web request
sus request_span TraceSpan = start_span(collector, "web_request")
request_span = add_tag(request_span, "method", "POST")
request_span = add_tag(request_span, "endpoint", "/api/users")

// Trace authentication
sus auth_span TraceSpan = start_span(collector, "authenticate")
auth_span.parent_span_id = request_span.span_id
collector = end_span(collector, auth_span)

// Trace database operations
sus db_span TraceSpan = start_span(collector, "db_query")
db_span.parent_span_id = request_span.span_id
collector = end_span(collector, db_span)

// End request
collector = end_span(collector, request_span)
```

### Microservice Tracing
```cursed
// Trace across service boundaries
sus service_span TraceSpan = start_span(collector, "order_service")
service_span = add_tag(service_span, "service", "order-api")
service_span = add_tag(service_span, "version", "1.2.3")

// Trace external service calls
sus external_span TraceSpan = start_span(collector, "payment_service")
external_span.parent_span_id = service_span.span_id
external_span = add_tag(external_span, "service", "payment-api")
external_span = add_tag(external_span, "timeout", "30s")

collector = end_span(collector, external_span)
collector = end_span(collector, service_span)
```

### Function Tracing
```cursed
// Trace function execution
sus function_result tea = trace_function(collector, "calculate_totals", "business_logic")
```

## Configuration

### Trace Collector Settings
```cursed
// Enable/disable tracing
collector.enabled = based

// Set sampling rate (0.0 to 1.0)
collector.sampling_rate = 0.5

// Set maximum spans to keep in memory
collector.max_spans = 10000
```

### Export Configuration
```cursed
// Configure export formats
sus json_config map[tea]tea = {}
json_config["pretty"] = "true"
json_config["include_events"] = "true"
```

## Best Practices

1. **Meaningful Span Names**: Use descriptive names that clearly identify the operation
2. **Appropriate Granularity**: Balance detail with performance overhead
3. **Consistent Tagging**: Use standardized tag names across your application
4. **Sampling Strategy**: Use appropriate sampling rates for different environments
5. **Resource Management**: Clean up old traces to prevent memory issues
6. **Error Handling**: Always trace errors and exceptions
7. **Performance Impact**: Monitor the tracing overhead itself

## Performance Considerations

### Tracing Overhead
- **Memory Usage**: Spans and events consume memory
- **CPU Impact**: Trace collection has minimal CPU overhead
- **Network Overhead**: Exporting traces requires network bandwidth
- **Storage**: Persistent trace storage requirements

### Optimization Strategies
- **Sampling**: Use sampling to reduce overhead in high-volume applications
- **Async Processing**: Process and export traces asynchronously
- **Batch Operations**: Batch trace exports for efficiency
- **Retention Policies**: Implement appropriate data retention policies

## Monitoring and Alerting

### Performance Thresholds
```cursed
// Define performance thresholds
sus slow_threshold normie = 1000  // 1 second
sus error_threshold normie = 5    // 5% error rate
```

### Automated Analysis
```cursed
// Automated performance analysis
sus metrics PerfMetrics = calculate_metrics(collector)

vibes metrics.avg_time > slow_threshold {
    vibez.spill("ALERT: High average response time")
}

vibes metrics.error_count > error_threshold {
    vibez.spill("ALERT: High error rate")
}
```

## Testing

Test the trace tea module:
```bash
cargo run --bin cursed stdlib/trace_tea/simple_test.csd
```

## Integration with Other Modules

### Logging Integration
Correlate traces with log entries using trace IDs.

### Metrics Integration
Export performance metrics to monitoring systems.

### Error Handling Integration
Automatically trace errors and exceptions.

### Configuration Integration
Configure tracing settings through application configuration.

This tracing system provides comprehensive performance monitoring and analysis capabilities for CURSED applications, enabling developers to identify bottlenecks, optimize performance, and ensure reliable operation.
