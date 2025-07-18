# trace_tea - Distributed Tracing and Observability Module

The `trace_tea` module provides comprehensive distributed tracing and observability capabilities for CURSED applications. It enables monitoring, debugging, and performance analysis of distributed systems with support for popular tracing backends.

## Features

### Core Tracing Capabilities
- **Span Management**: Create, configure, and manage distributed trace spans
- **Task Tracking**: High-level task abstraction for logical operation grouping  
- **Region Monitoring**: Fine-grained execution region tracking within tasks
- **Event Logging**: Structured event logging with categorization

### Distributed Tracing
- **Trace Context Propagation**: Automatic trace context injection/extraction for HTTP headers
- **Correlation IDs**: Request correlation across service boundaries
- **Baggage Handling**: Cross-service metadata propagation
- **Parent-Child Relationships**: Nested span hierarchies for complex operations

### Observability Features
- **Real-time Analysis**: Live monitoring with configurable alerts
- **Performance Monitoring**: Latency tracking and performance metrics
- **Sampling Strategies**: Configurable sampling rates for production efficiency
- **Filtering**: Selective tracing to reduce overhead and noise

### Integration Support
- **Jaeger Export**: Native Jaeger tracing backend integration
- **Zipkin Export**: Zipkin-compatible trace data export
- **OpenTelemetry**: OpenTelemetry standard compliance
- **Auto-instrumentation**: Automatic instrumentation for HTTP, database, and cache operations

## Quick Start

### Basic Tracing

```csd
yeet "trace_tea"
yeet "dropz"
yeet "vibe_context"

slay basic_example() {
    # Start tracing to a buffer
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Create a task for logical grouping
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "user-request")
    defer task.End()
    
    # Create spans for operations
    span := trace_tea.StartSpan(ctx, "process-data")
    span.SetTag("operation", "data-processing")
    
    # Log structured events
    trace_tea.Log(ctx, "info", "Processing started")
    
    # Simulate work
    timez.Sleep(50 * timez.Millisecond)
    
    span.End()
    
    vibez.spill("Trace completed: %d bytes", buffer.Len())
}
```

### Distributed Service Communication

```csd
slay service_call_example() {
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    correlation_id := trace_tea.GenerateCorrelationID()
    ctx = trace_tea.SetCorrelationID(ctx, correlation_id)
    
    # Create span for service call
    span := trace_tea.StartSpan(ctx, "call-user-service")
    span.SetTag("service", "user-service")
    span.SetBaggageItem("user-id", "12345")
    
    # Prepare headers for downstream service
    headers := make(map[tea]tea)
    trace_tea.PropagateTrace(ctx, headers)
    
    # Headers now contain:
    # x-trace-id: <trace-id>
    # x-span-id: <span-id>  
    # x-correlation-id: <correlation-id>
    
    # Simulate service call
    response := call_downstream_service(headers)
    
    span.SetTag("response-status", "200")
    span.End()
}

slay handle_upstream_request(headers map[tea]tea) {
    # Extract trace context from incoming request
    ctx := trace_tea.ReceiveTrace(headers)
    
    # Continue the trace in this service
    span := trace_tea.StartSpan(ctx, "handle-request")
    defer span.End()
    
    # Access baggage from upstream services
    user_id := span.GetBaggageItem("user-id")
    vibez.spill("Processing request for user: %s", user_id)
}
```

### Performance Monitoring

```csd
slay monitoring_example() {
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Configure real-time analysis
    analyzer := trace_tea.NewRealTimeAnalyzer()
    analyzer.OnHighLatency(100, slay(operation tea, duration normie) {
        vibez.spill("⚠️  High latency in %s: %dms", operation, duration)
    })
    
    trace_tea.RegisterAnalyzer(analyzer)
    
    # Simulate operations with varying latencies
    ctx := vibe_context.Background()
    
    # Fast operation
    fast_span := trace_tea.StartSpan(ctx, "fast-operation")
    timez.Sleep(30 * timez.Millisecond)
    fast_span.End()
    
    # Slow operation (will trigger alert)
    slow_span := trace_tea.StartSpan(ctx, "slow-operation")
    timez.Sleep(150 * timez.Millisecond)
    slow_span.End() # Alert: High latency in slow-operation: 150ms
}
```

### Selective Tracing with Filters

```csd
slay filtering_example() {
    # Create selective filter
    filter := trace_tea.NewFilter()
    filter.IncludeGoroutine("worker-*")    # Only trace worker goroutines
    filter.IncludeEvent(trace_tea.EventAPI)  # Only API events
    filter.ExcludeEvent(trace_tea.EventGC)   # Exclude GC events
    
    sus buffer dropz.Buffer
    trace_tea.StartWithFilter(&buffer, filter)
    defer trace_tea.Stop()
    
    # Only operations matching the filter will be traced
}
```

### Auto-instrumentation

