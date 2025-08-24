# Contributing to CURSED

## Getting Started

### Prerequisites
- Zig 0.12.0 or later
- LLVM 16+ (for compilation mode)
- Valgrind (for memory safety testing)
- Git

### Setup Development Environment
```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the compiler
zig build

# Verify installation
./zig-out/bin/cursed-zig --version
```

## Development Workflow

### 1. Making Changes

Before making changes, always:
```bash
# Run the full test suite
./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd

# Memory safety validation
valgrind --leak-check=full ./zig-out/bin/cursed-zig test.csd
```

### 2. Testing Your Changes

#### Unit Tests
```bash
# Test specific components
zig test src-zig/lexer.zig
zig test src-zig/parser.zig
zig test src-zig/type_system_runtime.zig
```

#### Integration Tests
```bash
# Test interpreter mode
./zig-out/bin/cursed-zig your_test_file.csd

# Test compilation mode
./zig-out/bin/cursed-zig --compile your_test_file.csd
```

#### Memory Safety (REQUIRED)
```bash
# All contributions MUST pass memory safety testing
valgrind --leak-check=full --error-exitcode=1 \
  ./zig-out/bin/cursed-zig your_test_file.csd
```

### 3. Code Style

#### Formatting
```bash
# Format your code before committing
./zig-out/bin/cursed-fmt src-zig/your_file.zig
```

#### Naming Conventions
- **Functions**: `snake_case`
- **Types**: `PascalCase`  
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `lowercase`

#### Comments
```zig
// Use // for single-line comments
/* Use /* */ for multi-line comments */

// Document public APIs
/// Parse a CURSED expression from the given token stream
pub fn parseExpression(tokens: []Token) ParseResult {
    // Implementation details...
}
```

## Contribution Areas

### 1. Standard Library Modules

Add new modules or improve existing ones:
```bash
# Create new module
mkdir stdlib/your_module
touch stdlib/your_module/mod.csd
touch stdlib/your_module/test_your_module.csd
touch stdlib/your_module/README.md
```

#### Module Requirements
- **100% Pure CURSED**: No FFI dependencies
- **Comprehensive Tests**: Minimum 90% code coverage
- **Documentation**: Complete API documentation
- **Memory Safe**: Zero memory leaks
- **Production Ready**: Real implementations, no placeholders

### 2. Compiler Features

#### Parser Enhancements
- New syntax features
- Error recovery improvements
- Performance optimizations

#### Type System
- Generic type improvements
- Interface enhancements
- Type inference refinements

#### Code Generation
- LLVM IR optimizations
- Cross-compilation improvements
- Debug information enhancements

### 3. Developer Tools

#### Language Server (LSP)
- Code completion improvements
- Diagnostics enhancements
- Refactoring support

#### Formatter
- Style configuration options
- Performance improvements
- Edge case handling

#### Linter
- New lint rules
- Performance optimizations
- Configuration improvements

## Testing Requirements

### Mandatory Tests

All contributions MUST include:

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test complete workflows
3. **Memory Safety**: Valgrind validation
4. **Performance Tests**: No significant regressions

### Example Test Structure
```cursed
// stdlib/your_module/test_your_module.csd
yeet "testz"
yeet "your_module"

slay test_basic_functionality() {
    sus result = your_function(42)
    testz.assert_eq_int(result, expected_value)
}

slay test_error_handling() {
    sus error_result = your_function(-1)
    testz.assert_error(error_result, "invalid input")
}

slay main() {
    testz.start_suite("YourModule Tests")
    test_basic_functionality()
    test_error_handling()
    testz.print_summary()
}
```

## Documentation Requirements

### API Documentation
Document all public APIs:
```cursed
fr fr Calculate fibonacci number recursively
fr fr @param n - The number to calculate fibonacci for (must be >= 0)
fr fr @return The nth fibonacci number
fr fr @throws "invalid input" when n < 0
slay fibonacci(n drip) drip yikes<tea> {
    ready (n < 0) {
        yikes "invalid input: n must be >= 0"
    }
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n-1) + fibonacci(n-2)
}
```

### Module Documentation
Each module needs a comprehensive README.md:
```markdown
# Module Name

## Overview
Brief description of the module's purpose.

## Usage
```cursed
yeet "module_name"
sus result = module_function(params)
```

## API Reference
- `function_name(params) -> return_type`: Description
- `Type`: Description of type

## Examples
Real-world usage examples.

## Performance
Performance characteristics and benchmarks.
```

## Pull Request Process

### 1. Before Submitting

Run the complete validation suite:
```bash
#!/bin/bash
# pre_submit_validation.sh

echo "Building CURSED..."
zig build || exit 1

echo "Running unit tests..."
zig test src-zig/lexer.zig || exit 1
zig test src-zig/parser.zig || exit 1

echo "Running integration tests..."
./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd || exit 1

echo "Memory safety validation..."
valgrind --leak-check=full --error-exitcode=1 \
  ./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd || exit 1

echo "All validations passed!"
```

### 2. Pull Request Guidelines

#### Title Format
- `feat: Add new standard library module xyz`
- `fix: Resolve memory leak in parser`
- `docs: Update API documentation for networking`
- `perf: Optimize compilation performance`

#### Description Template
```
## Summary
Brief description of changes

## Changes Made
- List of specific changes
- Include rationale for major decisions

## Testing
- Unit tests: [X] Passed
- Integration tests: [X] Passed  
- Memory safety: [X] Passed
- Performance tests: [X] No regression

## Documentation
- [X] API documentation updated
- [X] Examples added
- [X] README updated

## Breaking Changes
- None / List any breaking changes
```

### 3. Review Process

All pull requests require:
1. **Code Review**: At least one maintainer approval
2. **Automated Testing**: All CI checks must pass
3. **Memory Safety**: Valgrind validation required
4. **Documentation**: Complete documentation required

## Community Guidelines

### Code of Conduct
- Be respectful and professional
- Focus on constructive feedback
- Welcome newcomers and help them succeed
- Maintain high quality standards

### Communication
- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: General questions and community chat
- **Pull Requests**: Code contributions and reviews

### Support
- Check existing documentation first
- Search existing issues before creating new ones
- Provide minimal reproducible examples
- Include system information for bug reports

## Advanced Development

### Memory Management
```zig
// Use arena allocators for temporary data
var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
defer arena.deinit();
const allocator = arena.allocator();

// Always validate memory safety
// valgrind --leak-check=full your_program
```

### Concurrency
```zig
// Follow established patterns for goroutine implementation
// Test with race detection enabled
// Validate channel operations don't deadlock
```

### Performance
```zig
// Profile before optimizing
// Benchmark performance changes
// Consider memory layout impact
// Avoid premature optimization
```

## Recognition

Contributors are recognized through:
- GitHub contributor statistics
- Release notes acknowledgments
- Community highlights
- Maintainer recognition program

Thank you for contributing to CURSED! 🚀
