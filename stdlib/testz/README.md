# Enhanced CURSED Testing Framework (testz)

The enhanced testz framework provides comprehensive testing capabilities for CURSED programs with advanced features for error handling, performance testing, and build system integration.

## Features

### ✨ Core Features
- **Comprehensive Assertion Library**: Over 20 assertion functions for various data types
- **Property-Based Testing**: Automated testing with random input generation
- **Performance Benchmarking**: Built-in benchmarking with validation and comparison
- **Test Discovery**: Automatic test discovery with pattern matching and filtering
- **Multiple Report Formats**: JSON, XML, HTML, and text reporting
- **Build System Integration**: Seamless integration with CI/CD pipelines
- **Enhanced Error Handling**: Detailed error reporting with context and stack traces

### 🚀 Enhanced Features
- **Context-Aware Assertions**: Assertions with detailed context information
- **Test Execution Control**: Timeout and retry mechanisms
- **Test Fixtures**: Setup and teardown support
- **Test Grouping**: Logical grouping of related tests
- **Performance Validation**: Automatic performance regression detection
- **Continuous Integration**: Built-in CI/CD support

## Quick Start

### Basic Usage

```cursed
yeet "testz"

# Start a test
testz.test_start("My first test")
testz.assert_true(based)
testz.assert_eq_string("hello", "hello")
testz.assert_eq_int(42, 42)
testz.test_end()

# Print results
testz.print_test_summary()
```

### Enhanced Usage

```cursed
yeet "testz"
yeet "enhanced_testz"

# Enhanced error handling
enhanced_testz.assert_with_context(based, "Context-aware assertion", "test_context")
enhanced_testz.assert_eq_with_diff("expected", "actual", "String comparison")

# Performance benchmarking
enhanced_testz.benchmark_with_validation("performance_test", 100, "validation_func")

# Test discovery
enhanced_testz.discover_tests_in_directory("tests", "test_*")
enhanced_testz.filter_tests_by_tag("unit")
enhanced_testz.run_discovered_tests()

# Report generation
enhanced_testz.generate_test_report("json")
```

## API Reference

### Core Testing Functions

| Function | Description |
|----------|-------------|
| `test_start(name)` | Begin a new test |
| `test_end()` | End the current test |
| `test_pass(message)` | Record a passing assertion |
| `test_fail(message)` | Record a failing assertion |
| `print_test_summary()` | Display test results |

### Assertion Functions

| Function | Description |
|----------|-------------|
| `assert_true(condition)` | Assert condition is true |
| `assert_false(condition)` | Assert condition is false |
| `assert_eq_string(actual, expected)` | Assert string equality |
| `assert_eq_int(actual, expected)` | Assert integer equality |
| `assert_ne_int(actual, expected)` | Assert integer inequality |
| `assert_gt_int(actual, expected)` | Assert greater than |
| `assert_lt_int(actual, expected)` | Assert less than |
| `assert_ge_int(actual, expected)` | Assert greater than or equal |
| `assert_le_int(actual, expected)` | Assert less than or equal |
| `assert_contains(haystack, needle)` | Assert string contains substring |
| `assert_not_contains(haystack, needle)` | Assert string doesn't contain substring |
| `assert_starts_with(text, prefix)` | Assert string starts with prefix |
| `assert_ends_with(text, suffix)` | Assert string ends with suffix |
| `assert_empty_string(text)` | Assert string is empty |
| `assert_not_empty_string(text)` | Assert string is not empty |
| `assert_range_int(value, min, max)` | Assert value is within range |

### Enhanced Assertion Functions

| Function | Description |
|----------|-------------|
| `assert_with_context(condition, message, context)` | Assert with context information |
| `assert_eq_with_diff(actual, expected, message)` | Assert with difference reporting |
| `assert_approximately_equal(actual, expected, tolerance)` | Assert approximate equality |
| `assert_array_equals(actual, expected)` | Assert array equality |
| `assert_matches_pattern(text, pattern)` | Assert pattern matching |
| `assert_between(value, min, max)` | Assert value is between bounds |

### Performance Benchmarking

| Function | Description |
|----------|-------------|
| `benchmark_start(name)` | Start a benchmark |
| `benchmark_end()` | End the current benchmark |
| `benchmark_iteration_start()` | Start a benchmark iteration |
| `benchmark_iteration_end()` | End a benchmark iteration |
| `set_benchmark_iterations(count)` | Set number of iterations |
| `benchmark_with_validation(name, iterations, func)` | Run validated benchmark |
| `benchmark_comparison(name1, name2, func1, func2)` | Compare two benchmarks |

### Property-Based Testing

| Function | Description |
|----------|-------------|
| `property_test_start(name, iterations)` | Start property test |
| `property_test_end()` | End property test |
| `property_test_iteration()` | Process one iteration |
| `property_test_fail(message)` | Record property test failure |
| `random_int(min, max)` | Generate random integer |
| `random_string(length)` | Generate random string |
| `random_boolean()` | Generate random boolean |

### Test Discovery and Execution

| Function | Description |
|----------|-------------|
| `discover_tests_in_directory(dir, pattern)` | Discover tests in directory |
| `filter_tests_by_tag(tag)` | Filter tests by tag |
| `run_discovered_tests()` | Run discovered tests |
| `should_run_test(name)` | Check if test should run |
| `run_test_with_timeout(name, timeout)` | Run test with timeout |
| `run_test_with_retry(name, retries)` | Run test with retry |

### Test Configuration

