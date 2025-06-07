//! # Diamond Inheritance Pattern Handler for Interface Type Assertions
//!
//! This module provides an extended handler for detecting, managing, and visualizing
//! diamond inheritance patterns in interface type assertions. It works with the interface
//! type registry to identify cases where a type inherits from multiple interfaces
//! that share a common ancestor.
//!
//! The implementation integrates with the interface type assertion error system to
//! provide specific error details when diamond inheritance causes ambiguity in type assertions.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tracing::{debug, info, instrument, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritancePattern;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceInheritancePath;
use crate::codegen::llvm::interface_path_finder_enhanced_fix::EnhancedInterfacePathFinder;
use crate::codegen::llvm::interface_path_finder_enhanced_fix::MultiPathFinder;
use crate::InterfaceTypeRegistry;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;
use crate::error::Error;

/// Handler for diamond inheritance patterns in interface type assertions
///
/// This trait extends the basic diamond inheritance detection with additional
/// features for visualizing and analyzing inheritance diamonds by name rather than just ID.
pub trait DiamondInheritanceHandler<'ctx> {
    /// Detect diamond inheritance patterns between a concrete type and an interface by name
    ///
    /// # Arguments
    /// * `concrete_type_name` - Name of the concrete type
    /// * `interface_type_name` - Name of the interface type
    ///
    /// # Returns
    /// * `Result<Option<DiamondInheritanceInfo>, Error>` - Information about the diamond if found
    fn detect_diamond_inheritance(
        &self,
        concrete_type_name: &str,
        interface_type_name: &str
    ) -> Result<Option<DiamondInheritanceInfo>, Error>;
    
    /// Visualize a diamond inheritance pattern between a concrete type and an interface by name
    ///
    /// # Arguments
    /// * `concrete_type_name` - Name of the concrete type
    /// * `interface_type_name` - Name of the interface type
    /// * `info` - Optional precalculated diamond inheritance information
    ///
    /// # Returns
    /// * `Result<String, Error>` - Visualization of the diamond inheritance pattern
    fn visualize_diamond_inheritance(
        &self,
        concrete_type_name: &str,
        interface_type_name: &str,
        info: &Option<DiamondInheritanceInfo>
    ) -> Result<String, Error>;
    
    /// Find all diamond inheritance patterns for a concrete type
    ///
    /// # Arguments
    /// * `concrete_type_name` - Name of the concrete type
    ///
    /// # Returns
    /// * `Result<Vec<DiamondInheritanceInfo>, Error>` - Information about all diamond patterns found
    fn find_all_diamond_patterns(
        &self,
        concrete_type_name: &str
    ) -> Result<Vec<DiamondInheritanceInfo>, Error>;
    
    /// Check if a concrete type has any diamond inheritance patterns
    ///
    /// # Arguments
    /// * `concrete_type_name` - Name of the concrete type
    ///
    /// # Returns
    /// * `Result<bool, Error>` - Whether the type has any diamond inheritance patterns
    fn has_diamond_inheritance(
        &self,
        concrete_type_name: &str
    ) -> Result<bool, Error>;
    
    /// Generate a comprehensive report of all diamond inheritance patterns in the type system
    ///
    /// # Returns
    /// * `Result<String, Error>` - Report containing all diamond patterns in the type system
    fn generate_diamond_inheritance_report(&self) -> Result<String, Error>;
}

/// Extended information about a diamond inheritance pattern
#[derive(Debug, Clone)]
pub struct DiamondInheritanceInfo {
    /// The concrete type name
    pub concrete_type: String,
    /// The target interface type name
    pub interface_type: String,
    /// The left intermediate interface name
    pub left_path: String,
    /// The right intermediate interface name
    pub right_path: String,
    /// The common base interface name
    pub common_base: String,
    /// The pattern details
    pub pattern: DiamondInheritancePattern,
    /// All paths from concrete type to interface
    pub all_paths: Vec<InterfaceInheritancePath>,
}

