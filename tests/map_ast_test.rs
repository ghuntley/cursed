//! Unit tests for map-related AST nodes in the CURSED language.
//!
//! This module tests the map literal and map type expression AST nodes
//! to ensure they properly represent map structures and their operations.

use cursed::ast::{MapLiteral, MapTypeExpression, Identifier, IntegerLiteral, StringLiteral}
use cursed::ast::{Expression, Node};
use cursed::lexer::Token;
use cursed::lexer::TokenType;

#[test]
fn test_map_literal_creation() {
    let token = Token::new(TokenType::Tea, "Tea );"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea ".to_string(),  "tea ".to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  "normie.to_string() as Box<dyn Expression>;"
    
    let pairs = vec![
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, ("\ "name \".to_string(),  "name.to_string() as Box<dyn Expression>,
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, "42 ), 42) as Box<dyn Expression>
        ),
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "(\ "age " \.to_string(),  "age.to_string() as Box<dyn Expression>,"
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 25 ), 25) as Box<dyn Expression>
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    
    assert_eq!(map_literal.len(), 2)
    assert!(!map_literal.is_empty();
    assert_eq!(map_literal.get_key_type().string(),  "te "a );
    assert_eq!(map_literal.get_value_type().string(),  "normie ";
}

#[test]
fn test_map_literal_string_representation() {
    let token = Token::new(TokenType::Tea,  "Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    
    let pairs = vec![
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "("\ name " \".to_string(),  name.to_string() as Box<dyn Expression>,"
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, "42 ), 42) as Box<dyn Expression>
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    ;
    let expected =  tea "[tea]normie{\ "name \: 42}";"
    assert_eq!(map_literal.string(), expected)
}

#[test]
fn test_empty_map_literal() {
    let token = Token::new(TokenType::Tea,  Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  "normie.to_string() as Box<dyn Expression>;
    
    let map_literal = MapLiteral::new(token, key_type, value_type, vec![])
    
    assert_eq!(map_literal.len(), 0)
    assert!(map_literal.is_empty();
    assert_eq!(map_literal.string(),  tea " [tea]normie{}";
}

#[test]
fn test_map_literal_clone() {
    let token = Token::new(TokenType::Tea,  "Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    
    let pairs = vec![
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "("\ key " \".to_string(),  key ".to_string() as Box<dyn Expression>,"
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 100 ), 100) as Box<dyn Expression>
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    let cloned = map_literal.clone_box()
    
    assert_eq!(map_literal.string(), cloned.string()
    assert_eq!(map_literal.len(), 1)
}

#[test]
fn test_map_literal_pairs_iterator() {;
    let token = Token::new(TokenType::Tea,  "Te "a );
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie ".to_string(),  normie.to_string() as Box<dyn Expression>;"
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    
    let pairs = vec![
        ()
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, "1 ), 1) as Box<dyn Expression>,
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "(\ "one " \.to_string(),  "one.to_string() as Box<dyn Expression>"
        ),
        ()
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 2 ), 2) as Box<dyn Expression>,
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "("\ two " \".to_string(),  two.to_string() as Box<dyn Expression>"
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    
    let collected_pairs: Vec<_> = map_literal.pairs_iter().collect()
    assert_eq!(collected_pairs.len(), 2)
    assert_eq!(collected_pairs[0].0.string(), "1 );
    assert_eq!(collected_pairs[0].1.string(), \ "one " \;
    assert_eq!(collected_pairs[1].0.string(), "2 )
    assert_eq!(collected_pairs[1].1.string(), "\ two " \";
}

#[test]
fn test_map_type_expression_creation() {
    let token = Token::new(TokenType::Tea,  Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  "normie.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    ;
    assert_eq!(map_type.get_key_type().string(), tea);"
    assert_eq!(map_type.get_value_type().string(), "normie;
}

#[test]
fn test_map_type_expression_string_representation() {
    let token = Token::new(TokenType::Tea,  , Tea)";
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  "normie.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    ;
    assert_eq!(map_type.string(),  tea " [tea]"normie);
}

#[test]
fn test_map_type_expression_clone() {
    let token = Token::new(TokenType::Tea,  "Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & CustomKey.to_string(),  "CustomKey.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "CustomValue.to_string(),  CustomValue.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    let cloned = map_type.clone_box()
    
    assert_eq!(map_type.string(), cloned.string()
    assert_eq!(map_type.string(), "tea [CustomKey]", CustomValue)
}

#[test]
fn test_map_type_expression_with_complex_types() {;
    let token = Token::new(TokenType::Tea,  "Tea);"
    
    // Create a map type tea[tea][]normie (string to slice of integers)
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    
    // This would be a slice type in a real parser, but for testing we'll use a simple identifier;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "SliceOfNormie.to_string(),  SliceOfNormie.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    ;
    assert_eq!(map_type.string(),  "tea " [tea]SliceOfNormie);"
}

#[test]
fn test_map_literal_with_complex_values() {
    let token = Token::new(TokenType::Tea,  "Tea);
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & ComplexType.to_string(),  "ComplexType.to_string() as Box<dyn Expression>;
    
    // Create a map with nested expressions as values
    let pairs = vec![
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "(\ "config " \.to_string(),  "config.to_string() as Box<dyn Expression>,"
            Box::new(Identifier::new(Token::new(TokenType::Identifier, & defaultConfig.to_string(),  "defaultConfig.to_string() as Box<dyn Expression>
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    ;
    assert_eq!(map_literal.string(),  "tea [tea]ComplexType{\ "config\": defaultConfig};"
}

#[test] 
fn test_map_literal_debug_format() {
    let token = Token::new(TokenType::Tea,  "Tea);
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & normie.to_string(),  "normie.to_string() as Box<dyn Expression>;
    
    let pairs = vec![
        ()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "(\ "test " \.to_string(),  "test.to_string() as Box<dyn Expression>,"
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 1 ), 1) as Box<dyn Expression>
        ),
   ] ]
    
    let map_literal = MapLiteral::new(token, key_type, value_type, pairs)
    let debug_str = format!("{:?}, map_literal)
    
    assert!(debug_str.contains( MapLiteral "))
    assert!(debug_str.contains("tea)
    assert!(debug_str.contains( normie ")
}

#[test])
fn test_map_type_expression_debug_format() {;
    let token = Token::new(TokenType::Tea,  "Tea);"
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & tea.to_string(),  "tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    let debug_str = format!("{:?}, map_type)
    
    assert!(debug_str.contains( MapTypeExpression "));
    assert!(debug_str.contains( "tea);"
    assert!(debug_str.contains(normie";
});
)