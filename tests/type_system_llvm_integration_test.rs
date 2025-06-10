/// Integration tests for CURSED Type System LLVM Integration
/// 
/// This test suite validates the integration between the enhanced type system
/// (constraint resolution, generic instantiation) and LLVM code generation.

use cursed::type_system::  ::TypeSystem, TypeEnvironment, ConstraintResolver, GenericInstantiator;
use cursed::codegen::llvm::type_system::::TypeCompilationContext, CompiledGenericType, CompiledConstraint;
use cursed::codegen::llvm::expression_compiler::ExpressionContext;
use cursed::ast::declarations::{GenericConstraint, SquadStatement, FieldStatement;
use cursed::ast::identifiers::Identifier;
use cursed::error::Error;
use std::collections::HashMap;

#[path = common.rs]
mod common;

/// Test basic generic struct compilation with constraints
#[test]
fn test_generic_struct_compilation() {common::tracing::setup();
    
    let mut context = TypeCompilationContext::new(test_module.to_string();
    
    // Create a generic struct: Container<T: Comparable>
    let type_parameters = vec![T.to_string()]
fn test_generic_instantiation() {common::tracing::setup();
    
    let mut context = TypeCompilationContext::new(test_module.to_string();
    
    // First, compile the generic type
    let type_parameters = vec![T.to_string()];
    
    context.compile_generic_type("List"List", &type_args);
    
    assert!(instance.is_ok(), ");
    let compiled_instance = instance.unwrap();
    assert_eq!(compiled_instance.base_name, "List"normie"]);
    assert_eq!(compiled_instance.instance_name, ");
    assert!(compiled_instance.compiled_type.contains("i64"T".to_string(),
            constraint_type: ".to_string(),
            methods: vec!["add"subtract".to_string()]);
    assert!(invalid_result.is_err(), "Numeric constraint should reject tea type"Printable".to_string(),
            methods: vec![".to_string()], &interface_constraints).unwrap();
    // Test with types that implement Printable
    let valid_types = [normie, ", "facts"Display", &[type_name.to_string()]);
        assert!(result.is_ok(), ", type_name);}
/// Test multiple constraint handling
#[test]
fn test_multiple_constraints() {common::tracing::setup();
    
    let mut context = TypeCompilationContext::new(test_module.to_string();
    
    // Create generic with multiple constraints
    let type_parameters = vec![T.to_string()];
    
    let result = context.compile_generic_type("SortedSet"Multiple constraints should compile successfully");
    let compiled = result.unwrap();
    assert_eq!(compiled.constraints.len(), 2);
    
    // Instantiate with type that satisfies both constraints
    let instance_result = context.instantiate_generic(SortedSet, &[".to_string()]);
    assert!(instance_result.is_ok(), "normie should satisfy both Comparable and Hashable
%struct.Container_template = type {%TYPE_T, i32}
declare %TYPE_T* @operation_%TYPE_T(%TYPE_T*)
    "#;
    let mut substitutions = HashMap::new();
    substitutions.insert(".to_string(), "normie"i64"); // normie -> i64
    assert!(!substituted.contains(%TYPE_T); // All placeholders replaced
    assert!(substituted.contains(@operation_i64);}

/// Test error handling for invalid constraints
#[test]
fn test_invalid_constraint_handling() {common::tracing::setup();
    
    let mut context = TypeCompilationContext::new(test_module.to_string();
    
    // Try to use unknown constraint type
    let constraints = vec![GenericConstraint {parameter: T.to_string(),
            constraint_type: ".to_string(),
            methods: vec!["unknown_method"BadType", &[".to_string()],};
    let compiled = context.compile_constraint(&constraint);
    assert!(compiled.is_ok();
    
    let compiled_constraint = compiled.unwrap();
    assert_eq!(compiled_constraint.param_name, "T"Comparable");
    assert_eq!(compiled_constraint.constraint_methods.len(), 2);
    assert!(compiled_constraint.constraint_methods.contains(&".to_string();}
/// Test generic template generation
#[test]
fn test_generic_template_generation() {common::tracing::setup();
    
    let context = TypeCompilationContext::new(test_module.to_string();
    
    let type_parameters = vec!["T"U".to_string()];
    
    let template = context.generate_generic_template(", &type_parameters, &constraints);
    assert!(template.is_ok();
    
    let generated = template.unwrap();
    assert!(generated.contains("Pair_template"%TYPE_T");
    assert!(generated.contains(");
    assert!(generated.contains("Constraint check template"T".to_string()], &[]).unwrap();
    // Instantiate with many different types
    let types = [normie, ", "facts"Container", &[type_name.to_string()]);
        assert!(instance.is_ok();
        instances.push(instance.unwrap();}
    
    // Verify all instances are unique and cached
    assert_eq!(instances.len(), 3);
    for (i, instance) in instances.iter().enumerate()   {assert_eq!(instance.base_name, Container);
        assert_eq!(instance.concrete_types, vec![types[i];}
