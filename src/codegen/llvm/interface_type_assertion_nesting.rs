//! # Interface Type Assertion Nesting Tracker
//! 
//! This module provides functionality to track nesting levels of interface type
//! assertions, which improves error reporting and debugging especially in complex
//! interface hierarchies. It provides context about the full interface hierarchy
//! when assertions fail.
//!
//! The implementation provides:
//! 1. Tracking nesting level of interface assertions
//! 2. Proper context for error messages in nested assertions
//! 3. Path information for interface inheritance chains
//! 4. Integration with the existing type assertion framework
//! 5. Support for both success and failure cases at all nesting levels

use inkwell::values::{BasicValueEnum, PointerValue};
use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::ast::expressions::TypeAssertion;

use tracing::{debug, error, info, instrument, trace, warn, Level, Span};

/// Maximum nesting level that will be tracked
/// This prevents potential infinite recursion in circular interface references
const MAX_NESTING_LEVEL: usize = 10;

/// Nesting context for type assertions
#[derive(Debug, Clone)]
pub struct TypeAssertionNestingContext {
    /// Current nesting level (0 = top level)
    level: usize,
    /// Chain of interface names in the path (from most to least specific)
    path: Vec<String>,
    /// Whether we're in debug mode
    debug_enabled: bool,
}

impl TypeAssertionNestingContext {
    /// Create a new top-level context
    pub fn new(debug_enabled: bool) -> Self {
        Self {
            level: 0,
            path: Vec::new(),
            debug_enabled,
        }
    }
    
    /// Enter a nested context, returning a new context object
    pub fn enter(&self, interface_name: &str) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(interface_name.to_string());
        
        Self {
            level: self.level + 1,
            path: new_path,
            debug_enabled: self.debug_enabled,
        }
    }
    
    /// Get the current nesting level
    pub fn level(&self) -> usize {
        self.level
    }
    
    /// Check if we've exceeded the maximum nesting level
    pub fn is_too_deep(&self) -> bool {
        self.level >= MAX_NESTING_LEVEL
    }
    
    /// Get the path as a string
    pub fn path_string(&self) -> String {
        if self.path.is_empty() {
            return String::new();
        }
        
        let mut result = String::new();
        for (i, name) in self.path.iter().enumerate() {
            if i > 0 {
                result.push_str(" -> ");
            }
            result.push_str(name);
        }
        result
    }
    
    /// Check if debugging is enabled
    pub fn is_debug_enabled(&self) -> bool {
        self.debug_enabled
    }
    
    /// Create an error with proper nesting context information
    pub fn create_error(&self, message: &str) -> Error {
        if self.path.is_empty() {
            return Error::Compilation(message.to_string());
        }
        
        let path_info = format!("In interface path: {}", self.path_string());
        Error::Compilation(format!("{} - {}", message, path_info))
    }
}

/// Default implementation creates a top-level context with debugging disabled
impl Default for TypeAssertionNestingContext {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Trait for implementing interface type assertions with nesting level tracking
pub trait NestedTypeAssertion<'ctx> {
    /// Compile a type assertion with nesting context information
    fn compile_nested_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        context: Option<TypeAssertionNestingContext>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific target type with nesting context
    fn check_instance_of_with_nesting(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        context: &TypeAssertionNestingContext
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate debugging information for nested type assertions
    fn debug_nested_type_assertion(
        &self,
        target_type: &str,
        success: bool,
        context: &TypeAssertionNestingContext
    ) -> Result<(), Error>;
}

impl<'ctx> NestedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion, context), level = "debug")]
    fn compile_nested_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        context: Option<TypeAssertionNestingContext>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a context if none was provided
        let context = context.unwrap_or_else(|| {
            // Check if debugging is enabled via environment variable
            let debug_enabled = std::env::var("CURSED_TYPE_DEBUG")
                .or_else(|_| std::env::var("CURSED_DEBUG"))
                .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
                .unwrap_or(false);
            
            TypeAssertionNestingContext::new(debug_enabled)
        });
        
        // Check if we've exceeded the maximum nesting level
        if context.is_too_deep() {
            return Err(context.create_error("Type assertion nesting too deep, possible circular reference"));
        }
        
        debug!("Compiling nested type assertion for {} at level {}", 
              type_assertion.type_name, context.level());
        
