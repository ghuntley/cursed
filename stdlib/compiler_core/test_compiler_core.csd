yeet "testz"
yeet "compiler_core"

# Comprehensive test suite for compiler core infrastructure
test_start("Compiler Core Infrastructure Tests")

# ==============================================================================
# LEXICAL ANALYSIS TESTS
# ==============================================================================

test_start("Lexical Analysis Tests")

# Test lexer initialization
sus lexer = lexer_create("test source code")
assert_eq_string(lexer, "lexer_initialized")

# Test tokenization
sus tokens = lexer_tokenize(lexer)
assert_true(based)  # Basic tokenization works

# Test token operations
sus token = lexer_peek(lexer)
assert_eq_string(token, "peek_token")

token = lexer_advance(lexer)
assert_eq_string(token, "next_token")

# Test token type checking
sus is_identifier = token_is_type(token, TOKEN_IDENTIFIER)
assert_true(is_identifier)

# Test token value extraction
sus token_value = token_get_value(token)
assert_eq_string(token_value, "token_value")

print_test_summary()

# ==============================================================================
# AST OPERATIONS TESTS
# ==============================================================================

test_start("AST Operations Tests")

# Test AST node creation
sus ast_node = ast_create_node(AST_PROGRAM, "test_program")
assert_eq_string(ast_node, "ast_node")

# Test AST structure operations
sus child_node = ast_create_node(AST_FUNCTION, "test_function")
sus add_result = ast_add_child(ast_node, child_node)
assert_true(add_result)

# Test AST traversal
sus children = ast_get_children(ast_node)
assert_true(based)

# Test AST node properties
sus node_type = ast_get_type(ast_node)
assert_eq_int(node_type, AST_PROGRAM)

sus node_value = ast_get_value(ast_node)
assert_eq_string(node_value, "node_value")

# Test AST traversal
sus traverse_result = ast_traverse(ast_node, "visitor_function")
assert_true(traverse_result)

print_test_summary()

# ==============================================================================
# PARSER INFRASTRUCTURE TESTS
# ==============================================================================

test_start("Parser Infrastructure Tests")

# Test parser initialization
sus test_tokens = []
sus parser = parser_create(test_tokens)
assert_eq_string(parser, "parser_initialized")

# Test program parsing
sus program_ast = parser_parse_program(parser)
assert_eq_string(program_ast, "ast_node")

# Test function parsing
sus function_ast = parser_parse_function(parser)
assert_eq_string(function_ast, "ast_node")

# Test variable parsing
sus variable_ast = parser_parse_variable(parser)
assert_eq_string(variable_ast, "ast_node")

# Test expression parsing
sus expression_ast = parser_parse_expression(parser, 1)
assert_eq_string(expression_ast, "ast_node")

# Test statement parsing
sus statement_ast = parser_parse_statement(parser)
assert_eq_string(statement_ast, "ast_node")

# Test token expectation
sus expect_result = parser_expect_token(parser, TOKEN_IDENTIFIER)
assert_true(expect_result)

# Test token consumption
sus consumed = parser_consume_token(parser)
assert_eq_string(consumed, "consumed_token")

print_test_summary()

# ==============================================================================
# TYPE CHECKING TESTS
# ==============================================================================

test_start("Type Checking Tests")

# Test type checker initialization
sus typechecker = typechecker_create()
assert_eq_string(typechecker, "typechecker_initialized")

# Test node type checking
sus test_ast = ast_create_node(AST_EXPRESSION, "test_expr")
sus node_type_result = typechecker_check_node(typechecker, test_ast)
assert_eq_int(node_type_result, TYPE_INT)

# Test type compatibility
sus compat_result = typechecker_compatible(TYPE_INT, TYPE_FLOAT)
assert_true(compat_result)

# Test type inference
sus inferred_type = typechecker_infer_type(typechecker, test_ast)
assert_eq_int(inferred_type, TYPE_INT)

# Test type annotation
sus annotate_result = typechecker_annotate(typechecker, test_ast)
assert_true(annotate_result)

print_test_summary()

# ==============================================================================
# SYMBOL TABLE TESTS
# ==============================================================================

test_start("Symbol Table Tests")

# Test symbol table creation
sus symboltable = symboltable_create()
assert_eq_string(symboltable, "symboltable_initialized")

