//! Tests for the deep nested interface constraints implementation

use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use cursed::core::nested_interface_registry::{NestedInterfaceRegistry, NestedConstraint, EnhancedInterfaceRegistry};
use cursed::core::deep_nested_interface_registry::{DeepNestedInterfaceRegistry, ConstraintPath, DeepNestedInterfaceChecking};
use cursed::core::type_checker::Type;
use cursed::error::Error;

mod common;

#[test]
fn test_deep_constraint_basic_registration() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new();
    
    // Register a deep nested constraint
    registry.register_deep_nested_constraint(
        "Container",
        "T",
        "Stack",
        "E",
        "Comparable"
    );
    
    // Verify it was registered correctly
    let constraints = registry.enhanced_registry.get_nested_constraints("Container");
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].outer_type, "Container");
    assert_eq!(constraints[0].outer_param, "T");
    assert_eq!(constraints[0].inner_type, "Stack");
    assert_eq!(constraints[0].inner_params[0], "E");
    assert_eq!(constraints[0].interface, "Comparable");
}

#[test]
fn test_multi_level_constraint_registration() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new();
    
    // Register a multi-level constraint where Collection[Container[List[E]]] requires E to be Comparable
    registry.register_deep_multi_level_constraint(
        "Collection",
        "T",
        vec!["Container", "List"],
        vec!["U", "E"],
        "Comparable"
    );
    
    // Verify first level constraint (Collection -> Container)
    let constraints = registry.enhanced_registry.get_nested_constraints("Collection");
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].outer_type, "Collection");
    assert_eq!(constraints[0].outer_param, "T");
    assert_eq!(constraints[0].inner_type, "Container");
    assert_eq!(constraints[0].inner_params[0], "U");
    assert_eq!(constraints[0].interface, "DependentConstraint");
    
    // Verify second level constraint (Container -> List)
    let constraints = registry.enhanced_registry.get_nested_constraints("Container");
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].outer_type, "Container");
    assert_eq!(constraints[0].outer_param, "U");
    assert_eq!(constraints[0].inner_type, "List");
    assert_eq!(constraints[0].inner_params[0], "E");
    assert_eq!(constraints[0].interface, "Comparable");
}

#[test]
fn test_constraint_path_construction() {
    common::tracing::setup();
    
    let mut path = ConstraintPath::new();
    
    // Add segments to the path
    path.add_segment("Collection", "T", "Container");
    path.add_segment("Container", "U", "List");
    path.add_segment("List", "E", "Comparable");
    
    // Verify path properties
    assert_eq!(path.depth(), 3);
    assert_eq!(path.type_path, vec!["Collection", "Container", "List"]);
    assert_eq!(path.param_path, vec!["T", "U", "E"]);
    assert_eq!(path.interface_path, vec!["Container", "List", "Comparable"]);
    
    // Verify string representation
    let path_str = path.to_string();
    assert!(path_str.contains("Collection<T>"));
    assert!(path_str.contains("Container<U>"));
    assert!(path_str.contains("List<E>"));
}

#[test]
fn test_simple_deep_constraint_checking() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Register a deep nested constraint for Container[Stack[E]] where E must be Comparable
    registry.register_deep_nested_constraint(
        "Container",
        "T",
        "Stack",
        "E",
        "Comparable"
    );
    
    // Create a Stack[Int] type - Int implements Comparable
    let stack_int = Type::Struct(
        "Stack".to_string(),
        vec![Type::Normie]
    );
    
    // Create a Stack[NonComparable] type - NonComparable doesn't implement Comparable
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    let stack_non_comparable = Type::Struct(
        "Stack".to_string(),
        vec![non_comparable]
    );
    
    // Check constraints
    let mut path = ConstraintPath::new();
    let result = registry.check_deep_nested_implementation(
        "Container",
        "T",
        &stack_int,
        "Comparable",
        &mut path
    );
    assert!(result.unwrap());
    
    let mut path = ConstraintPath::new();
    let result = registry.check_deep_nested_implementation(
        "Container",
        "T",
        &stack_non_comparable,
        "Comparable",
        &mut path
    );
    assert!(!result.unwrap());
    
    // Verify path tracking worked
    assert_eq!(path.depth(), 1);
    assert_eq!(path.type_path[0], "Container");
    assert_eq!(path.param_path[0], "T");
    assert_eq!(path.interface_path[0], "Comparable");
}

