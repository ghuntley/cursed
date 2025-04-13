use cursed::lexer::{Lexer, Token};
use cursed::parser::preprocessor::TokenStream;
use cursed::parser::preprocessor::Preprocessor;
use cursed::error::Error;

#[test]
fn test_generic_preprocessor_squad() {
    let input = "be_like Box[T] squad { stuff normie }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens = preprocessor.process().unwrap();
    
    // Verify the tokens have been properly combined
    assert!(tokens.contains_generic_type_declaration("Box", &["T"]));
    assert!(!tokens.contains_separate_brackets());
}

#[test]
fn test_generic_preprocessor_function() {
    let input = "slay foo[T](x normie) T { yolo x }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens = preprocessor.process().unwrap();
    
    // Verify the tokens have been properly combined
    assert!(tokens.contains_generic_function_declaration("foo", &["T"]));
    assert!(!tokens.contains_separate_brackets());
}

#[test]
fn test_generic_preprocessor_function_call() {
    let input = "foo[normie](42)";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens = preprocessor.process().unwrap();
    
    // Verify the tokens have been properly combined
    assert!(tokens.contains_generic_function_call("foo", &["normie"]));
    assert!(!tokens.contains_separate_brackets());
}

#[test]
fn test_malformed_generic_syntax() {
    let input = "be_like Box[T squad { stuff normie }"; // Missing closing bracket
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens should return an error
    let result = preprocessor.process();
    assert!(result.is_err());
    
    if let Err(err) = result {
        let err_message = format!("{}", err); // Convert the error to string
        assert!(err_message.contains("Unclosed type parameter"));
    }
}

#[test]
fn test_nested_generic_syntax() {
    let input = "be_like Pair[K, V[T]] squad { first K, second V[T] }";
    let mut lexer = Lexer::new(input);
    let mut preprocessor = Preprocessor::new(&mut lexer);
    
    // Process tokens
    let tokens = preprocessor.process().unwrap();
    
    // Verify the tokens have been properly combined
    assert!(tokens.contains_nested_generic_type());
    assert!(!tokens.contains_separate_brackets());
}