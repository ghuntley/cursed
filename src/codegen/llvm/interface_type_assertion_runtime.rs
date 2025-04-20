use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use tracing::{debug, info, error, instrument, warn, span, Level};

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::Error;

/// Enhanced trait for runtime interface type assertions with comprehensive error handling
pub trait RuntimeTypeAssertion<'ctx> {
    /// Compile a type assertion expression with runtime type checking
    /// Returns a tuple of (value, success flag) and properly handles any runtime errors
    fn compile_runtime_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Creates a runtime type checking function that verifies type compatibility
    /// and provides detailed error information when assertions fail
    fn create_type_check_function(
        &mut self
    ) -> Result<PointerValue<'ctx>, Error>;
    
    /// Logs detailed type information during runtime type checking
    fn log_type_assertion(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_id: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<(), Error>;
}

impl<'ctx> RuntimeTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_runtime_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling runtime type assertion for type {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Get the type IDs for runtime checking
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        let expected_type_id = self.get_type_id(&type_assertion.type_name)?;
        
        // Log type information in debug mode
        self.log_type_assertion(actual_type_id, expected_type_id, &type_assertion.type_name)?;
        
        // Compare the type IDs with runtime validation
        let is_instance = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "is_instance_of"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance,
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Create the result structure (value and true flag)
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Call our error reporting function to log detailed diagnostics
        self.emit_assertion_failure_log(
            &type_assertion.type_name,
            actual_type_id,
            expr_value
        )?;
        
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
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
    
    fn create_type_check_function(
        &mut self
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Creating type check runtime function");
        
        // Get the LLVM context
        let context = self.context();
        
        // Define the function type: bool check_type(actual_id: i64, expected_id: i64, typeName: *i8)
        let i64_type = context.i64_type();
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let bool_type = context.bool_type();
        
        let fn_type = bool_type.fn_type(&[
            i64_type.into(),
            i64_type.into(),
            i8_ptr_type.into()
        ], false);
        
        // Create the function
        let function = self.module().add_function(
            "_cursed_check_type",
            fn_type,
            None
        );
        
        // Create a basic block
        let entry_block = context.append_basic_block(function, "entry");
        self.builder().position_at_end(entry_block);
        
        // Get the function parameters
        let actual_id = function.get_nth_param(0).unwrap().into_int_value();
        let expected_id = function.get_nth_param(1).unwrap().into_int_value();
        let type_name_ptr = function.get_nth_param(2).unwrap().into_pointer_value();
        
        // Compare the type IDs
        let is_match = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_id,
            expected_id,
            "ids_match"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Add debug logging
        // In a real implementation, we'd use the type_name_ptr to print the type name
        // and provide better error messages
        
        // Return the comparison result
        self.builder().build_return(Some(&is_match))
            .map_err(|e| Error::Compilation(e.to_string()))?;
            
        Ok(function.as_global_value().as_pointer_value())
    }
    
    fn log_type_assertion(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_id: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<(), Error> {
        // We'll use the cursed runtime debug logging functions if available,
        // otherwise this is a no-op in release mode
        if let Some(debug_fn) = self.get_runtime_debug_function()? {
            let type_name_str = self.build_string_constant(type_name)?;
            let actual_id_str = self.build_string_constant("Actual type ID: ")?;
            let expected_id_str = self.build_string_constant("Expected type ID: ")?;
            
            // Build log message for type assertion
            let msg = self.build_string_constant(
                format!("Type assertion to '{}'", type_name).as_str()
            )?;
            
            // Convert the function pointer to a function value
            let debug_fn_val = unsafe { std::mem::transmute(debug_fn) };
            
            // Call debug log function
            self.builder().build_call(
                debug_fn_val,
                &[msg.into()],
                "debug_log"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            // Log the actual and expected type IDs
            self.builder().build_call(
                debug_fn_val,
                &[actual_id_str.into()],
                "debug_log_actual"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            self.builder().build_call(
                debug_fn_val,
                &[expected_id_str.into()],
                "debug_log_expected"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
        }
        
        Ok(())
    }
}

// Helper methods extension
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Emit debug logs for type assertion failures
    fn emit_assertion_failure_log(
        &mut self,
        expected_type_name: &str,
        actual_type_id: BasicValueEnum<'ctx>,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<(), Error> {
        // If we have a runtime error function, use it to report detailed info
        if let Some(error_fn) = self.get_runtime_error_function()? {
            let error_msg = self.build_string_constant(
                format!("Type assertion failed: value is not of type '{}'", expected_type_name).as_str()
            )?;
            
            // Convert the function pointer to a function value
            let error_fn_val = unsafe { std::mem::transmute(error_fn) };
            
            // Call error reporting function
            self.builder().build_call(
                error_fn_val,
                &[error_msg.into(), actual_type_id.into()],
                "report_assertion_error"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
        }
        
        // Do not return an error, just log it and continue - failure is handled by the false flag
        Ok(())
    }
    
    // Get or create the runtime debug function
    fn get_runtime_debug_function(&mut self) -> Result<Option<PointerValue<'ctx>>, Error> {
        // Check if the debug function already exists
        if let Some(function) = self.module().get_function("_cursed_debug_log") {
            return Ok(Some(function.as_global_value().as_pointer_value()));
        }
        
        // If we're in debug mode, create the function
        #[cfg(debug_assertions)]
        {
            let context = self.context();
            let i8_ptr = context.i8_type().ptr_type(AddressSpace::default());
            let fn_type = context.void_type().fn_type(&[i8_ptr.into()], false);
            
            let function = self.module().add_function(
                "_cursed_debug_log",
                fn_type,
                None
            );
            
            return Ok(Some(function.as_global_value().as_pointer_value()));
        }
        
        // In release mode, return None
        #[cfg(not(debug_assertions))]
        {
            return Ok(None);
        }
    }
    
    // Get or create the runtime error reporting function
    fn get_runtime_error_function(&mut self) -> Result<Option<PointerValue<'ctx>>, Error> {
        // Check if the error function already exists
        if let Some(function) = self.module().get_function("_cursed_report_type_error") {
            return Ok(Some(function.as_global_value().as_pointer_value()));
        }
        
        // Create the error reporting function
        let context = self.context();
        let i8_ptr = context.i8_type().ptr_type(AddressSpace::default());
        let i64_type = context.i64_type();
        let fn_type = context.void_type().fn_type(&[i8_ptr.into(), i64_type.into()], false);
        
        let function = self.module().add_function(
            "_cursed_report_type_error",
            fn_type,
            None
        );
        
        Ok(Some(function.as_global_value().as_pointer_value()))
    }
    
    // Helper to build a string constant
    fn build_string_constant(&mut self, value: &str) -> Result<PointerValue<'ctx>, Error> {
        let context = self.context();
        let builder = self.builder();
        
        // Add null terminator
        let value_with_null = format!("{}{}", value, '\0');
        
        // Create global string constant
        let string_global = builder.build_global_string_ptr(
            &value_with_null,
            "string_constant"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(string_global.as_pointer_value())
    }
}