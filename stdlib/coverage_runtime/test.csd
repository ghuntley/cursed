yeet "testz"
yeet "coverage_runtime"

test_start("coverage_runtime Tests")

// Test Basic Coverage Recording
test_case("Basic Line Coverage Recording") {
    // Clear any existing coverage data
    clearCoverage()
    
    sus test_file tea = "test_module.csd"
    
    // Record some line executions
    recordLine(test_file, 10)
    recordLine(test_file, 15)
    recordLine(test_file, 20)
    recordLine(test_file, 15)  // Duplicate - should not increase count
    
    sus summary {} = getCoverageSummary()
    
    assert_eq_int(summary["total_files"], 1)
    assert(summary["files"][test_file] != nil)
    
    sus file_data {} = summary["files"][test_file]
    assert_eq_int(file_data["lines_covered"], 3)
}

// Test Function Coverage Recording
test_case("Function Coverage Recording") {
    clearCoverage()
    
    sus test_file tea = "functions_module.csd"
    
    // Record function entries
    recordFunction(test_file, "main", 5)
    recordFunction(test_file, "helper", 25)
    recordFunction(test_file, "cleanup", 50)
    
    sus summary {} = getCoverageSummary()
    sus file_data {} = summary["files"][test_file]
    
    assert_eq_int(file_data["functions_covered"], 3)
    
    // Check that function keys are properly stored
    sus coverage_data_file {} = getCoverageData(test_file)
    assert(coverage_data_file["func:main:5"] == based)
    assert(coverage_data_file["func:helper:25"] == based)
    assert(coverage_data_file["func:cleanup:50"] == based)
}

// Test Branch Coverage Recording
test_case("Branch Coverage Recording") {
    clearCoverage()
    
    sus test_file tea = "branches_module.csd"
    
    // Record branch executions
    recordBranch(test_file, "if_1_true", 30, based)
    recordBranch(test_file, "if_1_false", 30, false)
    recordBranch(test_file, "loop_1", 45, based)
    recordBranch(test_file, "match_case_a", 60, based)
    recordBranch(test_file, "match_case_b", 65, false)
    
    sus summary {} = getCoverageSummary()
    sus file_data {} = summary["files"][test_file]
    
    assert_eq_int(file_data["branches_covered"], 5)
    
    // Verify branch data storage
    sus coverage_data_file {} = getCoverageData(test_file)
    assert(coverage_data_file["branch:if_1_true:30"] == based)
    assert(coverage_data_file["branch:if_1_false:30"] == false)
    assert(coverage_data_file["branch:loop_1:45"] == based)
}

// Test Coverage Percentage Calculation
test_case("Coverage Percentage Calculation") {
    clearCoverage()
    
    sus test_file tea = "percentage_test.csd"
    
    // Set total counts for percentage calculation
    setTotalLines(test_file, 100)
    setTotalFunctions(test_file, 20)
    setTotalBranches(test_file, 40)
    
    // Record some coverage
    bestie (sus i normie = 1; i <= 75; i += 1) {
        recordLine(test_file, i)
    }
    
    bestie (sus i normie = 1; i <= 15; i += 1) {
        recordFunction(test_file, "func_" + string_from_int(i), i * 5)
    }
    
    bestie (sus i normie = 1; i <= 30; i += 1) {
        recordBranch(test_file, "branch_" + string_from_int(i), i * 2, based)
    }
    
    sus summary {} = getCoverageSummary()
    sus file_data {} = summary["files"][test_file]
    
    assert_eq_int(file_data["line_coverage_percent"], 75)    // 75/100 = 75%
    assert_eq_int(file_data["function_coverage_percent"], 75) // 15/20 = 75%
    assert_eq_int(file_data["branch_coverage_percent"], 75)   // 30/40 = 75%
}

// Test Multiple Files Coverage
test_case("Multiple Files Coverage") {
    clearCoverage()
    
    // Record coverage for multiple files
    recordLine("file1.csd", 10)
    recordLine("file1.csd", 20)
    recordFunction("file1.csd", "main", 5)
    
    recordLine("file2.csd", 15)
    recordLine("file2.csd", 25)
    recordLine("file2.csd", 35)
    recordFunction("file2.csd", "helper", 12)
    recordBranch("file2.csd", "if_1", 30, based)
    
    recordLine("file3.csd", 5)
    
    sus summary {} = getCoverageSummary()
    
    assert_eq_int(summary["total_files"], 3)
    
    // Check individual file coverage
    sus file1_data {} = summary["files"]["file1.csd"]
    assert_eq_int(file1_data["lines_covered"], 2)
    assert_eq_int(file1_data["functions_covered"], 1)
    
    sus file2_data {} = summary["files"]["file2.csd"]
    assert_eq_int(file2_data["lines_covered"], 3)
    assert_eq_int(file2_data["functions_covered"], 1)
    assert_eq_int(file2_data["branches_covered"], 1)
    
    sus file3_data {} = summary["files"]["file3.csd"]
    assert_eq_int(file3_data["lines_covered"], 1)
}

