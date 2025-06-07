use cursed::ast::expressions::TypeAssertion;
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder;
use cursed::codegen::llvm::interface_type_assertion_with_registry::InterfaceTypeAssertionWithRegistry;
use cursed::error::Error;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};

// # Interface Type Assertion with Registry Integration Tests
//
// This module tests the enhanced interface type assertion implementation
// that directly integrates with the interface registry extension checking system.
// It verifies that type assertions correctly handle inheritance relationships
// and provide useful error messages for debugging.



mod common;

/// Test stub implementation of a type assertion AST node
struct TestExpression {
    pub type_id: u64,
    pub name: String,
}

impl Node for TestExpression {
    fn token_literal(&self) -> String {
        "test".to_string()
    }
    
    fn string(&self) -> String {
        format!("TestExpr({})", self.name)
    }
}

impl Expression for TestExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(Self {
            type_id: self.type_id,
            name: self.name.clone(),
        })
    }
    
    fn node_type(&self) -> &str {
        "TestExpression"
    }
}

#[test]
fn test_interface_type_assertion_with_registry() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd");
    
    // Set up a test interface hierarchy
    // Reader -> FileReader -> JSONFileReader
    // Reader -> NetworkReader
    setup_test_interfaces(&mut codegen);
    
    // Create a test expression with a FileReader type
    let file_reader_expr = TestExpression {
        type_id: 1002,  // FileReader ID
        name: "fileReader".to_string(),
    };
    
    // Create a type assertion: fileReader.(Reader)
    let type_assertion = TypeAssertion {
        token: "token".to_string(),
        expression: Box::new(file_reader_expr),
        type_name: "Reader".to_string(),
    };
    
    // Mock the codegen methods to return our test values
    // (This is normally test setup that would be better handled with a mock framework)
    
    // Test that a direct relationship can be verified
    // FileReader directly extends Reader, so this should succeed
    assert!(codegen.check_extension_relationship_enhanced("FileReader", "Reader").unwrap();
    
    // Test that an indirect relationship can be verified
    // JSONFileReader indirectly extends Reader (through FileReader), so this should succeed
    assert!(codegen.check_extension_relationship_enhanced("JSONFileReader", "Reader").unwrap();
    
    // Test that the path finder can find the correct path
    let path = codegen.find_interface_path_enhanced("JSONFileReader", "Reader").unwrap();
    assert_eq!(path.path(), &vec!["JSONFileReader".to_string(), "FileReader".to_string(), "Reader".to_string())]);
    
    // Test reversed relationship detection
    let (reversed, _) = codegen.detect_reversed_inheritance_enhanced("Reader", "FileReader").unwrap();
    assert!(reversed);
}

