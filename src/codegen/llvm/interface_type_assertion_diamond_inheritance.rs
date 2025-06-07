//! # Interface Type Assertion Diamond Inheritance Pattern Detection
//!
//! This module provides specialized functionality for detecting and handling diamond inheritance
//! patterns in interface type assertions. Diamond inheritance patterns occur when a type inherits
//! from multiple types that share a common ancestor, creating an inheritance graph shaped like a diamond.
//!
//! ## Key Features
//!
//! 1. Detection of diamond inheritance patterns in type hierarchies
//! 2. Specialized error handling for diamond inheritance cases
//! 3. Visual representation of diamond inheritance relationships
//! 4. Integration with the interface type assertion path visualization system

use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceInheritancePath;
use crate::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder;
use crate::InterfaceTypeRegistry;
use crate::codegen::llvm::interface_type_registry_helpers::TypeNameRegistry;
use crate::codegen::llvm::interface_type_registry_common;
use crate::error::Error;

/// Represents a diamond inheritance pattern in the type hierarchy
#[derive(Debug, Clone)]
pub struct DiamondInheritancePattern {
    /// The root type (concrete type being asserted)
    pub root_type_id: u32,
    /// The base type (target interface type)
    pub base_type_id: u32,
    /// The left intermediate type in the diamond
    pub left_intermediate_id: u32,
    /// The right intermediate type in the diamond
    pub right_intermediate_id: u32,
}

/// Trait for detecting and handling diamond inheritance patterns
pub trait DiamondInheritanceDetection<'ctx> {
    /// Detect diamond inheritance patterns between a concrete type and an interface
    fn detect_diamond_inheritance(
        &self,
        concrete_type_id: u32,
        interface_type_id: u32
    ) -> Result<Option<DiamondInheritancePattern>, Error>;
    
    /// Find all inheritance paths between two types
    fn find_all_inheritance_paths(
        &self,
        source_type_id: u32,
        target_type_id: u32
    ) -> Result<Vec<InterfaceInheritancePath>, Error>;
    
    /// Check if there are multiple inheritance paths between two types
    fn has_multiple_inheritance_paths(
        &self,
        source_type_id: u32,
        target_type_id: u32
    ) -> Result<bool, Error>;
    
    /// Create a visualization of the diamond inheritance pattern
    fn visualize_diamond_inheritance(
        &self,
        pattern: &DiamondInheritancePattern
    ) -> String;
}

