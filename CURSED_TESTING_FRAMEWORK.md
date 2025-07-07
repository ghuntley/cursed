# CURSED Testing Framework Documentation

## 🎯 Overview

The CURSED Testing Framework provides a standardized way to run tests written in the CURSED language (.csd files) for both the standard library and user code. This framework ensures that the stdlib is tested using CURSED itself, creating a fully self-hosted testing environment.

## 🏗️ Architecture

### Core Components

1. **Test Runner (`src/tools/test_runner.rs`)**: Rust-based test discovery and execution engine
2. **Testing Framework (`stdlib/testz/mod.csd`)**: CURSED-based assertion and reporting library
3. **Test Discovery**: Automatic finding of `test_*.csd` and `*_test.csd` files
4. **CLI Integration**: Built into the main `cursed` command with `test` subcommand

### Directory Structure

```
stdlib/
├── math/
│   ├── mod.csd              # Implementation
│   ├── test_math.csd        # Tests
│   └── README.md
├── crypto/
│   ├── mod.csd              # Implementation  
│   ├── test_crypto.csd      # Tests
│   └── README.md
├── string/
│   ├── mod.csd              # Implementation
│   ├── test_string.csd      # Tests
│   └── README.md
├── testz/                   # Testing framework
│   ├── mod.csd              # Core testing functions
│   ├── runner.csd           # Test discovery & execution
│   └── test_testz.csd       # Self-tests
└── README.md
```

## 🚀 Quick Start

### Run All Tests
```bash
cargo run --bin cursed test
```

### Run Specific Module Tests
```bash
cargo run --bin cursed test --filter math
cargo run --bin cursed test --filter crypto
cargo run --bin cursed test --filter string
```

### Run with Different Output Formats
```bash
cargo run --bin cursed test --format pretty    # Default colorized output
cargo run --bin cursed test --format json      # JSON report
cargo run --bin cursed test --format xml       # JUnit XML
cargo run --bin cursed test --format html      # HTML report
```

## 📝 Writing Tests

### Test File Structure

Every test file should follow this pattern:

```cursed
fr fr Module description
fr fr Test file for [module_name]

yeet "testz"               fr fr Import testing framework
yeet "module_to_test"      fr fr Import module being tested

fr fr Test function - must start with "test_"
slay test_function_name() {
    testz.test_start("Test Description")
    
    fr fr Your test code here
    testz.assert_eq_int(actual, expected)
    testz.assert_eq_string(actual, expected)
    testz.assert_true(condition)
    testz.assert_false(condition)
}

slay main() {
    vibez.spill("🧪 Starting [Module] Tests")
    
    testz.reset_test_state()
    
    test_function_name()
    fr fr Add more test function calls here
    
    sus exit_code normie = testz.run_all_tests()
    vibez.spill("Tests completed!")
    
    damn exit_code
}
```

### Naming Conventions

- **Test Files**: `test_*.csd` or `*_test.csd`
- **Test Functions**: Must start with `test_`
- **Test Modules**: Place in same directory as implementation

### Available Assertions

```cursed
fr fr Integer comparisons
testz.assert_eq_int(actual, expected)

fr fr String comparisons
testz.assert_eq_string(actual, expected)

fr fr Boolean assertions
testz.assert_true(condition)
testz.assert_false(condition)
testz.assert_eq_bool(actual, expected)

fr fr Test lifecycle
testz.test_start("Test Name")        fr fr Begin a test
testz.test_pass("Success message")   fr fr Mark test as passed
testz.test_fail("Failure message")   fr fr Mark test as failed
testz.reset_test_state()             fr fr Reset counters
testz.run_all_tests()                fr fr Generate summary & return exit code
```

## 🔧 Command Line Options

### Basic Usage
```bash
cargo run --bin cursed test [OPTIONS]
```

### Available Options

| Option | Description | Default |
|--------|-------------|---------|
| `--test-dir DIR` | Directory to search for tests | `stdlib` |
| `--pattern PATTERN` | Test file pattern | `test_*.csd` |
| `--filter FILTER` | Filter tests by name | None |
| `--parallel` | Run tests in parallel | `false` |
| `--timeout SECONDS` | Test timeout | `30` |
| `--fail-fast` | Stop on first failure | `false` |
| `--format FORMAT` | Output format (pretty/json/xml/html) | `pretty` |
| `--verbose` | Verbose output | `false` |

### Examples

```bash
# Run all stdlib tests
cargo run --bin cursed test

# Run only math tests
cargo run --bin cursed test --filter math

# Run tests in parallel with JSON output
cargo run --bin cursed test --parallel --format json

# Run tests with 60 second timeout, stop on first failure
cargo run --bin cursed test --timeout 60 --fail-fast

# Run tests in custom directory
cargo run --bin cursed test --test-dir my_project/tests

# Verbose output with specific pattern
cargo run --bin cursed test --pattern "*_test.csd" --verbose
```

## 📊 Output Formats

