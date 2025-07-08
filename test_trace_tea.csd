yeet "testz"
yeet "trace_tea"
yeet "vibe_context"
yeet "io"

fr fr Test basic tracing functionality
slay test_basic_tracing() {
    test_start("Basic tracing")
    
    fr fr Start tracing
    err := trace_tea.Start(cap)
    assert_eq_string(err, "")
    
    fr fr Check if tracing is active
    assert_true(trace_tea.IsTraceActive())
    
    fr fr Stop tracing
    err = trace_tea.Stop()
    assert_eq_string(err, "")
    
    fr fr Check if tracing is inactive
    assert_false(trace_tea.IsTraceActive())
}

fr fr Test task creation and management
slay test_task_management() {
    test_start("Task management")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Create a task
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "test-task")
    
    fr fr Check task registry
    registry := trace_tea.GetTaskRegistry()
    assert_eq_int(len(registry), 1)
    
    fr fr End task
    task.End()
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test region functionality
slay test_region_functionality() {
    test_start("Region functionality")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Create a task
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "region-test")
    
    fr fr Start a region
    region := trace_tea.StartRegion(ctx, "test-region")
    
    fr fr Add log to region
    region.LazyLog("Region log test")
    
    fr fr End region
    region.End()
    
    fr fr End task
    task.End()
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test event logging
slay test_event_logging() {
    test_start("Event logging")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Create context
    ctx := vibe_context.Background()
    
    fr fr Log simple event
    trace_tea.Log(ctx, trace_tea.EventUserDefined, "Test message")
    
    fr fr Log formatted event
    trace_tea.Logf(ctx, trace_tea.EventAPI, "API call %d", 42)
    
    fr fr Create and log custom event
    event := trace_tea.NewEvent(trace_tea.EventDatabase, "db-query")
    event.LazyLog("Database query executed")
    
    fr fr Check trace buffer has content
    buffer := trace_tea.GetTraceBuffer()
    assert_true(len(buffer) > 0)
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test WithRegion helper
slay test_with_region() {
    test_start("WithRegion helper")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Create context
    ctx := vibe_context.Background()
    
    fr fr Use WithRegion helper
    trace_tea.WithRegion(ctx, "helper-region", func() {
        fr fr Simulate some work
        x := 1 + 2
        assert_eq_int(x, 3)
    })
    
    fr fr Check trace buffer has content
    buffer := trace_tea.GetTraceBuffer()
    assert_true(len(buffer) > 0)
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test WithSpan helper
slay test_with_span() {
    test_start("WithSpan helper")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Create context
    ctx := vibe_context.Background()
    
    fr fr Use WithSpan helper
    trace_tea.WithSpan(ctx, "span-task", func(spanCtx vibe_context.Context) {
        fr fr Simulate work with span context
        trace_tea.Log(spanCtx, trace_tea.EventCompute, "Computation work")
    })
    
    fr fr Check trace buffer has content
    buffer := trace_tea.GetTraceBuffer()
    assert_true(len(buffer) > 0)
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test filter functionality
slay test_filter_functionality() {
    test_start("Filter functionality")
    
    fr fr Create filter
    filter := trace_tea.NewFilter()
    
    fr fr Add filter rules
    filter.IncludeGoroutine("worker-*")
    filter.ExcludeEvent(trace_tea.EventGC)
    filter.IncludeEvent(trace_tea.EventAPI)
    
    fr fr Start tracing with filter
    err := trace_tea.StartWithFilter(cap, filter)
    assert_eq_string(err, "")
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test real-time analyzer
slay test_real_time_analyzer() {
    test_start("Real-time analyzer")
    
    fr fr Create analyzer
    analyzer := trace_tea.NewRealTimeAnalyzer()
    
    fr fr Set high latency handler
    analyzer.OnHighLatency(100000000, func(taskName tea, duration normie) {
        fr fr Handler for high latency
        assert_true(duration > 0)
    })
    
    fr fr Set deadlock handler
    analyzer.OnDeadlock(func(taskName tea) {
        fr fr Handler for deadlock
        assert_true(len(taskName) > 0)
    })
    
    fr fr Register analyzer
    trace_tea.RegisterAnalyzer(analyzer)
}

fr fr Test visualizer
slay test_visualizer() {
    test_start("Visualizer")
    
    fr fr Start tracing to collect data
    trace_tea.Start(cap)
    
    fr fr Create some trace data
    ctx := vibe_context.Background()
    ctx, task := trace_tea.NewTask(ctx, "viz-task")
    trace_tea.Log(ctx, trace_tea.EventAPI, "API call")
    task.End()
    
    fr fr Get trace data
    traceData := trace_tea.GetTraceBuffer()
    
    fr fr Stop tracing
    trace_tea.Stop()
    
    fr fr Create visualizer
    visualizer := trace_tea.NewVisualizer(traceData)
    
    fr fr Generate timeline
    timeline := visualizer.GenerateTimeline()
    assert_true(len(timeline.Events) > 0)
    assert_true(timeline.Duration > 0)
}

fr fr Test metrics extraction
slay test_metrics_extraction() {
    test_start("Metrics extraction")
    
    fr fr Start tracing to collect data
    trace_tea.Start(cap)
    
    fr fr Create some trace data
    ctx := vibe_context.Background()
    trace_tea.Logf(ctx, trace_tea.EventAPI, "API call %d", 1)
    trace_tea.Logf(ctx, trace_tea.EventDatabase, "Database query")
    
    fr fr Get trace data
    traceData := trace_tea.GetTraceBuffer()
    
    fr fr Stop tracing
    trace_tea.Stop()
    
    fr fr Extract metrics
    metrics := trace_tea.ExtractMetrics(traceData)
    
    fr fr Check metrics
    assert_true(metrics.MaxConcurrency() > 0)
    assert_true(metrics.AverageLatency(trace_tea.EventAPI) > 0)
    assert_true(metrics.AverageLatency(trace_tea.EventDatabase) > 0)
}

fr fr Test event categories
slay test_event_categories() {
    test_start("Event categories")
    
    fr fr Test all event category constants
    assert_eq_string(trace_tea.EventGoroutine, "goroutine")
    assert_eq_string(trace_tea.EventNet, "net")
    assert_eq_string(trace_tea.EventSyscall, "syscall")
    assert_eq_string(trace_tea.EventMemory, "memory")
    assert_eq_string(trace_tea.EventCPUSample, "cpu-sample")
    assert_eq_string(trace_tea.EventConcurrency, "concurrency")
    assert_eq_string(trace_tea.EventGC, "gc")
    assert_eq_string(trace_tea.EventBlock, "block")
    assert_eq_string(trace_tea.EventUserDefined, "user")
    assert_eq_string(trace_tea.EventAPI, "api")
    assert_eq_string(trace_tea.EventDatabase, "database")
    assert_eq_string(trace_tea.EventCache, "cache")
    assert_eq_string(trace_tea.EventFile, "file")
    assert_eq_string(trace_tea.EventCompute, "compute")
    assert_eq_string(trace_tea.EventAsyncWork, "async")
    assert_eq_string(trace_tea.EventNetwork, "network")
    assert_eq_string(trace_tea.EventRender, "render")
    assert_eq_string(trace_tea.EventLogger, "logger")
    assert_eq_string(trace_tea.EventPerformance, "performance")
}

fr fr Test buffer management
slay test_buffer_management() {
    test_start("Buffer management")
    
    fr fr Start tracing
    trace_tea.Start(cap)
    
    fr fr Add some events
    ctx := vibe_context.Background()
    trace_tea.Log(ctx, trace_tea.EventUserDefined, "Test event")
    
    fr fr Check buffer has content
    buffer := trace_tea.GetTraceBuffer()
    assert_true(len(buffer) > 0)
    
    fr fr Clear buffer
    trace_tea.ClearTraceBuffer()
    
    fr fr Check buffer is empty
    buffer = trace_tea.GetTraceBuffer()
    assert_eq_int(len(buffer), 0)
    
    fr fr Stop tracing
    trace_tea.Stop()
}

fr fr Test error handling
slay test_error_handling() {
    test_start("Error handling")
    
    fr fr Try to start tracing twice
    err1 := trace_tea.Start(cap)
    assert_eq_string(err1, "")
    
    err2 := trace_tea.Start(cap)
    assert_eq_string(err2, "Tracing already active")
    
    fr fr Stop tracing
    trace_tea.Stop()
    
    fr fr Try to stop tracing when not active
    err3 := trace_tea.Stop()
    assert_eq_string(err3, "Tracing not active")
}

fr fr Run all tests
slay main() {
    test_basic_tracing()
    test_task_management()
    test_region_functionality()
    test_event_logging()
    test_with_region()
    test_with_span()
    test_filter_functionality()
    test_real_time_analyzer()
    test_visualizer()
    test_metrics_extraction()
    test_event_categories()
    test_buffer_management()
    test_error_handling()
    
    print_test_summary()
}

main()
