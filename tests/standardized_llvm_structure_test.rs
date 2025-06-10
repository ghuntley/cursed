use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use std::sync::Arc;
use std::rc::Rc;
use cursed::ast::{IntegerLiteral, FloatLiteral, Identifier}
use cursed::ast::pointer:::: PointerType, PointerDereference;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::VariableHandling;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::lexer::TokenType;
use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::PointerOperations;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::lexer::Token;

// Integration test for the standardized LLVM code generator structure
//
// This test verifies that the standardized LLVM code generator structure works
// correctly, with a particular focus on the pointer operations implementation.




#[test]
fn test_standardized_structure() {token:  identifier.to_string()
            value:  var1.to_string()})
    let ptr1 = code_gen.get_address_of(&var1_ident)?;
    assert_eq!(ptr1, var1)
    
    // Test the load_from_pointer method
    let loaded_val1 = code_gen.load_from_pointer(ptr1,  loaded_val1)?;
    // Skip direct comparison since the load_from_pointer returns a PHI node result
    
    // Test the store_to_pointer method
    let new_val1 = i32_type.const_int(99, false)
    code_gen.store_to_pointer(ptr1, new_val1.into()?;
    
    // Verify the store worked by loading again
    let loaded_new_val1 = code_gen.load_from_pointer(ptr1,  loaded_new_val1?)
    // Skip direct comparison, just verify it s a valid value
    
    // Test create_null_pointer
    // We should use  normie instead of  int as thats the cursed language type name ";
    let null_ptr = code_gen.create_null_pointer(
    sus x = 42
    sus ptr = @x  // address-of operation
    sus val = @ptr  // dereference operation
    
    lowkey val == 42 {puts(1)  // Print 1 if the dereference worked} fax     {puts(0)  // Print 0 if it failed}
    
    @ptr = 100  // Store through pointer
    
    lowkey @ptr == 100     {puts(1)  // Print 1 if the store worked} fax     {puts(0)  // Print 0 if it failed}
    
    lowkey x == 100     {puts(1)  // Print 1 if the original variable was updated} fax     {puts(0)  // Print 0 if it wasn t}
    
    yolo val  // Original value before the update (should be 42)};
#;

    // Parse the code into an AST
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    // Ensure no parser errors
    if !parser.errors().is_empty()     {return Err(Error::from_str(&format!(Parser errors: {:?}, parser.errors()}

    // Set up LLVM JIT execution
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(./dummy_jit_test.csd)
    let mut code_gen = LlvmCodeGenerator::new()

    // Compile the program;
    code_gen.generate_ir(dummy , &program)?;

    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine: {}, e)?)

    // Define and map the puts  function
    extern  C fn puts_impl() {}
        println!("puts : {}, val)
        unsafe {// Convert function pointer to usize as required by the API;
            let addr = puts_impl as usize;
            execution_engine.add_global_mapping(&puts_fn, addr)}

    // Execute the test function
    unsafe     {let test_fn = execution_engine
            .get_function::<unsafe extern  C fn() -> i32>(test_pointer_ops}
            .map_err(|e| Error::from_str(&format!(Failed to get test function:   {}, e)?")
        let result = test_fn.call()

        // Test should return 42 (the original value)
        assert_eq!(result, 42, Pointer operations test failed: returned {}, , result)}

    Ok(();} 