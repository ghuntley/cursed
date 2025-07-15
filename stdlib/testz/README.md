# TestZ Framework v3.0 - Comprehensive Testing for CURSED

The TestZ framework provides enterprise-grade testing capabilities for CURSED stdlib development. This enhanced version includes advanced assertion functions, performance benchmarking, property-based testing, and comprehensive test reporting.

## Features

### 🚀 Core Testing Features
- **Advanced Assertion Library**: 20+ assertion functions for all data types
- **Test Suite Management**: Organization, filtering, and discovery
- **Performance Benchmarking**: Built-in timing and performance analysis
- **Property-Based Testing**: Automated testing with random data generation
- **Test Fixtures**: Setup/teardown lifecycle management
- **Comprehensive Reporting**: Detailed statistics and formatting options

### 🎯 Advanced Capabilities
- **Mock/Stub Support**: Test fixture data management
- **Parallel Testing**: Configuration for concurrent test execution
- **Error Handling**: Robust error detection and reporting
- **Test Control**: Skip, pending, and focus test management
- **Random Generators**: Built-in random data generation utilities
- **Statistical Analysis**: Success rates, timing, and memory usage

## Quick Start

```cursed
yeet "testz"

slay test_example() {
    test_start("Example Test")
    
    # Basic assertions
    assert_true(based)
    assert_eq_int(2 + 2, 4)
    assert_eq_string("hello", "hello")
    
    # String assertions
    assert_contains("hello world", "world")
    assert_starts_with("hello", "he")
    
    # Range assertions
    assert_range_int(50, 1, 100)
    
    test_end()
}

# Run test with summary
before_all_tests()
set_test_suite("My Test Suite")
test_example()
after_all_tests()
```

## Assertion Functions

### Boolean Assertions
- `assert_true(condition)` - Assert condition is true
- `assert_false(condition)` - Assert condition is false

### Integer Assertions
- `assert_eq_int(actual, expected)` - Assert integers are equal
- `assert_ne_int(actual, expected)` - Assert integers are not equal
- `assert_gt_int(actual, expected)` - Assert actual > expected
- `assert_lt_int(actual, expected)` - Assert actual < expected
- `assert_ge_int(actual, expected)` - Assert actual >= expected
- `assert_le_int(actual, expected)` - Assert actual <= expected
- `assert_range_int(actual, min, max)` - Assert value is in range

### String Assertions
- `assert_eq_string(actual, expected)` - Assert strings are equal
- `assert_contains(haystack, needle)` - Assert string contains substring
- `assert_not_contains(haystack, needle)` - Assert string doesn't contain substring
- `assert_starts_with(text, prefix)` - Assert string starts with prefix
- `assert_ends_with(text, suffix)` - Assert string ends with suffix
- `assert_empty_string(text)` - Assert string is empty
- `assert_not_empty_string(text)` - Assert string is not empty

### Error Handling Assertions
- `assert_throws(message)` - Assert that an error was thrown
- `assert_no_throw()` - Assert that no error was thrown

## Performance Benchmarking

```cursed
# Set up benchmarking
set_benchmark_mode(based)
set_benchmark_iterations(1000)

# Run benchmark
benchmark_start("Algorithm Performance")
sus i normie = 0
bestie i = 0; i < 1000; i = i + 1 {
    benchmark_iteration_start()
    
    # Code to benchmark
    sus result normie = expensive_operation(i)
    
    benchmark_iteration_end()
}
benchmark_end()
```

## Property-Based Testing

```cursed
# Test mathematical properties
property_test_start("Addition is commutative", 100)

sus i normie = 0
bestie i = 0; i < 100; i = i + 1 {
    property_test_iteration()
    
    sus a normie = random_int(1, 1000)
    sus b normie = random_int(1, 1000)
    
    highkey add(a, b) != add(b, a) {
        property_test_fail("Commutativity failed for " + tea(a) + " and " + tea(b))
    }
}

property_test_end()
```

## Test Configuration

### Modes
- `set_verbose_mode(enabled)` - Enable/disable verbose output
- `set_parallel_mode(enabled)` - Enable/disable parallel execution
- `set_benchmark_mode(enabled)` - Enable/disable benchmarking

### Test Management
- `set_test_suite(name)` - Set test suite name
- `set_test_filter(filter)` - Filter tests by name pattern
- `discover_tests(pattern)` - Auto-discover test functions

### Test Control
- `skip_test(reason)` - Skip current test with reason
- `pending_test(reason)` - Mark test as pending
- `focus_test()` - Focus on specific test

## Random Data Generation

