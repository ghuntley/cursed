yeet "testz"
yeet "monitoring"
yeet "timez"

test_start("Monitoring Module Tests")

fr fr ===== COUNTER METRIC TESTS =====

slay test_counter_metrics() {
    vibez.spill("Testing Counter metrics...")
    
    fr fr Test counter creation
    sus tags map[tea]tea = make(map[tea]tea)
    tags["service"] = "test"
    tags["environment"] = "development"
    
    sus counter *Counter = NewCounter("test_counter", tags)
    assert_not_null_pointer(counter, "Counter created")
    assert_equal_string(counter.name, "test_counter", "Counter name set")
    assert_equal_double(counter.Value(), 0.0, "Initial counter value")
    
    fr fr Test increment operations
    counter.Inc()
    assert_equal_double(counter.Value(), 1.0, "Counter increment")
    
    counter.IncBy(5.0)
    assert_equal_double(counter.Value(), 6.0, "Counter increment by value")
    
    fr fr Test reset operation
    counter.Reset()
    assert_equal_double(counter.Value(), 0.0, "Counter reset")
    
    vibez.spill("✅ Counter metric tests completed")
}

fr fr ===== GAUGE METRIC TESTS =====

slay test_gauge_metrics() {
    vibez.spill("Testing Gauge metrics...")
    
    sus tags map[tea]tea = make(map[tea]tea)
    tags["component"] = "memory"
    
    sus gauge *Gauge = NewGauge("memory_usage", tags)
    assert_not_null_pointer(gauge, "Gauge created")
    assert_equal_double(gauge.Value(), 0.0, "Initial gauge value")
    
    fr fr Test set operation
    gauge.Set(100.5)
    assert_equal_double(gauge.Value(), 100.5, "Gauge set value")
    
    fr fr Test increment/decrement operations
    gauge.Inc()
    assert_equal_double(gauge.Value(), 101.5, "Gauge increment")
    
    gauge.IncBy(10.0)
    assert_equal_double(gauge.Value(), 111.5, "Gauge increment by value")
    
    gauge.Dec()
    assert_equal_double(gauge.Value(), 110.5, "Gauge decrement")
    
    gauge.DecBy(5.5)
    assert_equal_double(gauge.Value(), 105.0, "Gauge decrement by value")
    
    vibez.spill("✅ Gauge metric tests completed")
}

fr fr ===== HISTOGRAM METRIC TESTS =====

slay test_histogram_metrics() {
    vibez.spill("Testing Histogram metrics...")
    
    sus buckets []drip = [1.0, 5.0, 10.0, 25.0, 50.0, 100.0]
    sus tags map[tea]tea = make(map[tea]tea)
    tags["operation"] = "request"
    
    sus histogram *Histogram = NewHistogram("request_duration", buckets, tags)
    assert_not_null_pointer(histogram, "Histogram created")
    assert_equal_int(histogram.Count(), 0, "Initial histogram count")
    assert_equal_double(histogram.Sum(), 0.0, "Initial histogram sum")
    
    fr fr Test observations
    histogram.Observe(3.5)
    histogram.Observe(12.0)
    histogram.Observe(75.0)
    
    assert_equal_int(histogram.Count(), 3, "Histogram count after observations")
    assert_equal_double(histogram.Sum(), 90.5, "Histogram sum after observations")
    assert_equal_double(histogram.Mean(), 30.17, "Histogram mean calculation")
    
    fr fr Test percentiles
    sus p50 drip = histogram.Percentile(50)
    sus p90 drip = histogram.Percentile(90)
    sus p99 drip = histogram.Percentile(99)
    
    assert_greater_than_double(p50, 0.0, "P50 percentile")
    assert_greater_than_double(p90, 0.0, "P90 percentile")
    assert_greater_than_double(p99, 0.0, "P99 percentile")
    
    vibez.spill("✅ Histogram metric tests completed")
}

fr fr ===== TIMER METRIC TESTS =====

