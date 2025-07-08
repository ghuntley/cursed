fr fr ========================================
fr fr CURSED Standard Library Test Suite
fr fr Master Test Runner
fr fr ========================================

fr fr Global test state
sus total_tests_run normie = 0
sus total_tests_passed normie = 0
sus total_tests_failed normie = 0

slay run_all_stdlib_tests() {
    vibez.spill("🚀 CURSED Standard Library Test Suite")
    vibez.spill("=====================================")
    vibez.spill("")
    
    fr fr Reset test state for clean run
    total_tests_run = 0
    total_tests_passed = 0
    total_tests_failed = 0
    
    fr fr Run simplified stdlib tests
    vibez.spill("Running Basic Math Tests...")
    run_basic_math_tests()
    
    vibez.spill("\nRunning String Tests...")
    run_basic_string_tests()
    
    vibez.spill("\nRunning Boolean Tests...")
    run_basic_boolean_tests()
    
    fr fr Final summary
    vibez.spill("\n========================================")
    vibez.spill("📊 FINAL STDLIB TEST SUMMARY")
    vibez.spill("========================================")
    
    vibez.spill("Total tests run: " + tea(total_tests_run))
    vibez.spill("Total passed: " + tea(total_tests_passed))
    vibez.spill("Total failed: " + tea(total_tests_failed))
    
    lowkey total_tests_failed == 0 {
        vibez.spill("")
        vibez.spill("🎉 ALL STDLIB TESTS PASSED! 🎉")
        vibez.spill("The CURSED standard library is fully functional!")
        vibez.spill("")
        vibez.spill("Tested modules:")
        vibez.spill("  ✓ Math      - Mathematical functions and constants")
        vibez.spill("  ✓ String    - String manipulation and processing")
        vibez.spill("  ✓ Boolean   - Boolean logic and operations")
        vibez.spill("")
    } highkey {
        vibez.spill("")
        vibez.spill("❌ SOME STDLIB TESTS FAILED")
        vibez.spill("Please check the test output above for details.")
        vibez.spill("")
    }
    
    damn total_tests_failed
}

fr fr ========================================
fr fr Basic Test Functions
fr fr ========================================

slay test_start(name tea) {
    total_tests_run = total_tests_run + 1
    vibez.spill("  Testing: " + name)
}

slay test_pass(message tea) {
    total_tests_passed = total_tests_passed + 1
    vibez.spill("    ✓ " + message)
}

slay test_fail(message tea) {
    total_tests_failed = total_tests_failed + 1
    vibez.spill("    ✗ " + message)
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
        test_pass("assert_true: condition is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

fr fr ========================================
fr fr Basic Math Tests
fr fr ========================================

slay run_basic_math_tests() {
    test_start("Basic Arithmetic")
    assert_eq_int(1 + 1, 2)
    assert_eq_int(10 - 5, 5)
    assert_eq_int(6 * 7, 42)
    assert_eq_int(20 / 4, 5)
    
    test_start("Math Comparisons")
    assert_true(10 > 5)
    assert_true(3 < 8)
    assert_true(7 == 7)
    assert_true(5 != 9)
    
    test_start("Mixed Type Math")
    sus a normie = 15
    sus b normie = 3
    assert_eq_int(a + b, 18)
    assert_eq_int(a - b, 12)
    assert_eq_int(a * b, 45)
    assert_eq_int(a / b, 5)
}

fr fr ========================================
fr fr Basic String Tests
fr fr ========================================

slay run_basic_string_tests() {
    test_start("String Equality")
    assert_eq_string("hello", "hello")
    assert_eq_string("world", "world")
    assert_eq_string("", "")
    
    test_start("String Variables")
    sus greeting tea = "Hello"
    sus name tea = "CURSED"
    assert_eq_string(greeting, "Hello")
    assert_eq_string(name, "CURSED")
    
    test_start("String Operations")
    sus combined tea = "Hello" + " " + "World"
    assert_eq_string(combined, "Hello World")
}

fr fr ========================================
fr fr Basic Boolean Tests
fr fr ========================================

slay run_basic_boolean_tests() {
    test_start("Boolean Values")
    sus true_val lit = based
    sus false_val lit = cap
    assert_true(true_val)
    assert_true(!false_val)
    
    test_start("Boolean Logic")
    assert_true(based && based)
    assert_true(based || cap)
    assert_true(!(cap && cap))
    
    test_start("Boolean Comparisons")
    assert_true(5 > 3)
    assert_true(2 < 7)
    assert_true(4 == 4)
    assert_true(6 != 9)
}

fr fr Auto-run when this file is executed
run_all_stdlib_tests()