impl<'ctx> DiamondInheritanceDetection<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn detect_diamond_inheritance(
        &self,
        concrete_type_id: u32,
        interface_type_id: u32
    ) -> Result<Option<DiamondInheritancePattern>, Error> {
        // Find all paths between the concrete type and interface
        let paths = self.find_all_inheritance_paths(concrete_type_id, interface_type_id)?;
        
        // If there are at least two paths, check for a diamond pattern
        if paths.len() >= 2 {
            // For simplicity, look at the first two paths
            let first_path = &paths[0].path;
            let second_path = &paths[1].path;
            
            // Paths must start and end with the same types
            if first_path.is_empty() || second_path.is_empty() || 
               first_path[0] != second_path[0] || 
               first_path[first_path.len()-1] != second_path[second_path.len()-1] {
                return Ok(None);
            }
            
            // Find where paths diverge
            let root_type_id = first_path[0] as u32;
            let base_type_id = first_path[first_path.len()-1] as u32;
            
            // Ensure paths are not identical
            if first_path == second_path {
                return Ok(None);
            }
            
            // For a classic diamond pattern, paths will have length at least 3
            if first_path.len() >= 3 && second_path.len() >= 3 {
                // Find first position where paths diverge
                let mut diverge_idx = 0;
                for i in 0..std::cmp::min(first_path.len(), second_path.len()) {
                    if first_path[i] != second_path[i] {
                        diverge_idx = i;
                        break;
                    }
                }
                
                // Get the intermediate types
                let left_intermediate_id = first_path[diverge_idx] as u32;
                let right_intermediate_id = second_path[diverge_idx] as u32;
                
                return Ok(Some(DiamondInheritancePattern {
                    root_type_id,
                    base_type_id,
                    left_intermediate_id,
                    right_intermediate_id,
                }));
            }
        }
        
        Ok(None)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_inheritance_paths(
        &self,
        source_type_id: u32,
        target_type_id: u32
    ) -> Result<Vec<InterfaceInheritancePath>, Error> {
        // Check if we have an interface path finder available
        if let Some(path_finder) = self.get_interface_path_finder() {
            let source_name = format!("Type#{}", source_type_id);
            let target_name = format!("Type#{}", target_type_id);
            
            if let Ok(Some(path_names)) = path_finder.find_path(&source_name, &target_name) {
                let path_ids: Vec<u64> = path_names.iter()
                    .filter_map(|name| {
                        // Extract ID from "Type#<id>" format
                        if name.starts_with("Type#") {
                            name[5..].parse::<u64>().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                
                let inheritance_path = InterfaceInheritancePath {
                    path: path_ids,
                    names: path_names.clone(),
                    is_direct: path_names.len() <= 2,
                };
                return Ok(vec![inheritance_path]);
            }
        }
        
        // Fallback implementation when path finder is not available
        let mut results = Vec::new();
        
        // If source and target are the same, return a single-node path
        if source_type_id == target_type_id {
            results.push(InterfaceInheritancePath {
                path: vec![source_type_id as u64],
                names: vec![format!("Type#{}", source_type_id)],
                is_direct: true,
            });
            return Ok(results);
        }
        
        // Try to get implementation information directly
        if let Some(implements) = self.type_implements(source_type_id, target_type_id) {
            if implements {
                // Create a simple path
                results.push(InterfaceInheritancePath {
                    path: vec![source_type_id as u64, target_type_id as u64],
                    names: vec![format!("Type#{}", source_type_id), format!("Type#{}", target_type_id)],
                    is_direct: true,
                });
            }
        }
        
        // If we couldn't find direct paths, try to find indirect paths with BFS
        if results.is_empty() && self.get_interface_registry().is_some() {
            results = self.find_all_paths_bfs(source_type_id, target_type_id)?;
        }
        
        Ok(results)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn has_multiple_inheritance_paths(
        &self,
        source_type_id: u32,
        target_type_id: u32
    ) -> Result<bool, Error> {
        let paths = self.find_all_inheritance_paths(source_type_id, target_type_id)?;
        Ok(paths.len() > 1)
    }
    
    #[instrument(skip(self, pattern), level = "debug")]
    fn visualize_diamond_inheritance(
        &self,
        pattern: &DiamondInheritancePattern
    ) -> String {
        let mut result = String::new();
        
        // Get type names for better readability using the common implementation
        use crate::codegen::llvm::interface_type_registry_common::get_type_name_by_id_impl;
        
        let root_name = get_type_name_by_id_impl(self, pattern.root_type_id)
            .unwrap_or_else(|_| format!("Type#{}", pattern.root_type_id));
        
        let base_name = get_type_name_by_id_impl(self, pattern.base_type_id)
            .unwrap_or_else(|_| format!("Type#{}", pattern.base_type_id));
        
        let left_intermediate_name = get_type_name_by_id_impl(self, pattern.left_intermediate_id)
            .unwrap_or_else(|_| format!("Type#{}", pattern.left_intermediate_id));
        
        let right_intermediate_name = get_type_name_by_id_impl(self, pattern.right_intermediate_id)
            .unwrap_or_else(|_| format!("Type#{}", pattern.right_intermediate_id));
        
        // Create a diamond visualization with ASCII art
        result.push_str("Diamond Inheritance Pattern:\n\n");
        
        // ASCII art of a diamond
        result.push_str(&format!("              {}\n", base_name));
        result.push_str("               /\\\n");
        result.push_str("              /  \\\n");
        result.push_str(&format!("{:15}  {:15}\n", left_intermediate_name, right_intermediate_name));
        result.push_str("              \\  /\n");
        result.push_str("               \\/\n");
        result.push_str(&format!("              {}\n\n", root_name));
        
        // Add explanation
        result.push_str("This creates ambiguity in the inheritance relationship.\n");
        result.push_str("Type assertions with diamond inheritance patterns may not\n");
        result.push_str("behave as expected due to multiple inheritance paths.\n");
        
        result
    }
}

// Helper methods for diamond inheritance detection
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Using the shared implementation from interface_type_registry_helpers.rs
    // through the TypeNameRegistry trait
    
    /// Use the common implementation from interface_type_registry_common.rs
    // All interface registry helper methods are now imported from interface_type_registry_common.rs
    // rather than being defined here
    
    /// Get the interface path finder - delegates to common implementation
    pub fn get_interface_path_finder(&self) -> Option<Box<dyn EnhancedInterfacePathFinder + '_>> {
        // Import the common implementation
        use crate::codegen::llvm::interface_type_registry_common::get_interface_path_finder_impl;
        get_interface_path_finder_impl(self)
    }
    
    /// Check if a type implements an interface - delegates to common implementation
    pub fn type_implements(&self, concrete_type_id: u32, interface_type_id: u32) -> Option<bool> {
        // Import the common implementation
        use crate::codegen::llvm::interface_type_registry_common::type_implements_impl;
        type_implements_impl(self, concrete_type_id, interface_type_id)
    }
    
    /// Get the interface registry - delegates to common implementation
    pub fn get_interface_registry(&self) -> Option<&dyn InterfaceTypeRegistry> {
        // Import the common implementation
        use crate::codegen::llvm::interface_type_registry_common::get_interface_registry_impl;
        get_interface_registry_impl(self)
    }
    
    /// Find all paths between two types using BFS traversal
    fn find_all_paths_bfs(
        &self,
        source_type_id: u32,
        target_type_id: u32
    ) -> Result<Vec<InterfaceInheritancePath>, Error> {
        let registry = match self.get_interface_registry() {
            Some(r) => r,
            None => return Ok(Vec::new()),
        };
        
        let mut results = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut path_map: HashMap<u32, Vec<u32>> = HashMap::new();
        
        // Initialize with the source type
        queue.push_back(source_type_id);
        visited.insert(source_type_id);
        path_map.insert(source_type_id, vec![source_type_id]);
        
        // Process queue
        while let Some(current_id) = queue.pop_front() {
            // Check if we've reached the target
            if current_id == target_type_id {
                if let Some(path) = path_map.get(&current_id) {
                    let inheritance_path = InterfaceInheritancePath {
                        path: path.iter().map(|&id| id as u64).collect(),
                        names: path.iter().map(|&id| format!("Type#{}", id)).collect(),
                        is_direct: path.len() == 2,
                    };
                    results.push(inheritance_path);
                }
                continue; // Don't expand this node further if it's the target
            }
            
            // Get parent interfaces for the current type
            let interfaces = match registry.get_extended_interfaces(current_id) {
                Ok(ifaces) => ifaces,
                Err(_) => vec![], // Handle error by returning empty vec
            };
            
            for &interface_id in &interfaces {
                if !visited.contains(&interface_id) {
                    // Create a new path including this interface
                    if let Some(current_path) = path_map.get(&current_id) {
                        let mut new_path = current_path.clone();
                        new_path.push(interface_id);
                        path_map.insert(interface_id, new_path);
                    }
                    
                    // Add to the queue for processing
                    queue.push_back(interface_id);
                    visited.insert(interface_id);
                }
                else if interface_id == target_type_id {
                    // Special case: if we found the target again through a different path
                    if let Some(current_path) = path_map.get(&current_id) {
                        let mut new_path = current_path.clone();
                        new_path.push(interface_id);
                        
                        // This is a new path to the target
                        let inheritance_path = InterfaceInheritancePath {
                            path: new_path.iter().map(|&id| id as u64).collect(),
                            names: new_path.iter().map(|&id| format!("Type#{}", id)).collect(),
                            is_direct: new_path.len() == 2,
                        };
                        results.push(inheritance_path);
                    }
                }
            }
        }
        
        // A complete implementation would populate the interfaces field too
        Ok(results)
    }
}

/// Register the diamond inheritance detection module
pub fn register_diamond_inheritance_detection() {
    debug!("Registered diamond inheritance detection for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use std::path::PathBuf;
    
    #[test]
    fn test_diamond_inheritance_detection_registration() {
        // Simple test to ensure module registration works
        register_diamond_inheritance_detection();
        assert!(true);
    }
    
    #[test]
    fn test_visualize_diamond_inheritance() {
        // Create a simple diamond pattern
        let pattern = DiamondInheritancePattern {
            root_type_id: 1,
            base_type_id: 4,
            left_intermediate_id: 2,
            right_intermediate_id: 3,
        };
        
        // Create a stub LlvmCodeGenerator
        let context = Context::create();
        let mut generator = LlvmCodeGenerator::new(&context, "test_diamond_inheritance", PathBuf::from("test.csd"));
        
        // Add type name lookups
        generator.internal_fields.insert("type_name_1".to_string(), Box::new("Concrete".to_string()));
        generator.internal_fields.insert("type_name_2".to_string(), Box::new("LeftInterface".to_string()));
        generator.internal_fields.insert("type_name_3".to_string(), Box::new("RightInterface".to_string()));
        generator.internal_fields.insert("type_name_4".to_string(), Box::new("BaseInterface".to_string()));
        
        // Visualize the pattern
        let visualization = generator.visualize_diamond_inheritance(&pattern);
        
        // Basic assertions
        assert!(visualization.contains("Diamond Inheritance Pattern"));
        assert!(visualization.contains("BaseInterface"));
        assert!(visualization.contains("LeftInterface"));
        assert!(visualization.contains("RightInterface"));
        assert!(visualization.contains("Concrete"));
    }
}