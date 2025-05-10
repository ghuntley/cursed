//! # Interface Registry Extension Checking Tests
//!
//! This module tests the comprehensive implementation of the InterfaceTypeRegistryExtensionChecking
//! trait for reliable inheritance verification in interface type assertions with proper
//! integration with the interface path finder for enhanced error diagnostics.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
use cursed::error::Error;

use inkwell::context::Context;
use std::collections::{HashMap, HashSet};

mod common;

#[test]
fn test_interface_registry_extension_checking() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module").unwrap();
    
    // Set up some test interfaces in the registry
    // Reader -> FileReader -> JSONFileReader
    // Reader -> NetworkReader
    // Serializable -> JSONSerializable
    codegen.register_type_in_registry(1001, "Reader");
    codegen.register_type_in_registry(1002, "FileReader");
    codegen.register_type_in_registry(1003, "JSONFileReader");
    codegen.register_type_in_registry(1004, "NetworkReader");
    codegen.register_type_in_registry(1005, "Serializable");
    codegen.register_type_in_registry(1006, "JSONSerializable");
    
    // Set up inheritance relationships for testing
    setup_test_inheritance_relationships(&mut codegen);
    
    // Using the enhanced interface path finder for relationship detection
    // Test transitive relationship - this should work with the enhanced path finder
    assert!(codegen.check_extension_relationship_enhanced("JSONFileReader", "Reader").unwrap());
    
    // Test unrelated interfaces
    assert!(!codegen.check_extension_relationship_enhanced("JSONFileReader", "Serializable").unwrap());
    
    // Test with non-existent interface
    assert!(!codegen.check_extension_relationship_enhanced("NonExistentInterface", "Reader").unwrap());
    
    // Test checking for reversed relationships
    assert!(!codegen.check_extension_relationship_enhanced("Reader", "JSONFileReader").unwrap());
    let (reversed, _) = codegen.detect_reversed_inheritance_enhanced("Reader", "JSONFileReader").unwrap();
    assert!(reversed);
    
    // Test visualization of interface hierarchies
    let hierarchy = codegen.visualize_interface_hierarchy("Reader", 2).unwrap();
    assert!(hierarchy.contains("Interface Hierarchy for 'Reader'"));
    assert!(hierarchy.contains("FileReader"));
    assert!(hierarchy.contains("NetworkReader"));
    assert!(hierarchy.contains("JSONFileReader"));
}

/// Test that path finding works even with partial relationships in the registry
#[test]
fn test_partial_extension_relationships() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module").unwrap();
    
    // Set up just a few test interfaces
    codegen.register_type_in_registry(1001, "Animal");
    codegen.register_type_in_registry(1002, "Mammal");
    codegen.register_type_in_registry(1003, "Dog");
    
    // Set up minimal inheritance relationships
    let mut test_inheritance_map = HashMap::new();
    let mut animal_extensions = HashSet::new();
    animal_extensions.insert("Mammal".to_string());
    test_inheritance_map.insert("Animal".to_string(), animal_extensions);
    
    let mut mammal_extensions = HashSet::new();
    mammal_extensions.insert("Dog".to_string());
    test_inheritance_map.insert("Mammal".to_string(), mammal_extensions);
    
    codegen.test_inheritance_map = Some(test_inheritance_map);
    
    // Path finding for direct and indirect relationships
    let direct_path = codegen.find_interface_path_enhanced("Mammal", "Animal").unwrap();
    assert_eq!(direct_path.path(), &vec!["Mammal".to_string(), "Animal".to_string()]);
    
    // Test indirect relationship via path finding (Dog -> Mammal -> Animal)
    let indirect_path = codegen.find_interface_path_enhanced("Dog", "Animal").unwrap();
    assert_eq!(indirect_path.path(), &vec!["Dog".to_string(), "Mammal".to_string(), "Animal".to_string()]);
    
    // Check that the extension relationship check uses the enhanced path finder internally
    assert!(codegen.check_extension_relationship_enhanced("Dog", "Animal").unwrap());
}

