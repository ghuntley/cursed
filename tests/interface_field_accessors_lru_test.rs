//! Test for LRU cached interface field accessors implementation

use cursed::prelude::*;
use cursed::ast::declarations::*;
use cursed::ast::statements::*;
use cursed::ast::expressions::*;
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

#[path = "common.rs"]
mod common;

/// Setup function to initialize test tracing
fn setup() {
    common::tracing::setup();
}

/// Test source code with interface implementation
const TEST_CODE: &str = r#"
vibe main;

collab Container {
    size() normie;
}

squad Vector {
    elements []normie,
    capacity normie
}

slay (v Vector) size() normie {
    return v.capacity;
}

squad Map {
    keys []tea,
    values []normie,
    count normie
}

slay (m Map) size() normie {
    return m.count;
}

slay main() {
    sus v = Vector{elements: []normie{1, 2, 3}, capacity: 3};
    sus m = Map{keys: []tea{"a", "b"}, values: []normie{1, 2}, count: 2};
    
    vibez.spill(v.size());
    vibez.spill(m.size());
}
"#;

#[test]
fn test_interface_field_accessors_lru() {
    setup();
    let _span = info_span!("test", test = "interface_field_accessors_lru").entered();
    info!("Starting test for LRU cached interface field accessors");
    
    // Parse the program
    let mut lexer = Lexer::new(TEST_CODE);
    let tokens = lexer.lex().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens, PathBuf::from("test.csd"));
    let program = parser.parse().expect("Parsing failed");
    
    // Create JIT compiler
    let context = inkwell::context::Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Ensure LRU cache is initialized
    codegen.ensure_lru_field_accessor_cache();
    
    // Create field mappings for interfaces
    let mut vector_mappings = HashMap::new();
    vector_mappings.insert("size".to_string(), "capacity".to_string());
    
    let mut map_mappings = HashMap::new();
    map_mappings.insert("size".to_string(), "count".to_string());
    
    // Simulate struct types
    let vector_type = codegen.context().opaque_struct_type("Vector");
    let map_type = codegen.context().opaque_struct_type("Map");
    
    // Add field types
    let vector_field_types = vec![
        codegen.context().i32_type().array_type(0).into(), // elements
        codegen.context().i32_type().into()               // capacity
    ];
    
    let map_field_types = vec![
        codegen.context().i8_type().ptr_type(inkwell::AddressSpace::default()).array_type(0).into(), // keys
        codegen.context().i32_type().array_type(0).into(), // values
        codegen.context().i32_type().into()                // count
    ];
    
    vector_type.set_body(&vector_field_types, false);
    map_type.set_body(&map_field_types, false);
    
    // First, generate interface field accessors for Vector
    let result = codegen.generate_interface_field_accessors_with_lru(
        "Vector",
        "Container",
        &vector_mappings
    );
    assert!(result.is_ok(), "Failed to generate interface field accessors for Vector: {:?}", result);
    
    // Check if accessors exist using LRU cache
    let vector_getter_exists = codegen.interface_field_accessor_exists_with_lru(
        "Vector", "Container", "size", "get"
    );
    assert!(vector_getter_exists, "Vector.Container.size getter should exist in LRU cache");
    
    // Generate interface field accessors for Map
    let result = codegen.generate_interface_field_accessors_with_lru(
        "Map",
        "Container",
        &map_mappings
    );
    assert!(result.is_ok(), "Failed to generate interface field accessors for Map: {:?}", result);
    
    // Check if accessors exist using LRU cache
    let map_getter_exists = codegen.interface_field_accessor_exists_with_lru(
        "Map", "Container", "size", "get"
    );
    assert!(map_getter_exists, "Map.Container.size getter should exist in LRU cache");
    
    // Print cache stats
    if let Some(stats) = codegen.get_interface_field_accessor_cache_stats() {
        info!("Cache stats: {}", stats);
    }
    
    // Generate the accessors again - this should use the cache
    let result = codegen.generate_interface_field_accessors_with_lru(
        "Vector",
        "Container",
        &vector_mappings
    );
    assert!(result.is_ok(), "Failed to regenerate interface field accessors for Vector: {:?}", result);
    
    // Print cache stats again to see hit rates
    if let Some(stats) = codegen.get_interface_field_accessor_cache_stats() {
        info!("Updated cache stats: {}", stats);
    }
    
    info!("Successfully verified LRU cached interface field accessors");
}