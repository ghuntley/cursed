/// Comprehensive Type System Integration Tests
/// 
/// Tests the integration between associated types, generic instantiation,
/// and constraint resolution in the CURSED type system.

use cursed::type_system::{
    TypeSystem, TypeExpression, TypeDefinition, TypeKind, MethodSignature,
    AssociatedType, AssociatedTypeProjection, ConstraintContext, ConstraintBinding,
    ConstraintStatus, InstantiatedType
};
use cursed::ast::traits::TypeParameter;
use cursed::ast::declarations::GenericConstraint;
use cursed::ast::types::Type;
use cursed::error::Error;
use std::collections::HashMap;

#[test]
fn test_associated_type_registry_basic_functionality() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Create an Iterator interface with Item associated type
    let item_type = AssociatedType {
        name: "Item".to_string(),
        constraints: vec![],
        default_type: None,
        documentation: Some("The type of items yielded by the iterator".to_string()),
    };
    
    let result = type_system.associated_type_registry()
        .register_interface_associated_types("Iterator", vec![item_type.clone()]);
    
    assert!(result.is_ok(), "Should successfully register associated types");
    
    // Retrieve the associated types
    let retrieved_types = type_system.associated_type_registry()
        .get_interface_associated_types("Iterator")
        .unwrap();
    
    assert_eq!(retrieved_types.len(), 1);
    assert_eq!(retrieved_types[0].name, "Item");
    assert_eq!(retrieved_types[0].documentation, Some("The type of items yielded by the iterator".to_string()));
}

#[test]
fn test_associated_type_constraint_validation() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Create an associated type with constraints
    let constrained_type = AssociatedType {
        name: "Output".to_string(),
        constraints: vec![
            GenericConstraint {
                constraint_name: "Clone".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            }
        ],
        default_type: Some(Type::Integer),
        documentation: None,
    };
    
    let result = type_system.associated_type_registry()
        .register_interface_associated_types("Transformer", vec![constrained_type]);
    
    assert!(result.is_ok(), "Should successfully validate and register constrained associated type");
}

#[test]
fn test_associated_type_projection_resolution() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Register Iterator interface with Item associated type
    let item_type = AssociatedType {
        name: "Item".to_string(),
        constraints: vec![],
        default_type: Some(Type::Integer),
        documentation: None,
    };
    
    type_system.associated_type_registry()
        .register_interface_associated_types("Iterator", vec![item_type])
        .unwrap();
    
    // Create a projection
    let projection = AssociatedTypeProjection {
        base_type: Type::Array(Box::new(Type::Integer)),
        interface_name: "Iterator".to_string(),
        associated_type_name: "Item".to_string(),
    };
    
    // Resolve the projection
    let resolved_type = type_system.associated_type_registry()
        .resolve_projection(&projection);
    
    assert!(resolved_type.is_ok(), "Should successfully resolve projection");
}

#[test]
fn test_generic_instantiation_with_constraints() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Create a generic type definition
    let vec_type = TypeDefinition {
        name: "Vec".to_string(),
        kind: TypeKind::Struct,
        type_parameters: vec![
            TypeParameter {
                name: "T".to_string(),
                constraints: vec![
                    GenericConstraint {
                        constraint_name: "Clone".to_string(),
                        type_parameters: vec!["T".to_string()],
                        bounds: vec![],
                    }
                ],
                default_type: None,
                variance: cursed::ast::traits::Variance::Invariant,
            }
        ],
        constraints: vec![
            GenericConstraint {
                constraint_name: "Clone".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            }
        ],
        methods: vec![
            MethodSignature {
                name: "push".to_string(),
                parameters: vec![TypeExpression::parameter("T")],
                return_type: None,
                type_parameters: vec![],
                constraints: vec![],
            }
        ],
        is_builtin: false,
    };
    
    // Register the type
    type_system.register_type(vec_type).unwrap();
    
    // Instantiate with normie (which should satisfy Clone constraint)
    let type_args = vec![TypeExpression::named("normie")];
    let result = type_system.instantiate_generic("Vec", &type_args);
    
    assert!(result.is_ok(), "Should successfully instantiate Vec<normie> with constraint satisfaction");
    
    let instantiated = result.unwrap();
    assert_eq!(instantiated.base_type, "Vec");
    assert_eq!(instantiated.type_arguments.len(), 1);
    assert_eq!(instantiated.type_arguments[0], TypeExpression::named("normie"));
}

