use cursed::lexer::{Lexer, Token};
use cursed::parser::preprocessor::{Preprocessor, TokenMetadata};
use cursed::error::Error;

#[test]
fn test_nested_generic_type_declaration() {
    let input = "be_like Pair[K, List[T]] squad { first K, second List[T] }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Preprocessor should process nested generic type declaration");
    
    let tokens = tokens_result.unwrap();
    
    // Check if nested generic type was detected
    let contains_nested = tokens.contains_nested_generic_type();
    println!("Contains nested generic type: {}", contains_nested);
    assert!(contains_nested, "Should detect nested generic type");
    
    // Check if there are separate brackets, which would indicate incomplete processing
    let has_separate_brackets = tokens.contains_separate_brackets();
    println!("Has separate brackets: {}", has_separate_brackets);
    assert!(!has_separate_brackets, "Should not have separate brackets");
    
    // Get the nested generic info
    let nested_info = tokens.get_nested_generic_info();
    assert!(!nested_info.is_empty(), "Should have nested generic info");
    
    // Verify the nested generic info
    let (outer_type, param_names) = &nested_info[0];
    assert_eq!(outer_type, "Pair", "Outer type should be 'Pair'");
    assert_eq!(param_names.len(), 2, "Should have 2 type parameters");
    assert_eq!(param_names[0], "K", "First parameter should be 'K'");
    assert_eq!(param_names[1], "List", "Second parameter should be 'List'");
}

#[test]
fn test_nested_generic_function_declaration() {
    let input = "slay transform[Container[T], U](source Container[T], fn slay(T) U) Container[U] { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Preprocessor should process nested generic function declaration");
    
    let tokens = tokens_result.unwrap();
    
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets();
    println!("Has separate brackets: {}", has_separate_brackets);
    assert!(!has_separate_brackets, "Should not have separate brackets");
    
    // Check if generic function was recognized
    let contains_generic_function = tokens.contains_generic_function_declaration("transform", &["Container", "U"]);
    assert!(contains_generic_function, "Should detect generic function declaration");
}

#[test]
fn test_nested_generic_function_call() {
    let input = "transform[List[normie], tea](myList, toString)";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Preprocessor should process nested generic function call");
    
    let tokens = tokens_result.unwrap();
    
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets();
    println!("Has separate brackets: {}", has_separate_brackets);
    assert!(!has_separate_brackets, "Should not have separate brackets");
    
    // Check if generic function call was recognized
    let contains_generic_call = tokens.contains_generic_function_call("transform", &["List", "tea"]);
    assert!(contains_generic_call, "Should detect generic function call");
}

#[test]
fn test_multiple_nested_levels() {
    let input = "be_like SuperComplex[A, Container[B, Wrapper[C]]] squad { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Preprocessor should process multiple nested levels");
    
    let tokens = tokens_result.unwrap();
    
    // Check if nested generic type was detected
    let contains_nested = tokens.contains_nested_generic_type();
    assert!(contains_nested, "Should detect nested generic type");
    
    // Check if there are separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets();
    assert!(!has_separate_brackets, "Should not have separate brackets");
    
    // Get the nested generic info
    let nested_info = tokens.get_nested_generic_info();
    assert!(!nested_info.is_empty(), "Should have nested generic info");
    
    // Verify the nested generic info
    let (outer_type, param_names) = &nested_info[0];
    assert_eq!(outer_type, "SuperComplex", "Outer type should be 'SuperComplex'");
    assert_eq!(param_names.len(), 2, "Should have 2 type parameters");
    assert_eq!(param_names[0], "A", "First parameter should be 'A'");
    assert_eq!(param_names[1], "Container", "Second parameter should be 'Container'");
}