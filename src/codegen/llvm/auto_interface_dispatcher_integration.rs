//! Integration module for auto interface dispatcher
//!
//! This module ensures proper initialization and integration of the
//! auto interface dispatcher functionality with the rest of the compiler.
//! It resolves the dependency issues by establishing relationships between
//! various interface-related components.

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::auto_interface_dispatcher::{AutoInterfaceDispatcher, AutoInterfaceDispatchExtension};
use crate::codegen::llvm::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
use tracing::{debug, info, instrument};

/// Trait for integrating the auto interface dispatcher with the compiler
pub trait AutoInterfaceDispatcherIntegration<'ctx> {
    /// Initialize the auto interface dispatcher and ensure all dependencies are properly set up
    fn init_auto_interface_dispatcher_integration(&mut self) -> Result<(), Error>;
    
    /// Find and register all struct methods that implement a specific interface
    fn discover_and_register_interface_implementations(
        &mut self,
        struct_name: &str,
        interface_name: &str,
    ) -> Result<(), Error>;
}

impl<'ctx> AutoInterfaceDispatcherIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn init_auto_interface_dispatcher_integration(&mut self) -> Result<(), Error> {
        debug!("Initializing auto interface dispatcher integration");
        
        // First, ensure the interface manager is initialized
        if self.interface_manager.is_none() {
            debug!("Creating interface manager for auto dispatcher");
            self.init_auto_interface_dispatcher()?;
        }
        
        // Additional initialization can be added here as needed
        // For example, registering built-in interfaces or common implementations
        
        info!("Auto interface dispatcher integration initialized successfully");
        Ok(())
    }
    
    #[instrument(skip(self), fields(struct_name = %struct_name, interface_name = %interface_name))]
    fn discover_and_register_interface_implementations(
        &mut self,
        struct_name: &str,
        interface_name: &str,
    ) -> Result<(), Error> {
        debug!("Discovering and registering implementations for {} implementing {}", struct_name, interface_name);
        
        // Ensure the auto interface dispatcher is initialized
        self.init_auto_interface_dispatcher_integration()?;
        
        // Use the auto registration functionality to register struct methods
        self.auto_register_struct_methods(struct_name, interface_name)?;
        
        info!("Successfully registered interface implementations");
        Ok(())
    }
}

/// Register and initialize the auto interface dispatcher integration module
pub fn register_auto_interface_dispatcher_integration() {
    debug!("Registering auto interface dispatcher integration module");
    // This would be called during compiler initialization
}