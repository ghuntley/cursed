//! Tests for the minimal bootstrap subset implementation
//!
//! These tests validate the core bootstrap functionality without
//! the complex dependencies that cause compilation issues.

use cursed::bootstrap_minimal::{BootstrapSubset, SubsetValidator, BootstrapConfig};
use cursed::lexer::Token;

#[test]
fn test_bootstrap_subset_creation() {
    let subset = BootstrapSubset::new();
    
    // Verify subset validation passes
    assert!(subset.validate_subset().is_ok());
    
    // Verify essential capabilities
    assert!(subset.allows_stdlib_access());
    assert!(subset.supports_control_flow());
    assert!(subset.supports_functions());
    assert!(subset.supports_variables());
}

#[test]
fn test_essential_tokens_allowed() {
    let subset = BootstrapSubset::new();
    
    // Test essential keywords
    assert!(subset.is_token_allowed(&Token::Vibe));      // package
    assert!(subset.is_token_allowed(&Token::Slay));      // func  
    assert!(subset.is_token_allowed(&Token::Sus));       // var
    assert!(subset.is_token_allowed(&Token::Lowkey));    // if
    assert!(subset.is_token_allowed(&Token::Bestie));    // for
    
    // Test basic types
    assert!(subset.is_token_allowed(&Token::Normie));    // int32
    assert!(subset.is_token_allowed(&Token::Lit));       // bool
    
    // Test operators
    assert!(subset.is_token_allowed(&Token::Plus));
    assert!(subset.is_token_allowed(&Token::Assign));
}

#[test]
fn test_advanced_features_excluded() {
    let subset = BootstrapSubset::new();
    
    // Advanced keywords should not be allowed
    assert!(!subset.is_token_allowed(&Token::Squad));     // struct
    assert!(!subset.is_token_allowed(&Token::Collab));    // interface
    assert!(!subset.is_token_allowed(&Token::Dm));        // chan
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Stan, "stan")));      // go
    assert!(!subset.is_token_allowed(&Token::VibeCheck)); // switch
    assert!(!subset.is_token_allowed(&Token::Choose));    // select
}

#[test]
fn test_expression_types() {
    let subset = BootstrapSubset::new();
    
    // Basic expressions should be allowed
    assert!(subset.is_expression_allowed("IntegerLiteral"));
    assert!(subset.is_expression_allowed("CallExpression"));
    assert!(subset.is_expression_allowed("DotExpression"));
    
    // Advanced expressions should not be allowed by default
    assert!(!subset.is_expression_allowed("StructLiteral"));
    assert!(!subset.is_expression_allowed("ChannelExpression"));
}

#[test]
fn test_config_creation() {
    let config = BootstrapConfig::default();
    assert!(!config.enabled);
    assert!(config.strict_mode);
    assert!(config.validate().is_ok());
}

#[test]
fn test_validator_creation() {
    let _validator = SubsetValidator::new();
    // Basic creation should work without errors
}

#[test]
fn test_config_descriptions() {
    let config = BootstrapConfig::strict();
    let description = config.describe();
    assert!(description.contains("Bootstrap mode enabled"));
    assert!(description.contains("strict"));
}