```cursed
# Generate random test data
sus random_num normie = random_int(1, 100)
sus random_str tea = random_string(10)
sus random_bool lit = random_boolean()

# Use in property-based tests
property_test_start("String operations", 50)
sus i normie = 0
bestie i = 0; i < 50; i = i + 1 {
    property_test_iteration()
    
    sus test_string tea = random_string(random_int(1, 20))
    sus test_length normie = stringz.Length(test_string)
    
    # Test string length property
    assert_ge_int(test_length, 1)
    assert_le_int(test_length, 20)
}
property_test_end()
```

## Test Fixtures

```cursed
# Set up test environment
slay setup_test_environment() {
    set_fixture_data("initialized")
    vibez.spill("Setting up test environment")
}

slay teardown_test_environment() {
    vibez.spill("Cleaning up test environment")
}

# Configure fixtures
set_setup_function("setup_test_environment")
set_teardown_function("teardown_test_environment")

# Use fixture data in tests
slay test_with_fixtures() {
    test_start("Test with fixtures")
    
    sus fixture_data tea = get_fixture_data()
    assert_eq_string(fixture_data, "initialized")
    
    test_end()
}
```

## Test Reporting

### Basic Summary
```cursed
print_test_summary()
```

### Detailed Report
```cursed
print_detailed_report()  # Includes execution time, memory usage, etc.
```

### Test Statistics
```cursed
sus total normie = get_test_results()
sus passed normie = get_passed_tests()
sus failed normie = get_failed_tests()
sus success_rate normie = get_success_rate()
sus all_passed lit = all_tests_passed()
```

## Complete Example

```cursed
yeet "testz"

slay test_comprehensive_example() {
    # Initialize test environment
    before_all_tests()
    set_verbose_mode(based)
    set_test_suite("Comprehensive Example")
    
    # Basic functionality test
    test_start("Basic Operations")
    assert_eq_int(add(2, 3), 5)
    assert_eq_string(concat("hello", "world"), "helloworld")
    test_end()
    
    # Performance benchmark
    set_benchmark_mode(based)
    benchmark_start("Performance Test")
    sus i normie = 0
    bestie i = 0; i < 1000; i = i + 1 {
        benchmark_iteration_start()
        sus result normie = fibonacci(10)
        benchmark_iteration_end()
    }
    benchmark_end()
    
    # Property-based test
    property_test_start("Mathematical Properties", 100)
    sus j normie = 0
    bestie j = 0; j < 100; j = j + 1 {
        property_test_iteration()
        
        sus a normie = random_int(1, 100)
        sus b normie = random_int(1, 100)
        
        # Test property
        highkey add(a, b) != add(b, a) {
            property_test_fail("Addition not commutative")
        }
    }
    property_test_end()
    
    # Generate final report
    after_all_tests()
}

test_comprehensive_example()
```

## Best Practices

### 1. Test Organization
- Group related tests into logical test functions
- Use descriptive test names and messages
- Set up test suites for different modules

### 2. Assertion Strategy
- Use the most specific assertion available
- Provide clear failure messages
- Test both positive and negative cases

### 3. Performance Testing
- Use benchmarking for performance-critical code
- Set appropriate iteration counts
- Monitor performance regressions

### 4. Property-Based Testing
- Use for testing mathematical properties
- Generate diverse test data
- Test edge cases with random inputs

### 5. Test Lifecycle
- Use fixtures for complex setup/teardown
- Reset test state between test runs
- Clean up resources properly

## Integration with CURSED Stdlib

The TestZ framework is designed to work seamlessly with all CURSED stdlib modules. Each module should include:

1. **Test file**: `test_[module].csd` using testz framework
2. **Comprehensive coverage**: All public functions tested
3. **Performance benchmarks**: Critical operations benchmarked
4. **Property tests**: Mathematical properties validated
5. **Error handling**: Edge cases and error conditions tested

## Running Tests

```bash
# Run individual module tests
cargo run --bin cursed stdlib/testz/test_testz_comprehensive.csd

# Run example stdlib module test
cargo run --bin cursed stdlib/testz/example_stdlib_module_test.csd

# Run all stdlib tests
cargo run --bin cursed test --test-dir stdlib
```

## Contributing

When adding new features to the TestZ framework:

1. Follow CURSED language conventions
2. Add comprehensive tests for new functionality
3. Update documentation with examples
4. Ensure backward compatibility
5. Test both interpretation and compilation modes

## Version History

- **v3.0**: Enhanced framework with advanced features
- **v2.0**: Basic testing framework with core assertions
- **v1.0**: Initial testing utilities

The TestZ framework provides everything needed for comprehensive stdlib module testing in CURSED, from basic assertions to advanced property-based testing and performance benchmarking.
