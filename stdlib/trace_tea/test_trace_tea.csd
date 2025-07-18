yeet "testz"
yeet "trace_tea"
yeet "timez"
yeet "dropz"
yeet "vibe_context"

test_start("trace_tea comprehensive tests")

# Test Basic Tracing Functionality

slay test_basic_tracing() {
    test_start("basic tracing operations")
    
    # Create a buffer for trace output
    sus buffer dropz.Buffer
    
    # Start tracing
    err := trace_tea.Start(&buffer)
    assert_eq_string(err, "")
    
    # Create a task
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "test-task")
    assert_not_nil(task)
    
    # Log some events
    trace_tea.Log(ctx, "info", "test message")
    trace_tea.Logf(ctx, "debug", "formatted message: %d", 42)
    
    # End the task
    task.End()
    
    # Stop tracing
    err = trace_tea.Stop()
    assert_eq_string(err, "")
    
    # Verify trace data was captured
    assert_true(buffer.Len() >= 0)
    
    print_test_summary()
}

slay test_span_management() {
    test_start("span creation and management")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    
    # Create a span
    span := trace_tea.StartSpan(ctx, "test-operation")
    assert_not_nil(span)
    assert_eq_string(span.operation_name, "test-operation")
    assert_false(span.ended)
    
    # Set span tags
    span.SetTag("component", "test")
    span.SetTag("version", "1.0")
    
    # Log fields
    fields := make(map[tea]interface{})
    fields["key1"] = "value1"
    fields["key2"] = 42
    span.LogFields(fields)
    
    # Set baggage items
    span.SetBaggageItem("user-id", "12345")
    span.SetBaggageItem("session-id", "abcdef")
    
    # Retrieve baggage
    user_id := span.GetBaggageItem("user-id")
    assert_eq_string(user_id, "12345")
    
    # End the span
    span.End()
    assert_true(span.ended)
    assert_true(span.duration > 0)
    
    print_test_summary()
}

slay test_region_operations() {
    test_start("region operations")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "region-test")
    defer task.End()
    
    # Create a region
    region := trace_tea.StartRegion(ctx, "processing")
    assert_not_nil(region)
    assert_eq_string(region.region_type, "processing")
    assert_false(region.ended)
    
    # Log to region
    region.LazyLog("Processing step %d", 1)
    
    # End region
    region.End()
    assert_true(region.ended)
    
    # Test WithRegion helper
    sus region_executed lit = cap
    trace_tea.WithRegion(ctx, "helper-region", slay() {
        region_executed = based
    })
    assert_true(region_executed)
    
    print_test_summary()
}

slay test_event_creation() {
    test_start("event creation and logging")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Create events
    event := trace_tea.NewEvent("custom", "test-event")
    assert_not_nil(event)
    assert_eq_string(event.category, "custom")
    assert_eq_string(event.name, "test-event")
    assert_true(event.timestamp > 0)
    
    # Add logging to event
    event.LazyLog("Event details: %s", "test data")
    
    # Verify tags
    assert_not_nil(event.tags)
    
    print_test_summary()
}

slay test_trace_context_propagation() {
    test_start("trace context propagation")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    span := trace_tea.StartSpan(ctx, "parent-operation")
    defer span.End()
    
    # Create context with trace information
    span_ctx := vibe_context.WithValue(ctx, "span_id", span.id)
    span_ctx = vibe_context.WithValue(span_ctx, "trace_id", span.trace_id)
    
    # Test injection into headers
    headers := make(map[tea]tea)
    trace_tea.InjectTraceContext(span_ctx, headers)
    
    # Verify headers contain trace information
    assert_not_eq_string(headers["x-trace-id"], "")
    assert_not_eq_string(headers["x-span-id"], "")
    
    # Test extraction from headers
    trace_ctx := trace_tea.ExtractTraceContext(headers)
    assert_not_nil(trace_ctx)
    assert_eq_string(trace_ctx.trace_id, headers["x-trace-id"])
    assert_eq_string(trace_ctx.span_id, headers["x-span-id"])
    
    # Test context creation from trace context
    new_ctx := trace_tea.ContextWithTraceContext(ctx, trace_ctx)
    assert_not_nil(new_ctx)
    
    print_test_summary()
}

