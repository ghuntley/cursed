yeet "testz"
yeet "testz"

# Test error reporting capabilities of the enhanced testz framework

# ===============================
# Error Reporting Test Suite
# ===============================

slay test_assertion_failures() {
    vibez.spill("❌ Testing assertion failure reporting...")
    
    testz.set_verbose_mode(based)
    testz.set_test_suite("Error Reporting Tests")
    
    # Test 1: Basic assertion failure
    testz.test_start("Basic assertion failure test")
    vibez.spill("  🔍 Testing basic assertion failure...")
    testz.assert_true(cap)  # This should fail
    testz.test_end()
    
    # Test 2: String comparison failure
    testz.test_start("String comparison failure test")
    vibez.spill("  🔍 Testing string comparison failure...")
    enhanced_testz.assert_eq_with_diff("expected", "actual", "String mismatch test")
    testz.test_end()
    
    # Test 3: Integer comparison failure
    testz.test_start("Integer comparison failure test")
    vibez.spill("  🔍 Testing integer comparison failure...")
    testz.assert_eq_int(42, 24)  # This should fail
    testz.test_end()
    
    # Test 4: Range assertion failure
    testz.test_start("Range assertion failure test")
    vibez.spill("  🔍 Testing range assertion failure...")
    testz.assert_range_int(150, 0, 100)  # This should fail
    testz.test_end()
    
    # Test 5: Enhanced context assertion failure
    testz.test_start("Context assertion failure test")
    vibez.spill("  🔍 Testing context assertion failure...")
    enhanced_testz.assert_with_context(cap, "Context-aware assertion", "test_context")
    testz.test_end()
}

slay test_detailed_error_reporting() {
    vibez.spill("🔍 Testing detailed error reporting...")
    
    testz.set_test_suite("Detailed Error Reporting Tests")
    
    # Test 1: Create detailed error report
    testz.test_start("Detailed error report creation")
    enhanced_testz.create_detailed_error_report(
        "sample_test",
        "Assertion failed with detailed context",
        "true",
        "false"
    )
    testz.assert_true(based)  # This passes to show contrast
    testz.test_end()
    
    # Test 2: Error with stack trace
    testz.test_start("Error with stack trace")
    enhanced_testz.create_test_error(
        "Stack trace error message",
        "Function: test_detailed_error_reporting",
        "test_error_reporting.csd",
        67
    )
    testz.assert_true(based)  # This passes to show contrast
    testz.test_end()
    
    # Test 3: Multiple error context
    testz.test_start("Multiple error context test")
    enhanced_testz.assert_with_context(cap, "First context error", "context_1")
    enhanced_testz.assert_with_context(cap, "Second context error", "context_2")
    testz.test_end()
}

slay test_property_test_failures() {
    vibez.spill("🔬 Testing property test failure reporting...")
    
    testz.set_test_suite("Property Test Failure Reporting")
    
    # Test 1: Property test with intentional failure
    testz.test_start("Property test failure")
    testz.property_test_start("Intentional failure property test", 20)
    
    bestie i := 0; i < 20; i++ {
        testz.property_test_iteration()
        sus a normie = testz.random_int(1, 100)
        sus b normie = testz.random_int(1, 100)
        
        # Intentional failure condition
        fr fr i == 10 {
            testz.property_test_fail("Intentional failure at iteration " + tea(i))
        }
        
        # Test property (this should pass for other iterations)
        fr fr (a + b) != (b + a) {
            testz.property_test_fail("Commutative property failed for " + tea(a) + " + " + tea(b))
        }
    }
    
    testz.property_test_end()
    testz.test_end()
}

slay test_timeout_failure_reporting() {
    vibez.spill("⏰ Testing timeout failure reporting...")
    
    testz.set_test_suite("Timeout Failure Reporting")
    
    # Test 1: Timeout failure
    testz.test_start("Timeout failure test")
    enhanced_testz.run_test_with_timeout("timeout_test", 1)  # Very short timeout
    testz.test_end()
    
    # Test 2: Retry failure
    testz.test_start("Retry failure test")
    enhanced_testz.run_test_with_retry("retry_test", 2)  # Will fail after retries
    testz.test_end()
}

slay test_benchmark_failure_reporting() {
    vibez.spill("📊 Testing benchmark failure reporting...")
    
    testz.set_test_suite("Benchmark Failure Reporting")
    
    # Test 1: Performance validation failure
    testz.test_start("Performance validation failure")
    enhanced_testz.benchmark_with_validation("failing_benchmark", 10, "slow_function")
    testz.test_end()
    
    # Test 2: Approximation failure
    testz.test_start("Approximation failure test")
    enhanced_testz.assert_approximately_equal(100, 200, 50)  # This should fail
    testz.test_end()
}

slay test_array_and_pattern_failures() {
    vibez.spill("🔍 Testing array and pattern failure reporting...")
    
    testz.set_test_suite("Array and Pattern Failure Reporting")
    
    # Test 1: Array comparison failure
    testz.test_start("Array comparison failure")
    enhanced_testz.assert_array_equals("[1,2,3]", "[1,2,4]")  # This should fail
    testz.test_end()
    
    # Test 2: Pattern matching failure
    testz.test_start("Pattern matching failure")
    enhanced_testz.assert_matches_pattern("hello world", "xyz")  # This should fail
    testz.test_end()
    
    # Test 3: Between assertion failure
    testz.test_start("Between assertion failure")
    enhanced_testz.assert_between(150, 0, 100)  # This should fail
    testz.test_end()
}

slay test_error_report_generation() {
    vibez.spill("📋 Testing error report generation...")
    
    testz.set_test_suite("Error Report Generation")
    
    # Test 1: Generate reports with errors
    testz.test_start("Error report generation")
    
    # Create some failures first
    testz.assert_true(cap)  # Failure
    testz.assert_eq_string("expected", "actual")  # Failure
    testz.assert_eq_int(42, 24)  # Failure
    
    # Generate reports
    enhanced_testz.generate_test_report("json")
    enhanced_testz.generate_test_report("xml")
    enhanced_testz.generate_test_report("html")
    enhanced_testz.generate_test_report("text")
    
    testz.test_end()
}

# ===============================
# Main Error Reporting Test Runner
# ===============================

slay main() {
    vibez.spill("🚀 Starting Enhanced Testz Error Reporting Tests")
    vibez.spill("================================================")
    vibez.spill("⚠️  NOTE: This test suite intentionally contains failures")
    vibez.spill("   to demonstrate error reporting capabilities")
    vibez.spill("================================================")
    
    # Run all error reporting tests
    test_assertion_failures()
    test_detailed_error_reporting()
    test_property_test_failures()
    test_timeout_failure_reporting()
    test_benchmark_failure_reporting()
    test_array_and_pattern_failures()
    test_error_report_generation()
    
    vibez.spill("================================================")
    vibez.spill("✅ Error reporting tests complete")
    vibez.spill("📊 Error reporting capabilities demonstrated")
    vibez.spill("🎉 Enhanced testz framework error handling verified")
    vibez.spill("================================================")
    
    # Final summary
    testz.after_all_tests()
}

# Run the main function
main()
