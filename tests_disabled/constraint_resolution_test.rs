/// Constraint Resolution System Tests
/// 
/// Tests the constraint resolution algorithms, type unification,
/// and constraint satisfaction checking in the CURSED type system.

use cursed::type_system::{
    ConstraintResolver, ConstraintSolution, ConstraintViolation, ViolationReason,
    TypeEnvironment, TypeExpression, TypeDefinition, TypeKind, MethodSignature,
    ConstraintContext, ConstraintBinding, ConstraintStatus
};
use cursed::type_system::{GenericConstraint, TypeParameter};
use cursed::error::Error;
use std::collections::HashMap;

#[test]
fn test_constraint_resolver_creation() {
    let resolver = ConstraintResolver::new();
    
    // Verify built-in constraints are registered
    assert!(resolver.validate_constraint(
        &GenericConstraint {
            constraint_name: "Clone".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        },
        &TypeEnvironment::new()
    ).is_ok());
    
    assert!(resolver.validate_constraint(
        &GenericConstraint {
            constraint_name: "Debug".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        },
        &TypeEnvironment::new()
    ).is_ok());
}

#[test]
fn test_primitive_type_constraint_satisfaction() {
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    // Test Clone constraint on primitive types
    let clone_constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["normie".to_string()],
        bounds: vec![],
    };
    
    let normie_type = TypeExpression::named("normie");
    let result = resolver.check_satisfaction(&normie_type, &[clone_constraint.clone()], &env);
    
    assert!(result.is_ok(), "Should successfully check constraint satisfaction");
    assert!(result.unwrap(), "normie should satisfy Clone constraint");
    
    // Test Debug constraint
    let debug_constraint = GenericConstraint {
        constraint_name: "Debug".to_string(),
        type_parameters: vec!["facts".to_string()],
        bounds: vec![],
    };
    
    let facts_type = TypeExpression::named("facts");
    let result = resolver.check_satisfaction(&facts_type, &[debug_constraint], &env);
    
    assert!(result.is_ok(), "Should successfully check Debug constraint");
    assert!(result.unwrap(), "facts should satisfy Debug constraint");
}

#[test]
fn test_constraint_validation() {
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    // Valid constraint
    let valid_constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    
    assert!(resolver.validate_constraint(&valid_constraint, &env).is_ok());
    
    // Invalid constraint with unknown name
    let invalid_constraint = GenericConstraint {
        constraint_name: "UnknownTrait".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    
    let result = resolver.validate_constraint(&invalid_constraint, &env);
    assert!(result.is_err(), "Should reject unknown constraint");
}

#[test]
fn test_complex_constraint_satisfaction() {
    let resolver = ConstraintResolver::new();
    let mut env = TypeEnvironment::new();
    
    // Create a custom type that implements Clone
    let custom_type = TypeDefinition {
        name: "CustomCloneable".to_string(),
        kind: TypeKind::Struct,
        type_parameters: vec![],
        constraints: vec![],
        methods: vec![
            MethodSignature {
                name: "clone".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("CustomCloneable")),
                type_parameters: vec![],
                constraints: vec![],
            }
        ],
        is_builtin: false,
    };
    
    env.type_definitions.insert("CustomCloneable".to_string(), custom_type);
    
    // Test Clone constraint on custom type
    let clone_constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["CustomCloneable".to_string()],
        bounds: vec![],
    };
    
    let custom_type_expr = TypeExpression::named("CustomCloneable");
    let result = resolver.check_satisfaction(&custom_type_expr, &[clone_constraint], &env);
    
    // This will currently return true for custom types with clone method
    assert!(result.is_ok(), "Should handle custom type constraint checking");
}

#[test]
fn test_constraint_context_resolution() {
    let mut resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    // Create constraint context
    let mut context = ConstraintContext {
        scope_id: "test_scope".to_string(),
        active_constraints: vec![],
        type_bindings: HashMap::new(),
    };
    
    // Add constraint binding
    let constraint_binding = ConstraintBinding {
        constraint: GenericConstraint {
            constraint_name: "Clone".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        },
        bound_types: vec!["normie".to_string()],
        satisfaction_status: ConstraintStatus::Pending,
    };
    
    context.active_constraints.push(constraint_binding);
    context.type_bindings.insert("T".to_string(), TypeExpression::named("normie"));
    
    // Resolve constraints
    let result = resolver.resolve_constraints(&context, &env);
    
    assert!(result.is_ok(), "Should successfully resolve constraints in context");
    
    let solution = result.unwrap();
    assert!(solution.is_satisfied, "Constraint should be satisfied");
    assert!(solution.violations.is_empty(), "Should have no violations");
}

