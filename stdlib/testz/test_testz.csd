fr fr ================================
fr fr CURSED Enhanced Testing Framework Self-Tests
fr fr Tests the testz module itself to verify all functionality
fr fr ================================

fr fr ================================
fr fr Test Suite: Basic Assertions
fr fr ================================

slay test_integer_assertions() {
    test_start("test_integer_assertions")
    
    fr fr Basic equality
    assert_eq_int(42, 42)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    
    fr fr Inequality
    assert_ne_int(42, 43)
    assert_ne_int(1, 2)
    
    fr fr Comparisons
    assert_greater_than(5, 3)
    assert_greater_than(100, 42)
    assert_less_than(3, 5)
    assert_less_than(42, 100)
    
    fr fr Range testing
    assert_in_range(5, 1, 10)
    assert_in_range(42, 40, 50)
    assert_in_range(0, -5, 5)
    
    test_end()
}

slay test_float_assertions() {
    test_start("test_float_assertions")
    
    fr fr Float equality with tolerance
    assert_eq_float(3.14159, 3.14159)
    assert_eq_float(1.0, 1.0)
    assert_eq_float(0.3, 0.3)  fr fr Simple float test
    
    fr fr Custom tolerance
    assert_eq_float_with_tolerance(3.14, 3.141, 0.01)
    assert_eq_float_with_tolerance(1.001, 1.002, 0.01)
    
    test_end()
}

slay test_string_assertions() {
    test_start("test_string_assertions")
    
    fr fr String equality
    assert_eq_string("hello", "hello")
    assert_eq_string("CURSED", "CURSED")
    assert_eq_string("", "")
    
    fr fr String inequality
    assert_ne_string("hello", "world")
    assert_ne_string("CURSED", "cursed")
    
    fr fr String contains (basic implementation)
    assert_string_contains("hello world", "world")
    assert_string_contains("CURSED programming", "CURSED")
    
    fr fr String starts/ends with (basic implementation)
    assert_string_starts_with("hello world", "hello")
    assert_string_ends_with("hello world", "world")
    
    test_end()
}

slay test_boolean_assertions() {
    test_start("test_boolean_assertions")
    
    fr fr Boolean values
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
    
    fr fr Boolean equality
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_eq_bool(5 > 3, based)
    assert_eq_bool(3 > 5, cap)
    
    test_end()
}

