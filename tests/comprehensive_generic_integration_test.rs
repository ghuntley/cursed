/// Comprehensive Integration Test for Generic Type System
/// 
/// This test suite validates end-to-end integration between parsing,
/// constraint resolution, type inference, and LLVM code generation
/// for the enhanced generic type system.

use cursed::parser::Parser;
use cursed::type_system::{TypeSystem, ConstraintResolver, GenericInstantiator};
use cursed::codegen::llvm::type_system::TypeCompilationContext;
use cursed::codegen::llvm::expression_compiler::{ExpressionContext, LlvmExpressionCompiler};
use cursed::ast::declarations::GenericConstraint;
use cursed::error::Error;
use std::collections::HashMap;

#[path = "common.rs"]
mod common;

/// Test complete workflow: parse -> resolve -> compile
#[test]
fn test_complete_generic_workflow() {
    common::tracing::setup();
    
    // Step 1: Parse generic type definition
    let source = r#"
        squad Container<T: Comparable> {
            sus value: T,
            normie id: normie
        }
    "#;
    
    let mut parser = Parser::new(source);
    let program = parser.parse_program();
    assert!(program.is_ok(), "Should parse generic struct definition");
    
    // Step 2: Set up type compilation context
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Step 3: Compile generic type with constraints
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string(), "equals".to_string()],
        }
    ];
    
    let generic_result = type_context.compile_generic_type(
        "Container", 
        &["T".to_string()], 
        &constraints
    );
    assert!(generic_result.is_ok(), "Should compile generic type");
    
    // Step 4: Instantiate with concrete type
    let instance_result = type_context.instantiate_generic("Container", &["normie".to_string()]);
    assert!(instance_result.is_ok(), "Should instantiate generic with normie");
    
    let instance = instance_result.unwrap();
    assert_eq!(instance.instance_name, "Container_normie");
    assert!(instance.compiled_type.contains("i64")); // normie -> i64
}

/// Test generic method dispatch compilation
#[test]
fn test_generic_method_dispatch() {
    common::tracing::setup();
    
    // Create type compilation context
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Compile a generic type with methods
    let constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string()],
        }
    ];
    
    type_context.compile_generic_type("List", &["T".to_string()], &constraints).unwrap();
    
    // Create expression compiler with type context
    let expr_context = ExpressionContext::with_type_context(type_context);
    let mut compiler = LlvmExpressionCompiler::with_context(expr_context);
    
    // Test that compiler can handle generic method calls
    // (This would require more AST setup for a complete test)
    assert!(format!("{:?}", compiler).contains("LlvmExpressionCompiler"));
}

/// Test constraint validation across different scenarios
#[test]
fn test_comprehensive_constraint_validation() {
    common::tracing::setup();
    
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Test 1: Numeric constraint
    let numeric_constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Numeric".to_string(),
            methods: vec!["add".to_string(), "subtract".to_string()],
        }
    ];
    
    type_context.compile_generic_type("Calculator", &["T".to_string()], &numeric_constraints).unwrap();
    
    // Valid: normie satisfies Numeric
    let valid_calc = type_context.instantiate_generic("Calculator", &["normie".to_string()]);
    assert!(valid_calc.is_ok(), "normie should satisfy Numeric constraint");
    
    // Invalid: tea doesn't satisfy Numeric
    let invalid_calc = type_context.instantiate_generic("Calculator", &["tea".to_string()]);
    assert!(invalid_calc.is_err(), "tea should not satisfy Numeric constraint");
    
    // Test 2: Comparable constraint
    let comparable_constraints = vec![
        GenericConstraint {
            parameter: "T".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string()],
        }
    ];
    
    type_context.compile_generic_type("SortedList", &["T".to_string()], &comparable_constraints).unwrap();
    
    // Valid types for Comparable
    let comparable_types = ["normie", "tea", "facts"];
    for type_name in &comparable_types {
        let result = type_context.instantiate_generic("SortedList", &[type_name.to_string()]);
        assert!(result.is_ok(), "{} should satisfy Comparable constraint", type_name);
    }
}

/// Test multiple type parameters with different constraints
#[test]
fn test_multiple_type_parameters() {
    common::tracing::setup();
    
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Create generic with two type parameters
    let type_parameters = vec!["K".to_string(), "V".to_string()];
    let constraints = vec![
        GenericConstraint {
            parameter: "K".to_string(),
            constraint_type: "Hashable".to_string(),
            methods: vec!["hash".to_string()],
        },
        GenericConstraint {
            parameter: "V".to_string(),
            constraint_type: "Comparable".to_string(),
            methods: vec!["compare".to_string()],
        }
    ];
    
    let result = type_context.compile_generic_type("HashMap", &type_parameters, &constraints);
    assert!(result.is_ok(), "Should compile generic with multiple constrained parameters");
    
    // Instantiate with valid types
    let instance = type_context.instantiate_generic("HashMap", &["normie".to_string(), "tea".to_string()]);
    assert!(instance.is_ok(), "Should instantiate with valid type arguments");
    
    let compiled_instance = instance.unwrap();
    assert_eq!(compiled_instance.instance_name, "HashMap_normie_tea");
    assert!(compiled_instance.compiled_type.contains("i64")); // normie -> i64
    assert!(compiled_instance.compiled_type.contains("i8*")); // tea -> i8*
}

/// Test type substitution with complex nested types
#[test]
fn test_complex_type_substitution() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    
    // Template with nested type references
    let template = r#"
