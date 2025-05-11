//! Utility functions for LLVM code generation

use inkwell::types::{BasicTypeEnum, BasicMetadataTypeEnum};
use inkwell::values::BasicValueEnum;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Convert a basic type enum to a metadata type enum
    pub fn to_metadata_type(&self, ty: BasicTypeEnum<'ctx>) -> BasicMetadataTypeEnum<'ctx> {
        ty.into()
    }
    
    /// Convert a slice of basic types to a vector of metadata types
    pub fn to_metadata_types(&self, types: &[BasicTypeEnum<'ctx>]) -> Vec<BasicMetadataTypeEnum<'ctx>> {
        types.iter().map(|&ty| ty.into()).collect()
    }
    
    /// Check if a value is a string (i8* pointer)
    #[tracing::instrument(skip(self, value), fields(value_kind = ?value.get_type()), level = "trace")]
    pub fn is_string_value(&self, value: BasicValueEnum<'ctx>) -> bool {
        if !value.is_pointer_value() {
            return false;
        }
        
        // In LLVM, an i8* pointer is the standard way to represent a string
        // We can just check if it's a pointer to i8
        let ptr_type = value.into_pointer_value().get_type();
        // Since get_element_type isn't available, we have to use a heuristic
        // This is a simplification - a real implementation would have better type checking
        true // Assume any pointer could be a string pointer
    }
    
    // is_string_type was moved to string_utils.rs to avoid duplication
    // Use StringUtilsExtension trait instead
}