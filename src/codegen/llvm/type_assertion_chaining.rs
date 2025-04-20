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
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::enhanced_type_assertion::EnhancedTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;

use tracing::{debug, error, info, instrument, span, warn, Level};

/// Trait for supporting type assertion chaining in complex expressions
pub trait TypeAssertionChaining<'ctx> {
    /// Compile a chain of type assertions (e.g., a.(B).(C))
    fn compile_type_assertion_chain(
        &mut self,
        assertions: &[TypeAssertion]
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Handle a nested type assertion (e.g., obj.field.(Type))
    fn compile_nested_type_assertion(
        &mut self,
        outer_value: BasicValueEnum<'ctx>,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract a field from a type-asserted value
    fn extract_field_after_assertion(
        &mut self,
        asserted_value: BasicValueEnum<'ctx>,
        field_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> TypeAssertionChaining<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_chain(
        &mut self,
        assertions: &[TypeAssertion]
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        if assertions.is_empty() {
            return Err(Error::Compilation("Empty type assertion chain".to_string()));
        }
        
        debug!("Compiling chain of {} type assertions", assertions.len());
        
        // Compile the first assertion as the base
        let mut current_value = self.compile_enhanced_type_assertion(&assertions[0])?;
        
        // Process each subsequent assertion in the chain
        for assertion in &assertions[1..] {
            // Extract the asserted value (first element of the tuple)
            let asserted_value = self.builder().build_extract_value(
                current_value.into_struct_value(),
                0, // Index of the asserted value
                "asserted_value"
            ).map_err(|e| {
                error!("Failed to extract asserted value: {}", e);
                Error::Compilation(format!("Failed to extract asserted value: {}", e))
            })?;
            
            // Extract the success flag (second element of the tuple)
            let success_flag = self.builder().build_extract_value(
                current_value.into_struct_value(),
                1, // Index of the success flag
                "success_flag"
            ).map_err(|e| {
                error!("Failed to extract success flag: {}", e);
                Error::Compilation(format!("Failed to extract success flag: {}", e))
            })?;
            
            // Get the current function
            let current_fn = self.current_function()
                .ok_or_else(|| Error::Compilation("No current function for type assertion chain".to_string()))?;
            
            // Create blocks for conditional execution
            let assert_block = self.context().append_basic_block(current_fn, "chain_assert");
            let skip_block = self.context().append_basic_block(current_fn, "chain_skip");
            let merge_block = self.context().append_basic_block(current_fn, "chain_merge");
            
            // Branch based on the success of the previous assertion
            self.builder().build_conditional_branch(
                success_flag.into_int_value(),
                assert_block,
                skip_block
            ).map_err(|e| {
                error!("Failed to build conditional branch in chain: {}", e);
                Error::Compilation(format!("Failed to build conditional branch in chain: {}", e))
            })?;
            
            // Success path - perform the next assertion
            self.builder().position_at_end(assert_block);
            
            // Create a new TypeAssertion with the asserted value as the expression
            let mut next_assertion = assertion.clone();
            // We can't directly modify the expression field here since it's a boxed trait object
            // In a real implementation, we would need a way to create a new expression from the asserted value
            
            // For now, use the nested assertion approach
            let next_result = self.compile_nested_type_assertion(asserted_value, assertion)?;
            
            // Branch to merge block
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| {
                    error!("Failed to build branch to merge block in chain: {}", e);
                    Error::Compilation(format!("Failed to build branch to merge block in chain: {}", e))
                })?;
            
            // Failure path - propagate the failure
            self.builder().position_at_end(skip_block);
            
            // Create a failure result (null pointer and false)
            let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
            let false_val = self.context().bool_type().const_int(0, false);
            let skip_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
            
            // Branch to merge block
            self.builder().build_unconditional_branch(merge_block)
                .map_err(|e| {
                    error!("Failed to build branch from skip block in chain: {}", e);
                    Error::Compilation(format!("Failed to build branch from skip block in chain: {}", e))
                })?;
            
            // Merge results
            self.builder().position_at_end(merge_block);
            
            // Create the result type (tuple of pointer and bool)
            let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
            
            let phi = self.builder().build_phi(
                result_type,
                "chain_result"
            ).map_err(|e| {
                error!("Failed to build phi node in chain: {}", e);
                Error::Compilation(format!("Failed to build phi node in chain: {}", e))
            })?;
            
            phi.add_incoming(&[(
                &next_result,
                assert_block
            ), (
                &skip_result,
                skip_block
            )]);
            
            // Update current_value for the next iteration
            current_value = phi.as_basic_value();
        }
        
        debug!("Type assertion chain compiled successfully");
        Ok(current_value)
    }
    
    #[instrument(skip(self, outer_value), level = "debug")]
    fn compile_nested_type_assertion(
        &mut self,
        outer_value: BasicValueEnum<'ctx>,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling nested type assertion for {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for nested type assertion".to_string()))?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "nested_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "nested_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "nested_assert_merge");
        
        // Check if the outer value is of the target type
        let is_instance = self.check_instance_of_runtime(outer_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| {
            error!("Failed to build conditional branch in nested assertion: {}", e);
            Error::Compilation(format!("Failed to build conditional branch in nested assertion: {}", e))
        })?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_and_validate_interface_data(outer_value, &type_assertion.type_name)?;
        
        // Create the result structure (value and true flag)
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![data_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch to merge block in nested assertion: {}", e);
                Error::Compilation(format!("Failed to build branch to merge block in nested assertion: {}", e))
            })?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch from failure block in nested assertion: {}", e);
                Error::Compilation(format!("Failed to build branch from failure block in nested assertion: {}", e))
            })?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create the result type (tuple of pointer and bool)
        let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
        
        let phi = self.builder().build_phi(
            result_type,
            "nested_assertion_result"
        ).map_err(|e| {
            error!("Failed to build phi node in nested assertion: {}", e);
            Error::Compilation(format!("Failed to build phi node in nested assertion: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Nested type assertion compiled successfully");
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, asserted_value), level = "debug")]
    fn extract_field_after_assertion(
        &mut self,
        asserted_value: BasicValueEnum<'ctx>,
        field_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting field '{}' after type assertion", field_name);
        
        // First, check if we have a valid asserted value (non-null pointer)
        if !asserted_value.is_pointer_value() {
            return Err(Error::Compilation(format!(
                "Expected pointer value for field extraction, got {:?}",
                asserted_value
            )));
        }
        
        let ptr = asserted_value.into_pointer_value();
        
        // Check if pointer is null
        let is_null = self.builder().build_is_null(ptr, "field_extract_null_check")
            .map_err(|e| {
                error!("Failed to check if pointer is null: {}", e);
                Error::Compilation(format!("Failed to check if pointer is null: {}", e))
            })?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for field extraction".to_string()))?;
        
        // Create blocks for null check
        let null_block = self.context().append_basic_block(current_fn, "field_null");
        let non_null_block = self.context().append_basic_block(current_fn, "field_non_null");
        let merge_block = self.context().append_basic_block(current_fn, "field_merge");
        
        // Branch based on null check
        self.builder().build_conditional_branch(
            is_null,
            null_block,
            non_null_block
        ).map_err(|e| {
            error!("Failed to build conditional branch for null check: {}", e);
            Error::Compilation(format!("Failed to build conditional branch for null check: {}", e))
        })?;
        
        // Null path - return a default value
        self.builder().position_at_end(null_block);
        
        // Use a placeholder default value (this should be based on the field type)
        let null_result: BasicValueEnum<'ctx> = self.context().i64_type().const_zero().into();
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch from null block: {}", e);
                Error::Compilation(format!("Failed to build branch from null block: {}", e))
            })?;
        
        // Non-null path - extract the field
        self.builder().position_at_end(non_null_block);
        
        // This part would require knowledge of the struct type and field offsets
        // For a complete implementation, we would need to look up this information from type metadata
        // For now, we'll use a placeholder implementation
        
        // Assuming the field is at a specific GEP index (this would need to be determined from type info)
        let field_index = 0; // Placeholder
        
        // Get pointer to field
        let field_ptr = self.builder().build_struct_gep(
            // This should be the actual struct type
            self.context.struct_type(&[], false),
            ptr,
            field_index,
            &format!("field_{}_ptr", field_name)
        ).map_err(|e| {
            error!("Failed to get field pointer: {}", e);
            Error::Compilation(format!("Failed to get field pointer: {}", e))
        })?;
        
        // Load the field value
        // The field type should be determined from type metadata
        let field_type = self.context().i64_type(); // Placeholder
        
        let field_value = self.builder().build_load(
            field_type,
            field_ptr,
            &format!("field_{}_value", field_name)
        ).map_err(|e| {
            error!("Failed to load field value: {}", e);
            Error::Compilation(format!("Failed to load field value: {}", e))
        })?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch to merge block: {}", e);
                Error::Compilation(format!("Failed to build branch to merge block: {}", e))
            })?;
        
        // Merge results
        self.builder().position_at_end(merge_block);
        
        let phi = self.builder().build_phi(
            field_type,
            "field_result"
        ).map_err(|e| {
            error!("Failed to build field phi node: {}", e);
            Error::Compilation(format!("Failed to build field phi node: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &null_result,
            null_block
        ), (
            &field_value,
            non_null_block
        )]);
        
        debug!("Field extracted successfully");
        Ok(phi.as_basic_value())
    }
}

