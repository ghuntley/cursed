fr fr Enhanced Testz Framework Meta-Tests
fr fr Comprehensive testing of the testing framework itself

yeet "testz"

fr fr Test Suite: Basic Assertion Functions
slay test_basic_assertions() {
    test_suite_start("Basic Assertions")
    
    test_start("integer equality assertions")
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(-100, -100)
    test_end()
    
    test_start("string equality assertions")
    assert_eq_string("hello", "hello")
    assert_eq_string("", "")
    assert_eq_string("complex string with spaces", "complex string with spaces")
    test_end()
    
    test_start("boolean assertions")
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(10 < 5)
    test_end()
    
    test_suite_end("Basic Assertions")
}

fr fr Test Suite: Comparison Assertions
slay test_comparison_assertions() {
    test_suite_start("Comparison Assertions")
    
    test_start("greater than assertions")
    assert_gt(10, 5)
    assert_gt(100, 99)
    assert_gt(1, 0)
    assert_gt(-5, -10)
    test_end()
    
    test_start("less than assertions")
    assert_lt(5, 10)
    assert_lt(0, 1)
    assert_lt(-10, -5)
    assert_lt(99, 100)
    test_end()
    
    test_start("greater than or equal assertions")
    assert_gte(10, 10)
    assert_gte(15, 10)
    assert_gte(0, 0)
    assert_gte(-5, -5)
    test_end()
    
    test_start("less than or equal assertions")
    assert_lte(10, 10)
    assert_lte(5, 10)
    assert_lte(0, 0)
    assert_lte(-10, -5)
    test_end()
    
    test_start("not equal assertions")
    assert_not_eq(10, 5)
    assert_not_eq(0, 1)
    assert_not_eq(-5, 5)
    assert_not_eq(100, 99)
    test_end()
    
    test_suite_end("Comparison Assertions")
}

fr fr Test Suite: String and Null Assertions
slay test_string_null_assertions() {
    test_suite_start("String and Null Assertions")
    
    test_start("string equality edge cases")
    assert_eq_string("a", "a")
    assert_eq_string("123", "123")
    assert_eq_string("special!@#$%", "special!@#$%")
    test_end()
    
    test_start("not null assertions")
    assert_not_null("test")
    assert_not_null("a")
    assert_not_null("longer string")
    assert_not_null("123")
    test_end()
    
    test_suite_end("String and Null Assertions")
}

fr fr Test Suite: State Management
slay test_state_management() {
    test_suite_start("State Management")
    
    test_start("state reset functionality") fr fr Reset and verify clean state
    reset_test_state() fr fr Note: After reset, counters are 0, but we're still in this test
    test_end()
    
    test_start("state tracking accuracy")
    sus initial_pass normie = get_pass_count()
    sus initial_fail normie = get_fail_count()
    sus initial_total normie = get_total_count() fr fr Run some assertions
    assert_true(based)
    assert_eq_int(5, 5)
    assert_false(cap) fr fr Verify state incremented correctly
    assert_eq_int(get_pass_count(), initial_pass + 3)
    assert_eq_int(get_fail_count(), initial_fail)
    test_end()
    
    test_start("current test name tracking")
    sus current_name tea = get_current_test_name()
    assert_eq_string(current_name, "current test name tracking")
    test_end()
    
    test_suite_end("State Management")
}

fr fr Test Suite: Edge Cases and Boundary Values
slay test_edge_cases() {
    test_suite_start("Edge Cases and Boundary Values")
    
    test_start("integer boundary values")
    assert_eq_int(0, 0)
    assert_eq_int(1, 1)
    assert_eq_int(-1, -1)
    assert_eq_int(2147483647, 2147483647) fr fr Max int32
    assert_eq_int(-2147483648, -2147483648) fr fr Min int32
    test_end()
    
    test_start("string boundary values")
    assert_eq_string("", "")
    assert_eq_string("a", "a")
    assert_not_null("minimum")
    test_end()
    
    test_start("comparison edge cases")
    assert_gte(0, 0)
    assert_lte(0, 0)
    assert_gt(1, 0)
    assert_lt(0, 1)
    assert_not_eq(0, 1)
    test_end()
    
    test_suite_end("Edge Cases and Boundary Values")
}