slay test_nil_assertions() {
    test_start("test_nil_assertions")
    
    fr fr Nil testing
    assert_nil("cringe")
    assert_not_nil("not cringe")
    assert_not_nil("hello")
    assert_not_nil("42")
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Advanced Features
fr fr ================================

slay test_test_suites() {
    suite_start("Advanced Features Suite")
    
    test_start("test_suite_functionality")
    
    fr fr Test suite-related functionality
    assert_eq_string(current_suite_name, "Advanced Features Suite")
    assert_greater_than(suite_count, 0)
    
    test_end()
    
    suite_end()
}

slay test_performance_testing() {
    test_start("test_performance_testing")
    
    fr fr Benchmark testing
    sus start_time normie = benchmark_start()
    
    fr fr Simulate some work
    sus result normie = 0
    bestie i := 0; i < 1000; i++ {
        result = result + i
    }
    
    benchmark_end(start_time)
    assert_eq_int(result, 499500)  fr fr Sum of 0 to 999
    
    test_end()
}

slay test_mock_functions() {
    test_start("test_mock_functions")
    
    fr fr Create a mock function
    sus mock MockFunction = create_mock("test_function")
    
    fr fr Configure mock behavior
    mock_return(mock, "mocked_result")
    
    fr fr Test mock configuration
    assert_eq_string(mock.name, "test_function")
    assert_eq_string(mock.return_value, "mocked_result")
    assert_eq_int(mock.call_count, 0)
    assert_false(mock.should_throw)
    
    fr fr Test mock error throwing
    mock_throw(mock, "Mocked error")
    assert_true(mock.should_throw)
    assert_eq_string(mock.throw_message, "Mocked error")
    
    test_end()
}

slay test_error_handling() {
    test_start("test_error_handling")
    
    fr fr Test error assertion functions
    assert_throws("Expected error message")
    assert_no_throw()
    expect_panic("risky_function")
    
    test_end()
}

slay test_array_assertions() {
    test_start("test_array_assertions")
    
    fr fr Array testing (basic implementation)
    sus test_array [normie] = [1, 2, 3, 4, 5]
    sus expected_array [normie] = [1, 2, 3, 4, 5]
    
    assert_array_eq_int(test_array, expected_array)
    assert_array_contains_int(test_array, 3)
    assert_array_not_contains_int(test_array, 10)
    assert_array_length(test_array, 5)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Configuration and Reporting
fr fr ================================

slay test_configuration() {
    test_start("test_configuration")
    
    fr fr Test default configuration
    sus config TestConfig = create_default_config()
    
    assert_eq_int(config.timeout, 5000)
    assert_true(config.verbose)
    assert_false(config.fail_fast)
    assert_false(config.parallel)
    assert_eq_string(config.test_dir, "tests/")
    assert_eq_string(config.pattern, "test_*")
    assert_eq_string(config.output_format, "console")
    assert_false(config.coverage_enabled)
    
    fr fr Test configuration setting
    set_test_config(config)
    
    test_end()
}

slay test_reporting_formats() {
    test_start("test_reporting_formats")
    
    fr fr Test different output formats
    vibez.spill("Testing JSON report generation:")
    generate_json_report()
    
    vibez.spill("Testing XML report generation:")
    generate_xml_report()
    
    vibez.spill("Testing HTML report generation:")
    generate_html_report()
    
    test_end()
}

slay test_filtering() {
    test_start("test_filtering")
    
    fr fr Test filtering functionality
    assert_true(should_run_test("test_example", "test_*"))
    assert_true(should_run_test("my_test", "*test*"))
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Edge Cases
fr fr ================================

slay test_edge_cases() {
    test_start("test_edge_cases")
    
    fr fr Test boundary conditions
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(2147483647, 2147483647)  fr fr Max 32-bit int
    
    fr fr Test empty strings
    assert_eq_string("", "")
    assert_ne_string("", " ")
    
    fr fr Test floating point edge cases
    assert_eq_float(0.0, 0.0)
    assert_eq_float(1.0, 1.0)
    
    test_end()
}

slay test_skipping() {
    test_start("test_skipping")
    
    fr fr Test skipping functionality
    test_skip("This test is intentionally skipped for demonstration")
    
    test_end()
}

fr fr ================================
fr fr Test Suite: State Management
fr fr ================================

slay test_state_management() {
    test_start("test_state_management")
    
    fr fr Test state tracking
    sus initial_count normie = test_count
    sus initial_passed normie = test_passed
    sus initial_failed normie = test_failed
    
    fr fr These should increment counters
    assert_eq_int(42, 42)  fr fr Should increment passed
    
    fr fr Verify state changes
    assert_greater_than(test_passed, initial_passed)
    assert_greater_than(test_count, initial_count)
    
    test_end()
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 Starting CURSED Enhanced Testing Framework Self-Tests...")
    vibez.spill("")
    
    fr fr Reset test state for clean run
    reset_test_state()
    
    fr fr Run all test suites
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_integer_assertions()
    test_float_assertions()
    test_string_assertions()
    test_boolean_assertions()
    test_nil_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_test_suites()
    test_performance_testing()
    test_mock_functions()
    test_error_handling()
    test_array_assertions()
    
    vibez.spill("=== Running Configuration and Reporting Tests ===")
    test_configuration()
    test_reporting_formats()
    test_filtering()
    
    vibez.spill("=== Running Edge Case Tests ===")
    test_edge_cases()
    test_skipping()
    
    vibez.spill("=== Running State Management Tests ===")
    test_state_management()
    
    fr fr Print comprehensive test summary
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🎯 Enhanced CURSED Testing Framework validation complete!")
    vibez.spill("")
    
    fr fr Test different output formats
    vibez.spill("=== Alternative Output Formats ===")
    vibez.spill("JSON Report:")
    generate_json_report()
    vibez.spill("")
    
    vibez.spill("XML Report:")
    generate_xml_report()
    vibez.spill("")
    
    vibez.spill("HTML Report:")
    generate_html_report()
    vibez.spill("")
    
    fr fr Return appropriate exit code
    lowkey test_failed > 0 {
        vibez.spill("❌ Some tests failed - returning exit code 1")
        damn 1
    } highkey {
        vibez.spill("✅ All tests passed - returning exit code 0")
        damn 0
    }
}
