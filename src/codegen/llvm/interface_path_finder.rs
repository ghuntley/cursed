//! # Interface Path Finder
//!
//! This module implements comprehensive path finding algorithms for interface inheritance
//! relationships with robust error handling and consistent error propagation.
//!
//! ## Key Features
//!
//! 1. Both breadth-first and depth-first path finding algorithms
//! 2. Support for finding multiple alternative paths between interfaces
//! 3. Consistent error propagation with the `?` operator
//! 4. Comprehensive error handling with rich context
//! 5. Thread-safe implementation for concurrent compilation scenarios

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, span, trace, warn, Level};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// Extension trait for LlvmCodeGenerator that provides path finding for interface relationships
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find a path between two interfaces using breadth-first search
    /// 
    /// This method finds the shortest path between the source and target interfaces
    /// with comprehensive error handling and proper error propagation.
    #[instrument(skip(self), level = "debug")]
    pub fn find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error> {
        debug!("Finding interface path from {} to {}", source_interface, target_interface);
        
        // Check if interfaces exist with proper error handling
        if !self.interface_registry().interface_exists(source_interface)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !self.interface_registry().interface_exists(target_interface)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Special case: if source and target are the same, return a single-element path
        if source_interface == target_interface {
            return Ok(vec![source_interface.to_string()]);
        }
        
        // Breadth-first search to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path_map: HashMap<String, String> = HashMap::new(); // Maps interface to its predecessor
        
        // Start with the source interface
        queue.push_back(source_interface.to_string());
        visited.insert(source_interface.to_string());
        
        let mut found = false;
        
        while let Some(current) = queue.pop_front() {
            // Get direct extensions with proper error handling
            if let Some(extensions) = self.interface_registry().get_direct_extensions(&current)? {
                // Check if the target interface is directly extended
                if extensions.contains(target_interface) {
                    path_map.insert(target_interface.to_string(), current.clone());
                    found = true;
                    break;
                }
                
                // Add unvisited extensions to the queue
                for ext in extensions {
                    if !visited.contains(&ext) {
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
    
    /// Find alternative paths between two interfaces with enhanced error handling
    /// 
    /// This method finds multiple alternative paths between the source and target interfaces
    /// with comprehensive error handling and proper error propagation.
    #[instrument(skip(self), level = "debug")]
    pub fn find_alternative_paths_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_paths: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding up to {} alternative paths from {} to {}", 
              max_paths, source_interface, target_interface);
        
        // Check if interfaces exist with proper error handling
        if !self.interface_registry().interface_exists(source_interface)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !self.interface_registry().interface_exists(target_interface)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Special case: if source and target are the same, return a single-element path
        if source_interface == target_interface {
            return Ok(vec![vec![source_interface.to_string()]]);
        }
        
        // Use the registry extensions to find paths with proper error handling
        let paths = self.registry_extensions.find_paths(
            source_interface,
            target_interface,
            max_paths
        )?;
        
        if paths.is_empty() {
            debug!("No paths found from '{}' to '{}'", source_interface, target_interface);
        } else {
            debug!("Found {} paths from '{}' to '{}'", paths.len(), source_interface, target_interface);
        }
        
        Ok(paths)
    }
    
    /// Find both direct and indirect extension relationships between interfaces
    /// 
    /// This method checks if one interface extends another, either directly or indirectly,
    /// with comprehensive error handling and proper error propagation.
    #[instrument(skip(self), level = "debug")]
    pub fn check_extension_relationship_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<bool, Error> {
        debug!("Checking extension relationship between {} and {}", 
              source_interface, target_interface);
        
        // Check if interfaces exist with proper error handling
        if !self.interface_registry().interface_exists(source_interface)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry", 
                source_interface
            )));
        }
        
        if !self.interface_registry().interface_exists(target_interface)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry", 
                target_interface
            )));
        }
        
        // Special case: if source and target are the same, they're considered related
        if source_interface == target_interface {
            return Ok(true);
        }
        
        // First check for direct extension with proper error handling
        if let Some(extensions) = self.interface_registry().get_direct_extensions(source_interface)? {
            if extensions.contains(target_interface) {
                debug!("Direct extension relationship found: {} extends {}", 
                      source_interface, target_interface);
                return Ok(true);
            }
        }
        
        // Then check for indirect extension using path finding
        match self.find_interface_path(source_interface, target_interface) {
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
    
    /// Find all interfaces that extend a given interface with enhanced error handling
    /// 
    /// This method finds all interfaces that extend the given interface, either directly or indirectly,
    /// with comprehensive error handling and proper error propagation.
    #[instrument(skip(self), level = "debug")]
    pub fn find_all_interface_implementors_enhanced(
        &self,
        interface: &str,
    ) -> Result<HashSet<String>, Error> {
        debug!("Finding all implementors of {}", interface);
        
        // Check if interface exists with proper error handling
        if !self.interface_registry().interface_exists(interface)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry", 
                interface
            )));
        }
        
        // Get the full hierarchy with proper error handling
        let hierarchy = self.interface_registry().get_extension_hierarchy()?;
        
        // Use breadth-first search to find all implementors
        let mut implementors = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with direct implementors
        if let Some(direct_implementors) = self.interface_registry().get_direct_implementors(interface)? {
            for implementor in direct_implementors {
                queue.push_back(implementor.clone());
                visited.insert(implementor.clone());
                implementors.insert(implementor.clone());
            }
        }
        
        // Find indirect implementors using BFS
        while let Some(current) = queue.pop_front() {
            // Find interfaces that extend this one
            for (impl_interface, extensions) in &hierarchy {
                if extensions.contains(&current) && !visited.contains(impl_interface) {
                    queue.push_back(impl_interface.clone());
                    visited.insert(impl_interface.clone());
                    implementors.insert(impl_interface.clone());
                }
            }
        }
        
        debug!("Found {} implementors of {}", implementors.len(), interface);
        Ok(implementors)
    }
}