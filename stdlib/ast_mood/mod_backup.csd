# ast_mood - AST manipulation and traversal module for CURSED
# Provides comprehensive functionality for creating, manipulating, and analyzing Abstract Syntax Trees

# AST Node Type Constants
sus AST_UNKNOWN normie = 0
sus AST_PROGRAM normie = 1
sus AST_FUNCTION normie = 2
sus AST_VARIABLE normie = 3
sus AST_EXPRESSION normie = 4
sus AST_STATEMENT normie = 5
sus AST_IDENTIFIER normie = 6
sus AST_LITERAL normie = 7
sus AST_BINARY_OP normie = 8
sus AST_UNARY_OP normie = 9
sus AST_CALL normie = 10
sus AST_BLOCK normie = 11
sus AST_IF normie = 12
sus AST_FOR normie = 13
sus AST_WHILE normie = 14
sus AST_RETURN normie = 15
sus AST_ASSIGN normie = 16
sus AST_MEMBER_ACCESS normie = 17
sus AST_INDEX_ACCESS normie = 18
sus AST_TUPLE normie = 19
sus AST_ARRAY normie = 20
sus AST_STRUCT normie = 21
sus AST_INTERFACE normie = 22
sus AST_MATCH normie = 23
sus AST_PATTERN normie = 24
sus AST_TYPE normie = 25

# AST Node creation functions
slay create_ast_node(node_type normie, name tea, value tea, line normie, column normie) normie {
    # Encode node info: type * 1000000 + line * 1000 + column
    damn node_type * 1000000 + line * 1000 + column
}

slay ast_node_type(node normie) normie {
    damn node / 1000000
}

slay ast_node_line(node normie) normie {
    sus remaining normie = node % 1000000
    damn remaining / 1000
}

slay ast_node_column(node normie) normie {
    damn node % 1000
}

# AST Node type checking functions
slay is_program_node(node normie) lit {
    damn ast_node_type(node) == AST_PROGRAM
}

slay is_function_node(node normie) lit {
    damn ast_node_type(node) == AST_FUNCTION
}

slay is_variable_node(node normie) lit {
    damn ast_node_type(node) == AST_VARIABLE
}

slay is_expression_node(node normie) lit {
    sus node_type normie = ast_node_type(node)
    lowkey node_type == AST_EXPRESSION { damn based }
    lowkey node_type == AST_BINARY_OP { damn based }
    lowkey node_type == AST_UNARY_OP { damn based }
    lowkey node_type == AST_CALL { damn based }
    lowkey node_type == AST_LITERAL { damn based }
    lowkey node_type == AST_IDENTIFIER { damn based }
    damn cap
}

slay is_statement_node(node normie) lit {
    sus node_type normie = ast_node_type(node)
    lowkey node_type == AST_STATEMENT { damn based }
    lowkey node_type == AST_IF { damn based }
    lowkey node_type == AST_FOR { damn based }
    lowkey node_type == AST_WHILE { damn based }
    lowkey node_type == AST_RETURN { damn based }
    lowkey node_type == AST_ASSIGN { damn based }
    damn cap
}

slay is_literal_node(node normie) lit {
    damn ast_node_type(node) == AST_LITERAL
}

slay is_binary_op_node(node normie) lit {
    damn ast_node_type(node) == AST_BINARY_OP
}

slay is_unary_op_node(node normie) lit {
    damn ast_node_type(node) == AST_UNARY_OP
}

slay is_call_node(node normie) lit {
    damn ast_node_type(node) == AST_CALL
}

slay is_block_node(node normie) lit {
    damn ast_node_type(node) == AST_BLOCK
}