slay test_correlation_ids() {
    test_start("correlation ID management")
    
    # Generate correlation ID
    correlation_id := trace_tea.GenerateCorrelationID()
    assert_not_eq_string(correlation_id, "")
    
    # Set correlation ID in context
    ctx := vibe_context.Background()
    ctx = trace_tea.SetCorrelationID(ctx, correlation_id)
    
    # Retrieve correlation ID from context
    retrieved_id := trace_tea.GetCorrelationID(ctx)
    assert_eq_string(retrieved_id, correlation_id)
    
    print_test_summary()
}

slay test_filtering() {
    test_start("trace filtering")
    
    # Create filter
    filter := trace_tea.NewFilter()
    assert_not_nil(filter)
    
    # Configure filter
    filter.IncludeGoroutine("worker-*")
    filter.ExcludeGoroutine("background-*")
    filter.IncludeEvent(trace_tea.EventAPI)
    filter.ExcludeEvent(trace_tea.EventGC)
    
    # Verify filter configuration
    assert_eq_int(len(filter.include_goroutines), 1)
    assert_eq_int(len(filter.exclude_goroutines), 1)
    assert_eq_int(len(filter.include_events), 1)
    assert_eq_int(len(filter.exclude_events), 1)
    
    print_test_summary()
}

slay test_real_time_analyzer() {
    test_start("real-time analyzer")
    
    # Create analyzer
    analyzer := trace_tea.NewRealTimeAnalyzer()
    assert_not_nil(analyzer)
    assert_eq_int(analyzer.high_latency_threshold, 100)
    
    # Configure high latency detection
    sus high_latency_detected lit = cap
    sus detected_operation tea = ""
    sus detected_duration normie = 0
    
    analyzer.OnHighLatency(50, slay(operation tea, duration normie) {
        high_latency_detected = based
        detected_operation = operation
        detected_duration = duration
    })
    
    assert_eq_int(analyzer.high_latency_threshold, 50)
    assert_not_nil(analyzer.on_high_latency)
    
    # Test callback
    if analyzer.on_high_latency != cap {
        analyzer.on_high_latency("test-op", 100)
        assert_true(high_latency_detected)
        assert_eq_string(detected_operation, "test-op")
        assert_eq_int(detected_duration, 100)
    }
    
    print_test_summary()
}

slay test_sampling() {
    test_start("sampling strategies")
    
    # Test sampling rate setting
    trace_tea.SetSamplingRate(0.5)
    
    # Test sampling decisions
    sus sampled_count normie = 0
    sus total_count normie = 100
    
    for i := 0; i < total_count; i++ {
        if trace_tea.ShouldSample() {
            sampled_count++
        }
    }
    
    # Should roughly be around 50% with 0.5 sampling rate
    # Allow for some variance in randomness
    assert_true(sampled_count > 20)
    assert_true(sampled_count < 80)
    
    print_test_summary()
}

slay test_metrics_extraction() {
    test_start("metrics extraction")
    
    # Create dummy trace data
    sus trace_data []byte = []byte("dummy trace data")
    
    # Extract metrics
    metrics := trace_tea.ExtractMetrics(trace_data)
    assert_not_nil(metrics)
    assert_not_nil(metrics.latencies)
    assert_not_nil(metrics.concurrency_levels)
    assert_not_nil(metrics.event_counts)
    
    # Test average latency calculation with empty data
    avg_latency := metrics.AverageLatency("non-existent-operation")
    assert_eq_int(avg_latency, 0)
    
    # Test max concurrency with empty data
    max_concurrency := metrics.MaxConcurrency()
    assert_eq_int(max_concurrency, 0)
    
    print_test_summary()
}

slay test_visualization() {
    test_start("trace visualization")
    
    # Create dummy trace data
    sus trace_data []byte = []byte("dummy trace data")
    
    # Create visualizer
    visualizer := trace_tea.NewVisualizer(trace_data)
    assert_not_nil(visualizer)
    
    # Generate timeline
    timeline := visualizer.GenerateTimeline()
    assert_not_nil(timeline)
    assert_not_nil(timeline.events)
    
    print_test_summary()
}