#[test]
fn test_constraint_violation_detection() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Create a type that requires specific constraints
    let constrained_type = TypeDefinition {
        name: "SortedVec".to_string(),
        kind: TypeKind::Struct,
        type_parameters: vec![
            TypeParameter {
                name: "T".to_string(),
                constraints: vec![
                    GenericConstraint {
                        constraint_name: "Ord".to_string(),
                        type_parameters: vec!["T".to_string()],
                        bounds: vec![],
                    }
                ],
                default_type: None,
                variance: cursed::ast::traits::Variance::Invariant,
            }
        ],
        constraints: vec![
            GenericConstraint {
                constraint_name: "Ord".to_string(),
                type_parameters: vec!["T".to_string()],
                bounds: vec![],
            }
        ],
        methods: vec![],
        is_builtin: false,
    };
    
    type_system.register_type(constrained_type).unwrap();
    
    // Try to instantiate with a custom type that doesn't implement Ord
    let custom_struct = TypeDefinition {
        name: "CustomStruct".to_string(),
        kind: TypeKind::Struct,
        type_parameters: vec![],
        constraints: vec![],
        methods: vec![],
        is_builtin: false,
    };
    
    type_system.register_type(custom_struct).unwrap();
    
    // Attempt instantiation should fail due to constraint violation
    let type_args = vec![TypeExpression::named("CustomStruct")];
    let result = type_system.instantiate_generic("SortedVec", &type_args);
    
    // This should fail because CustomStruct doesn't implement Ord
    assert!(result.is_err(), "Should fail to instantiate due to unmet constraint");
    
    match result {
        Err(Error::Type(message)) => {
            assert!(message.contains("constraint"), "Error message should mention constraint violation");
        }
        _ => panic!("Expected Type error with constraint message"),
    }
}

#[test]
fn test_interface_implementation_checking() {
    let type_system = TypeSystem::with_builtins();
    
    // Test primitive type implementations
    let normie_type = Type::Integer;
    let registry = type_system.associated_type_registry();
    
    // normie should implement Clone
    assert!(registry.type_implements_interface(&normie_type, "Clone").unwrap());
    
    // normie should implement Debug
    assert!(registry.type_implements_interface(&normie_type, "Debug").unwrap());
    
    // normie should not implement Display (only tea does)
    assert!(!registry.type_implements_interface(&normie_type, "Display").unwrap());
    
    // Test array types
    let array_type = Type::Array(Box::new(Type::Integer));
    assert!(registry.type_implements_interface(&array_type, "Iterator").unwrap());
    assert!(registry.type_implements_interface(&array_type, "Collection").unwrap());
    
    // Test map types
    let map_type = Type::Map(Box::new(Type::String), Box::new(Type::Integer));
    assert!(registry.type_implements_interface(&map_type, "Collection").unwrap());
    assert!(registry.type_implements_interface(&map_type, "Clone").unwrap()); // Both String and Integer implement Clone
}

#[test]
fn test_type_system_constraint_checking() {
    let type_system = TypeSystem::with_builtins();
    
    // Test constraint satisfaction for built-in types
    let constraints = vec![
        GenericConstraint {
            constraint_name: "Clone".to_string(),
            type_parameters: vec!["normie".to_string()],
            bounds: vec![],
        }
    ];
    
    let normie_type = TypeExpression::named("normie");
    let result = type_system.check_constraints(&normie_type, &constraints);
    
    assert!(result.is_ok(), "Should successfully check constraints");
    assert!(result.unwrap(), "normie should satisfy Clone constraint");
}

