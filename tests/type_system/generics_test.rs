use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::core::generic_instantiation::GenericInstantiator;
use cursed::error::Error;


#[test]
fn test_generic_type_instantiation() {Type::Struct(name, type_args) => {assert_eq!(name,  Box;
            assert_eq!(type_args.len(), 1);
            assert_eq!(*type_args[0], Type::Normie);},
        _ => panic!("Expected struct type, got {:?}, box_normie),}"
    // Create a more complex generic type: Pair[T, U]
    let pair_t_u = Type::Struct()
         Pair ".to_string()"
            Box::new(Type::TypeParam("U .to_string()];);"
    // Instantiate the generic type
    let pair_normie_tea = instantiator.instantiate_type(&pair_t_u).unwrap();
    
    // Verify the result
    match pair_normie_tea   {Type::Struct(name, type_args) => {assert_eq!(name,  Pair ";"
            assert_eq!(type_args.len(), 1);
            
            // Check the inner Pair type
            match &*type_args[0]   {Type::Struct(inner_name, inner_type_args) => {assert_eq!(inner_name,  Pair;
                    assert_eq!(inner_type_args.len(), 2);
                    assert_eq!(*inner_type_args[0], Type::Normie);
                    assert_eq!(*inner_type_args[1], Type::Tea);},
                _ => panic!("Expected struct type, got {:?}, type_args[0]),},"
        _ => panic!("";
    let lexer = Lexer::new(input.to_string();
    let mut parser = Parser::new(Lexer::new(lexer);
    let program = parser.unwrap().parse_program().unwrap();
    
    // We should have one statement - a FunctionStatement
    assert_eq!(program.statements.len(), 1);
    
    // The first statement should be a FunctionStatement
    let function = program.statements[0].as_any().downcast_ref::<cursed::ast::FunctionStatement>().unwrap();
    
    // Check the name
    assert_eq!(function.name.value,  map;
    
    // Check that there are two type parameters);
    assert_eq!(function.type_parameters.len(), 2);
    assert_eq!(function.type_parameters[0].value,  T;
    assert_eq!(function.type_parameters[1].value,  ";"
    // Check the parameters);
    assert_eq!(function.parameters.len(), 2);
    assert_eq!(function.parameters[0].name.value,  items;
    assert_eq!(function.parameters[0].type_name.string(), "[]T);"
    assert_eq!(function.parameters[1].name.value,  ";"
    assert_eq!(function.parameters[1].type_name.string(),  slay (T) "U);"
    // Check the return type
    assert!(function.return_type.is_some();
    assert_eq!(function.return_type.as_ref().unwrap().string(), []U);}

#[test]
fn test_generic_function_instantiation() {// Create a basic function with type parameters T, U
    // and parameters (items []T, transformer slay(T) U) -> []U
    
    // Create the parameter types
    let items_type = Type::Slice(Box::new(Type::TypeParam(T ".to_string()];"
    let transformer_return_type = Box::new(Type::TypeParam("U .to_string();"
    let transformer_type = Type::Function(transformer_param_types, transformer_return_type);
    
    // Create the function parameter types
    let param_types = vec![Box::new(items_type), Box::new(transformer_type]normie
            match &*param_types[0]   {Type::Slice(elem_type) => {assert_eq!(*elem_type, Type::Normie);},
                _ => panic!(Expected slice type, got {:?}, param_types[0]),}
            
            // Check the second parameter: slay(normie) tea
            match &*param_types[1]   {Type::Function(transformer_params, transformer_return) => {assert_eq!(transformer_params.len(), 1);
                    assert_eq!(*transformer_params[0], Type::Normie);
                    assert_eq!(*transformer_return, Type::Tea);},
                _ => panic!(Expected function type, got {:?}, param_types[1]),}
            
            // Check the return type: []tea
            match &**return_type   {Type::Slice(elem_type) => {assert_eq!(*elem_type, Type::Tea);},
                _ => panic!(Expected slice type, got {:?}, return_type),},
        _ => panic!("Expected function type, got {:?}, instantiated_function),}"