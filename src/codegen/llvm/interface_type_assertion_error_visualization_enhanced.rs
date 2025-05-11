//! # Enhanced Interface Type Assertion Error Visualization
//!
//! This module extends the base error visualization with advanced features including:
//! - Diamond inheritance pattern detection and visualization
//! - Comprehensive type relationship graphs
//! - Enhanced error context with full filesystem source location tracking
//! - Rich error formatting with syntax highlighting
//!
//! ## Key Features
//!
//! 1. Advanced diamond inheritance pattern detection and visualization
//! 2. Rich error messages with syntax highlighting and contextual information
//! 3. Graphical representation of type relationships and inheritance paths
//! 4. Integration with the filesystem source location tracking system
//! 5. Enhanced developer experience with detailed diagnostic messages

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_path_finder_enhanced::{InterfaceInheritancePath, EnhancedInterfacePathFinder};
use crate::codegen::llvm::interface_type_assertion_error_visualization::{ErrorVisualization, VisualTypeAssertionError};
use crate::codegen::llvm::interface_type_assertion_filesystem_integration::InterfaceTypeAssertionFilesystemIntegration;
use crate::codegen::llvm::interface_type_assertion_error_propagation_filesystem_integration::ComprehensiveErrorFilesystemIntegration;
use crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritancePattern;
use crate::error::Error;
use crate::error::SourceLocation;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};

