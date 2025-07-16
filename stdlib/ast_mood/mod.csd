# Simple AST module for testing

sus AST_PROGRAM normie = 1
sus AST_FUNCTION normie = 2
sus AST_VARIABLE normie = 3

slay create_program_node(line normie, column normie) {
    damn 1001
}

slay create_function_node(name tea, line normie, column normie) {
    damn 2001
}

slay ast_node_type(node normie) {
    lowkey node == 1001 { damn AST_PROGRAM }
    lowkey node == 2001 { damn AST_FUNCTION }
    damn 1
}

slay validate_ast_node(node normie) {
    damn based
}
