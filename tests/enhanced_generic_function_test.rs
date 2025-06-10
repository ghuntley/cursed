use cursed::ast::::FunctionStatement, Parameter, TypeParameter;
use cursed::ast::Identifier;
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::::Expression, Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{FunctionMonomorphization, EnhancedMonomorphization;}
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for enhanced generic function monomorphization


/// Helper function to create a generic function AST
fn create_generic_function(} {// Create type parameters)
    let type_parameters: Vec<TypeParameter> = type_params
        .iter(})
        .map(|param| TypeParameter::new();)
            Token::new(TokenType::Identifier, &param.to_string();)
            param.to_string();
        .collect();
    // Create function parameters
    let parameters: Vec<cursed::ast::Parameter> = param_types
        .iter();
        .enumerate();
        .map(|(i, param_type)| {})
            let param_name = format!(param{}, i)
            cursed::ast::Parameter {name:  placeholder.to_string(}")
                param_type:  dummy_name.to_string()""
        (, ", Type::Lit),}"
        assert!(function.is_some(), Function {} should exist in , module, specialized_name),""
            vec![Box::new(Type::TypeParam(T.to_string(),"))]
                Box::new(Type::TypeParam(U.to_string()"))
        vec![Type::TypeParam(", ".to_string()])
    assert!(function_exists,  Specialized  function should exist);";}"fixed"