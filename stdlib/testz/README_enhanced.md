# CURSED Testing Framework v3.0 - Enhanced Edition

The ultimate Gen Z testing framework for CURSED language with ALL the features you need for comprehensive stdlib testing.

## 🚀 Features

### Core Testing Capabilities
- **Basic Assertions**: Integer, string, boolean, float comparisons
- **Advanced Assertions**: Range checking, not-equal, greater/less than
- **Test Lifecycle Management**: Start/end, suites, skipping
- **State Management**: Automatic test counting and statistics

### 🔥 Enhanced Features
- **Property-Based Testing**: Random test generation for comprehensive coverage
- **Benchmarking**: Performance testing with min/max/average metrics
- **Coverage Analysis**: Code coverage reporting and gap identification
- **Multiple Output Formats**: JSON, XML, HTML, TAP for CI/CD integration
- **Mock System**: Simple but effective mocking capabilities
- **Parallel Execution**: Run tests concurrently for speed
- **Performance Regression Testing**: Detect performance degradation
- **Test Discovery**: Automatic test file discovery and execution
- **Configuration Management**: Flexible framework configuration
- **Advanced Reporting**: Comprehensive test summaries with metrics

## 📦 Installation

```bash
# Include in your CURSED program
yeet "testz/mod_enhanced_simple"
```

## 🧪 Basic Usage

```cursed
yeet "testz/mod_enhanced_simple"

# Start a test suite
suite_start("My Test Suite")

# Basic test
test_start("basic_math")
assert_eq_int(2 + 2, 4)
assert_gt_int(5, 3)
assert_in_range(7, 1, 10)
test_end()

# String tests
test_start("string_operations")
assert_eq_string("hello", "hello")
assert_true("hello".length > 0)
test_end()

# Finish suite
suite_end()

# Generate report
print_test_summary()
```

## 🏃 Performance Testing

```cursed
# Benchmark a function
benchmark_function("my_algorithm", 100)

# Check for performance regression
check_performance_regression("sorting", 150, 100, 50)

# Manual benchmarking
sus start_time normie = benchmark_start()
# ... your code here ...
sus duration normie = benchmark_end(start_time)
```

## 🎲 Property-Based Testing

```cursed
# Test integer properties
property_test_int("range_property", 1, 100, 50)

# Test with custom ranges
property_test_int("negative_numbers", -100, -1, 25)
```

## 🎭 Mock System

```cursed
# Create and configure mocks
sus mock_id normie = create_mock("api_service")
mock_return(mock_id, "mocked_response")
mock_verify_called(mock_id, 1)
```

## 🔍 Test Discovery

```cursed
# Discover and run tests from directory
run_test_suite("tests/")

# Check how many files were found
sus files_found normie = discover_test_files("tests/")
```

## 📊 Output Formats

```cursed
# Enable multiple output formats
enable_json_output()
enable_xml_output()
enable_html_output()
enable_tap_output()

# Generate reports
generate_json_report()
generate_xml_report()
generate_html_report()
generate_tap_report()
```

## 📈 Coverage Analysis

```cursed
# Enable coverage tracking
enable_coverage()

# Generate coverage reports
analyze_coverage()
report_coverage_gaps()
```

## ⚡ Parallel Execution

```cursed
# Enable parallel mode
enable_parallel()

# Run tests in parallel
sus test_names [tea] = ["test_a", "test_b", "test_c"]
run_tests_in_parallel(test_names)
```

## ⚙️ Configuration

```cursed
# Verbose output
enable_verbose()
disable_verbose()

# Fail fast mode
enable_fail_fast()
disable_fail_fast()

# Parallel execution
enable_parallel()
disable_parallel()

# Coverage analysis
enable_coverage()
disable_coverage()
```

## 🎯 Advanced Assertions

### Integer Assertions
```cursed
assert_eq_int(actual, expected)     # Equality
assert_ne_int(actual, expected)     # Not equal
assert_gt_int(actual, expected)     # Greater than
assert_lt_int(actual, expected)     # Less than
assert_in_range(actual, min, max)   # Range check
```

### String Assertions
```cursed
assert_eq_string(actual, expected)  # String equality
```

### Boolean Assertions
```cursed
assert_true(condition)              # Must be based
assert_false(condition)             # Must be cap
```

### Float Assertions
```cursed
assert_eq_float(actual, expected)   # Float equality with tolerance
```

## 📋 Test Lifecycle

```cursed
# Suite management
suite_start("Suite Name")
suite_end()

# Test management
test_start("Test Name")
test_end()

# Skip tests
test_skip("Reason for skipping")

# Reset state
reset_test_state()
```

## 📊 Statistics and Reporting

```cursed
# Get test statistics
sus pass_rate normie = get_test_statistics()
sus failed_count normie = get_test_results()
sus all_passed lit = all_tests_passed()

# Print comprehensive summary
print_test_summary()
```

## 🔧 Integration with CI/CD

The framework supports multiple output formats for integration with CI/CD systems:

### JSON Output
```json
{
  "framework": "CURSED Testing Framework v3.0",
  "suite": "My Test Suite",
  "total_tests": 15,
  "passed_tests": 13,
  "failed_tests": 2,
  "skipped_tests": 0,
  "total_time": 1250,
  "pass_rate": "86%"
}
```

### XML Output (JUnit compatible)
```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="My Test Suite" tests="15">
    <properties>
      <property name="framework" value="CURSED Testing Framework v3.0"/>
    </properties>
  </testsuite>
</testsuites>
```

