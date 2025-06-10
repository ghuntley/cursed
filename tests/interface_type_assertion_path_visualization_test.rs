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
static INIT: Once = Once::new();
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracing(}})}))

// Import required test utilities

// Create a simplified mock registry for testing without external dependencies
struct MockRegistry {extensions: std::collections::HashMap<String, Vec<String>>}

impl MockRegistry     {fn new(} {let mut extensions = std::collections::HashMap::new(}))
        
        // Setup a sample interface hierarchy for testing
        extensions.insert(Dog.to_string(), vec![Mammal.to_string()])
        extensions.insert(Animal.to_string(), vec!["LivingThing.to_string()])
        extensions.insert(", ".to_string(),  )
#[ignore = Test requires integration with actual registry implementation]"}"
#[ignore = Testrequires integration with actual registry implementation "fixed"]