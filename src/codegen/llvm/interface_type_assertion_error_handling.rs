//! # Enhanced Interface Type Assertion Error Handling
//!
//! This module provides improved error handling for interface type assertions,
//! specifically enhancing the integration between LLVM code generator and
//! interface type registry. It enables better error propagation with detailed
//! context and recovery suggestions.
//!
//! ## Features
//!
//! - Consistent error propagation between LLVM and type registry
//! - Structured error types that preserve context information
//! - Detailed error messages with inheritance path information
//! - Proper use of the `?` operator throughout the assertion pipeline
//! - Error recovery options when possible

use tracing::{debug, error, info, instrument, span, trace, warn, Level};
use std::fmt;

use inkwell::values::BasicValueEnum;
use inkwell::types::{BasicTypeEnum, StructType};
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use crate::error::Error;

/// Specialized error type for type assertion failures that preserves context
#[derive(Debug)]
pub struct TypeAssertionContextError {
    /// The source type that was being asserted
    pub source_type: String,
    
    /// The target type of the assertion
    pub target_type: String,
    
    /// Source code location of the assertion
    pub source_location: String,
    
    /// Detailed error message
    pub message: String,
    
    /// Path visualization if available
    pub path_visualization: Option<String>,
    
    /// Possible recovery suggestions
    pub recovery_hint: Option<String>,
}

impl fmt::Display for TypeAssertionContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type Assertion Error: {} is not a {}\nAt: {}", 
               self.source_type, self.target_type, self.source_location)?;
        
        if !self.message.is_empty() {
            write!(f, "\n\n{}", self.message)?;
        }
        
        if let Some(path) = &self.path_visualization {
            write!(f, "\n\nInheritance Path:\n{}", path)?;
        }
        
        if let Some(hint) = &self.recovery_hint {
            write!(f, "\n\nRecovery Suggestion: {}", hint)?;
        }
        
        Ok(())
    }
}

impl From<TypeAssertionContextError> for Error {
    fn from(err: TypeAssertionContextError) -> Self {
        Error::Compilation(err.to_string())
    }
}

