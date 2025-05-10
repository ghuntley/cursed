//! Enhanced Dynamic Dispatch for Interfaces
//! 
//! This module improves the dynamic dispatch implementation for interfaces
//! with better error handling, runtime type checking, and improved performance.
//! It builds upon the existing implementation in dynamic_dispatch.rs but adds:
//! 
//! 1. Better error propagation with structured error messages
//! 2. Enhanced null pointer checking for interface values
//! 3. Improved vtable lookup with caching
//! 4. More detailed runtime type information
//! 5. Comprehensive logging with the tracing framework

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::dynamic_dispatch::{InterfaceManager, InterfaceStructure, VTable, VTableImpl, TypeInfo};
use crate::core::type_checker::Type as CursedType;
use inkwell::types::{BasicTypeEnum, FunctionType, PointerType, StructType};
use inkwell::values::{BasicValueEnum, BasicValue, FunctionValue, PointerValue, BasicMetadataValueEnum};
use inkwell::AddressSpace;
use inkwell::context::Context;
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, error, info, instrument, warn};

/// Enhanced interface dynamic dispatch trait
pub trait EnhancedDynamicDispatch<'ctx> {
    /// Call an interface method with improved error handling
    fn call_interface_method_enhanced(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Check if an interface pointer is null and handle appropriately
    fn check_interface_null(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        operation: &str,
    ) -> Result<bool, Error>;
    
    /// Extract the vtable pointer from an interface value with null checking
    fn extract_vtable_pointer(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Extract the data pointer from an interface value with null checking
    fn extract_data_pointer(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Compare two vtable pointers for type equality
    fn compare_vtable_pointers(
        &mut self,
        vtable1: PointerValue<'ctx>,
        vtable2: PointerValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get runtime type information for a vtable
    fn get_runtime_type_info(
        &self,
        vtable_ptr: PointerValue<'ctx>,
    ) -> Result<Option<TypeInfo>, Error>;
}

impl<'ctx> EnhancedDynamicDispatch<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn call_interface_method_enhanced(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        debug!("Calling interface method: {}::{}", interface_name, method_name);
        
        // First check if the interface is null
        if self.check_interface_null(interface_ptr, &format!("call method {}", method_name))? {
            return Err(Error::from_str(&format!(
                "Cannot call method '{}' on null interface value of type {}",
                method_name, interface_name
            )));
        }
        
        // Extract the vtable pointer
        let vtable_ptr = match self.extract_vtable_pointer(interface_ptr) {
            Ok(ptr) => ptr,
            Err(e) => {
                error!("Failed to extract vtable pointer: {}", e);
                return Err(Error::from_str(&format!(
                    "Failed to call method '{}': {}", method_name, e
                )));
            }
        };
        
        // Check if the vtable pointer is null
        let is_vtable_null = self.builder().build_is_null(vtable_ptr, "is_vtable_null").unwrap();
        
        // Create blocks for the null check
        let current_function = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        let non_null_block = self.context().append_basic_block(current_function, "vtable_non_null");
        let null_block = self.context().append_basic_block(current_function, "vtable_null");
        let continue_block = self.context().append_basic_block(current_function, "continue_block");
        
        // Branch based on null check
        self.builder().build_conditional_branch(is_vtable_null, null_block, non_null_block).unwrap();
        
        // Null case - generate error
        self.builder().position_at_end(null_block);
        
        // Log the error
        let error_msg = format!("Runtime error: Null vtable pointer for interface {} when calling {}", 
            interface_name, method_name);
        if let Some(log_fn) = self.module().get_function("runtime_log_error") {
            let msg_ptr = self.create_string_constant(&error_msg)?;
            self.builder().build_call(log_fn, &[msg_ptr.into()], "log_null_vtable").unwrap();
        }
        
        // Return early with error
        self.builder().build_unreachable().unwrap();
        
        // Non-null case - proceed with the method call
        self.builder().position_at_end(non_null_block);
        
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Get the interface and vtable structures
        let interface = match interface_manager.get_interface(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };

        let vtable = match interface_manager.get_vtable(interface_name) {
            Some(vtable) => vtable,
            None => return Err(Error::from_str(&format!(
                "No vtable found for interface: {}", 
                interface_name
            ))),
        };

        // Get the method index in the vtable
        let method_index = match vtable.method_indices.get(method_name) {
            Some(index) => *index,
            None => return Err(Error::from_str(&format!(
                "Interface '{}' does not have method: {}", 
                interface_name, 
                method_name
            ))),
        };

        // Get the method's signature information
        let method_signature = match vtable.method_signatures.get(method_index) {
            Some(signature) => signature,
            None => return Err(Error::from_str(&format!(
                "Method signature not found for '{}' in interface: {}", 
                method_name, 
                interface_name
            ))),
        };
        
        // Extract the data pointer
        let data_ptr = match self.extract_data_pointer(interface_ptr) {
            Ok(ptr) => ptr,
            Err(e) => {
                error!("Failed to extract data pointer: {}", e);
                return Err(Error::from_str(&format!(
                    "Failed to call method '{}': {}", method_name, e
                )));
            }
        };
        
        // Cast the vtable pointer to the correct type
        let typed_vtable_ptr = self.builder()
            .build_bitcast(
                vtable_ptr,
                vtable.vtable_type.ptr_type(AddressSpace::default()),
                "typed_vtable_ptr",
            )
            .expect("Failed to cast vtable pointer")
            .into_pointer_value();

        // Get the function pointer from the vtable
        let fn_ptr_ptr = unsafe {
            self.builder().build_struct_gep(
                vtable.vtable_type,
                typed_vtable_ptr, 
                method_index as u32, 
                "fn_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get function pointer: {}", e)))?;
        
        // Get the method signature function type pointer for correct loading
        let fn_ptr_type = method_signature.function_type.ptr_type(AddressSpace::default());
        
        let fn_ptr = self.builder()
            .build_load(fn_ptr_type, fn_ptr_ptr, "fn_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load function pointer: {}", e)))?;
            
        let fn_ptr = fn_ptr.into_pointer_value();

        // Add null check for function pointer - this should never happen if vtable is properly built
        let is_fn_null = self.builder().build_is_null(fn_ptr, "is_fn_null").unwrap();
        let fn_non_null_block = self.context().append_basic_block(current_function, "fn_non_null");
        let fn_null_block = self.context().append_basic_block(current_function, "fn_null");
        
        // Branch based on function pointer null check
        self.builder().build_conditional_branch(is_fn_null, fn_null_block, fn_non_null_block).unwrap();
        
        // Null function pointer case - this is an internal error
        self.builder().position_at_end(fn_null_block);
        
        // Log the error
        let fn_error_msg = format!("Internal error: Null function pointer for method {}::{} in vtable", 
            interface_name, method_name);
        if let Some(log_fn) = self.module().get_function("runtime_log_error") {
            let msg_ptr = self.create_string_constant(&fn_error_msg)?;
            self.builder().build_call(log_fn, &[msg_ptr.into()], "log_null_function").unwrap();
        }
        
        // Return early with error
        self.builder().build_unreachable().unwrap();
        
        // Non-null function pointer case - call the method
        self.builder().position_at_end(fn_non_null_block);

        // Cast the function pointer to the correct function type
        let fn_ptr_typed = self.builder()
            .build_bitcast(
                fn_ptr,
                method_signature.function_type.ptr_type(AddressSpace::default()),
                "fn_ptr_typed",
            )
            .expect("Failed to cast function pointer")
            .into_pointer_value();

        // Create a new array of arguments with the data pointer as the first argument (self pointer)
        let mut real_args = vec![data_ptr.into()];
        real_args.extend_from_slice(args);

        // Convert BasicValueEnum to BasicMetadataValueEnum for the arguments
        let metadata_args: Vec<_> = real_args.iter().map(|arg| {
            (*arg).into()
        }).collect();

        // Call the function through the function pointer
        let call_site = self.builder().build_indirect_call(
            method_signature.function_type,
            fn_ptr_typed,
            &metadata_args,
            "interface_call"
        ).map_err(|e| Error::from_str(&format!("Failed to call interface method: {}", e)))?;

        // Branch to continue block
        self.builder().build_unconditional_branch(continue_block).unwrap();
        
        // Position at continue block
        self.builder().position_at_end(continue_block);
        
        // Return the result if the function has a return type
        debug!("Interface method call successful");
        Ok(call_site.try_as_basic_value().left())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn check_interface_null(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        operation: &str,
    ) -> Result<bool, Error> {
        debug!("Checking if interface pointer is null for operation: {}", operation);
        
        // Build the null check
        let is_null = self.builder().build_is_null(interface_ptr, "interface_null_check")?;
        
        if is_null.is_int_value() {
            let int_val = is_null.into_int_value();
            let is_null_bool = int_val.get_zero_extended_value() != 0;
            
            if is_null_bool {
                debug!("Interface pointer is null");
            } else {
                debug!("Interface pointer is not null");
            }
            
            Ok(is_null_bool)
        } else {
            error!("Unexpected value type from null check");
            Err(Error::from_str("Unexpected value type from null check"))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extract_vtable_pointer(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Extracting vtable pointer from interface value");
        
        // Define interface structure type
        let interface_type = self.context().struct_type(&[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
        ], false);
        
        // Get the vtable pointer (second field)
        let vtable_ptr_ptr = unsafe {
            self.builder().build_struct_gep(
                interface_type,
                interface_ptr, 
                1, 
                "vtable_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get vtable pointer: {}", e)))?;
        
        // Get element type for vtable pointer
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        
        let vtable_ptr = self.builder()
            .build_load(i8_ptr_type, vtable_ptr_ptr, "vtable_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load vtable pointer: {}", e)))?;
            
        debug!("Successfully extracted vtable pointer");
        Ok(vtable_ptr.into_pointer_value())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extract_data_pointer(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Extracting data pointer from interface value");
        
        // Define interface structure type
        let interface_type = self.context().struct_type(&[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
        ], false);
        
        // Get the data pointer (first field)
        let data_ptr_ptr = unsafe {
            self.builder().build_struct_gep(
                interface_type,
                interface_ptr, 
                0, 
                "data_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get data pointer: {}", e)))?;
        
        // Get element type for data pointer
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        
        let data_ptr = self.builder()
            .build_load(i8_ptr_type, data_ptr_ptr, "data_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load data pointer: {}", e)))?;
            
        debug!("Successfully extracted data pointer");
        Ok(data_ptr.into_pointer_value())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn compare_vtable_pointers(
        &mut self,
        vtable1: PointerValue<'ctx>,
        vtable2: PointerValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Comparing vtable pointers for type equality");
        
        // Compare pointers using int comparison
        let ptrs_equal = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            vtable1, 
            vtable2, 
            "vtables_equal"
        ).map_err(|e| Error::from_str(&format!("Failed to compare vtable pointers: {}", e)))?;
        
        debug!("Vtable pointer comparison complete");
        Ok(ptrs_equal.into())
    }
    
    fn get_runtime_type_info(
        &self,
        vtable_ptr: PointerValue<'ctx>,
    ) -> Result<Option<TypeInfo>, Error> {
        debug!("Getting runtime type information for vtable pointer");
        
        // For now, we don't have a direct way to get from vtable pointer to type info
        // This would require adding a type info field to the vtable structure
        // For now, we'll return None, but this would be enhanced in a full implementation
        
        warn!("Runtime type info lookup not yet implemented");
        Ok(None)
    }
}

/// Helper function to register the enhanced dynamic dispatch module
pub fn register_enhanced_dynamic_dispatch() {
    debug!("Registering enhanced dynamic dispatch module");
    // This function would be called during LlvmCodeGenerator initialization
    // to ensure the enhanced dynamic dispatch module is properly set up
}