### Pretty Format (Default)
Colorized console output with emojis and progress indicators:
```
🚀 CURSED stdlib test runner
INFO Found 3 test files
  ✓ test_math (250ms)
  ✓ test_crypto (180ms)
  ✗ test_string (120ms)
    String comparison failed: got "Hello", expected "hello"

============================================================
📋 CURSED STDLIB TEST SUMMARY
============================================================
Total tests: 3
Passed: 2
Failed: 1
Success rate: 66.7%

❌ FAILED TESTS:
  test_string - String comparison failed

❌ 1 tests failed
```

### JSON Format
Machine-readable JSON for CI/CD integration:
```json
{
  "summary": {
    "total": 3,
    "passed": 2,
    "failed": 1,
    "success_rate": 66.7
  },
  "tests": [
    {
      "name": "test_math",
      "file": "stdlib/math/test_math.csd",
      "passed": true,
      "message": "Test executed successfully",
      "duration": 250
    }
  ]
}
```

### XML Format
JUnit-compatible XML for integration with test reporting tools:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite tests="3" failures="1" time="550">
  <testcase name="test_math" classname="stdlib/math/test_math.csd" time="250">
  </testcase>
  <testcase name="test_string" classname="stdlib/string/test_string.csd" time="120">
    <failure message="String comparison failed" type="TestFailure">
      String comparison failed: got "Hello", expected "hello"
    </failure>
  </testcase>
</testsuite>
```

### HTML Format
Rich HTML report for web viewing:
```html
<!DOCTYPE html>
<html>
<head>
  <title>CURSED Test Report</title>
  <style>/* Styling for test results */</style>
</head>
<body>
  <h1>CURSED Stdlib Test Report</h1>
  <!-- Summary and detailed results -->
</body>
</html>
```

## 🔄 Integration with Development Workflow

### 1. During Development
```bash
# Quick test of specific module
cargo run --bin cursed test --filter math --fail-fast

# Continuous testing during development
cargo run --bin cursed test --verbose
```

### 2. CI/CD Integration
```bash
# Generate machine-readable results
cargo run --bin cursed test --format json > test_results.json

# JUnit XML for CI systems
cargo run --bin cursed test --format xml > test_results.xml
```

### 3. Pre-commit Hooks
```bash
# Ensure all tests pass before commit
cargo run --bin cursed test --fail-fast
```

## 🎯 Best Practices

### Test Organization
1. **One test file per module**: `stdlib/math/test_math.csd`
2. **Comprehensive coverage**: Test all public functions
3. **Edge cases**: Test boundary conditions and error cases
4. **Clear naming**: Descriptive test function names

### Test Writing
1. **Independent tests**: Each test should be self-contained
2. **Clear assertions**: Use appropriate assertion functions
3. **Good error messages**: Provide context in test names
4. **Test data**: Use meaningful test inputs

### Performance
1. **Fast tests**: Keep individual tests under 1 second
2. **Parallel execution**: Use `--parallel` for large test suites
3. **Timeouts**: Set appropriate timeouts for long-running tests

## 🚨 Troubleshooting

### Common Issues

#### 1. Test Not Found
```
WARNING No test files found in stdlib
```
**Solution**: Ensure test files follow naming convention (`test_*.csd` or `*_test.csd`)

#### 2. Test Timeout
```
⏱ test_math TIMEOUT
```
**Solution**: Increase timeout with `--timeout 60` or optimize test

#### 3. Import Errors
```
Failed to import "testz"
```
**Solution**: Ensure `yeet "testz"` is at top of test file

#### 4. Assertion Failures
```
✗ assert_eq_int failed: got 42, expected 24
```
**Solution**: Check test logic and expected values

### Debug Mode
```bash
# Run with maximum verbosity
cargo run --bin cursed test --verbose

# Test single file for debugging
cargo run --bin cursed stdlib/math/test_math.csd
```

## 🔮 Future Enhancements

### Planned Features
1. **Code Coverage**: Track which code is tested
2. **Benchmark Testing**: Performance regression detection
3. **Property-Based Testing**: Automated test case generation
4. **Test Fixtures**: Setup and teardown functions
5. **Mocking Framework**: Mock dependencies for unit tests

### Advanced Features
1. **Parallel Test Execution**: True parallelism within test files
2. **Test Tagging**: Categorize and filter tests by tags
3. **Integration Testing**: Cross-module integration tests
4. **Visual Test Reports**: Rich HTML reports with charts

## 📚 Examples

See the following example test files:
- [`stdlib/math/test_math.csd`](stdlib/math/test_math.csd) - Math function testing
- [`stdlib/crypto/test_crypto.csd`](stdlib/crypto/test_crypto.csd) - Cryptographic function testing  
- [`stdlib/string/test_string.csd`](stdlib/string/test_string.csd) - String manipulation testing

## 🤝 Contributing

When adding new stdlib modules:
1. Create implementation in `stdlib/module/mod.csd`
2. Create tests in `stdlib/module/test_module.csd`
3. Follow the testing patterns shown in examples
4. Ensure all tests pass with `cargo run --bin cursed test --filter module`

---

The CURSED Testing Framework ensures that the standard library is thoroughly tested using the CURSED language itself, creating a true self-hosted testing environment that validates both the language implementation and the standard library functionality.
