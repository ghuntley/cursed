use cursed::ast::FunctionStatement;
use cursed::ast::Identifier;
use cursed::ast::block::BlockStatement;
use cursed::ast::type_parameter::TypeParameter;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::function_monomorphization::FunctionMonomorphization;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Basic test for function monomorphization


#[test]
fn test_function_monomorphization_basic() {// Create a context and code generator}
    let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a generic function AST
    let type_param = TypeParameter::new(Token::new(TokenType::Identifier, &T.to_string(),  T ".to_string();))
        generic_constraints: vec![]).expect(Failed to generate specialized i32 function)" specialized identity function was not found;"fixed"