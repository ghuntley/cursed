# LLVM Code Generator Developer Guide

This guide provides information for developers working on the LLVM code generator for the CURSED language. It covers common tasks, best practices, and implementation patterns.

## Adding New Features

### 1. Extending Expression Compilation

To add support for a new expression type:

1. Create or modify the appropriate trait in a specialized module
2. Implement the trait for `LlvmCodeGenerator`
3. Update the `compile_expression` method in `expression.rs` to handle your new expression type

Example:

```rust
// 1. Define a trait for the new functionality
pub trait MyExpressionOperations<'ctx> {
    fn compile_my_expression(&mut self, expr: &MyExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

// 2. Implement the trait
impl<'ctx> MyExpressionOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_my_expression(&mut self, expr: &MyExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        // Implementation here
    }
}

// 3. Update expression.rs to handle the new expression type
fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
    let any = expr.as_any();
    
    // Handle existing types...
    
    // Handle the new expression type
    if let Some(my_expr) = any.downcast_ref::<MyExpression>() {
        return self.compile_my_expression(my_expr);
    }
    
    // Fall back to basic expressions
    self.compile_basic_expression(expr)
}
```

### 2. Adding New Statement Types

To add support for a new statement type:

1. Create or modify the appropriate trait in a specialized module
2. Implement the trait for `LlvmCodeGenerator`
3. Update the `compile_statement` method in `statement.rs` to handle your new statement type

### 3. Adding Runtime Support

For new runtime features (like concurrency primitives):

1. Add the necessary functions to the appropriate module
2. Register any external functions in `map_external_functions` in `jit.rs`
3. Update the initialization code to support the new features

## Testing Your Changes

Always test your changes thoroughly:

1. **Unit tests**: Test individual components in isolation
2. **Integration tests**: Test the interaction between different modules
3. **JIT execution tests**: Verify that the generated code executes correctly

Test examples can be found in the `tests/` directory.

## Common Tasks

### Working with Pointers

The pointer operations are handled by the `PointerOperations` trait in `pointer_ops.rs`. Key methods:

- `get_address_of`: Get the address of a variable
- `load_from_pointer`: Load a value from a pointer
- `store_to_pointer`: Store a value to a pointer

Always use these methods instead of directly building LLVM load/store instructions to ensure proper error handling and null pointer checks.

### Managing Variable Scopes

Variable scopes are managed by the `VariableHandling` trait in `variables.rs`:

- `add_variable`: Add a variable to the current scope
- `lookup_variable`: Look up a variable across all scopes
- `push_scope` / `pop_scope`: Manage scope stack when entering/exiting blocks

### Error Handling

Follow these guidelines for error handling:

1. Use the `Error` type for all errors
2. Provide detailed error messages that help diagnose issues
3. Use the `.map_err(|e| Error::from_str(...))` pattern for converting LLVM errors
4. Check for error conditions before performing operations that might fail

## Debugging Tips

1. Use `println!("DEBUG: ...")` for debug output
2. Dump the LLVM module with `module.print_to_string().to_string()`
3. Add verification passes with `module.verify()`
4. Test JIT execution with simple programs

## Performance Considerations

1. Avoid unnecessary allocas and loads/stores
2. Reuse LLVM types when possible
3. Use phi nodes instead of allocas for intermediate values when appropriate
4. Let LLVM handle optimizations - focus on generating correct IR

## Compatibility Considerations

When making changes:

1. Update any legacy adapters in `pointer.rs`, `generator.rs`, etc.
2. Maintain backward compatibility with existing interfaces
3. Add forwarding methods in the `LlvmCodeGenerator` impl blocks
4. Document any breaking changes clearly

## Known Limitations

1. Generic types don't yet support all type combinations
2. Some error messages could be improved
3. Some features might not be fully implemented (refer to individual module docs)

## Module Organization

Refer to the architecture diagram in `ARCHITECTURE.md` for an overview of the module organization.

The core principle is that functionality should be organized into cohesive modules that implement
traits for the `LlvmCodeGenerator` struct, keeping the codebase maintainable and extensible. 