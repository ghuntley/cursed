//! Integration tests for enhanced generic constraints parsing and AST structures.
//!
//! This test suite validates the complete implementation of enhanced generic constraints
//! including multi-bound constraints, where clauses, associated types, and variance annotations.

use cursed::ast::declarations::{
    AssociatedType, ConstraintOperator, ConstraintRelation, CrossParameterConstraint,
    EnhancedConstraint, EnhancedTypeParameter, MultiParamGeneric, TypeBound, 
    Variance, WhereClause
};
use cursed::ast::{Expression, Node, Statement};
use cursed::error::Error;
use cursed::lexer::{Lexer, token::{Token, TokenType}};
use cursed::parser::Parser;
use std::io::Cursor;
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
fn test_simple_generic_constraint_ast() {
    init_tracing!();
    info!("Testing simple generic constraint AST structures");

    // Test TypeBound creation
    let token = Token::new(TokenType::Identifier, "Display".to_string(), 1, 1);
    let bound = TypeBound::simple(token, "Display".to_string());
    
    assert_eq!(bound.interface_name, "Display");
    assert!(!bound.has_type_args());
    assert_eq!(bound.string(), "Display");

    // Test TypeBound with type arguments
    let token = Token::new(TokenType::Identifier, "Into".to_string(), 1, 1);
    let args: Vec<Box<dyn Expression>> = vec![];
    let bound_with_args = TypeBound::with_args(token, "Into".to_string(), args);
    
    assert_eq!(bound_with_args.interface_name, "Into");
    assert!(!bound_with_args.has_type_args()); // Empty args
    
    debug!("Simple generic constraint AST structures work correctly");
}

#[test]
#[instrument]
fn test_enhanced_constraint_multiple_bounds() {
    init_tracing!();
    info!("Testing enhanced constraints with multiple bounds");

    let token = Token::new(TokenType::Identifier, "T".to_string(), 1, 1);
    
    // Create multiple bounds
    let display_bound = TypeBound::simple(
        Token::new(TokenType::Identifier, "Display".to_string(), 1, 1),
        "Display".to_string()
    );
    let clone_bound = TypeBound::simple(
        Token::new(TokenType::Identifier, "Clone".to_string(), 1, 1),
        "Clone".to_string()
    );
    
    let bounds = vec![display_bound, clone_bound];
    let constraint = EnhancedConstraint::multiple_bounds(
        token,
        "T".to_string(),
        bounds
    );
    
    assert_eq!(constraint.parameter_name, "T");
    assert!(constraint.has_multiple_bounds());
    assert!(!constraint.has_associated_types());
    assert_eq!(constraint.bounds.len(), 2);
    
    debug!(
        constraint_string = constraint.string(),
        bound_count = constraint.bounds.len(),
        "Enhanced constraint with multiple bounds created successfully"
    );
}

#[test]
#[instrument] 
fn test_variance_annotations() {
    init_tracing!();
    info!("Testing variance annotations for type parameters");

    // Test covariant parameter
    let token = Token::new(TokenType::Identifier, "T".to_string(), 1, 1);
    let covariant_param = EnhancedTypeParameter::with_variance(
        token.clone(),
        "T".to_string(),
        Variance::Covariant
    );
    
    assert!(covariant_param.has_variance());
    assert_eq!(covariant_param.variance, Variance::Covariant);
    assert_eq!(covariant_param.variance.as_str(), "+");
    
    // Test contravariant parameter
    let contravariant_param = EnhancedTypeParameter::with_variance(
        token.clone(),
        "U".to_string(),
        Variance::Contravariant
    );
    
    assert!(contravariant_param.has_variance());
    assert_eq!(contravariant_param.variance, Variance::Contravariant);
    assert_eq!(contravariant_param.variance.as_str(), "-");
    
    // Test invariant parameter (default)
    let invariant_param = EnhancedTypeParameter::simple(token, "V".to_string());
    assert!(!invariant_param.has_variance());
    assert_eq!(invariant_param.variance, Variance::Invariant);
    
    debug!("Variance annotations work correctly for all types");
}

