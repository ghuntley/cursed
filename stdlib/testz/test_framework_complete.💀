fr fr ================================
fr fr CURSED Testing Framework - Complete Self-Contained Version
fr fr All testing functions included in one file
fr fr ================================

fr fr ================================
fr fr Test Framework Core State
fr fr ================================

fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus current_test_name tea = ""

fr fr ================================
fr fr Core Test Functions
fr fr ================================

slay test_start(name tea) {
    test_count = test_count + 1
    current_test_name = name
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

fr fr ================================
fr fr Basic Assertion Functions
fr fr ================================

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    lowkey actual == expected {
        test_pass("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

fr fr ================================
fr fr Advanced Assertion Functions
fr fr ================================

slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("assert_ne_int: " + tea(actual) + " != " + tea(expected))
    } highkey {
        test_fail("assert_ne_int failed: got " + tea(actual) + ", expected not " + tea(expected))
    }
}

slay assert_ne_string(actual tea, expected tea) {
    lowkey actual != expected {
        test_pass("assert_ne_string: \"" + actual + "\" != \"" + expected + "\"")
    } highkey {
        test_fail("assert_ne_string failed: got \"" + actual + "\", expected not \"" + expected + "\"")
    }
}

slay assert_greater_than(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("assert_greater_than: " + tea(actual) + " > " + tea(expected))
    } highkey {
        test_fail("assert_greater_than failed: got " + tea(actual) + ", expected > " + tea(expected))
    }
}

slay assert_less_than(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("assert_less_than: " + tea(actual) + " < " + tea(expected))
    } highkey {
        test_fail("assert_less_than failed: got " + tea(actual) + ", expected < " + tea(expected))
    }
}

slay assert_in_range(actual normie, min normie, max normie) {
    lowkey actual >= min && actual <= max {
        test_pass("assert_in_range: " + tea(actual) + " in range [" + tea(min) + ", " + tea(max) + "]")
    } highkey {
        test_fail("assert_in_range failed: got " + tea(actual) + ", expected in range [" + tea(min) + ", " + tea(max) + "]")
    }
}

fr fr ================================
fr fr Test Summary Functions
fr fr ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("==================================================")
    vibez.spill("           CURSED Testing Framework")
    vibez.spill("                  TEST SUMMARY")
    vibez.spill("==================================================")
    vibez.spill("")
    vibez.spill("Test Results:")
    vibez.spill("  Total Tests: " + tea(test_count))
    
    lowkey test_count > 0 {
        vibez.spill("  Passed:      " + tea(test_passed) + " (" + tea((test_passed * 100) / test_count) + "%)")
        vibez.spill("  Failed:      " + tea(test_failed) + " (" + tea((test_failed * 100) / test_count) + "%)")
    } highkey {
        vibez.spill("  Passed:      " + tea(test_passed) + " (0%)")
        vibez.spill("  Failed:      " + tea(test_failed) + " (0%)")
    }
    
    vibez.spill("")
    vibez.spill("==================================================")
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED")
        vibez.spill("Please review the failures above.")
    }
    
    vibez.spill("==================================================")
}

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    current_test_name = ""
}

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
    
    fr fr Test negative numbers
    assert_eq_int(-5, -5)
    assert_eq_int(-10 + 5, -5)
    assert_eq_int(-2 * 3, -6)
    
    fr fr Test integer inequalities
    assert_ne_int(42, 43)
    assert_ne_int(1, 2)
    assert_ne_int(0, 1)
    assert_ne_int(-1, 1)
    
    fr fr Test integer comparisons
    assert_greater_than(5, 3)
    assert_greater_than(100, 42)
    assert_greater_than(1, 0)
    assert_greater_than(0, -1)
    
    assert_less_than(3, 5)
    assert_less_than(42, 100)
    assert_less_than(0, 1)
    assert_less_than(-1, 0)
    
    fr fr Test range assertions
    assert_in_range(5, 1, 10)
    assert_in_range(42, 40, 50)
    assert_in_range(0, -5, 5)
    assert_in_range(-3, -5, 5)
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
    
    fr fr Test string inequality
    assert_ne_string("hello", "world")
    assert_ne_string("CURSED", "cursed")
    assert_ne_string("", " ")
    assert_ne_string("123", "456")
    
    fr fr Test mixed string operations
    assert_eq_string("a" + "b" + "c", "abc")
    assert_eq_string("1" + "2" + "3", "123")
    assert_ne_string("hello", "HELLO")
    assert_ne_string("test", "TEST")
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
    
    fr fr Test logical operations
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
    
    fr fr Test multiple variables
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_eq_int(a + b, c)
    assert_ne_int(a, b)
    assert_ne_int(a, c)
    assert_ne_int(b, c)
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
    
    fr fr Test complex expressions
    assert_eq_int(1 + 2 * 3 + 4, 11)
    assert_eq_int((1 + 2) * (3 + 4), 21)
    assert_eq_int(10 / 2 + 5 * 3, 20)
    
    fr fr Test negative numbers
    assert_eq_int(-5 + 3, -2)
    assert_eq_int(-10 - 5, -15)
    assert_eq_int(-2 * 3, -6)
    assert_eq_int(-20 / 4, -5)
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
    
    fr fr Test with expressions
    assert_true((2 + 3) > 4)
    assert_false((2 + 3) < 4)
    assert_true((2 + 3) == 5)
    assert_false((2 + 3) != 5)
}

