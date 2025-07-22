# Enhanced CURSED Testing Framework (testz)
# Comprehensive testing utilities for stdlib development with advanced primitives

# Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""
sus verbose_mode lit = cringe

# Core Test Management Functions

slay test_start(name tea) lit {
    current_test_name = name
    test_count = test_count + 1
    
    lowkey verbose_mode {
        vibez.spill("🧪 [TEST ", test_count, "] Starting: ", name)
    }
    
    damn based
}

slay test_end() lit {
    lowkey verbose_mode {
        vibez.spill("✨ Test completed: ", current_test_name)
    }
    
    damn based
}

# Enhanced Assertion Functions

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: ", actual, " == ", expected)
        }
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
        lowkey verbose_mode {
            vibez.spill("✅ PASS: \"", actual, "\" == \"", expected, "\"")
        }
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
        lowkey verbose_mode {
            vibez.spill("✅ PASS: assert_true")
        }
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
        lowkey verbose_mode {
            vibez.spill("✅ PASS: assert_false")
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected false, got true")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

# Test Configuration Functions

slay set_verbose_mode(enabled lit) lit {
    verbose_mode = enabled
    lowkey enabled {
        vibez.spill("🔊 Verbose mode enabled")
    } highkey {
        vibez.spill("🔇 Verbose mode disabled")
    }
    damn based
}

# State Accessors - Fixed syntax
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
    sus success_rate normie = 0
    
    lowkey total_assertions > 0 {
        success_rate = (pass_count * 100) / total_assertions
    }
    
    vibez.spill("")
    vibez.spill("📊 COMPREHENSIVE TEST REPORT")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Tests Run:          ", test_count)
    vibez.spill("Total Assertions:   ", total_assertions)
    vibez.spill("Assertions Pass:    ", pass_count)
    vibez.spill("Assertions Fail:    ", fail_count)
    vibez.spill("Success Rate:       ", success_rate, "%")
    vibez.spill("═══════════════════════════════════")
    
    lowkey fail_count == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! EXCELLENT WORK!")
    } highkey {
        vibez.spill("💥 ", fail_count, " TEST(S) FAILED - NEEDS ATTENTION")
    }
    vibez.spill("")
    damn based
}

# Reset functions for test isolation
slay reset_test_state() lit {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    verbose_mode = cringe
    vibez.spill("🔄 Test state reset complete")
    damn based
}
