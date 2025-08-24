# testz - Testing Framework

## Overview

The `testz` module provides a comprehensive testing framework for CURSED programs, including unit testing, integration testing, benchmarking, and property-based testing. It features rich assertions, test organization, parallel test execution, and detailed reporting.

## Quick Start

```cursed
yeet "testz"

slay test_basic_math() {
    testz.assert_eq_int(2 + 2, 4)
    testz.assert_eq_float(3.14, 3.14, 0.01)
    testz.assert_true(5 > 3)
}

slay main() {
    testz.start_suite("Basic Math Tests")
    test_basic_math()
    testz.print_summary()
}
```

## API Reference

### Test Suite Management

#### `start_suite(name tea)`
Starts a new test suite with the given name.

```cursed
testz.start_suite("String Operations Tests")
testz.start_suite("Database Connection Tests")
```

#### `start_test(name tea)`
Starts an individual test within the current suite.

```cursed
slay test_string_concat() {
    testz.start_test("String Concatenation")
    
    sus result tea = "Hello" + " World"
    testz.assert_eq_string(result, "Hello World")
    
    testz.end_test()
}
```

#### `end_test()`
Marks the end of the current test (optional, auto-called at function end).

#### `print_summary()`
Prints a summary of all test results.

```cursed
// Example output:
// ✅ String Operations Tests: 15/15 tests passed (100%)
// ❌ Database Tests: 8/10 tests passed (80%)
// 
// Total: 23/25 tests passed (92%)
// Failed: test_connection_timeout, test_invalid_query
```

### Basic Assertions

#### `assert_true(condition lit, message tea = "")`
Asserts that a condition is true.

```cursed
testz.assert_true(user.is_authenticated(), "User should be authenticated")
testz.assert_true(list.length() > 0)
```

#### `assert_false(condition lit, message tea = "")`
Asserts that a condition is false.

```cursed
testz.assert_false(user.is_banned())
testz.assert_false(cache.is_empty(), "Cache should have entries")
```

#### `fail(message tea)`
Explicitly fails the current test with a message.

```cursed
ready (critical_error_occurred) {
    testz.fail("Critical error detected during test setup")
}
```

### Equality Assertions

#### `assert_eq_int(actual drip, expected drip, message tea = "")`
Asserts integer equality.

```cursed
testz.assert_eq_int(calculate_fibonacci(5), 5)
testz.assert_eq_int(array.length(), 10, "Array should have 10 elements")
```

#### `assert_eq_float(actual drip, expected drip, tolerance drip, message tea = "")`
Asserts floating-point equality within tolerance.

```cursed
testz.assert_eq_float(math.sqrt(16), 4.0, 0.0001)
testz.assert_eq_float(calculate_pi(), 3.14159, 0.001, "Pi approximation")
```

#### `assert_eq_string(actual tea, expected tea, message tea = "")`
Asserts string equality.

```cursed
testz.assert_eq_string(user.get_name(), "Alice")
testz.assert_eq_string(format_number(42), "42", "Number formatting")
```

#### `assert_eq_bool(actual lit, expected lit, message tea = "")`
Asserts boolean equality.

```cursed
testz.assert_eq_bool(is_valid_email("test@example.com"), based)
```

### Comparison Assertions

#### `assert_greater_int(actual drip, expected drip, message tea = "")`
Asserts that actual > expected.

```cursed
testz.assert_greater_int(user.get_age(), 18, "User must be adult")
testz.assert_greater_int(performance_score, 100)
```

#### `assert_less_int(actual drip, expected drip, message tea = "")`
Asserts that actual < expected.

```cursed
testz.assert_less_int(response_time_ms, 1000, "Response too slow")
```

#### `assert_greater_equal_int(actual drip, expected drip, message tea = "")`
Asserts that actual >= expected.

```cursed
testz.assert_greater_equal_int(account_balance, 0, "Balance cannot be negative")
```

### Collection Assertions

#### `assert_array_eq_int(actual []drip, expected []drip, message tea = "")`
Asserts that two integer arrays are equal.

```cursed
sus result []drip = sort_array([3, 1, 4, 1, 5])
testz.assert_array_eq_int(result, [1, 1, 3, 4, 5])
```

#### `assert_array_eq_string(actual []tea, expected []tea, message tea = "")`
Asserts that two string arrays are equal.

