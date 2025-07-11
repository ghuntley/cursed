fr fr CURSED TestResult Type System
fr fr Complete test result handling with type safety and serialization
fr fr Author: CURSED Compiler Team
fr fr Version: 1.0.0

fr fr ================================
fr fr Core TestResult Type System
fr fr ================================

fr fr TestResult type with comprehensive metadata
slay TestResult.new(test_name tea, assertion_name tea, status tea, message tea) TestResult {
    damn TestResult {
        test_name: test_name,
        assertion_name: assertion_name,
        status: status,
        message: message,
        expected: "",
        actual: "",
        execution_time: 0,
        line_number: 0,
        file_name: "",
        metadata: {}
    }
}

fr fr TestResult factory functions
slay TestResult.pass(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult.new(test_name, assertion_name, "PASS", message)
}

slay TestResult.fail(test_name tea, assertion_name tea, message tea, expected tea, actual tea) TestResult {
    sus result TestResult = TestResult.new(test_name, assertion_name, "FAIL", message)
    result.expected = expected
    result.actual = actual
    damn result
}

slay TestResult.skip(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult.new(test_name, assertion_name, "SKIP", message)
}

slay TestResult.error(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult.new(test_name, assertion_name, "ERROR", message)
}

fr fr TestResult status checking functions
slay TestResult.is_pass(result TestResult) lit {
    damn result.status == "PASS"
}

slay TestResult.is_fail(result TestResult) lit {
    damn result.status == "FAIL"
}

slay TestResult.is_skip(result TestResult) lit {
    damn result.status == "SKIP"
}

slay TestResult.is_error(result TestResult) lit {
    damn result.status == "ERROR"
}

fr fr TestResult enhancement functions
slay TestResult.with_execution_time(result TestResult, time_ms normie) TestResult {
    result.execution_time = time_ms
    damn result
}

slay TestResult.with_line_number(result TestResult, line normie) TestResult {
    result.line_number = line
    damn result
}

slay TestResult.with_file_name(result TestResult, file_name tea) TestResult {
    result.file_name = file_name
    damn result
}

slay TestResult.with_metadata(result TestResult, key tea, value tea) TestResult {
    result.metadata[key] = value
    damn result
}

fr fr ================================
fr fr TestSuite Type System
fr fr ================================

slay TestSuite.new(suite_name tea) TestSuite {
    damn TestSuite {
        suite_name: suite_name,
        tests: [],
        setup_time: 0,
        teardown_time: 0,
        total_time: 0,
        metadata: {},
        created_at: time.now_string(),
        cursed_version: "8.1.0"
    }
}

slay TestSuite.add_test(suite TestSuite, test TestResult) TestSuite {
    suite.tests.append(test)
    damn suite
}

slay TestSuite.add_metadata(suite TestSuite, key tea, value tea) TestSuite {
    suite.metadata[key] = value
    damn suite
}

slay TestSuite.set_timing(suite TestSuite, setup_time normie, teardown_time normie, total_time normie) TestSuite {
    suite.setup_time = setup_time
    suite.teardown_time = teardown_time
    suite.total_time = total_time
    damn suite
}

fr fr TestSuite statistics functions
slay TestSuite.total_count(suite TestSuite) normie {
    damn suite.tests.length
}

slay TestSuite.passed_count(suite TestSuite) normie {
    sus count normie = 0
    bestie i := 0; i < suite.tests.length; i++ {
        lowkey TestResult.is_pass(suite.tests[i]) {
            count = count + 1
        }
    }
    damn count
}

slay TestSuite.failed_count(suite TestSuite) normie {
    sus count normie = 0
    bestie i := 0; i < suite.tests.length; i++ {
        lowkey TestResult.is_fail(suite.tests[i]) {
            count = count + 1
        }
    }
    damn count
}

slay TestSuite.skipped_count(suite TestSuite) normie {
    sus count normie = 0
    bestie i := 0; i < suite.tests.length; i++ {
        lowkey TestResult.is_skip(suite.tests[i]) {
            count = count + 1
        }
    }
    damn count
}

