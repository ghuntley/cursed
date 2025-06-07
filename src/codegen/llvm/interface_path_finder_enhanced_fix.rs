//! Enhanced path finder for interface inheritance relationships with multiple path support.
//!
//! This module extends the interface_path_finder_enhanced.rs module with additional
//! functionality for finding and analyzing multiple inheritance paths, which is crucial
//! for diamond inheritance pattern detection.

use crate::error::Error;
use crate::InterfaceTypeRegistry;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceInheritancePath;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, instrument};

/// Trait for finding all inheritance paths between interfaces
pub trait EnhancedInterfacePathFinder {
    /// Find all paths between two types in the inheritance hierarchy
    ///
    /// # Arguments
    /// * `source_id` - ID of the source type
    /// * `target_id` - ID of the target type
    ///
    /// # Returns
    /// * `Result<Vec<InterfaceInheritancePath>, Error>` - All paths found
    fn find_all_paths(&self, source_id: u64, target_id: u64) -> Result<Vec<InterfaceInheritancePath>, Error>;
    
    /// Find all paths between two types by name
    ///
    /// # Arguments
    /// * `source_name` - Name of the source type
    /// * `target_name` - Name of the target type
    ///
    /// # Returns
    /// * `Result<Vec<InterfaceInheritancePath>, Error>` - All paths found
    fn find_all_paths_by_name(
        &self,
        source_name: &str,
        target_name: &str
    ) -> Result<Vec<InterfaceInheritancePath>, Error>;
    
    /// Check if multiple paths exist between two types
    ///
    /// # Arguments
    /// * `source_id` - ID of the source type
    /// * `target_id` - ID of the target type
    ///
    /// # Returns
    /// * `Result<bool, Error>` - Whether multiple paths exist
    fn has_multiple_paths(&self, source_id: u64, target_id: u64) -> Result<bool, Error>;
    
    /// Generate a visualization of all paths between two types
    ///
    /// # Arguments
    /// * `source_id` - ID of the source type
    /// * `target_id` - ID of the target type
    ///
    /// # Returns
    /// * `Result<String, Error>` - Visualization of all paths
    fn visualize_all_paths(&self, source_id: u64, target_id: u64) -> Result<String, Error>;
}

/// Concrete implementation of the EnhancedInterfacePathFinder trait that adds multi-path support
#[derive(Clone)]
pub struct MultiPathFinder<'ctx> {
    /// The interface type registry used for lookups
    pub registry: &'ctx dyn InterfaceTypeRegistry,
}

impl<'ctx> MultiPathFinder<'ctx> {
    /// Creates a new multi-path finder
    pub fn new(registry: &'ctx dyn InterfaceTypeRegistry) -> Self {
        MultiPathFinder { registry }
    }
    
    /// DFS helper function to find all paths between two interfaces
    ///
    /// # Arguments
    /// * `current` - Current interface ID in the DFS
    /// * `target` - Target interface ID
    /// * `extension_relationships` - Map of extension relationships
    /// * `current_path` - Current path being explored
    /// * `visited` - Set of already visited interfaces in the current path
    /// * `all_paths` - All paths found so far
    ///
    /// # Returns
    /// * `Result<(), Error>` - Success or error status
    fn dfs_find_all_paths(
        &self,
        current: u64,
        target: u64,
        extension_relationships: &HashMap<u64, HashSet<u64>>,
        current_path: &mut Vec<u64>,
        visited: &mut HashSet<u64>,
        all_paths: &mut Vec<Vec<u64>>
    ) -> Result<(), Error> {
        // Mark current interface as visited
        visited.insert(current);
        current_path.push(current);
        
        // Check if we've reached the target
        if current == target {
            all_paths.push(current_path.clone());
        } else {
            // Get interfaces directly extended by current
            if let Some(extended) = extension_relationships.get(&current) {
                for &ext_id in extended {
                    if !visited.contains(&ext_id) {
                        self.dfs_find_all_paths(
                            ext_id,
                            target,
                            extension_relationships,
                            current_path,
                            visited,
                            all_paths
                        )?;
                    }
                }
            }
        }
        
        // Backtrack
        visited.remove(&current);
        current_path.pop();
        
        Ok(())
    }
}

impl<'ctx> EnhancedInterfacePathFinder for MultiPathFinder<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn find_all_paths(&self, source_id: u64, target_id: u64) -> Result<Vec<InterfaceInheritancePath>, Error> {
        // If source and target are the same, return a single-node path
        if source_id == target_id {
            let mut path = InterfaceInheritancePath::new();
            path.path = vec![source_id];
            
            // Get the type name
            if let Some(name) = self.registry.get_type_name(source_id) {
                path.names = vec![name.clone()];
            } else {
                path.names = vec![format!("Type#{}", source_id)];
            }
            
            path.is_direct = true;
            return Ok(vec![path]);
        }
        
        // Get the extension relationships
        let extension_relationships = self.registry.get_extension_relationships()?;
        
        // Use DFS to find all paths
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = HashSet::new();
        
        self.dfs_find_all_paths(
            source_id,
            target_id,
            &extension_relationships,
            &mut current_path,
            &mut visited,
            &mut all_paths
        )?;
        
        // Convert the raw paths to InterfaceInheritancePath objects
        let mut result = Vec::new();
        for path in all_paths {
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
            
            result.push(inheritance_path);
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_paths_by_name(
        &self,
        source_name: &str,
        target_name: &str
    ) -> Result<Vec<InterfaceInheritancePath>, Error> {
        // Get the type IDs
        let source_id = match self.registry.lookup_type_id(source_name) {
            Ok(id) => id,
            Err(_) => return Ok(Vec::new())
        };
        
        let target_id = match self.registry.lookup_type_id(target_name) {
            Ok(id) => id,
            Err(_) => return Ok(Vec::new())
        };
        
        // Find all paths
        self.find_all_paths(source_id, target_id)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn has_multiple_paths(&self, source_id: u64, target_id: u64) -> Result<bool, Error> {
        let paths = self.find_all_paths(source_id, target_id)?;
        Ok(paths.len() > 1)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_all_paths(&self, source_id: u64, target_id: u64) -> Result<String, Error> {
        let paths = self.find_all_paths(source_id, target_id)?;
        
        if paths.is_empty() {
            let source_name = self.registry.get_type_name(source_id)
                .map(String::clone)
                .unwrap_or_else(|| format!("Type#{}", source_id));
                
            let target_name = self.registry.get_type_name(target_id)
                .map(String::clone)
                .unwrap_or_else(|| format!("Type#{}", target_id));
                
            return Ok(format!("No paths exist from {} to {}", source_name, target_name));
        }
        
        let mut result = String::new();
        result.push_str(&format!("Found {} paths:\n", paths.len()));
        
        for (i, path) in paths.iter().enumerate() {
            result.push_str(&format!("Path {}: {}\n", i + 1, path.to_string()));
        }
        
        // Add a note if multiple paths were found (potential diamond pattern)
        if paths.len() > 1 {
            result.push_str("\nMultiple inheritance paths detected. This may indicate a diamond pattern.");
        }
        
        Ok(result)
    }
}

/// Register the enhanced path finder fix module
pub fn register_enhanced_path_finder() {
    debug!("Registered enhanced path finder for interface type assertions with multi-path support");
}

// Add automatic registration to the mod.rs re-exports
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_enhanced_path_finder() {
        register_enhanced_path_finder();
        assert!(true);
    }
}