# testz - Testing Framework Module

## Overview

The `testz` module provides a comprehensive testing framework for CURSED programs, featuring unit testing, integration testing, benchmarking, and advanced testing patterns. It includes assertion functions, test organization capabilities, performance testing, and detailed reporting with multiple output formats.

**Key Features:**
- Comprehensive assertion library with detailed failure messages
- Test organization with groups, suites, and fixtures
- Performance benchmarking and profiling
- Multiple output formats (console, JSON, XML, HTML)
- Property-based testing support
- Mock and stub functionality
- Parallel test execution
- Code coverage integration
- CI/CD integration helpers

**Status:** ✅ Production Ready - Fully implemented and tested

## Quick Start

```cursed
yeet "testz"

# Start a test suite
testz.test_start("my_application_tests")

# Basic assertions
testz.assert_true(based)
testz.assert_eq_int(2 + 2, 4)
testz.assert_eq_string("hello", "hello")

# Test grouping
testz.test_group("arithmetic_tests") {
    testz.assert_eq_int(add(2, 3), 5)
    testz.assert_eq_int(multiply(4, 5), 20)
}

# Performance testing
testz.benchmark_start("sorting_performance")
sus data []drip = [3, 1, 4, 1, 5, 9, 2, 6]
sort_array(data)
testz.benchmark_end()

# Display results
testz.print_test_summary()
```

## API Reference

### Test Suite Management

#### `test_start(suite_name)` / `test_end()` / `test_reset()`
Test suite lifecycle management functions.

**Parameters:**
- `suite_name` (`tea`) - Name of the test suite

**Returns:** `void`

**Examples:**
```cursed
# Basic test suite
testz.test_start("calculator_tests")

# Your tests here
testz.assert_eq_int(add(2, 3), 5)

testz.test_end()  # Optional - called automatically at end

# Reset for new test suite
testz.test_reset()
testz.test_start("string_tests")

# Run multiple test suites
sus test_suites []tea = ["unit_tests", "integration_tests", "performance_tests"]

bestie (suite_name tea : test_suites) {
    testz.test_start(suite_name)
    run_tests_for_suite(suite_name)
    testz.print_test_summary()
    testz.test_reset()
}
```

---

#### `test_group(group_name, tests)` / `test_suite(suite_name, groups)`
Test organization and grouping functions.

**Parameters:**
- `group_name` (`tea`) - Name of the test group
- `tests` (function) - Function containing test code
- `suite_name` (`tea`) - Name of the test suite
- `groups` (function) - Function containing test groups

**Returns:** `void`

**Examples:**
```cursed
testz.test_start("comprehensive_tests")

# Group related tests together
testz.test_group("basic_arithmetic") {
    testz.assert_eq_int(2 + 2, 4)
    testz.assert_eq_int(5 - 3, 2)
    testz.assert_eq_int(6 * 7, 42)
    testz.assert_eq_int(15 / 3, 5)
}

testz.test_group("string_operations") {
    testz.assert_eq_string(stringz.concat("hello", " world"), "hello world")
    testz.assert_true(stringz.contains("testing", "test"))
    testz.assert_eq_int(stringz.length("CURSED"), 6)
}

testz.test_group("array_functions") {
    sus arr []drip = [1, 2, 3, 4, 5]
    testz.assert_eq_int(len(arr), 5)
    testz.assert_eq_float(arrayz.sum_array(arr), 15.0, 0.001)
    testz.assert_eq_float(arrayz.mean(arr), 3.0, 0.001)
}

testz.print_test_summary()
```

### Assertion Functions

#### `assert_true(condition)` / `assert_false(condition)`
Boolean assertion functions.

**Parameters:**
- `condition` (`lit`) - Boolean value to test

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Basic boolean tests
testz.assert_true(based)                    # Passes
testz.assert_false(cap)                     # Passes

# Conditional tests
sus age drip = 25
testz.assert_true(age >= 18)                # Legal adult
testz.assert_false(age < 0)                 # Valid age

# Function result tests
testz.assert_true(stringz.contains("hello world", "world"))
testz.assert_false(stringz.starts_with("hello", "goodbye"))

# Custom conditions
slay is_prime(n drip) lit {
    ready (n < 2) damn cap
    bestie (i drip = 2; i * i <= n; i += 1) {
        ready (n % i == 0) damn cap
    }
    damn based
}

testz.assert_true(is_prime(17))             # 17 is prime
testz.assert_false(is_prime(15))            # 15 is not prime
```

---

#### `assert_eq_int(actual, expected)` / `assert_eq_float(actual, expected, epsilon)` / `assert_eq_string(actual, expected)`
Equality assertion functions with type-specific behavior.

**Parameters:**
- `actual` - Actual value from code under test
- `expected` - Expected value
- `epsilon` (`drip`, for float) - Tolerance for floating-point comparison

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Integer comparisons
testz.assert_eq_int(2 + 2, 4)
testz.assert_eq_int(fibonacci(7), 13)
testz.assert_eq_int(len([1, 2, 3]), 3)

# Floating-point comparisons (with tolerance)
testz.assert_eq_float(mathz.PI, 3.14159, 0.00001)
testz.assert_eq_float(mathz.sqrt(4), 2.0, 0.001)
testz.assert_eq_float(1.0 / 3.0, 0.3333, 0.0001)

# String comparisons
testz.assert_eq_string("hello", "hello")
testz.assert_eq_string(stringz.to_upper("test"), "TEST")
testz.assert_eq_string(stringz.trim("  spaces  "), "spaces")

# Variable comparisons
sus expected_name tea = "Alice"
sus actual_name tea = get_user_name(user_id)
testz.assert_eq_string(actual_name, expected_name)
```

---

#### `assert_ne_int(actual, not_expected)` / `assert_ne_string(actual, not_expected)`
Inequality assertion functions.

**Parameters:**
- `actual` - Actual value from code under test
- `not_expected` - Value that should NOT be equal to actual

**Returns:** `void` - Throws test failure if values are equal

