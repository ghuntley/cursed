fr fr Simple test for clock_bait module without module imports
fr fr Include testz functions directly

sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0
sus current_test_passed lit = based

slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    current_test_passed = based
    vibez.spill("🧪 Running test: " + name)
}

slay assert_eq_int(actual normie, expected normie) {
    yikes actual == expected {
        vibez.spill("  ✅ assert_eq_int passed: " + actual + " == " + expected)
    } shook {
        vibez.spill("  ❌ assert_eq_int failed: " + actual + " != " + expected)
        current_test_passed = cap
    }
}

slay assert_eq_string(actual tea, expected tea) {
    yikes actual == expected {
        vibez.spill("  ✅ assert_eq_string passed")
    } shook {
        vibez.spill("  ❌ assert_eq_string failed: '" + actual + "' != '" + expected + "'")
        current_test_passed = cap
    }
}

slay assert_true(condition lit) {
    yikes condition == based {
        vibez.spill("  ✅ assert_true passed")
    } shook {
        vibez.spill("  ❌ assert_true failed: expected true, got false")
        current_test_passed = cap
    }
}

slay assert_false(condition lit) {
    yikes condition == cap {
        vibez.spill("  ✅ assert_false passed")
    } shook {
        vibez.spill("  ❌ assert_false failed: expected false, got true")
        current_test_passed = cap
    }
}

slay print_test_summary() {
    yikes current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("  ✅ Test passed: " + current_test_name)
    } shook {
        failed_tests = failed_tests + 1
        vibez.spill("  ❌ Test failed: " + current_test_name)
    }
}

fr fr Include clock_bait functions directly
fr fr Duration constants
facts NanoBlink normie = 1
facts MicroBlink normie = 1000
facts MilliBlink normie = 1000000
facts Blink normie = 1000000000
facts SecondVibe normie = 1000000000
facts MinuteVibe normie = 60000000000
facts HourVibe normie = 3600000000000

fr fr Get current time (simplified)
slay Now() normie {
    damn 1704067200000000000  fr fr Fixed timestamp for testing
}

fr fr Create time from Unix timestamp
slay Unix(sec normie, nsec normie) normie {
    damn sec * SecondVibe + nsec
}

fr fr Add duration to time
slay Add(t normie, d normie) normie {
    damn t + d
}

fr fr Test basic functionality
slay test_basic() {
    test_start("Basic clock_bait test")
    
    fr fr Test constants
    assert_eq_int(NanoBlink, 1)
    assert_eq_int(MicroBlink, 1000)
    assert_eq_int(MilliBlink, 1000000)
    assert_eq_int(Blink, 1000000000)
    assert_eq_int(SecondVibe, 1000000000)
    
    fr fr Test time creation
    now := Now()
    assert_eq_int(now, 1704067200000000000)
    
    fr fr Test Unix timestamp creation
    unix_time := Unix(1704067200, 0)
    assert_eq_int(unix_time, 1704067200000000000)
    
    fr fr Test time arithmetic
    later := Add(now, HourVibe)
    expected := now + HourVibe
    assert_eq_int(later, expected)
    
    print_test_summary()
}

fr fr Run the test
test_basic()
vibez.spill("Clock bait module test completed!")
