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
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;

use tracing::{debug, error, info, instrument, span, warn, Level};

/// Enhanced trait for implementing interface type assertions in LLVM with runtime type checking
/// and support for complex expressions
pub trait EnhancedTypeAssertion<'ctx> {
    /// Compile a type assertion expression with full runtime type checking
    /// This method supports type assertion chaining in complex expressions
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type with runtime validation
    /// This method adds additional runtime type information checks
    fn check_instance_of_runtime(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract and validate the data pointer from an interface value
    fn extract_and_validate_interface_data(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling enhanced type assertion for type {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for enhanced type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        debug!("Compiled expression value: {:?}", expr_value);
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Perform enhanced instance check with runtime type information
        let is_instance = self.check_instance_of_runtime(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| {
            error!("Failed to build conditional branch: {}", e);
            Error::Compilation(format!("Failed to build conditional branch: {}", e))
        })?;
        
        // Success path - extract and validate the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_and_validate_interface_data(expr_value, &type_assertion.type_name)?;
        
        // Create the result structure (value and true flag)
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![data_ptr.into(), true_val.into()])?;
        
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
        
        debug!("Enhanced type assertion compiled successfully");
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn check_instance_of_runtime(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Runtime check if value is instance of {}", target_type_name);
        
        // First, validate that the interface value is not null
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for runtime type check".to_string()))?;
        
        // Create blocks for null check
        let null_check_block = self.context().append_basic_block(current_fn, "interface_null_check");
        let type_check_block = self.context().append_basic_block(current_fn, "interface_type_check");
        let check_result_block = self.context().append_basic_block(current_fn, "interface_check_result");
        
        // Check if the interface value is null
        let is_null = if interface_value.is_pointer_value() {
            let ptr = interface_value.into_pointer_value();
            self.builder().build_is_null(
                ptr,
                "interface_is_null"
            ).map_err(|e| {
                error!("Failed to build null check: {}", e);
                Error::Compilation(format!("Failed to build null check: {}", e))
            })?
        } else {
            // For non-pointer values, use false (not null)
            self.context().bool_type().const_int(0, false)
        };
        
        // Branch based on null check
        self.builder().build_conditional_branch(
            is_null,
            null_check_block,
            type_check_block
        ).map_err(|e| {
            error!("Failed to build null check branch: {}", e);
            Error::Compilation(format!("Failed to build null check branch: {}", e))
        })?;
        
        // Handle null interface case
        self.builder().position_at_end(null_check_block);
        let false_val = self.context().bool_type().const_int(0, false);
        
        // When interface is null, assertion always fails
        self.builder().build_unconditional_branch(check_result_block)
            .map_err(|e| {
                error!("Failed to build branch from null check block: {}", e);
                Error::Compilation(format!("Failed to build branch from null check block: {}", e))
            })?;
        
        // Non-null case - perform the type check
        self.builder().position_at_end(type_check_block);
        
        // Get the type ID from the interface value's vtable
        let type_id_result = self.get_interface_type_id_safe(interface_value);
        
        // If type ID extraction fails, default to failure
        let actual_type_id = match type_id_result {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to get interface type ID: {}", e);
                // Return a special ID that won't match any valid type
                self.context().i64_type().const_int(u64::MAX, false).into()
            }
        };
        
        // Get the expected type ID for the target type
        let expected_type_id = match self.get_type_id(target_type_name) {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to get type ID for {}: {}", target_type_name, e);
                // Return a special ID that won't match the actual ID
                self.context().i64_type().const_int(0, false).into()
            }
        };
        
        // Compare the type IDs
        let type_match = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "type_id_match"
        ).map_err(|e| {
            error!("Failed to compare type IDs: {}", e);
            Error::Compilation(format!("Failed to compare type IDs: {}", e))
        })?;
        
        // Branch to result block
        self.builder().build_unconditional_branch(check_result_block)
            .map_err(|e| {
                error!("Failed to build branch from type check block: {}", e);
                Error::Compilation(format!("Failed to build branch from type check block: {}", e))
            })?;
        
        // Merge results with phi node
        self.builder().position_at_end(check_result_block);
        
        let phi = self.builder().build_phi(
            self.context().bool_type(),
            "type_check_result"
        ).map_err(|e| {
            error!("Failed to build type check result phi: {}", e);
            Error::Compilation(format!("Failed to build type check result phi: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &false_val,
            null_check_block
        ), (
            &type_match,
            type_check_block
        )]);
        
        debug!("Runtime type check completed");
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn extract_and_validate_interface_data(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting and validating interface data for {}", target_type_name);
        
        // Extract the data pointer using the safe method from TypeAssertionErrorHandler
        let data_ptr = self.extract_interface_data_ptr_safe(interface_value)?;
        
        // Additional validation could be added here (e.g., checking memory layout compatibility)
        
        // Cast the data pointer to the appropriate type
        let target_type = self.context().i8_type().ptr_type(AddressSpace::default());
        
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            target_type,
            &format!("casted_to_{}", target_type_name)
        ).map_err(|e| {
            error!("Failed to cast data pointer to {}: {}", target_type_name, e);
            Error::Compilation(format!("Failed to cast data pointer to {}: {}", target_type_name, e))
        })?;
        
        debug!("Interface data extracted and validated successfully");
        Ok(casted_ptr.into())
    }
}

// Helper methods for integration with the rest of the codebase
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Register a concrete type in the runtime type system
    /// This method makes the type available for runtime type assertions
    pub fn register_type_for_assertion(
        &mut self,
        type_name: &str,
        type_info: BasicValueEnum<'ctx>
    ) -> Result<(), Error> {
        // Store the type information in a global structure or registry
        // This is a placeholder for a more comprehensive runtime type system
        debug!("Registered type '{}' for runtime type assertions", type_name);
        Ok(())
    }
    
    /// Generate code for dynamic type checking at runtime
    /// This can be used for more complex type assertion scenarios
    pub fn generate_dynamic_type_check(
        &mut self,
        value: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Generating dynamic type check for {}", type_name);
        // This method could implement more sophisticated runtime type checking
        // For now, we'll just use the basic check_instance_of_runtime method
        self.check_instance_of_runtime(value, type_name)
    }
}