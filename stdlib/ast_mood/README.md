# ast_mood - AST Manipulation Module

The `ast_mood` module provides comprehensive functionality for creating, manipulating, and analyzing Abstract Syntax Trees (ASTs) in CURSED. This module is essential for compiler development, code analysis tools, and metaprogramming.

## Features

### AST Node Types
- **Core Nodes**: Program, Function, Variable, Expression, Statement
- **Expression Nodes**: Identifier, Literal, Binary Operations, Unary Operations, Function Calls
- **Control Flow**: If, For, While, Return, Block statements
- **Advanced Nodes**: Member Access, Index Access, Tuples, Arrays, Structs, Interfaces
- **Pattern Matching**: Match expressions and patterns
- **Type System**: Type annotations and declarations

### Node Creation
```cursed
# Create basic nodes
sus program normie = create_program_node(1, 1)
sus func normie = create_function_node("myFunc", 2, 5)
sus var normie = create_variable_node("myVar", 3, 10)

# Create expression nodes
sus literal normie = create_literal_node("42", 4, 1)
sus binary_op normie = create_binary_op_node("+", 5, 1)
sus call normie = create_call_node("println", 6, 1)
```

### Node Type Checking
```cursed
# Check node types
lowkey is_function_node(node) {
    vibez.spill("This is a function node")
}

lowkey is_expression_node(node) {
    vibez.spill("This is an expression node")
}

# Pattern matching
lowkey match_function_pattern(node) {
    vibez.spill("Matched function pattern")
}
```

### AST Traversal
```cursed
# Traverse AST
sus node_count normie = count_ast_nodes(root)
sus depth normie = get_ast_depth(root, 0)

# Find specific node types
sus functions normie = find_nodes_by_type(root, AST_FUNCTION, 0)
sus variables normie = find_nodes_by_type(root, AST_VARIABLE, 0)
```

### AST Analysis
```cursed
# Analyze AST structure
sus summary tea = print_ast_summary(root)
vibez.spill(summary)

# Query AST
lowkey has_function_nodes(root) {
    vibez.spill("AST contains functions")
}

sus func_count normie = get_function_count(root)
sus expr_count normie = get_expression_count(root)
```

### AST Transformation
```cursed
# Transform nodes
sus new_node normie = transform_ast_node(old_node, AST_IDENTIFIER)
sus cloned normie = clone_ast_node(original)
```

## AST Node Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `AST_UNKNOWN` | 0 | Unknown node type |
| `AST_PROGRAM` | 1 | Program root node |
| `AST_FUNCTION` | 2 | Function definition |
| `AST_VARIABLE` | 3 | Variable declaration |
| `AST_EXPRESSION` | 4 | General expression |
| `AST_STATEMENT` | 5 | General statement |
| `AST_IDENTIFIER` | 6 | Identifier reference |
| `AST_LITERAL` | 7 | Literal value |
| `AST_BINARY_OP` | 8 | Binary operation |
| `AST_UNARY_OP` | 9 | Unary operation |
| `AST_CALL` | 10 | Function call |
| `AST_BLOCK` | 11 | Code block |
| `AST_IF` | 12 | If statement |
| `AST_FOR` | 13 | For loop |
| `AST_WHILE` | 14 | While loop |
| `AST_RETURN` | 15 | Return statement |
| `AST_ASSIGN` | 16 | Assignment |
| `AST_MEMBER_ACCESS` | 17 | Member access (.) |
| `AST_INDEX_ACCESS` | 18 | Index access ([]) |
| `AST_TUPLE` | 19 | Tuple expression |
| `AST_ARRAY` | 20 | Array literal |
| `AST_STRUCT` | 21 | Struct definition |
| `AST_INTERFACE` | 22 | Interface definition |
| `AST_MATCH` | 23 | Match expression |
| `AST_PATTERN` | 24 | Match pattern |
| `AST_TYPE` | 25 | Type annotation |

## API Reference

### Node Creation Functions
- `create_ast_node(type, name, value, line, column)` - Create generic AST node
- `create_program_node(line, column)` - Create program root node
- `create_function_node(name, line, column)` - Create function node
- `create_variable_node(name, line, column)` - Create variable node
- `create_identifier_node(name, line, column)` - Create identifier node
- `create_literal_node(value, line, column)` - Create literal node
- `create_binary_op_node(operator, line, column)` - Create binary operation
- `create_unary_op_node(operator, line, column)` - Create unary operation
- `create_call_node(function_name, line, column)` - Create function call
- `create_block_node(line, column)` - Create code block
- `create_if_node(line, column)` - Create if statement
- `create_for_node(line, column)` - Create for loop
- `create_while_node(line, column)` - Create while loop
- `create_return_node(line, column)` - Create return statement
- `create_assign_node(variable, line, column)` - Create assignment

