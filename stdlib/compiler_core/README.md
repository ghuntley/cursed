# Compiler Core Infrastructure Module

## Overview

The `compiler_core` module provides comprehensive compiler infrastructure essential for self-hosting the CURSED compiler. This module implements all the fundamental components needed for a complete compilation pipeline.

## Components

### 1. Lexical Analysis
- **Token definitions**: Identifier, number, string, keyword, operator, delimiter tokens
- **Lexer state management**: Stateful tokenization with lookahead
- **Tokenization**: Convert source code into token streams
- **Token operations**: Peek, advance, type checking, value extraction

### 2. AST (Abstract Syntax Tree) Operations  
- **AST node types**: Program, function, variable, expression, statement, literal, binary/unary operators
- **AST construction**: Create nodes, manage parent-child relationships
- **AST traversal**: Visitor pattern implementation for tree walking
- **AST utilities**: Type queries, value extraction, pretty printing

### 3. Parser Infrastructure
- **Recursive descent parser**: Top-down parsing with precedence handling
- **Grammar productions**: Program, function, variable, expression, statement parsing
- **Error recovery**: Graceful handling of syntax errors
- **Token consumption**: Controlled token stream processing

### 4. Type Checking System
- **Type definitions**: Integer, float, string, boolean, void, function, array types
- **Type inference**: Automatic type deduction for expressions
- **Type compatibility**: Validation of type assignments and operations
- **Type annotation**: Adding type information to AST nodes

### 5. Symbol Table Management
- **Scoped symbol tables**: Nested scope handling with push/pop operations
- **Symbol definition**: Register variables, functions, types in current scope
- **Symbol lookup**: Multi-scope symbol resolution
- **Symbol validation**: Check for redefinitions and undefined references

### 6. Code Generation
- **Multi-target support**: Generate code for different backends (LLVM, native)
- **AST-to-code translation**: Convert AST nodes to target code
- **Instruction emission**: Low-level instruction generation
- **Output management**: Collect and format generated code

### 7. Error Reporting System
- **Error severity levels**: Warning, error, fatal classifications
- **Location tracking**: Line and column information for errors
- **Error collection**: Accumulate multiple errors before reporting
- **Formatted output**: Human-readable error messages

### 8. Optimization Framework
- **Optimization passes**: Constant folding, dead code elimination, inlining
- **Optimization levels**: Configurable optimization intensity (0-3)
- **AST optimization**: Tree-level optimizations before code generation
- **Performance analysis**: Optimization effectiveness metrics

## Usage Examples

### Basic Compilation Pipeline

```cursed
yeet "compiler_core"

# Complete compilation process
slay compile_program(source tea) tea {
    # 1. Lexical analysis
    sus lexer = lexer_create(source)
    sus tokens = lexer_tokenize(lexer)
    
    # 2. Parsing
    sus parser = parser_create(tokens)
    sus ast = parser_parse_program(parser)
    
    # 3. Type checking
    sus typechecker = typechecker_create()
    typechecker_check_node(typechecker, ast)
    
    # 4. Code generation
    sus codegen = codegen_create("llvm")
    sus code = codegen_generate_node(codegen, ast)
    
    damn code
}
```

### Error-Safe Compilation

```cursed
slay safe_compile(source tea) tea {
    sus error_reporter = error_reporter_create()
    
    # Compile with error handling
    sus result = compiler_compile_safe(source, "native", 2)
    
    lowkey error_has_errors(error_reporter) {
        vibez.spill("Compilation failed with errors")
        damn "compilation_failed"
    } aight {
        damn result
    }
}
```

### Self-Hosting Bootstrap

```cursed
# Bootstrap the compiler to compile itself
slay bootstrap_self_hosting() tea {
    sus compiler_source = load_compiler_source()
    sus bootstrap_result = compiler_bootstrap_compile(compiler_source)
    
    lowkey bootstrap_result == "compilation_failed" {
        vibez.spill("Bootstrap compilation failed")
        damn cap
    } aight {
        vibez.spill("Self-hosting compiler successfully bootstrapped!")
        damn based
    }
}
```

### Advanced AST Manipulation

```cursed
# Create and manipulate AST programmatically
slay build_function_ast(name tea, params [tea], body tea) ASTNode {
    sus func_node = ast_create_node(AST_FUNCTION, name)
    
    # Add parameter nodes
    bestie i := 0; i < params.length; i++ {
        sus param_node = ast_create_node(AST_VARIABLE, params[i])
        ast_add_child(func_node, param_node)
    }
    
    # Add body node
    sus body_node = ast_create_node(AST_STATEMENT, body)
    ast_add_child(func_node, body_node)
    
    damn func_node
}
```

