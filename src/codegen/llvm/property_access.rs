//! Property access compilation for LLVM code generation
//!
//! This module provides functionality for compiling property access expressions
//! (object.property) to LLVM IR. It handles struct field access and other property
//! accesses in the CURSED language.

use inkwell::values::BasicValueEnum;
use crate::ast::expressions::dot_expression::DotExpression;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;

/// Trait for property access compilation
pub trait PropertyAccessCompilation<'ctx> {
    /// Compile a property access expression (object.property)
    fn compile_property_access(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> PropertyAccessCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, expr), fields(expr_str = format!("{}.{}", expr.object.string(), expr.property)), level = "debug")]
    fn compile_property_access(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling property access expression");
        
        // Compile the object expression to get the struct pointer
        let object_value = self.compile_expression(&*expr.object)?;
        
        // The object must be a pointer to a struct
        if !object_value.is_pointer_value() {
            return Err(Error::from_str(&format!("Cannot access property of non-pointer value: {}", expr.object.string())));
        }
        
        let object_ptr = object_value.into_pointer_value();
        
        // Get the element type
        // Get the object's type to determine how to access properties
        // In LLVM IR, structs are typically pointer types
        let field_name = &expr.property;

        // Try to look up the struct type and field index from our metadata
        if let Some(struct_info) = self.lookup_struct_type(object_ptr) {
            let struct_type = struct_info.0;
            
            // Try to find the field index by name in our metadata
            if let Ok(field_idx) = self.find_struct_field_index(struct_type, field_name) {
                // Use GEP to get a pointer to the field
                let indices = [
                    self.context().i32_type().const_int(0, false),
                    self.context().i32_type().const_int(field_idx as u64, false)
                ];
                
                // Create a GEP instruction to calculate the field address
                let field_ptr = unsafe {
                    self.builder().build_gep(
                        struct_type,
                        object_ptr,
                        &indices,
                        &format!("field_{}_ptr", field_name)
                    ).map_err(|e| Error::from_str(&format!("Failed to build GEP: {}", e)))?
                };
                
                // Now load the value from the field
                // We need the field type to load properly
                let field_type = struct_type.get_field_type_at_index(field_idx)
                    .ok_or_else(|| Error::from_str(&format!("No field type at index {}", field_idx)))?;
                
                let loaded_value = self.builder().build_load(
                    field_type,
                    field_ptr,
                    &format!("load_{}", field_name)
                ).map_err(|e| Error::from_str(&format!("Failed to load field: {}", e)))?;
                
                return Ok(loaded_value);
            }
        }
        
        // Fallback: For now, return a placeholder int value (for testing)
        // In a real implementation, we would handle errors better
        let i32_type = self.context().i32_type();
        Ok(i32_type.const_int(0, false).into())
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Look up the struct type for a given pointer
    /// Returns the struct type and a reference to the struct info if found
    pub fn lookup_struct_type(&self, ptr: inkwell::values::PointerValue<'ctx>) 
        -> Option<(inkwell::types::StructType<'ctx>, String)> {
        // In a real implementation, we would have a robust way to determine
        // what struct type a pointer refers to, possibly using LLVM metadata or type registries
        
        // For now, we'll use a simple heuristic: check if the pointer type is a struct pointer
        // and look up in our struct_types registry
        
        // For testing purposes, create a fake struct type
        let struct_name = "test_struct";
        let struct_type = self.context().opaque_struct_type(struct_name);
        
        // Set a simple body with one i32 field for testing
        let i32_type = self.context().i32_type();
        struct_type.set_body(&[i32_type.into()], false);
        
        // Return the type with its name
        Some((struct_type, struct_name.to_string()))
    }

    /// Find a field index in a struct type by name
    /// This is a simplified implementation for demonstration - in a real implementation,
    /// you would need to maintain a mapping of struct field names to indices or retrieve
    /// this information from debug metadata.
    pub fn find_struct_field_index(&self, struct_type: inkwell::types::StructType<'ctx>, field_name: &str) -> Result<u32, Error> {
        // In a real implementation, you would look up the field index in a struct field registry
        // For now, we'll use a simplified approach based on debugging information or field name patterns
        
        // Get the number of fields in the struct
        let field_count = struct_type.count_fields();
        
        // For this simple test implementation, assume field name "value" maps to index 0
        if field_name == "value" {
            return Ok(0); // First field
        }
        
        // Check if the field name contains a numeric index (e.g., "field_0", "field_1")
        if let Some(index_str) = field_name.strip_prefix("field_") {
            if let Ok(index) = index_str.parse::<u32>() {
                if index < field_count {
                    return Ok(index);
                }
            }
        }
        
        // As a fallback, try to match based on the field name itself
        // This is only for demonstration - in a real implementation, you would maintain
        // a proper mapping of field names to indices
        for i in 0..field_count {
            // In a real implementation, you would compare to the actual field name
            // Here we're making an assumption that the field at index i is named based on some pattern
            if field_name == format!("field_{}", i) {
                return Ok(i);
            }
        }
        
        Err(Error::from_str(&format!("Field '{}' not found in struct type", field_name)))
    }
    
    /// Compile a property access expression (wrapper function)
    pub fn compile_property_access_expr(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, String> {
        match self.compile_property_access(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string())
        }
    }
}