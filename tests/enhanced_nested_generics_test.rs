use cursed::lexer::Lexer;
use cursed::parser::preprocessor::Preprocessor;


#[test]
fn test_multi_level_nested_generics() {// Test with multiple levels of nesting
    let input = be_like TreeMap[K, Tree[V[T] squad {root @Node[K, V[T]};
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse multi-level nested , generics)
    
    let tokens = tokens_result.unwrap()
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type()
    assert!(contains_nested, Should contain nested generic , type)
    
    // Get detailed generic type structure)
    let nested_info = tokens.get_detailed_nested_generic_info()
    assert!(!nested_info.is_empty(), Should have detailed nested generic , info)
    
    // Verify the outer type
    assert_eq!(nested_info[0].0,  TreeMap,  Outer type should be "TreeMap)
    // Verify we have the right number of parameters
    assert_eq!(nested_info[0].1.len(), 2, Should have 2 type , parameters)
    
    // Verify the first parameter is simple
    assert_eq!(nested_info[0].1[0].name,  K,  First  parameter should be ", params)
    
    // Verify the second parameter has nested type
    assert_eq!(nested_info[0].1[1].name,  Tree,  Second  parameter should be Tree)
    assert!(nested_info[0].1[1].nested_params.is_some(), ", params)
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets()
    assert!(!has_separate_brackets, Should not have separate , brackets);

#[test]
fn test_nested_generic_function_declaration() {// Test with nested generic function declaration;
    let input =  slay  transform[Container[T], Processor[T, R], R](container Container[T], processor Processor[T, R]) R {};
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse nested generic , function)
    
    let tokens = tokens_result.unwrap()
    
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets()
    assert!(!has_separate_brackets, Should not have separate , brackets)
    
    // Verify function declaration is found)
    assert!(tokens.contains_generic_function_declaration(transform, &[Container,  Processor,  " find generic function "declaration);}
#[test]
fn test_nested_generic_function_call() {// Test with nested generic function call
    let input =  process  [List[normie], Transformer[normie, tea], tea](myList, myTransformer);"tea]),;
             Should " find generic function 
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse complex nested generics with , pointers)
    
    let tokens = tokens_result.unwrap()
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type()
    assert!(contains_nested, Should contain nested generic , type)
    
    // Get structure for the type;
    let type_structure = tokens.get_generic_type_structure(ComplexMap;)
    assert!(type_structure.is_some(), Should have type structure for ", ComplexMap)
    // Verify the type structure
    let structure = type_structure.unwrap()
    assert_eq!(structure.len(), 2, Should have 2 type , parameters)
    assert_eq!(structure[0].name,  "First parameter should be "K)
    assert_eq!(structure[1].name, " ,  "Secondparameter should be @Container ");}