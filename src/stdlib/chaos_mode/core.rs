/// Core ChaosMode runtime functionality
/// 
/// Basic goroutine and garbage collection management functions

use crate::stdlib::chaos_mode::error::{ChaosResult, runtime_error, system_error};
use crate::stdlib::vibecheck;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static MAX_HEAP: Mutex<Option<u64>> = Mutex::new(None);

pub fn initialize() -> ChaosResult<()> {
    if INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        // Initialize vibecheck if not already initialized
        vibecheck::init_scheduler();
        Ok(())
    } else {
        Ok(()) // Already initialized
    }
}

pub fn cleanup() -> ChaosResult<()> {
    INITIALIZED.store(false, Ordering::SeqCst);
    Ok(())
}

/// Returns the number of logical CPUs usable by the current process  
pub fn num_cpu() -> ChaosResult<i32> {
    Ok(vibecheck::num_cpu())
}

/// Returns the number of goroutines that currently exist
pub fn num_goroutine() -> ChaosResult<i32> {
    Ok(vibecheck::num_goroutine())
}

/// Yields the processor, allowing other goroutines to run
pub fn yield_processor() -> ChaosResult<()> {
    std::thread::yield_now();
    Ok(())
}

/// Puts the current goroutine into a waiting state and schedules another goroutine
pub fn gosched() -> ChaosResult<()> {
    // In a real implementation, this would interact with the goroutine scheduler
    // For now, we yield the OS thread
    std::thread::yield_now();
    Ok(())
}

/// Forces garbage collection to run
pub fn gc() -> ChaosResult<()> {
    vibecheck::run_gc();
    Ok(())
}

/// Increases GOMAXPROCS, returns the previous setting
pub fn gomaxprocs(n: i32) -> ChaosResult<i32> {
    Ok(vibecheck::gomaxprocs(n))
}

/// Controls the garbage collector's target percentage
pub fn set_gc_percent(percent: i32) -> ChaosResult<i32> {
    let old = vibecheck::get_gc_percent();
    vibecheck::set_gc_percent(percent);
    Ok(old)
}

/// Controls the fraction of memory that should be used for garbage collection
pub fn set_max_heap(max_heap: u64) -> ChaosResult<u64> {
    let mut heap_guard = MAX_HEAP.lock().map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    let old = heap_guard.unwrap_or(0);
    *heap_guard = Some(max_heap);
    
    // Set memory limit in vibecheck if available
    vibecheck::set_memory_limit(max_heap as usize);
    
    Ok(old)
}

/// Get the current max heap setting
pub fn get_max_heap() -> ChaosResult<Option<u64>> {
    let heap_guard = MAX_HEAP.lock().map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    Ok(*heap_guard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_cpu() {
        let result = num_cpu();
        assert!(result.is_ok());
        let cpu_count = result.unwrap();
        assert!(cpu_count > 0);
        assert!(cpu_count <= 256); // Reasonable upper bound
    }

    #[test]
    fn test_num_goroutine() {
        let result = num_goroutine();
        assert!(result.is_ok());
        let goroutine_count = result.unwrap();
        assert!(goroutine_count >= 0);
    }

    #[test]
    fn test_yield_processor() {
        let result = yield_processor();
        assert!(result.is_ok());
    }

    #[test]
    fn test_gosched() {
        let result = gosched();
        assert!(result.is_ok());
    }

    #[test]
    fn test_gc() {
        let result = gc();
        assert!(result.is_ok());
    }

    #[test]
    fn test_gomaxprocs() {
        let current = gomaxprocs(0).unwrap();
        assert!(current > 0);
        
        // Set to 2 and verify it returns the old value
        let old = gomaxprocs(2).unwrap();
        assert_eq!(old, current);
        
        // Restore original value
        gomaxprocs(current).unwrap();
    }

    #[test]
    fn test_set_gc_percent() {
        let current = vibecheck::get_gc_percent();
        
        // Set to 50% and verify it returns the old value
        let old = set_gc_percent(50).unwrap();
        assert_eq!(old, current);
        
        // Verify the new value was set
        assert_eq!(vibecheck::get_gc_percent(), 50);
        
        // Restore original value
        set_gc_percent(current).unwrap();
    }

    #[test]
    fn test_set_max_heap() {
        let result = set_max_heap(1024 * 1024 * 1024); // 1GB
        assert!(result.is_ok());
        
        let heap = get_max_heap().unwrap();
        assert_eq!(heap, Some(1024 * 1024 * 1024));
        
        // Test setting again returns old value
        let old = set_max_heap(2048 * 1024 * 1024).unwrap(); // 2GB
        assert_eq!(old, 1024 * 1024 * 1024);
    }

    #[test]
    fn test_initialize_cleanup() {
        assert!(initialize().is_ok());
        assert!(cleanup().is_ok());
        
        // Should be able to initialize again
        assert!(initialize().is_ok());
    }
}
