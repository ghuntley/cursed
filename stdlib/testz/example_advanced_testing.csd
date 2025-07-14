yeet "testz"

# Advanced Testing Framework Example
# Demonstrates all major features of the enhanced testz module

# ===============================
# Test Suite Configuration
# ===============================

testz.set_verbose_mode(based)
testz.set_test_suite("Advanced Testing Framework Demo")
testz.before_all_tests()

# ===============================
# Mock Data Structure for Testing
# ===============================

sus test_data_store tea = ""
sus test_counter normie = 0

slay mock_setup() {
    test_data_store = "initialized"
    test_counter = 0
}

slay mock_teardown() {
    test_data_store = ""
    test_counter = 0
}

slay mock_add_item(item tea) {
    test_data_store = test_data_store + item
    test_counter = test_counter + 1
}

slay mock_get_count() normie {
    damn test_counter
}

slay mock_get_data() tea {
    damn test_data_store
}

# ===============================
# Basic Unit Tests
# ===============================

testz.test_start("Mock data structure initialization")
mock_setup()
testz.assert_eq_string(mock_get_data(), "initialized")
testz.assert_eq_int(mock_get_count(), 0)
mock_teardown()
testz.test_end()

testz.test_start("Mock data structure operations")
mock_setup()
mock_add_item("test1")
mock_add_item("test2")
testz.assert_eq_int(mock_get_count(), 2)
testz.assert_contains(mock_get_data(), "test1")
testz.assert_contains(mock_get_data(), "test2")
mock_teardown()
testz.test_end()

# ===============================
# Property-Based Testing Example
# ===============================

testz.test_start("Property-based testing: Data structure invariants")
testz.property_test_start("Counter increments correctly", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    mock_setup()
    
    # Generate random number of items to add
    sus items_to_add normie = testz.random_int(1, 10)
    
    bestie j := 0; j < items_to_add; j++ {
        sus item tea = testz.random_string(5)
        mock_add_item(item)
    }
    
    # Property: counter should equal number of items added
    fr fr mock_get_count() != items_to_add {
        testz.property_test_fail("Counter mismatch: expected " + tea(items_to_add) + ", got " + tea(mock_get_count()))
    }
    
    mock_teardown()
}

testz.property_test_end()
testz.test_end()

# ===============================
# Performance Benchmarking Example
# ===============================

testz.test_start("Performance benchmarking: Data operations")
testz.set_benchmark_mode(based)
testz.set_benchmark_iterations(1000)

testz.benchmark_start("Mock data structure operations")

bestie i := 0; i < 1000; i++ {
    testz.benchmark_iteration_start()
    mock_setup()
    mock_add_item("benchmark_item")
    sus count normie = mock_get_count()
    mock_teardown()
    testz.benchmark_iteration_end()
}

testz.benchmark_end()
testz.test_end()

# ===============================
# Test Fixtures Example
# ===============================

testz.set_setup_function("mock_setup")
testz.set_teardown_function("mock_teardown")
testz.set_fixture_data("shared_test_data")

testz.test_start("Fixture-based testing")
# Setup is automatically called
mock_add_item("fixture_item")
testz.assert_eq_int(mock_get_count(), 1)
testz.assert_eq_string(testz.get_fixture_data(), "shared_test_data")
# Teardown is automatically called
testz.test_end()

# ===============================
# String Testing with Enhanced Assertions
# ===============================

testz.test_start("String manipulation testing")
sus test_string tea = "Hello, CURSED World!"

testz.assert_starts_with(test_string, "Hello")
testz.assert_ends_with(test_string, "World!")
testz.assert_contains(test_string, "CURSED")
testz.assert_not_contains(test_string, "Python")
testz.assert_not_empty_string(test_string)

# Test string operations
sus upper_string tea = "HELLO, CURSED WORLD!"
sus lower_string tea = "hello, cursed world!"

testz.assert_ne_string(test_string, upper_string)
testz.assert_ne_string(test_string, lower_string)
testz.test_end()

# ===============================
# Numeric Range Testing
# ===============================

testz.test_start("Numeric range and boundary testing")
sus test_value normie = 50

testz.assert_range_int(test_value, 0, 100)
testz.assert_range_int(test_value, 50, 50)  # Boundary test
testz.assert_gt_int(test_value, 0)
testz.assert_lt_int(test_value, 100)
testz.assert_ge_int(test_value, 50)
testz.assert_le_int(test_value, 50)

