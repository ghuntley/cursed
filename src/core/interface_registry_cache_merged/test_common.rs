//! # Test Common Utilities for Interface Registry
//!
//! This module provides common utilities for testing interface registries,
//! including setup functions and test data.

use crate::core::interface_registry_extensions::{InterfaceRegistryExtension, ThreadSafeInterfaceExtensionRegistry};
use std::sync::{Arc, RwLock};

/// Set up a test registry with common test interfaces
pub fn setup_test_registry() -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
    create_test_registry()
}

/// Create a test registry with common test interfaces (alias for setup_test_registry)
pub fn create_test_registry() -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Add test interfaces
    {
        let mut reg = registry.write().unwrap();
        reg.register_interface("A");
        reg.register_interface("B");
        reg.register_interface("C");
        reg.register_interface("D");
        reg.register_interface("E");
        
        // Create test relationships
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        reg.register_extension("E", "D").unwrap();
    }
    
    registry
}

/// Test diamond pattern interfaces
pub fn setup_diamond_test_registry() -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Add test interfaces for diamond pattern
    {
        let mut reg = registry.write().unwrap();
        reg.register_interface("Base");
        reg.register_interface("Left");
        reg.register_interface("Right");
        reg.register_interface("Child");
        
        // Create diamond pattern: Base <- Left, Right <- Child
        reg.register_extension("Left", "Base").unwrap();
        reg.register_extension("Right", "Base").unwrap();
        reg.register_extension("Child", "Left").unwrap();
        reg.register_extension("Child", "Right").unwrap();
    }
    
    registry
}

/// Test deep hierarchy interfaces
pub fn setup_deep_hierarchy_registry() -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
    let registry = ThreadSafeInterfaceExtensionRegistry::new();
    
    // Add test interfaces for a deep hierarchy
    {
        let mut reg = registry.write().unwrap();
        reg.register_interface("Root");
        reg.register_interface("Level1A");
        reg.register_interface("Level1B");
        reg.register_interface("Level2A");
        reg.register_interface("Level2B");
        reg.register_interface("Level3A");
        reg.register_interface("Level3B");
        reg.register_interface("Leaf");
        
        // Create deep hierarchy
        reg.register_extension("Level1A", "Root").unwrap();
        reg.register_extension("Level1B", "Root").unwrap();
        reg.register_extension("Level2A", "Level1A").unwrap();
        reg.register_extension("Level2B", "Level1B").unwrap();
        reg.register_extension("Level3A", "Level2A").unwrap();
        reg.register_extension("Level3B", "Level2B").unwrap();
        reg.register_extension("Leaf", "Level3A").unwrap();
        reg.register_extension("Leaf", "Level3B").unwrap();
    }
    
    registry
}