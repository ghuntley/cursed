fr fr Enhanced Testz Framework Usage Examples
fr fr Demonstrates all features of the testing framework

yeet "testz"

fr fr Example 1: Basic Testing Pattern
slay example_basic_testing() {
    vibez.spill("📖 Example 1: Basic Testing Pattern")
    
    test_suite_start("Basic Testing")
    
    test_start("simple assertions")
    assert_eq_int(2 + 2, 4)
    assert_true(5 > 3)
    assert_false(10 < 5)
    assert_eq_string("hello", "hello")
    test_end()
    
    test_start("comparison assertions")
    assert_gt(10, 5)
    assert_lt(3, 8)
    assert_gte(10, 10)
    assert_lte(5, 5)
    assert_not_eq(7, 3)
    test_end()
    
    test_suite_end("Basic Testing")
}

fr fr Example 2: Complex Data Testing
slay example_complex_data() {
    vibez.spill("📖 Example 2: Complex Data Testing")
    
    test_suite_start("Complex Data Structures")
    
    test_start("user data validation")
    sus user_name tea = "Alice"
    sus user_age normie = 30
    sus user_active lit = based
    
    assert_not_null(user_name)
    assert_eq_string(user_name, "Alice")
    assert_gt(user_age, 18)
    assert_lte(user_age, 100)
    assert_true(user_active)
    test_end()
    
    test_start("arithmetic operations")
    sus x normie = 15
    sus y normie = 25
    sus sum normie = x + y
    sus diff normie = y - x
    sus product normie = x * 2
    
    assert_eq_int(sum, 40)
    assert_eq_int(diff, 10)
    assert_eq_int(product, 30)
    assert_gt(sum, diff)
    assert_lt(diff, sum)
    test_end()
    
    test_suite_end("Complex Data Structures")
}

fr fr Example 3: Edge Cases and Boundary Testing
slay example_edge_cases() {
    vibez.spill("📖 Example 3: Edge Cases and Boundary Testing")
    
    test_suite_start("Edge Cases")
    
    test_start("integer boundaries")
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1, 1)
    assert_not_eq(0, 1)
    assert_not_eq(-1, 1)
    test_end()
    
    test_start("string boundaries")
    sus empty_string tea = ""
    sus single_char tea = "a"
    sus long_string tea = "this is a longer string for testing"
    
    assert_eq_string(empty_string, "")
    assert_eq_string(single_char, "a")
    assert_not_null(single_char)
    assert_not_null(long_string)
    test_end()
    
    test_start("comparison edge cases")
    assert_gte(-5, -5)
    assert_lte(100, 100)
    assert_gt(1, 0)
    assert_lt(-1, 0)
    test_end()
    
    test_suite_end("Edge Cases")
}

fr fr Example 4: Performance and Scale Testing
slay example_performance_testing() {
    vibez.spill("📖 Example 4: Performance and Scale Testing")
    
    test_suite_start("Performance Testing")
    
    test_start("large iteration test")
    sus i normie = 0
    sus total normie = 0
    
    bestie i < 100 {
        total = total + i
        assert_gte(total, 0)
        assert_eq_int(i, i)
        i = i + 1
    } fr fr Expected sum: 0+1+2+...+99 = 99*100/2 = 4950
    assert_eq_int(total, 4950)
    test_end()
    
    test_start("string performance test")
    sus j normie = 0
    bestie j < 50 {
        assert_eq_string("performance", "performance")
        assert_not_null("test")
        j = j + 1
    }
    test_end()
    
    test_suite_end("Performance Testing")
}

fr fr Example 5: Error Handling and Recovery
slay example_error_handling() {
    vibez.spill("📖 Example 5: Error Handling and Recovery")
    
    test_suite_start("Error Handling")
    
    test_start("mixed success and failure") fr fr These will pass
    assert_eq_int(1, 1)
    assert_true(based) fr fr This will intentionally fail to demonstrate error handling
    assert_eq_int(5, 10) fr fr FAIL: Expected 10, got 5 fr fr These should still execute after the failure
    assert_eq_string("recovery", "recovery")
    assert_false(cap)
    test_end()
    
    test_start("recovery validation") fr fr Verify the framework continues working after failures
    assert_eq_int(42, 42)
    assert_not_eq(1, 2)
    test_end()
    
    test_suite_end("Error Handling")
}

