# CURSED Testing Framework (testz v2.0)
# Pure CURSED implementation without FFI dependencies

yeet "vibez"

# Global test state
sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0
sus current_test_passed lit = based

# Start a new test case
slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    current_test_passed = based
    vibez.spill("🧪 Running test: " + name)
}

# Assert integer equality
slay assert_eq_int(actual normie, expected normie) {
    if actual == expected {
        vibez.spill("  ✅ assert_eq_int passed: " + actual + " == " + expected)
    } else {
        vibez.spill("  ❌ assert_eq_int failed: " + actual + " != " + expected)
        current_test_passed = cap
    }
}

# Assert string equality
slay assert_eq_string(actual tea, expected tea) {
    if actual == expected {
        vibez.spill("  ✅ assert_eq_string passed")
    } else {
        vibez.spill("  ❌ assert_eq_string failed: '" + actual + "' != '" + expected + "'")
        current_test_passed = cap
    }
}

# Assert float equality (with tolerance)
slay assert_eq_float(actual meal, expected meal) {
    sus tolerance meal = 0.0001
    sus diff meal = actual - expected
    if diff < 0.0 {
        diff = -diff
    }
    
    if diff <= tolerance {
        vibez.spill("  ✅ assert_eq_float passed")
    } else {
        vibez.spill("  ❌ assert_eq_float failed: expected " + expected + ", got " + actual)
        current_test_passed = cap
    }
}

# Assert boolean true
slay assert_true(condition lit) {
    if condition == based {
        vibez.spill("  ✅ assert_true passed")
    } else {
        vibez.spill("  ❌ assert_true failed: expected true, got false")
        current_test_passed = cap
    }
}

# Assert boolean false
slay assert_false(condition lit) {
    if condition == cap {
        vibez.spill("  ✅ assert_false passed")
    } else {
        vibez.spill("  ❌ assert_false failed: expected false, got true")
        current_test_passed = cap
    }
}

# Assert not equal integers
slay assert_ne_int(actual normie, expected normie) {
    if actual != expected {
        vibez.spill("  ✅ assert_ne_int passed: " + actual + " != " + expected)
    } else {
        vibez.spill("  ❌ assert_ne_int failed: " + actual + " == " + expected)
        current_test_passed = cap
    }
}

# Assert greater than
slay assert_gt_int(actual normie, expected normie) {
    if actual > expected {
        vibez.spill("  ✅ assert_gt_int passed: " + actual + " > " + expected)
    } else {
        vibez.spill("  ❌ assert_gt_int failed: " + actual + " <= " + expected)
        current_test_passed = cap
    }
}

# Assert less than
slay assert_lt_int(actual normie, expected normie) {
    if actual < expected {
        vibez.spill("  ✅ assert_lt_int passed: " + actual + " < " + expected)
    } else {
        vibez.spill("  ❌ assert_lt_int failed: " + actual + " >= " + expected)
        current_test_passed = cap
    }
}

# Finish current test and update counters
slay test_end() {
    if current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("✅ Test PASSED: " + current_test_name)
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("❌ Test FAILED: " + current_test_name)
    }
    vibez.spill("")
}

# Print comprehensive test summary
slay print_test_summary() {
    vibez.spill("═══════════════════════════════════════")
    vibez.spill("📊 CURSED Test Suite Summary (testz v2.0)")
    vibez.spill("═══════════════════════════════════════")
    vibez.spill("Total tests: " + total_tests)
    vibez.spill("Passed: " + passed_tests)
    vibez.spill("Failed: " + failed_tests)
    
    if failed_tests == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
        vibez.spill("Status: ✅ SUCCESS")
    } else {
        vibez.spill("⚠️  SOME TESTS FAILED")
        vibez.spill("Status: ❌ FAILURE")
    }
    vibez.spill("═══════════════════════════════════════")
}

# Reset test state (for multiple test runs)
slay reset_test_state() {
    current_test_name = ""
    total_tests = 0
    passed_tests = 0
    failed_tests = 0
    current_test_passed = based
    vibez.spill("🔄 Test state reset")
}

# Helper function to get test results
slay get_test_results() normie {
    damn failed_tests
}

# Helper function to check if all tests passed
slay all_tests_passed() lit {
    damn failed_tests == 0
}
