//! # Interface Type Assertion Error Propagation
//!
//! This module enhances error propagation in interface type assertions by properly
//! using the `?` operator for errors and improving error context information. It provides
//! a complete path from type assertion AST nodes to LLVM code generation with proper
//! error handling throughout.
//!
//! ## Features
//!
//! 1. Consistent use of the `?` operator for error propagation
//! 2. Rich error context including source location information
//! 3. Integration with structured logging for better debugging
//! 4. Support for nested type assertions with proper error propagation
//! 5. Clean integration with the expression compiler and existing type assertion code

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use tracing::{debug, info, trace, warn, instrument};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;

/// A trait that enhances error propagation for interface type assertions
pub trait TypeAssertionErrorPropagation<'ctx> {
    /// Compile a type assertion with proper error propagation using the `?` operator
    fn compile_type_assertion_with_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type, with proper error handling
    fn check_instance_of_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract data from an interface value with proper error handling
    fn extract_interface_data_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate a better error context for type assertion errors
    fn create_type_assertion_error_context(
        &self,
        interface_type: &str,
        target_type: &str,
        source_location: &str
    ) -> String;
}

impl<'ctx> TypeAssertionErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with propagation for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation(
                format!("No current function for type assertion at {}", source_location)
            ))?;
        
        // Compile the expression being asserted with proper error propagation
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())
            .map_err(|e| Error::Compilation(
                format!("Failed to compile expression for type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        debug!("Compiled expression value of type: {:?}", expr_value.get_type());
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type with proper error propagation
        let is_instance = self.check_instance_of_with_propagation(
            expr_value, 
            &type_assertion.type_name,
            &source_location
        )?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(
            format!("Failed to build conditional branch for type assertion at {}: {}", 
                    source_location, e)
        ))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)
            .map_err(|e| Error::Compilation(
                format!("Failed to extract data pointer in type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Look up the target type and create appropriate pointer type
        let target_struct_type = self
            .get_type_by_name(&type_assertion.type_name)
            .unwrap_or_else(|| self.context().opaque_struct_type(&type_assertion.type_name));
        
        let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
        
        // Cast the data pointer to the concrete type pointer with proper error handling
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            target_ptr_type,
            "casted_ptr"
        ).map_err(|e| Error::Compilation(
            format!("Failed to cast pointer in type assertion at {}: {}", 
                    source_location, e)
        ))?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])
            .map_err(|e| Error::Compilation(
                format!("Failed to build tuple in type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(
                format!("Failed to build branch to merge block in type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Add detailed logging for failed assertions in debug builds
        if cfg!(debug_assertions) {
            debug!("Type assertion failed: {} is not of type {}", 
                   type_assertion.expression.string(), type_assertion.type_name);
        }
        
        let null_ptr = target_ptr_type.const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])
            .map_err(|e| Error::Compilation(
                format!("Failed to build failure tuple in type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(
                format!("Failed to build branch to merge block from failure path in type assertion at {}: {}", 
                        source_location, e)
            ))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create appropriate tuple type for the phi node
        let result_type = self.tuple_type(vec![
            target_ptr_type.into(), 
            self.context().bool_type().into()
        ]);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(
            format!("Failed to build phi node in type assertion at {}: {}", 
                    source_location, e)
        ))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Completed type assertion compilation with result type: {:?}", result_type);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn check_instance_of_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        trace!("Checking if value is instance of type {} at {}", target_type_name, source_location);
        
        // Get the type ID from the interface value's vtable with proper error propagation
        let actual_type_id = self.get_interface_type_id(interface_value)
            .map_err(|e| Error::Compilation(
                format!("Failed to get interface type ID at {}: {}", source_location, e)
            ))?;
        
        // Get the expected type ID for the target type with proper error propagation
        let expected_type_id = self.get_type_id(target_type_name)
            .map_err(|e| Error::Compilation(
                format!("Failed to get type ID for {} at {}: {}", 
                        target_type_name, source_location, e)
            ))?;
        
        // Compare the type IDs with proper error handling
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "is_instance_of"
        ).map_err(|e| Error::Compilation(
            format!("Failed to compare type IDs at {}: {}", source_location, e)
        ))?;
        
        Ok(result.into())
    }
    
    fn extract_interface_data_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        trace!("Extracting data from interface value at {}", source_location);
        
        // Extract the data pointer with proper error handling
        let data_ptr = if interface_value.is_struct_value() {
            // Direct interface value
            self.builder().build_extract_value(
                interface_value.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(
                format!("Failed to extract data pointer from struct at {}: {}", 
                        source_location, e)
            ))?
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| Error::Compilation(
                format!("Failed to load interface value at {}: {}", source_location, e)
            ))?;
            
            self.builder().build_extract_value(
                loaded.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(
                format!("Failed to extract data pointer from loaded value at {}: {}", 
                        source_location, e)
            ))?
        } else {
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer at {}, got {:?}",
                source_location, interface_value
            )));
        };
        
        Ok(data_ptr)
    }
    
    fn create_type_assertion_error_context(
        &self,
        interface_type: &str,
        target_type: &str,
        source_location: &str
    ) -> String {
        format!(
            "Type assertion error: Cannot convert from '{}' to '{}' at {}",
            interface_type, target_type, source_location
        )
    }
}

// Register this module in the compiler's initialization
pub fn register_type_assertion_error_propagation() {
    trace!("Type assertion error propagation module registered");
}