#[test]
fn test_multi_level_deep_constraint_checking() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Register a multi-level constraint: Collection[Container[List[E]]] requires E to be Comparable
    registry.register_deep_multi_level_constraint(
        "Collection",
        "T",
        vec!["Container", "List"],
        vec!["U", "E"],
        "Comparable"
    );
    
    // Create test types
    let list_int = Type::Struct(
        "List".to_string(),
        vec![Type::Normie] // Int implements Comparable
    );
    
    let container_list_int = Type::Struct(
        "Container".to_string(),
        vec![list_int.clone()]
    );
    
    // Create test types with non-comparable element
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    let list_non_comparable = Type::Struct(
        "List".to_string(),
        vec![non_comparable.clone()]
    );
    
    let container_list_non_comparable = Type::Struct(
        "Container".to_string(),
        vec![list_non_comparable.clone()]
    );
    
    // Check constraints
    let result = registry.check_complex_nested_constraint(
        "Collection",
        "T",
        &container_list_int,
        "Comparable"
    );
    assert!(result.unwrap());
    
    let result = registry.check_complex_nested_constraint(
        "Collection",
        "T",
        &container_list_non_comparable,
        "Comparable"
    );
    assert!(!result.unwrap());
}

#[test]
fn test_error_creation_with_constraint_path() {
    common::tracing::setup();
    
    let registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Create a path
    let mut path = ConstraintPath::new();
    path.add_segment("Collection", "T", "Container");
    path.add_segment("Container", "U", "List");
    path.add_segment("List", "E", "Comparable");
    
    // Create a type that doesn't implement Comparable
    let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
    
    // Create an error with the path
    let error = registry.create_deep_constraint_error(
        &non_comparable,
        "Comparable",
        &path
    );
    
    // Verify the error contains the path information
    let message = error.message();
    assert!(message.contains("Deep nested constraint failure"));
    assert!(message.contains("Collection<T>"));
    assert!(message.contains("Container<U>"));
    assert!(message.contains("List<E>"));
    assert!(message.contains("Comparable"));
    assert!(message.contains("NonComparable"));
    
    // Verify error code
    assert_eq!(error.code(), "CNST03");
    
    // Verify recommendations are included
    assert!(message.contains("For nested container types, ensure that"));
    assert!(message.contains("innermost type"));
}

#[test]
fn test_extension_trait_for_base_registry() {
    common::tracing::setup();
    
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    
    // Use the extension trait to check a complex constraint
    let list_int = Type::Struct(
        "List".to_string(),
        vec![Type::Normie] // Int implements Comparable
    );
    
    // Should return true even though the base registry doesn't know about nested constraints
    // because the extension trait converts it to a deep registry first
    let result = registry.check_complex_nested_constraint(
        "Container",
        "T",
        &list_int,
        "Comparable"
    );
    
    assert!(result.unwrap());
    
    // Convert to deep registry directly
    let deep_registry = registry.to_deep_nested_registry();
    assert!(deep_registry.enhanced_registry.base_registry.check_implementation(&Type::Normie, "Comparable").unwrap());
}