# Test edge cases
testz.assert_range_int(0, 0, 100)
testz.assert_range_int(100, 0, 100)
testz.test_end()

# ===============================
# Mathematical Property Testing
# ===============================

testz.test_start("Mathematical properties validation")
testz.property_test_start("Arithmetic properties", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    sus a normie = testz.random_int(1, 100)
    sus b normie = testz.random_int(1, 100)
    sus c normie = testz.random_int(1, 100)
    
    # Test associative property: (a + b) + c = a + (b + c)
    fr fr ((a + b) + c) != (a + (b + c)) {
        testz.property_test_fail("Associative property failed")
    }
    
    # Test distributive property: a * (b + c) = (a * b) + (a * c)
    fr fr (a * (b + c)) != ((a * b) + (a * c)) {
        testz.property_test_fail("Distributive property failed")
    }
    
    # Test identity property: a + 0 = a
    fr fr (a + 0) != a {
        testz.property_test_fail("Identity property failed")
    }
}

testz.property_test_end()
testz.test_end()

# ===============================
# Advanced Test Features
# ===============================

testz.test_start("Advanced test features demo")
testz.focus_test()  # Mark this test as focused
testz.assert_no_throw()  # No error should occur
testz.assert_true(testz.is_verbose_mode())
testz.assert_true(testz.is_benchmark_mode())
testz.test_end()

# ===============================
# Test Discovery and Filtering
# ===============================

testz.test_start("Test discovery and filtering")
testz.discover_tests("test_*")
testz.set_test_filter("advanced")

testz.assert_true(testz.should_run_test("test_advanced_features"))
testz.assert_false(testz.should_run_test("test_basic_features"))
testz.test_end()

# ===============================
# Error Handling Testing
# ===============================

testz.test_start("Error handling validation")
# Test successful operations
testz.assert_no_throw()

# Test expected error scenarios
# (In a real scenario, this would test actual error conditions)
testz.assert_throws("Expected error for invalid input")
testz.test_end()

# ===============================
# Performance Comparison Testing
# ===============================

testz.test_start("Performance comparison between approaches")
testz.set_benchmark_iterations(100)

# Benchmark approach 1
testz.benchmark_start("String concatenation approach 1")
bestie i := 0; i < 100; i++ {
    testz.benchmark_iteration_start()
    sus result tea = "hello" + "world" + "test"
    testz.benchmark_iteration_end()
}
testz.benchmark_end()

# Benchmark approach 2
testz.benchmark_start("String concatenation approach 2")
bestie i := 0; i < 100; i++ {
    testz.benchmark_iteration_start()
    sus temp tea = "hello" + "world"
    sus result tea = temp + "test"
    testz.benchmark_iteration_end()
}
testz.benchmark_end()

testz.test_end()

# ===============================
# Test Statistics Validation
# ===============================

testz.test_start("Test statistics and reporting")
testz.assert_gt_int(testz.get_test_results(), 0)
testz.assert_ge_int(testz.get_passed_tests(), 0)
testz.assert_ge_int(testz.get_assertion_count(), 0)
testz.assert_ge_int(testz.get_success_rate(), 0)
testz.assert_le_int(testz.get_success_rate(), 100)

# Validate test execution time tracking
testz.assert_ge_int(testz.get_execution_time(), 0)
testz.test_end()

# ===============================
# Comprehensive Test Suite Cleanup
# ===============================

testz.test_start("Test suite cleanup and finalization")
testz.reset_test_state()
testz.assert_eq_int(testz.get_test_results(), 0)  # After reset
testz.test_end()

# ===============================
# Final Test Suite Summary
# ===============================

testz.after_all_tests()

# Display additional statistics
vibez.spill("")
vibez.spill("🎯 Advanced Testing Framework Demo Complete")
vibez.spill("✨ Features demonstrated:")
vibez.spill("  • Enhanced assertions with detailed error messages")
vibez.spill("  • Property-based testing with random generators")
vibez.spill("  • Performance benchmarking with statistical analysis")
vibez.spill("  • Test fixtures with setup/teardown lifecycle")
vibez.spill("  • Test discovery and filtering capabilities")
vibez.spill("  • Comprehensive reporting and statistics")
vibez.spill("  • Error handling and validation testing")
vibez.spill("  • Mathematical property validation")
vibez.spill("  • Advanced test control features")
vibez.spill("")
vibez.spill("🚀 Ready for production use in CURSED stdlib development!")
