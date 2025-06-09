//! Simple tests for enhanced generic constraints AST structures.
//!
//! This test suite validates basic AST functionality without complex parser integration.

use cursed::ast::declarations::{
    EnhancedConstraint, EnhancedTypeParameter, MultiParamGeneric, 
    TypeBound, Variance, WhereClause
};
use cursed::ast::{Expression, Node, Statement};
use cursed::lexer::Token;
use tracing::{debug, info, instrument};

mod common;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup()
    };
}

#[test]
#[instrument]
fn test_type_bound_creation() {
    init_tracing!();
    info!("Testing TypeBound creation");

    let token = Token::Identifier("Display".to_string());
    let bound = TypeBound::simple(token, "Display".to_string());
    
    assert_eq!(bound.interface_name, "Display");
    assert!(!bound.has_type_args());
    assert_eq!(bound.string(), "Display");

    debug!("TypeBound creation works correctly");
}

#[test]
#[instrument]
fn test_variance_annotations() {
    init_tracing!();
    info!("Testing Variance annotations");

    // Test covariant
    assert_eq!(Variance::Covariant.as_str(), "+");
    
    // Test contravariant
    assert_eq!(Variance::Contravariant.as_str(), "-");
    
    // Test invariant
    assert_eq!(Variance::Invariant.as_str(), "");
    
    // Test parsing
    assert_eq!(Variance::from_str("+"), Variance::Covariant);
    assert_eq!(Variance::from_str("-"), Variance::Contravariant);
    assert_eq!(Variance::from_str(""), Variance::Invariant);

    debug!("Variance annotations work correctly");
}

#[test]
#[instrument]
fn test_enhanced_type_parameter() {
    init_tracing!();
    info!("Testing EnhancedTypeParameter");

    let token = Token::Identifier("T".to_string());
    
    // Test simple parameter
    let simple_param = EnhancedTypeParameter::simple(token.clone(), "T".to_string());
    assert_eq!(simple_param.name, "T");
    assert!(!simple_param.has_variance());
    assert!(!simple_param.has_constraints());
    assert!(!simple_param.has_default());

    // Test with variance
    let covariant_param = EnhancedTypeParameter::with_variance(
        token.clone(),
        "T".to_string(),
        Variance::Covariant
    );
    assert!(covariant_param.has_variance());
    assert_eq!(covariant_param.variance, Variance::Covariant);

    debug!("EnhancedTypeParameter works correctly");
}

#[test]
#[instrument]
fn test_multi_param_generic() {
    init_tracing!();
    info!("Testing MultiParamGeneric");

    let token = Token::LeftBracket;
    
    // Test empty generic
    let empty_generic = MultiParamGeneric::new(token.clone(), vec![]);
    assert!(empty_generic.is_empty());
    assert_eq!(empty_generic.parameter_count(), 0);
    assert!(!empty_generic.has_constraints());

    // Test with parameters
    let param_t = EnhancedTypeParameter::simple(
        Token::Identifier("T".to_string()),
        "T".to_string()
    );
    let param_u = EnhancedTypeParameter::simple(
        Token::Identifier("U".to_string()),
        "U".to_string()
    );
    
    let multi_generic = MultiParamGeneric::new(token, vec![param_t, param_u]);
    assert!(!multi_generic.is_empty());
    assert_eq!(multi_generic.parameter_count(), 2);
    assert_eq!(multi_generic.parameter_names(), vec!["T".to_string(), "U".to_string()]);

    debug!("MultiParamGeneric works correctly");
}

#[test]
#[instrument]
fn test_where_clause() {
    init_tracing!();
    info!("Testing WhereClause");

    let token = Token::Identifier("where".to_string());
    
    // Test empty where clause
    let empty_where = WhereClause::new(token.clone(), vec![]);
    assert!(empty_where.is_empty());
    assert_eq!(empty_where.constraint_count(), 0);

    // Test with constraints
    let constraint = cursed::ast::declarations::GenericConstraint::new(
        Token::Identifier("T".to_string()),
        "T".to_string(),
        "Display".to_string()
    );
    
    let where_clause = WhereClause::new(token, vec![constraint]);
    assert!(!where_clause.is_empty());
    assert_eq!(where_clause.constraint_count(), 1);
    assert!(where_clause.string().contains("where"));

    debug!("WhereClause works correctly");
}

#[test]
#[instrument]
fn test_enhanced_constraint() {
    init_tracing!();
    info!("Testing EnhancedConstraint");

    let token = Token::Identifier("constraint".to_string());
    let bound = TypeBound::simple(
        Token::Identifier("Display".to_string()),
        "Display".to_string()
    );
    
    // Test single bound
    let constraint = EnhancedConstraint::single_bound(
        token.clone(),
        "T".to_string(),
        bound
    );
    
    assert_eq!(constraint.parameter_name, "T");
    assert!(!constraint.has_multiple_bounds());
    assert!(!constraint.has_associated_types());
    assert_eq!(constraint.bounds.len(), 1);

    debug!("EnhancedConstraint works correctly");
}

#[test]
#[instrument]
fn test_node_implementations() {
    init_tracing!();
    info!("Testing Node trait implementations");

    // Test TypeBound
    let bound = TypeBound::simple(
        Token::Identifier("Display".to_string()),
        "Display".to_string()
    );
    assert_eq!(bound.token_literal(), "Display");
    assert_eq!(bound.string(), "Display");

    // Test EnhancedTypeParameter
    let param = EnhancedTypeParameter::simple(
        Token::Identifier("T".to_string()),
        "T".to_string()
    );
    assert_eq!(param.token_literal(), "T");
    assert_eq!(param.string(), "T");

    // Test MultiParamGeneric
    let generic = MultiParamGeneric::new(
        Token::LeftBracket,
        vec![]
    );
    assert_eq!(generic.token_literal(), "[");

    debug!("Node trait implementations work correctly");
}

#[test]
#[instrument]
fn test_statement_implementations() {
    init_tracing!();
    info!("Testing Statement trait implementations");

    // Test WhereClause
    let where_clause = WhereClause::new(
        Token::Identifier("where".to_string()),
        vec![]
    );
    where_clause.statement_node(); // Should not panic
    assert!(where_clause.as_any().is::<WhereClause>());

    // Test MultiParamGeneric
    let generic = MultiParamGeneric::new(
        Token::LeftBracket,
        vec![]
    );
    generic.statement_node(); // Should not panic
    assert!(generic.as_any().is::<MultiParamGeneric>());

    debug!("Statement trait implementations work correctly");
}

#[test]
#[instrument]
fn test_string_representations() {
    init_tracing!();
    info!("Testing string representations");

    // Test parameter with variance
    let covariant_param = EnhancedTypeParameter::with_variance(
        Token::Identifier("T".to_string()),
        "T".to_string(),
        Variance::Covariant
    );
    let param_string = covariant_param.string();
    assert!(param_string.contains("+T"));

    // Test multi-param generic
    let param_t = EnhancedTypeParameter::simple(
        Token::Identifier("T".to_string()),
        "T".to_string()
    );
    let param_u = EnhancedTypeParameter::simple(
        Token::Identifier("U".to_string()),
        "U".to_string()
    );
    
    let generic = MultiParamGeneric::new(
        Token::LeftBracket,
        vec![param_t, param_u]
    );
    let generic_string = generic.string();
    assert!(generic_string.contains("["));
    assert!(generic_string.contains("T"));
    assert!(generic_string.contains("U"));
    assert!(generic_string.contains("]"));

    debug!("String representations work correctly");
}
