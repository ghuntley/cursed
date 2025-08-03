# CURSED Compiler End-to-End Testing Documentation

## Overview

This directory contains comprehensive end-to-end testing infrastructure for the CURSED compiler. The tests verify the entire compilation pipeline from source code to executable binaries, ensuring all language features work correctly together.

## Test Structure

### Test Scripts

#### 1. Master Test Runner
- **`run_all_e2e_tests.sh`** - Main orchestrator that runs all test suites
  - Checks prerequisites (cargo, rustc, bc, timeout)
  - Runs all test suites in sequence
  - Provides comprehensive final report
  - Supports CI and interactive modes

#### 2. Core Pipeline Tests
- **`comprehensive_e2e_pipeline_tests.sh`** - Tests complete compilation pipeline
  - Tests interpretation and compilation modes for each program
  - Verifies executable generation and execution
  - Covers all major language features
  - Validates expected output matching

#### 3. Component Tests
- **`lexer_parser_codegen_tests.sh`** - Tests individual compiler components
  - Lexer tokenization testing
  - Parser AST generation testing
  - Code generation LLVM IR testing
  - Error handling validation for malformed input

#### 4. Integration Tests
- **`comprehensive_integration_tests.sh`** - Tests complex feature interactions
  - Data processing pipelines
  - Error handling with resource management
  - Concurrent producer-consumer patterns
  - Interface polymorphism
  - Generic container operations
  - Pattern matching with guards

#### 5. Performance and Memory Tests
- **`performance_memory_leak_tests.sh`** - Performance and memory validation
  - Compilation time benchmarks
  - Memory usage monitoring
  - Valgrind leak detection (if available)
  - Performance threshold validation

#### 6. Basic Validation
- **`basic_e2e_validation.sh`** - Quick validation of core functionality
  - Fast checks for basic language features
  - Suitable for pre-test validation
  - Identifies build issues quickly

## Test Coverage

### Language Features Tested

#### Core Language Features
- ✅ Variable declarations (`sus`)
- ✅ Function definitions (`slay`)
- ✅ Control flow (`lowkey`, `highkey`, `bestie`)
- ✅ Data types (`drip`, `tea`, `lit`, `meal`)
- ✅ Comments (`fr fr`)
- ✅ Output statements (`vibez.spill`)

#### Advanced Features
- ✅ Structs (`squad`) and methods
- ✅ Interfaces (`collab`) and implementations (`flex`)
- ✅ Generics with type parameters
- ✅ Error handling (`yikes`, `fam`, `shook`)
- ✅ Pattern matching with guards
- ✅ Concurrency (`stan`, channels)
- ✅ Collections and arrays

#### Compiler Pipeline Components
- ✅ Lexical analysis (tokenization)
- ✅ Syntax analysis (parsing to AST)
- ✅ Semantic analysis (type checking)
- ✅ Code generation (LLVM IR)
- ✅ Executable generation
- ✅ Runtime execution

### Test Programs

Each test suite includes comprehensive test programs that exercise:

1. **Basic Syntax Programs**
   - Variable declarations and assignments
   - Function calls and returns
   - Control flow statements
   - Data type operations

2. **Complex Integration Programs**
   - Multi-module systems
   - Resource management patterns
   - Error propagation chains
   - Performance-critical algorithms

3. **Edge Case Programs**
   - Boundary conditions
   - Error scenarios
   - Resource exhaustion
   - Type system edge cases

## Usage Guide

### Running All Tests

```bash
# Run complete end-to-end test suite
./tests/run_all_e2e_tests.sh

# Interactive mode with user prompts
./tests/run_all_e2e_tests.sh

# CI mode (all tests, no prompts)
CI=1 ./tests/run_all_e2e_tests.sh
```

### Running Individual Test Suites

```bash
# Quick basic validation
./tests/basic_e2e_validation.sh

# Core pipeline tests
./tests/comprehensive_e2e_pipeline_tests.sh

# Component-specific tests
./tests/lexer_parser_codegen_tests.sh

# Integration tests
./tests/comprehensive_integration_tests.sh

# Performance tests
./tests/performance_memory_leak_tests.sh
```

### Prerequisites

#### Required Tools
- `cargo` - Rust build system
- `rustc` - Rust compiler
- `bc` - Calculator for performance metrics
- `timeout` - Command timeout utility

#### Optional Tools
- `valgrind` - Memory leak detection
- `hyperfine` - Advanced benchmarking
- `gdb` - Debugging support

