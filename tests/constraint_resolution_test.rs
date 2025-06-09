//! Comprehensive tests for the constraint resolution system
//!
//! This test suite validates the core type constraint resolution and type checking
//! logic for the enhanced generic system, including interface-based constraints,
//! where clause constraints, multi-parameter generic constraints, and constraint
//! satisfaction during type checking.

use cursed::ast::{GenericConstraint, FunctionStatement, SquadStatement, CollabStatement, Parameter, TypeParameter};
use cursed::ast::{FieldDeclaration, MethodDeclaration};
use cursed::ast::calls::CallExpression;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::core::constraint_resolver::{ConstraintResolver, ConstraintResolutionResult, ConstraintViolation};
use cursed::core::constraint_validator::{ConstraintValidator, ValidationContext, ValidationResult};
use cursed::core::enhanced_type_inference::{EnhancedTypeInference, InferenceContext, InferenceResult};
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod common;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = common::tracing::setup();
    };
}

/// Create a test token
fn test_token() -> Token {
    Token::new(TokenType::String, "test".to_string(), 1)
}

/// Create a test type parameter
fn create_test_type_parameter(name: &str) -> TypeParameter {
    TypeParameter {
        token: test_token(),
        name: name.to_string(),
        constraints: Vec::new(),
    }
}

/// Create a test generic constraint
fn create_test_constraint(param_name: &str, interface_name: &str) -> GenericConstraint {
    GenericConstraint::new(
        test_token(),
        param_name.to_string(),
        interface_name.to_string(),
    )
}

/// Create a test parameter
fn create_test_parameter(name: &str, type_: Type) -> Parameter {
    Parameter {
        token: test_token(),
        name: name.to_string(),
        parameter_type: type_,
    }
}

/// Create a test function with constraints
fn create_test_function_with_constraints(
    name: &str,
    type_params: Vec<&str>,
    constraints: Vec<(&str, &str)>,
    return_type: Type,
) -> FunctionStatement {
    let type_parameters = type_params.into_iter()
        .map(create_test_type_parameter)
        .collect();
    
    let generic_constraints = constraints.into_iter()
        .map(|(param, interface)| create_test_constraint(param, interface))
        .collect();
    
    FunctionStatement {
        token: test_token(),
        name: test_token(),
        type_parameters,
        parameters: Vec::new(),
        return_type,
        body: Vec::new(),
        generic_constraints,
    }
}

/// Create a test struct with constraints
fn create_test_struct_with_constraints(
    name: &str,
    type_params: Vec<&str>,
    constraints: Vec<(&str, &str)>,
) -> SquadStatement {
    let type_parameters = type_params.into_iter()
        .map(create_test_type_parameter)
        .collect();
    
    let generic_constraints = constraints.into_iter()
        .map(|(param, interface)| create_test_constraint(param, interface))
        .collect();
    
    SquadStatement {
        token: test_token(),
        name: test_token(),
        type_parameters,
        fields: Vec::new(),
        methods: Vec::new(),
        generic_constraints,
    }
}

/// Create a test interface with methods
fn create_test_interface(name: &str, methods: Vec<(&str, Vec<Type>, Type)>) -> CollabStatement {
    let interface_methods = methods.into_iter()
        .map(|(method_name, param_types, return_type)| {
            let parameters = param_types.into_iter()
                .enumerate()
                .map(|(i, param_type)| create_test_parameter(&format!("param{}", i), param_type))
                .collect();
            
            MethodDeclaration {
                token: test_token(),
                name: test_token(),
                type_parameters: Vec::new(),
                parameters,
                return_type,
                generic_constraints: Vec::new(),
            }
        })
        .collect();
    
    CollabStatement {
        token: test_token(),
        name: test_token(),
        type_parameters: Vec::new(),
        methods: interface_methods,
        generic_constraints: Vec::new(),
    }
}

#[test]
fn test_basic_constraint_resolution() {
    init_tracing!();
    
    // Create test components
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create a simple function with one constraint: func[T: Display](x T) T
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T"],
        vec![("T", "Display")],
        Type::TypeParam("T".to_string()),
    );
    
    // Test with a type that should satisfy the constraint
    let type_arguments = vec![Type::Tea]; // String type
    
    let result = constraint_resolver.resolve_function_constraints(&function, &type_arguments);
    assert!(result.is_ok());
    
    let resolution = result.unwrap();
    // We expect this to fail in basic setup since we haven't registered implementations
    assert!(!resolution.satisfied);
    assert!(!resolution.violations.is_empty());
}

