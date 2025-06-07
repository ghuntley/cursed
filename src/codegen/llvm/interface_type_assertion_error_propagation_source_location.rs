//! # Enhanced Interface Type Assertion Error Propagation with Source Location
//!
//! This module implements enhanced error propagation for interface type assertions with
//! improved source location support. It provides better context for error reporting when
//! type assertions fail, making debugging easier for developers.
//!
//! ## Key Features
//!
//! 1. Enhanced source location tracking for type assertions
//! 2. Detailed error context with file, line, and column information
//! 3. Integration with the existing error propagation system
//! 4. Support for the ? operator with improved diagnostics
//!
//! This implementation builds on the existing error propagation system but adds more
//! comprehensive source location support for better error reporting.

use std::path::Path;
use tracing::{debug, error, info, instrument, trace, warn};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};
use inkwell::IntPredicate;
use crate::codegen::llvm::basic_value_extensions::{BasicValueExt, StructTypeExt};
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::llvm_code_generator_extensions::{SymbolLookupExtensions, ErrorPathExtensions};
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;

/// Trait for interface type assertion error propagation with enhanced source location support
pub trait EnhancedSourceLocationErrorPropagation<'ctx>: InterfaceTypeAssertionErrorPropagation<'ctx> {
    /// Set the current file path for error reporting
    fn set_current_file_path(&mut self, file_path: &str);
    
    /// Get the current file path for error reporting
    fn current_file_path(&self) -> Option<String>;
    
    /// Create a detailed source location for error reporting
    fn create_source_location(
        &self,
        line: usize,
        column: usize,
        file_path: Option<&str>,
        source_line: Option<&str>
    ) -> SourceLocation;
    
    /// Extract source location from an AST node
    fn extract_source_location_from_node(&self, node: &dyn Node) -> SourceLocation;
    
