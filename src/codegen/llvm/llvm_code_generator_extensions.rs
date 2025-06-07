//! # LlvmCodeGenerator Extension Methods
//!
//! This module provides extension traits for LlvmCodeGenerator that add
//! missing functionality for source location tracking, symbol lookup,
//! type checking, and error message generation.

use std::collections::HashMap;
use std::path::Path;
use inkwell::values::BasicValueEnum;
use tracing::{debug, info, instrument, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{Error, SourceLocation};

/// Extension trait for source file and location management
pub trait SourceLocationExtensions {
    /// Initialize the source file cache for better error reporting
    fn init_source_file_cache(&mut self);
    
    /// Enhance a source location with additional context
    fn enhance_source_location(&self, location: &mut SourceLocation) -> Result<(), Error>;
    
    /// Add a search path for source files
    fn add_source_search_path(&mut self, path: &str);
}

/// Extension trait for symbol and type lookup operations
pub trait SymbolLookupExtensions<'ctx> {
    /// Look up a symbol by name
    fn lookup_symbol(&self, name: &str) -> Option<BasicValueEnum<'ctx>>;
    
    /// Check if an interface instance implements a type with propagation
    fn check_instance_of_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        interface_name: &str,
        target_type: &str
    ) -> Result<bool, Error>;
    
    /// Get a type by name from the type registry
    fn get_type_by_name(&self, type_name: &str) -> Option<BasicValueEnum<'ctx>>;
}

/// Extension trait for error handling and path operations
pub trait ErrorPathExtensions {
    /// Generate a path error message for failed type assertions
    fn generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        location: &str
    ) -> Result<String, Error>;
    
    /// Find alternative paths between interfaces
    fn find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_paths: usize
    ) -> Result<Vec<Vec<String>>, Error>;
}

// Default implementations for LlvmCodeGenerator
impl<'ctx> SourceLocationExtensions for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn init_source_file_cache(&mut self) {
        debug!("Initializing source file cache");
        // TODO: Initialize the filesystem integration if it hasn't been done yet
        // For now, this is a no-op but the method exists to satisfy trait requirements
    }
    
    #[instrument(skip(self, location), level = "debug")]
    fn enhance_source_location(&self, location: &mut SourceLocation) -> Result<(), Error> {
        debug!("Enhancing source location: {:?}", location);
        // TODO: Add file content and context to the source location
        // For now, this is a minimal implementation
        if location.file.is_none() {
            location.file = Some("unknown".to_string());
        }
        Ok(())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn add_source_search_path(&mut self, path: &str) {
        debug!("Adding source search path: {}", path);
        // TODO: Add path to the filesystem integration search paths
        // For now, this is a no-op but the method exists to satisfy trait requirements
    }
}

impl<'ctx> SymbolLookupExtensions<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn lookup_symbol(&self, name: &str) -> Option<BasicValueEnum<'ctx>> {
        debug!("Looking up symbol: {}", name);
        // TODO: Implement actual symbol lookup from symbol table
        // For now, return None to indicate symbol not found
        None
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn check_instance_of_with_propagation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        interface_name: &str,
        target_type: &str
    ) -> Result<bool, Error> {
        debug!("Checking instance relationship: {} -> {}", interface_name, target_type);
        // TODO: Implement actual type checking with propagation
        // For now, return a basic comparison
        Ok(interface_name == target_type)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_type_by_name(&self, type_name: &str) -> Option<BasicValueEnum<'ctx>> {
        debug!("Getting type by name: {}", type_name);
        // TODO: Implement actual type lookup from type registry
        // For now, return None to indicate type not found
        None
    }
}

impl<'ctx> ErrorPathExtensions for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        location: &str
    ) -> Result<String, Error> {
        debug!("Generating path error message: {} -> {} at {}", source_interface, target_interface, location);
        Ok(format!(
            "Type assertion failed: cannot convert from '{}' to '{}' at {}",
            source_interface, target_interface, location
        ))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_paths: usize
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding alternative paths: {} -> {} (max: {})", source_interface, target_interface, max_paths);
        // TODO: Implement actual path finding algorithm
        // For now, return empty paths to indicate no alternatives found
        Ok(vec![])
    }
}
