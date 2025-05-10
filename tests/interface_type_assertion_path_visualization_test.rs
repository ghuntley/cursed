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

// Import required test utilities
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;

// Create a simplified mock registry for testing without external dependencies
struct MockRegistry {
    extensions: std::collections::HashMap<String, Vec<String>>,
}

impl MockRegistry {
    fn new() -> Self {
        let mut extensions = std::collections::HashMap::new();
        
        // Setup a sample interface hierarchy for testing
        extensions.insert("Dog".to_string(), vec!["Mammal".to_string()]);
        extensions.insert("Cat".to_string(), vec!["Mammal".to_string()]);
        extensions.insert("Mammal".to_string(), vec!["Animal".to_string()]);
        extensions.insert("Bird".to_string(), vec!["Animal".to_string()]);
        extensions.insert("Reptile".to_string(), vec!["Animal".to_string()]);
        extensions.insert("Animal".to_string(), vec!["LivingThing".to_string()]);
        extensions.insert("Plant".to_string(), vec!["LivingThing".to_string()]);
        
        // Multiple inheritance
        extensions.insert("FlyingFish".to_string(), vec!["Fish".to_string(), "Flying".to_string()]);
        extensions.insert("Fish".to_string(), vec!["Animal".to_string()]);
        extensions.insert("Flying".to_string(), vec!["MovementType".to_string()]);
        
        // More complex relationships for renderer interfaces
        extensions.insert("Renderer".to_string(), vec!["Component".to_string()]);
        extensions.insert("AnimatedRenderer".to_string(), vec!["Renderer".to_string()]);
        extensions.insert("InteractiveRenderer".to_string(), vec!["Renderer".to_string()]);
        extensions.insert("AdvancedRenderer".to_string(), 
            vec!["AnimatedRenderer".to_string(), "InteractiveRenderer".to_string()]);
        
        MockRegistry { extensions }
    }
    
    // Helper to get extensions for an interface
    fn get_extensions(&self, interface: &str) -> Option<Vec<String>> {
        self.extensions.get(interface).cloned()
    }
}
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_interface_path_finding() {
    init_tracing!();
    
    // This test would require the actual registry to be integrated
    // For now, we're documenting the expected behavior
    
    // Create a test context
    let context = Context::create();
    let file_path = PathBuf::from("test_path_visualization.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test", file_path);
    
    // The path finding would test paths like:
    // - Dog -> Mammal (direct path)
    // - Dog -> LivingThing (longer path: Dog -> Mammal -> Animal -> LivingThing)
    // - FlyingFish -> Animal (path with multiple inheritance)
    // - And would expect errors for non-existent paths like Dog -> Plant
    
    // For actual implementation, we would inject our mock registry
    // into the code generator and test the results
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_dot_graph_generation() {
    init_tracing!();
    
    // This test would verify that the DOT graph generation works correctly
    // It would check that the output starts with "digraph interface_hierarchy {" 
    // and ends with "}\n", and contains appropriate node and edge definitions
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_path_visualization() {
    init_tracing!();
    
    // This test would verify the path visualization functionality
    // It would check that the visualization contains the expected elements:
    // - "Interface Inheritance Path:" header
    // - The nodes in the path (e.g., [Dog], [Mammal], [Animal])
    // - "DOT representation:" section
    // - Appropriate DOT syntax for visualizing the path
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_alternative_path_finding() {
    init_tracing!();
    
    // This test would verify that alternative paths can be found
    // between interfaces that don't have a direct inheritance relationship
    // For example, it would test that paths can be found from AdvancedRenderer
    // to Component through different intermediate interfaces
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_error_message_enhancement() {
    init_tracing!();
    
    // This test would verify that error messages are properly enhanced
    // with path information when a type assertion fails
    // It would check that the error message includes:
    // - The source location
    // - The type names involved in the assertion
    // - Suggestions for alternative paths when available
}