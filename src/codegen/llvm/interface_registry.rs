//! # Interface Registry for LLVM Code Generator
//!
//! This module provides an interface registry for the LLVM code generator,
//! allowing it to track and resolve interface relationships during code generation.

use crate::error::Error;
use std::collections::{HashMap, HashSet};

/// Interface Type Registry trait for LLVM code generator
pub trait InterfaceTypeRegistry {
    // This needs to match the trait name used in benchmarks and other modules
    /// Register an interface type
    fn register_interface(&mut self, name: &str) -> Result<(), Error>;
    
    /// Register an extension relationship between interfaces
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error>;
    
    /// Check if source extends target (directly or indirectly)
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Find inheritance path from source to target
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all interfaces registered in the registry
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Check if an interface exists in the registry
    fn interface_exists(&self, name: &str) -> Result<bool, Error>;
}

/// Alias for the InterfaceTypeRegistry trait (for backward compatibility)
pub type InterfaceRegistry = dyn InterfaceTypeRegistry + Send + Sync;

/// Basic implementation of InterfaceTypeRegistry
pub struct BasicInterfaceRegistry {
    /// Direct extensions map (interface -> set of interfaces it directly extends)
    direct_extensions: HashMap<String, HashSet<String>>,
    
    /// All registered interfaces
    interfaces: HashSet<String>,
}

impl BasicInterfaceRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            direct_extensions: HashMap::new(),
            interfaces: HashSet::new(),
        }
    }
}

impl InterfaceTypeRegistry for BasicInterfaceRegistry {
    fn register_interface(&mut self, name: &str) -> Result<(), Error> {
        self.interfaces.insert(name.to_string());
        Ok(())
    }
    
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        // Ensure both interfaces exist
        if !self.interfaces.contains(source) {
            return Err(Error::NotFound(format!("Source interface '{}' not found", source)));
        }
        
        if !self.interfaces.contains(target) {
            return Err(Error::NotFound(format!("Target interface '{}' not found", target)));
        }
        
        // Add to direct extensions
        self.direct_extensions.entry(source.to_string())
            .or_insert_with(HashSet::new)
            .insert(target.to_string());
        
        Ok(())
    }
    
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        // If source and target are the same, return true
        if source == target {
            return Ok(true);
        }
        
        // Check if there's a direct extension
        if let Some(extensions) = self.direct_extensions.get(source) {
            if extensions.contains(target) {
                return Ok(true);
            }
            
            // Recursively check extensions
            for ext in extensions {
                if self.extends(ext, target)? {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        // If source and target are the same, return a trivial path
        if source == target {
            return Ok(Some(vec![source.to_string()]));
        }
        
        // If source doesn't extend target, return None
        if !self.extends(source, target)? {
            return Ok(None);
        }
        
        // Use BFS to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut parent = HashMap::new();
        
        queue.push_back(source.to_string());
        visited.insert(source.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == target {
                // Found the target, reconstruct the path
                let mut path = Vec::new();
                let mut node = current;
                
                // Work backwards from target to source
                while node != source {
                    path.push(node.clone());
                    node = parent.get(&node).unwrap().clone();
                }
                
                // Add the source
                path.push(source.to_string());
                
                // Reverse to get the path from source to target
                path.reverse();
                
                return Ok(Some(path));
            }
            
            // Explore all direct extensions
            if let Some(extensions) = self.direct_extensions.get(&current) {
                for ext in extensions {
                    if !visited.contains(ext) {
                        visited.insert(ext.clone());
                        queue.push_back(ext.clone());
                        parent.insert(ext.clone(), current.clone());
                    }
                }
            }
        }
        
        // No path found (should not happen if extends returned true)
        Ok(None)
    }
    
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        Ok(self.interfaces.clone())
    }
    
    fn interface_exists(&self, name: &str) -> Result<bool, Error> {
        Ok(self.interfaces.contains(name))
    }
}

/// Helper module for documentation
mod docs {
    //! Documentation for the interface registry module
    
    /// Why tests are important for InterfaceTypeRegistry:
    /// 1. Interface type assertions are critical for language safety
    /// 2. They ensure diamond inheritance is correctly handled
    /// 3. They verify that circular inheritance is detected
    /// 4. They confirm interface relationships are properly tracked
    /// 5. They ensure type safety for interface conversions
    #[cfg(test)]
    fn test_requirements() {}
}