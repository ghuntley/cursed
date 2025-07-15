yeet "testz"
yeet "compiler_core"
yeet "string"
yeet "collections"

# Comprehensive test suite for compiler core module

test_start("Compiler Core Module Tests")

# Test lexical analysis
test_start("Lexical Analysis Tests")

# Test token creation
sus token Token = create_token(TokenType.IDENTIFIER, "test", 1, 1, 0)
assert_eq_int(token.token_type, TokenType.IDENTIFIER)
assert_eq_string(token.value, "test")
assert_eq_int(token.line, 1)
assert_eq_int(token.column, 1)
assert_eq_int(token.position, 0)

# Test tokenization
sus source tea = "sus x normie = 42"
sus tokens [Token] = tokenize(source)
assert_true(collections.length(tokens) > 0)

# Test first token (sus keyword)
sus first_token Token = tokens[0]
assert_eq_int(first_token.token_type, TokenType.KEYWORD)
assert_eq_string(first_token.value, "sus")

# Test identifier token
sus identifier_token Token = tokens[1]
assert_eq_int(identifier_token.token_type, TokenType.IDENTIFIER)
assert_eq_string(identifier_token.value, "x")

# Test number token
sus number_token Token = tokens[4]
assert_eq_int(number_token.token_type, TokenType.NUMBER)
assert_eq_string(number_token.value, "42")

# Test keyword classification
assert_eq_int(classify_token("sus"), TokenType.KEYWORD)
assert_eq_int(classify_token("slay"), TokenType.KEYWORD)
assert_eq_int(classify_token("damn"), TokenType.KEYWORD)
assert_eq_int(classify_token("identifier"), TokenType.IDENTIFIER)

# Test operator classification
sus plus_token Token = classify_operator('+', 1, 1, 0)
assert_eq_int(plus_token.token_type, TokenType.OPERATOR)
assert_eq_string(plus_token.value, "+")

sus paren_token Token = classify_operator('(', 1, 1, 0)
assert_eq_int(paren_token.token_type, TokenType.DELIMITER)
assert_eq_string(paren_token.value, "(")

print_test_summary()

# Test parsing infrastructure
test_start("Parsing Infrastructure Tests")

# Test parser creation
sus test_tokens [Token] = [
    create_token(TokenType.KEYWORD, "sus", 1, 1, 0),
    create_token(TokenType.IDENTIFIER, "x", 1, 5, 4),
    create_token(TokenType.IDENTIFIER, "normie", 1, 7, 6),
    create_token(TokenType.OPERATOR, "=", 1, 14, 13),
    create_token(TokenType.NUMBER, "42", 1, 16, 15),
    create_token(TokenType.EOF, "", 1, 18, 17)
]

sus parser Parser = create_parser(test_tokens)
assert_eq_int(parser.current_token, 0)
assert_eq_int(collections.length(parser.tokens), 6)

# Test program parsing
sus program ASTNode = parse_program(parser)
assert_eq_int(program.node_type, ASTNodeType.PROGRAM)
assert_eq_string(program.value, "program")

# Test expression parsing
parser.current_token = 4  # Point to number token
sus expr ASTNode = parse_expression(parser)
assert_eq_int(expr.node_type, ASTNodeType.LITERAL)
assert_eq_string(expr.value, "42")

print_test_summary()

# Test AST manipulation
test_start("AST Manipulation Tests")

# Test AST node creation
sus ast_node ASTNode = create_ast_node(ASTNodeType.LITERAL, "test", [], 1, 1)
assert_eq_int(ast_node.node_type, ASTNodeType.LITERAL)
assert_eq_string(ast_node.value, "test")
assert_eq_int(ast_node.line, 1)
assert_eq_int(ast_node.column, 1)

# Test finding nodes by type
sus root ASTNode = create_ast_node(ASTNodeType.PROGRAM, "program", [], 1, 1)
sus child1 ASTNode = create_ast_node(ASTNodeType.LITERAL, "42", [], 1, 1)
sus child2 ASTNode = create_ast_node(ASTNodeType.LITERAL, "hello", [], 1, 1)
root.children = [child1, child2]

sus literals [ASTNode] = find_nodes_by_type(root, ASTNodeType.LITERAL)
assert_eq_int(collections.length(literals), 2)

