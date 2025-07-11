fr fr TestResult System Integration Test
fr fr Simple test to verify TestResult works with testz framework

yeet "testz"
yeet "test_result"

fr fr ================================
fr fr Basic TestResult Integration
fr fr ================================

slay test_basic_integration() {
    test_start("Basic TestResult Integration")
    
    fr fr Initialize TestResult system
    test_result_init("basic_integration_tests")
    
    fr fr Test basic assertions with TestResult
    sus pass_result TestResult = test_result_pass("test_math", "assert_eq", "2 + 2 = 4")
    assert_true(test_result_is_pass(pass_result))
    assert_eq_string(pass_result.test_name, "test_math")
    assert_eq_string(pass_result.message, "2 + 2 = 4")
    
    fr fr Test fail result
    sus fail_result TestResult = test_result_fail("test_div", "assert_eq", "Division failed", "2", "error")
    assert_true(test_result_is_fail(fail_result))
    assert_eq_string(fail_result.expected, "2")
    assert_eq_string(fail_result.actual, "error")
    
    fr fr Test suite creation
    sus suite TestSuite = test_suite_new("integration_suite")
    suite = test_suite_add_test(suite, pass_result)
    suite = test_suite_add_test(suite, fail_result)
    
    assert_eq_int(suite.total_count, 2)
    assert_eq_int(suite.passed_count, 1)
    assert_eq_int(suite.failed_count, 1)
    assert_eq_int(suite.success_rate.(normie), 50)
    
    fr fr Test report creation
    sus report TestReport = test_report_new()
    report = test_report_add_suite(report, suite)
    
    assert_eq_int(report.total_tests, 2)
    assert_eq_int(report.passed_tests, 1)
    assert_eq_int(report.failed_tests, 1)
    assert_eq_int(report.success_rate.(normie), 50)
    
    vibez.spill("✓ TestResult integration working correctly")
}

slay test_enhanced_assertions() {
    test_start("Enhanced Assertions")
    
    fr fr Test enhanced assertion functions
    sus int_pass TestResult = assert_eq_int_result("test_int", 42, 42)
    assert_true(test_result_is_pass(int_pass))
    
    sus int_fail TestResult = assert_eq_int_result("test_int_fail", 42, 43)
    assert_true(test_result_is_fail(int_fail))
    
    sus string_pass TestResult = assert_eq_string_result("test_string", "hello", "hello")
    assert_true(test_result_is_pass(string_pass))
    
    sus bool_pass TestResult = assert_true_result("test_bool", based)
    assert_true(test_result_is_pass(bool_pass))
    
    vibez.spill("✓ Enhanced assertions working correctly")
}

slay test_report_generation() {
    test_start("Report Generation")
    
    fr fr Create a comprehensive test report
    sus report TestReport = test_report_new()
    
    fr fr Create math suite
    sus math_suite TestSuite = test_suite_new("math_tests")
    math_suite = test_suite_add_test(math_suite, test_result_pass("test_add", "assert_eq", "Addition test"))
    math_suite = test_suite_add_test(math_suite, test_result_pass("test_mul", "assert_eq", "Multiplication test"))
    math_suite = test_suite_add_test(math_suite, test_result_fail("test_div", "assert_eq", "Division test", "2", "error"))
    
    fr fr Create string suite
    sus string_suite TestSuite = test_suite_new("string_tests")
    string_suite = test_suite_add_test(string_suite, test_result_pass("test_concat", "assert_eq", "Concatenation test"))
    string_suite = test_suite_add_test(string_suite, test_result_skip("test_regex", "assert_eq", "Regex test skipped"))
    
    fr fr Add suites to report
    report = test_report_add_suite(report, math_suite)
    report = test_report_add_suite(report, string_suite)
    
    fr fr Verify report totals
    assert_eq_int(report.total_tests, 5)
    assert_eq_int(report.passed_tests, 3)
    assert_eq_int(report.failed_tests, 1)
    assert_eq_int(report.skipped_tests, 1)
    assert_eq_int(report.success_rate.(normie), 60)
    
    fr fr Generate console report
    sus console_output tea = test_report_to_console(report)
    vibez.spill("Console Report Generated:")
    vibez.spill(console_output)
    
    fr fr Generate JSON report
    sus json_output tea = test_report_to_json(report)
    vibez.spill("JSON Report Generated:")
    vibez.spill(json_output)
    
    vibez.spill("✓ Report generation working correctly")
}

slay test_global_collection() {
    test_start("Global Collection")
    
    fr fr Initialize global test collection
    test_result_init("global_collection_tests")
    
    fr fr Record some test results
    test_result_record_pass("test1", "assert_eq", "Test 1 passed")
    test_result_record_fail("test2", "assert_eq", "Test 2 failed", "expected", "actual")
    test_result_record_skip("test3", "assert_eq", "Test 3 skipped")
    
    fr fr Generate and print report
    test_result_print_report()
    
    fr fr Export JSON
    sus json_export tea = test_result_export_json()
    vibez.spill("Exported JSON:")
    vibez.spill(json_export)
    
    vibez.spill("✓ Global collection working correctly")
}

fr fr ================================
fr fr Main Test Execution
fr fr ================================

slay main() {
    vibez.spill("Starting TestResult System Integration Test")
    vibez.spill("=" * 50)
    
    test_basic_integration()
    test_enhanced_assertions()
    test_report_generation()
    test_global_collection()
    
    vibez.spill("=" * 50)
    vibez.spill("TestResult System Integration Test Complete")
    
    print_test_summary()
}

fr fr Run main test
main()
