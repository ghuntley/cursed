//! # Interface Registry Extensions
//!
//! This module provides extension traits and implementations for the interface registry
//! to support additional features like caching, LRU cache, and visualization.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::error::Error;
use tracing::{debug, trace, instrument};

/// Extension trait for interface registries to provide additional functionality
pub trait InterfaceRegistryExtension: Send + Sync {
    /// Register a new interface
    fn register_interface(&mut self, name: &str);
    
    /// Register an extension relationship between interfaces
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error>;
    
    /// Check if an extension relationship exists between interfaces
    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Get all registered interfaces
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Get all direct extensions of an interface
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all direct implementers of an interface
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Check if an interface extends another interface (direct or indirect)
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Find a common ancestor between two interfaces
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error>;
    
    /// Find the longest path between two interfaces
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get direct implementors (Vec version for compatibility)
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get extension hierarchy
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error>;
    
    /// Find inheritance path between two interfaces
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error>;
    
    /// Find all inheritance paths between two interfaces  
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error>;
    
    /// Get all extensions of an interface
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Get all implementors of an interface
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error>;
}

/// A thread-safe implementation of InterfaceRegistryExtension
pub struct ThreadSafeInterfaceExtensionRegistry {
    /// Direct extensions (interface -> set of interfaces it directly extends)
    direct_extensions: RwLock<HashMap<String, HashSet<String>>>,
    
    /// Direct implementers (interface -> set of interfaces that directly extend it)
    direct_implementers: RwLock<HashMap<String, HashSet<String>>>,
    
    /// All registered interfaces
    interfaces: RwLock<HashSet<String>>,
}

impl ThreadSafeInterfaceExtensionRegistry {
    /// Create a new thread-safe interface registry
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            direct_extensions: RwLock::new(HashMap::new()),
            direct_implementers: RwLock::new(HashMap::new()),
            interfaces: RwLock::new(HashSet::new()),
        }))
    }
    
    /// Get access to direct extensions
    pub fn direct_extensions(&self) -> &RwLock<HashMap<String, HashSet<String>>> {
        &self.direct_extensions
    }
    
    /// Get access to direct implementers
    pub fn direct_implementers(&self) -> &RwLock<HashMap<String, HashSet<String>>> {
        &self.direct_implementers
    }
    
    /// Get access to interfaces
    pub fn interfaces(&self) -> &RwLock<HashSet<String>> {
        &self.interfaces
    }
}

impl InterfaceRegistryExtension for ThreadSafeInterfaceExtensionRegistry {
    #[instrument(skip(self), level = "debug")]
    fn register_interface(&mut self, name: &str) {
        self.interfaces.write().unwrap().insert(name.to_string());
        debug!("Registered interface: {}", name);
    }
    
    #[instrument(skip(self), level = "debug")]
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        // Ensure both interfaces exist
        if !self.interfaces.read().unwrap().contains(source) {
            return Err(Error::NotFound(format!("Source interface '{}' not found", source)));
        }
        
        if !self.interfaces.read().unwrap().contains(target) {
            return Err(Error::NotFound(format!("Target interface '{}' not found", target)));
        }
        
        // Add to direct extensions
        self.direct_extensions.write().unwrap()
            .entry(source.to_string())
            .or_insert_with(HashSet::new)
            .insert(target.to_string());
        
        // Add to direct implementers
        self.direct_implementers.write().unwrap()
            .entry(target.to_string())
            .or_insert_with(HashSet::new)
            .insert(source.to_string());
        
