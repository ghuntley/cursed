# Basic testing functions

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    vibez.spill("Starting test: ", name)
    test_count = test_count + 1
}

slay assert_eq_int(actual normie, expected normie) lit {
    bestie actual == expected {
        vibez.spill("✓ PASS: ", actual, " == ", expected)
        test_passed = test_passed + 1
        damn based
    } else {
        vibez.spill("✗ FAIL: ", actual, " != ", expected)
        test_failed = test_failed + 1
        damn cap
    }
}

slay assert_eq_string(actual tea, expected tea) lit {
    bestie actual == expected {
        vibez.spill("✓ PASS: strings match")
        test_passed = test_passed + 1
        damn based
    } else {
        vibez.spill("✗ FAIL: strings don't match")
        test_failed = test_failed + 1
        damn cap
    }
}

slay assert_true(condition lit) lit {
    bestie condition {
        vibez.spill("✓ PASS: condition is true")
        test_passed = test_passed + 1
        damn based
    } else {
        vibez.spill("✗ FAIL: condition is false")
        test_failed = test_failed + 1
        damn cap
    }
}

slay assert_false(condition lit) lit {
    bestie !condition {
        vibez.spill("✓ PASS: condition is false")
        test_passed = test_passed + 1
        damn based
    } else {
        vibez.spill("✗ FAIL: condition is true")
        test_failed = test_failed + 1
        damn cap
    }
}

slay print_test_summary() {
    vibez.spill("Test Summary:")
    vibez.spill("Total tests: ", test_count)
    vibez.spill("Passed: ", test_passed)
    vibez.spill("Failed: ", test_failed)
}