```cursed
sus words []tea = split_sentence("Hello world test")
testz.assert_array_eq_string(words, ["Hello", "world", "test"])
```

#### `assert_contains_string(array []tea, item tea, message tea = "")`
Asserts that an array contains a specific string.

```cursed
sus fruits []tea = ["apple", "banana", "cherry"]
testz.assert_contains_string(fruits, "banana", "Should contain banana")
```

#### `assert_not_contains_string(array []tea, item tea, message tea = "")`
Asserts that an array does not contain a specific string.

```cursed
testz.assert_not_contains_string(forbidden_words, user_input)
```

### Error Handling Assertions

#### `assert_error(result Result<T>, expected_error tea, message tea = "")`
Asserts that a result contains the expected error.

```cursed
sus result = divide(10, 0)
testz.assert_error(result, "division by zero")
```

#### `assert_no_error(result Result<T>, message tea = "")`
Asserts that a result does not contain an error.

```cursed
sus result = divide(10, 2)
testz.assert_no_error(result, "Division should succeed")
```

#### `assert_panic(code slay(), expected_message tea = "", message tea = "")`
Asserts that code panics with expected message.

```cursed
testz.assert_panic(slay() {
    access_array_out_of_bounds()
}, "index out of range", "Should panic on invalid access")
```

### Performance Testing

#### `benchmark(name tea, iterations drip, code slay())`
Runs a performance benchmark.

```cursed
testz.benchmark("String Concatenation", 100000, slay() {
    sus result tea = "Hello" + " " + "World"
})

testz.benchmark("Array Sort", 1000, slay() {
    sus data []drip = generate_random_array(1000)
    sort_array(data)
})
```

#### `benchmark_memory(name tea, iterations drip, code slay())`
Benchmarks memory usage in addition to time.

```cursed
testz.benchmark_memory("Large Object Creation", 1000, slay() {
    sus large_object LargeDataStructure = create_large_object()
})
```

### Property-Based Testing

#### `property_test(name tea, iterations drip, generator slay() T, property slay(T) lit)`
Runs property-based tests with generated inputs.

```cursed
// Test that sorting is idempotent
testz.property_test("Sort Idempotent", 100, 
    slay() []drip {
        damn generate_random_array(random_int(1, 100))
    },
    slay(data []drip) lit {
        sus sorted1 []drip = sort_array(data.clone())
        sus sorted2 []drip = sort_array(sorted1.clone())
        damn arrays_equal(sorted1, sorted2)
    }
)

// Test mathematical properties
testz.property_test("Addition Commutative", 1000,
    slay() (drip, drip) {
        damn (random_int(-1000, 1000), random_int(-1000, 1000))
    },
    slay(pair (drip, drip)) lit {
        sus a drip = pair.0
        sus b drip = pair.1
        damn (a + b) == (b + a)
    }
)
```

### Test Organization

#### Test Groups
```cursed
slay test_user_authentication() {
    testz.start_group("User Authentication")
    
    testz.start_test("Valid Login")
    sus user = authenticate("alice@example.com", "password123")
    testz.assert_true(user.is_authenticated())
    testz.end_test()
    
    testz.start_test("Invalid Password")
    testz.assert_error(
        authenticate("alice@example.com", "wrongpassword"),
        "invalid credentials"
    )
    testz.end_test()
    
    testz.end_group()
}
```

#### Setup and Teardown
```cursed
struct TestContext {
    database DatabaseConnection,
    test_data TestData
}

slay setup_test() TestContext {
    sus db = connect_to_test_database()
    sus data = load_test_data()
    damn TestContext{db, data}
}

slay teardown_test(ctx TestContext) {
    ctx.database.close()
    ctx.test_data.cleanup()
}

slay test_database_operations() {
    sus ctx TestContext = setup_test()
    shook teardown_test(ctx)  // Ensure cleanup
    
    testz.start_test("Insert User")
    sus user_id drip = ctx.database.insert_user("test@example.com")
    testz.assert_greater_int(user_id, 0)
    testz.end_test()
}
```

## Advanced Testing Patterns