```csd
slay auto_instrumentation_example() {
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Enable automatic instrumentation
    trace_tea.AutoInstrumentHTTP(based)      # Auto-trace HTTP requests
    trace_tea.AutoInstrumentDatabase(based)  # Auto-trace DB operations
    trace_tea.AutoInstrumentCache(based)     # Auto-trace cache operations
    
    # HTTP requests, database queries, and cache operations
    # will now be automatically traced
}
```

## API Reference

### Core Types

#### Span
Represents a single operation in a distributed trace.

```csd
be_like Span squad {
    id tea
    trace_id tea
    parent_id tea
    operation_name tea
    start_time normie
    duration normie
    tags map[tea]tea
    logs []LogEntry
    baggage map[tea]tea
    ended lit
}
```

**Methods:**
- `SetTag(key, value tea)` - Add metadata tag
- `LogFields(fields map[tea]interface{})` - Log structured data
- `SetBaggageItem(key, value tea)` - Set baggage for propagation
- `GetBaggageItem(key tea) tea` - Retrieve baggage value
- `End()` - Complete the span

#### Task
High-level abstraction for grouping related operations.

```csd
be_like Task squad {
    id tea
    name tea
    start_time normie
    parent_id tea
    tags map[tea]tea
    data interface{}
    deterministic lit
    ended lit
}
```

**Methods:**
- `End()` - Complete the task
- `LazyLog(fmt tea, values ...interface{})` - Log formatted message
- `SetDeterministic(deterministic lit)` - Set deterministic flag

### Core Functions

#### Lifecycle Management
```csd
slay Start(w io.Writer) tea                    # Start tracing
slay Stop() tea                                # Stop tracing and flush
```

#### Span Operations
```csd
slay StartSpan(ctx vibe_context.Context, operation_name tea) *Span
slay WithSpan(ctx vibe_context.Context, name tea, fn slay(vibe_context.Context))
```

#### Task Operations
```csd
slay NewTask(ctx vibe_context.Context, taskType tea) (vibe_context.Context, *Task)
```

#### Region Operations
```csd
slay StartRegion(ctx vibe_context.Context, regionType tea) *Region
slay WithRegion(ctx vibe_context.Context, regionType tea, fn slay())
```

#### Event Logging
```csd
slay Log(ctx vibe_context.Context, category, message tea)
slay Logf(ctx vibe_context.Context, category, format tea, args ...interface{})
slay NewEvent(category, name tea) *Event
```

### Distributed Tracing

#### Context Propagation
```csd
slay InjectTraceContext(ctx vibe_context.Context, headers map[tea]tea)
slay ExtractTraceContext(headers map[tea]tea) *TraceContext
slay ContextWithTraceContext(ctx vibe_context.Context, trace_ctx *TraceContext) vibe_context.Context
```

#### Correlation Management
```csd
slay GenerateCorrelationID() tea
slay SetCorrelationID(ctx vibe_context.Context, correlation_id tea) vibe_context.Context
slay GetCorrelationID(ctx vibe_context.Context) tea
```

#### Service Communication
```csd
slay PropagateTrace(ctx vibe_context.Context, downstream_headers map[tea]tea)
slay ReceiveTrace(upstream_headers map[tea]tea) vibe_context.Context
```

### Advanced Features

#### Filtering
```csd
slay NewFilter() *Filter
slay (f *Filter) IncludeGoroutine(pattern tea)
slay (f *Filter) ExcludeGoroutine(pattern tea)
slay (f *Filter) IncludeEvent(event_type tea)
slay (f *Filter) ExcludeEvent(event_type tea)
slay StartWithFilter(w io.Writer, filter *Filter) tea
```

#### Real-time Analysis
```csd
slay NewRealTimeAnalyzer() *RealTimeAnalyzer
slay (a *RealTimeAnalyzer) OnHighLatency(threshold normie, callback slay(tea, normie))
slay (a *RealTimeAnalyzer) OnDeadlock(callback slay(tea))
slay RegisterAnalyzer(analyzer *RealTimeAnalyzer)
```

#### Sampling
```csd
slay SetSamplingRate(rate meal)
slay ShouldSample() lit
```

#### Metrics and Visualization
```csd
slay ExtractMetrics(trace_data []byte) *Metrics
slay (m *Metrics) AverageLatency(operation tea) normie
slay (m *Metrics) MaxConcurrency() normie

slay NewVisualizer(trace_data []byte) *Visualizer  
slay (v *Visualizer) GenerateTimeline() *Timeline
```

### External Integration

#### Export Functions
```csd
slay ExportToJaeger(trace_data []byte, jaeger_endpoint tea) tea
slay ExportToZipkin(trace_data []byte, zipkin_endpoint tea) tea
slay ExportToOpenTelemetry(trace_data []byte, otel_endpoint tea) tea
```

#### Auto-instrumentation
```csd
slay AutoInstrumentHTTP(enabled lit)
slay AutoInstrumentDatabase(enabled lit)
slay AutoInstrumentCache(enabled lit)
```