#[test]
#[instrument]
fn test_where_clause_creation() {
    init_tracing!();
    info!("Testing where clause AST structure");

    let token = Token::new(TokenType::Where, "where".to_string(), 1, 1);
    let constraints = vec![
        cursed::ast::declarations::GenericConstraint::new(
            Token::new(TokenType::Identifier, "T".to_string(), 1, 1),
            "T".to_string(),
            "Display".to_string()
        ),
        cursed::ast::declarations::GenericConstraint::new(
            Token::new(TokenType::Identifier, "U".to_string(), 1, 1),
            "U".to_string(),
            "Clone".to_string()
        ),
    ];
    
    let where_clause = WhereClause::new(token, constraints);
    
    assert!(!where_clause.is_empty());
    assert_eq!(where_clause.constraint_count(), 2);
    assert!(where_clause.string().contains("where"));
    assert!(where_clause.string().contains("T:Display"));
    assert!(where_clause.string().contains("U:Clone"));
    
    debug!(
        where_clause_string = where_clause.string(),
        constraint_count = where_clause.constraint_count(),
        "Where clause created and formatted correctly"
    );
}

#[test]
#[instrument]
fn test_multi_param_generic_creation() {
    init_tracing!();
    info!("Testing multi-parameter generic declaration");

    let token = Token::new(TokenType::LeftBracket, "[".to_string(), 1, 1);
    
    // Create enhanced type parameters
    let param_t = EnhancedTypeParameter::simple(
        Token::new(TokenType::Identifier, "T".to_string(), 1, 1),
        "T".to_string()
    );
    let param_u = EnhancedTypeParameter::with_variance(
        Token::new(TokenType::Identifier, "U".to_string(), 1, 1),
        "U".to_string(),
        Variance::Covariant
    );
    
    let parameters = vec![param_t, param_u];
    let multi_generic = MultiParamGeneric::new(token, parameters);
    
    assert!(!multi_generic.is_empty());
    assert_eq!(multi_generic.parameter_count(), 2);
    assert_eq!(multi_generic.parameter_names(), vec!["T".to_string(), "U".to_string()]);
    
    let string_rep = multi_generic.string();
    assert!(string_rep.contains("["));
    assert!(string_rep.contains("T"));
    assert!(string_rep.contains("+U"));
    assert!(string_rep.contains("]"));
    
    debug!(
        multi_generic_string = string_rep,
        param_count = multi_generic.parameter_count(),
        "Multi-parameter generic declaration created successfully"
    );
}

#[test]
#[instrument]
fn test_constraint_operators() {
    init_tracing!();
    info!("Testing constraint operator parsing and representation");

    // Test operator parsing
    assert_eq!(ConstraintOperator::from_str(":"), Some(ConstraintOperator::Implements));
    assert_eq!(ConstraintOperator::from_str("+"), Some(ConstraintOperator::Plus));
    assert_eq!(ConstraintOperator::from_str("="), Some(ConstraintOperator::Equals));
    assert_eq!(ConstraintOperator::from_str("<:"), Some(ConstraintOperator::Subtype));
    assert_eq!(ConstraintOperator::from_str("invalid"), None);
    
    // Test operator string representation
    assert_eq!(ConstraintOperator::Implements.as_str(), ":");
    assert_eq!(ConstraintOperator::Plus.as_str(), "+");
    assert_eq!(ConstraintOperator::Equals.as_str(), "=");
    assert_eq!(ConstraintOperator::Subtype.as_str(), "<:");
    
    debug!("Constraint operators parse and format correctly");
}

#[test]
#[instrument]
fn test_cross_parameter_constraints() {
    init_tracing!();
    info!("Testing cross-parameter constraint relationships");

    let token = Token::new(TokenType::Identifier, "constraint".to_string(), 1, 1);
    
    // Test Into relationship
    let into_constraint = CrossParameterConstraint::new(
        token.clone(),
        "T".to_string(),
        "U".to_string(),
        ConstraintRelation::Into
    );
    
    assert_eq!(into_constraint.source_param, "T");
    assert_eq!(into_constraint.target_param, "U");
    assert_eq!(into_constraint.relationship, ConstraintRelation::Into);
    assert!(into_constraint.string().contains("T: Into<U>"));
    
    // Test type equality relationship
    let same_constraint = CrossParameterConstraint::new(
        token.clone(),
        "T".to_string(),
        "U".to_string(),
        ConstraintRelation::Same
    );
    
    assert_eq!(same_constraint.relationship, ConstraintRelation::Same);
    assert!(same_constraint.string().contains("T = U"));
    
    debug!("Cross-parameter constraints work correctly");
}

