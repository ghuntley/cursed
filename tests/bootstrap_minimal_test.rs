//! Tests for the minimal bootstrap subset implementation
//!
//! These tests validate the core bootstrap functionality without
//! the complex dependencies that cause compilation issues.

use cursed::bootstrap_minimal::  ::BootstrapSubset, SubsetValidator, BootstrapConfig;
use cursed::lexer::Token;
use cursed::lexer::TokenType;

#[test]
fn test_bootstrap_subset_creation() {let subset = BootstrapSubset::new()
    
    // Test essential keywords;
    assert!(subset.is_token_allowed(&Token::new(TokenType::Vibe, Vibe);      // package, 
    assert!(subset.is_token_allowed(&Token::new(TokenType::Slay,  Slay);      // func);
    assert!(subset.is_token_allowed(&Token::new(TokenType::Sus,  Sus);       // var)
    assert!(subset.is_token_allowed(&Token::new(TokenType::Lowkey,  Lowkey)    // if
    assert!(subset.is_token_allowed(&Token::new(TokenType::Bestie, Bestie);    // for);
    // Test basic types)
    assert!(subset.is_token_allowed(&Token::new(TokenType::Normie,  , Normie)    // int32)
    assert!(subset.is_token_allowed(&Token::Lit);       // bool
    
    // Test operators
    assert!(subset.is_token_allowed(&Token::new(TokenType::Plus, Plus);
    assert!(subset.is_token_allowed(&Token::new(TokenType::Assign,  , Assign);

#[test]
fn test_advanced_features_excluded() {let subset = BootstrapSubset::new()
    
    // Advanced keywords should not be allowed;
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Squad, Squad);     // struct);
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Collab,  , Collab)    // interface
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Dm, Dm);        // chan)
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Stan,  , stan)      // go)
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Vibe, VibeCheck); // switch
    assert!(!subset.is_token_allowed(&Token::Choose);    // select}

#[test]
fn test_expression_types() {let subset = BootstrapSubset::new()
    
    // Basic expressions should be allowed
    assert!(subset.is_expression_allowed(, IntegerLiteral)
    assert!(subset.is_expression_allowed(CallExpression)
    assert!(subset.is_expression_allowed("DotExpression)
    // Advanced expressions should not be allowed by default
    assert!(!subset.is_expression_allowed(StructLiteral);
    assert!(!subset.is_expression_allowed(ChannelExpression);}

#[test]
fn test_config_creation() {let config = BootstrapConfig::default()
    assert!(!config.enabled)
    assert!(config.strict_mode)
    assert!(config.validate().is_ok()}

#[test]
fn test_validator_creation() {let _validator = SubsetValidator::new()
    // Basic creation should work without errors}

#[test]
fn test_config_descriptions() {let config = BootstrapConfig::strict()
    let description = config.describe()
    assert!(description.contains(Bootstrap mode enabled);
    assert!(description.contains(")});)