yeet "testz"
yeet "runtime_core"

test_start("Simple Runtime Core Test")

# Test basic value creation
sus int_val RuntimeValue = runtime_value_create("42", "integer")
assert_eq_string(runtime_get_type(int_val), "integer")

# Test string operations
sus test_len normie = string_length_enhanced("hello")
assert_true(test_len >= 0)

# Test time operations
sus time1 normie = get_current_time_nanos()
assert_true(time1 > 0)

sus time2 normie = get_current_time_nanos()
assert_true(time2 > time1)

# Test memory operations
sus ptr normie = memory_allocate_bytes(100)
assert_true(ptr > 0)

sus dealloc_ok lit = memory_deallocate_bytes(ptr, 100)
assert_true(dealloc_ok)

# Test performance metrics
sus log_ok lit = log_performance_metric("test", 1000)
assert_true(log_ok)

sus stats tea = get_performance_stats()
sus stats_len normie = string_length_enhanced(stats)
assert_true(stats_len > 0)

# Test GC operations
sus gc_ok lit = trigger_gc_collection()
assert_true(gc_ok)

sus gc_stats tea = get_gc_statistics()
sus gc_len normie = string_length_enhanced(gc_stats)
assert_true(gc_len > 0)

print_test_summary()