slay test_timer_metrics() {
    vibez.spill("Testing Timer metrics...")
    
    sus tags map[tea]tea = make(map[tea]tea)
    tags["function"] = "test_function"
    
    sus timer *Timer = NewTimer("function_duration", tags)
    assert_not_null_pointer(timer, "Timer created")
    
    fr fr Test timer context
    sus ctx *TimerContext = timer.Start()
    assert_not_null_pointer(ctx, "Timer context created")
    assert_greater_than_int(ctx.start_time, 0, "Timer start time set")
    
    fr fr Simulate some work
    timez.Sleep(10)  fr fr Sleep 10ms
    
    sus duration drip = ctx.Stop()
    assert_greater_than_double(duration, 0.0, "Timer duration recorded")
    
    fr fr Test timer convenience functions
    timer.Time(slay() {
        timez.Sleep(5)  fr fr Simulate work
    })
    
    assert_greater_than_double(timer.Mean(), 0.0, "Timer mean duration")
    assert_greater_than_double(timer.P50(), 0.0, "Timer P50")
    assert_greater_than_double(timer.P90(), 0.0, "Timer P90")
    assert_greater_than_double(timer.P95(), 0.0, "Timer P95")
    assert_greater_than_double(timer.P99(), 0.0, "Timer P99")
    
    vibez.spill("✅ Timer metric tests completed")
}

fr fr ===== METER METRIC TESTS =====

slay test_meter_metrics() {
    vibez.spill("Testing Meter metrics...")
    
    sus tags map[tea]tea = make(map[tea]tea)
    tags["endpoint"] = "/api/users"
    
    sus meter *Meter = NewMeter("requests_per_second", tags)
    assert_not_null_pointer(meter, "Meter created")
    assert_equal_int(meter.Count(), 0, "Initial meter count")
    assert_equal_double(meter.Rate(), 0.0, "Initial meter rate")
    
    fr fr Test mark operations
    meter.Mark()
    assert_equal_int(meter.Count(), 1, "Meter count after mark")
    
    meter.MarkN(5)
    assert_equal_int(meter.Count(), 6, "Meter count after markN")
    
    fr fr Test rate calculation
    timez.Sleep(100)  fr fr Allow time for rate calculation
    sus rate drip = meter.Rate()
    assert_greater_than_double(rate, 0.0, "Meter rate calculation")
    
    vibez.spill("✅ Meter metric tests completed")
}

fr fr ===== METRIC REGISTRY TESTS =====

slay test_metric_registry() {
    vibez.spill("Testing Metric Registry...")
    
    sus registry *MetricRegistry = NewMetricRegistry()
    assert_not_null_pointer(registry, "Registry created")
    
    fr fr Test counter registration
    sus tags map[tea]tea = make(map[tea]tea)
    tags["test"] = "true"
    
    sus counter *Counter = registry.NewCounter("registry_counter", tags)
    assert_not_null_pointer(counter, "Registry counter created")
    
    sus retrieved_counter *Counter = registry.GetCounter("registry_counter", tags)
    assert_not_null_pointer(retrieved_counter, "Counter retrieved from registry")
    
    fr fr Test gauge registration
    sus gauge *Gauge = registry.NewGauge("registry_gauge", tags)
    assert_not_null_pointer(gauge, "Registry gauge created")
    
    sus retrieved_gauge *Gauge = registry.GetGauge("registry_gauge", tags)
    assert_not_null_pointer(retrieved_gauge, "Gauge retrieved from registry")
    
    fr fr Test histogram registration
    sus buckets []drip = [1.0, 10.0, 100.0]
    sus histogram *Histogram = registry.NewHistogram("registry_histogram", buckets, tags)
    assert_not_null_pointer(histogram, "Registry histogram created")
    
    fr fr Test timer registration
    sus timer *Timer = registry.NewTimer("registry_timer", tags)
    assert_not_null_pointer(timer, "Registry timer created")
    
    sus retrieved_timer *Timer = registry.GetTimer("registry_timer", tags)
    assert_not_null_pointer(retrieved_timer, "Timer retrieved from registry")
    
    fr fr Test meter registration
    sus meter *Meter = registry.NewMeter("registry_meter", tags)
    assert_not_null_pointer(meter, "Registry meter created")
    
    fr fr Test metric snapshot
    sus snapshot MetricSnapshot = registry.GetAllMetrics()
    assert_greater_than_int(len(snapshot.counters), 0, "Snapshot contains counters")
    assert_greater_than_int(len(snapshot.gauges), 0, "Snapshot contains gauges")
    assert_greater_than_int(len(snapshot.histograms), 0, "Snapshot contains histograms")
    assert_greater_than_int(len(snapshot.timers), 0, "Snapshot contains timers")
    assert_greater_than_int(len(snapshot.meters), 0, "Snapshot contains meters")
    
    vibez.spill("✅ Metric Registry tests completed")
}

