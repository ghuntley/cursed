# QuickTest - Advanced Testing Framework for CURSED

QuickTest is a comprehensive testing framework that extends beyond the basic `testz` module to provide enterprise-grade testing capabilities including property-based testing, fixtures, benchmarks, mocks, and advanced reporting.

## Features

### 🚀 Core Testing Capabilities
- **Property-Based Testing**: Generate test data and verify properties automatically
- **Test Fixtures**: Manage test data and environment setup
- **Setup/Teardown**: Automatic test environment initialization and cleanup
- **Parameterized Tests**: Run the same test with multiple input parameters
- **Benchmark Testing**: Performance measurement with timing analysis
- **Mock/Stub System**: Create fake implementations for testing isolation
- **Test Discovery**: Automatically find and organize tests
- **Multi-Format Reporting**: Generate reports in JSON, XML, HTML, and text formats
- **testz Integration**: Full compatibility with existing testz framework

## Quick Start

```cursed
yeet "quick_test"

# Start a test suite
qt_start_suite("My Test Suite")

# Add fixtures
qt_add_fixture("test_data", "sample_value")

# Setup and teardown
qt_setup("initialize_environment")
qt_teardown("cleanup_environment")

# Property-based testing
qt_property_test("string_properties", "string", "non_empty")

# Parameterized testing
test_params := ["param1", "param2", "param3"]
qt_parameterized_test("my_test", test_params, 3, "string_length")

# Benchmark testing
avg_time := qt_benchmark("performance_test", "string_concat")

# Mock system
qt_create_mock("database_query", "mock_result")
result := qt_call_mock("database_query")

# Generate reports
qt_set_report_format("json")
qt_generate_report()

# End test suite
qt_end_suite()
```

## API Reference

### Suite Management

#### `qt_start_suite(suite_name tea) lit`
Starts a new test suite with the given name.

#### `qt_end_suite() lit`
Ends the current test suite and displays summary.

### Property-Based Testing

#### `qt_property_test(test_name tea, generator_func tea, property_func tea) lit`
Runs property-based testing with generated data.

**Generators:**
- `"int"` - Generate random integers
- `"string"` - Generate random strings
- `"bool"` - Generate random booleans

**Properties:**
- `"non_empty"` - Verify strings are not empty
- `"positive"` - Verify numbers are positive
- `"idempotent"` - Verify operations are idempotent

#### `qt_generate_data(generator tea) tea`
Generate test data using specified generator.

#### `qt_apply_property(property_func tea, test_data tea) lit`
Apply property function to test data and verify result.

### Test Fixtures

#### `qt_add_fixture(fixture_name tea, fixture_data tea) lit`
Add a test fixture with name and data.

#### `qt_get_fixture(fixture_name tea) tea`
Retrieve fixture data by name.

#### `qt_setup(setup_func tea) lit`
Register setup function to run before tests.

#### `qt_teardown(teardown_func tea) lit`
Register teardown function to run after tests.

#### `qt_run_setup() lit`
Execute registered setup function.

#### `qt_run_teardown() lit`
Execute registered teardown function.

### Parameterized Testing

#### `qt_parameterized_test(test_name tea, params [10]tea, param_count normie, test_func tea) lit`
Run test with multiple parameters.

**Test Functions:**
- `"string_length"` - Test string length >= 0
- `"string_not_empty"` - Test string is not empty
- `"numeric_positive"` - Test number is positive

### Benchmark Testing

#### `qt_benchmark(bench_name tea, bench_func tea) normie`
Run performance benchmark and return average execution time in nanoseconds.

**Benchmark Functions:**
- `"string_concat"` - String concatenation performance
- `"math_operation"` - Mathematical operation performance
- `"array_access"` - Array access performance

### Mock and Stub System

#### `qt_create_mock(function_name tea, return_value tea) lit`
Create a mock function that returns specified value.

#### `qt_create_stub(function_name tea, stub_response tea) lit`
Create a stub function with specified response.

#### `qt_call_mock(function_name tea) tea`
Call mock function and get return value.

#### `qt_call_stub(function_name tea) tea`
Call stub function and get response.

### Test Discovery and Organization

#### `qt_discover_tests(pattern tea) normie`
Discover tests matching the given pattern.

#### `qt_organize_tests(category tea) lit`
Organize tests by category.

**Categories:**
- `"unit"` - Unit tests
- `"integration"` - Integration tests
- `"performance"` - Performance tests
- `"all"` - All tests

