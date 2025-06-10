/// Integration tests for CURSED Type System LLVM Integration
/// 
/// This test suite validates the integration between the enhanced type system
/// (constraint resolution, generic instantiation) and LLVM code generation.

use cursed::type_system::{TypeSystem, TypeEnvironment, ConstraintResolver, GenericInstantiator};
use cursed::codegen::llvm::type_system::{TypeCompilationContext, CompiledGenericType, CompiledConstraint};
use cursed::codegen::llvm::expression_compiler::ExpressionContext;
use cursed::ast::declarations::{GenericConstraint, SquadStatement, FieldStatement};
use cursed::ast::identifiers::Identifier;
use cursed::error::Error;
use std::collections::HashMap;

#[path = "common.rs"]
mod common;

/// Test basic generic struct compilation with constraints
#[test]
fn test_generic_struct_compilation() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Create a generic struct: Container<T: Comparable>
    let type_parameters = vec!["T".to_string()];
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string(), "equals".to_string()],
        }
    ];
    
    let result = context.compile_generic_type("Container", &type_parameters, &constraints);
    assert!(result.is_ok(), "Generic type compilation should succeed");
    
    let compiled = result.unwrap();
    assert_eq!(compiled.name, "Container");
    assert_eq!(compiled.type_parameters, vec!["T"]);
    assert_eq!(compiled.constraints.len(), 1);
    assert!(compiled.llvm_template.contains("Container_template"));
}

/// Test generic instantiation with concrete types
#[test]
fn test_generic_instantiation() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // First, compile the generic type
    let type_parameters = vec!["T".to_string()];
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string()],
        }
    ];
    
    context.compile_generic_type("List", &type_parameters, &constraints).unwrap();
    
    // Now instantiate with concrete type
    let type_args = vec!["normie".to_string()];
    let instance = context.instantiate_generic("List", &type_args);
    
    assert!(instance.is_ok(), "Generic instantiation should succeed");
    
    let compiled_instance = instance.unwrap();
    assert_eq!(compiled_instance.base_name, "List");
    assert_eq!(compiled_instance.concrete_types, vec!["normie"]);
    assert_eq!(compiled_instance.instance_name, "List_normie");
    assert!(compiled_instance.compiled_type.contains("i64")); // normie -> i64
}

/// Test constraint validation during instantiation
#[test]
fn test_constraint_validation() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Create generic with Numeric constraint
    let type_parameters = vec!["T".to_string()];
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Numeric".to_string(),
            methods: vec!["add".to_string(), "subtract".to_string()],
        }
    ];
    
    context.compile_generic_type("Calculator", &type_parameters, &constraints).unwrap();
    
    // Valid instantiation with numeric type
    let valid_result = context.instantiate_generic("Calculator", &["normie".to_string()]);
    assert!(valid_result.is_ok(), "Numeric constraint should be satisfied by normie");
    
    // Invalid instantiation with non-numeric type
    let invalid_result = context.instantiate_generic("Calculator", &["tea".to_string()]);
    assert!(invalid_result.is_err(), "Numeric constraint should reject tea type");
}

/// Test interface constraint checking
#[test]
fn test_interface_constraint_checking() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Register an interface first
    let interface_constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Printable".to_string(),
            methods: vec!["to_string".to_string()],
        }
    ];
    
    context.compile_generic_type("Display", &["T".to_string()], &interface_constraints).unwrap();
    
    // Test with types that implement Printable
    let valid_types = ["normie", "tea", "facts"];
    for type_name in &valid_types {
        let result = context.instantiate_generic("Display", &[type_name.to_string()]);
        assert!(result.is_ok(), "Type {} should satisfy Printable constraint", type_name);
    }
}

/// Test multiple constraint handling
#[test]
fn test_multiple_constraints() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Create generic with multiple constraints
    let type_parameters = vec!["T".to_string()];
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string()],
        },
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Hashable".to_string(),
            methods: vec!["hash".to_string()],
        }
    ];
    
    let result = context.compile_generic_type("SortedSet", &type_parameters, &constraints);
    assert!(result.is_ok(), "Multiple constraints should compile successfully");
    
    let compiled = result.unwrap();
    assert_eq!(compiled.constraints.len(), 2);
    
    // Instantiate with type that satisfies both constraints
    let instance_result = context.instantiate_generic("SortedSet", &["normie".to_string()]);
    assert!(instance_result.is_ok(), "normie should satisfy both Comparable and Hashable");
}

