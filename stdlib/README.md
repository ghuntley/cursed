# CURSED Standard Library Test Suite

This directory contains comprehensive tests for all CURSED standard library modules using the native CURSED testing framework (testz).

## Overview

The CURSED standard library provides essential functionality across multiple domains:

- **Math** - Mathematical functions, constants, and calculations
- **String** - String manipulation, searching, and formatting
- **Crypto** - Cryptographic operations and security functions
- **I/O** - File system operations and console input/output
- **Collections** - Data structures (arrays, maps, sets, queues, stacks)
- **Time** - Date/time operations, formatting, and timing utilities

## Test Structure

Each module has its own comprehensive test suite:

```
stdlib/
├── testz/          # Testing framework
│   ├── mod.csd     # Core testing functions
│   └── README.md   # Testing framework documentation
├── math/
│   ├── mod.csd     # Math library implementation
│   ├── test_math.csd      # Math library tests
│   └── README.md   # Math library documentation
├── string/
│   ├── mod.csd     # String library implementation
│   ├── test_string.csd    # String library tests
│   └── README.md   # String library documentation
├── crypto/
│   ├── mod.csd     # Crypto library implementation
│   ├── test_crypto.csd    # Crypto library tests
│   └── README.md   # Crypto library documentation
├── io/
│   ├── mod.csd     # I/O library implementation
│   ├── test_io.csd        # I/O library tests
│   └── README.md   # I/O library documentation
├── collections/
│   ├── mod.csd     # Collections library implementation
│   ├── test_collections.csd  # Collections library tests
│   └── README.md   # Collections library documentation
├── time/
│   ├── mod.csd     # Time library implementation
│   ├── test_time.csd      # Time library tests
│   └── README.md   # Time library documentation
├── test_all_stdlib.csd    # Master test runner
└── README.md       # This file
```

## Running Tests

### Individual Module Tests

Run tests for specific modules:

```bash
# Math library tests
cargo run --bin cursed stdlib/math/test_math.csd

# String library tests
cargo run --bin cursed stdlib/string/test_string.csd

# Crypto library tests
cargo run --bin cursed stdlib/crypto/test_crypto.csd

# I/O library tests
cargo run --bin cursed stdlib/io/test_io.csd

# Collections library tests
cargo run --bin cursed stdlib/collections/test_collections.csd

# Time library tests
cargo run --bin cursed stdlib/time/test_time.csd
```

### All Standard Library Tests

Run the complete test suite:

```bash
# Run all stdlib tests with master runner
cargo run --bin cursed stdlib/test_all_stdlib.csd

# Run using CURSED test command
cargo run --bin cursed test --test-dir stdlib

# Run with specific patterns
cargo run --bin cursed test --pattern "test_*.csd"
```

## Testing Framework (testz)

The CURSED testing framework provides:

### Core Functions
- `testz.test_start(name)` - Begin a new test
- `testz.test_pass(message)` - Mark test as passed
- `testz.test_fail(message)` - Mark test as failed

### Assertion Functions
- `testz.assert_eq_int(actual, expected)` - Assert integer equality
- `testz.assert_eq_string(actual, expected)` - Assert string equality
- `testz.assert_eq_bool(actual, expected)` - Assert boolean equality
- `testz.assert_true(condition)` - Assert condition is true
- `testz.assert_false(condition)` - Assert condition is false

### Test Management
- `testz.print_test_summary()` - Display test results
- `testz.run_all_tests()` - Return exit code based on results
- `testz.reset_test_state()` - Reset for new test run

### Usage Example

```cursed
yeet "testz"
yeet "math"

slay test_basic_math() {
    testz.test_start("Basic Math Operations")
    
    testz.assert_eq_int(math_abs(-5), 5)
    testz.assert_eq_string(tea(math_pi()), "3.141592653589793")
    testz.assert_true(math_sqrt(4.0) == 2.0)
}

test_basic_math()
testz.print_test_summary()
```

## Test Coverage

### Math Library (test_math.csd)
- ✅ Mathematical constants (π, e, τ)
- ✅ Basic operations (abs, min, max, clamp)
- ✅ Power functions (pow, sqrt, cbrt)
- ✅ Trigonometric functions (sin, cos, tan, etc.)
- ✅ Rounding functions (floor, ceil, round)
- ✅ Utility functions (gcd, lcm, factorial)
- ✅ Random number generation
- ✅ Edge cases and special values

