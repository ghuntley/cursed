//! # Simple Interface Path Finder
//!
//! This module provides a simple implementation of path finding algorithms for interface
//! inheritance relationships with robust error handling and consistent error propagation.
//! The implementation is designed to work with the existing codebase and avoid complex
//! integration issues.

use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info, instrument, span, Level};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_registry::InterfaceTypeRegistryAccess;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
use crate::error::Error;

/// Simple path finder extension for interface inheritance relationships
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find a path between two interfaces using breadth-first search
    ///
    /// This method finds the shortest path between the source and target interfaces
    /// with comprehensive error handling and proper error propagation.
    #[instrument(skip(self), level = "debug")]
    pub fn find_interface_path_simple(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error> {
        debug!("Finding interface path from {} to {}", source_interface, target_interface);
        
        // Check if interfaces exist
        let interfaces = self.get_all_interfaces();
        if !interfaces.contains(&source_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !interfaces.contains(&target_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Special case: if source and target are the same, return a single-element path
        if source_interface == target_interface {
            return Ok(vec![source_interface.to_string()]);
        }
        
        // Get the complete hierarchy
        let hierarchy = self.get_extension_hierarchy();
        
        // Breadth-first search to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path_map: HashMap<String, String> = HashMap::new(); // Maps interface to its predecessor
        
        // Start with the source interface
        queue.push_back(source_interface.to_string());
        visited.insert(source_interface.to_string());
        
        let mut found = false;
        
        while let Some(current) = queue.pop_front() {
            // Get direct extensions for the current interface
            if let Some(extensions) = hierarchy.get(&current) {
                // Check if the target interface is directly extended
                if extensions.contains(&target_interface.to_string()) {
                    path_map.insert(target_interface.to_string(), current.clone());
                    found = true;
                    break;
                }
                
                // Add unvisited extensions to the queue
                for ext in extensions {
                    if !visited.contains(ext) {
                        queue.push_back(ext.clone());
                        visited.insert(ext.clone());
                        path_map.insert(ext.clone(), current.clone());
                    }
                }
            }
        }
        
        if !found {
            return Err(Error::Compilation(format!(
                "No path found from '{}' to '{}'", 
                source_interface, target_interface
            )));
        }
        
        // Reconstruct the path from target to source
        let mut path = Vec::new();
        let mut current = target_interface.to_string();
        
        while current != source_interface {
            path.push(current.clone());
            current = path_map[&current].clone();
        }
        
        path.push(source_interface.to_string());
        path.reverse();
        
        debug!("Found path: {:?}", path);
        Ok(path)
    }
    
    /// Find multiple paths between two interfaces using breadth-first search
    /// 
    /// This method finds multiple paths between source and target interfaces
    /// with a limit on the maximum number of paths to find.
    #[instrument(skip(self), level = "debug")]
    pub fn find_alternative_paths_simple(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_paths: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding up to {} alternative paths from {} to {}", 
              max_paths, source_interface, target_interface);
        
        // Get all interfaces
        let interfaces = self.get_all_interfaces();
        if !interfaces.contains(&source_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !interfaces.contains(&target_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Special case: if source and target are the same, return a single-element path
        if source_interface == target_interface {
            return Ok(vec![vec![source_interface.to_string()]]);
        }
        
        // Get the complete hierarchy
        let hierarchy = self.get_extension_hierarchy();
        
        // Use modified BFS to find multiple paths
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with source
        queue.push_back(vec![source_interface.to_string()]);
        
        while let Some(path) = queue.pop_front() {
            let current = path.last().unwrap();
            
            // If we've reached the target, add this path
            if current == target_interface {
                paths.push(path);
                if paths.len() >= max_paths {
                    break;
                }
                continue;
            }
            
            // Mark as visited only if not yet in the current path
            // This allows for finding multiple paths
            visited.insert(current.clone());
            
            // Get extensions from the current node
            if let Some(extensions) = hierarchy.get(current) {
                for extension in extensions {
                    // Skip if already in the current path (avoid cycles)
                    if path.contains(extension) {
                        continue;
                    }
                    
                    // Create new path including this extension
                    let mut new_path = path.clone();
                    new_path.push(extension.clone());
                    queue.push_back(new_path);
                }
            }
        }
        
        if paths.is_empty() {
            debug!("No paths found from '{}' to '{}'.", source_interface, target_interface);
            return Err(Error::Compilation(format!(
                "No path found from '{}' to '{}'", 
                source_interface, target_interface
            )));
        }
        
        debug!("Found {} paths from '{}' to '{}'.", paths.len(), source_interface, target_interface);
        Ok(paths)
    }
    
    /// Check if one interface extends another, either directly or indirectly
    #[instrument(skip(self), level = "debug")]
    pub fn check_extension_relationship_simple(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source_interface, target_interface);
        
        // Special case: if source and target are the same, they're considered related
        if source_interface == target_interface {
            return Ok(true);
        }
        
        // Get the complete hierarchy
        let hierarchy = self.get_extension_hierarchy();
        
        // First check for direct extension
        if let Some(extensions) = hierarchy.get(source_interface) {
            if extensions.contains(&target_interface.to_string()) {
                debug!("Direct extension relationship found: {} extends {}", 
                      source_interface, target_interface);
                return Ok(true);
            }
        }
        
        // Then check for indirect extension using BFS
        match self.find_interface_path_simple(source_interface, target_interface) {
            Ok(_) => {
                debug!("Indirect extension relationship found: {} extends {} indirectly", 
                      source_interface, target_interface);
                Ok(true)
            }
            Err(_) => {
                debug!("No extension relationship found between {} and {}", 
                      source_interface, target_interface);
                Ok(false)
            }
        }
    }
    
    /// Helper method to get all interfaces in the registry
    #[instrument(skip(self), level = "debug")]
    fn get_all_interfaces(&self) -> HashSet<String> {
        let _span = span!(Level::DEBUG, "get_all_interfaces").entered();
        
        // Get the real interfaces from the registry
        let registry = self.interface_type_registry();
        let all_interfaces = registry.all_types();
        
        // Extract the type names from the registry
        let interfaces: HashSet<String> = all_interfaces
            .into_iter()
            .map(|(_, name)| name)
            .collect();
            
        debug!("Found {} interfaces in registry", interfaces.len());
        interfaces
    }
    
    /// Helper method to get the extension hierarchy from the actual registry
    #[instrument(skip(self), level = "debug")]
    fn get_extension_hierarchy(&self) -> HashMap<String, HashSet<String>> {
        let _span = span!(Level::DEBUG, "get_extension_hierarchy").entered();
        let mut hierarchy = HashMap::new();
        
        // Get the registry
        let registry = self.interface_type_registry();
        
        // Get all interfaces
        let interfaces = self.get_all_interfaces();
        
        // For each interface, find all its extensions
        for interface in &interfaces {
            let mut extensions = HashSet::new();
            
            // Get all other interfaces that extend this one
            for other in &interfaces {
                if interface == other {
                    continue; // Skip self
                }
                
                // Check if other extends interface directly
                if let Ok(true) = registry.check_interface_extends(other, interface) {
                    extensions.insert(other.clone());
                }
            }
            
            // Only add non-empty extension sets
            if !extensions.is_empty() {
                hierarchy.insert(interface.clone(), extensions);
            }
        }
        
        debug!("Built extension hierarchy with {} entries", hierarchy.len());
        hierarchy
    }
    
    /// Check for reversed inheritance relationship with detailed diagnostics
    ///
    /// This method detects if the inheritance relationship might be reversed,
    /// which is a common error in interface type assertions. It provides detailed
    /// diagnostic information to help developers fix the issue.
    #[instrument(skip(self), level = "debug")]
    pub fn detect_reversed_inheritance_simple(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<(bool, String), Error> {
        let _span = span!(Level::DEBUG, "detect_reversed_inheritance").entered();
        debug!("Checking for reversed inheritance between {} and {}", 
               source_interface, target_interface);
        
        // Get interfaces to verify they exist
        let interfaces = self.get_all_interfaces();
        if !interfaces.contains(&source_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !interfaces.contains(&target_interface.to_string()) {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Check if target actually extends source (reverse of what was attempted)
        match self.check_extension_relationship(target_interface, source_interface) {
            Ok(true) => {
                // Found reversed relationship - provide helpful message
                let message = format!(
                    "Reversed inheritance detected: '{}' extends '{}', not the other way around. \
                    You might need to swap the interfaces in your type assertion.",
                    target_interface, source_interface
                );
                
                info!("Detected reversed inheritance: {} extends {}", 
                      target_interface, source_interface);
                
                // Try to find a path to show the inheritance chain
                if let Ok(path) = self.find_interface_path_simple(target_interface, source_interface) {
                    let path_str = path.join(" -> ");
                    let detail_message = format!(
                        "{}\nThe actual inheritance path is: {}", 
                        message, path_str
                    );
                    return Ok((true, detail_message));
                }
                
                Ok((true, message))
            },
            Ok(false) => {
                // No reversed relationship
                debug!("No reversed inheritance detected between {} and {}", 
                      source_interface, target_interface);
                Ok((false, String::from("No reversed inheritance detected.")))
            },
            Err(e) => {
                // Error checking relationship
                debug!("Error checking reversed inheritance: {}", e);
                Err(e)
            }
        }
    }
    
    /// Detect reversed inheritance relationship with enhanced error messages
    #[instrument(skip(self), level = "debug")]
    pub fn detect_reversed_inheritance_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<(bool, String), Error> {
        let _span = span!(Level::DEBUG, "detect_reversed_inheritance_enhanced").entered();
        debug!("Checking for enhanced reversed inheritance between {} and {}", 
               source_interface, target_interface);
        
        // Use the simple method as the base implementation
        match self.detect_reversed_inheritance_simple(source_interface, target_interface) {
            Ok((is_reversed, base_message)) => {
                if is_reversed {
                    // Enhance the message with more detailed information
                    let enhanced_message = format!(
                        "{}\nNote: This is a common mistake. Interface '{}' extends '{}', not the reverse.\nTo fix: Use '({}: {})' instead of '({}: {})'",
                        base_message,
                        target_interface,
                        source_interface,
                        target_interface,
                        source_interface,
                        source_interface,
                        target_interface
                    );
                    Ok((true, enhanced_message))
                } else {
                    Ok((false, base_message))
                }
            },
            Err(e) => {
                debug!("Error in enhanced reversed inheritance detection: {}", e);
                Err(e)
            }
        }
    }
}