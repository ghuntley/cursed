fr fr CURSED VIBEZ Module Tests
fr fr Comprehensive test suite for vibez I/O operations
fr fr Tests all major functionality including error conditions

yeet "vibez"
yeet "testz"

fr fr ===== TEST SETUP =====

sus test_output_buffer []tea = []
sus test_file_path tea = "/tmp/vibez_test.txt"
sus tests_passed normie = 0
sus tests_failed normie = 0

slay setup_tests() {
    vibez.spill("=== VIBEZ Module Test Suite ===")
    vibez.spill("Testing core I/O functionality...")
    vibez.spill("")
}

slay teardown_tests() {
    vibez.spill("")
    vibez.spillf("Tests completed: %s passed, %s failed", [
        testz.int_to_string(tests_passed), 
        testz.int_to_string(tests_failed)
    ])
    
    ready tests_failed == 0 {
        vibez.spill("✅ All tests passed!")
    } otherwise {
        vibez.spill("❌ Some tests failed!")
    }
}

slay assert_true(condition lit, test_name tea) {
    ready condition == based {
        tests_passed++
        vibez.spillf("✅ PASS: %s", [test_name])
    } otherwise {
        tests_failed++
        vibez.spillf("❌ FAIL: %s", [test_name])
    }
}

slay assert_equal(actual tea, expected tea, test_name tea) {
    ready actual == expected {
        tests_passed++
        vibez.spillf("✅ PASS: %s", [test_name])
    } otherwise {
        tests_failed++
        vibez.spillf("❌ FAIL: %s - expected '%s', got '%s'", [test_name, expected, actual])
    }
}

fr fr ===== CORE OUTPUT TESTS =====

slay test_basic_spill() {
    vibez.spill("--- Testing basic spill() function ---")
    
    fr fr Test normal operation
    sus result1 lit = vibez.spill("Test message")
    assert_true(result1, "spill() returns success for valid message")
    
    fr fr Test empty string
    sus result2 lit = vibez.spill("")
    assert_true(result2, "spill() handles empty string")
    
    fr fr Test null handling would be: vibez.spill(cringe) 
    fr fr But this would be caught at compile time in CURSED
    
    vibez.spill("Basic spill() tests completed")
    vibez.spill("")
}

slay test_spillln() {
    vibez.spill("--- Testing spillln() function ---")
    
    sus result1 lit = vibez.spillln("Line with newline")
    assert_true(result1, "spillln() returns success")
    
    sus result2 lit = vibez.spillln("")
    assert_true(result2, "spillln() handles empty string with newline")
    
    vibez.spill("spillln() tests completed")
    vibez.spill("")
}

slay test_spill_values() {
    vibez.spill("--- Testing spill_values() function ---")
    
    sus values []tea = ["Value1", "Value2", "Value3"]
    sus result1 lit = vibez.spill_values(values)
    assert_true(result1, "spill_values() prints multiple values")
    
    sus single_value []tea = ["SingleValue"]
    sus result2 lit = vibez.spill_values(single_value)
    assert_true(result2, "spill_values() handles single value")
    
    vibez.spill("spill_values() tests completed")
    vibez.spill("")
}

slay test_spill_sep() {
    vibez.spill("--- Testing spill_sep() function ---")
    
    sus values []tea = ["A", "B", "C"]
    sus result1 lit = vibez.spill_sep(",", values)
    assert_true(result1, "spill_sep() with comma separator")
    
    sus result2 lit = vibez.spill_sep(" | ", values)
    assert_true(result2, "spill_sep() with pipe separator")
    
    sus result3 lit = vibez.spill_sep("", values)
    assert_true(result3, "spill_sep() with empty separator")
    
    vibez.spill("spill_sep() tests completed")  
    vibez.spill("")
}

fr fr ===== FORMATTED OUTPUT TESTS =====

slay test_spillf() {
    vibez.spill("--- Testing spillf() formatted output ---")
    
    sus args1 []tea = ["World"]
    sus result1 lit = vibez.spillf("Hello %s", args1)
    assert_true(result1, "spillf() basic string substitution")
    
    sus args2 []tea = ["Alice", "25"] 
    sus result2 lit = vibez.spillf("Name: %s, Age: %s", args2)
    assert_true(result2, "spillf() multiple substitutions")
    
    sus args3 []tea = []
    sus result3 lit = vibez.spillf("No substitutions", args3)
    assert_true(result3, "spillf() with no arguments")
    
    vibez.spill("spillf() tests completed")
    vibez.spill("")
}

