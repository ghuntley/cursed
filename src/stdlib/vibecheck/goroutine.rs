/// Goroutine Management for vibecheck
/// 
/// Provides goroutine information, control, and debugging capabilities

use crate::error::Error;
use crate::runtime::goroutine::{GoroutineScheduler, GoroutineState};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

/// Global goroutine scheduler reference
static GLOBAL_SCHEDULER: OnceLock<Arc<Mutex<Option<GoroutineScheduler>>>> = OnceLock::new();

/// Thread-local goroutine ID for current thread
thread_local! {
    static CURRENT_GOROUTINE_ID: std::cell::Cell<Option<u64>> = std::cell::Cell::new(None);
}

/// Goroutine information structure
#[derive(Debug, Clone)]
pub struct GoroutineInfo {
    pub id: u64,
    pub state: GoroutineState,
    pub runtime: Duration,
    pub safe_points: usize,
    pub stack_size: usize,
    pub parent_id: Option<u64>,
}

/// Initialize the global goroutine scheduler
pub fn init_scheduler() -> Result<(), Error> {
    let scheduler_ref = GLOBAL_SCHEDULER.get_or_init(|| {
        Arc::new(Mutex::new(None))
    });
    
    let mut scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    if scheduler_opt.is_none() {
        let mut scheduler = GoroutineScheduler::new();
        scheduler.start()?;
        *scheduler_opt = Some(scheduler);
    }
    
    Ok(())
}

/// Get reference to global scheduler
fn get_scheduler() -> Result<Arc<Mutex<Option<GoroutineScheduler>>>, Error> {
    Ok(GLOBAL_SCHEDULER.get_or_init(|| {
        Arc::new(Mutex::new(None))
    }).clone())
}

/// Get the current number of goroutines
pub fn num_goroutine() -> Result<i32, Error> {
    let scheduler_ref = get_scheduler()?;
    let scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    if let Some(ref scheduler) = *scheduler_opt {
        let active_goroutines = scheduler.active_goroutines();
        Ok(active_goroutines.len() as i32)
    } else {
        // No scheduler initialized, return 1 for main thread
        Ok(1)
    }
}

/// Get the current goroutine ID
pub fn go_id() -> Result<i64, Error> {
    // Try to get from thread-local storage first
    let current_id = CURRENT_GOROUTINE_ID.with(|id| id.get());
    
    if let Some(id) = current_id {
        return Ok(id as i64);
    }
    
    // Fallback: use thread ID hash for non-goroutine threads
    let thread_id = thread::current().id();
    let id_hash = format!("{:?}", thread_id).bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    
    Ok(id_hash as i64)
}

/// Set the current goroutine ID (called by goroutine runtime)
pub fn set_current_goroutine_id(id: u64) {
    CURRENT_GOROUTINE_ID.with(|cell| cell.set(Some(id)));
}

/// Clear the current goroutine ID (called when goroutine exits)
pub fn clear_current_goroutine_id() {
    CURRENT_GOROUTINE_ID.with(|cell| cell.set(None));
}

/// Get stack trace for all goroutines
pub fn stack() -> Result<Vec<u8>, Error> {
    let scheduler_ref = get_scheduler()?;
    let scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    let mut stack_trace = String::new();
    stack_trace.push_str("goroutine stack trace:\n\n");
    
    if let Some(ref scheduler) = *scheduler_opt {
        let active_goroutines = scheduler.active_goroutines();
        
        for goroutine_id in active_goroutines {
            if let Some((state, runtime, safe_points)) = scheduler.get_goroutine_info(goroutine_id) {
                stack_trace.push_str(&format!(
                    "goroutine {} [{}] (runtime: {:?}, safe_points: {}):\n",
                    goroutine_id, format_state(state), runtime, safe_points
                ));
                
                // Add simplified stack trace (in a real implementation, this would
                // involve stack walking and symbol resolution)
                stack_trace.push_str("    main.cursed_function()\n");
                stack_trace.push_str("        runtime/goroutine.go:123\n");
                stack_trace.push_str("    runtime.goexit()\n");
                stack_trace.push_str("        runtime/asm.go:456\n");
                stack_trace.push_str("\n");
            }
        }
    } else {
        // No scheduler, show main thread
        stack_trace.push_str("goroutine 1 [running]:\n");
        stack_trace.push_str("    main.main()\n");
        stack_trace.push_str("        main.cursed:1\n");
        stack_trace.push_str("\n");
    }
    
    Ok(stack_trace.into_bytes())
}

