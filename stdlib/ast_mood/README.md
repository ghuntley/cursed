# ast_mood - Advanced AST Manipulation Module

The `ast_mood` module provides comprehensive Abstract Syntax Tree (AST) manipulation utilities for the CURSED compiler infrastructure. This module implements a complete suite of AST operations in pure CURSED without any FFI dependencies.

## Overview

The `ast_mood` module is essential for compiler passes, code analysis tools, and AST transformations. It provides a robust foundation for building sophisticated compiler infrastructure components.

## Features

### 🌳 Core AST Node Creation
- **26 AST Node Types**: Complete coverage of language constructs
- **Unique Node IDs**: Each node gets a globally unique identifier
- **Position Tracking**: Line and column information for all nodes
- **Type Safety**: Strong typing for all AST operations

### 🔍 Tree Traversal Utilities
- **Preorder Traversal**: Visit nodes before their children
- **Postorder Traversal**: Visit nodes after their children
- **Depth Calculation**: Compute maximum tree depth
- **Node Counting**: Count nodes by type or total

### 🔄 AST Transformation Operations
- **Node Transformation**: Convert nodes between types
- **Cloning**: Deep copy of AST nodes
- **Replacement**: Replace nodes matching patterns
- **Optimization**: Structure optimization utilities

### 🎨 Pretty Printing and Debugging
- **Node String Conversion**: Human-readable node representations
- **AST Summary**: High-level tree statistics
- **Debug Details**: Detailed node information
- **Type String Mapping**: Convert type constants to strings

### ✅ AST Validation and Integrity
- **Node Validation**: Check individual node integrity
- **Tree Validation**: Validate entire AST structure
- **Consistency Checking**: Type-specific validation rules
- **Error Detection**: Identify malformed AST structures

### 💾 Serialization/Deserialization
- **JSON Export**: Convert AST to JSON format
- **JSON Import**: Reconstruct AST from JSON
- **Node Serialization**: Serialize individual nodes
- **Compact Representation**: Memory-efficient formats

## AST Node Types

The module supports 26 different AST node types:

| Type | ID | Description |
|------|----| ------------|
| `AST_UNKNOWN` | 0 | Unknown or invalid nodes |
| `AST_PROGRAM` | 1 | Program root node |
| `AST_FUNCTION` | 2 | Function definition |
| `AST_VARIABLE` | 3 | Variable declaration |
| `AST_EXPRESSION` | 4 | General expression |
| `AST_STATEMENT` | 5 | General statement |
| `AST_IDENTIFIER` | 6 | Variable/function identifier |
| `AST_LITERAL` | 7 | Literal values |
| `AST_BINARY_OP` | 8 | Binary operations (+, -, *, /) |
| `AST_UNARY_OP` | 9 | Unary operations (-, !, ~) |
| `AST_CALL` | 10 | Function calls |
| `AST_BLOCK` | 11 | Code blocks |
| `AST_IF` | 12 | If statements |
| `AST_FOR` | 13 | For loops |
| `AST_WHILE` | 14 | While loops |
| `AST_RETURN` | 15 | Return statements |
| `AST_ASSIGN` | 16 | Assignment statements |
| `AST_MEMBER_ACCESS` | 17 | Member access (obj.field) |
| `AST_INDEX_ACCESS` | 18 | Array indexing (arr[i]) |
| `AST_TUPLE` | 19 | Tuple expressions |
| `AST_ARRAY` | 20 | Array expressions |
| `AST_STRUCT` | 21 | Struct definitions |
| `AST_INTERFACE` | 22 | Interface definitions |
| `AST_MATCH` | 23 | Pattern matching |
| `AST_PATTERN` | 24 | Match patterns |
| `AST_TYPE` | 25 | Type definitions |

## Core Functions

### Node Creation
```cursed
# Create various types of AST nodes
sus program normie = create_program_node(1, 1)
sus func_node normie = create_function_node("main", 5, 1)
sus var_node normie = create_variable_node("x", 10, 5)
sus literal normie = create_literal_node("42", 15, 10)
```

### Node Analysis
```cursed
# Analyze node properties
sus node_type normie = ast_node_type(my_node)
sus line normie = ast_node_line(my_node)
sus column normie = ast_node_column(my_node)

# Check node types
lowkey is_function_node(my_node) {
    vibez.spill("This is a function node")
}
```

### Tree Traversal
```cursed
# Traverse AST structure
sus node_count normie = count_ast_nodes(root)
sus tree_depth normie = get_ast_depth(root, 0)
sus func_count normie = get_function_count(root)
```

### Pattern Matching
```cursed
# Match AST patterns
lowkey match_function_pattern(node) {
    vibez.spill("Found a function")
}

lowkey match_ast_pattern(node, AST_EXPRESSION) {
    vibez.spill("Found an expression")
}
```

### Transformation
```cursed
# Transform AST nodes
sus new_node normie = transform_ast_node(old_node, AST_IDENTIFIER)
sus cloned normie = clone_ast_node(original)
```

### Pretty Printing
```cursed
# Generate readable output
sus summary tea = print_ast_summary(root)
sus node_str tea = ast_node_to_string(my_node)
sus type_str tea = ast_node_type_string(AST_FUNCTION)
```

### Validation
```cursed
# Validate AST integrity
lowkey validate_ast_node(node) {
    vibez.spill("Node is valid")
}

lowkey validate_ast_tree(root) {
    vibez.spill("Tree structure is valid")
}
```

### Serialization
```cursed
# Export/import AST data
sus json_str tea = export_ast_to_json(root)
sus serialized tea = serialize_ast_node(node)
sus imported normie = import_ast_from_json(json_str)
```

