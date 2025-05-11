//! # Interface Type Registry Common Utilities
//!
//! This module provides common utilities and helper functions for the interface type registry system.
//! It centralizes duplicate definitions that were previously spread across multiple files.

use std::boxed::Box;
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder;
use crate::codegen::llvm::interface_type_registry_helpers::TypeNameRegistry;
use crate::InterfaceTypeRegistry;
use crate::error::Error;

/// Get the interface path finder from the code generator
/// 
/// This function provides a single implementation used by all modules that need to access
/// the interface path finder.
pub fn get_interface_path_finder_impl<'ctx, 'a>(codegen: &'a LlvmCodeGenerator<'ctx>) -> Option<Box<dyn EnhancedInterfacePathFinder + 'a>> {
    // Check for path finder in the internal fields
    if codegen.internal_fields.contains_key("interface_path_finder") {
        if let Some(finder) = codegen.internal_fields.get("interface_path_finder") {
            // Try to access the path finder through the Any downcasting mechanism
            if let Some(enhanced_finder) = finder.downcast_ref::<Box<dyn EnhancedInterfacePathFinder>>() {
                // We need to clone the Box to return it
                // In a full implementation, we might want to use Arc instead
                return Some(enhanced_finder.box_clone());
            }
        }
    }
    None
}

/// Check if a type implements an interface
/// 
/// This function provides a single implementation used by all modules that need to check
/// type implementation relationships.
pub fn type_implements_impl<'ctx>(codegen: &LlvmCodeGenerator<'ctx>, concrete_type_id: u32, interface_type_id: u32) -> Option<bool> {
    // First check if we have a registry
    if let Some(registry) = get_interface_registry_impl(codegen) {
        // Try to check implementation relationship
        if let Ok(implements) = registry.type_implements_by_id(concrete_type_id, interface_type_id) {
            return Some(implements);
        }
    }
    
    // Alternative implementatio, checking in internal fields
    let key = format!("implements_{}_{}", concrete_type_id, interface_type_id);
    if let Some(value) = codegen.internal_fields.get(&key) {
        if let Some(implements) = value.downcast_ref::<bool>() {
            return Some(*implements);
        }
    }
    
    None
}

/// Get a reference to the interface registry
/// 
/// This function provides a single implementation used by all modules that need access
/// to the interface type registry.
pub fn get_interface_registry_impl<'ctx, 'a>(codegen: &'a LlvmCodeGenerator<'ctx>) -> Option<&'a dyn InterfaceTypeRegistry> {
    // Check for registry in the internal fields
    if codegen.internal_fields.contains_key("interface_registry") {
        if let Some(registry) = codegen.internal_fields.get("interface_registry") {
            // Try to access the registry through the Any downcasting mechanism
            if let Some(type_registry) = registry.downcast_ref::<Box<dyn InterfaceTypeRegistry>>() {
                return Some(type_registry.as_ref());
            }
        }
    }
    None
}

/// Get a mutable reference to the interface registry
/// 
/// This function provides a single implementation used by all modules that need mutable access
/// to the interface type registry.
pub fn get_interface_registry_mut_impl<'ctx, 'a>(codegen: &'a mut LlvmCodeGenerator<'ctx>) -> Option<&'a mut dyn InterfaceTypeRegistry> {
    // Check for registry in the internal fields
    if codegen.internal_fields.contains_key("interface_registry") {
        if let Some(registry) = codegen.internal_fields.get_mut("interface_registry") {
            // Try to access the registry through the Any downcasting mechanism
            if let Some(type_registry) = registry.downcast_mut::<Box<dyn InterfaceTypeRegistry>>() {
                return Some(type_registry.as_mut());
            }
        }
    }
    None
}

/// Detect diamond inheritance in the type hierarchy
/// 
/// This function provides a single implementation of diamond inheritance detection
/// used across all modules.
pub fn detect_diamond_inheritance_impl<'ctx>(codegen: &LlvmCodeGenerator<'ctx>, concrete_type_id: u32, interface_type_id: u32) -> Result<bool, Error> {
    // Get available paths between the types
    use crate::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritanceDetection;
    
    // Find all paths and check if there's more than one
    match codegen.find_all_inheritance_paths(concrete_type_id, interface_type_id) {
        Ok(paths) => Ok(paths.len() > 1),
        Err(err) => Err(err)
    }
}

/// Get the type name by ID with standardized error handling
/// 
/// This function provides a consistent implementation for looking up type names by ID
/// and converting to Result type for better error handling.
pub fn get_type_name_by_id_impl<'ctx>(codegen: &LlvmCodeGenerator<'ctx>, type_id: u32) -> Result<String, Error> {
    // Use TypeNameRegistry trait to get the type name
    TypeNameRegistry::get_type_name_by_id(codegen, type_id)
        .ok_or_else(|| Error::Compilation(format!("Could not find type name for ID {}", type_id)))
}