slay TestSuite.error_count(suite TestSuite) normie {
    sus count normie = 0
    bestie i := 0; i < suite.tests.length; i++ {
        lowkey TestResult.is_error(suite.tests[i]) {
            count = count + 1
        }
    }
    damn count
}

slay TestSuite.success_rate(suite TestSuite) meal {
    sus total normie = TestSuite.total_count(suite)
    lowkey total == 0 {
        damn 0.0
    }
    sus passed normie = TestSuite.passed_count(suite)
    damn (passed.(meal) / total.(meal)) * 100.0
}

slay TestSuite.is_successful(suite TestSuite) lit {
    damn TestSuite.failed_count(suite) == 0 && TestSuite.error_count(suite) == 0
}

fr fr ================================
fr fr TestReport Type System
fr fr ================================

slay TestReport.new() TestReport {
    damn TestReport {
        suites: [],
        total_tests: 0,
        passed_tests: 0,
        failed_tests: 0,
        skipped_tests: 0,
        error_tests: 0,
        success_rate: 0.0,
        execution_time: 0,
        timestamp: time.now_string(),
        metadata: {},
        cursed_version: "8.1.0"
    }
}

slay TestReport.add_suite(report TestReport, suite TestSuite) TestReport {
    report.suites.append(suite)
    report.total_tests = report.total_tests + TestSuite.total_count(suite)
    report.passed_tests = report.passed_tests + TestSuite.passed_count(suite)
    report.failed_tests = report.failed_tests + TestSuite.failed_count(suite)
    report.skipped_tests = report.skipped_tests + TestSuite.skipped_count(suite)
    report.error_tests = report.error_tests + TestSuite.error_count(suite)
    report.execution_time = report.execution_time + suite.total_time
    
    TestReport.calculate_success_rate(report)
    damn report
}

slay TestReport.calculate_success_rate(report TestReport) {
    lowkey report.total_tests == 0 {
        report.success_rate = 0.0
    } highkey {
        report.success_rate = (report.passed_tests.(meal) / report.total_tests.(meal)) * 100.0
    }
}

slay TestReport.is_successful(report TestReport) lit {
    damn report.failed_tests == 0 && report.error_tests == 0
}

slay TestReport.add_metadata(report TestReport, key tea, value tea) TestReport {
    report.metadata[key] = value
    damn report
}

fr fr ================================
fr fr Serialization Functions
fr fr ================================

slay TestResult.to_json(result TestResult) tea {
    sus json tea = "{"
    json = json + "\"test_name\":\"" + result.test_name + "\","
    json = json + "\"assertion_name\":\"" + result.assertion_name + "\","
    json = json + "\"status\":\"" + result.status + "\","
    json = json + "\"message\":\"" + result.message + "\","
    json = json + "\"expected\":\"" + result.expected + "\","
    json = json + "\"actual\":\"" + result.actual + "\","
    json = json + "\"execution_time\":" + tea(result.execution_time) + ","
    json = json + "\"line_number\":" + tea(result.line_number) + ","
    json = json + "\"file_name\":\"" + result.file_name + "\""
    json = json + "}"
    damn json
}

slay TestSuite.to_json(suite TestSuite) tea {
    sus json tea = "{"
    json = json + "\"suite_name\":\"" + suite.suite_name + "\","
    json = json + "\"total_count\":" + tea(TestSuite.total_count(suite)) + ","
    json = json + "\"passed_count\":" + tea(TestSuite.passed_count(suite)) + ","
    json = json + "\"failed_count\":" + tea(TestSuite.failed_count(suite)) + ","
    json = json + "\"skipped_count\":" + tea(TestSuite.skipped_count(suite)) + ","
    json = json + "\"error_count\":" + tea(TestSuite.error_count(suite)) + ","
    json = json + "\"success_rate\":" + tea(TestSuite.success_rate(suite)) + ","
    json = json + "\"total_time\":" + tea(suite.total_time) + ","
    json = json + "\"tests\":["
    
    bestie i := 0; i < suite.tests.length; i++ {
        lowkey i > 0 {
            json = json + ","
        }
        json = json + TestResult.to_json(suite.tests[i])
    }
    
    json = json + "]}"
    damn json
}

