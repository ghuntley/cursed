# Parser Module

The CURSED parser module provides comprehensive language parsing capabilities for self-hosting compilation. This module implements lexical analysis, AST construction, error handling, symbol table management, and type checking.

## Overview

The parser module is critical for self-hosting - it enables the CURSED compiler to parse CURSED source code and compile itself. The module follows a recursive descent parsing strategy with operator precedence handling.

## Core Components

### 1. Lexical Analysis

**Tokenization Functions:**
- `tokenize(source tea) []Token` - Convert source code into tokens
- `is_keyword(value tea) lit` - Check if string is a language keyword
- `is_operator(ch sip) lit` - Check if character is an operator
- `is_delimiter(ch sip) lit` - Check if character is a delimiter

**Token Types:**
- `TokenIdentifier` - Variable and function names
- `TokenNumber` - Numeric literals (integer and float)
- `TokenString` - String literals
- `TokenKeyword` - Language keywords (sus, slay, damn, etc.)
- `TokenOperator` - Operators (+, -, *, /, =, etc.)
- `TokenDelimiter` - Delimiters (parentheses, braces, semicolons)
- `TokenComment` - Comments (# prefixed)
- `TokenEOF` - End of file marker
- `TokenError` - Malformed tokens

### 2. AST Construction

**Core Parsing Functions:**
- `parse_program(parser *Parser) *ASTNode` - Parse complete program
- `parse_statement(parser *Parser) *ASTNode` - Parse statements
- `parse_expression(parser *Parser) *ASTNode` - Parse expressions with precedence
- `parse_variable_declaration(parser *Parser) *ASTNode` - Parse variable declarations
- `parse_function_declaration(parser *Parser) *ASTNode` - Parse function declarations

**Expression Parsing (Precedence Order):**
1. `parse_logical_or()` - Logical OR (`||`)
2. `parse_logical_and()` - Logical AND (`&&`)
3. `parse_equality()` - Equality (`==`, `!=`)
4. `parse_comparison()` - Comparison (`<`, `>`, `<=`, `>=`)
5. `parse_term()` - Addition/Subtraction (`+`, `-`)
6. `parse_factor()` - Multiplication/Division (`*`, `/`, `%`)
7. `parse_unary()` - Unary operators (`!`, `-`, `+`)
8. `parse_call()` - Function calls and member access
9. `parse_primary()` - Literals, identifiers, parenthesized expressions

**AST Node Types:**
- `NodeProgram` - Root program node
- `NodeStatement` - Statement nodes
- `NodeExpression` - Expression nodes
- `NodeDeclaration` - Declaration nodes
- `NodeFunction` - Function definition nodes
- `NodeVariable` - Variable declaration nodes
- `NodeBinary` - Binary expression nodes
- `NodeUnary` - Unary expression nodes
- `NodeCall` - Function call nodes
- `NodeLiteral` - Literal value nodes

### 3. Error Handling

**Error Management:**
- `parse_error(parser *Parser, message tea)` - Report parse errors
- `error_recovery(parser *Parser)` - Recover from syntax errors
- `get_errors(parser *Parser) []tea` - Get all parse errors
- `clear_errors(parser *Parser)` - Clear error list

**Error Recovery Strategy:**
- Synchronize to statement boundaries (semicolons, keywords)
- Continue parsing after errors to find additional issues
- Provide meaningful error messages with line/column information

### 4. Symbol Table Management

**Symbol Table Functions:**
- `create_symbol_table() []Symbol` - Create new symbol table
- `resolve_symbol(parser *Parser, name tea) *Symbol` - Look up symbols
- `add_symbol(parser *Parser, symbol Symbol)` - Add symbol to table

**Symbol Information:**
- Name and type of identifier
- Scope level for nested contexts
- Line number where declared
- Symbol category (variable, function, constant, type)

### 5. Type Checking

**Type Analysis:**
- `check_types(node *ASTNode, parser *Parser) lit` - Validate type compatibility
- `infer_type(node *ASTNode, parser *Parser) tea` - Infer expression types

**Type Inference Rules:**
- Literal values: `42` → `normie`, `3.14` → `meal`, `"text"` → `tea`
- Binary expressions: Type promotion (int + float → float)
- Function calls: Return type from function signature
- Member access: Type of accessed member

## Data Structures

### Token Structure
```cursed
be_like Token = struct {
    token_type TokenType    # Type of token
    value tea              # Token text
    line normie            # Line number
    column normie          # Column position
}
```

### AST Node Structure
```cursed
be_like ASTNode = struct {
    node_type ASTNodeType  # Type of AST node
    value tea              # Node value/name
    children []*ASTNode    # Child nodes
    line normie            # Source line
    column normie          # Source column
}
```

### Symbol Structure
```cursed
be_like Symbol = struct {
    name tea               # Symbol name
    symbol_type tea        # Symbol type
    scope normie           # Scope level
    declared_line normie   # Declaration line
}
```

### Parser State
```cursed
be_like Parser = struct {
    tokens []Token         # Token stream
    current normie         # Current position
    symbols []Symbol       # Symbol table
    errors []tea           # Parse errors
    current_scope normie   # Current scope level
}
```

## Usage Examples

### Basic Tokenization
```cursed
yeet "parser"

sus source := "sus x := 42 + 10"
sus tokens := tokenize(source)

# tokens[0] = Token{TokenKeyword, "sus", 1, 1}
# tokens[1] = Token{TokenIdentifier, "x", 1, 5}
# tokens[2] = Token{TokenOperator, ":=", 1, 7}
# etc.
```

### Complete Program Parsing
```cursed
yeet "parser"

sus source := `
    sus x normie := 42
    slay double(n normie) normie {
        damn n * 2
    }
    sus result := double(x)
`

sus ast, errors := parse_source(source)
shook len(errors) == 0 {
    vibez.spill("Parsing successful!")
    print_ast(ast, 0)  # Pretty print AST
}
```

### Symbol Table Usage
```cursed
yeet "parser"

sus tokens := tokenize("sus x := 42")
sus parser := create_parser(tokens)
sus program := parse_program(parser)

# Check symbol table
sus symbol := resolve_symbol(parser, "x")
shook symbol != cringe {
    vibez.spill("Found variable: " + symbol.name)
    vibez.spill("Type: " + symbol.symbol_type)
}
```

### Type Checking
```cursed
yeet "parser"

sus ast, _ := parse_source("1 + 2.5")
sus expr_type := infer_type(ast.children[0], cringe)
# expr_type = "meal" (float due to type promotion)
```

## Language Support

The parser supports all CURSED language features:

**Keywords:** `sus`, `slay`, `damn`, `shook`, `cap`, `bestie`, `yeet`, `vibez`, `based`, `cringe`, `facts`, `be_like`, `stan`, `dm`, `ready`, `yikes`, `fam`

**Operators:** Arithmetic (`+`, `-`, `*`, `/`, `%`), Comparison (`<`, `>`, `<=`, `>=`, `==`, `!=`), Logical (`&&`, `||`, `!`), Assignment (`:=`, `=`)

**Delimiters:** Parentheses, braces, brackets, semicolons, commas, dots

**Types:** `normie` (int), `meal` (float), `tea` (string), `lit` (bool), `sip` (char), arrays, pointers, structs

## Advanced Features

### AST Traversal
```cursed
# Visit all nodes with custom function
visit_ast(ast, node_processor)

# Count specific node types
sus function_count := count_nodes(ast, NodeFunction)

# Find nodes by value
sus test_nodes := find_nodes_by_value(ast, "test")
```

### Error Recovery
The parser implements panic-mode error recovery:
- Detects syntax errors and reports location
- Synchronizes to safe recovery points
- Continues parsing to find additional errors
- Maintains partial AST for error correction

### Operator Precedence
Implements standard operator precedence:
1. Parentheses (highest)
2. Unary operators (`!`, `-`, `+`)
3. Multiplicative (`*`, `/`, `%`)
4. Additive (`+`, `-`)
5. Comparison (`<`, `>`, `<=`, `>=`)
6. Equality (`==`, `!=`)
7. Logical AND (`&&`)
8. Logical OR (`||`) (lowest)

## Self-Hosting Integration

This parser module is designed for self-hosting compilation:

1. **Lexical Analysis:** Tokenizes CURSED source code
2. **Syntax Analysis:** Builds Abstract Syntax Tree
3. **Semantic Analysis:** Symbol resolution and type checking
4. **Error Reporting:** Comprehensive error messages
5. **Code Generation:** AST ready for LLVM IR generation

The parser integrates with other compiler phases:
- **Frontend:** Tokenization and parsing
- **Middle-end:** Semantic analysis and optimization
- **Backend:** Code generation from AST

## Testing

Run comprehensive tests:
```bash
# Test parsing module
cargo run --bin cursed stdlib/parser/test_parser.💀

# Test both interpretation and compilation
cargo run --bin cursed stdlib/parser/test_parser.💀
cargo run --bin cursed -- compile stdlib/parser/test_parser.💀
./test_parser
```

## Performance Characteristics

- **Tokenization:** Linear time O(n) in source length
- **Parsing:** Recursive descent with linear time for most constructs
- **Memory:** AST nodes allocated dynamically
- **Error Recovery:** Minimal performance impact
- **Symbol Table:** Linear search (suitable for typical program sizes)

## Future Enhancements

- **Incremental Parsing:** Parse only changed portions
- **Better Error Messages:** More context and suggestions
- **AST Optimization:** Constant folding during parsing
- **Parallel Parsing:** Multiple files simultaneously
- **LSP Integration:** Language server protocol support

## Dependencies

- `core` - Core CURSED runtime functions
- `stringz` - String manipulation utilities
- `testz` - Testing framework

## License

This module is part of the CURSED programming language implementation and follows the same license terms.
