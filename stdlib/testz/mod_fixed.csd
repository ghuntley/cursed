sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Starting test: ", name)
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill("PASS: ", actual, " == ", expected)
    } highkey {
        vibez.spill("FAIL: expected ", expected, ", got ", actual)
    }
}

slay assert_true(condition lit) {
    lowkey condition == based {
        pass_count = pass_count + 1
        vibez.spill("PASS: assert_true")
    } highkey {
        vibez.spill("FAIL: assert_true")
    }
}

slay print_test_summary() {
    vibez.spill("Test Summary Complete")
}
