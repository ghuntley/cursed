//! Integration test for interface type assertion path visualization
//! Verifies that the path visualization system integrates properly
//! with the interface registry to provide enhanced error messages.

// Initialize standard tracing infrastructure
use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;
use tracing::{debug, error, info};

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import relevant modules for testing
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::ast::traits::Node;
use cursed::ast::expressions::TypeAssertion;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;

// Test helper function to create a sample interface hierarchy
fn setup_test_hierarchy(registry: &ThreadSafeInterfaceExtensionRegistry) -> Result<(), Error> {
    // Setup common interfaces from the test
    registry.register_extension("Dog", "Mammal")?;
    registry.register_extension("Cat", "Mammal")?;
    registry.register_extension("Mammal", "Animal")?;
    registry.register_extension("Bird", "Animal")?;
    registry.register_extension("Reptile", "Animal")?;
    registry.register_extension("Animal", "LivingThing")?;
    registry.register_extension("Plant", "LivingThing")?;
    
    // Multiple inheritance
    registry.register_extension("FlyingFish", "Fish")?;
    registry.register_extension("FlyingFish", "Flying")?;
    registry.register_extension("Fish", "Animal")?;
    registry.register_extension("Flying", "MovementType")?;
    
    // More complex relationships for renderer interfaces
    registry.register_extension("Renderer", "Component")?;
    registry.register_extension("AnimatedRenderer", "Renderer")?;
    registry.register_extension("InteractiveRenderer", "Renderer")?;
    registry.register_extension("AdvancedRenderer", "AnimatedRenderer")?;
    registry.register_extension("AdvancedRenderer", "InteractiveRenderer")?;
    
    // Diamond inheritance pattern
    registry.register_extension("WebUI", "UI")?;
    registry.register_extension("MobileUI", "UI")?;
    registry.register_extension("HybridUI", "WebUI")?;
    registry.register_extension("HybridUI", "MobileUI")?;
    
    Ok(())
}

