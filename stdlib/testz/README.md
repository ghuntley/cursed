# CURSED Testing Framework (testz) Documentation

The testz framework provides comprehensive testing utilities for CURSED stdlib development and application testing.

## 🚀 Quick Start

```cursed
yeet "testz"

slay test_example() {
    test_start("example test")
    
    assert_eq_int(2 + 2, 4)
    assert_true(5 > 3)
    assert_eq_string("hello", "hello")
    
    test_end()
}

test_example()
print_test_summary()
```

## 📋 Core Functions

### Test Management

- **`test_start(name tea)`** - Begin a new test with the given name
- **`test_end()`** - End the current test and record timing
- **`reset_test_state()`** - Reset all test counters and state
- **`test_suite_start(suite_name tea)`** - Begin a test suite
- **`test_suite_end(suite_name tea)`** - End a test suite

### Basic Assertions

- **`assert_eq_int(actual normie, expected normie)`** - Assert integer equality
- **`assert_eq_string(actual tea, expected tea)`** - Assert string equality
- **`assert_true(condition lit)`** - Assert condition is true
- **`assert_false(condition lit)`** - Assert condition is false
- **`assert_not_null(value tea)`** - Assert string is not null/empty

### Comparison Assertions

- **`assert_gt(actual normie, expected normie)`** - Assert greater than
- **`assert_lt(actual normie, expected normie)`** - Assert less than
- **`assert_gte(actual normie, expected normie)`** - Assert greater than or equal
- **`assert_lte(actual normie, expected normie)`** - Assert less than or equal
- **`assert_not_eq(actual normie, expected normie)`** - Assert not equal

### State Access

- **`get_pass_count() normie`** - Get number of passed assertions
- **`get_fail_count() normie`** - Get number of failed assertions
- **`get_total_count() normie`** - Get total number of tests run
- **`get_current_test_name() tea`** - Get name of current test

### Reporting

- **`print_test_summary()`** - Print basic test results summary
- **`print_detailed_report()`** - Print comprehensive test metrics

## 🎨 Features

### ✨ Enhanced Output

The framework provides colored output for better readability:

- 🧪 **Blue** - Test information and progress
- ✅ **Green** - Passing assertions
- ❌ **Red** - Failing assertions and errors
- ⚠️ **Yellow** - Warnings and state changes
- ⏱️ **Timing** - Performance metrics

### 📊 Performance Metrics

- Individual test timing
- Total test suite execution time
- Success rate calculations
- Assertions per test metrics

### 🔧 Test Fixtures

```cursed
# Set up function to run before each test
set_test_setup("my_setup_function")

# Set teardown function to run after each test
set_test_teardown("my_cleanup_function")
```

### 📈 Benchmarking

```cursed
# Benchmark a function with specified iterations
sus avg_time normie = benchmark_function("my_function", 1000)
```

## 📖 Usage Examples

### Basic Test Structure

```cursed
yeet "testz"

slay test_arithmetic() {
    test_start("basic arithmetic")
    
    sus x normie = 10
    sus y normie = 20
    
    assert_eq_int(x + y, 30)
    assert_gt(y, x)
    assert_lt(x, y)
    assert_not_eq(x, y)
    
    test_end()
}

slay test_strings() {
    test_start("string operations")
    
    sus greeting tea = "hello"
    sus name tea = "world"
    
    assert_eq_string(greeting, "hello")
    assert_not_null(greeting)
    assert_not_null(name)
    
    test_end()
}

# Run tests
test_arithmetic()
test_strings()
print_test_summary()
```

### Test Suite Organization

```cursed
yeet "testz"

slay run_math_tests() {
    test_suite_start("Mathematics")
    
    test_start("addition")
    assert_eq_int(2 + 3, 5)
    assert_eq_int(0 + 0, 0)
    test_end()
    
    test_start("subtraction")
    assert_eq_int(10 - 5, 5)
    assert_eq_int(0 - 5, -5)
    test_end()
    
    test_suite_end("Mathematics")
}

slay run_string_tests() {
    test_suite_start("String Operations")
    
    test_start("equality")
    assert_eq_string("test", "test")
    assert_eq_string("", "")
    test_end()
    
    test_start("validation")
    assert_not_null("valid")
    test_end()
    
    test_suite_end("String Operations")
}

# Execute all test suites
run_math_tests()
run_string_tests()
print_detailed_report()
```

### Performance Testing