### Type Checking Integration

```cursed
# Advanced type checking with symbol tables
slay type_check_with_symbols(ast ASTNode) lit {
    sus typechecker = typechecker_create()
    sus symboltable = symboltable_create()
    
    # Enter global scope
    symboltable_push_scope(symboltable)
    
    # Type check with symbol resolution
    sus type_result = typechecker_check_node(typechecker, ast)
    
    # Annotate AST with type information
    typechecker_annotate(typechecker, ast)
    
    symboltable_pop_scope(symboltable)
    damn based
}
```

## Self-Hosting Implementation

The compiler core provides all necessary infrastructure for self-hosting:

1. **Lexical Analysis**: Tokenize CURSED source code
2. **Parsing**: Build AST from CURSED grammar
3. **Type Checking**: Validate CURSED type system
4. **Code Generation**: Generate LLVM IR or native code
5. **Error Handling**: Report compilation errors gracefully
6. **Optimization**: Optimize generated code for performance

### Bootstrap Process

```cursed
# Self-hosting bootstrap sequence
slay self_hosting_bootstrap() lit {
    # 1. Verify compiler core readiness
    lowkey !compiler_core_self_hosting_ready() {
        damn cap
    }
    
    # 2. Load compiler source code
    sus compiler_source = load_file("src/main.csd")
    
    # 3. Bootstrap compile
    sus bootstrap_code = compiler_bootstrap_compile(compiler_source)
    
    # 4. Validate bootstrap success
    lowkey bootstrap_code == "compilation_failed" {
        damn cap
    }
    
    vibez.spill("Self-hosting bootstrap complete!")
    damn based
}
```

## Performance Characteristics

- **Lexer**: Linear time complexity O(n) where n is source length
- **Parser**: O(n) for most constructs, O(n²) worst case for deeply nested expressions
- **Type Checker**: O(n) for type inference, O(nm) for constraint solving
- **Code Generator**: O(n) for AST traversal and code emission
- **Symbol Table**: O(1) average for lookups, O(n) worst case
- **Error Reporting**: O(1) for individual reports, O(n) for bulk operations

## Testing

Comprehensive test suite in `test_compiler_core.csd` covers:

- ✅ **Lexical Analysis**: Token generation and manipulation
- ✅ **Parser Infrastructure**: Grammar productions and error recovery  
- ✅ **AST Operations**: Tree construction and traversal
- ✅ **Type Checking**: Type inference and validation
- ✅ **Symbol Tables**: Scope management and symbol resolution
- ✅ **Code Generation**: Multi-target code emission
- ✅ **Error Reporting**: Error collection and formatting
- ✅ **Integration Tests**: Complete pipeline validation
- ✅ **Performance Tests**: Large code sample compilation
- ✅ **Self-Hosting Tests**: Bootstrap compilation validation

## Dependencies

- `testz`: Testing framework for comprehensive validation
- No external FFI dependencies - pure CURSED implementation

## Status

✅ **Production Ready**: Complete implementation with comprehensive test coverage
✅ **Self-Hosting Ready**: All components required for bootstrap compilation
✅ **Performance Optimized**: Efficient algorithms and data structures
✅ **Error Robust**: Comprehensive error handling and recovery
✅ **Extensible**: Clean architecture for adding new language features

## Future Enhancements

- **Incremental compilation**: Support for partial recompilation
- **Advanced optimizations**: More sophisticated optimization passes
- **Debugging support**: Integration with debug information generation
- **Parallel compilation**: Multi-threaded compilation pipeline
- **IDE integration**: Language server protocol support
- **Cross-compilation**: Target multiple architectures

## Architecture Diagram

```
Source Code
    ↓
[Lexical Analysis] → Tokens
    ↓
[Parser] → AST
    ↓
[Type Checker] → Typed AST
    ↓
[Symbol Table] → Scoped AST  
    ↓
[Optimizer] → Optimized AST
    ↓
[Code Generator] → Target Code
    ↓
[Error Reporter] → Diagnostics
```

## Contributing

When extending the compiler core:

1. Add comprehensive tests for new functionality
2. Update documentation for new APIs
3. Ensure FFI-free pure CURSED implementation
4. Validate self-hosting compatibility
5. Run complete test suite before submitting changes

The compiler core is critical infrastructure - all changes must maintain backward compatibility and self-hosting capabilities.
