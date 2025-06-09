# CURSED Stage 2 Self-Hosting Compiler

## Overview

The CURSED Stage 2 compiler is a self-hosting implementation of the CURSED language compiler written entirely in CURSED itself. This represents a major milestone in the language's development, demonstrating that CURSED is mature enough to compile itself.

## Architecture

### Compilation Stages

1. **Stage 1 (Rust)**: The bootstrap compiler written in Rust that can compile CURSED source code
2. **Stage 2 (CURSED)**: The self-hosting compiler written in CURSED, compiled by Stage 1
3. **Stage 3+ (Iterative)**: Further iterations where Stage 2 compiles itself

### Component Structure

The Stage 2 compiler consists of several key components, all written in CURSED:

```
src/bootstrap/stage2/
├── main.csd      # Main entry point and compiler coordinator
├── lexer.csd     # Lexical analysis (tokenization)
├── ast.csd       # Abstract Syntax Tree definitions
├── parser.csd    # Syntax analysis (parsing)
└── codegen.csd   # LLVM IR code generation
```

## Features

### Minimal Subset Support

The Stage 2 compiler implements a carefully chosen minimal subset of CURSED features:

**Data Types:**
- `int` - 32-bit signed integers
- `bool` - Boolean values (true/false)
- `string` - String literals
- `[]Type` - Arrays of elements

**Control Flow:**
- `if/else` statements
- `for` loops with init/condition/update
- `while` loops
- Function calls and returns

**Declarations:**
- Function definitions with parameters and return types
- Variable declarations with optional type annotations
- Struct type definitions
- Import statements

**Expressions:**
- Arithmetic operators: `+`, `-`, `*`, `/`
- Comparison operators: `==`, `!=`, `<`, `>`
- Logical operators: `&&`, `||`, `!`
- Array indexing and literals
- Function calls
- Variable assignment

### Code Generation

The Stage 2 compiler generates LLVM IR that is compatible with the Stage 1 output:

- **LLVM IR Generation**: Produces `.ll` files with equivalent semantics
- **Memory Management**: Basic stack allocation for local variables
- **Function Calling**: Standard calling conventions
- **Type System**: Maps CURSED types to appropriate LLVM types
- **Control Flow**: Generates proper basic blocks and branching

## Usage

### Building Stage 2

1. Build the Stage 2 compiler:
```bash
make stage2-build
# or
CURSED_BUILD_STAGE2=1 cargo build
```

2. Check if Stage 2 is available:
```bash
cursed stage2 status
```

### Compiling with Stage 2

1. Compile a CURSED program using Stage 2:
```bash
cursed stage2 compile hello.csd -o hello
```

2. Enable self-hosting mode (automatic Stage 2 usage):
```bash
cursed stage2 self-host on
```

3. Test Stage 2 functionality:
```bash
cursed stage2 test
```

### Command Reference

```bash
# Check Stage 2 availability and status
cursed stage2 status

# Build Stage 2 compiler from CURSED source
cursed stage2 build

# Compile using Stage 2 compiler
cursed stage2 compile <source.csd> [-o output]

# Enable/disable self-hosting mode
cursed stage2 self-host on|off

# Show Stage 2 compiler version
cursed stage2 version

# Test Stage 2 compiler functionality
cursed stage2 test

# Show detailed help
cursed stage2
```

## Implementation Details

### Lexer (lexer.csd)

The Stage 2 lexer implements tokenization of CURSED source code:

- **Token Types**: Minimal set covering keywords, operators, literals, and delimiters
- **String Processing**: Character-by-character scanning with proper state management
- **Error Handling**: Tracks line/column positions for meaningful error messages
- **Unicode Support**: Basic support for ASCII characters

### Parser (parser.csd)

The Stage 2 parser builds an Abstract Syntax Tree from tokens:

- **Recursive Descent**: Hand-written recursive descent parser
- **Operator Precedence**: Proper precedence handling for expressions
- **Error Recovery**: Basic error recovery to continue parsing after syntax errors
- **AST Construction**: Builds type-safe AST nodes defined in ast.csd

### Code Generator (codegen.csd)

The Stage 2 code generator produces LLVM IR:

- **IR Emission**: Direct LLVM IR text generation
- **Type Mapping**: CURSED types mapped to LLVM equivalents
- **Register Allocation**: Virtual register naming and management
- **Control Flow**: Proper basic block generation for conditionals and loops
- **Function Calls**: Standard calling convention implementation

### Limitations

