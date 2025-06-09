use cursed::ast::FunctionStatement;
use cursed::ast::Identifier;
use cursed::ast::block::BlockStatement;
use cursed::ast::type_parameter::TypeParameter;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::function_monomorphization::FunctionMonomorphization;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

// Basic test for function monomorphization


#[test]
fn test_function_monomorphization_basic() {
    // Create a context and code generator
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new());

    // Create a generic function AST
    let type_param = TypeParameter::new(Token::Identifier("T".to_string()), "T".to_string());

    let function = FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "identity".to_string(),
            value: "identity".to_string(),
        },
        parameters: vec![],
        body: BlockStatement {
            token: Token::LBrace,
            statements: vec![],
        },
        return_type: Some(Box::new(Identifier {
            token: "T".to_string(),
            value: "T".to_string(),
        })),
        type_parameters: vec![type_param],
        generic_constraints: vec![],
    };

    // Generate specialized function for i32
    let specialized_i32 = generator.generate_specialized_function(
        &function,
        "identity__Normie",
        &[Type::Normie]
    ).expect("Failed to generate specialized i32 function");

    // Verify the module
    let verify_result = generator.module().verify();
    assert!(verify_result.is_ok(), "Module verification failed: {:?}", verify_result.err());

    // Check that the specialized function exists
    let i32_identity = generator.module().get_function("identity__Normie");
    assert!(i32_identity.is_some(), "The specialized identity function was not found");
}