#### Installation Commands

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install bc coreutils valgrind
```

**macOS:**
```bash
brew install coreutils bc
```

**NixOS:**
```bash
# Tools are available in devenv.nix
direnv allow
```

## Test Configuration

### Environment Variables

- `CI` - Set to enable CI mode (no interactive prompts)
- `TIMEOUT_MULTIPLIER` - Multiply timeout values (default: 1)
- `SKIP_SLOW_TESTS` - Skip performance tests (default: false)

### Test Timeouts

- Basic tests: 10-30 seconds
- Compilation tests: 60-120 seconds
- Performance tests: 180-300 seconds
- Integration tests: 60-180 seconds

### Output Formats

All test scripts provide:
- Color-coded output (green=pass, red=fail, blue=info, yellow=warning)
- Progress indicators
- Summary statistics
- Failure details with context
- Performance metrics (where applicable)

## Interpreting Results

### Success Indicators

- **All tests passed**: Ready for production use
- **High success rate (>95%)**: Generally functional with minor issues
- **Moderate success rate (80-95%)**: Core functionality works, some advanced features need work

### Failure Analysis

#### Build Failures
- Check compilation errors with `cargo check`
- Review dependency issues
- Validate environment setup

#### Runtime Failures
- Check for missing language features
- Validate implementation completeness
- Review error handling paths

#### Performance Failures
- Monitor compilation time trends
- Check memory usage patterns
- Validate optimization effectiveness

### Common Issues and Solutions

#### Issue: "Build failed"
```bash
# Solution: Check compilation errors
cargo check
# Fix reported errors and retry
```

#### Issue: "Timeout in tests"
```bash
# Solution: Increase timeout multiplier
TIMEOUT_MULTIPLIER=2 ./tests/run_all_e2e_tests.sh
```

#### Issue: "Missing prerequisites"
```bash
# Solution: Install required tools
sudo apt install bc coreutils valgrind  # Ubuntu/Debian
brew install coreutils bc               # macOS
```

#### Issue: "Memory leak detected"
```bash
# Solution: Run with Valgrind for details
valgrind --leak-check=full ./compiled_program
```

## Test Development Guidelines

### Adding New Tests

1. **Choose appropriate test suite** based on what you're testing
2. **Follow naming conventions**: `test_feature_name.csd`
3. **Include expected output** for validation
4. **Add both positive and negative test cases**
5. **Document test purpose** in script comments

### Test Structure Template

```bash
# Test function template
run_test() {
    local test_name="$1"
    local test_file="$2"
    local expected_output="$3"
    
    echo "Testing: $test_name"
    
    # Test interpretation mode
    if cargo run --bin cursed "$test_file" | grep -q "$expected_output"; then
        echo "✅ Interpretation passed"
    else
        echo "❌ Interpretation failed"
        return 1
    fi
    
    # Test compilation mode
    if cargo run --bin cursed -- compile "$test_file"; then
        echo "✅ Compilation passed"
    else
        echo "❌ Compilation failed"
        return 1
    fi
}
```

### Best Practices

1. **Isolate tests**: Each test should be independent
2. **Clean up**: Remove temporary files after tests
3. **Timeout protection**: Use timeouts to prevent hanging
4. **Clear output**: Provide clear success/failure indicators
5. **Error context**: Include relevant error information
6. **Performance awareness**: Consider test execution time

## Integration with Development Workflow

### Pre-commit Testing
```bash
# Quick validation before committing
./tests/basic_e2e_validation.sh
```

### Development Cycle Testing
```bash
# Full validation during development
./tests/run_all_e2e_tests.sh
```

### CI/CD Integration
```bash
# Automated testing in CI
CI=1 ./tests/run_all_e2e_tests.sh
```

### Performance Monitoring
```bash
# Regular performance validation
./tests/performance_memory_leak_tests.sh
```

## Troubleshooting

### Debug Mode
Most test scripts support verbose output:
```bash
# Enable debug output
DEBUG=1 ./tests/comprehensive_e2e_pipeline_tests.sh
```

### Isolation Testing
Run individual components to isolate issues:
```bash
# Test only lexer
./tests/lexer_parser_codegen_tests.sh

# Test only basic features
./tests/basic_e2e_validation.sh
```

### Log Analysis
Test scripts generate logs in `/tmp/`:
- `/tmp/cursed_e2e_tests/` - Test program files
- `/tmp/*_output.txt` - Test execution logs
- `/tmp/build_output.txt` - Build logs

## Contributing

### Adding Test Coverage

1. Identify untested language features
2. Create comprehensive test programs
3. Add to appropriate test suite
4. Update documentation
5. Validate with existing tests

### Improving Test Infrastructure

1. Enhance error reporting
2. Add performance monitoring
3. Improve cross-platform support
4. Optimize test execution time
5. Add better failure analysis

## Conclusion

This comprehensive end-to-end testing infrastructure ensures the CURSED compiler works correctly across all supported features and platforms. Regular use of these tests helps maintain code quality and catch regressions early in the development process.

For questions or issues with the testing infrastructure, please refer to the individual test script documentation or create an issue in the repository.