slay TestReport.to_json(report TestReport) tea {
    sus json tea = "{"
    json = json + "\"total_tests\":" + tea(report.total_tests) + ","
    json = json + "\"passed_tests\":" + tea(report.passed_tests) + ","
    json = json + "\"failed_tests\":" + tea(report.failed_tests) + ","
    json = json + "\"skipped_tests\":" + tea(report.skipped_tests) + ","
    json = json + "\"error_tests\":" + tea(report.error_tests) + ","
    json = json + "\"success_rate\":" + tea(report.success_rate) + ","
    json = json + "\"execution_time\":" + tea(report.execution_time) + ","
    json = json + "\"timestamp\":\"" + report.timestamp + "\","
    json = json + "\"cursed_version\":\"" + report.cursed_version + "\","
    json = json + "\"suites\":["
    
    bestie i := 0; i < report.suites.length; i++ {
        lowkey i > 0 {
            json = json + ","
        }
        json = json + TestSuite.to_json(report.suites[i])
    }
    
    json = json + "]}"
    damn json
}

fr fr ================================
fr fr XML Serialization
fr fr ================================

slay TestResult.to_xml(result TestResult) tea {
    sus xml tea = "    <testcase"
    xml = xml + " name=\"" + result.assertion_name + "\""
    xml = xml + " classname=\"" + result.test_name + "\""
    xml = xml + " time=\"" + tea(result.execution_time) + "\""
    xml = xml + ">\n"
    
    lowkey result.status == "FAIL" {
        xml = xml + "      <failure message=\"" + result.message + "\">"
        xml = xml + "Expected: " + result.expected + ", Actual: " + result.actual
        xml = xml + "</failure>\n"
    }
    
    lowkey result.status == "ERROR" {
        xml = xml + "      <error message=\"" + result.message + "\"></error>\n"
    }
    
    lowkey result.status == "SKIP" {
        xml = xml + "      <skipped message=\"" + result.message + "\"></skipped>\n"
    }
    
    xml = xml + "    </testcase>\n"
    damn xml
}

slay TestSuite.to_xml(suite TestSuite) tea {
    sus xml tea = "  <testsuite"
    xml = xml + " name=\"" + suite.suite_name + "\""
    xml = xml + " tests=\"" + tea(TestSuite.total_count(suite)) + "\""
    xml = xml + " failures=\"" + tea(TestSuite.failed_count(suite)) + "\""
    xml = xml + " errors=\"" + tea(TestSuite.error_count(suite)) + "\""
    xml = xml + " skipped=\"" + tea(TestSuite.skipped_count(suite)) + "\""
    xml = xml + " time=\"" + tea(suite.total_time) + "\""
    xml = xml + ">\n"
    
    bestie i := 0; i < suite.tests.length; i++ {
        xml = xml + TestResult.to_xml(suite.tests[i])
    }
    
    xml = xml + "  </testsuite>\n"
    damn xml
}

slay TestReport.to_xml(report TestReport) tea {
    sus xml tea = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
    xml = xml + "<testsuites>\n"
    
    bestie i := 0; i < report.suites.length; i++ {
        xml = xml + TestSuite.to_xml(report.suites[i])
    }
    
    xml = xml + "</testsuites>\n"
    damn xml
}

fr fr ================================
fr fr HTML Serialization
fr fr ================================

slay TestResult.to_html_row(result TestResult) tea {
    sus status_class tea = "pass"
    lowkey result.status == "FAIL" {
        status_class = "fail"
    }
    lowkey result.status == "SKIP" {
        status_class = "skip"
    }
    lowkey result.status == "ERROR" {
        status_class = "error"
    }
    
    sus html tea = "<tr>"
    html = html + "<td>" + result.test_name + "</td>"
    html = html + "<td>" + result.assertion_name + "</td>"
    html = html + "<td class=\"" + status_class + "\">" + result.status + "</td>"
    html = html + "<td>" + result.message + "</td>"
    html = html + "<td>" + tea(result.execution_time) + "ms</td>"
    html = html + "</tr>\n"
    damn html
}

