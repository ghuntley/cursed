fr fr Simple CURSED Testing Framework (testz) - Basic Version for Testing

sus total_test_count normie = 0
sus pass_test_count normie = 0
sus fail_test_count normie = 0
sus current_test_name tea = ""

fr fr Core testing functions
slay test_start(name tea) lit {
    current_test_name = name
    total_test_count = total_test_count + 1
    vibez.spill("🧪 Starting test: ", name)
    damn based
}

fr fr Basic assertions
slay assert_true(condition lit) lit {
    lowkey condition == based {
        vibez.spill("✅ PASS: assert_true")
        pass_test_count = pass_test_count + 1
    } highkey {
        vibez.spill("❌ FAIL: assert_true - Expected: based, Got: cringe")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_false(condition lit) lit {
    lowkey condition == cringe {
        vibez.spill("✅ PASS: assert_false")
        pass_test_count = pass_test_count + 1
    } highkey {
        vibez.spill("❌ FAIL: assert_false - Expected: cringe, Got: based")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_int(actual normie, expected normie) lit {
    lowkey actual == expected {
        vibez.spill("✅ PASS: assert_eq_int")
        pass_test_count = pass_test_count + 1
    } highkey {
        vibez.spill("❌ FAIL: assert_eq_int - Expected: ", expected, ", Got: ", actual)
        fail_test_count = fail_test_count + 1
    }
    damn based
}

slay assert_eq_string(actual tea, expected tea) lit {
    lowkey actual == expected {
        vibez.spill("✅ PASS: assert_eq_string")
        pass_test_count = pass_test_count + 1
    } highkey {
        vibez.spill("❌ FAIL: assert_eq_string - Expected: '", expected, "', Got: '", actual, "'")
        fail_test_count = fail_test_count + 1
    }
    damn based
}

fr fr Summary function
slay print_test_summary() lit {
    vibez.spill("\n📊 Test Summary")
    vibez.spill("═══════════════════════════════════")
    vibez.spill("Total tests: ", total_test_count)
    vibez.spill("Passed: ", pass_test_count, " ✅")
    vibez.spill("Failed: ", fail_test_count, " ❌")
    
    lowkey fail_test_count == 0 {
        vibez.spill("🎉 All tests passed!")
    } highkey {
        sus pass_rate meal = pass_test_count / total_test_count
        vibez.spill("📈 Pass rate: ", pass_rate)
    }
    
    damn based
}
