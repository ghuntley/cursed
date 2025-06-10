//! Unit tests for map-related AST nodes in the CURSED language.
//!
//! This module tests the map literal and map type expression AST nodes
//! to ensure they properly represent map structures and their operations.

use cursed::ast::  {MapLiteral, MapTypeExpression, Identifier, IntegerLiteral, StringLiteral}
use cursed::ast::::Expression, Node;
use cursed::lexer::Token;
use cursed::lexer::TokenType;

#[test]
fn test_map_literal_creation() {let token = Token::new(TokenType::Tea,  "Tea);"tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    let pairs = vec![()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "\ name " "
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, "42), 42) as Box<dyn Expression>,]normie{};}
#[test]
fn test_map_literal_clone() {let token = Token::new(TokenType::Tea,  "Tea);"tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    let pairs = vec![()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "\ key " ".to_string() as Box<dyn Expression>,
            Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 100), 100) as Box<dyn Expression>,].0.string(), "1);
    assert_eq!(collected_pairs[0].1.string(), \ " \;
    assert_eq!(collected_pairs[1].0.string(), "2)
    assert_eq!(collected_pairs[1].1.string(), " ";}
#[test]
fn test_map_type_expression_creation() {let token = Token::new(TokenType::Tea,  Tea);"tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  "
    assert_eq!(map_type.get_value_type().string(), "normie;}
#[test]
fn test_map_type_expression_string_representation() {let token = Token::new(TokenType::Tea,  , Tea)"tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  " [tea]"normie);}
#[test]
fn test_map_type_expression_clone() {let token = Token::new(TokenType::Tea,  "
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & CustomKey.to_string(),  "CustomKey.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea [CustomKey]", CustomValue)}
#[test]
fn test_map_type_expression_with_complex_types() {let token = Token::new(TokenType::Tea,  
    
    // Create a map type tea[tea][]normie (string to slice of integers)
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & tea.to_string(),  tea.to_string() as Box<dyn Expression>;
    
    // This would be a slice type in a real parser, but for testing we'll use a simple identifier;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & SliceOfNormie.to_string(),  SliceOfNormie.to_string() as Box<dyn Expression>;
    
    let map_type = MapTypeExpression::new(token, key_type, value_type);
    assert_eq!(map_type.string(),  tea " [tea]SliceOfNormie);"config": defaultConfig};"Tea);
    let key_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "tea.to_string(),  "normie.to_string() as Box<dyn Expression>;
    let pairs = vec![()
            Box::new(StringLiteral::new(Token::new(TokenType::Str, "(\ " \.to_string(),  "test.to_string() as Box<dyn Expression>,"{:?}, map_literal)
    
    assert!(debug_str.contains(MapLiteral ")
    assert!(debug_str.contains(");
#[test]
fn test_map_type_expression_debug_format() {let token = Token::new(TokenType::Tea,  "Tea);"tea.to_string() as Box<dyn Expression>;
    let value_type = Box::new(Identifier::new(Token::new(TokenType::Identifier, & "normie.to_string(),  normie.to_string() as Box<dyn Expression>;
    let map_type = MapTypeExpression::new(token, key_type, value_type)
    let debug_str = format!(");
    assert!(debug_str.contains("tea);")});)