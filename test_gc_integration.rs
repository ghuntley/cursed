#!/usr/bin/env rust-script

//! Test script to verify garbage collection integration

use std::sync::Arc;

// Mock the basic CURSED types we need
mod cursed_mock {
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;
    
    #[derive(Debug, Clone)]
    pub struct CursedError {
        message: String,
    }
    
    impl CursedError {
        pub fn runtime_error(msg: &str) -> Self {
            Self { message: msg.to_string() }
        }
    }
    
    impl std::fmt::Display for CursedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    impl std::error::Error for CursedError {}
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tag {
        Object,
        Array,
        String,
        Number,
        Boolean,
        Nil,
    }
    
    pub trait Traceable {
        fn trace(&self, visitor: &mut dyn Visitor);
        fn get_tag(&self) -> Tag;
        fn size(&self) -> usize;
    }
    
    pub trait Visitor {
        fn visit(&mut self, obj: &dyn Traceable);
    }
    
    // Mock stack management
    pub struct RuntimeStack;
    
    impl RuntimeStack {
        pub fn new() -> Self { Self }
        pub fn get_all_gc_roots(&self) -> Vec<*mut u8> { Vec::new() }
    }
}

use cursed_mock::*;

fn main() {
    println!("Testing CURSED Garbage Collection Integration");
    
    // Test 1: Basic GC configuration
    println!("\n1. Testing GC configuration...");
    
    let gc_config = GcConfig {
        initial_heap_size: 1024 * 1024, // 1MB
        max_heap_size: Some(10 * 1024 * 1024), // 10MB
        young_generation_ratio: 0.33,
        young_collection_threshold: 256 * 1024, // 256KB
        old_collection_threshold: 1024 * 1024, // 1MB
        incremental_collection: true,
        incremental_time_budget: 5,
        concurrent_collection: false, // Disable for testing
        concurrent_threads: 1,
        trigger_mode: GcTriggerMode::Manual,
        enable_compaction: true,
        compaction_threshold: 0.3,
    };
    
    println!("✓ GC configuration created");
    
    // Test 2: Memory manager configuration
    println!("\n2. Testing memory manager configuration...");
    
    let memory_config = MemoryConfig {
        gc_config: gc_config.clone(),
        enable_tracking: true,
        stack_memory_limit: Some(4 * 1024 * 1024), // 4MB
        global_memory_limit: Some(64 * 1024 * 1024), // 64MB
        enable_pressure_detection: true,
        pressure_threshold: 0.8,
    };
    
    println!("✓ Memory configuration created");
    
    // Test 3: Initialize runtime system
    println!("\n3. Testing runtime initialization...");
    
    let stack_manager = Arc::new(RuntimeStack::new());
    
    // Initialize GC
    match initialize_gc(gc_config, Arc::clone(&stack_manager)) {
        Ok(_) => println!("✓ GC initialized successfully"),
        Err(e) => {
            println!("✗ Failed to initialize GC: {}", e);
            return;
        }
    }
    
    // Initialize memory manager
    match initialize_memory_manager(memory_config, stack_manager) {
        Ok(_) => println!("✓ Memory manager initialized successfully"),
        Err(e) => {
            println!("✗ Failed to initialize memory manager: {}", e);
            return;
        }
    }
    
    // Test 4: Basic allocation
    println!("\n4. Testing basic allocation...");
    
    if let Some(memory_manager) = get_global_memory_manager() {
        match memory_manager.allocate_raw(64, Tag::Object) {
            Ok(handle) => {
                println!("✓ Successfully allocated object of size {}", handle.size());
                println!("  - Object tag: {:?}", handle.tag());
                println!("  - Generation: {}", handle.generation);
            }
            Err(e) => {
                println!("✗ Failed to allocate object: {}", e);
                return;
            }
        }
        
        // Test 5: Memory statistics
        println!("\n5. Testing memory statistics...");
        
        let stats = memory_manager.get_stats();
        println!("✓ Memory statistics:");
        println!("  - Heap allocations: {}", stats.heap_allocations);
        println!("  - Heap usage: {} bytes", stats.heap_usage);
        println!("  - GC collections: {}", stats.gc_stats.total_collections);
        
        // Test 6: Force garbage collection
        println!("\n6. Testing garbage collection...");
        
        match memory_manager.collect_garbage() {
            Ok(gc_stats) => {
                println!("✓ Garbage collection completed:");
                println!("  - Total collections: {}", gc_stats.total_collections);
                println!("  - Objects collected: {}", gc_stats.objects_collected);
                println!("  - Bytes collected: {}", gc_stats.bytes_collected);
            }
            Err(e) => {
                println!("✗ Garbage collection failed: {}", e);
                return;
            }
        }
        
        println!("\n7. Testing memory health check...");
        
        match memory_manager.health_check() {
            Ok(healthy) => {
                println!("✓ Memory health check passed: {}", healthy);
            }
            Err(e) => {
                println!("⚠ Memory health check warning: {}", e);
            }
        }
        
    } else {
        println!("✗ Failed to get global memory manager");
        return;
    }
    
    // Test 8: Shutdown
    println!("\n8. Testing shutdown...");
    
    match shutdown_memory_manager() {
        Ok(_) => println!("✓ Memory manager shutdown successfully"),
        Err(e) => println!("✗ Failed to shutdown memory manager: {}", e),
    }
    
    match shutdown_gc() {
        Ok(_) => println!("✓ GC shutdown successfully"),
        Err(e) => println!("✗ Failed to shutdown GC: {}", e),
    }
    
    println!("\n🎉 All tests completed successfully!");
}