### Node Property Functions
- `ast_node_type(node)` - Get node type
- `ast_node_line(node)` - Get source line number
- `ast_node_column(node)` - Get source column number
- `ast_node_type_string(type)` - Convert type to string
- `ast_node_to_string(node)` - Convert node to string

### Type Checking Functions
- `is_program_node(node)` - Check if program node
- `is_function_node(node)` - Check if function node
- `is_variable_node(node)` - Check if variable node
- `is_expression_node(node)` - Check if expression node
- `is_statement_node(node)` - Check if statement node
- `is_literal_node(node)` - Check if literal node
- `is_binary_op_node(node)` - Check if binary operation
- `is_unary_op_node(node)` - Check if unary operation
- `is_call_node(node)` - Check if function call
- `is_block_node(node)` - Check if code block

### Traversal Functions
- `traverse_ast_preorder(root, depth)` - Preorder traversal
- `traverse_ast_postorder(root, depth)` - Postorder traversal
- `count_ast_nodes(root)` - Count total nodes
- `get_ast_depth(root, current_depth)` - Calculate AST depth

### Analysis Functions
- `find_nodes_by_type(root, target_type, depth)` - Find nodes of specific type
- `validate_ast_node(node)` - Validate node structure
- `print_ast_summary(root)` - Generate AST summary

### Pattern Matching Functions
- `match_ast_pattern(node, pattern_type)` - Match general patterns
- `match_function_pattern(node)` - Match function patterns
- `match_variable_pattern(node)` - Match variable patterns
- `match_literal_pattern(node)` - Match literal patterns

### Transformation Functions
- `transform_ast_node(node, new_type)` - Transform node type
- `clone_ast_node(node)` - Clone existing node

### Query Functions
- `has_function_nodes(root)` - Check for function nodes
- `has_variable_nodes(root)` - Check for variable nodes
- `has_expression_nodes(root)` - Check for expression nodes
- `get_function_count(root)` - Count function nodes
- `get_variable_count(root)` - Count variable nodes
- `get_expression_count(root)` - Count expression nodes

### Builder Functions
- `build_simple_program()` - Build basic program AST
- `build_function_with_return(name)` - Build function with return
- `build_variable_assignment(var_name, value)` - Build assignment

### Module Utilities
- `ast_mood_version()` - Get module version
- `ast_mood_status()` - Get module status
- `get_supported_node_types()` - Get number of supported types
- `is_ast_mood_ready()` - Check if module is ready

## Usage Examples

### Building a Simple AST
```cursed
yeet "ast_mood"

# Create program root
sus program normie = create_program_node(1, 1)

# Create function
sus main_func normie = create_function_node("main", 2, 1)

# Create variable
sus var_x normie = create_variable_node("x", 3, 5)

# Create assignment
sus assignment normie = create_assign_node("x", 4, 5)

# Print AST summary
sus summary tea = print_ast_summary(program)
vibez.spill(summary)
```

### AST Analysis and Traversal
```cursed
yeet "ast_mood"

# Analyze AST structure
sus root normie = build_simple_program()
sus depth normie = get_ast_depth(root, 0)
sus node_count normie = count_ast_nodes(root)

vibez.spill("AST depth: ", string.from_int(depth))
vibez.spill("Node count: ", string.from_int(node_count))

# Find specific node types
sus functions normie = find_nodes_by_type(root, AST_FUNCTION, 0)
sus variables normie = find_nodes_by_type(root, AST_VARIABLE, 0)

vibez.spill("Functions: ", string.from_int(functions))
vibez.spill("Variables: ", string.from_int(variables))
```

### Pattern Matching on AST
```cursed
yeet "ast_mood"

slay process_node(node normie) {
    lowkey match_function_pattern(node) {
        vibez.spill("Processing function node")
    }
    lowkey match_variable_pattern(node) {
        vibez.spill("Processing variable node")
    }
    lowkey match_literal_pattern(node) {
        vibez.spill("Processing literal node")
    }
}

# Use with any AST node
sus test_node normie = create_function_node("test", 1, 1)
process_node(test_node)
```

## Implementation Notes

- **Memory Efficient**: Uses integer encoding for node representation
- **Position Tracking**: All nodes track source location (line/column)
- **Type Safe**: Comprehensive type checking and validation
- **Recursive Safe**: Built-in recursion depth limits prevent stack overflow
- **Pattern Matching**: Advanced pattern matching capabilities for AST analysis
- **Pure CURSED**: No external dependencies or FFI calls

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/ast_mood/test_ast_mood.csd
```

Test both interpretation and compilation modes:
```bash
# Interpretation mode
cargo run --bin cursed stdlib/ast_mood/test_ast_mood.csd

# Compilation mode
cargo run --bin cursed -- compile stdlib/ast_mood/test_ast_mood.csd
./test_ast_mood
```

## Version

Current version: 1.0.0

## Status

Production ready - Comprehensive AST manipulation for CURSED compiler development.