| Function | Description |
|----------|-------------|
| `set_verbose_mode(enabled)` | Enable/disable verbose output |
| `set_parallel_mode(enabled)` | Enable/disable parallel execution |
| `set_benchmark_mode(enabled)` | Enable/disable benchmark mode |
| `set_test_suite(name)` | Set test suite name |
| `set_test_filter(filter)` | Set test filter |

### Report Generation

| Function | Description |
|----------|-------------|
| `generate_test_report(format)` | Generate test report |
| `generate_json_report()` | Generate JSON report |
| `generate_xml_report()` | Generate XML report |
| `generate_html_report()` | Generate HTML report |
| `generate_text_report()` | Generate text report |

### Test Fixtures

| Function | Description |
|----------|-------------|
| `set_fixture_data(data)` | Set fixture data |
| `get_fixture_data()` | Get fixture data |
| `create_test_fixture(name, data)` | Create test fixture |
| `cleanup_test_fixture(name)` | Clean up test fixture |
| `set_setup_function(func)` | Set setup function |
| `set_teardown_function(func)` | Set teardown function |

### Test Utilities

| Function | Description |
|----------|-------------|
| `skip_test(reason)` | Skip current test |
| `pending_test(reason)` | Mark test as pending |
| `focus_test()` | Focus on current test |
| `test_group_start(name)` | Start test group |
| `test_group_end(name)` | End test group |
| `reset_test_state()` | Reset test state |

## Running Tests

### Basic Test Execution

```bash
# Run testz module tests
cargo run --bin cursed stdlib/testz/test_testz.csd

# Run enhanced testz tests
cargo run --bin cursed stdlib/testz/test_enhanced_testz.csd

# Run error reporting tests
cargo run --bin cursed stdlib/testz/test_error_reporting.csd

# Run performance benchmarks
cargo run --bin cursed stdlib/testz/performance_benchmarks.csd
```

### Framework Testing

```bash
# Run all stdlib tests
cargo run --bin cursed test --test-dir stdlib

# Run tests with specific pattern
cargo run --bin cursed test --pattern "test_*.csd"

# Run tests with filter
cargo run --bin cursed test --filter math

# Run tests in parallel
cargo run --bin cursed test --parallel

# Run tests with verbose output
cargo run --bin cursed test --verbose
```

### Build System Integration

```bash
# Run build integration tests
cargo run --bin cursed stdlib/testz/build_integration.csd

# Generate test reports
cargo run --bin cursed test --format json
cargo run --bin cursed test --format xml
cargo run --bin cursed test --format html
```

## Configuration

### Test Suite Configuration

```cursed
# Set test suite name
testz.set_test_suite("My Test Suite")

# Enable verbose mode
testz.set_verbose_mode(based)

# Enable parallel execution
testz.set_parallel_mode(based)

# Enable benchmark mode
testz.set_benchmark_mode(based)

# Set test filter
testz.set_test_filter("unit")
```

### Performance Configuration

```cursed
# Set benchmark iterations
testz.set_benchmark_iterations(1000)

# Set performance thresholds
enhanced_testz.performance_memory_threshold = 2000000  # 2MB
enhanced_testz.performance_time_threshold = 5000000    # 5ms
```

## Best Practices

### Test Organization

1. **Group Related Tests**: Use `test_group_start()` and `test_group_end()` to organize tests
2. **Use Descriptive Names**: Test names should clearly describe what is being tested
3. **Single Responsibility**: Each test should test one specific behavior
4. **Setup and Teardown**: Use fixtures for test setup and cleanup

### Error Handling

1. **Provide Context**: Use `assert_with_context()` for better error messages
2. **Detailed Failures**: Use `assert_eq_with_diff()` for comparison failures
3. **Error Reports**: Generate detailed error reports for debugging

### Performance Testing

1. **Baseline Measurements**: Establish performance baselines
2. **Regression Detection**: Use performance validation to catch regressions
3. **Comparative Analysis**: Use benchmark comparison for algorithm evaluation

### Continuous Integration

1. **Test Discovery**: Use automatic test discovery for scalability
2. **Parallel Execution**: Enable parallel mode for faster CI runs
3. **Report Generation**: Generate multiple report formats for different tools

## Examples

### Property-Based Testing Example

```cursed
yeet "testz"

testz.test_start("Commutative property test")
testz.property_test_start("Addition is commutative", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus a normie = testz.random_int(1, 1000)
    sus b normie = testz.random_int(1, 1000)
    
    fr fr (a + b) != (b + a) {
        testz.property_test_fail("Commutative property failed")
    }
}

testz.property_test_end()
testz.test_end()
```

### Performance Benchmarking Example

```cursed
yeet "testz"
yeet "enhanced_testz"

testz.test_start("Algorithm performance")
enhanced_testz.benchmark_with_validation("sorting_algorithm", 1000, "sort_test")
testz.test_end()

enhanced_testz.benchmark_comparison("bubble_sort", "quick_sort", "bubble", "quick")
```

### Test Discovery Example

```cursed
yeet "enhanced_testz"

enhanced_testz.discover_tests_in_directory("tests", "test_*")
enhanced_testz.filter_tests_by_tag("unit")
enhanced_testz.run_discovered_tests()
enhanced_testz.generate_test_report("json")
```

## Contributing

When contributing to the testz framework:

1. Follow the established patterns for function naming and structure
2. Add comprehensive tests for new features
3. Update documentation for new functionality
4. Ensure backwards compatibility with existing tests
5. Add performance benchmarks for new features

## License

This testing framework is part of the CURSED language project and follows the same license terms.