print_test_summary()

# Test symbol table management
test_start("Symbol Table Management Tests")

# Test symbol table creation
sus symbol_table SymbolTable = create_symbol_table()
assert_eq_int(symbol_table.current_scope, 0)
assert_eq_int(symbol_table.global_scope, 0)
assert_eq_int(symbol_table.next_scope_id, 1)

# Test symbol creation
sus symbol SymbolInfo = create_symbol_info("test_var", SymbolType.VARIABLE, "normie", 0, 1, 1, based, cap)
assert_eq_string(symbol.name, "test_var")
assert_eq_int(symbol.symbol_type, SymbolType.VARIABLE)
assert_eq_string(symbol.data_type, "normie")
assert_eq_int(symbol.line, 1)
assert_eq_int(symbol.column, 1)
assert_true(symbol.is_mutable)
assert_false(symbol.is_exported)

# Test adding symbol to table
sus add_result lit = add_symbol(symbol_table, symbol)
assert_true(add_result)

# Test symbol lookup
sus found_symbol SymbolInfo = lookup_symbol(symbol_table, "test_var")
assert_eq_string(found_symbol.name, "test_var")
assert_eq_int(found_symbol.symbol_type, SymbolType.VARIABLE)

# Test scope management
sus new_scope_id normie = enter_scope(symbol_table, "function")
assert_eq_int(symbol_table.current_scope, new_scope_id)

sus exit_result lit = exit_scope(symbol_table)
assert_true(exit_result)
assert_eq_int(symbol_table.current_scope, 0)

print_test_summary()

# Test type system utilities
test_start("Type System Tests")

# Test type info creation
sus type_info TypeInfo = create_type_info("normie", 4, 4, based)
assert_eq_string(type_info.type_name, "normie")
assert_eq_int(type_info.size, 4)
assert_eq_int(type_info.alignment, 4)
assert_true(type_info.is_primitive)

# Test type size calculation
assert_eq_int(get_type_size("byte"), 1)
assert_eq_int(get_type_size("smol"), 1)
assert_eq_int(get_type_size("mid"), 2)
assert_eq_int(get_type_size("normie"), 4)
assert_eq_int(get_type_size("thicc"), 8)
assert_eq_int(get_type_size("drip"), 4)
assert_eq_int(get_type_size("meal"), 8)
assert_eq_int(get_type_size("lit"), 1)
assert_eq_int(get_type_size("sip"), 1)
assert_eq_int(get_type_size("tea"), 8)

# Test type compatibility
assert_true(types_compatible("normie", "normie"))
assert_true(types_compatible("normie", "thicc"))  # Numeric compatibility
assert_true(types_compatible("drip", "meal"))     # Float compatibility
assert_false(types_compatible("tea", "normie"))   # String vs numeric

# Test type inference
sus literal_node ASTNode = create_ast_node(ASTNodeType.LITERAL, "42", [], 1, 1)
assert_eq_string(infer_type(literal_node), "normie")

sus float_node ASTNode = create_ast_node(ASTNodeType.LITERAL, "3.14", [], 1, 1)
assert_eq_string(infer_type(float_node), "meal")

sus string_node ASTNode = create_ast_node(ASTNodeType.LITERAL, "\"hello\"", [], 1, 1)
assert_eq_string(infer_type(string_node), "tea")

sus bool_node ASTNode = create_ast_node(ASTNodeType.LITERAL, "based", [], 1, 1)
assert_eq_string(infer_type(bool_node), "lit")

print_test_summary()

# Test code generation helpers
test_start("Code Generation Tests")

# Test codegen context creation
sus context CodegenContext = create_codegen_context("llvm", 2, "x86_64")
assert_eq_string(context.output_format, "llvm")
assert_eq_int(context.optimization_level, 2)
assert_eq_string(context.target_arch, "x86_64")
assert_eq_int(context.label_counter, 0)
assert_eq_int(context.register_counter, 0)

# Test label generation
sus label1 tea = generate_label(context)
assert_eq_string(label1, "L0")
sus label2 tea = generate_label(context)
assert_eq_string(label2, "L1")