        // First compile the expression being asserted
        let expr_value = match ExpressionCompilation::compile_expression(self, type_assertion.expression.as_ref()) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to compile expression for nested type assertion: {}", e);
                return Err(context.create_error(
                    &format!("Failed to compile expression for type assertion: {}", e)
                ));
            }
        };
        
        // Create a new context for the next level
        let nested_context = context.enter(&type_assertion.type_name);
        
        // Use the error-handling implementation for the actual assertion
        let result = match self.check_instance_of_with_nesting(
            expr_value, 
            &type_assertion.type_name,
            &nested_context
        ) {
            Ok(val) => {
                // Extract success flag from assertion result
                let success = if val.is_struct_value() {
                    let success_val = self.builder().build_extract_value(
                        val.into_struct_value(),
                        1, // Success flag is second element
                        "success_flag"
                    ).map_err(|e| {
                        Error::Compilation(format!("Failed to extract success flag: {}", e))
                    })?;
                    
                    if success_val.is_int_value() {
                        success_val.into_int_value().get_zero_extended_constant() != 0
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                // Log debug info if needed
                if nested_context.is_debug_enabled() {
                    self.debug_nested_type_assertion(&type_assertion.type_name, success, &nested_context)?;
                }
                
                val
            },
            Err(e) => {
                error!("Nested type assertion error: {}", e);
                return Err(nested_context.create_error(
                    &format!("Type assertion failed for type '{}': {}", 
                        type_assertion.type_name, e)
                ));
            }
        };
        
        debug!("Nested type assertion compiled successfully at level {}", context.level());
        Ok(result)
    }
    
    #[instrument(skip(self, interface_value, context), level = "debug")]
    fn check_instance_of_with_nesting(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        context: &TypeAssertionNestingContext
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Checking if value is instance of {} at nesting level {}", 
              target_type_name, context.level());
        
        // Use the existing error handler to perform the actual check
        let result = self.check_instance_of_with_errors(interface_value, target_type_name)?;
        
        // Create a result tuple with the appropriate value and success flag
        let current_fn = self.current_function()
            .ok_or_else(|| context.create_error("No current function for nested type assertion"))?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "nested_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "nested_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "nested_assert_merge");
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            result.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| {
            context.create_error(&format!("Failed to build conditional branch: {}", e))
        })?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr_safe(interface_value)?;
        
        // Create the result structure (value and true flag)
        let type_id = self.get_type_id(target_type_name).map_err(|e| {
            context.create_error(&format!("Failed to get type ID for {}: {}", target_type_name, e))
        })?;
        
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| {
            context.create_error(&format!("Failed to cast data pointer: {}", e))
        })?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                context.create_error(&format!("Failed to build branch to merge block: {}", e))
            })?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.pointer_type().const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                context.create_error(&format!("Failed to build branch from failure block: {}", e))
            })?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create the result type (tuple of pointer and bool)
        let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
        
        let phi = self.builder().build_phi(
            result_type,
            "nested_assertion_result"
        ).map_err(|e| {
            context.create_error(&format!("Failed to build phi node: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Nested instance check completed at level {}", context.level());
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn debug_nested_type_assertion(
        &self,
        target_type: &str,
        success: bool,
        context: &TypeAssertionNestingContext
    ) -> Result<(), Error> {
        // Only log if debugging is enabled
        if !context.is_debug_enabled() {
            return Ok(());
        }
        
        // Create indentation based on nesting level for clearer logs
        let indent = "  ".repeat(context.level());
        
        if success {
            info!(
                "{}Type assertion SUCCESS: {} at nesting level {}", 
                indent, target_type, context.level()
            );
            
            if !context.path_string().is_empty() {
                debug!("{}Interface path: {}", indent, context.path_string());
            }
        } else {
            warn!(
                "{}Type assertion FAILED: {} at nesting level {}", 
                indent, target_type, context.level()
            );
            
            if !context.path_string().is_empty() {
                warn!("{}Interface path: {}", indent, context.path_string());
            }
        }
        
        Ok(())
    }
}

/// Register the module in the LLVM code generator
pub fn register_nested_type_assertion() {
    trace!("Registering nested type assertion implementation");
    // This function is called during LlvmCodeGenerator initialization
}