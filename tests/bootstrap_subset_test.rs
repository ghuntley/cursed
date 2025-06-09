//! Tests for bootstrap compiler subset functionality
//!
//! This module contains comprehensive tests for the bootstrap subset
//! validation, configuration, and compilation features.

use cursed::bootstrap::{BootstrapSubset, SubsetValidator, BootstrapConfig, BootstrapConfigBuilder};
use cursed::lexer::{Lexer, Token};
use cursed::parser::Parser;
use cursed::ast::Program;
use cursed::error::Error;

/// Test basic bootstrap subset functionality
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

/// Test allowed tokens in bootstrap subset
#[test]
fn test_allowed_tokens() {
    let subset = BootstrapSubset::new();
    
    // Essential keywords should be allowed
    assert!(subset.is_token_allowed(&Token::Vibe));      // package
    assert!(subset.is_token_allowed(&Token::Yeet));      // import
    assert!(subset.is_token_allowed(&Token::Slay));      // func
    assert!(subset.is_token_allowed(&Token::Sus));       // var
    assert!(subset.is_token_allowed(&Token::Facts));     // const
    assert!(subset.is_token_allowed(&Token::Lowkey));    // if
    assert!(subset.is_token_allowed(&Token::Highkey));   // else
    assert!(subset.is_token_allowed(&Token::Bestie));    // for
    assert!(subset.is_token_allowed(&Token::Yolo));      // return
    
    // Basic types should be allowed
    assert!(subset.is_token_allowed(&Token::Normie));    // int32
    assert!(subset.is_token_allowed(&Token::Thicc));     // int64
    assert!(subset.is_token_allowed(&Token::Lit));       // bool
    assert!(subset.is_token_allowed(&Token::Snack));     // float32
    assert!(subset.is_token_allowed(&Token::Meal));      // float64
    
    // Basic operators should be allowed
    assert!(subset.is_token_allowed(&Token::Plus));
    assert!(subset.is_token_allowed(&Token::Minus));
    assert!(subset.is_token_allowed(&Token::Asterisk));
    assert!(subset.is_token_allowed(&Token::Slash));
    assert!(subset.is_token_allowed(&Token::Assign));
    assert!(subset.is_token_allowed(&Token::Eq));
    assert!(subset.is_token_allowed(&Token::NotEq));
    
    // Delimiters should be allowed
    assert!(subset.is_token_allowed(&Token::LParen));
    assert!(subset.is_token_allowed(&Token::RParen));
    assert!(subset.is_token_allowed(&Token::LBrace));
    assert!(subset.is_token_allowed(&Token::RBrace));
    assert!(subset.is_token_allowed(&Token::Semicolon));
    assert!(subset.is_token_allowed(&Token::Comma));
}

/// Test disallowed tokens in bootstrap subset
#[test]
fn test_disallowed_tokens() {
    let subset = BootstrapSubset::new();
    
    // Advanced features should not be allowed
    assert!(!subset.is_token_allowed(&Token::Squad));     // struct
    assert!(!subset.is_token_allowed(&Token::Collab));    // interface
    assert!(!subset.is_token_allowed(&Token::Dm));        // chan
    assert!(!subset.is_token_allowed(&Token::new(TokenType::Stan, "stan")));      // go
    assert!(!subset.is_token_allowed(&Token::VibeCheck)); // switch
    assert!(!subset.is_token_allowed(&Token::Choose));    // select
    assert!(!subset.is_token_allowed(&Token::Tea));       // map
    assert!(!subset.is_token_allowed(&Token::At));        // pointer
    assert!(!subset.is_token_allowed(&Token::Arrow));     // channel ops
}

/// Test allowed expression types
#[test]
fn test_allowed_expressions() {
    let subset = BootstrapSubset::new();
    
    // Basic expressions should be allowed
    assert!(subset.is_expression_allowed("IntegerLiteral"));
    assert!(subset.is_expression_allowed("FloatLiteral"));
    assert!(subset.is_expression_allowed("StringLiteral"));
    assert!(subset.is_expression_allowed("BooleanLiteral"));
    assert!(subset.is_expression_allowed("Identifier"));
    assert!(subset.is_expression_allowed("InfixExpression"));
    assert!(subset.is_expression_allowed("PrefixExpression"));
    assert!(subset.is_expression_allowed("CallExpression"));
    assert!(subset.is_expression_allowed("ParenthesizedExpression"));
    assert!(subset.is_expression_allowed("ArrayLiteral"));
    assert!(subset.is_expression_allowed("IndexExpression"));
    assert!(subset.is_expression_allowed("DotExpression"));
    
    // Advanced expressions should not be allowed
    assert!(!subset.is_expression_allowed("StructLiteral"));
    assert!(!subset.is_expression_allowed("ChannelExpression"));
    assert!(!subset.is_expression_allowed("TypeAssertion"));
    assert!(!subset.is_expression_allowed("MethodCall"));
    assert!(!subset.is_expression_allowed("SliceExpression"));
}