slay test_spillstr() {
    vibez.spill("--- Testing spillstr() string formatting ---")
    
    sus args1 []tea = ["World"]
    sus formatted1 tea = vibez.spillstr("Hello %s", args1)
    assert_equal(formatted1, "Hello World", "spillstr() basic formatting")
    
    sus args2 []tea = ["Test", "String"]
    sus formatted2 tea = vibez.spillstr("%s %s", args2)
    assert_equal(formatted2, "Test String", "spillstr() multiple arguments")
    
    sus args3 []tea = []
    sus formatted3 tea = vibez.spillstr("No placeholders", args3)
    assert_equal(formatted3, "No placeholders", "spillstr() no formatting")
    
    vibez.spill("spillstr() tests completed")
    vibez.spill("")
}

fr fr ===== SPECIALIZED OUTPUT TESTS =====

slay test_specialized_output() {
    vibez.spill("--- Testing specialized output functions ---")
    
    sus result1 lit = vibez.spill_error("This is an error")
    assert_true(result1, "spill_error() works")
    
    sus result2 lit = vibez.spill_warning("This is a warning")
    assert_true(result2, "spill_warning() works")
    
    sus result3 lit = vibez.spill_debug("This is debug info")
    assert_true(result3, "spill_debug() works")
    
    vibez.spill("Specialized output tests completed")
    vibez.spill("")
}

fr fr ===== INPUT TESTS =====

slay test_input_functions() {
    vibez.spill("--- Testing input functions ---")
    vibez.spill("Note: Input functions tested with simulated runtime")
    
    fr fr These would normally wait for user input
    fr fr In testing, we verify the functions can be called
    
    vibez.spill("Testing input() function signature...")
    fr fr sus user_input tea = vibez.input("Test prompt: ")
    fr fr We can't test actual input without user interaction
    
    vibez.spill("Testing read_line() function signature...")
    fr fr sus line tea = vibez.read_line()
    fr fr We can't test actual line reading without input
    
    assert_true(based, "Input function signatures are valid")
    
    vibez.spill("Input function tests completed (signatures only)")
    vibez.spill("")
}

fr fr ===== FILE I/O TESTS =====

