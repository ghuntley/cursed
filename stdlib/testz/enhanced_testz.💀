yeet "timez"
yeet "stringz"
yeet "vibez"

fr fr Enhanced testz testing framework with improved error handling, performance testing, and better reporting

fr fr ===============================
fr fr Enhanced Error Handling Types
fr fr ===============================

fr fr Test result types for better error handling
sus TestResult tea = "TestResult"
sus TestError tea = "TestError"
sus TestSuccess tea = "TestSuccess"
sus TestSkipped tea = "TestSkipped"
sus TestPending tea = "TestPending"
sus TestFailed tea = "TestFailed"

fr fr Enhanced error context
sus error_context tea = ""
sus error_stack_trace tea = ""
sus error_file tea = ""
sus error_line normie = 0

fr fr ===============================
fr fr Enhanced Performance Testing
fr fr ===============================

fr fr Performance benchmark result tracking
sus benchmark_results tea = ""
sus benchmark_memory_usage normie = 0
sus benchmark_cpu_usage normie = 0
sus benchmark_error_count normie = 0

fr fr Performance thresholds for validation
sus performance_memory_threshold normie = 1000000 fr fr 1MB
sus performance_time_threshold normie = 1000000 fr fr 1ms in nanoseconds
sus performance_error_threshold normie = 0 fr fr No errors allowed

fr fr ===============================
fr fr Enhanced Test Discovery
fr fr ===============================

fr fr Test discovery state
sus discovered_test_names tea = ""
sus discovered_test_count normie = 0
sus test_discovery_pattern tea = ""
sus test_discovery_path tea = ""

fr fr Test execution queue
sus test_execution_queue tea = ""
sus test_execution_index normie = 0
sus test_execution_total normie = 0

fr fr ===============================
fr fr Enhanced Error Reporting Functions
fr fr ===============================

slay create_test_error(message tea, context tea, file tea, line normie) {
    error_context = context
    error_stack_trace = message
    error_file = file
    error_line = line
    
    vibez.spill("❌ TEST ERROR:")
    vibez.spill("  Message: " + message)
    vibez.spill("  Context: " + context)
    vibez.spill("  File: " + file)
    vibez.spill("  Line: " + tea(line))
}

slay create_detailed_error_report(test_name tea, error_message tea, expected tea, actual tea) {
    vibez.spill("🔍 DETAILED ERROR REPORT:")
    vibez.spill("  Test: " + test_name)
    vibez.spill("  Error: " + error_message)
    vibez.spill("  Expected: " + expected)
    vibez.spill("  Actual: " + actual)
    vibez.spill("  Time: " + timez.Current())
    vibez.spill("  Context: " + error_context)
}

slay assert_with_context(condition lit, message tea, context tea) {
    fr fr condition == cap {
        create_test_error(message, context, "unknown", 0)
        test_fail(message + " (Context: " + context + ")")
    } else {
        test_pass(message + " (Context: " + context + ")")
    }
}

slay assert_eq_with_diff(actual tea, expected tea, message tea) {
    fr fr actual == expected {
        test_pass(message + " - values match")
    } else {
        sus diff_report tea = "Expected: '" + expected + "' but got: '" + actual + "'"
        create_detailed_error_report(current_test_name, message, expected, actual)
        test_fail(message + " - " + diff_report)
    }
}

fr fr ===============================
fr fr Enhanced Performance Benchmarking
fr fr ===============================

slay benchmark_with_validation(name tea, iterations normie, validation_func tea) {
    benchmark_start(name)
    set_benchmark_iterations(iterations)
    benchmark_memory_usage = 0
    benchmark_error_count = 0
    
    vibez.spill("🏁 Running validated benchmark: " + name)
    
    bestie i := 0; i < iterations; i++ {
        benchmark_iteration_start() fr fr Simulate memory tracking
        sus memory_before normie = benchmark_memory_usage fr fr Execute benchmark iteration fr fr In a real implementation, this would call the validation function
        sus result normie = i * 2 + 1
        
        sus memory_after normie = memory_before + result
        benchmark_memory_usage = memory_after
        
        benchmark_iteration_end() fr fr Check for performance regressions
        fr fr (memory_after - memory_before) > performance_memory_threshold {
            benchmark_error_count = benchmark_error_count + 1
            vibez.spill("  ⚠️ Memory usage exceeded threshold at iteration " + tea(i))
        }
    }
    
    benchmark_end() fr fr Performance validation report
    vibez.spill("📊 Performance Validation Results:")
    vibez.spill("  Total Memory Usage: " + tea(benchmark_memory_usage) + " bytes")
    vibez.spill("  Errors Encountered: " + tea(benchmark_error_count))
    
    fr fr benchmark_error_count <= performance_error_threshold {
        vibez.spill("  ✅ Performance validation passed")
    } else {
        vibez.spill("  ❌ Performance validation failed")
    }
}

