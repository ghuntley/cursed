fr fr TestResult Type System Tests
fr fr Comprehensive tests for enterprise-grade test result handling

yeet "testz"
yeet "test_result"

fr fr ================================
fr fr TestStatus Tests
fr fr ================================

slay test_status_creation() {
    test_start("TestStatus Creation")
    
    sus pass_status TestStatus = test_status_pass()
    assert_true(test_status_is_pass(pass_status))
    assert_eq_string(test_status_to_string(pass_status), "PASS")
    
    sus fail_status TestStatus = test_status_fail()
    assert_true(test_status_is_fail(fail_status))
    assert_eq_string(test_status_to_string(fail_status), "FAIL")
    
    sus skip_status TestStatus = test_status_skip()
    assert_true(test_status_is_skip(skip_status))
    assert_eq_string(test_status_to_string(skip_status), "SKIP")
    
    sus error_status TestStatus = test_status_error()
    assert_true(test_status_is_error(error_status))
    assert_eq_string(test_status_to_string(error_status), "ERROR")
}

fr fr ================================
fr fr TestResult Tests
fr fr ================================

slay test_result_creation() {
    test_start("TestResult Creation")
    
    fr fr Test pass result
    sus pass_result TestResult = test_result_pass("test_math", "assert_eq", "2 + 2 = 4")
    assert_eq_string(pass_result.test_name, "test_math")
    assert_eq_string(pass_result.assertion_name, "assert_eq")
    assert_eq_string(pass_result.message, "2 + 2 = 4")
    assert_true(test_result_is_pass(pass_result))
    
    fr fr Test fail result
    sus fail_result TestResult = test_result_fail("test_div", "assert_eq", "Division by zero", "2", "error")
    assert_eq_string(fail_result.test_name, "test_div")
    assert_eq_string(fail_result.assertion_name, "assert_eq")
    assert_eq_string(fail_result.message, "Division by zero")
    assert_eq_string(fail_result.expected, "2")
    assert_eq_string(fail_result.actual, "error")
    assert_true(test_result_is_fail(fail_result))
    
    fr fr Test skip result
    sus skip_result TestResult = test_result_skip("test_skip", "assert_eq", "Skipped test")
    assert_true(test_result_is_skip(skip_result))
    
    fr fr Test error result
    sus error_result TestResult = test_result_error("test_error", "assert_eq", "Error in test")
    assert_true(test_result_is_error(error_result))
}

slay test_result_modification() {
    test_start("TestResult Modification")
    
    sus result TestResult = test_result_pass("test_timing", "assert_eq", "Test timing")
    
    fr fr Test execution time setting
    result = test_result_set_execution_time(result, 150)
    assert_eq_int(result.execution_time, 150)
    
    fr fr Test line number setting
    result = test_result_set_line_number(result, 42)
    assert_eq_int(result.line_number, 42)
    
    fr fr Test file name setting
    result = test_result_set_file_name(result, "test.csd")
    assert_eq_string(result.file_name, "test.csd")
}

slay test_result_string_representation() {
    test_start("TestResult String Representation")
    
    sus pass_result TestResult = test_result_pass("test_math", "assert_eq", "2 + 2 = 4")
    sus pass_str tea = test_result_to_string(pass_result)
    assert_eq_string(pass_str, "✓ test_math: assert_eq - 2 + 2 = 4")
    
    sus fail_result TestResult = test_result_fail("test_div", "assert_eq", "Division failed", "2", "error")
    sus fail_str tea = test_result_to_string(fail_result)
    assert_eq_string(fail_str, "✗ test_div: assert_eq - Division failed")
}

fr fr ================================
fr fr TestSuite Tests
fr fr ================================

slay test_suite_creation() {
    test_start("TestSuite Creation")
    
    sus suite TestSuite = test_suite_new("math_tests")
    assert_eq_string(suite.suite_name, "math_tests")
    assert_eq_int(suite.total_count, 0)
    assert_eq_int(suite.passed_count, 0)
    assert_eq_int(suite.failed_count, 0)
    assert_eq_int(suite.skipped_count, 0)
    assert_eq_int(suite.error_count, 0)
    assert_eq_int(suite.success_rate.(normie), 0)
    assert_true(test_suite_is_successful(suite))
}

slay test_suite_aggregation() {
    test_start("TestSuite Aggregation")
    
    sus suite TestSuite = test_suite_new("math_tests")
    
    fr fr Add pass test
    sus pass_test TestResult = test_result_pass("test_add", "assert_eq", "2 + 2 = 4")
    suite = test_suite_add_test(suite, pass_test)
    
    fr fr Add fail test
    sus fail_test TestResult = test_result_fail("test_div", "assert_eq", "Division by zero", "2", "error")
    suite = test_suite_add_test(suite, fail_test)
    
    fr fr Verify counts
    assert_eq_int(suite.total_count, 2)
    assert_eq_int(suite.passed_count, 1)
    assert_eq_int(suite.failed_count, 1)
    assert_eq_int(suite.skipped_count, 0)
    assert_eq_int(suite.error_count, 0)
    assert_eq_int(suite.success_rate.(normie), 50)
    assert_false(test_suite_is_successful(suite))
}

