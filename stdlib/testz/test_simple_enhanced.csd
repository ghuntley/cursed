fr fr ================================
fr fr CURSED Testing Framework - Simple Enhanced Tests
fr fr Comprehensive test suite using the current testz module
fr fr ================================

yeet "testz"

fr fr ================================
fr fr Test Suite: Basic Assertions
fr fr ================================

slay test_integer_assertions() {
    test_start("test_integer_assertions")
    
    fr fr Test basic integer equality
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_eq_int(10 / 2, 5)
    
    fr fr Test more complex calculations
    assert_eq_int(3 + 4 * 5, 23)
    assert_eq_int((3 + 4) * 5, 35)
    assert_eq_int(100 - 50, 50)
    assert_eq_int(2 * 3 * 4, 24)
}

slay test_string_assertions() {
    test_start("test_string_assertions")
    
    fr fr Test basic string equality
    assert_eq_string("hello", "hello")
    assert_eq_string("CURSED", "CURSED")
    assert_eq_string("", "")
    assert_eq_string("123", "123")
    assert_eq_string("hello world", "hello world")
    
    fr fr Test string concatenation
    assert_eq_string("hello" + " world", "hello world")
    assert_eq_string("test" + "ing", "testing")
    assert_eq_string("" + "hello", "hello")
    assert_eq_string("hello" + "", "hello")
}

slay test_boolean_assertions() {
    test_start("test_boolean_assertions")
    
    fr fr Test basic boolean values
    assert_true(based)
    assert_false(cap)
    
    fr fr Test boolean expressions
    assert_true(5 > 3)
    assert_false(3 > 5)
    assert_true(2 == 2)
    assert_false(2 == 3)
    assert_true(1 != 2)
    assert_false(1 != 1)
    
    fr fr Test boolean equality
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_eq_bool(5 > 3, based)
    assert_eq_bool(3 > 5, cap)
    assert_eq_bool(2 == 2, based)
    assert_eq_bool(2 == 3, cap)
}

fr fr ================================
fr fr Test Suite: Advanced Features
fr fr ================================

slay test_variable_declarations() {
    test_start("test_variable_declarations")
    
    fr fr Test variable declarations and usage
    sus test_int normie = 42
    sus test_string tea = "hello"
    sus test_bool lit = based
    
    assert_eq_int(test_int, 42)
    assert_eq_string(test_string, "hello")
    assert_eq_bool(test_bool, based)
    
    fr fr Test variable reassignment
    test_int = 100
    test_string = "world"
    test_bool = cap
    
    assert_eq_int(test_int, 100)
    assert_eq_string(test_string, "world")
    assert_eq_bool(test_bool, cap)
}

slay test_arithmetic_operations() {
    test_start("test_arithmetic_operations")
    
    fr fr Test basic arithmetic
    assert_eq_int(5 + 3, 8)
    assert_eq_int(10 - 4, 6)
    assert_eq_int(6 * 7, 42)
    assert_eq_int(20 / 4, 5)
    
    fr fr Test operator precedence
    assert_eq_int(2 + 3 * 4, 14)
    assert_eq_int((2 + 3) * 4, 20)
    assert_eq_int(10 - 2 * 3, 4)
    assert_eq_int((10 - 2) * 3, 24)
}

slay test_comparison_operations() {
    test_start("test_comparison_operations")
    
    fr fr Test comparison operators
    assert_true(5 > 3)
    assert_false(3 > 5)
    assert_true(5 >= 5)
    assert_true(5 >= 3)
    assert_false(3 >= 5)
    
    assert_true(3 < 5)
    assert_false(5 < 3)
    assert_true(5 <= 5)
    assert_true(3 <= 5)
    assert_false(5 <= 3)
    
    assert_true(5 == 5)
    assert_false(5 == 3)
    assert_true(5 != 3)
    assert_false(5 != 5)
}