slay TestSuite.to_html_section(suite TestSuite) tea {
    sus html tea = "<h3>Test Suite: " + suite.suite_name + "</h3>\n"
    html = html + "<table>\n"
    html = html + "<tr><th>Test</th><th>Assertion</th><th>Status</th><th>Message</th><th>Time</th></tr>\n"
    
    bestie i := 0; i < suite.tests.length; i++ {
        html = html + TestResult.to_html_row(suite.tests[i])
    }
    
    html = html + "</table>\n"
    damn html
}

slay TestReport.to_html(report TestReport) tea {
    sus html tea = "<!DOCTYPE html>\n<html>\n<head>\n"
    html = html + "<title>CURSED Test Report</title>\n"
    html = html + "<style>\n"
    html = html + "body { font-family: Arial, sans-serif; margin: 20px; }\n"
    html = html + ".pass { color: green; }\n"
    html = html + ".fail { color: red; }\n"
    html = html + ".skip { color: orange; }\n"
    html = html + ".error { color: purple; }\n"
    html = html + "table { border-collapse: collapse; width: 100%; }\n"
    html = html + "th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n"
    html = html + "th { background-color: #f2f2f2; }\n"
    html = html + ".summary { background-color: #f9f9f9; padding: 15px; margin-bottom: 20px; }\n"
    html = html + "</style>\n"
    html = html + "</head>\n<body>\n"
    
    html = html + "<h1>CURSED Test Report</h1>\n"
    html = html + "<div class=\"summary\">\n"
    html = html + "<h2>Summary</h2>\n"
    html = html + "<p>Total Tests: " + tea(report.total_tests) + "</p>\n"
    html = html + "<p>Passed: <span class=\"pass\">" + tea(report.passed_tests) + "</span></p>\n"
    html = html + "<p>Failed: <span class=\"fail\">" + tea(report.failed_tests) + "</span></p>\n"
    html = html + "<p>Skipped: <span class=\"skip\">" + tea(report.skipped_tests) + "</span></p>\n"
    html = html + "<p>Errors: <span class=\"error\">" + tea(report.error_tests) + "</span></p>\n"
    html = html + "<p>Success Rate: " + tea(report.success_rate) + "%</p>\n"
    html = html + "<p>Execution Time: " + tea(report.execution_time) + "ms</p>\n"
    html = html + "<p>Timestamp: " + report.timestamp + "</p>\n"
    html = html + "<p>CURSED Version: " + report.cursed_version + "</p>\n"
    html = html + "</div>\n"
    
    bestie i := 0; i < report.suites.length; i++ {
        html = html + TestSuite.to_html_section(report.suites[i])
    }
    
    html = html + "</body>\n</html>\n"
    damn html
}

fr fr ================================
fr fr Console Reporting
fr fr ================================

slay TestResult.to_console_line(result TestResult) tea {
    sus symbol tea = "✓"
    lowkey result.status == "FAIL" {
        symbol = "✗"
    }
    lowkey result.status == "SKIP" {
        symbol = "⚠"
    }
    lowkey result.status == "ERROR" {
        symbol = "⚠"
    }
    
    sus line tea = "  " + symbol + " " + result.test_name + ": " + result.assertion_name + " - " + result.message
    
    lowkey result.status == "FAIL" {
        line = line + "\n    Expected: " + result.expected
        line = line + "\n    Actual:   " + result.actual
    }
    
    damn line
}

slay TestSuite.to_console_section(suite TestSuite) tea {
    sus output tea = "Test Suite: " + suite.suite_name + "\n"
    output = output + "Tests: " + tea(TestSuite.total_count(suite))
    output = output + " | Passed: " + tea(TestSuite.passed_count(suite))
    output = output + " | Failed: " + tea(TestSuite.failed_count(suite))
    output = output + " | Skipped: " + tea(TestSuite.skipped_count(suite))
    output = output + " | Errors: " + tea(TestSuite.error_count(suite)) + "\n"
    output = output + "Success Rate: " + tea(TestSuite.success_rate(suite)) + "%\n"
    output = output + "Execution Time: " + tea(suite.total_time) + "ms\n\n"
    
    bestie i := 0; i < suite.tests.length; i++ {
        output = output + TestResult.to_console_line(suite.tests[i]) + "\n"
    }
    
    output = output + "\n"
    damn output
}

