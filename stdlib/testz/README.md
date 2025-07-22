# CURSED Testing Framework (testz)

A comprehensive, pure CURSED testing framework designed for stdlib development and general testing needs.

## Overview

The testz framework provides advanced testing primitives, performance benchmarking, test isolation, and comprehensive reporting capabilities. It's built entirely in CURSED with no external dependencies or FFI usage.

## Quick Start

```cursed
yeet "testz"

# Basic test example
test_start("my first test")
assert_eq_int(2 + 2, 4)
assert_true(based)
print_test_summary()
```

## Core Features

### Basic Assertions
- `assert_eq_int(actual, expected)` - Assert integer equality
- `assert_eq_string(actual, expected)` - Assert string equality  
- `assert_true(condition)` - Assert boolean true
- `assert_false(condition)` - Assert boolean false

### Advanced Assertions
- `assert_not_eq_int(actual, not_expected)` - Assert integer inequality
- `assert_greater_than(actual, expected)` - Assert greater than comparison
- `assert_less_than(actual, expected)` - Assert less than comparison
- `assert_in_range(actual, min, max)` - Assert value within range
- `assert_contains_string(haystack, needle)` - Assert string contains substring

### Test Management
- `test_start(name)` - Begin a new test
- `test_end()` - End current test (with cleanup)
- `suite_start(name)` - Begin a test suite
- `suite_end()` - End current test suite

### Configuration
- `set_verbose_mode(enabled)` - Enable/disable verbose output
- `set_setup_function(func_name)` - Configure setup function
- `set_teardown_function(func_name)` - Configure teardown function

### Performance Benchmarking
- `benchmark_start()` - Start timing benchmark
- `benchmark_end()` - End timing and return elapsed time
- `benchmark_iterations(count)` - Set benchmark iteration count
- `benchmark_test(name, iterations)` - Run full benchmark test

### Error Testing
- `expect_error(message)` - Expect specific error condition
- `assert_throws(description)` - Assert that operation throws error

### Test Data Management
- `generate_test_data(size)` - Generate test data of specified size
- `create_temp_data(pattern)` - Create temporary test data
- `cleanup_temp_data(data_id)` - Clean up temporary test data

### State Management
- `reset_test_state()` - Reset all test counters and state
- `reset_suite_state()` - Reset suite-specific state
- `get_pass_count()` - Get number of passed assertions
- `get_fail_count()` - Get number of failed assertions
- `get_total_count()` - Get total number of tests run
- `get_suite_name()` - Get current suite name
- `is_verbose()` - Check if verbose mode is enabled

### Reporting
- `print_test_summary()` - Print comprehensive test report
- `print_detailed_report()` - Print detailed analysis with configuration

## Usage Examples

### Basic Test Suite

```cursed
yeet "testz"

slay test_math_operations() lit {
    suite_start("Math Operations")
    
    test_start("addition")
    assert_eq_int(2 + 3, 5)
    assert_eq_int(0 + 0, 0)
    test_end()
    
    test_start("subtraction")
    assert_eq_int(10 - 3, 7)
    assert_greater_than(5, 2)
    test_end()
    
    suite_end()
    damn based
}

test_math_operations()
print_test_summary()
```

### Advanced Test with Configuration

```cursed
yeet "testz"

slay test_with_setup() lit {
    # Configure testing environment
    set_verbose_mode(based)
    set_setup_function("initialize_data")
    set_teardown_function("cleanup_data")
    
    suite_start("Advanced Features")
    
    test_start("range validation")
    assert_in_range(50, 0, 100)
    assert_in_range(25, 20, 30)
    test_end()
    
    test_start("string operations")
    assert_eq_string("hello", "hello")
    assert_contains_string("hello world", "hello")
    test_end()
    
    suite_end()
    print_detailed_report()
    damn based
}
```

### Performance Benchmarking

```cursed
yeet "testz"

slay test_performance() lit {
    suite_start("Performance Tests")
    
    # Simple benchmark
    benchmark_test("loop performance", 1000)
    
    # Custom benchmark
    test_start("custom benchmark")
    benchmark_start()
    
    # Your performance-critical code here
    sus i normie = 0
    lowkey i < 5000 {
        sus result normie = i * i
        i = i + 1
    }
    
    sus elapsed normie = benchmark_end()
    assert_greater_than(elapsed, 0)
    test_end()
    
    suite_end()
    damn based
}
```

### Error Testing

```cursed
yeet "testz"

slay test_error_conditions() lit {
    suite_start("Error Handling")
    
    test_start("expected errors")
    expect_error("division by zero")
    assert_throws("invalid operation")
    test_end()
    
    suite_end()
    damn based
}
```

## Best Practices

### Test Organization
1. Use descriptive test names
2. Group related tests into suites
3. Keep individual tests focused and atomic
4. Use setup/teardown for common initialization

### Assertion Guidelines
1. Use the most specific assertion available
2. Include meaningful test descriptions
3. Test both positive and negative cases
4. Use range assertions for approximate values

### Performance Testing
1. Run benchmarks multiple times for accuracy
2. Set appropriate iteration counts
3. Test with realistic data sizes
4. Compare results across implementations

### Error Testing
1. Test expected error conditions
2. Verify error messages when possible
3. Test edge cases and boundary conditions
4. Ensure proper cleanup after errors

## Framework Architecture

### Pure CURSED Implementation
The testz framework is implemented entirely in CURSED with no external dependencies:
- No Rust FFI calls
- No external library dependencies
- Cross-platform compatible
- Self-contained and portable

### State Management
The framework maintains several types of state:
- Global test counters (total, pass, fail)
- Suite-specific counters and names
- Configuration settings (verbose mode, setup/teardown)
- Benchmark timing and iteration settings

### Thread Safety
While CURSED provides concurrency primitives, the current testz implementation is designed for single-threaded testing. Future versions may include concurrent test execution capabilities.

## Integration with Stdlib Development

The testz framework is specifically designed for CURSED stdlib development:

1. **Module Testing**: Each stdlib module should include a `test_module.csd` file
2. **Validation Pattern**: Use testz assertions for all validation
3. **Performance Benchmarks**: Include performance tests for critical functions
4. **Error Handling**: Test error conditions and edge cases

### Example Stdlib Module Test

```cursed
# stdlib/mymodule/test_mymodule.csd
yeet "testz"
yeet "mymodule"

slay test_module_functions() lit {
    suite_start("MyModule Tests")
    
    test_start("basic functionality")
    sus result normie = mymodule.calculate(10, 5)
    assert_eq_int(result, 15)
    test_end()
    
    test_start("error conditions")
    expect_error("invalid input")
    # Test error conditions here
    test_end()
    
    suite_end()
    print_test_summary()
    damn based
}

test_module_functions()
```

## Development Commands

```bash
# Test the testz framework itself
cargo run --bin cursed stdlib/testz/test_testz.csd

# Test specific functionality
cargo run --bin cursed stdlib/testz/test_testz.csd --verbose

# Benchmark testz performance
time cargo run --bin cursed stdlib/testz/test_testz.csd
```

## Version History

- **v2.0.0** - Enhanced testing primitives with advanced assertions, benchmarking, and suite management
- **v1.0.0** - Basic testing framework with core assertions and reporting

## Contributing

When contributing to testz:

1. Maintain pure CURSED implementation (no FFI)
2. Add comprehensive tests for new features
3. Update documentation for new functions
4. Follow established naming conventions
5. Ensure backward compatibility

## License

Part of the CURSED programming language stdlib - see main project license.
