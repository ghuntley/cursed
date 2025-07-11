fr fr TestResult System Integration Test
fr fr Simple test to verify TestResult works with testz framework

yeet "testz"
yeet "test_result"

fr fr ================================
fr fr Basic TestResult Integration
fr fr ================================

slay test_basic_integration() {
    test_start("Basic TestResult Integration")
    
    fr fr Test basic assertions with TestResult
    sus pass_result TestResult = TestResult.pass("test_math", "assert_eq", "2 + 2 = 4")
    assert_true(TestResult.is_pass(pass_result))
    assert_eq_string(pass_result.test_name, "test_math")
    assert_eq_string(pass_result.message, "2 + 2 = 4")
    
    fr fr Test fail result
    sus fail_result TestResult = TestResult.fail("test_div", "assert_eq", "Division failed", "2", "error")
    assert_true(TestResult.is_fail(fail_result))
    assert_eq_string(fail_result.expected, "2")
    assert_eq_string(fail_result.actual, "error")
    
    fr fr Test suite creation
    sus suite TestSuite = TestSuite.new("integration_suite")
    suite = TestSuite.add_test(suite, pass_result)
    suite = TestSuite.add_test(suite, fail_result)
    
    assert_eq_int(TestSuite.total_count(suite), 2)
    assert_eq_int(TestSuite.passed_count(suite), 1)
    assert_eq_int(TestSuite.failed_count(suite), 1)
    assert_eq_int(TestSuite.success_rate(suite).(normie), 50)
    
    fr fr Test report creation
    sus report TestReport = TestReport.new()
    report = TestReport.add_suite(report, suite)
    
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
    assert_true(TestResult.is_pass(int_pass))
    
    sus int_fail TestResult = assert_eq_int_result("test_int_fail", 42, 43)
    assert_true(TestResult.is_fail(int_fail))
    
    sus string_pass TestResult = assert_eq_string_result("test_string", "hello", "hello")
    assert_true(TestResult.is_pass(string_pass))
    
    sus bool_pass TestResult = assert_true_result("test_bool", based)
    assert_true(TestResult.is_pass(bool_pass))
    
    vibez.spill("✓ Enhanced assertions working correctly")
}

slay test_report_generation() {
    test_start("Report Generation")
    
    fr fr Create a comprehensive test report
    sus report TestReport = TestReport.new()
    
    fr fr Create math suite
    sus math_suite TestSuite = TestSuite.new("math_tests")
    math_suite = TestSuite.add_test(math_suite, TestResult.pass("test_add", "assert_eq", "Addition test"))
    math_suite = TestSuite.add_test(math_suite, TestResult.pass("test_mul", "assert_eq", "Multiplication test"))
    math_suite = TestSuite.add_test(math_suite, TestResult.fail("test_div", "assert_eq", "Division test", "2", "error"))
    
    fr fr Create string suite
    sus string_suite TestSuite = TestSuite.new("string_tests")
    string_suite = TestSuite.add_test(string_suite, TestResult.pass("test_concat", "assert_eq", "Concatenation test"))
    string_suite = TestSuite.add_test(string_suite, TestResult.skip("test_regex", "assert_eq", "Regex test skipped"))
    
    fr fr Add suites to report
    report = TestReport.add_suite(report, math_suite)
    report = TestReport.add_suite(report, string_suite)
    
    fr fr Verify report totals
    assert_eq_int(report.total_tests, 5)
    assert_eq_int(report.passed_tests, 3)
    assert_eq_int(report.failed_tests, 1)
    assert_eq_int(report.skipped_tests, 1)
    assert_eq_int(report.success_rate.(normie), 60)
    
    fr fr Generate console report
    sus console_output tea = TestReport.to_console(report)
    vibez.spill("Console Report Generated:")
    vibez.spill(console_output)
    
    fr fr Generate JSON report
    sus json_output tea = TestReport.to_json(report)
    vibez.spill("JSON Report Generated:")
    vibez.spill(json_output)
    
    vibez.spill("✓ Report generation working correctly")
}

slay test_global_collection() {
    test_start("Global Collection")
    
    fr fr Create a simple collection test
    sus collection_suite TestSuite = TestSuite.new("collection_tests")
    
    fr fr Add some test results
    collection_suite = TestSuite.add_test(collection_suite, TestResult.pass("test1", "assert_eq", "Test 1 passed"))
    collection_suite = TestSuite.add_test(collection_suite, TestResult.fail("test2", "assert_eq", "Test 2 failed", "expected", "actual"))
    collection_suite = TestSuite.add_test(collection_suite, TestResult.skip("test3", "assert_eq", "Test 3 skipped"))
    
    fr fr Create report and add suite
    sus collection_report TestReport = TestReport.new()
    collection_report = TestReport.add_suite(collection_report, collection_suite)
    
    fr fr Generate console report
    sus console_report tea = TestReport.to_console(collection_report)
    vibez.spill("Collection Report:")
    vibez.spill(console_report)
    
    fr fr Export JSON
    sus json_export tea = TestReport.to_json(collection_report)
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
