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
    free_os_memory as vibecheck_free_os_memory
// };
pub use goroutine::*;
pub use version::*;
pub use gc::{
    free_os_memory as gc_free_os_memory, get_gc_stats, configure_gc
// };

// Re-export profiling functionality
pub use memory_profiler::{
    detect_memory_leaks, clear_memory_profile
// };
pub use cpu_profiler::{
    profile_function_enter, profile_function_exit, FunctionProfileGuard
// };
pub use profile_data::{
    MemoryProfileData, CpuProfileData, SystemInfo
// };
pub use profiler::{
    profiling_stats, generate_profiling_report, ProfileScope
// };

use crate::error::CursedError;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global runtime state
static RUNTIME_STATE: OnceLock<Arc<Mutex<RuntimeState>>> = OnceLock::new();

/// Internal runtime state
#[derive(Debug)]
struct RuntimeState {
    /// Program start time
    /// Memory allocator statistics
    /// GC configuration and state
    /// Runtime hooks and callbacks
/// Allocator statistics tracking
#[derive(Debug, Default)]
struct AllocatorStats {
/// GC state and configuration
#[derive(Debug)]
struct GcState {
/// Runtime event hooks
#[derive(Debug, Default)]
struct RuntimeHooks {
impl Default for GcState {
    fn default() -> Self {
        Self {
        }
    }
impl RuntimeState {
    fn new() -> Self {
        Self {
        }
    }
/// Initialize the runtime state
fn get_runtime_state() -> Arc<Mutex<RuntimeState>> {
    RUNTIME_STATE.get_or_init(|| {
        Arc::new(Mutex::new(RuntimeState::new()))
    }).clone()
/// Get the start time of the program in nanoseconds since epoch
pub fn start_time() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    
    let duration = runtime.start_time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CursedError::Runtime("Invalid start time".to_string()))?;
    
    Ok(duration.as_nanos() as i64)
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
    Ok(())
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
        drop(runtime); // Release lock before calling
        notifier_fn();
    Ok(())
/// Set GC notification callback
pub fn set_gc_notifier<F>(callback: F) -> crate::error::Result<()> 
where 
    F: Fn() + Send + Sync + 'static
{
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.gc_notifier = Some(Box::new(callback));
    Ok(())
/// Set memory limit
pub fn set_memory_limit(limit: usize) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.memory_limit = Some(limit);
    Ok(())
/// Set CPU profiling rate
pub fn set_cpu_profile_rate(rate: u32) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    runtime.hooks.cpu_profile_rate = Some(rate);
    Ok(())
/// Get current memory limit
pub fn get_memory_limit() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(runtime.hooks.memory_limit)
/// Get current CPU profile rate
pub fn get_cpu_profile_rate() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(runtime.hooks.cpu_profile_rate)
// Internal access for other modules
pub(crate) fn get_alloc_stats() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(AllocatorStats {
    })
pub(crate) fn get_gc_state() -> crate::error::Result<()> {
    let state = get_runtime_state();
    let runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    Ok(GcState {
    })
pub(crate) fn set_gc_target_percent(percent: i32) -> crate::error::Result<()> {
    let state = get_runtime_state();
    let mut runtime = state.lock().map_err(|_| CursedError::Runtime("Failed to lock runtime state".to_string()))?;
    let old_percent = runtime.gc_state.target_percent;
    runtime.gc_state.target_percent = percent;
    Ok(old_percent)
