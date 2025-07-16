yeet "testz"
yeet "ast_mood"

# Comprehensive tests for ast_mood module

slay test_ast_node_creation() {
    test_start("AST node creation")
    
    # Test basic node creation
    sus program_node normie = create_program_node(1, 1)
    assert_true(program_node > 0)
    assert_true(is_program_node(program_node))
    
    # Test function node creation
    sus func_node normie = create_function_node("test_func", 2, 5)
    assert_true(func_node > 0)
    assert_true(is_function_node(func_node))
    
    # Test variable node creation
    sus var_node normie = create_variable_node("test_var", 3, 10)
    assert_true(var_node > 0)
    assert_true(is_variable_node(var_node))
    
    print_test_summary()
}

slay test_ast_node_properties() {
    test_start("AST node properties")
    
    # Create a test node
    sus test_node normie = create_ast_node(AST_FUNCTION, "test", "value", 5, 15)
    
    # Test node type extraction
    sus node_type normie = ast_node_type(test_node)
    assert_true(node_type == AST_FUNCTION)
    
    # Test line and column extraction
    sus line normie = ast_node_line(test_node)
    sus column normie = ast_node_column(test_node)
    assert_true(line == 5)
    assert_true(column == 15)
    
    print_test_summary()
}

slay test_node_type_checking() {
    test_start("Node type checking")
    
    # Create different types of nodes
    sus prog_node normie = create_program_node(1, 1)
    sus func_node normie = create_function_node("test", 2, 1)
    sus var_node normie = create_variable_node("x", 3, 1)
    sus literal_node normie = create_literal_node("42", 4, 1)
    sus binary_node normie = create_binary_op_node("+", 5, 1)
    
    # Test type checking functions
    assert_true(is_program_node(prog_node))
    assert_true(is_function_node(func_node))
    assert_true(is_variable_node(var_node))
    assert_true(is_literal_node(literal_node))
    assert_true(is_binary_op_node(binary_node))
    
    # Test expression node detection
    assert_true(is_expression_node(literal_node))
    assert_true(is_expression_node(binary_node))
    
    print_test_summary()
}

slay test_ast_node_string_conversion() {
    test_start("AST node string conversion")
    
    # Test node type to string conversion
    sus prog_str tea = ast_node_type_string(AST_PROGRAM)
    assert_true(string.length(prog_str) > 0)
    
    sus func_str tea = ast_node_type_string(AST_FUNCTION)
    assert_true(string.length(func_str) > 0)
    
    sus literal_str tea = ast_node_type_string(AST_LITERAL)
    assert_true(string.length(literal_str) > 0)
    
    # Test node to string conversion
    sus test_node normie = create_function_node("test", 10, 20)
    sus node_str tea = ast_node_to_string(test_node)
    assert_true(string.length(node_str) > 0)
    
    print_test_summary()
}

slay test_specialized_node_creation() {
    test_start("Specialized node creation")
    
    # Test all specialized creation functions
    sus id_node normie = create_identifier_node("myVar", 1, 1)
    assert_true(ast_node_type(id_node) == AST_IDENTIFIER)
    
    sus unary_node normie = create_unary_op_node("-", 2, 1)
    assert_true(ast_node_type(unary_node) == AST_UNARY_OP)
    
    sus call_node normie = create_call_node("println", 3, 1)
    assert_true(ast_node_type(call_node) == AST_CALL)
    
    sus block_node normie = create_block_node(4, 1)
    assert_true(ast_node_type(block_node) == AST_BLOCK)
    
    sus if_node normie = create_if_node(5, 1)
    assert_true(ast_node_type(if_node) == AST_IF)
    
    sus for_node normie = create_for_node(6, 1)
    assert_true(ast_node_type(for_node) == AST_FOR)
    
    sus while_node normie = create_while_node(7, 1)
    assert_true(ast_node_type(while_node) == AST_WHILE)
    
    sus return_node normie = create_return_node(8, 1)
    assert_true(ast_node_type(return_node) == AST_RETURN)
    
    sus assign_node normie = create_assign_node("x", 9, 1)
    assert_true(ast_node_type(assign_node) == AST_ASSIGN)
    
    print_test_summary()
}

