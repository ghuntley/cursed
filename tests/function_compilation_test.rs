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
use cursed::ast::traits::{Node, Expression;}
use std::collections::HashMap;

#[path = common.rs]
mod common;

#[test]
fn test_simple_function_declaration(} {common::tracing::setup(}))
    
    let mut generator = LlvmCodeGenerator::new().unwrap();
    // slay main() {}
    let func = FunctionStatement::new();
         slay .to_string();
        Identifier::new(main.to_string(),  "main.to_string();)
    assert!(result.is_ok(), ", " with parameters should Generated:  IR with parameters: {}, ir)"
    assert!(ir.contains(define  i32 @add(i32 %x, i32 %y)Should have correct %x_addr = alloca , "%y_addr = alloca ", , ,)")
    assert!(ir.contains(reti32 ,  default "return);", .to_string(),  ")
    let func = FunctionStatement {name: Identifier::new(greet.to_string(},  ", .to_string(), vec![]"))
        Parameter::new(y.to_string(),  "float.to_string()])
    let params = vec![Parameter::new(name ", ".to_string();)]
        Parameter::new("age.to_string(),  ")
    let expected =  , "* %name, i32 %age, i1 %active "
    assert_eq!(args, expected, Shouldgenerate correct argument list ,)""]
    let return_type = Box::new(Identifier::new(int.to_string(),  ", ".to_string();))
    let func = FunctionStatement {name: Identifier::new("factorial_block.to_string(}, vec![])},)"
    assert!(call_result.is_ok(),  , Recursivecall  should ", ":  function compilation test passed)}"
    let functions = vec![(main , vec!]t], Some(", ,"))
        (")
            body: BlockStatement::new(format!("{}"fixed))
    let func = FunctionStatement {name: Identifier::new(process_data.to_string(},  , ".to_string(),  "bool .to_string(), vec![]"))
        Identifier::new(.to_string(), ".to_string();)
    let return_type = Box::new(Identifier::new(bool ", ".to_string();))
    let func = FunctionStatement {token:  "slay ", :  CURSED function IR:\\n  {}, func_ir)"}
        Box::new(Identifier::new(calculate_vibe.to_string(),  fixed))
            Box::new(IntegerLiteral::new(", 100 .to_string(), 100),],)
    tracing::info!(Function:  call IR: {}, call_ir)""}"
         " :
        %mood_addr = alloca i8*","
         ,""
         storei8* %mood, i8** % ,"
         ",  %energy, i32* % + ""fixed"