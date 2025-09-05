# CURSED Interpreter vs Compiler Parity Test Suite

This comprehensive test suite systematically compares interpreter vs compiled outputs to ensure true parity between execution modes in CURSED.

## Overview

The test suite validates that CURSED programs produce identical results whether run in interpreter mode or compiled to native binaries. This is crucial for ensuring the self-hosting compiler works correctly.

## Structure

```
test_suite/
├── parity_test_runner.sh          # Main test runner script
├── create_additional_tests.sh     # Script to generate more test files
├── test_programs/                 # Test program directory
│   ├── basic/                     # Basic language features
│   ├── arithmetic/                # Arithmetic operations
│   ├── functions/                 # Function definition and calls
│   ├── stdlib/                    # Standard library tests
│   ├── control_flow/              # Conditional and loop tests
│   ├── complex/                   # Complex nested operations
│   ├── strings/                   # String handling tests
│   ├── errors/                    # Error condition tests
│   ├── edge_cases/                # Boundary value tests
│   └── performance/               # Performance-heavy tests
├── results/                       # Test results and reports
└── temp/                          # Temporary execution files
```

## Usage

### Run All Tests

```bash
# Basic test run
./parity_test_runner.sh

# Verbose output
./parity_test_runner.sh --verbose

# Only show failing tests
./parity_test_runner.sh --only-failing

# Custom report file
./parity_test_runner.sh --report-file my_test_report.md
```

### Prerequisites

1. Build the CURSED compiler:
   ```bash
   cd /home/ghuntley/cursed
   make build
   ```

2. Ensure the compiler is available at `../zig-out/bin/cursed-zig`

### Test Categories

#### Basic Tests
- Hello World
- Simple arithmetic
- Variable assignment

#### Arithmetic Tests
- Mixed types (integers, floats)
- Edge cases (zero, negatives, large numbers)
- Operator precedence
- Complex expressions

#### Function Tests
- Simple function definition and calls
- Recursive functions
- Nested function calls
- Parameter handling

#### Standard Library Tests
- `mathz` module functions
- `stringz` module functions
- `collections` module (if available)
- Advanced stdlib functionality

#### Control Flow Tests
- If statements and conditionals
- Loops (while, for if available)
- Nested control structures

#### Complex Tests
- Nested operations
- FizzBuzz algorithm
- Multi-step calculations

#### String Tests
- String literals and operations
- String manipulation
- Special characters

#### Error Tests
- Division by zero
- Undefined variables
- Other error conditions

#### Edge Cases
- Boundary values
- Empty inputs
- Null-like conditions

#### Performance Tests
- Recursive depth testing
- Computation-intensive operations
- Fibonacci calculations

## Test Report

The test runner generates comprehensive reports showing:

- **Total tests run** and pass/fail breakdown
- **System Health Score** (0-100) indicating overall parity
- **Detailed failure analysis** with output comparisons
- **Recommendations** for addressing issues
- **Test category results** and coverage

### Health Score Interpretation

- **90-100:** CURSED self-hosting is production-ready
- **80-89:** Minor issues, mostly functional
- **60-79:** Moderate issues, needs work
- **40-59:** Significant problems, major work needed
- **0-39:** Critical issues, substantial development required

## Adding New Tests

### Manual Test Creation

Create `.💀` files in the appropriate `test_programs/` subdirectory:

```cursed
// Test Description
// Tests: What this test validates
// Expected: Expected behavior

fn main() {
    yap("=== Test Name ===")
    
    // Test logic here
    
    yap("=== Test Complete ===")
}
```

### Automated Test Generation

Use the helper script to create additional test files:

```bash
./create_additional_tests.sh
```

## Test Execution Details

For each test program, the runner:

1. **Interpreter Mode**: Runs `cursed-zig program.💀`
2. **Compile Mode**: Runs `cursed-zig --compile -o binary program.💀`
3. **Binary Execution**: Runs the compiled binary
4. **Output Comparison**: Compares stdout/stderr from both modes
5. **Result Classification**: Categorizes as PASS/FAIL/COMPILE_ERROR/RUNTIME_ERROR

### Output Cleaning

The test runner cleans outputs by removing:
- Memory addresses (`0x...`)
- Timing information (`X.Xms`)
- Compilation artifacts
- Debug line numbers

This ensures that only functional differences are reported.

## Continuous Integration

The test suite can be integrated into CI/CD pipelines:

```bash
# Exit code 0 = all tests pass
# Exit code 1 = some tests failed/errored
./parity_test_runner.sh --only-failing --report-file ci_report.md
```

## Troubleshooting

### Compiler Not Found
```bash
# Build the compiler first
cd /home/ghuntley/cursed
make build
```

### No Test Files Found
```bash
# Generate test files
./create_additional_tests.sh
```

### Permission Errors
```bash
# Make scripts executable
chmod +x parity_test_runner.sh
chmod +x create_additional_tests.sh
```

## Contributing

When adding new tests:

1. **Test Categories**: Place in appropriate subdirectory
2. **Naming Convention**: Use descriptive filenames with numerical prefixes
3. **Documentation**: Include test description and expected behavior
4. **Output Format**: Use consistent `yap()` statements for output
5. **Edge Cases**: Consider both success and failure scenarios

## Example Test

```cursed
// Simple Arithmetic Test
// Tests: Basic integer arithmetic operations
// Expected: Identical results in both modes

fn main() {
    yap("=== Simple Arithmetic Test ===")
    
    let a = 10
    let b = 5
    
    yap("Addition:")
    yap(a + b)
    
    yap("Subtraction:")
    yap(a - b)
    
    yap("=== Test Complete ===")
}
```

This test validates that basic arithmetic produces identical results in both interpreter and compiled modes, which is fundamental to ensuring CURSED's self-hosting capability.