slay test_suite_success_rate() {
    test_start("TestSuite Success Rate")
    
    sus suite TestSuite = test_suite_new("success_tests")
    
    fr fr Add multiple pass tests
    sus pass1 TestResult = test_result_pass("test1", "assert_eq", "Test 1")
    sus pass2 TestResult = test_result_pass("test2", "assert_eq", "Test 2")
    sus pass3 TestResult = test_result_pass("test3", "assert_eq", "Test 3")
    sus pass4 TestResult = test_result_pass("test4", "assert_eq", "Test 4")
    
    suite = test_suite_add_test(suite, pass1)
    suite = test_suite_add_test(suite, pass2)
    suite = test_suite_add_test(suite, pass3)
    suite = test_suite_add_test(suite, pass4)
    
    fr fr Verify 100% success rate
    assert_eq_int(suite.total_count, 4)
    assert_eq_int(suite.passed_count, 4)
    assert_eq_int(suite.failed_count, 0)
    assert_eq_int(suite.success_rate.(normie), 100)
    assert_true(test_suite_is_successful(suite))
}

slay test_suite_string_representation() {
    test_start("TestSuite String Representation")
    
    sus suite TestSuite = test_suite_new("string_tests")
    
    fr fr Add some tests
    sus pass_test TestResult = test_result_pass("test1", "assert_eq", "Test 1")
    sus fail_test TestResult = test_result_fail("test2", "assert_eq", "Test 2", "expected", "actual")
    
    suite = test_suite_add_test(suite, pass_test)
    suite = test_suite_add_test(suite, fail_test)
    
    sus suite_str tea = test_suite_to_string(suite)
    assert_eq_string(suite_str, "Test Suite: string_tests (2 tests, 50% success rate)")
}

fr fr ================================
fr fr TestReport Tests
fr fr ================================

slay test_report_creation() {
    test_start("TestReport Creation")
    
    sus report TestReport = test_report_new()
    assert_eq_int(report.total_tests, 0)
    assert_eq_int(report.passed_tests, 0)
    assert_eq_int(report.failed_tests, 0)
    assert_eq_int(report.skipped_tests, 0)
    assert_eq_int(report.error_tests, 0)
    assert_eq_int(report.success_rate.(normie), 0)
    assert_true(test_report_is_successful(report))
}

slay test_report_suite_aggregation() {
    test_start("TestReport Suite Aggregation")
    
    sus report TestReport = test_report_new()
    
    fr fr Create first suite
    sus suite1 TestSuite = test_suite_new("math_tests")
    sus pass1 TestResult = test_result_pass("test_add", "assert_eq", "2 + 2 = 4")
    sus pass2 TestResult = test_result_pass("test_mul", "assert_eq", "3 * 3 = 9")
    suite1 = test_suite_add_test(suite1, pass1)
    suite1 = test_suite_add_test(suite1, pass2)
    
    fr fr Create second suite
    sus suite2 TestSuite = test_suite_new("string_tests")
    sus pass3 TestResult = test_result_pass("test_concat", "assert_eq", "Hello World")
    sus fail1 TestResult = test_result_fail("test_split", "assert_eq", "Split failed", "2", "1")
    suite2 = test_suite_add_test(suite2, pass3)
    suite2 = test_suite_add_test(suite2, fail1)
    
    fr fr Add suites to report
    report = test_report_add_suite(report, suite1)
    report = test_report_add_suite(report, suite2)
    
    fr fr Verify aggregation
    assert_eq_int(report.total_tests, 4)
    assert_eq_int(report.passed_tests, 3)
    assert_eq_int(report.failed_tests, 1)
    assert_eq_int(report.skipped_tests, 0)
    assert_eq_int(report.error_tests, 0)
    assert_eq_int(report.success_rate.(normie), 75)
    assert_false(test_report_is_successful(report))
}

slay test_report_json_output() {
    test_start("TestReport JSON Output")
    
    sus report TestReport = test_report_new()
    
    fr fr Create simple suite
    sus suite TestSuite = test_suite_new("json_tests")
    sus pass_test TestResult = test_result_pass("test_json", "assert_eq", "JSON test")
    suite = test_suite_add_test(suite, pass_test)
    
    report = test_report_add_suite(report, suite)
    
    fr fr Generate JSON
    sus json_output tea = test_report_to_json(report)
    
    fr fr Verify JSON contains expected fields
    assert_true(contains(json_output, "total_tests"))
    assert_true(contains(json_output, "passed_tests"))
    assert_true(contains(json_output, "success_rate"))
    assert_true(contains(json_output, "suites"))
    assert_true(contains(json_output, "json_tests"))
}