/// Test multi-level inheritance hierarchies using enhanced extension checking
#[test]
fn test_multi_level_inheritance_hierarchies() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module").unwrap();
    
    // Set up a more complex test interface hierarchy
    // Vehicle -> LandVehicle -> Car -> SportsCar
    //         -> WaterVehicle -> Boat
    //         -> AirVehicle -> Plane -> Jet
    
    // Register all the interfaces
    codegen.register_type_in_registry(1001, "Vehicle");
    codegen.register_type_in_registry(1002, "LandVehicle");
    codegen.register_type_in_registry(1003, "WaterVehicle");
    codegen.register_type_in_registry(1004, "AirVehicle");
    codegen.register_type_in_registry(1005, "Car");
    codegen.register_type_in_registry(1006, "Boat");
    codegen.register_type_in_registry(1007, "Plane");
    codegen.register_type_in_registry(1008, "SportsCar");
    codegen.register_type_in_registry(1009, "Jet");
    
    // Set up the inheritance relationships
    let mut test_inheritance_map = HashMap::new();
    
    // Vehicle has three direct extensions
    let mut vehicle_extensions = HashSet::new();
    vehicle_extensions.insert("LandVehicle".to_string());
    vehicle_extensions.insert("WaterVehicle".to_string());
    vehicle_extensions.insert("AirVehicle".to_string());
    test_inheritance_map.insert("Vehicle".to_string(), vehicle_extensions);
    
    // LandVehicle extends to Car
    let mut land_vehicle_extensions = HashSet::new();
    land_vehicle_extensions.insert("Car".to_string());
    test_inheritance_map.insert("LandVehicle".to_string(), land_vehicle_extensions);
    
    // WaterVehicle extends to Boat
    let mut water_vehicle_extensions = HashSet::new();
    water_vehicle_extensions.insert("Boat".to_string());
    test_inheritance_map.insert("WaterVehicle".to_string(), water_vehicle_extensions);
    
    // AirVehicle extends to Plane
    let mut air_vehicle_extensions = HashSet::new();
    air_vehicle_extensions.insert("Plane".to_string());
    test_inheritance_map.insert("AirVehicle".to_string(), air_vehicle_extensions);
    
    // Car extends to SportsCar
    let mut car_extensions = HashSet::new();
    car_extensions.insert("SportsCar".to_string());
    test_inheritance_map.insert("Car".to_string(), car_extensions);
    
    // Plane extends to Jet
    let mut plane_extensions = HashSet::new();
    plane_extensions.insert("Jet".to_string());
    test_inheritance_map.insert("Plane".to_string(), plane_extensions);
    
    // Set the test inheritance map
    codegen.test_inheritance_map = Some(test_inheritance_map);
    
    // Test direct relationship
    assert!(codegen.check_extension_relationship_enhanced("LandVehicle", "Vehicle").unwrap());
    
    // Test single-level indirect relationship
    assert!(codegen.check_extension_relationship_enhanced("Car", "Vehicle").unwrap());
    
    // Test two-level indirect relationship
    assert!(codegen.check_extension_relationship_enhanced("SportsCar", "Vehicle").unwrap());
    
    // Test multi-path relationship (finding different paths)
    let jet_to_vehicle_path = codegen.find_interface_path_enhanced("Jet", "Vehicle").unwrap();
    assert_eq!(jet_to_vehicle_path.path(), &vec!["Jet".to_string(), "Plane".to_string(), 
                                              "AirVehicle".to_string(), "Vehicle".to_string()]);
    
    // Test finding multiple paths (should only be one in this case)
    let paths = codegen.find_alternative_paths_enhanced("SportsCar", "Vehicle", 3).unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].path(), &vec!["SportsCar".to_string(), "Car".to_string(), 
                                   "LandVehicle".to_string(), "Vehicle".to_string()]);
    
    // Verify path visualization
    let hierarchy = codegen.visualize_interface_hierarchy("Vehicle", 3).unwrap();
    assert!(hierarchy.contains("SportsCar"));
    assert!(hierarchy.contains("Jet"));
    
    // Test that non-existent paths return appropriate results
    assert!(!codegen.check_extension_relationship_enhanced("Boat", "LandVehicle").unwrap());
    assert!(!codegen.check_extension_relationship_enhanced("Car", "AirVehicle").unwrap());
    
    // Test reversed inheritance detection
    let (reversed, message) = codegen.detect_reversed_inheritance_enhanced("Vehicle", "Car").unwrap();
    assert!(reversed);
    assert!(message.contains("Reversed inheritance detected"));
}

/// Test hook to set up inheritance relationships for testing
fn setup_test_inheritance_relationships(codegen: &mut LlvmCodeGenerator) {
    // Create a test inheritance map in codegen
    let mut test_inheritance_map = HashMap::new();
    
    // Set up FileReader extends Reader
    let mut reader_extensions = HashSet::new();
    reader_extensions.insert("FileReader".to_string());
    reader_extensions.insert("NetworkReader".to_string());
    test_inheritance_map.insert("Reader".to_string(), reader_extensions);
    
    // Set up JSONFileReader extends FileReader
    let mut filereader_extensions = HashSet::new();
    filereader_extensions.insert("JSONFileReader".to_string());
    test_inheritance_map.insert("FileReader".to_string(), filereader_extensions);
    
    // Set up JSONSerializable extends Serializable
    let mut serializable_extensions = HashSet::new();
    serializable_extensions.insert("JSONSerializable".to_string());
    test_inheritance_map.insert("Serializable".to_string(), serializable_extensions);
    
    // Store this in the code generator for testing
    codegen.test_inheritance_map = Some(test_inheritance_map);
}