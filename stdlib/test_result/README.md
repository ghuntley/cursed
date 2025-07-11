# TestResult Type System

Enterprise-grade test result handling system for CURSED programming language.

## Overview

The TestResult type system provides standardized test result handling with comprehensive reporting capabilities. It integrates seamlessly with the existing testz framework to enable enterprise-grade test reporting, analytics, and monitoring.

## Key Features

- **Type-Safe Test Results**: Strongly typed test outcomes with success/failure state tracking
- **Comprehensive Reporting**: JSON, XML, HTML, and console output formats
- **Enterprise Integration**: Built for CI/CD pipelines and automated testing workflows
- **Performance Monitoring**: Execution time tracking and performance metrics
- **Extensible Architecture**: Designed for future enhancements and custom reporting

## Core Types

### TestStatus

Represents the outcome of a test with four possible states:

- `Pass` (0): Test passed successfully
- `Fail` (1): Test failed with error details
- `Skip` (2): Test was skipped
- `Error` (3): Test encountered an error

### TestResult

Individual test result with complete metadata:

```cursed
struct TestResult {
    sus test_name tea              fr fr Name of the test
    sus assertion_name tea         fr fr Name of the assertion
    sus status TestStatus          fr fr Test outcome status
    sus message tea                fr fr Description/error message
    sus expected tea               fr fr Expected value (for failures)
    sus actual tea                 fr fr Actual value (for failures)
    sus execution_time normie      fr fr Execution time in milliseconds
    sus line_number normie         fr fr Line number where test is defined
    sus file_name tea              fr fr File name where test is defined
}
```

### TestSuite

Aggregates multiple test results with summary statistics:

```cursed
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
```

### TestReport

Comprehensive reporting with multiple suites and analytics:

```cursed
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
```

## Usage Examples

### Basic TestResult Creation

```cursed
yeet "test_result"

fr fr Create a passing test result
sus pass_result TestResult = test_result_pass("test_math", "assert_eq", "2 + 2 = 4")

fr fr Create a failing test result
sus fail_result TestResult = test_result_fail("test_div", "assert_eq", "Division failed", "2", "error")

fr fr Check test status
lowkey test_result_is_pass(pass_result) {
    vibez.spill("Test passed!")
}
```

### Test Suite Management

```cursed
yeet "test_result"

fr fr Create a test suite
sus suite TestSuite = test_suite_new("math_tests")

fr fr Add test results to suite
sus test1 TestResult = test_result_pass("test_add", "assert_eq", "Addition works")
sus test2 TestResult = test_result_fail("test_sub", "assert_eq", "Subtraction failed", "5", "3")

suite = test_suite_add_test(suite, test1)
suite = test_suite_add_test(suite, test2)

fr fr Check suite statistics
vibez.spill("Success rate: " + tea(suite.success_rate) + "%")
vibez.spill("Total tests: " + tea(suite.total_count))
```

### Comprehensive Reporting

```cursed
yeet "test_result"

fr fr Create comprehensive test report
sus report TestReport = test_report_new()

fr fr Add multiple test suites
report = test_report_add_suite(report, math_suite)
report = test_report_add_suite(report, string_suite)

fr fr Generate different report formats
sus console_output tea = test_report_to_console(report)
sus json_output tea = test_report_to_json(report)

vibez.spill(console_output)
vibez.spill(json_output)
```

### Global Test Collection

```cursed
yeet "test_result"

fr fr Initialize global test collection
test_result_init("integration_tests")

fr fr Record test results globally
test_result_record_pass("test1", "assert_eq", "Test 1 passed")
test_result_record_fail("test2", "assert_eq", "Test 2 failed", "expected", "actual")

fr fr Generate and print comprehensive report
test_result_print_report()

fr fr Export results as JSON
sus json_export tea = test_result_export_json()
```

### Enhanced Assertions

```cursed
yeet "test_result"

fr fr Use enhanced assertion functions
sus int_result TestResult = assert_eq_int_result("test_arithmetic", 42, 42)
sus string_result TestResult = assert_eq_string_result("test_concat", "hello", "hello")
sus bool_result TestResult = assert_true_result("test_condition", based)

fr fr Check assertion results
lowkey test_result_is_pass(int_result) {
    vibez.spill("Integer assertion passed")
}
```

