# CURSED TestResult Type System

A comprehensive test result handling system for the CURSED programming language that provides type-safe test reporting with multiple output formats.

## Features

- **Type-Safe Test Results**: Robust TestResult enum with Success, Failure, Skip, and Error variants
- **Multiple Serialization Formats**: JSON, XML, HTML, and Console output
- **Enhanced Test Suites**: Aggregation and statistics for test collections
- **Comprehensive Reporting**: Full test reports with metadata and timing
- **Builder Pattern**: Fluent API for creating complex test results
- **Performance Monitoring**: Execution time tracking and benchmarking
- **Integration**: Seamless integration with existing testz framework

## Core Types

### TestResult

Represents the outcome of a single test assertion with comprehensive metadata.

```cursed
sus result TestResult = TestResult.pass("test_name", "assertion_name", "Test passed")
sus fail_result TestResult = TestResult.fail("test_name", "assertion_name", "Test failed", "expected", "actual")
sus skip_result TestResult = TestResult.skip("test_name", "assertion_name", "Test skipped")
sus error_result TestResult = TestResult.error("test_name", "assertion_name", "Test error")
```

### TestSuite

Aggregates multiple test results with statistics and metadata.

```cursed
sus suite TestSuite = TestSuite.new("math_tests")
suite = TestSuite.add_test(suite, result)
suite = TestSuite.add_metadata(suite, "category", "unit_tests")
suite = TestSuite.set_timing(suite, 10, 5, 100)
```

### TestReport

Provides comprehensive reporting across multiple test suites.

```cursed
sus report TestReport = TestReport.new()
report = TestReport.add_suite(report, suite)
report = TestReport.add_metadata(report, "environment", "CI")
```

## Usage Examples

### Basic Test Result Creation

```cursed
yeet "test_result"

fr fr Create different types of test results
sus pass_result TestResult = TestResult.pass("test_math", "assert_eq", "2 + 2 = 4")
sus fail_result TestResult = TestResult.fail("test_div", "assert_eq", "Division failed", "2", "error")
sus skip_result TestResult = TestResult.skip("test_feature", "assert_eq", "Feature not implemented")
sus error_result TestResult = TestResult.error("test_crash", "assert_eq", "Unexpected error")

fr fr Check result status
lowkey TestResult.is_pass(pass_result) {
    vibez.spill("Test passed!")
}
```

### Enhanced Test Results with Metadata

```cursed
sus result TestResult = TestResult.pass("test_enhanced", "assert_eq", "Enhanced test")
result = TestResult.with_execution_time(result, 150)
result = TestResult.with_line_number(result, 42)
result = TestResult.with_file_name(result, "test.csd")
result = TestResult.with_metadata(result, "author", "developer")
```

### Builder Pattern for Complex Results

```cursed
sus result TestResult = TestResultBuilder.new("test_complex", "assert_eq")
    .pass("Complex test passed")
    .expected("expected_value")
    .actual("actual_value")
    .execution_time(100)
    .line_number(25)
    .file_name("complex_test.csd")
    .metadata("complexity", "high")
    .build()
```

### Test Suite Management

```cursed
sus suite TestSuite = TestSuite.new("comprehensive_tests")

fr fr Add test results
suite = TestSuite.add_test(suite, pass_result)
suite = TestSuite.add_test(suite, fail_result)
suite = TestSuite.add_test(suite, skip_result)

fr fr Add metadata
suite = TestSuite.add_metadata(suite, "category", "integration")
suite = TestSuite.add_metadata(suite, "priority", "high")

fr fr Set timing information
suite = TestSuite.set_timing(suite, 50, 30, 200)

fr fr Get statistics
vibez.spill("Total tests: " + tea(TestSuite.total_count(suite)))
vibez.spill("Passed: " + tea(TestSuite.passed_count(suite)))
vibez.spill("Failed: " + tea(TestSuite.failed_count(suite)))
vibez.spill("Success rate: " + tea(TestSuite.success_rate(suite)) + "%")
```

### Comprehensive Test Reporting

```cursed
sus report TestReport = TestReport.new()
report = TestReport.add_suite(report, suite)
report = TestReport.add_metadata(report, "environment", "production")

fr fr Generate different report formats
sus console_report tea = TestReport.to_console(report)
sus json_report tea = TestReport.to_json(report)
sus xml_report tea = TestReport.to_xml(report)
sus html_report tea = TestReport.to_html(report)

fr fr Print console report
vibez.spill(console_report)
```

## Serialization Formats

### JSON Format

```json
{
  "total_tests": 3,
  "passed_tests": 2,
  "failed_tests": 1,
  "skipped_tests": 0,
  "error_tests": 0,
  "success_rate": 66.67,
  "execution_time": 250,
  "timestamp": "2025-01-10T12:00:00Z",
  "cursed_version": "8.1.0",
  "suites": [...]
}
```

### XML Format (JUnit Compatible)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="math_tests" tests="3" failures="1" errors="0" skipped="0" time="250">
    <testcase name="assert_eq" classname="test_addition" time="50">
    </testcase>
    <testcase name="assert_eq" classname="test_division" time="75">
      <failure message="Division failed">Expected: 2, Actual: error</failure>
    </testcase>
  </testsuite>
</testsuites>
```

### HTML Format

```html
<!DOCTYPE html>
<html>
<head>
  <title>CURSED Test Report</title>
  <style>
    .pass { color: green; }
    .fail { color: red; }
    .skip { color: orange; }
    .error { color: purple; }
  </style>
