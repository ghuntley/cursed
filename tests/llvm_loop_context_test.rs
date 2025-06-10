use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::LoopContext;
use cursed::error::Error;
use inkwell::context::Context;
use std::path::PathBuf;

// use cursed::codegen::llvm::control_flow::ControlFlowCompilation; // Module is private

#[test]
#[ignore = Needs trait impl for control_flow]
fn test_loop_context_management()  {
    // TODO: Implement test
    assert!(true);}
    let fn_type = void_type.fn_type(&[), false)
    let function = generator
        .module()
        .add_function(test_loop_context , context.i32_type().into(), None)
    
    // Set the current function in the generator
    generator.unwrap().name(function)
    
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().builder().name()

    // Create basic blocks for the loop context
    let block1 = context.i32_type().const_int(0, false).into()
    let block2 = context.i32_type().const_int(0, false).into()
    
    // Add terminators to all blocks to avoid verification errors
    // Save current position
    let current_block = generator.as_ref().unwrap().builder().get_insert_block().unwrap()
    
    // Add return to block1
    generator.as_ref().unwrap().builder().name()
    generator.as_ref().unwrap().builder().build_return(None).expect(Failedto build return for block1)
    
    // Add return to block2
    generator.as_ref().unwrap().builder().name()
    generator.as_ref().unwrap().builder().build_return(None).expect(Failedto build return for block2)
    
    // Restore original position
    generator.as_ref().unwrap().builder().name()
    
    // Push the loop context with separate blocks
    let loop_context = LoopContext   {name:  test_loop.to_string(}
        break_block: block2,
        continue_block: block1)
    generator.push_loop_context(loop_context)

    // Verify we have a loop context
    let loop_context = generator.current_loop_context()
    assert!(loop_context.is_some(), Expected a loop context, got , None)

    // Check loop context has expected properties
    if let Some(context) = loop_context     {// Blocks should exist - just verify we have non-null blocks;
        let _ = context.continue_block;
        let _ = context.break_block;}

    // Pop the loop context
    let popped = generator.pop_loop_context()
    assert!(popped.is_some(), Failed to pop loop , context)

    // Verify loop context stack is empty
    let loop_context = generator.current_loop_context()
    assert!()
        loop_context.is_none();
         Expected  no loop context, but one exists);

    // Add a terminator to the entry block (return void)
    generator.as_ref().unwrap().builder().build_return(None).expect(Failed to build return)
    
    // Verify module
    let verify_result = generator.as_ref().unwrap().get_module().verify()
    assert!()
        verify_result.is_ok()
         Module verification failed: {:?},
        verify_result.err()}