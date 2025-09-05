yeet "testz"
yeet "memory_profiler"

fr fr Comprehensive test suite for memory_profiler module

test_start("init_memory_profiler")
sus init_result lit = init_memory_profiler()
assert_true(init_result)
assert_eq_int(global_allocation_count, 0)
assert_eq_int(global_gc_count, 0)

test_start("start_profiling")
sus session_id tea = start_profiling()
assert_eq_string(session_id, "profiler_session_001")
assert_true(global_profiler_session.is_active)

test_start("track_allocation")
sus track_result lit = track_allocation("ptr_001", 1024, "string")
assert_true(track_result)
assert_eq_int(global_allocation_count, 1)
assert_eq_int(global_profiler_session.total_allocations, 1)
assert_eq_int(global_profiler_session.current_memory_usage, 1024)

test_start("track_multiple_allocations")
track_allocation("ptr_002", 512, "int")
track_allocation("ptr_003", 256, "float")
assert_eq_int(global_allocation_count, 3)
assert_eq_int(global_profiler_session.total_allocations, 3)
assert_eq_int(global_profiler_session.current_memory_usage, 1792)

test_start("track_deallocation")
sus dealloc_result lit = track_deallocation("ptr_002")
assert_true(dealloc_result)
assert_eq_int(global_profiler_session.total_deallocations, 1)
assert_eq_int(global_profiler_session.current_memory_usage, 1280)

test_start("capture_stack_trace")
sus trace tea = capture_stack_trace()
assert_true(len(trace) > 0)

test_start("detect_leaks")
sus leak_report tea = detect_leaks()
assert_true(len(leak_report) > 0)
fr fr Should contain information about unfreed allocations

test_start("analyze_allocation_patterns")
sus pattern_report tea = analyze_allocation_patterns()
assert_true(len(pattern_report) > 0)

test_start("analyze_fragmentation")
sus frag_report tea = analyze_fragmentation()
assert_true(len(frag_report) > 0)

test_start("monitor_gc_performance")
sus gc_monitor_result lit = monitor_gc_performance(50, 1024)
assert_true(gc_monitor_result)
assert_eq_int(global_gc_count, 1)

test_start("generate_gc_report")
sus gc_report tea = generate_gc_report()
assert_true(len(gc_report) > 0)

test_start("visualize_memory_usage")
sus viz tea = visualize_memory_usage()
assert_true(len(viz) > 0)

test_start("get_memory_stats")
sus stats tea = get_memory_stats()
assert_true(len(stats) > 0)

test_start("generate_profiling_report")
sus report tea = generate_profiling_report()
assert_true(len(report) > 0)

test_start("stop_profiling")
sus final_report tea = stop_profiling()
assert_true(len(final_report) > 0)
assert_false(global_profiler_session.is_active)

test_start("reset_profiler")
sus reset_result lit = reset_profiler()
assert_true(reset_result)
assert_eq_int(global_allocation_count, 0)
assert_eq_int(global_gc_count, 0)

fr fr Test allocation tracking with profiler inactive
test_start("track_allocation_inactive")
sus inactive_result lit = track_allocation("ptr_004", 128, "test")
assert_false(inactive_result)

fr fr Test edge cases
test_start("track_deallocation_not_found")
sus not_found_result lit = track_deallocation("nonexistent_ptr")
assert_false(not_found_result)

test_start("allocation_limit_test")
fr fr Test allocation tracking limit
start_profiling()
bestie i := 0; i < 1005; i++ {
    track_allocation("ptr_" + stringz.from_int(i), 64, "test")
}
fr fr Should handle overflow gracefully
assert_eq_int(global_allocation_count, 1000) fr fr Should cap at limit

print_test_summary()