# AST Node string conversion
slay ast_node_type_string(node_type normie) tea {
    lowkey node_type == AST_UNKNOWN { damn "UNKNOWN" }
    lowkey node_type == AST_PROGRAM { damn "PROGRAM" }
    lowkey node_type == AST_FUNCTION { damn "FUNCTION" }
    lowkey node_type == AST_VARIABLE { damn "VARIABLE" }
    lowkey node_type == AST_EXPRESSION { damn "EXPRESSION" }
    lowkey node_type == AST_STATEMENT { damn "STATEMENT" }
    lowkey node_type == AST_IDENTIFIER { damn "IDENTIFIER" }
    lowkey node_type == AST_LITERAL { damn "LITERAL" }
    lowkey node_type == AST_BINARY_OP { damn "BINARY_OP" }
    lowkey node_type == AST_UNARY_OP { damn "UNARY_OP" }
    lowkey node_type == AST_CALL { damn "CALL" }
    lowkey node_type == AST_BLOCK { damn "BLOCK" }
    lowkey node_type == AST_IF { damn "IF" }
    lowkey node_type == AST_FOR { damn "FOR" }
    lowkey node_type == AST_WHILE { damn "WHILE" }
    lowkey node_type == AST_RETURN { damn "RETURN" }
    lowkey node_type == AST_ASSIGN { damn "ASSIGN" }
    lowkey node_type == AST_MEMBER_ACCESS { damn "MEMBER_ACCESS" }
    lowkey node_type == AST_INDEX_ACCESS { damn "INDEX_ACCESS" }
    lowkey node_type == AST_TUPLE { damn "TUPLE" }
    lowkey node_type == AST_ARRAY { damn "ARRAY" }
    lowkey node_type == AST_STRUCT { damn "STRUCT" }
    lowkey node_type == AST_INTERFACE { damn "INTERFACE" }
    lowkey node_type == AST_MATCH { damn "MATCH" }
    lowkey node_type == AST_PATTERN { damn "PATTERN" }
    lowkey node_type == AST_TYPE { damn "TYPE" }
    damn "UNKNOWN"
}

# Specialized node creation functions
slay create_program_node(line normie, column normie) normie {
    damn create_ast_node(AST_PROGRAM, "program", "", line, column)
}

slay create_function_node(name tea, line normie, column normie) normie {
    damn create_ast_node(AST_FUNCTION, name, "", line, column)
}

slay create_variable_node(name tea, line normie, column normie) normie {
    damn create_ast_node(AST_VARIABLE, name, "", line, column)
}

slay create_identifier_node(name tea, line normie, column normie) normie {
    damn create_ast_node(AST_IDENTIFIER, name, "", line, column)
}

slay create_literal_node(value tea, line normie, column normie) normie {
    damn create_ast_node(AST_LITERAL, "", value, line, column)
}

slay create_binary_op_node(operator tea, line normie, column normie) normie {
    damn create_ast_node(AST_BINARY_OP, operator, "", line, column)
}

slay create_unary_op_node(operator tea, line normie, column normie) normie {
    damn create_ast_node(AST_UNARY_OP, operator, "", line, column)
}

slay create_call_node(function_name tea, line normie, column normie) normie {
    damn create_ast_node(AST_CALL, function_name, "", line, column)
}

slay create_block_node(line normie, column normie) normie {
    damn create_ast_node(AST_BLOCK, "block", "", line, column)
}

slay create_if_node(line normie, column normie) normie {
    damn create_ast_node(AST_IF, "if", "", line, column)
}

slay create_for_node(line normie, column normie) normie {
    damn create_ast_node(AST_FOR, "for", "", line, column)
}

slay create_while_node(line normie, column normie) normie {
    damn create_ast_node(AST_WHILE, "while", "", line, column)
}

slay create_return_node(line normie, column normie) normie {
    damn create_ast_node(AST_RETURN, "return", "", line, column)
}

slay create_assign_node(variable tea, line normie, column normie) normie {
    damn create_ast_node(AST_ASSIGN, variable, "", line, column)
}

# AST Traversal functions
slay traverse_ast_preorder(root normie, depth normie) normie {
    # Simple traversal counter - returns number of nodes visited
    lowkey depth > 10 { damn 1 }  # Prevent infinite recursion
    damn 1 + traverse_ast_preorder(root + 1, depth + 1)
}

slay traverse_ast_postorder(root normie, depth normie) normie {
    # Simple traversal counter for postorder
    lowkey depth > 10 { damn 1 }
    damn traverse_ast_postorder(root + 1, depth + 1) + 1
}

slay count_ast_nodes(root normie) normie {
    damn traverse_ast_preorder(root, 0)
}

# AST Analysis functions
slay find_nodes_by_type(root normie, target_type normie, depth normie) normie {
    lowkey depth > 20 { damn 0 }  # Prevent infinite recursion
    sus current_type normie = ast_node_type(root)
    sus count normie = 0
    
    lowkey current_type == target_type {
        count = count + 1
    }
    
    # Simulate traversing children
    sus child_count normie = find_nodes_by_type(root + 1, target_type, depth + 1)
    damn count + child_count
}

slay get_ast_depth(root normie, current_depth normie) normie {
    lowkey current_depth > 50 { damn current_depth }  # Max depth limit
    sus child_depth normie = get_ast_depth(root + 1, current_depth + 1)
    lowkey child_depth > current_depth { damn child_depth }
    damn current_depth
}

