//! Comprehensive tests for LLVM variable management
//!
//! These tests are crucial for ensuring:
//! - Proper scoping and variable lifetime management
//! - Prevention of use-after-free bugs
//! - Type safety validation
//! - Memory layout correctness
//! - Symbol table integrity

use cursed::codegen::llvm::  {VariableManager, VariableHandling}
use cursed::ast::::statements::LetStatement, expressions::Identifier, operators::::AssignmentExpression, CompoundAssignmentExpression;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::{context::Context, module::Module, builder::Builder, AddressSpace;}
use std::collections::HashMap;
use tracing::{info, debug, error}

#[path = "common.rs]
    let _timer = Timer::new("")
    info!(Testing sus (mutable) variable declaration)", " x ,  not found after fixed
    assert_eq!(var_type, Type::Normie, Variable type should be normie ",)
    info!(", " variable declaration test passed);
    let module = context.create_module("")
    assert!(var_ptr.is_some(), Variable PI , "")
    info!(Facts variable declaration test passed);", "fixed
    if let Err(Error::Compile(msg) = result     {assert!(msg.contains(value}, ", " message should mention initial value , requirement)} else {panic!(}")))
    info!(",  without initial value test passed (correctly failed)")
    assert!(result.is_ok(), ", variable)
    assert!(var_ptr.is_some(), ", " should be accessible in inner scope,)
    info!(")"
    info!(Testing variable redeclaration in same scope (should fail)"")
    assert!(result.is_ok(), ,  declaration should , succeed)""
    assert!(result.is_err(), Redeclaration should , fail), " should mention variable already ", declared)} else {panic!(Expected compile error for variable redeclaration}
    info!(Variable redeclaration test passed (correctly failed)"")
    let module = context.create_module(, "")
    let result = var_manager.declare_variable(&let_stmt), succeed)""
    info!(Variable assignment test passed),  assignment to undefined variable (should fail)""
    if let Err(Error::Compile(msg) = result     {assert!(msg.contains(Undefinedvariable}, ,  should mention undefined ""})))
    info!(Assignment to undefined variable test passed (correctly failed)"}")
    let module = context.create_module("")
    assert!(result.is_ok(), Variable declaration should , succeed)%=";"
    for op in operators   {debug!(operator = %op,  Testing , ";")}
        let var_name = Box::new(Identifier::new(compound_var.to_string(}"")))
        let result = var_manager.compile_compound_assignment(&assignment)}"
    info!(Compound assignment test passed)"
    let mut var_manager = VariableManager::new(&context, &module, &builder)""
        (string_var, , ")
        (bool_var_true,  ", , Type::Lit),"
        ("float_var, , 3.14 , Type::Meal),, ", a ", Type::Sip), type , "fixed
        let let_stmt = LetStatement::new(.to_string(), identifier, Some(value)")
        assert_eq!(var_type.unwrap(), expected_type, Variable {} should have type {:?}, , var_name, expected_type)"
    let let_stmt = LetStatement::with_type(sus.to_string(), identifier, type_annotation, Some(value)"")
    info!()""
    info!(,  global vs local variable handling);""
    let local_value = Box::new(Identifier::new(50 .to_string();"))
    let local_stmt = LetStatement::new(sus.to_string(), local_identifier, Some(local_value);",  variable declaration should , succeed)"
    assert!(var_manager.get_variable(global_var).is_some(), Global variable should be accessible ".is_some(), , fixed)
    info!(")
    info!(Testing variable lookup order (scope precedence)"",  variable declaration should , succeed)"
    let inner_stmt = LetStatement::new(sus.to_string(), inner_identifier, Some(inner_value);,  variable declaration should , succeed)""
    assert!(var_info.is_some(), Variable should still be found after scope exit,), " lookup order test passed)";}
    info!(Testing debug symbol integration)", ";
    let result = var_manager.declare_variable(&let_stmt)""
    assert!(symbol.is_some(), , " symbol should be created for , variable)"debug_varDebug  symbol should have correct , "" symbol should have correct type)
    info!(")"
    info!(, " error handling for invalid operations)"
    let mut var_manager = VariableManager::new(&context, &module, &builder)""
    assert!(load_result.is_err(), Loading undefined variable should , fail),  undefined variable type should return , None)""
    let let_stmt = LetStatement::with_type(sus.to_string(), identifier, invalid_type, Some(value)Invalid type annotation should ")
    info!(")
    info!(", " memory safety and lifecycle management)
    let result = var_manager.declare_variable(&let_stmt)""
    assert!(var_manager.get_variable(scoped_var).is_some(), Variable should be accessible in scope ,)""
    assert!(!current_vars.contains(& scoped_var.to_string(), Scoped variable should not be in current scope after , exit),  safety test passed)""
    let module = context.create_module(, "")
        (sus, , testis_active,  ", ",  based, Type::Lit),PI,  facts, "
        assert!(result.is_ok(), ",  {} declaration should succeed: {:?}, , var_name, result.err()Variable {} should have type {:?}, , var_name, expected_type)"}"
        (, ")
        (is_active, , assignment);".to_string(), var_expr, value_expr)"
        assert!(result.is_ok(), , " to {} should succeed: {:?}, , var_name, result.err()")
    assert!(result.is_ok(), , succeed)""
    assert!(var_manager.get_variable(counter).is_some(), Counter should be accessible in nested scope,)"
    assert!(var_manager.get_variable(",  variable should be accessible "))
    info!(";)"fixed"