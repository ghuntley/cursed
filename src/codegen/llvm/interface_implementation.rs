//! Interface implementation for LLVM code generator
//! 
//! This module adds support for interfaces and dynamic dispatch to the LLVM code generator.
//! It connects the type checker's interface verification with the LLVM code generation
//! process, enabling dynamic method dispatch and interface type checking.

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::dynamic_dispatch::{InterfaceManager, InterfaceStructure, VTable, VTableImpl};
use crate::core::type_checker::Type as CursedType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

/// Trait for implementing interface-related functionality
pub trait InterfaceImplementation<'ctx> {
    /// Register an interface in the code generator
    fn register_interface(
        &mut self,
        name: &str,
        methods: Vec<(String, Vec<CursedType>, Option<CursedType>)>,
        type_params: Vec<String>,
    ) -> Result<(), Error>;
    
    /// Register a struct as implementing an interface
    fn register_interface_implementation(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        methods: HashMap<String, FunctionValue<'ctx>>,
    ) -> Result<(), Error>;
    
    /// Create an interface value from a concrete type
    fn create_interface_value(
        &mut self,
        value: PointerValue<'ctx>,
        value_type: &CursedType,
        interface_name: &str,
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Call a method on an interface value (dynamic dispatch)
    fn call_interface_method(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Check if a value implements an interface at runtime
    fn check_instance_of(
        &mut self,
        interface_value: PointerValue<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile an interface type assertion (cast)
    fn compile_interface_type_assertion(
        &mut self,
        interface_value: PointerValue<'ctx>,
        target_type: &CursedType,
    ) -> Result<PointerValue<'ctx>, Error>;
}

impl<'ctx> InterfaceImplementation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn register_interface(
        &mut self,
        name: &str,
        methods: Vec<(String, Vec<CursedType>, Option<CursedType>)>,
        type_params: Vec<String>,
    ) -> Result<(), Error> {
        // Get the interface manager
        let interface_manager = match &mut self.interface_manager {
            Some(manager) => manager,
            None => {
                // Create a new interface manager if it doesn't exist
                self.interface_manager = Some(InterfaceManager::new());
                self.interface_manager.as_mut().unwrap()
            }
        };
        
        // Register the interface with the manager
        interface_manager.register_interface(
            &self.context,
            name,
            methods,
            type_params,
        )
    }
    
    fn register_interface_implementation(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        methods: HashMap<String, FunctionValue<'ctx>>,
    ) -> Result<(), Error> {
        // Get the interface manager
        let interface_manager = match &mut self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Create the implementing type
        let implementing_type = CursedType::Struct(struct_name.to_string(), Vec::new());
        
        // Create a vtable for the implementation
        interface_manager.create_vtable_for_implementation(
            &self.context,
            &self.module,
            interface_name,
            &implementing_type,
            methods,
        )
    }
    
    fn create_interface_value(
        &mut self,
        value: PointerValue<'ctx>,
        value_type: &CursedType,
        interface_name: &str,
    ) -> Result<PointerValue<'ctx>, Error> {
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Create an interface value
        interface_manager.create_interface_value(
            &self.context,
            &self.builder,
            value,
            value_type,
            interface_name,
        )
    }
    
    fn call_interface_method(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Call the method
        interface_manager.call_interface_method(
            &self.context,
            &self.builder,
            interface_ptr,
            interface_name,
            method_name,
            args,
        )
    }
    
    fn check_instance_of(
        &mut self,
        interface_value: PointerValue<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Check if the value implements the interface
        interface_manager.check_instance_of(
            &self.context,
            &self.builder,
            interface_value,
            target_type_name,
        )
    }
    
    fn compile_interface_type_assertion(
        &mut self,
        interface_value: PointerValue<'ctx>,
        target_type: &CursedType,
    ) -> Result<PointerValue<'ctx>, Error> {
        tracing::debug!("Compiling interface type assertion to {:?}", target_type);
        
        // Extract the target type name
        let type_name = match target_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Type assertion target must be a struct type")),
        };
        
        // Create function blocks for control flow
        let current_function = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let success_block = self.context.append_basic_block(current_function, "type_assert_success");
        let failure_block = self.context.append_basic_block(current_function, "type_assert_failure");
        let end_block = self.context.append_basic_block(current_function, "type_assert_end");
        
        // First check if the interface value is null
        let is_null = self.builder.build_is_null(interface_value, "is_interface_null").unwrap();
        
        // Create a descriptive error message for null interface
        let null_interface_msg = self.create_string_constant(
            &format!("Runtime error: Cannot perform type assertion on null interface value of type {}", type_name)
        )?;
        
        // Handle null interface case
        let null_check_block = self.context.append_basic_block(current_function, "null_check");
        self.builder.build_conditional_branch(is_null, failure_block, null_check_block).unwrap();
        
        // Continue with type checking if interface is not null
        self.builder.position_at_end(null_check_block);
        
        // Check if the interface value is actually of this type
        let is_instance_result = self.check_instance_of(interface_value, type_name)?;
        
        // Create error message for type mismatch
        let type_error_msg = self.create_string_constant(
            &format!("Runtime error: Interface value is not of type {}", type_name)
        )?;
        
        // Branch based on the type check result
        self.builder.build_conditional_branch(
            is_instance_result.into_int_value(),
            success_block,
            failure_block
        ).unwrap();
        
        // Failure block - log detailed error message and return null
        self.builder.position_at_end(failure_block);
        
        // Log the error - add a call to a runtime logging function if available
        if let Some(log_fn) = self.module().get_function("runtime_log_error") {
            // Determine which error message to use based on which block we came from
            let phi = self.builder.build_phi(
                self.context.i8_type().ptr_type(AddressSpace::default()),
                "error_msg"
            ).unwrap();
            
            // Add incoming values from each source block
            let entry_block = self.builder.get_insert_block().unwrap().get_previous_basic_block().unwrap();
            let null_check_block = self.builder.get_insert_block().unwrap().get_previous_basic_block().unwrap();
            
            phi.add_incoming(&[(&null_interface_msg, entry_block), (&type_error_msg, null_check_block)]);
            
            let msg_ptr = phi.as_basic_value().into_pointer_value();
            self.builder.build_call(
                log_fn,
                &[msg_ptr.into()],
                "log_call"
            ).unwrap();
        }
        
        // Return null pointer for the target type
        let null_ptr = self.context.i8_type()
            .ptr_type(AddressSpace::default())
            .const_null();
            
        // Jump to end block
        self.builder.build_unconditional_branch(end_block).unwrap();
        
        // Success block - extract and cast the data pointer
        self.builder.position_at_end(success_block);
        
        // Load the data pointer from the interface value (first field)
        let data_ptr_ptr = unsafe {
            let interface_type = self.context.struct_type(&[
                self.context.i8_type().ptr_type(AddressSpace::default()).into(),
                self.context.i8_type().ptr_type(AddressSpace::default()).into(),
            ], false);
            
            self.builder.build_struct_gep(
                interface_type,
                interface_value,
                0,
                "data_ptr_ptr",
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get data pointer: {}", e)))?;
        
        // Get element type for data pointer
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        let data_ptr = self.builder
            .build_load(i8_ptr_type, data_ptr_ptr, "data_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load data pointer: {}", e)))?;
            
        let data_ptr = data_ptr.into_pointer_value();
        
        // Find or create target LLVM type - create a basic one if not found
        let target_llvm_type = self.context.struct_type(&[], false);
        
        // In a full implementation, we'd look up the struct type in the registry
        tracing::debug!("Creating struct type for target type: {}", type_name);
        
        let target_ptr_type = target_llvm_type.ptr_type(AddressSpace::default());
        
        // Cast the data pointer to the target type
        let casted_value = self.builder
            .build_bitcast(
                data_ptr,
                target_ptr_type,
                "casted_value",
            )
            .expect("Failed to cast value")
            .into_pointer_value();
            
        // Jump to end block
        self.builder.build_unconditional_branch(end_block).unwrap();
        
        // End block - use phi node to select between success and failure results
        self.builder.position_at_end(end_block);
        
        // Create PHI node to select the appropriate result based on the path taken
        let result_phi = self.builder.build_phi(
            self.context.i8_type().ptr_type(AddressSpace::default()),
            "type_assert_result"
        ).unwrap();
        
        // Add the null value from the failure block
        let failure_block_val = self.builder.get_insert_block().unwrap().get_previous_basic_block().unwrap();
        result_phi.add_incoming(&[(&null_ptr, failure_block_val)]);
        
        // Add the casted value from the success block
        let success_block_val = failure_block_val.get_previous_basic_block().unwrap();
        
        // Cast to generic pointer for phi node compatibility
        let casted_generic = self.builder
            .build_bitcast(
                casted_value,
                self.context.i8_type().ptr_type(AddressSpace::default()),
                "casted_generic"
            )
            .expect("Failed to cast to generic pointer")
            .into_pointer_value();
            
        result_phi.add_incoming(&[(&casted_generic, success_block_val)]);
        
        // Get the result as a pointer value
        let result_ptr = result_phi.as_basic_value().into_pointer_value();
        
        // Cast back to the target type
        let final_result = self.builder
            .build_bitcast(
                result_ptr,
                target_ptr_type,
                "final_type_assert_result"
            )
            .expect("Failed to cast result to target type")
            .into_pointer_value();
            
        tracing::debug!("Completed interface type assertion");
        Ok(final_result)
    }
}