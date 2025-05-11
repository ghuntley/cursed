//! Tests for the interface type assertion path visualization
//!
//! This module tests the standalone functionality of the path visualization system
//! for interface type assertions, verifying that it correctly analyzes and visualizes
//! type relationships for error reporting and debugging.

use std::collections::HashMap;

use cursed::codegen::llvm::interface_type_assertion_path_visualization::*;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::error::Error;

// Import common test utilities
mod common;
use common::setup_test_environment;

// Test-specific mocks and utilities
struct MockInterfaceRegistry {
    // Map of type to interfaces it implements
    implementations: HashMap<String, Vec<String>>,
    // Map of interface to types that implement it
    implementors: HashMap<String, Vec<String>>,
    // Map of type ID to type name
    type_id_map: HashMap<u64, String>,
}

impl MockInterfaceRegistry {
    fn new() -> Self {
        Self {
            implementations: HashMap::new(),
            implementors: HashMap::new(),
            type_id_map: HashMap::new(),
        }
    }
    
    fn register_implementation(&mut self, type_name: &str, interface_name: &str) {
        // Register that type implements interface
        self.implementations
            .entry(type_name.to_string())
            .or_insert_with(Vec::new)
            .push(interface_name.to_string());
        
        // Register that interface is implemented by type
        self.implementors
            .entry(interface_name.to_string())
            .or_insert_with(Vec::new)
            .push(type_name.to_string());
    }
    
    fn register_type_id(&mut self, type_name: &str, type_id: u64) {
        self.type_id_map.insert(type_id, type_name.to_string());
    }
    
    fn get_implemented_interfaces(&self, type_name: &str) -> Result<Vec<String>, String> {
        Ok(self.implementations
            .get(type_name)
            .cloned()
            .unwrap_or_default())
    }
    
    fn get_interface_implementors(&self, interface_name: &str) -> Result<Vec<String>, String> {
        Ok(self.implementors
            .get(interface_name)
            .cloned()
            .unwrap_or_default())
    }
    
    fn get_extended_interfaces(&self, interface_name: &str) -> Result<Vec<String>, String> {
        self.get_implemented_interfaces(interface_name)
    }
    
    fn get_type_name_for_id(&self, type_id: u64) -> Result<String, String> {
        self.type_id_map
            .get(&type_id)
            .cloned()
            .ok_or_else(|| format!("Type ID {} not found", type_id))
    }
}

// Mock LLVM code generator with our visualization trait
struct MockGenerator {
    registry: MockInterfaceRegistry,
}

impl MockGenerator {
    fn new() -> Self {
        let mut registry = MockInterfaceRegistry::new();
        
        // Set up a test type hierarchy
        // Animal hierarchy
        registry.register_implementation("Dog", "Animal");
        registry.register_implementation("Cat", "Animal");
        registry.register_implementation("Bird", "Animal");
        
        // Pet hierarchy
        registry.register_implementation("Dog", "Pet");
        registry.register_implementation("Cat", "Pet");
        registry.register_implementation("Hamster", "Pet");
        
        // Flying creatures
        registry.register_implementation("Bird", "Flying");
        registry.register_implementation("Bat", "Flying");
        registry.register_implementation("Airplane", "Flying");
        
        // Vehicles
        registry.register_implementation("Car", "Vehicle");
        registry.register_implementation("Airplane", "Vehicle");
        registry.register_implementation("Boat", "Vehicle");
        
        // Type IDs
        registry.register_type_id("Dog", 1);
        registry.register_type_id("Cat", 2);
        registry.register_type_id("Bird", 3);
        registry.register_type_id("Hamster", 4);
        registry.register_type_id("Bat", 5);
        registry.register_type_id("Airplane", 6);
        registry.register_type_id("Car", 7);
        registry.register_type_id("Boat", 8);
        registry.register_type_id("Animal", 101);
        registry.register_type_id("Pet", 102);
        registry.register_type_id("Flying", 103);
        registry.register_type_id("Vehicle", 104);
        
        Self { registry }
    }
    
    // Mock registry initialization
    fn ensure_registry_initialized(&self) -> Result<(), Error> {
        Ok(())
    }
}