// Test Coverage Report Generation
test_case("Coverage Report Generation") {
    clearCoverage()
    
    sus test_file tea = "report_test.csd"
    setTotalLines(test_file, 50)
    setTotalFunctions(test_file, 10)
    setTotalBranches(test_file, 20)
    
    // Record partial coverage
    bestie (sus i normie = 1; i <= 40; i += 1) {
        recordLine(test_file, i)
    }
    bestie (sus i normie = 1; i <= 8; i += 1) {
        recordFunction(test_file, "func_" + string_from_int(i), i * 5)
    }
    bestie (sus i normie = 1; i <= 15; i += 1) {
        recordBranch(test_file, "branch_" + string_from_int(i), i * 2, based)
    }
    
    // Generate different report formats
    sus html_report tea = generateHTMLReport()
    assert(string_contains(html_report, "<html>"))
    assert(string_contains(html_report, "report_test.csd"))
    assert(string_contains(html_report, "80%"))  // 40/50 line coverage
    
    sus json_report tea = generateJSONReport()
    assert(string_contains(json_report, "\"total_files\": 1"))
    assert(string_contains(json_report, "\"line_coverage_percent\": 80"))
    
    sus xml_report tea = generateXMLReport()
    assert(string_contains(xml_report, "<coverage>"))
    assert(string_contains(xml_report, "<file name=\"report_test.csd\">"))
}

// Test Coverage Instrumentation Helpers
test_case("Coverage Instrumentation Helpers") {
    clearCoverage()
    
    sus test_file tea = "instrumentation_test.csd"
    
    // Test instrumentation macro helpers
    sus instrumented_line tea = instrumentLine(test_file, 42, "sus x normie = 10")
    assert(string_contains(instrumented_line, "recordLine"))
    assert(string_contains(instrumented_line, "42"))
    assert(string_contains(instrumented_line, "sus x normie = 10"))
    
    sus instrumented_function tea = instrumentFunction(test_file, "testFunc", 25, "slay testFunc() { damn 42 }")
    assert(string_contains(instrumented_function, "recordFunction"))
    assert(string_contains(instrumented_function, "testFunc"))
    assert(string_contains(instrumented_function, "25"))
    
    sus instrumented_branch tea = instrumentBranch(test_file, "if_condition", 30, "ready (x > 0) { damn x }")
    assert(string_contains(instrumented_branch, "recordBranch"))
    assert(string_contains(instrumented_branch, "if_condition"))
    assert(string_contains(instrumented_branch, "30"))
}

// Test Coverage Reset and Clear
test_case("Coverage Reset and Clear") {
    // Set up some coverage data
    recordLine("test1.csd", 10)
    recordLine("test2.csd", 20)
    recordFunction("test1.csd", "main", 5)
    
    sus summary_before {} = getCoverageSummary()
    assert_eq_int(summary_before["total_files"], 2)
    
    // Clear coverage for specific file
    clearFileCoverage("test1.csd")
    
    sus summary_after_clear {} = getCoverageSummary()
    assert_eq_int(summary_after_clear["total_files"], 1)
    assert(summary_after_clear["files"]["test2.csd"] != nil)
    assert(summary_after_clear["files"]["test1.csd"] == nil)
    
    // Clear all coverage
    clearCoverage()
    
    sus summary_final {} = getCoverageSummary()
    assert_eq_int(summary_final["total_files"], 0)
}

// Test Performance with Large Coverage Data
test_case("Performance Test - Large Coverage Dataset") {
    clearCoverage()
    
    sus test_file tea = "performance_test.csd"
    sus line_count normie = 10000
    sus function_count normie = 1000
    sus branch_count normie = 5000
    
    // Record large amount of coverage data
    sus start_time drip = get_current_time_ms()
    
    bestie (sus i normie = 1; i <= line_count; i += 1) {
        recordLine(test_file, i)
    }
    
    bestie (sus i normie = 1; i <= function_count; i += 1) {
        recordFunction(test_file, "func_" + string_from_int(i), i * 10)
    }
    
    bestie (sus i normie = 1; i <= branch_count; i += 1) {
        recordBranch(test_file, "branch_" + string_from_int(i), i * 2, i % 2 == 0)
    }
    
    sus record_time drip = get_current_time_ms() - start_time
    
    // Test summary generation performance
    start_time = get_current_time_ms()
    sus summary {} = getCoverageSummary()
    sus summary_time drip = get_current_time_ms() - start_time
    
    // Verify data integrity
    sus file_data {} = summary["files"][test_file]
    assert_eq_int(file_data["lines_covered"], line_count)
    assert_eq_int(file_data["functions_covered"], function_count)
    assert_eq_int(file_data["branches_covered"], branch_count)
    
    print_test_status("Record time for " + string_from_int(line_count + function_count + branch_count) + " items: " + string_from_int(record_time) + "ms")
    print_test_status("Summary generation time: " + string_from_int(summary_time) + "ms")
    
    // Performance should be reasonable (under 1 second for 16k items)
    assert(record_time < 1000)
    assert(summary_time < 500)
}

print_test_summary()
