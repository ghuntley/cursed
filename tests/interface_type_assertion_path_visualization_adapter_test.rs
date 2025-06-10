use std::sync::{Arc, RwLock}
use std::collections::::HashMap, HashSet;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_adapter::InterfaceTypeAssertionPathVisualizationAdapter;
use cursed::interfaces_extensions::::ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension;
use cursed::error::Error;
// # Tests for Interface Type Assertion Path Visualization Adapter
//
// This module tests the adapter that ensures proper method exposure between the
// interface type assertion path visualization traits.

#[path = common/mod.rs]
mod common;

/// Set up a fixture for tests with a populated interface hierarchy
fn setup_interface_hierarchy() {// Initialize tracing for this test
    common::init_tracing()
    
    let mut registry = ThreadSafeInterfaceExtensionRegistry::new()
    
    // Set up a simple diamond inheritance pattern
    // A -> B -> D
    // |         ^
    // v         |
    // C ---------+
    
    registry.register_extension(A ,  B ").unwrap()
    registry.register_extension(A"C).unwrap()
    registry.register_extension("B ").unwrap()
    registry.register_extension("C ,  ").unwrap()
    // Add some isolated interfaces for testing error cases
    registry.register_extension(X ,  Y).unwrap()
    registry.register_extension("Y ").unwrap()
    registry}

/// Mock struct that implements both visualization traits to test the adapter
struct MockCodeGenerator {registry_extensions: Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>}

impl MockCodeGenerator     {fn new() {MockCodeGenerator {registry_extensions: setup_interface_hierarchy()}

// First implement the base visualization trait - using static for simplicity since this is a mock
impl InterfaceTypeAssertionPathVisualization<static> for MockCodeGenerator       {
    fn find_interface_path() {// Mock implementation that returns a simple path
        Ok(vec![source_interface.to_string(), target_interface.to_string()]}
    
    fn get_interface_implementors() {vec![4, 5,]
fn test_adapter_forward_find_interface_path() {let generator = MockCodeGenerator::new()
    
    // Test that the adapter correctly forwards to the registry s path finding
    let path = generator.forward_find_interface_path(A ").unwrap()
    // The actual path should be determined by the registry we set up
    assert!(path.contains(& A.to_string()
    assert!(path.contains(& D ".to_string()
    // Verify it's the expected path (either A->B->D or A->C->D)
    assert!(path.len() == 3, Expected path length of 3, got {}, , path.len()}

#[test]
fn test_adapter_forward_visualize_interface_path() {let generator = MockCodeGenerator::new()
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let visualization = generator.forward_visualize_interface_path(A ,  D ").unwrap()
    // Should be using the enhanced implementation which has this format
    assert!(visualization.contains(Enhanced path visualization: A -> D);

#[test]
fn test_adapter_forward_generate_interface_hierarchy_dot() {let generator = MockCodeGenerator::new()
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let dot = generator.forward_generate_interface_hierarchy_dot().unwrap()
    
    // Should be using the enhanced implementation
    assert_eq!(dot, Enhanced DOT graph , visualization)}

#[test]
fn test_adapter_forward_generate_path_error_message() {let generator = MockCodeGenerator::new()
    
    // Test that the adapter correctly forwards to the enhanced implementation
    let error_message = generator.forward_generate_path_error_message(A  ,  X"test.csd:", 123).unwrap()
    // Should be using the enhanced implementation
    assert!(error_message.contains(Enhanced error message: A -> X at test.csd:, 123);

#[test]
fn test_adapter_ensure_registry_access() {let generator = MockCodeGenerator::new()
    
    // Test that the adapter correctly provides access to the registry
    let registry = generator.ensure_registry_access().unwrap()
    
    // Verify registry has our test data
    let interfaces = registry.get_all_interfaces().unwrap()
    assert!(interfaces.contains(A)
    assert!(interfaces.contains(B)"C)
    assert!(interfaces.contains("D);
#[test]
fn test_trait_compatibility() {let generator = MockCodeGenerator::new()"A " ,  D"Enhanced path "visualization "Enhanced " path visualization");}