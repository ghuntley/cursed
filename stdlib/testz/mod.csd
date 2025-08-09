fr fr CURSED Testing Framework (testz) - Simple Working Version

sus total_test_count drip = 0
sus pass_test_count drip = 0
sus fail_test_count drip = 0
sus current_test_name tea = ""

slay test_start(name tea) lit {
    current_test_name = name
    total_test_count = total_test_count + 1
    vibez.spill("🧪 Starting test:", name)
    damn based
}

slay assert_true(condition lit) lit {
    ready (condition == based) {
        vibez.spill("✅ PASS: assert_true")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_true")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_false(condition lit) lit {
    ready (condition == cringe) {
        vibez.spill("✅ PASS: assert_false")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_false")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_int(actual drip, expected drip) lit {
    ready (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_int")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_int - Expected:", expected, "Got:", actual)
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    ready (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_string")
        pass_test_count = pass_test_count + 1
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_string - Expected:", expected, "Got:", actual)
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay get_test_count() drip {
    damn total_test_count
}

slay get_pass_count() drip {
    damn pass_test_count
}

slay get_fail_count() drip {
    damn fail_test_count
}

slay all_tests_passed() lit {
    damn fail_test_count == 0
}

slay print_test_summary() lit {
    vibez.spill("")
    vibez.spill("📊 Test Summary")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Total tests:", total_test_count)
    vibez.spill("Passed:", pass_test_count)
    vibez.spill("Failed:", fail_test_count)
    
    ready (fail_test_count == 0) {
        vibez.spill("🎉 All tests passed!")
    } otherwise {
        vibez.spill("Some tests failed")
    }
    
    vibez.spill("═══════════════════════════════════")
    damn based
}
