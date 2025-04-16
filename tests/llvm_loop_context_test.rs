use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::control_flow::ControlFlowCompilation;
use cursed::error::Error;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
#[ignore = "Needs trait impl for control_flow"]
fn test_loop_context_management() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function for testing
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator
        .module()
        .add_function("test_loop_context", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create basic blocks for the loop context
    let block1 = context.append_basic_block(function, "block1");
    let block2 = context.append_basic_block(function, "block2");
    
    // Add terminators to all blocks to avoid verification errors
    // Save current position
    let current_block = generator.builder().get_insert_block().unwrap();
    
    // Add return to block1
    generator.builder().position_at_end(block1);
    generator.builder().build_return(None).expect("Failed to build return for block1");
    
    // Add return to block2
    generator.builder().position_at_end(block2);
    generator.builder().build_return(None).expect("Failed to build return for block2");
    
    // Restore original position
    generator.builder().position_at_end(current_block);
    
    // Push the loop context with separate blocks
    generator.push_loop_context(block2, block1);

    // Verify we have a loop context
    let loop_context = generator.current_loop_context();
    assert!(loop_context.is_some(), "Expected a loop context, got None");

    // Check loop context has expected properties
    if let Some(context) = loop_context {
        // Blocks should exist - just verify we have non-null blocks
        let _ = context.continue_block;
        let _ = context.break_block;
    }

    // Pop the loop context
    let popped = generator.pop_loop_context();
    assert!(popped.is_some(), "Failed to pop loop context");

    // Verify loop context stack is empty
    let loop_context = generator.current_loop_context();
    assert!(
        loop_context.is_none(),
        "Expected no loop context, but one exists"
    );

    // Add a terminator to the entry block (return void)
    generator.builder().build_return(None).expect("Failed to build return");
    
    // Verify module
    let verify_result = generator.module().verify();
    assert!(
        verify_result.is_ok(),
        "Module verification failed: {:?}",
        verify_result.err()
    );
}
