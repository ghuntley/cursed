//! # Integrated Interface Operations
//! 
//! This module provides a unified approach to interface operations by integrating
//! the enhanced dynamic dispatch and type assertion functionality into a single
//! coherent system with proper error handling and performance optimizations.
//!
//! The implementation aims to resolve the fragmentation in the interface implementation
//! by providing a single source of truth for interface operations including:
//! 1. Method dispatch with proper error handling
//! 2. Type assertions with comprehensive null checking
//! 3. Interface creation with runtime type information
//! 4. Better integration with the type checker

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
#[cfg(feature = "enhanced_dynamic_dispatch")]
use crate::codegen::llvm::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;
use crate::codegen::llvm::type_assertion_implementation::IntegratedTypeAssertion;
use crate::core::type_checker::Type as CursedType;
use crate::codegen::llvm::string_utils::StringUtilsExtension;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;
use tracing::{debug, error, info, instrument, trace, warn};

/// Trait for unified interface operations
pub trait IntegratedInterfaceOperations<'ctx> {
    /// Call an interface method with comprehensive error handling
    fn integrated_call_interface_method(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Perform a type assertion with proper error propagation
    fn integrated_type_assertion(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        target_type: &CursedType,
        emit_runtime_error: bool,
    ) -> Result<(PointerValue<'ctx>, BasicValueEnum<'ctx>), Error>;
    
    /// Create an interface value with runtime type information
    fn integrated_create_interface(
        &mut self,
        value_ptr: PointerValue<'ctx>,
        value_type: &CursedType,
        interface_name: &str,
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Check if a type implements an interface with detailed error information
    fn integrated_check_implements_interface(
        &mut self,
        value_type: &CursedType,
        interface_type: &CursedType,
    ) -> Result<bool, Error>;
}

impl<'ctx> IntegratedInterfaceOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, args), level = "debug")]
    fn integrated_call_interface_method(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        debug!("Performing integrated interface method call: {}::{}", interface_name, method_name);
        
        // First check if the enhanced dynamic dispatch is available
        #[cfg(feature = "enhanced_dynamic_dispatch")]
        {
            // Use the enhanced implementation
            debug!("Using enhanced dynamic dispatch");
            return self.call_interface_method_enhanced(interface_ptr, interface_name, method_name, args);
        }
        
        // Fall back to the standard implementation
        debug!("Using standard dynamic dispatch");
        self.call_interface_method(interface_ptr, interface_name, method_name, args)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn integrated_type_assertion(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        target_type: &CursedType,
        emit_runtime_error: bool,
    ) -> Result<(PointerValue<'ctx>, BasicValueEnum<'ctx>), Error> {
        debug!("Performing integrated type assertion to {:?}", target_type);
        
        // Extract type name for error reporting
        let type_name = match target_type {
            CursedType::Struct(name, _) => name.clone(),
            _ => return Err(Error::from_str("Type assertion target must be a struct type")),
        };

        // First check if the interface pointer is null
        if self.check_interface_null(interface_ptr, &format!("type assertion to {}", type_name))? {
            if emit_runtime_error {
                // Create null interface error message
                let null_error_msg = format!("Runtime error: Cannot perform type assertion on null interface value");
                
                // Log the error message
                if let Some(log_fn) = self.module().get_function("runtime_log_error") {
                    let msg_ptr = self.create_string_constant(&null_error_msg)?;
                    self.builder().build_call(log_fn, &[msg_ptr.into()], "log_null_interface").unwrap();
                }
            }
            
            debug!("Null interface pointer in type assertion");
            
            // Return null pointer and false success flag
            let null_ptr = self.context().i8_type()
                .ptr_type(AddressSpace::default())
                .const_null();
                
            let success_flag = self.context().bool_type().const_int(0, false);
            
            return Ok((null_ptr, success_flag.into()));
        }
        
        // Create function blocks for control flow
        let current_function = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        let success_block = self.context().append_basic_block(current_function, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_function, "type_assert_failure");
        let end_block = self.context().append_basic_block(current_function, "type_assert_end");
        
        // Extract vtable pointer
        let vtable_ptr = match self.extract_vtable_pointer(interface_ptr) {
            Ok(ptr) => ptr,
            Err(e) => {
                error!("Failed to extract vtable pointer: {}", e);
                
                if emit_runtime_error {
                    // Log the error
                    if let Some(log_fn) = self.module().get_function("runtime_log_error") {
                        let msg_ptr = self.create_string_constant(
                            &format!("Runtime error: Failed to extract vtable pointer: {}", e)
                        )?;
                        self.builder().build_call(log_fn, &[msg_ptr.into()], "log_vtable_error").unwrap();
                    }
                }
                
                // Return null pointer and false success flag
                let null_ptr = self.context().i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null();
                    
                let success_flag = self.context().bool_type().const_int(0, false);
                
                return Ok((null_ptr, success_flag.into()));
            }
        };
        
        // Check if the vtable pointer is null
        let is_vtable_null = self.builder().build_is_null(vtable_ptr, "is_vtable_null").unwrap();
        
        // Handle null vtable case
        self.builder().build_conditional_branch(is_vtable_null, failure_block, success_block).unwrap();
        
        // Failure block - log detailed error message and return null with false flag
        self.builder().position_at_end(failure_block);
        
        if emit_runtime_error {
            // Create vtable null error message
            let vtable_error_msg = format!("Runtime error: Null vtable pointer in interface value");
            
            // Log the error message
            if let Some(log_fn) = self.module().get_function("runtime_log_error") {
                let msg_ptr = self.create_string_constant(&vtable_error_msg)?;
                self.builder().build_call(log_fn, &[msg_ptr.into()], "log_null_vtable").unwrap();
            }
        }
        
        // Return null pointer and false success flag
        let null_ptr = self.context().i8_type()
            .ptr_type(AddressSpace::default())
            .const_null();
            
        let failure_result_ptr = null_ptr;
        let failure_success_flag = self.context().bool_type().const_int(0, false);
        
        // Jump to end block
        self.builder().build_unconditional_branch(end_block).unwrap();
        
        // Success block - extract data pointer and perform the type check
        self.builder().position_at_end(success_block);
        
        // Get the type name
        let type_name = match target_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Type assertion target must be a struct type")),
        };
        
        // Check if the interface value is of this type
        let type_check_result = self.check_instance_of(interface_ptr, type_name)?;
        
        // Create blocks for the type check result
        let type_match_block = self.context().append_basic_block(current_function, "type_match");
        let type_mismatch_block = self.context().append_basic_block(current_function, "type_mismatch");
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            type_check_result.into_int_value(),
            type_match_block,
            type_mismatch_block
        ).unwrap();
        
        // Type match block - extract the data pointer
        self.builder().position_at_end(type_match_block);
        
        // Extract the data pointer
        let data_ptr = match self.extract_data_pointer(interface_ptr) {
            Ok(ptr) => ptr,
            Err(e) => {
                error!("Failed to extract data pointer: {}", e);
                
                if emit_runtime_error {
                    // Log the error
                    if let Some(log_fn) = self.module().get_function("runtime_log_error") {
                        let msg_ptr = self.create_string_constant(
                            &format!("Runtime error: Failed to extract data pointer: {}", e)
                        )?;
                        self.builder().build_call(log_fn, &[msg_ptr.into()], "log_data_ptr_error").unwrap();
                    }
                }
                
                // Return null pointer and false success flag
                let null_ptr = self.context().i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null();
                    
                let success_flag = self.context().bool_type().const_int(0, false);
                
                return Ok((null_ptr, success_flag.into()));
            }
        };
        
        // Cast the data pointer to the target type
        let target_ptr = self.builder().
            build_bitcast(
                data_ptr,
                self.context().i8_type().ptr_type(AddressSpace::default()),
                "casted_value"
            )
            .expect("Failed to cast value")
            .into_pointer_value();
            
        let match_success_flag = self.context().bool_type().const_int(1, false);
        
        // Jump to end block
        self.builder().build_unconditional_branch(end_block).unwrap();
        
        // Type mismatch block - log error and return null with false flag
        self.builder().position_at_end(type_mismatch_block);
        
        if emit_runtime_error {
            // Create type mismatch error message
            let type_error_msg = format!("Runtime error: Interface value is not of type {}", type_name);
            
            // Log the error message
            if let Some(log_fn) = self.module().get_function("runtime_log_error") {
                let msg_ptr = self.create_string_constant(&type_error_msg)?;
                self.builder().build_call(log_fn, &[msg_ptr.into()], "log_type_mismatch").unwrap();
            }
        }
        
        // Return null pointer and false success flag
        let mismatch_result_ptr = self.context().i8_type()
            .ptr_type(AddressSpace::default())
            .const_null();
            
        let mismatch_success_flag = self.context().bool_type().const_int(0, false);
        
        // Jump to end block
        self.builder().build_unconditional_branch(end_block).unwrap();
        
        // End block - use phi nodes to select the appropriate results
        self.builder().position_at_end(end_block);
        
        // Create PHI node for the result pointer
        let result_ptr_phi = self.builder().build_phi(
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "type_assert_result_ptr"
        ).unwrap();
        
        // Create PHI node for the success flag
        let success_flag_phi = self.builder().build_phi(
            self.context().bool_type(),
            "type_assert_success_flag"
        ).unwrap();
        
        // Get the predecessor blocks
        let failure_block_term = self.builder().get_insert_block().unwrap().get_previous_basic_block().unwrap();
        let type_mismatch_block_term = failure_block_term.get_previous_basic_block().unwrap();
        let type_match_block_term = type_mismatch_block_term.get_previous_basic_block().unwrap();
        
        // Add incoming values to the PHI nodes
        result_ptr_phi.add_incoming(&[
            (&failure_result_ptr, failure_block_term),
            (&mismatch_result_ptr, type_mismatch_block_term),
            (&target_ptr, type_match_block_term)
        ]);
        
        success_flag_phi.add_incoming(&[
            (&failure_success_flag, failure_block_term),
            (&mismatch_success_flag, type_mismatch_block_term),
            (&match_success_flag, type_match_block_term)
        ]);
        
        // Get the final result values
        let final_result_ptr = result_ptr_phi.as_basic_value().into_pointer_value();
        let final_success_flag = success_flag_phi.as_basic_value();
        
        debug!("Completed integrated type assertion");
        Ok((final_result_ptr, final_success_flag))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn integrated_create_interface(
        &mut self,
        value_ptr: PointerValue<'ctx>,
        value_type: &CursedType,
        interface_name: &str,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Creating interface value for type {:?} implementing {}", value_type, interface_name);
        
        // Use the existing implementation (could be enhanced in the future)
        self.create_interface_value(value_ptr, value_type, interface_name)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn integrated_check_implements_interface(
        &mut self,
        value_type: &CursedType,
        interface_type: &CursedType,
    ) -> Result<bool, Error> {
        debug!("Checking if {:?} implements interface {:?}", value_type, interface_type);
        
        // Extract interface name
        let interface_name = match interface_type {
            CursedType::Interface(name, _) => name,
            _ => return Err(Error::from_str("Second type must be an interface")),
        };
        
        // Extract type name for value type
        let type_name = match value_type {
            CursedType::Struct(name, _) => name,
            _ => {
                // Non-struct types can't implement interfaces
                debug!("Non-struct type {:?} cannot implement interfaces", value_type);
                return Ok(false);
            }
        };
        
        // Check if a vtable implementation exists for this type and interface
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => {
                // If no interface manager exists, no implementations exist
                debug!("No interface manager initialized");
                return Ok(false);
            }
        };
        
        // Check if the implementation exists
        let implements = interface_manager.get_vtable_impl(interface_name, type_name).is_some();
        
        debug!("{} {} interface {}", type_name, 
               if implements { "implements" } else { "does not implement" }, 
               interface_name);
        
        Ok(implements)
    }
}

/// Initialize the integrated interface operations system
pub fn initialize_integrated_interface_operations() {
    debug!("Initializing integrated interface operations");
    // This function would be called during LlvmCodeGenerator initialization
    // to ensure the integrated interface operations are properly set up
}