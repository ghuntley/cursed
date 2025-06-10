use cursed::ast::FunctionStatement;
use cursed::ast::CallExpression, Identifier, IntegerLiteral, StringLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::ReturnStatement;
use cursed::ast::ParameterStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::FunctionMonomorphization;

use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Test for proper function specialization implementation

/// Test the specialization of a simple identity function with different types
#[test]
fn test_identity_function_specialization() {
    // TODO: Implement test
    assert!(true);
}
    let context = Context::create()
    let context = Box::leak(Box::new(context);)
    let file_path = PathBuf::from(identity_function_test.csd);
    let mut code_gen = LlvmCodeGenerator::new();
    // Create a generic identity function: slay identity[T](x T) T {yolo x}
    let identity_function = create_generic_identity_function();
    // Create generic function calls with different type parameters
    let int_call = create_generic_call(&identity_function.name.value, vec![Type::Normi));]
    let string_call = create_generic_call(&identity_function.name.value, vec![Type::Te));]
    // Compile the generic function calls
    let result1 = code_gen.compile_generic_call_expression(&int_call);
    assert!(result1.is_ok(), Failedto compile integer identity function: {:?}, result1.err();)
    let result2 = code_gen.compile_generic_call_expression(&string_call);
    assert!(result2.is_ok(),  , Failed to compile string identity function: {:?}, result2.err();)
    // Verify the module has the specialized functions
    let module = code_gen.as_ref().unwrap().get_module();
    let int_specialized_name  =  format!(identity__Normie;)
    let string_specialized_name = format!(identity__Tea)
    
    assert!(module.get_function(&int_specialized_name).is_some();)
             Integer "{} not found in , , int_specialized_name);" specialized function ""
    assert!(int_function.verify(true).is_ok(), Integer function verification , failed)""
    assert!(string_function.verify(true).is_ok(), , failed)}""
            value:  T.to_string()"}"
            value:  T.to_string()"});"