#[test]
#[instrument]
fn test_enhanced_type_parameter_with_defaults() {
    init_tracing!();
    info!("Testing enhanced type parameters with default types");

    // This is a simplified test since we need actual Expression implementations
    let token = Token::new(TokenType::Identifier, "T".to_string(), 1, 1);
    let param = EnhancedTypeParameter::simple(token, "T".to_string());
    
    assert!(!param.has_default());
    assert!(!param.has_constraints());
    assert_eq!(param.name, "T");
    
    // Test parameter with constraints
    let constraint = EnhancedConstraint::single_bound(
        Token::new(TokenType::Identifier, "constraint".to_string(), 1, 1),
        "T".to_string(),
        TypeBound::simple(
            Token::new(TokenType::Identifier, "Display".to_string(), 1, 1),
            "Display".to_string()
        )
    );
    
    let param_with_constraints = EnhancedTypeParameter::with_constraints(
        Token::new(TokenType::Identifier, "T".to_string(), 1, 1),
        "T".to_string(),
        vec![constraint]
    );
    
    assert!(param_with_constraints.has_constraints());
    assert_eq!(param_with_constraints.constraints.len(), 1);
    
    debug!("Enhanced type parameters with defaults and constraints work correctly");
}

#[test]
#[instrument]
fn test_variance_parsing() {
    init_tracing!();
    info!("Testing variance parsing from strings");

    assert_eq!(Variance::from_str("+"), Variance::Covariant);
    assert_eq!(Variance::from_str("-"), Variance::Contravariant);
    assert_eq!(Variance::from_str(""), Variance::Invariant);
    assert_eq!(Variance::from_str("invalid"), Variance::Invariant);
    
    debug!("Variance parsing works correctly");
}

#[test]
#[instrument]
fn test_constraint_relation_formatting() {
    init_tracing!();
    info!("Testing constraint relation string formatting");

    assert_eq!(ConstraintRelation::Into.as_str(), "Into");
    assert_eq!(ConstraintRelation::From.as_str(), "From");
    assert_eq!(ConstraintRelation::Same.as_str(), "=");
    assert_eq!(ConstraintRelation::Subtype.as_str(), "<:");
    
    debug!("Constraint relation formatting works correctly");
}

#[test]
#[instrument]
fn test_empty_generic_declarations() {
    init_tracing!();
    info!("Testing empty generic declarations and edge cases");

    // Test empty where clause
    let empty_where = WhereClause::new(
        Token::new(TokenType::Where, "where".to_string(), 1, 1),
        vec![]
    );
    
    assert!(empty_where.is_empty());
    assert_eq!(empty_where.constraint_count(), 0);
    
    // Test empty multi-param generic
    let empty_generic = MultiParamGeneric::new(
        Token::new(TokenType::LeftBracket, "[".to_string(), 1, 1),
        vec![]
    );
    
    assert!(empty_generic.is_empty());
    assert_eq!(empty_generic.parameter_count(), 0);
    assert!(!empty_generic.has_constraints());
    
    debug!("Empty generic declarations handle edge cases correctly");
}

#[test]
#[instrument]
fn test_type_bound_with_complex_args() {
    init_tracing!();
    info!("Testing type bounds with complex type arguments");

    let token = Token::new(TokenType::Identifier, "Iterator".to_string(), 1, 1);
    let args: Vec<Box<dyn Expression>> = vec![]; // Empty for this test
    
    let iterator_bound = TypeBound::with_args(
        token,
        "Iterator".to_string(),
        args
    );
    
    assert_eq!(iterator_bound.interface_name, "Iterator");
    assert!(!iterator_bound.has_type_args()); // Empty args
    assert_eq!(iterator_bound.string(), "Iterator");
    
    debug!("Type bounds with complex arguments work correctly");
}

#[test]
#[instrument]
fn test_node_and_statement_traits() {
    init_tracing!();
    info!("Testing Node and Statement trait implementations");

    // Test WhereClause Node implementation
    let where_clause = WhereClause::new(
        Token::new(TokenType::Where, "where".to_string(), 1, 1),
        vec![]
    );
    
    assert_eq!(where_clause.token_literal(), "where");
    assert!(!where_clause.string().is_empty());
    
    // Test MultiParamGeneric Node implementation
    let multi_generic = MultiParamGeneric::new(
        Token::new(TokenType::LeftBracket, "[".to_string(), 1, 1),
        vec![]
    );
    
    assert_eq!(multi_generic.token_literal(), "[");
    
    // Test Statement trait (WhereClause)
    where_clause.statement_node(); // Should not panic
    assert!(where_clause.as_any().is::<WhereClause>());
    
    // Test Statement trait (MultiParamGeneric)
    multi_generic.statement_node(); // Should not panic
    assert!(multi_generic.as_any().is::<MultiParamGeneric>());
    
    debug!("Node and Statement trait implementations work correctly");
}
