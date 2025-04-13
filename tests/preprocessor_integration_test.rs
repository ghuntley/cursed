use cursed::lexer::Lexer;
use cursed::parser::{Parser, TokenStream, Preprocessor};
use cursed::error::Error;

#[test]
fn test_preprocessor_functionality() {
    // Test generic struct declaration
    let input = "be_like Box[T] squad { value normie }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok());
    
    let tokens = tokens_result.unwrap();
    assert!(tokens.contains_generic_type_declaration("Box", &["T"]));
    assert!(!tokens.contains_separate_brackets());
    
    // Test generic function declaration
    let input = "slay foo[T](x normie) T { yolo x }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok());
    
    let tokens = tokens_result.unwrap();
    assert!(tokens.contains_generic_function_declaration("foo", &["T"]));
    assert!(!tokens.contains_separate_brackets());
    
    // Test generic function call
    let input = "foo[normie](42)";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens_result = preprocessor.process();
    assert!(tokens_result.is_ok());
    
    let tokens = tokens_result.unwrap();
    assert!(tokens.contains_generic_function_call("foo", &["normie"]));
    assert!(!tokens.contains_separate_brackets());
    
    // Test malformed generic syntax
    let input = "be_like Box[T squad { value normie }"; // Missing closing bracket
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens should return an error
    let result = preprocessor.process();
    assert!(result.is_err());
}

#[test]
fn test_nested_generics() {
    // Test nested generic types
    let input = "be_like Pair[K, V[T]] squad { first K, second V[T] }";
    let mut lexer = Lexer::new(input);
    
    // Create and run the preprocessor
    let mut preprocessor = Preprocessor::new(&mut lexer);
    let token_stream_result = preprocessor.process();
    assert!(token_stream_result.is_ok());
    
    let token_stream = token_stream_result.unwrap();
    assert!(token_stream.contains_nested_generic_type());
    assert!(!token_stream.contains_separate_brackets());
}