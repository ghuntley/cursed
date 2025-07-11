use cursed::type_system::*;
use cursed::ast::*;
use cursed::error::CursedError;
use std::collections::HashMap;

#[test]
fn test_advanced_constraints_creation() {
    let mut checker = AdvancedConstraintChecker::new();
    
    // Test constraint creation
    let constraints = vec![
        AdvancedConstraint::TraitBound("T".to_string(), "Clone".to_string()),
        AdvancedConstraint::DebugConstraint("T".to_string()),
        AdvancedConstraint::SizedConstraint("T".to_string()),
    ];
    
    checker.add_type_constraints("T", constraints);
    
    // Test constraint checking
    let mut type_bindings = HashMap::new();
    type_bindings.insert("T".to_string(), TypeExpression::named("normie"));
    
    assert!(checker.check_constraints(&type_bindings).is_ok());
}

#[test]
fn test_generic_interfaces_registration() {
    let mut checker = GenericInterfaceChecker::new();
    
    // Test built-in interfaces
    assert!(checker.get_interface("Clone").is_some());
    assert!(checker.get_interface("Iterator").is_some());
    assert!(checker.get_interface("IntoIterator").is_some());
    assert!(checker.get_interface("From").is_some());
    assert!(checker.get_interface("Functor").is_some());
    
    // Test custom interface registration
    let custom_interface = GenericInterface {
        name: "CustomInterface".to_string(),
        type_parameters: vec![
            GenericTypeParameter {
                name: "T".to_string(),
                variance: Variance::Invariant,
                constraints: vec![AdvancedConstraint::TraitBound("T".to_string(), "Clone".to_string())],
                default_type: None,
            }
        ],
        associated_types: vec![
            AssociatedType {
                name: "Output".to_string(),
                constraints: vec![AdvancedConstraint::DebugConstraint("Output".to_string())],
                default_type: None,
            }
        ],
        methods: vec![
            InterfaceMethod {
                name: "process".to_string(),
                type_parameters: vec![],
                parameters: vec![
                    Parameter {
                        name: "self".to_string(),
                        param_type: Some(Type::SelfType),
                        default_value: None,
                    }
                ],
                return_type: Some(Type::AssociatedType("Self".to_string(), "Output".to_string())),
                where_clauses: vec![],
                has_default: false,
            }
        ],
        superinterfaces: vec![],
        where_clauses: vec![],
        default_implementations: HashMap::new(),
    };
    
    assert!(checker.register_interface(custom_interface).is_ok());
    assert!(checker.get_interface("CustomInterface").is_some());
}

#[test]
fn test_higher_kinded_types() {
    let mut hkt_system = HigherKindedTypeSystem::new();
    
    // Test built-in constructors
    assert!(hkt_system.get_constructor("Array").is_some());
    assert!(hkt_system.get_constructor("Option").is_some());
    assert!(hkt_system.get_constructor("Result").is_some());
    
    // Test constructor application
    let int_type = TypeExpression::named("normie");
    let array_int = hkt_system.apply_constructor("Array", vec![int_type.clone()]);
    assert!(array_int.is_ok());
    
    let option_int = hkt_system.apply_constructor("Option", vec![int_type.clone()]);
    assert!(option_int.is_ok());
    
    let string_type = TypeExpression::named("tea");
    let result_int_string = hkt_system.apply_constructor("Result", vec![int_type.clone(), string_type.clone()]);
    assert!(result_int_string.is_ok());
}

#[test]
fn test_constraint_dependency_graph() {
    let mut graph = ConstraintDependencyGraph::new();
    
    // Add nodes with dependencies
    graph.add_node(AdvancedConstraintNode {
        name: "T".to_string(),
        constraints: vec![AdvancedConstraint::TraitBound("T".to_string(), "Clone".to_string())],
        dependencies: vec!["U".to_string()],
    });
    
    graph.add_node(AdvancedConstraintNode {
        name: "U".to_string(),
        constraints: vec![AdvancedConstraint::DebugConstraint("U".to_string())],
        dependencies: vec![],
    });
    
    graph.add_node(AdvancedConstraintNode {
        name: "V".to_string(),
        constraints: vec![AdvancedConstraint::SizedConstraint("V".to_string())],
        dependencies: vec!["T".to_string()],
    });
    
    // Test topological sort
    let order = graph.topological_sort();
    assert!(order.is_ok());
    
    let resolution_order = order.unwrap();
    assert_eq!(resolution_order.len(), 3);
    
    // U should come before T, T should come before V
    let u_pos = resolution_order.iter().position(|x| x == "U").unwrap();
    let t_pos = resolution_order.iter().position(|x| x == "T").unwrap();
    let v_pos = resolution_order.iter().position(|x| x == "V").unwrap();
    
    assert!(u_pos < t_pos);
    assert!(t_pos < v_pos);
}