**Examples:**
```cursed
# Test that values are different
testz.assert_ne_int(random_number(), 0)     # Random should not be exactly 0
testz.assert_ne_string(generate_id(), "")   # ID should not be empty

# Test function variations
sus password tea = generate_password()
testz.assert_ne_string(password, "password") # Should not be default
testz.assert_ne_string(password, "123456")   # Should not be weak

# Test uniqueness
sus id1 tea = generate_unique_id()
sus id2 tea = generate_unique_id()
testz.assert_ne_string(id1, id2)            # Should be unique
```

---

#### `assert_null(value)` / `assert_not_null(value)`
Null/undefined value assertion functions.

**Parameters:**
- `value` - Value to test for null/undefined

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Test null/undefined values
sus maybe_value tea = get_optional_value(key)
testz.assert_not_null(maybe_value)          # Should have a value

sus missing_value tea = get_nonexistent_value()
testz.assert_null(missing_value)            # Should be null/undefined

# Test function returns
sus user User = find_user_by_id(valid_id)
testz.assert_not_null(user)                 # User should exist

sus invalid_user User = find_user_by_id("invalid")
testz.assert_null(invalid_user)             # User should not exist
```

---

#### `assert_contains(container, element)` / `assert_not_contains(container, element)`
Container membership assertion functions.

**Parameters:**
- `container` - Array or string to search in
- `element` - Element to search for

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Array membership tests
sus numbers []drip = [1, 2, 3, 4, 5]
testz.assert_contains(numbers, 3)           # Array contains 3
testz.assert_not_contains(numbers, 10)      # Array does not contain 10

sus fruits []tea = ["apple", "banana", "cherry"]
testz.assert_contains(fruits, "banana")     # Array contains "banana"
testz.assert_not_contains(fruits, "grape")  # Array does not contain "grape"

# String membership tests
sus text tea = "Hello, World!"
testz.assert_contains(text, "World")        # String contains "World"
testz.assert_not_contains(text, "Goodbye")  # String does not contain "Goodbye"

# Function result tests
sus search_results []tea = search_database("query")
testz.assert_contains(search_results, expected_result)
```

---

#### `assert_greater(actual, minimum)` / `assert_less(actual, maximum)`
Numerical comparison assertion functions.

**Parameters:**
- `actual` (`drip`) - Actual numeric value
- `minimum`/`maximum` (`drip`) - Comparison value

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Numeric range tests
sus score drip = calculate_test_score()
testz.assert_greater(score, 0)              # Score should be positive
testz.assert_less(score, 100)               # Score should be less than 100

# Performance tests
sus start_time drip = timez.now_millis()
perform_operation()
sus end_time drip = timez.now_millis()
sus duration drip = end_time - start_time
testz.assert_less(duration, 1000)          # Should complete in under 1 second

# Boundary tests
testz.assert_greater(mathz.PI, 3.0)         # Pi is greater than 3
testz.assert_less(mathz.PI, 4.0)           # Pi is less than 4
```

---

#### `assert_between(value, min, max)` / `assert_throws(function, expected_error)`
Advanced assertion functions.

**Parameters:**
- `value` (`drip`) - Value to test
- `min`, `max` (`drip`) - Range boundaries
- `function` - Function that should throw an error
- `expected_error` (`tea`) - Expected error message/type

**Returns:** `void` - Throws test failure if assertion fails

**Examples:**
```cursed
# Range testing
sus temperature drip = get_room_temperature()
testz.assert_between(temperature, 60, 80)   # Reasonable room temp range

sus dice_roll drip = roll_dice()
testz.assert_between(dice_roll, 1, 6)       # Valid dice values

# Error testing
testz.assert_throws(slay() {
    divide_by_zero(10, 0)
}, "division_by_zero")

testz.assert_throws(slay() {
    access_invalid_index(array, -1)
}, "index_out_of_bounds")

# Test that function does NOT throw
testz.assert_no_throw(slay() {
    safe_operation()
})
```

### Performance Testing and Benchmarking

#### `benchmark_start(name)` / `benchmark_end()` / `benchmark_reset()`
Performance benchmarking functions.

**Parameters:**
- `name` (`tea`) - Name of the benchmark

**Returns:** `void`

**Examples:**
```cursed
# Basic benchmarking
testz.benchmark_start("array_sorting")

sus data []drip = generate_random_array(10000)
sus sorted []drip = arrayz.sort_array_ascending(data)

testz.benchmark_end()  # Records timing automatically

# Multiple benchmarks
sus algorithms []tea = ["quicksort", "mergesort", "heapsort"]
sus test_data []drip = generate_test_data(100000)

bestie (algorithm tea : algorithms) {
    testz.benchmark_start(algorithm)
    
    ready (stringz.equals(algorithm, "quicksort")) {
        quicksort(test_data)
    } elready (stringz.equals(algorithm, "mergesort")) {
        mergesort(test_data)
    } otherwise {
        heapsort(test_data)
    }
    
    testz.benchmark_end()
}

# Compare results
testz.print_benchmark_comparison()
```

---

#### `benchmark_function(name, function, iterations)`
Automated function benchmarking.

**Parameters:**
- `name` (`tea`) - Benchmark name
- `function` - Function to benchmark
- `iterations` (`drip`) - Number of iterations to run

**Returns:** `BenchmarkResult` - Timing and statistics

**Examples:**
```cursed
# Benchmark a function automatically
sus result BenchmarkResult = testz.benchmark_function(
    "string_concatenation",
    slay() {
        sus s tea = ""
        bestie (i drip = 0; i < 1000; i += 1) {
            s = stringz.concat(s, "x")
        }
    },
    100  # Run 100 times
)

vibez.spillf("Average time: {}μs\n", result.average_time_micros)
vibez.spillf("Min time: {}μs\n", result.min_time_micros)
vibez.spillf("Max time: {}μs\n", result.max_time_micros)