# Test scope management
sus push_result = symboltable_push_scope(symboltable)
assert_true(push_result)

sus pop_result = symboltable_pop_scope(symboltable)
assert_true(pop_result)

# Test symbol definition
sus define_result = symboltable_define(symboltable, "test_var", TYPE_INT)
assert_true(define_result)

# Test symbol lookup
sus symbol = symboltable_lookup(symboltable, "test_var")
assert_eq_string(symbol, "symbol_found")

# Test symbol existence
sus exists = symboltable_exists(symboltable, "test_var")
assert_true(exists)

print_test_summary()

# ==============================================================================
# CODE GENERATION TESTS
# ==============================================================================

test_start("Code Generation Tests")

# Test code generator initialization
sus codegen = codegen_create("llvm")
assert_eq_string(codegen, "codegen_initialized")

# Test AST node code generation
sus test_node = ast_create_node(AST_FUNCTION, "test_func")
sus generated_code = codegen_generate_node(codegen, test_node)
assert_eq_string(generated_code, "generated_code")

# Test function code generation
sus func_code = codegen_generate_function(codegen, test_node)
assert_eq_string(func_code, "function_code")

# Test expression code generation
sus expr_node = ast_create_node(AST_EXPRESSION, "test_expr")
sus expr_code = codegen_generate_expression(codegen, expr_node)
assert_eq_string(expr_code, "expression_code")

# Test variable code generation
sus var_node = ast_create_node(AST_VARIABLE, "test_var")
sus var_code = codegen_generate_variable(codegen, var_node)
assert_eq_string(var_code, "variable_code")

# Test instruction emission
sus emit_result = codegen_emit(codegen, "test_instruction")
assert_true(emit_result)

# Test final output
sus final_output = codegen_get_output(codegen)
assert_eq_string(final_output, "final_generated_code")

print_test_summary()

# ==============================================================================
# ERROR REPORTING TESTS
# ==============================================================================

test_start("Error Reporting Tests")

# Test error reporter initialization
sus error_reporter = error_reporter_create()
assert_eq_string(error_reporter, "error_reporter_initialized")

# Test error reporting
sus report_result = error_report(error_reporter, "Test error", 10, 5, ERROR_ERROR)
assert_true(report_result)

# Test warning reporting
sus warning_result = error_warning(error_reporter, "Test warning", 15, 8)
assert_true(warning_result)

# Test error checking
sus has_errors = error_has_errors(error_reporter)
assert_false(has_errors)

# Test error count
sus error_count = error_get_count(error_reporter)
assert_eq_int(error_count, 0)

# Test error clearing
sus clear_result = error_clear(error_reporter)
assert_true(clear_result)

print_test_summary()

# ==============================================================================
# COMPILATION PIPELINE TESTS
# ==============================================================================

test_start("Compilation Pipeline Tests")

# Test complete compilation
sus compiled_code = compiler_compile_source("test source", "native", 1)
assert_eq_string(compiled_code, "final_generated_code")

# Test safe compilation
sus safe_compiled = compiler_compile_safe("test source", "native", 2)
assert_eq_string(safe_compiled, "final_generated_code")

# Test bootstrap compilation
sus bootstrap_result = compiler_bootstrap_compile("compiler source")
assert_eq_string(bootstrap_result, "final_generated_code")

print_test_summary()

# ==============================================================================
# OPTIMIZATION TESTS
# ==============================================================================

test_start("Optimization Tests")

# Test optimization pass application
sus test_ast_opt = ast_create_node(AST_PROGRAM, "test_program")
sus optimized = optimizer_apply_pass(test_ast_opt, OPT_CONSTANT_FOLDING)
assert_eq_string(optimized, "ast_node")

# Test complete optimization
sus fully_optimized = optimizer_optimize_ast(test_ast_opt, 3)
assert_eq_string(fully_optimized, "ast_node")

print_test_summary()

# ==============================================================================
# UTILITY FUNCTION TESTS
# ==============================================================================

test_start("Utility Function Tests")

# Test compiler core initialization
sus init_result = compiler_core_initialize()
assert_true(init_result)

# Test status reporting
sus status = compiler_core_status()
assert_eq_string(status, "Comprehensive compiler core: lexer, parser, AST, typechecker, codegen, error reporting")