        debug!("Registered extension: {} extends {}", source, target);
        Ok(())
    }
    
    #[instrument(skip(self), level = "trace")]
    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        if let Some(extensions) = self.direct_extensions.read().unwrap().get(source) {
            if extensions.contains(target) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        Ok(self.interfaces.read().unwrap().clone())
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        if let Some(extensions) = self.direct_extensions.read().unwrap().get(interface) {
            Ok(Some(extensions.iter().cloned().collect()))
        } else {
            if !self.interfaces.read().unwrap().contains(interface) {
                return Err(Error::NotFound(format!("Interface '{}' not found", interface)));
            }
            Ok(None)
        }
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        if let Some(implementers) = self.direct_implementers.read().unwrap().get(interface) {
            Ok(Some(implementers.iter().cloned().collect()))
        } else {
            if !self.interfaces.read().unwrap().contains(interface) {
                return Err(Error::NotFound(format!("Interface '{}' not found", interface)));
            }
            Ok(None)
        }
    }
    
    /// Public method for get_direct_implementors (Vec version for compatibility)
    #[instrument(skip(self), level = "trace")]
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        self.get_direct_implementers(interface)
    }
    
    /// Public method for getting extension hierarchy
    #[instrument(skip(self), level = "trace")]
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        use std::collections::HashMap;
        let mut hierarchy = HashMap::new();
        
        if let Ok(interfaces) = self.get_all_interfaces() {
            for interface in interfaces.iter() {
                if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
                    hierarchy.insert(interface.clone(), extensions.into_iter().collect());
                }
            }
        }
        
        Ok(hierarchy)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        if source == target {
            return Ok(true); // An interface extends itself
        }
        
        // Check if there is a direct extension
        if self.has_extension(source, target)? {
            return Ok(true);
        }
        
        // Recursively check extensions
        if let Some(extensions) = self.get_direct_extensions(source)? {
            for ext in extensions {
                if self.extends(&ext, target)? {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error> {
        // First check if either is an ancestor of the other
        if self.extends(a, b)? {
            return Ok(Some(b.to_string()));
        }
        
        if self.extends(b, a)? {
            return Ok(Some(a.to_string()));
        }
        
        // Get the ancestors of a
        let mut a_ancestors = HashSet::new();
        self.collect_ancestors(a, &mut a_ancestors)?;
        
        // Get the ancestors of b
        let mut b_ancestors = HashSet::new();
        self.collect_ancestors(b, &mut b_ancestors)?;
        
        // Find the common ancestors
        let common_ancestors: Vec<_> = a_ancestors.intersection(&b_ancestors).cloned().collect();
        
        // Return the first common ancestor found
        Ok(common_ancestors.first().cloned())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        if !self.extends(source, target)? {
            return Ok(None);
        }
        
        // Use dynamic programming to find the longest path
        let mut memo = HashMap::new();
        let path = self.find_longest_path_dp(source, target, &mut memo)?;
        
        Ok(Some(path))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        if source == target {
            return Ok(vec![source.to_string()]);
        }
        
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        
        if self.find_path_recursive(source, target, &mut path, &mut visited)? {
            Ok(path)
        } else {
            Err(Error::NotFound(format!("No inheritance path from {} to {}", source, target)))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        let mut all_paths = Vec::new();
        let mut current_path = vec![source.to_string()];
        let mut visited = HashSet::new();
        
        self.find_all_paths_recursive(source, target, &mut current_path, &mut all_paths, &mut visited)?;
        
        Ok(all_paths)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let mut all_extensions = HashSet::new();
        let mut visited = HashSet::new();
        
        self.collect_all_extensions_recursive(interface, &mut all_extensions, &mut visited)?;
        
        Ok(all_extensions)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let mut all_implementors = HashSet::new();
        let interfaces = self.get_all_interfaces()?;
        
        for candidate in &interfaces {
            if self.extends(candidate, interface)? && candidate != interface {
                all_implementors.insert(candidate.clone());
            }
        }
        
        Ok(all_implementors)
    }
}

impl ThreadSafeInterfaceExtensionRegistry {
    // Helper methods for path finding
    fn find_path_recursive(
        &self,
        current: &str,
        target: &str,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> Result<bool, Error> {
        if visited.contains(current) {
            return Ok(false);
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        if current == target {
            return Ok(true);
        }
        
        if let Ok(Some(extensions)) = self.get_direct_extensions(current) {
            for ext in extensions {
                if self.find_path_recursive(&ext, target, path, visited)? {
                    return Ok(true);
                }
            }
        }
        
        path.pop();
        visited.remove(current);
        Ok(false)
    }
    
    fn find_all_paths_recursive(
        &self,
        current: &str,
        target: &str,
        current_path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
        visited: &mut HashSet<String>,
    ) -> Result<(), Error> {
        if visited.contains(current) {
            return Ok(());
        }
        
        if current == target {
            all_paths.push(current_path.clone());
            return Ok(());
        }
        
        visited.insert(current.to_string());
        
        if let Ok(Some(extensions)) = self.get_direct_extensions(current) {
            for ext in extensions {
                current_path.push(ext.clone());
                self.find_all_paths_recursive(&ext, target, current_path, all_paths, visited)?;
                current_path.pop();
            }
        }
        
        visited.remove(current);
        Ok(())
    }
    
    fn collect_all_extensions_recursive(
        &self,
        interface: &str,
        all_extensions: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), Error> {
        if visited.contains(interface) {
            return Ok(());
        }
        
        visited.insert(interface.to_string());
        
        if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
            for ext in extensions {
                all_extensions.insert(ext.clone());
                self.collect_all_extensions_recursive(&ext, all_extensions, visited)?;
            }
        }
        
        Ok(())
    }
}

/// Trait implementation for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>
impl InterfaceRegistryExtension for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>> {
    fn register_interface(&mut self, name: &str) {
        self.write().unwrap().register_interface(name);
    }
    
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        self.write().unwrap().register_extension(source, target)
    }
    
    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.read().unwrap().has_extension(source, target)
    }
    
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        self.read().unwrap().get_all_interfaces()
    }
    
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        self.read().unwrap().get_direct_extensions(interface)
    }
    
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        self.read().unwrap().get_direct_implementers(interface)
    }
    
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        match self.get_direct_implementers(interface)? {
            Some(implementers) => Ok(Some(implementers.into_iter().collect())),
            None => Ok(None),
        }
    }
    
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        use std::collections::HashMap;
        let mut hierarchy = HashMap::new();
        
        if let Ok(interfaces) = self.get_all_interfaces() {
            for interface in interfaces.iter() {
                if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
                    hierarchy.insert(interface.clone(), extensions.into_iter().collect());
                }
            }
        }
        
        Ok(hierarchy)
    }
    
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.read().unwrap().extends(source, target)
    }
    
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error> {
        self.read().unwrap().find_common_ancestor(a, b)
    }
    
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        self.read().unwrap().find_longest_path(source, target)
    }
    
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        self.read().unwrap().find_inheritance_path(source, target)
    }
    
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        self.read().unwrap().find_all_inheritance_paths(source, target)
    }
    
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        self.read().unwrap().get_all_extensions(interface)
    }
    
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        self.read().unwrap().get_all_implementors(interface)
    }
}

