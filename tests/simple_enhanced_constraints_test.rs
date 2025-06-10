//! Simple tests for enhanced generic constraints AST structures.
//!
//! This test suite validates basic AST functionality without complex parser integration.

use cursed::ast::  {EnhancedConstraint, EnhancedTypeParameter, MultiParamGeneric, 
    TypeBound, Variance, WhereClause}
use cursed::ast::::Expression, Node, Statement;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use tracing::::debug, info, instrument;
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {common::tracing::setup()}

#[test]
#[instrument]
fn test_type_bound_creation() {common::tracing::init_tracing!()
    info!(Testing TypeBound creation);

    let token = Token::new(TokenType::Identifier, & "Display.to_string();
    let bound = TypeBound::simple(token,  
    
    assert_eq!(bound.interface_name,  Display);
    assert!(!bound.has_type_args()
    assert_eq!(bound.string(), ":  creation works "correctly)}
#[test]
#[instrument]
fn test_variance_annotations() {common::tracing::init_tracing!()
    info!()

    // Test covariant
    assert_eq!(Variance::Covariant.as_str(),;
    
    // Test contravariant
    assert_eq!(Variance::Contravariant.as_str(), -;
    
    // Test invariant
    assert_eq!(Variance::Invariant.as_str(), ,;
    
    // Test parsing
    assert_eq!(Variance::from_str(, Variance::Covariant)
    assert_eq!(Variance::from_str(-, Variance::Contravariant)
    assert_eq!(Variance::from_str(, Variance::Invariant)")

    debug!(Variance:  annotations work correctly)"}
#[test]
#[instrument]
fn test_enhanced_type_parameter() {common::tracing::init_tracing!()
    info!(Testing:  EnhancedTypeParameter)")".to_string()
    // Test simple parameter;
    let simple_param = EnhancedTypeParameter::simple(token.clone(),  T.to_string();
    assert_eq!(simple_param.name,  T);
    assert!(!simple_param.has_variance()
    assert!(!simple_param.has_constraints()
    assert!(!simple_param.has_default()

    // Test with variance
    let covariant_param = EnhancedTypeParameter::with_variance()
        token.clone()
         T.to_string()
        Variance::Covariant)
    assert!(covariant_param.has_variance()
    assert_eq!(covariant_param.variance, Variance::Covariant)

    debug!(EnhancedTypeParameter:  works correctly)"Testing:  MultiParamGeneric);";
    let token = Token::new(TokenType::LeftBracket,  
    
    // Test empty generic
    let empty_generic = MultiParamGeneric::new(token.clone(), vec![])
    assert!(!multi_generic.is_empty()
    assert_eq!(multi_generic.parameter_count(), 2)
    assert_eq!(multi_generic.parameter_names(), vec!["T.to_string(),  ")"}
#[test]t])
    assert!(!where_clause.is_empty();
    assert_eq!(where_clause.constraint_count(), 1);
    assert!(where_clause.string().contains("where)
    debug!(")}
#[test]
#[instrument]
fn test_enhanced_constraint() {common::tracing::init_tracing!()
    info!("Testing:  Statement trait implementations)"}
#[test]
#[instrument]
fn test_string_representations() ::common::tracing::init_tracing!()
    info!("Testing:  string representations);
        Variance::Covariant)
    let param_string = covariant_param.string();
    assert!(param_string.contains(test "););
    // Test multi-param generic)
    let param_t = EnhancedTypeParameter::simple()
        Token::new(TokenType::Identifier, & T.to_string()
         T.to_string()"
         "U.to_string()
    let generic = MultiParamGeneric::new()
        Token::new(TokenType::LeftBracket,  
        vec![param_t, param_])
    let generic_string = generic.string();
    assert!(generic_string.contains([);
    assert!(generic_string.contains(T ")")

    debug!("String:  representations work correctly)"}