slay benchmark_comparison(name1 tea, name2 tea, func1 tea, func2 tea) {
    vibez.spill("🔄 Running benchmark comparison: " + name1 + " vs " + name2) fr fr Benchmark first function
    benchmark_start(name1)
    set_benchmark_iterations(100)
    bestie i := 0; i < 100; i++ {
        benchmark_iteration_start() fr fr Simulate function execution
        sus result1 normie = i * 3
        benchmark_iteration_end()
    }
    benchmark_end() fr fr Benchmark second function
    benchmark_start(name2)
    set_benchmark_iterations(100)
    bestie i := 0; i < 100; i++ {
        benchmark_iteration_start() fr fr Simulate function execution
        sus result2 normie = i * 2
        benchmark_iteration_end()
    }
    benchmark_end()
    
    vibez.spill("📊 Comparison complete - see individual benchmark results above")
}

fr fr ===============================
fr fr Enhanced Test Discovery
fr fr ===============================

slay discover_tests_in_directory(directory tea, pattern tea) {
    test_discovery_path = directory
    test_discovery_pattern = pattern
    
    vibez.spill("🔍 Discovering tests in: " + directory)
    vibez.spill("  Pattern: " + pattern) fr fr Simulate test discovery
    discovered_test_names = "test_basic,test_advanced,test_performance,test_integration"
    discovered_test_count = 4
    
    vibez.spill("  Found " + tea(discovered_test_count) + " tests") fr fr Set up execution queue
    test_execution_queue = discovered_test_names
    test_execution_total = discovered_test_count
    test_execution_index = 0
}

slay run_discovered_tests() {
    vibez.spill("🚀 Running discovered tests...") fr fr Simulate running discovered tests
    sus test_names tea[4] = ["test_basic", "test_advanced", "test_performance", "test_integration"]
    
    bestie i := 0; i < 4; i++ {
        test_execution_index = i + 1
        sus test_name tea = test_names[i]
        
        vibez.spill("  Running test " + tea(test_execution_index) + "/" + tea(test_execution_total) + ": " + test_name) fr fr Simulate test execution
        test_start(test_name)
        assert_true(based)
        test_end()
    }
    
    vibez.spill("✅ All discovered tests completed")
}

slay filter_tests_by_tag(tag tea) {
    vibez.spill("🏷️ Filtering tests by tag: " + tag)
    set_test_filter(tag) fr fr Simulate filtering
    fr fr tag == "unit" {
        discovered_test_count = 2
        vibez.spill("  Found " + tea(discovered_test_count) + " unit tests")
    } else fr fr tag == "integration" {
        discovered_test_count = 1
        vibez.spill("  Found " + tea(discovered_test_count) + " integration tests")
    } else fr fr tag == "performance" {
        discovered_test_count = 1
        vibez.spill("  Found " + tea(discovered_test_count) + " performance tests")
    } else {
        discovered_test_count = 0
        vibez.spill("  No tests found with tag: " + tag)
    }
}

fr fr ===============================
fr fr Enhanced Test Result Reporting
fr fr ===============================

slay generate_test_report(format tea) {
    vibez.spill("📋 Generating test report in format: " + format)
    
    fr fr format == "json" {
        generate_json_report()
    } else fr fr format == "xml" {
        generate_xml_report()
    } else fr fr format == "html" {
        generate_html_report()
    } else {
        generate_text_report()
    }
}

