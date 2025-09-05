// Comprehensive test suite for vibecheck module
// Tests pure CURSED runtime introspection without unsafe operations

yeet "testz"
yeet "vibecheck"

// Test runtime initialization
test_start("vibecheck_init")
assert_true(vibecheck.vibecheck_init())
assert_true(vibecheck.runtime_started)
print_test_summary()

// Test timing functions
test_start("timing_functions")
sus start_time thicc = vibecheck.get_start_time()
assert_true(start_time > 0)

sus uptime thicc = vibecheck.get_uptime()
assert_true(uptime >= 0)
print_test_summary()

// Test memory statistics initialization
test_start("memory_stats_init")
sus total_allocs thicc = vibecheck.get_total_allocations()
sus current_mem thicc = vibecheck.get_current_memory()
sus peak_mem thicc = vibecheck.get_peak_memory()

assert_true(total_allocs >= 0)
assert_true(current_mem >= 0)
assert_true(peak_mem >= 0)
print_test_summary()

// Test memory allocation tracking
test_start("memory_allocation_tracking")
sus old_alloc_count thicc = vibecheck.get_alloc_count()
sus old_current_mem thicc = vibecheck.get_current_memory()

// Simulate memory allocation
assert_true(vibecheck.update_memory_stats(1024, 0))

sus new_alloc_count thicc = vibecheck.get_alloc_count()
sus new_current_mem thicc = vibecheck.get_current_memory()

assert_true(new_alloc_count == old_alloc_count + 1)
assert_true(new_current_mem == old_current_mem + 1024)
print_test_summary()

// Test memory deallocation tracking
test_start("memory_deallocation_tracking")
sus old_free_count thicc = vibecheck.get_free_count()
sus old_current_mem thicc = vibecheck.get_current_memory()

// Simulate memory deallocation
assert_true(vibecheck.update_memory_stats(0, 512))

sus new_free_count thicc = vibecheck.get_free_count()
sus new_current_mem thicc = vibecheck.get_current_memory()

assert_true(new_free_count == old_free_count + 1)
assert_true(new_current_mem <= old_current_mem)
print_test_summary()

// Test peak memory tracking
test_start("peak_memory_tracking")
sus old_peak thicc = vibecheck.get_peak_memory()

// Allocate large chunk to trigger peak update
assert_true(vibecheck.update_memory_stats(2048, 0))

sus new_peak thicc = vibecheck.get_peak_memory()
assert_true(new_peak >= old_peak)
print_test_summary()

// Test memory efficiency calculation
test_start("memory_efficiency")
sus efficiency drip = vibecheck.get_memory_efficiency()
assert_true(efficiency >= 0.0)
assert_true(efficiency <= 100.0)
print_test_summary()

// Test garbage collection tracking
test_start("gc_tracking")
sus old_gc_count thicc = vibecheck.get_gc_count()
assert_true(vibecheck.trigger_gc())
sus new_gc_count thicc = vibecheck.get_gc_count()
assert_true(new_gc_count == old_gc_count + 1)
print_test_summary()

// Test goroutine counting
test_start("goroutine_management")
sus initial_count thicc = vibecheck.get_goroutine_count()
assert_true(initial_count >= 1)  // At least main goroutine

assert_true(vibecheck.increment_goroutine_count())
sus incremented_count thicc = vibecheck.get_goroutine_count()
assert_true(incremented_count == initial_count + 1)

assert_true(vibecheck.decrement_goroutine_count())
sus decremented_count thicc = vibecheck.get_goroutine_count()
assert_true(decremented_count == initial_count)
print_test_summary()

// Test function profiling
test_start("function_profiling")
sus initial_calls thicc = vibecheck.get_function_calls()
assert_true(vibecheck.profile_function_enter("test_function"))
sus after_enter thicc = vibecheck.get_function_calls()
assert_true(after_enter == initial_calls + 1)

assert_true(vibecheck.profile_function_exit("test_function"))
print_test_summary()

