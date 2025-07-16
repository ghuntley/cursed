# Simple demonstration of ast_mood module functionality
yeet "ast_mood"

# Test basic AST node creation
sus program_node normie = create_program_node(1, 1)
vibez.spill("Created program node: ", string.from_int(program_node))

# Test function node creation
sus func_node normie = create_function_node("main", 2, 1)
vibez.spill("Created function node: ", string.from_int(func_node))

# Test node type checking
lowkey is_program_node(program_node) {
    vibez.spill("✅ Program node type check passed")
}

lowkey is_function_node(func_node) {
    vibez.spill("✅ Function node type check passed")
}

# Test node property extraction
sus node_type normie = ast_node_type(func_node)
sus line normie = ast_node_line(func_node)
sus column normie = ast_node_column(func_node)

vibez.spill("Function node type: ", string.from_int(node_type))
vibez.spill("Function node line: ", string.from_int(line))
vibez.spill("Function node column: ", string.from_int(column))

# Test AST node type string conversion
sus type_str tea = ast_node_type_string(AST_FUNCTION)
vibez.spill("Function type string: ", type_str)

# Test node creation with different types
sus var_node normie = create_variable_node("x", 3, 5)
sus literal_node normie = create_literal_node("42", 4, 10)
sus binary_node normie = create_binary_op_node("+", 5, 15)

# Test expression node detection
lowkey is_expression_node(literal_node) {
    vibez.spill("✅ Literal is expression node")
}

lowkey is_expression_node(binary_node) {
    vibez.spill("✅ Binary op is expression node")
}

# Test module status
sus status tea = ast_mood_status()
vibez.spill("Module status: ", status)

sus version tea = ast_mood_version()
vibez.spill("Module version: ", version)

vibez.spill("🎉 ast_mood module demonstration complete!")
