use cursed::prelude::*;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::operators::*;
use cursed::ast::types::*;
use cursed::ast::traits::*;
use cursed::lexer::*;
use cursed::parser::*;
use cursed::core::type_checker::*;
use cursed::codegen::llvm::*;
use cursed::codegen::llvm::lru_field_accessors::*;
use cursed::codegen::llvm::interface_field_accessors_lru::*;
use cursed::memory::gc::GarbageCollector;
use std::path::PathBuf;
use std::collections::HashMap;
use tracing::*;

use cursed::lexer::Lexer;
// Test for LRU cached interface field accessors implementation


#[path = common/mod.rs]
mod common;

/// Setup function to initialize test tracing
fn setup() {common::tracing::setup(}})

/// Test source code with interface implementation
const TEST_CODE: &str = r#"vibe main;
    sus m = Map{keys: []tea{a ,  "}, values: []normie{1, 2}, count: 2}"
    vibez.spill(m.size()}##"")
    assert!(result.is_ok(), ,  to generate interface field accessors for Vector:   {:?}, , result)""
    assert!(vector_getter_exists, Vector.Container.size getter should exist in LRU , cache), "fixed
         Map,  Container,  size,  get)"
    assert!(result.is_ok(), Failed to regenerate interface field accessors for Vector:   {:?}, , result)""fixed"