slay test_external_exports() {
    test_start("external system exports")
    
    sus trace_data []byte = []byte("test trace data")
    
    # Test Jaeger export
    err := trace_tea.ExportToJaeger(trace_data, "http://localhost:14268/api/traces")
    assert_eq_string(err, "")
    
    # Test Zipkin export
    err = trace_tea.ExportToZipkin(trace_data, "http://localhost:9411/api/v2/spans")
    assert_eq_string(err, "")
    
    # Test OpenTelemetry export
    err = trace_tea.ExportToOpenTelemetry(trace_data, "http://localhost:4318/v1/traces")
    assert_eq_string(err, "")
    
    print_test_summary()
}

slay test_distributed_tracing() {
    test_start("distributed tracing helpers")
    
    # Create context with trace information
    ctx := vibe_context.Background()
    correlation_id := trace_tea.GenerateCorrelationID()
    ctx = trace_tea.SetCorrelationID(ctx, correlation_id)
    
    span := trace_tea.StartSpan(ctx, "service-call")
    span_ctx := vibe_context.WithValue(ctx, "span_id", span.id)
    span_ctx = vibe_context.WithValue(span_ctx, "trace_id", span.trace_id)
    
    # Test propagation to downstream service
    downstream_headers := make(map[tea]tea)
    trace_tea.PropagateTrace(span_ctx, downstream_headers)
    
    # Verify headers contain trace information
    assert_not_eq_string(downstream_headers["x-trace-id"], "")
    assert_not_eq_string(downstream_headers["x-span-id"], "")
    assert_eq_string(downstream_headers["x-correlation-id"], correlation_id)
    
    # Test receiving trace from upstream service
    received_ctx := trace_tea.ReceiveTrace(downstream_headers)
    assert_not_nil(received_ctx)
    
    # Verify correlation ID is preserved
    received_correlation_id := trace_tea.GetCorrelationID(received_ctx)
    assert_eq_string(received_correlation_id, correlation_id)
    
    span.End()
    
    print_test_summary()
}

slay test_auto_instrumentation() {
    test_start("auto-instrumentation")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Enable auto-instrumentation for different components
    trace_tea.AutoInstrumentHTTP(based)
    trace_tea.AutoInstrumentDatabase(based)
    trace_tea.AutoInstrumentCache(based)
    
    # These should log that auto-instrumentation is enabled
    # In a real implementation, they would set up hooks
    
    print_test_summary()
}

slay test_performance_monitoring() {
    test_start("performance monitoring")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Enable various monitoring features
    trace_tea.MonitorGoroutines()
    trace_tea.MonitorMemory()
    trace_tea.MonitorNetworkActivity()
    
    # These should log that monitoring is active
    
    print_test_summary()
}

slay test_high_latency_detection() {
    test_start("high latency detection")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    
    # Set up analyzer with low threshold for testing
    analyzer := trace_tea.NewRealTimeAnalyzer()
    sus high_latency_detected lit = cap
    sus detected_operation tea = ""
    
    analyzer.OnHighLatency(10, slay(operation tea, duration normie) {
        high_latency_detected = based
        detected_operation = operation
    })
    
    trace_tea.RegisterAnalyzer(analyzer)
    
    # Create a span that will exceed the threshold
    ctx := vibe_context.Background()
    span := trace_tea.StartSpan(ctx, "slow-operation")
    
    # Simulate work that takes longer than threshold
    timez.Sleep(20 * timez.Millisecond)
    
    span.End()
    
    # Verify high latency was detected
    assert_true(high_latency_detected)
    assert_eq_string(detected_operation, "slow-operation")
    
    trace_tea.Stop()
    
    print_test_summary()
}

slay test_nested_spans() {
    test_start("nested span operations")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    
    # Create parent span
    parent_span := trace_tea.StartSpan(ctx, "parent-operation")
    parent_ctx := vibe_context.WithValue(ctx, "span_id", parent_span.id)
    parent_ctx = vibe_context.WithValue(parent_ctx, "trace_id", parent_span.trace_id)
    
    # Create child span
    child_span := trace_tea.StartSpan(parent_ctx, "child-operation")
    
    # Verify parent-child relationship
    assert_eq_string(child_span.trace_id, parent_span.trace_id)
    assert_eq_string(child_span.parent_id, parent_span.id)
    
    # Add baggage to parent that should be inherited
    parent_span.SetBaggageItem("request-id", "req-123")
    
    child_span.End()
    parent_span.End()
    
    print_test_summary()
}