#### Performance Monitoring
```csd
slay MonitorGoroutines()
slay MonitorMemory()
slay MonitorNetworkActivity()
```

## Event Categories

The module provides predefined event categories for consistent classification:

```csd
const (
    EventGoroutine   = "goroutine"     # Goroutine lifecycle events
    EventNet         = "net"           # Network operations
    EventSyscall     = "syscall"       # System calls
    EventMemory      = "memory"        # Memory operations
    EventCPUSample   = "cpu-sample"    # CPU profiling samples
    EventConcurrency = "concurrency"   # Concurrency primitives
    EventGC          = "gc"            # Garbage collection
    EventBlock       = "block"         # Blocking operations
    EventUserDefined = "user"          # User-defined events
    EventAPI         = "api"           # API calls
    EventDatabase    = "database"      # Database operations
    EventCache       = "cache"         # Cache operations
    EventFile        = "file"          # File operations
    EventCompute     = "compute"       # Computation-heavy tasks
    EventAsyncWork   = "async"         # Asynchronous operations
    EventNetwork     = "network"       # Network communication
    EventRender      = "render"        # UI rendering
    EventLogger      = "logger"        # Logging operations
    EventPerformance = "performance"   # Performance metrics
)
```

## Best Practices

### 1. Use Correlation IDs
Always generate and propagate correlation IDs for request tracking:

```csd
correlation_id := trace_tea.GenerateCorrelationID()
ctx = trace_tea.SetCorrelationID(ctx, correlation_id)
```

### 2. Set Meaningful Tags
Add contextual information to spans:

```csd
span.SetTag("user-id", user_id)
span.SetTag("endpoint", "/api/users")
span.SetTag("method", "GET")
```

### 3. Use Baggage for Cross-Service Data
Propagate important context across service boundaries:

```csd
span.SetBaggageItem("tenant-id", tenant_id)
span.SetBaggageItem("feature-flag", "new-ui")
```

### 4. Configure Sampling for Production
Use appropriate sampling rates for production environments:

```csd
trace_tea.SetSamplingRate(0.1) # Sample 10% of traces
```

### 5. Filter Noisy Events
Exclude high-frequency, low-value events:

```csd
filter := trace_tea.NewFilter()
filter.ExcludeEvent(trace_tea.EventGC)
filter.ExcludeEvent(trace_tea.EventCPUSample)
```

### 6. Monitor Performance
Set up real-time alerts for performance issues:

```csd
analyzer := trace_tea.NewRealTimeAnalyzer()
analyzer.OnHighLatency(500, slay(operation tea, duration normie) {
    alert_system.Send("High latency detected in " + operation)
})
```

## Integration Examples

### Jaeger Integration

```csd
slay setup_jaeger_tracing() {
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    
    # ... application code ...
    
    trace_tea.Stop()
    
    # Export to Jaeger
    err := trace_tea.ExportToJaeger(buffer.Bytes(), "http://jaeger:14268/api/traces")
    if err != "" {
        vibez.spill("Failed to export to Jaeger: %s", err)
    }
}
```

### Microservice Architecture

```csd
# Service A
slay call_service_b(ctx vibe_context.Context, data tea) tea {
    span := trace_tea.StartSpan(ctx, "call-service-b")
    defer span.End()
    
    headers := make(map[tea]tea)
    trace_tea.PropagateTrace(ctx, headers)
    
    response := http_client.Post("http://service-b/api", data, headers)
    span.SetTag("response-status", response.status)
    
    damn response.body
}

# Service B
slay handle_request(headers map[tea]tea, data tea) tea {
    ctx := trace_tea.ReceiveTrace(headers)
    span := trace_tea.StartSpan(ctx, "process-request")
    defer span.End()
    
    # Process request with full trace context
    result := process_data(ctx, data)
    
    damn result
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/trace_tea/test_trace_tea.csd
```

The test suite covers:
- Basic tracing operations
- Span lifecycle management
- Distributed trace propagation
- Performance monitoring
- Real-time analysis
- External system integration
- Concurrent tracing scenarios
- Complete end-to-end workflows

## Dependencies

- `testz` - Testing framework
- `timez` - Time operations
- `dropz` - I/O operations  
- `atomic_drip` - Atomic operations
- `vibe_context` - Context management

## Performance Considerations

- **Sampling**: Use appropriate sampling rates in production
- **Filtering**: Filter out high-frequency, low-value events
- **Buffering**: Configure appropriate buffer sizes for trace data
- **Async Export**: Export trace data asynchronously to avoid blocking
- **Memory Management**: Monitor memory usage with large trace volumes

## Security Notes

- Avoid including sensitive data in span tags or logs
- Use baggage carefully - it's propagated across service boundaries
- Implement proper authentication for trace export endpoints
- Consider encryption for trace data in transit and at rest
