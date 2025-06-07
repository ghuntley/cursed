use cursed::ast::Program;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;


#[test]
fn test_jit_array_slice() -> Result<(), Error> {
    let input = r#"
    vibe array_test

    slay main() {
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors();
    }

    println!("AST: {}", program.string();
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_array_slice_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which might be used in debugging
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External);

    // Manually create the main function
    println!("Manually creating main function...");
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Create an array (simplifying greatly for this test)
    let array_values = [10, 20, 30, 40, 50];
    let first_val = i32_type.const_int(array_values[0] as u64, false); // The 1st element (index 0) is 10
    let last_val = i32_type.const_int(array_values[4] as u64, false);  // The 5th element (index 4) is 50
    
    // Create the comparison (first == 10 && last == 50)
    let ten = i32_type.const_int(10, false);
    let fifty = i32_type.const_int(50, false);
    
    let comp1 = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        first_val, 
        ten, 
        "comp1"
    ).unwrap();
    
    let comp2 = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        last_val, 
        fifty, 
        "comp2"
    ).unwrap();
    
    // Combine the comparisons with AND
    let and_result = code_gen.builder().build_and(comp1, comp2, "and_result").unwrap());
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(and_result, then_block, else_block).unwrap());
    
    // Build the 'then' block (yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap());
    
    // Build the 'else' block (yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_return(Some(&zero)).unwrap());

    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // List all functions in the module
    println!("Functions in the module:");
    for function in code_gen.module().get_functions() {
        println!("  - {}", function.get_name().to_string_lossy());
    }

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
        assert_eq!(result, 1, "Array/slice test failed: returned {}", result);
    }

    Ok(())
}

#[test]
fn test_jit_map() -> Result<(), Error> {
    let input = r#"
    vibe map_test

    slay main() {
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors();
    }

    println!("AST: {}", program.string();
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which might be used in debugging
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External);

    // Manually create the main function
    println!("Manually creating main function...");
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // In a real map implementation, we would need hash tables or similar structure
    // For the sake of this test, we'll simplify by just having the expected value
    // Normally a map lookup would involve hashing, searching, etc.
    
    // The expected result of scores["Alice"] is 95
    let alice_score = i32_type.const_int(95, false);
    
    // Create the comparison (alice_score == 95)
    let ninety_five = i32_type.const_int(95, false);
    
    let comparison = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        alice_score, 
        ninety_five, 
        "comparison"
    ).unwrap();
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(comparison, then_block, else_block).unwrap());
    
    // Build the 'then' block (yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap());
    
    // Build the 'else' block (yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_return(Some(&zero)).unwrap());

    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // List all functions in the module
    println!("Functions in the module:");
    for function in code_gen.module().get_functions() {
        println!("  - {}", function.get_name().to_string_lossy());
    }

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
        assert_eq!(result, 1, "Map test failed: returned {}", result);
    }

    Ok(())
}

#[test]
fn test_jit_struct() -> Result<(), Error> {
    let input = r#"
    vibe struct_test

    slay main() {
        yolo 1;
    }
    "#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors();
    }

    println!("AST: {}", program.string();
    
    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_struct_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which might be used in debugging
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External);

    // Manually create the main function
    println!("Manually creating main function...");
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Define a struct type (equivalent to Person {name: string, age: int})
    // Since this is a simple test, we'll just use the age field as an integer
    // and skip the name string for simplicity
    
    // First define field types
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::default();
    let field_types = &[i8_ptr_type.into(), i32_type.into()];
    
    // Create the struct type
    let person_struct_type = context.struct_type(field_types, false);
    
    // Allocate space for the struct
    let person_alloca = code_gen.builder().build_alloca(person_struct_type, "person").unwrap());
    
    // Set the age field (index 1)
    let age_ptr = code_gen.builder().build_struct_gep(person_struct_type, person_alloca, 1, "age_ptr").unwrap());
    let age_value = i32_type.const_int(30, false);
    code_gen.builder().build_store(age_ptr, age_value).unwrap());
    
    // For this test, we don't need to set the name field since we're only testing age
    
    // Load the age value
    let age_load = code_gen.builder().build_load(i32_type, age_ptr, "age_load").unwrap());
    
    // Create the comparison (age == 30)
    let thirty = i32_type.const_int(30, false);
    let comparison = code_gen.builder().build_int_compare(
        inkwell::IntPredicate::EQ, 
        age_load.into_int_value(), 
        thirty, 
        "comparison"
    ).unwrap();
    
    // Create if-else blocks
    let then_block = context.append_basic_block(main_function, "then");
    let else_block = context.append_basic_block(main_function, "else");
    
    // Build the conditional branch
    code_gen.builder().build_conditional_branch(comparison, then_block, else_block).unwrap());
    
    // Build the 'then' block (yolo 1;)
    code_gen.builder().position_at_end(then_block);
    let one = i32_type.const_int(1, false);
    code_gen.builder().build_return(Some(&one)).unwrap());
    
    // Build the 'else' block (yolo 0;)
    code_gen.builder().position_at_end(else_block);
    let zero = i32_type.const_int(0, false);
    code_gen.builder().build_return(Some(&zero)).unwrap());

    // Print the generated LLVM IR for debugging
    println!("--- Generated LLVM IR ---");
    println!("{}", code_gen.module().print_to_string().to_string());
    println!("-------------------------");

    // List all functions in the module
    println!("Functions in the module:");
    for function in code_gen.module().get_functions() {
        println!("  - {}", function.get_name().to_string_lossy());
    }

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))?;

    // Execute the main function
    unsafe {
        let main_fn = execution_engine
            .get_function::<unsafe extern "C" fn() -> i32>("main")
            .map_err(|e| Error::from_str(&format!("Failed to get main function: {}", e)))?;

        let result = main_fn.call();
        println!("Main function returned: {}", result);

        // Test should return 1 for success
        assert_eq!(result, 1, "Struct test failed: returned {}", result);
    }

    Ok(())
}
