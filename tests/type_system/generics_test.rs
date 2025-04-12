use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::error::Error;

#[test]
fn test_generic_type_instantiation() {
    // Setup a generic instantiator
    let mut instantiator = GenericInstantiator::new();
    
    // Setup concrete types for type parameters
    instantiator.add_type_param("T", Type::Normie);
    instantiator.add_type_param("U", Type::Tea);
    
    // Create a generic type: Box[T]
    let box_t = Type::Struct("Box".to_string(), vec![Box::new(Type::TypeParam("T".to_string()))]);
    
    // Instantiate the generic type
    let box_normie = instantiator.instantiate_type(&box_t).unwrap();
    
    // Verify the result
    match box_normie {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Box");
            assert_eq!(type_args.len(), 1);
            assert_eq!(*type_args[0], Type::Normie);
        },
        _ => panic!("Expected struct type, got {:?}", box_normie),
    }
    
    // Create a more complex generic type: Pair[T, U]
    let pair_t_u = Type::Struct(
        "Pair".to_string(), 
        vec![
            Box::new(Type::TypeParam("T".to_string())),
            Box::new(Type::TypeParam("U".to_string())),
        ]
    );
    
    // Instantiate the generic type
    let pair_normie_tea = instantiator.instantiate_type(&pair_t_u).unwrap();
    
    // Verify the result
    match pair_normie_tea {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Pair");
            assert_eq!(type_args.len(), 2);
            assert_eq!(*type_args[0], Type::Normie);
            assert_eq!(*type_args[1], Type::Tea);
        },
        _ => panic!("Expected struct type, got {:?}", pair_normie_tea),
    }
    
    // Test nested generics: Box[Pair[T, U]]
    let box_pair = Type::Struct(
        "Box".to_string(), 
        vec![Box::new(pair_t_u)]
    );
    
    // Instantiate the nested generic type
    let box_pair_normie_tea = instantiator.instantiate_type(&box_pair).unwrap();
    
    // Verify the result
    match box_pair_normie_tea {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Box");
            assert_eq!(type_args.len(), 1);
            
            // Check the inner Pair type
            match &*type_args[0] {
                Type::Struct(inner_name, inner_type_args) => {
                    assert_eq!(inner_name, "Pair");
                    assert_eq!(inner_type_args.len(), 2);
                    assert_eq!(*inner_type_args[0], Type::Normie);
                    assert_eq!(*inner_type_args[1], Type::Tea);
                },
                _ => panic!("Expected struct type, got {:?}", type_args[0]),
            }
        },
        _ => panic!("Expected struct type, got {:?}", box_pair_normie_tea),
    }
}

#[test]
fn test_generic_function_parsing() {
    let input = "
        vibe main
        
        slay map[T, U](items []T, transformer slay(T) U) []U {
            sus result = make([]U, len(items))
            sus i = 0
            
            bestie i < len(items) {
                result[i] = transformer(items[i])
                i = i + 1
            }
            
            yolo result
        }
    ";
    
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    
    // We should have one statement - a FunctionStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a FunctionStatement
    let function = program.statements[0].as_any().downcast_ref::<cursed::ast::FunctionStatement>().unwrap();
    
    // Check the name
    assert_eq!(function.name.value, "map");
    
    // Check that there are two type parameters
    assert_eq!(function.type_parameters.len(), 2);
    assert_eq!(function.type_parameters[0].value, "T");
    assert_eq!(function.type_parameters[1].value, "U");
    
    // Check the parameters
    assert_eq!(function.parameters.len(), 2);
    assert_eq!(function.parameters[0].name.value, "items");
    assert_eq!(function.parameters[0].type_name.string(), "[]T");
    assert_eq!(function.parameters[1].name.value, "transformer");
    assert_eq!(function.parameters[1].type_name.string(), "slay(T) U");
    
    // Check the return type
    assert!(function.return_type.is_some());
    assert_eq!(function.return_type.as_ref().unwrap().string(), "[]U");
}

#[test]
fn test_generic_function_instantiation() {
    // Create a basic function with type parameters T, U
    // and parameters (items []T, transformer slay(T) U) -> []U
    
    // Create the parameter types
    let items_type = Type::Slice(Box::new(Type::TypeParam("T".to_string())));
    let transformer_param_types = vec![Box::new(Type::TypeParam("T".to_string()))];
    let transformer_return_type = Box::new(Type::TypeParam("U".to_string()));
    let transformer_type = Type::Function(transformer_param_types, transformer_return_type);
    
    // Create the function parameter types
    let param_types = vec![Box::new(items_type), Box::new(transformer_type)];
    
    // Create the function return type
    let return_type = Box::new(Type::Slice(Box::new(Type::TypeParam("U".to_string()))));
    
    // Create the function type
    let function_type = Type::Function(param_types, return_type);
    
    // Setup a generic instantiator
    let mut instantiator = GenericInstantiator::new();
    
    // Setup concrete types for type parameters
    instantiator.add_type_param("T", Type::Normie);
    instantiator.add_type_param("U", Type::Tea);
    
    // Instantiate the function type
    let instantiated_function = instantiator.instantiate_type(&function_type).unwrap();
    
    // Verify the result
    match instantiated_function {
        Type::Function(param_types, return_type) => {
            // Check the parameter types
            assert_eq!(param_types.len(), 2);
            
            // Check the first parameter: []normie
            match &*param_types[0] {
                Type::Slice(elem_type) => {
                    assert_eq!(**elem_type, Type::Normie);
                },
                _ => panic!("Expected slice type, got {:?}", param_types[0]),
            }
            
            // Check the second parameter: slay(normie) tea
            match &*param_types[1] {
                Type::Function(transformer_params, transformer_return) => {
                    assert_eq!(transformer_params.len(), 1);
                    assert_eq!(*transformer_params[0], Type::Normie);
                    assert_eq!(**transformer_return, Type::Tea);
                },
                _ => panic!("Expected function type, got {:?}", param_types[1]),
            }
            
            // Check the return type: []tea
            match &**return_type {
                Type::Slice(elem_type) => {
                    assert_eq!(**elem_type, Type::Tea);
                },
                _ => panic!("Expected slice type, got {:?}", return_type),
            }
        },
        _ => panic!("Expected function type, got {:?}", instantiated_function),
    }
}