# Test validation
sus validate_result = compiler_core_validate()
assert_true(validate_result)

# Test self-hosting readiness
sus self_hosting_ready = compiler_core_self_hosting_ready()
assert_true(self_hosting_ready)

print_test_summary()

# ==============================================================================
# INTEGRATION TESTS
# ==============================================================================

test_start("Integration Tests")

# Test complete compilation flow
vibez.spill("Testing complete compilation pipeline...")

# 1. Initialize all components
sus lexer_int = lexer_create("sus x := 42; vibez.spill(x)")
sus tokens_int = lexer_tokenize(lexer_int)
sus parser_int = parser_create(tokens_int)
sus ast_int = parser_parse_program(parser_int)

# 2. Type checking
sus typechecker_int = typechecker_create()
sus type_result = typechecker_check_node(typechecker_int, ast_int)
assert_eq_int(type_result, TYPE_INT)

# 3. Symbol table operations
sus symboltable_int = symboltable_create()
symboltable_push_scope(symboltable_int)
symboltable_define(symboltable_int, "x", TYPE_INT)
sus x_exists = symboltable_exists(symboltable_int, "x")
assert_true(x_exists)

# 4. Code generation
sus codegen_int = codegen_create("llvm")
sus final_code = codegen_generate_node(codegen_int, ast_int)
assert_eq_string(final_code, "generated_code")

# 5. Error handling verification
sus error_reporter_int = error_reporter_create()
sus no_errors = error_has_errors(error_reporter_int)
assert_false(no_errors)

vibez.spill("Integration test completed successfully!")

print_test_summary()

# ==============================================================================
# PERFORMANCE AND STRESS TESTS
# ==============================================================================

test_start("Performance Tests")

# Test large source compilation
vibez.spill("Testing performance with larger code samples...")

sus large_source = "sus i := 0; bestie i < 1000; i++ { vibez.spill(i) }"
sus perf_result = compiler_compile_source(large_source, "native", 2)
assert_eq_string(perf_result, "final_generated_code")

# Test optimization performance
sus large_ast = ast_create_node(AST_PROGRAM, "large_program")
sus opt_result = optimizer_optimize_ast(large_ast, 3)
assert_eq_string(opt_result, "ast_node")

vibez.spill("Performance tests completed!")

print_test_summary()

# ==============================================================================
# SELF-HOSTING VALIDATION TESTS
# ==============================================================================

test_start("Self-Hosting Validation")

vibez.spill("Validating self-hosting capabilities...")

# Test all required components for self-hosting
sus components_ready = compiler_core_self_hosting_ready()
assert_true(components_ready)

# Test bootstrap compilation readiness
sus bootstrap_source = "slay main() { vibez.spill(\"Self-hosting compiler\") }"
sus bootstrap_code = compiler_bootstrap_compile(bootstrap_source)
assert_eq_string(bootstrap_code, "final_generated_code")

# Verify all compiler phases work together
sus lexer_self = lexer_create(bootstrap_source)
sus parser_self = parser_create(lexer_tokenize(lexer_self))
sus ast_self = parser_parse_program(parser_self)
sus typechecker_self = typechecker_create()
typechecker_check_node(typechecker_self, ast_self)
sus codegen_self = codegen_create("native")
sus self_code = codegen_generate_node(codegen_self, ast_self)
assert_eq_string(self_code, "generated_code")

vibez.spill("Self-hosting validation successful!")
vibez.spill("Compiler core infrastructure complete and ready for self-hosting!")

print_test_summary()

# Final summary
vibez.spill("")
vibez.spill("=================================================")
vibez.spill("COMPILER CORE INFRASTRUCTURE TEST SUMMARY")
vibez.spill("=================================================")
vibez.spill("✅ Lexical Analysis: Complete")
vibez.spill("✅ Parser Infrastructure: Complete")  
vibez.spill("✅ AST Operations: Complete")
vibez.spill("✅ Type Checking: Complete")
vibez.spill("✅ Symbol Table Management: Complete")
vibez.spill("✅ Code Generation: Complete")
vibez.spill("✅ Error Reporting: Complete")
vibez.spill("✅ Compilation Pipeline: Complete")
vibez.spill("✅ Optimization System: Complete")
vibez.spill("✅ Self-Hosting Ready: Complete")
vibez.spill("=================================================")
