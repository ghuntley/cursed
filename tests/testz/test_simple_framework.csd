fr fr Simple Testing Framework Test
fr fr A minimal test to verify the basic testing approach works

fr fr Global test counters
sus test_count normie = 0;
sus test_passed normie = 0;
sus test_failed normie = 0;

fr fr ================================
fr fr Basic Test Functions
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
fr fr Simple Assertions
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
fr fr Test Cases
fr fr ================================

slay test_basic_math() {
    test_start("test_basic_math");
    
    sus result normie = 2 + 2;
    assert_eq(result, 4);
    
    sus product normie = 3 * 7;
    assert_eq(product, 21);
    
    sus difference normie = 10 - 3;
    assert_eq(difference, 7);
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
    
    sus true_val lit = based;
    sus false_val lit = cap;
    sus condition lit = 5 > 3;
    sus false_condition lit = 5 < 3;
    
    assert_true(true_val);
    assert_false(false_val);
    assert_true(condition);
    assert_false(false_condition);
}

slay test_variable_declarations() {
    test_start("test_variable_declarations");
    
    sus number normie = 42;
    sus text tea = "test";
    sus flag lit = based;
    
    assert_eq(number, 42);
    assert_eq_string(text, "test");
    assert_true(flag);
}

slay test_arithmetic_operations() {
    test_start("test_arithmetic_operations");
    
    sus a normie = 10;
    sus b normie = 5;
    
    assert_eq(a + b, 15);
    assert_eq(a - b, 5);
    assert_eq(a * b, 50);
    assert_eq(a / b, 2);
    assert_eq(a % b, 0);
}

slay test_comparison_operations() {
    test_start("test_comparison_operations");
    
    sus x normie = 10;
    sus y normie = 5;
    
    assert_true(x > y);
    assert_false(x < y);
    assert_true(x >= y);
    assert_false(x <= y);
    assert_false(x == y);
    assert_true(x != y);
}

slay test_failing_example() {
    test_start("test_failing_example");
    
    fr fr This test is designed to fail to demonstrate failure reporting
    sus wrong_result normie = 2 + 2;
    assert_eq(wrong_result, 5);  fr fr This will fail
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("CURSED Simple Testing Framework");
    vibez.spill("===============================");
    
    fr fr Run all tests
    test_basic_math();
    test_string_operations();
    test_boolean_logic();
    test_variable_declarations();
    test_arithmetic_operations();
    test_comparison_operations();
    test_failing_example();
    
    fr fr Print final summary
    print_test_summary();
    
    fr fr Return appropriate exit code
    lowkey test_failed > 0 {
        yolo 1;
    } highkey {
        yolo 0;
    }
}
