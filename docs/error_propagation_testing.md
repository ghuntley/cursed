# Error Propagation Testing Documentation

This document provides comprehensive information about the CURSED error propagation testing infrastructure, including test coverage, execution strategies, and maintenance guidelines.

## Overview

The error propagation system enables the `?` operator to work with `Result<T, E>` and `Option<T>` types, providing early return semantics similar to Rust. The testing infrastructure validates this functionality across multiple dimensions:

- **Integration Testing**: End-to-end functionality validation
- **LLVM IR Generation**: Correctness of compiled output
- **Runtime Execution**: Behavior during program execution
- **Performance Analysis**: Compilation and execution efficiency
- **Memory Safety**: Resource management and cleanup
- **Concurrency**: Thread safety and parallel execution

## Test Suite Structure

### 1. Integration Tests (`tests/error_propagation_integration_test.rs`)

**Purpose**: Validate complete error propagation pipeline from source to execution.

**Key Test Categories**:
- Basic Result<T,E> and Option<T> propagation
- Chained error propagation (`a?.b?.c?`)
- Mixed Result/Option patterns
- Generic type integration
- Custom error types
- Complex control flow scenarios
- Performance characteristics
- Memory safety validation

**Critical Tests**:
```rust
test_basic_error_propagation_compilation()     // Basic ? operator
test_chained_error_propagation()               // Multiple ? in sequence
test_mixed_result_option_propagation()         // Result + Option mixing
test_error_propagation_contexts()              // Different syntactic contexts
test_error_propagation_performance()           // Compilation performance
test_error_propagation_memory_safety()         // Memory management
```

### 2. LLVM IR Generation Tests (`tests/error_propagation_llvm_test.rs`)

**Purpose**: Ensure correct LLVM IR generation for all error propagation patterns.

**Key Test Categories**:
- IR structure validation
- Type-specific IR patterns
- Control flow correctness
- Optimization opportunities
- Debug information preservation
- Helper method validation

**Critical Tests**:
```rust
test_basic_result_ir_generation()              // Result<T,E> IR patterns
test_option_ir_generation()                    // Option<T> IR patterns  
test_chained_propagation_ir()                  // Complex control flow
test_ir_validity()                             // LLVM IR structural validation
test_ir_specific_patterns()                    // Pattern-specific generation
```

### 3. Runtime Execution Tests (`tests/error_propagation_runtime_test.rs`)

**Purpose**: Validate runtime behavior and performance characteristics.

**Key Test Categories**:
- Success path execution
- Error path handling
- Performance benchmarks
- Memory safety during execution
- Concurrent execution
- Resource cleanup
- Stack trace preservation

**Critical Tests**:
```rust
test_runtime_basic_result_propagation()        // Basic execution success
test_runtime_result_error_propagation()        // Error case handling
test_runtime_performance_success_path()        // Success path performance
test_runtime_performance_error_path()          // Error path performance
test_runtime_concurrent_propagation()          // Thread safety
test_runtime_memory_safety()                   // Resource management
```

## Test Execution

### Quick Validation
```bash
# Run essential error propagation tests
make error-propagation-test-quick

# Or directly:
./tests/run_error_propagation_tests.sh --quick
```

### Category-Specific Testing
```bash
# Integration tests only
make error-propagation-test-integration

# LLVM IR generation tests
make error-propagation-test-llvm

# Runtime execution tests  
make error-propagation-test-runtime

# Performance benchmarks
make error-propagation-test-performance
```

### Comprehensive Testing
```bash
# Run all error propagation tests
make error-propagation-test-all

# With coverage analysis
make error-propagation-test-coverage

# Generate detailed report
make error-propagation-test-report
```

### Advanced Options
```bash
# Verbose output with detailed logging
./tests/run_error_propagation_tests.sh --verbose

# Parallel execution where possible
./tests/run_error_propagation_tests.sh --parallel

# Custom report location
./tests/run_error_propagation_tests.sh --report custom_report.md
```

## Test Coverage Areas

### 1. Syntax and Parsing
- `?` operator recognition and parsing
- AST node generation for error propagation
- Integration with expression parsing
- Error handling for malformed syntax

### 2. Type System Integration
- Result<T, E> type validation
- Option<T> type validation
- Generic type parameter handling
- Custom error type support
- Type inference with error propagation

### 3. Code Generation
- LLVM IR instruction generation
- Control flow graph construction
- Optimization opportunity identification
- Debug information preservation
- FFI function integration

### 4. Runtime Behavior
- Early return semantics
- Error value propagation
- Stack unwinding simulation
- Memory cleanup on error paths
- Exception-like behavior patterns

### 5. Performance Characteristics
- Compilation time impact
- Runtime overhead measurement
- Memory usage analysis
- Optimization effectiveness
- Scalability with chain length

## Key Testing Scenarios

### Basic Error Propagation
```cursed
function example() -> Result<i32, String> {
    sus value = might_fail()?;
    facts value + 10
}
```
**Tests**: Compilation, IR generation, success/error execution paths

### Chained Propagation
```cursed
function chain_example() -> Result<i32, String> {
    sus result = step_one()?.step_two()?.step_three()?;
    facts result
}
```
**Tests**: Multiple error checks, control flow complexity, performance impact

### Mixed Types
```cursed
function mixed_example() -> Result<i32, String> {
    sus optional = get_option()?;  // Option<T>
    sus result = process(optional)?;  // Result<T,E>
    facts result
}
```
**Tests**: Type system integration, conversion handling, IR correctness