// Note: Cannot implement on Arc<RwLock<T>> as it's outside our crate
// Users can call .extends() directly on the Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>

impl ThreadSafeInterfaceExtensionRegistry {
    
    /// Helper method to collect all ancestors of an interface
    #[instrument(skip(self, ancestors), level = "trace")]
    fn collect_ancestors(&self, interface: &str, ancestors: &mut HashSet<String>) -> Result<(), Error> {
        if let Some(extensions) = self.get_direct_extensions(interface)? {
            for ext in extensions {
                ancestors.insert(ext.clone());
                self.collect_ancestors(&ext, ancestors)?;
            }
        }
        
        Ok(())
    }
    
    /// Helper method to find the longest path between two interfaces using dynamic programming
    #[instrument(skip(self, memo), level = "trace")]
    fn find_longest_path_dp(
        &self,
        source: &str,
        target: &str,
        memo: &mut HashMap<(String, String), Vec<String>>
    ) -> Result<Vec<String>, Error> {
        // Check if result is already memoized
        let key = (source.to_string(), target.to_string());
        if let Some(path) = memo.get(&key) {
            return Ok(path.clone());
        }
        
        // Direct case
        if source == target {
            let path = vec![source.to_string()];
            memo.insert(key, path.clone());
            return Ok(path);
        }
        
        // If there is a direct extension, return it
        if self.has_extension(source, target)? {
            let path = vec![source.to_string(), target.to_string()];
            memo.insert(key, path.clone());
            return Ok(path);
        }
        
        // Try all possible paths through extensions
        let mut longest_path = Vec::new();
        if let Some(extensions) = self.get_direct_extensions(source)? {
            for ext in extensions {
                if self.extends(&ext, target)? {
                    let mut path = self.find_longest_path_dp(&ext, target, memo)?;
                    path.insert(0, source.to_string());
                    
                    if path.len() > longest_path.len() {
                        longest_path = path;
                    }
                }
            }
        }
        
        // Memoize the result
        memo.insert(key, longest_path.clone());
        
        Ok(longest_path)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        if source == target {
            return Ok(vec![source.to_string()]);
        }
        
        if !self.extends(source, target)? {
            return Err(Error::NotFound(format!("No inheritance path from {} to {}", source, target)));
        }
        
        // Use BFS to find the shortest path
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();
        
        queue.push_back(source.to_string());
        visited.insert(source.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == target {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = target.to_string();
                
                while let Some(p) = parent.get(&node) {
                    path.push(node.clone());
                    node = p.clone();
                }
                path.push(source.to_string());
                path.reverse();
                
                return Ok(path);
            }
            
            if let Some(extensions) = self.get_direct_extensions(&current)? {
                for ext in extensions {
                    if !visited.contains(&ext) {
                        visited.insert(ext.clone());
                        parent.insert(ext.clone(), current.clone());
                        queue.push_back(ext);
                    }
                }
            }
        }
        
        Err(Error::NotFound(format!("No inheritance path from {} to {}", source, target)))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        if source == target {
            return Ok(vec![vec![source.to_string()]]);
        }
        
        if !self.extends(source, target)? {
            return Ok(Vec::new());
        }
        
        let mut all_paths = Vec::new();
        let mut current_path = vec![source.to_string()];
        
        self.find_all_paths_recursive(source, target, &mut current_path, &mut all_paths)?;
        
        Ok(all_paths)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let mut all_extensions = HashSet::new();
        
        // Add direct extensions
        if let Some(direct_extensions) = self.get_direct_extensions(interface)? {
            for ext in direct_extensions {
                all_extensions.insert(ext.clone());
                
                // Recursively add extensions of extensions
                let nested_extensions = self.get_all_extensions(&ext)?;
                all_extensions.extend(nested_extensions);
            }
        }
        
        Ok(all_extensions)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let mut all_implementors = HashSet::new();
        
        // Add direct implementors
        if let Some(direct_implementors) = self.get_direct_implementors(interface)? {
            for impl_name in direct_implementors {
                all_implementors.insert(impl_name.clone());
                
                // Recursively add implementors of implementors
                let nested_implementors = self.get_all_implementors(&impl_name)?;
                all_implementors.extend(nested_implementors);
            }
        }
        
        Ok(all_implementors)
    }
    
