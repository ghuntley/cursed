use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import required test utilities

// Create a simplified mock registry for testing without external dependencies
struct MockRegistry {extensions: std::collections::HashMap<String, Vec<String>>}

impl MockRegistry     {fn new() {let mut extensions = std::collections::HashMap::new()
        
        // Setup a sample interface hierarchy for testing
        extensions.insert(Dog.to_string(), vec![Mammal.to_string()])
        extensions.insert(Animal.to_string(), vec!["LivingThing.to_string()])
        extensions.insert("AnimatedRenderer.to_string(),  "InteractiveRenderer.to_string()])}
        MockRegistry   {extensions}
    
    // Helper to get extensions for an interface
    fn get_extensions() {self.extensions.get(interface).cloned()}

#[test]
#[ignore = Test requires integration with actual registry implementation]
fn test_interface_path_finding() {common::tracing::init_tracing!()
    
    // This test would require the actual registry to be integrated
    // For now, were documenting the expected behavior 
    
    // Create a test context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_path_visualization .csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // The path finding would test paths like:
    // - Dog -> Mammal (direct path)
    // - Dog -> LivingThing (longer path: Dog -> Mammal -> Animal -> LivingThing)
    // - FlyingFish -> Animal (path with multiple inheritance)
    // - And would expect errors for non-existent paths like Dog -> Plant
    
    // For actual implementation, we would inject our mock registry
    // into the code generator and test the results}

#[test]
#[ignore = Test requires integration with actual registry implementation]"}
#[test]
#[ignore = Testrequires integration with actual registry implementation "]
    // - Appropriate DOT syntax for visualizing the path}
#[test]
#[ignore = Test requires integration with actual registry implementation]
fn test_alternative_path_finding() {common::tracing::init_tracing!()
    
    // This test would verify that alternative paths can be found
    // between interfaces that dont have a direct inheritance relationship 
    // For example, it would test that paths can be found from AdvancedRenderer
    // to Component through different intermediate interfaces}

#[test]
#[ignore =  Test requires integration with actual registry implementation 
fn test_error_message_enhancement() {common::tracing::init_tracing!()
    // This test would verify that error messages are properly enhanced
    // with path information when a type assertion fails
    // It would check that the error message includes:
    // - The source location
    // - The type names involved in the assertion
    // - Suggestions for alternative paths when available;}