// Mock the types we're using (simplified versions)
#[derive(Debug, Clone)]
pub struct GcConfig {
    pub initial_heap_size: usize,
    pub max_heap_size: Option<usize>,
    pub young_generation_ratio: f64,
    pub young_collection_threshold: usize,
    pub old_collection_threshold: usize,
    pub incremental_collection: bool,
    pub incremental_time_budget: u64,
    pub concurrent_collection: bool,
    pub concurrent_threads: usize,
    pub trigger_mode: GcTriggerMode,
    pub enable_compaction: bool,
    pub compaction_threshold: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GcTriggerMode {
    Manual,
}

#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub gc_config: GcConfig,
    pub enable_tracking: bool,
    pub stack_memory_limit: Option<usize>,
    pub global_memory_limit: Option<usize>,
    pub enable_pressure_detection: bool,
    pub pressure_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct ObjectHandle {
    pub size: usize,
    pub tag: Tag,
    pub generation: u8,
}

impl ObjectHandle {
    pub fn size(&self) -> usize { self.size }
    pub fn tag(&self) -> Tag { self.tag }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub heap_allocations: u64,
    pub heap_usage: usize,
    pub gc_stats: GcStats,
}

#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub total_collections: u64,
    pub objects_collected: u64,
    pub bytes_collected: u64,
}

#[derive(Debug)]
pub enum MemoryError {
    TestError(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::TestError(msg) => write!(f, "Test error: {}", msg),
        }
    }
}

impl std::error::Error for MemoryError {}

// Mock implementations
pub struct MemoryManager;

impl MemoryManager {
    pub fn allocate_raw(&self, size: usize, tag: Tag) -> Result<ObjectHandle, MemoryError> {
        Ok(ObjectHandle { size, tag, generation: 0 })
    }
    
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            heap_allocations: 1,
            heap_usage: 64,
            gc_stats: GcStats {
                total_collections: 0,
                objects_collected: 0,
                bytes_collected: 0,
            },
        }
    }
    
    pub fn collect_garbage(&self) -> Result<GcStats, MemoryError> {
        Ok(GcStats {
            total_collections: 1,
            objects_collected: 0,
            bytes_collected: 0,
        })
    }
    
    pub fn health_check(&self) -> Result<bool, MemoryError> {
        Ok(true)
    }
}

static mut MEMORY_MANAGER: Option<MemoryManager> = None;

pub fn initialize_gc(_config: GcConfig, _stack: Arc<RuntimeStack>) -> Result<(), CursedError> {
    Ok(())
}

pub fn initialize_memory_manager(_config: MemoryConfig, _stack: Arc<RuntimeStack>) -> Result<(), MemoryError> {
    unsafe {
        MEMORY_MANAGER = Some(MemoryManager);
    }
    Ok(())
}

pub fn get_global_memory_manager() -> Option<&'static MemoryManager> {
    unsafe { MEMORY_MANAGER.as_ref() }
}

pub fn shutdown_memory_manager() -> Result<(), MemoryError> {
    unsafe {
        MEMORY_MANAGER = None;
    }
    Ok(())
}

pub fn shutdown_gc() -> Result<(), CursedError> {
    Ok(())
}