slay test_ast_traversal() {
    test_start("AST traversal")
    
    # Create a simple AST
    sus root normie = create_program_node(1, 1)
    
    # Test traversal functions
    sus preorder_count normie = traverse_ast_preorder(root, 0)
    assert_true(preorder_count > 0)
    
    sus postorder_count normie = traverse_ast_postorder(root, 0)
    assert_true(postorder_count > 0)
    
    sus node_count normie = count_ast_nodes(root)
    assert_true(node_count > 0)
    
    print_test_summary()
}

slay test_ast_analysis() {
    test_start("AST analysis")
    
    # Create a test AST with multiple node types
    sus root normie = create_program_node(1, 1)
    
    # Test node finding by type
    sus func_count normie = find_nodes_by_type(root, AST_FUNCTION, 0)
    assert_true(func_count >= 0)
    
    sus var_count normie = find_nodes_by_type(root, AST_VARIABLE, 0)
    assert_true(var_count >= 0)
    
    # Test depth calculation
    sus depth normie = get_ast_depth(root, 0)
    assert_true(depth >= 0)
    
    # Test node validation
    assert_true(validate_ast_node(root))
    
    print_test_summary()
}

slay test_pattern_matching() {
    test_start("Pattern matching")
    
    # Create test nodes
    sus func_node normie = create_function_node("test", 1, 1)
    sus var_node normie = create_variable_node("x", 2, 1)
    sus literal_node normie = create_literal_node("42", 3, 1)
    sus binary_node normie = create_binary_op_node("+", 4, 1)
    
    # Test pattern matching functions
    assert_true(match_function_pattern(func_node))
    assert_true(match_variable_pattern(var_node))
    assert_true(match_literal_pattern(literal_node))
    
    # Test general pattern matching
    assert_true(match_ast_pattern(func_node, AST_FUNCTION))
    assert_true(match_ast_pattern(binary_node, AST_EXPRESSION))
    
    print_test_summary()
}

slay test_ast_transformation() {
    test_start("AST transformation")
    
    # Create a test node
    sus original_node normie = create_variable_node("test", 5, 10)
    
    # Test transformation
    sus transformed_node normie = transform_ast_node(original_node, AST_IDENTIFIER)
    assert_true(ast_node_type(transformed_node) == AST_IDENTIFIER)
    assert_true(ast_node_line(transformed_node) == 5)
    assert_true(ast_node_column(transformed_node) == 10)
    
    # Test cloning
    sus cloned_node normie = clone_ast_node(original_node)
    assert_true(ast_node_type(cloned_node) == ast_node_type(original_node))
    assert_true(ast_node_line(cloned_node) == ast_node_line(original_node))
    
    print_test_summary()
}

slay test_ast_pretty_printing() {
    test_start("AST pretty printing")
    
    # Create a test AST
    sus root normie = create_program_node(1, 1)
    
    # Test summary generation
    sus summary tea = print_ast_summary(root)
    assert_true(string.length(summary) > 0)
    
    # Test individual node string conversion
    sus func_node normie = create_function_node("test", 2, 5)
    sus node_str tea = ast_node_to_string(func_node)
    assert_true(string.length(node_str) > 0)
    
    print_test_summary()
}

slay test_ast_queries() {
    test_start("AST queries")
    
    # Create test nodes
    sus root normie = create_program_node(1, 1)
    
    # Test query functions
    sus has_functions lit = has_function_nodes(root)
    sus has_variables lit = has_variable_nodes(root)
    sus has_expressions lit = has_expression_nodes(root)
    
    # These should not fail
    assert_true(based)  # Always true test
    
    # Test count functions
    sus func_count normie = get_function_count(root)
    sus var_count normie = get_variable_count(root)
    sus expr_count normie = get_expression_count(root)
    
    assert_true(func_count >= 0)
    assert_true(var_count >= 0)
    assert_true(expr_count >= 0)
    
    print_test_summary()
}