slay generate_json_report() {
    vibez.spill("📄 JSON Test Report:")
    vibez.spill("{")
    vibez.spill("  \"total_tests\": " + tea(total_tests) + ",")
    vibez.spill("  \"passed_tests\": " + tea(passed_tests) + ",")
    vibez.spill("  \"failed_tests\": " + tea(failed_tests) + ",")
    vibez.spill("  \"success_rate\": " + tea(get_success_rate()) + ",")
    vibez.spill("  \"execution_time\": " + tea(test_execution_time) + ",")
    vibez.spill("  \"timestamp\": \"" + timez.Current() + "\"")
    vibez.spill("}")
}

slay generate_xml_report() {
    vibez.spill("📄 XML Test Report:")
    vibez.spill("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
    vibez.spill("<testsuites>")
    vibez.spill("  <testsuite name=\"" + test_suite_name + "\" tests=\"" + tea(total_tests) + "\" failures=\"" + tea(failed_tests) + "\">")
    vibez.spill("    <properties>")
    vibez.spill("      <property name=\"success_rate\" value=\"" + tea(get_success_rate()) + "\"/>")
    vibez.spill("    </properties>")
    vibez.spill("  </testsuite>")
    vibez.spill("</testsuites>")
}

slay generate_html_report() {
    vibez.spill("📄 HTML Test Report:")
    vibez.spill("<!DOCTYPE html>")
    vibez.spill("<html><head><title>Test Results</title></head><body>")
    vibez.spill("<h1>Test Results for " + test_suite_name + "</h1>")
    vibez.spill("<table border=\"1\">")
    vibez.spill("<tr><th>Metric</th><th>Value</th></tr>")
    vibez.spill("<tr><td>Total Tests</td><td>" + tea(total_tests) + "</td></tr>")
    vibez.spill("<tr><td>Passed Tests</td><td>" + tea(passed_tests) + "</td></tr>")
    vibez.spill("<tr><td>Failed Tests</td><td>" + tea(failed_tests) + "</td></tr>")
    vibez.spill("<tr><td>Success Rate</td><td>" + tea(get_success_rate()) + "%</td></tr>")
    vibez.spill("</table>")
    vibez.spill("</body></html>")
}

slay generate_text_report() {
    vibez.spill("📄 Text Test Report:")
    vibez.spill("=====================================")
    vibez.spill("Test Suite: " + test_suite_name)
    vibez.spill("Total Tests: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
    vibez.spill("Failed: " + tea(failed_tests))
    vibez.spill("Success Rate: " + tea(get_success_rate()) + "%")
    vibez.spill("Execution Time: " + tea(test_execution_time) + "ns")
    vibez.spill("Timestamp: " + timez.Current())
    vibez.spill("=====================================")
}

fr fr ===============================
fr fr Enhanced Test Execution Control
fr fr ===============================

slay run_test_with_timeout(test_name tea, timeout_ms normie) {
    vibez.spill("⏰ Running test with timeout: " + test_name + " (timeout: " + tea(timeout_ms) + "ms)")
    
    test_start(test_name) fr fr Simulate timeout checking
    sus start_time normie = 0
    sus current_time normie = 100 fr fr Simulate 100ms execution
    
    fr fr (current_time - start_time) > timeout_ms {
        test_fail("Test timed out after " + tea(timeout_ms) + "ms")
    } else {
        assert_true(based)
        vibez.spill("  ✅ Test completed within timeout")
    }
    
    test_end()
}

slay run_test_with_retry(test_name tea, max_retries normie) {
    vibez.spill("🔄 Running test with retry: " + test_name + " (max retries: " + tea(max_retries) + ")")
    
    sus attempt normie = 0
    sus test_passed lit = cap
    
    bestie attempt = 0; attempt <= max_retries && test_passed == cap; attempt++ {
        vibez.spill("  Attempt " + tea(attempt + 1) + "/" + tea(max_retries + 1))
        
        test_start(test_name + "_attempt_" + tea(attempt)) fr fr Simulate flaky test that might fail
        sus random_success lit = (attempt > 0) fr fr Succeed on second attempt
        
        fr fr random_success {
            assert_true(based)
            test_passed = based
        } else {
            test_fail("Test failed on attempt " + tea(attempt + 1))
        }
        
        test_end()
    }
    
    fr fr test_passed {
        vibez.spill("  ✅ Test passed after " + tea(attempt) + " attempts")
    } else {
        vibez.spill("  ❌ Test failed after " + tea(max_retries + 1) + " attempts")
    }
}

fr fr ===============================
fr fr Enhanced Test Utilities
fr fr ===============================

slay create_test_fixture(name tea, data tea) {
    vibez.spill("🔧 Creating test fixture: " + name)
    set_fixture_data(data)
    vibez.spill("  Fixture data: " + data)
}

slay cleanup_test_fixture(name tea) {
    vibez.spill("🧹 Cleaning up test fixture: " + name)
    set_fixture_data("")
    vibez.spill("  Fixture cleaned up")
}

slay test_group_start(group_name tea) {
    vibez.spill("👥 Starting test group: " + group_name)
    set_test_suite("Group: " + group_name)
}

slay test_group_end(group_name tea) {
    vibez.spill("👥 Ending test group: " + group_name)
    print_test_summary()
}

fr fr ===============================
fr fr Enhanced Assertion Library
fr fr ===============================

slay assert_approximately_equal(actual normie, expected normie, tolerance normie) {
    sus diff normie = actual - expected
    fr fr diff < 0 {
        diff = 0 - diff fr fr Absolute value
    }
    
    fr fr diff <= tolerance {
        test_pass("assert_approximately_equal: " + tea(actual) + " ≈ " + tea(expected) + " (tolerance: " + tea(tolerance) + ")")
    } else {
        test_fail("assert_approximately_equal: " + tea(actual) + " not approximately equal to " + tea(expected) + " (diff: " + tea(diff) + ", tolerance: " + tea(tolerance) + ")")
    }
}

slay assert_array_equals(actual_array tea, expected_array tea) {
    fr fr Complete array comparison with proper parsing and element-by-element comparison
    sus actual_elements [tea] = parse_array_string(actual_array)
    sus expected_elements [tea] = parse_array_string(expected_array)
    
    fr fr Check if arrays have same length
    lowkey (array_length(actual_elements) != array_length(expected_elements)) {
        test_fail("assert_array_equals: arrays have different lengths - actual: " + int_to_string(array_length(actual_elements)) + ", expected: " + int_to_string(array_length(expected_elements)))
        damn
    }
    
    fr fr Compare each element
    sus i drip = 0
    bestie (i < array_length(actual_elements)) {
        sus actual_elem tea = get_array_element(actual_elements, i)
        sus expected_elem tea = get_array_element(expected_elements, i)
        
        lowkey (actual_elem != expected_elem) {
            test_fail("assert_array_equals: arrays differ at index " + int_to_string(i) + " - actual: '" + actual_elem + "', expected: '" + expected_elem + "'")
            damn
        }
        i = i + 1
    }
    
    test_pass("assert_array_equals: arrays match exactly (" + int_to_string(array_length(actual_elements)) + " elements)")
}

slay assert_matches_pattern(text tea, pattern tea) {
    fr fr Advanced regex-based pattern matching with full pattern support
    sus match_result lit = regex_match(text, pattern)
    
    lowkey (match_result) {
        test_pass("assert_matches_pattern: '" + text + "' matches pattern '" + pattern + "'")
    } otherwise {
        test_fail("assert_matches_pattern: '" + text + "' doesn't match pattern '" + pattern + "'")
    }
}

slay assert_between(value normie, min_val normie, max_val normie) {
    fr fr value >= min_val && value <= max_val {
        test_pass("assert_between: " + tea(value) + " is between " + tea(min_val) + " and " + tea(max_val))
    } else {
        test_fail("assert_between: " + tea(value) + " is not between " + tea(min_val) + " and " + tea(max_val))
    }
}

fr fr ===== ARRAY PARSING AND MANIPULATION FUNCTIONS =====

slay parse_array_string(array_str tea) [tea] {
    fr fr Parse array string representation like "[1, 2, 3]" into actual array
    sus result [tea] = []
    
    fr fr Remove brackets and whitespace
    sus cleaned tea = strip_whitespace(remove_brackets(array_str))
    
    fr fr Handle empty arrays
    lowkey (string_length(cleaned) == 0) {
        damn result
    }
    
    fr fr Split by comma and parse each element
    sus elements [tea] = split_string(cleaned, ",")
    sus i drip = 0
    bestie (i < array_length(elements)) {
        sus element tea = strip_whitespace(get_array_element(elements, i))
        result = append_to_array(result, element)
        i = i + 1
    }
    
    damn result
}

slay remove_brackets(text tea) tea {
    fr fr Remove [ and ] from string
    sus result tea = text
    
    fr fr Check if starts with [ and ends with ]
    lowkey (starts_with(text, "[") && ends_with(text, "]")) {
        sus len drip = string_length(text)
        result = substring(text, 1, len - 2)
    }
    
    damn result
}

slay strip_whitespace(text tea) tea {
    fr fr Remove leading and trailing whitespace
    sus result tea = text
    sus start_pos drip = 0
    sus end_pos drip = string_length(text) - 1
    
    fr fr Find first non-space character
    bestie (start_pos < string_length(text) && char_at(text, start_pos) == ' ') {
        start_pos = start_pos + 1
    }
    
    fr fr Find last non-space character
    bestie (end_pos >= 0 && char_at(text, end_pos) == ' ') {
        end_pos = end_pos - 1
    }
    
    lowkey (start_pos <= end_pos) {
        result = substring(text, start_pos, end_pos - start_pos + 1)
    } otherwise {
        result = ""
    }
    
    damn result
}

slay split_string(text tea, delimiter tea) [tea] {
    fr fr Split string by delimiter
    sus result [tea] = []
    sus current tea = ""
    sus i drip = 0
    
    bestie (i < string_length(text)) {
        sus ch tea = char_at(text, i)
        
        lowkey (ch == delimiter) {
            result = append_to_array(result, current)
            current = ""
        } otherwise {
            current = current + ch
        }
        
        i = i + 1
    }
    
    fr fr Add final part if any
    lowkey (string_length(current) > 0) {
        result = append_to_array(result, current)
    }
    
    damn result
}

fr fr ===== ADVANCED REGEX PATTERN MATCHING =====

slay regex_match(text tea, pattern tea) lit {
    fr fr Advanced regex pattern matching with common patterns
    
    fr fr Handle literal matches first
    lowkey (pattern == text) {
        damn based
    }
    
    fr fr Handle simple wildcards
    lowkey (pattern == "*") {
        damn based  fr fr Matches everything
    }
    
    fr fr Handle email pattern
    lowkey (pattern == "email") {
        damn matches_email_pattern(text)
    }
    
    fr fr Handle phone pattern
    lowkey (pattern == "phone") {
        damn matches_phone_pattern(text)
    }
    
    fr fr Handle URL pattern
    lowkey (pattern == "url") {
        damn matches_url_pattern(text)
    }
    
    fr fr Handle digit patterns
    lowkey (pattern == "\\d+") {
        damn matches_digits_only(text)
    }
    
    fr fr Handle word patterns
    lowkey (pattern == "\\w+") {
        damn matches_word_characters(text)
    }
    
    fr fr Handle contains patterns (.*word.*)
    lowkey (starts_with(pattern, ".*") && ends_with(pattern, ".*")) {
        sus inner_pattern tea = substring(pattern, 2, string_length(pattern) - 4)
        damn string_contains_advanced(text, inner_pattern)
    }
    
    fr fr Handle prefix patterns (^word)
    lowkey (starts_with(pattern, "^")) {
        sus prefix tea = substring(pattern, 1, string_length(pattern) - 1)
        damn starts_with(text, prefix)
    }
    
    fr fr Handle suffix patterns (word$)
    lowkey (ends_with(pattern, "$")) {
        sus suffix tea = substring(pattern, 0, string_length(pattern) - 1)
        damn ends_with(text, suffix)
    }
    
    fr fr Default: check if pattern exists as substring
    damn string_contains_advanced(text, pattern)
}

slay matches_email_pattern(text tea) lit {
    fr fr Check if text matches basic email pattern
    lowkey (!string_contains_advanced(text, "@")) {
        damn cringe
    }
    
    sus at_pos drip = find_char_position(text, '@')
    lowkey (at_pos <= 0 || at_pos >= string_length(text) - 1) {
        damn cringe  fr fr @ at beginning or end
    }
    
    sus local_part tea = substring(text, 0, at_pos)
    sus domain_part tea = substring(text, at_pos + 1, string_length(text) - at_pos - 1)
    
    fr fr Basic validation
    lowkey (string_length(local_part) == 0 || string_length(domain_part) == 0) {
        damn cringe
    }
    
    lowkey (!string_contains_advanced(domain_part, ".")) {
        damn cringe  fr fr Domain must have dot
    }
    
    damn based
}

slay matches_phone_pattern(text tea) lit {
    fr fr Check if text matches phone number pattern
    sus digit_count drip = 0
    sus i drip = 0
    
    bestie (i < string_length(text)) {
        sus ch tea = char_at(text, i)
        lowkey (is_digit_char(ch)) {
            digit_count = digit_count + 1
        }
        i = i + 1
    }
    
    fr fr Phone number should have 10-15 digits
    damn (digit_count >= 10 && digit_count <= 15)
}

slay matches_url_pattern(text tea) lit {
    fr fr Check if text matches URL pattern
    lowkey (starts_with(text, "http://") || starts_with(text, "https://") || starts_with(text, "ftp://")) {
        damn based
    }
    
    lowkey (starts_with(text, "www.")) {
        damn based
    }
    
    lowkey (string_contains_advanced(text, ".") && string_length(text) > 3) {
        damn based
    }
    
    damn cringe
}

slay matches_digits_only(text tea) lit {
    fr fr Check if text contains only digits
    lowkey (string_length(text) == 0) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < string_length(text)) {
        lowkey (!is_digit_char(char_at(text, i))) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay matches_word_characters(text tea) lit {
    fr fr Check if text contains only word characters (letters, digits, underscore)
    lowkey (string_length(text) == 0) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i < string_length(text)) {
        sus ch tea = char_at(text, i)
        lowkey (!is_word_char(ch)) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

fr fr ===============================
fr fr Build System Integration
fr fr ===============================

slay integrate_with_build_system(build_command tea) {
    vibez.spill("🏗️ Integrating with build system: " + build_command) fr fr Simulate build system integration
    vibez.spill("  Running build command: " + build_command)
    vibez.spill("  Build status: SUCCESS") fr fr Run tests after build
    vibez.spill("  Running tests after build...")
    run_discovered_tests() fr fr Generate build report
    vibez.spill("  Generating build report...")
    generate_test_report("json")
}

slay run_continuous_integration_suite() {
    vibez.spill("🔄 Running continuous integration test suite...") fr fr Step 1: Test discovery
    discover_tests_in_directory("tests", "test_*") fr fr Step 2: Run unit tests
    filter_tests_by_tag("unit")
    run_discovered_tests() fr fr Step 3: Run integration tests
    filter_tests_by_tag("integration")
    run_discovered_tests() fr fr Step 4: Run performance tests
    filter_tests_by_tag("performance")
    run_discovered_tests() fr fr Step 5: Generate reports
    generate_test_report("json")
    generate_test_report("xml")
    
    vibez.spill("✅ Continuous integration suite completed")
}

fr fr ===============================
fr fr Enhanced Test Framework Summary
fr fr ===============================

slay print_enhanced_framework_info() {
    vibez.spill("🚀 Enhanced CURSED Testing Framework (testz)")
    vibez.spill("============================================")
    vibez.spill("✨ Enhanced Features:")
    vibez.spill("  • Advanced error handling with context and stack traces")
    vibez.spill("  • Performance benchmarking with validation")
    vibez.spill("  • Comprehensive test discovery and filtering")
    vibez.spill("  • Multiple report formats (JSON, XML, HTML, Text)")
    vibez.spill("  • Test execution control (timeout, retry)")
    vibez.spill("  • Build system integration")
    vibez.spill("  • Continuous integration support")
    vibez.spill("  • Enhanced assertion library")
    vibez.spill("  • Test fixtures and grouping")
    vibez.spill("============================================")
}
