# testz_simple - Comprehensive CURSED Testing Framework

A powerful, self-contained testing framework implemented in pure CURSED with advanced capabilities for property-based testing, benchmarking, fixtures, and comprehensive test management.

## Features

### 🧪 Core Testing
- **Basic Assertions**: `assert_true`, `assert_false`, `assert_eq_*`, `assert_ne_*`
- **Enhanced Assertions**: Range checks, string operations, comparison operators
- **Test Lifecycle**: `test_start`, `test_end`, automatic statistics tracking
- **Verbose Mode**: Detailed output with execution details

### 🔬 Property-Based Testing
- **Random Generators**: `random_int`, `random_boolean`
- **Property Validation**: Test properties across many random inputs
- **Iteration Control**: Configurable test iterations with failure tracking
- **Mathematical Properties**: Test invariants and contracts

### 🏃 Performance Benchmarking
- **Micro-benchmarks**: Basic performance measurement
- **Iteration Control**: Configurable benchmark iterations
- **Simple Timing**: Basic timing analysis

### 🔧 Test Fixtures
- **Fixture Data**: Shared test data across test functions
- **Setup/Teardown**: Configure test environment
- **Lifecycle Management**: Automatic setup/teardown tracking

### 📊 Advanced Reporting
- **Detailed Statistics**: Success rates, assertion counts
- **Multiple Formats**: Verbose, summary, and detailed reports
- **Test Tracking**: Monitor passed/failed tests and assertions

## Quick Start

```cursed
yeet "testz_simple"

# Configure testing framework
testz_simple.set_verbose_mode(based)
testz_simple.set_test_suite("My Test Suite")
testz_simple.before_all_tests()

# Basic test
testz_simple.test_start("Basic arithmetic")
testz_simple.assert_eq_int(2 + 2, 4)
testz_simple.assert_gt_int(10, 5)
testz_simple.test_end()

# Property-based test
testz_simple.test_start("Mathematical properties")
testz_simple.property_test_start("Addition commutative", 50)

bestie i := 0; i < 50; i++ {
    testz_simple.property_test_iteration()
    sus a normie = testz_simple.random_int(1, 100)
    sus b normie = testz_simple.random_int(1, 100)
    
    fr fr (a + b) != (b + a) {
        testz_simple.property_test_fail("Commutative property failed")
    }
}

testz_simple.property_test_end()
testz_simple.test_end()

# Print results
testz_simple.after_all_tests()
```

## Assertions

### Basic Assertions
```cursed
testz_simple.assert_true(condition)
testz_simple.assert_false(condition)
testz_simple.assert_eq_string(actual, expected)
testz_simple.assert_eq_int(actual, expected)
testz_simple.assert_ne_int(actual, expected)
```

### Comparison Assertions
```cursed
testz_simple.assert_gt_int(actual, expected)      # Greater than
testz_simple.assert_lt_int(actual, expected)      # Less than
testz_simple.assert_ge_int(actual, expected)      # Greater than or equal
testz_simple.assert_le_int(actual, expected)      # Less than or equal
testz_simple.assert_range_int(value, min, max)    # Range check
```

### String Assertions
```cursed
testz_simple.assert_empty_string(text)
testz_simple.assert_not_empty_string(text)
```

## Property-Based Testing

Test properties across many random inputs:

```cursed
testz_simple.property_test_start("Property name", 100)

bestie i := 0; i < 100; i++ {
    testz_simple.property_test_iteration()
    sus value normie = testz_simple.random_int(1, 1000)
    
    # Test property
    fr fr value <= 0 {
        testz_simple.property_test_fail("Value should be positive")
    }
}

testz_simple.property_test_end()
```

### Random Generators
```cursed
testz_simple.random_int(min, max)    # Random integer in range
testz_simple.random_boolean()        # Random boolean value
```

## Benchmarking

Measure performance:

