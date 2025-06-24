use crate::error::Error;
/// ChaosMode - CURSED Runtime System Package
/// 
/// Provides comprehensive access to the CURSED runtime system for goroutine management,
/// debugging, memory control, and system information with a chaotic twist focused on
/// performance and observability.

pub mod core;
pub mod memory;
pub mod goroutines;
pub mod profiling;
pub mod runtime_info;
pub mod enhanced;
pub mod error;

pub use error::{ChaosError, ChaosResult};

// Re-export all core functionality
pub use core::{
    // Basic runtime functions
    num_cpu, num_goroutine, yield_processor, gosched, gc, gomaxprocs,
    set_gc_percent, set_max_heap,
};

pub use memory::{
    // Memory management
    MemoryStats, mem_stats, read_mem_stats, set_gc_enabled, free_os_memory,
    set_mem_profile_rate,
    
    // Enhanced memory debugging
    allocation_size_histogram, top_allocated_types, TypeAllocationInfo,
    is_valid_pointer, get_object_size, get_pointer_info, PointerInfo,
};

pub use goroutines::{
    // Stack and goroutine management
    stack_trace, all_goroutine_ids, all_goroutine_stacks, callers,
    pc_to_file_and_line, pc_to_func_name, goroutine_stack,
    
    // Enhanced goroutine management
    GoroutineData, goroutine_info, set_goroutine_label, goroutines_by_label,
    goroutines_by_state, kill_goroutine,
};

pub use profiling::{
    // Profiling and tracing
    start_trace, stop_trace, read_trace, set_traceback_limit,
    set_cpu_profile_rate, start_cpu_profile, stop_cpu_profile,
};

pub use runtime_info::{
    // Runtime information
    version, goarch, goos, compiler, runtime_stats, goroot,
};

pub use enhanced::{
    // Enhanced garbage collection
    GCMode, set_gc_mode, get_gc_mode, start_gc, wait_for_gc,
    register_gc_notification,
    
    // Performance tuning
    SchedulerMode, set_max_threads, num_threads, set_cpu_frequency,
    set_thread_priority, set_scheduler_mode, get_scheduler_mode,
};

// Module initialization
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the ChaosMode runtime system
pub fn initialize() -> ChaosResult<()> {
    INIT.call_once(|| {
        // Initialize subsystems
        if let Err(e) = core::initialize() {
            eprintln!("Failed to initialize ChaosMode core: {:?}", e);
        }
        if let Err(e) = memory::initialize() {
            eprintln!("Failed to initialize ChaosMode memory: {:?}", e);
        }
        if let Err(e) = goroutines::initialize() {
            eprintln!("Failed to initialize ChaosMode goroutines: {:?}", e);
        }
        if let Err(e) = profiling::initialize() {
            eprintln!("Failed to initialize ChaosMode profiling: {:?}", e);
        }
        if let Err(e) = enhanced::initialize() {
            eprintln!("Failed to initialize ChaosMode enhanced: {:?}", e);
        }
    });
    Ok(())
}

/// Cleanup the ChaosMode runtime system
pub fn cleanup() -> ChaosResult<()> {
    // Cleanup all subsystems
    enhanced::cleanup()?;
    profiling::cleanup()?;
    goroutines::cleanup()?;
    memory::cleanup()?;
    core::cleanup()?;
    Ok(())
}

/// Get comprehensive runtime statistics
pub fn chaos_stats() -> ChaosResult<serde_json::Value> {
    use serde_json::json;
    
    Ok(json!({
        "goroutines": {
            "count": num_goroutine()?,
            "max_procs": gomaxprocs(0)?,
        },
        "memory": {
            "stats": mem_stats()?,
            "gc_percent": crate::stdlib::vibecheck::get_gc_percent(),
        },
        "system": {
            "num_cpu": num_cpu()?,
            "version": version()?,
            "goos": goos()?,
            "goarch": goarch()?,
        },
        "enhanced": {
            "gc_mode": get_gc_mode() as i32,
            "scheduler_mode": get_scheduler_mode() as i32,
            "num_threads": num_threads()?,
        }
    }))
}
