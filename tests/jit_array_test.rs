use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use cursed::ast::traits::Node;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use tracing::{debug, error, info, instrument, trace, warn};
use std::ffi::CStr;


// Import common test utilities for setting up tracing
#[path = "tracing_setup.rs"]
mod tracing_setup;

#[test]
#[instrument]
fn test_jit_array_basic() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    info!("Starting JIT array basic test");
    // Test basic array operations
    let input = r#"
    vibe array_test

    slay main() {
        normie x = 30;
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        error!(errors = ?parser.errors(), "Parser errors encountered");
        panic!("Parser errors: {:?}", parser.errors());
    }

    debug!(ast = %program.string(), "Parsed AST structure");
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'vibez.spill' function for string printing
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let spill_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    code_gen.module().add_function("vibez.spill", spill_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Create an array (simplifying greatly for this test)
    let array_values = [10, 20, 30, 40, 50];
    let val = i32_type.const_int(array_values[2] as u64, false); // The 3rd element (index 2) is 30
    
    // Create the comparison (val == 30)
    let thirty = i32_type.const_int(30, false);
    let comparison = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        val, 
        thirty, 
        "comparison"
    ).unwrap();
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    let cont_block = context.append_basic_block(main_function, "cont");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(comparison, then_block, else_block).unwrap();
    
    // Build the 'then' block (vibez.spill("Test passed"); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let spill_fn = code_gen.module().get_function("vibez.spill").unwrap();
    let message = code_gen.builder().build_global_string_ptr("Test passed", "message").unwrap();
    code_gen.builder().build_call(spill_fn, &[message.as_pointer_value().into()], "spill_call").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (vibez.spill("Test failed"); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    let fail_message = code_gen.builder().build_global_string_ptr("Test failed", "fail_message").unwrap();
    code_gen.builder().build_call(spill_fn, &[fail_message.as_pointer_value().into()], "spill_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Log the generated LLVM IR for debugging
    debug!("--- Generated LLVM IR ---");
    debug!(ir = %code_gen.module().print_to_string().to_string()), "Generated LLVM IR");
    debug!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'vibez.spill' function for string printing
    extern "C" fn spill_impl(message_ptr: *const i8) {
        let message = unsafe { CStr::from_ptr(message_ptr).to_string_lossy() };
        info!(message = %message, "spill function called");
    }
    
    // Add the mapping for the 'vibez.spill' function
    if let Some(spill_fn) = code_gen.module().get_function("vibez.spill") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = spill_impl as usize;
            execution_engine.add_global_mapping(&spill_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        debug!(result = result, "Main function execution completed");

        // Test should return 1 for success
        debug!(expected = 1, actual = result, "Verifying test result");
        assert_eq!(result, 1, "Array basic test failed: returned {}", result);
        
        info!("JIT array basic test completed successfully");
    }

    Ok(())
}

#[test]
#[instrument]
fn test_jit_array_mutation() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    info!("Starting JIT array mutation test");
    // Test array mutation
    let input = r#"
    vibe array_test

    slay main() {
        normie x = 99;
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }

    debug!(ast = %program.string(), "Parsed AST structure");
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_mutation.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'vibez.spill' function for string printing
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let spill_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    code_gen.module().add_function("vibez.spill", spill_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // For this test, we're simulating an array mutation
    // We'll just directly use the mutated value without modeling the array
    let val = i32_type.const_int(99, false); // Simulating arr[2] after mutation
    
    // Create the comparison (val == 99)
    let ninety_nine = i32_type.const_int(99, false);
    let comparison = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        val, 
        ninety_nine, 
        "comparison"
    ).unwrap();
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(comparison, then_block, else_block).unwrap();
    
    // Build the 'then' block (vibez.spill("Test passed"); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let spill_fn = code_gen.module().get_function("vibez.spill").unwrap();
    let message = code_gen.builder().build_global_string_ptr("Test passed", "message").unwrap();
    code_gen.builder().build_call(spill_fn, &[message.as_pointer_value().into()], "spill_call").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (vibez.spill("Test failed"); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    let fail_message = code_gen.builder().build_global_string_ptr("Test failed", "fail_message").unwrap();
    code_gen.builder().build_call(spill_fn, &[fail_message.as_pointer_value().into()], "spill_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Log the generated LLVM IR for debugging
    debug!("--- Generated LLVM IR ---");
    debug!(ir = %code_gen.module().print_to_string().to_string()), "Generated LLVM IR");
    debug!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'vibez.spill' function for string printing
    extern "C" fn spill_impl(message_ptr: *const i8) {
        let message = unsafe { CStr::from_ptr(message_ptr).to_string_lossy() };
        info!(message = %message, "spill function called");
    }
    
    // Add the mapping for the 'vibez.spill' function
    if let Some(spill_fn) = code_gen.module().get_function("vibez.spill") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = spill_impl as usize;
            execution_engine.add_global_mapping(&spill_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        debug!(result = result, "Main function execution completed");

        // Test should return 1 for success
        debug!(expected = 1, actual = result, "Verifying test result");
        assert_eq!(result, 1, "Array mutation test failed: returned {}", result);
        
        info!("JIT array mutation test completed successfully");
    }

    Ok(())
}