// Test CPU sampling
test_start("cpu_sampling")
sus initial_samples thicc = vibecheck.get_cpu_samples()
assert_true(vibecheck.add_cpu_sample())
sus after_sample thicc = vibecheck.get_cpu_samples()
assert_true(after_sample == initial_samples + 1)
print_test_summary()

// Test system information
test_start("system_info")
sus system_info tea = vibecheck.get_system_info()
assert_true(system_info.length() > 0)
assert_true(system_info.contains("CURSED Runtime"))
assert_true(system_info.contains("Uptime"))
assert_true(system_info.contains("Memory"))
print_test_summary()

// Test memory pressure detection
test_start("memory_pressure_detection")
sus has_pressure lit = vibecheck.detect_memory_pressure()
// Should be boolean
assert_true(has_pressure == based || has_pressure == cap)
print_test_summary()

// Test performance metrics
test_start("performance_metrics")
sus metrics tea = vibecheck.get_performance_metrics()
assert_true(metrics.length() > 0)
assert_true(metrics.contains("Performance Metrics"))
assert_true(metrics.contains("Memory Efficiency"))
print_test_summary()

// Test runtime health check
test_start("runtime_health_check")
sus health lit = vibecheck.runtime_health_check()
assert_true(health == based || health == cap)
print_test_summary()

// Test type-safe reflection
test_start("type_safe_reflection")
sus value normie = 42
sus type_info tea = vibecheck.get_type_info(value)
assert_eq_string(type_info, "normie")

sus value_size thicc = vibecheck.get_value_size(value)
assert_eq_int(value_size, 4)
print_test_summary()

// Test memory layout inspection (safe)
test_start("safe_memory_inspection")
sus layout tea = vibecheck.inspect_memory_layout()
assert_true(layout.length() > 0)
assert_true(layout.contains("Memory Layout"))
assert_true(layout.contains("Current Memory"))
print_test_summary()

// Test GC configuration
test_start("gc_configuration")
assert_true(vibecheck.set_gc_target_percent(80))
sus target normie = vibecheck.get_gc_target_percent()
assert_true(target >= 0)
print_test_summary()

// Test memory limit management
test_start("memory_limit_management")
assert_true(vibecheck.set_memory_limit(1048576))  // 1MB
sus limit thicc = vibecheck.get_memory_limit()
assert_eq_int(limit, 1048576)

sus within_limit lit = vibecheck.check_memory_limit()
assert_true(within_limit == based || within_limit == cap)
print_test_summary()

// Test performance monitoring
test_start("performance_monitoring")
assert_true(vibecheck.start_performance_monitoring())

// Simulate some activity
assert_true(vibecheck.profile_function_enter("monitored_func"))
assert_true(vibecheck.add_cpu_sample())

sus report tea = vibecheck.stop_performance_monitoring()
assert_true(report.length() > 0)
assert_true(report.contains("Performance Report"))
assert_true(report.contains("CPU Samples"))
print_test_summary()

// Test module main function
test_start("module_main")
assert_true(vibecheck.vibecheck_main())
print_test_summary()

// Integration test - comprehensive runtime inspection
test_start("comprehensive_runtime_inspection")
// Initialize fresh state
assert_true(vibecheck.vibecheck_init())

// Simulate runtime activity
assert_true(vibecheck.update_memory_stats(4096, 0))  // Allocate
assert_true(vibecheck.increment_goroutine_count())   // New goroutine
assert_true(vibecheck.profile_function_enter("main"))
assert_true(vibecheck.add_cpu_sample())
assert_true(vibecheck.trigger_gc())                  // GC cycle

// Verify all statistics updated
assert_true(vibecheck.get_total_allocations() >= 4096)
assert_true(vibecheck.get_goroutine_count() >= 2)
assert_true(vibecheck.get_function_calls() >= 1)
assert_true(vibecheck.get_cpu_samples() >= 1)
assert_true(vibecheck.get_gc_count() >= 1)

// Verify health check passes
assert_true(vibecheck.runtime_health_check())

vibez.spill("✅ All vibecheck tests passed!")
print_test_summary()
