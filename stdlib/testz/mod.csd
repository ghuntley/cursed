sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0

slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    vibez.spill("Test: " + name)
}

slay assert_true(condition lit) {
    lowkey condition {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS")
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL")
    }
}

slay assert_false(condition lit) {
    lowkey condition {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL")
    } nah {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS: " + tea(actual) + " == " + tea(expected))
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: " + tea(actual) + " != " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS: \"" + actual + "\" == \"" + expected + "\"")
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: \"" + actual + "\" != \"" + expected + "\"")
    }
}

slay print_test_summary() {
    vibez.spill("Tests: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
}