slay test_logical_operations() {
    test_start("test_logical_operations")
    
    fr fr Test logical operators
    assert_true(based && based)
    assert_false(based && cap)
    assert_false(cap && based)
    assert_false(cap && cap)
    
    assert_true(based || based)
    assert_true(based || cap)
    assert_true(cap || based)
    assert_false(cap || cap)
    
    assert_false(!based)
    assert_true(!cap)
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
    assert_true("" == "")
    assert_false("" == " ")
    
    fr fr Test boolean edge cases
    assert_true(based)
    assert_false(cap)
    assert_true(based == based)
    assert_true(cap == cap)
    assert_false(based == cap)
}

slay test_complex_expressions() {
    test_start("test_complex_expressions")
    
    fr fr Test complex arithmetic expressions
    assert_eq_int(1 + 2 * 3 + 4, 11)
    assert_eq_int((1 + 2) * (3 + 4), 21)
    assert_eq_int(10 / 2 + 5 * 3, 20)
    assert_eq_int((10 + 5) * 2 / 3, 10)
    
    fr fr Test complex boolean expressions
    assert_true((5 > 3) && (2 < 4))
    assert_false((5 > 3) && (2 > 4))
    assert_true((5 > 3) || (2 > 4))
    assert_false((5 < 3) || (2 > 4))
    
    fr fr Test mixed type expressions
    assert_true((5 + 3) == 8)
    assert_false((5 + 3) == 9)
    assert_true((10 - 2) > 5)
    assert_false((10 - 2) < 5)
}

fr fr ================================
fr fr Test Suite: Function Testing
fr fr ================================

slay test_function_calls() {
    test_start("test_function_calls")
    
    fr fr Test that test functions themselves work
    assert_eq_int(test_count, 8)  fr fr Should have run 8 tests so far
    assert_true(test_passed > 0)
    assert_true(current_test_name == "test_function_calls")
    
    fr fr Test string conversion
    assert_eq_string(tea(42), "42")
    assert_eq_string(tea(0), "0")
    assert_eq_string(tea(-1), "-1")
    assert_eq_string(tea(based), "based")
    assert_eq_string(tea(cap), "cap")
}

fr fr ================================
fr fr Test Suite: State Management
fr fr ================================

slay test_state_management() {
    test_start("test_state_management")
    
    fr fr Test state tracking
    sus initial_count normie = test_count
    sus initial_passed normie = test_passed
    
    fr fr These should increment counters
    assert_eq_int(42, 42)  fr fr Should increment passed
    assert_eq_string("test", "test")  fr fr Should increment passed
    
    fr fr Verify state changes
    assert_true(test_passed > initial_passed)
    assert_true(test_count > initial_count)
}

fr fr ================================
fr fr Test Suite: Comprehensive Validation
fr fr ================================

slay test_comprehensive_validation() {
    test_start("test_comprehensive_validation")
    
    fr fr Test all types of operations in one test
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_true(c > a)
    assert_true(c > b)
    assert_true(a < c)
    assert_true(b < c)
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    assert_true(greeting != name)
    assert_true(name == "CURSED")
    
    sus flag lit = based
    sus opposite lit = !flag
    
    assert_eq_bool(flag, based)
    assert_eq_bool(opposite, cap)
    assert_true(flag && !opposite)
    assert_true(flag || opposite)
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 Starting CURSED Testing Framework - Simple Enhanced Tests...")
    vibez.spill("")
    
    fr fr Run all test suites
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_integer_assertions()
    test_string_assertions()
    test_boolean_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_variable_declarations()
    test_arithmetic_operations()
    test_comparison_operations()
    test_logical_operations()
    
    vibez.spill("=== Running Edge Case Tests ===")
    test_edge_cases()
    test_complex_expressions()
    
    vibez.spill("=== Running Function and State Tests ===")
    test_function_calls()
    test_state_management()
    
    vibez.spill("=== Running Comprehensive Validation Tests ===")
    test_comprehensive_validation()
    
    fr fr Print comprehensive test summary
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🎯 CURSED Testing Framework - Simple Enhanced Tests Complete!")
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
