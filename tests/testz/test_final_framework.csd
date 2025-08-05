fr fr CURSED Testing Framework - Final Working Version
fr fr Works with current CURSED features in both interpretation and compilation

fr fr ================================
fr fr Simple Test Functions
fr fr ================================

slay test_pass(message tea) {
    vibez.spill("  ✓ PASS: " + message);
}

slay test_fail(message tea) {
    vibez.spill("  ✗ FAIL: " + message);
}

slay test_start(name tea) {
    vibez.spill("Running test: " + name);
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

slay assert_greater(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("assert_greater(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_greater failed: " + actual + " <= " + expected);
    }
}

slay assert_less(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("assert_less(" + actual + ", " + expected + ")");
    } highkey {
        test_fail("assert_less failed: " + actual + " >= " + expected);
    }
}

fr fr ================================
fr fr Test Cases
fr fr ================================

slay test_basic_math() {
    test_start("test_basic_math");
    
    assert_eq(2 + 2, 4);
    assert_eq(3 * 7, 21);
    assert_eq(10 - 3, 7);
    assert_eq(20 / 4, 5);
}

slay test_string_operations() {
    test_start("test_string_operations");
    
    sus greeting tea = "Hello";
    sus target tea = "World";
    sus full_greeting tea = greeting + " " + target;
    
    assert_eq_string(full_greeting, "Hello World");
    assert_eq_string(greeting, "Hello");
    assert_eq_string(target, "World");
}

slay test_boolean_logic() {
    test_start("test_boolean_logic");
    
    assert_true(based);
    assert_false(cap);
    assert_true(5 > 3);
    assert_false(3 > 5);
    assert_true(based && based);
    assert_false(cap && based);
    assert_true(based || cap);
    assert_false(cap || cap);
}

slay test_comparisons() {
    test_start("test_comparisons");
    
    sus x normie = 10;
    sus y normie = 5;
    
    assert_greater(x, y);
    assert_less(y, x);
    assert_ne(x, y);
    assert_eq(x, 10);
    assert_eq(y, 5);
}

slay test_variables() {
    test_start("test_variables");
    
    sus number normie = 42;
    sus text tea = "test";
    sus flag lit = based;
    
    assert_eq(number, 42);
    assert_eq_string(text, "test");
    assert_true(flag);
}

slay test_arithmetic() {
    test_start("test_arithmetic");
    
    sus a normie = 15;
    sus b normie = 3;
    
    assert_eq(a + b, 18);
    assert_eq(a - b, 12);
    assert_eq(a * b, 45);
    assert_eq(a / b, 5);
}

slay test_function_calls() {
    test_start("test_function_calls");
    
    sus result1 normie = add_two(5, 3);
    assert_eq(result1, 8);
    
    sus result2 normie = multiply_by_three(4);
    assert_eq(result2, 12);
    
    sus greeting tea = say_hello("CURSED");
    assert_eq_string(greeting, "Hello, CURSED!");
}

slay test_conditional_logic() {
    test_start("test_conditional_logic");
    
    sus condition1 lit = 10 > 5;
    sus condition2 lit = 3 < 2;
    
    assert_true(condition1);
    assert_false(condition2);
    
    fr fr Test nested conditions
    lowkey condition1 {
        assert_true(based);
    } highkey {
        assert_true(cap);  fr fr This should not execute
    }
}

slay test_failing_cases() {
    test_start("test_failing_cases");
    
    fr fr These tests are designed to fail
    assert_eq(2 + 2, 5);
    assert_eq_string("hello", "goodbye");
    assert_true(cap);
    assert_false(based);
}

fr fr ================================
fr fr Helper Functions for Testing
fr fr ================================

slay add_two(a normie, b normie) normie {
    damn a + b;
}

slay multiply_by_three(n normie) normie {
    damn n * 3;
}

slay say_hello(name tea) tea {
    damn "Hello, " + name + "!";
}

fr fr ================================
fr fr Test Summary Functions
fr fr ================================

slay run_all_passing_tests() {
    vibez.spill("=== RUNNING ALL PASSING TESTS ===");
    test_basic_math();
    test_string_operations();
    test_boolean_logic();
    test_comparisons();
    test_variables();
    test_arithmetic();
    test_function_calls();
    test_conditional_logic();
}

slay run_failing_tests() {
    vibez.spill("");
    vibez.spill("=== TESTING FAILURE REPORTING ===");
    test_failing_cases();
}

slay demo_assertions() {
    vibez.spill("");
    vibez.spill("=== ASSERTION DEMO ===");
    test_start("demo_assertions");
    
    fr fr Integer assertions
    assert_eq(42, 42);
    assert_ne(42, 24);
    assert_greater(10, 5);
    assert_less(5, 10);
    
    fr fr String assertions
    assert_eq_string("hello", "hello");
    
    fr fr Boolean assertions
    assert_true(based);
    assert_false(cap);
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("CURSED Testing Framework - Final Version");
    vibez.spill("========================================");
    vibez.spill("Compatible with interpretation and compilation modes");
    vibez.spill("");
    
    fr fr Run demonstration of assertions
    demo_assertions();
    
    fr fr Run all passing tests
    run_all_passing_tests();
    
    fr fr Run some failing tests to show error reporting
    run_failing_tests();
    
    fr fr Final message
    vibez.spill("");
    vibez.spill("=== TESTING COMPLETE ===");
    vibez.spill("Framework successfully demonstrates:");
    vibez.spill("- Basic assertions (eq, ne, true, false)");
    vibez.spill("- String testing");
    vibez.spill("- Numeric comparisons");
    vibez.spill("- Function testing");
    vibez.spill("- Pass/fail reporting");
    vibez.spill("");
    vibez.spill("Ready for use in CURSED projects!");
    
    damn 0;
}