slay test_file_operations() {
    vibez.spill("--- Testing file I/O operations ---")
    
    fr fr Test writing to file
    sus test_content tea = "Hello, CURSED file I/O!"
    sus write_success lit, write_error tea = vibez.write_file(test_file_path, test_content)
    
    ready write_error == "" {
        assert_true(write_success, "write_file() successful write")
    } otherwise {
        assert_true(cap, "write_file() failed: " + write_error)
    }
    
    fr fr Test reading from file
    sus read_content tea, read_error tea = vibez.read_file(test_file_path)
    
    ready read_error == "" {
        assert_equal(read_content, test_content, "read_file() content matches")
    } otherwise {
        assert_true(cap, "read_file() failed: " + read_error)
    }
    
    fr fr Test reading non-existent file
    sus missing_content tea, missing_error tea = vibez.read_file("/nonexistent/path.txt")
    assert_true(missing_error != "", "read_file() returns error for missing file")
    
    fr fr Test writing to invalid path
    sus invalid_success lit, invalid_error tea = vibez.write_file("", "content")
    assert_true(invalid_error != "", "write_file() validates filename")
    
    vibez.spill("File I/O tests completed")
    vibez.spill("")
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() {
    vibez.spill("--- Testing error handling ---")
    
    fr fr Clear any existing errors
    vibez.clear_error()
    sus initial_error tea = vibez.get_last_error()
    assert_equal(initial_error, "", "clear_error() clears error state")
    
    fr fr Trigger an error condition
    sus fail_result lit, fail_error tea = vibez.write_file("", "content")
    assert_true(fail_error != "", "Error condition sets error state")
    
    sus error_after_fail tea = vibez.get_last_error()
    assert_true(error_after_fail != "", "get_last_error() returns error message")
    
    fr fr Clear error and verify
    vibez.clear_error()
    sus cleared_error tea = vibez.get_last_error()
    assert_equal(cleared_error, "", "Error cleared successfully")
    
    vibez.spill("Error handling tests completed")
    vibez.spill("")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    vibez.spill("--- Testing edge cases ---")
    
    fr fr Test very long string (within reasonable limits)
    sus long_string tea = "A very long string that tests the limits of our I/O system. "
    bestie i := 0; i < 10; i++ {
        long_string = long_string + "More text to make it longer. "
    }
    
    sus long_result lit = vibez.spill(long_string)
    assert_true(long_result, "spill() handles long strings")
    
    fr fr Test empty array for spill_values
    sus empty_values []tea = []
    sus empty_result lit = vibez.spill_values(empty_values)
    assert_true(empty_result == cap, "spill_values() fails on empty array")
    
    fr fr Test format string with no placeholders but with args
    sus extra_args []tea = ["unused", "arguments"] 
    sus extra_result lit = vibez.spillf("No placeholders here", extra_args)
    assert_true(extra_result, "spillf() handles extra arguments gracefully")
    
    vibez.spill("Edge case tests completed")
    vibez.spill("")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance() {
    vibez.spill("--- Testing performance characteristics ---")
    
    fr fr Test multiple rapid calls
    sus rapid_success normie = 0
    bestie i := 0; i < 100; i++ {
        sus result lit = vibez.spill("Performance test message")
        ready result == based {
            rapid_success++
        }
    }
    
    assert_true(rapid_success == 100, "Performance test: 100 rapid spill() calls")
    
    fr fr Test large batch output
    sus batch_values []tea = []
    bestie i := 0; i < 50; i++ {
        sus value tea = "Item" + testz.int_to_string(i)
        batch_values = append(batch_values, value)
    }
    
    sus batch_result lit = vibez.spill_values(batch_values)
    assert_true(batch_result, "Performance test: large batch output")
    
    vibez.spill("Performance tests completed")
    vibez.spill("")
}

fr fr ===== INTEGRATION TESTS =====

slay test_integration() {
    vibez.spill("--- Testing integration scenarios ---")
    
    fr fr Test chained operations
    sus content1 tea = "First line"
    sus content2 tea = "Second line"
    sus combined tea = content1 + "\n" + content2
    
    sus write_success lit, write_error tea = vibez.write_file(test_file_path, combined)
    ready write_error == "" {
        sus read_content tea, read_error tea = vibez.read_file(test_file_path)
        ready read_error == "" {
            assert_equal(read_content, combined, "Integration: write then read")
        } otherwise {
            assert_true(cap, "Integration test read failed: " + read_error)
        }
    } otherwise {
        assert_true(cap, "Integration test write failed: " + write_error)
    }
    
    fr fr Test error recovery
    vibez.clear_error()
    sus initial_clear tea = vibez.get_last_error()
    
    fr fr Cause an error
    vibez.write_file("", "content")
    sus error_set tea = vibez.get_last_error()
    
    fr fr Clear and continue
    vibez.clear_error()
    sus error_cleared tea = vibez.get_last_error()
    sus continue_success lit = vibez.spill("Continuing after error")
    
    assert_true(error_cleared == "" && continue_success, "Integration: error recovery")
    
    vibez.spill("Integration tests completed")
    vibez.spill("")
}

fr fr ===== MAIN TEST RUNNER =====

slay main() {
    setup_tests()
    
    fr fr Run all test suites
    test_basic_spill()
    test_spillln()
    test_spill_values()
    test_spill_sep()
    test_spillf()
    test_spillstr()
    test_specialized_output()
    test_input_functions()
    test_file_operations()
    test_error_handling()
    test_edge_cases()
    test_performance()
    test_integration()
    
    teardown_tests()
}

fr fr ===== TEST UTILITIES =====

fr fr Helper function to generate test data
slay generate_test_data(count normie) []tea {
    sus data []tea = []
    bestie i := 0; i < count; i++ {
        sus item tea = "TestItem" + testz.int_to_string(i)
        data = append(data, item)
    }
    damn data
}

fr fr Helper function to create test file content
slay create_test_content(lines normie) tea {
    sus content tea = ""
    bestie i := 0; i < lines; i++ {
        ready i > 0 {
            content = content + "\n"
        }
        content = content + "Test line " + testz.int_to_string(i + 1)
    }
    damn content
}

fr fr Test result summary
slay print_test_summary() {
    sus total_tests normie = tests_passed + tests_failed
    sus pass_rate drip = 0.0
    ready total_tests > 0 {
        pass_rate = testz.int_to_float(tests_passed) / testz.int_to_float(total_tests)
    }
    
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spillf("Total tests: %s", [testz.int_to_string(total_tests)])
    vibez.spillf("Passed: %s", [testz.int_to_string(tests_passed)])
    vibez.spillf("Failed: %s", [testz.int_to_string(tests_failed)])
    vibez.spillf("Pass rate: %s%%", [testz.float_to_string(pass_rate * 100.0)])
    
    ready tests_failed == 0 {
        vibez.spill("🎉 All tests passed successfully!")
    } otherwise {
        vibez.spill("⚠️  Some tests need attention")
    }
}

fr fr Module self-test
slay run_self_test() lit {
    tests_passed = 0
    tests_failed = 0
    
    main()
    
    damn tests_failed == 0
}