/// Get the number of logical CPUs usable by the current process
pub fn num_cpu() -> Result<i32, Error> {
    Ok(num_cpus::get() as i32)
}

/// Set/get maximum number of CPUs that can execute simultaneously
pub fn gomaxprocs(n: i32) -> Result<i32, Error> {
    static MAXPROCS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
    
    if n <= 0 {
        // Get current value
        let current = MAXPROCS.load(std::sync::atomic::Ordering::SeqCst);
        if current == 0 {
            // Initialize with number of CPUs
            let cpus = num_cpu()?;
            MAXPROCS.store(cpus, std::sync::atomic::Ordering::SeqCst);
            Ok(cpus)
        } else {
            Ok(current)
        }
    } else {
        // Set new value
        let old = MAXPROCS.swap(n, std::sync::atomic::Ordering::SeqCst);
        if old == 0 {
            Ok(num_cpu()?) // Return default if not previously set
        } else {
            Ok(old)
        }
    }
}

/// Get detailed information about all goroutines
pub fn get_all_goroutine_info() -> Result<Vec<(u64, GoroutineInfo)>, Error> {
    let scheduler_ref = get_scheduler()?;
    let scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    let mut goroutine_infos = Vec::new();
    
    if let Some(ref scheduler) = *scheduler_opt {
        let active_goroutines = scheduler.active_goroutines();
        
        for goroutine_id in active_goroutines {
            if let Some((state, runtime, safe_points)) = scheduler.get_goroutine_info(goroutine_id) {
                let info = GoroutineInfo {
                    id: goroutine_id,
                    state,
                    runtime,
                    safe_points,
                    stack_size: 64 * 1024, // Default stack size
                    parent_id: None, // Would need scheduler enhancement to track this
                };
                goroutine_infos.push((goroutine_id, info));
            }
        }
    } else {
        // No scheduler, create info for main thread
        let main_info = GoroutineInfo {
            id: 1,
            state: GoroutineState::Running,
            runtime: Duration::from_secs(0),
            safe_points: 0,
            stack_size: 1024 * 1024, // 1MB for main thread
            parent_id: None,
        };
        goroutine_infos.push((1, main_info));
    }
    
    Ok(goroutine_infos)
}

/// Enable blocking profile for goroutine debugging
pub fn block_profile(enabled: bool) -> Result<(), Error> {
    // In a real implementation, this would enable collection of
    // blocking statistics for goroutines
    if enabled {
        // Enable blocking profile collection
        // This would involve instrumenting synchronization primitives
    } else {
        // Disable blocking profile collection
    }
    
    Ok(())
}

/// Get information about a specific goroutine
pub fn goroutine_info(goroutine_id: u64) -> Result<Option<GoroutineInfo>, Error> {
    let all_infos = get_all_goroutine_info()?;
    
    for (id, info) in all_infos {
        if id == goroutine_id {
            return Ok(Some(info));
        }
    }
    
    Ok(None)
}

/// Coordinate with garbage collection
pub fn coordinate_gc(timeout_ms: u64) -> Result<bool, Error> {
    let scheduler_ref = get_scheduler()?;
    let scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    if let Some(ref scheduler) = *scheduler_opt {
        let timeout = Duration::from_millis(timeout_ms);
        Ok(scheduler.coordinate_gc(timeout))
    } else {
        // No scheduler, assume coordination successful
        Ok(true)
    }
}

/// Get stack bounds for GC scanning
pub fn get_stack_bounds() -> Result<Vec<(*mut u8, *mut u8)>, Error> {
    let scheduler_ref = get_scheduler()?;
    let scheduler_opt = scheduler_ref.lock()
        .map_err(|_| Error::Runtime("Failed to lock scheduler".to_string()))?;
    
    if let Some(ref scheduler) = *scheduler_opt {
        Ok(scheduler.get_stack_bounds())
    } else {
        // No scheduler, return empty bounds
        Ok(Vec::new())
    }
}

