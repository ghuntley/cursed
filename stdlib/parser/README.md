# Parser Module

The parser module provides comprehensive parsing utilities and AST manipulation capabilities for the CURSED language. It enables building parsers, working with Abstract Syntax Trees (ASTs), and performing code analysis and generation.

## Features

- **Lexical Analysis**: Tokenization and lexer functionality
- **Parsing**: Recursive descent parser with error recovery
- **AST Manipulation**: Create, modify, and traverse Abstract Syntax Trees
- **Code Generation**: Generate code from AST representations
- **Grammar Support**: Load and validate custom grammars
- **Operator Precedence**: Configurable operator precedence and associativity
- **Error Handling**: Comprehensive error reporting with line/column information
- **Performance Metrics**: Parse time and memory usage tracking
- **Pure CURSED Implementation**: No external dependencies

## Token Types

```cursed
TOKEN_IDENTIFIER = 1
TOKEN_NUMBER = 2
TOKEN_STRING = 3
TOKEN_KEYWORD = 4
TOKEN_OPERATOR = 5
TOKEN_PUNCTUATION = 6
TOKEN_WHITESPACE = 7
TOKEN_COMMENT = 8
TOKEN_EOF = 9
```

## AST Node Types

```cursed
AST_EXPRESSION = 1
AST_STATEMENT = 2
AST_DECLARATION = 3
AST_BLOCK = 4
AST_FUNCTION = 5
AST_LITERAL = 6
AST_IDENTIFIER = 7
AST_BINARY_OP = 8
AST_UNARY_OP = 9
AST_CALL = 10
```

## Parser States

```cursed
PARSER_STATE_INITIAL = 0
PARSER_STATE_PARSING = 1
PARSER_STATE_ERROR = 2
PARSER_STATE_COMPLETE = 3
```

## Basic Usage

### Lexical Analysis

```cursed
yeet "parser"

# Create lexer
sus lexer_id normie = parser_lexer_create("let x = 42")

# Get next token
sus token_id normie = parser_lexer_next_token(lexer_id)

# Get token information
sus token_type smol = parser_token_get_type(token_id)
sus token_value tea = parser_token_get_value(token_id)
sus line normie = parser_token_get_line(token_id)
sus column normie = parser_token_get_column(token_id)
```

### Parser Creation

```cursed
# Create parser
sus parser_id normie = parser_create("let x = 42")

# Get parser state
sus state smol = parser_get_state(parser_id)

# Reset parser
parser_reset(parser_id)

# Destroy parser
parser_destroy(parser_id)
```

### AST Node Creation

```cursed
# Create AST nodes
sus expr_node normie = parser_ast_create_node(AST_EXPRESSION)
sus stmt_node normie = parser_ast_create_node(AST_STATEMENT)

# Set node values
parser_ast_set_value(expr_node, "x + 1")
parser_ast_set_type(expr_node, AST_BINARY_OP)

# Get node information
sus node_value tea = parser_ast_get_value(expr_node)
sus node_type smol = parser_ast_get_type(expr_node)
```

### AST Tree Structure

```cursed
# Add child nodes
parser_ast_add_child(stmt_node, expr_node)

# Get child nodes
sus child_node normie = parser_ast_get_child(stmt_node, 0)
sus child_count normie = parser_ast_get_child_count(stmt_node)
```

### Expression Parsing

```cursed
# Parse expressions
sus expr_ast normie = parser_parse_expression(parser_id)
sus binary_ast normie = parser_parse_binary_expression(parser_id, left_node, "+")
sus unary_ast normie = parser_parse_unary_expression(parser_id, "-")
sus primary_ast normie = parser_parse_primary_expression(parser_id)
```

### Statement Parsing

```cursed
# Parse statements
sus stmt_ast normie = parser_parse_statement(parser_id)
sus decl_ast normie = parser_parse_declaration(parser_id)
sus block_ast normie = parser_parse_block(parser_id)
sus func_ast normie = parser_parse_function(parser_id)
```