```cursed
testz_simple.set_benchmark_iterations(1000)
testz_simple.benchmark_start("Operation name")

bestie i := 0; i < 1000; i++ {
    # Code to benchmark
    sus result normie = i * 2
}

testz_simple.benchmark_end()
```

## Test Fixtures

Set up shared test data:

```cursed
testz_simple.set_fixture_data("test_data")
testz_simple.set_setup_function("setup_function")
testz_simple.set_teardown_function("teardown_function")

testz_simple.test_start("Test with fixtures")
sus data tea = testz_simple.get_fixture_data()
testz_simple.assert_eq_string(data, "test_data")
testz_simple.test_end()
```

## Configuration

```cursed
testz_simple.set_verbose_mode(based)              # Enable verbose output
testz_simple.set_test_suite("Suite name")         # Set test suite name
testz_simple.set_test_filter("filter")            # Filter tests
testz_simple.set_benchmark_iterations(1000)       # Set benchmark iterations
```

## Test Utilities

```cursed
testz_simple.skip_test("Reason")                  # Skip test
testz_simple.pending_test("Reason")               # Mark test as pending
testz_simple.focus_test()                         # Focus on test
testz_simple.reset_test_state()                   # Reset all state
```

## Statistics

```cursed
testz_simple.get_test_results()                   # Total tests
testz_simple.get_passed_tests()                   # Passed tests
testz_simple.get_failed_tests()                   # Failed tests
testz_simple.get_assertion_count()                # Total assertions
testz_simple.get_assertion_failures()             # Failed assertions
testz_simple.get_success_rate()                   # Success percentage
testz_simple.all_tests_passed()                   # All tests passed?
```

## Lifecycle Hooks

```cursed
testz_simple.before_all_tests()                   # Before all tests
testz_simple.after_all_tests()                    # After all tests
testz_simple.before_each_test()                   # Before each test
testz_simple.after_each_test()                    # After each test
```

## Reporting

```cursed
testz_simple.print_test_summary()                 # Basic summary
testz_simple.print_detailed_report()              # Detailed report
```

## Example: Complete Test Suite

```cursed
yeet "testz_simple"

# Setup
testz_simple.set_verbose_mode(based)
testz_simple.set_test_suite("Math Operations")
testz_simple.before_all_tests()

# Unit tests
testz_simple.test_start("Basic arithmetic")
testz_simple.assert_eq_int(2 + 2, 4)
testz_simple.assert_eq_int(10 - 5, 5)
testz_simple.test_end()

# Property-based tests
testz_simple.test_start("Mathematical properties")
testz_simple.property_test_start("Commutative property", 50)

bestie i := 0; i < 50; i++ {
    testz_simple.property_test_iteration()
    sus a normie = testz_simple.random_int(1, 100)
    sus b normie = testz_simple.random_int(1, 100)
    
    fr fr (a + b) != (b + a) {
        testz_simple.property_test_fail("Addition not commutative")
    }
}

testz_simple.property_test_end()
testz_simple.test_end()

# Benchmarks
testz_simple.test_start("Performance")
testz_simple.set_benchmark_iterations(1000)
testz_simple.benchmark_start("Addition")

bestie i := 0; i < 1000; i++ {
    sus result normie = i + i
}

testz_simple.benchmark_end()
testz_simple.test_end()

# Finalize
testz_simple.after_all_tests()
```

## Best Practices

1. **Use Descriptive Names**: Clear test names help identify issues
2. **Property-Based Testing**: Use for testing mathematical properties
3. **Fixtures**: Share common test data across tests
4. **Assertions**: Use the most specific assertion available
5. **Verbose Mode**: Enable for detailed debugging information
6. **Test Organization**: Group related tests together
7. **Error Handling**: Test both success and failure cases

## Integration with CURSED

This framework integrates seamlessly with CURSED projects:

- **Pure CURSED**: No external dependencies
- **Self-contained**: All functionality in one module
- **Extensible**: Easy to add new assertion types
- **Portable**: Works in any CURSED environment

Perfect for stdlib development, application testing, and ensuring code quality in CURSED projects.