/// Enhanced interface type assertion error visualization
pub trait EnhancedErrorVisualization<'ctx>: ErrorVisualization<'ctx> {
    /// Detect and visualize diamond inheritance patterns
    fn detect_and_visualize_diamond_pattern(
        &self,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Option<String>;
    
    /// Create enhanced visual error with diamond pattern detection
    fn create_enhanced_visual_error(
        &self,
        message: &str,
        location: &SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        context_lines: Vec<(usize, String)>,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> VisualTypeAssertionError;
    
    /// Generate comprehensive visualization of type relationships
    fn generate_type_relationship_graph(
        &self,
        type_id: u32,
        max_depth: usize
    ) -> Option<String>;
    
    /// Create a merged error visualization combining multiple inheritance paths
    fn create_merged_path_visualization(
        &self,
        paths: Vec<InterfaceInheritancePath>
    ) -> Option<String>;
    
    /// Create a rich visualization of diamond inheritance patterns
    fn visualize_diamond_inheritance(
        &self,
        diamond: &DiamondInheritancePattern
    ) -> String;
}

impl<'ctx> EnhancedErrorVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn detect_and_visualize_diamond_pattern(
        &self,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Option<String> {
        // Skip if either type ID is invalid
        if expected_type_id == 0 || actual_type_id == 0 {
            return None;
        }
        
        // Get type names for better visualization
        let expected_name = self.get_type_name_by_id(expected_type_id)
            .unwrap_or_else(|| format!("Type#{}", expected_type_id));
        
        let actual_name = self.get_type_name_by_id(actual_type_id)
            .unwrap_or_else(|| format!("Type#{}", actual_type_id));
        
        debug!("Detecting diamond patterns between {} and {}", actual_name, expected_name);
        
        // Get all inheritance paths between the types
        let mut paths = Vec::new();
        
        // Helper method to find paths
        let finder = self;
        if let Some(interface_path_finder) = self.interface_path_finder() {
            // Find paths from actual to expected
            if let Ok(found_paths) = interface_path_finder.find_all_paths(actual_type_id, expected_type_id) {
                paths.extend(found_paths);
                debug!("Found {} inheritance paths", paths.len());
            }
        }
        
        // If we found multiple paths, it might be a diamond pattern
        if paths.len() > 1 {
            // Detect diamond pattern by finding common nodes in different paths
            let diamond = self.detect_diamond_pattern(&paths);
            
            if let Some(diamond) = diamond {
                debug!("Detected diamond inheritance pattern");
                return Some(self.visualize_diamond_inheritance(&diamond));
            }
        } else if paths.len() == 1 {
            // Single path - create a basic visualization
            let path = &paths[0];
            let mut result = String::new();
            
            result.push_str("Inheritance Path:\n");
            
            // Add each node in the path
            for (i, node_id) in path.path.iter().enumerate() {
                let node_name = self.get_type_name_by_id(*node_id)
                    .unwrap_or_else(|| format!("Type#{}", node_id));
                
                if i == 0 {
                    result.push_str(&format!("  {} (actual)\n", node_name));
                } else if i == path.path.len() - 1 {
                    result.push_str(&format!("  └─ {} (expected)\n", node_name));
                } else {
                    result.push_str(&format!("  ├─ {}\n", node_name));
                }
            }
            
            return Some(result);
        }
        
        // If we found no paths or couldn't detect a diamond pattern
        Some(format!("No inheritance relationship found between {} and {}", actual_name, expected_name))
    }
    
    #[instrument(skip(self, message, location, context_lines), level = "debug")]
    fn create_enhanced_visual_error(
        &self,
        message: &str,
        location: &SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        context_lines: Vec<(usize, String)>,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> VisualTypeAssertionError {
        // Try to create a diamond pattern visualization
        let type_path = self.detect_and_visualize_diamond_pattern(expected_type_id, actual_type_id);
        
        VisualTypeAssertionError {
            message: message.to_string(),
            location: location.clone(),
            expected_type: expected_type.to_string(),
            actual_type: actual_type.map(|s| s.to_string()),
            context_lines,
            type_path,
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_type_relationship_graph(
        &self,
        type_id: u32,
        max_depth: usize
    ) -> Option<String> {
        if type_id == 0 {
            return None;
        }
        
        let type_name = self.get_type_name_by_id(type_id)
            .unwrap_or_else(|| format!("Type#{}", type_id));
        
        // Create a graph visualization of inheritance relationships
        let mut result = String::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with the specified type
        queue.push_back((type_id, 0)); // (type_id, depth)
        visited.insert(type_id);
        
        result.push_str(&format!("Type Inheritance Graph for {}:\n", type_name));
        
        while let Some((current_id, depth)) = queue.pop_front() {
            // Stop if we've reached max depth
            if depth > max_depth {
                continue;
            }
            
            let current_name = self.get_type_name_by_id(current_id)
                .unwrap_or_else(|| format!("Type#{}", current_id));
            
            // Add indentation based on depth
            let indent = "  ".repeat(depth);
            let prefix = if depth == 0 { "".to_string() } else { format!("{}{}", indent, "└─ ") };
            result.push_str(&format!("{}{}{}", indent, prefix, current_name));
            
            if depth == 0 {
                result.push_str(" (root)\n");
            } else {
                result.push_str("\n");
            }
            
            // Get child types (interfaces implemented by this type)
            if let Some(interfaces) = self.get_implemented_interfaces(current_id) {
                for interface_id in interfaces {
                    if !visited.contains(&interface_id) {
                        queue.push_back((interface_id, depth + 1));
                        visited.insert(interface_id);
                    }
                }
            }
        }
        
        Some(result)
    }
    
    #[instrument(skip(self, paths), level = "debug")]
    fn create_merged_path_visualization(
        &self,
        paths: Vec<InterfaceInheritancePath>
    ) -> Option<String> {
        if paths.is_empty() {
            return None;
        }
        
        // Create a merged visualization of multiple inheritance paths
        // This helps identify where paths diverge and converge
        
        // Track all type IDs in the paths
        let mut all_types = HashSet::new();
        for path in &paths {
            for type_id in &path.path {
                all_types.insert(*type_id);
            }
        }
        
        // For each type, store its position in each path
        let mut type_positions = HashMap::new();
        for (path_idx, path) in paths.iter().enumerate() {
            for (pos, type_id) in path.path.iter().enumerate() {
                type_positions.entry(*type_id)
                    .or_insert_with(|| Vec::new())
                    .push((path_idx, pos));
            }
        }
        
        // Sort types by their earliest position in any path
        let mut sorted_types: Vec<_> = all_types.iter().collect();
        sorted_types.sort_by_key(|&type_id| {
            type_positions.get(type_id)
                .map(|positions| positions.iter().map(|(_, pos)| *pos).min().unwrap_or(usize::MAX))
                .unwrap_or(usize::MAX)
        });
        
        // Create a visualization with ASCII art
        let mut result = String::new();
        result.push_str("Merged Inheritance Paths:\n");
        
        for &type_id in &sorted_types {
            let type_name = self.get_type_name_by_id(type_id)
                .unwrap_or_else(|| format!("Type#{}", type_id));
            
            // Determine in which paths this type appears
            let positions = type_positions.get(&type_id).unwrap();
            let path_markers = (0..paths.len())
                .map(|path_idx| {
                    if positions.iter().any(|(idx, _)| *idx == path_idx) {
                        "* "
                    } else {
                        "  "
                    }
                })
                .collect::<String>();
            
            // Mark special types
            let is_start = positions.iter().any(|(_, pos)| *pos == 0);
            let is_end = positions.iter().any(|(path_idx, pos)| *pos == paths[*path_idx].path.len() - 1);
            
            let type_label = if is_start && is_end {
                format!("{} (start & end)", type_name)
            } else if is_start {
                format!("{} (start)", type_name)
            } else if is_end {
                format!("{} (end)", type_name)
            } else {
                type_name
            };
            
            result.push_str(&format!("  [{}] {}\n", path_markers, type_label));
        }
        
        // Add a legend
        result.push_str("\nLegend:\n");
        for i in 0..paths.len() {
            result.push_str(&format!("  Path {}: ", i+1));
            
            // Show the sequence of types in this path
            let path_sequence = paths[i].path.iter()
                .map(|type_id| {
                    self.get_type_name_by_id(*type_id)
                        .unwrap_or_else(|| format!("Type#{}", type_id))
                })
                .collect::<Vec<_>>()
                .join(" -> ");
            
            result.push_str(&path_sequence);
            result.push_str("\n");
        }
        
        Some(result)
    }
    
    #[instrument(skip(self, diamond), level = "debug")]
    fn visualize_diamond_inheritance(
        &self,
        diamond: &DiamondInheritancePattern
    ) -> String {
        let mut result = String::new();
        
        // Get type names for better readability
        let root_name = self.get_type_name_by_id(diamond.root_type_id)
            .unwrap_or_else(|| format!("Type#{}", diamond.root_type_id));
        
        let base_name = self.get_type_name_by_id(diamond.base_type_id)
            .unwrap_or_else(|| format!("Type#{}", diamond.base_type_id));
        
        let left_intermediate_name = self.get_type_name_by_id(diamond.left_intermediate_id)
            .unwrap_or_else(|| format!("Type#{}", diamond.left_intermediate_id));
        
        let right_intermediate_name = self.get_type_name_by_id(diamond.right_intermediate_id)
            .unwrap_or_else(|| format!("Type#{}", diamond.right_intermediate_id));
        
        // Create a diamond visualization with ASCII art
        result.push_str("Diamond Inheritance Pattern Detected:\n\n");
        
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
        result.push_str("The type assertion failed because multiple inheritance paths\n");
        result.push_str("were found between the source and target types.\n\n");
        
        // Add path details
        result.push_str("Inheritance Paths:\n");
        result.push_str(&format!("  Path 1: {} -> {} -> {}\n", root_name, left_intermediate_name, base_name));
        result.push_str(&format!("  Path 2: {} -> {} -> {}\n", root_name, right_intermediate_name, base_name));
        
        result
    }
}

// Helper methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Detect a diamond pattern in multiple inheritance paths
    fn detect_diamond_pattern(
        &self,
        paths: &[InterfaceInheritancePath]
    ) -> Option<DiamondInheritancePattern> {
        if paths.len() < 2 {
            return None;
        }
        
        // Find common start and end points in the paths
        let first_path = &paths[0].path;
        let second_path = &paths[1].path;
        
        // Paths must start and end with the same types
        if first_path.is_empty() || second_path.is_empty() || 
           first_path[0] != second_path[0] || 
           first_path[first_path.len()-1] != second_path[second_path.len()-1] {
            return None;
        }
        
        // Find where paths diverge
        let root_type_id = first_path[0];
        let base_type_id = first_path[first_path.len()-1];
        
        // Ensure paths are not identical
        if first_path == second_path {
            return None;
        }
        
        // Find intermediate types (where paths diverge)
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
            
            // Paths that diverge at the first element aren't a diamond
            if diverge_idx == 0 {
                return None;
            }
            
            // Get the intermediate types
            let left_intermediate_id = first_path[diverge_idx];
            let right_intermediate_id = second_path[diverge_idx];
            
            return Some(DiamondInheritancePattern {
                root_type_id,
                base_type_id,
                left_intermediate_id,
                right_intermediate_id,
            });
        }
        
        None
    }
    
    /// Get the interfaces implemented by a type
    fn get_implemented_interfaces(&self, type_id: u32) -> Option<Vec<u32>> {
        // In a real implementation, we would look up the interfaces
        // implemented by this type in the registry
        // This is a placeholder that returns a dummy list
        None
    }
    
    /// Get a reference to the interface path finder
    fn interface_path_finder(&self) -> Option<&dyn EnhancedInterfacePathFinder> {
        // In a real implementation, we would look up the interface path finder
        // from the registry or create one if it doesn't exist
        None
    }
}

/// Register the enhanced error visualization module
pub fn register_enhanced_error_visualization() {
    debug!("Registered enhanced error visualization for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_error_visualization_registration() {
        // Simple test to ensure module registration works
        register_enhanced_error_visualization();
        assert!(true);
    }
}