### Error Handling

```cursed
# Check for errors
sus has_error lit = parser_has_error(parser_id)

# Get error information
sus error_msg tea = parser_get_error(parser_id)
sus error_line normie = parser_get_error_line(parser_id)
sus error_col normie = parser_get_error_column(parser_id)
```

### AST Traversal

```cursed
# Traverse AST
parser_ast_traverse(root_node, "visitor_function")

# Find nodes by type
sus found_nodes tea = parser_ast_find_nodes(root_node, AST_EXPRESSION)

# Clone nodes
sus cloned_node normie = parser_ast_clone_node(original_node)

# Replace nodes
parser_ast_replace_node(old_node, new_node)
```

### Code Generation

```cursed
# Generate code from AST
sus generated_code tea = parser_ast_to_code(root_node)

# Convert AST to JSON
sus json_ast tea = parser_ast_to_json(root_node)

# Create AST from JSON
sus ast_from_json normie = parser_ast_from_json("{\"type\": \"expression\"}")
```

### Operator Precedence

```cursed
# Get operator precedence
sus precedence normie = parser_get_operator_precedence("+")
sus associativity smol = parser_get_operator_associativity("*")

# Check operator types
sus is_binary lit = parser_is_binary_operator("+")
sus is_unary lit = parser_is_unary_operator("-")
```

### Grammar Support

```cursed
# Load and validate grammar
sus is_valid lit = parser_validate_grammar("grammar.bnf")
sus grammar_id normie = parser_load_grammar("grammar.bnf")
parser_set_grammar(parser_id, grammar_id)
```

### Utility Functions

```cursed
# Check token types
sus is_keyword lit = parser_is_keyword("let")
sus is_identifier lit = parser_is_identifier("variable")
sus is_number lit = parser_is_number("42")
sus is_string lit = parser_is_string_literal("\"hello\"")
```

### Parser Configuration

```cursed
# Set parser options
parser_set_option(parser_id, "debug", "true")
parser_set_option(parser_id, "strict", "false")

# Get parser options
sus debug_mode tea = parser_get_option(parser_id, "debug")
```

### Performance Metrics

```cursed
# Get performance information
sus parse_time normie = parser_get_parse_time(parser_id)
sus memory_usage normie = parser_get_memory_usage(parser_id)
sus node_count normie = parser_get_node_count(parser_id)
```

## Functions

### Lexer Functions
- `parser_lexer_create(input tea) normie` - Create lexer
- `parser_lexer_next_token(lexer_id normie) normie` - Get next token
- `parser_token_get_type(token_id normie) smol` - Get token type
- `parser_token_get_value(token_id normie) tea` - Get token value
- `parser_token_get_position(token_id normie) normie` - Get token position
- `parser_token_get_line(token_id normie) normie` - Get token line
- `parser_token_get_column(token_id normie) normie` - Get token column

### Parser Functions
- `parser_create(input tea) normie` - Create parser
- `parser_destroy(parser_id normie) lit` - Destroy parser
- `parser_get_state(parser_id normie) smol` - Get parser state
- `parser_reset(parser_id normie) lit` - Reset parser

### AST Node Functions
- `parser_ast_create_node(node_type smol) normie` - Create AST node
- `parser_ast_set_value(node_id normie, value tea) lit` - Set node value
- `parser_ast_get_value(node_id normie) tea` - Get node value
- `parser_ast_set_type(node_id normie, node_type smol) lit` - Set node type
- `parser_ast_get_type(node_id normie) smol` - Get node type

### AST Tree Functions
- `parser_ast_add_child(parent_id normie, child_id normie) lit` - Add child node
- `parser_ast_get_child(parent_id normie, index normie) normie` - Get child node
- `parser_ast_get_child_count(parent_id normie) normie` - Get child count

