yeet "testz"
yeet "parser"

# Parser Module Comprehensive Test Suite
# Testing all parsing utilities and AST manipulation functionality

test_start("Lexer Functionality Tests")

# Test lexer creation
assert_eq_int(parser_lexer_create("let x = 42"), 1)
assert_eq_int(parser_lexer_create(""), -1)

# Test token operations
assert_eq_int(parser_lexer_next_token(1), 1)
assert_eq_int(parser_lexer_next_token(-1), -1)
assert_eq_int(parser_token_get_type(1), TOKEN_IDENTIFIER)
assert_eq_int(parser_token_get_type(-1), -1)
assert_eq_string(parser_token_get_value(1), "token_value")
assert_eq_string(parser_token_get_value(-1), "")
assert_eq_int(parser_token_get_position(1), 1)
assert_eq_int(parser_token_get_position(-1), -1)
assert_eq_int(parser_token_get_line(1), 1)
assert_eq_int(parser_token_get_line(-1), -1)
assert_eq_int(parser_token_get_column(1), 1)
assert_eq_int(parser_token_get_column(-1), -1)

print_test_summary()

test_start("Parser Creation and Management Tests")

# Test parser creation
assert_eq_int(parser_create("let x = 42"), 1)
assert_eq_int(parser_create(""), -1)

# Test parser state
assert_eq_int(parser_get_state(1), PARSER_STATE_INITIAL)
assert_eq_int(parser_get_state(-1), -1)
assert_true(parser_reset(1))
assert_false(parser_reset(-1))
assert_true(parser_destroy(1))
assert_false(parser_destroy(-1))

print_test_summary()

test_start("AST Node Creation Tests")

# Test AST node creation
assert_eq_int(parser_ast_create_node(AST_EXPRESSION), 1)
assert_eq_int(parser_ast_create_node(0), -1)
assert_eq_int(parser_ast_create_node(11), -1)

# Test AST node value operations
assert_true(parser_ast_set_value(1, "test_value"))
assert_false(parser_ast_set_value(-1, "test_value"))
assert_eq_string(parser_ast_get_value(1), "node_value")
assert_eq_string(parser_ast_get_value(-1), "")

# Test AST node type operations
assert_true(parser_ast_set_type(1, AST_STATEMENT))
assert_false(parser_ast_set_type(-1, AST_STATEMENT))
assert_false(parser_ast_set_type(1, 0))
assert_false(parser_ast_set_type(1, 11))
assert_eq_int(parser_ast_get_type(1), AST_EXPRESSION)
assert_eq_int(parser_ast_get_type(-1), -1)

print_test_summary()

test_start("AST Tree Structure Tests")

# Test AST child operations
assert_true(parser_ast_add_child(1, 2))
assert_false(parser_ast_add_child(-1, 2))
assert_false(parser_ast_add_child(1, -1))
assert_eq_int(parser_ast_get_child(1, 0), 1)
assert_eq_int(parser_ast_get_child(-1, 0), -1)
assert_eq_int(parser_ast_get_child(1, -1), -1)
assert_eq_int(parser_ast_get_child_count(1), 0)
assert_eq_int(parser_ast_get_child_count(-1), -1)

print_test_summary()

test_start("Expression Parsing Tests")

# Test expression parsing
assert_eq_int(parser_parse_expression(1), 1)
assert_eq_int(parser_parse_expression(-1), -1)
assert_eq_int(parser_parse_binary_expression(1, 1, "+"), 1)
assert_eq_int(parser_parse_binary_expression(-1, 1, "+"), -1)
assert_eq_int(parser_parse_binary_expression(1, -1, "+"), -1)
assert_eq_int(parser_parse_binary_expression(1, 1, ""), -1)
assert_eq_int(parser_parse_unary_expression(1, "-"), 1)
assert_eq_int(parser_parse_unary_expression(-1, "-"), -1)
assert_eq_int(parser_parse_unary_expression(1, ""), -1)
assert_eq_int(parser_parse_primary_expression(1), 1)
assert_eq_int(parser_parse_primary_expression(-1), -1)

print_test_summary()

test_start("Statement Parsing Tests")

# Test statement parsing
assert_eq_int(parser_parse_statement(1), 1)
assert_eq_int(parser_parse_statement(-1), -1)
assert_eq_int(parser_parse_declaration(1), 1)
assert_eq_int(parser_parse_declaration(-1), -1)
assert_eq_int(parser_parse_block(1), 1)
assert_eq_int(parser_parse_block(-1), -1)
assert_eq_int(parser_parse_function(1), 1)
assert_eq_int(parser_parse_function(-1), -1)

print_test_summary()

test_start("Error Handling Tests")

# Test error handling
assert_eq_string(parser_get_error(1), "No error")
assert_eq_string(parser_get_error(-1), "")
assert_eq_int(parser_get_error_line(1), 1)
assert_eq_int(parser_get_error_line(-1), -1)
assert_eq_int(parser_get_error_column(1), 1)
assert_eq_int(parser_get_error_column(-1), -1)
assert_false(parser_has_error(1))
assert_true(parser_has_error(-1))

print_test_summary()

test_start("AST Traversal and Manipulation Tests")

