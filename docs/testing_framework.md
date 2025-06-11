# CURSED Testing Framework Documentation

The CURSED Testing Framework provides a comprehensive unit testing infrastructure for CURSED applications, featuring test discovery, execution, reporting, and a rich assertion library.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Test Structure](#test-structure)
3. [Assertion Framework](#assertion-framework)
4. [Test Attributes](#test-attributes)
5. [Test Discovery](#test-discovery)
6. [Test Execution](#test-execution)
7. [Reporting](#reporting)
8. [Advanced Features](#advanced-features)
9. [Best Practices](#best-practices)
10. [CLI Usage](#cli-usage)

## Quick Start

### Writing Your First Test

```cursed
import "stdlib::testing";
use testing::*;

#[test]
slay test_basic_math() {
    facts result = 2 + 2;
    assert_eq(result, 4)?;
}

#[test]
slay test_string_operations() {
    facts greeting = "Hello, World!";
    assert_contains(greeting, "World")?;
    assert_starts_with(greeting, "Hello")?;
    assert_length(greeting, 13)?;
}
```

### Running Tests

```bash
# Run all tests
cursed test

# Run tests with verbose output
cursed test --verbose

# Run specific test pattern
cursed test --filter "test_math_*"

# Run tests in parallel
cursed test --parallel 4
```

## Test Structure

### Basic Test Function

Tests in CURSED are functions marked with the `#[test]` attribute:

```cursed
#[test]
slay test_function_name() {
    // Test implementation
    assert_true(some_condition)?;
}
```

### Test Organization

Tests can be organized into modules and files:

```cursed
// tests/math_tests.csd
import "stdlib::testing";
use testing::*;

mod arithmetic_tests {
    #[test]
    slay test_addition() {
        assert_eq(1 + 1, 2)?;
    }
    
    #[test]
    slay test_multiplication() {
        assert_eq(3 * 4, 12)?;
    }
}

mod geometry_tests {
    #[test]
    slay test_area_calculation() {
        facts area = calculate_rectangle_area(5, 3);
        assert_eq(area, 15)?;
    }
}
```

## Assertion Framework

The CURSED Testing Framework provides a comprehensive set of assertion functions:

### Basic Assertions

```cursed
// Boolean assertions
assert_true(condition)?;
assert_false(condition)?;

// Equality assertions
assert_eq(actual, expected)?;
assert_ne(actual, unexpected)?;

// Null checks
assert_null(optional_value)?;
assert_not_null(optional_value)?;
```

### Numeric Assertions

```cursed
// Comparison assertions
assert_greater(10, 5)?;
assert_greater_equal(10, 10)?;
assert_less(5, 10)?;
assert_less_equal(5, 5)?;

// Floating point comparisons
assert_close_to(3.14159, 3.14, 0.01)?;

// Range checks
assert_between(value, min, max)?;

// Sign checks
assert_positive(5)?;
assert_negative(-3)?;
assert_zero(0)?;
```

### String Assertions

```cursed
facts text = "Hello, World!";

// Content checks
assert_contains(text, "World")?;
assert_not_contains(text, "Goodbye")?;
assert_starts_with(text, "Hello")?;
assert_ends_with(text, "!")?;

// Pattern matching
assert_matches_regex(text, "Hello, *")?;

// Length and emptiness
assert_length(text, 13)?;
assert_empty_string("")?;
```

### Collection Assertions

```cursed
facts numbers = [1, 2, 3, 4, 5];

// Collection properties
assert_not_empty(numbers)?;
assert_has_length(numbers, 5)?;

// Element presence
assert_contains_element(numbers, 3)?;
assert_not_contains_element(numbers, 10)?;

// Boolean collections
facts all_true = [true, true, true];
facts mixed = [true, false, true];

assert_all_true(all_true)?;
assert_any_true(mixed)?;
assert_none_true([false, false])?;
```

### Error Assertions

```cursed
// Result testing
facts result = risky_operation();
assert_error(result)?;
assert_no_error(safe_operation())?;

// Specific error types
assert_error_type(result, ExpectedErrorType)?;
assert_error_message(result, "Expected error message")?;

// Panic testing
assert_panic(|| {
    panic!("This should panic");
})?;

assert_no_panic(|| {
    safe_function();
})?;
```

### Advanced Assertions

```cursed
// Time-based assertions
assert_eventually(|| {
    check_condition()
}, Duration::from_secs(5))?;

assert_within_timeout(|| {
    fast_operation()
}, Duration::from_millis(100))?;

// File system assertions
assert_file_exists("output.txt")?;
assert_file_content("output.txt", "expected content")?;
```

## Test Attributes

### Basic Attributes

```cursed
// Basic test
#[test]
slay test_function() {
    // Test code
}

// Ignored test
#[test]
#[ignore("Not implemented yet")]
slay test_future_feature() {
    // Test code
}

// Expected panic
#[test]
#[should_panic("Division by zero")]
slay test_division_by_zero() {
    divide(10, 0);
}

// Custom timeout
#[test]
#[timeout(5000)] // 5 seconds
slay test_long_operation() {
    slow_operation();
}
```

### Tags and Categories

```cursed
// Tag tests for organization
#[test]
#[tag("integration")]
slay test_database_integration() {
    // Integration test code
}

#[test]
#[tag("performance")]
#[tag("slow")]
slay test_performance_benchmark() {
    // Performance test code
}
```

### Setup and Teardown

```cursed
#[setup]
slay setup_test_environment() {
    init_test_database();
    create_test_data();
}

#[teardown]
slay cleanup_test_environment() {
    cleanup_test_data();
    close_test_database();
}

#[test]
slay test_with_setup() {
    // This test will run setup before and teardown after
    facts user = get_test_user();
    assert_not_null(user)?;
}
```

## Test Discovery

The testing framework automatically discovers tests based on configurable patterns:

### Default Patterns

- `**/*test*.csd` - Files containing "test" in the name
- `**/test_*.csd` - Files starting with "test_"
- `tests/**/*.csd` - All files in tests directories

### Custom Discovery

```cursed
import "stdlib::testing";

slay main() {
    sus config = TestFrameworkConfig {
        test_root: PathBuf::from("./my_tests"),
        test_patterns: vec![
            "**/*_test.csd".to_string(),
            "**/integration_*.csd".to_string(),
        ],
        ..TestFrameworkConfig::default()
    };
    
    sus mut framework = TestFramework::with_config(config);
    facts report = framework.run_tests()?;
}
```

### Filtering Tests

```cursed
// Filter by pattern
sus filter = TestFilter::new()
    .include_pattern("test_math_*".to_string())
    .exclude_pattern("*_slow".to_string());

// Filter by tags
sus filter = TestFilter::new()
    .include_tag("unit".to_string())
    .exclude_tag("integration".to_string());

// Filter by module
sus filter = TestFilter::new()
    .include_module("math".to_string())
    .exclude_module("deprecated".to_string());
```

## Test Execution

### Execution Modes

```cursed
// Sequential execution
sus config = TestFrameworkConfig {
    execution_mode: TestExecutionMode::Sequential,
    ..TestFrameworkConfig::default()
};

// Parallel execution
sus config = TestFrameworkConfig {
    execution_mode: TestExecutionMode::Parallel,
    max_parallel_tests: 4,
    ..TestFrameworkConfig::default()
};

// Adaptive execution (framework chooses)
sus config = TestFrameworkConfig {
    execution_mode: TestExecutionMode::Adaptive,
    ..TestFrameworkConfig::default()
};
```

### Timeouts and Fail-Fast

```cursed
sus config = TestFrameworkConfig {
    default_timeout: Duration::from_secs(30),
    fail_fast: true, // Stop on first failure
    capture_output: true,
    ..TestFrameworkConfig::default()
};
```

## Reporting

### Report Formats

The framework supports multiple report formats:

#### Console (Default)

```
╔══════════════════════════════════════════════════════════════╗
║                    CURSED Test Report                       ║
╠══════════════════════════════════════════════════════════════╣
║ Total Tests:        25                                       ║
║ Passed:             23 ( 92.0%)                             ║
║ Failed:              2                                       ║
║ Ignored:             0                                       ║
║ Total Time:      2.345s                                      ║
║ Average Time:    0.094s                                      ║
╚══════════════════════════════════════════════════════════════╝
```

#### JSON

```json
{
  "summary": {
    "total_tests": 25,
    "passed": 23,
    "failed": 2,
    "ignored": 0,
    "success_rate": 92.0,
    "total_time_secs": 2.345
  },
  "test_results": [
    {
      "name": "test_math_addition",
      "status": "passed",
      "execution_time_secs": 0.001,
      "file_path": "tests/math_test.csd",
      "line_number": 10
    }
  ]
}
```

#### XML (JUnit Compatible)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites tests="25" failures="2" time="2.345">
  <testsuite name="CURSED Tests" tests="25" failures="2" time="2.345">
    <testcase name="test_math_addition" time="0.001"/>
    <testcase name="test_failure_example" time="0.050">
      <failure message="Assertion failed">Expected 4, got 5</failure>
    </testcase>
  </testsuite>
</testsuites>
```

#### HTML

Generates a comprehensive HTML report with interactive features, charts, and detailed failure analysis.

### Custom Reporting

```cursed
// Generate reports programmatically
facts report = framework.run_tests()?;

// Generate JSON report
facts json_reporter = JsonReporter::new();
facts json_content = json_reporter.generate_report(&report)?;
write_file("report.json", json_content)?;

// Generate HTML report
facts html_reporter = HtmlReporter::new();
facts html_content = html_reporter.generate_report(&report)?;
write_file("report.html", html_content)?;
```

## Advanced Features

### Performance Testing

```cursed
#[test]
#[timeout(10000)]
#[tag("performance")]
slay test_performance_benchmark() {
    facts start_time = now();
    
    // Performance-critical operation
    periodt (sus i = 0; i < 1000000; i++) {
        expensive_operation(i);
    }
    
    facts duration = now() - start_time;
    assert_less(duration, Duration::from_secs(5))?;
}
```

### Memory Testing

```cursed
#[test]
slay test_memory_usage() {
    facts initial_memory = get_memory_usage();
    
    // Operations that might leak memory
    create_large_objects();
    
    // Force garbage collection
    force_gc();
    
    facts final_memory = get_memory_usage();
    facts memory_increase = final_memory - initial_memory;
    
    assert_less(memory_increase, 1024 * 1024)?; // Less than 1MB increase
}
```

### Integration Testing

```cursed
#[test]
#[tag("integration")]
#[setup("start_test_services")]
#[teardown("stop_test_services")]
slay test_api_integration() {
    // Test external service integration
    facts client = create_test_client();
    facts response = client.get("/api/users")?;
    
    assert_eq(response.status(), 200)?;
    assert_contains(response.body(), "users")?;
}
```

### Property-Based Testing

```cursed
#[test]
slay test_addition_property() {
    // Property: addition is commutative
    periodt (sus i = 0; i < 100; i++) {
        facts a = random_int();
        facts b = random_int();
        
        assert_eq(a + b, b + a)?;
    }
}
```

## Best Practices

### 1. Test Organization

```cursed
// Group related tests in modules
mod user_service_tests {
    use super::*;
    
    #[test]
    slay test_create_user() { /* ... */ }
    
    #[test]
    slay test_update_user() { /* ... */ }
    
    #[test]
    slay test_delete_user() { /* ... */ }
}
```

### 2. Test Naming

Use descriptive test names that explain what is being tested:

```cursed
// Good: Describes what is tested and expected outcome
#[test]
slay test_user_creation_with_valid_email_succeeds() { /* ... */ }

// Good: Describes the scenario and expected behavior
#[test]
slay test_division_by_zero_returns_error() { /* ... */ }

// Avoid: Generic or unclear names
#[test]
slay test_user() { /* ... */ }
```

### 3. Test Independence

Each test should be independent and not rely on other tests:

```cursed
#[test]
slay test_independent_scenario() {
    // Setup test data within the test
    facts user = create_test_user();
    
    // Perform test
    facts result = user_service.authenticate(user);
    
    // Assert results
    assert_true(result.is_success())?;
    
    // Cleanup if necessary
    cleanup_test_user(user);
}
```

### 4. Use Setup and Teardown

For expensive setup operations, use setup and teardown functions:

```cursed
#[setup]
slay setup_database() {
    init_test_database();
    seed_test_data();
}

#[teardown]
slay cleanup_database() {
    clear_test_data();
    close_database_connections();
}
```

### 5. Test Error Conditions

Always test both success and failure scenarios:

```cursed
#[test]
slay test_valid_input_succeeds() {
    facts result = process_input("valid_input");
    assert_no_error(result)?;
}

#[test]
slay test_invalid_input_fails() {
    facts result = process_input("invalid_input");
    assert_error(result)?;
    assert_error_message(result, "Invalid input format")?;
}
```

### 6. Use Appropriate Assertions

Choose the most specific assertion for better error messages:

```cursed
// Good: Specific assertion with clear failure message
assert_contains(error_message, "user not found")?;

// Less ideal: Generic assertion with unclear failure
assert_true(error_message.includes("user not found"))?;
```

## CLI Usage

### Basic Commands

```bash
# Run all tests
cursed test

# Run tests with verbose output
cursed test --verbose

# Run tests in parallel
cursed test --parallel 4

# Stop on first failure
cursed test --fail-fast

# Run specific test pattern
cursed test --filter "test_math_*"

# Run tests with specific tags
cursed test --tag "unit"

# Exclude specific tags
cursed test --exclude-tag "slow"

# Set custom timeout
cursed test --timeout 60

# Generate specific report format
cursed test --format json
cursed test --format xml
cursed test --format html

# Save reports to directory
cursed test --report-dir ./test_reports

# Run ignored tests
cursed test --ignored
```

### Advanced Options

```bash
# Run tests with custom configuration
cursed test --config test_config.toml

# Run tests from specific directory
cursed test --test-root ./custom_tests

# Run tests with specific patterns
cursed test --pattern "integration_*.csd"

# Run tests with environment variables
cursed test --env "TEST_MODE=integration"

# Run tests with memory profiling
cursed test --profile-memory

# Run tests with coverage analysis
cursed test --coverage
```

### Configuration File

Create a `test_config.toml` file for custom test configuration:

```toml
[test_framework]
test_root = "./tests"
patterns = ["**/*test*.csd", "**/integration_*.csd"]
max_parallel_tests = 8
default_timeout = "60s"
capture_output = true
fail_fast = false
verbose = true
show_timing = true

[filter]
include_patterns = ["test_*"]
exclude_patterns = ["*_slow"]
include_tags = ["unit", "integration"]
exclude_tags = ["manual"]

[reporting]
format = "console"
output_dir = "./test_reports"
show_stack_traces = true
use_colors = true
```

## Integration with Build Systems

### Makefile Integration

```makefile
# Test targets
test:
	cursed test

test-verbose:
	cursed test --verbose

test-ci:
	cursed test --format xml --report-dir ./test_reports --fail-fast

test-coverage:
	cursed test --coverage --format html --report-dir ./coverage

test-performance:
	cursed test --tag performance --timeout 300

.PHONY: test test-verbose test-ci test-coverage test-performance
```

### CI/CD Integration

```yaml
# GitHub Actions example
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup CURSED
        run: |
          # Install CURSED
      - name: Run Tests
        run: cursed test --format xml --report-dir ./test_reports
      - name: Upload Test Results
        uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: test_reports/
```

This documentation provides a comprehensive guide to using the CURSED Testing Framework. The framework is designed to be powerful yet easy to use, supporting everything from simple unit tests to complex integration and performance testing scenarios.
