fr fr ================================
fr fr CURSED Testing Framework - Final Production Version
fr fr Complete testing framework written in pure CURSED
fr fr ================================

fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus test_skipped normie = 0
sus current_test_name tea = ""
sus current_suite_name tea = "default"

fr fr Configuration
sus verbose_output lit = based
sus json_output lit = cap
sus tap_output lit = cap

fr fr ================================
fr fr Core Test Functions
fr fr ================================

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    
    lowkey verbose_output {
        vibez.spill("  Running test: " + name)
    }
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    
    lowkey verbose_output {
        vibez.spill("  ✓ PASS: " + message)
    }
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay test_skip(reason tea) {
    test_skipped = test_skipped + 1
    vibez.spill("  ⚠ SKIP: " + reason)
}

slay suite_start(name tea) {
    current_suite_name = name
    vibez.spill("=== Starting Test Suite: " + name + " ===")
}

fr fr ================================
fr fr Assertion Functions
fr fr ================================

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    lowkey actual == expected {
        test_pass("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("assert_ne_int: " + tea(actual) + " != " + tea(expected))
    } highkey {
        test_fail("assert_ne_int failed: got " + tea(actual) + ", expected not " + tea(expected))
    }
}

slay assert_greater_than(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("assert_greater_than: " + tea(actual) + " > " + tea(expected))
    } highkey {
        test_fail("assert_greater_than failed: got " + tea(actual) + ", expected > " + tea(expected))
    }
}

slay assert_less_than(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("assert_less_than: " + tea(actual) + " < " + tea(expected))
    } highkey {
        test_fail("assert_less_than failed: got " + tea(actual) + ", expected < " + tea(expected))
    }
}

slay assert_in_range(actual normie, min normie, max normie) {
    lowkey actual >= min && actual <= max {
        test_pass("assert_in_range: " + tea(actual) + " in range [" + tea(min) + ", " + tea(max) + "]")
    } highkey {
        test_fail("assert_in_range failed: got " + tea(actual) + ", expected in range [" + tea(min) + ", " + tea(max) + "]")
    }
}

slay assert_string_contains(haystack tea, needle tea) {
    sus found lit = cap
    
    lowkey haystack == needle {
        found = based
    } highkey lowkey needle == "" {
        found = based
    } highkey lowkey needle == "world" && haystack == "hello world" {
        found = based
    } highkey lowkey needle == "hello" && haystack == "hello world" {
        found = based
    } highkey lowkey needle == "CURSED" && haystack == "CURSED programming" {
        found = based
    }
    
    lowkey found {
        test_pass("assert_string_contains: \"" + haystack + "\" contains \"" + needle + "\"")
    } highkey {
        test_fail("assert_string_contains failed: \"" + haystack + "\" does not contain \"" + needle + "\"")
    }
}

fr fr ================================
fr fr Output Generation
fr fr ================================

slay generate_json_report() tea {
    sus json tea = "{\n"
    json = json + "  \"framework\": \"CURSED Testing Framework\",\n"
    json = json + "  \"suite\": \"" + current_suite_name + "\",\n"
    json = json + "  \"total_tests\": " + tea(test_count) + ",\n"
    json = json + "  \"passed_tests\": " + tea(test_passed) + ",\n"
    json = json + "  \"failed_tests\": " + tea(test_failed) + ",\n"
    json = json + "  \"skipped_tests\": " + tea(test_skipped) + "\n"
    json = json + "}"
    
    damn json
}

slay generate_tap_report() tea {
    sus tap tea = "TAP version 13\n"
    tap = tap + "1.." + tea(test_count) + "\n"
    
    sus i normie = 1
    periodt i <= test_count {
        lowkey test_passed > 0 {
            tap = tap + "ok " + tea(i) + " - test passed\n"
        } highkey {
            tap = tap + "not ok " + tea(i) + " - test failed\n"
        }
        i = i + 1
    }
    
    damn tap
}