#[test]
fn test_circular_dependency_detection() {
    let mut graph = ConstraintDependencyGraph::new();
    
    // Create circular dependency
    graph.add_node(AdvancedConstraintNode {
        name: "T".to_string(),
        constraints: vec![],
        dependencies: vec!["U".to_string()],
    });
    
    graph.add_node(AdvancedConstraintNode {
        name: "U".to_string(),
        constraints: vec![],
        dependencies: vec!["T".to_string()],
    });
    
    // Should detect circular dependency
    assert!(graph.topological_sort().is_err());
}

#[test]
fn test_interface_implementation_validation() {
    let mut checker = GenericInterfaceChecker::new();
    
    // Create a simple interface
    let interface = GenericInterface {
        name: "TestInterface".to_string(),
        type_parameters: vec![],
        associated_types: vec![],
        methods: vec![
            InterfaceMethod {
                name: "test_method".to_string(),
                type_parameters: vec![],
                parameters: vec![],
                return_type: Some(Type::Basic("normie".to_string())),
                where_clauses: vec![],
                has_default: false,
            }
        ],
        superinterfaces: vec![],
        where_clauses: vec![],
        default_implementations: HashMap::new(),
    };
    
    assert!(checker.register_interface(interface).is_ok());
    
    // Create implementation
    let mut method_impls = HashMap::new();
    method_impls.insert("test_method".to_string(), FunctionStatement {
        name: "test_method".to_string(),
        type_parameters: vec![],
        parameters: vec![],
        return_type: Some(Type::Basic("normie".to_string())),
        body: vec![],
        where_clauses: vec![],
    });
    
    let implementation = InterfaceImplementation {
        interface_name: "TestInterface".to_string(),
        implementing_type: TypeExpression::named("normie"),
        type_bindings: HashMap::new(),
        associated_type_bindings: HashMap::new(),
        method_implementations: method_impls,
        where_clauses: vec![],
    };
    
    assert!(checker.register_implementation(implementation).is_ok());
}

#[test]
fn test_variance_annotations() {
    // Test variance checking
    let covariant = GenericTypeParameter {
        name: "T".to_string(),
        variance: Variance::Covariant,
        constraints: vec![],
        default_type: None,
    };
    
    let contravariant = GenericTypeParameter {
        name: "T".to_string(),
        variance: Variance::Contravariant,
        constraints: vec![],
        default_type: None,
    };
    
    let invariant = GenericTypeParameter {
        name: "T".to_string(),
        variance: Variance::Invariant,
        constraints: vec![],
        default_type: None,
    };
    
    assert_eq!(covariant.variance, Variance::Covariant);
    assert_eq!(contravariant.variance, Variance::Contravariant);
    assert_eq!(invariant.variance, Variance::Invariant);
}

#[test]
fn test_where_clauses() {
    let where_clause = WhereClause {
        type_expr: TypeExpression::named("T"),
        constraints: vec![
            AdvancedConstraint::TraitBound("T".to_string(), "Clone".to_string()),
            AdvancedConstraint::DebugConstraint("T".to_string()),
        ],
    };
    
    assert_eq!(where_clause.constraints.len(), 2);
    assert_eq!(where_clause.type_expr.name, Some("T".to_string()));
}

#[test]
fn test_associated_types() {
    let assoc_type = AssociatedType {
        name: "Item".to_string(),
        constraints: vec![AdvancedConstraint::DebugConstraint("Item".to_string())],
        default_type: Some(TypeExpression::named("normie")),
    };
    
    assert_eq!(assoc_type.name, "Item");
    assert_eq!(assoc_type.constraints.len(), 1);
    assert!(assoc_type.default_type.is_some());
}

