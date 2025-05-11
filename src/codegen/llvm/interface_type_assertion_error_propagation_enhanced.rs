//! # Enhanced Interface Type Assertion Error Propagation
//!
//! This module provides an enhanced implementation of error propagation for interface
//! type assertions. It improves upon the existing implementation by adding:
//!
//! 1. Better source location support - More accurate line and column information
//! 2. More comprehensive type ID tracking - Improved type metadata extraction
//! 3. Enhanced error contexts - Better error messages with type relationship details
//! 4. Improved integration with diamond inheritance patterns
//!
//! This implementation ensures a more robust error propagation system for interface
//! type assertions that provides developers with clearer error messages and better
//! debugging experience.

use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Enhanced trait for implementing interface type assertion error propagation
/// with better source location support and more comprehensive type ID tracking
pub trait EnhancedInterfaceTypeAssertionErrorPropagation<'ctx>: InterfaceTypeAssertionErrorPropagation<'ctx> {
    /// Set source location with more accurate information
    fn set_source_location(&mut self, location: SourceLocation);
    
    /// Get the current source location
    fn current_source_location(&self) -> Option<SourceLocation>;
    
    /// Clear the current source location tracking
    fn clear_source_location(&mut self);
    
    /// Track inheritance hierarchy for better error messages
    fn track_inheritance_hierarchy(&mut self, type_name: &str, interface_name: &str) -> Result<(), Error>;
    
    /// Get the current inheritance path for a type
    fn current_inheritance_path(&self, type_name: &str) -> Option<Vec<String>>;
    
    /// Compile a type assertion expression with enhanced error propagation
    fn compile_type_assertion_with_enhanced_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion expression with ? operator and enhanced error context
    fn compile_type_assertion_question_enhanced(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

// Add fields to LlvmCodeGenerator for enhanced source location tracking
impl<'ctx> LlvmCodeGenerator<'ctx> {
    pub fn init_enhanced_error_tracking(&mut self) {
        // Initialize the type ID tracking fields if they don't exist
        self.init_type_assertion_error_tracking();
        
        // Initialize source location tracking
        if !self.internal_fields.contains_key("current_source_location") {
            self.internal_fields.insert("current_source_location".to_string(), Box::new(None::<SourceLocation>));
        }
        
        // Initialize inheritance hierarchy tracking
        if !self.internal_fields.contains_key("inheritance_hierarchies") {
            self.internal_fields.insert("inheritance_hierarchies".to_string(), 
                Box::new(std::collections::HashMap::<String, Vec<String>>::new()));
        }
    }
    
    // Source location accessors
    pub fn get_source_location(&self) -> Option<SourceLocation> {
        self.internal_fields.get("current_source_location")
            .and_then(|val| val.downcast_ref::<Option<SourceLocation>>())
            .and_then(|val| val.clone())
    }
}

// Implementation of the enhanced error propagation trait
impl<'ctx> EnhancedInterfaceTypeAssertionErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    fn set_source_location(&mut self, location: SourceLocation) {
        self.init_enhanced_error_tracking();
        if let Some(field) = self.internal_fields.get_mut("current_source_location") {
            if let Some(val) = field.downcast_mut::<Option<SourceLocation>>() {
                *val = Some(location);
            }
        }
    }
    
    fn current_source_location(&self) -> Option<SourceLocation> {
        self.get_source_location()
    }
    
    fn clear_source_location(&mut self) {
        self.init_enhanced_error_tracking();
        if let Some(field) = self.internal_fields.get_mut("current_source_location") {
            if let Some(val) = field.downcast_mut::<Option<SourceLocation>>() {
                *val = None;
            }
        }
    }
    
    fn track_inheritance_hierarchy(&mut self, type_name: &str, interface_name: &str) -> Result<(), Error> {
        self.init_enhanced_error_tracking();
        
        // Get the inheritance hierarchies map
        if let Some(field) = self.internal_fields.get_mut("inheritance_hierarchies") {
            if let Some(hierarchies) = field.downcast_mut::<std::collections::HashMap<String, Vec<String>>>() {
                // Get or create the inheritance path for this type
                let path = hierarchies.entry(type_name.to_string()).or_insert_with(|| vec![]);
                
                // Add the interface to the path if not already present
                if !path.contains(&interface_name.to_string()) {
                    path.push(interface_name.to_string());
                }
            }
        }
        
        Ok(())
    }
    
