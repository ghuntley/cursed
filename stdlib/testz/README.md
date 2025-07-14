# Enhanced testz - Comprehensive CURSED Testing Framework

A powerful, pure CURSED testing framework with advanced capabilities for property-based testing, benchmarking, fixtures, and comprehensive test management.

## Features

### 🧪 Core Testing
- **Basic Assertions**: `assert_true`, `assert_false`, `assert_eq_*`, `assert_ne_*`
- **Enhanced Assertions**: Range checks, string operations, comparison operators
- **Test Lifecycle**: `test_start`, `test_end`, automatic timing and statistics
- **Verbose Mode**: Detailed output with execution times and assertion tracking

### 🔬 Property-Based Testing
- **Random Generators**: `random_int`, `random_string`, `random_boolean`
- **Property Validation**: Automatically test properties across many inputs
- **Iteration Control**: Configurable test iterations with failure tracking
- **Advanced Properties**: Test mathematical properties, invariants, and contracts

### 🏃 Performance Benchmarking
- **Micro-benchmarks**: Time individual operations with nanosecond precision
- **Statistical Analysis**: Min, max, average, and total execution times
- **Iteration Control**: Configurable benchmark iterations
- **Memory Tracking**: Monitor memory usage before and after tests

### 🔧 Test Fixtures and Setup/Teardown
- **Setup Functions**: Configure test environment before each test
- **Teardown Functions**: Clean up after each test
- **Fixture Data**: Shared test data across test functions
- **Lifecycle Management**: Automatic setup/teardown execution

### 🔍 Test Discovery and Execution
- **Pattern Matching**: Discover tests by filename patterns
- **Filtering**: Run specific tests with filter expressions
- **Test Suites**: Organize tests into logical suites
- **Parallel Execution**: Support for concurrent test execution

### 📊 Advanced Reporting
- **Detailed Statistics**: Success rates, assertion counts, execution times
- **Multiple Formats**: Verbose, summary, and detailed report formats
- **Test Tracking**: Monitor passed/failed tests and assertion failures
- **Performance Metrics**: Execution time and memory usage reporting

## Basic Usage

```cursed
yeet "testz"

# Configure testing framework
testz.set_verbose_mode(based)
testz.set_test_suite("My Test Suite")

# Basic test
testz.test_start("Basic arithmetic")
testz.assert_eq_int(2 + 2, 4)
testz.assert_gt_int(10, 5)
testz.test_end()

# Print results
testz.print_test_summary()
```

## Enhanced Assertions

### Numeric Assertions
```cursed
testz.assert_eq_int(actual, expected)      # Equality
testz.assert_ne_int(actual, expected)      # Inequality
testz.assert_gt_int(actual, expected)      # Greater than
testz.assert_lt_int(actual, expected)      # Less than
testz.assert_ge_int(actual, expected)      # Greater than or equal
testz.assert_le_int(actual, expected)      # Less than or equal
testz.assert_range_int(value, min, max)    # Range check
```

### String Assertions
```cursed
testz.assert_eq_string(actual, expected)           # String equality
testz.assert_contains(haystack, needle)            # Contains substring
testz.assert_not_contains(haystack, needle)        # Does not contain
testz.assert_starts_with(text, prefix)             # Starts with prefix
testz.assert_ends_with(text, suffix)               # Ends with suffix
testz.assert_empty_string(text)                    # Empty string
testz.assert_not_empty_string(text)                # Non-empty string
```

### Boolean Assertions
```cursed
testz.assert_true(condition)                       # True assertion
testz.assert_false(condition)                      # False assertion
```

### Error Handling Assertions
```cursed
testz.assert_throws("Expected error message")      # Expect error
testz.assert_no_throw()                            # No error expected
```

## Property-Based Testing

Test properties that should hold for many random inputs:

```cursed
# Test commutative property of addition
testz.property_test_start("Addition commutative", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus a normie = testz.random_int(1, 1000)
    sus b normie = testz.random_int(1, 1000)
    
    fr fr (a + b) != (b + a) {
        testz.property_test_fail("Commutative property failed")
    }
}

testz.property_test_end()
```

### Random Generators
```cursed
testz.random_int(min, max)          # Random integer in range
testz.random_string(length)         # Random string of specified length
testz.random_boolean()              # Random boolean value
```

## Performance Benchmarking

Measure performance of code with detailed statistics:

```cursed
testz.set_benchmark_mode(based)
testz.set_benchmark_iterations(1000)

testz.benchmark_start("String concatenation")

bestie i := 0; i < 1000; i++ {
    testz.benchmark_iteration_start()
    sus result tea = "hello" + "world"
    testz.benchmark_iteration_end()
}

testz.benchmark_end()
```