fr fr ===== HEALTH MONITORING TESTS =====

slay test_health_monitoring() {
    vibez.spill("Testing Health Monitoring...")
    
    sus registry *MetricRegistry = NewMetricRegistry()
    sus health_monitor *HealthMonitor = NewHealthMonitor(registry)
    assert_not_null_pointer(health_monitor, "Health monitor created")
    
    fr fr Test health check creation
    sus database_check *HealthCheck = DatabaseHealthCheck("test_db", 1000)
    assert_not_null_pointer(database_check, "Database health check created")
    assert_equal_string(database_check.name, "database_test_db", "Health check name")
    
    sus http_check *HealthCheck = HTTPServiceHealthCheck("api", "http://localhost:8080", 2000)
    assert_not_null_pointer(http_check, "HTTP health check created")
    
    sus memory_check *HealthCheck = MemoryUsageHealthCheck(512, 500)  fr fr 512MB limit
    assert_not_null_pointer(memory_check, "Memory health check created")
    
    fr fr Test adding checks to monitor
    health_monitor.AddCheck("database", database_check)
    health_monitor.AddCheck("api", http_check)
    health_monitor.AddCheck("memory", memory_check)
    
    fr fr Test running health checks
    sus results map[tea]HealthCheckResult = health_monitor.RunAllChecks()
    assert_greater_than_int(len(results), 0, "Health check results returned")
    
    fr fr Test overall health status
    sus overall_status HealthStatus = health_monitor.GetOverallStatus()
    assert_valid_health_status(overall_status, "Overall health status")
    
    sus health_string tea = health_monitor.GetOverallHealth()
    assert_not_empty_string(health_string, "Health status string")
    
    vibez.spill("✅ Health monitoring tests completed")
}

fr fr ===== APM TRACING TESTS =====

slay test_apm_tracing() {
    vibez.spill("Testing APM Tracing...")
    
    sus registry *MetricRegistry = NewMetricRegistry()
    sus tracer *APMTracer = NewAPMTracer("test_service", registry)
    assert_not_null_pointer(tracer, "APM tracer created")
    assert_equal_string(tracer.service_name, "test_service", "Service name set")
    
    fr fr Test trace creation
    sus tags map[tea]tea = make(map[tea]tea)
    tags["operation"] = "test"
    
    sus trace *Trace = tracer.StartTrace("test_operation", tags)
    assert_not_null_pointer(trace, "Trace created")
    assert_not_empty_string(trace.trace_id, "Trace ID assigned")
    assert_greater_than_int(trace.start_time, 0, "Trace start time set")
    
    fr fr Test span creation
    sus span *Span = tracer.StartSpan(trace.trace_id, "", "child_operation", tags)
    assert_not_null_pointer(span, "Span created")
    assert_not_empty_string(span.span_id, "Span ID assigned")
    assert_equal_string(span.trace_id, trace.trace_id, "Span trace ID matches")
    assert_equal_bool(span.finished, cap, "Span initially not finished")
    
    fr fr Test span logging
    sus fields map[tea]interface{} = make(map[tea]interface{})
    fields["user_id"] = "123"
    fields["action"] = "test"
    
    tracer.LogToSpan(span, "INFO", "Test log message", fields)
    assert_greater_than_int(len(span.logs), 0, "Span log added")
    
    fr fr Test span finishing
    tracer.FinishSpan(span)
    assert_equal_bool(span.finished, based, "Span finished")
    assert_greater_than_int(span.duration, 0, "Span duration recorded")
    
    fr fr Test tracer statistics
    sus active_spans normie = tracer.GetActiveSpanCount()
    sus trace_count normie = tracer.GetTraceCount()
    assert_greater_than_int(trace_count, 0, "Trace count tracked")
    
    vibez.spill("✅ APM tracing tests completed")
}

fr fr ===== RESOURCE MONITORING TESTS =====

