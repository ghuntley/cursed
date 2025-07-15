# Compiler Core Module

The compiler core module provides essential infrastructure for the CURSED self-hosting compiler. This module contains all the fundamental components needed to build a complete compiler in pure CURSED language.

## Features

### 1. Lexical Analysis Utilities
- **Token Types**: Complete enumeration of all CURSED language tokens
- **Tokenization**: Convert source code into token streams
- **Token Classification**: Distinguish between keywords, identifiers, operators, and literals
- **Position Tracking**: Maintain accurate line and column information for error reporting

### 2. Parsing Infrastructure
- **Recursive Descent Parser**: Full parser implementation for CURSED syntax
- **AST Generation**: Build abstract syntax trees from token streams
- **Error Recovery**: Graceful handling of syntax errors
- **Precedence Handling**: Correct operator precedence for expressions

### 3. AST Manipulation
- **Node Creation**: Utilities for creating and managing AST nodes
- **Tree Traversal**: Visitor pattern implementation for AST processing
- **Node Search**: Find specific node types within the AST
- **Tree Transformation**: Modify AST structure for optimization

### 4. Symbol Table Management
- **Scoped Symbol Tables**: Hierarchical scope management
- **Symbol Storage**: Store variables, functions, types, and constants
- **Symbol Lookup**: Efficient symbol resolution with scope traversal
- **Scope Operations**: Enter and exit scopes during compilation

### 5. Type System Utilities
- **Type Information**: Complete type metadata storage
- **Type Compatibility**: Check type compatibility for operations
- **Type Inference**: Automatic type deduction for expressions
- **Size Calculation**: Determine type sizes for memory layout

### 6. Code Generation Helpers
- **Code Generation Context**: Manage compilation state and settings
- **Label Generation**: Create unique labels for control flow
- **Register Allocation**: Generate virtual registers for intermediate code
- **Code Emission**: Generate target code from AST nodes

### 7. Error Reporting
- **Error Types**: Comprehensive error classification system
- **Error Formatting**: Human-readable error message generation
- **Error Context**: Provide source location and context information
- **Severity Levels**: Distinguish between errors and warnings

## Usage

### Basic Compilation Pipeline

```cursed
yeet "compiler_core"

# Tokenize source code
sus source tea = "sus x normie = 42"
sus tokens [Token] = tokenize(source)

# Parse tokens into AST
sus parser Parser = create_parser(tokens)
sus ast ASTNode = parse_program(parser)

# Generate code
sus context CodegenContext = create_codegen_context("llvm", 2, "x86_64")
sus output tea = generate_code(ast, context)
```

### Symbol Table Management

```cursed
yeet "compiler_core"

# Create symbol table
sus table SymbolTable = create_symbol_table()

# Add symbol
sus symbol SymbolInfo = create_symbol_info("variable", SymbolType.VARIABLE, "normie", 0, 1, 1, based, cap)
add_symbol(table, symbol)

# Lookup symbol
sus found SymbolInfo = lookup_symbol(table, "variable")
```

### Type System Operations

```cursed
yeet "compiler_core"

# Check type compatibility
sus compatible lit = types_compatible("normie", "thicc")

# Infer type from AST node
sus node ASTNode = create_ast_node(ASTNodeType.LITERAL, "42", [], 1, 1)
sus inferred_type tea = infer_type(node)

# Get type size
sus size normie = get_type_size("normie")
```

## API Reference

### Core Functions

#### Lexical Analysis
- `tokenize(source tea) [Token]` - Tokenize source code
- `create_token(type, value, line, column, position) Token` - Create token
- `classify_token(value tea) normie` - Classify token type
- `classify_operator(ch sip, line, column, position) Token` - Classify operator

#### Parsing
- `create_parser(tokens [Token]) Parser` - Create parser
- `parse_program(parser Parser) ASTNode` - Parse complete program
- `parse_expression(parser Parser) ASTNode` - Parse expression
- `parse_statement(parser Parser) ASTNode` - Parse statement

#### AST Manipulation
- `create_ast_node(type, value, children, line, column) ASTNode` - Create AST node
- `traverse_ast(node ASTNode, visitor_func) void` - Traverse AST
- `find_nodes_by_type(node ASTNode, type normie) [ASTNode]` - Find nodes by type
- `transform_ast(node ASTNode, transformer_func) ASTNode` - Transform AST

