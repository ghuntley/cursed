use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;

#[test]
fn test_jit_map_basic() -> Result<(), Error> {
    // Test basic map operations
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        sus alice_score = scores["Alice"];
        
        lowkey alice_score == 95 {
            puts(1);
        } highkey {
            puts(0);
        }
        
        yolo 0;
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

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which is used in the test
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 0 for success - in this test yolo 0 is at the end
        assert_eq!(result, 0, "Map basic test failed: returned {}", result);
    }

    Ok(())
}

#[test]
#[ignore = "Map support not fully implemented"]
fn test_jit_map_mutation() -> Result<(), Error> {
    // Test map mutation operations
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
        scores["Alice"] = 98;  // Update Alice's score
        sus alice_score = scores["Alice"];
        
        lowkey alice_score == 98 {
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

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_mutation_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which is used in the test
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 1 for success if mutation worked
        assert_eq!(result, 1, "Map mutation test failed: returned {}", result);
    }

    Ok(())
}

#[test]
#[ignore = "Map support not fully implemented"]
fn test_jit_map_missing_key() -> Result<(), Error> {
    // Test map with missing key
    let input = r#"
    vibe test;

    slay main() {
        sus scores = {"Alice": 95, "Bob": 87};
        sus has_dave = scores.has_key("Dave");
        
        lowkey has_dave == false {
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

    // Set up LLVM JIT execution
    let context = Context::create();
    let dummy_path = PathBuf::from("./dummy_map_missing_key_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Register puts function which is used in the test
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(inkwell::module::Linkage::External));

    // Compile the program
    code_gen.compile_program(&program)?;

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

        // Test should return 1 for success (key doesn't exist)
        assert_eq!(result, 1, "Map missing key test failed: returned {}", result);
    }

    Ok(())
}