slay TestReport.to_console(report TestReport) tea {
    sus output tea = "CURSED Test Report\n"
    output = output + "==================\n\n"
    
    bestie i := 0; i < report.suites.length; i++ {
        output = output + TestSuite.to_console_section(report.suites[i])
    }
    
    output = output + "Summary\n"
    output = output + "=======\n"
    output = output + "Total Tests: " + tea(report.total_tests) + "\n"
    output = output + "Passed: " + tea(report.passed_tests) + "\n"
    output = output + "Failed: " + tea(report.failed_tests) + "\n"
    output = output + "Skipped: " + tea(report.skipped_tests) + "\n"
    output = output + "Errors: " + tea(report.error_tests) + "\n"
    output = output + "Success Rate: " + tea(report.success_rate) + "%\n"
    output = output + "Total Execution Time: " + tea(report.execution_time) + "ms\n"
    output = output + "Timestamp: " + report.timestamp + "\n"
    output = output + "CURSED Version: " + report.cursed_version + "\n"
    
    lowkey TestReport.is_successful(report) {
        output = output + "\n🎉 ALL TESTS PASSED! 🎉\n"
    } highkey {
        output = output + "\n❌ Some tests failed\n"
    }
    
    damn output
}

fr fr ================================
fr fr Builder Pattern for Fluent API
fr fr ================================

slay TestResultBuilder.new(test_name tea, assertion_name tea) TestResultBuilder {
    damn TestResultBuilder {
        test_name: test_name,
        assertion_name: assertion_name,
        status: "PASS",
        message: "",
        expected: "",
        actual: "",
        execution_time: 0,
        line_number: 0,
        file_name: "",
        metadata: {}
    }
}

slay TestResultBuilder.pass(builder TestResultBuilder, message tea) TestResultBuilder {
    builder.status = "PASS"
    builder.message = message
    damn builder
}

slay TestResultBuilder.fail(builder TestResultBuilder, message tea) TestResultBuilder {
    builder.status = "FAIL"
    builder.message = message
    damn builder
}

slay TestResultBuilder.skip(builder TestResultBuilder, message tea) TestResultBuilder {
    builder.status = "SKIP"
    builder.message = message
    damn builder
}

slay TestResultBuilder.error(builder TestResultBuilder, message tea) TestResultBuilder {
    builder.status = "ERROR"
    builder.message = message
    damn builder
}

slay TestResultBuilder.expected(builder TestResultBuilder, expected tea) TestResultBuilder {
    builder.expected = expected
    damn builder
}

slay TestResultBuilder.actual(builder TestResultBuilder, actual tea) TestResultBuilder {
    builder.actual = actual
    damn builder
}

slay TestResultBuilder.execution_time(builder TestResultBuilder, time_ms normie) TestResultBuilder {
    builder.execution_time = time_ms
    damn builder
}

slay TestResultBuilder.line_number(builder TestResultBuilder, line normie) TestResultBuilder {
    builder.line_number = line
    damn builder
}

slay TestResultBuilder.file_name(builder TestResultBuilder, file_name tea) TestResultBuilder {
    builder.file_name = file_name
    damn builder
}

slay TestResultBuilder.metadata(builder TestResultBuilder, key tea, value tea) TestResultBuilder {
    builder.metadata[key] = value
    damn builder
}