#[test]
fn test_deep_nested_registry_defaults() {
    common::tracing::setup();
    
    let registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Verify that base registry defaults were populated
    assert!(registry.enhanced_registry.base_registry.check_implementation(&Type::Normie, "Comparable").unwrap());
    assert!(registry.enhanced_registry.base_registry.check_implementation(&Type::Tea, "Comparable").unwrap());
    
    // Verify deep nested defaults were populated
    let constraints = registry.enhanced_registry.get_nested_constraints("NestedMap");
    assert!(!constraints.is_empty());
    
    let constraints = registry.enhanced_registry.get_nested_constraints("Triple");
    assert!(!constraints.is_empty());
    
    let constraints = registry.enhanced_registry.get_nested_constraints("MultiContainer");
    assert!(!constraints.is_empty());
}

#[test]
fn test_constraint_cache() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Register a multi-level constraint
    registry.register_deep_multi_level_constraint(
        "Collection",
        "T",
        vec!["Container", "List"],
        vec!["U", "E"],
        "Comparable"
    );
    
    // Create test types
    let list_int = Type::Struct(
        "List".to_string(),
        vec![Type::Normie] // Int implements Comparable
    );
    
    let container_list_int = Type::Struct(
        "Container".to_string(),
        vec![list_int.clone()]
    );
    
    // First check should compute the result
    let result1 = registry.check_complex_nested_constraint(
        "Collection",
        "T",
        &container_list_int,
        "Comparable"
    );
    assert!(result1.unwrap());
    
    // Second check should use the cached result
    let result2 = registry.check_complex_nested_constraint(
        "Collection",
        "T",
        &container_list_int,
        "Comparable"
    );
    assert!(result2.unwrap());
    
    // Hard to test directly that the cache was used, but we can verify correct result
}

#[test]
fn test_diamond_constraint_pattern() {
    common::tracing::setup();
    
    let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
    
    // Register constraints for a diamond pattern:
    // A[B[D]] requires D: Comparable
    // A[C[D]] requires D: Numeric
    registry.register_deep_multi_level_constraint(
        "A",
        "X",
        vec!["B", "D"],
        vec!["Y", "Z"],
        "Comparable"
    );
    
    registry.register_deep_multi_level_constraint(
        "A",
        "X",
        vec!["C", "D"],
        vec!["Y", "Z"],
        "Numeric"
    );
    
    // Create test types
    // D[Int] - Int implements both Comparable and Numeric
    let d_int = Type::Struct(
        "D".to_string(),
        vec![Type::Normie]
    );
    
    // B[D[Int]]
    let b_d_int = Type::Struct(
        "B".to_string(),
        vec![d_int.clone()]
    );
    
    // C[D[Int]]
    let c_d_int = Type::Struct(
        "C".to_string(),
        vec![d_int.clone()]
    );
    
    // Check constraints
    let result = registry.check_complex_nested_constraint(
        "A",
        "X",
        &b_d_int,
        "Comparable"
    );
    assert!(result.unwrap());
    
    let result = registry.check_complex_nested_constraint(
        "A",
        "X",
        &c_d_int,
        "Numeric"
    );
    assert!(result.unwrap());
    
    // Now create a type that implements Comparable but not Numeric
    let only_comparable = Type::Struct("OnlyComparable".to_string(), vec![]);
    registry.enhanced_registry.base_registry.register_implementation(
        only_comparable.clone(),
        "Comparable".to_string()
    );
    
    let d_only_comparable = Type::Struct(
        "D".to_string(),
        vec![only_comparable.clone()]
    );
    
    let b_d_only_comparable = Type::Struct(
        "B".to_string(),
        vec![d_only_comparable.clone()]
    );
    
    let c_d_only_comparable = Type::Struct(
        "C".to_string(),
        vec![d_only_comparable.clone()]
    );
    
    // Should pass for the B path (Comparable constraint)
    let result = registry.check_complex_nested_constraint(
        "A",
        "X",
        &b_d_only_comparable,
        "Comparable"
    );
    assert!(result.unwrap());
    
    // Should fail for the C path (Numeric constraint)
    let result = registry.check_complex_nested_constraint(
        "A",
        "X",
        &c_d_only_comparable,
        "Numeric"
    );
    assert!(!result.unwrap());
}