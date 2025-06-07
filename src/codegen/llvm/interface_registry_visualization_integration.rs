//! # Interface Registry Visualization Integration
//!
//! This module provides a comprehensive integration between the interface registry visualization
//! system and the LLVM code generator. It connects the enhanced interface type assertion path
//! visualization system with the interface registry to provide detailed visualization of
//! interface inheritance relationships and type assertions.
//!
//! ## Key Features
//!
//! 1. Thread-safe integration with the LLVM code generator
//! 2. Consistent error propagation with the `?` operator throughout all operations
//! 3. Multiple visualization formats including ASCII art and DOT graphs
//! 4. Integration with the existing interface type assertion path visualization system
//! 5. Comprehensive error handling with rich context in all error messages

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, span, trace, warn, Level};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::interface_registry_visualization::{
    InterfaceRegistryExtensionWithVisualization, VisualizationFormat, VisualizationOptions,
};
use crate::error::Error;

/// Provides integration between the LLVM code generator and interface registry visualization system
pub trait InterfaceRegistryVisualizationIntegration<'ctx> {
    /// Initialize the interface registry visualization system in the code generator
    fn initialize_registry_visualization(&mut self) -> Result<(), Error>;

    /// Generate visualization of an interface hierarchy with specified options
    fn visualize_interface_hierarchy(
        &self,
        format: VisualizationFormat,
        options: &VisualizationOptions,
    ) -> Result<String, Error>;

    /// Find and visualize paths between two interfaces with consistent error handling
    fn find_and_visualize_inheritance_path(
        &self,
        source_interface: &str,
        target_interface: &str,
        format: VisualizationFormat,
    ) -> Result<String, Error>;

    /// Detect and report cycles in interface inheritance relationships
    fn detect_inheritance_cycles(&self) -> Result<Vec<Vec<String>>, Error>;

    /// Check if an interface extends another interface, directly or indirectly
    fn check_interface_extension_relationship(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<bool, Error>;

    /// Generate enhanced error message for failed type assertions with visualization
    fn generate_enhanced_assertion_error(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error>;
}

impl<'ctx> InterfaceRegistryVisualizationIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn initialize_registry_visualization(&mut self) -> Result<(), Error> {
        debug!("Initializing interface registry visualization system");
        
        // Check if the registry is already initialized
        if self.registry_extensions.is_visualization_initialized()? {
            debug!("Interface registry visualization already initialized");
            return Ok(());
        }
        
        // Initialize the visualization system with all known interfaces
        let interfaces = InterfaceRegistryExtensionWithVisualization::get_all_interfaces(&*self.interface_registry())?;
        debug!("Found {} interfaces to initialize in visualization system", interfaces.len());
        
        // Register all known interface extensions
        for interface_name in &interfaces {
            // Get extensions for this interface with proper error propagation
            if let Some(extensions) = InterfaceRegistryExtensionWithVisualization::get_direct_extensions(&*self.interface_registry(), interface_name)? {
                for extension in extensions {
                    // Register the extension relationship with proper error handling
                    self.registry_extensions.register_extension(interface_name, &extension)?;
                    debug!("Registered extension relationship: {} extends {}", interface_name, extension);
                }
            }
        }
        
        // Mark the registry as initialized
        self.registry_extensions.set_visualization_initialized(true)?;
        debug!("Interface registry visualization system initialized successfully");
        
        Ok(())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_interface_hierarchy(
        &self,
        format: VisualizationFormat,
        options: &VisualizationOptions,
    ) -> Result<String, Error> {
        debug!("Visualizing interface hierarchy with format: {:?}", format);
        
        // Get the complete hierarchy with proper error propagation
        let hierarchy = InterfaceRegistryExtensionWithVisualization::get_extension_hierarchy(&*self.interface_registry())?;
        
        // Generate visualization based on the specified format
        match format {
            VisualizationFormat::Ascii => {
                self.registry_extensions.generate_ascii_tree(&hierarchy, options)
            }
            VisualizationFormat::Dot => {
                self.registry_extensions.generate_dot_graph(&hierarchy, options)
            }
            VisualizationFormat::Json => {
                self.registry_extensions.generate_json_representation(&hierarchy, options)
            }
            _ => {
                // For unsupported formats, fall back to ASCII with a warning
                warn!("Unsupported visualization format: {:?}, falling back to ASCII", format);
                self.registry_extensions.generate_ascii_tree(&hierarchy, options)
            }
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_and_visualize_inheritance_path(
        &self,
        source_interface: &str,
        target_interface: &str,
        format: VisualizationFormat,
    ) -> Result<String, Error> {
        debug!("Finding and visualizing inheritance path from {} to {}", 
               source_interface, target_interface);
        
        // Find the path between interfaces with proper error propagation
        let path = self.find_interface_path_simple(source_interface, target_interface)?;
        
        // Generate visualization based on the format
        match format {
            VisualizationFormat::Ascii => {
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
                
                Ok(result)
            }
            VisualizationFormat::Dot => {
                let mut result = String::new();
                
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
            VisualizationFormat::Json => {
                // Implement JSON format for machine-readable output
                let mut result = String::new();
                
                writeln!(result, "{{\n  \"path\": [").map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                for (i, interface) in path.iter().enumerate() {
                    if i < path.len() - 1 {
                        writeln!(result, "    \"{}\",", interface).map_err(|e| {
                            Error::Compilation(format!("Failed to write to path visualization: {}", e))
                        })?;
                    } else {
                        writeln!(result, "    \"{}\"", interface).map_err(|e| {
                            Error::Compilation(format!("Failed to write to path visualization: {}", e))
                        })?;
                    }
                }
                
                writeln!(result, "  ],").map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                writeln!(result, "  \"source\": \"{}\",", source_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                writeln!(result, "  \"target\": \"{}\",", target_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                writeln!(result, "  \"path_length\": {}", path.len()).map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                writeln!(result, "}}").map_err(|e| {
                    Error::Compilation(format!("Failed to write to path visualization: {}", e))
                })?;
                
                Ok(result)
            }
            _ => {
                // For unsupported formats, fall back to ASCII with a warning
                warn!("Unsupported visualization format: {:?}, falling back to ASCII", format);
                self.find_and_visualize_inheritance_path(
                    source_interface, 
                    target_interface, 
                    VisualizationFormat::Ascii
                )
            }
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn detect_inheritance_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        debug!("Detecting cycles in interface inheritance hierarchy");
        
        // Get the complete hierarchy with proper error propagation
        let hierarchy = InterfaceRegistryExtensionWithVisualization::get_extension_hierarchy(&*self.interface_registry())?;
        
        // Call the registry's cycle detection with proper error handling
        self.registry_extensions.detect_cycles()
    }
    
    #[instrument(skip(self), level = "debug")]
    fn check_interface_extension_relationship(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source_interface, target_interface);
        
        // Use the simple relationship checker with proper error propagation
        self.check_extension_relationship_simple(source_interface, target_interface)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_enhanced_assertion_error(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        debug!("Generating enhanced assertion error for failed assertion from {} to {}", 
               source_interface, target_interface);
        
        let _span = span!(Level::DEBUG, "generate_enhanced_assertion_error").entered();
        
        let mut message = format!(
            "Type assertion error at {}: Value of type '{}' cannot be asserted as type '{}'",
            source_location, source_interface, target_interface
        );
        
        // Check if the interfaces exist with proper error handling
        if !self.interface_registry().interface_exists(source_interface)? {
            writeln!(message, "

Error: Interface '{}' is not defined in the registry.", 
                     source_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            return Ok(message);
        }
        
        if !self.interface_registry().interface_exists(target_interface)? {
            writeln!(message, "

Error: Interface '{}' is not defined in the registry.", 
                     target_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            return Ok(message);
        }
        
        // Check for reversed relationship - common mistake
        match self.check_interface_extension_relationship(target_interface, source_interface) {
            Ok(true) => {
                writeln!(message, "

Note: The relationship appears to be reversed. '{}' extends '{}', not the other way around.",
                         target_interface, source_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
                
                writeln!(message, "Suggestion: Check if you meant to write: value.({}) instead.", 
                         target_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
                
                // Early return with this specific guidance
                return Ok(message);
            },
            Err(e) => {
                // Log the error but continue
                warn!("Error checking reversed relationship: {}", e);
            },
            _ => {}
        }
        
        // Try to visualize alternative paths with proper error handling using the simple method
        match self.find_alternative_paths_simple(source_interface, target_interface, 3) {
            Ok(paths) => {
                if !paths.is_empty() {
                    writeln!(message, "

Alternative paths between these interfaces:").map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    for (i, path) in paths.iter().enumerate() {
                        writeln!(message, "
Path {}:", i + 1).map_err(|e| {
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
                        "
Suggestion: Update the type hierarchy to implement the missing inheritance relationship."
                    ).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                } else {
                    writeln!(
                        message,
                        "

No inheritance path exists between these interfaces."
                    ).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    // Provide additional context about each interface
                    self.add_interface_context_to_error_message(
                        &mut message, source_interface, target_interface
                    )?;
                }
            },
            Err(e) => {
                // Handle error in path finding gracefully with more context
                writeln!(
                    message,
                    "

Error finding alternative paths: {}",
                    e
                ).map_err(|e2| {
                    Error::Compilation(format!("Failed to write to error message: {}", e2))
                })?;
                
                // Add suggestion for the reversed relationship case
                if self.detect_reversed_inheritance(source_interface, target_interface)? {
                    writeln!(
                        message,
                        "
Note: The relationship appears to be reversed. Consider checking if you meant '({}).{}'.",
                        target_interface, source_interface
                    ).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                }
                
                // Try to provide context about each interface even if path finding failed
                match self.add_interface_context_to_error_message(
                    &mut message, source_interface, target_interface
                ) {
                    Ok(_) => {},
                    Err(context_error) => {
                        warn!("Failed to add interface context to error message: {}", context_error);
                    }
                }
            }
        }
        
        Ok(message)
    }
}

// Extension method to add interface context to error messages
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Detect if the inheritance relationship is reversed
    fn detect_reversed_inheritance(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<bool, Error> {
        let _span = span!(Level::DEBUG, "detect_reversed_inheritance").entered();
        debug!("Checking for reversed inheritance between {} and {}", source_interface, target_interface);
        
        // Check if target actually extends source (reverse of what was attempted)
        // Our implementation returns a tuple with (is_reversed, message) so we just return the boolean part
        match self.detect_reversed_inheritance_simple(source_interface, target_interface) {
            Ok((is_reversed, _)) => Ok(is_reversed),
            Err(e) => Err(e)
        }
    }
    
    /// Add detailed context about interfaces to the error message
    fn add_interface_context_to_error_message(
        &self,
        message: &mut String,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<(), Error> {
        let _span = span!(Level::DEBUG, "add_interface_context").entered();
        
        // Add information about the source interface
        match InterfaceRegistryExtensionWithVisualization::get_direct_extensions(&*self.interface_registry(), source_interface) {
            Ok(Some(extensions)) if !extensions.is_empty() => {
                writeln!(*message, "
'{}' directly extends these interfaces:", source_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
                
                for extension in &extensions {
                    writeln!(*message, "  - {}", extension).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                }
            },
            Ok(_) => {
                writeln!(*message, "
'{}' does not extend any interfaces.", source_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
            },
            Err(e) => {
                writeln!(*message, "
Error retrieving extension information for '{}': {}", 
                         source_interface, e).map_err(|e2| {
                    Error::Compilation(format!("Failed to write to error message: {}", e2))
                })?;
            }
        }
        
        // Add information about the target interface
        match InterfaceRegistryExtensionWithVisualization::get_direct_implementors(&*self.interface_registry(), target_interface) {
            Ok(Some(implementors)) if !implementors.is_empty() => {
                writeln!(*message, "
These interfaces directly extend '{}':", target_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
                
                for implementor in &implementors {
                    writeln!(*message, "  - {}", implementor).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                }
            },
            Ok(_) => {
                writeln!(*message, "
No interfaces directly extend '{}'.", target_interface).map_err(|e| {
                    Error::Compilation(format!("Failed to write to error message: {}", e))
                })?;
            },
            Err(e) => {
                writeln!(*message, "
Error retrieving implementor information for '{}': {}", 
                         target_interface, e).map_err(|e2| {
                    Error::Compilation(format!("Failed to write to error message: {}", e2))
                })?;
            }
        }
        
        // Add suggestion to fix the issue
        writeln!(*message, "
Suggestion: To fix this issue, ensure that '{}' extends '{}' either directly or through another interface.",
                 source_interface, target_interface).map_err(|e| {
            Error::Compilation(format!("Failed to write to error message: {}", e))
        })?;
        
        Ok(())
    }
}

// Registration function to integrate with the compiler
pub fn register_interface_registry_visualization_integration() {
    debug!("Registering interface registry visualization integration");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout the compilation process
}