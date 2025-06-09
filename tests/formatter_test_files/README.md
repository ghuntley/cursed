# CURSED Formatter Test Files

This directory contains test files for the CURSED programming language formatter. These files are used to validate the formatter's behavior across various scenarios and language constructs.

## File Organization

### Before/After Pairs
These files demonstrate expected formatting transformations:

- `*_before.csd` - Unformatted source code
- `*_after.csd` - Expected formatted output

### Test Categories

#### Basic Formatting Tests
- **simple_function_before/after.csd** - Basic function formatting with parameters and return types
- **complex_program_before/after.csd** - Complete program with structs, interfaces, and methods

#### Advanced Language Features
- **generics_before/after.csd** - Generic functions and structs with type parameters
- **edge_cases_before/after.csd** - Complex nested structures, error handling, channels

#### Special Cases
- **comments_before/after.csd** - Comment formatting and preservation
- **real_world_example.csd** - Realistic HTTP server implementation
- **syntax_errors.csd** - File with various syntax errors for error handling tests

#### Configuration
- **sample_config.toml** - Example configuration file with all available options

## Test File Conventions

### Naming Convention
- Test files use the `.csd` extension (CURSED source)
- Before/after pairs follow the pattern `{description}_{before|after}.csd`
- Error test files end with `_errors.csd`
- Real-world examples use descriptive names

### Content Guidelines
- Each test file focuses on specific formatting scenarios
- Files include comments explaining the test purpose
- Complex examples demonstrate multiple language features
- Error files contain intentional syntax issues

## Language Features Tested

### Function Declarations
```cursed
// Before
slay add(x normie,y normie)normie{yolo x+y}

// After
slay add(x normie, y normie) normie {
    yolo x + y
}
```

### Control Flow
```cursed
// Before
lowkey x>0{yolo "positive"}highkey{yolo "zero"}

// After
lowkey x > 0 {
    yolo "positive"
} highkey {
    yolo "zero"
}
```

### Struct and Interface Declarations
```cursed
// Before
squad Person{name sip age normie}

// After
squad Person {
    name sip
    age normie
}
```

### Generic Types
```cursed
// Before
slay process[T](items[]T)T{yolo items[0]}

// After
slay process[T](items []T) T {
    yolo items[0]
}
```

## Configuration Options Tested

### Indentation
- Space vs tab indentation
- Configurable indent size (2, 4, 6, 8 spaces)
- Tab width settings

### Brace Styles
- **Same Line**: `function() {`
- **Next Line**: `function()\n    {`
- **Next Line Unindented**: `function()\n{`

### Spacing
- Operators: `x + y` vs `x+y`
- Commas: `func(a, b)` vs `func(a,b)`
- Comments: alignment and formatting

### Line Management
- Maximum line width enforcement
- Empty line preservation and limits
- Trailing whitespace removal

## Running Formatter Tests

### Individual Test Suites
```bash
# Unit tests
cargo test --test formatter_unit_test

# Integration tests
cargo test --test formatter_integration_test

# CLI tests
cargo test --test formatter_cli_test

# Golden file tests
cargo test --test formatter_golden_test

# Configuration tests
cargo test --test formatter_config_test
```

### All Formatter Tests
```bash
# Run all formatter tests
./tests/run_formatter_tests.sh

# Verbose output
./tests/run_formatter_tests.sh --verbose

# Specific test suite
./tests/run_formatter_tests.sh --test unit

# Generate coverage report
./tests/run_formatter_tests.sh --report
```

## Expected Test Coverage

### Functionality Coverage
- ✅ All CURSED language constructs
- ✅ All formatting configuration options
- ✅ Error handling and edge cases
- ✅ CLI tool functionality
- ✅ Configuration file loading

### Quality Assurance
- ✅ Formatting stability (idempotency)
- ✅ Semantic preservation
- ✅ Performance with large files
- ✅ Memory usage optimization
- ✅ Unicode and special character handling

### Error Scenarios
- ✅ Malformed syntax handling
- ✅ Invalid configuration detection
- ✅ File system errors
- ✅ Permission issues
- ✅ Binary file detection

## Adding New Test Files

### Creating Before/After Pairs
1. Create unformatted source in `{name}_before.csd`
2. Create expected formatted output in `{name}_after.csd`
3. Add test case to `formatter_golden_test.rs`
4. Run tests to validate expectations

### Testing New Features
1. Add feature-specific test files
2. Update golden tests with new expectations
3. Add configuration tests if new options are introduced
4. Update CLI tests for new command-line options

### Regression Testing
1. Add test files that reproduce reported issues
2. Verify fixes with before/after comparisons
3. Include edge cases that might cause regressions
4. Document the specific issue being tested

## Test File Maintenance

### Regular Updates
- Update test files when language syntax changes
- Refresh golden files when formatter behavior improves
- Add new edge cases as they're discovered
- Remove obsolete test cases for deprecated features

### Performance Considerations
- Keep test files reasonably sized for fast execution
- Use separate large file tests for performance validation
- Balance comprehensive coverage with test execution time
- Profile test execution to identify bottlenecks

## Integration with CI/CD

### Automated Testing
- All formatter tests run on every commit
- Golden file comparisons detect unintended changes
- Performance regression detection
- Cross-platform compatibility validation

### Release Validation
- Comprehensive test suite must pass before release
- Configuration compatibility across versions
- Backward compatibility with existing formatted code
- Documentation updates for new features