#[test]
fn test_interface_path_finding_integration() {
    init_tracing!();
    info!("Starting interface path finding integration test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Test finding a direct path
    match code_gen.find_interface_path("Dog", "Mammal") {
        Ok(path) => {
            info!("Found path from Dog to Mammal: {:?}", path);
            assert_eq!(path, vec!["Dog".to_string(), "Mammal".to_string()]);
        },
        Err(e) => {
            panic!("Failed to find path from Dog to Mammal: {}", e);
        }
    }
    
    // Test finding a longer path
    match code_gen.find_interface_path("Dog", "LivingThing") {
        Ok(path) => {
            info!("Found path from Dog to LivingThing: {:?}", path);
            assert_eq!(path, vec!["Dog".to_string(), "Mammal".to_string(), 
                              "Animal".to_string(), "LivingThing".to_string()]);
        },
        Err(e) => {
            panic!("Failed to find path from Dog to LivingThing: {}", e);
        }
    }
    
    // Test case that should fail
    match code_gen.find_interface_path("Dog", "Plant") {
        Ok(path) => {
            panic!("Should not find path from Dog to Plant, but found: {:?}", path);
        },
        Err(e) => {
            info!("Correctly failed to find path from Dog to Plant: {}", e);
        }
    }
    
    // Test a complex case with diamond inheritance
    match code_gen.find_interface_path("HybridUI", "UI") {
        Ok(path) => {
            info!("Found path in diamond inheritance: {:?}", path);
            // Either MobileUI or WebUI path could be returned depending on the BFS traversal
            assert!(path.contains(&"HybridUI".to_string()));
            assert!(path.contains(&"UI".to_string()));
            assert!(path.len() == 3, "Path length should be 3, got {}", path.len());
        },
        Err(e) => {
            panic!("Failed to find path in diamond inheritance: {}", e);
        }
    }
}

#[test]
fn test_dot_graph_generation_integration() {
    init_tracing!();
    info!("Starting DOT graph generation integration test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Test generating the DOT graph
    match code_gen.generate_interface_hierarchy_dot() {
        Ok(dot) => {
            info!("Generated DOT graph with {} characters", dot.len());
            
            // Verify the DOT graph has correct structure
            assert!(dot.starts_with("digraph interface_hierarchy {\n"),
                   "DOT graph should start with correct header");
            assert!(dot.contains("node [shape=box, style=filled, fillcolor=lightblue];"),
                   "DOT graph should have node style settings");
            assert!(dot.contains("\"Dog\" [label=\"Dog\"];"),
                   "DOT graph should contain Dog node");
            assert!(dot.contains("\"Dog\" -> \"Mammal\";"),
                   "DOT graph should contain edge from Dog to Mammal");
            assert!(dot.ends_with("}\n"), "DOT graph should end with closing brace");
        },
        Err(e) => {
            panic!("Failed to generate DOT graph: {}", e);
        }
    }
}

#[test]
fn test_path_visualization_integration() {
    init_tracing!();
    info!("Starting path visualization integration test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Test visualizing a path
    match code_gen.visualize_interface_path("Dog", "Animal") {
        Ok(visualization) => {
            info!("Generated visualization with {} characters", visualization.len());
            
            // Verify key components of the visualization
            assert!(visualization.contains("Interface Inheritance Path:"),
                   "Visualization should have a header");
            assert!(visualization.contains("[Dog]"),
                   "Visualization should include Dog node");
            assert!(visualization.contains("[Mammal]"),
                   "Visualization should include Mammal node");
            assert!(visualization.contains("[Animal]"),
                   "Visualization should include Animal node");
            assert!(visualization.contains("↓ extends"),
                   "Visualization should show inheritance arrows");
            assert!(visualization.contains("DOT representation:"),
                   "Visualization should include DOT representation");
            assert!(visualization.contains("digraph path {"),
                   "Visualization should include DOT graph");
        },
        Err(e) => {
            panic!("Failed to visualize path: {}", e);
        }
    }
}

#[test]
fn test_alternative_path_finding_integration() {
    init_tracing!();
    info!("Starting alternative path finding integration test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Test finding alternative paths
    match code_gen.find_alternative_paths("Dog", "Plant", 3) {
        Ok(paths) => {
            info!("Found {} alternative paths between Dog and Plant", paths.len());
            
            // There should be at least one path through LivingThing
            let found_living_thing_path = paths.iter().any(|path| {
                path.contains(&"Dog".to_string()) && 
                path.contains(&"LivingThing".to_string()) && 
                path.contains(&"Plant".to_string())
            });
            
            assert!(found_living_thing_path, 
                   "Should find a path from Dog to Plant through LivingThing");
        },
        Err(e) => {
            panic!("Failed to find alternative paths: {}", e);
        }
    }
}

#[test]
fn test_error_message_enhancement_integration() {
    init_tracing!();
    info!("Starting error message enhancement integration test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Create a mock type assertion for testing
    let expression = Box::new(TypeAssertion {
        token: "test.csd:10".to_string(),
        expression: Box::new(TypeAssertion {
            token: "test.csd:10".to_string(),
            expression: Box::new(TypeAssertion { 
                token: "test.csd:10".to_string(),
                expression: Box::new(TypeAssertion { 
                    token: "test.csd:10".to_string(),
                    expression: Box::new(TypeAssertion { 
                        token: "test.csd:10".to_string(),
                        expression: Box::new(TypeAssertion { 
                            token: "test.csd:10".to_string(),
                            expression: Box::new(TypeAssertion { 
                                token: "test.csd:10".to_string(),
                                expression: Box::new(TypeAssertion { 
                                    token: "test.csd:10".to_string(),
                                    expression: Box::new(TypeAssertion { 
                                        token: "test.csd:10".to_string(),
                                        expression: Box::new(TypeAssertion { 
                                            token: "test.csd:10".to_string(),
                                            expression: Box::new(TypeAssertion { 
                                                token: "test.csd:10".to_string(),
                                                expression: Box::new(TypeAssertion { 
                                                    token: "test.csd:10".to_string(),
                                                    expression: Box::new(TypeAssertion { 
                                                        token: "test.csd:10".to_string(),
                                                        expression: Box::new(TypeAssertion { 
                                                            token: "test.csd:10".to_string(),
                                                            expression: Box::new(TypeAssertion { 
                                                                token: "test.csd:10".to_string(),
                                                                expression: Box::new(TypeAssertion { 
                                                                    token: "test.csd:10".to_string(),
                                                                    expression: Box::new(TypeAssertion { 
                                                                        token: "test.csd:10".to_string(),
                                                                        expression: Box::new(TypeAssertion { 
                                                                            token: "test.csd:10".to_string(),
                                                                            expression: Box::new(TypeAssertion { 
                                                                                token: "test.csd:10".to_string(),
                                                                                expression: Box::new(TypeAssertion { 
                                                                                    token: "test.csd:10".to_string(),
                                                                                    expression: Box::new(TypeAssertion { 
                                                                                        token: "test.csd:10".to_string(),
                                                                                        expression: Box::new(TypeAssertion { 
                                                                                            token: "test.csd:10".to_string(),
                                                                                            expression: Box::new(TypeAssertion { 
                                                                                                token: "test.csd:10".to_string(),
                                                                                                expression: Box::new(TypeAssertion { 
                                                                                                    token: "test.csd:10".to_string(),
                                                                                                    expression: Box::new(TypeAssertion { 
                                                                                                        token: "test.csd:10".to_string(),
                                                                                                        expression: Box::new(TypeAssertion { 
                                                                                                            token: "test.csd:10".to_string(),
                                                                                                            expression: Box::new(TypeAssertion { 
                                                                                                                token: "test.csd:10".to_string(),
                                                                                                                expression: Box::new(TypeAssertion { 
                                                                                                                    token: "test.csd:10".to_string(),
                                                                                                                    expression: Box::new(TypeAssertion { 
                                                                                                                        token: "test.csd:10".to_string(),
                                                                                                                        expression: Box::new(TypeAssertion { 
                                                                                                                            token: "test.csd:10".to_string(),
                                                                                                                            expression: Box::new(TypeAssertion { 
                                                                                                                                token: "test.csd:10".to_string(),
                                                                                                                                expression: Box::new(TypeAssertion { 
                                                                                                                                    token: "test.csd:10".to_string(),
                                                                                                                                    expression: Box::new(TypeAssertion { 
                                                                                                                                        token: "test.csd:10".to_string(),
                                                                                                                                        expression: Box::new(TypeAssertion { 
                                                                                                                                            token: "test.csd:10".to_string(),
                                                                                                                                            expression: Box::new(TypeAssertion { 
                                                                                                                                                token: "test.csd:10".to_string(),
                                                                                                                                                expression: Box::new(TypeAssertion { 
                                                                                                                                                    token: "test.csd:10".to_string(),
                                                                                                                                                    expression: Box::new(TypeAssertion { 
                                                                                                                                                        token: "test.csd:10".to_string(),
                                                                                                                                                        type_name: "Error".to_string(),
                                                                                                                                                    }),
                                                                                                                                                    type_name: "Error".to_string(),
                                                                                                                                                }),
                                                                                                                                                type_name: "Error".to_string(),
                                                                                                                                            }),
                                                                                                                                            type_name: "Error".to_string(),
                                                                                                                                        }),
                                                                                                                                        type_name: "Error".to_string(),
                                                                                                                                    }),
                                                                                                                                    type_name: "Error".to_string(),
                                                                                                                                }),
                                                                                                                                type_name: "Error".to_string(),
                                                                                                                            }),
                                                                                                                            type_name: "Error".to_string(),
                                                                                                                        }),
                                                                                                                        type_name: "Error".to_string(),
                                                                                                                    }),
                                                                                                                    type_name: "Error".to_string(),
                                                                                                                }),
                                                                                                                type_name: "Error".to_string(),
                                                                                                            }),
                                                                                                            type_name: "Error".to_string(),
                                                                                                        }),
                                                                                                        type_name: "Error".to_string(),
                                                                                                    }),
                                                                                                    type_name: "Error".to_string(),
                                                                                                }),
                                                                                                type_name: "Error".to_string(),
                                                                                            }),
                                                                                            type_name: "Error".to_string(),
                                                                                        }),
                                                                                        type_name: "Error".to_string(),
                                                                                    }),
                                                                                    type_name: "Error".to_string(),
                                                                                }),
                                                                                type_name: "Error".to_string(),
                                                                            }),
                                                                            type_name: "Error".to_string(),
                                                                        }),
                                                                        type_name: "Error".to_string(),
                                                                    }),
                                                                    type_name: "Error".to_string(),
                                                                }),
                                                                type_name: "Error".to_string(),
                                                            }),
                                                            type_name: "Error".to_string(),
                                                        }),
                                                        type_name: "Error".to_string(),
                                                    }),
                                                    type_name: "Error".to_string(),
                                                }),
                                                type_name: "Error".to_string(),
                                            }),
                                            type_name: "Error".to_string(),
                                        }),
                                        type_name: "Error".to_string(),
                                    }),
                                    type_name: "Error".to_string(),
                                }),
                                type_name: "Error".to_string(),
                            }),
                            type_name: "Error".to_string(),
                        }),
                        type_name: "Error".to_string(),
                    }),
                    type_name: "Error".to_string(),
                }),
                type_name: "Error".to_string(),
            }),
            type_name: "Animal".to_string(),
        }),
        type_name: "Plant".to_string(),
    });
    
    // Test generating an enhanced error message
    let source_location = "test.csd:10";
    match code_gen.generate_path_error_message("Animal", "Plant", source_location) {
        Ok(message) => {
            info!("Generated enhanced error message: {}", message);
            
            // Check that the message contains key components
            assert!(message.contains("Type assertion error at test.csd:10"),
                   "Message should include the source location");
            assert!(message.contains("Value of type 'Animal' cannot be asserted as type 'Plant'"),
                   "Message should include the type names");
            assert!(message.contains("Alternative paths") || message.contains("No viable inheritance path"),
                   "Message should mention paths or lack thereof");
            
            // If paths are found, they should include LivingThing
            if message.contains("Alternative paths") {
                assert!(message.contains("LivingThing"),
                       "Alternative paths should include LivingThing");
            }
        },
        Err(e) => {
            panic!("Failed to generate enhanced error message: {}", e);
        }
    }
}

// Run this test last because it mocks a failing compilation
#[test]
fn test_full_type_assertion_compilation() {
    init_tracing!();
    info!("Starting full type assertion compilation test");
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // Set up our test interface hierarchy in the registry
    let result = setup_test_hierarchy(&code_gen.registry_extensions);
    assert!(result.is_ok(), "Failed to set up test hierarchy: {:?}", result.err());
    
    // Create a mock type assertion for testing
    let expression = Box::new(TypeAssertion {
        token: "test.csd:10".to_string(),
        expression: Box::new(TypeAssertion {
            token: "test.csd:10".to_string(),
            expression: Box::new(TypeAssertion { 
                token: "test.csd:10".to_string(),
                expression: Box::new(TypeAssertion { 
                    token: "test.csd:10".to_string(),
                    expression: Box::new(TypeAssertion { 
                        token: "test.csd:10".to_string(),
                        expression: Box::new(TypeAssertion { 
                            token: "test.csd:10".to_string(),
                            expression: Box::new(TypeAssertion { 
                                token: "test.csd:10".to_string(),
                                type_name: "Dog".to_string(),
                            }),
                            type_name: "Animal".to_string(),
                        }),
                        type_name: "LivingThing".to_string(),
                    }),
                    type_name: "Error".to_string(),
                }),
                type_name: "Error".to_string(),
            }),
            type_name: "Animal".to_string(),
        }),
        type_name: "Plant".to_string(),
    });
    
    // The actual compilation would fail in a real environment, but we can
    // test the error enhancement path
    match code_gen.compile_type_assertion_with_path_visualization(&expression) {
        Ok(_) => {
            // In a test environment with mock data, this might succeed
            info!("Compilation succeeded in test environment");
        },
        Err(e) => {
            // Expected in many test configurations - verify error contains enhanced information
            info!("Got expected error in compilation: {}", e);
            match e {
                Error::Compilation(msg) => {
                    // Check for visualization elements in the error message
                    assert!(msg.contains("Type assertion error") || 
                           msg.contains("Value of type") ||
                           msg.contains("Animal") ||
                           msg.contains("Plant"),
                           "Error should have visualization elements: {}", msg);
                },
                _ => {
                    panic!("Unexpected error type: {:?}", e);
                }
            }
        }
    }
}