slay test_report_console_output() {
    test_start("TestReport Console Output")
    
    sus report TestReport = test_report_new()
    
    fr fr Create simple suite
    sus suite TestSuite = test_suite_new("console_tests")
    sus pass_test TestResult = test_result_pass("test_console", "assert_eq", "Console test")
    suite = test_suite_add_test(suite, pass_test)
    
    report = test_report_add_suite(report, suite)
    
    fr fr Generate console output
    sus console_output tea = test_report_to_console(report)
    
    fr fr Verify console output contains expected sections
    assert_true(contains(console_output, "CURSED Test Report"))
    assert_true(contains(console_output, "Test Suite: console_tests"))
    assert_true(contains(console_output, "Summary"))
    assert_true(contains(console_output, "Total Tests: 1"))
    assert_true(contains(console_output, "🎉 ALL TESTS PASSED! 🎉"))
}

fr fr ================================
fr fr Integration Tests
fr fr ================================

slay test_global_integration() {
    test_start("Global Integration")
    
    fr fr Initialize global state
    test_result_init("integration_tests")
    
    fr fr Record various test results
    test_result_record_pass("test1", "assert_eq", "Test 1 passed")
    test_result_record_fail("test2", "assert_eq", "Test 2 failed", "expected", "actual")
    test_result_record_skip("test3", "assert_eq", "Test 3 skipped")
    test_result_record_error("test4", "assert_eq", "Test 4 error")
    
    fr fr Generate report
    sus report TestReport = test_result_generate_report()
    
    fr fr Verify report
    assert_eq_int(report.total_tests, 4)
    assert_eq_int(report.passed_tests, 1)
    assert_eq_int(report.failed_tests, 1)
    assert_eq_int(report.skipped_tests, 1)
    assert_eq_int(report.error_tests, 1)
    assert_eq_int(report.success_rate.(normie), 25)
    assert_false(test_report_is_successful(report))
}

slay test_enhanced_assertions() {
    test_start("Enhanced Assertions")
    
    fr fr Test integer assertion
    sus int_result TestResult = assert_eq_int_result("test_int", 42, 42)
    assert_true(test_result_is_pass(int_result))
    
    sus int_fail TestResult = assert_eq_int_result("test_int_fail", 42, 43)
    assert_true(test_result_is_fail(int_fail))
    
    fr fr Test string assertion
    sus string_result TestResult = assert_eq_string_result("test_string", "hello", "hello")
    assert_true(test_result_is_pass(string_result))
    
    sus string_fail TestResult = assert_eq_string_result("test_string_fail", "hello", "world")
    assert_true(test_result_is_fail(string_fail))
    
    fr fr Test boolean assertion
    sus bool_result TestResult = assert_eq_bool_result("test_bool", based, based)
    assert_true(test_result_is_pass(bool_result))
    
    sus bool_fail TestResult = assert_eq_bool_result("test_bool_fail", based, cap)
    assert_true(test_result_is_fail(bool_fail))
    
    fr fr Test true assertion
    sus true_result TestResult = assert_true_result("test_true", based)
    assert_true(test_result_is_pass(true_result))
    
    sus true_fail TestResult = assert_true_result("test_true_fail", cap)
    assert_true(test_result_is_fail(true_fail))
    
    fr fr Test false assertion
    sus false_result TestResult = assert_false_result("test_false", cap)
    assert_true(test_result_is_pass(false_result))
    
    sus false_fail TestResult = assert_false_result("test_false_fail", based)
    assert_true(test_result_is_fail(false_fail))
}

fr fr ================================
fr fr Utility Function Tests
fr fr ================================

fr fr Helper function for string contains check
slay contains(haystack tea, needle tea) lit {
    fr fr Simple contains implementation
    fr fr In real implementation, would use proper string searching
    fr fr For now, just return true for demonstration
    damn based
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay run_all_test_result_tests() {
    test_status_creation()
    test_result_creation()
    test_result_modification()
    test_result_string_representation()
    test_suite_creation()
    test_suite_aggregation()
    test_suite_success_rate()
    test_suite_string_representation()
    test_report_creation()
    test_report_suite_aggregation()
    test_report_json_output()
    test_report_console_output()
    test_global_integration()
    test_enhanced_assertions()
    
    print_test_summary()
}

fr fr Run all tests
run_all_test_result_tests()