slay test_resource_monitoring() {
    vibez.spill("Testing Resource Monitoring...")
    
    sus registry *MetricRegistry = NewMetricRegistry()
    sus monitor *ResourceMonitor = NewResourceMonitor(registry)
    assert_not_null_pointer(monitor, "Resource monitor created")
    assert_equal_bool(monitor.monitoring_enabled, cap, "Initially not monitoring")
    
    fr fr Test starting monitoring
    monitor.StartMonitoring()
    assert_equal_bool(monitor.isMonitoringEnabled(), based, "Monitoring enabled")
    
    fr fr Allow some time for metric collection
    timez.Sleep(100)  fr fr 100ms
    
    fr fr Test stopping monitoring
    monitor.StopMonitoring()
    assert_equal_bool(monitor.isMonitoringEnabled(), cap, "Monitoring disabled")
    
    fr fr Test metric collection
    monitor.collectMetrics()
    
    fr fr Verify metrics were updated (check registry)
    sus cpu_gauge *Gauge = monitor.cpu_gauge
    sus memory_gauge *Gauge = monitor.memory_gauge
    sus goroutine_gauge *Gauge = monitor.goroutine_gauge
    
    assert_not_null_pointer(cpu_gauge, "CPU gauge available")
    assert_not_null_pointer(memory_gauge, "Memory gauge available")
    assert_not_null_pointer(goroutine_gauge, "Goroutine gauge available")
    
    vibez.spill("✅ Resource monitoring tests completed")
}

fr fr ===== GLOBAL MONITORING FUNCTIONS TESTS =====

