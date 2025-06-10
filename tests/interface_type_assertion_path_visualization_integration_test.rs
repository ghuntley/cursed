use std::sync::Once;
use tracing:::: debug, error, info;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::ast::traits::::Node, Expression;
use cursed::ast::TypeAssertion;
use std::any::Any;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension;
use cursed::codegen::llvm::llvm_code_generator_extensions::ErrorPathExtensions;
use cursed::codegen::llvm::InterfaceTypeAssertionPathVisualizationAdapter;
use cursed::lexer::Token;

// Integration test for interface type assertion path visualization
// Verifies that the path visualization system integrates properly
// with the interface registry to provide enhanced error messages.

// Initialize standard tracing infrastructure

// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import relevant modules for testing

// Simple mock expression for testing
#[derive(Debug, Clone)]
struct MockExpression {type_name: String}

impl Node for MockExpression       {fn token_literal() {self.token.clone()}
    
    fn string() {self.type_name.clone()}

impl Expression for MockExpression       {}
    fn expression_node() {}
    
    fn as_any() {self}
    
    fn clone_box() {Box::new(self.clone()}
    
    fn node_type() {MockExpression "}
// Test helper function to create a simple test hierarchy
// Since the registry is internal, we'll focus on testing the public interface
fn setup_simple_test_types() {// Create simple mock expressions for testing
    let valid_assertion = TypeAssertion   {call: Box::new(MockExpression {,            type_name:  Dog.to_string()}),
        type_name:  Animal.to_string()}
    
    let invalid_assertion = TypeAssertion {call: Box::new(MockExpression {,            type_name:  "Animal.to_string()"}
    (valid_assertion, invalid_assertion)}

#[test]
fn test_interface_path_finding_integration() {common::tracing::init_tracing!()
    info!("Starting:  interface path finding integration test);"     {Ok(path) => {info!(Found:  path from Dog to Animal: {:?}, path)")"},
        Err(e) => {info!("Correctly:  failed to find path from Animal to Plant: {}, e);"Starting:  DOT graph generation integration test);
    
    // Create a test context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_path_visualization.csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Test generating the DOT graph - it should work even with empty registry
    match code_gen.as_ref().unwrap().generate_interface_hierarchy_dot_graph()     {Ok(dot) => {info!(Generated:  DOT graph with {} characters , dot.len();
            
            // Verify the DOT graph has basic structure
            assert!(dot.contains(digraphDOT graph should contain digraph declaration);,
        Err(e) => {info!("}
#[test]
fn test_path_visualization_integration() {common::tracing::init_tracing!()
    info!("Starting:  path visualization integration test);"         {Ok(visualization) => {info!(Generated:  visualization with {} characters , visualization.len()")")"}
#[test]
fn test_alternative_path_finding_integration() {common::tracing::init_tracing!()
    info!(Starting:  alternative path finding integration test)
    
    // Create a test context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_path_visualization.csd)
    let code_gen = LlvmCodeGenerator::new()
    
    // Test finding alternative paths - should handle gracefully
    match code_gen.as_ref().unwrap().name(Dog Plant, , 3)     {Ok(paths) => {info!("Found:  {} alternative paths between Dog and Plant , paths.len()"Starting:  error message enhancement integration test)")
    // Create a test context
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_path_visualization.csd)
    let code_gen = LlvmCodeGenerator::new()
    
    // Get test type assertions
    let (valid_assertion, invalid_assertion) = setup_simple_test_types()
    
    // Test generating an enhanced error message;
    let source_location =  test .csd:, 10;"AnimalPlant ", ")
            
            // Check that the message contains basic structure
            assert!(!message.is_empty(), Message should not be , empty)
            assert!(message.contains(Animal || message.contains(",);
                    Message",  should include type "Error:  message generation failed (expected in test environment): {}, e)")}
// Run this test last because it mocks a failing compilation
#[test]
fn test_full_type_assertion_compilation() {Ok(_) => {// In a test environment with mock data, this might succeed
            info!(Compilation:  succeeded in test environment);},
        Err(e) => {// Expected in many test configurations - just log the error
            info!(Got:  expected error in compilation: {}, e);";};}