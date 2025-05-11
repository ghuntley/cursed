//! # Interface Registry Visualization Implementation
//!
//! This module provides the implementation of the InterfaceRegistryVisualization trait
//! for the ThreadSafeInterfaceExtensionRegistry type. It enables visualizing the
//! inheritance hierarchy between interfaces and finding paths between different interfaces.
//!
//! This implementation bridges the gap between the interface registry and the type assertion
//! path visualization system, allowing for better error messages and debugging support.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::error::Error;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;

/// Implementation of InterfaceRegistryExtensionWithVisualization for ThreadSafeInterfaceExtensionRegistry
impl InterfaceRegistryExtensionWithVisualization for ThreadSafeInterfaceExtensionRegistry {
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, HashSet<String>>, Error> {
        // Use the existing method that already does error handling
        self.get_extension_hierarchy()
    }
    
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        // Use the existing method that already does error handling
        self.get_direct_extensions(interface)
    }
    
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        // Use existing get_extension_hierarchy to build the direct implementors
        let hierarchy = self.get_extension_hierarchy()?;
        
        // Get interfaces that directly extend the given interface
        let mut implementors = HashSet::new();
        
        for (impl_interface, extensions) in hierarchy {
            if extensions.contains(interface) {
                implementors.insert(impl_interface);
            }
        }
        
        if implementors.is_empty() {
            Ok(None)
        } else {
            Ok(Some(implementors))
        }
    }
    
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        // Use the existing method that already does error handling
        self.get_all_interfaces()
    }
}

/// Register the interface registry visualization implementation
pub fn register_interface_registry_visualization_implementation() {
    trace!("Interface registry visualization implementation registered");
}