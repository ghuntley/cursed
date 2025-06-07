use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::error::Error;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::error::SourceLocation;

/// Trait for implementing interface type assertions in LLVM
pub trait InterfaceTypeAssertion<'ctx> {
    /// Compile a type assertion expression, returning both the converted value and a success flag
    fn compile_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type
    fn check_instance_of(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: Option<SourceLocation>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get the type ID from an interface value's vtable
    fn get_interface_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract the data pointer from an interface value
    fn extract_interface_data_ptr(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Cast a value to an interface type
    fn cast_to_interface_type(
        &mut self,
        value: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a source location if possible
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({})", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Create the result structure (value and true flag)
        let type_id = self.get_type_id(&type_assertion.type_name)?;
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]),
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn check_instance_of(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: Option<SourceLocation>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Ensure the registry is initialized
        let _ = self.ensure_registry_visualization_initialized();
        
        // Get the type ID from the interface value's vtable
        let actual_type_id = self.get_interface_type_id(interface_value)?;
        
        // Get the expected type ID for the target type from the registry
        let expected_type_id_u64 = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type_name).map_err(|err| {
                // Convert to TypeAssertionError with enhanced context
                let assertion_error = TypeAssertionError::new("interface", target_type_name)
                    .with_message(format!("Failed to get type ID from registry: {}", err))
                    .with_target_type_id(self.hash_type_name(target_type_name));
                
                if let Some(loc) = source_location.clone() {
                    Error::TypeAssertion(assertion_error.with_location(loc).into())
                } else {
                    Error::TypeAssertion(assertion_error.into())
                }
            })?,
            None => self.hash_type_name(target_type_name) // Fallback to direct hash if registry not available
        };
        
        let expected_type_id = self.context().i64_type().const_int(expected_type_id_u64, false);
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id,
            "is_instance_of"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(result.into())
    }
    
    fn get_interface_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Interface value is a struct with two fields:
        // 1. Data pointer
        // 2. VTable pointer
        
        // Extract the vtable pointer (assuming it's a pointer to a structure)
        let vtable_ptr_ptr = if interface_value.is_struct_value() {
            // Direct interface value - extract vtable pointer field
            self.builder().build_extract_value(
                interface_value.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value - load and extract vtable pointer
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            self.builder().build_extract_value(
                loaded.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?
        } else {
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_value
            )));
        };
        
        // Load the vtable structure
        let vtable_ptr = vtable_ptr_ptr.into_pointer_value();
        
        // Type ID is the first field in the vtable
        let vtable_type = self.context().struct_type(&[
            self.context().i64_type().into(), // type ID
            self.context().i32_type().into(), // method count
            self.context().i8_type().ptr_type(AddressSpace::default()).into(), // methods table
        ], false);
        
        let type_id_ptr = self.builder().build_struct_gep(
            vtable_type,
            vtable_ptr,
            0, // Index of type ID field
            "type_id_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Load the type ID
        let type_id = self.builder().build_load(
            self.context().i64_type(),
            type_id_ptr,
            "type_id"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(type_id)
    }
    
    fn extract_interface_data_ptr(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        // Extract the data pointer (first field of interface value)
        let data_ptr = if interface_value.is_struct_value() {
            // Direct interface value
            self.builder().build_extract_value(
                interface_value.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            self.builder().build_extract_value(
                loaded.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?
        } else {
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_value
            )));
        };
        
        Ok(data_ptr.into_pointer_value())
    }
    
    fn cast_to_interface_type(
        &mut self,
        value: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // For now, this is a simple implementation that just returns the value
        // In a more sophisticated implementation, this would:
        // 1. Create an interface struct with the value and proper vtable
        // 2. Handle proper type conversion if needed
        // 3. Set up the correct vtable for the target interface type
        
        // Get the type ID for the target interface type
        let type_id = self.get_type_id(type_name)?;
        
        // Create a simple interface structure with data pointer and type ID
        // This is a simplified implementation - in practice you'd need proper vtable setup
        if value.is_pointer_value() {
            // Value is already a pointer, can use directly as data pointer
            let interface_struct = self.build_tuple(vec![
                value, // data pointer
                type_id, // type ID (simplified vtable)
            ])?;
            Ok(interface_struct)
        } else {
            // Need to allocate space for the value and get a pointer to it
            let alloca = self.builder().build_alloca(value.get_type(), "cast_temp")
                .map_err(|e| Error::Compilation(e.to_string()))?;
            self.builder().build_store(alloca, value)
                .map_err(|e| Error::Compilation(e.to_string()))?;
            
            let interface_struct = self.build_tuple(vec![
                alloca.into(), // data pointer
                type_id, // type ID (simplified vtable)
            ])?;
            Ok(interface_struct)
        }
    }
    
}

// Helper methods extension
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Make these methods public so they can be used by the error propagation implementation
    // Get a type ID for a given type name
    pub fn get_type_id(&mut self, type_name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // For now, use a hash of the type name as a simple approximation
        // In a real implementation, this would use proper runtime type information
        let hash = self.hash_type_name(type_name);
        let type_id = self.context().i64_type().const_int(hash, false);
        Ok(type_id.into())
    }
    
    // Simple hash function for type names
    pub fn hash_type_name(&self, type_name: &str) -> u64 {
        // FNV-1a hash algorithm
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
    
    // Build a tuple structure (for returning value and success flag)
    pub fn build_tuple(&mut self, values: Vec<BasicValueEnum<'ctx>>) -> Result<BasicValueEnum<'ctx>, Error> {
        let ctx = self.context();
        let tuple_type = ctx.struct_type(
            &values.iter().map(|v| v.get_type()).collect::<Vec<_>>(),
            false
        );
        
        let mut tuple = tuple_type.const_named_struct(&[]);
        for (i, value) in values.iter().enumerate() {
            let inserted = self.builder().build_insert_value(tuple, *value, i as u32, "tuple_insert")
            .map_err(|e| Error::Compilation(e.to_string()))?;
              tuple = inserted.into_struct_value();
        }
        
        Ok(tuple.into())
    }
    
    // Get tuple type from a list of element types
    pub fn tuple_type(&self, element_types: Vec<BasicTypeEnum<'ctx>>) -> StructType<'ctx> {
        self.context().struct_type(&element_types, false)
    }
    
    // Helper for getting pointer type with default address space
    pub fn pointer_type(&self) -> inkwell::types::PointerType<'ctx> {
        self.context().i8_type().ptr_type(AddressSpace::default())
    }
}