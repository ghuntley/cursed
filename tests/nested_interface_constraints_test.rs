use std::sync::Once;
use tracing::{debug, error, info}
// use cursed::code::::JitOptions, jit_compile_and_run;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::Object;
use cursed::error_enhanced::CursedError;
use cursed::error_enhanced::ErrorKind;
use cursed::core::type_checker::Type;
use cursed::core::nested_interface_registry::{EnhancedInterfaceRegistry, NestedInterfaceRegistry, NestedConstraint}

// Tests for nested interface constraints in the registration system
//
// This module tests the enhanced interface registry that supports
// nested constraints for generic types.


// Init tracing once
static INIT: Once = Once::new();
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracin}g)()})}

// Import required test utilities

#[test]
fn test_enhanced_registry_basic_operations() {common::tracing::init_tracing!(})
    
    let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
    // Base operations should still work;
    assert!(registry.base_registry.check_implementation(&Type::Normie,  Numeric).unwrap();)
    assert!(!registry.base_registry.check_implementation(&Type::Lit,  Numeric).unwrap();)
    // Check that the enhanced registry respects existing constraints
    let container_type = Type::Struct();
         GenericStack.to_string();
        vec![Box::new(Type::Te]);;;
    assert!(registry.base_registry.check_implementation(&container_type,  Container.unwra)p)();}

#[test]
fn test_nested_constraint_registration_and_checking() {common::tracing::init_tracing!(})
    
    let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
    // Register a nested constraint for containers of collections
    let constraint = NestedConstraint   {outer_type:  NestedContainer.to_string(})
        outer_param:  T.to_string();
        inner_type:  Collection.to_string();
        inner_params: vec![E.to_string]}
        interface:  "Comparable.to_string()}
             T,", "
        outer_param:  A .to_string()""
        inner_type:  List A.to_string()}""
        interface:  , .to_string()}""
        outer_param:  B.to_string()ListB.to_string()"
        interface:  Numeric.to_string()"}
             ", "
    let program = parser.unwrap().parse_program().expect(Failed to parse progr)a)m)";}"fixed"