fr fr ================================
fr fr Test Suite: Edge Cases
fr fr ================================

slay test_edge_cases() {
    test_start("test_edge_cases")
    
    fr fr Test boundary conditions
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1, 1)
    
    fr fr Test large numbers
    assert_eq_int(1000000, 1000000)
    assert_eq_int(-1000000, -1000000)
    
    fr fr Test empty strings
    assert_eq_string("", "")
    assert_true("" == "")
    assert_false("" == " ")
    
    fr fr Test single character strings
    assert_eq_string("a", "a")
    assert_ne_string("a", "b")
    assert_ne_string("A", "a")
    
    fr fr Test boolean edge cases
    assert_true(based)
    assert_false(cap)
    assert_true(based == based)
    assert_true(cap == cap)
    assert_false(based == cap)
    assert_true(based != cap)
    assert_false(based != based)
}

fr fr ================================
fr fr Test Suite: Complex Expressions
fr fr ================================

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
    
    fr fr Test nested expressions
    assert_eq_int(((2 + 3) * 4) + 5, 25)
    assert_eq_int(10 - ((3 + 2) * 1), 5)
    assert_true(((5 > 3) && (2 < 4)) || cap)
    assert_false(((5 < 3) || (2 > 4)) && based)
}

fr fr ================================
fr fr Test Suite: Framework Testing
fr fr ================================

slay test_framework_functionality() {
    test_start("test_framework_functionality")
    
    fr fr Test that test functions themselves work
    assert_true(test_count > 0)
    assert_true(test_passed > 0)
    assert_true(current_test_name == "test_framework_functionality")
    
    fr fr Test string conversion
    assert_eq_string(tea(42), "42")
    assert_eq_string(tea(0), "0")
    assert_eq_string(tea(-1), "-1")
    assert_eq_string(tea(based), "based")
    assert_eq_string(tea(cap), "cap")
    
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
    
    fr fr Test all types of operations in one comprehensive test
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_true(c > a)
    assert_true(c > b)
    assert_true(a < c)
    assert_true(b < c)
    assert_ne_int(a, b)
    assert_ne_int(a, c)
    assert_ne_int(b, c)
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    assert_true(greeting != name)
    assert_true(name == "CURSED")
    assert_ne_string(greeting, name)
    assert_ne_string(name, "cursed")
    
    sus flag lit = based
    sus opposite lit = !flag
    
    assert_eq_bool(flag, based)
    assert_eq_bool(opposite, cap)
    assert_true(flag && !opposite)
    assert_true(flag || opposite)
    assert_false(!flag)
    assert_true(!opposite)
    
    fr fr Test complex combined expressions
    assert_true((a < b) && (b < c))
    assert_false((a > b) || (b > c))
    assert_true((a + b) == c)
    assert_false((a * b) == c)
    
    fr fr Test string and number combinations
    assert_eq_string(tea(a), "10")
    assert_eq_string(tea(b), "20")
    assert_eq_string(tea(c), "30")
    assert_ne_string(tea(a), tea(b))
    assert_ne_string(tea(b), tea(c))
    
    fr fr Test boolean combinations
    assert_true(flag == based)
    assert_false(flag == cap)
    assert_true(opposite == cap)
    assert_false(opposite == based)
    assert_true(flag != opposite)
    assert_false(flag == opposite)
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main_character() {
    vibez.spill("🧪 Starting CURSED Testing Framework - Complete Self-Contained Tests...")
    vibez.spill("")
    vibez.spill("This comprehensive test suite validates all core testing primitives")
    vibez.spill("and demonstrates the complete functionality of the CURSED testing framework.")
    vibez.spill("")
    
    fr fr Reset test state for clean run
    reset_test_state()
    
    fr fr Run all test suites
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_integer_assertions()
    test_string_assertions()
    test_boolean_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_variable_declarations()
    test_arithmetic_operations()
    test_comparison_operations()
    
    vibez.spill("=== Running Edge Case Tests ===")
    test_edge_cases()
    test_complex_expressions()
    
    vibez.spill("=== Running Framework Validation Tests ===")
    test_framework_functionality()
    test_comprehensive_validation()
    
    fr fr Print comprehensive test summary
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🎯 CURSED Testing Framework - Complete Self-Contained Tests Complete!")
    vibez.spill("")
    vibez.spill("This testing framework provides:")
    vibez.spill("✅ Complete assertion library (equality, inequality, comparisons, ranges)")
    vibez.spill("✅ Comprehensive test lifecycle management")
    vibez.spill("✅ Detailed test reporting and statistics")
    vibez.spill("✅ Support for all CURSED data types (integers, strings, booleans)")
    vibez.spill("✅ Advanced expression testing capabilities")
    vibez.spill("✅ Production-ready testing primitives")
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