</head>
<body>
  <h1>CURSED Test Report</h1>
  <div class="summary">
    <h2>Summary</h2>
    <p>Total Tests: 3</p>
    <p>Success Rate: 66.67%</p>
  </div>
  <h3>Test Suite: math_tests</h3>
  <table>
    <tr><th>Test</th><th>Assertion</th><th>Status</th><th>Message</th><th>Time</th></tr>
    <tr><td>test_addition</td><td>assert_eq</td><td class="pass">PASS</td><td>Test passed</td><td>50ms</td></tr>
  </table>
</body>
</html>
```

### Console Format

```
CURSED Test Report
==================

Test Suite: math_tests
Tests: 3 | Passed: 2 | Failed: 1 | Skipped: 0 | Errors: 0
Success Rate: 66.67%
Execution Time: 250ms

  ✓ test_addition: assert_eq - Test passed
  ✗ test_division: assert_eq - Division failed
    Expected: 2
    Actual:   error
  ✓ test_multiplication: assert_eq - Test passed

Summary
=======
Total Tests: 3
Passed: 2
Failed: 1
Success Rate: 66.67%
Total Execution Time: 250ms

❌ Some tests failed
```

## Enhanced Assertion Functions

The TestResult system provides enhanced assertion functions that return TestResult objects:

```cursed
sus int_result TestResult = assert_eq_int_result("test_int", 42, 42)
sus string_result TestResult = assert_eq_string_result("test_string", "hello", "hello")
sus bool_result TestResult = assert_eq_bool_result("test_bool", based, based)
sus true_result TestResult = assert_true_result("test_true", based)
sus false_result TestResult = assert_false_result("test_false", cap)
```

## Performance Utilities

### Benchmark Testing

```cursed
slay performance_test() {
    fr fr Some operation to benchmark
    sus sum normie = 0
    bestie i := 0; i < 1000; i++ {
        sum = sum + i
    }
}

sus benchmark_result TestResult = TestResult.benchmark("perf_test", "benchmark", performance_test)
vibez.spill("Execution time: " + tea(benchmark_result.execution_time) + "ms")
```

### Benchmark Suite

```cursed
sus operations []slay() = [operation1, operation2, operation3]
sus benchmark_suite TestSuite = TestSuite.benchmark_suite("performance_tests", operations)
```

## Integration with testz Framework

The TestResult system provides full backward compatibility with the existing testz framework:

```cursed
yeet "testz"
yeet "test_result"

fr fr Use enhanced testing with TestResult
create_test_suite("enhanced_tests")

test_start("enhanced_test_1")
assert_eq_int_enhanced(2 + 2, 4)
assert_eq_string_enhanced("hello", "hello")
assert_true_enhanced(based)

finalize_test_suite()
print_test_report()
```

## Testing Commands

### Run TestResult Tests

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/test_result/test_test_result.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/test_result/test_test_result.csd
./test_test_result

# Test integration with testz
cargo run --bin cursed test_testz_working.csd
```

### Test Type System Integration

```bash
# Test Rust type system integration
cargo test type_system::test_result_simple

# Test full integration
cargo test test_result
```

## Best Practices

1. **Use Type-Safe Results**: Always use TestResult objects for test outcomes
2. **Add Metadata**: Include relevant metadata for debugging and analysis
3. **Track Timing**: Use execution time tracking for performance analysis
4. **Multiple Formats**: Generate reports in multiple formats for different audiences
5. **Builder Pattern**: Use the builder pattern for complex test results
6. **Integration**: Leverage integration with existing testz framework
7. **Performance**: Use benchmark utilities for performance testing

## Configuration

The TestResult system can be configured through metadata:

```cursed
sus report TestReport = TestReport.new()
report = TestReport.add_metadata(report, "environment", "CI")
report = TestReport.add_metadata(report, "branch", "main")
report = TestReport.add_metadata(report, "build_id", "12345")
```

## Error Handling

The TestResult system provides comprehensive error handling:

```cursed
fr fr Handle different error types
sus error_result TestResult = TestResult.error("test_error", "runtime_error", "Unexpected panic")
error_result = TestResult.with_metadata(error_result, "error_type", "panic")
error_result = TestResult.with_metadata(error_result, "stack_trace", "trace_info")
```

## Thread Safety

The TestResult system is designed to be thread-safe for concurrent testing:

```cursed
fr fr Each test result is independent
sus results []TestResult = []
yolo {
    results.append(TestResult.pass("concurrent_test_1", "assert_eq", "Concurrent test"))
}
yolo {
    results.append(TestResult.pass("concurrent_test_2", "assert_eq", "Concurrent test"))
}
```

## Production Deployment

For production deployment, use the TestResult system with:

1. **Comprehensive Reporting**: Generate multiple report formats
2. **Performance Monitoring**: Track execution times and resource usage
3. **Metadata Collection**: Include environment and build information
4. **Error Classification**: Categorize different types of test failures
5. **Integration**: Connect with CI/CD pipelines and monitoring systems

## Version History

- **v1.0.0**: Initial release with comprehensive TestResult type system
- **v1.1.0**: Added performance utilities and benchmark support
- **v1.2.0**: Enhanced serialization formats and metadata support
- **v1.3.0**: Improved integration with testz framework
- **v1.4.0**: Added builder pattern and fluent API
- **v1.5.0**: Performance optimizations and thread safety improvements

## License

This TestResult system is part of the CURSED programming language and is distributed under the same license as the CURSED compiler.