// Implement the path visualization trait on our mock
impl InterfaceTypeAssertionPathVisualization for MockGenerator {
    fn visualize_interface_path(
        &mut self,
        target_type: &str,
        max_depth: usize
    ) -> Result<String, Error> {
        // Get all direct interfaces that the target type implements
        let interfaces = self.registry.get_implemented_interfaces(target_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        let mut result = format!("Interface hierarchy for '{}':\n", target_type);
        
        if interfaces.is_empty() {
            result.push_str("  No interfaces implemented directly.\n");
        } else {
            result.push_str("  Directly implements:\n");
            for interface in &interfaces {
                result.push_str(&format!("    - {}\n", interface));
            }
        }
        
        Ok(result)
    }
    
    fn find_alternative_paths(
        &mut self,
        source_type: &str,
        target_type: &str,
        max_paths: usize
    ) -> Result<Vec<String>, Error> {
        let mut paths = Vec::new();
        
        // Check for direct implementation
        let source_interfaces = self.registry.get_implemented_interfaces(source_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        if source_interfaces.contains(&target_type.to_string()) {
            paths.push(format!("{} -> {}", source_type, target_type));
        }
        
        // Check common interfaces
        let target_interfaces = self.registry.get_implemented_interfaces(target_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        for source_interface in &source_interfaces {
            for target_interface in &target_interfaces {
                if source_interface == target_interface {
                    let path = format!("{} -> {} -> {}", source_type, source_interface, target_type);
                    if !paths.contains(&path) && paths.len() < max_paths {
                        paths.push(path);
                    }
                }
            }
        }
        
        Ok(paths)
    }
    
    fn check_extension_relationship_simple(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<bool, Error> {
        let interfaces = self.registry.get_implemented_interfaces(source_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        Ok(interfaces.contains(&target_type.to_string()))
    }
    
    fn find_shortest_path(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<Vec<String>>, Error> {
        // Simple implementation for testing
        if source_type == target_type {
            return Ok(Some(vec![source_type.to_string()]));
        }
        
        let interfaces = self.registry.get_implemented_interfaces(source_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        if interfaces.contains(&target_type.to_string()) {
            return Ok(Some(vec![source_type.to_string(), target_type.to_string()]));
        }
        
        Ok(None)
    }
    
    fn generate_inheritance_tree(
        &mut self,
        root_type: &str,
        max_depth: usize
    ) -> Result<String, Error> {
        let mut result = format!("{} (root)\n", root_type);
        let interfaces = self.registry.get_implemented_interfaces(root_type)
            .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
        
        for interface in interfaces {
            result.push_str(&format!("└── {}\n", interface));
        }
        
        Ok(result)
    }
    
    fn get_implementors(
        &mut self,
        interface_type: &str
    ) -> Result<Vec<String>, Error> {
        self.registry.get_interface_implementors(interface_type)
            .map_err(|e| Error::Compilation(format!("Failed to get implementors: {}", e)))
    }
    
    fn get_runtime_type_id(
        &mut self,
        value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<u64, Error> {
        // Mock implementation
        Ok(1)
    }
    
    fn get_type_name_for_id(
        &mut self,
        type_id: u64
    ) -> Result<String, Error> {
        self.registry.get_type_name_for_id(type_id)
            .map_err(|e| Error::Compilation(e))
    }
    
    fn extract_interface_type_id(
        &mut self,
        _value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        // Mock implementation
        Err(Error::Compilation("Not implemented in mock".to_string()))
    }
}

#[test]
fn test_visualize_interface_path() {
    setup_test_environment();
    
    let mut generator = MockGenerator::new();
    
    let visualization = generator.visualize_interface_path("Dog", 2).unwrap();
    println!("{}", visualization);
    
    assert!(visualization.contains("Animal"));
    assert!(visualization.contains("Pet"));
}

#[test]
fn test_find_alternative_paths() {
    setup_test_environment();
    
    let mut generator = MockGenerator::new();
    
    // Test finding paths between types
    let paths = generator.find_alternative_paths("Dog", "Cat", 3).unwrap();
    println!("Paths between Dog and Cat: {:?}", paths);
    
    // Should have common interfaces
    assert!(paths.iter().any(|p| p.contains("Animal")));
    assert!(paths.iter().any(|p| p.contains("Pet")));
    
    // Test finding paths between unrelated types
    let paths = generator.find_alternative_paths("Dog", "Car", 3).unwrap();
    println!("Paths between Dog and Car: {:?}", paths);
    
    // Should be empty - no relationship
    assert!(paths.is_empty());
}

#[test]
fn test_check_extension_relationship() {
    setup_test_environment();
    
    let mut generator = MockGenerator::new();
    
    // Check existing relationship
    assert!(generator.check_extension_relationship_simple("Dog", "Animal").unwrap());
    
    // Check non-existing relationship
    assert!(!generator.check_extension_relationship_simple("Dog", "Vehicle").unwrap());
}

#[test]
fn test_get_implementors() {
    setup_test_environment();
    
    let mut generator = MockGenerator::new();
    
    let animal_implementors = generator.get_implementors("Animal").unwrap();
    println!("Animal implementors: {:?}", animal_implementors);
    
    assert!(animal_implementors.contains(&"Dog".to_string()));
    assert!(animal_implementors.contains(&"Cat".to_string()));
    assert!(animal_implementors.contains(&"Bird".to_string()));
    assert!(!animal_implementors.contains(&"Car".to_string()));
}

#[test]
fn test_inheritance_tree() {
    setup_test_environment();
    
    let mut generator = MockGenerator::new();
    
    let tree = generator.generate_inheritance_tree("Dog", 2).unwrap();
    println!("{}", tree);
    
    assert!(tree.contains("Animal"));
    assert!(tree.contains("Pet"));
}