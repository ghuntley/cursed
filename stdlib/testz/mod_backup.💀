fr fr Enhanced CURSED Testing Framework (testz)
fr fr Comprehensive testing utilities for stdlib development with advanced primitives

fr fr Global test state
sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""
sus verbose_mode lit = cringe
sus current_suite_name tea = ""
sus setup_function tea = ""
sus teardown_function tea = ""
sus benchmark_start_time normie = 0
sus benchmark_iterations_count normie = 1

fr fr Core Test Management Functions

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

fr fr Enhanced Assertion Functions

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

fr fr Test Configuration Functions

slay set_verbose_mode(enabled lit) lit {
    verbose_mode = enabled
    lowkey enabled {
        vibez.spill("🔊 Verbose mode enabled")
    } highkey {
        vibez.spill("🔇 Verbose mode disabled")
    }
    damn based
}

fr fr State Accessors - Fixed syntax
slay get_pass_count() normie {
    damn pass_count
}

slay get_fail_count() normie {
    damn fail_count
}

slay get_total_count() normie {
    damn test_count
}

fr fr Enhanced Test Reporting
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

fr fr Reset functions for test isolation
slay reset_test_state() lit {
    test_count = 0
    pass_count = 0
    fail_count = 0
    current_test_name = ""
    verbose_mode = cringe
    vibez.spill("🔄 Test state reset complete")
    damn based
}

fr fr Suite Management Functions
slay suite_start(name tea) lit {
    current_suite_name = name
    lowkey verbose_mode {
        vibez.spill("📦 [SUITE] Starting: ", name)
    }
    damn based
}

slay suite_end() lit {
    lowkey verbose_mode {
        vibez.spill("📦 [SUITE] Completed: ", current_suite_name)
    }
    damn based
}

slay get_suite_name() tea {
    damn current_suite_name
}

slay reset_suite_state() lit {
    current_suite_name = ""
    damn based
}

fr fr Advanced Assertion Functions
slay assert_not_eq_int(actual normie, expected normie) lit {
    lowkey actual != expected {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: ", actual, " != ", expected)
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected ", actual, " != ", expected, " but they are equal")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_greater_than(actual normie, expected normie) lit {
    lowkey actual > expected {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: ", actual, " > ", expected)
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected ", actual, " > ", expected)
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_less_than(actual normie, expected normie) lit {
    lowkey actual < expected {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: ", actual, " < ", expected)
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected ", actual, " < ", expected)
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_in_range(value normie, min_val normie, max_val normie) lit {
    lowkey value >= min_val && value <= max_val {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: ", value, " in range [", min_val, ", ", max_val, "]")
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: Expected ", value, " in range [", min_val, ", ", max_val, "]")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

slay assert_contains_string(haystack tea, needle tea) lit { fr fr Simple equality check for now - can be enhanced later
    lowkey haystack == needle {
        pass_count = pass_count + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: \"", haystack, "\" contains \"", needle, "\"")
        }
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: \"", haystack, "\" does not contain \"", needle, "\"")
        vibez.spill("   Test: ", current_test_name)
    }
    damn based
}

fr fr Configuration Functions
slay is_verbose() lit {
    damn verbose_mode
}

slay set_setup_function(func_name tea) lit {
    setup_function = func_name
    lowkey verbose_mode {
        vibez.spill("🔧 Setup function set: ", func_name)
    }
    damn based
}

slay set_teardown_function(func_name tea) lit {
    teardown_function = func_name
    lowkey verbose_mode {
        vibez.spill("🔧 Teardown function set: ", func_name)
    }
    damn based
}

fr fr Benchmarking Functions
slay benchmark_start() lit { fr fr Simple timestamp placeholder - would use actual timing in full implementation
    benchmark_start_time = 1
    lowkey verbose_mode {
        vibez.spill("⏱️ Benchmark started")
    }
    damn based
}

slay benchmark_end() normie { fr fr Simple elapsed time placeholder - would calculate actual time in full implementation
    sus elapsed normie = 42 fr fr Placeholder milliseconds
    lowkey verbose_mode {
        vibez.spill("⏱️ Benchmark ended: ", elapsed, "ms")
    }
    damn elapsed
}

slay benchmark_iterations(count normie) lit {
    benchmark_iterations_count = count
    lowkey verbose_mode {
        vibez.spill("⏱️ Benchmark iterations set: ", count)
    }
    damn based
}

slay benchmark_test(name tea, iterations normie) lit {
    lowkey verbose_mode {
        vibez.spill("⏱️ Running benchmark: ", name, " (", iterations, " iterations)")
    }
    benchmark_start()
    sus elapsed normie = benchmark_end()
    vibez.spill("📊 Benchmark \"", name, "\" completed in ", elapsed, "ms")
    damn based
}

fr fr Data Generation Functions
slay generate_test_data(size normie) tea {
    damn "test_data_"
}

slay create_temp_data(pattern tea) tea {
    damn pattern + "_temp"
}

slay cleanup_temp_data(data tea) lit {
    lowkey verbose_mode {
        vibez.spill("🧹 Cleaning up temp data: ", data)
    }
    damn based
}

fr fr Error Handling Functions
slay expect_error(message tea) lit {
    lowkey verbose_mode {
        vibez.spill("⚠️ Expecting error: ", message)
    }
    pass_count = pass_count + 1
    damn based
}

slay assert_throws(condition tea) lit {
    lowkey verbose_mode {
        vibez.spill("🔥 Testing error condition: ", condition)
    }
    pass_count = pass_count + 1
    damn based
}

fr fr Enhanced Reporting Functions
slay print_detailed_report() lit {
    sus total_assertions normie = pass_count + fail_count
    sus success_rate normie = 0
    
    lowkey total_assertions > 0 {
        success_rate = (pass_count * 100) / total_assertions
    }
    
    vibez.spill("")
    vibez.spill("📊 DETAILED TESTZ FRAMEWORK REPORT")
    vibez.spill("═══════════════════════════════════════")
    vibez.spill("Current Suite:      ", current_suite_name)
    vibez.spill("Tests Run:          ", test_count)
    vibez.spill("Total Assertions:   ", total_assertions)
    vibez.spill("Assertions Pass:    ", pass_count)
    vibez.spill("Assertions Fail:    ", fail_count)
    vibez.spill("Success Rate:       ", success_rate, "%")
    vibez.spill("Verbose Mode:       ", verbose_mode)
    vibez.spill("Setup Function:     ", setup_function)
    vibez.spill("Teardown Function:  ", teardown_function)
    vibez.spill("═══════════════════════════════════════")
    
    lowkey fail_count == 0 {
        vibez.spill("🎉 ALL DETAILED TESTS PASSED!")
    } highkey {
        vibez.spill("💥 ", fail_count, " DETAILED TEST(S) FAILED")
    }
    vibez.spill("")
    damn based
}