#[test]
fn test_multi_parameter_constraint_resolution() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create function with multiple constraints: func[T: Display, U: Comparable](x T, y U) T
    let function = create_test_function_with_constraints(
        "multi_param_func",
        vec!["T", "U"],
        vec![("T", "Display"), ("U", "Comparable")],
        Type::TypeParam("T".to_string()),
    );
    
    let type_arguments = vec![Type::Tea, Type::Thicc]; // String, Integer
    
    let result = constraint_resolver.resolve_function_constraints(&function, &type_arguments);
    assert!(result.is_ok());
    
    let resolution = result.unwrap();
    // Should have violations for both constraints in basic setup
    assert!(!resolution.satisfied);
    assert_eq!(resolution.violations.len(), 2);
}

#[test]
fn test_struct_constraint_resolution() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create struct with constraint: struct Container[T: Serializable] { data: T }
    let struct_stmt = create_test_struct_with_constraints(
        "Container",
        vec!["T"],
        vec![("T", "Serializable")],
    );
    
    let type_arguments = vec![Type::Tea]; // String type
    
    let result = constraint_resolver.resolve_struct_constraints(&struct_stmt, &type_arguments);
    assert!(result.is_ok());
    
    let resolution = result.unwrap();
    // Should fail in basic setup
    assert!(!resolution.satisfied);
}

#[test]
fn test_constraint_violation_details() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T"],
        vec![("T", "Display")],
        Type::TypeParam("T".to_string()),
    );
    
    let type_arguments = vec![Type::Thicc]; // Integer type
    
    let result = constraint_resolver.resolve_function_constraints(&function, &type_arguments);
    assert!(result.is_ok());
    
    let resolution = result.unwrap();
    assert!(!resolution.satisfied);
    assert_eq!(resolution.violations.len(), 1);
    
    let violation = &resolution.violations[0];
    assert_eq!(violation.type_parameter, "T");
    assert_eq!(violation.concrete_type, Type::Thicc);
    assert_eq!(violation.interface_constraint, "Display");
    assert!(!violation.context.is_empty());
}

#[test]
fn test_constraint_propagation() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create some constraints
    let constraints = vec![
        create_test_constraint("T", "Display"),
        create_test_constraint("U", "Comparable"),
    ];
    
    // Create type relationships (T depends on U)
    let mut type_relationships = HashMap::new();
    type_relationships.insert("T".to_string(), vec!["U".to_string()]);
    
    let result = constraint_resolver.propagate_constraints(&constraints, &type_relationships);
    assert!(result.is_ok());
    
    let propagated = result.unwrap();
    // Should have original constraints plus propagated ones
    assert!(propagated.len() >= constraints.len());
}

#[test]
fn test_constraint_unification() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    let constraints_a = vec![create_test_constraint("T", "Display")];
    let constraints_b = vec![create_test_constraint("T", "Comparable")];
    
    let result = constraint_resolver.unify_constraints(&constraints_a, &constraints_b);
    assert!(result.is_ok());
    
    let unified = result.unwrap();
    // Should contain both constraints for parameter T
    assert!(!unified.is_empty());
}

#[test]
fn test_constraint_validator_basic() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone())
    ));
    
    let mut validator = ConstraintValidator::new(
        type_checker,
        interface_registry,
        constraint_resolver,
    );
    
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T"],
        vec![("T", "Display")],
        Type::TypeParam("T".to_string()),
    );
    
    // Create a mock call expression
    let call_expr = CallExpression {
        token: test_token(),
        function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(), "test_func".to_string())),
        arguments: Vec::new(),
        type_arguments: vec![Type::Tea], // String type
    };
    
    let context = ValidationContext::new();
    
    let result = validator.validate_function_call_constraints(&call_expr, &function, &context);
    assert!(result.is_ok());
    
    let validation = result.unwrap();
    // Should fail in basic setup without interface implementations
    assert!(!validation.valid);
}

#[test]
fn test_constraint_validator_struct_instantiation() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone())
    ));
    
    let mut validator = ConstraintValidator::new(
        type_checker,
        interface_registry,
        constraint_resolver,
    );
    
    let struct_stmt = create_test_struct_with_constraints(
        "Container",
        vec!["T"],
        vec![("T", "Serializable")],
    );
    
    let type_arguments = vec![Type::Tea];
    let context = ValidationContext::new();
    
    let result = validator.validate_struct_instantiation_constraints(&struct_stmt, &type_arguments, &context);
    assert!(result.is_ok());
    
    let validation = result.unwrap();
    // Should fail without proper interface setup
    assert!(!validation.valid);
}