#[test]
fn test_interface_hierarchy() {
    let checker = GenericInterfaceChecker::new();
    
    // Test inheritance checking (would need to register parent-child relationships)
    // For now, just test that the hierarchy system exists
    assert!(!checker.is_subinterface("Clone", "Iterator"));
    assert!(checker.get_superinterfaces("Clone").is_empty());
}

#[test]
fn test_kind_inference() {
    let mut hkt_system = HigherKindedTypeSystem::new();
    
    // Test basic type inference
    let int_type = TypeExpression::named("normie");
    let kind = hkt_system.infer_kind(&int_type);
    assert!(kind.is_ok());
    assert_eq!(kind.unwrap(), Kind::Type);
    
    // Test constructor kind inference
    let array_constructor = TypeExpression::named("Array");
    let array_kind = hkt_system.infer_kind(&array_constructor);
    assert!(array_kind.is_ok());
    assert!(matches!(array_kind.unwrap(), Kind::TypeConstructor(_, _)));
}

#[test]
fn test_constraint_satisfaction() {
    let mut checker = AdvancedConstraintChecker::new();
    
    // Test built-in trait implementations
    let int_type = TypeExpression::named("normie");
    assert!(checker.check_trait_bound(&int_type, "Clone").is_ok());
    assert!(checker.check_trait_bound(&int_type, "Copy").is_ok());
    assert!(checker.check_trait_bound(&int_type, "Debug").is_ok());
    
    // Test non-existent trait
    assert!(checker.check_trait_bound(&int_type, "NonExistent").is_err());
}

#[test]
fn test_type_equality() {
    let checker = AdvancedConstraintChecker::new();
    
    let int_type1 = TypeExpression::named("normie");
    let int_type2 = TypeExpression::named("normie");
    let string_type = TypeExpression::named("tea");
    
    assert!(checker.types_equal(&int_type1, &int_type2));
    assert!(!checker.types_equal(&int_type1, &string_type));
}

#[test]
fn test_generic_function_constraints() {
    let mut checker = GenericTypeChecker::new();
    let env = TypeEnvironment::new();
    
    // Create a generic function with constraints
    let func = FunctionStatement {
        name: "test_func".to_string(),
        type_parameters: vec![
            TypeParameter {
                name: "T".to_string(),
                bounds: vec![
                    GenericConstraint {
                        name: "Clone".to_string(),
                        type_params: vec![],
                    }
                ],
            }
        ],
        parameters: vec![
            Parameter {
                name: "value".to_string(),
                param_type: Some(Type::Generic("T".to_string(), vec![])),
                default_value: None,
            }
        ],
        return_type: Some(Type::Generic("T".to_string(), vec![])),
        body: vec![],
        where_clauses: vec![],
    };
    
    assert!(checker.check_generic_function(&func, &env).is_ok());
}

#[test]
fn test_utility_functions() {
    // Test interface utility functions
    let simple_interface = interface_utils::create_simple_interface("TestInterface", vec![]);
    assert_eq!(simple_interface.name, "TestInterface");
    assert_eq!(simple_interface.type_parameters.len(), 0);
    
    let type_param = interface_utils::create_type_parameter("T", vec![]);
    assert_eq!(type_param.name, "T");
    assert_eq!(type_param.variance, Variance::Invariant);
    
    // Test HKT utility functions
    let unary_constructor = hkt_utils::make_unary_constructor("List");
    assert_eq!(unary_constructor.name, "List");
    assert_eq!(unary_constructor.parameters.len(), 1);
    
    let binary_constructor = hkt_utils::make_binary_constructor("Map");
    assert_eq!(binary_constructor.name, "Map");
    assert_eq!(binary_constructor.parameters.len(), 2);
    
    let kind_str = hkt_utils::format_kind(&Kind::TypeConstructor(
        Box::new(Kind::Type),
        Box::new(Kind::Type)
    ));
    assert_eq!(kind_str, "* -> *");
}