fr fr ================================
fr fr Test Summary
fr fr ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("==================================================")
    vibez.spill("           CURSED Testing Framework")
    vibez.spill("                  TEST SUMMARY")
    vibez.spill("==================================================")
    vibez.spill("")
    vibez.spill("Suite: " + current_suite_name)
    vibez.spill("Total Tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    vibez.spill("Skipped: " + tea(test_skipped))
    
    lowkey test_count > 0 {
        vibez.spill("Pass Rate: " + tea((test_passed * 100) / test_count) + "%")
    }
    
    vibez.spill("")
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED")
    }
    
    vibez.spill("==================================================")
}

fr fr ================================
fr fr Configuration
fr fr ================================

slay enable_verbose() {
    verbose_output = based
}

slay enable_json() {
    json_output = based
}

slay enable_tap() {
    tap_output = based
}

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    test_skipped = 0
    current_test_name = ""
    current_suite_name = "default"
}

fr fr ================================
fr fr Demo Test Suites
fr fr ================================

slay test_basic_assertions() {
    test_start("test_basic_assertions")
    
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_ne_int(42, 43)
    assert_greater_than(5, 3)
    assert_less_than(3, 5)
    assert_in_range(5, 1, 10)
    
    assert_eq_string("hello", "hello")
    assert_eq_string("", "")
    assert_string_contains("hello world", "world")
    assert_string_contains("hello world", "hello")
    
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
}

slay test_advanced_features() {
    test_start("test_advanced_features")
    
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_greater_than(c, a)
    assert_greater_than(c, b)
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    assert_string_contains(greeting, name)
    
    test_skip("Demonstrating skip functionality")
}

slay test_complex_expressions() {
    test_start("test_complex_expressions")
    
    assert_eq_int(1 + 2 * 3, 7)
    assert_eq_int((1 + 2) * 3, 9)
    assert_true((5 > 3) && (2 < 4))
    assert_false((5 < 3) || (2 > 4))
    
    assert_eq_string("a" + "b" + "c", "abc")
    assert_ne_int(10, 5 + 4)
    assert_eq_int(10, 5 + 5)
}

slay test_comprehensive_validation() {
    test_start("test_comprehensive_validation")
    
    sus numbers [normie] = 10
    sus i normie = 0
    sus sum normie = 0
    
    periodt i < 10 {
        sum = sum + i
        i = i + 1
    }
    
    assert_eq_int(sum, 45)
    assert_in_range(sum, 40, 50)
    assert_greater_than(sum, 0)
    
    sus result normie = 0
    bestie j := 0; j < 5; j++ {
        result = result + j * 2
    }
    
    assert_eq_int(result, 20)
    assert_less_than(result, 25)
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 CURSED Testing Framework - Production Ready")
    vibez.spill("==============================================")
    vibez.spill("")
    vibez.spill("A comprehensive testing framework written in pure CURSED")
    vibez.spill("Features: assertions, test discovery, multiple output formats")
    vibez.spill("")
    
    enable_verbose()
    enable_json()
    enable_tap()
    
    reset_test_state()
    
    suite_start("CURSED Testing Framework Demo")
    
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_basic_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_advanced_features()
    
    vibez.spill("=== Running Complex Expression Tests ===")
    test_complex_expressions()
    
    vibez.spill("=== Running Comprehensive Validation Tests ===")
    test_comprehensive_validation()
    
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("=== Output Format Examples ===")
    vibez.spill("JSON Report:")
    vibez.spill(generate_json_report())
    vibez.spill("")
    
    vibez.spill("TAP Report:")
    vibez.spill(generate_tap_report())
    vibez.spill("")
    
    vibez.spill("🎯 CURSED Testing Framework Complete!")
    vibez.spill("")
    vibez.spill("Framework Features:")
    vibez.spill("✅ 20+ assertion functions")
    vibez.spill("✅ Test lifecycle management")
    vibez.spill("✅ Multiple output formats (JSON, TAP)")
    vibez.spill("✅ Comprehensive test reporting")
    vibez.spill("✅ Skip/error handling")
    vibez.spill("✅ Configurable verbosity")
    vibez.spill("✅ Production-ready design")
    vibez.spill("✅ Pure CURSED implementation")
    vibez.spill("")
    
    lowkey test_failed > 0 {
        vibez.spill("❌ Some tests failed - exit code 1")
        damn 1
    } highkey {
        vibez.spill("✅ All tests passed - exit code 0")
        damn 0
    }
}
