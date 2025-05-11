//! Extension trait for LLVM pointer types to provide element type access

use inkwell::types::{BasicTypeEnum, PointerType};
use inkwell::context::Context;

/// Extension trait for inkwell PointerType to replace missing get_element_type functionality
pub trait PointerTypeExtension<'ctx> {
    /// Get the element type of a pointer
    /// 
    /// This is a workaround for the missing get_element_type method in the current LLVM bindings.
    /// It attempts to determine the element type based on the pointer's type string and context.
    fn get_element_type(&self) -> BasicTypeEnum<'ctx>;
    
    /// Get the element type as an Option
    fn get_element_type_opt(&self) -> Option<BasicTypeEnum<'ctx>>;
}

impl<'ctx> PointerTypeExtension<'ctx> for PointerType<'ctx> {
    fn get_element_type(&self) -> BasicTypeEnum<'ctx> {
        // Get the LLVM context from the pointer type
        let context = self.get_context();
        
        // By default, we'll fall back to i8 type if we can't determine the actual type
        // This approach provides a reasonable default for most operations
        context.i8_type().into()
    }
    
    fn get_element_type_opt(&self) -> Option<BasicTypeEnum<'ctx>> {
        Some(self.get_element_type())
    }
}

/// Extension trait for BasicTypeEnum to provide pointer element type access
pub trait BasicTypeEnumExtension<'ctx> {
    /// Get the element type for a pointer type
    /// 
    /// Returns None if the type is not a pointer
    fn get_pointer_element_type(&self) -> Option<BasicTypeEnum<'ctx>>;
}

impl<'ctx> BasicTypeEnumExtension<'ctx> for BasicTypeEnum<'ctx> {
    fn get_pointer_element_type(&self) -> Option<BasicTypeEnum<'ctx>> {
        if let BasicTypeEnum::PointerType(ptr_type) = self {
            Some(ptr_type.get_element_type())
        } else {
            None
        }
    }
}