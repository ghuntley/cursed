fr fr Complete TestResult Integration Test
fr fr Demonstrates the complete TestResult type system with testz framework

yeet "testz/mod_complete"

fr fr ================================
fr fr TestResult Integration Test
fr fr ================================

slay test_basic_testresult_functionality() {
    test_start("Basic TestResult Functionality")
    
    fr fr Test TestResult.pass creation
    sus pass_result TestResult = TestResult.pass("test_math", "assert_eq", "2 + 2 = 4")
    assert_true(TestResult.is_pass(pass_result))
    
    fr fr Test TestResult.fail creation
    sus fail_result TestResult = TestResult.fail("test_div", "assert_eq", "Division failed", "2", "error")
    assert_true(TestResult.is_fail(fail_result))
    
    fr fr Test TestResult.skip creation
    sus skip_result TestResult = TestResult.skip("test_skip", "assert_eq", "Test skipped")
    assert_true(TestResult.is_skip(skip_result))
    
    fr fr Test TestResult.error creation
    sus error_result TestResult = TestResult.error("test_error", "assert_eq", "Test error")
    assert_true(TestResult.is_error(error_result))
    
    vibez.spill("✓ TestResult basic functionality working")
}

slay test_enhanced_assertions() {
    test_start("Enhanced Assertions with TestResult")
    
    fr fr Test assert_eq_int_result function
    sus int_pass TestResult = assert_eq_int_result("test_int", 42, 42)
    assert_true(TestResult.is_pass(int_pass))
    
    sus int_fail TestResult = assert_eq_int_result("test_int_fail", 42, 43)
    assert_true(TestResult.is_fail(int_fail))
    
    fr fr Test assert_eq_string_result function
    sus string_pass TestResult = assert_eq_string_result("test_string", "hello", "hello")
    assert_true(TestResult.is_pass(string_pass))
    
    sus string_fail TestResult = assert_eq_string_result("test_string_fail", "hello", "world")
    assert_true(TestResult.is_fail(string_fail))
    
    fr fr Test assert_eq_bool_result function
    sus bool_pass TestResult = assert_eq_bool_result("test_bool", based, based)
    assert_true(TestResult.is_pass(bool_pass))
    
    sus bool_fail TestResult = assert_eq_bool_result("test_bool_fail", based, cap)
    assert_true(TestResult.is_fail(bool_fail))
    
    vibez.spill("✓ Enhanced assertions working correctly")
}

slay test_result_enhancement() {
    test_start("TestResult Enhancement Functions")
    
    fr fr Test enhancement functions
    sus result TestResult = TestResult.pass("test_enhanced", "assert_eq", "Enhanced test")
    result = TestResult.with_execution_time(result, 150)
    result = TestResult.with_line_number(result, 42)
    result = TestResult.with_file_name(result, "test.csd")
    
    assert_eq_int(result.execution_time, 150)
    assert_eq_int(result.line_number, 42)
    assert_eq_string(result.file_name, "test.csd")
    
    vibez.spill("✓ TestResult enhancement functions working")
}

slay test_collection_and_reporting() {
    test_start("Collection and Reporting")
    
    fr fr Test collection functions
    sus initial_count normie = get_test_count()
    sus initial_passed normie = get_passed_count()
    sus initial_failed normie = get_failed_count()
    
    fr fr Add some test results
    sus test_result TestResult = TestResult.pass("collection_test", "assert_eq", "Collection test")
    add_test_result(test_result)
    
    sus new_count normie = get_test_count()
    sus new_passed normie = get_passed_count()
    
    assert_eq_int(new_count, initial_count + 1)
    assert_eq_int(new_passed, initial_passed + 1)
    
    vibez.spill("✓ Collection and reporting working")
}

slay test_output_formats() {
    test_start("Output Format Generation")
    
    fr fr Test JSON output generation
    sus json_output tea = generate_json_report()
    assert_true(json_output.length > 0)
    
    fr fr Test TAP output generation
    sus tap_output tea = generate_tap_report()
    assert_true(tap_output.length > 0)
    
    vibez.spill("✓ Output format generation working")
    vibez.spill("JSON Output Preview:")
    vibez.spill(json_output)
    vibez.spill("TAP Output Preview:")
    vibez.spill(tap_output)
}

slay main() {
    vibez.spill("Starting Complete TestResult System Integration Test")
    vibez.spill("=======================================================")
    
    test_basic_testresult_functionality()
    test_enhanced_assertions()
    test_result_enhancement()
    test_collection_and_reporting()
    test_output_formats()
    
    vibez.spill("")
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("Complete TestResult System Integration Test Complete")
    vibez.spill("====================================================")
    
    lowkey get_failed_count() == 0 {
        vibez.spill("🎉 Complete TestResult System Integration Successful! 🎉")
    } highkey {
        vibez.spill("❌ Some TestResult integration tests failed")
    }
}
