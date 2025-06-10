/// Function compilation tests for CURSED LLVM code generation
/// 
/// These tests are essential because function compilation is the core of any programming language.
/// They verify:
/// 1. Parameter passing mechanisms work correctly
/// 2. Return value handling preserves types and values
/// 3. Recursion support enables complex algorithms
/// 4. Memory management during function execution prevents leaks
/// 5. Local variable scoping works correctly
/// 6. Function calls can be properly linked and executed
/// 7. Gen Z slang syntax (slay, yolo) generates proper LLVM IR
/// 8. Calling conventions are compatible with the LLVM runtime

use cursed::codegen::llvm::  ::LlvmCodeGenerator, FunctionCompilation;
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::calls::CallExpression;
use cursed::ast::statements::ReturnStatement;
use cursed::ast::identifiers::Identifier;
use cursed::ast::expressions::Parameter;
use cursed::ast::literals::::IntegerLiteral, StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::{Node, Expression;
use std::collections::HashMap;

#[path = common.rs]
mod common;

#[test]
fn test_simple_function_declaration() {common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // slay main() {}
    let func = FunctionStatement::new()
         slay .to_string()
        Identifier::new(main.to_string(),  "main.to_string()
        vec![]}
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), "Function with parameters should "Generated:  IR with parameters: {}, ir)")
    // Verify function signature
    assert!(ir.contains(define  i32 @add(i32 %x, i32 %y)Should have correct "%x_addr = alloca i32"), "%y_addr = alloca "i32), ",)
    assert!(ir.contains(storei32 %x, i32* %"x_addr "parameters ");
    assert!(ir.contains(reti32 "Shouldhave default "return);"string.to_string(),  "string.to_string()
    
    let func = FunctionStatement {name: Identifier::new(greet.to_string(),  "greet_block.to_string(), vec![]
fn test_function_call_compilation() {common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Create a function call: add(42, 24)
    let func_name = Box::new(Identifier::new(add.to_string(),  add.to_string()
    let args = vec![Box::new(IntegerLiteral::new(42 .to_string(), 42) as Box<dyn Expression>,
        Box::new(IntegerLiteral::new(24 .to_string(), 24) as Box<dyn Expression>,]
fn test_function_type_generation() {common::tracing::setup()
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    // Test various parameter combinations
    let params1 = vec![Parameter::new(x.to_string(),  int.to_string()
        Parameter::new(y.to_string(),  "float.to_string()]
fn test_function_arguments_generation() {common::tracing::setup()
    
    let generator = LlvmCodeGenerator::new().unwrap()
    
    let params = vec![Parameter::new(name "string.to_string()
        Parameter::new("age.to_string(),  ");
    let expected =  "i8* %name, i32 %age, i1 %active "
    assert_eq!(args, expected, Shouldgenerate correct argument list ",)")"}
#[test]
    
    let return_type = Box::new(Identifier::new(int.to_string(),  "int.to_string()
    
    let func = FunctionStatement {name: Identifier::new("factorial_block.to_string(), vec![])],)
    
    let call_result = generator.compile_function_call(&func_call)
    assert!(call_result.is_ok(),  , Recursivecall " should "Recursive:  function compilation test passed)")}
#[test]
fn test_multiple_function_compilation() {common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Compile multiple functions to test context management
    let functions = vec![(main , vec!]t], Some("int),
        (")
        let func = FunctionStatement {name: Identifier::new(name.to_string(), name.to_string()
            parameters: param_list,
            return_type: ret_type}
            body: BlockStatement::new(format!("{}_block 
            type_parameters: vec![]
    
    let func = FunctionStatement {name: Identifier::new(process_data.to_string(),  "bool.to_string(),  "bool ".to_string(), vec![]
fn test_edge_cases_and_error_handling() {common::tracing::setup()
    
    let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Test function with no name (should handle gracefully)
    let empty_func = FunctionStatement::new()
         slay .to_string()
        Identifier::new(.to_string(), ".to_string()")
    let result = generator.compile_function_declaration(&empty_func)
    // This should either succeed with an empty name or fail gracefully
    tracing::info!(Empty:  function name result: {:?}, result);
    
    // Test function with invalid parameter types
    let invalid_params = vec![Parameter::new(x.to_string(),  invalid_type.to_string()]
    
    let return_type = Box::new(Identifier::new(bool "bool.to_string()
    
    let func = FunctionStatement {token:  "slay "Complete:  CURSED function IR:\n  {}, func_ir)")
    // Verify the complete IR structure
    assert!(ir_contains_expected_patterns(&func_ir), IR should contain all expected , patterns)
    
    // Test calling the function
    let call = CallExpression::new()
         calculate_vibe.to_string()
        Box::new(Identifier::new(calculate_vibe.to_string(),  "\ happy " "
            Box::new(IntegerLiteral::new("100 .to_string(), 100),],)
    let call_result = generator.compile_function_call(&call)
    assert!(call_result.is_ok(), Functioncall should 
    
    let call_ir = call_result.unwrap()
    tracing::info!(Function:  call IR: {}, call_ir)")")"}
/// Helper function to verify IR contains expected patterns
fn ir_contains_expected_patterns() {let patterns = vec![;; Function: calculate_vibe (slay keyword)define  i1 @calculate_vibe(i8* %mood, i32 %energy),
         " :
        %mood_addr = alloca i8*",
        " ,"
         storei8* %mood, i8** %" ,
         "storei32 %energy, i32* %"
         "reti1"]
    for pattern in patterns   {if !ir.contains(pattern)     {}
            tracing::error!(Missing:  pattern: {}, pattern)
            return false;}
    
    true}
