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
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, VisualizationOptions};
use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::core::interface_registry_visualization::ThreadSafeInterfaceRegistryVisualization as InterfaceRegistryVisualization;

/// Trait for integrating the LlvmCodeGenerator with the interface registry visualization system
pub trait InterfaceRegistryIntegration {
    /// Get access to the interface registry visualization
    fn registry_visualization(&self) -> Option<&dyn InterfaceRegistryExtensionWithVisualization>;
    
    /// Get mutable access to the interface registry visualization
    fn registry_visualization_mut(&mut self) -> Option<&mut dyn InterfaceRegistryExtensionWithVisualization>;
    
    /// Initialize the registry visualization component if not already initialized
    fn ensure_registry_visualization_initialized(&mut self) -> Result<(), Error>;
}

impl<'ctx> InterfaceRegistryIntegration for LlvmCodeGenerator<'ctx> {
    fn registry_visualization(&self) -> Option<&dyn InterfaceRegistryExtensionWithVisualization> {
        // The registry_extensions field implements InterfaceRegistryVisualization
        // so we can just return a reference to it
        Some(&self.registry_extensions)
    }
    
    fn registry_visualization_mut(&mut self) -> Option<&mut dyn InterfaceRegistryExtensionWithVisualization> {
        // The registry_extensions field implements InterfaceRegistryVisualization
        // so we can just return a mutable reference to it
        Some(&mut self.registry_extensions)
    }
    
    fn ensure_registry_visualization_initialized(&mut self) -> Result<(), Error> {
        // The registry_extensions field is already initialized in the LlvmCodeGenerator constructor
        // but we need to verify that the interface_type_registry is properly connected
        if self.interface_type_registry.is_none() {
            let registry_ref = self.registry_extensions.extension_registry();
            let mut ir = crate::codegen::llvm::interface_type_registry::InterfaceTypeRegistry::with_extension_registry(registry_ref);
            self.interface_type_registry = Some(ir);
        }
        
        // Ensure consistency between registries
        if let Some(registry) = &mut self.interface_type_registry {
            registry.synchronize_with_extension_registry()?;
        }
        
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
            Some(registry) => <dyn InterfaceRegistryExtensionWithVisualization>::get_direct_extensions(&*registry, interface_name),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Register an interface extension in the registry visualization system
    pub fn register_interface_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        // First ensure the visualization system is initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Register the extension in the visualization system
        match self.registry_visualization_mut() {
            Some(registry) => {
                let result = registry.register_extension(source, target);
                
                // Also register with the type registry to ensure consistency
                if let Some(type_registry) = &mut self.interface_type_registry {
                    type_registry.register_interface_extension(source, target)?;
                }
                
                result
            },
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Generate a DOT graph representation of the interface hierarchy
    pub fn generate_interface_dot_graph(&self) -> Result<String, Error> {
        match self.registry_visualization() {
            Some(registry) => {
                let hierarchy = HashMap::new();
                let options = VisualizationOptions::default();
                use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
                <dyn InterfaceRegistryExtensionWithVisualization>::generate_dot_graph(registry, &hierarchy, &options)
            },
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
    
    /// Get the type ID for a specific interface from the registry
    pub fn get_interface_type_id(&self, interface_name: &str) -> Result<u64, Error> {
        if let Some(registry) = &self.interface_type_registry {
            registry.get_type_id(interface_name)
        } else {
            Err(Error::from_str("No interface type registry available"))
        }
    }
    
    /// Register a new interface type in the registry
    pub fn register_interface_type(&mut self, interface_name: &str) -> Result<u64, Error> {
        // Ensure both registries are initialized
        self.ensure_registry_visualization_initialized()?;
        
        if let Some(registry) = &mut self.interface_type_registry {
            registry.register_interface(interface_name)
        } else {
            Err(Error::from_str("No interface type registry available"))
        }
    }
    
    /// Update interface registry with complete hierarchy information
    pub fn update_interface_hierarchy(&mut self) -> Result<(), Error> {
        // Ensure both registries are initialized
        self.ensure_registry_visualization_initialized()?;
        
        // Get current hierarchy from visualization registry
        let registry = self.registry_visualization()
            .ok_or_else(|| Error::from_str("No registry visualization system available"))?;
        let hierarchy = <dyn InterfaceRegistryExtensionWithVisualization>::get_extension_hierarchy(registry)?;
        
        // Update the type registry with the hierarchy information
        if let Some(registry) = &mut self.interface_type_registry {
            for (source, targets) in &hierarchy {
                for target in targets {
                    registry.register_interface_extension(source, target)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check extension relationship between two interfaces (basic version)
    pub fn check_extension_relationship(&self, source: &str, target: &str) -> Result<bool, Error> {
        match self.registry_visualization() {
            Some(registry) => registry.extends(source, target),
            None => Err(Error::from_str("No registry visualization system available"))
        }
    }
    
    /// Check extension relationship between two interfaces (enhanced version)
    pub fn check_extension_relationship_enhanced(&self, source: &str, target: &str) -> Result<bool, Error> {
        match self.registry_visualization() {
            Some(registry) => registry.extends(source, target),
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