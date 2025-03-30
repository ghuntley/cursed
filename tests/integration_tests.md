# CURSED Integration Testing

This document describes the integration testing approach for the CURSED programming language.

## Rust Integration Tests

Instead of relying on shell scripts for testing, we've implemented integration tests in Rust. These tests provide:

1. Better error reporting
2. Type safety
3. Integration with Rust's test framework
4. The ability to run tests individually or as a suite

The integration tests are located in the following files:
- `tests/jit_integration_tests.rs` - Tests for the Just-In-Time execution capabilities

## Test Organization

The JIT integration tests verify that CURSED code can be:
1. Parsed correctly
2. Compiled to LLVM IR
3. Executed via JIT
4. Produce the expected output

### Available Tests

The following JIT tests are implemented:

| Test Name | Description | Status |
|-----------|-------------|--------|
| `test_puts_integer` | Tests the `puts` function with integer arguments | ✅ Passing |
| `test_println_string` | Tests the `println` function with string arguments | ✅ Passing |
| `test_variable_arithmetic` | Tests variable declarations and arithmetic operations | ✅ Passing |
| `test_if_else` | Tests conditional branching with `lowkey`/`highkey` (if/else) | ⚠️ Ignored (syntax issues) |
| `test_while_loop` | Tests looping with `periodt` (while) statement | ⚠️ Ignored (not implemented) |
| `test_complex` | Tests multiple language features together | ⚠️ Ignored (depends on unimplemented features) |

There's also a comprehensive test that runs all non-ignored tests:
- `test_all_jit_files` - Discovers and runs all CURSED test files that don't have known issues

## Running the Tests

You can run all the integration tests with:

```bash
cargo test
```

Or run a specific test file:

```bash
cargo test --test jit_integration_tests
```

Or run a specific test:

```bash
cargo test --test jit_integration_tests test_println_string
```

To see the test output:

```bash
cargo test --test jit_integration_tests -- --nocapture
```

## Extending the Tests

To add new integration tests:

1. Create a new CURSED file in the `tests/jit/` directory
2. Add a corresponding test function in `tests/jit_integration_tests.rs`
3. Run the test to verify it works as expected

For tests of features that aren't yet implemented, mark them with `#[ignore = "reason"]` to prevent them from failing the test suite while they're in development.

## Test Implementation Details

The integration tests work by:

1. Launching the CURSED compiler on test files
2. Capturing stdout and stderr
3. Checking the exit code to determine success
4. Verifying the output contains expected strings

Tests that are known to fail due to unimplemented features are tracked in a list and automatically skipped. 