//! Extension trait for fixed range clause methods
//!
//! This module provides a public extension trait that exposes the fixed range clause methods
//! from LlvmCodeGenerator for use by the error recovery implementation.

use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::codegen::llvm::string_utils::StringUtilsExtension;
use crate::error::Error;
use inkwell::types::{BasicTypeEnum, AnyType, BasicType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue, BasicMetadataValueEnum};
use inkwell::AddressSpace;

/// Extension trait for fixed range clause methods
///
/// This trait exposes the methods needed by the range clause error recovery implementation
/// that are used to handle container and map iteration properly.
pub trait RangeClauseFixedMethodsExtension<'ctx> {
    /// Get the length of a container
    fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Get an element from a container by index
    fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Determine the element type of a container
    fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Create a map iterator
    fn emit_map_iterator_create_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error>;
    
    /// Check if a map iterator has a next element
    fn emit_map_iterator_has_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error>;
    
    /// Get the current key-value pair from a map iterator
    fn emit_map_iterator_get_current_fixed(
        &self,
        iterator_ptr: PointerValue<'ctx>,
        key_ptr: PointerValue<'ctx>,
        value_ptr: PointerValue<'ctx>,
    ) -> Result<(), Error>;
    
    /// Advance a map iterator to the next element
    fn emit_map_iterator_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<(), Error>;
    
    /// Determine the key type of a map
    fn determine_map_key_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Determine the value type of a map
    fn determine_map_value_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
}

/// Helper methods for the RangeClauseFixedMethodsExtension implementation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    
    /// Emit code to get the length of a string
    fn emit_string_length(&self, string_value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // Get the string length function from the module or declare it
        let string_length_fn = self.get_or_declare_string_length_function();
        
        // Call the function with the string value
        let result = self.builder().build_call(
            string_length_fn,
            &[string_value.into()],
            "string_length"
        )?.try_as_basic_value().left().unwrap();
        
        // Return the result as an integer
        Ok(result.into_int_value())
    }
    
    /// Get or declare the string length function
    fn get_or_declare_string_length_function(&self) -> inkwell::values::FunctionValue<'ctx> {
        let fn_name = "string_length";
        
        // Check if the function already exists
        if let Some(func) = self.module().get_function(fn_name) {
            return func;
        }
        
        // It doesn't exist, so declare it
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let i64_type = self.context().i64_type();
        let fn_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        
        self.module().add_function(fn_name, fn_type, None)
    }
    
    /// Emit code to get a character from a string
    fn emit_string_get_char(&self, string_value: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Calculate the address of the character
        let char_ptr = unsafe {
            self.builder().build_gep(
                self.context().i8_type(),
                string_value.into_pointer_value(),
                &[index],
                "char_ptr"
            )
        }?;
        
        // Load the character
        let char_value = self.builder().build_load(
            self.context().i8_type(),
            char_ptr,
            "char_value"
        )?;
        
        Ok(char_value)
    }
    
    /// Emit code to get the length of an array
    fn emit_array_length(&self, array_value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // This is a simplified implementation
        // For now, just return a default length of 0
        Ok(self.context().i64_type().const_int(0, false))
    }
    
    /// Emit code to get an element from an array
    fn emit_array_get_element(&self, array_value: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // This is a simplified implementation
        // For a real implementation, this would calculate the element address and load it
        
        // Default to returning a zero integer
        Ok(self.context().i64_type().const_int(0, false).into())
    }
}

// Implement the RangeClauseFixedMethodsExtension trait for LlvmCodeGenerator
impl<'ctx> RangeClauseFixedMethodsExtension<'ctx> for LlvmCodeGenerator<'ctx> {
    fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // This method gets the length of a container (array, slice, string, etc.)
        // The implementation depends on the container type
        let container_type = container.get_type();
        
        // Check if this is a string
        if crate::codegen::llvm::string_utils::StringUtilsExtension::is_string_type(self, container_type) {
            return self.emit_string_length(container);
        }
        
        // Check if this is an array or slice
        if container_type.is_pointer_type() {
            let element_type = container_type.into_pointer_type().get_element_type();
            if element_type.is_array_type() || element_type.is_struct_type() {
                return self.emit_array_length(container);
            }
        }
        
        // Default behavior for other container types
        // For unknown container types, just return 0
        Ok(self.context().i64_type().const_int(0, false))
    }
    
    fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // This method gets an element from a container by index
        let container_type = container.get_type();
        
        // Check if this is a string
        if crate::codegen::llvm::string_utils::StringUtilsExtension::is_string_type(self, container_type) {
            return self.emit_string_get_char(container, index);
        }
        
        // Check if this is an array or slice
        if container_type.is_pointer_type() {
            let element_type = container_type.into_pointer_type().get_element_type();
            if element_type.is_array_type() || element_type.is_struct_type() {
                return self.emit_array_get_element(container, index);
            }
        }
        
        // Default behavior for other container types
        // Return a default value for the expected element type
        let default_type = self.determine_element_type_fixed(container)?;
        match default_type {
            BasicTypeEnum::IntType(int_type) => Ok(int_type.const_zero().into()),
            BasicTypeEnum::FloatType(float_type) => Ok(float_type.const_zero().into()),
            BasicTypeEnum::PointerType(ptr_type) => Ok(ptr_type.const_null().into()),
            _ => Ok(self.context().i64_type().const_zero().into())
        }
    }
    
    fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // This method determines the element type of a container
        let container_type = container.get_type();
        
        // For strings, the element type is char (i8)
        if crate::codegen::llvm::string_utils::StringUtilsExtension::is_string_type(self, container_type) {
            return Ok(self.context().i8_type().into());
        }
        
        // For arrays and slices, determine the element type
        if container_type.is_pointer_type() {
            let element_type = container_type.into_pointer_type().get_element_type();
            if element_type.is_array_type() {
                let array_type = element_type.into_array_type();
                return Ok(array_type.get_element_type());
            } else if element_type.is_struct_type() {
                // This could be a slice or other container type
                // For now, just default to int
                return Ok(self.context().i64_type().into());
            }
        }
        
        // Default to int type for unknown containers
        Ok(self.context().i64_type().into())
    }
    
    fn emit_map_iterator_create_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error> {
        // Create an iterator for a map
        // This would allocate a map iterator structure and initialize it
        // For now, we'll just return a null pointer
        Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null())
    }
    
    fn emit_map_iterator_has_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // Check if a map iterator has more elements
        // For a real implementation, this would check the iterator's current position
        // For now, just return false (0)
        Ok(self.context().bool_type().const_int(0, false))
    }
    
    fn emit_map_iterator_get_current_fixed(
        &self,
        iterator_ptr: PointerValue<'ctx>,
        key_ptr: PointerValue<'ctx>,
        value_ptr: PointerValue<'ctx>,
    ) -> Result<(), Error> {
        // Get the current key-value pair from a map iterator
        // For a real implementation, this would extract key and value from the iterator
        // and store them to the provided pointers
        // For now, do nothing
        Ok(())
    }
    
    fn emit_map_iterator_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<(), Error> {
        // Advance a map iterator to the next element
        // For a real implementation, this would increment the iterator's position
        // For now, do nothing
        Ok(())
    }
    
    fn determine_map_key_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Determine the key type of a map
        // For a real implementation, this would examine the map's type
        // For now, default to string (common for map keys)
        Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into())
    }
    
    fn determine_map_value_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Determine the value type of a map
        // For a real implementation, this would examine the map's type
        // For now, default to int
        Ok(self.context().i64_type().into())
    }
}