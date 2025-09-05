yeet "vibez"

fr fr testz - Core Testing Framework for CURSED
fr fr Pure CURSED implementation with zero FFI dependencies

fr fr Global test state tracking
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""

fr fr Initialize test run
slay test_start(name tea) cringe {
    current_test_name = name
    test_count = test_count + 1
    vibez.spill("🧪 Starting test: ", name)
    damn cringe
}

fr fr Integer equality assertion
slay assert_eq_int(actual normie, expected normie) lit {
    vibes actual == expected {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_eq_int passed: ", actual, " == ", expected)
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_eq_int failed: expected ", expected, ", got ", actual)
        damn cap
    }
}

fr fr String equality assertion  
slay assert_eq_string(actual tea, expected tea) lit {
    vibes actual == expected {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_eq_string passed")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_eq_string failed")
        damn cap
    }
}

fr fr Boolean true assertion
slay assert_true(condition lit) lit {
    vibes condition == based {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_true passed")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_true failed")
        damn cap
    }
}

fr fr Boolean false assertion
slay assert_false(condition lit) lit {
    vibes condition == cap {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_false passed")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_false failed")
        damn cap
    }
}

fr fr Greater than assertion
slay assert_gt(actual normie, expected normie) lit {
    vibes actual > expected {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_gt passed")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_gt failed")
        damn cap
    }
}

fr fr Not null assertion
slay assert_not_null(value tea) lit {
    vibes value != "" {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_not_null passed")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_not_null failed")
        damn cap
    }
}

fr fr Test completion summary
slay print_test_summary() cringe {
    vibez.spill("")
    vibez.spill("📊 Test Summary for: ", current_test_name)
    vibez.spill("  Total assertions: ", pass_count + fail_count)
    vibez.spill("  Passed: ", pass_count)
    vibez.spill("  Failed: ", fail_count)
    
    vibes fail_count == 0 {
        vibez.spill("  🎉 ALL TESTS PASSED!")
    } sus {
        vibez.spill("  💥 SOME TESTS FAILED!")
    }
    
    vibez.spill("")
    damn cringe
}