/// Format goroutine state for display
fn format_state(state: GoroutineState) -> &'static str {
    match state {
        GoroutineState::Created => "created",
        GoroutineState::Ready => "ready",
        GoroutineState::Running => "running",
        GoroutineState::Waiting => "waiting",
        GoroutineState::Yielded => "yielded", 
        GoroutineState::Completed => "completed",
        GoroutineState::Terminated => "terminated",
    }
}

/// Simplified num_cpus implementation for environments where it's not available
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_cpu() {
        let cpus = num_cpu().unwrap();
        assert!(cpus > 0);
        assert!(cpus <= 256); // Reasonable upper bound
    }

    #[test] 
    fn test_gomaxprocs() {
        // Get current value
        let current = gomaxprocs(0).unwrap();
        assert!(current > 0);
        
        // Set new value
        let old = gomaxprocs(2).unwrap();
        assert!(old > 0);
        
        // Verify new value
        let new_current = gomaxprocs(0).unwrap();
        assert_eq!(new_current, 2);
        
        // Restore old value
        gomaxprocs(old).unwrap();
    }

    #[test]
    fn test_go_id() {
        let id = go_id().unwrap();
        assert!(id > 0);
        
        // Should be consistent for same thread
        let id2 = go_id().unwrap();
        assert_eq!(id, id2);
    }

    #[test]
    fn test_current_goroutine_id() {
        // Initially should not be set
        let initial_id = CURRENT_GOROUTINE_ID.with(|cell| cell.get());
        assert_eq!(initial_id, None);
        
        // Set and verify
        set_current_goroutine_id(42);
        let set_id = CURRENT_GOROUTINE_ID.with(|cell| cell.get());
        assert_eq!(set_id, Some(42));
        
        // Clear and verify
        clear_current_goroutine_id();
        let cleared_id = CURRENT_GOROUTINE_ID.with(|cell| cell.get());
        assert_eq!(cleared_id, None);
    }

    #[test]
    fn test_num_goroutine_without_scheduler() {
        // Should return 1 for main thread when no scheduler is initialized
        let count = num_goroutine().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_stack_without_scheduler() {
        let stack_trace = stack().unwrap();
        let trace_str = String::from_utf8(stack_trace).unwrap();
        
        assert!(trace_str.contains("goroutine 1"));
        assert!(trace_str.contains("running"));
        assert!(trace_str.contains("main.main"));
    }

    #[test]
    fn test_get_all_goroutine_info_without_scheduler() {
        let infos = get_all_goroutine_info().unwrap();
        
        assert_eq!(infos.len(), 1);
        let (id, info) = &infos[0];
        assert_eq!(*id, 1);
        assert_eq!(info.id, 1);
        assert_eq!(info.state, GoroutineState::Running);
        assert_eq!(info.parent_id, None);
    }

    #[test]
    fn test_goroutine_info_lookup() {
        let info = goroutine_info(1).unwrap();
        assert!(info.is_some());
        
        let info = info.unwrap();
        assert_eq!(info.id, 1);
        
        // Non-existent goroutine
        let missing = goroutine_info(999).unwrap();
        assert!(missing.is_none());
    }

    #[test]
    fn test_block_profile() {
        // Should not panic
        block_profile(true).unwrap();
        block_profile(false).unwrap();
    }

    #[test]
    fn test_coordinate_gc_without_scheduler() {
        // Should succeed when no scheduler is present
        let result = coordinate_gc(1000).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_get_stack_bounds_without_scheduler() {
        let bounds = get_stack_bounds().unwrap();
        // Should return empty when no scheduler is present
        assert_eq!(bounds.len(), 0);
    }

    #[test]
    fn test_format_state() {
        assert_eq!(format_state(GoroutineState::Created), "created");
        assert_eq!(format_state(GoroutineState::Running), "running");
        assert_eq!(format_state(GoroutineState::Completed), "completed");
    }
}
