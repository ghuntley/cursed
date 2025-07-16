# Test AST constants and basic functionality
sus AST_UNKNOWN normie = 0
sus AST_PROGRAM normie = 1
sus AST_FUNCTION normie = 2

vibez.spill("AST constants defined successfully")
vibez.spill("AST_UNKNOWN = " + AST_UNKNOWN)
vibez.spill("AST_PROGRAM = " + AST_PROGRAM)
vibez.spill("AST_FUNCTION = " + AST_FUNCTION)

# Test simple node creation simulation
sus node_id normie = 1001
sus node_type normie = 1  # AST_PROGRAM
sus line normie = 1
sus column normie = 1

vibez.spill("Basic AST node simulation complete")
vibez.spill("Node ID: " + node_id)
vibez.spill("Node Type: " + node_type)
vibez.spill("Position: line " + line + ", column " + column)