### Complex Control Flow
```cursed
function complex_example() -> Result<i32, String> {
    lowkey (condition()?) {
        sus value = true_branch()?;
        bestie (value < 100) {
            value = process(value)?;
        }
        facts value
    } flex {
        facts false_branch()?
    }
}
```
**Tests**: Branching with error propagation, loop integration, nested contexts

## Performance Expectations

### Compilation Performance
- **Target**: < 5 seconds for 20+ chained operations
- **Measurement**: Compilation time from source to LLVM IR
- **Baseline**: Single function with multiple ? operators

### Runtime Performance  
- **Success Path**: Minimal overhead vs direct calls
- **Error Path**: Early return should be faster than full execution
- **Memory**: No leaks in error or success paths

### Scalability
- **Linear growth** with chain length for compilation
- **Constant overhead** per ? operator at runtime
- **Memory usage** should not grow with chain depth

## Memory Safety Validation

### Resource Cleanup
```cursed
function resource_test() -> Result<i32, String> {
    sus resource = acquire_resource()?;
    sus result = use_resource(resource)?;
    // Automatic cleanup should occur on both success and error
    facts result
}
```

### Stack Safety
- No buffer overflows in error handling
- Proper stack unwinding simulation
- Call stack preservation for debugging

### Heap Safety
- No memory leaks on error paths
- Proper deallocation of temporary values
- No double-free errors

## Error Scenarios and Edge Cases

### 1. Malformed Syntax
- Missing ? operator
- Invalid placement of ?
- Type mismatches with error propagation

### 2. Type System Edge Cases
- Complex generic constraints
- Recursive error types
- Multiple error type conversions

### 3. Runtime Edge Cases
- Stack overflow with deep chains
- Out of memory conditions
- Concurrent modification during propagation

### 4. Performance Edge Cases
- Very long propagation chains (100+ operators)
- High-frequency error propagation
- Large error values requiring copying

## Test Maintenance Guidelines

### Adding New Tests
1. **Identify the category**: Integration, LLVM, or Runtime
2. **Follow naming convention**: `test_[category]_[specific_feature]()`
3. **Include documentation**: Explain why the test is important
4. **Add performance expectations**: Include timing assertions where relevant
5. **Update test runner**: Add to appropriate test categories

### Updating Existing Tests
1. **Maintain backward compatibility**: Don't break existing test interfaces
2. **Update documentation**: Reflect changes in test behavior
3. **Performance baselines**: Update if implementation improves
4. **Add regression protection**: Test previously failing scenarios

### Test Infrastructure Updates
1. **Script enhancements**: Improve test runner capabilities
2. **Reporting improvements**: Add new metrics or visualizations
3. **CI/CD integration**: Ensure tests run in automated environments
4. **Coverage analysis**: Maintain high coverage percentages

## Troubleshooting Common Issues

### Compilation Failures
```bash
# Check LLVM IR generation
./tests/run_error_propagation_tests.sh --test llvm --verbose

# Verify type system integration
cargo test --test error_propagation_integration_test test_error_propagation_type_inference
```

### Runtime Failures
```bash
# Check runtime execution
./tests/run_error_propagation_tests.sh --test runtime --verbose

# Focus on specific runtime behavior
cargo test --test error_propagation_runtime_test test_runtime_basic_result_propagation
```

### Performance Issues
```bash
# Run performance benchmarks
./tests/run_error_propagation_tests.sh --performance

# Analyze specific performance tests
cargo test --test error_propagation_integration_test test_error_propagation_performance -- --nocapture
```

### Linking Issues (Nix Environment)
The test runner automatically applies the linking fix for Nix environments:
```bash
# Manual linking fix if needed
./fix_linking.sh cargo test --test error_propagation_integration_test
```

## Integration with CI/CD

### Automated Testing
The test suite is designed for CI/CD integration:
```yaml
# Example CI configuration
- name: Error Propagation Tests
  run: |
    make error-propagation-test-all
    make error-propagation-test-coverage
```

### Test Reporting
Automated reports can be generated for CI systems:
```bash
./tests/run_error_propagation_tests.sh --report ci_report.md --coverage
```

### Performance Monitoring
Track performance regressions in CI:
```bash
./tests/run_error_propagation_tests.sh --performance --report performance_baseline.md
```

## Future Enhancements

### Test Coverage Expansion
- [ ] More complex generic scenarios
- [ ] Integration with custom derive macros
- [ ] Foreign function interface (FFI) error propagation
- [ ] Async/await integration patterns

### Performance Optimization Testing
- [ ] LLVM optimization pass validation
- [ ] Profile-guided optimization testing
- [ ] Micro-benchmark integration
- [ ] Memory allocation pattern analysis

### Advanced Error Handling
- [ ] Stack trace capture and analysis
- [ ] Error context propagation
- [ ] Custom error conversion testing
- [ ] Error recovery mechanism validation

## References

- [CURSED Language Specification](../specs/language_spec.md)
- [Error Handling Design](../docs/error_handling_design.md)
- [LLVM Integration Guide](../docs/llvm_integration.md)
- [Testing Framework Documentation](../docs/testing_framework.md)

## Support

For questions about the error propagation testing infrastructure:

1. **Review this documentation** for comprehensive coverage
2. **Run diagnostic tests** using the test runner
3. **Check existing test examples** for implementation patterns
4. **Create issues** for bugs or enhancement requests

The error propagation testing infrastructure provides comprehensive validation for one of CURSED's core features, ensuring reliability, performance, and correctness across all usage scenarios.
