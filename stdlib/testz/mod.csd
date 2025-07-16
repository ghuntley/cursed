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
    vibez.spill("Assert: " + tea(condition))
}

slay print_test_summary() {
    vibez.spill("Tests: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
}
