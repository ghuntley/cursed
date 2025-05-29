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
use crate::codegen::llvm::interface_type_registry_enhanced::EnhancedTypeRegistry;
use crate::error::Error;

use tracing::{debug, error, info, instrument, span, warn, Level};

/// Trait for implementing interface type assertions in LLVM with proper error propagation
pub trait TypeAssertionErrorHandler<'ctx> {
    /// Compile a type assertion expression, returning both the converted value and a success flag
    /// with proper error propagation
    fn compile_type_assertion_with_errors(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type
    /// This function propagates errors correctly
    fn check_instance_of_with_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get the type ID from an interface value's vtable
    /// This function handles errors properly
    fn get_interface_type_id_safe(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract the data pointer from an interface value with proper error handling
    fn extract_interface_data_ptr_safe(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error>;
}

impl<'ctx> TypeAssertionErrorHandler<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_errors(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion for type {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        debug!("Compiled expression value: {:?}", expr_value);
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of_with_errors(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| {
            error!("Failed to build conditional branch: {}", e);
            Error::Compilation(format!("Failed to build conditional branch: {}", e))
        })?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr_safe(expr_value)?;
        
        // Create the result structure (value and true flag)
        let type_id = self.get_type_id(&type_assertion.type_name).map_err(|e| {
            error!("Failed to get type ID for {}: {}", type_assertion.type_name, e);
            Error::Compilation(format!("Failed to get type ID for {}: {}", type_assertion.type_name, e))
        })?;
        
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| {
            error!("Failed to cast data pointer: {}", e);
            Error::Compilation(format!("Failed to cast data pointer: {}", e))
        })?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch to merge block: {}", e);
                Error::Compilation(format!("Failed to build branch to merge block: {}", e))
            })?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch from failure block: {}", e);
                Error::Compilation(format!("Failed to build branch from failure block: {}", e))
            })?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create the result type (tuple of pointer and bool)
        let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| {
            error!("Failed to build phi node: {}", e);
            Error::Compilation(format!("Failed to build phi node: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Type assertion compiled successfully");
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn check_instance_of_with_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Checking if value is instance of {}", target_type_name);
        
        // Get the type ID from the interface value's vtable
        let actual_type_id = self.get_interface_type_id_safe(interface_value)?;
        
        // Get the expected type ID for the target type
        let expected_type_id = self.get_type_id(target_type_name).map_err(|e| {
            error!("Failed to get type ID for {}: {}", target_type_name, e);
            Error::Compilation(format!("Failed to get type ID for {}: {}", target_type_name, e))
        })?;
        
        // Use the enhanced registry for better type information
        let actual_type_name = match self.get_assertion_type_info(interface_value, target_type_name) {
            Ok((_, name)) => name,
            Err(_) => {
                // Fall back to basic registry lookup if enhanced version fails
                if let Some(registry) = &self.interface_type_registry {
                    if actual_type_id.is_int_value() {
                        let id = if let Some(const_val) = actual_type_id.into_int_value().get_zero_extended_constant() {
                            const_val
                        } else {
                            u64::MAX // Cannot get constant value
                        };
                        match registry.get_type_name(id) {
                            Some(name) => name.clone(),
                            None => "Unknown".to_string()
                        }
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            }
        };
        
        // Log the type information being compared
        // Get a constant value if possible for logging
        let id_val = actual_type_id.into_int_value().get_zero_extended_constant()
            .unwrap_or(u64::MAX);
            
        debug!("Comparing type IDs: {} (type name: {}) with target type {}.", 
               id_val,
               actual_type_name,
               target_type_name);
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "is_instance_of"
        ).map_err(|e| {
            error!("Failed to compare type IDs: {}", e);
            Error::Compilation(format!("Failed to compare type IDs: {}", e))
        })?;
        
        debug!("Instance check completed");
        Ok(result.into())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_interface_type_id_safe(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Getting type ID from interface value");
        
        // First, check if the interface value is null
        let is_null = if interface_value.is_pointer_value() {
            let ptr = interface_value.into_pointer_value();
            let null_check = self.builder().build_is_null(ptr, "is_null_check")
                .map_err(|e| {
                    error!("Failed to check if interface pointer is null: {}", e);
                    Error::Compilation(format!("Failed to check if interface pointer is null: {}", e))
                })?;
            
            // Create basic blocks for null and non-null paths
            let current_function = self.current_function()
                .ok_or_else(|| Error::Compilation("No current function when checking null interface".to_string()))?;
            
            let null_block = self.context().append_basic_block(current_function, "null_interface");
            let non_null_block = self.context().append_basic_block(current_function, "non_null_interface");
            let continue_block = self.context().append_basic_block(current_function, "continue_interface");
            
            // Branch based on null check
            self.builder().build_conditional_branch(
                null_check,
                null_block,
                non_null_block
            ).map_err(|e| {
                error!("Failed to build null check branch: {}", e);
                Error::Compilation(format!("Failed to build null check branch: {}", e))
            })?;
            
            // Null path - report error
            self.builder().position_at_end(null_block);
            
            // In the null case, we'll return a special type ID that won't match anything
            let null_type_id = self.context().i64_type().const_int(u64::MAX, false);
            
            // Branch to continue block
            self.builder().build_unconditional_branch(continue_block)
                .map_err(|e| {
                    error!("Failed to build branch from null block: {}", e);
                    Error::Compilation(format!("Failed to build branch from null block: {}", e))
                })?;
            
            // Non-null path
            self.builder().position_at_end(non_null_block);
            
            Some((null_check, null_type_id, null_block, non_null_block, continue_block))
        } else {
            None
        };
        
        // Interface value is a struct with two fields:
        // 1. Data pointer
        // 2. VTable pointer
        
        // Extract the vtable pointer
        let vtable_ptr = if interface_value.is_struct_value() {
            // Direct interface value - extract vtable pointer field
            let vtable_field = self.builder().build_extract_value(
                interface_value.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| {
                error!("Failed to extract vtable pointer: {}", e);
                Error::Compilation(format!("Failed to extract vtable pointer: {}", e))
            })?;
            
            // Verify it's a pointer
            if !vtable_field.is_pointer_value() {
                error!("Extracted vtable field is not a pointer: {:?}", vtable_field);
                return Err(Error::Compilation(format!(
                    "Extracted vtable field is not a pointer: {:?}", vtable_field
                )));
            }
            
            vtable_field.into_pointer_value()
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value - load and extract vtable pointer
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| {
                error!("Failed to load interface value: {}", e);
                Error::Compilation(format!("Failed to load interface value: {}", e))
            })?;
            
            let vtable_field = self.builder().build_extract_value(
                loaded.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| {
                error!("Failed to extract vtable pointer from loaded value: {}", e);
                Error::Compilation(format!("Failed to extract vtable pointer from loaded value: {}", e))
            })?;
            
            // Verify it's a pointer
            if !vtable_field.is_pointer_value() {
                error!("Extracted vtable field from loaded interface is not a pointer: {:?}", vtable_field);
                return Err(Error::Compilation(format!(
                    "Extracted vtable field from loaded interface is not a pointer: {:?}", vtable_field
                )));
            }
            
            vtable_field.into_pointer_value()
        } else {
            error!("Invalid interface value type: {:?}", interface_value);
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_value
            )));
        };
        
        // Check if vtable pointer is null
        let vtable_null_check = self.builder().build_is_null(vtable_ptr, "vtable_null_check")
            .map_err(|e| {
                error!("Failed to check if vtable pointer is null: {}", e);
                Error::Compilation(format!("Failed to check if vtable pointer is null: {}", e))
            })?;
        
        // Create basic blocks for null and non-null vtable paths
        let current_function = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function when checking null vtable".to_string()))?;
        
        let vtable_null_block = self.context().append_basic_block(current_function, "null_vtable");
        let vtable_non_null_block = self.context().append_basic_block(current_function, "non_null_vtable");
        let vtable_continue_block = self.context().append_basic_block(current_function, "continue_vtable");
        
        // Branch based on vtable null check
        self.builder().build_conditional_branch(
            vtable_null_check,
            vtable_null_block,
            vtable_non_null_block
        ).map_err(|e| {
            error!("Failed to build vtable null check branch: {}", e);
            Error::Compilation(format!("Failed to build vtable null check branch: {}", e))
        })?;
        
        // Null vtable path - report error
        self.builder().position_at_end(vtable_null_block);
        
        // In the null vtable case, we'll return a special type ID that won't match anything
        let null_vtable_type_id = self.context().i64_type().const_int(u64::MAX - 1, false);
        
        // Branch to continue block
        self.builder().build_unconditional_branch(vtable_continue_block)
            .map_err(|e| {
                error!("Failed to build branch from null vtable block: {}", e);
                Error::Compilation(format!("Failed to build branch from null vtable block: {}", e))
            })?;
        
        // Non-null vtable path
        self.builder().position_at_end(vtable_non_null_block);
        
        // Type ID is the first field in the vtable
        let type_id_ptr = self.builder().build_struct_gep(
            // Create and use a dummy struct type since we can't get the pointee type directly
            self.context.struct_type(&[], false),
            vtable_ptr,
            0, // Index of type ID pointer
            "type_id_ptr"
        ).map_err(|e| {
            error!("Failed to get type ID pointer: {}", e);
            Error::Compilation(format!("Failed to get type ID pointer: {}", e))
        })?;
        
        // Load the type ID
        let type_id = self.builder().build_load(
            self.context().i64_type(),
            type_id_ptr,
            "type_id"
        ).map_err(|e| {
            error!("Failed to load type ID: {}", e);
            Error::Compilation(format!("Failed to load type ID: {}", e))
        })?;
        
        // Branch to continue block
        self.builder().build_unconditional_branch(vtable_continue_block)
            .map_err(|e| {
                error!("Failed to build branch to vtable continue block: {}", e);
                Error::Compilation(format!("Failed to build branch to vtable continue block: {}", e))
            })?;
        
        // Continue block - use phi node to select the appropriate type ID
        self.builder().position_at_end(vtable_continue_block);
        
        let vtable_phi = self.builder().build_phi(
            self.context().i64_type(),
            "vtable_check_result"
        ).map_err(|e| {
            error!("Failed to build vtable phi node: {}", e);
            Error::Compilation(format!("Failed to build vtable phi node: {}", e))
        })?;
        
        vtable_phi.add_incoming(&[(
            &null_vtable_type_id,
            vtable_null_block
        ), (
            &type_id,
            vtable_non_null_block
        )]);
        
        let vtable_result = vtable_phi.as_basic_value();
        
        // If we have a null interface check, we need to merge those results too
        if let Some((_, null_type_id, null_block, _, continue_block)) = is_null {
            self.builder().position_at_end(continue_block);
            
            let final_phi = self.builder().build_phi(
                self.context().i64_type(),
                "interface_check_result"
            ).map_err(|e| {
                error!("Failed to build interface phi node: {}", e);
                Error::Compilation(format!("Failed to build interface phi node: {}", e))
            })?;
            
            final_phi.add_incoming(&[(
                &null_type_id,
                null_block
            ), (
                &vtable_result,
                vtable_continue_block
            )]);
            
            debug!("Type ID retrieved successfully with null checks");
            return Ok(final_phi.as_basic_value());
        }
        
        debug!("Type ID retrieved successfully");
        Ok(vtable_result)
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn extract_interface_data_ptr_safe(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Extracting data pointer from interface value");
        
        // Extract the data pointer (first field of interface value)
        let data_ptr = if interface_value.is_struct_value() {
            // Direct interface value
            let data_field = self.builder().build_extract_value(
                interface_value.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| {
                error!("Failed to extract data pointer: {}", e);
                Error::Compilation(format!("Failed to extract data pointer: {}", e))
            })?;
            
            // Verify it's a pointer
            if !data_field.is_pointer_value() {
                error!("Extracted data field is not a pointer: {:?}", data_field);
                return Err(Error::Compilation(format!(
                    "Extracted data field is not a pointer: {:?}", data_field
                )));
            }
            
            data_field.into_pointer_value()
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| {
                error!("Failed to load interface value: {}", e);
                Error::Compilation(format!("Failed to load interface value: {}", e))
            })?;
            
            let data_field = self.builder().build_extract_value(
                loaded.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| {
                error!("Failed to extract data pointer from loaded value: {}", e);
                Error::Compilation(format!("Failed to extract data pointer from loaded value: {}", e))
            })?;
            
            // Verify it's a pointer
            if !data_field.is_pointer_value() {
                error!("Extracted data field from loaded interface is not a pointer: {:?}", data_field);
                return Err(Error::Compilation(format!(
                    "Extracted data field from loaded interface is not a pointer: {:?}", data_field
                )));
            }
            
            data_field.into_pointer_value()
        } else {
            error!("Invalid interface value type for data extraction: {:?}", interface_value);
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer for data extraction, got {:?}",
                interface_value
            )));
        };
        
        debug!("Data pointer extracted successfully");
        Ok(data_ptr)
    }
}