# Compare multiple implementations
sus implementations []NamedFunction = [
    NamedFunction{"naive_concat", naive_string_concat},
    NamedFunction{"builder_concat", builder_string_concat},
    NamedFunction{"array_join", array_join_concat}
]

sus comparison_results []BenchmarkResult = []

bestie (impl NamedFunction : implementations) {
    sus result BenchmarkResult = testz.benchmark_function(impl.name, impl.function, 100)
    comparison_results = comparison_results + [result]
    
    vibez.spillf("{}: {}μs average\n", impl.name, result.average_time_micros)
}

# Find fastest implementation
sus fastest BenchmarkResult = arrayz.min_element_by(comparison_results, 
    slay(r BenchmarkResult) drip { damn r.average_time_micros })

vibez.spillf("Fastest: {} at {}μs\n", fastest.name, fastest.average_time_micros)
```

---

#### `memory_benchmark_start(name)` / `memory_benchmark_end()`
Memory usage benchmarking functions.

**Parameters:**
- `name` (`tea`) - Benchmark name

**Returns:** `void`

**Examples:**
```cursed
# Memory usage testing
testz.memory_benchmark_start("large_array_creation")

sus large_arrays [][]drip = []
bestie (i drip = 0; i < 1000; i += 1) {
    large_arrays = large_arrays + [generate_array(1000)]
}

testz.memory_benchmark_end()  # Reports peak memory usage

# Compare memory usage of algorithms
slay compare_memory_usage() {
    # Test memory usage of different data structures
    testz.memory_benchmark_start("hash_map_insertion")
    sus map map<tea, drip> = {}
    bestie (i drip = 0; i < 100000; i += 1) {
        map[stringz.from_int(i)] = i
    }
    testz.memory_benchmark_end()
    
    testz.memory_benchmark_start("array_insertion")
    sus array []drip = []
    bestie (i drip = 0; i < 100000; i += 1) {
        array = array + [i]
    }
    testz.memory_benchmark_end()
    
    testz.print_memory_comparison()
}
```

### Test Fixtures and Setup

#### `setup(function)` / `teardown(function)` / `before_each(function)` / `after_each(function)`
Test fixture management functions.

**Parameters:**
- `function` - Setup or teardown function

**Returns:** `void`

**Examples:**
```cursed
# Global setup/teardown
testz.setup(slay() {
    # Initialize test database
    initialize_test_db()
    create_test_tables()
    
    vibez.spillln("Test environment initialized")
})

testz.teardown(slay() {
    # Cleanup test database
    drop_test_tables()
    close_test_db()
    
    vibez.spillln("Test environment cleaned up")
})

# Per-test setup/teardown
testz.before_each(slay() {
    # Reset state before each test
    clear_test_data()
    reset_counters()
})

testz.after_each(slay() {
    # Cleanup after each test
    validate_invariants()
    log_test_metrics()
})

# Example test with fixtures
testz.test_start("database_tests")

testz.test_group("user_operations") {
    # before_each runs here
    
    testz.assert_eq_int(count_users(), 0)  # Should start empty
    
    sus user User = create_test_user("Alice")
    testz.assert_not_null(user)
    testz.assert_eq_int(count_users(), 1)
    
    # after_each runs here
}
```

---

#### `fixture(name, factory)` / `use_fixture(name)`
Named fixture management for complex test data.

**Parameters:**
- `name` (`tea`) - Fixture name
- `factory` - Function that creates fixture data

**Returns:** Fixture data when using fixture

**Examples:**
```cursed
# Define reusable fixtures
testz.fixture("sample_users", slay() []User {
    damn [
        User{name: "Alice", age: 25, email: "alice@test.com"},
        User{name: "Bob", age: 30, email: "bob@test.com"},
        User{name: "Charlie", age: 35, email: "charlie@test.com"}
    ]
})

testz.fixture("test_database", slay() Database {
    sus db Database = create_in_memory_db()
    initialize_schema(db)
    damn db
})

# Use fixtures in tests
testz.test_group("user_validation") {
    sus users []User = testz.use_fixture("sample_users")
    sus db Database = testz.use_fixture("test_database")
    
    bestie (user User : users) {
        testz.assert_true(validate_user_email(user.email))
        testz.assert_greater(user.age, 0)
        testz.assert_greater(stringz.length(user.name), 0)
    }
}

# Parameterized tests with fixtures
testz.fixture("math_test_cases", slay() []MathTestCase {
    damn [
        MathTestCase{a: 2, b: 3, expected_sum: 5, expected_product: 6},
        MathTestCase{a: -1, b: 1, expected_sum: 0, expected_product: -1},
        MathTestCase{a: 0, b: 5, expected_sum: 5, expected_product: 0}
    ]
})

testz.test_group("arithmetic_operations") {
    sus test_cases []MathTestCase = testz.use_fixture("math_test_cases")
    
    bestie (case MathTestCase : test_cases) {
        testz.assert_eq_int(add(case.a, case.b), case.expected_sum)
        testz.assert_eq_int(multiply(case.a, case.b), case.expected_product)
    }
}

struct MathTestCase {
    a drip
    b drip
    expected_sum drip
    expected_product drip
}
```

### Advanced Testing Features

#### `mock_function(original, replacement)` / `restore_mocks()`
Function mocking and stubbing.

**Parameters:**
- `original` - Original function to mock
- `replacement` - Mock function replacement

**Returns:** `void`

**Examples:**
```cursed
# Mock external dependencies
testz.mock_function(network_request, slay(url tea) tea {
    # Return mock response based on URL
    ready (stringz.contains(url, "/users/1")) {
        damn "{\"id\": 1, \"name\": \"Test User\"}"
    } otherwise {
        damn "{\"error\": \"Not found\"}"
    }
})

# Test with mocked function
testz.test_group("api_client_tests") {
    sus user User = fetch_user_by_id(1)
    testz.assert_eq_string(user.name, "Test User")
    
    # Test error handling
    testz.assert_throws(slay() {
        fetch_user_by_id(999)
    }, "user_not_found")
}

# Restore original functions
testz.restore_mocks()

