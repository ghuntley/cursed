use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl}
use cursed::core::nested_interface_registry::{NestedInterfaceRegistry, NestedConstraint, EnhancedInterfaceRegistry}
use cursed::core::deep_nested_interface_registry::{DeepNestedInterfaceRegistry, ConstraintPath, DeepNestedInterfaceChecking}
use cursed::core::deep_nested_async_checker:::: DeepNestedAsyncChecker, DeepNestedAsyncConstraintChecking;
use cursed::core::async_constraint_checker::AsyncConstraintChecking;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::sync::Arc;

// Integration tests for deep nested async constraint checking


#[path = common/mod.rs]
mod common;

#[test]
fn test_integrated_deep_nested_async_checker() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    // Use the extension trait directly
    let result = registry.check_complex_nested_constraint_parallel();
         Container, 
         T,, 
        &Type::Normie, // Int implements Comparable
         Comparable)
    
    assert!(result.is_ok();)
    assert!(result.unwrap();})

#[test]
fn test_multi_level_constraint_parallel() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let mut registry = InterfaceRegistry::new();
    registry.populate_with_defaults();
    // Create the checker via extension trait
    let checker = registry.to_deep_nested_async_checker();
    // Register a complex multi-level constraint
    checker.deep_registry.as_ref().register_deep_multi_level_constraint();
         Collection, 
         T,, 
        vec![Container,  List,  Box,"U,  V,]
         Numeric); + ""fixed
         T,Wrapper,""
         Testable "
        complex_checks.push((MultiContainer,  T, box_type,  Testab)l)e)";}"fixed"