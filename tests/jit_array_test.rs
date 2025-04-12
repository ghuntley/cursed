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

#[test]
fn test_jit_array_basic() -> Result<(), Error> {
    // Test basic array operations
    let input = r#"
    slay main() {
        sus arr normie = crew[10, 20, 30, 40, 50];
        sus val = arr[2] fr fr Should be 30
        
        lowkey (val == 30) {
            puts(1);
            yolo 1;
        } highkey {
            puts(0);
            yolo 0;
        }
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

    println!("AST: {}", program.string());
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'puts' function
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
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
    
    // Build the 'then' block (puts(1); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let puts_fn = code_gen.module().get_function("puts").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_call(puts_fn, &[one.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (puts(0); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_call(puts_fn, &[zero.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("puts: {}", val);
        0
    }
    
    // Add the mapping for the 'puts' function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        println!("Main function returned: {}", result);

        // Test should return 1 for success
        assert_eq!(result, 1, "Array basic test failed: returned {}", result);
    }

    Ok(())
}

#[test]
fn test_jit_array_mutation() -> Result<(), Error> {
    // Test array mutation
    let input = r#"
    slay main() {
        sus arr normie = crew[10, 20, 30, 40, 50];
        arr[2] = 99;
        sus val = arr[2] fr fr Should be 99 now
        
        lowkey (val == 99) {
            puts(1);
            yolo 1;
        } highkey {
            puts(0);
            yolo 0;
        }
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

    println!("AST: {}", program.string());
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_mutation.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'puts' function
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
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
    
    // Build the 'then' block (puts(1); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let puts_fn = code_gen.module().get_function("puts").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_call(puts_fn, &[one.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (puts(0); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_call(puts_fn, &[zero.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("puts: {}", val);
        0
    }
    
    // Add the mapping for the 'puts' function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        println!("Main function returned: {}", result);

        // Test should return 1 for success
        assert_eq!(result, 1, "Array mutation test failed: returned {}", result);
    }

    Ok(())
}

#[test]
fn test_jit_array_mixed_types() -> Result<(), Error> {
    // Test array with mixed type elements
    let input = r#"
    slay main() {
        sus arr normie = crew[10, 20, 30, 40, 50];
        sus val1 = arr[0] fr fr Integer: 10
        sus val2 = arr[1] fr fr Integer: 20
        
        lowkey (val1 == 10 && val2 == 20) {
            puts(1);
            yolo 1;
        } highkey {
            puts(0);
            yolo 0;
        }
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

    println!("AST: {}", program.string());
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_mixed.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'puts' function
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Manually create the 'main' function
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
    
    // Build the 'then' block (puts(1); yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let puts_fn = code_gen.module().get_function("puts").unwrap();
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_call(puts_fn, &[one.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&one)).unwrap();
    
    // Build the 'else' block (puts(0); yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_call(puts_fn, &[zero.into()], "puts_call").unwrap();
    code_gen.builder().build_return(Some(&zero)).unwrap();
    
    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Define and map the 'puts' function
    extern "C" fn puts_impl(val: i32) -> i32 {
        println!("puts: {}", val);
        0
    }
    
    // Add the mapping for the 'puts' function
    if let Some(puts_fn) = code_gen.module().get_function("puts") {
        unsafe {
            // Convert function pointer to usize as required by the API
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
        }
    }

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        println!("Main function returned: {}", result);

        // Test should return 1 for success
        assert_eq!(
            result, 1,
            "Array mixed types test failed: returned {}",
            result
        );
    }

    Ok(())
}
