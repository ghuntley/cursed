fr fr CURSED Testing Framework (testz) - Minimal Working Version

fr fr Global test state - managed by runtime
sus __testz_total drip = 0
sus __testz_passed drip = 0  
sus __testz_failed drip = 0
sus __testz_current_name tea = ""

fr fr Core test functions that work with CURSED runtime
slay test_start(name tea) lit {
    vibez.spill("🧪 Starting test:", name)
    __testz_total = __testz_total + 1
    __testz_current_name = name
    damn based
}

slay assert_true(condition lit) lit {
    lowkey (condition == based) {
        vibez.spill("✅ PASS: assert_true")
        __testz_passed = __testz_passed + 1
        damn based
    } otherwise {
        vibez.spill("❌ FAIL: assert_true")
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

slay assert_false(condition lit) lit {
    lowkey (condition == cringe) {
        vibez.spill("✅ PASS: assert_false")
        __testz_passed = __testz_passed + 1
        damn based
    } otherwise {
        vibez.spill("❌ FAIL: assert_false")
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

slay assert_eq_int(actual drip, expected drip) lit {
    lowkey (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_int")
        __testz_passed = __testz_passed + 1
        damn based
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_int - Expected:", expected, "Got:", actual)
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey (actual == expected) {
        vibez.spill("✅ PASS: assert_eq_string")
        __testz_passed = __testz_passed + 1
        damn based
    } otherwise {
        vibez.spill("❌ FAIL: assert_eq_string - Expected:", expected, "Got:", actual)  
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

slay assert_not_eq_int(actual drip, expected drip) lit {
    lowkey (actual != expected) {
        vibez.spill("✅ PASS: assert_not_eq_int")
        __testz_passed = __testz_passed + 1
        damn based
    } otherwise {
        vibez.spill("❌ FAIL: assert_not_eq_int - Values should not be equal:", actual)
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

slay assert_not_eq_string(actual tea, expected tea) lit {
    lowkey (actual != expected) {
        vibez.spill("✅ PASS: assert_not_eq_string")
        __testz_passed = __testz_passed + 1
        damn based  
    } otherwise {
        vibez.spill("❌ FAIL: assert_not_eq_string - Values should not be equal:", actual)
        __testz_failed = __testz_failed + 1
        damn cringe
    }
}

fr fr Summary and reporting functions
slay print_test_summary() lit {
    vibez.spill("")
    vibez.spill("📊 Test Summary")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Total tests:", __testz_total)
    vibez.spill("Passed:", __testz_passed)
    vibez.spill("Failed:", __testz_failed)
    
    lowkey (__testz_failed == 0) {
        vibez.spill("🎉 All tests passed!")
    } otherwise {
        vibez.spill("❌ Some tests failed")
    }
    
    vibez.spill("═══════════════════════════════════")
    damn based
}

slay all_tests_passed() lit {
    damn __testz_failed == 0
}

fr fr Test suite organization
slay test_section(name tea) lit {
    vibez.spill("")
    vibez.spill("📂", name)
    vibez.spill("───────────────────────────────────")
    damn based
}

slay run_test_suite(name tea) lit {
    vibez.spill("🚀 Running test suite:", name)
    vibez.spill("")
    damn based
}

fr fr Utility functions
slay skip_test(reason tea) lit {
    vibez.spill("⏭️ SKIP:", reason)
    damn based
}

slay test_todo(description tea) lit {
    vibez.spill("📝 TODO:", description) 
    damn based
}

fr fr Reset state for multiple test runs
slay reset_tests() lit {
    __testz_total = 0
    __testz_passed = 0
    __testz_failed = 0
    __testz_current_name = ""
    damn based
}

fr fr Simple benchmark placeholder  
slay benchmark_start(name tea) lit {
    vibez.spill("⏱️ Benchmarking:", name)
    damn based
}

slay benchmark_end() lit {
    vibez.spill("⏱️ Benchmark complete")
    damn based  
}
