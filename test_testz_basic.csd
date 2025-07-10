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
        vibez.spill("  ✗ FAIL: " + tea(actual) + " != " + tea(expected))
    }
}

slay print_test_summary() {
    vibez.spill("Tests run: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
}

test_start("simple test")
assert_eq_int(1, 1)
print_test_summary()
