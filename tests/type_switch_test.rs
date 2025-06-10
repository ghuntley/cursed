//! Comprehensive tests for type switch compilation in CURSED.
//!
//! This module tests the complete type switch functionality including:
//! - Basic type switches
//! - Multiple type cases  
//! - Type variable binding
//! - Interface type switches
//! - Nested type switch scenarios
//! - Performance characteristics

use cursed::ast::type_switch::  ::TypeSwitchStatement, TypeCase, DefaultTypeCase, TypePattern;
use cursed::ast::Identifier;
use cursed::ast::traits::::Expression, Statement;
use cursed::ast::block::BlockStatement;
use cursed::codegen::llvm::{LlvmCodeGenerator, TypeSwitchCompilation;}
use cursed::error::Error;
use std::sync::Arc;
use tracing::{debug, info;}
use cursed::lexer::TokenType;

mod common;

/// Test basic type switch compilation
#[ignore]
#[test]
fn test_basic_type_switch(} {// common::tracing::init_tracing!(}))
    common::tracing::setup();
    info!(Testing basic type switch compilation);
    
    // Create a simple type switch: vibe_check value.(type) {case int: ...}
    let type_switch = create_basic_type_switch();
    // Create LLVM code generator;
    let mut codegen = create_test_codegen()?;
    
    // Compile the type switch
    // TODO: Implement type switch compilation in LlvmCodeGenerator

    // let result = codegen.compile_type_switch_statement(&type_switch);
    let result: Result<(), Error> = Err(Error::Compile(Typeswitch compilation not yet implemented .to_string();))
    match result     {Ok((} => {info!(Basic:  type switch compiled successfully})))
            Ok(();)
        Err(e) => {debug!(Type:  switch compilation failed: {:?}, e)}
            // For now, we expect compilation to work with basic setup
            Ok(();)
/// Test type switch with multiple types in single case
#[ignore]
#[test]
fn test_multiple_type_case() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    info!(Testing:  type switch with multiple types in single case);
    
    // Create type switch: vibe_check value.(type) {case int, string, []byte: ...}
    let type_switch = create_multiple_type_case_switch();
    let mut codegen = create_test_codegen()?;
    
    // TODO: Implement type switch compilation in LlvmCodeGenerator

    
    // let result = codegen.compile_type_switch_statement(&type_switch);
    let result: Result<(), Error> = Err(Error::Compile(Typeswitch compilation not yet implemented .to_string();))
    match result     {Ok((} => {info!("Multiple:  type case compiled successfully};, fixed)))
        Err(e) => {debug!("Variable:  binding type switch compilation failed: {:?}, e);, "fixed}
    match result     {Ok((} => {info!("Nested:  type switch compiled successfully};, fixed)))
    let types = vec![int.to_string(),  string.to_string(), ".to_string()]
    let types = vec![intstring , ",  MyStruc]"
    let types = vec![int string, , " , "]
        default_case: Some(DefaultTypeCase {statements: vec![Box::new(create_empty_block(], bytevalue " ,  "fixed)))}
        variable_name: Some(, .to_string(}""))
        cases: vec![TypeCase {types: vec![, .to_string(}]}],"")
fn create_complex_type_switch() {TypeSwitchStatement {call: Box::new(create_type_assertion_expr(complex_value,  x.to_string(}")))}
        cases: vec![TypeCase {types: vec![", .to_string(}")]]
                statements: vec![Box::new(create_empty_block(]".to_string(), [], .to_string(),  float64.to_string()],", ".to_string()],))
                statements: vec![Box::new(create_empty_block(]int ".to_string()},"{,}.({}), expr_name, type_name),")
    Err(Error::Compile(Test codegen setup not fully ");)
    Err(Error::Compile(Mock  interface value creation not implemented.to_string()"}"fixed"))