#[test]
fn test_constraint_validator_hierarchies() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone())
    ));
    
    let mut validator = ConstraintValidator::new(
        type_checker,
        interface_registry,
        constraint_resolver,
    );
    
    let constraints = vec![
        create_test_constraint("T", "Display"),
        create_test_constraint("T", "Comparable"),
        create_test_constraint("U", "Serializable"),
    ];
    
    let context = ValidationContext::new();
    
    let result = validator.validate_constraint_hierarchies(&constraints, &context);
    assert!(result.is_ok());
    
    let validation = result.unwrap();
    // Should be valid for hierarchy checking (no circular dependencies)
    assert!(validation.valid);
}

#[test]
fn test_enhanced_type_inference_basic() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry)
    ));
    
    let mut inference = EnhancedTypeInference::new(type_checker, constraint_resolver);
    
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T"],
        vec![("T", "Display")],
        Type::TypeParam("T".to_string()),
    );
    
    // Create a call expression without explicit type arguments
    let call_expr = CallExpression {
        token: test_token(),
        function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(), "test_func".to_string())),
        arguments: Vec::new(),
        type_arguments: Vec::new(), // No explicit types
    };
    
    let context = InferenceContext::new();
    
    let result = inference.infer_generic_call_type(&call_expr, &function, &context);
    assert!(result.is_ok());
    
    let inference_result = result.unwrap();
    // Should infer some type, even if not fully resolved
    assert!(inference_result.confidence > 0.0);
}

#[test]
fn test_enhanced_type_inference_with_explicit_types() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry)
    ));
    
    let mut inference = EnhancedTypeInference::new(type_checker, constraint_resolver);
    
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T"],
        vec![("T", "Display")],
        Type::TypeParam("T".to_string()),
    );
    
    // Create a call expression with explicit type arguments
    let call_expr = CallExpression {
        token: test_token(),
        function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(), "test_func".to_string())),
        arguments: Vec::new(),
        type_arguments: vec![Type::Tea], // Explicit string type
    };
    
    let context = InferenceContext::new();
    
    let result = inference.infer_generic_call_type(&call_expr, &function, &context);
    assert!(result.is_ok());
    
    let inference_result = result.unwrap();
    // Should have lower confidence due to constraint violations
    assert!(inference_result.confidence < 1.0);
    assert!(inference_result.constraint_result.is_some());
}

#[test]
fn test_enhanced_type_inference_struct_instantiation() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry)
    ));
    
    let mut inference = EnhancedTypeInference::new(type_checker, constraint_resolver);
    
    let struct_stmt = create_test_struct_with_constraints(
        "Container",
        vec!["T"],
        vec![("T", "Serializable")],
    );
    
    // Create field types for inference
    let mut field_types = HashMap::new();
    field_types.insert("data".to_string(), Type::Tea);
    
    let context = InferenceContext::new();
    
    let result = inference.infer_struct_instantiation_type(&struct_stmt, &field_types, &context);
    assert!(result.is_ok());
    
    let inference_result = result.unwrap();
    // Should infer a struct type
    match inference_result.inferred_type {
        Type::Struct(name, _) => {
            assert_eq!(name, "Container");
        }
        _ => panic!("Expected struct type"),
    }
}

#[test]
fn test_type_parameter_substitution() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry)
    ));
    
    let inference = EnhancedTypeInference::new(type_checker, constraint_resolver);
    
    // Create a complex type with type parameters
    let generic_type = Type::Generic(
        "List".to_string(),
        vec![Box::new(Type::TypeParam("T".to_string()))],
    );
    
    // Create substitution map
    let mut substitutions = HashMap::new();
    substitutions.insert("T".to_string(), Type::Tea);
    
    let result = inference.substitute_type_parameters(&generic_type, &substitutions);
    assert!(result.is_ok());
    
    let substituted = result.unwrap();
    match substituted {
        Type::Generic(name, args) => {
            assert_eq!(name, "List");
            assert_eq!(args.len(), 1);
            assert_eq!(*args[0], Type::Tea);
        }
        _ => panic!("Expected generic type"),
    }
}

