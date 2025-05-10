//! Property access compilation for LLVM code generation
//!
//! This module provides functionality for compiling property access expressions
//! (object.property) to LLVM IR. It handles struct field access and other property
//! accesses in the CURSED language.

use inkwell::values::BasicValueEnum;
use inkwell::types::BasicTypeEnum;
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
        let field_name = &expr.property;
        
        // Try to determine the struct type from the pointer
        let result = self.get_struct_type_from_ptr(object_ptr);
        
        if let Ok((struct_type, struct_name)) = result {
            tracing::debug!(struct_name = struct_name, field_name = field_name, "Looking up field in struct");
            
            // Try to find the field index by name using our improved field registry
            if let Ok((field_idx, _)) = self.get_field_index(struct_name, field_name) {
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
                
                tracing::debug!(field_name = field_name, struct_name = struct_name, "Successfully loaded field");
                return Ok(loaded_value);
            } else {
                return Err(Error::from_str(&format!("Field '{}' not found in struct '{}'", field_name, struct_name)));
            }
        }
        
        // If we couldn't determine the struct type or find the field, return a detailed error
        Err(Error::from_str(&format!("Cannot access property '{}' on expression '{}': unable to determine struct type", 
                                    field_name, expr.object.string())))
    }
}

// Extension methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the struct type and name from a pointer value
    /// This method determines what struct a pointer points to based on LLVM type information
    #[tracing::instrument(skip(self, ptr), level = "debug")]
    pub fn get_struct_type_from_ptr(&self, ptr: inkwell::values::PointerValue<'ctx>) 
        -> Result<(inkwell::types::StructType<'ctx>, String), Error> {
        // Get the element type that this pointer points to
        let pointee_type = ptr.get_type().get_pointee_type();
        
        // Check if it's a struct type - we can directly check this from the LLVM type
        if let Ok(struct_type) = pointee_type.try_into_struct_type() {
            // Get the struct name from LLVM (if it has one)
            if let Some(struct_name) = struct_type.get_name() {
                // Convert from CString to regular String
                if let Ok(name_str) = struct_name.to_str() {
                    // Strip any LLVM prefixes if present (like "struct.")
                    let cleaned_name = if let Some(stripped) = name_str.strip_prefix("struct.") {
                        stripped.to_string()
                    } else {
                        name_str.to_string()
                    };
                    
                    tracing::debug!(struct_name = &cleaned_name, "Found struct type from pointer");
                    return Ok((struct_type, cleaned_name));
                }
            }
            
            // If we have a struct type but couldn't get a name, use a generated one
            let generic_name = format!("anonymous_struct_{}", struct_type.count_fields());
            tracing::debug!(struct_name = &generic_name, "Anonymous struct type detected");
            
            return Ok((struct_type, generic_name));
        }
        
        // If we get here, it's not a struct pointer
        Err(Error::from_str("Pointer does not point to a struct type"))
    }

    /// Get the field index and type for a given struct and field name
    /// This uses the struct field registry maintained during compilation
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn get_field_index(&self, struct_name: &str, field_name: &str) -> Result<(u32, BasicTypeEnum<'ctx>), Error> {
        // First, try to look up in our field registry
        // Search for the struct in all packages, starting with the current one
        tracing::debug!(struct_name = struct_name, field_name = field_name, "Looking for field in struct");
        
        // Look in our struct registry to find the struct type
        let mut struct_type = None;
        
        // Check in current package first
        if let Some(structs) = self.struct_types.get(&self.current_package_name) {
            if let Some(st) = structs.get(struct_name) {
                struct_type = Some(*st);
            }
        }
        
        // If not found, check all packages
        if struct_type.is_none() {
            for (_, structs) in &self.struct_types {
                if let Some(st) = structs.get(struct_name) {
                    struct_type = Some(*st);
                    break;
                }
            }
        }
        
        // If we found the struct type, look for the field
        if let Some(struct_type) = struct_type {
            // Get field count to validate access
            let field_count = struct_type.count_fields();
            
            // Check in GC metadata for field name -> index mapping
            if let Some(gc_fields) = self.gc_metadata.get(struct_name) {
                for (idx, field_info) in gc_fields.iter() {
                    if &field_info.1 == field_name {
                        // Make sure the index is valid
                        if *idx < field_count as usize {
                            // Get the field type
                            if let Some(field_type) = struct_type.get_field_type_at_index(*idx as u32) {
                                tracing::debug!(field_name = field_name, index = idx, "Found field in gc_metadata");
                                return Ok((*idx as u32, field_type));
                            }
                        }
                    }
                }
            }
            
            // If we have debug info for the struct, use that to find the field
            // This would be populated during struct compilation
            
            // As a fallback, scan through AST to find struct definition and field
            // This would require maintaining a registry of struct definitions
            
            // For now, we'll use some heuristics based on common field naming patterns
            
            // Common field names to check at specific indices
            let common_fields = [
                ("value", 0),
                ("data", 0),
                ("length", 1),
                ("capacity", 2),
                ("x", 0),
                ("y", 1),
                ("z", 2),
                ("name", 0),
                ("next", 1),
                ("prev", 2),
            ];
            
            for (name, idx) in common_fields.iter() {
                if field_name == *name && (*idx as u32) < field_count {
                    if let Some(field_type) = struct_type.get_field_type_at_index(*idx as u32) {
                        tracing::debug!(field_name = field_name, index = idx, "Found field using common field pattern");
                        return Ok((*idx as u32, field_type));
                    }
                }
            }
            
            // Check if field name has pattern like "field_0", "field_1", etc.
            if let Some(index_str) = field_name.strip_prefix("field_") {
                if let Ok(index) = index_str.parse::<u32>() {
                    if index < field_count {
                        if let Some(field_type) = struct_type.get_field_type_at_index(index) {
                            tracing::debug!(field_name = field_name, index = index, "Found field by index pattern");
                            return Ok((index, field_type));
                        }
                    }
                }
            }
            
            // If field name exactly matches position, try that
            for i in 0..field_count {
                if field_name == format!("field{}", i) || field_name == i.to_string() {
                    if let Some(field_type) = struct_type.get_field_type_at_index(i) {
                        tracing::debug!(field_name = field_name, index = i, "Found field by position");
                        return Ok((i, field_type));
                    }
                }
            }
        }
        
        // If we couldn't find the field, return an error
        Err(Error::from_str(&format!("Field '{}' not found in struct '{}'", field_name, struct_name)))
    }
    
    /// Compile a property access expression (wrapper function)
    pub fn compile_property_access_expr(&mut self, expr: &DotExpression) -> Result<BasicValueEnum<'ctx>, String> {
        match self.compile_property_access(expr) {
            Ok(val) => Ok(val),
            Err(e) => Err(e.to_string())
        }
    }
}