//! Automatic code generation for interface method dispatching
//! 
//! This module implements automatic code generation for interface method dispatching,
//! making it easier to use interfaces in CURSED by automatically handling the
//! vtable setup and method dispatch logic.

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::dynamic_dispatch::{InterfaceManager, VTableImpl};
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;
#[cfg(feature = "enhanced_dynamic_dispatch")]
use crate::codegen::llvm::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
use crate::core::type_checker::Type as CursedType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;
use tracing::{debug, info, instrument, error};

/// Trait for automatic generation of interface dispatch code
pub trait AutoInterfaceDispatcher<'ctx> {
    /// Auto-generate interface implementation for a struct
    /// 
    /// This function takes a struct and an interface it implements, and automatically
    /// generates the necessary code to connect the struct's methods to the interface
    /// methods through the vtable mechanism.
    fn auto_generate_interface_implementation(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        struct_methods: HashMap<String, FunctionValue<'ctx>>,
    ) -> Result<(), Error>;
    
    /// Auto-generate method dispatching code for an interface method call
    ///
    /// This function generates optimized code for calling a method on an interface
    /// value, handling all the vtable lookup and dynamic dispatch logic.
    fn auto_generate_method_dispatch(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Auto-generate direct dispatch when the concrete type is known
    ///
    /// This is an optimization that bypasses the vtable lookup when the concrete
    /// type of an interface value is known at compile time.
    fn auto_generate_direct_dispatch(
        &mut self,
        value: PointerValue<'ctx>,
        concrete_type: &CursedType,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Register all methods from a struct as implementing an interface
    /// 
    /// Automatically finds and registers all methods from a struct that match
    /// the interface's method signatures.
    fn auto_register_struct_methods(
        &mut self,
        struct_name: &str,
        interface_name: &str,
    ) -> Result<(), Error>;
}

impl<'ctx> AutoInterfaceDispatcher<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, struct_methods), fields(struct_name = %struct_name, interface_name = %interface_name))]
    fn auto_generate_interface_implementation(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        struct_methods: HashMap<String, FunctionValue<'ctx>>,
    ) -> Result<(), Error> {
        debug!("Auto-generating interface implementation for {} methods", struct_methods.keys().len());
        
        // 1. Get the interface definition to verify method signatures
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        let interface = match interface_manager.get_interface(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };
        
        // 2. Verify that all required methods are provided
        let mut missing_methods = Vec::new();
        for (method_name, _, _) in &interface.methods {
            if !struct_methods.contains_key(method_name) {
                missing_methods.push(method_name.clone());
            }
        }
        
        if !missing_methods.is_empty() {
            return Err(Error::from_str(&format!(
                "Struct '{}' is missing methods required by interface '{}': {}",
                struct_name,
                interface_name,
                missing_methods.join(", ")
            )));
        }
        
        // 3. Register the implementation with the interface manager
        self.register_interface_implementation(
            struct_name,
            interface_name,
            struct_methods,
        )?;
        
        info!("Successfully registered {} as implementing {}", struct_name, interface_name);
        Ok(())
    }
    
    #[instrument(skip(self, interface_ptr, args), fields(interface_name = %interface_name, method_name = %method_name))]
    fn auto_generate_method_dispatch(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        debug!("Auto-generating method dispatch code");
        
        // This is an optimized version of the call_interface_method function
        // that generates more efficient code for method dispatching
        
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
        
        // Load the data pointer from the interface value
        let data_ptr_ptr = unsafe {
            self.builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                0, 
                "data_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get data pointer: {}", e)))?;
        
        // Get element type for data pointer
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        
        let data_ptr = self.builder
            .build_load(i8_ptr_type, data_ptr_ptr, "data_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load data pointer: {}", e)))?;
        
        let data_ptr = data_ptr.into_pointer_value();
        
        // Load the vtable pointer from the interface value
        let vtable_ptr_ptr = unsafe {
            self.builder.build_struct_gep(
                interface.interface_type,
                interface_ptr, 
                1, 
                "vtable_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get vtable pointer: {}", e)))?;
        
        // Get element type for vtable pointer
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        
        let vtable_ptr = self.builder
            .build_load(i8_ptr_type, vtable_ptr_ptr, "vtable_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load vtable pointer: {}", e)))?;
            
        let vtable_ptr = vtable_ptr.into_pointer_value();
        
        // Cast the vtable pointer to the correct type
        let typed_vtable_ptr = self.builder
            .build_bitcast(
                vtable_ptr,
                vtable.vtable_type.ptr_type(inkwell::AddressSpace::default()),
                "typed_vtable_ptr",
            )
            .expect("Failed to cast vtable pointer")
            .into_pointer_value();
        
        // Get the function pointer from the vtable
        let fn_ptr_ptr = unsafe {
            self.builder.build_struct_gep(
                vtable.vtable_type,
                typed_vtable_ptr, 
                method_index as u32, 
                "fn_ptr_ptr"
            )
        }.map_err(|e| Error::from_str(&format!("Failed to get function pointer: {}", e)))?;
        
        // Get the method signature function type pointer for correct loading
        let fn_ptr_type = method_signature.function_type.ptr_type(inkwell::AddressSpace::default());
        
        let fn_ptr = self.builder
            .build_load(fn_ptr_type, fn_ptr_ptr, "fn_ptr")
            .map_err(|e| Error::from_str(&format!("Failed to load function pointer: {}", e)))?;
            
        let fn_ptr = fn_ptr.into_pointer_value();
        
        // Cast the function pointer to the correct function type if needed
        let fn_ptr_typed = fn_ptr;
        
        // Create a new array of arguments with the data pointer as the first argument (self pointer)
        let mut real_args = vec![data_ptr.into()];
        real_args.extend_from_slice(args);
        
        // Convert BasicValueEnum to BasicMetadataValueEnum for the arguments
        let metadata_args: Vec<_> = real_args.iter().map(|arg| {
            (*arg).into()
        }).collect();
        
        debug!("Calling interface method {} on {}", method_name, interface_name);
        
        // Call the function through the function pointer
        let call_site = self.builder.build_indirect_call(
            method_signature.function_type,
            fn_ptr_typed,
            &metadata_args,
            "interface_call"
        ).map_err(|e| Error::from_str(&format!("Failed to call interface method: {}", e)))?;
        
        // Return the result if the function has a return type
        Ok(call_site.try_as_basic_value().left())
    }
    
    #[instrument(skip(self, value, args), fields(concrete_type = ?concrete_type, interface_name = %interface_name, method_name = %method_name))]
    fn auto_generate_direct_dispatch(
        &mut self,
        value: PointerValue<'ctx>,
        concrete_type: &CursedType,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        debug!("Auto-generating direct method dispatch for known concrete type");
        
        // Extract type name from concrete type
        let type_name = match concrete_type {
            CursedType::Struct(name, _) => name,
            _ => return Err(Error::from_str("Direct dispatch only supports struct types")),
        };
        
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Look up the vtable implementation for this type
        let vtable_impl = match interface_manager.get_vtable_impl(interface_name, type_name) {
            Some(impl_) => impl_,
            None => return Err(Error::from_str(&format!(
                "No vtable implementation found for {} implementing {}", 
                type_name, 
                interface_name
            ))),
        };
        
        // Here we would typically look up the method implementation directly
        // and call it, but for simplicity we'll first convert to an interface value
        // and then use the standard method dispatch
        
        // Create interface value from the concrete value
        let interface_value = self.create_interface_value(
            value,
            concrete_type,
            interface_name,
        )?;
        
        // Call the method on the interface value
        self.auto_generate_method_dispatch(
            interface_value,
            interface_name,
            method_name,
            args,
        )
    }
    
    #[instrument(skip(self), fields(struct_name = %struct_name, interface_name = %interface_name))]
    fn auto_register_struct_methods(
        &mut self,
        struct_name: &str,
        interface_name: &str,
    ) -> Result<(), Error> {
        debug!("Auto-registering struct methods for interface implementation");
        
        // Get the interface definition
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        let interface = match interface_manager.get_interface(interface_name) {
            Some(interface) => interface,
            None => return Err(Error::from_str(&format!(
                "Unknown interface: {}", 
                interface_name
            ))),
        };
        
        // Create a map to store the method implementations
        let mut method_implementations = HashMap::new();
        
        // For each method in the interface, try to find a corresponding method in the struct
        for (method_name, _, _) in &interface.methods {
            // In a real implementation, we would search the module for methods of the struct
            // that match the interface method signature. For now, we'll return an error.
            
            // Look for a function named either:
            // 1. struct_name_method_name (e.g., Person_greet)
            // 2. struct_name.method_name (e.g., Person.greet)
            
            let possible_names = vec![
                format!("{}.{}", struct_name, method_name),
                format!("{}.{}", struct_name.to_lowercase(), method_name),
                format!("{}-{}", struct_name, method_name),
                format!("{}{}", struct_name, method_name),
                format!("{}.{}", struct_name, method_name.to_uppercase()),
                format!("{}.{}", struct_name, method_name.to_lowercase()),
            ];
            
            let mut found_method = None;
            
            for func_name in possible_names {
                if let Some(func) = self.module().get_function(&func_name) {
                    debug!("Found method implementation: {}", func_name);
                    found_method = Some(func);
                    break;
                }
            }
            
            // If we couldn't find the method, try looking for a method with the struct name as a prefix
            if found_method.is_none() {
                if let Some(func) = self.module().get_function(&format!("{}{}", struct_name, method_name)) {
                    debug!("Found method implementation with struct name prefix: {}{}", struct_name, method_name);
                    found_method = Some(func);
                }
            }
            
            if let Some(func) = found_method {
                method_implementations.insert(method_name.clone(), func);
            } else {
                return Err(Error::from_str(&format!(
                    "Could not find method '{}' for struct '{}' to implement interface '{}'",
                    method_name,
                    struct_name,
                    interface_name
                )));
            }
        }
        
        // Register the implementation with the interface manager
        self.auto_generate_interface_implementation(
            struct_name,
            interface_name,
            method_implementations,
        )?;
        
        info!("Successfully auto-registered {} as implementing {}", struct_name, interface_name);
        Ok(())
    }
}

