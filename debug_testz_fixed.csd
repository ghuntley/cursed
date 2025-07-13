# Direct inclusion of testz functions instead of using yeet import

# Global test state variables
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
    vibez.spill("Running test: " + name)
}

# Assert integer equality
slay assert_eq_int(actual normie, expected normie) {
    if actual == expected {
        vibez.spill("  PASS: assert_eq_int " + actual + " == " + expected)
    } else {
        vibez.spill("  FAIL: assert_eq_int " + actual + " != " + expected)
        current_test_passed = cap
    }
}

# Assert string equality  
slay assert_eq_string(actual tea, expected tea) {
    if actual == expected {
        vibez.spill("  PASS: assert_eq_string")
    } else {
        vibez.spill("  FAIL: assert_eq_string '" + actual + "' != '" + expected + "'")
        current_test_passed = cap
    }
}

# Print test summary
slay print_test_summary() {
    vibez.spill("Test Summary")
    vibez.spill("Total tests: " + total_tests)
    vibez.spill("Passed: " + passed_tests)
    vibez.spill("Failed: " + failed_tests)
    
    if failed_tests == 0 {
        vibez.spill("ALL TESTS PASSED!")
    } else {
        vibez.spill("SOME TESTS FAILED")
    }
}

# Test the framework
test_start("simple test")
assert_eq_int(42, 42)
assert_eq_string("hello", "hello")
assert_eq_string("", "")

# Update counters manually for now
if current_test_passed == based {
    passed_tests = passed_tests + 1
} else {
    failed_tests = failed_tests + 1
}

print_test_summary()
