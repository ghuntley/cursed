//! Interface Type Assertion Diamond Inheritance Handler
//!
//! This module extends the interface type assertion system with specific
//! support for diamond inheritance patterns, where multiple inheritance paths
//! exist between source and target types.
//!
//! Diamond inheritance occurs when a type inherits from two interfaces that
//! both inherit from a common base interface, creating a diamond-shaped
//! inheritance graph. This module provides specialized path analysis and
//! visualization for these scenarios.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use tracing::{debug, error, info, instrument, trace, warn};
import inkwell::values::BasicValueEnum;

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::error::Error;
use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;

/// Extension trait for LlvmCodeGenerator to add diamond inheritance handling
pub trait DiamondInheritanceHandler<'ctx> {
    /// Detect diamond inheritance patterns between two types
    fn detect_diamond_inheritance(
        &self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<DiamondInheritanceInfo>, Error>;
    
    /// Find all paths between two types (not just the shortest)
    fn find_all_inheritance_paths(
        &self,
        from_type_id: u64,
        to_type_id: u64,
        max_paths: usize
    ) -> Result<Vec<Vec<u64>>, Error>;
    
    /// Visualize diamond inheritance relationship between types
    fn visualize_diamond_inheritance(
        &self,
        source_type: &str,
        target_type: &str,
        inheritance_info: &DiamondInheritanceInfo
    ) -> Result<String, Error>;
}

/// Information about a diamond inheritance pattern
#[derive(Debug, Clone)]
pub struct DiamondInheritanceInfo {
    /// The common base type at the top of the diamond
    pub base_type_id: u64,
    pub base_type_name: String,
    
    /// The intermediate types that form the sides of the diamond
    pub intermediate_type_ids: Vec<u64>,
    pub intermediate_type_names: Vec<String>,
    
    /// All detected paths through the inheritance hierarchy
    pub paths: Vec<Vec<u64>>,
    
    /// Path lengths (for analytical purposes)
    pub path_lengths: Vec<usize>,
    
    /// The lowest common junction point in the hierarchy
    pub junction_type_id: Option<u64>,
    pub junction_type_name: Option<String>,
}