### Expression Parsing Functions
- `parser_parse_expression(parser_id normie) normie` - Parse expression
- `parser_parse_binary_expression(parser_id normie, left_id normie, operator tea) normie` - Parse binary expression
- `parser_parse_unary_expression(parser_id normie, operator tea) normie` - Parse unary expression
- `parser_parse_primary_expression(parser_id normie) normie` - Parse primary expression

### Statement Parsing Functions
- `parser_parse_statement(parser_id normie) normie` - Parse statement
- `parser_parse_declaration(parser_id normie) normie` - Parse declaration
- `parser_parse_block(parser_id normie) normie` - Parse block
- `parser_parse_function(parser_id normie) normie` - Parse function

### Error Handling Functions
- `parser_get_error(parser_id normie) tea` - Get error message
- `parser_get_error_line(parser_id normie) normie` - Get error line
- `parser_get_error_column(parser_id normie) normie` - Get error column
- `parser_has_error(parser_id normie) lit` - Check for errors

### AST Traversal Functions
- `parser_ast_traverse(root_id normie, visitor_name tea) lit` - Traverse AST
- `parser_ast_find_nodes(root_id normie, node_type smol) tea` - Find nodes by type
- `parser_ast_replace_node(old_node_id normie, new_node_id normie) lit` - Replace node
- `parser_ast_clone_node(node_id normie) normie` - Clone node

### Code Generation Functions
- `parser_ast_to_code(root_id normie) tea` - Generate code from AST
- `parser_ast_to_json(root_id normie) tea` - Convert AST to JSON
- `parser_ast_from_json(json_string tea) normie` - Create AST from JSON

### Operator Functions
- `parser_get_operator_precedence(operator tea) normie` - Get operator precedence
- `parser_get_operator_associativity(operator tea) smol` - Get operator associativity
- `parser_is_binary_operator(operator tea) lit` - Check if binary operator
- `parser_is_unary_operator(operator tea) lit` - Check if unary operator

### Grammar Functions
- `parser_validate_grammar(grammar_file tea) lit` - Validate grammar
- `parser_load_grammar(grammar_file tea) normie` - Load grammar
- `parser_set_grammar(parser_id normie, grammar_id normie) lit` - Set grammar

### Utility Functions
- `parser_is_keyword(word tea) lit` - Check if keyword
- `parser_is_identifier(word tea) lit` - Check if identifier
- `parser_is_number(word tea) lit` - Check if number
- `parser_is_string_literal(word tea) lit` - Check if string literal

### Configuration Functions
- `parser_set_option(parser_id normie, option_name tea, option_value tea) lit` - Set option
- `parser_get_option(parser_id normie, option_name tea) tea` - Get option

### Performance Functions
- `parser_get_parse_time(parser_id normie) normie` - Get parse time
- `parser_get_memory_usage(parser_id normie) normie` - Get memory usage
- `parser_get_node_count(parser_id normie) normie` - Get node count

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/parser/test_parser.csd
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/parser/test_parser.csd
cargo run --bin cursed -- compile stdlib/parser/test_parser.csd
./test_parser
```

## Error Handling

All functions return appropriate error values:
- Boolean functions return `cap` (false) on error
- Integer functions return -1 on error
- String functions return empty string on error

## Performance

- Efficient tokenization with minimal memory allocation
- Optimized AST operations with smart caching
- Fast parsing with predictive lookahead
- Memory-efficient tree structures
- Optimized for both interpretation and compilation modes

## Use Cases

- **Language Implementation**: Build parsers for custom languages
- **Code Analysis**: Analyze and transform source code
- **AST Manipulation**: Modify and generate Abstract Syntax Trees
- **Compiler Construction**: Build compilers and interpreters
- **Code Generation**: Generate code from AST representations
- **Syntax Validation**: Validate and format source code

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures
- `json` - JSON handling

## License

Part of the CURSED language standard library.
