//! # Enhanced Interface Type Assertion Path Visualization
//!
//! This module improves the interface type assertion path visualization system with better
//! error handling and more consistent error propagation. It enhances debugging and error
//! reporting for interface inheritance relationships, making it easier for developers to
//! understand complex interface hierarchies.
//!
//! ## Key Improvements
//!
//! 1. Consistent use of `?` operator for all registry operations
//! 2. Enhanced error context in path visualization
//! 3. Improved error recovery in visualization functions
//! 4. Better handling of complex inheritance hierarchies
//! 5. Cleaner code structure for error propagation

use inkwell::values::BasicValueEnum;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use crate::error::Error;

/// Enhanced trait for visualizing interface inheritance paths with improved error propagation
pub trait EnhancedInterfaceTypeAssertionPathVisualization<'ctx>: InterfaceTypeAssertionPathVisualization<'ctx> {
    /// Access the interface registry for visualization
    fn interface_registry(&self) -> &dyn InterfaceRegistryExtensionWithVisualization;
    
    /// Access the interface registry with mutable reference for visualization
    fn interface_registry_mut(&mut self) -> &mut dyn InterfaceRegistryExtensionWithVisualization;
    /// Generate a DOT graph representation of the interface hierarchy with enhanced error handling
    fn generate_interface_hierarchy_dot_enhanced(&self) -> Result<String, Error>;
    
    /// Find alternative paths between interfaces with improved error recovery
    fn find_alternative_paths_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error>;
    
    /// Generate enhanced error message with path information and improved error context
    fn generate_path_error_message_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error>;
    
    /// Visualize interface path with better error context
    fn visualize_interface_path_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error>;
    