## Builder Utilities

The module includes convenient builder functions for common AST patterns:

```cursed
# Build common structures
sus simple_prog normie = build_simple_program()
sus func_with_ret normie = build_function_with_return("myFunc")
sus assignment normie = build_variable_assignment("x", "42")
sus binary_expr normie = build_binary_expression("a", "+", "b")
sus function_call normie = build_function_call("println", 1)
```

## Advanced Operations

### AST Metrics Analysis
```cursed
sus metrics tea = ast_metrics_analysis(root)
vibez.spill(metrics)
# Output: "Metrics: 15 nodes, 4 depth, 3 functions, 5 variables, 7 expressions"
```

### Node Finding
```cursed
# Find specific node types
sus func_nodes normie = find_nodes_by_type(root, AST_FUNCTION, 0)
sus has_vars lit = has_variable_nodes(root)
sus expr_count normie = get_expression_count(root)
```

### Structure Optimization
```cursed
# Optimize AST structure
sus optimized normie = optimize_ast_structure(root)
sus compressed normie = compress_ast_representation(root)
```

## Module Information

```cursed
# Get module information
sus version tea = ast_mood_version()              # "1.0.0"
sus status tea = ast_mood_status()                # "Production Ready..."
sus types normie = get_supported_node_types()     # 26
sus ready lit = is_ast_mood_ready()               # based
```

## Usage Examples

### Building a Simple Function AST
```cursed
yeet "ast_mood"

# Create program structure
sus program normie = create_program_node(1, 1)
sus func normie = create_function_node("factorial", 3, 1)
sus param normie = create_variable_node("n", 3, 20)
sus body normie = create_block_node(4, 1)

# Validate structure
lowkey validate_ast_tree(program) {
    vibez.spill("AST structure is valid")
}

# Generate summary
sus summary tea = print_ast_summary(program)
vibez.spill(summary)
```

### AST Analysis Pipeline
```cursed
yeet "ast_mood"

slay analyze_ast(root normie) {
    vibez.spill("=== AST Analysis Report ===")
    
    # Basic metrics
    sus nodes normie = count_ast_nodes(root)
    sus depth normie = get_ast_depth(root, 0)
    vibez.spill("Total nodes: " + nodes)
    vibez.spill("Tree depth: " + depth)
    
    # Count by category
    sus functions normie = get_function_count(root)
    sus variables normie = get_variable_count(root)
    sus expressions normie = get_expression_count(root)
    
    vibez.spill("Functions: " + functions)
    vibez.spill("Variables: " + variables)
    vibez.spill("Expressions: " + expressions)
    
    # Validation
    lowkey validate_ast_tree(root) {
        vibez.spill("✅ AST structure is valid")
    } else {
        vibez.spill("❌ AST structure has issues")
    }
}
```

### Pattern Matching Example
```cursed
yeet "ast_mood"

slay process_node(node normie) {
    lowkey match_function_pattern(node) {
        vibez.spill("Processing function node")
        sus func_name tea = ast_node_to_string(node)
        vibez.spill("Function: " + func_name)
    }
    
    lowkey match_variable_pattern(node) {
        vibez.spill("Processing variable node")
    }
    
    lowkey match_ast_pattern(node, AST_EXPRESSION) {
        vibez.spill("Processing expression node")
    }
}
```

## Testing

The module includes comprehensive tests in `test_ast_mood.💀`:

```bash
# Run AST mood tests
cargo run --bin cursed stdlib/ast_mood/test_ast_mood.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/ast_mood/test_ast_mood.💀
./test_ast_mood
```

## Implementation Details

### Node Encoding
Nodes are encoded as integers with the following format:
- **Type** (8 bits): Node type identifier
- **Line** (12 bits): Source line number
- **Column** (12 bits): Source column number  
- **ID** (remaining bits): Unique node identifier

### Memory Management
- All operations use CURSED's built-in memory management
- No manual memory allocation required
- Automatic cleanup of temporary nodes

### Performance Characteristics
- **Node Creation**: O(1) constant time
- **Type Checking**: O(1) constant time
- **Tree Traversal**: O(n) linear time
- **Pattern Matching**: O(1) constant time
- **Validation**: O(n) linear time

## Integration with Compiler Infrastructure

The `ast_mood` module integrates seamlessly with other compiler components:

- **Parser**: Creates AST nodes during parsing
- **Semantic Analysis**: Validates and annotates AST
- **Code Generation**: Traverses AST for code emission
- **Optimization**: Transforms AST structure
- **Debug Info**: Generates debug symbols from AST

## Pure CURSED Implementation

This module is implemented entirely in pure CURSED:
- ✅ Zero FFI dependencies
- ✅ Platform independent
- ✅ Self-hosting ready
- ✅ Memory safe
- ✅ Type safe

## Version History

- **v1.0.0**: Initial comprehensive implementation
  - Complete AST node type coverage
  - Full traversal and analysis utilities
  - Pretty printing and debugging support
  - Validation and integrity checking
  - Serialization capabilities
  - Comprehensive test suite

## Contributing

When extending the `ast_mood` module:

1. Add new node types to constants section
2. Implement corresponding creation functions
3. Add type checking predicates
4. Update validation logic
5. Add comprehensive tests
6. Update documentation

## Performance Notes

- Node operations are highly optimized for compiler use
- Integer encoding minimizes memory overhead
- Pattern matching uses efficient type checking
- Traversal algorithms are optimized for typical AST shapes
- Validation is optional and can be disabled for performance

The `ast_mood` module provides a solid foundation for building sophisticated compiler infrastructure in pure CURSED, enabling advanced code analysis, transformation, and generation capabilities.
