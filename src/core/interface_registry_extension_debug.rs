//! # Interface Registry Extension Debug
//!
//! This module provides the implementation of the `Debug` trait for the `ThreadSafeInterfaceExtensionRegistry`
//! to support visualization and debugging capabilities.
//!
//! It's important to have proper Debug implementation for the interface registry to:
//! 1. Aid in troubleshooting during development
//! 2. Enable visualization of interface hierarchies
//! 3. Provide clear and concise error messages 
//! 4. Support testing and validation of the registry

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::RwLockReadGuard;
use tracing::{debug, instrument};

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::error::Error;

impl fmt::Debug for ThreadSafeInterfaceExtensionRegistry {
    #[instrument(level = "debug", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug!("Formatting ThreadSafeInterfaceExtensionRegistry for Debug");
        
        // Start with a simplified representation
        let mut debug_struct = f.debug_struct("ThreadSafeInterfaceExtensionRegistry");
        
        // Attempt to get all interfaces
        let all_interfaces = match self.get_all_interfaces() {
            Ok(interfaces) => interfaces,
            Err(_) => {
                // If we can't acquire the lock, show a placeholder
                debug_struct.field("registry", &"<lock acquisition failed>");
                return debug_struct.finish();
            }
        };
        
        // Format the registry content
        let mut registry_map = HashMap::new();
        
        for interface in all_interfaces {
            let extensions = match self.get_direct_extensions(&interface) {
                Ok(Some(exts)) => exts,
                _ => continue,
            };
            
            registry_map.insert(interface, extensions);
        }
        
        debug_struct.field("registry", &registry_map);
        debug_struct.finish()
    }
}

/// Helper module for visualizing the interface extension registry
pub mod visualization {
    use super::*;
    use std::fmt::Write;
    
    impl ThreadSafeInterfaceExtensionRegistry {
        /// Generates a visualization of the interface hierarchy as plain text
        #[instrument(level = "debug")]
        pub fn visualize_as_text(&self) -> Result<String, Error> {
            debug!("Generating text visualization of interface hierarchy");
            
            let mut result = String::from("Interface Extension Hierarchy:\n");
            
            // Get all interfaces
            let all_interfaces = self.get_all_interfaces().ok_or_else(|| Error::Internal("Failed to get interfaces".to_string()))?;
            
            // Sort interfaces for consistent output
            let mut sorted_interfaces: Vec<_> = all_interfaces.into_iter().collect();
            sorted_interfaces.sort();
            
            for interface in sorted_interfaces {
                writeln!(&mut result, "Interface: {}", interface).unwrap();
                
                if let Some(extensions) = self.get_direct_extensions(&interface)? {
                    let mut sorted_extensions: Vec<_> = extensions.into_iter().collect();
                    sorted_extensions.sort();
                    
                    for extension in sorted_extensions {
                        writeln!(&mut result, "  ↓ extends {}", extension).unwrap();
                    }
                }
                
                writeln!(&mut result).unwrap();
            }
            
            Ok(result)
        }
        
        /// Generates a visualization of the interface hierarchy as a DOT graph
        #[instrument(level = "debug")]
        pub fn visualize_as_dot(&self) -> Result<String, Error> {
            debug!("Generating DOT visualization of interface hierarchy");
            
            let mut result = String::from("digraph interface_hierarchy {\n");
            result.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
            
            // Get all interfaces
            let all_interfaces = self.get_all_interfaces().ok_or_else(|| Error::Internal("Failed to get interfaces".to_string()))?;
            
            // Add nodes
            for interface in &all_interfaces {
                writeln!(&mut result, "  \"{}\" [label=\"{}\"];", interface, interface).unwrap();
            }
            
            // Add edges
            for interface in &all_interfaces {
                if let Some(extensions) = self.get_direct_extensions(interface)? {
                    for extension in extensions {
                        writeln!(&mut result, "  \"{}\" -> \"{}\";", interface, extension).unwrap();
                    }
                }
            }
            
            result.push_str("}\n");
            
            Ok(result)
        }
        
        /// Find paths between two interfaces
        #[instrument(level = "debug")]
        pub fn find_paths(&self, source: &str, target: &str, max_paths: usize) -> Result<Vec<Vec<String>>, Error> {
            debug!("Finding paths from {} to {}", source, target);
            
            self.find_interface_paths(source, target, max_paths)
        }
    }
}