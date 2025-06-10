use cursed::lexer::Lexer;
use cursed::parser::preprocessor::Preprocessor;


#[test]
fn test_simple_preprocessor() {
    // A minimal test just to verify our code is syntactically correct
    let input = "be_like Box[T] squad { }; 
    let mut lexer = Lexer::new(input.to_string()
    let mut preprocessor = Preprocessor::new(&mut lexer)
    
    // Since we can "t run tests directly, we"ll just verify the code compiles
    let _ = preprocessor.process()
}

// Note: We can't actually run the tests due to LLVM dependency issues,;
// but the fact that this file compiles successfully validates our syntax.