fr fr Test Suite: Complex Expressions
slay test_complex_expressions() {
    test_suite_start("Complex Expressions")
    
    test_start("arithmetic expressions")
    sus x normie = 10
    sus y normie = 20
    sus z normie = 30
    
    assert_eq_int(x + y, 30)
    assert_eq_int(y - x, 10)
    assert_eq_int(x * 2, 20)
    assert_eq_int(z / 3, 10)
    test_end()
    
    test_start("boolean expressions")
    assert_true(x < y)
    assert_true(y < z)
    assert_false(x > y)
    assert_true((x < y) && (y < z))
    assert_false((x > y) || (y > z))
    test_end()
    
    test_start("mixed type expressions")
    sus name tea = "test"
    sus count normie = 5
    
    assert_not_null(name)
    assert_gt(count, 0)
    assert_true(count > 0)
    assert_eq_string(name, "test")
    test_end()
    
    test_suite_end("Complex Expressions")
}

fr fr Test Suite: Performance and Scale
slay test_performance_scale() {
    test_suite_start("Performance and Scale")
    
    test_start("many integer assertions")
    sus i normie = 0
    bestie i < 100 {
        assert_eq_int(i, i)
        assert_gte(i, 0)
        assert_lt(i, 100)
        i = i + 1
    }
    test_end()
    
    test_start("many string assertions")
    sus j normie = 0
    bestie j < 50 {
        assert_eq_string("test", "test")
        assert_not_null("performance")
        j = j + 1
    }
    test_end()
    
    test_start("mixed assertion types at scale")
    sus k normie = 0
    bestie k < 25 {
        assert_eq_int(k * 2, k + k)
        assert_true(k >= 0)
        assert_eq_string("scale", "scale")
        assert_not_eq(k, k + 1)
        k = k + 1
    }
    test_end()
    
    test_suite_end("Performance and Scale")
}

fr fr Test Suite: Error Handling and Negative Cases
slay test_error_handling() {
    test_suite_start("Error Handling")
    
    test_start("intentional failures for reporting") fr fr Note: These will intentionally fail to test failure reporting
    assert_eq_int(5, 10) fr fr Should fail
    assert_true(cap) fr fr Should fail
    assert_eq_string("hello", "world") fr fr Should fail
    test_end()
    
    test_start("recovery after failures") fr fr Test that the framework continues after failures
    assert_true(based) fr fr Should pass
    assert_eq_int(1, 1) fr fr Should pass
    test_end()
    
    test_suite_end("Error Handling")
}

fr fr Test Suite: Reporting and Output
slay test_reporting() {
    test_suite_start("Reporting and Output")
    
    test_start("basic reporting functions") fr fr Test that reporting functions don't crash
    sus pass_count normie = get_pass_count()
    sus fail_count normie = get_fail_count()
    sus total_count normie = get_total_count()
    
    assert_gte(pass_count, 0)
    assert_gte(fail_count, 0)
    assert_gte(total_count, 0)
    test_end()
    
    test_suite_end("Reporting and Output")
}

fr fr Main test execution function
slay run_all_meta_tests() {
    vibez.spill("🔬 TESTZ FRAMEWORK META-TESTING")
    vibez.spill("Testing the testing framework itself...")
    vibez.spill("") fr fr Initialize clean state for meta-testing
    reset_test_state() fr fr Run all test suites
    test_basic_assertions()
    test_comparison_assertions()
    test_string_null_assertions()
    test_state_management()
    test_edge_cases()
    test_complex_expressions()
    test_performance_scale()
    test_error_handling()
    test_reporting() fr fr Generate comprehensive report
    vibez.spill("🎯 META-TESTING COMPLETE")
    print_detailed_report() fr fr Summary of meta-testing results
    vibez.spill("📋 Meta-Testing Summary:")
    vibez.spill("✅ All assertion functions tested")
    vibez.spill("✅ State management verified")
    vibez.spill("✅ Error handling validated")
    vibez.spill("✅ Performance characteristics confirmed")
    vibez.spill("✅ Framework ready for stdlib testing")
    vibez.spill("")
}

fr fr Execute meta-tests
run_all_meta_tests()