Current limitations of the Stage 2 compiler:

1. **Subset Language**: Only supports the minimal subset of CURSED features
2. **Standard Library**: Limited standard library support
3. **Error Messages**: Basic error reporting (less detailed than Stage 1)
4. **Optimization**: No optimization passes (relies on LLVM/clang backend)
5. **Debugging**: Limited debugging information generation
6. **Memory Management**: No garbage collection (uses stack allocation only)

## Testing

### Test Suite

The Stage 2 compiler includes comprehensive testing:

```bash
# Run Stage 2 specific tests
make stage2-test

# Test with example program
cursed stage2 compile examples/stage2_test.csd -o test_output
./test_output
```

### Example Programs

Test programs for Stage 2 validation:

1. **examples/stage2_test.csd**: Comprehensive test covering all supported features
2. **Basic arithmetic**: Simple mathematical operations
3. **Control flow**: If statements and loops
4. **Functions**: Function definitions and calls
5. **Arrays**: Array creation and indexing

### Verification

Stage 2 correctness is verified by:

1. **Functional Equivalence**: Same programs produce same results as Stage 1
2. **Self-Compilation**: Stage 2 can compile its own source code
3. **Iterative Compilation**: Multiple bootstrap cycles converge to identical output
4. **Test Coverage**: Comprehensive test suite covering all language features

## Self-Hosting Verification

The ultimate test of Stage 2 is its ability to compile itself:

```bash
# Stage 2 compiles its own source
cursed stage2 compile src/bootstrap/stage2/main.csd -o cursed-stage3

# Stage 3 should be functionally identical to Stage 2
./cursed-stage3 src/bootstrap/stage2/main.csd -o cursed-stage4

# Verify binary equivalence
cmp cursed-stage3 cursed-stage4
```

## Future Enhancements

Planned improvements for Stage 2:

1. **Expanded Language Support**: Gradually add more CURSED features
2. **Better Error Messages**: Improve error reporting and recovery
3. **Optimization**: Add basic optimization passes
4. **Standard Library**: Expand available library functions
5. **Debugging Support**: Generate DWARF debugging information
6. **Memory Management**: Integrate garbage collection
7. **Module System**: Support for multi-file compilation
8. **Package Management**: Integration with CURSED package system

## Integration with Build System

The Stage 2 compiler integrates seamlessly with the existing build infrastructure:

### Makefile Targets

- `make stage2-build`: Build Stage 2 compiler
- `make stage2-test`: Test Stage 2 functionality
- `make stage2-status`: Check Stage 2 availability
- `make stage2-enable`: Enable self-hosting mode
- `make stage2-compile FILE=source.csd OUTPUT=binary`: Compile with Stage 2

### Environment Variables

- `CURSED_USE_STAGE2=1`: Use Stage 2 compiler when available
- `CURSED_BUILD_STAGE2=1`: Build Stage 2 during cargo build
- `CURSED_SELF_HOSTING=1`: Enable self-hosting mode

### CI/CD Integration

The Stage 2 compiler is designed for CI/CD integration:

1. **Automatic Building**: Stage 2 can be built automatically in CI
2. **Testing**: Comprehensive test suite validates functionality
3. **Verification**: Self-compilation tests ensure correctness
4. **Performance**: Benchmarking compares Stage 1 and Stage 2 performance

## Troubleshooting

### Common Issues

1. **Stage 2 Not Available**: Run `make stage2-build` to build Stage 2
2. **Compilation Errors**: Check that source uses only minimal subset features
3. **Runtime Errors**: Verify LLVM and clang are properly installed
4. **Performance Issues**: Stage 2 may be slower than Stage 1 for large programs

### Debugging

Enable verbose output for troubleshooting:

```bash
cursed stage2 compile program.csd -o output --verbose
```

Check intermediate files:

```bash
# LLVM IR file is preserved for inspection
cat output.ll
```

Verify Stage 2 installation:

```bash
cursed stage2 status
cursed stage2 version
cursed stage2 test
```

## Conclusion

The CURSED Stage 2 self-hosting compiler represents a significant achievement in language implementation. It demonstrates that CURSED is mature enough to compile itself while maintaining compatibility with the original Rust implementation. This foundation enables future development of more advanced compiler features and optimizations.

The Stage 2 compiler serves as both a practical tool for CURSED development and a proof of concept for the language's self-hosting capabilities. As the minimal subset expands and more features are added, Stage 2 will eventually become the primary CURSED compiler, with Stage 1 serving only as a bootstrap.
