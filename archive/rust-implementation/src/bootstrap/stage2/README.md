# CURSED Stage 2 Self-Hosting Compiler

This directory contains the Stage 2 implementation of the CURSED compiler - a complete compiler written **in the CURSED language itself**. This is the critical component that enables true self-hosting capability.

## Overview

The Stage 2 compiler is implemented entirely in CURSED syntax and provides:

- **Lexical Analysis**: Tokenizes CURSED source code
- **Parsing**: Builds Abstract Syntax Trees (AST) from tokens  
- **Type Checking**: Validates semantic correctness and type safety
- **Code Generation**: Generates LLVM IR from validated AST
- **Error Handling**: Comprehensive error reporting with source location information

## Architecture

### Core Modules

- **`main.csd`** - Main compiler entry point and command-line interface
- **`lexer.csd`** - Lexical analysis (source → tokens)
- **`parser.csd`** - Recursive descent parser (tokens → AST)
- **`type_checker.csd`** - Semantic analysis and type validation
- **`codegen.csd`** - LLVM IR code generation
- **`error.csd`** - Error handling and reporting system

### Compilation Pipeline

```
Source Code → Lexer → Parser → Type Checker → Code Generator → LLVM IR
```

## Language Features Supported

The Stage 2 compiler implements a comprehensive subset of CURSED including:

### Core Language Constructs
- Variables: `sus` (mutable), `facts` (immutable)
- Functions: `slay function_name(params) -> return_type { ... }`
- Control Flow: `lowkey`/`highkey` (if/else), `periodt` (while), `bestie` (for)
- Return statements: `yolo value`

### Data Types
- `normie` (integers)
- `tea` (strings) 
- `cap` (booleans)
- `void` (no return value)

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`

### Language Constructs
- Block statements with proper scoping
- Expression statements
- Binary and unary expressions
- Function calls and parameters
- Type annotations and inference

## Usage

The Stage 2 compiler is invoked through the bootstrap verification system:

```bash
# Run bootstrap verification (includes Stage 2 compilation)
./run_bootstrap_verification.sh

# Test Stage 2 compiler directly with Stage 1
cursed compile src/bootstrap/stage2/main.csd -o stage2_compiler
```

### Command Line Interface

```bash
cursed compile <input.csd> [OPTIONS]

OPTIONS:
    -o <file>     Specify output file
    -O0           No optimization (default)  
    -O2           Enable optimizations
    --debug       Enable debug information
    --verbose     Verbose output
    --help        Show help message
```

## Implementation Details

### AST Design
The parser generates a typed AST with nodes for:
- Program (root)
- Function declarations with parameters and return types
- Variable declarations (mutable/immutable)
- Control flow statements (if, while, return)
- Expressions (binary, unary, literals, identifiers)

### Type System
- Strong static typing with compile-time checking
- Type inference for `auto` declarations
- Symbol table with lexical scoping
- Type compatibility checking for operations
- Function signature validation

### Code Generation
- Generates LLVM IR compatible with the Stage 1 compiler
- Supports basic blocks for control flow
- Register allocation through SSA form
- Function calling conventions
- Memory management for local variables

### Error Handling
- Comprehensive error reporting with source locations
- Multiple error severity levels (info, warning, error, fatal)
- Helpful error messages with suggestions
- Graceful error recovery during parsing

## Testing

Test programs are included to validate the Stage 2 compiler:

- **`test_simple.csd`** - Basic functionality test with arithmetic, control flow, and functions

## Bootstrap Process Integration

The Stage 2 compiler integrates with the bootstrap verification system:

1. **Stage 1** (Rust-based compiler) compiles the Stage 2 source files
2. **Stage 2** (CURSED-based compiler) can then compile CURSED programs
3. **Functional equivalence** testing ensures both stages produce identical results
4. **Convergence testing** validates the compiler can compile itself

## Implementation Status

✅ **Lexical Analysis** - Complete token recognition for all CURSED constructs
✅ **Parsing** - Recursive descent parser with error recovery  
✅ **AST Generation** - Typed AST nodes with proper relationships
✅ **Type Checking** - Symbol table management and type validation
✅ **Code Generation** - LLVM IR generation for core language features
✅ **Error Handling** - Comprehensive error reporting system
✅ **CLI Interface** - Command-line argument parsing and help

## Future Enhancements

- **Advanced Features**: Generics, interfaces, structs, channels
- **Optimization Passes**: Dead code elimination, constant folding
- **Debugging Support**: Source maps, debug information generation
- **Standard Library**: Integration with CURSED standard library modules
- **Language Server**: LSP support for IDE integration

## Self-Hosting Significance

This Stage 2 implementation represents a critical milestone in CURSED's development:

- **Proves Language Completeness**: CURSED is expressive enough to implement its own compiler
- **Enables True Self-Hosting**: The language can evolve using itself
- **Validates Design Decisions**: Real-world usage validates language design
- **Bootstrap Independence**: Reduces dependency on the Rust-based Stage 1 compiler

The successful implementation of Stage 2 demonstrates that CURSED has achieved the fundamental goal of being a self-hosting programming language capable of supporting its own development and evolution.
