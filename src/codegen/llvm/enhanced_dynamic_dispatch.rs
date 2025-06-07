use crate::error::Error;
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;
use crate::codegen::llvm::pointer_type_extension::PointerTypeExtension;
use inkwell::values::{BasicValueEnum, PointerValue};

/// Enhanced dynamic dispatch for interfaces
/// 
/// This trait provides enhanced dynamic dispatch capabilities for interfaces,
/// allowing more efficient method calls by using type information to bypass
/// the vtable lookup when possible.
pub trait EnhancedDynamicDispatch<'ctx> {
    /// Initialize the enhanced dynamic dispatch system
    fn init_dynamic_dispatch(&mut self) -> Result<(), Error>;
    
    /// Extract the vtable pointer from an interface value
    fn extract_vtable_pointer(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Extract the data pointer from an interface value
    fn extract_data_pointer(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Check if an interface value is null
    fn check_interface_null(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>,
        error_context: &str
    ) -> Result<bool, Error>;
    
    /// Compare two vtable pointers for equality
    fn compare_vtable_pointers(
        &mut self,
        vtable_ptr1: PointerValue<'ctx>,
        vtable_ptr2: PointerValue<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Enhanced version of calling an interface method
    fn call_interface_method_enhanced(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: Vec<BasicValueEnum<'ctx>>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

// Helper function to register the enhanced dynamic dispatch module
pub fn register_enhanced_dynamic_dispatch() {
    tracing::debug!("Registering enhanced dynamic dispatch module");
}

// Implementation for LlvmCodeGenerator
impl<'ctx> EnhancedDynamicDispatch<'ctx> for crate::codegen::llvm::LlvmCodeGenerator<'ctx> {
    fn init_dynamic_dispatch(&mut self) -> Result<(), Error> {
        // This initializes any cached state needed for the enhanced dispatch system
        Ok(())
    }
    
    fn extract_vtable_pointer(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        // Similar to get_interface_type_id but just returns the vtable pointer
        // Extract the vtable pointer (field 1 of interface struct)
        if interface_ptr.is_struct_value() {
            // Direct interface value - extract vtable pointer field
            let vtable_ptr = self.builder().build_extract_value(
                interface_ptr.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            Ok(vtable_ptr.into_pointer_value())
        } else if interface_ptr.is_pointer_value() {
            // Pointer to interface value - load and extract vtable pointer
            let interface_type = interface_ptr.into_pointer_value().get_type().get_element_type();
            let loaded = self.builder().build_load(
                interface_type,
                interface_ptr.into_pointer_value(),
                "interface_value"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            let vtable_ptr = self.builder().build_extract_value(
                loaded.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            Ok(vtable_ptr.into_pointer_value())
        } else {
            Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_ptr
            )))
        }
    }
    
    fn extract_data_pointer(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        // Delegate to the existing implementation
        if let Some(type_assertion) = self.type_assertion_implementation.as_mut() {
            type_assertion.extract_interface_data_ptr(interface_ptr)
        } else {
            Err(Error::Compilation("Type assertion implementation not initialized".to_string()))
        }
    }
    
    fn check_interface_null(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>,
        error_context: &str
    ) -> Result<bool, Error> {
        // For a pointer, check if it's null
        if interface_ptr.is_pointer_value() {
            let is_null = self.builder().build_is_null(
                interface_ptr.into_pointer_value(),
                "is_null"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            // Check if the value is a constant and extract its value
            if let Some(const_val) = is_null.get_zero_extended_constant() {
                return Ok(const_val != 0);
            }
            
            // For non-constant values, we can't determine at compile time
            return Ok(false);
        }
        
        // For a struct, check if both pointers are null
        if interface_ptr.is_struct_value() {
            // Extract data pointer
            let data_ptr = self.builder().build_extract_value(
                interface_ptr.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            // Check if data pointer is null
            let is_null = self.builder().build_is_null(
                data_ptr.into_pointer_value(),
                "is_data_null"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            // Check if the value is a constant and extract its value
            if let Some(const_val) = is_null.get_zero_extended_constant() {
                return Ok(const_val != 0);
            }
            
            // For non-constant values, we can't determine at compile time
            return Ok(false);
        }
        
        Err(Error::Compilation(format!(
            "Cannot check if interface is null for {:?} in {}",
            interface_ptr, error_context
        )))
    }
    
    fn compare_vtable_pointers(
        &mut self,
        vtable_ptr1: PointerValue<'ctx>,
        vtable_ptr2: PointerValue<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Compare pointers for equality using pointer comparison
        let is_equal = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            vtable_ptr1,
            vtable_ptr2,
            "vtables_equal"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(is_equal.into())
    }
    
    fn call_interface_method_enhanced(
        &mut self,
        interface_ptr: BasicValueEnum<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: Vec<BasicValueEnum<'ctx>>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // For now, delegate to the standard interface method call
        // Convert BasicValueEnum to PointerValue if it's a pointer
        let ptr = if let BasicValueEnum::PointerValue(ptr) = interface_ptr {
            ptr
        } else {
            return Err(Error::from_str("Interface value must be a pointer"));
        };
        
        // Call with correct signature and handle Option result
        let result = self.call_interface_method(ptr, interface_name, method_name, &args)?;
        match result {
            Some(value) => Ok(value),
            None => Err(Error::from_str("Interface method returned no value")),
        }
    }
}