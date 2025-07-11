fr fr CURSED Testing Framework - Final Working Version
fr fr All essential testing primitives in pure CURSED

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus current_test_name tea = ""

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: value is based")
    } highkey {
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: value is cap")
    } highkey {
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: got " + tea(value) + ", expected cap")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== CURSED Testing Framework Summary ===")
    vibez.spill("Total Tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    lowkey test_count > 0 {
        vibez.spill("Pass Rate: " + tea((test_passed * 100) / test_count) + "%")
    }
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED")
    }
    
    vibez.spill("=========================================")
}

slay test_integer_operations() {
    test_start("test_integer_operations")
    
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_eq_int(10 / 2, 5)
    assert_eq_int(3 + 4 * 5, 23)
    assert_eq_int((3 + 4) * 5, 35)
}

slay test_string_operations() {
    test_start("test_string_operations")
    
    assert_eq_string("hello", "hello")
    assert_eq_string("", "")
    assert_eq_string("hello" + " world", "hello world")
    assert_eq_string("test" + "ing", "testing")
}

slay test_boolean_operations() {
    test_start("test_boolean_operations")
    
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
    assert_true(2 == 2)
    assert_false(2 == 3)
}

slay test_variables_and_expressions() {
    test_start("test_variables_and_expressions")
    
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_eq_int(a + b, 30)
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    
    sus flag lit = based
    assert_true(flag)
    assert_false(!flag)
}

slay main() {
    vibez.spill("🧪 CURSED Testing Framework - Final Version")
    vibez.spill("==========================================")
    vibez.spill("")
    vibez.spill("This is a production-ready testing framework")
    vibez.spill("written entirely in CURSED language.")
    vibez.spill("")
    
    test_integer_operations()
    test_string_operations()
    test_boolean_operations()
    test_variables_and_expressions()
    
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("Framework Features:")
    vibez.spill("✅ Core assertion functions")
    vibez.spill("✅ Test lifecycle management")
    vibez.spill("✅ Comprehensive reporting")
    vibez.spill("✅ Pure CURSED implementation")
    vibez.spill("✅ Production-ready design")
    vibez.spill("")
    
    lowkey test_failed > 0 {
        vibez.spill("❌ Tests failed - exit code 1")
        damn 1
    } highkey {
        vibez.spill("✅ All tests passed - exit code 0")
        damn 0
    }
}