#[test]
fn test_constraint_dependency_graph() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create constraints with dependencies
    let constraints = vec![
        create_test_constraint("T", "Display"),
        create_test_constraint("U", "Comparable"),
        create_test_constraint("V", "Serializable"),
    ];
    
    let context = HashMap::new();
    
    let result = constraint_resolver.infer_constraint_satisfying_types(&constraints, &context);
    assert!(result.is_ok());
    
    let inferred = result.unwrap();
    // Should return some inferred types
    assert!(!inferred.is_empty() || constraints.is_empty());
}

#[test]
fn test_validation_context_operations() {
    init_tracing!();
    
    let mut context = ValidationContext::new();
    assert_eq!(context.validation_depth, 0);
    
    // Test adding bindings and constraints
    context.add_type_binding("T".to_string(), Type::Tea);
    context.add_constraint(create_test_constraint("T", "Display"));
    
    assert_eq!(context.type_bindings.len(), 1);
    assert_eq!(context.active_constraints.len(), 1);
    
    // Test child context
    let child = context.child_context();
    assert_eq!(child.validation_depth, 1);
    assert_eq!(child.type_bindings.len(), 1);
    assert_eq!(child.active_constraints.len(), 1);
}

#[test]
fn test_inference_context_operations() {
    init_tracing!();
    
    let mut context = InferenceContext::new();
    assert!(context.type_bindings.is_empty());
    
    // Test adding bindings and constraints
    context.add_binding("T".to_string(), Type::Tea);
    context.add_constraint(create_test_constraint("T", "Display"));
    
    assert_eq!(context.type_bindings.len(), 1);
    assert_eq!(context.constraints.len(), 1);
    
    // Test with expected return type
    let context_with_return = InferenceContext::new()
        .with_expected_return_type(Type::Thicc);
    
    assert_eq!(context_with_return.expected_return_type, Some(Type::Thicc));
}

#[test]
fn test_constraint_resolution_caching() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone())
    ));
    
    let mut validator = ConstraintValidator::new(
        type_checker,
        interface_registry,
        constraint_resolver,
    );
    
    // Get initial metrics
    let initial_metrics = validator.get_metrics().clone();
    
    // Perform some validation operations
    let constraints = vec![create_test_constraint("T", "Display")];
    let context = ValidationContext::new();
    
    let _ = validator.validate_constraint_hierarchies(&constraints, &context);
    
    // Check that metrics were updated
    let final_metrics = validator.get_metrics();
    assert!(final_metrics.validation_time_us >= initial_metrics.validation_time_us);
    
    // Test cache clearing
    validator.clear_cache();
    assert_eq!(validator.get_metrics().cache_hit_rate, 0.0);
}

#[test]
fn test_error_generation_for_violations() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = ConstraintResolver::new(type_checker, interface_registry);
    
    // Create a resolution result with violations
    let resolution_result = ConstraintResolutionResult {
        satisfied: false,
        violations: vec![
            ConstraintViolation {
                type_parameter: "T".to_string(),
                concrete_type: Type::Thicc,
                interface_constraint: "Display".to_string(),
                context: "Test violation".to_string(),
                missing_methods: vec!["display".to_string()],
            }
        ],
        type_substitutions: HashMap::new(),
        inferred_types: HashMap::new(),
    };
    
    let result = constraint_resolver.generate_error_report(&resolution_result);
    assert!(result.is_ok());
    
    let errors = result.unwrap();
    assert_eq!(errors.len(), 1);
    
    let error_msg = format!("{}", errors[0]);
    assert!(error_msg.contains("Display"));
    assert!(error_msg.contains("Thicc") || error_msg.contains("T"));
}

#[test]
fn test_performance_metrics() {
    init_tracing!();
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()));
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()));
    let constraint_resolver = Arc::new(RwLock::new(
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone())
    ));
    
    let mut validator = ConstraintValidator::new(
        type_checker,
        interface_registry,
        constraint_resolver,
    );
    
    // Perform operations to generate metrics
    let function = create_test_function_with_constraints(
        "test_func",
        vec!["T", "U"],
        vec![("T", "Display"), ("U", "Comparable")],
        Type::TypeParam("T".to_string()),
    );
    
    let call_expr = CallExpression {
        token: test_token(),
        function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(), "test_func".to_string())),
        arguments: Vec::new(),
        type_arguments: vec![Type::Tea, Type::Thicc],
    };
    
    let context = ValidationContext::new();
    
    let _ = validator.validate_function_call_constraints(&call_expr, &function, &context);
    
    let metrics = validator.get_metrics();
    assert!(metrics.constraints_checked > 0);
    assert!(metrics.validation_time_us >= 0);
}