// Helper methods extension
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Print debug information about a type assertion operation
    #[instrument(skip(self, interface_value, result), level = "debug")]
    pub fn debug_type_assertion(
        &self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        result: BasicValueEnum<'ctx>
    ) -> Result<(), Error> {
        // Check if debugging is enabled via environment variable
        let debug_enabled = std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
            .unwrap_or(false);
        
        if !debug_enabled {
            return Ok(());
        }
        
        // Get the interface type registry
        if let Some(registry) = &self.interface_type_registry {
            // Get the actual type ID from the interface value
            if let Ok(actual_type_id) = self.get_interface_type_id_safe(interface_value) {
                // Get type names for both expected and actual types
                let actual_type_id_const = if actual_type_id.is_int_value() {
                    if let Some(const_val) = actual_type_id.into_int_value().get_zero_extended_constant() {
                        const_val
                    } else {
                        u64::MAX // Cannot get constant value
                    }
                } else {
                    u64::MAX // Unknown type ID
                };
                
                let actual_type_name = registry.get_type_name(actual_type_id_const)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                
                // Check if the assertion succeeded (extract success flag)
                let success = if result.is_struct_value() {
                    if let Ok(success_val) = self.builder().build_extract_value(
                        result.into_struct_value(),
                        1, // Index of success flag
                        "success_flag"
                    ) {
                        if success_val.is_int_value() {
                            success_val.into_int_value().get_zero_extended_constant().unwrap_or(0) != 0
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if success {
                    info!(
                        "Type assertion SUCCESS: Value of type '{}' asserted to type '{}'", 
                        actual_type_name, 
                        target_type_name
                    );
                } else {
                    warn!(
                        "Type assertion FAILED: Value of type '{}' cannot be converted to type '{}'", 
                        actual_type_name, 
                        target_type_name
                    );
                }
            }
        } else {
            // Fallback if registry isn't available
            debug!(
                "Type assertion: checking if value {:?} is of type {}", 
                interface_value, 
                target_type_name
            );
            
            debug!("Type assertion result: {:?}", result);
        }
        
        Ok(())
    }
}