#### Symbol Table
- `create_symbol_table() SymbolTable` - Create symbol table
- `add_symbol(table SymbolTable, symbol SymbolInfo) lit` - Add symbol
- `lookup_symbol(table SymbolTable, name tea) SymbolInfo` - Lookup symbol
- `enter_scope(table SymbolTable, type tea) normie` - Enter new scope
- `exit_scope(table SymbolTable) lit` - Exit current scope

#### Type System
- `create_type_info(name, size, alignment, primitive) TypeInfo` - Create type info
- `get_type_size(type_name tea) normie` - Get type size
- `types_compatible(type1 tea, type2 tea) lit` - Check compatibility
- `infer_type(node ASTNode) tea` - Infer type from AST

#### Code Generation
- `create_codegen_context(format, opt_level, arch) CodegenContext` - Create context
- `generate_code(node ASTNode, context CodegenContext) tea` - Generate code
- `generate_label(context CodegenContext) tea` - Generate unique label
- `generate_register(context CodegenContext) tea` - Generate unique register

#### Error Reporting
- `create_error(type, message, line, column, file, severity) CompilerError` - Create error
- `format_error(error CompilerError) tea` - Format error message
- `report_error(error CompilerError) void` - Report error

## Data Structures

### Token
```cursed
vibe Token {
    token_type normie     # Token type identifier
    value tea            # Token value/lexeme
    line normie          # Line number
    column normie        # Column number
    position normie      # Character position
}
```

### ASTNode
```cursed
vibe ASTNode {
    node_type normie     # AST node type
    value tea           # Node value
    children [ASTNode]  # Child nodes
    line normie         # Line number
    column normie       # Column number
    symbol_info SymbolInfo  # Associated symbol info
}
```

### SymbolInfo
```cursed
vibe SymbolInfo {
    name tea            # Symbol name
    symbol_type normie  # Symbol type
    data_type tea       # Data type
    scope normie        # Scope identifier
    line normie         # Declaration line
    column normie       # Declaration column
    is_mutable lit      # Mutability flag
    is_exported lit     # Export flag
}
```

### TypeInfo
```cursed
vibe TypeInfo {
    type_name tea       # Type name
    size normie         # Size in bytes
    alignment normie    # Alignment requirement
    is_primitive lit    # Primitive type flag
    is_pointer lit      # Pointer type flag
    is_array lit        # Array type flag
    element_type tea    # Element type (for arrays)
    is_function lit     # Function type flag
    params [tea]        # Parameter types
    return_type tea     # Return type
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/compiler_core/test_compiler_core.csd
```

### Test Coverage

The test suite covers:
- **Lexical Analysis**: Token creation, classification, and tokenization
- **Parsing**: Parser creation, program parsing, and expression parsing
- **AST Manipulation**: Node creation, traversal, and transformation
- **Symbol Table**: Symbol creation, lookup, and scope management
- **Type System**: Type inference, compatibility, and size calculation
- **Code Generation**: Context creation, code emission, and register allocation
- **Error Reporting**: Error creation, formatting, and reporting

## Self-Hosting Integration

This module is designed specifically for the CURSED self-hosting compiler:

1. **Pure CURSED Implementation**: No external dependencies or FFI calls
2. **Complete Coverage**: All essential compiler components included
3. **Extensible Design**: Easy to add new language features
4. **Performance Optimized**: Efficient algorithms for production use
5. **Error Resilient**: Robust error handling and recovery

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation utilities
- `collections` - Data structure operations
- `error_drip` - Error handling utilities
- `dropz` - I/O operations

## Status

**✅ Production Ready** - Complete implementation with comprehensive test coverage ready for self-hosting compiler bootstrap.

## Performance Characteristics

- **Tokenization**: O(n) linear time complexity
- **Parsing**: O(n) for most constructs, O(n²) worst case for deeply nested expressions
- **Symbol Lookup**: O(log n) average case with scope hierarchy
- **Type Inference**: O(n) for expression trees
- **Code Generation**: O(n) linear traversal of AST

## Future Enhancements

- **Optimization Passes**: Additional code optimization phases
- **Advanced Error Recovery**: More sophisticated error recovery strategies
- **Incremental Compilation**: Support for incremental compilation
- **Parallel Processing**: Multi-threaded compilation support
- **Debug Information**: Enhanced debugging metadata generation
