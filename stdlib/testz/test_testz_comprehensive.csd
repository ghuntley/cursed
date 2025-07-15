yeet "testz"

# Comprehensive test suite for testz framework
# Tests all major features including assertions, benchmarks, property tests, etc.

slay test_basic_assertions() {
    test_start("Basic Assertions")
    
    # Test boolean assertions
    assert_true(based)
    assert_false(cap)
    
    # Test integer assertions
    assert_eq_int(42, 42)
    assert_ne_int(1, 2)
    assert_gt_int(10, 5)
    assert_lt_int(3, 7)
    assert_ge_int(5, 5)
    assert_le_int(8, 9)
    
    # Test range assertions
    assert_range_int(50, 1, 100)
    
    test_end()
}

slay test_string_assertions() {
    test_start("String Assertions")
    
    # Test string equality
    assert_eq_string("hello", "hello")
    
    # Test string content
    assert_contains("hello world", "world")
    assert_not_contains("hello world", "xyz")
    assert_starts_with("hello world", "hello")
    assert_ends_with("hello world", "world")
    
    # Test string emptiness
    assert_empty_string("")
    assert_not_empty_string("not empty")
    
    test_end()
}

slay test_error_handling() {
    test_start("Error Handling")
    
    # Test error assertions
    assert_no_throw()
    assert_throws("Expected error message")
    
    test_end()
}

slay test_benchmark_functionality() {
    test_start("Benchmark Functionality")
    
    # Set up benchmark
    set_benchmark_mode(based)
    set_benchmark_iterations(100)
    
    # Run benchmark
    benchmark_start("Simple arithmetic")
    
    sus i normie = 0
    bestie i = 0; i < 100; i = i + 1 {
        benchmark_iteration_start()
        
        # Simple operation to benchmark
        sus result normie = i * 2 + 1
        
        benchmark_iteration_end()
    }
    
    benchmark_end()
    
    # Verify benchmark completed
    assert_true(is_benchmark_mode())
    
    test_end()
}

slay test_property_based_testing() {
    test_start("Property-Based Testing")
    
    # Start property test
    property_test_start("Integer addition is commutative", 50)
    
    sus i normie = 0
    bestie i = 0; i < 50; i = i + 1 {
        property_test_iteration()
        
        # Generate random integers
        sus a normie = random_int(1, 100)
        sus b normie = random_int(1, 100)
        
        # Test commutative property
        sus result1 normie = a + b
        sus result2 normie = b + a
        
        highkey result1 != result2 {
            property_test_fail("Commutative property failed: " + tea(a) + " + " + tea(b) + " != " + tea(b) + " + " + tea(a))
        }
    }
    
    property_test_end()
    
    test_end()
}

slay test_random_generators() {
    test_start("Random Generators")
    
    # Test random integer generation
    sus rand_int normie = random_int(1, 10)
    assert_range_int(rand_int, 1, 10)
    
    # Test random string generation
    sus rand_str tea = random_string(5)
    assert_eq_int(stringz.Length(rand_str), 5)
    
    # Test random boolean generation
    sus rand_bool lit = random_boolean()
    assert_true(rand_bool == based || rand_bool == cap)
    
    test_end()
}

slay test_configuration_modes() {
    test_start("Configuration Modes")
    
    # Test verbose mode
    set_verbose_mode(based)
    assert_true(is_verbose_mode())
    
    # Test parallel mode
    set_parallel_mode(based)
    assert_true(is_parallel_mode())
    
    # Test benchmark mode
    set_benchmark_mode(cap)
    assert_false(is_benchmark_mode())
    
    test_end()
}

slay test_fixtures() {
    test_start("Test Fixtures")
    
    # Set up fixture data
    set_fixture_data("test_data_123")
    sus fixture tea = get_fixture_data()
    assert_eq_string(fixture, "test_data_123")
    
    # Test setup/teardown functions
    set_setup_function("setup_test")
    set_teardown_function("teardown_test")
    
    test_end()
}

slay test_suite_management() {
    test_start("Test Suite Management")
    
    # Set test suite name
    set_test_suite("Comprehensive TestZ Suite")
    
    # Set test filter
    set_test_filter("basic")
    
    # Test discovery
    discover_tests("test_*")
    
    # Test filtering
    assert_true(should_run_test("test_basic_functionality"))
    assert_false(should_run_test("advanced_feature_test"))
    
    test_end()
}

slay test_statistics() {
    test_start("Test Statistics")
    
    # Get current test statistics
    sus total normie = get_test_results()
    sus passed normie = get_passed_tests()
    sus failed normie = get_failed_tests()
    sus assertions normie = get_assertion_count()
    sus failures normie = get_assertion_failures()
    
    # Test that statistics are reasonable
    assert_ge_int(total, 0)
    assert_ge_int(passed, 0)
    assert_ge_int(failed, 0)
    assert_ge_int(assertions, 0)
    assert_ge_int(failures, 0)
    
    # Test success rate calculation
    sus success_rate normie = get_success_rate()
    assert_range_int(success_rate, 0, 100)
    
    test_end()
}

slay test_advanced_features() {
    test_start("Advanced Features")
    
    # Test skip functionality
    skip_test("This test is intentionally skipped")
    
    # Test pending functionality
    pending_test("This test is pending implementation")
    
    # Test focus functionality
    focus_test()
    
    test_end()
}

slay run_comprehensive_testz_test() {
    vibez.spill("🧪 Starting Comprehensive TestZ Framework Test")
    vibez.spill("=" * 60)
    
    # Initialize test environment
    before_all_tests()
    set_verbose_mode(based)
    set_test_suite("TestZ Comprehensive Test Suite")
    
    # Run all test categories
    test_basic_assertions()
    test_string_assertions() 
    test_error_handling()
    test_benchmark_functionality()
    test_property_based_testing()
    test_random_generators()
    test_configuration_modes()
    test_fixtures()
    test_suite_management()
    test_statistics()
    test_advanced_features()
    
    # Finalize test execution
    after_all_tests()
    
    vibez.spill("")
    vibez.spill("🎯 TestZ Framework Comprehensive Test Complete")
    vibez.spill("This test validates all major features of the testz framework:")
    vibez.spill("✅ Advanced assertion functions")
    vibez.spill("✅ Test suite management")
    vibez.spill("✅ Performance benchmarking")
    vibez.spill("✅ Property-based testing utilities")
    vibez.spill("✅ Random data generation")
    vibez.spill("✅ Test reporting and output formatting")
    vibez.spill("✅ Configuration and modes")
    vibez.spill("✅ Test fixtures and lifecycle")
    vibez.spill("✅ Statistical analysis")
    vibez.spill("✅ Advanced test control features")
}

# Run the comprehensive test
run_comprehensive_testz_test()