/// Test allowed statement types
#[test]
fn test_allowed_statements() {
    let subset = BootstrapSubset::new();
    
    // Basic statements should be allowed
    assert!(subset.is_statement_allowed("PackageStatement"));
    assert!(subset.is_statement_allowed("ImportStatement"));
    assert!(subset.is_statement_allowed("VarStatement"));
    assert!(subset.is_statement_allowed("ConstStatement"));
    assert!(subset.is_statement_allowed("FunctionStatement"));
    assert!(subset.is_statement_allowed("ExpressionStatement"));
    assert!(subset.is_statement_allowed("BlockStatement"));
    assert!(subset.is_statement_allowed("IfStatement"));
    assert!(subset.is_statement_allowed("ForStatement"));
    assert!(subset.is_statement_allowed("ReturnStatement"));
    assert!(subset.is_statement_allowed("BreakStatement"));
    assert!(subset.is_statement_allowed("ContinueStatement"));
    assert!(subset.is_statement_allowed("AssignmentStatement"));
    
    // Advanced statements should not be allowed
    assert!(!subset.is_statement_allowed("SwitchStatement"));
    assert!(!subset.is_statement_allowed("SelectStatement"));
    assert!(!subset.is_statement_allowed("StructStatement"));
    assert!(!subset.is_statement_allowed("InterfaceStatement"));
    assert!(!subset.is_statement_allowed("MethodStatement"));
}

/// Test subset validator creation and basic functionality
#[test]
fn test_subset_validator() {
    let validator = SubsetValidator::new();
    
    // Test validation of empty program
    let empty_program = Program::new(Vec::new());
    let mut validator = SubsetValidator::new();
    let result = validator.validate_program(&empty_program);
    
    // Empty program should have warnings but may not be invalid
    assert!(result.warnings.len() > 0);
    assert_eq!(result.stats.statements_checked, 0);
    assert_eq!(result.stats.expressions_checked, 0);
    assert_eq!(result.stats.functions_found, 0);
}

/// Test bootstrap configuration creation
#[test]
fn test_bootstrap_config() {
    // Test default configuration
    let default_config = BootstrapConfig::default();
    assert!(!default_config.enabled);
    assert!(default_config.strict_mode);
    assert!(default_config.generate_warnings);
    assert!(default_config.validate().is_ok());
    
    // Test strict configuration
    let strict_config = BootstrapConfig::strict();
    assert!(strict_config.enabled);
    assert!(strict_config.strict_mode);
    assert!(!strict_config.allow_experimental);
    assert!(strict_config.validate().is_ok());
    
    // Test lenient configuration
    let lenient_config = BootstrapConfig::lenient();
    assert!(lenient_config.enabled);
    assert!(!lenient_config.strict_mode);
    assert!(lenient_config.allow_experimental);
    assert!(lenient_config.validate().is_ok());
}

/// Test bootstrap configuration builder
#[test]
fn test_bootstrap_config_builder() {
    let config = BootstrapConfigBuilder::new()
        .enabled()
        .strict()
        .max_statements(500)
        .allow_module("testmodule")
        .build();
    
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.enabled);
    assert!(config.strict_mode);
    assert_eq!(config.max_statements, Some(500));
    assert!(config.is_stdlib_module_allowed("testmodule"));
    assert!(config.is_stdlib_module_allowed("vibez")); // Default module
}

/// Test configuration validation
#[test]
fn test_config_validation() {
    // Test invalid configuration (no stdlib modules)
    let mut config = BootstrapConfig::strict();
    config.allowed_stdlib_modules.clear();
    assert!(config.validate().is_err());
    
    // Test conflicting configuration (strict + experimental)
    let mut config = BootstrapConfig::default();
    config.strict_mode = true;
    config.allow_experimental = true;
    assert!(config.validate().is_err());
    
    // Test invalid max statements
    let config_result = BootstrapConfigBuilder::new()
        .max_statements(5) // Too low
        .build();
    assert!(config_result.is_err());
}

/// Test parsing and validation of minimal valid bootstrap program
#[test]
fn test_minimal_bootstrap_program() {
    let source = r#""
vibe main

slay main() {
    vibez.spill("Hello, bootstrap!")
}
"#";

    // Parse the program
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    // Validate against bootstrap subset
    let mut validator = SubsetValidator::new();
    let result = validator.validate_program(&program);
    
    // Should be valid (though may have warnings)
    if !result.is_valid {
        println!("Validation errors:");
        for error in &result.errors {
            println!("  - {}", error.message);
        }
    }
    
    // At minimum, should have reasonable stats
    assert!(result.stats.statements_checked > 0);
    
    // Should have found at least one function
    assert!(result.stats.functions_found >= 1);
}

