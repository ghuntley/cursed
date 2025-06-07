//! Enhanced path finder for interface inheritance relationships.
//!
//! This module provides functionality for finding paths through the interface
//! inheritance graph, which is useful for checking whether one interface
//! extends another and visualizing inheritance relationships.

use crate::error::Error;
use crate::codegen::llvm::interface_type_registry::InterfaceTypeRegistry;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::debug;

/// Trait for checking extension relationships between interfaces
/// 
/// This trait defines methods for querying and visualizing the inheritance
/// relationships between interfaces in the type registry.
pub trait InterfaceTypeRegistryExtensionChecking {
    /// Checks if one interface extends another by ID
    ///
    /// # Arguments
    /// * `source_id` - ID of the potential subinterface
    /// * `target_id` - ID of the potential superinterface
    ///
    /// # Returns
    /// * `Result<bool, Error>` - Whether source extends target
    fn check_extension(&self, source_id: u64, target_id: u64) -> Result<bool, Error>;
    
    /// Finds a path between two interfaces in the inheritance graph
    ///
    /// # Arguments
    /// * `source_id` - ID of the starting interface
    /// * `target_id` - ID of the ending interface
    ///
    /// # Returns
    /// * `Result<Option<Vec<u64>>, Error>` - Path if one exists, None if no path
    fn find_path(&self, source_id: u64, target_id: u64) -> Result<Option<Vec<u64>>, Error>;
    
    /// Gets all interfaces that extend a given interface
    ///
    /// # Arguments
    /// * `interface_id` - ID of the interface to find extensions of
    ///
    /// # Returns
    /// * `Result<HashSet<u64>, Error>` - Set of interfaces that extend the given one
    fn get_extensions(&self, interface_id: u64) -> Result<HashSet<u64>, Error>;
    
    /// Visualizes the inheritance path between two interfaces
    ///
    /// # Arguments
    /// * `source_id` - ID of the starting interface
    /// * `target_id` - ID of the ending interface
    ///
    /// # Returns
    /// * `Result<String, Error>` - String representation of the path
    fn visualize_path(&self, source_id: u64, target_id: u64) -> Result<String, Error>;
}

/// Implementation of InterfaceTypeRegistryExtensionChecking for InterfaceTypeRegistry
impl<'ctx> InterfaceTypeRegistryExtensionChecking for InterfaceTypeRegistry<'ctx> {
    fn check_extension(&self, source_id: u64, target_id: u64) -> Result<bool, Error> {
        // Use the existing interface_extension_info method
        let extension_relationships = self.get_extension_relationships()?;
        
        // If source and target are the same, return true
        if source_id == target_id {
            return Ok(true);
        }
        
        // Check if there's a direct extension relationship
        if let Some(extended_interfaces) = extension_relationships.get(&source_id) {
            if extended_interfaces.contains(&target_id) {
                return Ok(true);
            }
        }
        
        // Check for indirect extension relationships (transitive closure)
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(source_id);
        
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            
            // Get interfaces directly extended by current
            if let Some(extended) = extension_relationships.get(&current) {
                for &ext_id in extended {
                    if ext_id == target_id {
                        return Ok(true);
                    }
                    queue.push_back(ext_id);
                }
            }
        }
        
        Ok(false)
    }
    
    fn find_path(&self, source_id: u64, target_id: u64) -> Result<Option<Vec<u64>>, Error> {
        // If source and target are the same, return a path with just that ID
        if source_id == target_id {
            return Ok(Some(vec![source_id]));
        }
        
        // Get the extension relationships
        let extension_relationships = self.get_extension_relationships()?;
        
        // BFS with path tracking
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessor = HashMap::new(); // For reconstructing the path
        
        queue.push_back(source_id);
        visited.insert(source_id);
        
        while let Some(current) = queue.pop_front() {
            // Get interfaces directly extended by current
            if let Some(extended) = extension_relationships.get(&current) {
                for &ext_id in extended {
                    if !visited.contains(&ext_id) {
                        predecessor.insert(ext_id, current);
                        visited.insert(ext_id);
                        queue.push_back(ext_id);
                        
                        if ext_id == target_id {
                            // Path found, reconstruct it
                            let mut path = Vec::new();
                            let mut current = target_id;
                            
                            path.push(current);
                            while let Some(&pred) = predecessor.get(&current) {
                                path.push(pred);
                                current = pred;
                            }
                            
                            path.reverse(); // Path should be from source to target
                            return Ok(Some(path));
                        }
                    }
                }
            }
        }
        
        // No path found
        Ok(None)
    }
    
    fn get_extensions(&self, interface_id: u64) -> Result<HashSet<u64>, Error> {
        // Delegate to the existing method
        self.get_extending_interfaces(interface_id)
    }
    
    fn visualize_path(&self, source_id: u64, target_id: u64) -> Result<String, Error> {
        // Find the path first
        let path_result = self.find_path(source_id, target_id)?;
        
        if let Some(path) = path_result {
            // Get names for the interfaces in the path
            let mut path_names = Vec::new();
            for &id in &path {
                if let Some(name) = self.get_type_name(id) {
                    path_names.push(name.clone());
                } else {
                    path_names.push(format!("Interface#{}", id));
                }
            }
            
            // Format the path into a string
            let path_str = path_names.join(" -> ");
            Ok(format!("Path: {}", path_str))
        } else {
            // No path exists
            let source_name = self.get_type_name(source_id)
                .map(String::clone)
                .unwrap_or_else(|| format!("Interface#{}", source_id));
                
            let target_name = self.get_type_name(target_id)
                .map(String::clone)
                .unwrap_or_else(|| format!("Interface#{}", target_id));
                
            Ok(format!("No path exists from {} to {}", source_name, target_name))
        }
    }
}

