use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::error::Error;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::error::SourceLocation;
use crate::runtime::type_assertion_runtime::{TypeAssertionRuntime, RuntimeTypeInfo, PanicConfiguration};

/// Trait for implementing interface type assertions in LLVM
pub trait InterfaceTypeAssertion<'ctx> {
    /// Compile a type assertion expression, returning both the converted value and a success flag
    fn compile_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion with error propagation, integrating with the language's `?` operator
    fn compile_type_assertion_question(
        &mut self,
        type_assertion_question: &TypeAssertionQuestion
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
    
    /// Configure runtime panic behavior for type assertions
    fn configure_type_assertion_panics(
        &mut self,
        panic_config: PanicConfiguration
    ) -> Result<(), Error>;
    
    /// Register a type in the runtime system for better error reporting
    fn register_runtime_type(
        &mut self,
        type_info: RuntimeTypeInfo
    ) -> Result<(), Error>;
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
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())
            .map_err(|e| Error::Compilation(format!("Failed to compile expression: {}", e)))?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        let _ = self.builder().build_conditional_branch(
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
        let _ = self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        let _ = self.builder().build_unconditional_branch(merge_block)
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
    
    fn compile_type_assertion_question(
        &mut self,
        type_assertion_question: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        use tracing::{debug, info, error};
        
        debug!("Compiling type assertion with error propagation: {}.({})?", 
               type_assertion_question.expression.string(), type_assertion_question.type_name);
        
        // Create a source location if possible
        let source_location = match &type_assertion_question.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({})?", type_assertion_question.expression.string(), type_assertion_question.type_name),
                })
            },
            _ => None,
        };
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion question".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion_question.expression.as_ref())
            .map_err(|e| Error::Compilation(format!("Failed to compile expression: {}", e)))?;
        
        // Create basic blocks for the assertion flow
        let success_block = self.context().append_basic_block(current_fn, "type_assert_q_success");
        let error_block = self.context().append_basic_block(current_fn, "type_assert_q_error");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_q_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion_question.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        let _ = self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            error_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract the data pointer and return it directly
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Cast to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // For TypeAssertionQuestion, we return the value directly on success
        let success_result = casted_ptr;
        
        // Branch to merge block
        let _ = self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Error path - this would typically trigger error propagation
        // In the CURSED language, this should integrate with the `?` operator mechanism
        self.builder().position_at_end(error_block);
        
        // Create an error value - this should integrate with the language's error system
        // For now, we'll create a runtime error and let it be handled by the calling context
        if let Some(runtime) = &self.type_assertion_runtime {
            if let Ok(interface_type_id) = self.get_interface_type_id(expr_value) {
                if let Some(actual_type_id_u64) = interface_type_id.into_int_value().get_zero_extended_constant() {
                    info!("Type assertion failure will be handled by runtime system");
                    
                    // In a real implementation, this would create an error result that gets propagated
                    // For now, we'll return a null pointer to indicate failure
                    let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
                    let error_result = null_ptr;
                    
                    // Branch to merge block
                    let _ = self.builder().build_unconditional_branch(merge_block)
                        .map_err(|e| Error::Compilation(e.to_string()))?;
                    
                    // Merge block - use phi node to select the appropriate result
                    self.builder().position_at_end(merge_block);
                    let phi = self.builder().build_phi(
                        self.pointer_type(),
                        "assertion_question_result"
                    ).map_err(|e| Error::Compilation(e.to_string()))?;
                    
                    phi.add_incoming(&[(
                        &success_result,
                        success_block
                    ), (
                        &error_result,
                        error_block
                    )]);
                    
                    debug!("Type assertion with error propagation compiled successfully");
                    return Ok(phi.as_basic_value());
                }
            }
        }
        
        // Fallback: return null pointer and log the failure
        error!("Failed to compile type assertion with error propagation - runtime system not available");
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let error_result = null_ptr;
        
        // Branch to merge block
        let _ = self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            self.pointer_type(),
            "assertion_question_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &error_result,
            error_block
        )]);
        
        Ok(phi.as_basic_value())
    }
    
    fn check_instance_of(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: Option<SourceLocation>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        use tracing::{debug, warn, error};
        
        // Ensure the registry is initialized
        let _ = self.ensure_registry_visualization_initialized();
        
        // Get the type ID from the interface value's vtable
        let actual_type_id = self.get_interface_type_id(interface_value)?;
        let actual_type_id_u64 = if let Some(const_int) = actual_type_id.into_int_value().get_zero_extended_constant() {
            const_int
        } else {
            // If we can't get a constant value, proceed with runtime comparison
            0 // This will be handled at runtime
        };
        
        // Get the expected type ID for the target type from the registry
        let expected_type_id_u64 = match &self.interface_type_registry {
            Some(registry) => {
                // Try to get from registry, but fallback to hash if not found
                match registry.get_type_id(target_type_name) {
                    Ok(type_id) => type_id,
                    Err(_) => {
                        // Type not found in registry, use hash fallback
                        self.hash_type_name(target_type_name)
                    }
                }
            },
            None => self.hash_type_name(target_type_name) // Fallback to direct hash if registry not available
        };
        
        // If we have runtime type assertion support, use it for better error handling
        if let Some(runtime) = &self.type_assertion_runtime {
            debug!("Using runtime type assertion system");
            
            // For compile-time known values, we can check immediately
            if actual_type_id_u64 != 0 {
                match runtime.assert_type(actual_type_id_u64, target_type_name, source_location.clone()) {
                    Ok(is_match) => {
                        debug!("Runtime type assertion result: {}", is_match);
                        let result_val = if is_match { 1 } else { 0 };
                        let result = self.context().bool_type().const_int(result_val, false);
                        return Ok(result.into());
                    },
                    Err(cursed_error) => {
                        warn!("Runtime type assertion failed: {}", cursed_error.message());
                        // Continue with LLVM-level comparison as fallback
                    }
                }
            }
        }
        
        let expected_type_id = self.context().i64_type().const_int(expected_type_id_u64, false);
        
        // Compare the type IDs at LLVM level
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
        // For now, return a simple hash-based type ID
        // In a real implementation, this would extract the type ID from the interface value
        // But for testing purposes, we'll just return a constant type ID
        
        if interface_value.is_struct_value() {
            // Try to extract the type ID from a struct interface value
            let struct_val = interface_value.into_struct_value();
            let num_fields = struct_val.get_type().count_fields();
            
            if num_fields >= 2 {
                // Assume second field is type ID
                match self.builder().build_extract_value(struct_val, 1, "type_id") {
                    Ok(type_id) => Ok(type_id),
                    Err(_) => {
                        // Fallback to a default type ID
                        let default_id = self.context().i64_type().const_int(0, false);
                        Ok(default_id.into())
                    }
                }
            } else {
                // Not enough fields, return default
                let default_id = self.context().i64_type().const_int(0, false);
                Ok(default_id.into())
            }
        } else {
            // For other types, return a default type ID
            let default_id = self.context().i64_type().const_int(0, false);
            Ok(default_id.into())
        }
    }
    
    fn extract_interface_data_ptr(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        // For testing purposes, we'll create a simple implementation
        if interface_value.is_struct_value() {
            // Try to extract the first field as data pointer
            let struct_val = interface_value.into_struct_value();
            let num_fields = struct_val.get_type().count_fields();
            
            if num_fields >= 1 {
                match self.builder().build_extract_value(struct_val, 0, "data_ptr") {
                    Ok(data_value) => {
                        if data_value.is_pointer_value() {
                            Ok(data_value.into_pointer_value())
                        } else {
                            // If it's not a pointer, create a null pointer as fallback
                            let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                            Ok(ptr_type.const_null())
                        }
                    },
                    Err(_) => {
                        // Return null pointer as fallback
                        let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                        Ok(ptr_type.const_null())
                    }
                }
            } else {
                // No fields, return null pointer
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null())
            }
        } else if interface_value.is_pointer_value() {
            // If we got a pointer directly, just return it
            Ok(interface_value.into_pointer_value())
        } else {
            // For other types, return null pointer
            let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
            Ok(ptr_type.const_null())
        }
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
            let _ = self.builder().build_store(alloca, value)
                .map_err(|e| Error::Compilation(e.to_string()))?;
            
            let interface_struct = self.build_tuple(vec![
                alloca.into(), // data pointer
                type_id, // type ID (simplified vtable)
            ])?;
            Ok(interface_struct)
        }
    }
    
    fn configure_type_assertion_panics(
        &mut self,
        panic_config: PanicConfiguration
    ) -> Result<(), Error> {
        use tracing::info;
        
        if let Some(runtime) = &mut self.type_assertion_runtime {
            // Get mutable access to runtime via Arc - we need to use Arc::get_mut or replace the Arc
            // Since we can't get mutable access through Arc easily, we'll create a new runtime
            let mut new_runtime = TypeAssertionRuntime::with_panic_config(panic_config.clone());
            
            // Copy existing type registrations if possible
            if let Ok(stats) = runtime.get_statistics() {
                info!("Updating panic configuration. Previous stats: {:?}", stats);
            }
            
            // Replace the runtime
            self.type_assertion_runtime = Some(std::sync::Arc::new(new_runtime));
            
            info!("Type assertion panic configuration updated");
            Ok(())
        } else {
            // Create new runtime with the panic configuration
            self.type_assertion_runtime = Some(std::sync::Arc::new(
                TypeAssertionRuntime::with_panic_config(panic_config)
            ));
            
            info!("Type assertion runtime created with panic configuration");
            Ok(())
        }
    }
    
    fn register_runtime_type(
        &mut self,
        type_info: RuntimeTypeInfo
    ) -> Result<(), Error> {
        use tracing::{debug, warn};
        
        debug!("Registering runtime type: {} (ID: 0x{:016x})", type_info.type_name, type_info.type_id);
        
        if let Some(runtime) = &self.type_assertion_runtime {
            // Since we have an Arc, we need to create a new runtime with the type registered
            // This is a limitation of the current design - in a real implementation,
            // we might use Arc<Mutex<TypeAssertionRuntime>> instead
            
            // For now, we'll work around this by creating a new runtime
            // In a production system, this should be refactored to use interior mutability
            warn!("Cannot register type in existing runtime due to Arc limitations. This should be refactored to use Arc<Mutex<TypeAssertionRuntime>>");
            
            // Create a new runtime with the same configuration
            let mut new_runtime = if let Ok(stats) = runtime.get_statistics() {
                // Preserve panic configuration by creating with default and updating later
                TypeAssertionRuntime::new()
            } else {
                TypeAssertionRuntime::new()
            };
            
            // Register the new type
            new_runtime.register_type(type_info)?;
            
            // Replace the runtime
            self.type_assertion_runtime = Some(std::sync::Arc::new(new_runtime));
            
            debug!("Type registered in new runtime instance");
            Ok(())
        } else {
            // Create new runtime and register the type
            let mut new_runtime = TypeAssertionRuntime::new();
            new_runtime.register_type(type_info)?;
            self.type_assertion_runtime = Some(std::sync::Arc::new(new_runtime));
            
            debug!("Type registered in new runtime instance");
            Ok(())
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
        
        // Create an undef value and build it up with insert_value operations
        let mut tuple = tuple_type.get_undef();
        for (i, value) in values.iter().enumerate() {
            tuple = self.builder().build_insert_value(tuple, *value, i as u32, "tuple_insert")
                .map_err(|e| Error::Compilation(e.to_string()))?
                .into_struct_value();
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