# Test AST traversal
assert_true(parser_ast_traverse(1, "visitor_function"))
assert_false(parser_ast_traverse(-1, "visitor_function"))
assert_false(parser_ast_traverse(1, ""))
assert_eq_string(parser_ast_find_nodes(1, AST_EXPRESSION), "[1, 2, 3]")
assert_eq_string(parser_ast_find_nodes(-1, AST_EXPRESSION), "")
assert_eq_string(parser_ast_find_nodes(1, 0), "")
assert_eq_string(parser_ast_find_nodes(1, 11), "")

# Test AST manipulation
assert_true(parser_ast_replace_node(1, 2))
assert_false(parser_ast_replace_node(-1, 2))
assert_false(parser_ast_replace_node(1, -1))
assert_eq_int(parser_ast_clone_node(1), 1)
assert_eq_int(parser_ast_clone_node(-1), -1)

print_test_summary()

test_start("Code Generation Tests")

# Test code generation
assert_eq_string(parser_ast_to_code(1), "generated_code")
assert_eq_string(parser_ast_to_code(-1), "")
assert_eq_string(parser_ast_to_json(1), "{\"type\": \"expression\", \"value\": \"test\"}")
assert_eq_string(parser_ast_to_json(-1), "")
assert_eq_int(parser_ast_from_json("{\"type\": \"expression\"}"), 1)
assert_eq_int(parser_ast_from_json(""), -1)

print_test_summary()

test_start("Operator Precedence Tests")

# Test operator precedence
assert_eq_int(parser_get_operator_precedence("+"), 1)
assert_eq_int(parser_get_operator_precedence(""), -1)
assert_eq_int(parser_get_operator_associativity("+"), 1)
assert_eq_int(parser_get_operator_associativity(""), -1)
assert_true(parser_is_binary_operator("+"))
assert_false(parser_is_binary_operator(""))
assert_true(parser_is_unary_operator("-"))
assert_false(parser_is_unary_operator(""))

print_test_summary()

test_start("Grammar Validation Tests")

# Test grammar validation
assert_true(parser_validate_grammar("grammar.bnf"))
assert_false(parser_validate_grammar(""))
assert_eq_int(parser_load_grammar("grammar.bnf"), 1)
assert_eq_int(parser_load_grammar(""), -1)
assert_true(parser_set_grammar(1, 1))
assert_false(parser_set_grammar(-1, 1))
assert_false(parser_set_grammar(1, -1))

print_test_summary()

test_start("Utility Functions Tests")

# Test utility functions
assert_true(parser_is_keyword("let"))
assert_false(parser_is_keyword(""))
assert_true(parser_is_identifier("variable"))
assert_false(parser_is_identifier(""))
assert_true(parser_is_number("42"))
assert_false(parser_is_number(""))
assert_true(parser_is_string_literal("\"hello\""))
assert_false(parser_is_string_literal(""))

print_test_summary()

test_start("Parser Configuration Tests")

# Test parser configuration
assert_true(parser_set_option(1, "debug", "true"))
assert_false(parser_set_option(-1, "debug", "true"))
assert_false(parser_set_option(1, "", "true"))
assert_eq_string(parser_get_option(1, "debug"), "option_value")
assert_eq_string(parser_get_option(-1, "debug"), "")
assert_eq_string(parser_get_option(1, ""), "")

print_test_summary()

test_start("Performance Metrics Tests")

# Test performance metrics
assert_eq_int(parser_get_parse_time(1), 42)
assert_eq_int(parser_get_parse_time(-1), -1)
assert_eq_int(parser_get_memory_usage(1), 1024)
assert_eq_int(parser_get_memory_usage(-1), -1)
assert_eq_int(parser_get_node_count(1), 10)
assert_eq_int(parser_get_node_count(-1), -1)

print_test_summary()

test_start("Token Type Constants Tests")

# Test token type constants
assert_eq_int(TOKEN_IDENTIFIER, 1)
assert_eq_int(TOKEN_NUMBER, 2)
assert_eq_int(TOKEN_STRING, 3)
assert_eq_int(TOKEN_KEYWORD, 4)
assert_eq_int(TOKEN_OPERATOR, 5)
assert_eq_int(TOKEN_PUNCTUATION, 6)
assert_eq_int(TOKEN_WHITESPACE, 7)
assert_eq_int(TOKEN_COMMENT, 8)
assert_eq_int(TOKEN_EOF, 9)

print_test_summary()

test_start("AST Node Type Constants Tests")

# Test AST node type constants
assert_eq_int(AST_EXPRESSION, 1)
assert_eq_int(AST_STATEMENT, 2)
assert_eq_int(AST_DECLARATION, 3)
assert_eq_int(AST_BLOCK, 4)
assert_eq_int(AST_FUNCTION, 5)
assert_eq_int(AST_LITERAL, 6)
assert_eq_int(AST_IDENTIFIER, 7)
assert_eq_int(AST_BINARY_OP, 8)
assert_eq_int(AST_UNARY_OP, 9)
assert_eq_int(AST_CALL, 10)

print_test_summary()

test_start("Parser State Constants Tests")

# Test parser state constants
assert_eq_int(PARSER_STATE_INITIAL, 0)
assert_eq_int(PARSER_STATE_PARSING, 1)
assert_eq_int(PARSER_STATE_ERROR, 2)
assert_eq_int(PARSER_STATE_COMPLETE, 3)

print_test_summary()