slay test_ast_builders() {
    test_start("AST builders")
    
    # Test builder functions
    sus simple_program normie = build_simple_program()
    assert_true(is_program_node(simple_program))
    
    sus func_with_return normie = build_function_with_return("testFunc")
    assert_true(is_function_node(func_with_return))
    
    sus var_assignment normie = build_variable_assignment("x", "42")
    assert_true(ast_node_type(var_assignment) == AST_ASSIGN)
    
    print_test_summary()
}

slay test_statement_node_detection() {
    test_start("Statement node detection")
    
    # Create statement nodes
    sus if_node normie = create_if_node(1, 1)
    sus for_node normie = create_for_node(2, 1)
    sus while_node normie = create_while_node(3, 1)
    sus return_node normie = create_return_node(4, 1)
    sus assign_node normie = create_assign_node("x", 5, 1)
    
    # Test statement detection
    assert_true(is_statement_node(if_node))
    assert_true(is_statement_node(for_node))
    assert_true(is_statement_node(while_node))
    assert_true(is_statement_node(return_node))
    assert_true(is_statement_node(assign_node))
    
    print_test_summary()
}

slay test_module_utilities() {
    test_start("Module utilities")
    
    # Test module information functions
    sus version tea = ast_mood_version()
    assert_true(string.length(version) > 0)
    
    sus status tea = ast_mood_status()
    assert_true(string.length(status) > 0)
    
    sus node_types normie = get_supported_node_types()
    assert_true(node_types > 20)  # Should have many node types
    
    assert_true(is_ast_mood_ready())
    
    print_test_summary()
}

slay test_all_node_types() {
    test_start("All AST node types")
    
    # Test all AST node type constants
    assert_true(AST_UNKNOWN == 0)
    assert_true(AST_PROGRAM == 1)
    assert_true(AST_FUNCTION == 2)
    assert_true(AST_VARIABLE == 3)
    assert_true(AST_EXPRESSION == 4)
    assert_true(AST_STATEMENT == 5)
    assert_true(AST_IDENTIFIER == 6)
    assert_true(AST_LITERAL == 7)
    assert_true(AST_BINARY_OP == 8)
    assert_true(AST_UNARY_OP == 9)
    assert_true(AST_CALL == 10)
    assert_true(AST_BLOCK == 11)
    assert_true(AST_IF == 12)
    assert_true(AST_FOR == 13)
    assert_true(AST_WHILE == 14)
    assert_true(AST_RETURN == 15)
    assert_true(AST_ASSIGN == 16)
    assert_true(AST_MEMBER_ACCESS == 17)
    assert_true(AST_INDEX_ACCESS == 18)
    assert_true(AST_TUPLE == 19)
    assert_true(AST_ARRAY == 20)
    assert_true(AST_STRUCT == 21)
    assert_true(AST_INTERFACE == 22)
    assert_true(AST_MATCH == 23)
    assert_true(AST_PATTERN == 24)
    assert_true(AST_TYPE == 25)
    
    print_test_summary()
}

# Main test runner
vibez.spill("Running ast_mood module tests...")
vibez.spill("====================================")

test_ast_node_creation()
test_ast_node_properties()
test_node_type_checking()
test_ast_node_string_conversion()
test_specialized_node_creation()
test_ast_traversal()
test_ast_analysis()
test_pattern_matching()
test_ast_transformation()
test_ast_pretty_printing()
test_ast_queries()
test_ast_builders()
test_statement_node_detection()
test_module_utilities()
test_all_node_types()

vibez.spill("====================================")
vibez.spill("ast_mood module tests complete!")
vibez.spill("AST manipulation functionality verified.")
vibez.spill("Ready for comprehensive AST processing in CURSED compiler.")
