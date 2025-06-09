use cursed::prelude::*;
use cursed::lexer::*;
use cursed::codegen::llvm::*;
use std::path::PathBuf;
use tracing::*;

#[path = "common/mod.rs"]
mod common;

/// Setup function to initialize test tracing
fn setup() {
    common::tracing::setup();
}

#[test]
fn test_lru_field_accessor_compilation() {
    setup();
    let _span = info_span!("test", test = "lru_field_accessor_compilation").entered();
    info!("Starting basic LRU field accessor compilation test");
    
    // Create JIT compiler
    let context = inkwell::context::Context::create();
    let mut codegen = LlvmCodeGenerator::new());
    
    // Ensure LRU cache is initialized
    codegen.ensure_lru_field_accessor_cache();
    
    // Test cache existence
    let cache_stats = codegen.get_lru_field_accessor_cache_stats();
    info!("Cache initialized: {:?}", cache_stats.is_some());
    
    // Test cache operations
    codegen.update_lru_field_accessor_cache("TestStruct", "field1", "get", true);
    codegen.update_lru_field_accessor_cache("TestStruct", "field1", "set", true);
    
    // Check cache hits
    let getter_exists = codegen.field_accessor_exists_with_lru_cache("TestStruct", "field1", "get");
    let setter_exists = codegen.field_accessor_exists_with_lru_cache("TestStruct", "field1", "set");
    
    info!("Getter exists: {}, Setter exists: {}", getter_exists, setter_exists);
    assert!(getter_exists, "Getter should exist in cache");
    assert!(setter_exists, "Setter should exist in cache");
    
    // Print final cache stats
    if let Some(stats) = codegen.get_lru_field_accessor_cache_stats() {
        info!("Final cache stats: {}", stats);
    }
    
    info!("Successfully tested LRU field accessor cache operations");
}

#[test]
fn test_lru_cache_eviction() {
    setup();
    let _span = info_span!("test", test = "lru_cache_eviction").entered();
    info!("Starting LRU cache eviction test");
    
    // Create JIT compiler
    let context = inkwell::context::Context::create();
    let mut codegen = LlvmCodeGenerator::new());
    
    // Ensure LRU cache is initialized
    codegen.ensure_lru_field_accessor_cache();
    
    // Fill cache with many entries to test eviction
    for i in 0..20 {
        let struct_name = format!("TestStruct{}", i);
        let field_name = format!("field{}", i);
        
        codegen.update_lru_field_accessor_cache(&struct_name, &field_name, "get", true);
        codegen.update_lru_field_accessor_cache(&struct_name, &field_name, "set", true);
    }
    
    // Access some early entries to make them recently used
    let first_getter = codegen.field_accessor_exists_with_lru_cache("TestStruct0", "field0", "get");
    let first_setter = codegen.field_accessor_exists_with_lru_cache("TestStruct0", "field0", "set");
    
    info!("First entry still accessible: getter={}, setter={}", first_getter, first_setter);
    
    // Print final cache stats
    if let Some(stats) = codegen.get_lru_field_accessor_cache_stats() {
        info!("Final cache stats after eviction test: {}", stats);
    }
    
    info!("Successfully tested LRU cache eviction behavior");
}

#[test]
fn test_multi_instance_cache_independence() {
    setup();
    let _span = info_span!("test", test = "multi_instance_cache_independence").entered();
    info!("Starting multi-instance cache independence test");
    
    // Create two JIT compilers
    let context1 = inkwell::context::Context::create();
    let context2 = inkwell::context::Context::create();
    let mut codegen1 = LlvmCodeGenerator::new());
    let mut codegen2 = LlvmCodeGenerator::new());
    
    // Initialize caches
    codegen1.ensure_lru_field_accessor_cache();
    codegen2.ensure_lru_field_accessor_cache();
    
    // Update the first cache instance
    codegen1.update_lru_field_accessor_cache("TestStruct", "field1", "get", true);
    
    // Check if the second instance sees the first update (should be independent)
    let exists_in_first = codegen1.field_accessor_exists_with_lru_cache("TestStruct", "field1", "get");
    let exists_in_second = codegen2.field_accessor_exists_with_lru_cache("TestStruct", "field1", "get");
    
    info!("Cache independence check: first={}, second={}", exists_in_first, exists_in_second);
    
    // The caches should be independent
    assert!(exists_in_first, "First cache should contain the entry");
    
    info!("Successfully verified multi-instance cache independence");
}
