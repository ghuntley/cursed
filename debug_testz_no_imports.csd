# Core testing state
sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0

slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    vibez.spill("Test: " + name)
}

slay test_pass(message tea) {
    passed_tests = passed_tests + 1
    vibez.spill("  ✓ " + message)
}

slay assert_true(condition lit) {
    fr fr condition == based {
        test_pass("assert_true: condition is based")
    } else {
        vibez.spill("  ✗ assert_true: condition is not based")
    }
}

slay print_test_summary() {
    vibez.spill("====================================")
    vibez.spill("📋 Test Summary")
    vibez.spill("====================================")
    vibez.spill("Total Tests: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
    vibez.spill("====================================")
}

# Test the minimal implementation
test_start("minimal test")
assert_true(based)
print_test_summary()
