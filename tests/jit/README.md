# JIT Execution Tests for CURSED

This directory contains tests for the Just-In-Time (JIT) execution capabilities of the CURSED programming language.

## Test Files

1. **puts_integer.csd** - Basic test for outputting an integer using the `puts` function
2. **println_string.csd** - Basic test for outputting a string using the `println` function
3. **variable_arithmetic.csd** - Tests variable declaration, assignment, and arithmetic operations
4. **if_else.csd** - Tests conditional branching with `lowkey`/`highkey` (if/else)
5. **while_loop.csd** - Tests looping with `periodt` (while) statement
6. **complex_test.csd** - A comprehensive test combining multiple language features:
   - Variable declarations and assignments
   - String and integer literals
   - Conditional logic
   - Loops
   - Multiple output statements

## Running the Tests

You can run all the tests with the `run_jit_tests.sh` script in the parent directory:

```bash
cd /path/to/cursed
./tests/run_jit_tests.sh
```

Alternatively, you can run individual tests directly:

```bash
cd /path/to/cursed
cargo run -- tests/jit/puts_integer.csd
```

## Expected Results

All tests should compile successfully and execute without errors. The JIT execution should produce the expected output for each test, verifying that the various language features correctly generate executable LLVM IR.

## Test Coverage

The tests collectively verify the following aspects of CURSED's JIT execution:

- Compilation from source to LLVM IR
- Execution of LLVM IR via JIT
- Correct handling of literals (string, integer)
- Proper variable declaration and scoping
- Arithmetic operations
- Control flow constructs (if/else, while)
- Input/output functionality (puts, println)

## Extending Tests

To add new tests:
1. Create a new `.csd` file in this directory
2. Write CURSED code that tests specific language features
3. Run the tests to verify that the feature works as expected
