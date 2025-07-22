fr fr Simplified CURSED Testing Framework (testz)
fr fr Basic testing utilities for stdlib development

fr fr Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""

fr fr Core Test Management Functions
slay test_start(name tea) lit {
    current_test_name = name
    test_count = test_count + 1
    damn based
}

fr fr Basic Assertion Functions
slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected ", expected, ", got ", actual)
        vibez.spill("Test: ", current_test_name)
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey actual == expected {
        pass_count = pass_count + 1
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected \"", expected, "\", got \"", actual, "\"")
        vibez.spill("Test: ", current_test_name)
    }
    damn based
}

slay assert_true(condition lit) lit {
    lowkey condition == based {
        pass_count = pass_count + 1
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected true, got false")
        vibez.spill("Test: ", current_test_name)
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey condition == cringe {
        pass_count = pass_count + 1
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("FAIL: Expected false, got true")
        vibez.spill("Test: ", current_test_name)
    }
    damn based
}

fr fr State Accessors
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn test_count
}

fr fr Test Reporting
slay print_test_summary() lit {
    sus total_assertions normie = pass_count + fail_count
    sus success_rate normie = 0
    
    lowkey total_assertions > 0 {
        success_rate = (pass_count * 100) / total_assertions
    }
    
    vibez.spill("")
    vibez.spill("TEST REPORT")
    vibez.spill("===========")
    vibez.spill("Tests Run: ", test_count)
    vibez.spill("Assertions: ", total_assertions)
    vibez.spill("Pass: ", pass_count)
    vibez.spill("Fail: ", fail_count)
    vibez.spill("Success Rate: ", success_rate, "%")
    vibez.spill("===========")
    
    lowkey fail_count == 0 {
        vibez.spill("ALL TESTS PASSED!")
    } highkey {
        vibez.spill(fail_count, " TEST(S) FAILED")
    }
    vibez.spill("")
    damn based
}

fr fr Reset function
slay reset_test_state() lit {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    damn based
}