## Test Fixtures

Set up shared test data and environment:

```cursed
# Configure fixtures
testz.set_setup_function("setup_database")
testz.set_teardown_function("cleanup_database")
testz.set_fixture_data("test_data_123")

testz.test_start("Database operations")
sus data tea = testz.get_fixture_data()
testz.assert_eq_string(data, "test_data_123")
testz.test_end()
```

## Test Discovery and Filtering

Organize and run specific tests:

```cursed
# Set up test suite
testz.set_test_suite("Integration Tests")
testz.set_test_filter("database")

# Discover tests
testz.discover_tests("test_*")

# Check if test should run
fr fr testz.should_run_test("test_database_connection") {
    # Run the test
}
```

## Configuration Options

### Execution Modes
```cursed
testz.set_verbose_mode(based)        # Detailed output
testz.set_parallel_mode(based)       # Parallel execution
testz.set_benchmark_mode(based)      # Performance benchmarking
```

### Query Configuration
```cursed
testz.is_verbose_mode()              # Check if verbose mode enabled
testz.is_parallel_mode()             # Check if parallel mode enabled
testz.is_benchmark_mode()            # Check if benchmark mode enabled
```

## Advanced Test Control

### Test Lifecycle Hooks
```cursed
testz.before_all_tests()             # Run before all tests
testz.after_all_tests()              # Run after all tests
testz.before_each_test()             # Run before each test
testz.after_each_test()              # Run after each test
```

### Test Utilities
```cursed
testz.skip_test("Reason for skipping")       # Skip test with reason
testz.pending_test("Feature not ready")      # Mark test as pending
testz.focus_test()                           # Focus on specific test
testz.reset_test_state()                     # Reset all test state
```

## Statistics and Reporting

### Test Results
```cursed
testz.get_test_results()             # Total number of tests
testz.get_passed_tests()             # Number of passed tests
testz.get_failed_tests()             # Number of failed tests
testz.get_assertion_count()          # Total assertions made
testz.get_assertion_failures()       # Number of failed assertions
testz.get_success_rate()             # Success rate percentage
testz.all_tests_passed()             # Boolean: all tests passed
```

### Performance Metrics
```cursed
testz.get_execution_time()           # Total execution time
testz.get_memory_usage()             # Memory usage difference
```

### Reporting Options
```cursed
testz.print_test_summary()           # Basic summary
testz.print_detailed_report()        # Detailed report with metrics
```

## Example: Complete Test Suite

```cursed
yeet "testz"

# Configure framework
testz.set_verbose_mode(based)
testz.set_test_suite("Math Operations Test Suite")
testz.before_all_tests()

# Basic functionality test
testz.test_start("Basic arithmetic operations")
testz.assert_eq_int(2 + 2, 4)
testz.assert_eq_int(10 - 5, 5)
testz.assert_eq_int(3 * 4, 12)
testz.assert_eq_int(8 / 2, 4)
testz.test_end()

# Property-based test
testz.test_start("Mathematical properties")
testz.property_test_start("Multiplication commutative", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    sus a normie = testz.random_int(1, 100)
    sus b normie = testz.random_int(1, 100)
    
    fr fr (a * b) != (b * a) {
        testz.property_test_fail("Multiplication not commutative")
    }
}

testz.property_test_end()
testz.test_end()

# Performance benchmark
testz.test_start("Performance benchmarks")
testz.set_benchmark_mode(based)
testz.benchmark_start("Addition performance")

bestie i := 0; i < 1000; i++ {
    testz.benchmark_iteration_start()
    sus result normie = i + i
    testz.benchmark_iteration_end()
}

testz.benchmark_end()
testz.test_end()

# Complete test suite
testz.after_all_tests()
```

## Best Practices

1. **Use Descriptive Test Names**: Make test names clear and specific
2. **Property-Based Testing**: Use for testing mathematical properties and invariants
3. **Benchmarking**: Use for performance-critical code paths
4. **Fixtures**: Use for shared test data and environment setup
5. **Assertions**: Use the most specific assertion available
6. **Error Handling**: Test both success and failure cases
7. **Test Organization**: Group related tests into suites
8. **Continuous Integration**: Use verbose mode for CI/CD pipelines

## Integration with CURSED Stdlib

The enhanced testz framework integrates seamlessly with other CURSED stdlib modules:

- **timez**: For precise timing measurements
- **stringz**: For string manipulation in tests
- **mathz**: For random number generation
- **vibez**: For output and logging

This makes it a comprehensive testing solution for all CURSED applications and libraries.
