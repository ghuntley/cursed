// Simple testing framework
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
    vibes actual == expected {
        test_pass("assert_eq_int passed")
    } nah {
        test_fail("assert_eq_int failed")
    }
}

slay assert_eq_string(actual tea, expected tea) {
    vibes actual == expected {
        test_pass("assert_eq_string passed")
    } nah {
        test_fail("assert_eq_string failed")
    }
}

slay assert_true(value lit) {
    vibes value == based {
        test_pass("assert_true passed")
    } nah {
        test_fail("assert_true failed")
    }
}

slay assert_false(value lit) {
    vibes value == cap {
        test_pass("assert_false passed")
    } nah {
        test_fail("assert_false failed")
    }
}

slay print_test_summary() {
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    vibes test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } nah {
        vibez.spill("❌ Some tests failed")
    }
}