# Mock with call tracking
sus call_tracker CallTracker = testz.create_call_tracker()

testz.mock_function(log_message, slay(message tea) {
    call_tracker.record_call("log_message", [message])
})

# Run tests
perform_logging_operation()

# Verify mock calls
testz.assert_eq_int(call_tracker.call_count("log_message"), 3)
testz.assert_contains(call_tracker.get_calls("log_message")[0].args, "Starting operation")
```

---

#### `property_test(name, generator, property, iterations)`
Property-based testing support.

**Parameters:**
- `name` (`tea`) - Test name
- `generator` - Function that generates test data
- `property` - Function that checks a property
- `iterations` (`drip`) - Number of iterations to run

**Returns:** `void`

**Examples:**
```cursed
# Property-based testing for list reverse
testz.property_test(
    "reverse_reverse_is_identity",
    slay() []drip { damn generate_random_array(100) },
    slay(arr []drip) lit {
        sus twice_reversed []drip = arrayz.reverse(arrayz.reverse(arr))
        damn arrayz.equal(arr, twice_reversed)
    },
    1000
)

# Property-based testing for string operations
testz.property_test(
    "concat_length_property",
    slay() ConcatTestData {
        damn ConcatTestData{
            s1: generate_random_string(50),
            s2: generate_random_string(50)
        }
    },
    slay(data ConcatTestData) lit {
        sus concatenated tea = stringz.concat(data.s1, data.s2)
        sus expected_length drip = stringz.length(data.s1) + stringz.length(data.s2)
        damn stringz.length(concatenated) == expected_length
    },
    500
)

struct ConcatTestData {
    s1 tea
    s2 tea
}

# Custom generators for complex data
slay generate_valid_email() tea {
    sus domains []tea = ["test.com", "example.org", "mock.net"]
    sus usernames []tea = ["user", "test", "demo", "sample"]
    
    sus username tea = usernames[mathz.random_int(0, len(usernames))]
    sus domain tea = domains[mathz.random_int(0, len(domains))]
    
    damn stringz.concat(username, "@", domain)
}

testz.property_test(
    "email_validation_property",
    generate_valid_email,
    slay(email tea) lit {
        damn stringz.is_valid_email(email)
    },
    200
)
```

---

#### `parallel_test_group(name, tests, max_workers)`
Parallel test execution for performance.

**Parameters:**
- `name` (`tea`) - Test group name
- `tests` - Array of test functions
- `max_workers` (`drip`) - Maximum parallel workers

**Returns:** `void`

**Examples:**
```cursed
# Define independent tests
sus independent_tests []NamedTest = [
    NamedTest{"test_math_functions", test_math_functions},
    NamedTest{"test_string_operations", test_string_operations},
    NamedTest{"test_array_operations", test_array_operations},
    NamedTest{"test_file_operations", test_file_operations}
]

# Run tests in parallel
testz.parallel_test_group("unit_tests", independent_tests, 4)

# Parallel property testing
testz.parallel_property_test(
    "concurrent_property_test",
    generate_test_data,
    check_property,
    10000,  # Total iterations
    8       # Parallel workers
)

struct NamedTest {
    name tea
    test_function TestFunction
}

# Example of parallel integration tests
slay run_integration_tests() {
    sus integration_tests []NamedTest = [
        NamedTest{"database_tests", run_database_tests},
        NamedTest{"api_tests", run_api_tests},
        NamedTest{"cache_tests", run_cache_tests},
        NamedTest{"auth_tests", run_auth_tests}
    ]
    
    testz.parallel_test_group("integration_tests", integration_tests, 2)
}
```

### Test Reporting and Output

#### `print_test_summary()` / `print_detailed_report()` / `get_test_results()`
Test result reporting functions.

**Parameters:** None (uses internal test state)

**Returns:** `void` for print functions, `TestResults` for get_test_results

**Examples:**
```cursed
# Basic test summary
testz.test_start("my_tests")
run_all_tests()
testz.print_test_summary()

# Output:
# ✅ Test Suite: my_tests
# ✅ Passed: 25
# ❌ Failed: 2
# ⏱️  Total time: 1.23s
# 📊 Success rate: 92.6%

# Detailed report with failure information
testz.print_detailed_report()

# Output:
# === Detailed Test Report ===
# 
# ✅ test_group: basic_operations (12 tests, all passed)
# ❌ test_group: edge_cases (2 tests, 1 failed)
#   ❌ test_negative_input: Expected 0, got -1
#      at line 45 in test_edge_cases()
#      
# Performance Summary:
# - Fastest test: test_simple_add (0.1ms)
# - Slowest test: test_large_array (123ms)

# Programmatic access to results
sus results TestResults = testz.get_test_results()
vibez.spillf("Total tests: {}\n", results.total_tests)
vibez.spillf("Passed: {}\n", results.passed_tests)
vibez.spillf("Failed: {}\n", results.failed_tests)
vibez.spillf("Success rate: {:.1f}%\n", results.success_rate)

ready (results.failed_tests > 0) {
    vibez.spillln("Failed tests:")
    bestie (failure TestFailure : results.failures) {
        vibez.spillf("- {}: {}\n", failure.test_name, failure.error_message)
    }
}
```

---

#### `export_results_json(filename)` / `export_results_xml(filename)` / `export_results_html(filename)`
Export test results to different formats.

**Parameters:**
- `filename` (`tea`) - Output file path

**Returns:** `void`

**Examples:**
```cursed
# Export results for CI/CD integration
testz.export_results_json("test-results.json")
# Creates JSON file compatible with CI systems

testz.export_results_xml("test-results.xml")
# Creates JUnit-compatible XML file

testz.export_results_html("test-report.html")
# Creates human-readable HTML report

# Custom export with filtering
sus results TestResults = testz.get_test_results()

# Export only failed tests
sus failed_only TestResults = TestResults{
    total_tests: results.failed_tests,
    passed_tests: 0,
    failed_tests: results.failed_tests,
    failures: results.failures,
    benchmarks: [],
    success_rate: 0.0
}

