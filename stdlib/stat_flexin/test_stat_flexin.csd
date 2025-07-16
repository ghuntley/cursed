yeet "testz"
yeet "stat_flexin"

test_start("StatFlexin comprehensive test suite")

fr fr Test FlexCounter functionality
slay test_flex_counter() {
    sus counter := stat_flexin.NewFlexCounter("test_counter")
    
    fr fr Test initial state
    assert_eq_int(counter.Get(), 0)
    assert_eq_string(counter.String(), "0")
    
    fr fr Test increment
    counter.Inc()
    assert_eq_int(counter.Get(), 1)
    
    fr fr Test add value
    counter.Add(5)
    assert_eq_int(counter.Get(), 6)
    
    fr fr Test reset
    counter.Reset()
    assert_eq_int(counter.Get(), 0)
    
    vibez.spill("✅ FlexCounter tests passed")
}

fr fr Test FlexFloat functionality
slay test_flex_float() {
    sus float_val := stat_flexin.NewFlexFloat("test_float")
    
    fr fr Test initial state
    assert_eq_string(float_val.String(), "0.0")
    
    fr fr Test set value
    float_val.Set(3.14)
    assert_eq_float(float_val.Get(), 3.14)
    
    fr fr Test add value
    float_val.Add(1.86)
    assert_eq_float(float_val.Get(), 5.0)
    
    vibez.spill("✅ FlexFloat tests passed")
}

fr fr Test FlexString functionality
slay test_flex_string() {
    sus str_val := stat_flexin.NewFlexString("test_string")
    
    fr fr Test initial state
    assert_eq_string(str_val.Get(), "")
    assert_eq_string(str_val.String(), "")
    
    fr fr Test set value
    str_val.Set("hello")
    assert_eq_string(str_val.Get(), "hello")
    assert_eq_string(str_val.String(), "hello")
    
    vibez.spill("✅ FlexString tests passed")
}

fr fr Test Registry functionality
slay test_registry() {
    sus registry := stat_flexin.NewRegistry()
    
    fr fr Test empty registry
    assert_eq_string(registry.String(), "Registry")
    
    fr fr Test adding variables
    sus counter := stat_flexin.NewFlexCounter("test_counter")
    registry.Set("counter", counter)
    
    sus retrieved := registry.Get("counter")
    assert_true(retrieved != cringe)
    
    fr fr Test deletion
    registry.Delete("counter")
    sus deleted := registry.Get("counter")
    assert_true(deleted == cringe)
    
    fr fr Test clear
    registry.Set("counter", counter)
    registry.Clear()
    sus cleared := registry.Get("counter")
    assert_true(cleared == cringe)
    
    vibez.spill("✅ Registry tests passed")
}

fr fr Test global registry functions
slay test_global_registry() {
    fr fr Clear global registry first
    stat_flexin.Clear()
    
    fr fr Test global registration
    sus counter := stat_flexin.NewFlexCounter("global_counter")
    stat_flexin.Register("global_counter", counter)
    
    sus retrieved := stat_flexin.Get("global_counter")
    assert_true(retrieved != cringe)
    
    fr fr Test global deletion
    stat_flexin.Delete("global_counter")
    sus deleted := stat_flexin.Get("global_counter")
    assert_true(deleted == cringe)
    
    vibez.spill("✅ Global registry tests passed")
}

fr fr Test performance monitoring
slay test_performance_monitor() {
    sus monitor := stat_flexin.NewPerformanceMonitor()
    
    fr fr Test initial state
    assert_true(monitor.IsEnabled())
    
    fr fr Test counter operations
    monitor.IncrementCounter("test_counter")
    monitor.AddCounterValue("test_counter", 5)
    assert_eq_int(monitor.GetCounterValue("test_counter"), 6)
    
    fr fr Test gauge operations
    monitor.SetGauge("test_gauge", 10.5)
    assert_eq_float(monitor.GetGaugeValue("test_gauge"), 10.5)
    
    fr fr Test timer operations
    monitor.RecordTimer("test_timer", 100)
    monitor.RecordTimer("test_timer", 200)
    sus total, count, min, max := monitor.GetTimerStats("test_timer")
    assert_eq_int(total, 300)
    assert_eq_int(count, 2)
    assert_eq_int(min, 100)
    assert_eq_int(max, 200)
    
    fr fr Test histogram operations
    monitor.RecordHistogram("test_histogram", 15.0)
    monitor.RecordHistogram("test_histogram", 25.0)
    
    fr fr Test get all metrics
    sus metrics := monitor.GetAllMetrics()
    assert_true(len(metrics) > 0)
    
    fr fr Test reset
    monitor.Reset()
    assert_eq_int(monitor.GetCounterValue("test_counter"), 0)
    
    vibez.spill("✅ Performance monitor tests passed")
}