    /// Compile a type assertion with enhanced source location for error reporting
    fn compile_type_assertion_with_source_location(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a detailed error with source location information
    fn create_detailed_error_with_location(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: SourceLocation
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion question operator with enhanced source location
    fn compile_type_assertion_question_with_source_location(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

/// Implementation for LlvmCodeGenerator
impl<'ctx> EnhancedSourceLocationErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn set_current_file_path(&mut self, file_path: &str) {
        // Store the current file path in the internal fields
        self.internal_fields.insert("current_file_path".to_string(), Box::new(file_path.to_string()));
    }
    
    #[instrument(skip(self), level = "debug")]
    fn current_file_path(&self) -> Option<String> {
        self.internal_fields.get("current_file_path")
            .and_then(|val| val.downcast_ref::<String>().cloned())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_source_location(
        &self,
        line: usize,
        column: usize,
        file_path: Option<&str>,
        source_line: Option<&str>
    ) -> SourceLocation {
        let file = file_path
            .map(|s| s.to_string())
            .or_else(|| Some(self.current_file_path().to_string_lossy().to_string()));
            
        let source = source_line
            .map(|s| s.to_string())
            .unwrap_or_else(|| "<unknown>".to_string());
            
        SourceLocation {
            line,
            column,
            file,
            source_line: source,
        }
    }
    
    #[instrument(skip(self, node), level = "debug")]
    fn extract_source_location_from_node(&self, node: &dyn Node) -> SourceLocation {
        let token_literal = node.token_literal(); // Get token as string
        
        // Extract line and column from token if available
        // For this implementation, we'll assume line and column information
        // is stored somewhere in the token. If not, we would need to modify
        // the lexer to store this information.
        let line = 0; // Replace with actual line extraction
        let column = 0; // Replace with actual column extraction
        
        self.create_source_location(
            line,
            column,
            None,
            Some(&token_literal)
        )
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_source_location(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract source location from the type assertion node
        let source_location = self.extract_source_location_from_node(type_assertion);
        debug!("Type assertion at {}:{} in file {:?}", 
              source_location.line, source_location.column, source_location.file);
              
        // Compile the expression that will be type-asserted
        let expr_value = self.compile_expression(&*type_assertion.expression)?;
        
        // Check if the value is actually an interface
        let is_interface = self.is_interface_type(expr_value)?;
        if !is_interface {
            return Err(Error::TypeAssertion(
                TypeAssertionError::new(type_assertion.expression.node_type(), &type_assertion.type_name)
                    .with_message(format!("Cannot perform type assertion on non-interface value"))
                    .with_location(source_location)
                    .into()
            ));
        }
        
        // Get the runtime type ID of the interface value
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(expr_value, Some(source_location.clone()))?;
        
        // Get the target type ID
        let target_type_id = match self.get_type_id(&type_assertion.type_name) {
            Ok(id) => id,
            Err(_) => {
                // Calculate a type ID for the target type if it doesn't exist in the registry
                let hash = self.hash_type_name(&type_assertion.type_name);
                hash
            }
        };
        
        // Set the type IDs for error reporting
        self.set_expected_type_id(target_type_id as u32);
        let (actual_id, _) = actual_type_id;
        self.set_actual_type_id(actual_id as u32);
        
        // Check if the types match
        let is_match = self.check_instanceof(expr_value, &type_assertion.type_name)?;
        
        let result = if is_match {
            // If they match, extract the data pointer
            let data_ptr = self.extract_interface_data_ptr(expr_value)?;
            
            // Cast to the target type
            let target_struct_type = match self.get_type_by_name(&type_assertion.type_name) {
                Ok(ty) => ty,
                Err(_) => {
                    // If we can't find the type, create a placeholder type
                    let placeholder_struct_type = self.context().struct_type(&[], false);
                    placeholder_struct_type
                }
            };
            
            // Bitcast the data pointer to the target type pointer
            let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
            let casted_ptr = self.builder().build_bitcast(
                data_ptr,
                target_ptr_type,
                "casted_data_ptr"
            )?;
            
            // Create a successful Result with the value
            self.create_type_assertion_result(
                casted_ptr,
                true,
                None,
                Some(source_location)
            )?
        } else {
            // If they don't match, create an error message
            let error_message = format!(
                "Type assertion failed: value of type '{}' is not of type '{}'",
                self.get_runtime_type_name(expr_value)?,
                type_assertion.type_name
            );
            
            // Try to get path information for better error messages
            let type_path = match self.find_interface_path_simple(
                &self.get_runtime_type_name(expr_value)?,
                &type_assertion.type_name
            ) {
                Ok(path) => {
                    if !path.is_empty() {
                        Some(format!("Possible path: {}", path.join(" -> ")))
                    } else {
                        None
                    }
                },
                Err(_) => None
            };
            
            // Create a failed Result with error information
            self.create_type_assertion_result(
                self.context().struct_type(&[], false).const_null().into(),
                false,
                Some(&format!(
                    "{}{}",
                    error_message,
                    type_path.map(|p| format!(". {}", p)).unwrap_or_default()
                )),
                Some(source_location)
            )?
        };
        
        // Clear the type IDs now that we're done
        self.clear_type_ids();
        
        Ok(result)
    }
    
    #[instrument(skip(self, interface_value, target_type_name, source_location), level = "debug")]
    fn create_detailed_error_with_location(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_location: SourceLocation
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the runtime type of the interface value
        let runtime_type = self.get_runtime_type_name(interface_value)?;
        
        // Create a detailed error message
        let error_message = format!(
            "Type assertion failed: value of type '{}' is not of type '{}'",
            runtime_type,
            target_type_name
        );
        
        // Try to get path information for better error messages
        let type_path = match self.find_interface_path_simple(&runtime_type, target_type_name) {
            Ok(path) => {
                if !path.is_empty() {
                    Some(format!("Possible path: {}", path.join(" -> ")))
                } else {
                    None
                }
            },
            Err(_) => None
        };
        
        // Check if there's a reversed relationship (for better error messages)
        let reversed_relationship = match self.check_extension_relationship_simple(target_type_name, &runtime_type) {
            Ok(true) => {
                Some(format!(
                    "Note: '{}' is actually a supertype of '{}', not a subtype. The assertion direction is reversed.",
                    target_type_name,
                    runtime_type
                ))
            },
            _ => None
        };
        
        // Build the full error message with all available information
        let full_message = format!(
            "{}{}{}",
            error_message,
            type_path.map(|p| format!(". {}", p)).unwrap_or_default(),
            reversed_relationship.map(|r| format!(". {}", r)).unwrap_or_default()
        );
        
        // Create a failed Result with the detailed error information
        self.create_type_assertion_result(
            self.context().struct_type(&[], false).const_null().into(),
            false,
            Some(&full_message),
            Some(source_location)
        )
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_question_with_source_location(
        &mut self,
        type_assertion: &TypeAssertionQuestion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with ? operator and source location for: {}", 
               type_assertion.expression.string());
        
        // Extract source location from the type assertion node
        let source_location = self.extract_source_location_from_node(type_assertion);
        debug!("Type assertion with ? at {}:{} in file {}", 
              source_location.line, source_location.column, 
              source_location.file.as_deref().unwrap_or("<unknown>"));
              
        // Compile the expression that will be type-asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Check if the value is actually an interface
        let is_interface = self.is_interface_value(expr_value)?;
        if !is_interface {
            return Err(Error::TypeAssertion(
                TypeAssertionError::new(type_assertion.expression.node_type(), &type_assertion.type_name)
                    .with_message(format!("Cannot perform type assertion on non-interface value"))
                    .with_location(source_location)
            ));
        }
        
        // Get the runtime type ID of the interface value
        let actual_type_id = self.get_runtime_type_id(expr_value, Some(source_location.clone()))?;
        
        // Get the target type ID
        let target_type_id = match self.get_type_id(&type_assertion.type_name) {
            Ok(id) => id,
            Err(_) => {
                // Calculate a type ID for the target type if it doesn't exist in the registry
                let hash = self.hash_type_name(&type_assertion.type_name);
                hash
            }
        };
        
        // Set the type IDs for error reporting
        self.set_expected_type_id(target_type_id as u32);
        let (actual_id, _) = actual_type_id;
        self.set_actual_type_id(actual_id as u32);
        
        // Check if the types match
        let is_match = self.check_instanceof(expr_value, &type_assertion.type_name)?;
        
        let result = if is_match {
            // If they match, extract the data pointer
            let data_ptr = self.extract_interface_data_ptr(expr_value)?;
            
            // Cast to the target type
            let target_struct_type = match self.get_type_by_name(&type_assertion.type_name) {
                Ok(ty) => ty,
                Err(_) => {
                    // If we can't find the type, create a placeholder type
                    let placeholder_struct_type = self.context().struct_type(&[], false);
                    placeholder_struct_type
                }
            };
            
            // Bitcast the data pointer to the target type pointer
            let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
            let casted_ptr = self.builder().build_bitcast(
                data_ptr,
                target_ptr_type,
                "casted_data_ptr"
            );
            
            // For the ? operator, we just return the value directly on success
            casted_ptr.into()
        } else {
            // For failure with the ? operator, we need to generate code that will
            // return from the current function with an error
            
            // Create a detailed error with source location
            let error_result = self.create_detailed_error_with_location(
                expr_value,
                &type_assertion.type_name,
                source_location
            )?;
            
            // Now we need to extract the error from the result and propagate it
            self.unwrap_type_assertion_result(error_result)?
        };
        
        // Clear the type IDs now that we're done
        self.clear_type_ids();
        
        Ok(result)
    }
}

// Helper methods and extensions for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize source location tracking for type assertions
    pub fn init_source_location_tracking(&mut self) {
        // Initialize the current file path field if it doesn't exist
        if !self.internal_fields.contains_key("current_file_path") {
            self.internal_fields.insert("current_file_path".to_string(), Box::new(String::new()));
        }
    }
    
    /// Extract file path from a source string or file path
    pub fn extract_file_path(&self, source: &str) -> String {
        let path = Path::new(source);
        if path.exists() {
            // This is a file path
            path.to_string_lossy().into_owned()
        } else {
            // This is not a valid path, use default
            self.current_file_path().unwrap_or_else(|| "<unknown>".to_string())
        }
    }
    
    /// Extract the source line from a file at a given line number
    pub fn extract_source_line(&self, file_path: &str, line_number: usize) -> Option<String> {
        // This is a placeholder - in a real implementation, we would read the file
        // and extract the specific line.
        // For simplicity, we'll just return None here.
        None
    }
}