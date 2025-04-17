//! Test for improved generic parameter substitution and constraint checking

use cursed::ast::base::Program;
use cursed::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
use cursed::ast::expressions::Identifier;
use cursed::ast::expressions::constraint::TypeConstraint;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::MonomorphizationManager;
use cursed::core::type_checker::Type;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::error::Error;
use inkwell::context::Context;
use std::path::PathBuf;

/// Helper function to create a function with constraints
fn create_constrained_function(
    name: &str,
    type_params: Vec<&str>,
    constraints: Vec<(&str, &str)>, // (param, interface)
    param_types: Vec<Type>,
    return_type: Type,
) -> FunctionStatement {
    // Create type parameters
    let type_parameters: Vec<Identifier> = type_params
        .iter()
        .map(|param| Identifier {
            token: "IDENT".to_string(),
            value: param.to_string(),
        })
        .collect();

    // Create function parameters
    let parameters = param_types
        .iter()
        .enumerate()
        .map(|(i, param_type)| {
            let param_name = format!("param{}", i);
            ast::ParameterStatement {
                token: "IDENT".to_string(),
                name: Identifier {
                    token: "IDENT".to_string(),
                    value: param_name,
                },
                type_name: Box::new(Identifier {
                    token: "IDENT".to_string(),
                    value: param_type.to_string(),
                }),
            }
        })
        .collect();

    // Create constraints
    let generic_constraints = constraints
        .iter()
        .map(|(param, interface)| {
            TypeConstraint {
                token: "where".to_string(),
                type_param: Identifier {
                    token: "IDENT".to_string(),
                    value: param.to_string(),
                },
                interface: Identifier {
                    token: "IDENT".to_string(),
                    value: interface.to_string(),
                },
            }
        })
        .collect();

    // Create return type expression
    let return_type_expr = Box::new(Identifier {
        token: "IDENT".to_string(),
        value: return_type.to_string(),
    }) as Box<dyn Expression>;

    // Create function body (empty for this test)
    let body = BlockStatement {
        token: "{".to_string(),
        statements: Vec::new(),
    };

    // Create the function statement
    FunctionStatement {
        token: "function".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: name.to_string(),
        },
        parameters,
        body,
        return_type: Some(return_type_expr),
        type_parameters,
        generic_constraints,
    }
}

#[test]
fn test_type_parameter_substitution_nested() {
    // Create a generic instantiator
    let mut instantiator = GenericInstantiator::new();
    
    // Test nested types
    let map_type = Type::Map(
        Box::new(Type::TypeParam("K".to_string())),
        Box::new(Type::Slice(Box::new(Type::TypeParam("V".to_string()))))
    );
    
    // Add type mappings
    instantiator.add_type_param("K", Type::Tea);
    instantiator.add_type_param("V", Type::Normie);
    
    // Instantiate the type
    let concrete_type = instantiator.instantiate_type(&map_type).unwrap();
    
    // The result should be Map<Tea, Slice<Normie>>
    match concrete_type {
        Type::Map(key_type, value_type) => {
            assert_eq!(*key_type, Type::Tea);
            
            match *value_type {
                Type::Slice(elem_type) => {
                    assert_eq!(*elem_type, Type::Normie);
                }
                _ => panic!("Expected Slice type, got {:?}", value_type),
            }
        }
        _ => panic!("Expected Map type, got {:?}", concrete_type),
    }
}