    /// Helper method for finding all paths recursively
    #[instrument(skip(self, current_path, all_paths), level = "trace")]
    fn find_all_paths_recursive(
        &self,
        current: &str,
        target: &str,
        current_path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>
    ) -> Result<(), Error> {
        if current == target {
            all_paths.push(current_path.clone());
            return Ok(());
        }
        
        if let Some(extensions) = self.get_direct_extensions(current)? {
            for ext in extensions {
                // Avoid cycles
                if !current_path.contains(&ext) {
                    current_path.push(ext.clone());
                    self.find_all_paths_recursive(&ext, target, current_path, all_paths)?;
                    current_path.pop();
                }
            }
        }
        
        Ok(())
    }
    
    /// Helper method to find a path between two interfaces
    #[instrument(skip(self, visited, path), level = "trace")]
    fn find_path_helper(&self, current: &str, target: &str, visited: &mut HashSet<String>, path: &mut Vec<String>) -> Result<bool, Error> {
        path.push(current.to_string());
        
        if current == target {
            return Ok(true);
        }
        
        visited.insert(current.to_string());
        
        if let Some(extensions) = self.get_direct_extensions(current)? {
            for next in extensions {
                if !visited.contains(&next) {
                    if self.find_path_helper(&next, target, visited, path)? {
                        return Ok(true);
                    }
                }
            }
        }
        
        path.pop();
        Ok(false)
    }
    
    /// Helper method to find all paths between two interfaces
    #[instrument(skip(self, current_path, visited, all_paths), level = "trace")]
    fn find_all_paths_helper(&self, current: &str, target: &str, current_path: &mut Vec<String>, visited: &mut HashSet<String>, all_paths: &mut Vec<Vec<String>>) -> Result<(), Error> {
        if current == target {
            all_paths.push(current_path.clone());
            return Ok(());
        }
        
        visited.insert(current.to_string());
        
        if let Some(extensions) = self.get_direct_extensions(current)? {
            for next in extensions {
                if !visited.contains(&next) {
                    current_path.push(next.clone());
                    self.find_all_paths_helper(&next, target, current_path, visited, all_paths)?;
                    current_path.pop();
                }
            }
        }
        
        visited.remove(current);
        Ok(())
    }
    
    /// Helper method to collect all descendants of an interface
    #[instrument(skip(self, descendants), level = "trace")]
    fn collect_descendants(&self, interface: &str, descendants: &mut HashSet<String>) -> Result<(), Error> {
        if let Some(implementers) = self.get_direct_implementers(interface)? {
            for impl_name in implementers {
                descendants.insert(impl_name.clone());
                self.collect_descendants(&impl_name, descendants)?;
            }
        }
        
        Ok(())
    }
}

/// Create a thread-safe interface registry with the given name and interfaces
pub fn create_interface_registry(name: &str, interfaces: &[&str]) -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Register interfaces
    for interface in interfaces {
        registry.write().unwrap().register_interface(interface);
    }
    
    debug!("Created interface registry: {} with {} interfaces", 
           name, interfaces.len());
    
    registry
}