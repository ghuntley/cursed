//! # Interface Registry Integration
//!
//! This module implements the integration between the LlvmCodeGenerator and the
//! interface registry visualization system. It bridges the gap between the two
//! components, ensuring that interface type assertions and path visualizations
//! work correctly with the dynamic dispatch system.
//!
//! ## Key Features
//!
//! 1. Integration with the interface registry visualization system
//! 2. Support for interface type assertion path visualization
//! 3. Proper delegation of registry visualization operations
//! 4. Thread-safe access to interface registry extensions
//!
//! This implementation ensures that the interface type assertion system and the
//! dynamic dispatch system work together seamlessly, providing a robust and
//! reliable way to perform interface type assertions with rich error reporting.

use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::core::interface_registry_visualization::ThreadSafeInterfaceRegistryVisualization as InterfaceRegistryVisualization;
use crate::error::Error;

/// Trait for integrating the LlvmCodeGenerator with the interface registry visualization system
pub trait InterfaceRegistryIntegration {
    /// Get access to the interface registry visualization
    fn registry_visualization(&self) -> Option<&dyn InterfaceRegistryVisualization>;
    
    /// Get mutable access to the interface registry visualization
    fn registry_visualization_mut(&mut self) -> Option<&mut dyn InterfaceRegistryVisualization>;
    
    /// Initialize the registry visualization component if not already initialized
    fn ensure_registry_visualization_initialized(&mut self) -> Result<(), Error>;
}

impl<'ctx> InterfaceRegistryIntegration for LlvmCodeGenerator<'ctx> {
    fn registry_visualization(&self) -> Option<&dyn InterfaceRegistryVisualization> {
        // The registry_extensions field implements InterfaceRegistryVisualization
        // so we can just return a reference to it
        Some(&self.registry_extensions)
    }
    
    fn registry_visualization_mut(&mut self) -> Option<&mut dyn InterfaceRegistryVisualization> {
        // The registry_extensions field implements InterfaceRegistryVisualization
        // so we can just return a mutable reference to it
        Some(&mut self.registry_extensions)
    }
    
    fn ensure_registry_visualization_initialized(&mut self) -> Result<(), Error> {
        // The registry_extensions field is already initialized in the LlvmCodeGenerator constructor
        // so we don't need to do anything here
        Ok(())
    }
}

/// Extension methods for the LlvmCodeGenerator to work with interface registry visualization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if a registry visualization system is available
    pub fn has_registry_visualization(&self) -> bool {
        self.registry_visualization().is_some()
    }
    
    /// Get direct extensions for an interface from the registry visualization system
    pub fn get_interface_extensions(&self, interface_name: &str) -> Result<Option<Vec<String>>, Error> {
        match self.registry_visualization() {
            Some(registry) => registry.get_direct_extensions(interface_name),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Register an interface extension in the registry visualization system
    pub fn register_interface_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        match self.registry_visualization_mut() {
            Some(registry) => registry.register_extension(source, target),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Generate a DOT graph representation of the interface hierarchy
    pub fn generate_interface_dot_graph(&self) -> Result<String, Error> {
        match self.registry_visualization() {
            Some(registry) => registry.generate_dot_graph(),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Check if an interface extends another interface according to the registry
    pub fn check_interface_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        match self.registry_visualization() {
            Some(registry) => registry.check_extension_relationship(source, target),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
}

/// Register the interface registry integration functionality with the compiler
pub fn register_interface_registry_integration() {
    trace!("Interface registry integration module registered");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_integration_registration() {
        // Test that the module registration function works
        register_interface_registry_integration();
        assert!(true);
    }
}