#[test]
fn test_type_unification() {
    use cursed::type_system::constraint_resolver::TypeUnifier;
    
    let mut unifier = TypeUnifier::new();
    
    // Test parameter unification
    let param_type = TypeExpression::parameter("T");
    let concrete_type = TypeExpression::named("normie");
    
    let result = unifier.unify(&param_type, &concrete_type);
    assert!(result.is_ok(), "Should successfully unify parameter with concrete type");
    
    let substitutions = result.unwrap();
    assert_eq!(substitutions.get("T"), Some(&TypeExpression::named("normie")));
    
    // Test constructor unification
    let type1 = TypeExpression::named("normie");
    let type2 = TypeExpression::named("normie");
    
    let result = unifier.unify(&type1, &type2);
    assert!(result.is_ok(), "Should successfully unify identical types");
    assert!(result.unwrap().is_empty(), "No substitutions needed for identical types");
    
    // Test failed unification
    let type1 = TypeExpression::named("normie");
    let type2 = TypeExpression::named("tea");
    
    let result = unifier.unify(&type1, &type2);
    assert!(result.is_err(), "Should fail to unify different concrete types");
}

#[test]
fn test_generic_type_unification() {
    use cursed::type_system::constraint_resolver::TypeUnifier;
    
    let mut unifier = TypeUnifier::new();
    
    // Test generic type unification
    let generic1 = TypeExpression::generic("Vec", vec![TypeExpression::parameter("T")]);
    let generic2 = TypeExpression::generic("Vec", vec![TypeExpression::named("normie")]);
    
    let result = unifier.unify(&generic1, &generic2);
    assert!(result.is_ok(), "Should successfully unify generic types");
    
    let substitutions = result.unwrap();
    assert_eq!(substitutions.get("T"), Some(&TypeExpression::named("normie")));
    
    // Test generic type mismatch
    let generic1 = TypeExpression::generic("Vec", vec![TypeExpression::parameter("T")]);
    let generic2 = TypeExpression::generic("Map", vec![TypeExpression::parameter("T"), TypeExpression::parameter("U")]);
    
    let result = unifier.unify(&generic1, &generic2);
    assert!(result.is_err(), "Should fail to unify different generic types");
}

#[test]
fn test_function_type_unification() {
    use cursed::type_system::constraint_resolver::TypeUnifier;
    
    let mut unifier = TypeUnifier::new();
    
    // Test function type unification
    let func1 = TypeExpression::function(
        vec![TypeExpression::parameter("T")],
        TypeExpression::parameter("U")
    );
    let func2 = TypeExpression::function(
        vec![TypeExpression::named("normie")],
        TypeExpression::named("tea")
    );
    
    let result = unifier.unify(&func1, &func2);
    assert!(result.is_ok(), "Should successfully unify function types");
    
    let substitutions = result.unwrap();
    assert_eq!(substitutions.get("T"), Some(&TypeExpression::named("normie")));
    assert_eq!(substitutions.get("U"), Some(&TypeExpression::named("tea")));
    
    // Test function arity mismatch
    let func1 = TypeExpression::function(
        vec![TypeExpression::parameter("T")],
        TypeExpression::parameter("U")
    );
    let func2 = TypeExpression::function(
        vec![TypeExpression::parameter("T"), TypeExpression::parameter("V")],
        TypeExpression::parameter("U")
    );
    
    let result = unifier.unify(&func1, &func2);
    assert!(result.is_err(), "Should fail to unify functions with different parameter counts");
}

#[test]
fn test_occurs_check() {
    use cursed::type_system::constraint_resolver::TypeUnifier;
    
    let mut unifier = TypeUnifier::new();
    
    // Test occurs check failure (infinite type)
    let param = TypeExpression::parameter("T");
    let recursive = TypeExpression::array(TypeExpression::parameter("T"));
    
    let result = unifier.unify(&param, &recursive);
    assert!(result.is_err(), "Should fail occurs check for recursive type");
    
    match result {
        Err(constraint_violation) => {
            // Placeholder - just verify we got an error
            assert!(true, "Expected constraint violation for occurs check");
        }
        _ => panic!("Expected constraint violation"),
    }
}

#[test]
fn test_constraint_violation_detection() {
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    // Test constraint that should fail
    let impossible_constraint = GenericConstraint {
        constraint_name: "NonExistentTrait".to_string(),
        type_parameters: vec!["normie".to_string()],
        bounds: vec![],
    };
    
    let normie_type = TypeExpression::named("normie");
    let result = resolver.check_satisfaction(&normie_type, &[impossible_constraint], &env);
    
    // This should fail due to unknown constraint
    assert!(result.is_err(), "Should fail for unknown constraint");
}

