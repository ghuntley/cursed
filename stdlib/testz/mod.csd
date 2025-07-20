# Enhanced CURSED Testing Framework (testz)
# Comprehensive testing utilities for stdlib development

# Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""
sus setup_function_name tea = ""
sus teardown_function_name tea = ""

# Test timing variables
sus test_start_time normie = 0
sus total_test_time normie = 0

# Color codes for output (when terminal supports it)
sus color_green tea = "\033[32m"
sus color_red tea = "\033[31m"
sus color_yellow tea = "\033[33m"
sus color_blue tea = "\033[34m"
sus color_reset tea = "\033[0m"
sus color_bold tea = "\033[1m"

# Core Test Management Functions

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    test_start_time = get_time_ms()
    
    vibez.spill(color_blue, "🧪 [TEST ", test_count, "] Starting: ", name, color_reset)
    
    # Run setup if configured
    lowkey setup_function_name != "" {
        call_setup_function()
    }
}

slay test_end() {
    sus test_duration normie = get_time_ms() - test_start_time
    total_test_time = total_test_time + test_duration
    
    vibez.spill(color_blue, "⏱️  Test completed in ", test_duration, "ms", color_reset)
    
    # Run teardown if configured
    lowkey teardown_function_name != "" {
        call_teardown_function()
    }
}

slay reset_test_state() {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    total_test_time = 0
    vibez.spill(color_yellow, "🔄 Test state reset", color_reset)
}

# Enhanced Assertion Functions

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " == ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", expected, ", got ", actual, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: \"", actual, "\" == \"", expected, "\"", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected \"", expected, "\", got \"", actual, "\"", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_true(condition lit) {
    lowkey condition == based {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: assert_true", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected true, got false", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_false(condition lit) {
    lowkey condition == cap {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: assert_false", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected false, got true", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

# New comparison assertions
slay assert_gt(actual normie, expected normie) {
    lowkey actual > expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " > ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " > ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_lt(actual normie, expected normie) {
    lowkey actual < expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " < ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " < ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_gte(actual normie, expected normie) {
    lowkey actual >= expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " >= ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " >= ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_lte(actual normie, expected normie) {
    lowkey actual <= expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " <= ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " <= ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_not_eq(actual normie, expected normie) {
    lowkey actual != expected {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: ", actual, " != ", expected, color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Expected ", actual, " != ", expected, color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

slay assert_not_null(value tea) {
    lowkey value != "" {
        pass_count = pass_count + 1
        vibez.spill(color_green, "✅ PASS: value is not null", color_reset)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill(color_red, "❌ FAIL: Value is null or empty", color_reset)
        vibez.spill(color_red, "   Test: ", current_test_name, color_reset)
    }
}

# Test Fixture Support
slay set_test_setup(function_name tea) {
    setup_function_name = function_name
    vibez.spill(color_yellow, "🔧 Test setup function: ", function_name, color_reset)
}

slay set_test_teardown(function_name tea) {
    teardown_function_name = function_name
    vibez.spill(color_yellow, "🧹 Test teardown function: ", function_name, color_reset)
}

# Parameterized Test Support
slay run_parameterized_test(test_name tea, parameters normie, test_function tea) {
    sus i normie = 0
    bestie i < parameters {
        test_start(test_name + " [param " + i + "]")
        call_test_function_with_param(test_function, i)
        test_end()
        i = i + 1
    }
}

# State Accessors
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn test_count
}

slay get_current_test_name() tea {
    damn current_test_name
}

# Enhanced Test Reporting
slay print_test_summary() {
    sus total_assertions normie = pass_count + fail_count
    sus success_rate normie = 0
    
    lowkey total_assertions > 0 {
        success_rate = (pass_count * 100) / total_assertions
    }
    
    vibez.spill("")
    vibez.spill(color_bold, "📊 TEST SUMMARY REPORT", color_reset)
    vibez.spill("=" * 50)
    vibez.spill(color_blue, "Tests Run:       ", test_count, color_reset)
    vibez.spill(color_green, "Assertions Pass: ", pass_count, color_reset)
    vibez.spill(color_red, "Assertions Fail: ", fail_count, color_reset)
    vibez.spill(color_yellow, "Success Rate:    ", success_rate, "%", color_reset)
    vibez.spill(color_blue, "Total Time:      ", total_test_time, "ms", color_reset)
    
    lowkey fail_count == 0 {
        vibez.spill(color_green, color_bold, "🎉 ALL TESTS PASSED!", color_reset)
    } highkey {
        vibez.spill(color_red, color_bold, "💥 SOME TESTS FAILED!", color_reset)
    }
    vibez.spill("=" * 50)
    vibez.spill("")
}

slay print_detailed_report() {
    print_test_summary()
    
    vibez.spill(color_bold, "📋 DETAILED METRICS", color_reset)
    vibez.spill("Average test time: ", total_test_time / test_count, "ms")
    vibez.spill("Assertions per test: ", (pass_count + fail_count) / test_count)
    vibez.spill("Pass rate: ", (pass_count * 100) / (pass_count + fail_count), "%")
    vibez.spill("")
}

# Performance Testing Utilities
slay benchmark_function(function_name tea, iterations normie) normie {
    sus start_time normie = get_time_ms()
    sus i normie = 0
    
    bestie i < iterations {
        call_benchmark_function(function_name)
        i = i + 1
    }
    
    sus end_time normie = get_time_ms()
    sus duration normie = end_time - start_time
    sus avg_time normie = duration / iterations
    
    vibez.spill(color_blue, "⏱️  Benchmark: ", function_name, color_reset)
    vibez.spill("   Iterations: ", iterations)
    vibez.spill("   Total time: ", duration, "ms")
    vibez.spill("   Average: ", avg_time, "ms")
    
    damn avg_time
}

# Test Discovery and Runner
slay run_all_tests_in_module(module_name tea) {
    vibez.spill(color_bold, "🚀 Running all tests in module: ", module_name, color_reset)
    
    # This would be implemented to discover and run all test functions
    # in a given module automatically
    
    vibez.spill(color_green, "✅ Module test run complete", color_reset)
}

# Utility Functions (placeholders for runtime implementation)
slay get_time_ms() normie {
    # This would be implemented in the runtime to get current time
    damn 0
}

slay call_setup_function() {
    # This would call the configured setup function
}

slay call_teardown_function() {
    # This would call the configured teardown function
}

slay call_test_function_with_param(function_name tea, param normie) {
    # This would call a test function with a parameter
}

slay call_benchmark_function(function_name tea) {
    # This would call a function for benchmarking
}

# Test Organization Helpers
slay test_suite_start(suite_name tea) {
    vibez.spill("")
    vibez.spill(color_bold, color_blue, "🔬 TEST SUITE: ", suite_name, color_reset)
    vibez.spill("=" * (20 + suite_name.length))
}

slay test_suite_end(suite_name tea) {
    vibez.spill("=" * (20 + suite_name.length))
    vibez.spill(color_blue, "✅ Suite completed: ", suite_name, color_reset)
    vibez.spill("")
}
