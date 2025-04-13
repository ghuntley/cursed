# LLVM Code Generator Specifications

This document outlines the architecture and standards for the LLVM code generator in the CURSED language compiler.

## Architecture Overview

The LLVM code generator follows a modular design with the following key principles:

1. **Single Source of Truth**: The `LlvmCodeGenerator` struct is defined in `context.rs` and is the central component for all code generation.
2. **Trait-Based Extensions**: Functionality is added through trait implementations in specialized modules.
3. **Re-export Pattern**: The main `mod.rs` re-exports all necessary types and traits for external use.
4. **Backward Compatibility**: Legacy interfaces are maintained through forwarding methods.

## Module Structure

```
src/codegen/llvm/
├── mod.rs                  # Module entry point, re-exports public types
├── context.rs              # Main LlvmCodeGenerator implementation
├── basic_expressions.rs    # Compilation of literals and basic operations
├── pointer_ops.rs          # Pointer operations (standardized implementation)
├── variables.rs            # Variable declaration and access
├── statement.rs            # Statement compilation
├── expression.rs           # Expression compilation entry point
├── control_flow.rs         # Control flow statements (if, while, for, switch)
├── break_continue.rs       # Break and continue statement handling
├── generator.rs            # Backward compatibility adapter for context.rs
├── pointer.rs              # Backward compatibility adapter for pointer_ops.rs
└── ... additional modules
```

## Module Relationships

- **context.rs** provides the core `LlvmCodeGenerator` struct with essential fields and methods
- All other modules import from **context.rs** and implement functionality through traits
- Specialized modules avoid circular dependencies by focusing on a single responsibility
- Every trait for a major piece of functionality is defined in its own module

## Trait Design

Each module defines a trait with a consistent naming pattern:

```rust
trait XXXOperations<'ctx> {
    fn compile_xxx(&mut self, ...) -> Result<...>;
    // Additional methods...
}

impl<'ctx> XXXOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    // Implementation...
}
```

## Pointer Operations Specification

The `PointerOperations` trait in `pointer_ops.rs` defines the following operations:

1. **get_address_of**: Takes the address of an expression, returning a pointer
2. **compile_pointer_type**: Handles pointer type expressions
3. **compile_pointer_dereference**: Handles pointer dereference expressions
4. **load_from_pointer**: Loads a value from a pointer with error handling
5. **store_to_pointer**: Stores a value to a pointer with error handling
6. **create_null_pointer**: Creates a null pointer of a given type

### Error Handling

All pointer operations include thorough error handling:

- Null pointer checks to prevent segmentation faults
- Type compatibility checks
- Runtime error reporting

### Safety Features

The pointer operations implement safety features:

1. **Null Pointer Detection**: Checks for null pointers before dereferencing
2. **Error Reporting**: Reports detailed error messages for debugging
3. **Type Safety**: Verifies type compatibility for pointer operations

## Testing Requirements

All components of the LLVM code generator must have comprehensive tests:

1. **Unit Tests**: Test individual operations in isolation
2. **Integration Tests**: Test the interaction between different modules
3. **JIT Execution Tests**: Verify execution results match expectations

## Backward Compatibility

To maintain backward compatibility:

1. Legacy modules (like `pointer.rs`) re-export from new standardized modules
2. Legacy methods forward to the new standardized implementations
3. No breaking changes to public APIs

## Performance Considerations

The implementation prioritizes:

1. Correctness
2. Maintainability
3. Performance

Optimizations should focus on reducing unnecessary LLVM IR generation and simplifying the output to help LLVM's optimizer. 