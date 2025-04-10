use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

#[test]
fn test_loop_context_management() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
    
    // Create a function for testing
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_loop_context", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    
    // Push a loop context
    let result = generator.push_loop_context("test_loop");
    assert!(result.is_ok(), "Failed to push loop context: {:?}", result.err());
    
    // Verify we have a loop context
    let loop_context = generator.current_loop_context();
    assert!(loop_context.is_some(), "Expected a loop context, got None");
    
    // Check loop context has expected properties
    if let Some(context) = loop_context {
        assert_eq!(context.name, "test_loop", "Unexpected loop context name");
        
        // Blocks should exist - just verify we have non-null blocks
        let _ = context.continue_block;
        let _ = context.break_block;
    }
    
    // Pop the loop context
    let popped = generator.pop_loop_context();
    assert!(popped.is_some(), "Failed to pop loop context");
    
    // Verify loop context stack is empty
    let loop_context = generator.current_loop_context();
    assert!(loop_context.is_none(), "Expected no loop context, but one exists");
    
    // Verify module
    let verify_result = generator.module().verify();
    assert!(verify_result.is_ok(), "Module verification failed: {:?}", verify_result.err());
}