/// Represents a path between interfaces in the inheritance graph
#[derive(Debug, Clone)]
pub struct InterfaceInheritancePath {
    /// The sequence of interface IDs in the path
    pub path: Vec<u64>,
    
    /// The names of the interfaces in the path
    pub names: Vec<String>,
    
    /// Whether this is a direct inheritance relationship
    pub is_direct: bool,
}

impl InterfaceInheritancePath {
    /// Creates a new empty path
    pub fn new() -> Self {
        InterfaceInheritancePath {
            path: Vec::new(),
            names: Vec::new(),
            is_direct: false,
        }
    }
    
    /// Converts the path to a string representation
    pub fn to_string(&self) -> String {
        if self.path.is_empty() {
            return "No path".to_string();
        }
        
        let relation_type = if self.is_direct {
            "directly extends"
        } else {
            "extends"
        };
        
        let path_str = self.names.join(" -> ");
        format!("{} ({})", path_str, relation_type)
    }
}

/// Enhanced path finder for interface inheritance relationships trait
pub trait EnhancedInterfacePathFinder {
    /// Find path between interfaces
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Visualize the path between interfaces
    fn visualize_path(&self, source: &str, target: &str) -> Result<Option<String>, Error>;
    
    /// Clone the trait object into a Box
    fn box_clone(&self) -> Box<dyn EnhancedInterfacePathFinder>;
}

/// Concrete implementation of EnhancedInterfacePathFinder
#[derive(Clone)]
pub struct EnhancedInterfacePathFinderImpl<'ctx> {
    /// Reference to the interface type registry
    registry: &'ctx InterfaceTypeRegistry<'ctx>,
}

impl<'ctx> EnhancedInterfacePathFinderImpl<'ctx> {
    /// Creates a new enhanced path finder
    ///
    /// # Arguments
    /// * `registry` - Reference to the interface type registry
    pub fn new(registry: &'ctx InterfaceTypeRegistry<'ctx>) -> Self {
        EnhancedInterfacePathFinderImpl { registry }
    }
    
    /// Finds a path between two interfaces by name
    ///
    /// # Arguments
    /// * `source_name` - Name of the source interface
    /// * `target_name` - Name of the target interface
    ///
    /// # Returns
    /// * `Result<Option<InterfaceInheritancePath>, Error>` - Path if found
    pub fn find_path_by_name(
        &self,
        source_name: &str,
        target_name: &str
    ) -> Result<Option<InterfaceInheritancePath>, Error> {
        // Get all registered types
        let all_types = self.registry.all_types();
        
        // Find the type IDs for source and target interfaces
        let source_id = all_types.iter()
            .find(|(_, name)| name.as_str() == source_name)
            .map(|(id, _)| *id);
            
        let target_id = all_types.iter()
            .find(|(_, name)| name.as_str() == target_name)
            .map(|(id, _)| *id);
            
        match (source_id, target_id) {
            (Some(src), Some(tgt)) => {
                self.find_path(src, tgt)
            },
            _ => Ok(None),
        }
    }
    
    /// Finds a path between two interfaces by ID
    ///
    /// # Arguments
    /// * `source_id` - ID of the source interface
    /// * `target_id` - ID of the target interface
    ///
    /// # Returns
    /// * `Result<Option<InterfaceInheritancePath>, Error>` - Path if found
    pub fn find_path(
        &self,
        source_id: u64,
        target_id: u64
    ) -> Result<Option<InterfaceInheritancePath>, Error> {
        // Use the trait method to find the path
        let path_result = self.registry.find_path(source_id, target_id)?;
        
        if let Some(path) = path_result {
            let mut inheritance_path = InterfaceInheritancePath::new();
            inheritance_path.path = path.clone();
            
            // Get names for the interfaces in the path
            for &id in &path {
                if let Some(name) = self.registry.get_type_name(id) {
                    inheritance_path.names.push(name.clone());
                } else {
                    inheritance_path.names.push(format!("Interface#{}", id));
                }
            }
            
            // Check if it's a direct relationship
            inheritance_path.is_direct = path.len() == 2; // Source -> Target
            
            Ok(Some(inheritance_path))
        } else {
            Ok(None)
        }
    }
    
    /// Visualizes the inheritance path between two interfaces
    ///
    /// # Arguments
    /// * `source_id` - ID of the source interface
    /// * `target_id` - ID of the target interface
    ///
    /// # Returns
    /// * `Result<String, Error>` - String representation of the path
    pub fn visualize_path(
        &self,
        source_id: u64,
        target_id: u64
    ) -> Result<String, Error> {
        // Use the trait method to visualize the path
        self.registry.visualize_path(source_id, target_id)
    }
}

// Implement the trait for our concrete implementation
impl<'ctx> EnhancedInterfacePathFinder for EnhancedInterfacePathFinderImpl<'ctx> {
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        let path = self.find_path_by_name(source, target)?;
        Ok(path.map(|p| p.names))
    }
    
    fn visualize_path(&self, source: &str, target: &str) -> Result<Option<String>, Error> {
        let path = self.find_path_by_name(source, target)?;
        Ok(path.map(|p| p.to_string()))
    }
    
    fn box_clone(&self) -> Box<dyn EnhancedInterfacePathFinder> {
        Box::new(EnhancedInterfacePathFinderImpl {
            registry: self.registry,
        })
    }
}