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
    
    /// Check if source interface extends target interface (alias for extends)
    fn check_interface_extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extends(source, target)
    }
    
    /// Find inheritance path from source to target
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all interfaces registered in the registry
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Check if an interface exists in the registry
    fn interface_exists(&self, name: &str) -> Result<bool, Error>;
    
    // Additional methods needed by interface type assertions and debugging
    
    /// Check if a concrete type implements an interface by type IDs
    fn type_implements_by_id(&self, concrete_type_id: u32, interface_type_id: u32) -> Result<bool, Error> {
        // Default implementation - checks for ID-based relationships
        // Override in concrete implementations for specific behavior
        Ok(false)
    }
    
    /// Get the type name for a given type ID
    fn get_type_name(&self, type_id: u64) -> Result<String, Error> {
        Err(Error::Compilation(format!("Type name lookup not implemented for ID {}", type_id)))
    }
    
    /// Look up a type ID by name
    fn lookup_type_id(&self, type_name: &str) -> Result<u64, Error> {
        Err(Error::Compilation(format!("Type ID lookup not implemented for name {}", type_name)))
    }
    
    /// Get the inheritance map (interface -> set of interfaces it extends)
    fn get_inheritance_map(&self) -> Option<HashMap<String, HashSet<String>>> {
        None
    }
    
    /// Get all registered types as (id, name) pairs
    fn all_types(&self) -> Vec<(u64, String)> {
        Vec::new()
    }
    
    /// Check if a given type ID represents an interface
    fn is_interface(&self, type_id: u32) -> Result<bool, Error> {
        // Default implementation returns false
        // Override in concrete implementations
        Ok(false)
    }
    
    /// Check if a type implements an interface (32-bit type IDs)
    fn type_implements_interface(&self, concrete_id: u32, interface_id: u32) -> bool {
        // Default implementation delegates to type_implements_by_id
        self.type_implements_by_id(concrete_id, interface_id).unwrap_or(false)
    }

    /// Get extension relationships as a map of interface IDs to sets of extended interface IDs
    fn get_extension_relationships(&self) -> Result<HashMap<u64, HashSet<u64>>, Error> {
        // Default implementation returns an empty map
        // Override in concrete implementations to provide actual extension relationships
        Ok(HashMap::new())
    }
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
    
    /// Generate a hash for a type name (FNV-1a algorithm)
    fn hash_name(&self, type_name: &str) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
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
        let mut parent: HashMap<String, String> = HashMap::new();
        
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
    
    // Implementation of new methods - these are basic implementations
    // Concrete registries should override as needed
    
    fn get_inheritance_map(&self) -> Option<HashMap<String, HashSet<String>>> {
        Some(self.direct_extensions.clone())
    }
    
    fn all_types(&self) -> Vec<(u64, String)> {
        // Create type IDs by hashing the interface names
        let mut result = Vec::new();
        for interface in &self.interfaces {
            let id = self.hash_name(interface);
            result.push((id, interface.clone()));
        }
        result
    }
    
    fn get_type_name(&self, type_id: u64) -> Result<String, Error> {
        // Search for a matching type ID
        for interface in &self.interfaces {
            if self.hash_name(interface) == type_id {
                return Ok(interface.clone());
            }
        }
        Err(Error::NotFound(format!("No interface found with type ID {}", type_id)))
    }
    
    fn lookup_type_id(&self, type_name: &str) -> Result<u64, Error> {
        if self.interfaces.contains(type_name) {
            Ok(self.hash_name(type_name))
        } else {
            Err(Error::NotFound(format!("Interface '{}' not found", type_name)))
        }
    }
    
    fn is_interface(&self, type_id: u32) -> Result<bool, Error> {
        // For BasicInterfaceRegistry, all registered types are interfaces
        let type_id_64 = type_id as u64;
        for interface in &self.interfaces {
            if self.hash_name(interface) == type_id_64 {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    fn type_implements_interface(&self, concrete_id: u32, interface_id: u32) -> bool {
        // For basic registry, check if the concrete type extends the interface
        if let (Ok(concrete_name), Ok(interface_name)) = 
            (self.get_type_name(concrete_id as u64), self.get_type_name(interface_id as u64)) {
            self.extends(&concrete_name, &interface_name).unwrap_or(false)
        } else {
            false
        }
    }

    fn get_extension_relationships(&self) -> Result<HashMap<u64, HashSet<u64>>, Error> {
        let mut extension_map = HashMap::new();
        
        // Convert string-based extension relationships to ID-based relationships
        for (source_name, target_names) in &self.direct_extensions {
            let source_id = self.hash_name(source_name);
            let mut target_ids = HashSet::new();
            
            for target_name in target_names {
                let target_id = self.hash_name(target_name);
                target_ids.insert(target_id);
            }
            
            extension_map.insert(source_id, target_ids);
        }
        
        Ok(extension_map)
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