# CURSED Compiler Regression Test Suite

This directory contains **1000+ comprehensive regression tests** for the CURSED compiler, designed to catch regressions early and ensure parser/AST functionality remains stable.

## Overview

The regression test suite provides:

- **1000+ test cases** covering all major language features
- **Parser round-trip testing** to ensure AST serialization works correctly  
- **Memory safety validation** with valgrind integration
- **Error condition testing** to ensure graceful failure handling
- **Automated test discovery** and execution
- **CI/CD integration** for continuous testing

## Test Categories

### 1. Parser Tests (`parser/`)
**409 tests** covering core parsing functionality:

- **Basic tests** (`basic/`): Variables, functions, expressions, arrays
- **Advanced tests** (`advanced/`): Complex expressions, nested calls, structs, interfaces
- **Edge cases** (`edge_cases/`): Large numbers, deep nesting, performance, recursion, patterns, concurrency

### 2. Standard Library Tests (`stdlib/`)
**180 tests** for module imports and function calls:

- **Basic tests** (`basic/`): mathz, stringz, arrayz, cryptz, jsonz, httpz, filez modules
- **Multiple imports**: Testing interaction between modules
- **Function validation**: Ensuring stdlib functions work correctly

### 3. Memory Safety Tests (`memory/`)  
**104 tests** for memory leak detection:

- **Variable lifecycle**: Scope-based memory management
- **String operations**: String concatenation and manipulation
- **Array operations**: Array creation, slicing, manipulation
- **Recursive functions**: Stack and heap management
- **Stress tests**: High-volume memory operations

### 4. Error Handling Tests (`errors/`)
**104 tests** for graceful error handling:

- **Syntax errors**: Missing tokens, malformed expressions
- **Type errors**: Type mismatches, invalid operations
- **Undefined variables**: Reference errors
- **Import errors**: Module loading failures
- **Recovery tests**: Error handling without crashes

### 5. Round-trip Tests (`roundtrip/`)
**62 tests** for parser consistency:

- **Complete programs**: Full CURSED programs with all features
- **Complex expressions**: Multi-level nested expressions
- **Integration tests**: Cross-module functionality

## Running Tests

### Quick Test Run
```bash
# Run all regression tests
./run_regression_tests_simple.sh

# Run with verbose output
./run_regression_tests_simple.sh --verbose

# Run specific category
find tests/regression/parser -name "*.csd" -exec ./zig-out/bin/cursed-zig {} \;
```

### Memory Safety Testing
```bash
# Run with valgrind memory checking (requires valgrind)
./run_regression_tests_simple.sh

# Manual valgrind test
valgrind --leak-check=full ./zig-out/bin/cursed-zig tests/regression/memory/basic/lifecycle_1.csd
```

### Build System Integration
```bash
# Run through build system (when working)
zig build test-regression

# Run memory-specific tests
zig build test-memory-regression
```

## Test File Format

Each test file follows CURSED syntax:

```cursed
// Test description and purpose
yeet "required_modules"  // If needed

// Test code
sus variable drip = 42
slay function() {
    vibez.spill("Test output")
}

// Expected behavior depends on test category
```

## Expected Behavior by Category

- **Parser tests**: Should parse successfully and execute
- **Stdlib tests**: Should import modules and call functions correctly
- **Memory tests**: Should execute without memory leaks
- **Error tests**: Should fail gracefully without crashing
- **Round-trip tests**: Should parse, serialize, and re-parse consistently

## Test Naming Convention

Tests follow a consistent naming pattern:
- `category/subcategory/testtype_number.csd`
- Example: `parser/basic/var_test_1.csd`

## Adding New Tests

1. **Create test file** in appropriate category directory
2. **Follow naming convention** for consistency
3. **Add test description** comment at top
4. **Test specific functionality** with clear expected behavior
5. **Run test suite** to ensure it integrates correctly

## Continuous Integration

The regression test suite is designed for CI/CD integration:

- **Fast execution**: Most tests complete in under 1 second
- **Clear exit codes**: 0 for success, 1 for failure
- **Detailed reporting**: Machine-readable output available
- **Memory leak detection**: Automatic valgrind integration
- **Parallel execution**: Support for concurrent test runs

## Performance Characteristics

- **Total tests**: 1009
- **Execution time**: ~5-10 minutes for full suite
- **Memory usage**: Minimal (arena allocators used)
- **Disk usage**: ~2MB for all test files

## Troubleshooting

### Common Issues

1. **Build failures**: Ensure `zig build` completes successfully
2. **Missing binary**: Check for `./zig-out/bin/cursed-zig` executable
3. **Permission errors**: Ensure test script is executable (`chmod +x`)
4. **Valgrind not found**: Install valgrind or use `--no-valgrind` flag

### Debug Mode

Run individual tests for debugging:
```bash
# Single test with output
./zig-out/bin/cursed-zig tests/regression/parser/basic/var_test_1.csd

# With debug information
./zig-out/bin/cursed-zig --verbose tests/regression/parser/basic/var_test_1.csd
```

## Maintenance

The test suite requires minimal maintenance:

- **Add tests** for new language features
- **Update expected behavior** when language evolves
- **Remove obsolete tests** when features are deprecated
- **Monitor execution time** and optimize slow tests

## Integration with Development Workflow

Recommended workflow:

1. **Before commits**: Run full regression suite
2. **During development**: Run relevant category tests
3. **After feature changes**: Add new tests for coverage
4. **Before releases**: Full suite with memory checking

This comprehensive test suite ensures the CURSED compiler maintains stability and catches regressions early in the development process.
