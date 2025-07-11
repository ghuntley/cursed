fr fr TestResult Type System for CURSED
fr fr Enterprise-grade test result handling with standardized types
fr fr Integrates with testz framework for comprehensive test reporting

fr fr ================================
fr fr TestResult Type and Enums
fr fr ================================

fr fr TestStatus enum for test outcomes
struct TestStatus {
    sus status normie  fr fr 0=Pass, 1=Fail, 2=Skip, 3=Error
}

fr fr TestResult struct for individual test results
struct TestResult {
    sus test_name tea              fr fr Name of the test
    sus assertion_name tea         fr fr Name of the assertion
    sus status TestStatus          fr fr Test outcome status
    sus message tea                fr fr Description/error message
    sus expected tea               fr fr Expected value (optional)
    sus actual tea                 fr fr Actual value (optional)
    sus execution_time normie      fr fr Execution time in milliseconds
    sus line_number normie         fr fr Line number where test is defined
    sus file_name tea              fr fr File name where test is defined
}

fr fr TestSuite struct for aggregating test results
struct TestSuite {
    sus suite_name tea             fr fr Name of the test suite
    sus tests [TestResult]         fr fr Array of test results
    sus total_count normie         fr fr Total number of tests
    sus passed_count normie        fr fr Number of passed tests
    sus failed_count normie        fr fr Number of failed tests
    sus skipped_count normie       fr fr Number of skipped tests
    sus error_count normie         fr fr Number of error tests
    sus success_rate meal          fr fr Success rate percentage
    sus execution_time normie      fr fr Total execution time
}

fr fr TestReport struct for comprehensive reporting
struct TestReport {
    sus suites [TestSuite]         fr fr Array of test suites
    sus total_tests normie         fr fr Total number of tests
    sus passed_tests normie        fr fr Total passed tests
    sus failed_tests normie        fr fr Total failed tests
    sus skipped_tests normie       fr fr Total skipped tests
    sus error_tests normie         fr fr Total error tests
    sus success_rate meal          fr fr Overall success rate
    sus execution_time normie      fr fr Total execution time
    sus timestamp tea              fr fr Report generation timestamp
}

fr fr ================================
fr fr TestStatus Functions
fr fr ================================

slay test_status_pass() TestStatus {
    sus status TestStatus
    status.status = 0
    damn status
}

slay test_status_fail() TestStatus {
    sus status TestStatus
    status.status = 1
    damn status
}

slay test_status_skip() TestStatus {
    sus status TestStatus
    status.status = 2
    damn status
}

slay test_status_error() TestStatus {
    sus status TestStatus
    status.status = 3
    damn status
}

slay test_status_is_pass(status TestStatus) lit {
    damn status.status == 0
}

slay test_status_is_fail(status TestStatus) lit {
    damn status.status == 1
}

slay test_status_is_skip(status TestStatus) lit {
    damn status.status == 2
}

slay test_status_is_error(status TestStatus) lit {
    damn status.status == 3
}

slay test_status_to_string(status TestStatus) tea {
    lowkey status.status == 0 {
        damn "PASS"
    } lowkey status.status == 1 {
        damn "FAIL"
    } lowkey status.status == 2 {
        damn "SKIP"
    } lowkey status.status == 3 {
        damn "ERROR"
    } highkey {
        damn "UNKNOWN"
    }
}

fr fr ================================
fr fr TestResult Functions
fr fr ================================

slay test_result_pass(test_name tea, assertion_name tea, message tea) TestResult {
    sus result TestResult
    result.test_name = test_name
    result.assertion_name = assertion_name
    result.status = test_status_pass()
    result.message = message
    result.expected = ""
    result.actual = ""
    result.execution_time = 0
    result.line_number = 0
    result.file_name = ""
    damn result
}

slay test_result_fail(test_name tea, assertion_name tea, message tea, expected tea, actual tea) TestResult {
    sus result TestResult
    result.test_name = test_name
    result.assertion_name = assertion_name
    result.status = test_status_fail()
    result.message = message
    result.expected = expected
    result.actual = actual
    result.execution_time = 0
    result.line_number = 0
    result.file_name = ""
    damn result
}

