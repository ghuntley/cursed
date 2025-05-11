//! # Interface Type Assertion Path Visualization Adapter
//!
//! This module provides adapter functionality to ensure proper method exposure between
//! the interface type assertion path visualization traits. It ensures trait compatibility
//! by implementing the proper forwarding methods and handle type conversions.
//!
//! The adapter pattern is used to bridge the gap between the base path visualization trait
//! and the enhanced version, ensuring that all methods are properly exposed and accessible
//! from the code generator.

use std::sync::Arc;
use std::collections::{HashMap, HashSet};

use inkwell::values::BasicValueEnum;
use tracing::{debug, instrument};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use crate::error::Error;
use crate::error::SourceLocation;

/// Trait adapter to ensure proper method exposure between interface type assertion path visualization traits
pub trait InterfaceTypeAssertionPathVisualizationAdapter<'ctx>: InterfaceTypeAssertionPathVisualization<'ctx> + EnhancedInterfaceTypeAssertionPathVisualization<'ctx> {
    /// Forward interface path finding to the appropriate implementation
    fn forward_find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error>;
    
    /// Forward visualization to the appropriate implementation
    fn forward_visualize_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error>;
    
    /// Forward DOT graph generation to the appropriate implementation
    fn forward_generate_interface_hierarchy_dot(&self) -> Result<String, Error>;
    
    /// Forward error message generation to the appropriate implementation
    fn forward_generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error>;
    
    /// Forward alternative path finding to the appropriate implementation
    fn forward_find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error>;
    
    /// Forward type assertion compilation to the appropriate implementation
    fn forward_compile_type_assertion_with_path_visualization(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Ensure the adapter has proper registry access
    fn ensure_registry_access(&self) -> Result<&dyn InterfaceRegistryExtensionWithVisualization, Error>;
}

/// Define a generic implementation of the adapter for any code generator that implements both traits
impl<'ctx, T> InterfaceTypeAssertionPathVisualizationAdapter<'ctx> for T 
where 
    T: InterfaceTypeAssertionPathVisualization<'ctx> + EnhancedInterfaceTypeAssertionPathVisualization<'ctx>
{
    #[instrument(skip(self), level = "debug")]
    fn forward_find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error> {
        debug!("Forwarding interface path finding: {} -> {}", source_interface, target_interface);
        
        // Call the base trait implementation with proper error propagation
        InterfaceTypeAssertionPathVisualization::find_interface_path(
            self, 
            source_interface, 
            target_interface
        )
    }
    
    #[instrument(skip(self), level = "debug")]
    fn forward_visualize_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        debug!("Forwarding interface path visualization: {} -> {}", source_interface, target_interface);
        
        // Call the enhanced implementation for better visualization
        EnhancedInterfaceTypeAssertionPathVisualization::visualize_interface_path_enhanced(
            self, 
            source_interface, 
            target_interface
        )
    }
    
    #[instrument(skip(self), level = "debug")]
    fn forward_generate_interface_hierarchy_dot(&self) -> Result<String, Error> {
        debug!("Forwarding interface hierarchy DOT generation");
        
        // Call the enhanced implementation for better visualization
        EnhancedInterfaceTypeAssertionPathVisualization::generate_interface_hierarchy_dot_enhanced(self)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn forward_generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        debug!("Forwarding path error message generation: {} -> {}", source_interface, target_interface);
        
        // Call the enhanced implementation for better error messages
        EnhancedInterfaceTypeAssertionPathVisualization::generate_path_error_message_enhanced(
            self, 
            source_interface, 
            target_interface, 
            source_location
        )
    }
    
    #[instrument(skip(self), level = "debug")]
    fn forward_find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Forwarding alternative path finding: {} -> {}", source_interface, target_interface);
        
        // Call the enhanced implementation for better alternative path finding
        EnhancedInterfaceTypeAssertionPathVisualization::find_alternative_paths_enhanced(
            self, 
            source_interface, 
            target_interface, 
            max_alternatives
        )
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn forward_compile_type_assertion_with_path_visualization(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Forwarding type assertion compilation: {}", type_assertion.token_literal());
        
        // Call the enhanced implementation for better type assertion compilation
        EnhancedInterfaceTypeAssertionPathVisualization::compile_type_assertion_with_path_visualization_enhanced(
            self, 
            type_assertion
        )
    }
    
    #[instrument(skip(self), level = "debug")]
    fn ensure_registry_access(&self) -> Result<&dyn InterfaceRegistryExtensionWithVisualization, Error> {
        debug!("Ensuring registry access");
        
        // Get the registry through the enhanced trait
        Ok(EnhancedInterfaceTypeAssertionPathVisualization::interface_registry(self))
    }
}

/// Register the adapter with the compiler
pub fn register_interface_type_assertion_path_visualization_adapter() {
    tracing::trace!("Interface type assertion path visualization adapter registered");
}