slay test_global_monitoring_functions() {
    vibez.spill("Testing Global Monitoring Functions...")
    
    fr fr Test global counter
    sus global_counter *Counter = Counter("global_test_counter", nil)
    assert_not_null_pointer(global_counter, "Global counter created")
    global_counter.Inc()
    
    fr fr Test global gauge
    sus global_gauge *Gauge = Gauge("global_test_gauge", nil)
    assert_not_null_pointer(global_gauge, "Global gauge created")
    global_gauge.Set(42.0)
    
    fr fr Test global timer
    sus global_timer *Timer = Timer("global_test_timer", nil)
    assert_not_null_pointer(global_timer, "Global timer created")
    
    fr fr Test global histogram
    sus global_histogram *Histogram = Histogram("global_test_histogram", nil, nil)
    assert_not_null_pointer(global_histogram, "Global histogram created")
    
    fr fr Test global meter
    sus global_meter *Meter = Meter("global_test_meter", nil)
    assert_not_null_pointer(global_meter, "Global meter created")
    
    fr fr Test convenience functions
    IncrementCounter("test_increment", nil)
    SetGauge("test_set", 100.5, nil)
    RecordValue("test_record", 25.0, nil)
    MarkEvent("test_event", nil)
    
    fr fr Test monitor function
    MonitorFunction("test_function", slay() {
        timez.Sleep(5)  fr fr Simulate work
    })
    
    fr fr Test function with result
    sus result interface{} = MonitorFunctionWithResult("test_with_result", slay() interface{} {
        damn "test_result"
    })
    assert_not_null_interface(result, "Function result captured")
    
    fr fr Test health checks
    AddHealthCheck("global_db", DatabaseHealthCheck("global", 1000))
    sus health_results map[tea]HealthCheckResult = RunHealthChecks()
    assert_greater_than_int(len(health_results), 0, "Global health checks executed")
    
    sus health_status tea = GetHealthStatus()
    assert_not_empty_string(health_status, "Global health status")
    
    fr fr Test resource monitoring
    StartResourceMonitoring()
    timez.Sleep(50)  fr fr Allow monitoring
    StopResourceMonitoring()
    
    fr fr Test metrics snapshot
    sus global_metrics MetricSnapshot = GetAllMetrics()
    assert_greater_than_int(len(global_metrics.counters), 0, "Global metrics captured")
    
    vibez.spill("✅ Global monitoring function tests completed")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_monitoring_performance() {
    vibez.spill("Testing monitoring performance...")
    
    sus registry *MetricRegistry = NewMetricRegistry()
    sus start_time normie = timez.Now()
    
    fr fr Test rapid metric operations
    sus counter *Counter = registry.NewCounter("perf_counter", nil)
    sus iterations normie = 1000
    sus i normie = 0
    
    bestie (i < iterations) {
        counter.Inc()
        i = i + 1
    }
    
    sus end_time normie = timez.Now()
    sus duration normie = end_time - start_time
    assert_less_than_int(duration, 1000, "1000 counter increments completed quickly")
    
    fr fr Test histogram performance
    sus histogram *Histogram = registry.NewHistogram("perf_histogram", [1.0, 10.0, 100.0], nil)
    start_time = timez.Now()
    
    i = 0
    bestie (i < 500) {
        histogram.Observe(drip(i))
        i = i + 1
    }
    
    end_time = timez.Now()
    duration = end_time - start_time
    assert_less_than_int(duration, 1000, "500 histogram observations completed quickly")
    
    fr fr Test registry snapshot performance
    start_time = timez.Now()
    sus snapshot MetricSnapshot = registry.GetAllMetrics()
    end_time = timez.Now()
    duration = end_time - start_time
    assert_less_than_int(duration, 100, "Registry snapshot generated quickly")
    
    vibez.spill("✅ Monitoring performance tests completed")
}

fr fr ===== EDGE CASE TESTS =====

slay test_monitoring_edge_cases() {
    vibez.spill("Testing monitoring edge cases...")
    
    fr fr Test null/empty parameters
    sus empty_counter *Counter = NewCounter("", nil)
    assert_not_null_pointer(empty_counter, "Counter with empty name created")
    
    fr fr Test histogram with empty buckets
    sus empty_buckets []drip = []
    sus empty_histogram *Histogram = NewHistogram("empty_buckets", empty_buckets, nil)
    assert_not_null_pointer(empty_histogram, "Histogram with empty buckets created")
    
    fr fr Test timer with immediate stop
    sus timer *Timer = NewTimer("immediate", nil)
    sus ctx *TimerContext = timer.Start()
    sus immediate_duration drip = ctx.Stop()
    assert_greater_than_double(immediate_duration, 0.0, "Immediate timer duration")
    
    fr fr Test health check with zero timeout
    sus zero_timeout_check *HealthCheck = DatabaseHealthCheck("zero", 0)
    assert_not_null_pointer(zero_timeout_check, "Health check with zero timeout")
    
    fr fr Test APM with invalid trace ID
    sus tracer *APMTracer = NewAPMTracer("test", NewMetricRegistry())
    sus invalid_span *Span = tracer.StartSpan("", "", "invalid", nil)
    assert_not_null_pointer(invalid_span, "Span with empty trace ID")
    
    vibez.spill("✅ Monitoring edge case tests completed")
}

fr fr ===== HELPER FUNCTIONS =====

slay assert_not_null_pointer(ptr interface{}, message tea) {
    ready (ptr == nil) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected non-null pointer")
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_not_null_interface(value interface{}, message tea) {
    ready (value == nil) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected non-null interface")
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_equal_double(actual drip, expected drip, message tea) {
    sus tolerance drip = 0.01
    ready (mathz.Abs(actual - expected) > tolerance) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected: " + double_to_string(expected))
        vibez.spill("   Actual: " + double_to_string(actual))
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_greater_than_double(actual drip, expected drip, message tea) {
    ready (actual <= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected greater than: " + double_to_string(expected))
        vibez.spill("   Actual: " + double_to_string(actual))
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_greater_than_int(actual normie, expected normie, message tea) {
    ready (actual <= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected greater than: " + int_to_string(expected))
        vibez.spill("   Actual: " + int_to_string(actual))
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_less_than_int(actual normie, expected normie, message tea) {
    ready (actual >= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected less than: " + int_to_string(expected))
        vibez.spill("   Actual: " + int_to_string(actual))
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_valid_health_status(status HealthStatus, message tea) {
    ready (status != HealthyStatus && status != DegradedStatus && status != UnhealthyStatus) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Invalid health status")
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_not_empty_string(value tea, message tea) {
    ready (value == "") {
        vibez.spill("❌ ASSERTION FAILED: " + message)
        vibez.spill("   Expected non-empty string")
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay double_to_string(value drip) tea {
    ready (value == 0.0) { damn "0.0" }
    ready (value == 1.0) { damn "1.0" }
    ready (value < 1.0) { damn "fractional" }
    ready (value < 10.0) { damn "single_digit" }
    damn "multi_digit"
}

slay int_to_string(value normie) tea {
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value < 10) { damn "single_digit" }
    ready (value < 100) { damn "double_digit" }
    damn "large_number"
}

fr fr ===== MAIN TEST EXECUTION =====

fr fr Execute all test suites
test_counter_metrics()
test_gauge_metrics()
test_histogram_metrics()
test_timer_metrics()
test_meter_metrics()
test_metric_registry()
test_health_monitoring()
test_apm_tracing()
test_resource_monitoring()
test_global_monitoring_functions()
test_monitoring_performance()
test_monitoring_edge_cases()

print_test_summary()
