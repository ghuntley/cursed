//! # Enhanced Interface Registry Visualization (Reference Implementation)
//!
//! This is a stand-alone reference implementation for enhanced interface registry visualization.
//! It provides improved error messages and visualization for interface type assertions with
//! comprehensive error handling and consistent error propagation.
//!
//! Note: This is meant to be a reference implementation that can be integrated once the
//! existing codebase issues are resolved.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::error::Error;

/// Enhanced interface registry visualization with comprehensive error handling
pub trait EnhancedInterfaceRegistryVisualization {
    /// Get a map of all interface extension relationships for visualization
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error>;
    
    /// Get the list of interfaces that a given interface directly extends
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get the list of interfaces that directly extend a given interface
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all interfaces in the registry
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Checks if one interface extends another (directly or indirectly)
    fn does_extend(&self, interface: &str, extends: &str) -> Result<bool, Error>;
    
    /// Find paths between two interfaces
    fn find_interface_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_paths: usize,
    ) -> Result<Vec<Vec<String>>, Error>;
    
    /// Get a detailed view of the interface hierarchy with comprehensive error handling
    fn get_detailed_hierarchy(&self) -> Result<String, Error>;
    
    /// Generate an ASCII art visualization of the interface hierarchy
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error>;
    
    /// Generate a detailed error message for an interface type assertion failure
    fn generate_detailed_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error>;
    
    /// Generate a DOT graph representation of the interface hierarchy
    fn generate_interface_hierarchy_dot(&self) -> Result<String, Error>;
}

/// Extension trait for integrating with the code generator
pub trait EnhancedInterfaceVisualizationCodegenIntegration<Context, TypeAssertionNode, Result> {
    /// Enhanced type assertion with path visualization and improved error propagation
    fn compile_type_assertion_with_enhanced_visualization(
        &mut self,
        type_assertion: &TypeAssertionNode,
    ) -> Result;
    
    /// Access the interface registry for visualization
    fn enhanced_interface_registry(&self) -> &dyn EnhancedInterfaceRegistryVisualization;
}

/// Implementation helpers for the enhanced interface registry visualization
pub struct EnhancedVisualizationHelpers;

impl EnhancedVisualizationHelpers {
    /// Helper to build an ASCII tree representation for interface hierarchies
    pub fn build_ascii_tree(
        result: &mut String,
        interface: &str,
        hierarchy: &HashMap<String, HashSet<String>>,
        depth: usize,
    ) -> Result<(), Error> {
        // Add indentation based on depth
        let indent = "  ".repeat(depth);
        let prefix = if depth > 0 { "u2514u2500 " } else { "" };
        
        writeln!(result, "{}{}{}", indent, prefix, interface).map_err(|e| {
            Error::Compilation(format!("Failed to write to ASCII visualization: {}", e))
        })?;
        
        // Find all interfaces that extend this one
        let mut implementors = Vec::new();
        
        for (impl_interface, extensions) in hierarchy {
            if extensions.iter().any(|ext| ext == interface) {
                implementors.push(impl_interface.clone());
            }
        }
        
        // Sort implementors for consistent output
        implementors.sort();
        
        // Recurse for each implementor
        for implementor in implementors {
            Self::build_ascii_tree(result, &implementor, hierarchy, depth + 1)?;
        }
        
        Ok(())
    }
    
    /// Helper to find paths between interfaces using breadth-first search
    pub fn find_paths_bfs(
        source: &str,
        target: &str,
        get_extensions: impl Fn(&str) -> Result<Option<HashSet<String>>, Error>,
        max_paths: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with source
        queue.push_back(vec![source.to_string()]);
        visited.insert(source.to_string());
        
        while let Some(path) = queue.pop_front() {
            let current = path.last().unwrap();
            
            // If we've reached the target, add this path
            if current == target {
                paths.push(path);
                if paths.len() >= max_paths {
                    break;
                }
                continue;
            }
            
            // Get extensions from the current node
            if let Some(extensions) = get_extensions(current)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        let mut new_path = path.clone();
                        new_path.push(extension.clone());
                        queue.push_back(new_path);
                        visited.insert(extension);
                    }
                }
            }
        }
        
        Ok(paths)
    }
    
    /// Extract source type from error message
    pub fn extract_source_type_from_error(error_msg: &str) -> Option<String> {
        // Multiple patterns for robust extraction
        if let Some(start) = error_msg.find("Value of type '") {
            if let Some(end) = error_msg[start + 14..].find("'") {
                return Some(error_msg[start + 14..start + 14 + end].to_string());
            }
        }
        
        if let Some(start) = error_msg.find("from '") {
            if let Some(end) = error_msg[start + 6..].find("'") {
                return Some(error_msg[start + 6..start + 6 + end].to_string());
            }
        }
        
        None
    }
    
    /// Extract target type from error message
    pub fn extract_target_type_from_error(error_msg: &str) -> Option<String> {
        // Multiple patterns for robust extraction
        if let Some(start) = error_msg.find("cannot be asserted as type '") {
            if let Some(end) = error_msg[start + 27..].find("'") {
                return Some(error_msg[start + 27..start + 27 + end].to_string());
            }
        }
        
        if let Some(start) = error_msg.find("to '") {
            if let Some(end) = error_msg[start + 4..].find("'") {
                return Some(error_msg[start + 4..start + 4 + end].to_string());
            }
        }
        
        None
    }
}

/// Register the enhanced interface registry visualization reference implementation
pub fn register_enhanced_interface_registry_visualization_reference() {
    trace!("Enhanced interface registry visualization reference implementation registered");
}