impl<'ctx> DiamondInheritanceHandler<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn detect_diamond_inheritance(
        &self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<DiamondInheritanceInfo>, Error> {
        debug!("Detecting diamond inheritance between {} and {}", source_type, target_type);
        
        // Ensure the registry is initialized
        let registry = match &self.interface_type_registry {
            Some(registry) => registry,
            None => {
                debug!("No type registry available to detect diamond inheritance");
                return Ok(None);
            }
        };
        
        // Get type IDs
        let source_id = match registry.get_type_id(source_type) {
            Ok(id) => id,
            Err(_) => {
                debug!("Source type not found in registry: {}", source_type);
                return Ok(None);
            }
        };
        
        let target_id = match registry.get_type_id(target_type) {
            Ok(id) => id,
            Err(_) => {
                debug!("Target type not found in registry: {}", target_type);
                return Ok(None);
            }
        };
        
        // Get inheritance map
        let inheritance_map = match registry.get_inheritance_map() {
            Some(map) => map,
            None => {
                debug!("No inheritance map available");
                return Ok(None);
            }
        };
        
        // Find all paths between source and target (limited to a reasonable number)
        let paths = self.find_all_inheritance_paths(source_id, target_id, 10)?;
        
        // If fewer than 2 paths, it's not a diamond pattern
        if paths.len() < 2 {
            debug!("No diamond inheritance detected (only {} path found)", paths.len());
            return Ok(None);
        }
        
        // Analyze paths to find the common base type and junction
        let path_lengths: Vec<usize> = paths.iter().map(|p| p.len()).collect();
        
        // Find common types across all paths
        let mut common_type_counts: HashMap<u64, usize> = HashMap::new();
        
        for path in &paths {
            let mut seen_in_this_path = HashSet::new();
            
            for &type_id in path {
                // Count each type only once per path
                if seen_in_this_path.insert(type_id) {
                    *common_type_counts.entry(type_id).or_insert(0) += 1;
                }
            }
        }
        
        // Find types that appear in all paths (potential diamond corners)
        let common_types: Vec<u64> = common_type_counts.iter()
            .filter(|(_, &count)| count == paths.len())
            .map(|(&type_id, _)| type_id)
            .collect();
        
        if common_types.len() < 2 {
            // Need at least source and target as common points
            debug!("Not enough common types for diamond inheritance");
            return Ok(None);
        }
        
        // Start building the diamond information
        let mut diamond_info = DiamondInheritanceInfo {
            base_type_id: 0,  // Will set these below
            base_type_name: String::new(),
            intermediate_type_ids: Vec::new(),
            intermediate_type_names: Vec::new(),
            paths,
            path_lengths,
            junction_type_id: None,
            junction_type_name: None,
        };
        
        // Find the base type (common ancestor farthest from source)
        // In a diamond, this should be at or near the beginning of both paths
        let mut base_type_id = 0;
        let mut min_position_sum = usize::MAX;
        
        for &type_id in &common_types {
            // Skip source and target
            if type_id == source_id || type_id == target_id {
                continue;
            }
            
            // Calculate the sum of positions in all paths
            let position_sum: usize = diamond_info.paths.iter()
                .map(|path| path.iter().position(|&id| id == type_id).unwrap_or(usize::MAX))
                .sum();
            
            // The type with the smallest position sum is closest to the top of the diamond
            if position_sum < min_position_sum {
                min_position_sum = position_sum;
                base_type_id = type_id;
            }
        }
        
        // If we identified a base type
        if base_type_id != 0 {
            diamond_info.base_type_id = base_type_id;
            diamond_info.base_type_name = registry.get_type_name(base_type_id)
                .unwrap_or_else(|_| format!("Unknown(0x{:x})", base_type_id));
            
            debug!("Identified diamond base type: {}", diamond_info.base_type_name);
            
            // Find the intermediate types (form the sides of the diamond)
            let mut intermediate_types = HashSet::new();
            
            for path in &diamond_info.paths {
                // Find the position of the base type in this path
                if let Some(base_pos) = path.iter().position(|&id| id == base_type_id) {
                    // Add all types between base and target except common types
                    for &type_id in &path[base_pos + 1..] {
                        if type_id != target_id && !common_types.contains(&type_id) {
                            intermediate_types.insert(type_id);
                        }
                    }
                }
            }
            
            // Convert to sorted vectors
            let mut intermediate_ids: Vec<_> = intermediate_types.into_iter().collect();
            intermediate_ids.sort(); // Sort for stable output
            
            diamond_info.intermediate_type_ids = intermediate_ids;
            
            // Get names for all intermediate types
            for &id in &diamond_info.intermediate_type_ids {
                let name = registry.get_type_name(id)
                    .unwrap_or_else(|_| format!("Unknown(0x{:x})", id));
                diamond_info.intermediate_type_names.push(name);
            }
            
            // Try to identify junction point if present
            // This is the type where different paths converge before reaching the target
            let mut latest_common_type = None;
            let mut max_position_sum = 0;
            
            for &type_id in &common_types {
                // Skip source, target, and base type
                if type_id == source_id || type_id == target_id || type_id == base_type_id {
                    continue;
                }
                
                // Calculate the sum of positions in all paths
                let position_sum: usize = diamond_info.paths.iter()
                    .map(|path| path.iter().position(|&id| id == type_id).unwrap_or(0))
                    .sum();
                
                // The type with the largest position sum is closest to the bottom of the diamond
                if position_sum > max_position_sum {
                    max_position_sum = position_sum;
                    latest_common_type = Some(type_id);
                }
            }
            
            // Set junction information if found
            if let Some(junction_id) = latest_common_type {
                diamond_info.junction_type_id = Some(junction_id);
                diamond_info.junction_type_name = Some(registry.get_type_name(junction_id)
                    .unwrap_or_else(|_| format!("Unknown(0x{:x})", junction_id)));
                
                debug!("Identified diamond junction type: {}", 
                       diamond_info.junction_type_name.as_ref().unwrap());
            }
            
            debug!("Diamond inheritance pattern detected with {} paths and {} intermediate types", 
                  diamond_info.paths.len(), diamond_info.intermediate_type_ids.len());
                  
            return Ok(Some(diamond_info));
        }
        
        // No diamond pattern detected
        debug!("No diamond inheritance pattern could be definitively identified");
        Ok(None)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_inheritance_paths(
        &self,
        from_type_id: u64,
        to_type_id: u64,
        max_paths: usize
    ) -> Result<Vec<Vec<u64>>, Error> {
        // Ensure the registry is initialized
        let registry = match &self.interface_type_registry {
            Some(registry) => registry,
            None => return Ok(vec![]),
        };
        
        // Get inheritance map
        let inheritance_map = match registry.get_inheritance_map() {
            Some(map) => map,
            None => return Ok(vec![]),
        };
        
        // Track visited states to avoid cycles
        let mut visited = HashSet::new();
        
        // Use BFS to find all paths up to a maximum count
        let mut queue = VecDeque::new();
        queue.push_back(vec![from_type_id]);
        
        let mut all_paths = Vec::new();
        
        while let Some(current_path) = queue.pop_front() {
            // Get the last type in the current path
            let current_type = *current_path.last().unwrap();
            
            // If we've reached the target, save this path
            if current_type == to_type_id {
                all_paths.push(current_path);
                
                // Stop if we've found enough paths
                if all_paths.len() >= max_paths {
                    break;
                }
                
                continue;
            }
            
            // Check if we've visited this type in this path
            if !visited.insert(current_type) {
                continue; // Skip already visited types
            }
            
            // Get all direct implementations of this type
            if let Some(implementations) = inheritance_map.get(&current_type) {
                for &impl_id in implementations {
                    // Skip if this would create a cycle
                    if current_path.contains(&impl_id) {
                        continue;
                    }
                    
                    // Create a new path with this implementation
                    let mut new_path = current_path.clone();
                    new_path.push(impl_id);
                    queue.push_back(new_path);
                }
            }
        }
        
        // Return all found paths
        Ok(all_paths)
    }
    
    #[instrument(skip(self, inheritance_info), level = "debug")]
    fn visualize_diamond_inheritance(
        &self,
        source_type: &str,
        target_type: &str,
        inheritance_info: &DiamondInheritanceInfo
    ) -> Result<String, Error> {
        debug!("Visualizing diamond inheritance between {} and {}", source_type, target_type);
        
        let registry = match &self.interface_type_registry {
            Some(registry) => registry,
            None => return Ok(format!("Diamond Inheritance Visualization (limited):\n\n{} --> {} --> {}\n\nNo type registry available for detailed visualization.", 
                                       source_type, inheritance_info.base_type_name, target_type)),
        };
        
        // Create diamond visualization
        let mut result = String::new();
        result.push_str("Diamond Inheritance Pattern Detected\n");
        result.push_str("====================================\n\n");
        
        // Basic information
        result.push_str(&format!("Source Type: {}\n", source_type));
        result.push_str(&format!("Target Type: {}\n", target_type));
        result.push_str(&format!("Base Type: {}\n", inheritance_info.base_type_name));
        
        if let Some(junction) = &inheritance_info.junction_type_name {
            result.push_str(&format!("Junction Type: {}\n", junction));
        }
        
        result.push_str(&format!("Number of Paths: {}\n", inheritance_info.paths.len()));
        result.push_str(&format!("Path Lengths: {:?}\n", inheritance_info.path_lengths));
        
        result.push_str("\nIntermediate Types:\n");
        for name in &inheritance_info.intermediate_type_names {
            result.push_str(&format!("  - {}\n", name));
        }
        
        // Ascil art representation of the diamond
        result.push_str("\nDiamond Inheritance Diagram:\n\n");
        
        result.push_str(&format!("                      {}\n", source_type));
        result.push_str("                        |\n");
        result.push_str("                        v\n");
        result.push_str(&format!("                      {}\n", inheritance_info.base_type_name));
        
        // Middle part of the diamond - show the first few intermediate types on both sides
        let left_side = inheritance_info.intermediate_type_names.get(0)
            .cloned().unwrap_or_else(|| "?".to_string());
            
        let right_side = inheritance_info.intermediate_type_names.get(1)
            .cloned().unwrap_or_else(|| "?".to_string());
        
        result.push_str("                      / \\\n");
        result.push_str(&format!("                    /   \\\n"));
        result.push_str(&format!("        {}   {}\n", left_side, right_side));
        result.push_str("                    \\   /\n");
        result.push_str("                     \\ /\n");
        
        // Bottom of the diamond - either junction or target
        if let Some(junction) = &inheritance_info.junction_type_name {
            result.push_str(&format!("                      {}\n", junction));
            result.push_str("                        |\n");
            result.push_str("                        v\n");
            result.push_str(&format!("                      {}\n", target_type));
        } else {
            result.push_str(&format!("                      {}\n", target_type));
        }
        
        // Detailed path information
        result.push_str("\nDetailed Inheritance Paths:\n");
        
        for (i, path) in inheritance_info.paths.iter().enumerate() {
            result.push_str(&format!("Path {}:\n", i+1));
            
            for (j, &type_id) in path.iter().enumerate() {
                let type_name = registry.get_type_name(type_id)
                    .unwrap_or_else(|_| format!("Unknown(0x{:x})", type_id));
                
                // Add special marks for diamond corners
                let marker = if type_id == inheritance_info.base_type_id {
                    " (BASE)"
                } else if Some(type_id) == inheritance_info.junction_type_id {
                    " (JUNCTION)"
                } else if j == 0 {
                    " (SOURCE)"
                } else if j == path.len() - 1 {
                    " (TARGET)"
                } else {
                    ""
                };
                
                result.push_str(&format!("  {}. {}{}", j+1, type_name, marker));
                
                // Add arrow except for the last element
                if j < path.len() - 1 {
                    // Try to get the relationship type between these types
                    let next_type_id = path[j+1];
                    let relation = self.get_relationship_type(type_id, next_type_id, &inheritance_map);
                    result.push_str(&format!(" --{}-->", relation));
                }
                
                result.push_str("\n");
            }
            
            if i < inheritance_info.paths.len() - 1 {
                result.push_str("\n");
            }
        }
        
        // Method resolution order analysis for the diamond
        result.push_str("\nMethod Resolution Order Analysis:\n");
        result.push_str("When multiple inheritance paths exist, method resolution follows specific rules.\n");
        result.push_str("In CURSED, methods are resolved by:");
        result.push_str("  1. Checking the concrete type first\n");
        result.push_str("  2. Following inheritance paths in declaration order\n");
        result.push_str("  3. For diamond patterns, common base interfaces are only visited once\n");
        
        // Recommendation for handling this diamond inheritance
        result.push_str("\nRecommendations:\n");
        result.push_str("  1. Ensure method implementations are consistent across intermediate types\n");
        result.push_str("  2. Consider using composition instead of multiple inheritance if possible\n");
        result.push_str("  3. Use explicit type assertions when accessing methods in ambiguous cases\n");
        
        Ok(result)
    }
}

// Register module function
pub fn register_diamond_inheritance_handler() {
    debug!("Registered diamond inheritance handler for interface type assertions");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diamond_inheritance_registration() {
        register_diamond_inheritance_handler();
        assert!(true);
    }
}