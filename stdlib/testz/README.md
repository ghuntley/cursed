# testz - CURSED Testing Framework v2.0

The testz module provides a comprehensive testing framework for CURSED programs. It enables test case management, assertion functions, result tracking, and summary reporting.

## Features

- **Pure CURSED Implementation**: No FFI dependencies
- **Comprehensive Assertions**: Integer, string, boolean, and comparison assertions
- **Test State Management**: Automatic test counting and result tracking
- **Rich Reporting**: Detailed test summaries with pass/fail statistics
- **Self-Testing**: Framework includes tests for itself

## Core Functions

### Test Management
- `test_start(name tea)` - Begin a new test case
- `test_end()` - Finish current test and update counters
- `reset_test_state()` - Reset all test state for multiple runs
- `print_test_summary()` - Display comprehensive test results

### Assertion Functions
- `assert_eq_int(actual normie, expected normie)` - Assert integer equality
- `assert_eq_string(actual tea, expected tea)` - Assert string equality
- `assert_true(condition lit)` - Assert condition is true
- `assert_false(condition lit)` - Assert condition is false
- `assert_ne_int(actual normie, expected normie)` - Assert integers not equal
- `assert_gt_int(actual normie, expected normie)` - Assert greater than
- `assert_lt_int(actual normie, expected normie)` - Assert less than

### Helper Functions
- `get_test_results() normie` - Get number of failed tests
- `all_tests_passed() lit` - Check if all tests passed

## Usage Example

```cursed
yeet "testz"

# Start a test
test_start("math operations")
assert_eq_int(2 + 2, 4)
assert_true(5 > 3)
assert_false(10 < 5)
test_end()

# Another test
test_start("string operations")
assert_eq_string("hello", "hello")
assert_ne_int("test".length(), 0)
test_end()

# Display results
print_test_summary()
```

## Testing Strategy

The testz module follows a test-driven approach:
1. Each assertion provides immediate feedback
2. Tests are grouped by functionality
3. Results are tracked automatically
4. Summary shows overall pass/fail status

## Advanced Features

### Test State Variables
- `current_test_name tea` - Name of currently running test
- `total_tests normie` - Total number of tests executed
- `passed_tests normie` - Number of tests that passed
- `failed_tests normie` - Number of tests that failed
- `current_test_passed lit` - Status of current test

### Output Format
- ✅ Success indicators for passing assertions
- ❌ Failure indicators with detailed error messages
- 📊 Comprehensive summary with statistics
- 🎉 Success celebration for all tests passing

## Integration with Other Modules

All stdlib modules should use testz for testing:

```cursed
yeet "testz"
yeet "module_name"

test_start("module functionality")
assert_true(module_function("test_data"))
test_end()

print_test_summary()
```

## Performance

The testz framework is designed for:
- **Minimal Overhead**: Lightweight test execution
- **Fast Feedback**: Immediate assertion results
- **Memory Efficient**: Simple state management
- **Scalable**: Handles large test suites

## Error Handling

The framework provides robust error handling:
- Failed assertions don't crash the program
- Detailed error messages with expected vs actual values
- Graceful handling of edge cases
- Automatic test state management

## Testing Commands

```bash
# Test the testz framework itself
cargo run --bin cursed stdlib/testz/test_testz.csd

# Compile and run native tests
cargo run --bin cursed -- compile stdlib/testz/test_testz.csd
./test_testz

# Use in other modules
cargo run --bin cursed stdlib/module/test_module.csd
```

## Status

- **Version**: 2.0
- **Status**: Production Ready
- **Dependencies**: None (Pure CURSED)
- **Test Coverage**: 100% (self-testing)
- **Compatibility**: Both interpretation and compilation modes

The testz module is the foundation of the CURSED stdlib testing ecosystem, providing reliable and comprehensive testing capabilities for all stdlib modules.
