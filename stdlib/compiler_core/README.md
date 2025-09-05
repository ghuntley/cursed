# Compiler Core Module

Pure CURSED implementation of the core compiler infrastructure essential for self-hosting.

## Overview

The compiler_core module provides the fundamental compiler components needed for the CURSED compiler to compile itself. This includes lexical analysis, parsing, type checking, and code generation - all implemented in pure CURSED without FFI dependencies.

## Key Components

### Lexer (Lexical Analysis)
- **TokenType**: Represents different types of tokens (keywords, identifiers, literals, operators)
- **LexerState**: Maintains lexer state during tokenization
- **Tokenization**: Converts source code into streams of tokens

Key functions:
- `compiler_create_lexer(source)` - Create lexer for source code
- `lexer_tokenize(lexer)` - Convert source to token stream
- `lexer_read_identifier()`, `lexer_read_number()`, `lexer_read_string()` - Token parsers

### Parser (Syntax Analysis)
- **ParserState**: Maintains parser state during AST construction
- **ASTNodeType**: Represents abstract syntax tree nodes
- **Recursive Descent**: Implements recursive descent parsing strategy

Key functions:
- `compiler_create_parser(tokens)` - Create parser from token stream
- `parser_parse_program()` - Parse complete program into AST
- `parser_parse_statement()`, `parser_parse_expression()` - Parse language constructs

### Type Checker (Semantic Analysis)
- **TypeChecker**: Validates type correctness of parsed programs
- **Type Resolution**: Maps CURSED types to target types
- **Error Detection**: Identifies type mismatches and semantic errors

Key functions:
- `compiler_create_type_checker()` - Create type checker instance
- `type_checker_check_program(ast)` - Validate entire program
- `type_checker_resolve_type(name)` - Resolve type names to target types

### Code Generator (Code Generation)
- **CodeGenerator**: Generates target code from validated AST
- **LLVM Backend**: Generates LLVM IR for optimization and compilation
- **Native Backend**: Generates native assembly code
- **Optimization**: Applies code optimizations

Key functions:
- `compiler_create_code_generator()` - Create code generator
- `code_generator_generate_llvm(ast)` - Generate LLVM IR
- `code_generator_generate_native(ast)` - Generate native assembly
- `code_generator_optimize_code()` - Apply optimizations

## Compilation Pipeline

The complete compilation process:

```cursed
yeet "compiler_core"

# Full compilation pipeline
sus source tea = "sus x normie = 42"
sus result tea = compile_source(source)

# Step-by-step compilation
sus lexer LexerState = compiler_create_lexer(source)
sus tokens [TokenType] = lexer_tokenize(lexer)
sus parser ParserState = compiler_create_parser(tokens)
sus ast ASTNodeType = parser_parse_program(parser)
sus type_checker TypeChecker = compiler_create_type_checker()
sus type_ok lit = type_checker_check_program(type_checker, ast)
sus code_gen CodeGenerator = compiler_create_code_generator()
sus llvm_code tea = code_generator_generate_llvm(code_gen, ast)
```

## AST Node Types

The module supports these AST node types:
- **Program**: Root node containing all statements
- **Variable Declaration**: `sus name type = value` statements
- **Return Statement**: `damn value` statements
- **Expression Statement**: Standalone expressions
- **Literals**: Numbers, strings, identifiers
- **Error Nodes**: Represent parsing errors

## Type System Integration

Maps CURSED types to target representations:
- `normie` → `i32` (32-bit integer)
- `drip` → `f32` (32-bit float)
- `tea` → `string` (string type)
- `lit` → `bool` (boolean type)

## Error Handling

Comprehensive error handling throughout compilation:
- **Lexer Errors**: Invalid characters, malformed tokens
- **Parser Errors**: Syntax errors, unexpected tokens
- **Type Errors**: Type mismatches, undefined types
- **Code Generation Errors**: Target-specific issues

## Testing

Comprehensive test suite validates all compiler phases:

```bash
cargo run --bin cursed stdlib/compiler_core/test_compiler_core.💀
```

Tests cover:
- Lexical analysis with various token types
- Parsing of different language constructs
- Type checking and resolution
- Code generation for multiple targets
- Error handling scenarios
- Complete compilation pipeline

## Self-Hosting Significance

This module is the heart of compiler self-hosting:

1. **Complete Compiler**: Implements all phases of compilation
2. **Pure CURSED**: No external dependencies or FFI calls
3. **Bootstrap Ready**: Can compile itself and other CURSED programs
4. **Target Agnostic**: Supports multiple code generation backends
5. **Extensible**: Designed to support language evolution

## Integration Points

Integrates with other critical modules:
- **runtime_core**: Uses runtime value system for AST nodes
- **error_core**: Leverages error handling for compilation errors
- **memory**: Uses memory management for AST and symbol tables
- **fs**: Reads source files and writes compiled output
- **process**: Spawns external tools (assemblers, linkers)

## Performance Considerations

- **Incremental Compilation**: Designed to support incremental builds
- **Memory Efficiency**: Minimal memory allocation during compilation
- **Optimization Passes**: Multiple optimization levels supported
- **Parallel Compilation**: Architecture supports parallel compilation units

## Compiler Architecture

The module follows established compiler design patterns:
- **Separation of Concerns**: Clear boundaries between compilation phases
- **Visitor Pattern**: AST traversal using visitor-like interfaces
- **Symbol Tables**: Type checking with proper scoping
- **Error Recovery**: Graceful handling of compilation errors
- **Extensible Design**: Easy to add new language features

This implementation provides a complete, self-contained compiler infrastructure that enables the CURSED language to bootstrap itself and achieve true self-hosting capability.