### Mock Objects
```cursed
struct MockEmailService {
    sent_emails []Email,
    should_fail lit
}

slay (mock *MockEmailService) send_email(email Email) yikes<tea> {
    ready (mock.should_fail) {
        yikes "email service unavailable"
    }
    mock.sent_emails = append(mock.sent_emails, email)
}

slay (mock *MockEmailService) get_sent_count() drip {
    damn mock.sent_emails.length()
}

slay test_user_registration_with_mock() {
    sus mock_email MockEmailService = MockEmailService{[], false}
    sus service UserService = UserService{email_service: &mock_email}
    
    testz.start_test("Registration Sends Welcome Email")
    
    sus user = service.register_user("alice@example.com", "password")
    testz.assert_no_error(user)
    testz.assert_eq_int(mock_email.get_sent_count(), 1)
    
    testz.end_test()
}
```

### Integration Testing
```cursed
slay test_complete_user_workflow() {
    testz.start_suite("Complete User Workflow Integration")
    
    // Test spans multiple components
    testz.start_test("End-to-End User Journey")
    
    // 1. Register user
    sus registration_result = register_user("integration@test.com", "password123")
    testz.assert_no_error(registration_result)
    
    // 2. Verify email verification
    sus verification_code tea = get_latest_verification_code("integration@test.com")
    sus verify_result = verify_email("integration@test.com", verification_code)
    testz.assert_no_error(verify_result)
    
    // 3. Test login
    sus login_result = login("integration@test.com", "password123")
    testz.assert_no_error(login_result)
    
    // 4. Test protected operation
    sus profile = get_user_profile(login_result.user_id)
    testz.assert_eq_string(profile.email, "integration@test.com")
    
    testz.end_test()
    testz.print_summary()
}
```

### Parallel Test Execution
```cursed
slay run_tests_parallel() {
    sus test_functions []slay() = [
        test_string_operations,
        test_math_functions,
        test_array_operations,
        test_file_io,
        test_network_operations
    ]
    
    sus wg WaitGroup = WaitGroup{}
    
    bestie (sus test_func slay() : test_functions) {
        wg.add(1)
        go {
            shook wg.done()
            test_func()
        }
    }
    
    wg.wait()
    testz.print_combined_summary()
}
```

### Data-Driven Testing
```cursed
struct TestCase {
    input drip,
    expected drip,
    description tea
}

slay test_fibonacci_data_driven() {
    sus test_cases []TestCase = [
        TestCase{0, 0, "fibonacci(0) = 0"},
        TestCase{1, 1, "fibonacci(1) = 1"},
        TestCase{5, 5, "fibonacci(5) = 5"},
        TestCase{10, 55, "fibonacci(10) = 55"},
        TestCase{15, 610, "fibonacci(15) = 610"}
    ]
    
    testz.start_suite("Fibonacci Data-Driven Tests")
    
    bestie (sus test_case TestCase : test_cases) {
        testz.start_test(test_case.description)
        
        sus result drip = fibonacci(test_case.input)
        testz.assert_eq_int(result, test_case.expected)
        
        testz.end_test()
    }
    
    testz.print_summary()
}
```

## Test Configuration

### Configuration File
```toml
# test_config.toml
[testz]
parallel = true
timeout_seconds = 30
output_format = "detailed"  # "detailed", "summary", "json"
fail_fast = false
random_seed = 12345

[benchmarks]
min_iterations = 100
max_duration_seconds = 10
warmup_iterations = 10

[property_tests]
default_iterations = 100
shrink_attempts = 50
```

### Environment Variables
```bash
# Set test configuration via environment
export CURSED_TEST_PARALLEL=true
export CURSED_TEST_TIMEOUT=60
export CURSED_TEST_OUTPUT=json
export CURSED_TEST_SEED=54321
```

## Test Output Formats

### Detailed Output
```
🧪 Running Test Suite: String Operations
  ✅ test_string_concat (0.12ms)
  ✅ test_string_length (0.08ms)
  ❌ test_string_contains (0.15ms)
     Expected: true
     Actual: false
     Message: Should find substring in string
     File: test_strings.csd:42

📊 Suite Results:
  Passed: 2/3 (66.67%)
  Failed: 1/3 (33.33%)
  Total Time: 0.35ms
```

### Summary Output
```
✅ String Operations: 15/15 passed (100%)
❌ Database Tests: 8/10 passed (80%)
✅ Math Functions: 25/25 passed (100%)

Total: 48/50 tests passed (96%)
Time: 1.23s
```