fr fr Test timer functionality
slay test_flex_timer() {
    sus timer := stat_flexin.NewFlexTimer("test_timer")
    
    fr fr Test initial state
    assert_eq_int(timer.count, 0)
    assert_eq_float(timer.GetAverage(), 0.0)
    
    fr fr Test recording times
    timer.Record(100)
    timer.Record(200)
    timer.Record(300)
    
    assert_eq_int(timer.count, 3)
    assert_eq_int(timer.total_time, 600)
    assert_eq_int(timer.min_time, 100)
    assert_eq_int(timer.max_time, 300)
    assert_eq_float(timer.GetAverage(), 200.0)
    
    fr fr Test reset
    timer.Reset()
    assert_eq_int(timer.count, 0)
    assert_eq_int(timer.total_time, 0)
    
    vibez.spill("✅ FlexTimer tests passed")
}

fr fr Test gauge functionality
slay test_flex_gauge() {
    sus gauge := stat_flexin.NewFlexGauge("test_gauge", 0.0)
    
    fr fr Test initial state
    assert_eq_float(gauge.Get(), 0.0)
    assert_eq_float(gauge.GetMin(), 0.0)
    assert_eq_float(gauge.GetMax(), 0.0)
    
    fr fr Test setting values
    gauge.Set(10.5)
    assert_eq_float(gauge.Get(), 10.5)
    assert_eq_float(gauge.GetMax(), 10.5)
    
    gauge.Set(5.0)
    assert_eq_float(gauge.Get(), 5.0)
    assert_eq_float(gauge.GetMin(), 0.0)
    assert_eq_float(gauge.GetMax(), 10.5)
    
    fr fr Test increment/decrement
    gauge.Inc()
    assert_eq_float(gauge.Get(), 6.0)
    
    gauge.Dec()
    assert_eq_float(gauge.Get(), 5.0)
    
    fr fr Test add/subtract
    gauge.Add(2.5)
    assert_eq_float(gauge.Get(), 7.5)
    
    gauge.Sub(1.5)
    assert_eq_float(gauge.Get(), 6.0)
    
    vibez.spill("✅ FlexGauge tests passed")
}

fr fr Test histogram functionality
slay test_flex_histogram() {
    sus histogram := stat_flexin.NewFlexHistogram("test_histogram")
    
    fr fr Test initial state
    assert_eq_int(histogram.total_count, 0)
    assert_eq_float(histogram.sum_value, 0.0)
    assert_eq_float(histogram.GetAverage(), 0.0)
    
    fr fr Test recording values
    histogram.Record(15.0)
    histogram.Record(25.0)
    histogram.Record(35.0)
    
    assert_eq_int(histogram.total_count, 3)
    assert_eq_float(histogram.sum_value, 75.0)
    assert_eq_float(histogram.GetAverage(), 25.0)
    
    fr fr Test buckets
    sus buckets := histogram.GetBuckets()
    assert_true(len(buckets) > 0)
    
    fr fr Test reset
    histogram.Reset()
    assert_eq_int(histogram.total_count, 0)
    assert_eq_float(histogram.sum_value, 0.0)
    
    vibez.spill("✅ FlexHistogram tests passed")
}

fr fr Test health check system
slay test_health_checks() {
    fr fr Register a health check
    stat_flexin.RegisterHealthCheck("test_check", slay() (lit, tea) {
        damn based, ""
    }, 1000)
    
    fr fr Run health check
    sus result, error := stat_flexin.RunHealthCheck("test_check")
    assert_true(result)
    assert_eq_string(error, "")
    
    fr fr Test health check status
    sus status, status_error, last_check := stat_flexin.GetHealthCheckStatus("test_check")
    assert_true(status)
    assert_eq_string(status_error, "")
    assert_true(last_check > 0)
    
    fr fr Test running all health checks
    sus all_results := stat_flexin.RunAllHealthChecks()
    assert_true(len(all_results) > 0)
    assert_true(all_results["test_check"])
    
    vibez.spill("✅ Health check tests passed")
}

