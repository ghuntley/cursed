# CURSED Test Runner Documentation

The CURSED Test Runner is a comprehensive testing framework designed for the CURSED programming language, providing test discovery, execution, reporting, and assertion utilities.

## Table of Contents

1. [Overview](#overview)
2. [Getting Started](#getting-started)
3. [Test Writing Guide](#test-writing-guide)
4. [Command Line Interface](#command-line-interface)
5. [Configuration](#configuration)
6. [Assertions](#assertions)
7. [Test Fixtures](#test-fixtures)
8. [Reporting](#reporting)
9. [Performance Testing](#performance-testing)
10. [Integration Testing](#integration-testing)
11. [Advanced Features](#advanced-features)
12. [API Reference](#api-reference)

## Overview

The CURSED Test Runner provides:

- **Test Discovery**: Automatic discovery of test files and functions using glob patterns
- **Test Execution**: Parallel execution with configurable concurrency
- **Rich Assertions**: Comprehensive assertion library with detailed error messages
- **Multiple Report Formats**: Console, JSON, XML, HTML, CSV, and Markdown reports
- **Test Fixtures**: Setup/teardown functionality with dependency management
- **Performance Testing**: Built-in benchmarking and performance metrics
- **Coverage Collection**: Test coverage analysis and reporting
- **Watch Mode**: Automatic re-execution on file changes

## Getting Started

### Installation

The test runner is included with the CURSED language installation. No additional setup is required.

### Writing Your First Test

Create a test file with the `.csd` extension and test functions starting with `test_`:

```cursed
// test_example.csd
import "stdlib::testing::assertions"

slay test_basic_math() {
    sus result = 2 + 2
    assert_equal(result, 4, "Basic addition should work")
}

slay test_string_operations() {
    sus greeting = "Hello, World!"
    assert_contains(greeting, "World", "Should contain 'World'")
    assert_length(greeting.chars(), 13, "Should have correct length")
}
```

### Running Tests

```bash
# Run all tests in current directory
cursed test

# Run specific test pattern
cursed test --pattern math

# Run with verbose output
cursed test --verbose

# Run with JSON output
cursed test --format json
```

## Test Writing Guide

### Test Function Naming

Test functions must follow naming conventions:

- Start with `test_` for unit tests
- Start with `integration_` for integration tests
- Start with `bench_` for benchmark tests
- Include descriptive names: `test_user_creation_with_valid_data`

### Test Structure

```cursed
slay test_function_name() {
    // Arrange - Set up test data
    sus user = User {
        id: 1,
        name: "Alice",
        email: "alice@example.com"
    }
    
    // Act - Perform the operation
    sus result = validate_user(user)
    
    // Assert - Verify the outcome
    assert_ok(result, "User validation should succeed")
    assert_equal(result.unwrap().id, 1, "User ID should be preserved")
}
```

### Error Handling Tests

```cursed
slay test_error_conditions() {
    sus result = divide(10, 0)
    assert_err(result, "Division by zero should fail")
    
    sus error = result.unwrap_err()
    assert_contains(error.message, "zero", "Error message should mention zero")
}
```

### Async Test Support

```cursed
slay test_async_operation() {
    sus future = async_fetch_data()
    sus result = await future
    
    assert_ok(result, "Async operation should succeed")
}
```

## Command Line Interface

### Basic Commands

```bash
# Run all tests
cursed test

# Run specific pattern
cursed test --pattern "user_*"

# Run specific file
cursed test path/to/test_file.csd

# Run in watch mode
cursed test --watch

# Show help
cursed test --help
```

### Advanced Options

```bash
# Parallel execution
cursed test --jobs 4

# Timeout configuration
cursed test --timeout 60

# Coverage collection
cursed test --coverage

# Fail fast mode
cursed test --fail-fast

# Dry run (discover tests only)
cursed test --dry-run

# Random test order
cursed test --randomize --seed 42

# Custom output format
cursed test --format json --output test_results.json
```

### Watch Mode

```bash
# Basic watch mode
cursed test --watch

# Custom watch patterns
cursed test --watch --watch-pattern "*.csd" --watch-pattern "*.toml"

# Custom debounce delay
cursed test --watch --debounce 1000

# Watch with pattern filtering
cursed test --watch --pattern "integration_*"
```

## Configuration

### File-based Configuration

Create a `cursed-test.toml` file in your project root:

```toml
[test]
# Test discovery patterns
include_patterns = ["**/*_test.csd", "**/test_*.csd"]
exclude_patterns = ["target/**", ".git/**"]

# Execution settings
max_parallel_tests = 4
timeout_seconds = 30
fail_fast = false

# Output settings
verbose = false
coverage = false

# Working directory
working_directory = "."

# Test data directory
test_data_dir = "test_data"

[environment]
# Environment variables for tests
TEST_ENV = "development"
DEBUG = "false"

[watch]
# Watch mode settings
patterns = ["*.csd", "*.toml"]
debounce_ms = 500
ignore = ["*.tmp", "target/*"]
```

### Environment Variables

```bash
# Set test environment
export CURSED_TEST_ENV=development

# Enable debug output
export CURSED_TEST_DEBUG=true

# Custom test data directory
export CURSED_TEST_DATA_DIR=/path/to/test/data
```

## Assertions

### Basic Assertions

```cursed
// Boolean assertions
assert_true(condition, "Should be true")
assert_false(condition, "Should be false")

// Equality assertions
assert_equal(actual, expected, "Values should be equal")
assert_not_equal(actual, unexpected, "Values should not be equal")

// Null/Option assertions
assert_null(value, "Should be null")
assert_not_null(value, "Should not be null")
assert_some(option, "Should be Some")
assert_none(option, "Should be None")
```

### String Assertions

```cursed
// String content assertions
assert_contains(text, "substring", "Should contain substring")
assert_not_contains(text, "substring", "Should not contain substring")
assert_starts_with(text, "prefix", "Should start with prefix")
assert_ends_with(text, "suffix", "Should end with suffix")

// String properties
assert_empty(text, "Should be empty")
assert_not_empty(text, "Should not be empty")
assert_length(text, 10, "Should have length 10")
```

### Collection Assertions

```cursed
// Collection content
assert_empty(collection, "Should be empty")
assert_not_empty(collection, "Should not be empty")
assert_length(collection, 5, "Should have 5 items")
assert_contains_item(collection, &item, "Should contain item")

// Collection properties
assert_sorted(collection, "Should be sorted")
assert_all_unique(collection, "All items should be unique")
```

### Numeric Assertions

```cursed
// Range assertions
assert_in_range(value, min, max, "Should be in range")
assert_greater_than(value, threshold, "Should be greater")
assert_less_than(value, threshold, "Should be less")

// Floating point assertions
assert_float_equal(actual, expected, epsilon, "Should be approximately equal")
assert_close_to(actual, expected, tolerance, "Should be close")
```

### Custom Assertions

```cursed
// Custom predicate assertion
assert_that(value, |x| x > 0 && x < 100, "Should be between 0 and 100")

// Builder pattern assertions
assert_that(user)
    .with_context("User validation")
    .equals(expected_user)

assert_that(result)
    .with_context("Operation result")
    .is_ok()
    .map(|value| value.id)
    .equals(42)
```

## Test Fixtures

### Simple Fixtures

```cursed
// Global setup/teardown
slay setup() {
    // Initialize test environment
    initialize_database()
    create_test_data()
}

slay teardown() {
    // Clean up test environment
    cleanup_database()
    remove_test_files()
}
```

### Fixture Classes

```cursed
squad DatabaseFixture {
    sus connection: DatabaseConnection,
    sus test_data: TestData,
}

impl TestFixture for DatabaseFixture {
    slay setup(&mut self) -> Result<(), TestError> {
        self.connection = create_test_database()?
        self.test_data = load_test_data()?
        Ok(())
    }
    
    slay teardown(&mut self) -> Result<(), TestError> {
        cleanup_test_database(&self.connection)?
        Ok(())
    }
}
```

### Fixture Dependencies

```cursed
// Register fixtures with dependencies
fixture_manager.register_fixture("database", database_fixture)
fixture_manager.register_fixture("http_server", http_fixture)
fixture_manager.register_dependencies("http_server", ["database"])
```

### Using Fixtures

```cursed
slay test_with_fixtures() {
    sus fixture_manager = get_fixture_manager()
    sus db_data = fixture_manager.get_fixture_data("database")?
    
    sus connection = db_data.get("connection")?
    sus result = query_database(connection, "SELECT * FROM users")
    
    assert_not_empty(result, "Should return some users")
}
```

## Reporting

### Console Output

The default console reporter provides color-coded output:

```
🧪 CURSED Test Results
═══════════════════════

✅ Test Run PASSED
   Total:   25
   Passed:  23 ✅
   Failed:  2 ❌
   Success: 92.0%
   Duration: 2.45s

📦 Suite: user_management
   Duration: 1.23s
   Failures:
     ❌ test_user_validation: Invalid email format not caught

📦 Suite: data_processing
   Duration: 0.89s

⚡ Performance Metrics
   Compilation: 0.45s (cache hit rate: 85.2%)
   Execution:   1.89s (throughput: 13.2 tests/s)
   Memory:      Peak 45.2MB, Avg 23.1MB
```

### JSON Output

```bash
cursed test --format json --output results.json
```

```json
{
  "summary": {
    "total_tests": 25,
    "passed": 23,
    "failed": 2,
    "skipped": 0,
    "success_rate": 92.0,
    "total_duration": "2.45s"
  },
  "suite_results": [
    {
      "suite_name": "user_management",
      "test_results": [
        {
          "test_function": {
            "name": "test_user_creation",
            "test_type": "Unit"
          },
          "status": "Passed",
          "duration": "0.023s"
        }
      ]
    }
  ],
  "performance": {
    "compilation": {
      "total_duration": "0.45s",
      "cache_hit_rate": 0.852
    }
  }
}
```

### XML Output (JUnit Compatible)

```bash
cursed test --format xml --output results.xml
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites tests="25" failures="2" time="2.45">
  <testsuite name="user_management" tests="10" time="1.23">
    <testcase name="test_user_creation" time="0.023" />
    <testcase name="test_user_validation" time="0.045">
      <failure message="Invalid email format not caught">
        Expected validation to fail for invalid email
      </failure>
    </testcase>
  </testsuite>
</testsuites>
```

### HTML Output

```bash
cursed test --format html --output report.html
```

Generates a comprehensive HTML report with:
- Interactive test results table
- Filtering and sorting capabilities
- Performance charts
- Coverage visualization

## Performance Testing

### Benchmark Tests

```cursed
slay bench_string_operations() {
    sus iterations = 10000
    sus start = current_time_nanos()
    
    lowkey (sus i = 0; i < iterations; i++) {
        sus result = "hello" + " world"
    }
    
    sus duration = current_time_nanos() - start
    record_metric("string_concat_ns_per_op", duration / iterations)
}
```

### Performance Assertions

```cursed
slay test_performance_requirements() {
    sus start = current_time_ms()
    
    sus result = expensive_operation()
    
    sus duration = current_time_ms() - start
    assert_less_than(duration, 1000, "Should complete in under 1 second")
    
    assert_memory_usage_under(50_000_000, "Should use less than 50MB")
}
```

### Memory Profiling

```cursed
slay test_memory_usage() {
    enable_memory_tracking()
    
    sus large_array = allocate_large_array(1000000)
    process_array(large_array)
    
    sus stats = get_memory_stats()
    assert_less_than(stats.peak_memory_mb, 100, "Should not exceed 100MB")
    assert_equal(stats.memory_leaks, 0, "Should not have memory leaks")
}
```

## Integration Testing

### Multi-Component Tests

```cursed
slay test_full_user_workflow() {
    // Setup test environment
    sus server = start_test_server()
    sus database = create_test_database()
    
    // Test complete workflow
    sus user_data = UserRegistration {
        name: "Alice Smith",
        email: "alice@example.com",
        password: "secure_password"
    }
    
    // Register user
    sus response = server.post("/register", user_data)
    assert_equal(response.status, 201, "Registration should succeed")
    
    // Verify user in database
    sus stored_user = database.find_user_by_email("alice@example.com")
    assert_not_null(stored_user, "User should be stored in database")
    
    // Test login
    sus login_response = server.post("/login", {
        email: "alice@example.com",
        password: "secure_password"
    })
    assert_equal(login_response.status, 200, "Login should succeed")
    assert_not_null(login_response.headers.get("Authorization"), "Should return auth token")
}
```

### External Service Testing

```cursed
slay test_external_api_integration() {
    sus mock_server = create_mock_server()
    mock_server.expect_get("/api/users/1")
        .return_json({
            id: 1,
            name: "Test User",
            email: "test@example.com"
        })
    
    sus api_client = ApiClient::new(mock_server.url())
    sus user = api_client.get_user(1)
    
    assert_ok(user, "API call should succeed")
    assert_equal(user.unwrap().name, "Test User", "Should return correct user")
}
```

## Advanced Features

### Parameterized Tests

```cursed
slay test_parameterized_math(test_cases: &[(i32, i32, i32)]) {
    for (a, b, expected) in test_cases {
        sus result = add(*a, *b)
        assert_equal(result, *expected, &format!("{} + {} should equal {}", a, b, expected))
    }
}

// Test data
facts test_cases = [
    (1, 2, 3),
    (5, 5, 10),
    (-1, 1, 0),
    (0, 0, 0),
]
```

### Property-Based Testing

```cursed
slay test_sort_properties() {
    sus generator = arbitrary_vec::<i32>(0..1000)
    
    property_test(generator, |input| {
        sus sorted = sort(input.clone())
        
        // Property: sorted array has same length
        assert_equal(sorted.len(), input.len(), "Length should be preserved")
        
        // Property: sorted array is actually sorted
        assert_is_sorted(sorted, "Array should be sorted")
        
        // Property: sorted array contains same elements
        assert_same_elements(sorted, input, "Elements should be preserved")
    })
}
```

### Snapshot Testing

```cursed
slay test_output_snapshot() {
    sus output = generate_report(test_data)
    assert_snapshot_matches(output, "report_snapshot.txt", "Output should match snapshot")
}
```

### Fuzz Testing

```cursed
slay fuzz_test_parser() {
    sus fuzzer = StringFuzzer::new()
        .with_charset("abcdefghijklmnopqrstuvwxyz0123456789")
        .with_length_range(0..1000)
    
    fuzzer.run(1000, |input| {
        // Parser should not crash on any input
        sus result = parse_expression(input)
        
        // If parsing succeeds, result should be valid
        if let Ok(ast) = result {
            assert_valid_ast(ast, "Parsed AST should be valid")
        }
    })
}
```

## API Reference

### Core Types

```cursed
// Test configuration
squad TestConfig {
    sus include_patterns: Vec<String>,
    sus exclude_patterns: Vec<String>,
    sus test_patterns: Vec<String>,
    sus max_parallel_tests: usize,
    sus timeout_seconds: u64,
    sus verbose: bool,
    sus fail_fast: bool,
    sus coverage: bool,
    sus working_directory: PathBuf,
    sus environment: HashMap<String, String>,
    sus test_data_dir: Option<PathBuf>,
}

// Test runner builder
squad TestRunnerBuilder {
    slay new() -> Self
    slay with_config(self, config: TestConfig) -> Self
    slay with_report_format(self, format: ReportFormat) -> Self
    slay with_fail_fast(self, fail_fast: bool) -> Self
    slay with_coverage(self, coverage: bool) -> Self
    slay build(self) -> Result<TestRunner, TestError>
}

// Test runner
squad TestRunner {
    slay run_all_tests(&mut self) -> Result<TestReport, TestError>
    slay run_tests_matching(&mut self, pattern: &str) -> Result<TestReport, TestError>
    slay run_test_file(&mut self, file_path: &str) -> Result<TestReport, TestError>
}
```

### Assertion Functions

```cursed
// Basic assertions
slay assert_true(condition: bool, message: &str) -> Result<(), AssertionError>
slay assert_false(condition: bool, message: &str) -> Result<(), AssertionError>
slay assert_equal<T: PartialEq + Debug>(expected: T, actual: T, message: &str) -> Result<(), AssertionError>

// String assertions
slay assert_contains(haystack: &str, needle: &str, message: &str) -> Result<(), AssertionError>
slay assert_starts_with(text: &str, prefix: &str, message: &str) -> Result<(), AssertionError>

// Collection assertions
slay assert_empty<T>(collection: &[T], message: &str) -> Result<(), AssertionError>
slay assert_length<T>(collection: &[T], expected_length: usize, message: &str) -> Result<(), AssertionError>

// Numeric assertions
slay assert_float_equal(expected: f64, actual: f64, epsilon: f64, message: &str) -> Result<(), AssertionError>
slay assert_in_range<T: PartialOrd + Debug>(value: T, min: T, max: T, message: &str) -> Result<(), AssertionError>
```

### Fixture Management

```cursed
collab TestFixture {
    slay setup(&mut self) -> Result<(), TestError>
    slay teardown(&mut self) -> Result<(), TestError>
    slay get_data(&self) -> HashMap<String, TestValue>
    slay set_data(&mut self, key: String, value: TestValue)
    slay name(&self) -> &str
}

squad FixtureManager {
    slay new() -> Self
    slay register_fixture(&mut self, name: String, fixture: Box<dyn TestFixture>) -> Result<(), TestError>
    slay setup_all_fixtures(&mut self) -> Result<(), TestError>
    slay teardown_all_fixtures(&mut self) -> Result<(), TestError>
}
```

### Convenience Functions

```cursed
// High-level test execution
slay run_tests() -> Result<TestReport, TestError>
slay run_tests_in_directory(directory: &str) -> Result<TestReport, TestError>
slay run_tests_with_pattern(pattern: &str) -> Result<TestReport, TestError>
slay run_test_file(file_path: &str) -> Result<TestReport, TestError>
```

## Best Practices

### Test Organization

1. **Group related tests** into test suites using descriptive file names
2. **Use clear test names** that describe what is being tested
3. **Keep tests focused** - one concept per test function
4. **Use setup/teardown** for common test initialization

### Test Writing

1. **Follow AAA pattern** - Arrange, Act, Assert
2. **Use descriptive assertion messages** for better failure diagnosis
3. **Test edge cases** and error conditions
4. **Avoid test interdependencies** - tests should be independent

### Performance

1. **Use parallel execution** for faster test runs
2. **Enable caching** for compilation speedup
3. **Profile slow tests** and optimize if necessary
4. **Use fixtures** to avoid repeated setup

### Maintenance

1. **Keep tests up to date** with code changes
2. **Remove obsolete tests** that no longer provide value
3. **Refactor test code** to reduce duplication
4. **Monitor test coverage** and aim for high coverage

## Troubleshooting

### Common Issues

**Tests not discovered:**
- Check file naming conventions (`*_test.csd`, `test_*.csd`)
- Verify include/exclude patterns
- Check working directory setting

**Compilation errors:**
- Ensure test files have correct CURSED syntax
- Check import statements for test framework
- Verify test function signatures

**Assertion failures:**
- Read assertion error messages carefully
- Use verbose mode for more details
- Add debug output to understand test state

**Performance issues:**
- Reduce parallel job count if system is overloaded
- Check for inefficient test code
- Use profiling to identify bottlenecks

### Debug Mode

```bash
# Enable debug output
CURSED_TEST_DEBUG=true cursed test --verbose

# Run with additional logging
RUST_LOG=debug cursed test
```

### Getting Help

- Check this documentation for comprehensive information
- Use `cursed test --help` for command-line reference
- Run `cursed test --dry-run` to see what tests would be executed
- Enable verbose mode (`--verbose`) for detailed output

## Examples

See the `examples/` directory for comprehensive test examples:

- `test_example.csd` - Basic test patterns and assertions
- `integration_test_example.csd` - Integration testing patterns
- `performance_test_example.csd` - Performance and benchmark testing
- `fixture_test_example.csd` - Test fixture usage patterns

## Contributing

The CURSED test runner is part of the CURSED language project. Contributions are welcome:

1. **Report bugs** and request features via GitHub issues
2. **Submit pull requests** with improvements and fixes
3. **Add test examples** to help other developers
4. **Improve documentation** for better clarity

## License

The CURSED test runner is licensed under the same license as the CURSED language project.