testz.export_custom_json(failed_only, "failures-only.json")

# Export with additional metadata
testz.export_results_with_metadata("full-report.json", {
    "build_version": "1.2.3",
    "commit_hash": "abc123",
    "test_environment": "ci",
    "timestamp": timez.now_iso_string()
})
```

---

#### `configure_output(options)` / `set_verbosity(level)`
Configure test output and reporting.

**Parameters:**
- `options` (`OutputOptions`) - Output configuration
- `level` (`drip`) - Verbosity level (0-3)

**Returns:** `void`

**Examples:**
```cursed
# Configure detailed output
testz.configure_output(OutputOptions{
    show_passing_tests: based,
    show_benchmark_details: based,
    show_memory_usage: based,
    use_colors: based,
    max_failure_details: 10
})

# Set verbosity levels
testz.set_verbosity(0)  # Silent (only final summary)
testz.set_verbosity(1)  # Normal (pass/fail indicators)
testz.set_verbosity(2)  # Verbose (detailed test info)
testz.set_verbosity(3)  # Debug (all internal details)

# Environment-based configuration
ready (envz.get("CI") == "true") {
    # CI environment: minimal output, machine-readable format
    testz.configure_output(OutputOptions{
        show_passing_tests: cap,
        show_benchmark_details: cap,
        use_colors: cap,
        export_json: based,
        export_xml: based
    })
} otherwise {
    # Development environment: rich output with colors
    testz.configure_output(OutputOptions{
        show_passing_tests: based,
        show_benchmark_details: based,
        use_colors: based,
        show_progress_bar: based
    })
}

struct OutputOptions {
    show_passing_tests lit
    show_benchmark_details lit
    show_memory_usage lit
    use_colors lit
    export_json lit
    export_xml lit
    export_html lit
    max_failure_details drip
    show_progress_bar lit
}
```

## Usage Guide

### Common Testing Patterns

#### Unit Testing Structure
```cursed
yeet "testz"
yeet "stringz"
yeet "mathz"

# Comprehensive unit test suite
slay run_unit_tests() {
    testz.test_start("calculator_unit_tests")
    
    # Test basic arithmetic operations
    testz.test_group("basic_arithmetic") {
        test_addition()
        test_subtraction()
        test_multiplication()
        test_division()
    }
    
    # Test edge cases
    testz.test_group("edge_cases") {
        test_division_by_zero()
        test_overflow_conditions()
        test_negative_numbers()
    }
    
    # Test complex operations
    testz.test_group("complex_operations") {
        test_square_root()
        test_exponentiation()
        test_logarithms()
    }
    
    testz.print_test_summary()
}

slay test_addition() {
    # Basic cases
    testz.assert_eq_float(add(2, 3), 5.0, 0.001)
    testz.assert_eq_float(add(-1, 1), 0.0, 0.001)
    testz.assert_eq_float(add(0, 0), 0.0, 0.001)
    
    # Edge cases
    testz.assert_eq_float(add(mathz.MAX_FLOAT / 2, mathz.MAX_FLOAT / 2), mathz.MAX_FLOAT, 0.001)
    testz.assert_eq_float(add(mathz.MIN_FLOAT, 0), mathz.MIN_FLOAT, 0.001)
}

slay test_division_by_zero() {
    testz.assert_throws(slay() {
        divide(10, 0)
    }, "division_by_zero")
    
    testz.assert_throws(slay() {
        divide(-5, 0)
    }, "division_by_zero")
}

slay test_square_root() {
    testz.assert_eq_float(sqrt(4), 2.0, 0.001)
    testz.assert_eq_float(sqrt(9), 3.0, 0.001)
    testz.assert_eq_float(sqrt(2), 1.414, 0.001)
    
    # Test negative input handling
    testz.assert_throws(slay() {
        sqrt(-1)
    }, "domain_error")
}
```

#### Integration Testing
```cursed
yeet "testz"
yeet "vibez"
yeet "jsonz"

# Integration tests for API client
slay run_integration_tests() {
    testz.test_start("api_integration_tests")
    
    # Setup test environment
    testz.setup(slay() {
        start_test_server()
        initialize_test_data()
    })
    
    testz.teardown(slay() {
        stop_test_server()
        cleanup_test_data()
    })
    
    testz.test_group("user_api_tests") {
        test_user_creation()
        test_user_retrieval()
        test_user_update()
        test_user_deletion()
    }
    
    testz.test_group("authentication_tests") {
        test_login_success()
        test_login_failure()
        test_token_refresh()
        test_logout()
    }
    
    testz.print_detailed_report()
    testz.export_results_xml("integration-test-results.xml")
}

slay test_user_creation() {
    # Create a new user
    sus user_data User = User{
        name: "Test User",
        email: "test@example.com",
        age: 30
    }
    
    sus response ApiResponse = api_client.create_user(user_data)
    
    testz.assert_eq_int(response.status_code, 201)
    testz.assert_not_null(response.body)
    
    sus created_user User = jsonz.unmarshal(response.body, User)
    testz.assert_eq_string(created_user.name, user_data.name)
    testz.assert_eq_string(created_user.email, user_data.email)
    testz.assert_not_null(created_user.id)
    
    # Store for cleanup
    test_user_id = created_user.id
}

slay test_user_retrieval() {
    # Retrieve the created user
    sus response ApiResponse = api_client.get_user(test_user_id)
    
    testz.assert_eq_int(response.status_code, 200)
    
    sus retrieved_user User = jsonz.unmarshal(response.body, User)
    testz.assert_eq_string(retrieved_user.id, test_user_id)
    testz.assert_eq_string(retrieved_user.name, "Test User")
}
```

#### Performance Testing
```cursed
yeet "testz"
yeet "arrayz"
yeet "stringz"

