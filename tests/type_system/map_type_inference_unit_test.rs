use cursed::core::type_checker::{Type, TypeChecker};
use cursed::ast::expressions::collections::HashLiteral;
use cursed::ast::expressions::{StringLiteral, IntegerLiteral};
use cursed::ast::Expression;
use cursed::ast::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;

//! Unit tests for map literal type inference in the type checker


#[test]
fn test_map_type_inference_unit() {
    // Create a TypeChecker instance
    let mut type_checker = TypeChecker::new();
    
    // Create a simple hash literal with string keys and integer values
    // {"name": 1, "age": 30}
    let mut pairs = Vec::new();
    
    // Create "name" key
    let name_key = StringLiteral {
        token: "token".to_string(),
        value: "name".to_string(),
    };
    
    // Create 1 value
    let name_value = IntegerLiteral {
        token: "token".to_string(),
        value: 1,
    };
    
    // Create "age" key
    let age_key = StringLiteral {
        token: "token".to_string(),
        value: "age".to_string(),
    };
    
    // Create 30 value
    let age_value = IntegerLiteral {
        token: "token".to_string(),
        value: 30,
    };
    
    // Add key-value pairs
    pairs.push((Box::new(name_key) as Box<dyn Expression>, Box::new(name_value) as Box<dyn Expression>));
    pairs.push((Box::new(age_key) as Box<dyn Expression>, Box::new(age_value) as Box<dyn Expression>));
    
    // Create the hash literal
    let hash_literal = HashLiteral {
        token: Token::new(TokenType::LeftBrace, "{".to_string(),
        pairs,
    };
    
    // Infer the type using our type checker
    let inferred_type = type_checker.check_expression(&hash_literal);
    
    // Check that the inferred type is correct
    assert!(inferred_type.is_ok());
    match inferred_type.unwrap() {
        Type::Map(key_type, value_type) => {
            assert_eq!(*key_type, Type::Tea); // Keys should be strings (tea)
            assert_eq!(*value_type, Type::Normie); // Values should be normie (32-bit int)
        },
        other => panic!("Expected Map type, got {:?}", other),
    }
}