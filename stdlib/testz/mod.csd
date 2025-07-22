fr fr CURSED Testing Framework (testz) - Minimal Working Version

sus total_test_count normie = 0
sus pass_test_count normie = 0
sus fail_test_count normie = 0

slay test_start(name tea) lit {
    vibez.spill("🧪 Starting test: ", name)
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        vibez.spill("✅ PASS: assert_true")
    } highkey {
        vibez.spill("❌ FAIL: assert_true")
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey condition == cringe {
        vibez.spill("✅ PASS: assert_false")
    } highkey {
        vibez.spill("❌ FAIL: assert_false")
    }
    damn based
}

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        vibez.spill("✅ PASS: assert_eq_int")
    } highkey {
        vibez.spill("❌ FAIL: assert_eq_int")
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey actual == expected {
        vibez.spill("✅ PASS: assert_eq_string")
    } highkey {
        vibez.spill("❌ FAIL: assert_eq_string")
    }
    damn based
}

slay print_test_summary() lit {
    vibez.spill("📊 Test Summary")
    vibez.spill("Total tests: ", total_test_count)
    vibez.spill("Passed: ", pass_test_count)
    vibez.spill("Failed: ", fail_test_count)
    damn based
}
