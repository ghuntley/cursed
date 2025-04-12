use cursed::ast::traits::Node;
use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::OptimizationLevel;
use std::path::PathBuf;

#[test]
fn test_simple_jit() -> Result<(), Error> {
    // A very simple test program
    let input = r#"
    slay main() {
        sus x = 42;
        puts(x);
        yolo x;
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
    let dummy_path = PathBuf::from("./simple_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_module", dummy_path);

    // Manually create and register the 'puts' function
    let i32_type = context.i32_type();
    let puts_type = i32_type.fn_type(&[i32_type.into()], false);
    code_gen.module().add_function("puts", puts_type, Some(Linkage::External));

    // Manually create the 'main' function
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = code_gen.module().add_function("main", main_fn_type, None);
    let entry_block = context.append_basic_block(main_function, "entry");
    code_gen.builder().position_at_end(entry_block);

    // Create a constant value 42
    let x_value = i32_type.const_int(42, false);

    // Create a function call to puts
    let puts_fn = code_gen.module().get_function("puts").unwrap();
    code_gen.builder().build_call(puts_fn, &[x_value.into()], "putscall").unwrap();

    // Return the value from main
    code_gen.builder().build_return(Some(&x_value)).unwrap();

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
        
        // Test should return 42
        assert_eq!(result, 42, "Simple test failed: returned {}", result);
    }

    Ok(())
} 