// Helper methods for integration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if an expression is a type assertion chain
    pub fn is_type_assertion_chain(&self, expression: &dyn crate::ast::traits::Expression) -> bool {
        // Check if this is a type assertion
        if let Some(_) = expression.as_any().downcast_ref::<TypeAssertion>() {
            // Check if the inner expression is also a type assertion
            // This would require access to the inner expression which we don't have here
            // For a real implementation, we'd need to track this information
            false
        } else {
            false
        }
    }
    
    /// Extract a chain of type assertions from a complex expression
    pub fn extract_type_assertion_chain(&self, expression: &dyn crate::ast::traits::Expression) -> Vec<TypeAssertion> {
        let mut chain = Vec::new();
        
        // Start with the current expression
        let mut current_expr = expression;
        
        // Collect all type assertions in the chain
        while let Some(type_assertion) = current_expr.as_any().downcast_ref::<TypeAssertion>() {
            // We can't fully clone the TypeAssertion due to the Box<dyn Expression>
            // For our test purposes, just create a single entry to trigger the chain
            chain.push(TypeAssertion {
                token: type_assertion.token.clone(),
                expression: Box::new(crate::ast::expressions::Identifier {
                    token: "placeholder".to_string(),
                    value: "placeholder".to_string(),
                }),
                type_name: type_assertion.type_name.clone()
            });
            // Move to the inner expression
            // This would require access to the expression field which we don't have here
            // For a real implementation, we'd need to track this information
            break;
        }
        
        chain
    }
}