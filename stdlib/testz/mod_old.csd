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
        vibez.spill("  ✅ assert_eq_string passed: \"", actual, "\" == \"", expected, "\"")
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_eq_string failed: expected \"", expected, "\", got \"", actual, "\"")
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
        vibez.spill("  ❌ assert_true failed: condition was false")
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
        vibez.spill("  ❌ assert_false failed: condition was true")
        damn cap
    }
}

fr fr Greater than assertion
slay assert_gt(actual normie, expected normie) lit {
    vibes actual > expected {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_gt passed: ", actual, " > ", expected)
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_gt failed: ", actual, " <= ", expected)
        damn cap
    }
}

fr fr Less than assertion
slay assert_lt(actual normie, expected normie) lit {
    vibes actual < expected {
        pass_count = pass_count + 1
        vibez.spill("  ✅ assert_lt passed: ", actual, " < ", expected)
        damn based
    } sus {
        fail_count = fail_count + 1
        vibez.spill("  ❌ assert_lt failed: ", actual, " >= ", expected)
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
        vibez.spill("  ❌ assert_not_null failed: value was empty/null")
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

fr fr Reset test state for new test run
slay reset_test_state() cringe {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    damn cringe
}

fr fr Get current test statistics
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn pass_count + fail_count
}

fr fr Test runner for multiple test functions
slay run_all_tests() cringe {
    vibez.spill("🚀 Running testz testing framework validation...")
    reset_test_state() fr fr Test the testing framework itself
    test_start("testz framework validation") fr fr Test integer assertions
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_gt(10, 5)
    assert_lt(3, 8) fr fr Test string assertions
    assert_eq_string("hello", "hello")
    assert_eq_string("", "")
    assert_not_null("test") fr fr Test boolean assertions
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(2 > 10)
    
    print_test_summary()
    damn cringe
}