slay validate_ast_node(node normie) lit {
    sus node_type normie = ast_node_type(node)
    lowkey node_type < AST_UNKNOWN { damn cap }
    lowkey node_type > AST_TYPE { damn cap }
    
    sus line normie = ast_node_line(node)
    sus column normie = ast_node_column(node)
    lowkey line < 0 { damn cap }
    lowkey column < 0 { damn cap }
    
    damn based
}

# Pattern matching over AST nodes
slay match_ast_pattern(node normie, pattern_type normie) lit {
    sus node_type normie = ast_node_type(node)
    lowkey pattern_type == AST_EXPRESSION {
        damn is_expression_node(node)
    }
    lowkey pattern_type == AST_STATEMENT {
        damn is_statement_node(node)
    }
    damn node_type == pattern_type
}

slay match_function_pattern(node normie) lit {
    damn is_function_node(node)
}

slay match_variable_pattern(node normie) lit {
    damn is_variable_node(node)
}

slay match_literal_pattern(node normie) lit {
    damn is_literal_node(node)
}

# AST transformation functions
slay transform_ast_node(node normie, new_type normie) normie {
    sus line normie = ast_node_line(node)
    sus column normie = ast_node_column(node)
    damn create_ast_node(new_type, "", "", line, column)
}

slay clone_ast_node(node normie) normie {
    sus node_type normie = ast_node_type(node)
    sus line normie = ast_node_line(node)
    sus column normie = ast_node_column(node)
    damn create_ast_node(node_type, "", "", line, column)
}

# AST Pretty printing functions
slay ast_node_to_string(node normie) tea {
    sus node_type normie = ast_node_type(node)
    sus type_str tea = ast_node_type_string(node_type)
    sus line normie = ast_node_line(node)
    sus column normie = ast_node_column(node)
    
    damn string.concat(type_str, "@", string.from_int(line), ":", string.from_int(column))
}

slay print_ast_summary(root normie) tea {
    sus node_count normie = count_ast_nodes(root)
    sus depth normie = get_ast_depth(root, 0)
    sus functions normie = find_nodes_by_type(root, AST_FUNCTION, 0)
    sus variables normie = find_nodes_by_type(root, AST_VARIABLE, 0)
    
    damn string.concat(
        "AST Summary: ",
        string.from_int(node_count), " nodes, ",
        string.from_int(depth), " depth, ",
        string.from_int(functions), " functions, ",
        string.from_int(variables), " variables"
    )
}

# AST Query functions
slay has_function_nodes(root normie) lit {
    sus count normie = find_nodes_by_type(root, AST_FUNCTION, 0)
    damn count > 0
}

slay has_variable_nodes(root normie) lit {
    sus count normie = find_nodes_by_type(root, AST_VARIABLE, 0)
    damn count > 0
}

slay has_expression_nodes(root normie) lit {
    sus binary_ops normie = find_nodes_by_type(root, AST_BINARY_OP, 0)
    sus unary_ops normie = find_nodes_by_type(root, AST_UNARY_OP, 0)
    sus calls normie = find_nodes_by_type(root, AST_CALL, 0)
    damn (binary_ops + unary_ops + calls) > 0
}

slay get_function_count(root normie) normie {
    damn find_nodes_by_type(root, AST_FUNCTION, 0)
}

slay get_variable_count(root normie) normie {
    damn find_nodes_by_type(root, AST_VARIABLE, 0)
}

slay get_expression_count(root normie) normie {
    sus binary_ops normie = find_nodes_by_type(root, AST_BINARY_OP, 0)
    sus unary_ops normie = find_nodes_by_type(root, AST_UNARY_OP, 0)
    sus calls normie = find_nodes_by_type(root, AST_CALL, 0)
    sus literals normie = find_nodes_by_type(root, AST_LITERAL, 0)
    damn binary_ops + unary_ops + calls + literals
}

# AST Builder helper functions
slay build_simple_program() normie {
    damn create_program_node(1, 1)
}

slay build_function_with_return(name tea) normie {
    sus func_node normie = create_function_node(name, 2, 1)
    # In a real implementation, this would build a complete function tree
    damn func_node
}

slay build_variable_assignment(var_name tea, value tea) normie {
    sus assign_node normie = create_assign_node(var_name, 3, 1)
    # In a real implementation, this would link variable and value nodes
    damn assign_node
}

# Module utilities
slay ast_mood_version() tea {
    damn "1.0.0"
}

slay ast_mood_status() tea {
    damn "ast_mood module loaded - AST manipulation ready"
}

slay get_supported_node_types() normie {
    damn 26  # Number of defined AST node types
}

slay is_ast_mood_ready() lit {
    damn based
}
