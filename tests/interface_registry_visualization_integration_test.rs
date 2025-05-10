//! # Integration Test for Interface Registry Visualization
//!
//! This module tests the integration of the interface registry visualization system with the
//! existing codebase, verifying that it properly interacts with the interface type assertion
//! path visualization enhanced module.

use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required modules
use cursed::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, ThreadSafeInterfaceRegistryVisualization};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::error::Error;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_interface_registry_visualization_integration() {
    init_tracing!();
    
    // Create a new visualization registry
    let registry = ThreadSafeInterfaceRegistryVisualization::new();
    
    // Register test interface relationships
    registry.register_extension("Animal", "Mammal").unwrap();
    registry.register_extension("Mammal", "Dog").unwrap();
    registry.register_extension("Mammal", "Cat").unwrap();
    registry.register_extension("Animal", "Bird").unwrap();
    registry.register_extension("Bird", "Eagle").unwrap();
    
    // Test the visualization methods
    let dot = registry.visualize_hierarchy_dot().unwrap();
    let ascii = registry.visualize_hierarchy_ascii().unwrap();
    
    // Test finding an inheritance path
    let path = registry.find_inheritance_path("Animal", "Dog").unwrap();
    assert_eq!(path, vec!["Animal", "Mammal", "Dog"]);
    
    // Test the path visualization methods
    let path_ascii = registry.visualize_path_ascii(&path).unwrap();
    let path_dot = registry.visualize_path_dot(&path).unwrap();
    
    // Verify that all the visualizations contain the expected content
    assert!(dot.contains("digraph interface_hierarchy"));
    assert!(dot.contains("\"Animal\" -> \"Mammal\""));
    assert!(dot.contains("\"Mammal\" -> \"Dog\""));
    
    assert!(ascii.contains("Interface Hierarchy:"));
    assert!(ascii.contains("Animal"));
    assert!(ascii.contains("Mammal"));
    assert!(ascii.contains("Dog"));
    
    assert!(path_ascii.contains("Interface Inheritance Path:"));
    assert!(path_ascii.contains("[Animal]"));
    assert!(path_ascii.contains("↓ extends"));
    assert!(path_ascii.contains("[Mammal]"));
    assert!(path_ascii.contains("[Dog]"));
    
    assert!(path_dot.contains("digraph path"));
    assert!(path_dot.contains("\"Animal\" -> \"Mammal\""));
    assert!(path_dot.contains("\"Mammal\" -> \"Dog\""));
    
    // Test cycle detection
    registry.register_extension("Dog", "Animal").unwrap(); // Create a cycle
    let cycles = registry.detect_cycles().unwrap();
    assert!(!cycles.is_empty());
    
    // Verify that at least one cycle contains all the expected interfaces
    let mut has_cycle = false;
    for cycle in &cycles {
        if cycle.contains(&"Animal".to_string()) && 
           cycle.contains(&"Mammal".to_string()) && 
           cycle.contains(&"Dog".to_string()) {
            has_cycle = true;
            break;
        }
    }
    assert!(has_cycle);
}

#[test]
fn test_interface_type_assertion_with_visualization() {
    init_tracing!();
    
    // Test code with interface type assertions
    let input = r#"
        collab Animal {
            makeSound() tea;
        }
        
        collab Mammal {
            makeSound() tea;
            giveBirth() tea;
        }
        
        collab Dog {
            makeSound() tea;
            giveBirth() tea;
            bark() tea;
        }
        
        squad Canine {
            name tea,
            age normie
        }
        
        slay (c Canine) makeSound() tea {
            yolo "Woof!"
        }
        
        slay (c Canine) giveBirth() tea {
            yolo "Puppies!"
        }
        
        slay (c Canine) bark() tea {
            yolo "Bark bark!"
        }
        
        // Function that uses type assertions with improved error messages
        slay testAnimal(animal Animal) tea {
            // Try to assert as Dog (should succeed because Canine implements Dog)
            sus dog, isDog = animal.(Dog)
            if isDog {
                yolo "It's a dog: " + dog.bark()
            }
            
            // Assert as Mammal (should succeed)
            sus mammal, isMammal = animal.(Mammal)
            if isMammal {
                yolo "It's a mammal: " + mammal.giveBirth()
            }
            
            // Use the base interface method
            yolo "Animal sound: " + animal.makeSound()
        }
        
        slay main() tea {
            sus canine = Canine{name: "Rex", age: 5}
            
            // This will work as expected because Canine implements all required methods
            sus result = testAnimal(canine)
            
            yolo result
        }
    "#;
    
    // Create lexer and parser
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Assert no parser errors
    assert!(parser.errors().is_empty());
    
    // Create LLVM context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_interface_visualization.csd");
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(&context, "main", file_path);
    
    // Set up the registry (this would normally be done in the code generator constructor)
    // but we're doing it explicitly for this test to verify correct integration
    code_gen.registry_extensions = ThreadSafeInterfaceRegistryVisualization::new();
    
    // Compile the program
    let result = code_gen.compile(&program);
    assert!(result.is_ok());
    
    // Check that the registry now contains the interface relationships
    let dot = code_gen.interface_registry().visualize_hierarchy_dot().unwrap();
    
    // Verify the dot graph contains the expected interfaces
    assert!(dot.contains("Animal"));
    assert!(dot.contains("Mammal"));
    assert!(dot.contains("Dog"));
}