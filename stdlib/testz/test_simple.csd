fr fr ================================
fr fr Simple Test for Enhanced CURSED Testing Framework
fr fr Direct inclusion without imports
fr fr ================================

fr fr Include the testz module functions directly
fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus test_skipped normie = 0
sus current_test_name tea = ""
sus current_suite_name tea = ""

fr fr Test configuration
sus test_verbose lit = based
sus floating_point_tolerance drip = 0.0001

fr fr Core functions
slay test_start(name tea) {
    test_count = test_count + 1
    current_test_name = name
    lowkey test_verbose == based {
        vibez.spill("Running test: " + name)
    }
}

slay test_end() {
    lowkey test_verbose == based {
        vibez.spill("Test completed: " + current_test_name)
    }
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    lowkey test_verbose == based {
        vibez.spill("  ✓ PASS: " + message)
    }
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay test_skip(reason tea) {
    test_skipped = test_skipped + 1
    lowkey test_verbose == based {
        vibez.spill("  ⚠ SKIP: " + reason)
    }
}

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

slay assert_eq_float(actual drip, expected drip) {
    sus diff drip = actual - expected
    lowkey diff < 0.0 {
        diff = -diff
    }
    
    lowkey diff <= floating_point_tolerance {
        test_pass("assert_eq_float: " + tea(actual) + " ≈ " + tea(expected))
    } highkey {
        test_fail("assert_eq_float failed: " + tea(actual) + " is not ≈ " + tea(expected))
    }
}

slay suite_start(name tea) {
    current_suite_name = name
    vibez.spill("")
    vibez.spill("=== Test Suite: " + name + " ===")
}

slay suite_end() {
    vibez.spill("=== End Suite: " + current_suite_name + " ===")
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== ENHANCED TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    vibez.spill("Skipped: " + tea(test_skipped))
    
    lowkey test_count > 0 {
        sus success_rate normie = (test_passed * 100) / test_count
        vibez.spill("Success rate: " + tea(success_rate) + "%")
    }
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ " + tea(test_failed) + " test(s) failed")
    }
}

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    test_skipped = 0
    current_test_name = ""
    current_suite_name = ""
}

fr fr ================================
fr fr Simple Test Cases
fr fr ================================

slay test_basic_assertions() {
    test_start("test_basic_assertions")
    
    fr fr Integer tests
    assert_eq_int(42, 42)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    
    fr fr String tests
    assert_eq_string("hello", "hello")
    assert_eq_string("CURSED", "CURSED")
    
    fr fr Boolean tests
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
    
    test_end()
}

slay test_float_assertions() {
    test_start("test_float_assertions")
    
    fr fr Float equality tests
    assert_eq_float(3.14159, 3.14159)
    assert_eq_float(1.0, 1.0)
    assert_eq_float(0.0, 0.0)
    
    test_end()
}

slay test_suite_functionality() {
    suite_start("Test Suite Demo")
    
    test_start("test_in_suite")
    assert_eq_int(1, 1)
    assert_eq_string("suite", "suite")
    test_end()
    
    suite_end()
}

slay test_skipping_functionality() {
    test_start("test_skipping")
    
    assert_eq_int(1, 1)
    test_skip("This is an intentional skip for demonstration")
    
    test_end()
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 Starting Simple Enhanced Testing Framework Test...")
    vibez.spill("")
    
    fr fr Reset for clean test run
    reset_test_state()
    
    fr fr Run test suites
    vibez.spill("=== Running Basic Tests ===")
    test_basic_assertions()
    test_float_assertions()
    
    vibez.spill("=== Running Suite Tests ===")
    test_suite_functionality()
    
    vibez.spill("=== Running Skip Tests ===")
    test_skipping_functionality()
    
    fr fr Print final results
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🎯 Simple testing framework validation complete!")
    
    fr fr Return appropriate exit code
    lowkey test_failed > 0 {
        vibez.spill("❌ Some tests failed - returning exit code 1")
        damn 1
    } highkey {
        vibez.spill("✅ All tests passed - returning exit code 0")
        damn 0
    }
}