fr fr Example 6: State Management
slay example_state_management() {
    vibez.spill("📖 Example 6: State Management")
    
    test_suite_start("State Management")
    
    test_start("state tracking")
    sus initial_pass normie = get_pass_count()
    sus initial_fail normie = get_fail_count()
    sus initial_total normie = get_total_count() fr fr Run some assertions
    assert_true(based)
    assert_eq_int(10, 10) fr fr Verify state changed
    assert_gt(get_pass_count(), initial_pass)
    test_end()
    
    test_start("test name tracking")
    sus current_name tea = get_current_test_name()
    assert_eq_string(current_name, "test name tracking")
    test_end()
    
    test_suite_end("State Management")
}

fr fr Example 7: Boolean Logic Testing
slay example_boolean_logic() {
    vibez.spill("📖 Example 7: Boolean Logic Testing")
    
    test_suite_start("Boolean Logic")
    
    test_start("basic boolean operations")
    sus a lit = based
    sus b lit = cap
    
    assert_true(a)
    assert_false(b)
    assert_true(a == based)
    assert_true(b == cap)
    test_end()
    
    test_start("complex boolean expressions")
    sus x normie = 5
    sus y normie = 10
    
    assert_true(x < y)
    assert_false(x > y)
    assert_true((x < y) && (y > 0))
    assert_false((x > y) || (y < 0))
    test_end()
    
    test_suite_end("Boolean Logic")
}

fr fr Example 8: Comprehensive Module Testing Pattern
slay example_module_testing_pattern() {
    vibez.spill("📖 Example 8: Module Testing Pattern") fr fr This demonstrates how to test a stdlib module
    test_suite_start("Module Testing Pattern")
    
    test_start("module initialization") fr fr Test that module components are available
    assert_true(based) fr fr Placeholder for module check
    test_end()
    
    test_start("core functionality") fr fr Test main module functions
    sus result normie = 42 fr fr Placeholder for module function call
    assert_eq_int(result, 42)
    assert_gt(result, 0)
    test_end()
    
    test_start("edge case handling") fr fr Test module edge cases
    assert_eq_int(0, 0) fr fr Placeholder for edge case test
    test_end()
    
    test_start("error conditions") fr fr Test module error handling
    assert_true(based) fr fr Placeholder for error condition test
    test_end()
    
    test_suite_end("Module Testing Pattern")
}

fr fr Main execution function
slay run_all_examples() {
    vibez.spill("🎯 TESTZ FRAMEWORK EXAMPLES")
    vibez.spill("Demonstrating comprehensive testing capabilities")
    vibez.spill("=" * 60)
    vibez.spill("") fr fr Reset state for clean example run
    reset_test_state() fr fr Run all examples
    example_basic_testing()
    vibez.spill("")
    
    example_complex_data()
    vibez.spill("")
    
    example_edge_cases()
    vibez.spill("")
    
    example_performance_testing()
    vibez.spill("")
    
    example_error_handling()
    vibez.spill("")
    
    example_state_management()
    vibez.spill("")
    
    example_boolean_logic()
    vibez.spill("")
    
    example_module_testing_pattern()
    vibez.spill("") fr fr Generate final comprehensive report
    vibez.spill("🎉 ALL EXAMPLES COMPLETED")
    print_detailed_report() fr fr Example summary
    vibez.spill("📋 Example Features Demonstrated:")
    vibez.spill("✅ Basic assertion patterns")
    vibez.spill("✅ Complex data validation")
    vibez.spill("✅ Edge case testing")
    vibez.spill("✅ Performance testing")
    vibez.spill("✅ Error handling and recovery")
    vibez.spill("✅ State management")
    vibez.spill("✅ Boolean logic testing")
    vibez.spill("✅ Module testing patterns")
    vibez.spill("")
    vibez.spill("🚀 Framework ready for stdlib development!")
}

fr fr Execute all examples
run_all_examples()