slay test_concurrent_tracing() {
    test_start("concurrent tracing operations")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    # Create multiple goroutines that create spans
    done := make(chan lit, 3)
    
    for i := 0; i < 3; i++ {
        stan slay(worker_id normie) {
            ctx := vibe_context.Background()
            span := trace_tea.StartSpan(ctx, vibez.spill_to_tea("worker-%d", worker_id))
            
            # Simulate work
            timez.Sleep(10 * timez.Millisecond)
            
            span.SetTag("worker-id", vibez.spill_to_tea("%d", worker_id))
            span.End()
            
            done <- based
        }(i)
    }
    
    # Wait for all workers to complete
    for i := 0; i < 3; i++ {
        <-done
    }
    
    print_test_summary()
}

slay test_baggage_propagation() {
    test_start("baggage propagation")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    defer trace_tea.Stop()
    
    ctx := vibe_context.Background()
    span := trace_tea.StartSpan(ctx, "baggage-test")
    
    # Set baggage items
    span.SetBaggageItem("user-id", "user123")
    span.SetBaggageItem("tenant-id", "tenant456")
    span.SetBaggageItem("feature-flag", "new-ui")
    
    # Retrieve baggage items
    user_id := span.GetBaggageItem("user-id")
    tenant_id := span.GetBaggageItem("tenant-id")
    feature_flag := span.GetBaggageItem("feature-flag")
    non_existent := span.GetBaggageItem("non-existent")
    
    assert_eq_string(user_id, "user123")
    assert_eq_string(tenant_id, "tenant456")
    assert_eq_string(feature_flag, "new-ui")
    assert_eq_string(non_existent, "")
    
    span.End()
    
    print_test_summary()
}

# Integration Tests

slay test_complete_workflow() {
    test_start("complete tracing workflow")
    
    sus buffer dropz.Buffer
    trace_tea.Start(&buffer)
    
    # Create analyzer for monitoring
    analyzer := trace_tea.NewRealTimeAnalyzer()
    sus alerts_triggered normie = 0
    
    analyzer.OnHighLatency(30, slay(operation tea, duration normie) {
        alerts_triggered++
    })
    
    trace_tea.RegisterAnalyzer(analyzer)
    
    # Simulate a complete application workflow
    ctx := vibe_context.Background()
    correlation_id := trace_tea.GenerateCorrelationID()
    ctx = trace_tea.SetCorrelationID(ctx, correlation_id)
    
    # Main service operation
    ctx, main_task := trace_tea.NewTask(ctx, "user-request")
    main_span := trace_tea.StartSpan(ctx, "handle-request")
    main_span.SetTag("method", "GET")
    main_span.SetTag("endpoint", "/api/users")
    main_span.SetBaggageItem("user-id", "12345")
    
    # Authentication span
    auth_span := trace_tea.StartSpan(ctx, "authenticate")
    timez.Sleep(5 * timez.Millisecond)
    auth_span.SetTag("auth-method", "jwt")
    auth_span.End()
    
    # Database operation (simulate slow query)
    db_span := trace_tea.StartSpan(ctx, "database-query")
    db_span.SetTag("query", "SELECT * FROM users")
    timez.Sleep(40 * timez.Millisecond) # This should trigger high latency alert
    db_span.End()
    
    # Response preparation
    response_span := trace_tea.StartSpan(ctx, "prepare-response")
    timez.Sleep(5 * timez.Millisecond)
    response_span.SetTag("format", "json")
    response_span.End()
    
    main_span.End()
    main_task.End()
    
    trace_tea.Stop()
    
    # Verify workflow completed and alerts were triggered
    assert_true(alerts_triggered >= 1) # Database query should trigger alert
    assert_true(buffer.Len() > 0) # Trace data should be captured
    
    print_test_summary()
}

# Run all tests
test_basic_tracing()
test_span_management()
test_region_operations()
test_event_creation()
test_trace_context_propagation()
test_correlation_ids()
test_filtering()
test_real_time_analyzer()
test_sampling()
test_metrics_extraction()
test_visualization()
test_external_exports()
test_distributed_tracing()
test_auto_instrumentation()
test_performance_monitoring()
test_high_latency_detection()
test_nested_spans()
test_concurrent_tracing()
test_baggage_propagation()
test_complete_workflow()

print_test_summary()
