# LLVM Code Generator Refactoring - Next Steps

## Overview

This document outlines the progress made on refactoring the LLVM code generator and the remaining work to complete the refactoring process.

## Completed Work

- Created modular structure with separate files for different concerns:
  - `context.rs`: Core LlvmCodeGenerator struct and initialization
  - `types.rs`: Shared type definitions
  - `errors.rs`: Error handling utilities
  - `struct_type.rs`: Struct-related functionality
  - `channel.rs`: Channel-related functionality
  - `pointer.rs`: Pointer operations
  - `string.rs`: String handling operations
  - `stan.rs`: Stan (goroutine) implementation
  - `util.rs`: Utility functions
  - `array.rs`: Array operations
  - `hash.rs`: Hash map operations

- Added proper getters/setters for the LlvmCodeGenerator struct
- Added basic test coverage (llvm_refactor_test.rs, llvm_refactor_integration_test.rs)
- Created compatibility layer to maintain backward compatibility

## Remaining Implementation Work

### 1. Expression Implementation

The `expression.rs` module has only stub implementations. Implement:

- `compile_identifier`: Load values from variables
- `compile_prefix_expression`: Unary operations like !x, -x
- `compile_infix_expression`: Binary operations like x + y, x < y
- `compile_if_expression`: Conditional expressions
- `compile_property_access`: Access struct fields
- `compile_assignment`: Variable assignment operations

### 2. Statement Implementation

Complete the statement implementations:

- `compile_break_statement`: Break out of loops
- `compile_import_statement`: Proper module import handling
- `compile_later_statement`: Defer statement implementation

### 3. Control Flow

Implement the remaining control flow structures:

- Loop control flow tracking
- Continue statement implementation
- Switch/case statement implementation

### 4. Type System

Implement proper type handling:

- Generic type support
- Interface implementation
- Type inference

### 5. Memory Management

Implement proper memory management:

- Garbage collection interface
- Memory allocation/deallocation
- Reference counting (if needed)

## Testing Strategy

1. **Unit Tests**:
   - Write unit tests for each module/function
   - Test edge cases for each operation
   - Test error handling

2. **Integration Tests**:
   - End-to-end tests for code generation
   - Test all language features working together
   - Compare generated LLVM IR to expected output

3. **Compatibility Tests**:
   - Ensure all existing code works with the refactored implementation
   - Run all existing tests

## Integration Steps

1. **Expression Module Implementation**:
   - Implement the core expression operations
   - Test each operation separately
   - Integrate with existing code

2. **Statement Module Implementation**:
   - Implement the remaining statement operations
   - Test each statement type separately
   - Integrate with existing code

3. **Full Compatibility Testing**:
   - Run all existing tests to ensure compatibility
   - Fix any issues found

4. **Performance Benchmarking**:
   - Benchmark the refactored implementation vs. the original
   - Address any performance regressions

5. **Documentation**:
   - Update documentation to reflect the new structure
   - Document all new public APIs

## Specific Implementation Details

### Implement Expression Compilation

The `compile_infix_expression` function needs to handle various operators:

```rust
pub fn compile_infix_expression(&mut self, infix: &InfixExpression) -> Result<BasicValueEnum<'ctx>, String> {
    // Compile left and right expressions
    let left = self.compile_expression(infix.left.as_ref())?;
    let right = self.compile_expression(infix.right.as_ref())?;
    
    // Handle operation based on operator
    match infix.operator.as_str() {
        "+" => self.compile_addition(left, right),
        "-" => self.compile_subtraction(left, right),
        "*" => self.compile_multiplication(left, right),
        "/" => self.compile_division(left, right),
        // Handle comparisons
        "<" => self.compile_comparison(left, right, IntPredicate::SLT),
        "<=" => self.compile_comparison(left, right, IntPredicate::SLE),
        ">" => self.compile_comparison(left, right, IntPredicate::SGT),
        ">=" => self.compile_comparison(left, right, IntPredicate::SGE),
        "==" => self.compile_equality(left, right, true),
        "!=" => self.compile_equality(left, right, false),
        // Other operators...
        _ => Err(format!("Unsupported operator: {}", infix.operator))
    }
}
```

### Implement Property Access

The property access implementation needs to handle struct field access:

```rust
pub fn compile_property_access(
    &mut self,
    prop_access: &PropertyAccessExpression
) -> Result<BasicValueEnum<'ctx>, String> {
    // Compile the left side (the struct)
    let struct_val = self.compile_expression(prop_access.object.as_ref())?;
    
    // Must be a pointer to a struct
    if !struct_val.is_pointer_value() {
        return Err("Cannot access property of non-struct value".to_string());
    }
    
    let struct_ptr = struct_val.into_pointer_value();
    let struct_type = struct_ptr.get_type().get_element_type();
    
    // Must be a struct type
    if !struct_type.is_struct_type() {
        return Err("Cannot access property of non-struct type".to_string());
    }
    
    // Find the field index
    let struct_type = struct_type.into_struct_type();
    let field_name = &prop_access.property.value;
    let field_index = self.find_struct_field_index(struct_type, field_name)?;
    
    // Get pointer to the field
    let field_ptr = self.builder.build_struct_gep(
        struct_type,
        struct_ptr,
        field_index,
        &format!("field_{}", field_name)
    )?;
    
    // Load the field value
    let field_type = struct_type.get_field_type(field_index).unwrap();
    let field_val = self.builder.build_load(field_type, field_ptr, field_name)?;
    
    Ok(field_val)
}
```

## Conclusion

This refactoring is a significant undertaking that will improve the maintainability and extensibility of the LLVM code generator. By breaking it down into smaller, focused modules, we make it easier to understand, test, and extend.

The next immediate steps are to implement the expression operations, as these are the core building blocks for all other functionality. After that, implement the remaining statement operations and control flow structures.

Testing at each step is crucial to ensure compatibility with existing code and to catch regressions early.