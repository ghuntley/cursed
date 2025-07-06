fr fr CURSED Testing Framework - Compilation Compatible
fr fr A version that works with both interpretation and compilation modes

fr fr ================================
fr fr Test Result Structure
fr fr ================================

struct TestStats {
    test_count normie;
    test_passed normie;
    test_failed normie;
}

fr fr ================================
fr fr Core Test Functions
fr fr ================================

slay create_test_stats() TestStats {
    sus stats TestStats = TestStats{
        test_count: 0,
        test_passed: 0,
        test_failed: 0
    };
    yolo stats;
}

slay test_start(stats TestStats, name tea) TestStats {
    stats.test_count = stats.test_count + 1;
    vibez.spill("Running test: " + name);
    yolo stats;
}

slay test_pass(stats TestStats, message tea) TestStats {
    stats.test_passed = stats.test_passed + 1;
    vibez.spill("  ✓ PASS: " + message);
    yolo stats;
}

slay test_fail(stats TestStats, message tea) TestStats {
    stats.test_failed = stats.test_failed + 1;
    vibez.spill("  ✗ FAIL: " + message);
    yolo stats;
}

fr fr ================================
fr fr Assertion Functions
fr fr ================================

slay assert_eq(stats TestStats, actual normie, expected normie) TestStats {
    lowkey actual == expected {
        stats = test_pass(stats, "assert_eq(" + actual + ", " + expected + ")");
    } highkey {
        stats = test_fail(stats, "assert_eq failed: got " + actual + ", expected " + expected);
    }
    yolo stats;
}

slay assert_eq_string(stats TestStats, actual tea, expected tea) TestStats {
    lowkey actual == expected {
        stats = test_pass(stats, "assert_eq_string(\"" + actual + "\", \"" + expected + "\")");
    } highkey {
        stats = test_fail(stats, "assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"");
    }
    yolo stats;
}

slay assert_true(stats TestStats, value lit) TestStats {
    lowkey value == based {
        stats = test_pass(stats, "assert_true(" + value + ")");
    } highkey {
        stats = test_fail(stats, "assert_true failed: got " + value + ", expected based");
    }
    yolo stats;
}

slay assert_false(stats TestStats, value lit) TestStats {
    lowkey value == cap {
        stats = test_pass(stats, "assert_false(" + value + ")");
    } highkey {
        stats = test_fail(stats, "assert_false failed: got " + value + ", expected cap");
    }
    yolo stats;
}

slay assert_ne(stats TestStats, actual normie, expected normie) TestStats {
    lowkey actual != expected {
        stats = test_pass(stats, "assert_ne(" + actual + ", " + expected + ")");
    } highkey {
        stats = test_fail(stats, "assert_ne failed: got " + actual + ", expected not " + expected);
    }
    yolo stats;
}

fr fr ================================
fr fr Test Summary
fr fr ================================

slay print_test_summary(stats TestStats) {
    vibez.spill("");
    vibez.spill("=== TEST SUMMARY ===");
    vibez.spill("Total tests: " + stats.test_count);
    vibez.spill("Passed: " + stats.test_passed);
    vibez.spill("Failed: " + stats.test_failed);
    
    lowkey stats.test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉");
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED");
    }
}

fr fr ================================
fr fr Test Cases
fr fr ================================

slay test_basic_math(stats TestStats) TestStats {
    stats = test_start(stats, "test_basic_math");
    
    sus result normie = 2 + 2;
    stats = assert_eq(stats, result, 4);
    
    sus product normie = 3 * 7;
    stats = assert_eq(stats, product, 21);
    
    sus difference normie = 10 - 3;
    stats = assert_eq(stats, difference, 7);
    
    yolo stats;
}

slay test_string_operations(stats TestStats) TestStats {
    stats = test_start(stats, "test_string_operations");
    
    sus greeting tea = "Hello";
    sus target tea = "World";
    sus full_greeting tea = greeting + " " + target;
    
    stats = assert_eq_string(stats, full_greeting, "Hello World");
    stats = assert_eq_string(stats, greeting, "Hello");
    stats = assert_eq_string(stats, target, "World");
    
    yolo stats;
}

slay test_boolean_logic(stats TestStats) TestStats {
    stats = test_start(stats, "test_boolean_logic");
    
    sus true_val lit = based;
    sus false_val lit = cap;
    sus condition lit = 5 > 3;
    sus false_condition lit = 5 < 3;
    
    stats = assert_true(stats, true_val);
    stats = assert_false(stats, false_val);
    stats = assert_true(stats, condition);
    stats = assert_false(stats, false_condition);
    
    yolo stats;
}

slay test_arithmetic_operations(stats TestStats) TestStats {
    stats = test_start(stats, "test_arithmetic_operations");
    
    sus a normie = 10;
    sus b normie = 5;
    
    stats = assert_eq(stats, a + b, 15);
    stats = assert_eq(stats, a - b, 5);
    stats = assert_eq(stats, a * b, 50);
    stats = assert_eq(stats, a / b, 2);
    
    yolo stats;
}

slay test_function_calls(stats TestStats) TestStats {
    stats = test_start(stats, "test_function_calls");
    
    sus result1 normie = add_numbers(5, 3);
    stats = assert_eq(stats, result1, 8);
    
    sus result2 normie = double_number(7);
    stats = assert_eq(stats, result2, 14);
    
    sus greeting tea = make_greeting("CURSED");
    stats = assert_eq_string(stats, greeting, "Hello, CURSED!");
    
    yolo stats;
}

slay test_failing_example(stats TestStats) TestStats {
    stats = test_start(stats, "test_failing_example");
    
    fr fr This test is designed to fail
    stats = assert_eq(stats, 2 + 2, 5);
    stats = assert_eq_string(stats, "hello", "goodbye");
    stats = assert_true(stats, cap);
    
    yolo stats;
}

fr fr ================================
fr fr Helper Functions
fr fr ================================

slay add_numbers(a normie, b normie) normie {
    yolo a + b;
}

slay double_number(n normie) normie {
    yolo n * 2;
}

slay make_greeting(name tea) tea {
    yolo "Hello, " + name + "!";
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("CURSED Testing Framework - Compilation Compatible");
    vibez.spill("=================================================");
    
    fr fr Create test statistics
    sus stats TestStats = create_test_stats();
    
    fr fr Run all tests
    stats = test_basic_math(stats);
    stats = test_string_operations(stats);
    stats = test_boolean_logic(stats);
    stats = test_arithmetic_operations(stats);
    stats = test_function_calls(stats);
    
    fr fr Run a failing test to show error handling
    vibez.spill("");
    vibez.spill("=== Testing Failure Reporting ===");
    stats = test_failing_example(stats);
    
    fr fr Print final summary
    print_test_summary(stats);
    
    fr fr Calculate success rate
    sus success_rate normie = 0;
    lowkey stats.test_count > 0 {
        success_rate = (stats.test_passed * 100) / stats.test_count;
    }
    vibez.spill("Success rate: " + success_rate + "%");
    
    fr fr Return appropriate exit code
    lowkey stats.test_failed > 0 {
        vibez.spill("Exiting with error code 1");
        yolo 1;
    } highkey {
        vibez.spill("Exiting with success code 0");
        yolo 0;
    }
}
