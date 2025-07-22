# Enhanced CURSED Testing Framework (testz)
# Comprehensive testing utilities for stdlib development

# Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""

# Core Test Management Functions

slay test_start(name tea) lit {
    current_test_name = name
    test_count = test_count + 1
    vibez.spill("🧪 [TEST ", test_count, "] Starting: ", name)
    damn based
}

# Enhanced Assertion Functions

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill("✅ PASS: ", actual, " == ", expected)
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected ", expected, ", got ", actual)
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
        vibez.spill("✅ PASS: \"", actual, "\" == \"", expected, "\"")
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected \"", expected, "\", got \"", actual, "\"")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        pass_count = pass_count + 1
        vibez.spill("✅ PASS: assert_true")
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected true, got false")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey condition == cringe {
        pass_count = pass_count + 1
        vibez.spill("✅ PASS: assert_false")
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected false, got true")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

# State Accessors
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn test_count
}

# Enhanced Test Reporting
slay print_test_summary() lit {
    sus total_assertions normie = pass_count + fail_count
    
    vibez.spill("")
    vibez.spill("📊 TEST SUMMARY REPORT")
    vibez.spill("Tests Run:       ", test_count)
    vibez.spill("Assertions Pass: ", pass_count)
    vibez.spill("Assertions Fail: ", fail_count)
    
    lowkey fail_count == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } highkey {
        vibez.spill("💥 SOME TESTS FAILED!")
    }
    vibez.spill("")
    damn based
}