```cursed
yeet "testz"

slay test_performance() {
    test_start("performance validation")
    
    # Test many iterations
    sus i normie = 0
    bestie i < 1000 {
        assert_eq_int(i * 2, i + i)
        i = i + 1
    }
    
    test_end()
}

test_performance()
print_test_summary()
```

### Error Handling

```cursed
yeet "testz"

slay test_error_recovery() {
    test_start("error recovery")
    
    # This will fail but test continues
    assert_eq_int(1, 2)
    
    # This will pass
    assert_eq_int(2, 2)
    
    test_end()
}

test_error_recovery()
print_test_summary()
```

## 🎯 Best Practices

### 1. Test Organization

```cursed
# Group related tests in functions
slay test_user_management() {
    test_suite_start("User Management")
    
    test_create_user()
    test_update_user()
    test_delete_user()
    
    test_suite_end("User Management")
}
```

### 2. Descriptive Test Names

```cursed
# Good: Descriptive and specific
test_start("user creation with valid email")

# Bad: Vague and unclear  
test_start("test1")
```

### 3. Clear Assertions

```cursed
# Good: Test specific behavior
assert_eq_int(user.age, 25)
assert_eq_string(user.name, "John")
assert_true(user.is_active)

# Better: Group related assertions in same test
test_start("user properties after creation")
assert_eq_string(user.name, "John")
assert_eq_int(user.age, 25)
assert_true(user.is_active)
test_end()
```

### 4. State Management

```cursed
# Reset state between test runs
reset_test_state()

# Use setup/teardown for initialization
set_test_setup("initialize_test_data")
set_test_teardown("cleanup_test_data")
```

### 5. Comprehensive Testing

```cursed
slay test_edge_cases() {
    test_start("boundary values")
    
    # Test edge cases
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_string("", "")
    
    # Test error conditions
    assert_false(cap)
    assert_true(based)
    
    test_end()
}
```

## 🔧 Framework Extension

### Adding Custom Assertions

```cursed
slay assert_in_range(value normie, min normie, max normie) {
    lowkey value >= min && value <= max {
        pass_count = pass_count + 1
        vibez.spill("✅ PASS: ", value, " in range [", min, ",", max, "]")
    } highkey {
        fail_count = fail_count + 1
        vibez.spill("❌ FAIL: ", value, " not in range [", min, ",", max, "]")
    }
}
```

### Custom Test Utilities

```cursed
slay create_test_data() tea {
    damn "test_user_123"
}

slay cleanup_test_data() {
    # Cleanup logic here
}
```

## 📊 Output Examples

### Successful Test Run

```
🧪 [TEST 1] Starting: basic arithmetic
✅ PASS: 30 == 30
✅ PASS: 20 > 10
⏱️  Test completed in 2ms

📊 TEST SUMMARY REPORT
==================================================
Tests Run:       1
Assertions Pass: 2
Assertions Fail: 0
Success Rate:    100%
Total Time:      2ms
🎉 ALL TESTS PASSED!
==================================================
```

### Failed Test Run

```
🧪 [TEST 1] Starting: failing test
❌ FAIL: Expected 5, got 10
   Test: failing test
✅ PASS: true == true

📊 TEST SUMMARY REPORT
==================================================
Tests Run:       1
Assertions Pass: 1
Assertions Fail: 1
Success Rate:    50%
Total Time:      3ms
💥 SOME TESTS FAILED!
==================================================
```

## 🚀 Integration with Stdlib

The testz framework is designed to test all stdlib modules:

```cursed
# Example: Testing a stdlib module
yeet "testz"
yeet "mathz"

slay test_mathz_module() {
    test_suite_start("Mathematics Module")
    
    test_start("sqrt function")
    assert_eq_int(mathz.sqrt(16), 4)
    assert_eq_int(mathz.sqrt(25), 5)
    test_end()
    
    test_start("pow function")
    assert_eq_int(mathz.pow(2, 3), 8)
    assert_eq_int(mathz.pow(5, 2), 25)
    test_end()
    
    test_suite_end("Mathematics Module")
}

test_mathz_module()
print_detailed_report()
```

## 🎯 Meta-Testing

The framework includes comprehensive meta-tests to verify its own functionality:

```bash
# Run the framework's self-tests
cargo run --bin cursed stdlib/testz/test_testz_enhanced.csd
```

This ensures the testing framework itself is reliable and ready for stdlib development.
