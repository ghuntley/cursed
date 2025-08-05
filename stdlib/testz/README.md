# CURSED Testing Framework (testz) - Comprehensive Documentation

## Overview

The CURSED Testing Framework (`testz`) provides a comprehensive, production-ready testing infrastructure for the CURSED standard library and user applications. Built entirely in pure CURSED, it offers advanced testing capabilities including property-based testing, benchmarking, coverage tracking, and automated test discovery.

## Features

### ✅ Core Testing Primitives
- **Basic Assertions**: `assert_true`, `assert_false`, `assert_eq_int`, `assert_eq_string`
- **Advanced Assertions**: `assert_near`, `assert_throws`, `assert_array_eq`
- **Test Organization**: `test_start`, `test_group_start`, `test_group_end`
- **Result Reporting**: `print_test_summary`, coverage reports, benchmark summaries

### ✅ Advanced Testing Features
- **Property-Based Testing**: Generate random inputs and verify properties hold
- **Benchmarking**: Performance measurement with operations per second
- **Memory Testing**: Track memory usage and detect leaks
- **Coverage Tracking**: Line-level code coverage analysis
- **Test Discovery**: Automatic discovery of all stdlib test files

### ✅ Test Templates and Utilities
- **Module Templates**: Generate comprehensive test suites for new modules
- **Type-Specific Templates**: Specialized templates for collections, math, I/O, crypto
- **Property Testing Templates**: Ready-to-use property test cases
- **Error Handling Templates**: Comprehensive error condition testing

## Quick Start

### Basic Test Structure

```cursed
fr fr Import the testing framework
yeet "testz"
yeet "testz/advanced"  fr fr For advanced features
yeet "your_module"     fr fr Module under test

fr fr Start a test group
test_group_start("Your Module Tests")

fr fr Basic test
test_start("basic functionality")
sus result = your_function("input")
assert_eq_string(result, "expected_output")

fr fr Property-based test
property_test(PropertyTestCase{
    name: "your_property",
    generator: slay() tea { damn "random_input" },
    property: slay(input tea) lit {
        sus result = your_function(input)
        damn result != ""  fr fr Property: never returns empty
    },
    iterations: 100
})

fr fr Benchmark test
benchmark("your_function performance", slay() {
    your_function("benchmark_input")
})

test_group_end()
print_test_summary()
print_benchmark_summary()
```

### Running Tests

```bash
# Run a single test file
./cursed-unified stdlib/module/test_module.csd

# Run all stdlib tests (automated discovery)
./cursed-unified stdlib/testz/run_all_tests.csd

# Generate missing test files
./cursed-unified stdlib/testz/generate_tests.csd
```

## Module Structure

```
stdlib/testz/
├── mod.csd              # Core testing primitives
├── advanced.csd         # Advanced testing features
├── templates.csd        # Test templates and utilities
├── discovery.csd        # Test discovery and execution
├── README.md           # This documentation
└── test_testz.csd      # Tests for the testing framework
```

## Core API Reference

### Basic Assertions

```cursed
slay assert_true(condition lit) lit
slay assert_false(condition lit) lit
slay assert_eq_int(actual normie, expected normie) lit
slay assert_eq_string(actual tea, expected tea) lit
```

### Advanced Assertions

```cursed
slay assert_near(actual meal, expected meal, tolerance meal) lit
slay assert_throws(operation slay()) lit
slay assert_array_eq(actual []normie, expected []normie) lit
slay assert_memory_usage_under(threshold normie) lit
```

### Test Organization

```cursed
slay test_start(name tea) lit
slay test_group_start(group_name tea) lit
slay test_group_end() lit
slay print_test_summary() lit
```

### Benchmarking

```cursed
squad BenchmarkResult {
    spill name tea
    spill duration_ns normie
    spill iterations normie
    spill memory_used normie
    spill ops_per_sec meal
}

slay benchmark(name tea, operation slay()) BenchmarkResult
slay print_benchmark_summary() lit
```

### Property-Based Testing

```cursed
squad PropertyTestCase {
    spill name tea
    spill generator slay() tea          # Generate random test inputs
    spill property slay(tea) lit        # Property that should always hold
    spill iterations normie             # Number of test cases to generate
}

slay property_test(test_case PropertyTestCase) lit
```

## Test Templates

### Module Template

Use `create_module_test_template(module_name)` to generate comprehensive test suite:

```cursed
yeet "testz/templates"

sus test_content tea = create_module_test_template("my_module")
fr fr Generates complete test file with:
fr fr - Basic functionality tests
fr fr - Performance benchmarks  
fr fr - Error handling tests
fr fr - Property-based tests
```

### Collection Testing

```cursed
test_collection_properties("my_collection", 
    create_fn,  # slay() tea - creates empty collection
    add_fn,     # slay(tea, tea) - adds item to collection  
    get_fn      # slay(tea, normie) tea - gets item by index
)
```

### Math Function Testing

```cursed
sus test_cases [][]meal = [
    [0.0, 0.0],      # input, expected
    [1.0, 1.0],
    [3.14159, 3.14159]
]

test_math_function("my_math_func", my_math_func, test_cases)
```

### String Property Testing

```cursed
test_string_properties("my_string_func", my_string_func)
fr fr Tests properties like:
fr fr - Consistency (same input -> same output)
fr fr - Empty string handling
fr fr - Length preservation/modification patterns
```

## Test Discovery and Automation

### Discover All Tests

```cursed
yeet "testz/discovery"

sus discovery TestDiscoveryResult = discover_all_stdlib_tests()
vibez.spillf("Found {} test files for {} modules", 
             discovery.test_files.len(), discovery.modules_with_tests)
```

