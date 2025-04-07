//! LLVM code generation for array operations

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use crate::ast::{ArrayLiteral, IndexExpression};
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile an array literal
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
        
        // If array is empty, default to i64 as element type
        let elem_type = element_type.unwrap_or_else(|| self.context.i64_type().into());
        
        // Create an array type with the appropriate element type and length
        let array_type: inkwell::types::BasicTypeEnum<'ctx> = if let Some(first_type) = element_type {
            // BasicValueEnum is already what we need for an array element
            // We'll just use first_type directly
            let element_count = element_values.len() as u32;
            
            // Use the context to create appropriate array types based on first_type
            if first_type.is_int_type() {
                self.context.i64_type().array_type(element_count).into()
            } else if first_type.is_float_type() {
                self.context.f64_type().array_type(element_count).into()
            } else if first_type.is_pointer_type() {
                let void_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
                void_ptr.array_type(element_count).into()
            } else {
                // Default to i64 array if unsupported type
                self.context.i64_type().array_type(element_count).into()
            }
        } else {
            // Empty array, use i64 as default element type
            self.context.i64_type().array_type(0).into()
        };
        
        // Allocate the array on the stack
        let array_alloca = self.builder.build_alloca(array_type, "array").unwrap();
        
        // Store each element in the array
        for (i, val) in element_values.iter().enumerate() {
            let i_val = self.context.i32_type().const_int(i as u64, false);
            
            // Get pointer to the array element
            let elem_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    array_type,
                    array_alloca,
                    &[self.context.i32_type().const_zero(), i_val],
                    &format!("array_elem_{}", i)
                ).unwrap()
            };
            
            // Store the element
            self.builder.build_store(elem_ptr, *val).unwrap();
        }
        
        // Return the array pointer
        Ok(array_alloca.into())
    }
    
    /// Compile an index expression (array[index])
    pub fn compile_index_expression(&mut self, index_expr: &IndexExpression) -> Result<BasicValueEnum<'ctx>, String> {
        // First compile the left (array) expression
        let array_val = self.compile_expression(index_expr.left.as_ref())?;
        
        // And the index expression
        let index_val = self.compile_expression(index_expr.index.as_ref())?;
        
        // Make sure the array is a pointer (to either an array or a struct)
        if !array_val.is_pointer_value() {
            return Err("Cannot index a non-pointer value".to_string());
        }
        
        // Make sure the index is an integer
        if !index_val.is_int_value() {
            return Err("Array index must be an integer".to_string());
        }
        
        let array_ptr = array_val.into_pointer_value();
        // We can't directly use get_element_type() in this implementation
        // For a full implementation, we'd need proper type tracking
        let index = index_val.into_int_value();
        
        // For bounds checking in a full implementation, we'd need to know the array type
        // For this simplified implementation, we'll skip bounds checking
        
        // In a simplified implementation without full type information, we'll use a more direct approach
        // We'll treat this as a simple array access using GEP with a zero base and index offset
        
        // This performs pointer arithmetic: ptr + index
        let elem_ptr = unsafe {
            // We need to specify the element type, but without get_element_type(),
            // we'll use a generic i8 (byte) type for the element
            let elem_type = self.context.i8_type();
            
            self.builder.build_gep(
                elem_type,
                array_ptr,
                &[index],
                "array_elem_ptr"
            ).unwrap()
        };
        
        // For the simplified implementation, we'll use i32 as a default type to load
        // A full implementation would maintain type information to know the right type to load
        let elem_val = self.builder.build_load(self.context.i32_type(), elem_ptr, "indexed_value").unwrap();
        
        Ok(elem_val)
    }
}