#[test]
fn test_recursive_generic_type_instantiation() {
    // Create a generic instantiator
    let mut instantiator = GenericInstantiator::new();
    
    // Define a recursive generic type: Tree<T> = Node with T value and list of Tree<T>
    let tree_type = Type::Struct(
        "Tree".to_string(),
        vec![Box::new(Type::TypeParam("T".to_string()))]
    );
    
    // Create a Tree<Tree<Normie>> type
    let nested_tree_type = Type::Struct(
        "Tree".to_string(), 
        vec![Box::new(tree_type.clone())]
    );
    
    // Add type mappings
    instantiator.add_type_param("T", Type::Normie);
    
    // Instantiate the nested type
    let concrete_type = instantiator.instantiate_type(&nested_tree_type).unwrap();
    
    // The result should be Tree<Tree<Normie>>
    match concrete_type {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Tree");
            
            // Check the inner Tree<Normie>
            match &*type_args[0] {
                Type::Struct(inner_name, inner_type_args) => {
                    assert_eq!(inner_name, "Tree");
                    assert_eq!(*inner_type_args[0], Type::Normie);
                }
                _ => panic!("Expected Struct type, got {:?}", type_args[0]),
            }
        }
        _ => panic!("Expected Struct type, got {:?}", concrete_type),
    }
}

#[test]
#[ignore] // Enable once constraint checking is implemented
fn test_constraint_checking_during_monomorphization() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_constraints.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_constraints", file_path);
    
    // Create a MonomorphizationManager
    let mut mono_manager = MonomorphizationManager::new();
    
    // Define an interface
    let comparable_interface = CollabStatement {
        token: "interface".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: "Comparable".to_string(),
        },
        type_parameters: Vec::new(),
        methods: Vec::new(), // For this test we don't need actual methods
    };
    
    // Register the interface with the code generator
    // In a real implementation, we would need to add this interface to a symbol table
    
    // Create a constrained generic function
    let max_function = create_constrained_function(
        "max",
        vec!["T"],
        vec![("T", "Comparable")], // T must implement Comparable
        vec![Type::TypeParam("T".to_string()), Type::TypeParam("T".to_string())],
        Type::TypeParam("T".to_string()),
    );
    
    // Valid specialization (Normie implements Comparable)
    let normie_result = mono_manager.specialize_function(
        &mut code_gen, 
        &max_function, 
        &[Type::Normie]
    );
    assert!(normie_result.is_ok());
    
    // Invalid specialization (assuming StructType doesn't implement Comparable)
    let custom_type = Type::Struct("CustomType".to_string(), Vec::new());
    let custom_result = mono_manager.specialize_function(
        &mut code_gen, 
        &max_function, 
        &[custom_type]
    );
    assert!(custom_result.is_err());
    
    // Check error message contains constraint information
    if let Err(err) = custom_result {
        assert!(err.to_string().contains("Comparable"));
    }
}

#[test]
fn test_generic_struct_field_access() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_generic_struct.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_generic_struct", file_path);
    
    // Create a MonomorphizationManager
    let mut mono_manager = MonomorphizationManager::new();
    
    // Define a generic struct
    let generic_struct = SquadStatement {
        token: "struct".to_string(),
        name: Identifier {
            token: "IDENT".to_string(),
            value: "Pair".to_string(),
        },
        type_parameters: vec![
            Identifier {
                token: "IDENT".to_string(),
                value: "T".to_string(),
            },
            Identifier {
                token: "IDENT".to_string(),
                value: "U".to_string(),
            },
        ],
        fields: vec![
            ast::declarations::FieldStatement {
                token: "IDENT".to_string(),
                name: Identifier {
                    token: "IDENT".to_string(),
                    value: "first".to_string(),
                },
                type_name: Identifier {
                    token: "IDENT".to_string(),
                    value: "T".to_string(),
                },
            },
            ast::declarations::FieldStatement {
                token: "IDENT".to_string(),
                name: Identifier {
                    token: "IDENT".to_string(),
                    value: "second".to_string(),
                },
                type_name: Identifier {
                    token: "IDENT".to_string(),
                    value: "U".to_string(),
                },
            },
        ],
    };
    
    // Specialize the struct
    let specialized_name = mono_manager.specialize_struct(
        &mut code_gen,
        &generic_struct,
        &[Type::Normie, Type::Tea]
    ).unwrap();
    
    assert_eq!(specialized_name, "Pair__Normie_Tea");
    
    // In a real implementation, we would also verify that field access works correctly
    // by checking the LLVM IR or executing compiled code, but that's beyond the scope of this test
}