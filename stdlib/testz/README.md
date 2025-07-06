# CURSED Testing Framework (testz)

A comprehensive testing framework for the CURSED programming language that allows writing tests in CURSED itself.

## Features

- **Test Functions**: Define test cases using standard CURSED functions
- **Assertions**: Comprehensive assertion library for different data types
- **Test Discovery**: Automatic discovery of test functions
- **Test Runner**: Execute tests and report results
- **Multiple Output Formats**: JSON, XML, HTML, and console output
- **Benchmarking**: Simple performance measurement utilities
- **Configuration**: Customizable test execution settings

## Quick Start

### 1. Import the testing framework

```cursed
yeet "testz"
```

### 2. Write your first test

```cursed
slay test_basic_math() {
    testz.test_start("test_basic_math")
    
    sus result normie = 2 + 2
    testz.assert_eq(result, 4)
    
    sus message tea = "Hello, CURSED!"
    testz.assert_eq_string(message, "Hello, CURSED!")
}
```

### 3. Run your tests

```cursed
slay main() {
    test_basic_math()
    testz.print_test_summary()
    
    lowkey testz.test_failed > 0 {
        yolo 1
    } highkey {
        yolo 0
    }
}
```

## Assertion Functions

### Integer Assertions
- `assert_eq(actual, expected)` - Assert two integers are equal
- `assert_ne(actual, expected)` - Assert two integers are not equal
- `assert_greater_than(actual, expected)` - Assert actual > expected
- `assert_less_than(actual, expected)` - Assert actual < expected
- `assert_in_range(value, min, max)` - Assert value is in range

### Float Assertions
- `assert_eq_float(actual, expected)` - Assert two floats are equal (with tolerance)

### String Assertions
- `assert_eq_string(actual, expected)` - Assert two strings are equal
- `assert_ne_string(actual, expected)` - Assert two strings are not equal
- `assert_string_contains(haystack, needle)` - Assert string contains substring
- `assert_string_starts_with(str, prefix)` - Assert string starts with prefix
- `assert_string_ends_with(str, suffix)` - Assert string ends with suffix

### Boolean Assertions
- `assert_true(value)` - Assert value is `based` (true)
- `assert_false(value)` - Assert value is `cap` (false)
- `assert_eq_bool(actual, expected)` - Assert two booleans are equal

### Array Assertions
- `assert_array_eq(actual, expected)` - Assert two arrays are equal
- `assert_array_contains(array, value)` - Assert array contains value
- `assert_array_not_contains(array, value)` - Assert array doesn't contain value

### Nil Assertions
- `assert_nil(value)` - Assert value is `cringe` (nil)
- `assert_not_nil(value)` - Assert value is not `cringe`

## Test Structure

### Basic Test Structure
```cursed
slay test_function_name() {
    testz.test_start("test_function_name")
    
    fr fr Test setup
    sus data normie = 42
    
    fr fr Test execution
    sus result normie = some_operation(data)
    
    fr fr Assertions
    testz.assert_eq(result, expected_value)
}
```

### Test with Multiple Assertions
```cursed
slay test_array_operations() {
    testz.test_start("test_array_operations")
    
    sus arr [normie] = [1, 2, 3, 4, 5]
    
    testz.assert_eq(arr.length, 5)
    testz.assert_eq(arr[0], 1)
    testz.assert_eq(arr[4], 5)
    testz.assert_array_contains(arr, 3)
    testz.assert_array_not_contains(arr, 10)
}
```

## Test Discovery and Execution

### Manual Test Execution
```cursed
slay main() {
    testz.reset_test_state()
    
    fr fr Run individual tests
    test_function_1()
    test_function_2()
    test_function_3()
    
    fr fr Print results
    testz.print_test_summary()
    
    fr fr Return appropriate exit code
    lowkey testz.test_failed > 0 {
        yolo 1
    } highkey {
        yolo 0
    }
}
```

### Automatic Test Discovery
```cursed
yeet "testz/runner"

slay main() {
    sus config testz.TestConfig = testz.create_default_config()
    config.test_dir = "tests/"
    config.pattern = "test_*"
    config.verbose = based
    
    sus exit_code normie = testz.run_tests_with_config(config)
    yolo exit_code
}
```

## Test Configuration

```cursed
struct TestConfig {
    parallel lit        fr fr Run tests in parallel (future feature)
    timeout normie      fr fr Test timeout in milliseconds
    verbose lit         fr fr Verbose output
    fail_fast lit       fr fr Stop on first failure
    test_dir tea        fr fr Directory to search for tests
    pattern tea         fr fr Pattern to match test functions
}
```

## Test Output Formats

### Console Output (Default)
```
Running test: test_basic_math
  ✓ PASS: assert_eq(4, 4)
  ✓ PASS: assert_eq_string("Hello, CURSED!", "Hello, CURSED!")

=== TEST SUMMARY ===
Total tests: 1
Passed: 2
Failed: 0
Success rate: 100%

🎉 ALL TESTS PASSED! 🎉
```

### JSON Output
```cursed
testz.generate_json_report()
```

### XML Output
```cursed
testz.generate_xml_report()
```

### HTML Output
```cursed
testz.generate_html_report()
```

## Benchmarking

```cursed
slay test_performance() {
    testz.test_start("test_performance")
    
    sus start_time normie = testz.benchmark_start()
    
    fr fr Your code to benchmark
    sus result normie = expensive_operation()
    
    testz.benchmark_end(start_time)
    testz.assert_eq(result, expected_result)
}
```

## Test Utilities

### Test Data Creation
```cursed
sus test_array [normie] = testz.create_test_array()
sus test_string tea = testz.create_test_string()
sus test_struct TestResult = testz.create_test_struct()
```

### Test Control
```cursed
testz.skip_test("Reason for skipping")
testz.fail_test("Explicit failure reason")
testz.expect_panic(risky_function)
```

## Integration with CURSED Compiler

### Running Tests in Interpretation Mode
```bash
cargo run --bin cursed tests/testz/test_basic_assertions.csd
```

### Running Tests in Compilation Mode
```bash
cargo run --bin cursed -- compile tests/testz/test_basic_assertions.csd
./test_basic_assertions
```

### Running All Tests
```bash
# Run all tests in the tests/testz/ directory
find tests/testz/ -name "*.csd" -exec cargo run --bin cursed {} \;
```

## Best Practices

1. **Name tests descriptively**: Use clear, descriptive names for test functions
2. **One concept per test**: Each test should focus on one specific behavior
3. **Arrange-Act-Assert**: Structure tests with clear setup, execution, and verification
4. **Use appropriate assertions**: Choose the most specific assertion for your use case
5. **Test edge cases**: Include tests for boundary conditions and error cases
6. **Keep tests independent**: Tests should not depend on each other's state

## Examples

See the `tests/testz/` directory for comprehensive examples:

- `test_basic_assertions.csd` - Basic assertion examples
- `test_array_operations.csd` - Array testing examples
- `test_advanced_features.csd` - Advanced language feature tests
- `test_runner_demo.csd` - Complete testing framework demonstration

## Future Enhancements

- Macro support for more concise test definitions
- Parallel test execution
- Code coverage reporting
- Property-based testing
- Mocking and stubbing utilities
- Integration with external testing tools

## Contributing

The testing framework is part of the CURSED standard library. To contribute:

1. Write tests for your changes
2. Ensure all existing tests pass
3. Update documentation as needed
4. Follow CURSED coding conventions
