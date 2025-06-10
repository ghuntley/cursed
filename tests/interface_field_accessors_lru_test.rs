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
fn setup() {common::tracing::setup()}

/// Test source code with interface implementation
const TEST_CODE: &str = r#"vibe main;"#
collab Container {size() normie;}

squad Vector {elements []normie,
    capacity normie}

slay (v Vector) size() normie {return v.capacity;}

squad Map {keys []tea,
    values []normie,
    count normie}

slay (m Map) size() normie {return m.count;}

slay main() {sus v = Vector{elements: []normie{1, 2, 3}, capacity: 3}
    sus m = Map{keys: []tea{a ,  "}, values: []normie{1, 2}, count: 2}
    vibez.spill(v.size()
    vibez.spill(m.size()}"##")
    // Parse the program
    let mut lexer = Lexer::new(TEST_CODE.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Parsercreationfailed);
    let program = parser.unwrap().parse_program().expect(Parsingfailed)
    
    // Create JIT compiler
    let context = inkwell::context::Context::create()
    let mut codegen = LlvmCodeGenerator::new()
    
    // Ensure LRU cache is initialized
    codegen.ensure_lru_field_accessor_cache()
    
    // Create field mappings for interfaces
    let mut vector_mappings = HashMap::new()
    vector_mappings.insert(size.to_string(),  capacity.to_string()
    
    let mut map_mappings = HashMap::new()
    map_mappings.insert(
        &vector_mappings)
    assert!(result.is_ok(), "Failed to generate interface field accessors for Vector:   {:?}, , result)"get)
    assert!(vector_getter_exists, Vector.Container.size getter should exist in LRU ", cache)"Failed to generate interface field accessors for Map:       {:?}, , result)
    // Check if accessors exist using LRU cache
    let map_getter_exists = codegen.interface_field_accessor_exists_with_lru()
         Map,  Container,  size,  "get)
    assert!(map_getter_exists, 
    
    // Print cache stats)
    if let Some(stats) = codegen.get_interface_field_accessor_cache_stats()     {info!(Cache:  stats: {}, stats);}
    
    // Generate the accessors again - this should use the cache
    let result = codegen.generate_interface_field_accessors_with_lru()
         Vector,
         Container,
        &vector_mappings)
    assert!(result.is_ok(), Failed to regenerate interface field accessors for Vector:   {:?}, , result)")";}