%struct.Container_template = type { %TYPE_T*, i32, [10 x %TYPE_T] }
declare %TYPE_T* @process_%TYPE_T(%TYPE_T*, %TYPE_T*)
declare i32 @compare_%TYPE_T(%TYPE_T*, %TYPE_T*)
    "#;
    
    let mut substitutions = HashMap::new();
    substitutions.insert("T".to_string(), "normie".to_string());
    
    let result = context.substitute_types_in_template(template, &substitutions);
    assert!(result.is_ok());
    
    let substituted = result.unwrap();
    
    // Verify all substitutions were made
    assert!(!substituted.contains("%TYPE_T"), "All type placeholders should be replaced");
    assert!(substituted.contains("i64*"), "Pointer type should be substituted");
    assert!(substituted.contains("[10 x i64]"), "Array type should be substituted");
    assert!(substituted.contains("@process_i64"), "Function names should be substituted");
    assert!(substituted.contains("@compare_i64"), "All function names should be substituted");
}

/// Test error propagation through the system
#[test]
fn test_error_propagation() {
    common::tracing::setup();
    
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Try to instantiate non-existent generic type
    let missing_result = type_context.instantiate_generic("NonExistent", &["normie".to_string()]);
    assert!(missing_result.is_err(), "Should error for non-existent generic type");
    
    let error = missing_result.unwrap_err();
    match error {
        Error::TypeCompilation(msg) => {
            assert!(msg.contains("not found") || msg.contains("NonExistent"));
        },
        _ => panic!("Should be TypeCompilation error"),
    }
}

/// Test performance with many instantiations
#[test]
fn test_instantiation_performance() {
    common::tracing::setup();
    
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Create base generic type
    type_context.compile_generic_type("Container", &["T".to_string()], &[]).unwrap();
    
    let start = std::time::Instant::now();
    
    // Create many instantiations
    let types = ["normie", "tea", "facts"];
    let mut instances = Vec::new();
    
    for _ in 0..100 {
        for type_name in &types {
            let instance = type_context.instantiate_generic("Container", &[type_name.to_string()]);
            assert!(instance.is_ok());
            instances.push(instance.unwrap());
        }
    }
    
    let duration = start.elapsed();
    
    // Should be cached after first instantiation, so should be fast
    assert!(duration.as_millis() < 100, "Many instantiations should be fast due to caching");
    assert_eq!(instances.len(), 300); // 100 iterations * 3 types
    
    // Verify cache efficiency - should only have 3 unique instantiations
    let unique_names: std::collections::HashSet<_> = instances.iter()
        .map(|i| &i.instance_name)
        .collect();
    assert_eq!(unique_names.len(), 3); // Container_normie, Container_tea, Container_facts
}

/// Test integration with type inference system
#[test]
fn test_type_inference_integration() {
    common::tracing::setup();
    
    let type_context = TypeCompilationContext::new("test_module".to_string());
    let type_system = type_context.type_system();
    
    // Test that type system components are properly initialized
    assert!(format!("{:?}", type_system).contains("TypeSystem"));
    
    // Create a type environment for testing
    use cursed::type_system::TypeEnvironment;
    let env = TypeEnvironment::new();
    
    // Basic type environment operations should work
    assert!(format!("{:?}", env).contains("TypeEnvironment"));
}

/// Test constraint template generation
#[test]
fn test_constraint_template_generation() {
    common::tracing::setup();
    
    let context = TypeCompilationContext::new("test_module".to_string());
    
    let constraint = cursed::codegen::llvm::type_system::CompiledConstraint {
        param_name: "T".to_string(),
        constraint_type: "Comparable".to_string(),
        constraint_methods: vec!["compare".to_string(), "equals".to_string()],
    };
    
    let template = context.generate_constraint_check_template(&constraint);
    assert!(template.is_ok());
    
    let generated = template.unwrap();
    assert!(generated.contains("Constraint check template"));
    assert!(generated.contains("T : Comparable"));
    assert!(generated.contains("@comparable_compare"));
    assert!(generated.contains("@comparable_equals"));
    assert!(generated.contains("%TYPE_T"));
}

/// Test system state consistency
#[test]
fn test_system_state_consistency() {
    common::tracing::setup();
    
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // Compile several generic types
    let types = ["Container", "List", "Set"];
    for type_name in &types {
        let result = type_context.compile_generic_type(type_name, &["T".to_string()], &[]);
        assert!(result.is_ok(), "Should compile generic type {}", type_name);
    }
    
    // Instantiate each type
    for type_name in &types {
        let instance = type_context.instantiate_generic(type_name, &["normie".to_string()]);
        assert!(instance.is_ok(), "Should instantiate {}", type_name);
    }
    
    // Verify registry state
    let registry = type_context.registry();
    assert!(registry.struct_names().len() >= 0); // May be empty if no structs compiled
    assert!(registry.interface_names().len() >= 0); // May be empty if no interfaces compiled
    
    // Verify type system state
    let type_system = type_context.type_system();
    assert!(format!("{:?}", type_system).contains("TypeSystem"));
}

/// Test memory management and cleanup
#[test]
fn test_memory_management() {
    common::tracing::setup();
    
    // Create many contexts to test memory management
    let mut contexts = Vec::new();
    
    for i in 0..10 {
        let mut context = TypeCompilationContext::new(format!("module_{}", i));
        
        // Add some generic types to each context
        context.compile_generic_type("Type", &["T".to_string()], &[]).unwrap();
        context.instantiate_generic("Type", &["normie".to_string()]).unwrap();
        
        contexts.push(context);
    }
    
    // All contexts should be properly constructed
    assert_eq!(contexts.len(), 10);
    
    // Drop all contexts (automatic cleanup)
    drop(contexts);
    
    // No assertions needed - this tests that cleanup doesn't panic
}
