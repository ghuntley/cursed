# CURSED Testing Framework v5.0

A production-ready testing framework written entirely in the CURSED programming language. This framework provides comprehensive testing utilities, assertions, and reporting capabilities for CURSED programs.

## Features

- **Complete Assertion Library**: 20+ assertion functions for all CURSED data types
- **Test Lifecycle Management**: Comprehensive test organization and execution
- **Multiple Output Formats**: Console, JSON, TAP, and HTML reporting
- **Production-Ready Design**: Robust error handling and comprehensive reporting
- **Pure CURSED Implementation**: No external dependencies, written entirely in CURSED
- **Both Mode Support**: Works in both interpretation and compilation modes

## Core Assertion Functions

### Integer Assertions
- `assert_eq_int(actual, expected)` - Assert integer equality
- `assert_ne_int(actual, expected)` - Assert integer inequality
- `assert_greater_than(actual, expected)` - Assert greater than
- `assert_less_than(actual, expected)` - Assert less than
- `assert_greater_equal(actual, expected)` - Assert greater than or equal
- `assert_less_equal(actual, expected)` - Assert less than or equal
- `assert_in_range(actual, min, max)` - Assert value is within range

### String Assertions
- `assert_eq_string(actual, expected)` - Assert string equality
- `assert_ne_string(actual, expected)` - Assert string inequality
- `assert_string_contains(haystack, needle)` - Assert string contains substring
- `assert_string_starts_with(text, prefix)` - Assert string starts with prefix
- `assert_string_ends_with(text, suffix)` - Assert string ends with suffix

### Boolean Assertions
- `assert_eq_bool(actual, expected)` - Assert boolean equality
- `assert_true(value)` - Assert value is true (based)
- `assert_false(value)` - Assert value is false (cap)

### Float Assertions
- `assert_eq_float(actual, expected)` - Assert float equality with default tolerance
- `assert_eq_float_with_tolerance(actual, expected, tolerance)` - Assert float equality with custom tolerance

### Nil Assertions
- `assert_nil(value)` - Assert value is nil (cringe)
- `assert_not_nil(value)` - Assert value is not nil

## Test Lifecycle Functions

### Test Organization
- `test_start(name)` - Begin a new test
- `test_end()` - End current test
- `suite_start(name)` - Begin a new test suite
- `suite_end()` - End current test suite

### Test Results
- `test_pass(message)` - Record a test pass
- `test_fail(message)` - Record a test failure
- `test_skip(reason)` - Skip a test with reason
- `test_error(message)` - Record a test error

## Configuration Functions

### Output Control
- `enable_verbose_output()` - Enable verbose test output
- `disable_verbose_output()` - Disable verbose test output
- `enable_fail_fast()` - Stop on first failure
- `disable_fail_fast()` - Continue after failures

### Report Formats
- `enable_json_output()` - Enable JSON report generation
- `enable_tap_output()` - Enable TAP report generation
- `enable_html_output()` - Enable HTML report generation
- `enable_xml_output()` - Enable XML report generation

### Timeouts and Limits
- `set_timeout(seconds)` - Set test timeout
- `set_max_failures(max)` - Set maximum failures before stopping

## Reporting Functions

### Summary Reports
- `print_test_summary()` - Print comprehensive test summary
- `run_all_tests()` - Run all tests and return exit code

### Format Generation
- `generate_json_report()` - Generate JSON test report
- `generate_tap_report()` - Generate TAP test report
- `generate_html_report()` - Generate HTML test report
- `generate_xml_report()` - Generate XML test report

## State Management

### Test Statistics
- `get_test_count()` - Get total test count
- `get_passed_count()` - Get passed test count
- `get_failed_count()` - Get failed test count
- `get_skipped_count()` - Get skipped test count
- `get_error_count()` - Get error test count

### State Control
- `reset_test_state()` - Reset all test counters and state

## Usage Examples

### Basic Testing

```cursed
yeet "testz"

slay test_basic_math() {
    test_start("test_basic_math")
    
    assert_eq_int(2 + 2, 4)
    assert_eq_int(10 - 5, 5)
    assert_eq_int(3 * 4, 12)
    assert_eq_int(20 / 4, 5)
    
    test_end()
}

slay main() {
    test_basic_math()
    print_test_summary()
}
```