### Reporting System

#### `qt_set_report_format(format tea) lit`
Set the report output format.

**Formats:**
- `"json"` - JSON format
- `"xml"` - XML format
- `"html"` - HTML format
- `"text"` - Plain text format

#### `qt_generate_report() lit`
Generate test report in current format.

### testz Integration

#### `qt_testz_assert_true(condition lit) lit`
testz-compatible assertion for true conditions.

#### `qt_testz_assert_eq(actual tea, expected tea) lit`
testz-compatible equality assertion.

#### `qt_run_with_testz(test_name tea) lit`
Run test with full testz compatibility.

## Advanced Examples

### Property-Based Testing Example

```cursed
yeet "quick_test"

# Test that string operations preserve certain properties
qt_property_test("string_reverse_property", "string", "idempotent")

# Test that mathematical operations maintain positivity
qt_property_test("math_positive_property", "int", "positive")
```

### Comprehensive Test Suite Example

```cursed
yeet "quick_test"

slay run_comprehensive_tests() lit {
    # Start test suite
    qt_start_suite("Comprehensive Test Suite")
    
    # Setup test environment
    qt_add_fixture("database", "test_db_connection")
    qt_add_fixture("config", "test_configuration")
    qt_setup("initialize_test_database")
    qt_teardown("cleanup_test_database")
    
    # Property-based testing
    qt_property_test("user_input_validation", "string", "non_empty")
    qt_property_test("calculation_accuracy", "int", "positive")
    
    # Parameterized testing
    test_inputs := ["input1", "input2", "input3", "input4"]
    qt_parameterized_test("input_processing", test_inputs, 4, "string_length")
    
    # Performance benchmarking
    processing_time := qt_benchmark("data_processing", "string_concat")
    calculation_time := qt_benchmark("mathematical_ops", "math_operation")
    
    # Mock external dependencies
    qt_create_mock("external_api_call", "success_response")
    qt_create_stub("database_query", "test_data_result")
    
    # Test with mocks
    api_result := qt_call_mock("external_api_call")
    db_result := qt_call_stub("database_query")
    
    # Generate comprehensive reports
    qt_set_report_format("json")
    qt_generate_report()
    
    qt_set_report_format("html")
    qt_generate_report()
    
    # End test suite
    qt_end_suite()
    
    damn based
}

# Run the comprehensive test suite
run_comprehensive_tests()
```

### Mock Testing Example

```cursed
yeet "quick_test"

slay test_with_mocks() lit {
    # Create mocks for external dependencies
    qt_create_mock("payment_processor", "payment_successful")
    qt_create_mock("email_service", "email_sent")
    qt_create_stub("user_database", "user_found")
    
    # Test business logic with mocked dependencies
    payment_result := qt_call_mock("payment_processor")
    email_result := qt_call_mock("email_service")
    user_data := qt_call_stub("user_database")
    
    # Verify mock interactions
    qt_testz_assert_eq(payment_result, "payment_successful")
    qt_testz_assert_eq(email_result, "email_sent")
    qt_testz_assert_eq(user_data, "user_found")
    
    damn based
}
```

## Integration with testz

QuickTest is fully compatible with the existing testz framework. You can use testz functions within QuickTest or use QuickTest functions within testz test suites.

```cursed
yeet "testz"
yeet "quick_test"

# Use testz within QuickTest
qt_start_suite("Mixed Testing")
test_start("basic_test")
assert_true(based)
qt_benchmark("performance_check", "string_concat")
print_test_summary()
qt_end_suite()

# Use QuickTest within testz
test_start("advanced_test")
qt_property_test("property_check", "string", "non_empty")
assert_true(based)
print_test_summary()
```

## Performance Considerations

- Property-based tests run 100 iterations by default
- Benchmarks run 1000 iterations by default
- Mock/stub systems support up to 50 functions each
- Fixture system supports up to 100 fixtures
- Test results can store up to 1000 entries

## Testing the Framework

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/quick_test/test_quick_test.💀
```

This will test all QuickTest features and provide a demonstration of capabilities.

## Contributing

When adding new features to QuickTest:

1. Follow CURSED language syntax conventions
2. Add comprehensive tests for new functionality
3. Update this documentation
4. Ensure testz compatibility
5. Test both interpretation and compilation modes

## License

QuickTest is part of the CURSED standard library and follows the same license terms as the CURSED language project.