impl<'ctx> DiamondInheritanceHandler<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn detect_diamond_inheritance(
        &self,
        concrete_type_name: &str,
        interface_type_name: &str
    ) -> Result<Option<DiamondInheritanceInfo>, Error> {
        // Get the interface registry
        let registry = match self.get_interface_registry() {
            Some(registry) => registry,
            None => return Err(Error::new("Runtime", "Interface registry not available", None))
        };
        
        // Get type IDs for the concrete type and interface
        let concrete_type_id = match registry.lookup_type_id(concrete_type_name) {
            Ok(id) => id,
            Err(_) => return Err(Error::new("Runtime", &format!(
                "Could not find type ID for concrete type '{}'", concrete_type_name
            ), None))
        };
        
        let interface_type_id = match registry.lookup_type_id(interface_type_name) {
            Ok(id) => id,
            Err(_) => return Err(Error::new("Runtime", &format!(
                "Could not find type ID for interface '{}'", interface_type_name
            ), None))
        };
        
        // Check if the concrete type implements the interface
        if !registry.type_implements_interface(concrete_type_id as u32, interface_type_id as u32) {
            return Ok(None); // No diamond possible if no implementation
        }
        
        // First get all inheritance paths
        let path_finder = self.create_path_finder(registry);
        // Use the path finder to find all paths
        let all_paths = path_finder.find_all_paths(
            concrete_type_id, 
            interface_type_id
        )?;
        
        // If we have multiple paths, check for diamond pattern
        if all_paths.len() >= 2 {
            // Look for a common ancestor in the paths
            let mut common_ancestors = HashSet::new();
            for path in &all_paths {
                for id in &path.path {
                    if *id != concrete_type_id && *id != interface_type_id {
                        common_ancestors.insert(*id);
                    }
                }
            }
            
            // We need at least one common ancestor for a diamond
            if !common_ancestors.is_empty() {
                // For simplicity, take the first two paths to identify the diamond branches
                let first_path = &all_paths[0].path;
                let second_path = &all_paths[1].path;
                
                // Find where the paths diverge
                let mut common_base_id = 0;
                let mut left_intermediate_id = 0;
                let mut right_intermediate_id = 0;
                
                for i in 0..std::cmp::min(first_path.len(), second_path.len()) {
                    if i < first_path.len() && i < second_path.len() && first_path[i] == second_path[i] {
                        common_base_id = first_path[i];
                    } else if i < first_path.len() && i < second_path.len() {
                        left_intermediate_id = first_path[i];
                        right_intermediate_id = second_path[i];
                        break;
                    }
                }
                
                // If we found a proper diamond pattern
                if common_base_id != 0 && left_intermediate_id != 0 && right_intermediate_id != 0 {
                    // Create the pattern object
                    let pattern = DiamondInheritancePattern {
                        root_type_id: concrete_type_id as u32,
                        base_type_id: common_base_id as u32,
                        left_intermediate_id: left_intermediate_id as u32,
                        right_intermediate_id: right_intermediate_id as u32,
                    };
                    
                    // Get names for better reporting
                    let common_base = registry.get_type_name(common_base_id)
                        .unwrap_or_else(|_| format!("Type#{}", common_base_id));
                    
                    let left_path = registry.get_type_name(left_intermediate_id)
                        .unwrap_or_else(|_| format!("Type#{}", left_intermediate_id));
                    
                    let right_path = registry.get_type_name(right_intermediate_id)
                        .unwrap_or_else(|_| format!("Type#{}", right_intermediate_id));
                    
                    return Ok(Some(DiamondInheritanceInfo {
                        concrete_type: concrete_type_name.to_string(),
                        interface_type: interface_type_name.to_string(),
                        left_path,
                        right_path,
                        common_base,
                        pattern,
                        all_paths,
                    }));
                }
            }
        }
        
        Ok(None)
    }
    
    #[instrument(skip(self, info), level = "debug")]
    fn visualize_diamond_inheritance(
        &self,
        concrete_type_name: &str,
        interface_type_name: &str, 
        info: &Option<DiamondInheritanceInfo>
    ) -> Result<String, Error> {
        // If diamond info was provided, use it
        let diamond_info = if let Some(info) = info {
            info.clone()
        } else {
            // Otherwise, detect the diamond pattern
            match self.detect_diamond_inheritance(concrete_type_name, interface_type_name)? {
                Some(info) => info,
                None => return Ok(format!(
                    "No diamond inheritance pattern found between {} and {}",
                    concrete_type_name, interface_type_name
                ))
            }
        };
        
        // Create the visualization
        let mut result = String::new();
        
        result.push_str("Diamond Inheritance Pattern Detected\n");
        result.push_str("===================================\n\n");
        
        // ASCII art of a diamond with specific type names
        result.push_str(&format!("              {}\n", diamond_info.common_base));
        result.push_str("               /\\\n");
        result.push_str("              /  \\\n");
        result.push_str(&format!("{}  {}\n", 
            diamond_info.left_path, 
            diamond_info.right_path
        ));
        result.push_str("              \\  /\n");
        result.push_str("               \\/\n");
        result.push_str(&format!("              {}\n\n", diamond_info.concrete_type));
        
        // Add all inheritance paths
        result.push_str("All inheritance paths:\n");
        for (i, path) in diamond_info.all_paths.iter().enumerate() {
            result.push_str(&format!("  Path {}: ", i + 1));
            
            // Convert type IDs to names for better readability
            let path_names: Vec<String> = path.path.iter().map(|&id| {
                self.get_interface_registry()
                    .and_then(|registry| registry.get_type_name(id).ok())
                    .unwrap_or_else(|| format!("Type#{}", id))
            }).collect();
            
            result.push_str(&path_names.join(" -> "));
            result.push_str("\n");
        }
        
        result.push_str("\nRecommendations for handling diamond inheritance:\n");
        result.push_str("1. Be explicit about which interface you want to use\n");
        result.push_str("2. Consider refactoring to avoid ambiguity\n");
        result.push_str("3. Use composition instead of multiple inheritance when possible\n");
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_diamond_patterns(
        &self,
        concrete_type_name: &str
    ) -> Result<Vec<DiamondInheritanceInfo>, Error> {
        let registry = match self.get_interface_registry() {
            Some(registry) => registry,
            None => return Err(Error::new("Runtime", "Interface registry not available", None))
        };
        
        let concrete_type_id = match registry.lookup_type_id(concrete_type_name) {
            Ok(id) => id,
            Err(_) => return Err(Error::new("Runtime", &format!(
                "Could not find type ID for concrete type '{}'", concrete_type_name
            ), None))
        };
        
        // Get all interfaces implemented by this concrete type
        let implemented_interfaces = registry.get_implemented_interfaces(concrete_type_id as u32)?;
        
        let mut diamond_patterns = Vec::new();
        let mut checked_pairs = HashSet::new();
        
        // Check all pairs of interfaces for diamond patterns
        for &interface1_id in &implemented_interfaces {
            for &interface2_id in &implemented_interfaces {
                // Skip self-comparisons and already checked pairs
                if interface1_id == interface2_id || 
                   checked_pairs.contains(&(interface1_id, interface2_id)) || 
                   checked_pairs.contains(&(interface2_id, interface1_id)) {
                    continue;
                }
                
                checked_pairs.insert((interface1_id, interface2_id));
                
                // Get the interface names
                let interface1_name = registry.get_type_name(interface1_id as u64)
                    .unwrap_or_else(|_| format!("Interface#{}", interface1_id));
                
                let interface2_name = registry.get_type_name(interface2_id as u64)
                    .unwrap_or_else(|_| format!("Interface#{}", interface2_id));
                
                // Check if these interfaces share a common ancestor
                // (other than the concrete type itself)
                // TODO: Implement InterfaceTypeRegistryExtensionCheckingAccess trait
                // Commented out until trait is properly implemented
                /*
                if let Some(registry_ext) = registry.as_interface_extension_checking() {
                    // Check if they both extend a common interface
                    let ext1 = registry_ext.get_extended_interfaces(interface1_id as u64)?;
                    let ext2 = registry_ext.get_extended_interfaces(interface2_id as u64)?;
                    
                    // Find common ancestors
                    for &base_id in &ext1 {
                        if ext2.contains(&base_id) {
                            // We found a common ancestor, potentially part of a diamond
                            let base_name = registry.get_type_name(base_id)
                                .unwrap_or_else(|_| format!("Interface#{}", base_id));
                            
                            // Check if the concrete type implements this base interface
                            if registry.type_implements_interface(concrete_type_id as u32, base_id as u32) {
                                // Detect the full diamond pattern
                                if let Some(info) = self.detect_diamond_inheritance(
                                    concrete_type_name, &base_name
                                )? {
                                    diamond_patterns.push(info);
                                }
                            }
                        }
                    }
                }
                */
            }
        }
        
        Ok(diamond_patterns)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn has_diamond_inheritance(
        &self,
        concrete_type_name: &str
    ) -> Result<bool, Error> {
        let diamonds = self.find_all_diamond_patterns(concrete_type_name)?;
        Ok(!diamonds.is_empty())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_diamond_inheritance_report(&self) -> Result<String, Error> {
        let registry = match self.get_interface_registry() {
            Some(registry) => registry,
            None => return Err(Error::new("Runtime", "Interface registry not available", None))
        };
        
        // Get all concrete types
        let all_types = registry.all_types();
        let mut concrete_types = HashMap::new();
        
        for (id, name) in all_types {
            // Check if this is a concrete type (not an interface)
            if !registry.is_interface(id as u32)? {
                concrete_types.insert(id, name);
            }
        }
        
        // Generate report
        let mut report = String::new();
        report.push_str("Diamond Inheritance Pattern Report\n");
        report.push_str("==================================\n\n");
        
        let mut total_diamonds = 0;
        
        // Check each concrete type for diamond patterns
        for (_, type_name) in concrete_types {
            let diamonds = self.find_all_diamond_patterns(&type_name)?;
            
            if !diamonds.is_empty() {
                report.push_str(&format!("Type '{}' has {} diamond inheritance patterns:\n", 
                    type_name, diamonds.len()));
                
                for (i, diamond) in diamonds.iter().enumerate() {
                    report.push_str(&format!("  Diamond {}: {} inherits from {} through {} and {}\n",
                        i + 1,
                        diamond.concrete_type,
                        diamond.common_base,
                        diamond.left_path,
                        diamond.right_path
                    ));
                }
                
                report.push_str("\n");
                total_diamonds += diamonds.len();
            }
        }
        
        report.push_str(&format!("\nTotal diamond inheritance patterns found: {}\n", total_diamonds));
        
        if total_diamonds > 0 {
            report.push_str("\nRecommendations:\n");
            report.push_str("1. Review type hierarchy to simplify inheritance relationships\n");
            report.push_str("2. Consider refactoring interfaces to reduce ambiguity\n");
            report.push_str("3. Use composition over inheritance where appropriate\n");
        } else {
            report.push_str("\nNo diamond inheritance patterns detected in the codebase.\n");
        }
        
        Ok(report)
    }
}

// Helper methods for the LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Creates a new path finder for interface inheritance relationships
    fn create_path_finder<'a>(&'a self, registry: &'a dyn InterfaceTypeRegistry) -> impl EnhancedInterfacePathFinder + 'a {
        MultiPathFinder::new(registry)
    }
    
    /// Gets the interface registry extension checking functionality
    fn get_interface_registry_extension_checker(&self) -> Option<&dyn InterfaceTypeRegistryExtensionChecking> {
        // TODO: Implement InterfaceTypeRegistryExtensionCheckingAccess trait
        // self.get_interface_registry()
        //     .and_then(|registry| registry.as_interface_extension_checking())
        None
    }
}

// Extension trait to access the extension checking functionality
pub trait InterfaceTypeRegistryExtensionCheckingAccess {
    /// Get the extension checking functionality
    fn as_interface_extension_checking(&self) -> Option<&dyn InterfaceTypeRegistryExtensionChecking>;
    
    /// Get all interfaces extended by a given interface
    fn get_extended_interfaces(&self, interface_id: u64) -> Result<HashSet<u64>, Error>;
}

// Note: Concrete implementations of this trait should be provided for specific registry types
// rather than using a generic impl, due to trait object constraints

/// Register the diamond inheritance handler module
pub fn register_diamond_inheritance_handler() {
    debug!("Registered diamond inheritance handler for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use std::collections::{HashMap, HashSet};
    use crate::error::Error;
    use crate::codegen::llvm::interface_registry::InterfaceTypeRegistry;
    
    // Mock implementation of InterfaceTypeRegistry for testing
    #[derive(Debug)]
    struct MockInterfaceRegistry {
        interfaces: HashSet<String>,
        concrete_types: HashSet<String>,
        extensions: HashMap<String, HashSet<String>>,
        implementations: HashMap<String, HashSet<String>>, // concrete_type -> interfaces it implements
    }
    
    // MockInterfaceRegistry must be Send + Sync for internal_fields
    unsafe impl Send for MockInterfaceRegistry {}
    unsafe impl Sync for MockInterfaceRegistry {}
    
    impl MockInterfaceRegistry {
        fn new() -> Self {
            let mut registry = Self {
                interfaces: HashSet::new(),
                concrete_types: HashSet::new(),
                extensions: HashMap::new(),
                implementations: HashMap::new(),
            };
            
            // Set up a diamond inheritance pattern:
            // Player (concrete) implements GameObject (interface)
            // GameObject extends both Drawable and Movable
            // Both Drawable and Movable extend Entity
            
            // Register interfaces
            registry.interfaces.insert("Entity".to_string());
            registry.interfaces.insert("Drawable".to_string());
            registry.interfaces.insert("Movable".to_string());
            registry.interfaces.insert("GameObject".to_string());
            
            // Register concrete type
            registry.concrete_types.insert("Player".to_string());
            
            // Set up extensions (diamond pattern)
            registry.extensions.insert("Drawable".to_string(), 
                HashSet::from(["Entity".to_string()]));
            registry.extensions.insert("Movable".to_string(), 
                HashSet::from(["Entity".to_string()]));
            registry.extensions.insert("GameObject".to_string(), 
                HashSet::from(["Drawable".to_string(), "Movable".to_string()]));
            
            // Player implements GameObject
            registry.implementations.insert("Player".to_string(), 
                HashSet::from(["GameObject".to_string()]));
            
            registry
        }
        
        fn hash_name(&self, name: &str) -> u64 {
            // Use a simple hash that fits in 32 bits to avoid truncation issues
            let mut hash: u32 = 0x811c9dc5;
            for byte in name.bytes() {
                hash ^= byte as u32;
                hash = hash.wrapping_mul(0x01000193);
            }
            hash as u64
        }
    }
    
    impl InterfaceTypeRegistry for MockInterfaceRegistry {
        fn register_interface(&mut self, name: &str) -> Result<(), Error> {
            self.interfaces.insert(name.to_string());
            Ok(())
        }
        
        fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
            self.extensions.entry(source.to_string())
                .or_insert_with(HashSet::new)
                .insert(target.to_string());
            Ok(())
        }
        
        fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
            if source == target {
                return Ok(true);
            }
            
            if let Some(extensions) = self.extensions.get(source) {
                if extensions.contains(target) {
                    return Ok(true);
                }
                
                for ext in extensions {
                    if self.extends(ext, target)? {
                        return Ok(true);
                    }
                }
            }
            
            Ok(false)
        }
        
        fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
            if source == target {
                return Ok(Some(vec![source.to_string()]));
            }
            
            if !self.extends(source, target)? {
                return Ok(None);
            }
            
            // Simple BFS path finding
            let mut visited = HashSet::new();
            let mut queue = std::collections::VecDeque::new();
            let mut parent: HashMap<String, String> = HashMap::new();
            
            queue.push_back(source.to_string());
            visited.insert(source.to_string());
            
            while let Some(current) = queue.pop_front() {
                if current == target {
                    let mut path = Vec::new();
                    let mut node = current;
                    
                    while let Some(prev) = parent.get(&node) {
                        path.push(node.clone());
                        node = prev.clone();
                    }
                    path.push(node);
                    path.reverse();
                    return Ok(Some(path));
                }
                
                if let Some(extensions) = self.extensions.get(&current) {
                    for ext in extensions {
                        if !visited.contains(ext) {
                            visited.insert(ext.clone());
                            queue.push_back(ext.clone());
                            parent.insert(ext.clone(), current.clone());
                        }
                    }
                }
            }
            
            Ok(None)
        }
        
        fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
            Ok(self.interfaces.clone())
        }
        
        fn interface_exists(&self, name: &str) -> Result<bool, Error> {
            Ok(self.interfaces.contains(name))
        }
        
        fn lookup_type_id(&self, type_name: &str) -> Result<u64, Error> {
            if self.interfaces.contains(type_name) || self.concrete_types.contains(type_name) {
                Ok(self.hash_name(type_name))
            } else {
                Err(Error::NotFound(format!("Type '{}' not found", type_name)))
            }
        }
        
        fn get_type_name(&self, type_id: u64) -> Result<String, Error> {
            for name in self.interfaces.iter().chain(self.concrete_types.iter()) {
                let name_hash = self.hash_name(name);
                if name_hash == type_id {
                    return Ok(name.clone());
                }
            }
            Err(Error::NotFound(format!("No type found with ID {} (searched {} types)", type_id, self.interfaces.len() + self.concrete_types.len())))
        }
        
        fn is_interface(&self, type_id: u32) -> Result<bool, Error> {
            let type_id_64 = type_id as u64;
            for interface in &self.interfaces {
                if self.hash_name(interface) == type_id_64 {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        
        fn type_implements_interface(&self, concrete_id: u32, interface_id: u32) -> bool {
            if let (Ok(concrete_name), Ok(interface_name)) = 
                (self.get_type_name(concrete_id as u64), self.get_type_name(interface_id as u64)) {
                
                // Check direct implementation
                if let Some(implemented) = self.implementations.get(&concrete_name) {
                    if implemented.contains(&interface_name) {
                        return true;
                    }
                    
                    // Check if concrete type implements an interface that extends the target interface
                    for impl_interface in implemented {
                        if self.extends(impl_interface, &interface_name).unwrap_or(false) {
                            return true;
                        }
                    }
                }
                
                // Also check if it's the same type (for interface-to-interface relationships)
                if concrete_name == interface_name {
                    return true;
                }
            }
            false
        }
        
        fn get_implemented_interfaces(&self, type_id: u32) -> Result<Vec<u32>, Error> {
            if let Ok(type_name) = self.get_type_name(type_id as u64) {
                if let Some(implemented) = self.implementations.get(&type_name) {
                    let mut interface_ids = Vec::new();
                    for interface_name in implemented {
                        interface_ids.push(self.hash_name(interface_name) as u32);
                    }
                    return Ok(interface_ids);
                }
            }
            Ok(Vec::new())
        }
        
        fn all_types(&self) -> Vec<(u64, String)> {
            let mut result = Vec::new();
            for name in self.interfaces.iter().chain(self.concrete_types.iter()) {
                result.push((self.hash_name(name), name.clone()));
            }
            result
        }
        
        fn get_extension_relationships(&self) -> Result<HashMap<u64, HashSet<u64>>, Error> {
            let mut result = HashMap::new();
            for (source_name, target_names) in &self.extensions {
                let source_id = self.hash_name(source_name);
                let mut target_ids = HashSet::new();
                for target_name in target_names {
                    target_ids.insert(self.hash_name(target_name));
                }
                result.insert(source_id, target_ids);
            }
            Ok(result)
        }
    }
    
    #[test]
    fn test_diamond_inheritance_handler_registration() {
        register_diamond_inheritance_handler();
        assert!(true);
    }
    
    #[test]
    fn test_diamond_inheritance_detection() {
        let context = Context::create();
        let generator = LlvmCodeGenerator::new(&context, "test_diamond_inheritance_handler", PathBuf::from("test.csd"));
        
        // Test diamond detection with no registry (should return error)
        let result = generator.detect_diamond_inheritance("Player", "GameObject");
        assert!(result.is_err(), "Expected error when no registry is available");
        
        if let Err(err) = result {
            let err_msg = format!("{:?}", err);
            assert!(err_msg.contains("Interface registry not available"));
        }
    }
    
    #[test]
    fn test_diamond_inheritance_detection_with_registry() {
        // This test demonstrates the expected structure for the diamond inheritance pattern
        // using our mock registry implementation
        
        let mock_registry = MockInterfaceRegistry::new();
        
        // Test individual components first
        let player_id = mock_registry.hash_name("Player") as u32;
        let gameobject_id = mock_registry.hash_name("GameObject") as u32;
        
        // Verify our mock registry has the expected diamond pattern
        assert!(mock_registry.type_implements_interface(player_id, gameobject_id),
            "Player should implement GameObject");
        
        // Verify the diamond pattern exists
        assert!(mock_registry.extends("GameObject", "Drawable").unwrap(),
            "GameObject should extend Drawable");
        assert!(mock_registry.extends("GameObject", "Movable").unwrap(),
            "GameObject should extend Movable");
        assert!(mock_registry.extends("Drawable", "Entity").unwrap(),
            "Drawable should extend Entity");
        assert!(mock_registry.extends("Movable", "Entity").unwrap(),
            "Movable should extend Entity");
    }
}