#[test]
fn test_complex_type_constraint_satisfaction() {
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    // Test array type with Clone constraint
    let array_type = TypeExpression::array(TypeExpression::named("normie"));
    let clone_constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["[normie]".to_string()],
        bounds: vec![],
    };
    
    let result = resolver.check_satisfaction(&array_type, &[clone_constraint], &env);
    assert!(result.is_ok(), "Should handle array type constraint checking");
    
    // Test map type
    let map_type = TypeExpression::map(
        TypeExpression::named("tea"),
        TypeExpression::named("normie")
    );
    let clone_constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["tea[tea]normie".to_string()],
        bounds: vec![],
    };
    
    let result = resolver.check_satisfaction(&map_type, &[clone_constraint], &env);
    assert!(result.is_ok(), "Should handle map type constraint checking");
}

#[test]
fn test_constraint_cache_functionality() {
    let resolver = ConstraintResolver::new();
    let env = TypeEnvironment::new();
    
    let constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["normie".to_string()],
        bounds: vec![],
    };
    
    let normie_type = TypeExpression::named("normie");
    
    // First check should populate cache
    let result1 = resolver.check_satisfaction(&normie_type, &[constraint.clone()], &env);
    assert!(result1.is_ok());
    
    // Second check should use cache
    let result2 = resolver.check_satisfaction(&normie_type, &[constraint], &env);
    assert!(result2.is_ok());
    
    // Both results should be identical
    assert_eq!(result1.unwrap(), result2.unwrap());
}

#[test]
fn test_constraint_propagation() {
    use cursed::type_system::constraint_resolver::{ConstraintPropagator, ConstraintGraph};
    
    let propagator = ConstraintPropagator::new();
    
    // Create constraint bindings
    let bindings = vec![
        ConstraintBinding {
            constraint: GenericConstraint {
                constraint_name: "Clone".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            },
            bound_types: vec!["T".to_string()],
            satisfaction_status: ConstraintStatus::Pending,
        },
        ConstraintBinding {
            constraint: GenericConstraint {
                constraint_name: "Debug".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            },
            bound_types: vec!["T".to_string()],
            satisfaction_status: ConstraintStatus::Pending,
        }
    ];
    
    // Build constraint graph
    let result = propagator.build_constraint_graph(&bindings);
    assert!(result.is_ok(), "Should successfully build constraint graph");
    
    let graph = result.unwrap();
    assert_eq!(graph.nodes.len(), 2, "Should have two constraint nodes");
}

#[test]
fn test_topological_sort() {
    use cursed::type_system::constraint_resolver::ConstraintGraph;
    
    let mut graph = ConstraintGraph::new();
    
    // Add nodes (constraints)
    let node1 = cursed::type_system::constraint_resolver::ConstraintNode {
        id: "constraint_0".to_string(),
        constraint: GenericConstraint {
            constraint_name: "Clone".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        },
        bound_types: vec!["T".to_string()],
        status: ConstraintStatus::Pending,
        dependencies: vec![],
        binding: ConstraintBinding {
            constraint: GenericConstraint {
                constraint_name: "Clone".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            },
            bound_types: vec!["T".to_string()],
            satisfaction_status: ConstraintStatus::Pending,
        },
    };
    
    let node2 = cursed::type_system::constraint_resolver::ConstraintNode {
        id: "constraint_1".to_string(),
        constraint: GenericConstraint {
            constraint_name: "Debug".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec![],
        },
        bound_types: vec!["T".to_string()],
        status: ConstraintStatus::Pending,
        dependencies: vec![],
        binding: ConstraintBinding {
            constraint: GenericConstraint {
                constraint_name: "Debug".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            },
            bound_types: vec!["T".to_string()],
            satisfaction_status: ConstraintStatus::Pending,
        },
    };
    
    graph.add_node(node1);
    graph.add_node(node2);
    
    // Perform topological sort
    let result = graph.topological_sort();
    assert!(result.is_ok(), "Should successfully perform topological sort");
    
    let order = result.unwrap();
    assert_eq!(order.len(), 2, "Should have two items in sort order");
}

#[test]
fn test_error_message_generation() {
    use cursed::type_system::constraint_resolver::ViolationReason;
    
    let resolver = ConstraintResolver::new();
    
    // Test violation reason determination
    let constraint = GenericConstraint {
        constraint_name: "Clone".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    
    let type_expr = TypeExpression::named("UnknownType");
    let env = TypeEnvironment::new();
    
    let reason = resolver.determine_violation_reason(&constraint, &type_expr, &env);
    
    // Should detect missing interface
    match reason {
        ViolationReason::MissingInterface(interface_name) => {
            assert_eq!(interface_name, "Clone");
        }
        _ => panic!("Expected MissingInterface violation reason"),
    }
    
    // Test suggested fixes generation
    let fixes = resolver.generate_suggested_fixes(&constraint, &type_expr);
    assert!(!fixes.is_empty(), "Should provide suggested fixes");
    assert!(fixes.iter().any(|fix| fix.contains("Clone")), "Should suggest implementing Clone");
}
