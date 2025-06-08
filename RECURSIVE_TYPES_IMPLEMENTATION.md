# Recursive Type Definitions Implementation for CURSED

## Summary

This document describes the comprehensive implementation of recursive type definitions for the CURSED programming language. The implementation provides full support for direct recursive structs, mutually recursive types, complex recursive scenarios, and proper integration with LLVM code generation and the garbage collector.

## Features Implemented

### 1. Core Recursive Type System (`src/core/recursive_types.rs`)

- **RecursiveType Structure**: Represents recursive type definitions with cycle detection
- **RecursiveTypeRegistry**: Manages registration and resolution of recursive types
- **Cycle Detection**: Identifies direct and indirect recursion using DFS algorithm
- **Dependency Tracking**: Tracks type dependencies for proper resolution order
- **Forward Declarations**: Enables mutually recursive types through forward declarations
- **Lazy Resolution**: Defers type resolution to break circular dependencies

### 2. TypeChecker Integration (`src/core/type_checker.rs`)

- **Extended TypeChecker**: Added recursive type registry to TypeChecker
- **Three-Pass Type Collection**:
  1. Forward declarations for all type names
  2. Register actual type definitions
  3. Resolve all recursive types
- **RecursiveTypeResolver Trait**: Interface for resolving recursive type references
- **Enhanced Field Type Parsing**: Supports generic types and complex recursive patterns

### 3. LLVM Code Generation (`src/codegen/llvm/recursive_types.rs`)

- **RecursiveTypeLLVM Trait**: Interface for LLVM generation of recursive types
- **Forward Declarations**: Creates opaque LLVM struct types for recursive references
- **Memory Layout Generation**: Ensures proper memory layout for recursive structures
- **Cycle Breaking**: Uses pointers to break infinite type size issues
- **RecursiveTypeExtensions**: Additional utilities for validation and dependency analysis

### 4. Enhanced Struct Compilation (`src/codegen/llvm/struct_type.rs`)

- **Recursive Struct Detection**: Automatically detects potentially recursive structs
- **Forward Declaration Integration**: Creates forward declarations as needed
- **Pointer-Based Recursion**: Handles recursive references through pointers
- **Memory Safety**: Ensures recursive types have finite size through pointer indirection

## Supported Recursive Type Patterns

### 1. Direct Recursive Structs

```cursed
be_like Node squad {
    value normie
    next *Node
}
```

### 2. Binary Trees

```cursed
be_like TreeNode squad {
    value normie
    left *TreeNode
    right *TreeNode
}
```

### 3. Mutually Recursive Types

```cursed
be_like GraphNode squad {
    id normie
    edges []*GraphEdge
}

be_like GraphEdge squad {
    from *GraphNode
    to *GraphNode
    weight normie
}
```

### 4. Generic Recursive Types

```cursed
be_like List[T] squad {
    head *ListNode[T]
    size normie
}

be_like ListNode[T] squad {
    value T
    next *ListNode[T]
}
```

### 5. Complex Recursive Scenarios

```cursed
be_like Graph squad {
    nodes []Node
    edges []Edge
}

be_like Node squad {
    id normie
    edges []*Edge
}

be_like Edge squad {
    from *Node
    to *Node
    weight normie
}
```

## Technical Implementation Details

### Cycle Detection Algorithm

The implementation uses a Depth-First Search (DFS) algorithm with a recursion stack to detect cycles:

1. **Visited Set**: Tracks all visited types during traversal
2. **Recursion Stack**: Tracks types currently in the recursion path
3. **Path Tracking**: Maintains the current dependency path for cycle identification
4. **Cycle Collection**: Stores detected cycles for analysis and warnings

### Type Resolution Process

1. **Forward Declaration Phase**: Creates placeholder types for all declared types
2. **Registration Phase**: Registers actual type definitions with dependency information
3. **Topological Sort**: Determines resolution order based on dependencies
4. **Resolution Phase**: Resolves types in dependency order with cycle handling
5. **Pointer Injection**: Injects pointers to break cycles where necessary

