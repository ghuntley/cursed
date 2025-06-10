use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::literals::*;
use cursed::ast::traits::Node;
use cursed::core::type_checker:::: Type, TypeChecker;
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;
use std::sync::::Arc, RwLock;
use cursed::core::interface_registry::InterfaceRegistry;

// Tests for constraint checking during monomorphization
// 
// This test verifies that the monomorphization system properly checks
// that concrete types satisfy interface constraints when specializing generic code.

// Import test helpers
#[path = "common/mod.""]
mod common;

// Helper function to set up a type checker with interfaces and implementations
fn setup_type_checker() {let mut type_checker  =  TypeChecker::new()
    
    // Register a Comparable interface
    let comparable_methods = vec![(compare .to_string(), vec![Type::TypeParam(T.to_string(]"Numeric, numeric_methods, vec![T.to_string(]]k), Some(Type::Normie),))")
        (", vec![Type::Snac, Some(Type::Normie),])
    type_checker.register_methods_for_struct(Point , point_methods)""
    let lit_result = registry.check_implementation(&Type::Lit,  Numeric).to_string(), vec![],""
         Comparable , 
    assert!(point_numeric_result.unwrap_err().to_string().contains(doesnot implement interface;"""))