use cursed::lexer::Lexer;
use cursed::parser::preprocessor::Preprocessor;

#[test]
fn test_parse_nested_generic_parameters() {
    // Simple test for parsing a nested generic parameter
    let input = "be_like Pair[K, List[T]] squad { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Should parse nested generic parameters");
    
    let tokens = tokens_result.unwrap();
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type();
    assert!(contains_nested, "Should contain nested generic type");
    
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets();
    assert!(!has_separate_brackets, "Should not have separate brackets");
}

#[test]
fn test_deeply_nested_generics() {
    // Test with multiple levels of nesting
    let input = "be_like Complex[A, B[C[D]]] squad { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Should parse deeply nested generics");
    
    let tokens = tokens_result.unwrap();
    
    // Check for nested generics
    let contains_nested = tokens.contains_nested_generic_type();
    assert!(contains_nested, "Should contain nested generic type");
    
    // Check for separate brackets
    let has_separate_brackets = tokens.contains_separate_brackets();
    assert!(!has_separate_brackets, "Should not have separate brackets");
}

#[test]
fn test_multiple_generic_parameters() {
    // Test with multiple type parameters
    let input = "be_like Map[K, V] squad { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Should parse multiple generic parameters");
}

#[test]
fn test_generic_function_declaration() {
    // Test generic function declaration
    let input = "slay transform[Container[T], U](x Container[T]) U { }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Should parse generic function declaration");
    
    // Check for separate brackets
    let tokens = tokens_result.unwrap();
    let has_separate_brackets = tokens.contains_separate_brackets();
    assert!(!has_separate_brackets, "Should not have separate brackets");
}

#[test]
fn test_generic_function_call() {
    // Test generic function call
    let input = "transform[List[normie], tea](myList)";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok(), "Should parse generic function call");
    
    // Check for separate brackets
    let tokens = tokens_result.unwrap();
    let has_separate_brackets = tokens.contains_separate_brackets();
    assert!(!has_separate_brackets, "Should not have separate brackets");
}