#[test]
fn test_type_parameter_collection() {
    // Test type parameter extraction from complex expressions
    let complex_type = TypeExpression::function(
        vec![
            TypeExpression::parameter("T"),
            TypeExpression::generic("Vec", vec![TypeExpression::parameter("U")]),
        ],
        TypeExpression::map(
            TypeExpression::parameter("K"),
            TypeExpression::parameter("V")
        )
    );
    
    let params = complex_type.collect_parameters();
    assert_eq!(params, vec!["K", "T", "U", "V"]); // Should be sorted and deduplicated
}

#[test]
fn test_type_expression_concrete_checking() {
    // Concrete type
    let concrete = TypeExpression::generic("Vec", vec![TypeExpression::named("normie")]);
    assert!(concrete.is_concrete());
    
    // Type with parameters
    let with_params = TypeExpression::generic("Vec", vec![TypeExpression::parameter("T")]);
    assert!(!with_params.is_concrete());
    
    // Complex concrete type
    let complex_concrete = TypeExpression::function(
        vec![TypeExpression::named("normie")],
        TypeExpression::map(
            TypeExpression::named("tea"),
            TypeExpression::named("facts")
        )
    );
    assert!(complex_concrete.is_concrete());
}

#[test]
fn test_associated_type_statistics() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Register multiple interfaces with associated types
    let iterator_types = vec![
        AssociatedType {
            name: "Item".to_string(),
            constraints: vec![],
            default_type: None,
            documentation: None,
        }
    ];
    
    let collection_types = vec![
        AssociatedType {
            name: "Item".to_string(),
            constraints: vec![],
            default_type: None,
            documentation: None,
        },
        AssociatedType {
            name: "Index".to_string(),
            constraints: vec![],
            default_type: Some(Type::Integer),
            documentation: None,
        }
    ];
    
    type_system.associated_type_registry()
        .register_interface_associated_types("Iterator", iterator_types)
        .unwrap();
    
    type_system.associated_type_registry()
        .register_interface_associated_types("Collection", collection_types)
        .unwrap();
    
    // Get statistics
    let stats = type_system.associated_type_registry()
        .get_statistics()
        .unwrap();
    
    assert_eq!(stats.total_interfaces, 2);
    assert_eq!(stats.total_associated_types, 3);
    assert_eq!(stats.cached_projections, 0); // No projections resolved yet
}

#[test]
fn test_error_handling_and_edge_cases() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Test empty interface name
    let result = type_system.associated_type_registry()
        .get_interface_associated_types("");
    assert!(result.is_ok()); // Should return empty vector, not error
    assert!(result.unwrap().is_empty());
    
    // Test invalid constraint
    let invalid_constraint = AssociatedType {
        name: "".to_string(), // Empty name should be invalid
        constraints: vec![],
        default_type: None,
        documentation: None,
    };
    
    let result = type_system.associated_type_registry()
        .register_interface_associated_types("TestInterface", vec![invalid_constraint]);
    
    assert!(result.is_err(), "Should fail to register associated type with empty name");
}

#[test]
fn test_cache_functionality() {
    let mut type_system = TypeSystem::with_builtins();
    
    // Register interface with associated type
    let item_type = AssociatedType {
        name: "Item".to_string(),
        constraints: vec![],
        default_type: Some(Type::Integer),
        documentation: None,
    };
    
    type_system.associated_type_registry()
        .register_interface_associated_types("Iterator", vec![item_type])
        .unwrap();
    
    // Create projection
    let projection = AssociatedTypeProjection {
        base_type: Type::Array(Box::new(Type::Integer)),
        interface_name: "Iterator".to_string(),
        associated_type_name: "Item".to_string(),
    };
    
    // First resolution should cache the result
    let result1 = type_system.associated_type_registry()
        .resolve_projection(&projection)
        .unwrap();
    
    // Second resolution should use cache
    let result2 = type_system.associated_type_registry()
        .resolve_projection(&projection)
        .unwrap();
    
    assert_eq!(result1, result2, "Cached result should be identical");
    
    // Check statistics show cached projection
    let stats = type_system.associated_type_registry()
        .get_statistics()
        .unwrap();
    assert_eq!(stats.cached_projections, 1);
}
