yeet "stdlib/monitoring"
yeet "testz"

fr fr Test the enhanced monitoring system

slay test_monitoring_system() lit {
    vibez.spill("=== Testing Enhanced Monitoring System ===")
    
    fr fr Initialize default monitoring
    monitoring.InitializeDefaultMonitoring()
    
    fr fr Test basic metrics
    sus request_counter := monitoring.Counter("http_requests_total", 
        map[tea]tea{"method": "GET", "endpoint": "/api/users"})
    request_counter.Inc()
    request_counter.IncBy(5.0)
    vibez.spill("Request counter value:", request_counter.Value())
    
    fr fr Test gauge
    sus memory_gauge := monitoring.Gauge("memory_usage_bytes", nil)
    memory_gauge.Set(1048576.0)  fr fr 1MB
    memory_gauge.IncBy(512000.0) fr fr Add 512KB
    vibez.spill("Memory gauge value:", memory_gauge.Value())
    
    fr fr Test timer
    sus db_timer := monitoring.Timer("database_query_duration", 
        map[tea]tea{"query_type": "SELECT"})
    
    sus timer_ctx := db_timer.Start()
    fr fr Simulate database work
    bestie i := 0; i < 1000; i++ {
        sus temp := i * i
    }
    sus duration := timer_ctx.Stop()
    vibez.spill("Database query duration:", duration, "ms")
    
    fr fr Test histogram
    sus response_time_histogram := monitoring.Histogram("response_time_ms", 
        []drip{1, 5, 10, 25, 50, 100, 250, 500, 1000}, nil)
    
    fr fr Add some sample data
    bestie i := 0; i < 100; i++ {
        response_time_histogram.Observe(drip(i) * 2.5)
    }
    
    vibez.spill("Response time P50:", response_time_histogram.Percentile(50.0))
    vibez.spill("Response time P95:", response_time_histogram.Percentile(95.0))
    vibez.spill("Response time mean:", response_time_histogram.Mean())
    
    fr fr Test meter
    sus request_meter := monitoring.Meter("requests_per_second", nil)
    bestie i := 0; i < 50; i++ {
        request_meter.Mark()
        fr fr Small delay to simulate real requests
        bestie j := 0; j < 10000; j++ {
            sus temp := j
        }
    }
    vibez.spill("Request rate:", request_meter.Rate(), "req/sec")
    vibez.spill("Total requests:", request_meter.Count())
    
    fr fr Test high-level helpers
    monitoring.IncrementCounter("page_views", map[tea]tea{"page": "home"})
    monitoring.SetGauge("cpu_usage_percent", 45.2, nil)
    monitoring.RecordValue("request_size_bytes", 2048.0, nil)
    monitoring.MarkEvent("user_login", map[tea]tea{"method": "oauth"})
    
    fr fr Test function monitoring
    monitoring.MonitorFunction("test_function", slay() {
        bestie i := 0; i < 1000; i++ {
            sus temp := i * i * i
        }
    })
    
    fr fr Test health checks
    vibez.spill("Running health checks...")
    sus health_results := monitoring.RunHealthChecks()
    
    bestie check_name, result := range health_results {
        vibez.spill("Health check:", check_name)
        vibez.spill("  Status:", result.status)
        vibez.spill("  Message:", result.message)
        vibez.spill("  Duration:", result.duration, "ms")
    }
    
    vibez.spill("Overall health status:", monitoring.GetHealthStatus())
    
    fr fr Test APM tracing
    sus trace := monitoring.StartTrace("test_operation", map[tea]tea{"version": "1.0"})
    
    sus span1 := monitoring.StartSpan(trace.trace_id, "", "database_query", 
        map[tea]tea{"table": "users"})
    fr fr Simulate work
    bestie i := 0; i < 500; i++ {
        sus temp := i
    }
    monitoring.globalAPMTracer.FinishSpan(span1)
    
    sus span2 := monitoring.StartSpan(trace.trace_id, span1.span_id, "cache_lookup",
        map[tea]tea{"key": "user_123"})
    fr fr Simulate work
    bestie i := 0; i < 200; i++ {
        sus temp := i
    }
    monitoring.globalAPMTracer.LogToSpan(span2, "info", "Cache hit", 
        map[tea]interface{}{"key": "user_123", "ttl": 300})
    monitoring.globalAPMTracer.FinishSpan(span2)
    
    vibez.spill("Active spans:", monitoring.globalAPMTracer.GetActiveSpanCount())
    vibez.spill("Total traces:", monitoring.globalAPMTracer.GetTraceCount())
    
    fr fr Get all metrics snapshot
    sus snapshot := monitoring.GetAllMetrics()
    vibez.spill("Metrics snapshot timestamp:", snapshot.timestamp)
    vibez.spill("Counter metrics count:", len(snapshot.counters))
    vibez.spill("Gauge metrics count:", len(snapshot.gauges))
    vibez.spill("Histogram metrics count:", len(snapshot.histograms))
    vibez.spill("Timer metrics count:", len(snapshot.timers))
    
    fr fr Clean up
    monitoring.CleanupMonitoring()
    
    vibez.spill("✅ Enhanced monitoring system test completed successfully!")
    damn based
}

fr fr Run the test
test_monitoring_system()