### Execute All Tests

```cursed
sus results []TestExecutionResult = execute_all_stdlib_tests()
bestie result in results {
    vibez.spillf("{}: {} passed, {} failed", 
                 result.module_name, result.passed, result.failed)
}
```

### Generate Missing Tests

```cursed
generate_missing_test_files()
fr fr Automatically creates test files for modules without tests
```

## Advanced Features

### Memory Testing

```cursed
fr fr Track memory usage during test
sus baseline normie = get_memory_usage()
perform_memory_intensive_operation()
assert_memory_usage_under(baseline + 1000000)  # Within 1MB
```

### Coverage Tracking

```cursed
fr fr Mark lines as covered during execution
mark_line_covered("module.csd", 42)
sus coverage meal = get_coverage_percentage()
print_coverage_report()
```

### Concurrency Testing

```cursed
test_concurrency_module("my_concurrent_module",
    spawn_func,    # slay(slay()) - spawn goroutine
    channel_func   # slay() tea - create channel
)
```

### Error Handling Testing

```cursed
test_error_handling_module("my_error_module",
    error_creation_func,  # slay(tea) tea - create error
    error_handling_func   # slay(tea) lit - handle error
)
```

## Test Execution Results

### TestDiscoveryResult

```cursed
squad TestDiscoveryResult {
    spill total_modules normie         # Total stdlib modules found
    spill modules_with_tests normie    # Modules that have test files
    spill missing_tests []tea          # Modules without tests
    spill test_files []tea             # All discovered test files
    spill coverage_percentage meal     # Test coverage percentage
}
```

### TestExecutionResult

```cursed
squad TestExecutionResult {
    spill module_name tea              # Name of tested module
    spill test_file tea                # Path to test file
    spill passed normie                # Number of passed tests
    spill failed normie                # Number of failed tests
    spill duration_ms normie           # Execution time in milliseconds
    spill success lit                  # Overall success (failed == 0)
}
```

## Best Practices

### 1. Test Organization

```cursed
fr fr Group related tests together
test_group_start("Core Functionality")
test_start("basic_operation")
test_start("edge_cases")
test_group_end()

test_group_start("Error Conditions")
test_start("invalid_input")
test_start("null_handling")
test_group_end()
```

### 2. Property-Based Testing

```cursed
fr fr Test fundamental properties
property_test(PropertyTestCase{
    name: "idempotent_operation",
    generator: slay() tea { damn generate_random_input() },
    property: slay(input tea) lit {
        sus result1 = operation(input)
        sus result2 = operation(result1)
        damn result1 == result2  # Property: operation is idempotent
    },
    iterations: 100
})
```

### 3. Benchmarking

```cursed
fr fr Always benchmark critical operations
benchmark("critical_operation", slay() {
    critical_operation(sample_input)
})

fr fr Compare different implementations
benchmark("algorithm_v1", slay() { algorithm_v1(input) })
benchmark("algorithm_v2", slay() { algorithm_v2(input) })
```

### 4. Error Testing

```cursed
fr fr Test both success and failure paths
test_start("success_path")
assert_true(operation_succeeds())

test_start("failure_path")
assert_throws(slay() {
    operation_that_should_fail()
})
```

## Stdlib Module Coverage Status

As of the latest test discovery run:

- **Total Stdlib Modules**: 380+
- **Modules with Tests**: ~330 (87% coverage)
- **Critical Modules**: All core modules have comprehensive tests
- **Missing Tests**: Primarily newer/experimental modules

### High-Priority Modules (100% Tested)
- ✅ `testz` - Testing framework itself
- ✅ `collections` - Data structures
- ✅ `string_simple` - String operations
- ✅ `mathz` - Mathematical functions
- ✅ `error_drip` - Error handling
- ✅ `concurrenz` - Concurrency primitives
- ✅ `crypto` - Cryptographic functions

### Modules Needing Test Enhancement
- 🔄 `networking` - Needs integration tests
- 🔄 `database` - Needs transaction tests
- 🔄 `parser` - Needs edge case coverage
- 🔄 `compiler_core` - Needs stress testing

## Integration with CURSED Compiler

The testing framework integrates seamlessly with both CURSED compiler implementations:

### Rust Implementation
```bash
cargo run --bin cursed test_file.csd
cargo run --bin cursed -- compile test_file.csd && ./test_file
```

### Zig Implementation (Primary)
```bash
./cursed-unified test_file.csd
./cursed-unified --compile test_file.csd && ./test_file
```

## Contributing

When adding new stdlib modules:

1. **Auto-generate test template**: Use `generate_test_file_for_module(module_name)`
2. **Implement module-specific tests**: Add tests for your module's unique functionality
3. **Add property tests**: Define properties that should always hold
4. **Include benchmarks**: Measure performance of critical operations
5. **Test error conditions**: Verify proper error handling

## Future Enhancements

- **Mutation Testing**: Automatically modify code to verify test quality
- **Fuzz Testing**: Generate random inputs to find edge cases
- **Integration Testing**: Test module interactions
- **Visual Test Reports**: HTML/web-based test result dashboards
- **Continuous Integration**: Automated testing on code changes
- **Performance Regression Detection**: Alert on performance degradation

## Support

For issues with the testing framework:

1. Check that all dependencies are imported correctly
2. Verify module paths in `yeet` statements
3. Run test discovery to identify missing test files
4. Use `print_test_summary()` to get detailed results
5. Check the CURSED compiler documentation for syntax updates

The CURSED Testing Framework is production-ready and used to validate the entire CURSED standard library with high confidence in code quality and correctness.
