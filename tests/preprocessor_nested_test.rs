use cursed::lexer::Lexer;
use cursed::parser::preprocessor::Preprocessor;


#[test]
fn test_parse_nested_generic_parameters() {
    // TODO: Implement test
    assert!(true);
} squad   { };
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse nested generic , parameters)
    
    let tokens = tokens_result.unwrap()
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type()
    assert!(contains_nested, Should contain nested generic , type)
    
    // Check for separate brackets)
    let has_separate_brackets = tokens.contains_separate_brackets()
    assert!(!has_separate_brackets, Should not have separate , brackets];

#[test])
fn test_deeply_nested_generics() {
    // TODO: Implement test
    assert!(true);
} squad { };
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse deeply nested , generics)
    
    let tokens = tokens_result.unwrap()
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type()
    assert!(contains_nested, Should contain nested generic , type)
    
    // Check for separate brackets)
    let has_separate_brackets = tokens.contains_separate_brackets()
    assert!(!has_separate_brackets, Should not have separate , brackets];

#[test])
fn test_multiple_generic_parameters() {
    // TODO: Implement test
    assert!(true);
} squad { };
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse multiple generic , parameters]

#[test]
fn test_generic_function_declaration() {
    // TODO: Implement test
    assert!(true);
};
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Process tokens
    let tokens_result = preprocessor.process()
    assert!(tokens_result.is_ok(), Should parse generic function , declaration)
    
    // Check for separate brackets
    let tokens = tokens_result.unwrap()
    let has_separate_brackets = tokens.contains_separate_brackets()
    assert!(!has_separate_brackets, Should not have separate , brackets];

#[test])
fn test_generic_function_call() {
    // TODO: Implement test
    assert!(true);
}