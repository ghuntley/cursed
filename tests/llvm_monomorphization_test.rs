use cursed::ast::base::Program;
use cursed::ast::expressions::Identifier;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::traits::Expression;
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::MonomorphizationManager;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

// Test for the MonomorphizationManager integration with LLVM code generator


/// A helper struct for MonomorphizationManager testing
struct TestGenericFunction {
    name: String,
    type_params: Vec<String>,
    param_types: Vec<Type>,
    return_type: Type,
}

#[test]
fn test_monomorphization_manager_creation() {
    // Create a MonomorphizationManager
    let manager = MonomorphizationManager::new();

    // Verify it's initially empty - now using public methods
    assert!(!manager.is_function_instantiated("test", &[Type::Normie]));
}

#[test]
fn test_monomorphization_name_generation() {
    let manager = MonomorphizationManager::new();

    // Test with simple types
    let name = manager.generate_specialized_name("add", &[Type::Normie, Type::Normie]);
    assert_eq!(name, "add__Normie_Normie");

    // Test with complex types
    let slice_type = Type::Slice(Box::new(Type::Tea));
    let name = manager.generate_specialized_name("process", &[slice_type]);
    assert_eq!(name, "process__Slice_Tea");
}

#[test]
fn test_monomorphization_function_specialization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_mono.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Create a MonomorphizationManager directly to avoid the borrow issues
    let mut mono_manager = MonomorphizationManager::new();

    // Create a generic function AST node
    let generic_func = create_generic_function(
        "max",
        vec!["T"],
        vec![
            Type::TypeParam("T".to_string()),
            Type::TypeParam("T".to_string()),
        ],
        Type::TypeParam("T".to_string()),
    );

    // Specialize with concrete type Normie (i32)
    let specialized_name = mono_manager
        .specialize_function(&mut code_gen, &generic_func, &[Type::Normie])
        .expect("Should succeed");

    // Verify the specialized function name
    assert_eq!(specialized_name, "max__Normie");

    // Verify the function is in the instantiated map
    assert!(mono_manager.is_function_instantiated("max", &[Type::Normie]));

    // Specializing again should return the same name
    let specialized_name2 = mono_manager
        .specialize_function(&mut code_gen, &generic_func, &[Type::Normie])
        .expect("Should succeed");

    assert_eq!(
        specialized_name, specialized_name2,
        "Should reuse existing specialization"
    );

    // Verify the LLVM module contains the specialized function
    let module = code_gen.module();
    let function = module.get_function(&specialized_name);
    assert!(
        function.is_some(),
        "Specialized function should exist in module"
    );
}

/// Helper function to create a generic function AST node
fn create_generic_function(
    name: &str,
    type_params: Vec<&str>,
    param_types: Vec<Type>,
    return_type: Type,
) -> FunctionStatement {
    // Create type parameters
    let type_parameters: Vec<Identifier> = type_params
        .iter()
        .map(|param| Identifier {
            token: "token".to_string(),
            value: param.to_string(),
        })
        .collect();

    // Create function parameters
    let parameters: Vec<ParameterStatement> = param_types
        .iter()
        .enumerate()
        .map(|(i, param_type)| {
            let param_name = format!("param{}", i);
            ParameterStatement {
                token: Token::Identifier("IDENT".to_string()),
                name: Identifier {
                    token: "token".to_string(),
                    value: param_name,
                },
                type_name: Box::new(Identifier {
                    token: "token".to_string(),
                    value: param_type.to_string(),
                }),
            }
        })
        .collect();

    // Create return type expression
    let return_type_expr = Box::new(Identifier {
        token: "token".to_string(),
        value: return_type.to_string(),
    }) as Box<dyn Expression>;

    // Create function body (empty for this test)
    let body = BlockStatement {
        token: Token::LBrace,
        statements: Vec::new(),
    };

    // Create the function statement
    FunctionStatement {
        token: Token::Slay,
        name: Identifier {
            token: "token".to_string(),
            value: name.to_string(),
        },
        parameters,
        body,
        return_type: Some(return_type_expr),
        type_parameters,
        generic_constraints: vec![],  // No constraints in this test
    }
}

#[test]
fn test_monomorphization_manager_in_llvm_generator() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_mono_integration.csd");
    let mut code_gen = LlvmCodeGenerator::new();

    // Create a MonomorphizationManager directly
    let mut mono_manager = MonomorphizationManager::new();

    // Create generic function
    let generic_fn = create_generic_function(
        "map",
        vec!["T", "U"],
        vec![Type::TypeParam("T".to_string())],
        Type::TypeParam("U".to_string()),
    );

    // Specialize with concrete types
    let spec_name = mono_manager
        .specialize_function(&mut code_gen, &generic_fn, &[Type::Thicc, Type::Tea])
        .expect("Specialization should succeed");

    // Verify name format
    assert_eq!(spec_name, "map__Thicc_Tea");

    // Verify it's cached
    assert!(mono_manager.is_function_instantiated("map", &[Type::Thicc, Type::Tea]));

    // Get the function from the LLVM module
    let function = code_gen.module().get_function(&spec_name);
    assert!(function.is_some(), "Function should be in the module");
}