slay test_result_skip(test_name tea, assertion_name tea, message tea) TestResult {
    sus result TestResult
    result.test_name = test_name
    result.assertion_name = assertion_name
    result.status = test_status_skip()
    result.message = message
    result.expected = ""
    result.actual = ""
    result.execution_time = 0
    result.line_number = 0
    result.file_name = ""
    damn result
}

slay test_result_error(test_name tea, assertion_name tea, message tea) TestResult {
    sus result TestResult
    result.test_name = test_name
    result.assertion_name = assertion_name
    result.status = test_status_error()
    result.message = message
    result.expected = ""
    result.actual = ""
    result.execution_time = 0
    result.line_number = 0
    result.file_name = ""
    damn result
}

slay test_result_is_pass(result TestResult) lit {
    damn test_status_is_pass(result.status)
}

slay test_result_is_fail(result TestResult) lit {
    damn test_status_is_fail(result.status)
}

slay test_result_is_skip(result TestResult) lit {
    damn test_status_is_skip(result.status)
}

slay test_result_is_error(result TestResult) lit {
    damn test_status_is_error(result.status)
}

slay test_result_set_execution_time(result TestResult, time normie) TestResult {
    result.execution_time = time
    damn result
}

slay test_result_set_line_number(result TestResult, line normie) TestResult {
    result.line_number = line
    damn result
}

slay test_result_set_file_name(result TestResult, file tea) TestResult {
    result.file_name = file
    damn result
}

slay test_result_to_string(result TestResult) tea {
    sus status_str tea = test_status_to_string(result.status)
    sus symbol tea
    
    lowkey test_status_is_pass(result.status) {
        symbol = "✓"
    } lowkey test_status_is_fail(result.status) {
        symbol = "✗"
    } lowkey test_status_is_skip(result.status) {
        symbol = "⚠"
    } lowkey test_status_is_error(result.status) {
        symbol = "⚠"
    } highkey {
        symbol = "?"
    }
    
    damn symbol + " " + result.test_name + ": " + result.assertion_name + " - " + result.message
}

fr fr ================================
fr fr TestSuite Functions
fr fr ================================

slay test_suite_new(suite_name tea) TestSuite {
    sus suite TestSuite
    suite.suite_name = suite_name
    suite.tests = []
    suite.total_count = 0
    suite.passed_count = 0
    suite.failed_count = 0
    suite.skipped_count = 0
    suite.error_count = 0
    suite.success_rate = 0.0
    suite.execution_time = 0
    damn suite
}

slay test_suite_add_test(suite TestSuite, test TestResult) TestSuite {
    fr fr Add test to suite and update counts
    suite.tests = append(suite.tests, test)
    suite.total_count = suite.total_count + 1
    
    lowkey test_result_is_pass(test) {
        suite.passed_count = suite.passed_count + 1
    } lowkey test_result_is_fail(test) {
        suite.failed_count = suite.failed_count + 1
    } lowkey test_result_is_skip(test) {
        suite.skipped_count = suite.skipped_count + 1
    } lowkey test_result_is_error(test) {
        suite.error_count = suite.error_count + 1
    }
    
    fr fr Calculate success rate
    lowkey suite.total_count > 0 {
        suite.success_rate = (suite.passed_count.(meal) / suite.total_count.(meal)) * 100.0
    } highkey {
        suite.success_rate = 0.0
    }
    
    damn suite
}

slay test_suite_is_successful(suite TestSuite) lit {
    damn suite.failed_count == 0 && suite.error_count == 0
}

slay test_suite_set_execution_time(suite TestSuite, time normie) TestSuite {
    suite.execution_time = time
    damn suite
}

slay test_suite_to_string(suite TestSuite) tea {
    damn "Test Suite: " + suite.suite_name + " (" + tea(suite.total_count) + " tests, " + tea(suite.success_rate) + "% success rate)"
}

fr fr ================================
fr fr TestReport Functions
fr fr ================================

slay test_report_new() TestReport {
    sus report TestReport
    report.suites = []
    report.total_tests = 0
    report.passed_tests = 0
    report.failed_tests = 0
    report.skipped_tests = 0
    report.error_tests = 0
    report.success_rate = 0.0
    report.execution_time = 0
    report.timestamp = "2025-01-07T00:00:00Z"  fr fr TODO: Get current timestamp
    damn report
}