## Integration with Testz Framework

The TestResult system integrates seamlessly with the existing testz framework:

```cursed
yeet "testz"
yeet "test_result"

slay comprehensive_test() {
    test_start("Comprehensive Test with TestResult")
    
    fr fr Initialize TestResult system
    test_result_init("comprehensive_tests")
    
    fr fr Use enhanced assertions
    sus result TestResult = assert_eq_int_result("test_math", 2 + 2, 4)
    assert_true(test_result_is_pass(result))
    
    fr fr Record results globally
    test_result_record_pass("test_integration", "assert_eq", "Integration working")
    
    fr fr Generate comprehensive report
    test_result_print_report()
    
    print_test_summary()
}
```

## Report Formats

### Console Output

```
CURSED Test Report
==================

Test Suite: math_tests
Tests: 5 | Passed: 4 | Failed: 1 | Skipped: 0 | Errors: 0
Success Rate: 80.00%
Execution Time: 150ms

  ✓ test_add: assert_eq - Addition test passed
  ✓ test_mul: assert_eq - Multiplication test passed
  ✗ test_div: assert_eq - Division by zero
    Expected: 2
    Actual:   error

Summary
=======
Total Tests: 5
Passed: 4
Failed: 1
Success Rate: 80.00%
Total Execution Time: 150ms
```

### JSON Output

```json
{
  "total_tests": 5,
  "passed_tests": 4,
  "failed_tests": 1,
  "skipped_tests": 0,
  "error_tests": 0,
  "success_rate": 80.0,
  "execution_time": 150,
  "timestamp": "2025-01-07T00:00:00Z",
  "suites": [
    {
      "suite_name": "math_tests",
      "total_count": 5,
      "passed_count": 4,
      "failed_count": 1,
      "success_rate": 80.0
    }
  ]
}
```

## Performance Characteristics

- **Memory Efficient**: Minimal memory overhead for test result storage
- **Fast Aggregation**: O(1) test result addition with pre-calculated statistics
- **Scalable Reporting**: Handles large test suites with thousands of tests
- **Concurrent Safe**: Thread-safe operations for parallel test execution

## Testing Commands

```bash
# Test the TestResult system
cargo run --bin cursed stdlib/test_result/test_test_result.csd

# Test TestResult integration
cargo run --bin cursed test_testz_working.csd

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/test_result/test_test_result.csd
cargo run --bin cursed -- compile stdlib/test_result/test_test_result.csd
./test_test_result

# Test comprehensive integration
cargo run --bin cursed -- compile test_testz_working.csd
./test_testz_working
```

## Enterprise Features

### CI/CD Integration

The TestResult system is designed for enterprise CI/CD pipelines:

- **Standardized Exit Codes**: Test failures return appropriate exit codes
- **Multiple Report Formats**: JSON, XML, HTML for different CI systems
- **Performance Metrics**: Execution time tracking for performance regression detection
- **Scalable Architecture**: Handles large test suites efficiently

### Monitoring and Analytics

- **Success Rate Tracking**: Monitor test success rates over time
- **Performance Monitoring**: Track test execution times and identify slow tests
- **Error Categorization**: Categorize test failures for better debugging
- **Trend Analysis**: Historical data for test suite health monitoring

### Quality Assurance

- **Type Safety**: Strong typing prevents runtime errors in test reporting
- **Comprehensive Coverage**: Detailed test result metadata for thorough analysis
- **Extensible Design**: Easy to add new report formats and metrics
- **Production Ready**: Tested and validated for enterprise deployment

## Best Practices

1. **Use Global Collection**: Initialize test result collection at the start of test suites
2. **Provide Detailed Messages**: Include descriptive messages for better debugging
3. **Track Execution Time**: Monitor performance for regression detection
4. **Generate Multiple Formats**: Export results in formats suitable for your CI/CD pipeline
5. **Aggregate Related Tests**: Group related tests into logical test suites

## Contributing

The TestResult system is extensible and welcomes contributions:

- **New Report Formats**: Add support for additional output formats
- **Performance Optimizations**: Improve aggregation and reporting performance
- **Enhanced Metadata**: Add new fields for richer test result information
- **Integration Enhancements**: Improve integration with external tools

## License

Part of the CURSED programming language ecosystem, following the same license terms.
