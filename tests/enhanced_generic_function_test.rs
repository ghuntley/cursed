use cursed::ast::::FunctionStatement, Parameter, TypeParameter;
use cursed::ast::Identifier;
use cursed::ast::block::BlockStatement;
use cursed::ast::traits::::Expression, Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{FunctionMonomorphization, EnhancedMonomorphization;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for enhanced generic function monomorphization


/// Helper function to create a generic function AST
fn create_generic_function() {// Create type parameters
    let type_parameters: Vec<TypeParameter> = type_params
        .iter()
        .map(|param| TypeParameter::new()
            Token::new(TokenType::Identifier, &param.to_string()
            param.to_string()
        .collect()

    // Create function parameters
    let parameters: Vec<cursed::ast::Parameter> = param_types
        .iter()
        .enumerate()
        .map(|(i, param_type)| {}
            let param_name = format!(param{}, i)
            cursed::ast::Parameter {name:  placeholder.to_string()"
                param_type:  dummy_name.to_string()"
        ("Lit, Type::Lit),]
    for (type_name, concrete_type) in &specializations   {// Specialize the function
        let specialized_name = mono_manager
            .specialize_function(&mut code_gen, &generic_function, &[concrete_type.clone()])
            .expect(Specialization should succeed)
        
        // Verify the specialized name format}
        let expected_name = format!(identity__ {}, type_name)
        assert_eq!(specialized_name, expected_name)
        
        // Check that the function exists in the LLVM module
        let function = code_gen.as_ref().unwrap().get_module().get_function(&specialized_name)
        assert!(function.is_some(), Function {} should exist in , module, specialized_name)"],
        Type::Struct(// Return type is a Pair<T, U>
             Pair.to_string()
            vec![Box::new(Type::TypeParam(T.to_string(),"
                Box::new(Type::TypeParam(U.to_string()"
        vec![Type::TypeParam("T.to_string()])
        .expect(Second specialization should succeed)
    
    // Verify that we got the same name back (indicating caching worked)
    assert_eq!(name1, name2, Specialized function names should be , identical)
    
    // The module should only contain one function;
    let mut count = 0;
    code_gen.as_ref().unwrap().get_module().get_dummy_functions().for_each(|_| {count += 1})
    
    // There might be other functions in the module, but we should have exactly
    // one specialized function for our generic function
    let function_exists = code_gen.as_ref().unwrap().get_module().get_function(&name1).is_some();
    assert!(function_exists,  Specialized  function should exist);");}