    /// Enhanced type assertion with path visualization and improved error propagation
    fn compile_type_assertion_with_path_visualization_enhanced(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedInterfaceTypeAssertionPathVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn interface_registry(&self) -> &dyn InterfaceRegistryExtensionWithVisualization {
        // The interface extension registry is stored in the registry_extensions field
        // of the LlvmCodeGenerator, which is initialized in the constructor
        // It now uses the adapter pattern to resolve implementation conflicts
        &self.registry_extensions
    }
    
    fn interface_registry_mut(&mut self) -> &mut dyn InterfaceRegistryExtensionWithVisualization {
        // The interface extension registry is stored in the registry_extensions field
        // of the LlvmCodeGenerator, which is initialized in the constructor
        // It now uses the adapter pattern to resolve implementation conflicts
        &mut self.registry_extensions
    }
    #[instrument(skip(self), level = "debug")]
    fn generate_interface_hierarchy_dot_enhanced(&self) -> Result<String, Error> {
        debug!("Generating enhanced DOT graph for interface hierarchy with improved error handling");
        
        let mut dot = String::from("digraph interface_hierarchy {\n");
        writeln!(dot, "  node [shape=box, style=filled, fillcolor=lightblue];").map_err(|e| {
            Error::Compilation(format!("Failed to write to DOT graph: {}", e))
        })?;
        
        // Get the complete hierarchy from the registry with proper error propagation
        let hierarchy = self.interface_registry().get_extension_hierarchy()?;
        
        // Add all nodes first with consistent error handling
        let mut all_interfaces = HashSet::new();
        
        // Collect all interface names with proper error handling
        for (source, targets) in &hierarchy {
            all_interfaces.insert(source.clone());
            for target in targets {
                all_interfaces.insert(target.clone());
            }
        }
        
        // Add nodes to DOT with proper error propagation
        for interface in &all_interfaces {
            writeln!(dot, "  \"{}\" [label=\"{}\"];", interface, interface).map_err(|e| {
                Error::Compilation(format!("Failed to write node to DOT graph: {}", e))
            })?;
        }
        
        // Add edges with proper error propagation
        for (source, targets) in &hierarchy {
            for target in targets {
                writeln!(dot, "  \"{}\" -> \"{}\";", source, target).map_err(|e| {
                    Error::Compilation(format!("Failed to write edge to DOT graph: {}", e))
                })?;
            }
        }
        
        // Finalize the DOT graph with proper error handling
        writeln!(dot, "}}").map_err(|e| {
            Error::Compilation(format!("Failed to finalize DOT graph: {}", e))
        })?;
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_alternative_paths_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding alternative paths from {} to {} with enhanced error recovery", 
               source_interface, target_interface);
        
        // Get all interfaces with proper error propagation
        let all_interfaces = self.interface_registry().get_all_interfaces()?;
        
        let mut alternative_paths = Vec::new();
        
        // Try indirect paths through other interfaces with improved error handling
        for intermediate in all_interfaces {
            // Skip source and target
            let intermediate_str = intermediate.as_str();
            if intermediate_str == source_interface || intermediate_str == target_interface {
                continue;
            }
            
            // Try to find a path from source to intermediate with proper error recovery
            let path1 = match self.find_interface_path(source_interface, intermediate_str) {
                Ok(path) => path,
                Err(e) => {
                    // Log error but continue with other paths - improved recovery
                    trace!("Could not find path from {} to {}: {}", 
                           source_interface, intermediate_str, e);
                    continue;
                }
            };
            
            // Try to find a path from intermediate to target with proper error recovery
            let path2 = match self.find_interface_path(intermediate_str, target_interface) {
                Ok(path) => path,
                Err(e) => {
                    // Log error but continue with other paths - improved recovery
                    trace!("Could not find path from {} to {}: {}", 
                           intermediate_str, target_interface, e);
                    continue;
                }
            };
            
            // Combine paths (remove duplicate intermediate node)
            let mut combined_path = path1;
            combined_path.extend(path2.into_iter().skip(1));
            
            alternative_paths.push(combined_path);
            
            // Limit the number of alternatives
            if alternative_paths.len() >= max_alternatives {
                break;
            }
        }
        
        Ok(alternative_paths)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_path_error_message_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        debug!("Generating enhanced error message for failed type assertion from {} to {}", 
               source_interface, target_interface);
        
        let mut message = format!(
            "Type assertion error at {}: Value of type '{}' cannot be asserted as type '{}'",
            source_location, source_interface, target_interface
        );
        
        // Try to find alternative paths with enhanced error handling
        let paths = self.find_alternative_paths_enhanced(source_interface, target_interface, 3)?;
        
        if !paths.is_empty() {
            writeln!(message, "\n\nAlternative paths between these interfaces:").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            for (i, path) in paths.iter().enumerate() {
                writeln!(message, "\nPath {}:", i + 1).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
                
                for (j, interface) in path.iter().enumerate() {
                    if j > 0 {
                        writeln!(message, "  ↓ extends").map_err(|e| {
                            Error::Compilation(format!("Failed to write to error message: {}", e))
                        })?;
                    }
                    writeln!(message, "  [{}]", interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                }
            }
            
            writeln!(
                message,
                "\nConsider implementing the missing interfaces in the hierarchy."
            ).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
        } else {
            writeln!(
                message,
                "\n\nNo viable inheritance path exists between these interfaces."
            ).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            // List all interfaces that the source implements with proper error propagation
            match self.interface_registry().get_direct_extensions(source_interface) {
                Ok(Some(implementations)) if !implementations.is_empty() => {
                    writeln!(message, "\n'{}' directly extends these interfaces:", source_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    for impl_interface in &implementations {
                        writeln!(message, "  - {}", impl_interface).map_err(|e| {
                            Error::Compilation(format!("Failed to write to error message: {}", e))
                        })?;
                    }
                },
                Ok(_) => {
                    // No implementations found - provide helpful message
                    writeln!(message, "\n'{}' does not extend any interfaces.", source_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                },
                Err(e) => {
                    // Handle registry error gracefully
                    writeln!(message, "\nError retrieving interface information: {}", e).map_err(|e2| {
                        Error::Compilation(format!("Failed to write to error message: {}", e2))
                    })?;
                }
            }
            
            // List all interfaces that extend the target with proper error propagation
            match self.interface_registry().get_direct_implementors(target_interface) {
                Ok(Some(implementors)) if !implementors.is_empty() => {
                    writeln!(message, "\nThese interfaces directly extend '{}':", target_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    for impl_interface in &implementors {
                        writeln!(message, "  - {}", impl_interface).map_err(|e| {
                            Error::Compilation(format!("Failed to write to error message: {}", e))
                        })?;
                    }
                },
                Ok(_) => {
                    // No implementors found - provide helpful message
                    writeln!(message, "\nNo interfaces directly extend '{}'.", target_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                },
                Err(e) => {
                    // Handle registry error gracefully
                    writeln!(message, "\nError retrieving implementor information: {}", e).map_err(|e2| {
                        Error::Compilation(format!("Failed to write to error message: {}", e2))
                    })?;
                }
            }
        }
        
        Ok(message)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_interface_path_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        debug!("Visualizing path from {} to {} with enhanced error handling", 
               source_interface, target_interface);
        
        // Find the path with proper error propagation
        let path = self.find_interface_path(source_interface, target_interface)?;
        
        // Generate a simple ASCII art representation with consistent error handling
        let mut result = String::new();
        
        writeln!(result, "Interface Inheritance Path:").map_err(|e| {
            Error::Compilation(format!("Failed to write to path visualization: {}", e))
        })?;
        
        for (i, interface) in path.iter().enumerate() {
            if i > 0 {
                writeln!(result, "  ↓ extends").map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
            }
            writeln!(result, "  [{}]", interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to path visualization: {}", e))
            })?;
        }
        
        // Also generate a DOT subgraph for just this path
        writeln!(result, "\nDOT representation:").map_err(|e| {
            Error::Compilation(format!("Failed to write to path visualization: {}", e))
        })?;
        
        writeln!(result, "digraph path {{").map_err(|e| {
            Error::Compilation(format!("Failed to write to path visualization: {}", e))
        })?;
        
        writeln!(result, "  node [shape=box, style=filled, fillcolor=lightblue];").map_err(|e| {
            Error::Compilation(format!("Failed to write to path visualization: {}", e))
        })?;
        
        for i in 0..path.len() {
            writeln!(result, "  \"{}\" [label=\"{}\"];", path[i], path[i]).map_err(|e| {
                Error::Compilation(format!("Failed to write to path visualization: {}", e))
            })?;
            
            if i < path.len() - 1 {
                writeln!(result, "  \"{}\" -> \"{}\";", path[i], path[i + 1]).map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
            }
        }
        
        writeln!(result, "}}").map_err(|e| {
            Error::Compilation(format!("Failed to write to path visualization: {}", e))
        })?;
        
        Ok(result)
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_path_visualization_enhanced(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with enhanced path visualization: {}", 
               type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Try to compile using standard type assertion logic with error propagation
        let result = TypeAssertionErrorHandler::compile_type_assertion_with_errors(self, type_assertion);
        
        // If compilation fails, enhance the error message with path information
        if let Err(Error::Compilation(err_msg)) = &result {
            // Extract interface names from error message with proper error handling
            if let (Some(source_type), Some(target_type)) = 
                (extract_source_type_from_error(&err_msg), extract_target_type_from_error(&err_msg)) {
                
                // Generate enhanced error message with path visualization
                match self.generate_path_error_message_enhanced(
                    &source_type,
                    &target_type,
                    &source_location
                ) {
                    Ok(enhanced_msg) => {
                        // Return the enhanced error
                        return Err(Error::Compilation(enhanced_msg));
                    },
                    Err(visualization_err) => {
                        // Log visualization error but continue with original error
                        warn!("Failed to generate enhanced error message: {}", visualization_err);
                        // Enhanced error fallback to avoid breaking compilation
                        let fallback_msg = format!(
                            "{}\n\nNote: Additional visualization information could not be generated: {}",
                            err_msg, visualization_err
                        );
                        return Err(Error::Compilation(fallback_msg));
                    }
                }
            }
        }
        
        // Return the original result (success or original error if we couldn't enhance it)
        result
    }
}

// Helper functions to extract type names from error messages with improved robustness
fn extract_source_type_from_error(error_msg: &str) -> Option<String> {
    // More robust extraction logic with multiple patterns
    if let Some(start) = error_msg.find("Value of type '") {
        if let Some(end) = error_msg[start + 14..].find("'") {
            return Some(error_msg[start + 14..start + 14 + end].to_string());
        }
    }
    
    // Alternative pattern
    if let Some(start) = error_msg.find("from '") {
        if let Some(end) = error_msg[start + 6..].find("'") {
            return Some(error_msg[start + 6..start + 6 + end].to_string());
        }
    }
    
    None
}

fn extract_target_type_from_error(error_msg: &str) -> Option<String> {
    // More robust extraction logic with multiple patterns
    if let Some(start) = error_msg.find("cannot be asserted as type '") {
        if let Some(end) = error_msg[start + 27..].find("'") {
            return Some(error_msg[start + 27..start + 27 + end].to_string());
        }
    }
    
    // Alternative pattern
    if let Some(start) = error_msg.find("to '") {
        if let Some(end) = error_msg[start + 4..].find("'") {
            return Some(error_msg[start + 4..start + 4 + end].to_string());
        }
    }
    
    None
}

// Registration function to integrate with the compiler
pub fn register_enhanced_interface_type_assertion_path_visualization() {
    trace!("Enhanced interface type assertion path visualization module registered");
    // This function is called during the compiler's initialization
    // to register this enhanced implementation for use throughout compilation
}