slay TestResultBuilder.build(builder TestResultBuilder) TestResult {
    sus result TestResult = TestResult.new(
        builder.test_name,
        builder.assertion_name,
        builder.status,
        builder.message
    )
    
    result.expected = builder.expected
    result.actual = builder.actual
    result.execution_time = builder.execution_time
    result.line_number = builder.line_number
    result.file_name = builder.file_name
    result.metadata = builder.metadata
    
    damn result
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay TestResult.display_symbol(result TestResult) tea {
    lowkey result.status == "PASS" {
        damn "✓"
    }
    lowkey result.status == "FAIL" {
        damn "✗"
    }
    lowkey result.status == "SKIP" {
        damn "⚠"
    }
    lowkey result.status == "ERROR" {
        damn "⚠"
    }
    damn "?"
}

slay TestResult.display_color(result TestResult) tea {
    lowkey result.status == "PASS" {
        damn "green"
    }
    lowkey result.status == "FAIL" {
        damn "red"
    }
    lowkey result.status == "SKIP" {
        damn "orange"
    }
    lowkey result.status == "ERROR" {
        damn "purple"
    }
    damn "black"
}

slay TestSuite.display_summary(suite TestSuite) tea {
    sus summary tea = "Suite: " + suite.suite_name
    summary = summary + " (" + tea(TestSuite.total_count(suite)) + " tests"
    summary = summary + ", " + tea(TestSuite.success_rate(suite)) + "% success rate)"
    damn summary
}

slay TestReport.display_summary(report TestReport) tea {
    sus summary tea = "Report: " + tea(report.total_tests) + " tests"
    summary = summary + ", " + tea(report.success_rate) + "% success rate"
    damn summary
}

fr fr ================================
fr fr Performance Utilities
fr fr ================================

slay TestResult.benchmark(test_name tea, assertion_name tea, operation slay()) TestResult {
    sus start_time normie = time.now_millis()
    operation()
    sus end_time normie = time.now_millis()
    sus execution_time normie = end_time - start_time
    
    sus result TestResult = TestResult.pass(test_name, assertion_name, "Benchmark completed")
    result = TestResult.with_execution_time(result, execution_time)
    damn result
}

slay TestSuite.benchmark_suite(suite_name tea, operations []slay()) TestSuite {
    sus suite TestSuite = TestSuite.new(suite_name)
    
    bestie i := 0; i < operations.length; i++ {
        sus benchmark_result TestResult = TestResult.benchmark(
            "benchmark_" + tea(i),
            "performance_test",
            operations[i]
        )
        suite = TestSuite.add_test(suite, benchmark_result)
    }
    
    damn suite
}

fr fr ================================
fr fr Integration with testz
fr fr ================================

fr fr Provide backward compatibility and enhanced integration
slay create_test_result(test_name tea, assertion_name tea, status tea, message tea) TestResult {
    damn TestResult.new(test_name, assertion_name, status, message)
}

slay create_test_suite(suite_name tea) TestSuite {
    damn TestSuite.new(suite_name)
}

slay create_test_report() TestReport {
    damn TestReport.new()
}

fr fr Enhanced assertion functions that return TestResult
slay assert_eq_int_result(test_name tea, actual normie, expected normie) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_int", "Integer equality passed")
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_int", "Integer equality failed", tea(expected), tea(actual))
    }
}

slay assert_eq_string_result(test_name tea, actual tea, expected tea) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_string", "String equality passed")
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_string", "String equality failed", expected, actual)
    }
}

slay assert_eq_bool_result(test_name tea, actual lit, expected lit) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_bool", "Boolean equality passed")
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_bool", "Boolean equality failed", tea(expected), tea(actual))
    }
}

slay assert_true_result(test_name tea, value lit) TestResult {
    lowkey value == based {
        damn TestResult.pass(test_name, "assert_true", "Assert true passed")
    } highkey {
        damn TestResult.fail(test_name, "assert_true", "Assert true failed", "based", tea(value))
    }
}

slay assert_false_result(test_name tea, value lit) TestResult {
    lowkey value == cap {
        damn TestResult.pass(test_name, "assert_false", "Assert false passed")
    } highkey {
        damn TestResult.fail(test_name, "assert_false", "Assert false failed", "cap", tea(value))
    }
}

fr fr ================================
fr fr Export Summary
fr fr ================================

fr fr Core Types: TestResult, TestSuite, TestReport, TestResultBuilder
fr fr Factory Functions: TestResult.new, TestSuite.new, TestReport.new
fr fr Status Functions: TestResult.is_pass, TestResult.is_fail, etc.
fr fr Serialization: to_json, to_xml, to_html, to_console
fr fr Builder Pattern: TestResultBuilder for fluent API
fr fr Integration: Enhanced assertion functions returning TestResult
fr fr Performance: Benchmark utilities and timing functions
fr fr Backward Compatibility: Compatible with existing testz framework