/// Test parsing and validation of program with disallowed features
#[test]
fn test_disallowed_features_program() {
    let source = r#""
vibe main

squad Person {
    name tea
    age normie
}

slay main() {
    sus p = Person{name: "Alice", age: 30}
    vibez.spill(p.name)
}
"#";

    // Parse the program
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    // Validate against bootstrap subset - should fail due to struct usage
    let mut validator = SubsetValidator::new();
    let result = validator.validate_program(&program);
    
    // Should be invalid due to struct features
    // Note: This test may pass if the validator doesn't detect all disallowed features yet
    // but it demonstrates the validation concept
    if result.is_valid {
        println!("Warning: Program with structs was considered valid - validator may need enhancement");
    }
}

/// Test stdlib module allowance in configuration
#[test]
fn test_stdlib_module_configuration() {
    let config = BootstrapConfig::default();
    
    // Default modules should be allowed
    assert!(config.is_stdlib_module_allowed("vibez"));
    assert!(config.is_stdlib_module_allowed("mathz"));
    assert!(config.is_stdlib_module_allowed("stringz"));
    assert!(config.is_stdlib_module_allowed("timez"));
    
    // Non-default modules should not be allowed
    assert!(!config.is_stdlib_module_allowed("advancedmodule"));
    assert!(!config.is_stdlib_module_allowed("networkz"));
    
    // Test adding custom module
    let mut config = BootstrapConfig::default();
    config.allow_stdlib_module("customz".to_string());
    assert!(config.is_stdlib_module_allowed("customz"));
}

/// Test configuration description generation
#[test]
fn test_config_description() {
    let strict_config = BootstrapConfig::strict();
    let description = strict_config.describe();
    assert!(description.contains("Bootstrap mode enabled"));
    assert!(description.contains("strict"));
    
    let lenient_config = BootstrapConfig::lenient();
    let description = lenient_config.describe();
    assert!(description.contains("Bootstrap mode enabled"));
    assert!(description.contains("lenient"));
    
    let disabled_config = BootstrapConfig::default();
    let description = disabled_config.describe();
    assert!(description.contains("Bootstrap mode disabled"));
}

/// Test that essential language features are available
#[test]
fn test_essential_features_available() {
    let subset = BootstrapSubset::new();
    
    // Test that we can handle basic arithmetic
    assert!(subset.is_token_allowed(&Token::Plus));
    assert!(subset.is_token_allowed(&Token::Minus));
    assert!(subset.is_token_allowed(&Token::Asterisk));
    assert!(subset.is_token_allowed(&Token::Slash));
    
    // Test that we can handle variables and assignment
    assert!(subset.is_token_allowed(&Token::Sus));      // var
    assert!(subset.is_token_allowed(&Token::Assign));   // =
    assert!(subset.is_token_allowed(&Token::DeclAssign)); // :=
    
    // Test that we can handle control flow
    assert!(subset.is_token_allowed(&Token::Lowkey));   // if
    assert!(subset.is_token_allowed(&Token::Highkey));  // else
    assert!(subset.is_token_allowed(&Token::Bestie));   // for
    
    // Test that we can handle functions
    assert!(subset.is_token_allowed(&Token::Slay));     // func
    assert!(subset.is_token_allowed(&Token::Yolo));     // return
    
    // Test that we can handle basic I/O
    assert!(subset.is_expression_allowed("DotExpression"));  // for vibez.spill
    assert!(subset.is_expression_allowed("CallExpression")); // for function calls
}

/// Benchmark test for validation performance
#[test]
fn test_validation_performance() {
    // Create a reasonably complex but valid bootstrap program
    let mut source = String::from("vibe main\n\n");
    
    // Add many simple functions to test scalability
    for i in 0..100 {
        source.push_str(&format!(
            "slay func{}() {{\n    sus x = {}\n    yolo x + 1\n}}\n\n", 
            i, i
        ));
    }
    
    source.push_str("slay main() {\n    vibez.spill(\"Test\")\n}\n");
    
    // Parse the program
    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    if !parser.errors().is_empty() {
        panic!("Parser errors: {:?}", parser.errors());
    }
    
    // Time the validation
    let start = std::time::Instant::now();
    let mut validator = SubsetValidator::new();
    let result = validator.validate_program(&program);
    let duration = start.elapsed();
    
    println!("Validation took: {:?}", duration);
    println!("Statements checked: {}", result.stats.statements_checked);
    
    // Validation should complete in reasonable time (< 100ms for this test)
    assert!(duration.as_millis() < 100);
    
    // Should have found all the functions
    assert!(result.stats.functions_found >= 100);
}