/// Extension trait for LlvmCodeGenerator to add auto interface dispatch functionality
pub trait AutoInterfaceDispatchExtension<'ctx> {
    /// Initialize the auto interface dispatcher
    fn init_auto_interface_dispatcher(&mut self) -> Result<(), Error>;
    
    /// Generate optimized code for interface method calls
    fn optimize_interface_call(
        &mut self,
        receiver: PointerValue<'ctx>,
        receiver_type: &CursedType,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
}

impl<'ctx> AutoInterfaceDispatchExtension<'ctx> for LlvmCodeGenerator<'ctx> {
    fn init_auto_interface_dispatcher(&mut self) -> Result<(), Error> {
        // Ensure interface manager is initialized
        if self.interface_manager.is_none() {
            self.interface_manager = Some(InterfaceManager::new());
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, receiver, args), fields(receiver_type = ?receiver_type, method_name = %method_name))]
    fn optimize_interface_call(
        &mut self,
        receiver: PointerValue<'ctx>,
        receiver_type: &CursedType,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // This is a higher-level function that chooses the best dispatch strategy
        // based on the receiver type
        debug!("Optimizing interface call for method {}", method_name);
        
        // Ensure interface manager is initialized
        if self.interface_manager.is_none() {
            self.init_auto_interface_dispatcher()?;
        }
        
        match receiver_type {
            CursedType::Interface(interface_name, _) => {
                debug!("Using dynamic dispatch for interface value");
                
                // For interface values, use enhanced dynamic dispatch if available
                #[cfg(feature = "enhanced_dynamic_dispatch")]
                {
                    debug!("Using enhanced dynamic dispatch");
                    // Try to use the enhanced dynamic dispatch for better error handling
                    return self.call_interface_method_enhanced(
                        receiver,
                        interface_name,
                        method_name,
                        args,
                    );
                }
                
                // Default implementation when enhanced_dynamic_dispatch is not enabled
                debug!("Using standard dynamic dispatch");
                return self.call_interface_method(
                    receiver,
                    interface_name,
                    method_name,
                    args
                );
            },
            CursedType::Struct(struct_name, _) => {
                debug!("Handling struct type {}", struct_name);
                // For struct types, try to determine if it implements any interfaces
                // and which one has the required method
                
                // Get the interface manager
                let interface_manager = match &self.interface_manager {
                    Some(manager) => manager,
                    None => return Err(Error::from_str("Interface manager not initialized")),
                };
                
                // Find interfaces implemented by this struct by checking all registered vtable implementations
                let mut implemented_interfaces: Vec<String> = Vec::new();
                
                // Look through each registered interface's vtables to see if this struct implements any
                for interface_name in interface_manager.interfaces().keys() {
                    if interface_manager.get_vtable_impl(interface_name, struct_name).is_some() {
                        implemented_interfaces.push(interface_name.clone());
                    }
                }
                
                debug!("Found {} interfaces implemented by {}", implemented_interfaces.len(), struct_name);
                
                // Find an interface that has this method
                for interface_name in implemented_interfaces {
                    if let Some(vtable) = interface_manager.get_vtable(&interface_name) {
                        if vtable.method_indices.contains_key(method_name) {
                            debug!("Found method '{}' in interface '{}'", method_name, interface_name);
                            // Found an interface with this method - use direct dispatch
                            return self.auto_generate_direct_dispatch(
                                receiver,
                                receiver_type,
                                &interface_name,
                                method_name,
                                args,
                            );
                        }
                    }
                }
                
                // If no interface implements this method, it might be a direct method call
                error!("Method '{}' not found on type '{:?}' or any of its interfaces", method_name, receiver_type);
                Err(Error::from_str(&format!(
                    "Method '{}' not found on type '{}' or any of its interfaces",
                    method_name,
                    format!("{:?}", receiver_type)
                )))
            },
            _ => {
                // Other types don't support interface method calls directly
                error!("Type '{:?}' doesn't support interface method calls", receiver_type);
                Err(Error::from_str(&format!(
                    "Type '{}' doesn't support interface method calls",
                    format!("{:?}", receiver_type)
                )))
            }
        }
    }
}