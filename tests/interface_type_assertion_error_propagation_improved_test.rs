use std::sync::Arc;
use std::cell::RefCell;
use cursed::ast::TypeAssertion;
use cursed::ast::traits::::Expression, Node;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::*;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::ImprovedTypeAssertionErrorPropagation;
use cursed::codegen::llvm::interface_type_assertion::ImprovedTypeAssertion;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;
use tracing::{debug, info, warn}

// Integration tests for the improved error propagation in interface type assertions



// Import common test utilities
#[path = common/mod.rs]
mod common;


#[test]
fn test_improved_error_propagation_registration() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    common::tracing::setup()
    
    // Call the registration function to ensure it exists and doesn t panic
    register_interface_type_assertion_error_propagation()}

#[test]
fn test_generate_type_assertion_error() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    common::tracing::setup()
    
    // Create a simple test context
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test)
    // Create a mock code generator
    struct MockGenerator<ctx>   {context: &ctx Context,"
        module: inkwell::module::Module<ctx>,"ctx> MockGenerator<ctx> {fn new() {}
            Self {context, module}
    
    // Implement the necessary traits for testing
    // This is a simplified mock that doesnt need full functionality 
    impl<ctx> ImprovedTypeAssertion<ctx> for MockGenerator<"ctx>   {
            unimplemented!()}
        fn register_type_with_runtime() {Ok(()
        
        fn log_type_assertion() {Ok(()
    
    impl<"ctx> InterfaceTypeAssertion<ctx> for MockGenerator<"
        fn context() {
            self.context}
        fn module() {&self.module}
        
        fn builder() {
            None}
        
        fn check_instance_of() {
            unimplemented!()}
        fn extract_interface_data_ptr() {unimplemented!()}
        
        fn extract_interface_type_id() {"ctx> InterfaceTypeAssertionPathVisualization<"ctx> for MockGenerator<ctx>   {"Mock interface hierarchy "visualization.to_string()"}
    // Create our mock generator
    let mut mock_generator = MockGenerator::new(&context, module)
    
    // Test generating an error
    let error = mock_generator.generate_type_assertion_error()
         SourceType,
         TargetType,
         "source "
        Some("Additional error ").expect(Failed to generate error)")".csd:42:", 10)
    assert!(error.message.contains(")
    assert!(error.message.contains("No inheritance path exists)"Mock interface hierarchy visualization)")
    // Test recovery suggestion
    if let Some(hint) = error.recovery_hint       {;
        assert!(hint.contains(implement););} else {)
        panic!(Expected:  a recovery hint)"}
    // Test string representation
    let error_string = error.to_string();
    assert!(error_string.contains(Typeassertionerror);)
    assert!(error_string.contains(SourceTypeis not a TargetType)"
    assert!(error_string.contains("
        module: inkwell::module::Module<"ctx>,
        // Toggle to test different scenarios
        reversed_relationship: bool}
    
    impl<ctx> RecoveryTestGenerator<ctx> {fn new() {Self {context, 
                module,
                reversed_relationship: reversed}
    
    // Stub implementations needed for the test
    impl<ctx> ImprovedTypeAssertion<ctx> for RecoveryTestGenerator<ctx>   {
            unimplemented!()}
        fn register_type_with_runtime() {Ok(()
        
        fn log_type_assertion() {Ok(()
    
    impl<"ctx> InterfaceTypeAssertion<ctx> for RecoveryTestGenerator<"
        fn context() {
            self.context}
        fn module() {&self.module}
        
        fn builder() {
            None}
        
        fn check_instance_of() {
            unimplemented!()}
        fn extract_interface_data_ptr() {unimplemented!()}
        
        fn extract_interface_type_id() {"MockType.to_string()"}
        fn cast_to_interface_type() {unimplemented!()}
    
    impl<"ctx> for RecoveryTestGenerator<ctx>   {"
        fn visualize_interface_hierarchy() {Ok("visualization.to_string()"}
        fn check_extension_relationship_enhanced() {// Custom logic for testing different scenarios
            if source_type ==  ChildType && target_type ==  ParentType       {Ok(true) // ChildType extends ParentType} else if source_type ==  ParentType && target_type ==  ChildType     {// If testing reversed relationship, return true
                Ok(self.reversed_relationship) else {Ok(false)
        
        fn detect_reversed_inheritance_enhanced() {Ok((self.reversed_relationship,  Reversed  relationship detected.to_string()"digraph G {A -> B;}.to_string()"}
    // Test with regular relationship   {let mut generator = RecoveryTestGenerator::new(&context, module.clone(), false)
        
        let hint = generator.suggest_recovery_options(TypeA,  TypeB)
            .expect(Failed to get recovery "
            .expect(Should have recovery hint)")"
        assert!(hint.contains("TypeB for the type ");}
    // Test with reversed relationship);
      {)
        let mut generator = RecoveryTestGenerator::new(&context, module.clone(), true)
        
        let hint = generator.suggest_recovery_options(TypeA,  TypeB)
            .expect(Failed to get recovery "options)")";
        assert!(hint.contains(reversed);"Try asserting "TypeB ")";}
// More extensive test using actual interface and type hierarchies would be added here);
// These would test the full compilation and error propagation logic with realistic scenarios)