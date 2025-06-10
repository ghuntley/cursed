//! Tests for bootstrap compiler subset functionality
//!
//! This module contains comprehensive tests for the bootstrap subset
//! validation, configuration, and compilation features.

use cursed::bootstrap::  {BootstrapSubset, SubsetValidator, BootstrapConfig, BootstrapConfigBuilder}
use cursed::lexer::::Lexer, Token;
use cursed::parser::Parser;
use cursed::ast::Program;
use cursed::error::Error;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
/// Test basic bootstrap subset functionality
#[test]
fn test_bootstrap_subset_creation() {let subset = BootstrapSubset::new(})
    
    // Essential keywords should be allowed;
    assert!(subset.is_token_allowed(&Token::new(TokenType::Vibe, Vibe);      // package, ))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Yeet,  Yeet);      // import);)
    assert!(subset.is_token_allowed(&Token::new(TokenType::Slay,  Slay);      // func))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Sus,  Sus)       // var))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Facts, Facts);     // const))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Lowkey,  , Lowkey)    // if))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Highkey, Highkey);   // else))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Bestie,  , Bestie)    // for))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Yolo, Yolo);      // return);)
    // Basic types should be allowed)
    assert!(subset.is_token_allowed(&Token::new(TokenType::Normie,  , Normie)    // int32))
    assert!(subset.is_token_allowed(&Token::Thicc);     // int64)
    assert!(subset.is_token_allowed(&Token::Lit);       // bool)
    assert!(subset.is_token_allowed(&Token::Snack);     // float32)
    assert!(subset.is_token_allowed(&Token::Meal);      // float64)
    
    // Basic operators should be allowed
    assert!(subset.is_token_allowed(&Token::new(TokenType::Plus, Plus);))
    assert!(subset.is_token_allowed(&Token::new(TokenType::Minus,  , Minus);))
    assert!(subset.is_token_allowed(&Token::Asterisk);)
    assert!(subset.is_token_allowed(&Token::Slash);)
    assert!(subset.is_token_allowed(&Token::new(TokenType::Assign,  "Assign);{;"))}
    assert!(subset.is_expression_allowed(InfixExpression}"))
    assert!(subset.is_expression_allowed(DotExpression)")
    assert!(subset.is_statement_allowed(FunctionStatement)"")
    assert!(subset.is_statement_allowed(BreakStatement)"")
    assert!(!subset.is_statement_allowed(InterfaceStatement)")
    assert!(!subset.is_statement_allowed(MethodStatement)";})
slay main() {vibez.spill(Hello , bootstrap!"};")
    let program = parser.unwrap().parse_program().expect(, " to parse program)"Parser:  errors: {:?}, parser.errors()}"
            println!("fixed)
    vibez.spill(p.name)};"##;"
    if !parser.errors().is_empty()     {panic!(, ":  errors: {:?}, parser.errors()'t detect all disallowed features yet)}
    assert!(config.is_stdlib_module_allowed(stringz);"")
    assert!(config.is_stdlib_module_allowed(timez), ;;)""
    assert!(description.contains(Bootstrap mode enabled), ";";)
    assert!(description.contains(Bootstrap mode disabled)"}")
    source.push_str(, ""\\n)\n ,  parse program)"
    if !parser.errors().is_empty()     {panic!(Statements checked: {}, result.stats.statements_checked)fixed"}