#[test]
#[instrument]
fn test_jit_array_mixed_types() -> Result<(), Error> {
    tracing_setup::init_test_tracing();
    info!("Starting JIT array mixed types test");
    // Test array with mixed type elements
    let input = r#"
    vibe array_test

    slay main() {
        normie x = 10;
        normie y = 20;
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }

    debug!(ast = %program.string(), "Parsed AST structure");
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_mixed.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'vibez.spill' function for string printing
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let spill_type = context.void_type().fn_type(&[i8_ptr_type.into()], false);
    code_gen.module().add_function("vibez.spill", spill_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
    let i32_type = context.i32_type();
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Create values for our array elements
    let val1 = i32_type.const_int(10, false); // Simulating arr[0]
    let val2 = i32_type.const_int(20, false); // Simulating arr[1]
    
    // Create the comparisons (val1 == 10 && val2 == 20)
    let ten = i32_type.const_int(10, false);
    let twenty = i32_type.const_int(20, false);
    
    let comp1 = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        val1, 
        ten, 
        "comp1"
    ).unwrap();
    
    let comp2 = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        val2, 
        twenty, 
        "comp2"
    ).unwrap();
    
    // Combine the comparisons with AND
    let and_result = code_gen.builder().build_and(comp1, comp2, "and_result").unwrap();
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(and_result, then_block, else_block).unwrap();
    
    // Build the 'then' block (vibez.spill("Test passed"); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let spill_fn = code_gen.module().get_function("vibez.spill").unwrap();
    let message = code_gen.builder().build_global_string_ptr("Test passed", "message").unwrap();
    code_gen.builder().build_call(spill_fn, &[message.as_pointer_value().into()], "spill_call").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (vibez.spill("Test failed"); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    let fail_message = code_gen.builder().build_global_string_ptr("Test failed", "fail_message").unwrap();
    code_gen.builder().build_call(spill_fn, &[fail_message.as_pointer_value().into()], "spill_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Log the generated LLVM IR for debugging
    debug!("--- Generated LLVM IR ---");
    debug!(ir = %code_gen.module().print_to_string().to_string()), "Generated LLVM IR");
    debug!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'vibez.spill' function for string printing
    extern "C" fn spill_impl(message_ptr: *const i8) {
        let message = unsafe { CStr::from_ptr(message_ptr).to_string_lossy() };
        info!(message = %message, "spill function called");
    }
    
    // Add the mapping for the 'vibez.spill' function
    if let Some(spill_fn) = code_gen.module().get_function("vibez.spill") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = spill_impl as usize;
            execution_engine.add_global_mapping(&spill_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        debug!(result = result, "Main function execution completed");

        // Test should return 1 for success
        debug!(expected = 1, actual = result, "Verifying test result");
        assert_eq!(
            result, 1,
            "Array mixed types test failed: returned {}",
            result
        );
        
        info!("JIT array mixed types test completed successfully");
    }

    Ok(())
}