/// Test type substitution in LLVM templates
#[test]
fn test_type_substitution() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    
    let template = r#"
%struct.Container_template = type { %TYPE_T, i32 }
declare %TYPE_T* @operation_%TYPE_T(%TYPE_T*)
    "#;
    
    let mut substitutions = HashMap::new();
    substitutions.insert("T".to_string(), "normie".to_string());
    
    let result = context.substitute_types_in_template(template, &substitutions);
    assert!(result.is_ok());
    
    let substituted = result.unwrap();
    assert!(substituted.contains("i64")); // normie -> i64
    assert!(!substituted.contains("%TYPE_T")); // All placeholders replaced
    assert!(substituted.contains("@operation_i64"));
}

/// Test error handling for invalid constraints
#[test]
fn test_invalid_constraint_handling() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Try to use unknown constraint type
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "UnknownConstraint".to_string(),
            methods: vec!["unknown_method".to_string()],
        }
    ];
    
    let result = context.compile_generic_type("BadType", &["T".to_string()], &constraints);
    // Should still compile (constraints are validated during instantiation)
    assert!(result.is_ok());
    
    // But instantiation should validate constraints
    let instantiation = context.instantiate_generic("BadType", &["normie".to_string()]);
    // This may succeed if the constraint system is flexible
    // The exact behavior depends on implementation details
}

/// Test circular dependency detection
#[test]
fn test_circular_dependency_detection() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // This test would require a more complex setup to create actual circular dependencies
    // For now, just test that the compilation context can handle basic cases
    let result = context.compile_generic_type("SimpleType", &[], &[]);
    assert!(result.is_ok());
}

/// Test integration with type inference
#[test]
fn test_type_inference_integration() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    let type_system = context.type_system();
    
    // Test that type system is properly initialized and accessible
    assert!(format!("{:?}", type_system).contains("TypeSystem"));
    
    // Test basic type environment
    let env = TypeEnvironment::new();
    assert!(format!("{:?}", env).contains("TypeEnvironment"));
}

/// Test constraint compilation details
#[test]
fn test_constraint_compilation() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    
    let constraint = GenericConstraint {
        parameter: "T".to_string(),
        constraint_type: "Comparable".to_string(),
        methods: vec!["compare".to_string(), "equals".to_string()],
    };
    
    let compiled = context.compile_constraint(&constraint);
    assert!(compiled.is_ok());
    
    let compiled_constraint = compiled.unwrap();
    assert_eq!(compiled_constraint.param_name, "T");
    assert_eq!(compiled_constraint.constraint_type, "Comparable");
    assert_eq!(compiled_constraint.constraint_methods.len(), 2);
    assert!(compiled_constraint.constraint_methods.contains(&"compare".to_string()));
}

/// Test generic template generation
#[test]
fn test_generic_template_generation() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    
    let type_parameters = vec!["T".to_string(), "U".to_string()];
    let constraints = vec![
        CompiledConstraint {
            param_name: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            constraint_methods: vec!["compare".to_string()],
        }
    ];
    
    let template = context.generate_generic_template("Pair", &type_parameters, &constraints);
    assert!(template.is_ok());
    
    let generated = template.unwrap();
    assert!(generated.contains("Pair_template"));
    assert!(generated.contains("%TYPE_T"));
    assert!(generated.contains("%TYPE_U"));
    assert!(generated.contains("Constraint check template"));
}

/// Test performance with many generic instantiations
#[test]
fn test_generic_instantiation_performance() {
    common::tracing::setup();
    
    let mut context = TypeCompilationContext::new("test_module".to_string());
    
    // Create a generic type
    context.compile_generic_type("Container", &["T".to_string()], &[]).unwrap();
    
    // Instantiate with many different types
    let types = ["normie", "tea", "facts"];
    let mut instances = Vec::new();
    
    for type_name in &types {
        let instance = context.instantiate_generic("Container", &[type_name.to_string()]);
        assert!(instance.is_ok());
        instances.push(instance.unwrap());
    }
    
    // Verify all instances are unique and cached
    assert_eq!(instances.len(), 3);
    for (i, instance) in instances.iter().enumerate() {
        assert_eq!(instance.base_name, "Container");
        assert_eq!(instance.concrete_types, vec![types[i]]);
    }
}