#[test]
fn test_interface_type_assertion_path_registry() {
    common::tracing::setup();
    
    // Create a context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd");
    
    // Set up a more complex test interface hierarchy
    // Vehicle -> LandVehicle -> Car -> SportsCar
    //         -> WaterVehicle -> Boat
    //         -> AirVehicle -> Plane -> Jet
    setup_vehicle_interfaces(&mut codegen);
    
    // Test that a type assertion from an unrelated branch would fail
    // Boat should not extend LandVehicle
    assert!(!codegen.check_extension_relationship_enhanced("Boat", "LandVehicle").unwrap();
    
    // Test that a multi-level inheritance can be verified
    // SportsCar indirectly extends Vehicle (through Car and LandVehicle)
    assert!(codegen.check_extension_relationship_enhanced("SportsCar", "Vehicle").unwrap();
    
    // Test that the path finder can find the correct path for multi-level inheritance
    let path = codegen.find_interface_path_enhanced("Jet", "Vehicle").unwrap();
    assert_eq!(path.path(), &vec!["Jet".to_string(), "Plane".to_string(), 
                               "AirVehicle".to_string(), "Vehicle".to_string())]);
    
    // Test visualization of interface hierarchies
    let hierarchy = codegen.visualize_interface_hierarchy("Vehicle", 3).unwrap();
    assert!(hierarchy.contains("Vehicle");
    assert!(hierarchy.contains("LandVehicle");
    assert!(hierarchy.contains("SportsCar");
    
    // DOT graph generation
    let dot_graph = codegen.generate_interface_hierarchy_dot_graph().unwrap();
    assert!(dot_graph.contains("digraph interface_hierarchy");
    assert!(dot_graph.contains("Vehicle");
    assert!(dot_graph.contains("Car");
}

/// Setup helper to register test interfaces and their relationships
fn setup_test_interfaces(codegen: &mut LlvmCodeGenerator) {
    // Register test interface types
    codegen.register_type_in_registry(1001, "Reader");
    codegen.register_type_in_registry(1002, "FileReader");
    codegen.register_type_in_registry(1003, "JSONFileReader");
    codegen.register_type_in_registry(1004, "NetworkReader");
    codegen.register_type_in_registry(1005, "XMLFileReader");
    codegen.register_type_in_registry(1006, "BinaryFileReader");
    
    // Setup inheritance map (FileReader and NetworkReader extend Reader)
    let mut test_inheritance_map = HashMap::new();
    
    let mut reader_extensions = HashSet::new();
    reader_extensions.insert("FileReader".to_string());
    reader_extensions.insert("NetworkReader".to_string());
    test_inheritance_map.insert("Reader".to_string(, reader_extensions);
    
    // JSONFileReader, XMLFileReader and BinaryFileReader extend FileReader
    let mut filereader_extensions = HashSet::new();
    filereader_extensions.insert("JSONFileReader".to_string());
    filereader_extensions.insert("XMLFileReader".to_string());
    filereader_extensions.insert("BinaryFileReader".to_string());
    test_inheritance_map.insert("FileReader".to_string(, filereader_extensions);
    
    // Store inheritance map in codegen
    codegen.test_inheritance_map = Some(test_inheritance_map);
}

/// Setup helper for a more complex inheritance hierarchy
fn setup_vehicle_interfaces(codegen: &mut LlvmCodeGenerator) {
    // Register vehicle interface types
    codegen.register_type_in_registry(2001, "Vehicle");
    codegen.register_type_in_registry(2002, "LandVehicle");
    codegen.register_type_in_registry(2003, "WaterVehicle");
    codegen.register_type_in_registry(2004, "AirVehicle");
    codegen.register_type_in_registry(2005, "Car");
    codegen.register_type_in_registry(2006, "Boat");
    codegen.register_type_in_registry(2007, "Plane");
    codegen.register_type_in_registry(2008, "SportsCar");
    codegen.register_type_in_registry(2009, "Jet");
    codegen.register_type_in_registry(2010, "Submarine");
    
    // Setup inheritance map
    let mut test_inheritance_map = HashMap::new();
    
    // Vehicle has three direct extensions
    let mut vehicle_extensions = HashSet::new();
    vehicle_extensions.insert("LandVehicle".to_string());
    vehicle_extensions.insert("WaterVehicle".to_string());
    vehicle_extensions.insert("AirVehicle".to_string());
    test_inheritance_map.insert("Vehicle".to_string(, vehicle_extensions);
    
    // LandVehicle extends to Car
    let mut land_vehicle_extensions = HashSet::new();
    land_vehicle_extensions.insert("Car".to_string());
    test_inheritance_map.insert("LandVehicle".to_string(, land_vehicle_extensions);
    
    // WaterVehicle extends to Boat and Submarine
    let mut water_vehicle_extensions = HashSet::new();
    water_vehicle_extensions.insert("Boat".to_string());
    water_vehicle_extensions.insert("Submarine".to_string());
    test_inheritance_map.insert("WaterVehicle".to_string(, water_vehicle_extensions);
    
    // AirVehicle extends to Plane
    let mut air_vehicle_extensions = HashSet::new();
    air_vehicle_extensions.insert("Plane".to_string());
    test_inheritance_map.insert("AirVehicle".to_string(, air_vehicle_extensions);
    
    // Car extends to SportsCar
    let mut car_extensions = HashSet::new();
    car_extensions.insert("SportsCar".to_string());
    test_inheritance_map.insert("Car".to_string(, car_extensions);
    
    // Plane extends to Jet
    let mut plane_extensions = HashSet::new();
    plane_extensions.insert("Jet".to_string());
    test_inheritance_map.insert("Plane".to_string(, plane_extensions);
    
    // Store inheritance map in codegen
    codegen.test_inheritance_map = Some(test_inheritance_map);
}