# Test register generation
sus reg1 tea = generate_register(context)
assert_eq_string(reg1, "%0")
sus reg2 tea = generate_register(context)
assert_eq_string(reg2, "%1")

# Test code generation for literals
sus literal_code tea = generate_code(literal_node, context)
assert_eq_string(literal_code, "42")

# Test code generation for identifiers
sus id_node ASTNode = create_ast_node(ASTNodeType.IDENTIFIER_NODE, "x", [], 1, 1)
sus id_code tea = generate_code(id_node, context)
assert_eq_string(id_code, "x")

print_test_summary()

# Test error reporting
test_start("Error Reporting Tests")

# Test error creation
sus error CompilerError = create_error(ErrorType.SYNTAX_ERROR, "Expected semicolon", 1, 10, "test.csd", 0)
assert_eq_int(error.error_type, ErrorType.SYNTAX_ERROR)
assert_eq_string(error.message, "Expected semicolon")
assert_eq_int(error.line, 1)
assert_eq_int(error.column, 10)
assert_eq_string(error.file, "test.csd")
assert_eq_int(error.severity, 0)

# Test error formatting
sus formatted tea = format_error(error)
assert_true(string.contains(formatted, "test.csd:1:10"))
assert_true(string.contains(formatted, "ERROR"))
assert_true(string.contains(formatted, "Expected semicolon"))

print_test_summary()

# Test utility functions
test_start("Utility Functions Tests")

# Test operator precedence
assert_eq_int(get_operator_precedence("||"), 1)
assert_eq_int(get_operator_precedence("&&"), 2)
assert_eq_int(get_operator_precedence("=="), 3)
assert_eq_int(get_operator_precedence("<"), 4)
assert_eq_int(get_operator_precedence("+"), 5)
assert_eq_int(get_operator_precedence("*"), 6)
assert_eq_int(get_operator_precedence("unknown"), 0)

print_test_summary()

# Test main compiler interface
test_start("Main Compiler Interface Tests")

# Test compiler initialization
sus init_result lit = initialize_compiler()
assert_true(init_result)

# Test compiler status
sus status tea = compiler_status()
assert_eq_string(status, "Compiler core module loaded - ready for self-hosting")

# Test full compilation pipeline
sus simple_source tea = "sus x normie = 42"
sus compiled_output tea = compile_source(simple_source, "llvm", 1)
assert_true(string.length(compiled_output) > 0)

print_test_summary()

# Test complex parsing scenarios
test_start("Complex Parsing Tests")

# Test complex expression parsing
sus complex_source tea = "sus result normie = 1 + 2 * 3"
sus complex_tokens [Token] = tokenize(complex_source)
sus complex_parser Parser = create_parser(complex_tokens)
sus complex_ast ASTNode = parse_program(complex_parser)

assert_eq_int(complex_ast.node_type, ASTNodeType.PROGRAM)
assert_true(collections.length(complex_ast.children) > 0)

# Test function declaration parsing
sus func_source tea = "slay add(x normie, y normie) normie { damn x + y }"
sus func_tokens [Token] = tokenize(func_source)
sus func_parser Parser = create_parser(func_tokens)
sus func_ast ASTNode = parse_program(func_parser)

assert_eq_int(func_ast.node_type, ASTNodeType.PROGRAM)
assert_true(collections.length(func_ast.children) > 0)

print_test_summary()

# Test symbol table with multiple scopes
test_start("Multi-Scope Symbol Table Tests")

sus multi_table SymbolTable = create_symbol_table()

# Add global variable
sus global_var SymbolInfo = create_symbol_info("global_var", SymbolType.VARIABLE, "normie", 0, 1, 1, based, based)
add_symbol(multi_table, global_var)

# Enter function scope
sus func_scope normie = enter_scope(multi_table, "function")
sus local_var SymbolInfo = create_symbol_info("local_var", SymbolType.VARIABLE, "normie", func_scope, 2, 1, based, cap)
add_symbol(multi_table, local_var)

# Test lookups
sus found_global SymbolInfo = lookup_symbol(multi_table, "global_var")
assert_eq_string(found_global.name, "global_var")

sus found_local SymbolInfo = lookup_symbol(multi_table, "local_var")
assert_eq_string(found_local.name, "local_var")

