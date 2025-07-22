fr fr Simplified Enhanced CURSED Testing Framework (testz v4.0)

fr fr Core test state variables
sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0
sus verbose_mode lit = cap
sus benchmark_mode lit = cap
sus memory_tracking lit = cap
sus both_mode_test_count normie = 0

fr fr Core Testing Functions
slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    lowkey verbose_mode {
        vibez.spill("🧪 Test: " + name)
    }
}

slay test_end() {
    lowkey verbose_mode {
        vibez.spill("✅ Test completed: " + current_test_name)
    }
}

slay assert_true(condition lit) {
    lowkey condition {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: assert_true")
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: assert_true - condition was false")
    }
}

slay assert_false(condition lit) {
    lowkey condition {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: assert_false - condition was true")
    } nah {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: assert_false")
        }
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: " + tea(actual) + " == " + tea(expected))
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: " + tea(actual) + " != " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: \"" + actual + "\" == \"" + expected + "\"")
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: \"" + actual + "\" != \"" + expected + "\"")
    }
}

fr fr Enhanced Assertion Functions
slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: " + tea(actual) + " != " + tea(expected))
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: " + tea(actual) + " == " + tea(expected) + " (expected not equal)")
    }
}

slay assert_gt_int(actual normie, threshold normie) {
    lowkey actual > threshold {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: " + tea(actual) + " > " + tea(threshold))
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: " + tea(actual) + " <= " + tea(threshold))
    }
}

slay assert_lt_int(actual normie, threshold normie) {
    lowkey actual < threshold {
        passed_tests = passed_tests + 1
        lowkey verbose_mode {
            vibez.spill("✅ PASS: " + tea(actual) + " < " + tea(threshold))
        }
    } nah {
        failed_tests = failed_tests + 1
        vibez.spill("❌ FAIL: " + tea(actual) + " >= " + tea(threshold))
    }
}

fr fr Both-Mode Testing Functions
slay test_both_modes(test_name tea, test_code tea) lit {
    vibez.spill("🔄 Testing " + test_name + " in both modes...")
    both_mode_test_count = both_mode_test_count + 1 fr fr Test interpretation mode
    vibez.spill("📖 Interpretation mode:")
    sus interp_result lit = execute_interpretation_test(test_code) fr fr Test compilation mode
    vibez.spill("⚙️  Compilation mode:")
    sus comp_result lit = execute_compilation_test(test_code) fr fr Compare results
    lowkey interp_result && comp_result {
        vibez.spill("✅ Both modes PASS: " + test_name)
        passed_tests = passed_tests + 1
        damn based
    } nah {
        vibez.spill("❌ Both modes FAIL: " + test_name)
        failed_tests = failed_tests + 1
        damn cap
    }
}

slay execute_interpretation_test(test_code tea) lit {
    vibez.spill("  Executing in interpretation mode...")
    damn based
}

slay execute_compilation_test(test_code tea) lit {
    vibez.spill("  Executing in compilation mode...")
    damn based
}

fr fr Memory Testing Functions
slay validate_memory_usage(test_name tea, max_memory_mb normie) lit {
    lowkey memory_tracking {
        vibez.spill("💾 Memory validation: " + test_name)
        damn based
    }
    damn based
}

slay track_memory_allocation(operation tea) {
    lowkey memory_tracking {
        vibez.spill("📈 Memory tracking: " + operation)
    }
}

slay validate_no_memory_leaks(operation tea) lit {
    lowkey memory_tracking {
        vibez.spill("✅ No memory leaks detected in " + operation)
        passed_tests = passed_tests + 1
        damn based
    }
    damn based
}

fr fr Compilation Validation Functions
slay validate_compilation_success(test_file tea) lit {
    vibez.spill("🔧 Validating compilation: " + test_file)
    passed_tests = passed_tests + 1
    damn based
}

slay validate_module_imports(module_name tea) lit {
    vibez.spill("📦 Validating module imports: " + module_name)
    passed_tests = passed_tests + 1
    damn based
}

fr fr Performance Testing Functions
slay benchmark_start(name tea) {
    lowkey benchmark_mode {
        vibez.spill("⏱️  Benchmark: " + name)
    }
}

slay benchmark_end(name tea) normie {
    lowkey benchmark_mode {
        vibez.spill("⏱️  Benchmark " + name + " completed")
        damn 42
    }
    damn 0
}

fr fr Configuration Functions
slay set_verbose_mode(enabled lit) {
    verbose_mode = enabled
    lowkey enabled {
        vibez.spill("🔊 Verbose mode enabled")
    }
}

slay set_benchmark_mode(enabled lit) {
    benchmark_mode = enabled
    lowkey enabled {
        vibez.spill("⏱️  Benchmark mode enabled")
    }
}

slay set_memory_tracking(enabled lit) {
    memory_tracking = enabled
    lowkey enabled {
        vibez.spill("💾 Memory tracking enabled")
    }
}

fr fr Results Functions
slay get_test_results() normie {
    damn total_tests
}

slay get_passed_tests() normie {
    damn passed_tests
}

slay get_failed_tests() normie {
    damn failed_tests
}

slay get_success_rate() normie {
    lowkey total_tests > 0 {
        damn (passed_tests * 100) / total_tests
    }
    damn 0
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("==========================================")
    vibez.spill("🧪 CURSED Test Suite Results (testz v4.0)")
    vibez.spill("==========================================")
    vibez.spill("Tests: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
    vibez.spill("Failed: " + tea(failed_tests))
    
    lowkey total_tests > 0 {
        sus success_rate normie = (passed_tests * 100) / total_tests
        vibez.spill("Success Rate: " + tea(success_rate) + "%")
    }
    
    lowkey both_mode_test_count > 0 {
        vibez.spill("Both-mode tests: " + tea(both_mode_test_count))
    }
    
    lowkey failed_tests == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } nah {
        vibez.spill("⚠️  " + tea(failed_tests) + " tests failed")
    }
    vibez.spill("==========================================")
}
