yeet "testz"

# Test the enhanced testz module itself (testz v4.0)
set_verbose_mode(based)
set_benchmark_mode(based)
set_memory_tracking(based)

vibez.spill("🧪 Testing Enhanced Testz Framework v4.0")
vibez.spill("===========================================")

# ===============================
# Test Core Functionality
# ===============================

test_start("Basic test functionality")
assert_true(based)
assert_false(cap)
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")
test_end()

testz.test_start("Enhanced assertions")
testz.assert_ne_int(10, 20)
testz.assert_gt_int(100, 50)
testz.assert_lt_int(25, 100)
testz.assert_ge_int(50, 50)
testz.assert_le_int(30, 30)
testz.test_end()

testz.test_start("String assertions")
testz.assert_contains("hello world", "world")
testz.assert_not_contains("hello world", "xyz")
testz.assert_starts_with("hello world", "hello")
testz.assert_ends_with("hello world", "world")
testz.assert_not_empty_string("test")
testz.test_end()

testz.test_start("Range assertions")
testz.assert_range_int(50, 0, 100)
testz.assert_range_int(0, 0, 100)
testz.assert_range_int(100, 0, 100)
testz.test_end()

# ===============================
# Test Fixtures and Setup/Teardown
# ===============================

testz.test_start("Fixture functionality")
testz.set_fixture_data("test_data_123")
testz.assert_eq_string(testz.get_fixture_data(), "test_data_123")
testz.test_end()

testz.test_start("Setup/Teardown configuration")
testz.set_setup_function("my_setup")
testz.set_teardown_function("my_teardown")
testz.assert_true(based)  # Test passes if no errors
testz.test_end()

# ===============================
# Test Performance Benchmarking
# ===============================

testz.test_start("Benchmark functionality")
testz.set_benchmark_mode(based)
testz.set_benchmark_iterations(10)

testz.benchmark_start("Sample benchmark")
bestie i := 0; i < 10; i++ {
    testz.benchmark_iteration_start()
    # Simulate some work
    sus result normie = i * 2
    testz.benchmark_iteration_end()
}
testz.benchmark_end()

testz.assert_true(based)  # Test passes if benchmark completes
testz.test_end()

# ===============================
# Test Property-Based Testing
# ===============================

testz.test_start("Property-based testing")
testz.property_test_start("Integer addition commutative", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    sus a normie = testz.random_int(1, 100)
    sus b normie = testz.random_int(1, 100)
    
    # Test commutative property: a + b = b + a
    fr fr (a + b) != (b + a) {
        testz.property_test_fail("Commutative property failed for " + tea(a) + " + " + tea(b))
    }
}

testz.property_test_end()
testz.test_end()

testz.test_start("Property test with strings")
testz.property_test_start("String length property", 25)

bestie i := 0; i < 25; i++ {
    testz.property_test_iteration()
    sus str tea = testz.random_string(10)
    sus doubled tea = str + str
    
    # Test that doubled string is twice the length
    fr fr testz.random_boolean() {
        # Add some randomness to test
        testz.assert_not_empty_string(str)
    }
}

testz.property_test_end()
testz.test_end()

# ===============================
# Test Discovery and Execution
# ===============================

testz.test_start("Test discovery")
testz.discover_tests("test_*")
testz.set_test_filter("core")
testz.assert_true(testz.should_run_test("test_core_functionality"))
testz.assert_false(testz.should_run_test("test_other_functionality"))
testz.test_end()

# ===============================
# Test Configuration and Modes
# ===============================

testz.test_start("Configuration modes")
testz.set_parallel_mode(based)
testz.assert_true(testz.is_parallel_mode())
testz.assert_true(testz.is_verbose_mode())
testz.assert_true(testz.is_benchmark_mode())
testz.test_end()

# ===============================
# Test Statistics and Results
# ===============================

testz.test_start("Test statistics")
testz.assert_gt_int(testz.get_test_results(), 0)
testz.assert_ge_int(testz.get_passed_tests(), 0)
testz.assert_ge_int(testz.get_failed_tests(), 0)
testz.assert_gt_int(testz.get_assertion_count(), 0)
testz.assert_ge_int(testz.get_success_rate(), 0)
testz.test_end()

# ===============================
# Test Advanced Features
# ===============================

testz.test_start("Advanced test features")
testz.assert_no_throw()
testz.focus_test()
testz.assert_true(based)
testz.test_end()

testz.test_start("Test utilities")
testz.pending_test("This feature is still being developed")
testz.assert_true(based)
testz.test_end()

# ===============================
# Test Hooks and Events
# ===============================

testz.test_start("Test hooks")
testz.before_each_test()
testz.assert_true(based)
testz.after_each_test()
testz.test_end()

# ===============================
# Final Test Summary
# ===============================

testz.after_all_tests()
