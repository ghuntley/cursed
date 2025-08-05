fr fr Working Testing Framework
fr fr A working version of the CURSED testing framework

fr fr Global test counters
sus test_count normie = 0;
sus test_passed normie = 0;
sus test_failed normie = 0;

fr fr ================================
fr fr Core Test Functions
fr fr ================================

slay test_start(name tea) {
    test_count = test_count + 1;
    vibez.spill("Running test: " + name);
}

slay test_pass(message tea) {
    test_passed = test_passed + 1;
    vibez.spill("  ✓ PASS: " + message);
}

slay test_fail(message tea) {
    test_failed = test_failed + 1;
    vibez.spill("  ✗ FAIL: " + message);
}

fr fr ================================
fr fr Assertion Functions
fr fr ================================

slay assert_eq(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_eq failed: got " + actual + ", expected " + expected);
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string(\"" + actual + "\", \"" + expected + "\")");
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"");
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true(" + value + ")");
    } highkey {
        test_fail("assert_true failed: got " + value + ", expected based");
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false(" + value + ")");
    } highkey {
        test_fail("assert_false failed: got " + value + ", expected cap");
    }
}

slay assert_ne(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("assert_ne(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_ne failed: got " + actual + ", expected not " + expected);
    }
}

slay assert_greater_than(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("assert_greater_than(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_greater_than failed: " + actual + " <= " + expected);
    }
}

slay assert_less_than(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("assert_less_than(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_less_than failed: " + actual + " >= " + expected);
    }
}

fr fr ================================
fr fr Test Summary
fr fr ================================

slay print_test_summary() {
    vibez.spill("");
    vibez.spill("=== TEST SUMMARY ===");
    vibez.spill("Total tests: " + test_count);
    vibez.spill("Passed: " + test_passed);
    vibez.spill("Failed: " + test_failed);
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉");
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED");
    }
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay reset_test_state() {
    test_count = 0;
    test_passed = 0;
    test_failed = 0;
}

slay get_success_rate() normie {
    lowkey test_count == 0 {
        damn 0;
    } highkey {
        damn (test_passed * 100) / test_count;
    }
}

fr fr ================================
fr fr Example Test Cases
fr fr ================================

slay test_basic_assertions() {
    test_start("test_basic_assertions");
    
    fr fr Test integer equality
    assert_eq(42, 42);
    assert_eq(0, 0);
    assert_eq(-5, -5);
    
    fr fr Test string equality
    assert_eq_string("hello", "hello");
    assert_eq_string("", "");
    
    fr fr Test boolean values
    assert_true(based);
    assert_false(cap);
    assert_true(5 > 3);
    assert_false(3 > 5);
    
    fr fr Test inequality
    assert_ne(42, 24);
    assert_ne(0, 1);
    
    fr fr Test comparisons
    assert_greater_than(10, 5);
    assert_less_than(5, 10);
}

slay test_arithmetic_operations() {
    test_start("test_arithmetic_operations");
    
    sus a normie = 10;
    sus b normie = 5;
    
    fr fr Basic arithmetic
    assert_eq(a + b, 15);
    assert_eq(a - b, 5);
    assert_eq(a * b, 50);
    assert_eq(a / b, 2);
    
    fr fr Test with different values
    assert_eq(3 + 7, 10);
    assert_eq(15 - 8, 7);
    assert_eq(4 * 6, 24);
    assert_eq(20 / 4, 5);
}

slay test_string_operations() {
    test_start("test_string_operations");
    
    sus hello tea = "Hello";
    sus world tea = "World";
    sus space tea = " ";
    
    fr fr String concatenation
    sus greeting tea = hello + space + world;
    assert_eq_string(greeting, "Hello World");
    
    fr fr Individual strings
    assert_eq_string(hello, "Hello");
    assert_eq_string(world, "World");
    assert_eq_string(space, " ");
    
    fr fr Empty string
    sus empty tea = "";
    assert_eq_string(empty, "");
}

slay test_conditional_logic() {
    test_start("test_conditional_logic");
    
    sus x normie = 10;
    sus y normie = 5;
    
    fr fr Comparison operators
    assert_true(x > y);
    assert_false(x < y);
    assert_true(x >= y);
    assert_false(x <= y);
    assert_false(x == y);
    assert_true(x != y);
    
    fr fr Boolean logic
    assert_true(based && based);
    assert_false(based && cap);
    assert_true(based || cap);
    assert_false(cap || cap);
}

slay test_variable_types() {
    test_start("test_variable_types");
    
    fr fr Integer variables
    sus int_var normie = 123;
    assert_eq(int_var, 123);
    
    fr fr String variables
    sus str_var tea = "test string";
    assert_eq_string(str_var, "test string");
    
    fr fr Boolean variables
    sus bool_var lit = based;
    assert_true(bool_var);
    
    sus false_var lit = cap;
    assert_false(false_var);
}

slay test_function_calls() {
    test_start("test_function_calls");
    
    fr fr Test that functions can be called and return values
    sus result normie = add_two_numbers(5, 3);
    assert_eq(result, 8);
    
    sus result2 normie = multiply_by_two(7);
    assert_eq(result2, 14);
    
    sus greeting tea = create_greeting("CURSED");
    assert_eq_string(greeting, "Hello, CURSED!");
}

fr fr Helper functions for testing
slay add_two_numbers(a normie, b normie) normie {
    damn a + b;
}

slay multiply_by_two(n normie) normie {
    damn n * 2;
}

slay create_greeting(name tea) tea {
    damn "Hello, " + name + "!";
}

slay test_failing_example() {
    test_start("test_failing_example");
    
    fr fr This test is designed to fail
    assert_eq(2 + 2, 5);
    assert_eq_string("hello", "goodbye");
    assert_true(cap);
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("CURSED Testing Framework - Working Demo");
    vibez.spill("=======================================");
    
    fr fr Reset state
    reset_test_state();
    
    fr fr Run all tests
    test_basic_assertions();
    test_arithmetic_operations();
    test_string_operations();
    test_conditional_logic();
    test_variable_types();
    test_function_calls();
    
    fr fr Run a failing test to show error handling
    vibez.spill("");
    vibez.spill("=== Testing Failure Reporting ===");
    test_failing_example();
    
    fr fr Print final summary
    print_test_summary();
    
    fr fr Show success rate
    sus success_rate normie = get_success_rate();
    vibez.spill("Success rate: " + success_rate + "%");
    
    fr fr Return appropriate exit code
    lowkey test_failed > 0 {
        vibez.spill("Exiting with error code 1");
        damn 1;
    } highkey {
        vibez.spill("Exiting with success code 0");
        damn 0;
    }
}
