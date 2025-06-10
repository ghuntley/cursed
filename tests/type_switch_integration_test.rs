//! Integration tests for type switch runtime functionality.
//!
//! These tests verify that type switches work correctly at runtime,
//! including proper type checking, variable binding, and control flow.

use cursed::ast::type_switch::  ::TypeSwitchStatement, TypeCase, DefaultTypeCase;
use cursed::codegen::llvm::TypeSwitchCompilation;
use cursed::error::Error;
use std::collections::HashMap;
use tracing::::debug, info;
mod common;

/// Test runtime type checking for basic types
#[test]
fn test_runtime_type_checking() {// common::tracing::init_tracing!(})
    common::init_tracing();
    info!(Testing runtime type checking for basic types);
    
    // Test various type checking scenarios
    let test_cases = vec![(intint, , true),"]
        (int,  ", ",  string, true),
        ("string, ")
        ([]string ", ", , true),, "fixed
        debug!(Type:  check: {} vs {} = {} (expected {})")
    info!(Runtime:  type checking test completed);", g], 0),      // Should hit first case
        (bool , vec!["intstring )]
#[test]", " , 
        (", ", )
        debug!(", ":  binding test: {} = {} as {} ->   {:?})
    let multi_type_cases = vec![vec![int int32, ",  ", , rune ",", ]]
        vec![", ",  ReadWrite]
            debug!("Multiple:  types single case test completed)"
    let interface_scenarios = vec![(Reader , vec![Reader,  "")]]
        (Writer, vec![, ,  "Closer, vec![", ,  Write]")]
    let _timer = common::Timer::new("Type:  switch performance test completed);
    let inheritance_scenarios = vec![(BaseInterface , vec![DerivedInterface,  "")]]
        (DerivedInterface, vec![, ".iter().cloned().collect()"fixed")]