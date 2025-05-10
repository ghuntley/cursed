//! # Enhanced Interface Registry Visualization Integration
//!
//! This module integrates the interface registry with enhanced visualization capabilities.
//! It provides additional utility functions that work with the existing visualization
//! system to add more comprehensive error handling and better diagnostics.
//!
//! ## Key Features
//!
//! 1. Integration with existing interface type assertion path visualization
//! 2. Enhanced error diagnostics for interface assertions
//! 3. More descriptive error messages with context
//! 4. Consistent error propagation using the `?` operator

use std::fmt::Write;
use tracing::{debug, trace, instrument};

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::error::Error;

/// Enhanced visualization functions for interface registries
#[derive(Debug)]
pub struct EnhancedVisualizationIntegration;

impl EnhancedVisualizationIntegration {
    /// Generate a detailed error message for an interface type assertion failure
    #[instrument(level = "debug")]
    pub fn generate_detailed_error_message(
        registry: &ThreadSafeInterfaceExtensionRegistry,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        debug!("Generating detailed error message for failed type assertion from {} to {}",
               source_interface, target_interface);
        
        let mut message = format!(
            "\nType Assertion Error at {}:\n\n",
            source_location
        );
        
        // Visual error message
        writeln!(message, "Cannot assert type '{}' as '{}'!", source_interface, target_interface).map_err(|e| {
            Error::Compilation(format!("Failed to write to error message: {}", e))
        })?;
        
        // Check if there is a possible inheritance relationship in the wrong direction
        let does_extend_reverse = registry.does_extend(target_interface, source_interface);
        
        if let Ok(true) = does_extend_reverse {
            writeln!(message, "\nError: The inheritance relationship is reversed!").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "'{}' extends '{}', not the other way around.",
                   target_interface, source_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "\nYou can either:").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "1. Reverse the type assertion (assert '{}' as '{}')",
                   target_interface, source_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "2. Change the interface inheritance hierarchy").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
        } else {
            writeln!(message, "\nNo inheritance relationship exists between these interfaces!").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            // Show what interfaces the source does extend
            if let Ok(Some(source_extensions)) = registry.get_direct_extensions(source_interface) {
                if !source_extensions.is_empty() {
                    writeln!(message, "\n'{}' directly extends:", source_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    for ext in source_extensions {
                        writeln!(message, "  - {}", ext).map_err(|e| {
                            Error::Compilation(format!("Failed to write to error message: {}", e))
                        })?;
                    }
                }
            }
            
            // Show what interfaces extend the target
            if let Ok(Some(target_implementors)) = registry.get_direct_implementors(target_interface) {
                if !target_implementors.is_empty() {
                    writeln!(message, "\nInterfaces that directly extend '{}':", target_interface).map_err(|e| {
                        Error::Compilation(format!("Failed to write to error message: {}", e))
                    })?;
                    
                    for impl_interface in target_implementors {
                        writeln!(message, "  - {}", impl_interface).map_err(|e| {
                            Error::Compilation(format!("Failed to write to error message: {}", e))
                        })?;
                    }
                }
            }
            
            writeln!(message, "\nTo fix this error, you need to:").map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "1. Modify '{}' to extend '{}', or", 
                   source_interface, target_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
            
            writeln!(message, "2. Use a different interface that extends '{}' for this assertion", 
                   target_interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to error message: {}", e))
            })?;
        }
        
        Ok(message)
    }
    
    /// Generate a comprehensive ASCII art visualization of the interface hierarchy
    #[instrument(level = "debug")]
    pub fn generate_ascii_hierarchy(
        registry: &ThreadSafeInterfaceExtensionRegistry,
    ) -> Result<String, Error> {
        debug!("Generating ASCII hierarchy visualization");
        
        let hierarchy = registry.get_extension_hierarchy()?;
        
        let mut result = String::new();
        writeln!(result, "Interface Hierarchy Tree:").map_err(|e| {
            Error::Compilation(format!("Failed to write to ASCII visualization: {}", e))
        })?;
        
        // Find root interfaces (those that don't extend any other interface)
        let mut all_interfaces = std::collections::HashSet::new();
        let mut extended_interfaces = std::collections::HashSet::new();
        
        for (interface, extensions) in &hierarchy {
            all_interfaces.insert(interface.clone());
            for extension in extensions {
                all_interfaces.insert(extension.clone());
                extended_interfaces.insert(extension.clone());
            }
        }
        
        let mut roots: Vec<_> = all_interfaces.difference(&extended_interfaces).cloned().collect();
        roots.sort(); // Sort for consistent output
        
        // Helper function for recursive tree building
        fn build_tree(
            result: &mut String,
            interface: &str,
            hierarchy: &std::collections::HashMap<String, std::collections::HashSet<String>>,
            depth: usize,
        ) -> Result<(), Error> {
            // Add indentation based on depth
            let indent = "  ".repeat(depth);
            let prefix = if depth > 0 { "└─ " } else { "" };
            
            writeln!(result, "{}{}{}", indent, prefix, interface).map_err(|e| {
                Error::Compilation(format!("Failed to write to ASCII visualization: {}", e))
            })?;
            
            // Find all interfaces that extend this one
            let mut implementors = Vec::new();
            
            for (impl_interface, extensions) in hierarchy {
                if extensions.contains(interface) {
                    implementors.push(impl_interface.clone());
                }
            }
            
            // Sort implementors for consistent output
            implementors.sort();
            
            // Recurse for each implementor
            for implementor in implementors {
                build_tree(result, &implementor, hierarchy, depth + 1)?;
            }
            
            Ok(())
        }
        
        // Build the tree starting from each root
        for (i, root) in roots.iter().enumerate() {
            if i > 0 {
                writeln!(result).map_err(|e| {
                    Error::Compilation(format!("Failed to write to ASCII visualization: {}", e))
                })?;
            }
            
            build_tree(&mut result, root, &hierarchy, 0)?;
        }
        
        Ok(result)
    }
}

/// Register the enhanced visualization integration module
pub fn register_enhanced_visualization_integration() {
    trace!("Enhanced visualization integration module registered");
}