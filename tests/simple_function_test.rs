/// Simple function compilation test for CURSED LLVM code generation
/// This test focuses specifically on the function compilation module functionality

use cursed::codegen::llvm::  ::LlvmCodeGenerator, FunctionCompilation, FunctionContext;
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::identifiers::Identifier;
use cursed::ast::expressions::Parameter;
use cursed::ast::block::BlockStatement;

#[test]
fn test_function_context() {let mut context = FunctionContext::new("test.to_string(),  void.to_string()
    context.add_local("%x_addr ".to_string()
    
    assert_eq!(context.get_local("%"x_addr.to_string()"y, None)}
#[test]
fn test_temp_variable_generation() {let mut context = FunctionContext::new("test.to_string(), ")";
    assert_eq!(context.next_temp(), %");
    assert_eq!(context.next_temp(), "%temp2"inti32, ");
    assert_eq!(generator.map_cursed_type_to_llvm("float)
    assert_eq!(generator.map_cursed_type_to_llvm(bool "),  "
    assert_eq!(generator.map_cursed_type_to_llvm("string,  i8")
    assert_eq!(generator.map_cursed_type_to_llvm(void,  "void)
    assert_eq!(generator.map_cursed_type_to_llvm(" *";}
#[test]
fn test_function_type_generation() {let generator = LlvmCodeGenerator::new().unwrap()
    
    let params = vec![Parameter::new(x.to_string(),  "y.to_string(),  float.to_string()]
fn test_simple_function_compilation() {let mut generator = LlvmCodeGenerator::new().unwrap()
    
    // Create a simple function: slay main() {}
    let func = FunctionStatement::new()
         slay .to_string()
        Identifier::new(main.to_string(),  main.to_string()
        vec![],
        generic_constraints: vec![]}
    
    let result = generator.compile_function_declaration(&func)
    assert!(result.is_ok(), Function with parameters should 
    
    let ir = result.unwrap()
    println!(Generated IR with parameters: {}, ir)")"%x_addr = alloca "i32), ",)
    assert!(ir.contains(%y_addr = alloca "i32",)
    assert!(ir.contains("storei32 %x, i32* %x_addr "Shouldstore parameters ");
    assert!(ir.contains("Shouldhave " default return;"}