### String Library (test_string.csd)
- ✅ String properties (length, empty check)
- ✅ Case conversion (upper, lower, capitalize)
- ✅ String trimming and padding
- ✅ Search operations (contains, index, count)
- ✅ String slicing and splitting
- ✅ String replacement and repetition
- ✅ Validation (numeric, alpha, etc.)
- ✅ Type conversion (string ↔ int/float/bool)
- ✅ Regular expressions
- ✅ Edge cases and Unicode handling

### Crypto Library (test_crypto.csd)
- ✅ Hash functions (SHA-256, SHA-512, MD5, BLAKE3)
- ✅ Random generation (bytes, integers, strings)
- ✅ Base encoding (Base64, Hex)
- ✅ Symmetric encryption (AES)
- ✅ Message authentication (HMAC)
- ✅ Key derivation (PBKDF2, Scrypt)
- ✅ Digital signatures (Ed25519)
- ✅ Password hashing (Argon2, bcrypt)
- ✅ Security utilities (constant-time comparison)

### I/O Library (test_io.csd)
- ✅ Console I/O (print, read)
- ✅ File operations (read, write, delete, copy, move)
- ✅ Binary file operations
- ✅ Directory operations
- ✅ Path manipulation
- ✅ Stream I/O
- ✅ Buffered I/O
- ✅ Temporary files
- ✅ File metadata and timestamps
- ✅ Error handling and edge cases

### Collections Library (test_collections.csd)
- ✅ Array operations (push, pop, insert, remove)
- ✅ Array searching and manipulation
- ✅ HashMap operations (set, get, remove)
- ✅ Set operations (add, remove, union, intersection)
- ✅ Queue operations (FIFO)
- ✅ Stack operations (LIFO)
- ✅ Utility functions (range, zip, unique)
- ✅ Type conversions between collections
- ✅ Edge cases and memory management

### Time Library (test_time.csd)
- ✅ Current time functions
- ✅ Time creation and parsing
- ✅ Time formatting
- ✅ Date/time component extraction
- ✅ Time arithmetic
- ✅ Duration operations
- ✅ Timezone handling
- ✅ Time validation
- ✅ Sleep and timing functions
- ✅ Benchmarking utilities
- ✅ Edge cases and leap year handling

## Test Results

All tests are designed to:
- Verify correct functionality in both interpretation and compilation modes
- Test edge cases and error conditions
- Ensure proper type handling
- Validate performance characteristics
- Check integration between modules

## Continuous Integration

The test suite can be integrated into CI/CD pipelines:

```bash
# Basic test run
cargo run --bin cursed test

# Test with verbose output
cargo run --bin cursed test --verbose

# Test with specific timeout
cargo run --bin cursed test --timeout 60

# Test with different output formats
cargo run --bin cursed test --format json
cargo run --bin cursed test --format xml
```

## Contributing

When adding new stdlib functionality:

1. **Add tests first** - Write comprehensive tests for new functions
2. **Follow patterns** - Use existing test structure and naming
3. **Test edge cases** - Include boundary conditions and error cases
4. **Update documentation** - Keep README files current
5. **Run full suite** - Ensure all tests pass before committing

### Test Naming Conventions

- Test files: `test_[module].csd`
- Test functions: `test_[category]_[operation]()`
- Test names: Descriptive strings explaining what's being tested

### Example New Test

```cursed
slay test_new_feature() {
    testz.test_start("New Feature Testing")
    
    fr fr Test basic functionality
    testz.assert_eq_int(new_function(5), 25)
    
    fr fr Test edge cases
    testz.assert_true(new_function(0) == 0)
    testz.assert_true(new_function(-1) == 1)
    
    fr fr Test error conditions
    // Add appropriate error testing
}
```

## Performance Considerations

The test suite is designed to:
- Run quickly for development feedback
- Cover comprehensive scenarios
- Avoid system-dependent timing issues
- Handle resource cleanup automatically
- Scale with library growth

Tests that create files, network connections, or other resources automatically clean up after themselves to prevent test pollution.
