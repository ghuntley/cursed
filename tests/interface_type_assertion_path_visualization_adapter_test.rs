//! # Tests for Interface Type Assertion Path Visualization Adapter
//!
//! This module tests the adapter that ensures proper method exposure between the
//! interface type assertion path visualization traits.

use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};

// Import the modules we need to test
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_adapter::InterfaceTypeAssertionPathVisualizationAdapter;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use cursed::error::Error;

// Import the common test utilities
use crate::common;

#[path = "common.rs"]
mod common;

/// Set up a fixture for tests with a populated interface hierarchy
fn setup_interface_hierarchy() -> Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>> {
    // Initialize tracing for this test
    common::tracing::setup();
    
    let registry = Arc::new(RwLock::new(ThreadSafeInterfaceExtensionRegistry::new()));
    
    // Set up a simple diamond inheritance pattern
    // A -> B -> D
    // |         ^
    // v         |
    // C ---------+
    
    registry.write().unwrap().register_extension("A", "B").unwrap();
    registry.write().unwrap().register_extension("A", "C").unwrap();
    registry.write().unwrap().register_extension("B", "D").unwrap();
    registry.write().unwrap().register_extension("C", "D").unwrap();
    
    // Add some isolated interfaces for testing error cases
    registry.write().unwrap().register_extension("X", "Y").unwrap();
    registry.write().unwrap().register_extension("Y", "Z").unwrap();
    
    registry
}

/// Mock struct that implements both visualization traits to test the adapter
struct MockCodeGenerator {
    registry_extensions: Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>,
}

impl MockCodeGenerator {
    fn new() -> Self {
        MockCodeGenerator {
            registry_extensions: setup_interface_hierarchy(),
        }
    }
}

// First implement the base visualization trait
impl InterfaceTypeAssertionPathVisualization<'_> for MockCodeGenerator {
    fn visualize_type_path(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Result<String, Error> {
        Ok(format!("Base visualization: {} -> {}", from_type, to_type))
    }
    
    fn get_runtime_type_id(
        &mut self,
        _interface_value: inkwell::values::BasicValueEnum<'_>,
        _source_location: Option<cursed::error::SourceLocation>,
    ) -> Result<(u64, String), Error> {
        Ok((123, "MockType".to_string()))
    }
    
    fn check_type_assertion_with_visualization(
        &mut self,
        _interface_value: inkwell::values::BasicValueEnum<'_>,
        target_type: &str,
        _source_location: Option<cursed::error::SourceLocation>,
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        // This is a mock implementation that doesn't need to return a real BasicValueEnum
        Err(Error::Compilation(format!("Mock implementation for {}", target_type)))
    }
    
    fn get_implemented_interfaces(&self, _type_id: u64, _inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
        vec![1, 2, 3]
    }
    
    fn get_interface_implementors(&self, _interface_id: u64, _inheritance_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
        vec![4, 5, 6]
    }
    
    fn get_relationship_type(&self, _from_id: u64, _to_id: u64, _inheritance_map: &HashMap<u64, HashSet<u64>>) -> String {
        "implements".to_string()
    }
    
    fn get_or_insert_runtime_function(&mut self, _name: &str) -> Option<inkwell::values::FunctionValue<'_>> {
        None
    }
}

// Then implement the enhanced visualization trait
impl EnhancedInterfaceTypeAssertionPathVisualization<'_> for MockCodeGenerator {
    fn interface_registry(&self) -> &dyn cursed::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization {
        &self.registry_extensions
    }
    
    fn interface_registry_mut(&mut self) -> &mut dyn cursed::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization {
        &mut self.registry_extensions
    }
    
    fn generate_interface_hierarchy_dot_enhanced(&self) -> Result<String, Error> {
        Ok("Enhanced DOT graph visualization".to_string())
    }
    
    fn find_alternative_paths_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        _max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        Ok(vec![vec![source_interface.to_string(), target_interface.to_string()]])
    }
    
    fn generate_path_error_message_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        Ok(format!("Enhanced error message: {} -> {} at {}", 
            source_interface, target_interface, source_location))
    }
    
    fn visualize_interface_path_enhanced(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        Ok(format!("Enhanced path visualization: {} -> {}", 
            source_interface, target_interface))
    }
    
    fn compile_type_assertion_with_path_visualization_enhanced(
        &mut self,
        _type_assertion: &cursed::ast::expressions::TypeAssertion,
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        Err(Error::Compilation("Mock enhanced compilation".to_string()))
    }
}

// No need to explicitly implement the adapter trait, it has a blanket implementation
// for any type that implements both the base and enhanced traits

#[test]
fn test_adapter_forward_find_interface_path() {
    let generator = MockCodeGenerator::new();
    
    // Test that the adapter correctly forwards to the registry's path finding
    let path = generator.forward_find_interface_path("A", "D").unwrap();
    
    // The actual path should be determined by the registry we set up
    assert!(path.contains(&"A".to_string()));
    assert!(path.contains(&"D".to_string()));
    // Verify it's the expected path (either A->B->D or A->C->D)
    assert!(path.len() == 3, "Expected path length of 3, got {}", path.len());
}

#[test]
fn test_adapter_forward_visualize_interface_path() {
    let generator = MockCodeGenerator::new();
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let visualization = generator.forward_visualize_interface_path("A", "D").unwrap();
    
    // Should be using the enhanced implementation which has this format
    assert!(visualization.contains("Enhanced path visualization: A -> D"));
}

#[test]
fn test_adapter_forward_generate_interface_hierarchy_dot() {
    let generator = MockCodeGenerator::new();
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let dot = generator.forward_generate_interface_hierarchy_dot().unwrap();
    
    // Should be using the enhanced implementation
    assert_eq!(dot, "Enhanced DOT graph visualization");
}

#[test]
fn test_adapter_forward_generate_path_error_message() {
    let generator = MockCodeGenerator::new();
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let error_message = generator.forward_generate_path_error_message("A", "X", "test.csd:123").unwrap();
    
    // Should be using the enhanced implementation
    assert!(error_message.contains("Enhanced error message: A -> X at test.csd:123"));
}

#[test]
fn test_adapter_ensure_registry_access() {
    let generator = MockCodeGenerator::new();
    
    // Test that the adapter correctly provides access to the registry
    let registry = generator.ensure_registry_access().unwrap();
    
    // Verify registry has our test data
    let interfaces = registry.get_all_interfaces().unwrap();
    assert!(interfaces.contains("A"));
    assert!(interfaces.contains("B"));
    assert!(interfaces.contains("C"));
    assert!(interfaces.contains("D"));
}

#[test]
fn test_trait_compatibility() {
    let generator = MockCodeGenerator::new();
    
    // Verify that we can call both base trait and enhanced trait methods directly
    let base_result = InterfaceTypeAssertionPathVisualization::visualize_type_path(&generator, "A", "D").unwrap();
    let enhanced_result = EnhancedInterfaceTypeAssertionPathVisualization::visualize_interface_path_enhanced(&generator, "A", "D").unwrap();
    
    // Verify different implementations were called
    assert!(base_result.contains("Base visualization"));
    assert!(enhanced_result.contains("Enhanced path visualization"));
    
    // Verify adapter methods work
    let adapter_result = InterfaceTypeAssertionPathVisualizationAdapter::forward_visualize_interface_path(&generator, "A", "D").unwrap();
    assert!(adapter_result.contains("Enhanced path visualization"));
}