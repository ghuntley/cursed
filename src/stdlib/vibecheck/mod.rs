/// CURSED Runtime Introspection Module (vibecheck)
/// 
/// Provides low-level runtime functionality and direct interaction with the 
/// Cursed runtime system including memory stats, GC control, goroutine management,
/// runtime configuration, and comprehensive profiling infrastructure.

pub mod mem_stats;
pub mod goroutine;
pub mod version;
pub mod gc;
pub mod memory_profiler;
pub mod cpu_profiler;
pub mod profile_data;
pub mod profiler;

// Re-export core types and functions
pub use mem_stats::{
    MemStats, read_mem_stats, update_allocation_stats, memory_profile, write_profile,
    free_os_memory as vibecheck_free_os_memory
};
pub use goroutine::*;
pub use version::*;
pub use gc::{
    GcStats, run_gc as collect_garbage, is_gc_enabled as gc_enabled, 
    free_os_memory as gc_free_os_memory, get_gc_stats, configure_gc
};

// Re-export profiling functionality
pub use memory_profiler::{
    MemoryProfilerConfig, MemoryStats, MemoryLeak, HeapAnalysis, AllocationPattern,
    get_memory_profiler, configure_memory_profiler, profile_allocation, profile_deallocation,
    detect_memory_leaks, clear_memory_profile
};
pub use cpu_profiler::{
    CpuProfilerConfig, CpuProfile, FunctionCall, CpuSample, HotPath, PerformanceBottleneck,
    get_cpu_profiler, configure_cpu_profiler, start_cpu_profiling, stop_cpu_profiling,
    profile_function_enter, profile_function_exit, FunctionProfileGuard
};
pub use profile_data::{
    ProfileData, ProfileMetadata, ProfileReportConfig, ReportFormat, MetricValue,
    MemoryProfileData, CpuProfileData, SystemInfo
};
pub use profiler::{
    Profiler, ProfilerConfig, ProfilerState, ProfilingStats, 
    get_profiler, configure_profiler, start_profiling, stop_profiling,
    profiling_stats, generate_profiling_report, ProfileScope
};

use crate::error::CursedError;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global runtime state
static RUNTIME_STATE: OnceLock<Arc<Mutex<RuntimeState>>> = OnceLock::new();

/// Internal runtime state
#[derive(Debug)]
struct RuntimeState {
    /// Program start time
    start_time: SystemTime,
    /// Memory allocator statistics
    alloc_stats: AllocatorStats,
    /// GC configuration and state
    gc_state: GcState,
    /// Runtime hooks and callbacks
    hooks: RuntimeHooks,
}

/// Allocator statistics tracking
#[derive(Debug, Default)]
struct AllocatorStats {
    total_allocated: u64,
    total_freed: u64,
    current_allocated: u64,
    allocation_count: u64,
    free_count: u64,
    peak_allocated: u64,
}

/// GC state and configuration
#[derive(Debug)]
struct GcState {
    target_percent: i32,
    last_gc_time: u64,
    total_pause_time: u64,
    gc_count: u32,
    cpu_fraction: f64,
}

/// Runtime event hooks
#[derive(Debug, Default)]
struct RuntimeHooks {
    gc_notifier: Option<Box<dyn Fn() + Send + Sync>>,
    memory_limit: Option<usize>,
    cpu_profile_rate: Option<u32>,
}

impl Default for GcState {
    fn default() -> Self {
        Self {
            target_percent: 100,
            last_gc_time: 0,
            total_pause_time: 0,
            gc_count: 0,
            cpu_fraction: 0.0,
        }
    }
}

impl RuntimeState {
    fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
            alloc_stats: AllocatorStats::default(),
            gc_state: GcState::default(),
            hooks: RuntimeHooks::default(),
        }
    }
}

/// Initialize the runtime state
fn get_runtime_state() -> Arc<Mutex<RuntimeState>> {
    RUNTIME_STATE.get_or_init(|| {
        Arc::new(Mutex::new(RuntimeState::new()))
    }).clone()
}

/// Get the start time of the program in nanoseconds since epoch
pub fn start_time() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    
    let duration = runtime.start_time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CursedError::Runtime("Invalid start time".to_string()))?;
    
    Ok(duration.as_nanos() as i64)
}

/// Update allocator statistics (called by memory allocator)
pub fn update_alloc_stats(allocated: u64, freed: u64) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    
    if allocated > 0 {
        runtime.alloc_stats.total_allocated += allocated;
        runtime.alloc_stats.current_allocated += allocated;
        runtime.alloc_stats.allocation_count += 1;
        
        if runtime.alloc_stats.current_allocated > runtime.alloc_stats.peak_allocated {
            runtime.alloc_stats.peak_allocated = runtime.alloc_stats.current_allocated;
        }
    }
    
    if freed > 0 {
        runtime.alloc_stats.total_freed += freed;
        runtime.alloc_stats.current_allocated = runtime.alloc_stats.current_allocated.saturating_sub(freed);
        runtime.alloc_stats.free_count += 1;
    }
    
    Ok(())
}

/// Update GC statistics (called by GC)
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CursedError::Runtime("Invalid system time".to_string()))?
        .as_nanos() as u64;
    
    runtime.gc_state.last_gc_time = current_time;
    runtime.gc_state.total_pause_time += pause_time_ns;
    runtime.gc_state.gc_count += 1;
    runtime.gc_state.cpu_fraction = cpu_fraction;
    
    // Call GC notifier hook if registered
    if let Some(ref notifier) = runtime.hooks.gc_notifier {
        // Clone the notifier to avoid holding the lock
        let notifier_fn = unsafe { 
            std::mem::transmute::<&Box<dyn Fn() + Send + Sync>, &'static Box<dyn Fn() + Send + Sync>>(notifier)
        };
        drop(runtime); // Release lock before calling
        notifier_fn();
    }
    
    Ok(())
}

/// Set GC notification callback
pub fn set_gc_notifier<F>(callback: F) -> crate::error::Result<()> 
where 
    F: Fn() + Send + Sync + 'static
{
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.gc_notifier = Some(Box::new(callback));
    Ok(())
}

/// Set memory limit
pub fn set_memory_limit(limit: usize) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.memory_limit = Some(limit);
    Ok(())
}

/// Set CPU profiling rate
pub fn set_cpu_profile_rate(rate: u32) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.cpu_profile_rate = Some(rate);
    Ok(())
}

/// Get current memory limit
pub fn get_memory_limit() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(runtime.hooks.memory_limit)
}

/// Get current CPU profile rate
pub fn get_cpu_profile_rate() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(runtime.hooks.cpu_profile_rate)
}

// Internal access for other modules
pub(crate) fn get_alloc_stats() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(AllocatorStats {
        total_allocated: runtime.alloc_stats.total_allocated,
        total_freed: runtime.alloc_stats.total_freed,
        current_allocated: runtime.alloc_stats.current_allocated,
        allocation_count: runtime.alloc_stats.allocation_count,
        free_count: runtime.alloc_stats.free_count,
        peak_allocated: runtime.alloc_stats.peak_allocated,
    })
}

pub(crate) fn get_gc_state() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(GcState {
        target_percent: runtime.gc_state.target_percent,
        last_gc_time: runtime.gc_state.last_gc_time,
        total_pause_time: runtime.gc_state.total_pause_time,
        gc_count: runtime.gc_state.gc_count,
        cpu_fraction: runtime.gc_state.cpu_fraction,
    })
}

pub(crate) fn set_gc_target_percent(percent: i32) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    let old_percent = runtime.gc_state.target_percent;
    runtime.gc_state.target_percent = percent;
    Ok(old_percent)
}

