# Test the fixed testz framework
# Direct inclusion to test functionality

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

# Assert string equality - simplified to avoid type conversion issues
slay assert_eq_string(actual tea, expected tea) {
    if actual == expected {
        vibez.spill("  PASS: assert_eq_string")
    } else {
        vibez.spill("  FAIL: assert_eq_string")
        current_test_passed = cap
    }
}

# Test end function
slay test_end() {
    if current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("PASSED: " + current_test_name)
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("FAILED: " + current_test_name)
    }
}

# Print summary
slay print_test_summary() {
    vibez.spill("Test Summary")
    vibez.spill("Total: " + total_tests)
    vibez.spill("Passed: " + passed_tests)
    vibez.spill("Failed: " + failed_tests)
}

# Run tests
test_start("string test")
assert_eq_string("", "")
assert_eq_string("hello", "hello")
test_end()

print_test_summary()
