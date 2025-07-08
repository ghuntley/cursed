fr fr ========================================
fr fr CURSED Math Library Simple Test
fr fr ========================================

fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
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

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

slay test_basic_math() {
    test_start("Basic Math Operations")
    
    fr fr Test basic arithmetic
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 - 3, 2)
    assert_eq_int(4 * 3, 12)
    assert_eq_int(10 / 2, 5)
    
    fr fr Test comparison operations
    assert_true(5 > 3)
    assert_true(3 < 5)
    assert_true(5 == 5)
    assert_true(5 != 3)
}

slay test_math_types() {
    test_start("Math Type Operations")
    
    fr fr Test integer math
    sus a normie = 10
    sus b normie = 3
    assert_eq_int(a + b, 13)
    assert_eq_int(a - b, 7)
    assert_eq_int(a * b, 30)
    
    fr fr Test mixed arithmetic (if supported)
    sus x normie = 5
    sus y meal = 2.0
    assert_true((x + y) > 6.0)
}

slay test_boolean_logic() {
    test_start("Boolean Logic Operations")
    
    fr fr Test boolean values
    sus true_val lit = based
    sus false_val lit = cap
    
    assert_true(true_val)
    assert_true(!false_val)
    assert_true(true_val && true_val)
    assert_true(true_val || false_val)
    assert_true(!(false_val && false_val))
}

slay run_simple_math_tests() {
    vibez.spill("🧮 Running Simple CURSED Math Tests")
    vibez.spill("==================================")
    
    test_basic_math()
    test_math_types()
    test_boolean_logic()
    
    print_test_summary()
    
    lowkey test_failed > 0 {
        damn 1
    } highkey {
        damn 0
    }
}

fr fr Auto-run tests when this file is executed
run_simple_math_tests()