# Comprehensive performance testing
slay run_performance_tests() {
    testz.test_start("performance_benchmarks")
    
    # Data structure performance
    testz.test_group("data_structure_benchmarks") {
        benchmark_array_operations()
        benchmark_string_operations()
        benchmark_hash_map_operations()
    }
    
    # Algorithm performance
    testz.test_group("algorithm_benchmarks") {
        benchmark_sorting_algorithms()
        benchmark_search_algorithms()
        benchmark_string_matching()
    }
    
    # Memory usage benchmarks
    testz.test_group("memory_benchmarks") {
        benchmark_memory_allocation()
        benchmark_garbage_collection()
    }
    
    testz.print_benchmark_comparison()
}

slay benchmark_array_operations() {
    sus sizes []drip = [1000, 10000, 100000]
    
    bestie (size drip : sizes) {
        sus test_data []drip = arrayz.range(0, size)
        
        # Benchmark sum operation
        testz.benchmark_start(stringz.concat("array_sum_", stringz.from_int(size)))
        sus sum drip = arrayz.sum_array(test_data)
        testz.benchmark_end()
        
        # Benchmark max finding
        testz.benchmark_start(stringz.concat("array_max_", stringz.from_int(size)))
        sus max drip = arrayz.max_element(test_data)
        testz.benchmark_end()
        
        # Benchmark filtering
        testz.benchmark_start(stringz.concat("array_filter_", stringz.from_int(size)))
        sus evens []drip = arrayz.filter(test_data, slay(x drip) lit { damn x % 2 == 0 })
        testz.benchmark_end()
    }
}

slay benchmark_sorting_algorithms() {
    sus algorithms []SortingAlgorithm = [
        SortingAlgorithm{"quicksort", quicksort_implementation},
        SortingAlgorithm{"mergesort", mergesort_implementation},
        SortingAlgorithm{"heapsort", heapsort_implementation}
    ]
    
    sus data_sizes []drip = [1000, 10000, 100000]
    
    bestie (size drip : data_sizes) {
        sus test_data []drip = generate_random_array(size)
        
        bestie (algo SortingAlgorithm : algorithms) {
            sus benchmark_name tea = stringz.concat(algo.name, "_", stringz.from_int(size))
            
            # Run multiple iterations for statistical significance
            sus iterations drip = 10
            sus total_time drip = 0
            
            bestie (i drip = 0; i < iterations; i += 1) {
                sus data_copy []drip = arrayz.copy(test_data)
                
                testz.benchmark_start(benchmark_name)
                algo.sort_function(data_copy)
                testz.benchmark_end()
                
                total_time += testz.get_last_benchmark_time()
            }
            
            sus average_time drip = total_time / iterations
            vibez.spillf("{}: {}ms average\n", benchmark_name, average_time)
        }
    }
}

struct SortingAlgorithm {
    name tea
    sort_function SortFunction
}
```

#### Property-Based Testing
```cursed
yeet "testz"
yeet "arrayz"
yeet "stringz"
yeet "mathz"

# Property-based testing for robust validation
slay run_property_tests() {
    testz.test_start("property_based_tests")
    
    # Test mathematical properties
    testz.test_group("mathematical_properties") {
        test_arithmetic_properties()
        test_array_properties()
        test_string_properties()
    }
    
    testz.print_test_summary()
}

slay test_arithmetic_properties() {
    # Test addition commutativity: a + b = b + a
    testz.property_test(
        "addition_commutativity",
        slay() ArithmeticPair {
            damn ArithmeticPair{
                a: mathz.random_range(-1000, 1000),
                b: mathz.random_range(-1000, 1000)
            }
        },
        slay(pair ArithmeticPair) lit {
            sus sum1 drip = add(pair.a, pair.b)
            sus sum2 drip = add(pair.b, pair.a)
            damn mathz.abs(sum1 - sum2) < 0.000001
        },
        1000
    )
    
    # Test multiplication associativity: (a * b) * c = a * (b * c)
    testz.property_test(
        "multiplication_associativity",
        slay() ArithmeticTriple {
            damn ArithmeticTriple{
                a: mathz.random_range(-100, 100),
                b: mathz.random_range(-100, 100),
                c: mathz.random_range(-100, 100)
            }
        },
        slay(triple ArithmeticTriple) lit {
            sus left drip = multiply(multiply(triple.a, triple.b), triple.c)
            sus right drip = multiply(triple.a, multiply(triple.b, triple.c))
            damn mathz.abs(left - right) < 0.000001
        },
        500
    )
}

slay test_array_properties() {
    # Test that reverse(reverse(array)) = array
    testz.property_test(
        "reverse_involution",
        slay() []drip { damn generate_random_int_array(100) },
        slay(arr []drip) lit {
            sus double_reversed []drip = arrayz.reverse(arrayz.reverse(arr))
            damn arrayz.equal(arr, double_reversed)
        },
        1000
    )
    
    # Test that sort preserves length
    testz.property_test(
        "sort_preserves_length",
        slay() []drip { damn generate_random_int_array(200) },
        slay(arr []drip) lit {
            sus sorted []drip = arrayz.sort_array_ascending(arr)
            damn len(sorted) == len(arr)
        },
        500
    )
    
    # Test that filter never increases length
    testz.property_test(
        "filter_never_increases_length",
        slay() ArrayAndPredicate {
            sus arr []drip = generate_random_int_array(150)
            sus predicate TestPredicate = generate_random_predicate()
            damn ArrayAndPredicate{array: arr, predicate: predicate}
        },
        slay(data ArrayAndPredicate) lit {
            sus filtered []drip = apply_predicate_filter(data.array, data.predicate)
            damn len(filtered) <= len(data.array)
        },
        800
    )
}

slay test_string_properties() {
    # Test that trim never increases length
    testz.property_test(
        "trim_never_increases_length",
        slay() tea { damn generate_random_string_with_whitespace(100) },
        slay(str tea) lit {
            sus trimmed tea = stringz.trim(str)
            damn stringz.length(trimmed) <= stringz.length(str)
        },
        1000
    )
    
    # Test concatenation length property
    testz.property_test(
        "concat_length_property",
        slay() StringPair {
            damn StringPair{
                s1: generate_random_string(50),
                s2: generate_random_string(50)
            }
        },
        slay(pair StringPair) lit {
            sus concatenated tea = stringz.concat(pair.s1, pair.s2)
            sus expected_length drip = stringz.length(pair.s1) + stringz.length(pair.s2)
            damn stringz.length(concatenated) == expected_length
        },
        1000
    )
}

