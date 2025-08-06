# CURSED End-to-End Test Suite

## Overview

This comprehensive end-to-end integration test suite validates the entire CURSED compilation pipeline from source code to execution. The test suite covers all major language features and ensures both interpretation and compilation modes work correctly.

## Test Structure

```
tests/e2e/
├── README.md                    # Test documentation
├── test_config.yaml            # Test configuration
├── run_e2e_tests.sh           # Main test runner
├── quick_test.sh              # Quick validation for development
├── basic/                     # Basic language features
│   ├── 01_variables.csd       # Variable declarations, types
│   ├── 02_functions.csd       # Function definitions, calls
│   ├── 03_basic_io.csd        # I/O operations with vibez
│   └── error_cases/           # Syntax and runtime errors
├── control_flow/              # Control flow constructs
│   ├── 01_if_else.csd         # Conditional statements
│   └── 02_loops.csd           # Loop constructs (bestie, range)
├── data_structures/           # Data structures
│   ├── 01_structs.csd         # Struct definitions, methods
│   └── 02_arrays.csd          # Array operations, slicing
├── error_handling/            # Error handling
│   └── 01_yikes_shook.csd     # Panic and recovery (yikes/shook)
├── concurrency/               # Concurrency features
│   ├── 01_stan_basics.csd     # Goroutines (stan)
│   └── 02_channels.csd        # Channel communication (dm)
├── stdlib/                    # Standard library integration
│   ├── 01_testz_framework.csd # Testing framework validation
│   └── 02_vibez_io.csd        # I/O library testing
└── integration/               # Full integration scenarios
    └── 01_full_program.csd    # Complete program demo
```

## Test Categories

### 1. Basic Features (`basic/`)
- **Variables**: Declaration, type inference, type assertions
- **Functions**: Definition, calling, parameters, return values
- **I/O**: Basic output with `vibez.spill`, formatted output with `spillf`

### 2. Control Flow (`control_flow/`)
- **Conditionals**: `ready` (if/else) statements, complex conditions
- **Loops**: `bestie` (while), `range` (for), `break`/`continue`

### 3. Data Structures (`data_structures/`)
- **Structs**: Definition with `squad`, methods, nested structs
- **Arrays**: Declaration, indexing, slicing, multi-dimensional arrays

### 4. Error Handling (`error_handling/`)
- **Panic**: `yikes` for unrecoverable errors
- **Recovery**: `shook`/`catch` for panic recovery
- **Error Returns**: Multiple return values for error handling

### 5. Concurrency (`concurrency/`)
- **Goroutines**: `stan` for concurrent execution
- **Channels**: `dm_make`, `dm_send`, `dm_recv` for communication
- **Synchronization**: Channel-based coordination

### 6. Standard Library (`stdlib/`)
- **Testing Framework**: `testz` module with assertions
- **I/O Library**: `vibez` module for input/output operations

### 7. Integration (`integration/`)
- **Full Programs**: Complete applications demonstrating all features
- **Real-world Scenarios**: Task management, concurrent processing

## Test Execution

### Test Modes

1. **Interpretation Mode**: Direct execution of CURSED source
   - Uses `cursed-zig` or `cursed-unified` binary
   - Fast execution for development

2. **Compilation Mode**: Compile to native executable then run
   - Uses `cursed-zig --compile` 
   - Tests full compilation pipeline

### Running Tests

```bash
# Run all tests in both modes
./run_e2e_tests.sh

# Run specific category
./run_e2e_tests.sh basic

# Run in interpretation mode only (faster for development)
./run_e2e_tests.sh --interpret-only

# Run with verbose output for debugging
./run_e2e_tests.sh --verbose basic

# Quick validation of critical tests
./quick_test.sh
```

### Binary Requirements

The test suite requires one of these binaries:
- `zig-out/bin/cursed-zig` (main binary, built with `zig build`)
- `cursed-unified` (unified binary, built with `zig build-exe src-zig/main_unified.zig`)

## Test Validation

Each test performs the following validations:

1. **Compilation/Interpretation**: Code must parse and execute without errors
2. **Output Verification**: Expected output is produced
3. **Assertions**: Test framework assertions must pass
4. **Error Handling**: Error cases must fail as expected

### Success Criteria

- All assertions pass (`assert_eq_int`, `assert_eq_string`, `assert_true`, `assert_false`)
- Test summary shows "All tests passed"
- No runtime panics or crashes
- Expected output matches actual output

## Implementation Status

### ✅ Working Features
- Basic variable declarations and functions
- Simple I/O operations
- Test framework integration
- Basic struct operations (limited)
- Simple goroutine execution

### ⚠️ Partially Working
- String concatenation (shows literal concatenation)
- Complex control flow
- Advanced struct methods
- Channel operations (basic implementation)

### ❌ Not Yet Implemented
- Full error handling with `yikes`/`shook`
- Advanced array operations
- Complex channel synchronization
- Compilation mode (LLVM backend issues)

## Development Workflow

1. **Quick Validation**: Run `./quick_test.sh` during development
2. **Category Testing**: Test specific features with `./run_e2e_tests.sh <category>`
3. **Full Validation**: Run complete suite before commits
4. **Debug Mode**: Use `--verbose` flag to see detailed output

## Test Configuration

The `test_config.yaml` file contains:
- Timeout settings for different test categories
- Binary path configurations
- Feature flags for conditional testing
- Validation patterns for success/failure detection

## Future Enhancements

1. **Error Case Testing**: Implement proper validation of error conditions
2. **Performance Testing**: Add benchmarks for compilation and execution speed
3. **Memory Testing**: Validate garbage collection and memory safety
4. **Cross-Platform Testing**: Test on multiple platforms and architectures
5. **Regression Testing**: Automated testing on code changes

## Troubleshooting

### Common Issues

1. **Binary Not Found**: Ensure `zig build` completes successfully or use unified binary
2. **Test Timeouts**: Increase timeout in `test_config.yaml` for slow systems
3. **String Output Issues**: String concatenation shows literal operations (known issue)
4. **Compilation Failures**: Use interpretation mode only with `--interpret-only`

### Debug Information

Use `--verbose` flag to see:
- Detailed test execution steps
- Command line invocations
- Full output and error streams
- Binary detection results

This test suite provides comprehensive validation of the CURSED language implementation and serves as both quality assurance and documentation of language capabilities.
