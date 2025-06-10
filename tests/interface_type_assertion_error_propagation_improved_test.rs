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
fn test_improved_error_propagation_registration() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    // Call the registration function to ensure it exists and doesn t panic
    register_interface_type_assertion_error_propagation()}

#[test]
fn test_generate_type_assertion_error() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    // Create a simple test context
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module = context.create_module(test);
    // Create a mock code generator
    struct MockGenerator<ctx>   {context: &ctx Context,"}
        module: inkwell::module::Module<ctx>,", "> MockGenerator<ctx> {fn new(} {})
    impl<ctx> ImprovedTypeAssertion<ctx> for MockGenerator<"ctx>   {"}
    impl<, "> InterfaceTypeAssertion<ctx> for MockGenerator<"
        fn extract_interface_type_id(} {, "> InterfaceTypeAssertionPathVisualization<"ctx> for MockGenerator<ctx>   {,  interface hierarchy "visualization.to_string(}"}))
         ", "
        Some(,  error ".expect(Failed to generate error)".csd:42:, 10)"
    assert!(error.message.contains(,  inheritance path exists)"Mock interface hierarchy visualization)"
        panic!(Expected:  a recovery hint)"}"
    assert!(error_string.contains(SourceTypeis not a TargetType)"")
    impl<ctx> InterfaceTypeAssertion<ctx> for RecoveryTestGenerator<"
        fn extract_interface_type_id() {", .to_string(}")
    impl<", > for RecoveryTestGenerator<ctx>   {"}
        fn visualize_interface_hierarchy(} {Ok(", .to_string(}")))
        fn detect_reversed_inheritance_enhanced() {Ok((self.reversed_relationship,  Reversed  relationship detected.to_string(}",  G {A -> B;}.to_string()")))
            .expect(Failed to get recovery ")
            .expect(Should have recovery hint)""
            .expect(Failed to get recovery , "";")
        assert!(hint.contains(reversed);",  asserting TypeB ""fixed")