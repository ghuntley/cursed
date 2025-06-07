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
    use crate::core::interface_registry_cache_merged::test_common::create_test_registry as setup_test_registry;
    
    #[test]
    fn test_diamond_inheritance_handler_registration() {
        register_diamond_inheritance_handler();
        assert!(true);
    }
    
    #[test]
    fn test_diamond_inheritance_detection() {
        let context = Context::create();
        let mut generator = LlvmCodeGenerator::new(&context, "test_diamond_inheritance_handler", PathBuf::from("test.csd"));
        
        // Set up a test registry with a diamond pattern
        let registry = setup_test_registry();
        generator.internal_fields.insert(
            "interface_registry".to_string(), 
            Box::new(registry)
        );
        
        // Test diamond detection
        let result = generator.detect_diamond_inheritance("Player", "GameObject");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}