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
        // Extract the target type name
        let type_name = match target_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Type assertion target must be a struct type")),
        };
        
        // Check if the interface value is actually of this type
        let is_instance_result = self.check_instance_of(interface_value, type_name)?;
        
        // TODO: In a more complete implementation, we'd generate runtime checks here
        // and handle type errors properly, but for now we'll just assume the assertion is valid
        
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
        
        // Cast to the target type
        let target_llvm_type = match target_type {
            CursedType::Struct(name, _) => {
                // In a real implementation, we'd look up the struct type in a registry
                // For now, we'll just create a dummy struct type for demonstration
                self.context.struct_type(&[], false)
            },
            _ => return Err(Error::from_str("Target type for assertion must be a struct")),
        };
        
        let target_ptr_type = target_llvm_type.ptr_type(AddressSpace::default());
        
        let casted_value = self.builder
            .build_bitcast(
                data_ptr,
                target_ptr_type,
                "casted_value",
            )
            .expect("Failed to cast value")
            .into_pointer_value();
        
        Ok(casted_value)
    }
}