fr fr Test global monitoring functions
slay test_global_monitoring() {
    fr fr Reset metrics first
    stat_flexin.ResetAllMetrics()
    
    fr fr Test global counter operations
    stat_flexin.IncrementCounter("global_counter")
    stat_flexin.AddCounterValue("global_counter", 5)
    
    fr fr Test global gauge operations
    stat_flexin.SetGauge("global_gauge", 42.0)
    
    fr fr Test global timer operations
    stat_flexin.RecordTimer("global_timer", 150)
    
    fr fr Test global histogram operations
    stat_flexin.RecordHistogram("global_histogram", 30.0)
    
    fr fr Test get all metrics
    sus metrics := stat_flexin.GetAllMetrics()
    assert_true(len(metrics) > 0)
    
    fr fr Test monitoring enable/disable
    stat_flexin.DisableMonitoring()
    assert_true(!stat_flexin.IsMonitoringEnabled())
    
    stat_flexin.EnableMonitoring()
    assert_true(stat_flexin.IsMonitoringEnabled())
    
    vibez.spill("✅ Global monitoring tests passed")
}

fr fr Test metric export formats
slay test_metric_export() {
    fr fr Reset and add some metrics
    stat_flexin.ResetAllMetrics()
    stat_flexin.IncrementCounter("export_counter")
    stat_flexin.SetGauge("export_gauge", 100.0)
    
    fr fr Test JSON export
    sus json_export := stat_flexin.ExportMetricsAsJSON()
    assert_true(len(json_export) > 0)
    assert_true(json_export != "")
    
    fr fr Test Prometheus export
    sus prometheus_export := stat_flexin.ExportMetricsAsPrometheus()
    assert_true(len(prometheus_export) > 0)
    assert_true(prometheus_export != "")
    
    vibez.spill("✅ Metric export tests passed")
}

fr fr Test benchmark utilities
slay test_benchmark_utilities() {
    fr fr Test benchmark function
    stat_flexin.BenchmarkFunction("test_benchmark", 10, slay() {
        fr fr Simple operation to benchmark
        sus x := 1 + 1
    })
    
    fr fr Test profile function
    stat_flexin.ProfileFunction("test_profile", slay() {
        fr fr Simple operation to profile
        sus y := 2 * 2
    })
    
    fr fr Test memory profiling
    stat_flexin.RecordMemoryUsage("memory_test")
    
    sus start_memory := stat_flexin.StartMemoryProfiling("memory_profile")
    fr fr Do some work
    sus z := 3 * 3
    stat_flexin.EndMemoryProfiling("memory_profile", start_memory)
    
    vibez.spill("✅ Benchmark utilities tests passed")
}

fr fr Test alert system
slay test_alert_system() {
    fr fr Register an alert
    stat_flexin.RegisterAlert("high_counter", "counter_alert_test", slay(value interface{}) lit {
        damn based  fr fr Always trigger for testing
    }, 5, slay(name tea, value interface{}) {
        vibez.spill("Alert triggered: " + name)
    })
    
    fr fr Set up metric to trigger alert
    stat_flexin.IncrementCounter("alert_test")
    
    fr fr Test alert enable/disable
    stat_flexin.EnableAlert("high_counter")
    stat_flexin.DisableAlert("high_counter")
    stat_flexin.EnableAlert("high_counter")
    
    fr fr Check alerts
    stat_flexin.CheckAlerts()
    
    vibez.spill("✅ Alert system tests passed")
}

fr fr Run all tests
test_flex_counter()
test_flex_float()
test_flex_string()
test_registry()
test_global_registry()
test_performance_monitor()
test_flex_timer()
test_flex_gauge()
test_flex_histogram()
test_health_checks()
test_global_monitoring()
test_metric_export()
test_benchmark_utilities()
test_alert_system()

print_test_summary()
vibez.spill("🎉 All StatFlexin tests completed successfully!")
