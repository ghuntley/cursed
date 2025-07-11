fr fr Simple self-contained testing framework

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
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

slay test_basic_assertions() {
    test_start("test_basic_assertions")
    
    assert_eq_int(42, 42)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_eq_int(10 / 2, 5)
}

slay main() {
    vibez.spill("🧪 Starting Simple Testing Framework...")
    
    test_basic_assertions()
    
    print_test_summary()
    
    vibez.spill("🎯 Testing Complete!")
}
