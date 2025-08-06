# End-to-End Integration Test Suite

This directory contains comprehensive end-to-end tests for the CURSED programming language compilation pipeline.

## Test Categories

1. **basic/** - Simple programs (variables, functions, basic I/O)
2. **control_flow/** - Control flow constructs (if/else, loops)
3. **data_structures/** - Data structures (structs, arrays, maps)
4. **error_handling/** - Error handling (yikes, shook)
5. **concurrency/** - Concurrency features (stan, dm channels)
6. **stdlib/** - Standard library integration (testz, vibez)
7. **integration/** - Full integration scenarios

## Test Structure

Each test directory contains:
- `*.csd` - CURSED source files
- `expected_output.txt` - Expected output for successful runs
- `error_cases/` - Test cases that should fail with specific errors

## Running Tests

```bash
# Run all e2e tests
./run_e2e_tests.sh

# Run specific category
./run_e2e_tests.sh basic
./run_e2e_tests.sh concurrency

# Run in interpretation mode only
./run_e2e_tests.sh --interpret-only

# Run in compilation mode only
./run_e2e_tests.sh --compile-only
```