slay test_report_add_suite(report TestReport, suite TestSuite) TestReport {
    fr fr Add suite to report and update totals
    report.suites = append(report.suites, suite)
    report.total_tests = report.total_tests + suite.total_count
    report.passed_tests = report.passed_tests + suite.passed_count
    report.failed_tests = report.failed_tests + suite.failed_count
    report.skipped_tests = report.skipped_tests + suite.skipped_count
    report.error_tests = report.error_tests + suite.error_count
    report.execution_time = report.execution_time + suite.execution_time
    
    fr fr Calculate overall success rate
    lowkey report.total_tests > 0 {
        report.success_rate = (report.passed_tests.(meal) / report.total_tests.(meal)) * 100.0
    } highkey {
        report.success_rate = 0.0
    }
    
    damn report
}

slay test_report_is_successful(report TestReport) lit {
    damn report.failed_tests == 0 && report.error_tests == 0
}

slay test_report_to_console(report TestReport) tea {
    sus output tea = "CURSED Test Report\n"
    output = output + "==================\n\n"
    
    fr fr Generate report for each suite
    bestie i := 0; i < len(report.suites); i++ {
        sus suite TestSuite = report.suites[i]
        output = output + "Test Suite: " + suite.suite_name + "\n"
        output = output + "Tests: " + tea(suite.total_count) + " | Passed: " + tea(suite.passed_count) + " | Failed: " + tea(suite.failed_count) + " | Skipped: " + tea(suite.skipped_count) + " | Errors: " + tea(suite.error_count) + "\n"
        output = output + "Success Rate: " + tea(suite.success_rate) + "%\n"
        output = output + "Execution Time: " + tea(suite.execution_time) + "ms\n\n"
        
        fr fr Generate output for each test
        bestie j := 0; j < len(suite.tests); j++ {
            sus test TestResult = suite.tests[j]
            output = output + "  " + test_result_to_string(test) + "\n"
            
            lowkey test_result_is_fail(test) {
                lowkey test.expected != "" && test.actual != "" {
                    output = output + "    Expected: " + test.expected + "\n"
                    output = output + "    Actual:   " + test.actual + "\n"
                }
            }
        }
        
        output = output + "\n"
    }
    
    fr fr Generate summary
    output = output + "Summary\n"
    output = output + "=======\n"
    output = output + "Total Tests: " + tea(report.total_tests) + "\n"
    output = output + "Passed: " + tea(report.passed_tests) + "\n"
    output = output + "Failed: " + tea(report.failed_tests) + "\n"
    output = output + "Skipped: " + tea(report.skipped_tests) + "\n"
    output = output + "Errors: " + tea(report.error_tests) + "\n"
    output = output + "Success Rate: " + tea(report.success_rate) + "%\n"
    output = output + "Total Execution Time: " + tea(report.execution_time) + "ms\n"
    
    lowkey test_report_is_successful(report) {
        output = output + "\n🎉 ALL TESTS PASSED! 🎉\n"
    } highkey {
        output = output + "\n❌ Some tests failed\n"
    }
    
    damn output
}

slay test_report_to_json(report TestReport) tea {
    fr fr Generate JSON report format
    sus json tea = "{\n"
    json = json + "  \"total_tests\": " + tea(report.total_tests) + ",\n"
    json = json + "  \"passed_tests\": " + tea(report.passed_tests) + ",\n"
    json = json + "  \"failed_tests\": " + tea(report.failed_tests) + ",\n"
    json = json + "  \"skipped_tests\": " + tea(report.skipped_tests) + ",\n"
    json = json + "  \"error_tests\": " + tea(report.error_tests) + ",\n"
    json = json + "  \"success_rate\": " + tea(report.success_rate) + ",\n"
    json = json + "  \"execution_time\": " + tea(report.execution_time) + ",\n"
    json = json + "  \"timestamp\": \"" + report.timestamp + "\",\n"
    json = json + "  \"suites\": [\n"
    
    bestie i := 0; i < len(report.suites); i++ {
        sus suite TestSuite = report.suites[i]
        json = json + "    {\n"
        json = json + "      \"suite_name\": \"" + suite.suite_name + "\",\n"
        json = json + "      \"total_count\": " + tea(suite.total_count) + ",\n"
        json = json + "      \"passed_count\": " + tea(suite.passed_count) + ",\n"
        json = json + "      \"failed_count\": " + tea(suite.failed_count) + ",\n"
        json = json + "      \"success_rate\": " + tea(suite.success_rate) + "\n"
        json = json + "    }"
        
        lowkey i < len(report.suites) - 1 {
            json = json + ","
        }
        json = json + "\n"
    }
    
    json = json + "  ]\n"
    json = json + "}\n"
    
    damn json
}