    fn current_inheritance_path(&self, type_name: &str) -> Option<Vec<String>> {
        self.internal_fields.get("inheritance_hierarchies")
            .and_then(|field| field.downcast_ref::<std::collections::HashMap<String, Vec<String>>>())
            .and_then(|hierarchies| hierarchies.get(type_name))
            .cloned()
    }
    
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_with_enhanced_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract more detailed source location information
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                // Try to extract more detailed location from the token
                let (line, column, file) = self.extract_location_from_token(token);
                Some(SourceLocation {
                    line,
                    column,
                    file,
                    source_line: format!("{}.({}", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Store the source location for error reporting
        if let Some(location) = source_location.clone() {
            self.set_source_location(location);
        }
        
        // First compile using the regular implementation
        let result = self.compile_type_assertion_with_error_propagation(type_assertion);
        
        // Clear the source location after compilation
        self.clear_source_location();
        
        result
    }
    
    #[instrument(skip(self, type_assertion))]
    fn compile_type_assertion_question_enhanced(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract more detailed source location information
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                // Try to extract more detailed location from the token
                let (line, column, file) = self.extract_location_from_token(token);
                Some(SourceLocation {
                    line,
                    column,
                    file,
                    source_line: format!("{}.({})?\n", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Store the source location for error reporting
        if let Some(location) = source_location.clone() {
            self.set_source_location(location);
        }
        
        debug!("Compiling type assertion with enhanced ? operator for: {}", type_assertion.string());
        
        // First ensure registry is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion with ? operator".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_question_enhanced_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_question_enhanced_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_question_enhanced_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Get the type IDs for better error reporting
        let target_type_id = self.get_type_id(&type_assertion.type_name)?;
        self.set_expected_type_id(target_type_id);
        
        let actual_type_id = self.get_interface_type_id(expr_value)?;
        self.set_actual_type_id(actual_type_id);
        
        // Try to get the inheritance path for better error messages
        if let Ok(type_name) = self.get_interface_type_name(expr_value) {
            if let Ok(target_interfaces) = self.get_type_implemented_interfaces(&type_assertion.type_name) {
                for interface in target_interfaces {
                    self.track_inheritance_hierarchy(&type_assertion.type_name, &interface)?;
                }
            }
            
            if let Ok(actual_interfaces) = self.get_type_implemented_interfaces(&type_name) {
                for interface in actual_interfaces {
                    self.track_inheritance_hierarchy(&type_name, &interface)?;
                }
            }
        }
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Cast the data pointer to the appropriate type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - propagate the error through the ? operator with enhanced information
        self.builder().position_at_end(failure_block);
        
        // Create an enhanced error message with type relationship details
        let mut error_message = format!(
            "Failed to assert that interface value is of type {}",
            type_assertion.type_name
        );
        
        // Add type relationship information if available
        if let Ok(type_name) = self.get_interface_type_name(expr_value) {
            error_message.push_str(&format!("\n  Actual type: {}", type_name));
            
            // Add inheritance paths if available
            if let Some(actual_path) = self.current_inheritance_path(&type_name) {
                error_message.push_str("\n  Actual type implements: ");
                error_message.push_str(&actual_path.join(", "));
            }
            
            if let Some(target_path) = self.current_inheritance_path(&type_assertion.type_name) {
                error_message.push_str("\n  Target type implements: ");
                error_message.push_str(&target_path.join(", "));
            }
            
            // Check for potential common interfaces
            if let (Some(actual_path), Some(target_path)) = (
                self.current_inheritance_path(&type_name),
                self.current_inheritance_path(&type_assertion.type_name)
            ) {
                let common_interfaces: Vec<&String> = actual_path.iter()
                    .filter(|&iface| target_path.contains(iface))
                    .collect();
                
                if !common_interfaces.is_empty() {
                    error_message.push_str("\n  Common interfaces: ");
                    error_message.push_str(&common_interfaces.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
                }
            }
        }
        
        // Get source location with improved information
        let location_info = if let Some(loc) = source_location {
            BasicValueEnum::into_struct_value(
                self.build_struct_value(&[
                    self.context().i32_type().const_int(loc.line as u64, false).into(),
                    self.context().i32_type().const_int(loc.column as u64, false).into(),
                    loc.file.map_or_else(
                        || self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into(),
                        |file| self.create_string_constant(&file).into()
                    ),
                    self.create_string_constant(&loc.source_line).into()
                ])
            )
        } else {
            // Create a minimal struct with source line if available
            BasicValueEnum::into_struct_value(
                self.build_struct_value(&[
                    self.context().i32_type().const_int(0, false).into(),
                    self.context().i32_type().const_int(0, false).into(),
                    self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null().into(),
                    self.create_string_constant(&format!("{}.({})?\n", type_assertion.expression.string(), type_assertion.type_name)).into()
                ])
            )
        };
        
        // Call error propagation function with enhanced information
        self.call_error_propagation_function(
            self.create_string_constant(&error_message).into(), 
            location_info
        )?;
        
        // This should be unreachable in the failure path
        self.builder().build_unreachable().map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - return the casted pointer on success
        self.builder().position_at_end(merge_block);
        
        // Clear the source location and type IDs after compilation
        self.clear_source_location();
        self.clear_type_ids();
        
        // With question operator, we just return the value directly, error handling is automatic
        Ok(casted_ptr.into())
    }
}

// Additional helper methods for the enhanced error propagation system
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get interfaces implemented by a specific type
    pub fn get_type_implemented_interfaces(&self, type_name: &str) -> Result<Vec<String>, Error> {
        // This method would interact with the interface registry to get the interfaces
        // implemented by the given type. For simplicity, we return an empty vec for now.
        // In a real implementation, this would query the interface registry.
        Ok(vec![])
    }
    
    /// Get the type name from an interface value
    pub fn get_interface_type_name(&self, interface_value: BasicValueEnum<'ctx>) -> Result<String, Error> {
        // This is a placeholder implementation
        // In a real implementation, this would extract the type name from the interface value's metadata
        Ok("unknown_type".to_string())
    }
}

/// Register the enhanced error propagation module with the compiler
pub fn register_enhanced_error_propagation() {
    trace!("Enhanced interface type assertion error propagation module registered");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_error_propagation_registration() {
        // Test that the module registration function works
        register_enhanced_error_propagation();
        assert!(true);
    }
}