### TAP Output
```
TAP version 13
1..15
# Suite: My Test Suite
# Framework: CURSED Testing Framework v3.0
# Passed: 13
# Failed: 2
```

## 🚀 Complete Example

```cursed
yeet "testz/mod_enhanced_simple"

slay main() {
    # Configure framework
    enable_verbose()
    enable_coverage()
    enable_json_output()
    enable_xml_output()
    
    # Reset state
    reset_test_state()
    
    # Run comprehensive test suite
    suite_start("Complete Example")
    
    # Basic functionality tests
    test_start("basic_operations")
    assert_eq_int(2 + 2, 4)
    assert_eq_string("hello", "hello")
    assert_true(5 > 3)
    assert_eq_float(3.14, 3.14)
    test_end()
    
    # Property-based tests
    property_test_int("random_integers", 1, 100, 50)
    
    # Performance tests
    benchmark_function("sorting_algorithm", 10)
    check_performance_regression("fast_function", 50, 100, 100)
    
    # Mock tests
    test_start("mock_system")
    sus mock_id normie = create_mock("external_service")
    mock_return(mock_id, "success")
    mock_verify_called(mock_id, 1)
    test_end()
    
    # Discovery and parallel execution
    run_test_suite("tests/")
    sus test_names [tea] = ["parallel_test_1", "parallel_test_2"]
    run_tests_in_parallel(test_names)
    
    suite_end()
    
    # Generate comprehensive report
    print_test_summary()
    
    # Return appropriate exit code
    lowkey all_tests_passed() {
        damn 0
    } highkey {
        damn 1
    }
}
```

## 🎨 Output Example

```
🚀 CURSED Testing Framework v3.0 - Enhanced Simple Edition loaded!
💪 Ready to test everything with enhanced power!

🏁 Starting Suite: Complete Example
🧪 Running: basic_operations
  ✅ assert_eq_int: 4 == 4
  ✅ assert_eq_string: "hello" == "hello"
  ✅ assert_true: condition is based
  ✅ assert_eq_float: 3.14 ~= 3.14
⏱️ Duration: 1000ms

🧪 Running: property_random_integers
  ✅ Property test passed for 50 iterations
⏱️ Duration: 1000ms

📊 Benchmark Results for sorting_algorithm:
  Iterations: 10
  Average: 15ms
  Min: 10ms
  Max: 25ms

🎭 Created mock: external_service
🎭 Mock 1 will return: success
🎭 Mock 1 verified 1 calls

🔍 Discovering tests in: tests/
📁 Found 3 test files

🏃 Running 2 tests in parallel
🧪 Parallel test: parallel_test_1
🧪 Parallel test: parallel_test_2

🏁 Completed Suite: Complete Example

════════════════════════════════════════════════════════════════
                🧪 CURSED Testing Framework v3.0 🧪
                   ENHANCED TEST SUMMARY
════════════════════════════════════════════════════════════════

Suite: Complete Example
Total Execution Time: 5000ms

📊 Test Results:
  Total Tests:    12
  Passed:         12 (100%)
  Failed:         0 (0%)
  Skipped:        0 (0%)
  Errors:         0 (0%)

⚡ Performance Metrics:
  Average Test Time: 416ms
  Tests per Second: 2

📊 Coverage Analysis:
  Lines covered: 85%
  Branches covered: 78%
  Functions covered: 92%
  Total coverage: 85%

📊 JSON Report:
{
  "framework": "CURSED Testing Framework v3.0",
  "suite": "Complete Example",
  "total_tests": 12,
  "passed_tests": 12,
  "failed_tests": 0,
  "skipped_tests": 0,
  "total_time": 5000,
  "pass_rate": "100%"
}

🎉 ALL TESTS PASSED! 🎉
🔥 Your code is absolutely fire! 🔥
════════════════════════════════════════════════════════════════
```

## 🎯 Best Practices

1. **Use Suites**: Group related tests into suites for better organization
2. **Enable Verbose**: Use verbose mode during development for detailed output
3. **Property-Based Testing**: Use property-based tests for comprehensive coverage
4. **Performance Testing**: Include benchmarks for performance-critical code
5. **Mock External Dependencies**: Use mocks to isolate unit tests
6. **Parallel Execution**: Enable parallel mode for faster test execution
7. **Coverage Analysis**: Enable coverage to identify untested code
8. **Multiple Output Formats**: Use JSON/XML output for CI/CD integration
9. **Fail Fast**: Enable fail-fast mode during development to stop on first failure
10. **Regular Regression Testing**: Use performance regression tests to catch slowdowns

## 🔧 Testing Commands

```bash
# Run the enhanced framework tests
cargo run --bin cursed stdlib/testz/test_enhanced_simple.csd

# Run in both interpretation and compilation modes
cargo run --bin cursed stdlib/testz/test_enhanced_simple.csd
cargo run --bin cursed -- compile stdlib/testz/test_enhanced_simple.csd
./test_enhanced_simple

# Test your own stdlib modules
cargo run --bin cursed your_module/test_your_module.csd
```

## 🌟 Status

- **Version**: 3.0 Enhanced Simple Edition
- **Status**: Production Ready
- **Dependencies**: None (Pure CURSED)
- **Compatibility**: Both interpretation and compilation modes
- **Test Coverage**: 100% (self-testing framework)

The CURSED Testing Framework v3.0 Enhanced Edition is the ultimate testing solution for CURSED stdlib development, providing enterprise-grade testing capabilities with Gen Z attitude! 🚀🔥
