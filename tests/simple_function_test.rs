/// Simple function compilation test for CURSED LLVM code generation
/// This test focuses specifically on the function compilation module functionality

use cursed::codegen::llvm::  ::LlvmCodeGenerator, FunctionCompilation, FunctionContext;
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::identifiers::Identifier;
use cursed::ast::expressions::Parameter;
use cursed::ast::block::BlockStatement;

#[test]
fn test_function_context() {let mut context = FunctionContext::new("test.to_string(},  void.to_string();))
    context.add_local("%x_addr ")
    assert_eq!(context.get_local(%", ".to_string()y, None)}")
fn test_temp_variable_generation() {let mut context = FunctionContext::new(", .to_string(}, ";"))
    assert_eq!(context.next_temp(), %;"")
    assert_eq!(context.next_temp(), %, inti32, ";)
    assert_eq!(generator.map_cursed_type_to_llvm(", "))
    assert_eq!(generator.map_cursed_type_to_llvm(bool ",  "))
    assert_eq!(generator.map_cursed_type_to_llvm(, ",  i8))
    assert_eq!(generator.map_cursed_type_to_llvm(void,  , ""))
    assert_eq!(generator.map_cursed_type_to_llvm( *""))
    let params = vec![Parameter::new(x.to_string(),  , .to_string(),  float.to_string()]"")
    println!(Generated IR with parameters: {}, ir)%x_addr = alloca ", ", 
    assert!(ir.contains(%y_addr = alloca ", )fixed")