# Helper structures and generators
struct ArithmeticPair {
    a drip
    b drip
}

struct ArithmeticTriple {
    a drip
    b drip
    c drip
}

struct StringPair {
    s1 tea
    s2 tea
}

struct ArrayAndPredicate {
    array []drip
    predicate TestPredicate
}

slay generate_random_int_array(max_size drip) []drip {
    sus size drip = mathz.random_int(0, max_size)
    sus result []drip = []
    
    bestie (i drip = 0; i < size; i += 1) {
        result = result + [mathz.random_int(-1000, 1000)]
    }
    
    damn result
}

slay generate_random_string_with_whitespace(max_length drip) tea {
    sus length drip = mathz.random_int(0, max_length)
    sus result tea = ""
    sus chars tea = "abcdefghijklmnopqrstuvwxyz \t\n"
    
    bestie (i drip = 0; i < length; i += 1) {
        sus char_index drip = mathz.random_int(0, stringz.length(chars))
        result = stringz.concat(result, stringz.char_at(chars, char_index))
    }
    
    damn result
}
```

### Best Practices

#### Test Organization
```cursed
# Good: Organize tests in logical groups
testz.test_start("user_management_tests")

testz.test_group("user_validation") {
    # All validation-related tests
    test_email_validation()
    test_password_validation()
    test_age_validation()
}

testz.test_group("user_persistence") {
    # All database-related tests
    test_user_creation()
    test_user_updates()
    test_user_deletion()
}

testz.test_group("user_authentication") {
    # All auth-related tests
    test_login_success()
    test_login_failure()
    test_password_reset()
}
```

#### Assertion Best Practices
```cursed
# Good: Specific, descriptive assertions
slay test_user_creation() {
    sus user User = create_user("Alice", "alice@test.com")
    
    testz.assert_not_null(user)
    testz.assert_eq_string(user.name, "Alice")
    testz.assert_eq_string(user.email, "alice@test.com")
    testz.assert_true(stringz.length(user.id) > 0)
    testz.assert_greater(user.created_at, 0)
}

# Avoid: Vague or multiple assertions in one
slay test_user_creation_bad() {
    sus user User = create_user("Alice", "alice@test.com")
    testz.assert_true(user != undefined && user.name == "Alice")  # Don't do this
}
```

#### Error Testing
```cursed
# Good: Test specific error conditions
testz.test_group("error_handling") {
    testz.assert_throws(slay() {
        create_user("", "valid@email.com")  # Empty name
    }, "invalid_name")
    
    testz.assert_throws(slay() {
        create_user("Valid Name", "invalid-email")  # Invalid email
    }, "invalid_email")
    
    testz.assert_throws(slay() {
        create_user("Valid Name", "valid@email.com")  # Duplicate user
        create_user("Valid Name", "valid@email.com")
    }, "duplicate_user")
}
```

#### Performance Testing Guidelines
```cursed
# Good: Test performance with realistic data sizes
slay performance_test_guidelines() {
    sus realistic_sizes []drip = [100, 1000, 10000]  # Realistic data sizes
    
    bestie (size drip : realistic_sizes) {
        testz.benchmark_start(stringz.concat("operation_", stringz.from_int(size)))
        
        # Run with realistic data
        sus data []drip = generate_realistic_test_data(size)
        perform_operation(data)
        
        testz.benchmark_end()
        
        # Assert performance requirements
        sus last_time drip = testz.get_last_benchmark_time()
        testz.assert_less(last_time, size * 0.1)  # Linear time requirement
    }
}
```

## Performance Notes

### Testing Framework Performance

**Test Execution Overhead:**
```
Basic assertion:        ~100ns
String assertion:       ~500ns
Array assertion:        ~1μs per element
Mock function call:     ~200ns
Fixture setup:          ~10μs
Test group overhead:    ~1ms
```

**Memory Usage:**
```
Test framework overhead:  ~50KB
Per test metadata:        ~500 bytes
Per assertion:           ~100 bytes
Mock function storage:    ~1KB per mock
Benchmark data:          ~2KB per benchmark
```

**Benchmarking Accuracy:**
- Timing resolution: 1μs (microsecond precision)
- Memory tracking: Page-level accuracy (4KB granularity)
- Statistical analysis: Mean, min, max, standard deviation
- Outlier detection: Automatic filtering of extreme values

### Integration Examples

### With CI/CD Systems
```cursed
# GitHub Actions integration
slay run_ci_tests() {
    # Configure for CI environment
    testz.configure_output(OutputOptions{
        use_colors: cap,           # No colors in CI logs
        show_passing_tests: cap,   # Reduce log noise
        export_json: based,        # Machine-readable results
        export_xml: based          # JUnit format for test reporting
    })
    
    testz.test_start("ci_test_suite")
    
    # Run all test categories
    run_unit_tests()
    run_integration_tests()
    run_performance_tests()
    
    # Export results
    testz.export_results_json("test-results.json")
    testz.export_results_xml("junit-results.xml")
    
    # Exit with proper code for CI
    sus results TestResults = testz.get_test_results()
    ready (results.failed_tests > 0) {
        vibez.spill_error(stringz.concat("Tests failed: ", stringz.from_int(results.failed_tests)))
        exit(1)
    }
}
```

### With Code Coverage
```cursed
yeet "coveragez"

