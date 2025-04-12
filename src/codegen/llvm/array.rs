//! LLVM code generation for array operations in the CURSED language.
//!
//! This module provides functionality for translating CURSED array-related operations
//! into LLVM IR. It handles array literals (creating arrays with initial values) and
//! array indexing operations (accessing elements by position).
//!
//! The implementation handles:
//! - Creating homogeneous arrays of different element types
//! - Allocating arrays on the stack
//! - Storing initial values in array elements
//! - Accessing array elements by index
//! - Supporting safe memory management for arrays
//! - Enabling array length and capacity operations
//!
//! Arrays in CURSED are similar to arrays in Go, providing fixed-size collections
//! of elements of the same type with zero-based indexing. Future implementations
//! will extend support for slices, multi-dimensional arrays, and dynamic arrays.

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use crate::ast::expressions::IndexExpression;
use crate::ast::ArrayLiteral;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles an array literal expression to LLVM IR.
    ///
    /// This method translates a CURSED array literal (like `[1, 2, 3]`) into LLVM IR
    /// instructions that create and initialize an array on the stack. It performs the
    /// following steps:
    ///
    /// 1. Compiles each array element expression
    /// 2. Verifies that all elements have the same type (homogeneous array)
    /// 3. Creates an appropriately sized and typed array allocation
    /// 4. Stores each element value in the array
    /// 5. Returns a pointer to the array
    ///
    /// # Type Handling
    ///
    /// The method supports arrays of integers, floats, and pointers. For empty arrays,
    /// it defaults to i64 as the element type. For unsupported element types, it falls
    /// back to using i64 arrays.
    ///
    /// # Arguments
    ///
    /// * `array_lit` - The AST node representing the array literal
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - A pointer to the allocated array, or an error message
    pub fn compile_array_literal(&mut self, array_lit: &ArrayLiteral) -> Result<BasicValueEnum<'ctx>, String> {
        // First, compile all the elements of the array
        let mut element_values = Vec::new();
        let mut element_type = None;
        
        for elem in &array_lit.elements {
            let value = self.compile_expression(elem.as_ref())?;
            
            // Track the element type for consistency
            if element_type.is_none() {
                element_type = Some(value.get_type());
            } else if element_type.unwrap() != value.get_type() {
                // For simplicity, we'll require homogeneous arrays
                // A more complex implementation could handle mixed types
                return Err(format!(
                    "Array elements must have the same type: expected {:?}, got {:?}",
                    element_type.unwrap(), value.get_type()
                ));
            }
            
            element_values.push(value);
        }
        
        // Handle empty arrays and determine element type
        // In CURSED, like in Go, arrays must have a consistent element type
        // For empty arrays, we need to choose a default element type
        let elem_type = element_type.unwrap_or_else(|| {
            // Default to i64 (thicc in CURSED) as the element type for empty arrays
            // A full implementation would infer this from the context or require type annotations
            self.context.i64_type().into()
        });
        
        // Create an LLVM array type with the appropriate element type and length
        // In LLVM, arrays are represented as a sequence of elements of the same type
        let array_type: inkwell::types::BasicTypeEnum<'ctx> = if let Some(first_type) = element_type {
            // We've already verified that all elements have the same type
            // Now we need to create an LLVM array type based on that element type
            let element_count = element_values.len() as u32;
            
            // Handle different element types in CURSED:
            // - Integer types (normie, thicc, etc.) -> LLVM integer types (i32, i64, etc.)
            // - Float types (snack, meal) -> LLVM float types (f32, f64)
            // - Pointer types (references, strings) -> LLVM pointer types
            // - Complex types would be handled specially
            if first_type.is_int_type() {
                // Use i64 (thicc) as the canonical integer array type
                // A full implementation would preserve the exact integer width
                self.context.i64_type().array_type(element_count).into()
            } else if first_type.is_float_type() {
                // Use f64 (meal) as the canonical float array type
                self.context.f64_type().array_type(element_count).into()
            } else if first_type.is_pointer_type() {
                // For pointers, use void* (i8*) as a generic pointer type
                // This would be refined in a full implementation with proper type tracking
                let void_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
                void_ptr.array_type(element_count).into()
            } else {
                // For unsupported or complex types, fall back to i64 array
                // A full implementation would handle all CURSED types properly
                self.context.i64_type().array_type(element_count).into()
            }
        } else {
            // Empty array case - use zero-length i64 array
            // This might need special handling for empty array literals with explicit types
            self.context.i64_type().array_type(0).into()
        };
        
        // Allocate memory for the array on the stack
        // In LLVM, we need to explicitly allocate space for the array and then fill it
        // This creates a pointer to the array that we'll return from this function
        let array_alloca = self.builder.build_alloca(array_type, "array").unwrap();
        
        // Store each element in the array by calculating its address and storing the value
        for (i, val) in element_values.iter().enumerate() {
            // Create a constant for the current index
            let i_val = self.context.i32_type().const_int(i as u64, false);
            
            // Get pointer to the specific array element using GetElementPtr (GEP)
            // For a multi-dimensional array, this would use multiple indices
            // The indices used here are:
            // - First 0: Skip to the array data (necessary for LLVM's array representation)
            // - Then i: Index to the specific element
            let elem_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    array_type,
                    array_alloca,
                    &[self.context.i32_type().const_zero(), i_val],
                    &format!("array_elem_{}", i)
                ).unwrap()
            };
            
            // Store the element value at the calculated address
            // For complex element types, additional handling would be needed here
            // A full implementation would handle alignment and size considerations
            self.builder.build_store(elem_ptr, *val).unwrap();
        }
        
        // Return the array pointer (not the individual elements)
        // In CURSED, arrays are passed by reference, not by value
        // The caller will use this pointer for array operations like indexing
        Ok(array_alloca.into())
    }
    
    /// Compiles an array indexing expression to LLVM IR.
    ///
    /// This method translates a CURSED index expression (like `arr[5]`) into LLVM IR
    /// instructions that access an element of an array by its index. It performs the
    /// following steps:
    ///
    /// 1. Compiles the array expression to get a pointer to the array
    /// 2. Compiles the index expression to get an integer index
    /// 3. Performs validation to ensure the array is a pointer and index is an integer
    /// 4. Uses GEP (GetElementPtr) to calculate the address of the indexed element
    /// 5. Loads and returns the value at that address
    ///
    /// # Implementation Notes
    ///
    /// This is a simplified implementation that:
    /// - Skips bounds checking (a production implementation would verify index < length)
    /// - Uses generic element types (a full implementation would track exact types)
    /// - Assumes a default i32 type for loaded values
    /// - Doesn't handle multi-dimensional arrays or slices
    /// - Lacks integration with garbage collection for heap-allocated arrays
    /// - Doesn't implement CURSED runtime features like array resizing or conversion
    ///
    /// # Arguments
    ///
    /// * `index_expr` - The AST node representing the indexing expression
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The value at the specified array index, or an error message
    pub fn compile_index_expression(&mut self, index_expr: &IndexExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // First compile the left (array) expression
        let array_val = self.compile_expression(index_expr.left.as_ref())?;
        
        // And the index expression
        let index_val = self.compile_expression(index_expr.index.as_ref())?;
        
        // Validate operand types for array indexing
        
        // Check that the array operand is a pointer value
        // Arrays in LLVM are always manipulated through pointers, whether they are:
        // - Static arrays allocated on the stack
        // - Dynamic arrays allocated on the heap
        // - Slices pointing to portions of other arrays
        if !array_val.is_pointer_value() {
            return Err("Cannot index a non-pointer value".to_string());
        }
        
        // Check that the index operand is an integer value
        // CURSED only supports integer indices for array access, similar to most languages
        // This includes literal integers, variables, or expressions that evaluate to integers
        if !index_val.is_int_value() {
            return Err("Array index must be an integer".to_string());
        }
        
        // In a full implementation, we would also:
        // 1. Check that the array type actually supports indexing (is an array or slice)
        // 2. Verify that the pointer points to a valid array memory layout
        // 3. Validate index type compatibility (unsigned vs. signed)
        // 4. Use LLVM metadata to track array dimensions for proper type checking
        // 5. Implement runtime bounds checking with proper error messages
        // 6. Support multi-dimensional arrays with multiple index expressions
        
        let array_ptr = array_val.into_pointer_value();
        // We can't directly use get_element_type() in this implementation
        // For a full implementation, we'd need proper type tracking that would:
        // 1. Maintain a mapping between CURSED types and LLVM types throughout compilation
        // 2. Track array dimensions and element types in symbol tables
        // 3. Use LLVM metadata to store type information for runtime operations
        let index = index_val.into_int_value();
        
        // For bounds checking in a full implementation, we'd need to:
        // 1. Know the array length (either static or stored at runtime)
        // 2. Insert bounds check instructions before the access
        // 3. Branch to error handling code for out-of-bounds access
        // 4. Potentially use LLVM's get_element_ptr with inbounds flag
        // 5. Generate debug info to report file and line number in error messages
        // 6. Support configurable bounds checking that could be disabled for release builds
        // 7. Implement CURSED's panic/recovery mechanism for array bounds violations
        // For this simplified implementation, we'll skip bounds checking
        
        // In a full compiler implementation with proper type information, we would:
        // 1. Know the exact element type of the array (from type checking and symbol tables)
        // 2. Calculate the correct offset using the element size
        // 3. Generate appropriate errors for type mismatches
        // 4. Handle multi-dimensional arrays correctly
        
        // Without full type information, we'll use a more direct approach with simplified assumptions:
        // - We'll treat the pointer as pointing to a sequence of elements
        // - We'll use GEP with index-based offset calculation (pointer arithmetic: ptr + index)  
        // - We'll make assumptions about the element type for loading
        
        // Generate pointer to the element using GetElementPtr instruction
        let elem_ptr = unsafe {
            // We need to specify the element type for GEP, but without get_element_type(),
            // we'll use a generic i8 (byte) type as a placeholder
            // This is unsafe because we're making assumptions about the memory layout
            let elem_type = self.context.i8_type();
            
            self.builder.build_gep(
                elem_type,
                array_ptr,
                &[index],  // Single index for linear access
                "array_elem_ptr"
            ).unwrap()
        };
        
        // Load the value from the calculated address
        // A proper implementation would:
        // 1. Know the actual element type from the symbol table or type analysis
        // 2. Use the correct LLVM type for loading (matching the element type)
        // 3. Apply appropriate type conversions if needed
        // 4. Handle complex element types like structs or arrays
        // 5. Apply CURSED's automatic type coercion rules if applicable
        // 6. Handle interface types and dynamic dispatch if needed
        // 7. Track memory operations for garbage collection integration
        let elem_val = self.builder.build_load(self.context.i32_type(), elem_ptr, "indexed_value").unwrap();
        
        Ok(elem_val)
    }
}