### Memory Layout Strategy

- **Finite Size Guarantee**: All recursive types must use pointers for recursive references
- **LLVM Opaque Types**: Uses LLVM opaque struct types for forward declarations
- **Body Assignment**: Sets struct body after all forward declarations are created
- **Pointer Indirection**: Ensures recursive references go through pointers for finite size

### Integration with Garbage Collector

The recursive type system is designed to work seamlessly with CURSED's garbage collector:

- **Traceable Fields**: Recursive pointer fields are automatically marked as traceable
- **Cycle Collection**: GC can handle cycles in recursive data structures
- **Memory Safety**: No memory leaks from circular references

## Testing Coverage

### Unit Tests (`tests/recursive_types_test.rs`)

1. **Direct Recursive Structs**: Tests simple self-referencing structures
2. **Binary Tree Structures**: Tests recursive structures with multiple references
3. **Mutually Recursive Types**: Tests types that reference each other
4. **Complex Scenarios**: Tests multi-level recursive dependencies
5. **Forward Declarations**: Tests forward declaration resolution
6. **TypeChecker Integration**: Tests integration with type checking system
7. **Cycle Detection**: Tests cycle detection in complex scenarios
8. **Generic Recursive Types**: Tests recursive types with type parameters
9. **Indirect Recursion**: Tests recursion through intermediate types
10. **Resolution Order**: Tests proper dependency resolution ordering
11. **Memory Safety**: Tests that recursive types maintain memory safety

### Example Code (`examples/recursive_types.csd`)

Comprehensive examples demonstrating:
- Linked list implementation
- Binary tree structures
- Graph data structures with nodes and edges
- Generic recursive containers
- Complex expression trees
- Memory-safe recursive algorithms

## Performance Characteristics

### Time Complexity
- **Type Registration**: O(1) per type
- **Cycle Detection**: O(V + E) where V = types, E = dependencies
- **Topological Sort**: O(V + E) for resolution order
- **Type Resolution**: O(V) with memoization

### Space Complexity
- **Type Storage**: O(V) for type definitions
- **Dependency Graph**: O(E) for dependency edges
- **Resolution Cache**: O(V) for resolved types

### LLVM Generation
- **Forward Declarations**: Constant time per type
- **Memory Layout**: Linear in number of fields
- **Code Generation**: No overhead for recursive references

## Error Handling

### Compile-Time Errors
- **Infinite Size Detection**: Prevents direct recursive embedding
- **Missing Forward Declarations**: Clear error messages for undefined types
- **Circular Dependencies**: Warnings for circular type dependencies
- **Invalid Type References**: Errors for malformed recursive references

### Runtime Safety
- **Null Pointer Handling**: Safe handling of null recursive references
- **Memory Bounds**: Prevents stack overflow in recursive operations
- **GC Integration**: Proper cleanup of circular structures

## Future Enhancements

### Potential Improvements
1. **Advanced Cycle Breaking**: More sophisticated cycle breaking strategies
2. **Performance Optimization**: Caching and memoization improvements
3. **Better Error Messages**: More descriptive error messages for complex scenarios
4. **Debug Support**: Enhanced debugging support for recursive structures
5. **Static Analysis**: Additional static analysis for recursive type safety

### Integration Opportunities
1. **Pattern Matching**: Enhanced pattern matching for recursive types
2. **Serialization**: Automatic serialization support for recursive structures
3. **Reflection**: Runtime reflection capabilities for recursive types
4. **Documentation**: Automatic documentation generation for recursive type hierarchies

## Conclusion

The recursive type implementation for CURSED provides a robust, safe, and efficient foundation for complex data structures. The implementation follows modern compiler design principles with proper separation of concerns, comprehensive error handling, and excellent performance characteristics. The system is fully integrated with CURSED's type checker, LLVM code generator, and garbage collector, providing a seamless experience for developers working with recursive data structures.

The implementation successfully handles all common recursive patterns while maintaining memory safety and providing clear error messages. The extensive test coverage ensures reliability, and the modular design allows for future enhancements and optimizations.