slay run_tests_with_coverage() {
    # Start coverage tracking
    coveragez.start_tracking()
    
    testz.test_start("coverage_test_suite")
    
    # Run comprehensive tests
    run_all_tests()
    
    # Stop coverage and generate report
    sus coverage_data CoverageData = coveragez.stop_tracking()
    
    # Display coverage summary
    vibez.spillf("Line coverage: {:.1f}%\n", coverage_data.line_coverage)
    vibez.spillf("Branch coverage: {:.1f}%\n", coverage_data.branch_coverage)
    vibez.spillf("Function coverage: {:.1f}%\n", coverage_data.function_coverage)
    
    # Export coverage report
    coveragez.export_html_report(coverage_data, "coverage-report.html")
    
    testz.print_test_summary()
    
    # Fail if coverage is below threshold
    ready (coverage_data.line_coverage < 80.0) {
        vibez.spill_error("Code coverage below 80% threshold")
        exit(1)
    }
}
```

## Migration Guide

### From Jest (JavaScript)
```javascript
// Jest
describe('Calculator', () => {
  test('adds 1 + 2 to equal 3', () => {
    expect(add(1, 2)).toBe(3);
  });
  
  test('throws on division by zero', () => {
    expect(() => divide(1, 0)).toThrow('Division by zero');
  });
});
```

```cursed
# CURSED testz
testz.test_start("calculator_tests")

testz.test_group("calculator") {
    testz.assert_eq_int(add(1, 2), 3)
    
    testz.assert_throws(slay() {
        divide(1, 0)
    }, "division_by_zero")
}

testz.print_test_summary()
```

### From pytest (Python)
```python
# pytest
import pytest

def test_string_operations():
    assert len("hello") == 5
    assert "hello".upper() == "HELLO"
    
def test_list_operations():
    data = [1, 2, 3]
    assert sum(data) == 6
    assert max(data) == 3

@pytest.fixture
def sample_data():
    return {"name": "Alice", "age": 30}

def test_with_fixture(sample_data):
    assert sample_data["name"] == "Alice"
```

```cursed
# CURSED testz
testz.test_start("python_migration_tests")

testz.test_group("string_operations") {
    testz.assert_eq_int(stringz.length("hello"), 5)
    testz.assert_eq_string(stringz.to_upper("hello"), "HELLO")
}

testz.test_group("array_operations") {
    sus data []drip = [1, 2, 3]
    testz.assert_eq_float(arrayz.sum_array(data), 6.0, 0.001)
    testz.assert_eq_float(arrayz.max_element(data), 3.0, 0.001)
}

testz.fixture("sample_data", slay() map<tea, tea> {
    damn {"name": "Alice", "age": "30"}
})

testz.test_group("with_fixture") {
    sus data map<tea, tea> = testz.use_fixture("sample_data")
    testz.assert_eq_string(data["name"], "Alice")
}

testz.print_test_summary()
```

### From Go testing
```go
// Go testing
func TestAdd(t *testing.T) {
    result := add(2, 3)
    if result != 5 {
        t.Errorf("Expected 5, got %d", result)
    }
}

func BenchmarkAdd(b *testing.B) {
    for i := 0; i < b.N; i++ {
        add(2, 3)
    }
}
```

```cursed
# CURSED testz
testz.test_start("go_migration_tests")

testz.test_group("add_test") {
    sus result drip = add(2, 3)
    testz.assert_eq_int(result, 5)
}

testz.benchmark_function("add_benchmark", slay() {
    add(2, 3)
}, 1000)

testz.print_test_summary()
```

## Troubleshooting

### Common Issues

**Issue: Test Failures with Floating Point**
```cursed
# Problem: Exact floating point comparison
testz.assert_eq_float(1.0 / 3.0, 0.333333333, 0.0)  # May fail

# Solution: Use appropriate epsilon
testz.assert_eq_float(1.0 / 3.0, 0.333333333, 0.000001)  # Better

# Best: Use reasonable precision for context
testz.assert_eq_float(calculate_percentage(1, 3), 33.33, 0.01)  # Most practical
```

**Issue: Flaky Tests Due to Timing**
```cursed
# Problem: Time-dependent tests
testz.benchmark_start("flaky_test")
perform_network_operation()  # Variable timing
testz.benchmark_end()
testz.assert_less(testz.get_last_benchmark_time(), 100)  # May fail randomly

# Solution: Use multiple runs and statistical analysis
sus times []drip = []
bestie (i drip = 0; i < 10; i += 1) {
    testz.benchmark_start("stable_test")
    perform_network_operation()
    testz.benchmark_end()
    times = times + [testz.get_last_benchmark_time()]
}

sus median_time drip = arrayz.median(times)
testz.assert_less(median_time, 200)  # More stable
```

**Issue: Memory Leaks in Tests**
```cursed
# Problem: Tests accumulating memory
testz.test_group("memory_leak_tests") {
    bestie (i drip = 0; i < 1000; i += 1) {
        sus large_data []drip = create_large_test_data()
        process_data(large_data)
        # large_data not cleaned up
    }
}

# Solution: Explicit cleanup in fixtures
testz.before_each(slay() {
    reset_memory_pools()
})

testz.after_each(slay() {
    cleanup_test_data()
    force_garbage_collection()
})
```

### Debugging Test Failures

**Enable Debug Output:**
```cursed
# Set high verbosity for debugging
testz.set_verbosity(3)

# Enable detailed failure information
testz.configure_output(OutputOptions{
    show_passing_tests: based,
    show_benchmark_details: based,
    max_failure_details: 50,
    use_colors: based
})
```

**Isolate Failing Tests:**
```cursed
# Run only specific test groups
testz.test_start("debug_session")

# Comment out passing groups to focus on failures
# testz.test_group("working_tests") { ... }

testz.test_group("failing_tests") {
    # Add debug output
    vibez.spillln("DEBUG: Starting failing test")
    
    sus result drip = problematic_function()
    vibez.spillf("DEBUG: Result = {}\n", result)
    
    testz.assert_eq_int(result, expected_value)
}

testz.print_detailed_report()
```

---

**Module Status:** ✅ Production Ready  
**Version:** 1.0.0  
**Last Updated:** 2025-08-23  
**Stability:** Stable - Safe for production use  
**Features:** Comprehensive testing framework with benchmarking, mocking, and CI/CD integration
