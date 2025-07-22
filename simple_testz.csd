fr fr Simple testz implementation

sus total_test_count normie = 0
sus pass_test_count normie = 0

slay test_start(name tea) lit {
    vibez.spill("Starting test: ", name)
    total_test_count = total_test_count + 1
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        pass_test_count = pass_test_count + 1
        vibez.spill("✅ PASS")
    } highkey {
        vibez.spill("❌ FAIL")
    }
    damn based
}

slay print_test_summary() lit {
    vibez.spill("Tests run: ", total_test_count)
    vibez.spill("Passed: ", pass_test_count)
    damn based
}