/// Trait for enhanced error handling in interface type assertions
pub trait EnhancedTypeAssertionErrorHandling<'ctx>: 
    InterfaceTypeAssertion<'ctx> + 
    InterfaceTypeAssertionPathVisualization<'ctx> + 
    InterfaceRegistryIntegration 
{
    /// Compile a type assertion with enhanced error handling
    fn compile_type_assertion_with_enhanced_errors(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a context-rich error for type assertion failures
    fn create_assertion_context_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: &str,
        additional_info: Option<&str>
    ) -> Result<TypeAssertionContextError, Error>;
    
    /// Attempt to resolve type names from runtime type IDs for better errors
    fn resolve_type_name_for_error(
        &self,
        type_id: u64
    ) -> Result<String, Error>;
    
    /// Get potential recovery options for a failed assertion
    fn get_assertion_recovery_options(
        &self, 
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error>;
}

impl<'ctx> EnhancedTypeAssertionErrorHandling<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_enhanced_errors(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with enhanced error handling: {}", type_assertion.string());
        
        // Ensure registry visualization is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Extract source location for error reporting
        let source_location = type_assertion.token_literal();
        
        // Attempt to compile using the standard mechanism
        let result = self.compile_type_assertion(type_assertion);
        
        // If it succeeds, just return the result
        if result.is_ok() {
            return result;
        }
        
        // If it fails, try to extract detailed information and create an enhanced error
        if let Err(Error::Compilation(error_msg)) = &result {
            // Attempt to extract the source and target types from the error
            // This is a fallback in case direct source extraction fails
            let source_type = extract_source_type_from_error(error_msg)
                .unwrap_or_else(|| "Unknown Source Type".to_string());
                
            let target_type = type_assertion.type_name.clone();
            
            // Try to create a context-rich error with inheritance path visualization
            match self.create_assertion_context_error(
                &source_type,
                &target_type,
                &source_location,
                Some(error_msg)
            ) {
                Ok(context_error) => return Err(context_error.into()),
                Err(_) => {
                    // If enhanced error creation fails, return the original error
                    warn!("Failed to create enhanced error, returning original error");
                    return result;
                }
            }
        }
        
        // If error was not a compilation error, just return it
        result
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_assertion_context_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: &str,
        additional_info: Option<&str>
    ) -> Result<TypeAssertionContextError, Error> {
        debug!("Creating context error for assertion: {} to {}", source_type, target_type);
        
        // Generate path visualization if possible
        let path_visualization = match self.visualize_interface_path(source_type, target_type) {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        
        // Get recovery suggestions
        let recovery_hint = self.get_assertion_recovery_options(source_type, target_type)?;
        
        // Build detailed message including additional info
        let message = if let Some(info) = additional_info {
            format!("{}\n\nAdditional info: {}", 
                if path_visualization.is_none() {
                    format!("No inheritance path exists between '{}' and '{}'", 
                            source_type, target_type)
                } else {
                    "Inheritance path exists but assertion failed at runtime".to_string()
                },
                info
            )
        } else {
            if path_visualization.is_none() {
                format!("No inheritance path exists between '{}' and '{}'", 
                        source_type, target_type)
            } else {
                "Inheritance path exists but assertion failed at runtime".to_string()
            }
        };
        
        Ok(TypeAssertionContextError {
            source_type: source_type.to_string(),
            target_type: target_type.to_string(),
            source_location: source_location.to_string(),
            message,
            path_visualization,
            recovery_hint,
        })
    }
    
    #[instrument(skip(self), level = "debug")]
    fn resolve_type_name_for_error(
        &self,
        type_id: u64
    ) -> Result<String, Error> {
        // Try to resolve the type name from the registry
        if let Some(registry) = &self.interface_type_registry {
            if let Some(name) = registry.get_type_name(type_id) {
                return Ok(name.clone());
            }
        }
        
        // Fall back to a generic name with the ID
        Ok(format!("Type#{}", type_id))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_assertion_recovery_options(
        &self, 
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error> {
        // No registry visualization available
        if self.registry_visualization().is_none() {
            return Ok(Some("Add proper interface implementation to enable assertion".to_string()));
        }
        
        // Check if there's a reversed relationship
        let reversed = match self.check_interface_extension(target_type, source_type) {
            Ok(result) => result,
            Err(_) => false,
        };
        
        if reversed {
            return Ok(Some(format!(
                "The inheritance relationship appears to be reversed. '{}' extends '{}', not the other way around.",
                target_type, source_type
            )));
        }
        
        // Find common interfaces that both types implement
        let common_interfaces = match self.find_common_interfaces(source_type, target_type) {
            Ok(interfaces) => interfaces,
            Err(_) => vec![],
        };
        
        if !common_interfaces.is_empty() {
            let mut hint = format!("Both '{}' and '{}' implement these common interfaces:\n", 
                               source_type, target_type);
                               
            for (i, interface) in common_interfaces.iter().enumerate() {
                if i < 3 { // Limit to 3 examples
                    hint.push_str(&format!("  - {}\n", interface));
                } else {
                    hint.push_str(&format!("  - and {} more...\n", common_interfaces.len() - 3));
                    break;
                }
            }
            
            hint.push_str("Consider using one of these common interfaces instead.");
            return Ok(Some(hint));
        }
        
        // Default suggestion
        Ok(Some(format!(
            "Implement the '{}' interface for the '{}' type.",
            target_type, source_type
        )))
    }
}

// Helper methods for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find common interfaces that both types implement
    fn find_common_interfaces(
        &self, 
        type1: &str,
        type2: &str
    ) -> Result<Vec<String>, Error> {
        if let Some(registry) = self.registry_visualization() {
            // Get all interfaces that type1 implements
            let type1_interfaces = match InterfaceRegistryExtensionWithVisualization::get_extension_hierarchy(registry.as_ref()) {
                Ok(hierarchy) => {
                    let mut result = Vec::new();
                    for (source, targets) in hierarchy {
                        if source == type1 {
                            for target in targets {
                                result.push(target);
                            }
                        }
                    }
                    result
                },
                Err(_) => vec![],
            };
            
            // Get all interfaces that type2 implements
            let type2_interfaces = match InterfaceRegistryExtensionWithVisualization::get_extension_hierarchy(registry.as_ref()) {
                Ok(hierarchy) => {
                    let mut result = Vec::new();
                    for (source, targets) in hierarchy {
                        if source == type2 {
                            for target in targets {
                                result.push(target);
                            }
                        }
                    }
                    result
                },
                Err(_) => vec![],
            };
            
            // Find the intersection
            let common: Vec<String> = type1_interfaces.into_iter()
                .filter(|iface| type2_interfaces.contains(iface))
                .collect();
                
            return Ok(common);
        }
        
        Ok(vec![])
    }
}

/// Helper functions to extract type information from error messages
fn extract_source_type_from_error(error_msg: &str) -> Option<String> {
    // Attempt to extract the source type from error messages like:
    // "Value of type 'SourceType' cannot be asserted as 'TargetType'"
    if let Some(start_idx) = error_msg.find("Value of type '") {
        if let Some(end_idx) = error_msg[start_idx + 15..].find("'") {
            return Some(error_msg[start_idx + 15..start_idx + 15 + end_idx].to_string());
        }
    }
    
    // Try alternative patterns
    if let Some(start_idx) = error_msg.find("Type assertion error: ") {
        if let Some(is_idx) = error_msg[start_idx..].find(" is not a ") {
            return Some(error_msg[start_idx + 21..start_idx + is_idx].to_string());
        }
    }
    
    None
}

/// Register this module with the compiler
pub fn register_interface_type_assertion_error_handling() {
    debug!("Interface type assertion error handling module registered");
}