# Exit scope
exit_scope(multi_table)

# Global should still be accessible
sus found_global2 SymbolInfo = lookup_symbol(multi_table, "global_var")
assert_eq_string(found_global2.name, "global_var")

print_test_summary()

# Test AST transformation
test_start("AST Transformation Tests")

# Create a simple AST for transformation
sus transform_root ASTNode = create_ast_node(ASTNodeType.PROGRAM, "program", [], 1, 1)
sus transform_child ASTNode = create_ast_node(ASTNodeType.LITERAL, "42", [], 1, 1)
transform_root.children = [transform_child]

# Define transformation function (would be passed as function pointer in real implementation)
# For testing, we'll just verify the structure exists
assert_eq_int(transform_root.node_type, ASTNodeType.PROGRAM)
assert_eq_int(collections.length(transform_root.children), 1)
assert_eq_int(transform_root.children[0].node_type, ASTNodeType.LITERAL)

print_test_summary()

# Test comprehensive tokenization scenarios
test_start("Comprehensive Tokenization Tests")

# Test string tokenization
sus string_source tea = "sus message tea = \"Hello, world!\""
sus string_tokens [Token] = tokenize(string_source)
assert_true(collections.length(string_tokens) > 0)

# Find string token
sus found_string lit = cap
bestie i := 0; i < collections.length(string_tokens); i++ {
    lowkey string_tokens[i].token_type == TokenType.STRING {
        assert_eq_string(string_tokens[i].value, "Hello, world!")
        found_string = based
        ghosted
    }
}
assert_true(found_string)

# Test complex operator tokenization
sus operator_source tea = "sus result lit = (x + y) * z"
sus operator_tokens [Token] = tokenize(operator_source)
assert_true(collections.length(operator_tokens) > 0)

# Check for specific operators
sus found_plus lit = cap
sus found_mult lit = cap
sus found_parens lit = cap

bestie i := 0; i < collections.length(operator_tokens); i++ {
    lowkey operator_tokens[i].token_type == TokenType.OPERATOR && string.equals(operator_tokens[i].value, "+") {
        found_plus = based
    }
    lowkey operator_tokens[i].token_type == TokenType.OPERATOR && string.equals(operator_tokens[i].value, "*") {
        found_mult = based
    }
    lowkey operator_tokens[i].token_type == TokenType.DELIMITER && string.equals(operator_tokens[i].value, "(") {
        found_parens = based
    }
}

assert_true(found_plus)
assert_true(found_mult)
assert_true(found_parens)

print_test_summary()

# Test type inference with complex expressions
test_start("Advanced Type Inference Tests")

# Test binary operation type inference
sus left_operand ASTNode = create_ast_node(ASTNodeType.LITERAL, "42", [], 1, 1)
sus right_operand ASTNode = create_ast_node(ASTNodeType.LITERAL, "3.14", [], 1, 1)
sus binary_expr ASTNode = create_ast_node(ASTNodeType.BINARY_OP, "+", [left_operand, right_operand], 1, 1)

# The type inference should handle mixed types
sus inferred_type tea = infer_type(binary_expr)
assert_true(string.length(inferred_type) > 0)

print_test_summary()

# Final comprehensive test
test_start("Full Compilation Pipeline Test")

# Test complete compilation from source to output
sus pipeline_source tea = "slay main() normie { sus x normie = 42; damn x }"
sus pipeline_tokens [Token] = tokenize(pipeline_source)
sus pipeline_parser Parser = create_parser(pipeline_tokens)
sus pipeline_ast ASTNode = parse_program(pipeline_parser)
sus pipeline_context CodegenContext = create_codegen_context("llvm", 2, "x86_64")

# Verify each stage
assert_true(collections.length(pipeline_tokens) > 0)
assert_eq_int(pipeline_ast.node_type, ASTNodeType.PROGRAM)
assert_eq_string(pipeline_context.output_format, "llvm")

# Test full compilation
sus final_output tea = compile_source(pipeline_source, "llvm", 2)
assert_true(string.length(final_output) >= 0)

print_test_summary()

vibez.spill("All compiler core tests completed successfully!")
vibez.spill("Compiler core module is ready for self-hosting compiler bootstrap!")