### Advanced Testing

```cursed
yeet "testz"

slay test_string_operations() {
    test_start("test_string_operations")
    
    assert_eq_string("hello" + " world", "hello world")
    assert_string_contains("hello world", "world")
    assert_string_starts_with("hello world", "hello")
    assert_string_ends_with("hello world", "world")
    
    test_end()
}

slay test_boolean_logic() {
    test_start("test_boolean_logic")
    
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
    assert_true((5 > 3) && (2 < 4))
    
    test_end()
}

slay main() {
    enable_verbose_output()
    enable_json_output()
    
    suite_start("Comprehensive Tests")
    
    test_string_operations()
    test_boolean_logic()
    
    suite_end()
    
    print_test_summary()
    generate_json_report()
}
```

### Test Suite Organization

```cursed
yeet "testz"

slay test_arithmetic() {
    test_start("test_arithmetic")
    
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_greater_than(c, a)
    assert_greater_than(c, b)
    assert_in_range(c, 25, 35)
    
    test_end()
}

slay test_variables() {
    test_start("test_variables")
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    assert_string_contains(greeting, name)
    assert_ne_string(greeting, name)
    
    test_end()
}

slay main() {
    enable_verbose_output()
    enable_fail_fast()
    
    suite_start("Variable and Expression Tests")
    
    test_arithmetic()
    test_variables()
    
    suite_end()
    
    sus exit_code normie = run_all_tests()
    damn exit_code
}
```

## Output Formats

### Console Output
Default human-readable output with colored indicators and comprehensive summaries.

### JSON Output
```json
{
  "framework": "CURSED Testing Framework v5.0",
  "suite_name": "default",
  "total_tests": 5,
  "passed_tests": 4,
  "failed_tests": 1,
  "skipped_tests": 0,
  "error_tests": 0
}
```

### TAP Output
```tap
TAP version 13
1..5
ok 1 - test passed
ok 2 - test passed
not ok 3 - test failed
ok 4 - test passed
ok 5 - test passed
```

### HTML Output
Complete HTML report with styling and detailed test information.

## Commands

### Running Tests

```bash
# Run CURSED test file in interpretation mode
cargo run --bin cursed my_test.csd

# Compile and run CURSED test file
cargo run --bin cursed -- compile my_test.csd
./my_test

# Run all stdlib tests
cargo run --bin cursed test --test-dir stdlib

# Run tests with specific pattern
cargo run --bin cursed test --filter math
```

### Using the Framework

```bash
# Test the framework itself
cargo run --bin cursed cursed_test_final.csd

# Test in both modes
cargo run --bin cursed cursed_test_final.csd
cargo run --bin cursed -- compile cursed_test_final.csd
./cursed_test_final
```

## Best Practices

1. **Organize Tests**: Use meaningful test names and group related tests
2. **Use Descriptive Assertions**: Include clear messages in custom assertions
3. **Test Edge Cases**: Include boundary conditions and error cases
4. **Use Appropriate Assertions**: Choose the most specific assertion for each case
5. **Configure Output**: Enable appropriate output formats for your needs
6. **Test Both Modes**: Verify tests work in both interpretation and compilation modes

## Integration with Stdlib Modules

The testing framework is designed to work seamlessly with all CURSED stdlib modules:

```cursed
yeet "testz"
yeet "math"
yeet "string"
yeet "crypto"

slay test_math_module() {
    test_start("test_math_module")
    
    assert_eq_int(math.add(2, 3), 5)
    assert_eq_int(math.multiply(4, 5), 20)
    
    test_end()
}
```

## Performance Considerations

- Tests run efficiently in both interpretation and compilation modes
- Native compilation provides significant performance improvements for large test suites
- Memory usage is optimized for long-running test sessions
- Parallel test execution is supported for faster test runs

## Contributing

The framework is implemented in pure CURSED and can be extended by adding new assertion functions or output formats. All contributions should maintain compatibility with both execution modes.

## License

This testing framework is part of the CURSED programming language project and follows the same license terms.