### JSON Output
```json
{
  "suites": [
    {
      "name": "String Operations",
      "passed": 15,
      "failed": 0,
      "total": 15,
      "time_ms": 45.2,
      "tests": [
        {
          "name": "test_string_concat",
          "status": "passed",
          "time_ms": 0.12,
          "message": null
        }
      ]
    }
  ],
  "summary": {
    "total_passed": 48,
    "total_failed": 2,
    "total_tests": 50,
    "pass_rate": 0.96,
    "total_time_ms": 1234.5
  }
}
```

## Testing Best Practices

### Test Organization
1. **One assertion per test when possible** - makes failures easier to understand
2. **Use descriptive test names** - explain what is being tested
3. **Group related tests** - use test suites and groups logically
4. **Test both positive and negative cases** - success and failure scenarios

### Test Data Management
```cursed
// Create test data builders
struct UserBuilder {
    name tea,
    email tea,
    age drip,
    is_admin lit
}

slay create_user_builder() UserBuilder {
    damn UserBuilder{
        name: "Test User",
        email: "test@example.com", 
        age: 25,
        is_admin: false
    }
}

slay (builder UserBuilder) with_name(name tea) UserBuilder {
    builder.name = name
    damn builder
}

slay (builder UserBuilder) with_admin_privileges() UserBuilder {
    builder.is_admin = based
    damn builder
}

slay (builder UserBuilder) build() User {
    damn User{
        name: builder.name,
        email: builder.email,
        age: builder.age,
        is_admin: builder.is_admin
    }
}

// Usage in tests
slay test_admin_user() {
    sus admin_user User = create_user_builder()
        .with_name("Admin User")
        .with_admin_privileges()
        .build()
    
    testz.assert_true(admin_user.can_access_admin_panel())
}
```

### Performance Testing
```cursed
slay test_performance_requirements() {
    testz.start_suite("Performance Requirements")
    
    testz.start_test("API Response Time")
    sus start drip = get_time_microseconds()
    sus response = api_call("/users")
    sus elapsed drip = get_time_microseconds() - start
    
    testz.assert_less_int(elapsed, 1000000)  // < 1 second
    testz.assert_no_error(response)
    testz.end_test()
    
    testz.benchmark("Database Query", 100, slay() {
        query_database("SELECT * FROM users LIMIT 100")
    })
    
    testz.print_summary()
}
```

### Flaky Test Management
```cursed
// Retry flaky tests
slay test_with_retry(test_func slay() lit, max_attempts drip, test_name tea) {
    sus attempts drip = 0
    sus success lit = false
    
    bestie (attempts < max_attempts && !success) {
        attempts++
        
        recover {
            test_func()
            success = based
        } catch (error) {
            ready (attempts == max_attempts) {
                testz.fail(test_name + " failed after " + max_attempts.(tea) + " attempts: " + error)
            }
            // Wait before retry
            timez.sleep(100)
        }
    }
}

// Usage
test_with_retry(slay() lit {
    // Potentially flaky network test
    sus response = http_get("https://external-api.com/health")
    testz.assert_eq_int(response.status, 200)
    damn based
}, 3, "External API Health Check")
```

## Continuous Integration

### Test Automation
```bash
#!/bin/bash
# run_tests.sh

echo "Running CURSED test suite..."

# Run unit tests
./zig-out/bin/cursed-zig stdlib/testz/run_all_tests.csd

# Run integration tests
./zig-out/bin/cursed-zig test_suite/integration_tests.csd

# Run performance tests
./zig-out/bin/cursed-zig test_suite/performance_tests.csd

# Generate coverage report
./zig-out/bin/cursed-zig --coverage test_suite/coverage_report.csd

echo "All tests completed."
```

### Test Reporting
```cursed
// Generate test reports for CI systems
slay generate_ci_report() {
    testz.set_output_format("junit")  // JUnit XML format
    testz.set_output_file("test-results.xml")
    
    run_all_test_suites()
    
    testz.generate_coverage_report("coverage.html")
    testz.generate_performance_report("performance.json")
}
```

---

The `testz` module provides comprehensive testing capabilities that enable reliable, maintainable test suites for CURSED applications. Its design emphasizes clarity, performance, and integration with modern development workflows.