fr fr ================================
fr fr Integration Functions
fr fr ================================

fr fr Global test result collection
sus global_test_results [TestResult]
sus global_test_suite TestSuite
sus global_test_report TestReport

slay test_result_init(suite_name tea) {
    global_test_suite = test_suite_new(suite_name)
    global_test_report = test_report_new()
    global_test_results = []
}

slay test_result_record_pass(test_name tea, assertion_name tea, message tea) {
    sus result TestResult = test_result_pass(test_name, assertion_name, message)
    global_test_results = append(global_test_results, result)
    global_test_suite = test_suite_add_test(global_test_suite, result)
}

slay test_result_record_fail(test_name tea, assertion_name tea, message tea, expected tea, actual tea) {
    sus result TestResult = test_result_fail(test_name, assertion_name, message, expected, actual)
    global_test_results = append(global_test_results, result)
    global_test_suite = test_suite_add_test(global_test_suite, result)
}

slay test_result_record_skip(test_name tea, assertion_name tea, message tea) {
    sus result TestResult = test_result_skip(test_name, assertion_name, message)
    global_test_results = append(global_test_results, result)
    global_test_suite = test_suite_add_test(global_test_suite, result)
}

slay test_result_record_error(test_name tea, assertion_name tea, message tea) {
    sus result TestResult = test_result_error(test_name, assertion_name, message)
    global_test_results = append(global_test_results, result)
    global_test_suite = test_suite_add_test(global_test_suite, result)
}

slay test_result_generate_report() TestReport {
    global_test_report = test_report_add_suite(global_test_report, global_test_suite)
    damn global_test_report
}

slay test_result_print_report() {
    sus report TestReport = test_result_generate_report()
    sus output tea = test_report_to_console(report)
    vibez.spill(output)
}

slay test_result_export_json() tea {
    sus report TestReport = test_result_generate_report()
    damn test_report_to_json(report)
}

fr fr ================================
fr fr Testz Integration Functions
fr fr ================================

fr fr Enhanced assertion functions that work with TestResult
slay assert_eq_int_result(test_name tea, actual normie, expected normie) TestResult {
    lowkey actual == expected {
        damn test_result_pass(test_name, "assert_eq_int", "assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        damn test_result_fail(test_name, "assert_eq_int", "assert_eq_int failed", tea(expected), tea(actual))
    }
}

slay assert_eq_string_result(test_name tea, actual tea, expected tea) TestResult {
    lowkey actual == expected {
        damn test_result_pass(test_name, "assert_eq_string", "assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        damn test_result_fail(test_name, "assert_eq_string", "assert_eq_string failed", expected, actual)
    }
}

slay assert_eq_bool_result(test_name tea, actual lit, expected lit) TestResult {
    lowkey actual == expected {
        damn test_result_pass(test_name, "assert_eq_bool", "assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        damn test_result_fail(test_name, "assert_eq_bool", "assert_eq_bool failed", tea(expected), tea(actual))
    }
}

slay assert_true_result(test_name tea, value lit) TestResult {
    lowkey value == based {
        damn test_result_pass(test_name, "assert_true", "assert_true: value is based")
    } highkey {
        damn test_result_fail(test_name, "assert_true", "assert_true failed", "based", tea(value))
    }
}

slay assert_false_result(test_name tea, value lit) TestResult {
    lowkey value == cap {
        damn test_result_pass(test_name, "assert_false", "assert_false: value is cap")
    } highkey {
        damn test_result_fail(test_name, "assert_false", "assert_false failed", "cap", tea(value))
    }
}

fr fr ================================
fr fr Export Functions
fr fr ================================

fr fr Note: CURSED module export system
fr fr All functions are available when module is imported
