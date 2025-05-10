//! Tests for the constraint recovery strategies

use cursed::core::constraint_recovery::{ConstraintRecovery, RecoveryConfig, RecoveryStrategy, RecoveryResult};
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::type_checker::Type;
use std::collections::HashMap;

// Import common test utilities
#[path = "common.rs"]
mod common;

#[test]
fn test_basic_recovery_fail_strategy() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure to use the Fail strategy (default)
    let config = RecoveryConfig::default();
    registry.set_recovery_config(config);
    
    // Test recovery with a type that doesn't implement the interface
    let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
    let interface_name = "Comparable";
    
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        interface_name,
        None,
    );
    
    // Should be a Failed result with appropriate error information
    match result {
        RecoveryResult::Failed(error) => {
            assert!(error.message.contains("does not implement interface"));
            assert!(error.message.contains("TestStruct"));
            assert!(error.message.contains("Comparable"));
            
            // Check for error code
            assert!(error.code.starts_with("CNST"));
        },
        _ => panic!("Expected Failed result"),
    }
}

#[test]
fn test_find_alternative_strategy() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure to use the FindAlternative strategy
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::FindAlternative;
    registry.set_recovery_config(config);
    
    // Test the FindAlternative strategy with Comparable
    let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Comparable",
        None,
    );
    
    // Should be an AlternativeType result
    match result {
        RecoveryResult::AlternativeType(alt_type) => {
            // Should suggest one of the standard types that implement Comparable
            match alt_type {
                Type::Normie | Type::Thicc | Type::Snack | 
                Type::Meal | Type::Tea | Type::Lit => {
                    // These are expected
                },
                _ => panic!("Unexpected alternative type: {:?}", alt_type),
            }
        },
        _ => panic!("Expected AlternativeType result"),
    }
    
    // Test with Numeric interface
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Numeric",
        None,
    );
    
    // Should be an AlternativeType result with a numeric type
    match result {
        RecoveryResult::AlternativeType(alt_type) => {
            // Should suggest one of the standard numeric types
            match alt_type {
                Type::Normie | Type::Thicc | Type::Snack | Type::Meal => {
                    // These are expected
                },
                _ => panic!("Unexpected numeric alternative type: {:?}", alt_type),
            }
        },
        _ => panic!("Expected AlternativeType result for Numeric"),
    }
}

#[test]
fn test_use_placeholder_strategy() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure to use the UsePlaceholder strategy
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::UsePlaceholder;
    registry.set_recovery_config(config);
    
    // Test the UsePlaceholder strategy
    let concrete_type = Type::Struct("UserData".to_string(), vec![]);
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Hashable",
        None,
    );
    
    // Should be a Placeholder result
    match result {
        RecoveryResult::Placeholder(code) => {
            // Check the content of the placeholder implementation
            assert!(code.contains("AUTO-GENERATED PLACEHOLDER"));
            assert!(code.contains("UserData"));
            assert!(code.contains("Hashable"));
            assert!(code.contains("implementation Hashable for"));
            assert!(code.contains("placeholder implementation for testing only"));
        },
        _ => panic!("Expected Placeholder result"),
    }
}

#[test]
fn test_generate_stubs_strategy() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure to use the GenerateStubs strategy
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::GenerateStubs;
    registry.set_recovery_config(config);
    
    // Test the GenerateStubs strategy
    let concrete_type = Type::Struct("Config".to_string(), vec![]);
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Serializable",
        None,
    );
    
    // Should be a GeneratedStubs result
    match result {
        RecoveryResult::GeneratedStubs(code) => {
            // Check the content of the generated stubs
            assert!(code.contains("AUTO-GENERATED STUB"));
            assert!(code.contains("Config"));
            assert!(code.contains("Serializable"));
            assert!(code.contains("implementation Serializable for"));
        },
        _ => panic!("Expected GeneratedStubs result"),
    }
}

#[test]
fn test_interface_specific_strategies() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure with different strategies for different interfaces
    let mut interface_strategies = HashMap::new();
    interface_strategies.insert("Comparable".to_string(), RecoveryStrategy::FindAlternative);
    interface_strategies.insert("Serializable".to_string(), RecoveryStrategy::GenerateStubs);
    interface_strategies.insert("Hashable".to_string(), RecoveryStrategy::UsePlaceholder);
    
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::Fail; // Default is fail
    config.interface_strategies = interface_strategies;
    registry.set_recovery_config(config);
    
    // Test with a struct type
    let concrete_type = Type::Struct("User".to_string(), vec![]);
    
    // Test Comparable (should use FindAlternative)
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Comparable",
        None,
    );
    assert!(matches!(result, RecoveryResult::AlternativeType(_)));
    
    // Test Serializable (should use GenerateStubs)
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Serializable",
        None,
    );
    assert!(matches!(result, RecoveryResult::GeneratedStubs(_)));
    
    // Test Hashable (should use UsePlaceholder)
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Hashable",
        None,
    );
    assert!(matches!(result, RecoveryResult::Placeholder(_)));
    
    // Test an interface with no specific strategy (should use default Fail)
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Unknown",
        None,
    );
    assert!(matches!(result, RecoveryResult::Failed(_)));
}

#[test]
fn test_nested_constraint_recovery() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure to use the GenerateStubs strategy
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::GenerateStubs;
    registry.set_recovery_config(config);
    
    // Test nested constraint recovery
    let generic_type_name = "SortedList";
    let type_param_name = "T";
    let concrete_arg = Type::Struct("UserType".to_string(), vec![]);
    
    let result = registry.recover_from_nested_constraint_failure(
        generic_type_name,
        type_param_name,
        &concrete_arg,
        "Comparable",
    );
    
    // Should be a GeneratedStubs result with context about the generic type
    match result {
        RecoveryResult::GeneratedStubs(code) => {
            assert!(code.contains("AUTO-GENERATED STUB"));
            assert!(code.contains("UserType"));
            assert!(code.contains("Comparable"));
            assert!(code.contains("implementation Comparable for"));
        },
        _ => panic!("Expected GeneratedStubs result"),
    }
}

#[test]
fn test_recovery_disabled() {
    common::tracing::setup();
    
    let registry = InterfaceRegistry::new();
    
    // Configure with recovery disabled
    let mut config = RecoveryConfig::default();
    config.default_strategy = RecoveryStrategy::GenerateStubs; // Would generate stubs if enabled
    config.enabled = false; // But recovery is disabled
    registry.set_recovery_config(config);
    
    // Test with recovery disabled
    let concrete_type = Type::Struct("TestStruct".to_string(), vec![]);
    let result = registry.recover_from_constraint_failure(
        &concrete_type,
        "Comparable",
        None,
    );
    
    // Should be a Failed result even though the strategy is GenerateStubs
    assert!(matches!(result, RecoveryResult::Failed(_)));
}