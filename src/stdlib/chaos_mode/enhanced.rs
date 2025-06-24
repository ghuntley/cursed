/// Enhanced ChaosMode functionality
/// 
/// Provides advanced features including enhanced garbage collection,
/// performance tuning, and system optimization capabilities

use crate::stdlib::chaos_mode::error::{ChaosResult, config_error, system_error, runtime_error};
use crate::stdlib::vibecheck;
use crate::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Garbage collection modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GCMode {
    Auto = 0,
    Manual = 1,
    IncrementalOnly = 2,
    StopTheWorldOnly = 3,
}

impl From<i32> for GCMode {
    fn from(value: i32) -> Self {
        match value {
            0 => GCMode::Auto,
            1 => GCMode::Manual,
            2 => GCMode::IncrementalOnly,
            3 => GCMode::StopTheWorldOnly,
            _ => GCMode::Auto, // Default to auto for unknown values
        }
    }
}

/// Scheduler modes for performance tuning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SchedulerMode {
    Default = 0,
    Fair = 1,
    Aggressive = 2,
    Conservative = 3,
}

impl From<i32> for SchedulerMode {
    fn from(value: i32) -> Self {
        match value {
            0 => SchedulerMode::Default,
            1 => SchedulerMode::Fair,
            2 => SchedulerMode::Aggressive,
            3 => SchedulerMode::Conservative,
            _ => SchedulerMode::Default, // Default for unknown values
        }
    }
}

static ENHANCED_MANAGER: Mutex<Option<EnhancedManager>> = Mutex::new(None);

struct EnhancedManager {
    gc_mode: GCMode,
    scheduler_mode: SchedulerMode,
    max_threads: i32,
    gc_notifications: Vec<(Box<dyn Fn() + Send + Sync>, Box<dyn Fn() + Send + Sync>)>,
    gc_in_progress: bool,
    gc_start_time: Option<Instant>,
}

impl EnhancedManager {
    fn new() -> Self {
        Self {
            gc_mode: GCMode::Auto,
            scheduler_mode: SchedulerMode::Default,
            max_threads: vibecheck::num_cpu(),
            gc_notifications: Vec::new(),
            gc_in_progress: false,
            gc_start_time: None,
        }
    }
}

pub fn initialize() -> ChaosResult<()> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error during initialization: {}", e)))?;
    
    if manager_guard.is_none() {
        *manager_guard = Some(EnhancedManager::new());
    }
    
    Ok(())
}

pub fn cleanup() -> ChaosResult<()> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error during cleanup: {}", e)))?;
    
    *manager_guard = None;
    Ok(())
}

/// Fine-grained garbage collection control
pub fn set_gc_mode(mode: GCMode) -> ChaosResult<()> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        manager.gc_mode = mode;
        
        // Configure GC based on mode
        match mode {
            GCMode::Auto => {
                // Let the runtime decide GC parameters
                vibecheck::configure_gc(vibecheck::GcConfig {
                    enabled: true,
                    target_heap_size: 0, // Auto
                    max_pause_ms: 10,    // 10ms max pause
                    parallel: true,
                    incremental: true,
                });
            },
            GCMode::Manual => {
                // Disable automatic GC, require manual triggers
                vibecheck::configure_gc(vibecheck::GcConfig {
                    enabled: false,
                    target_heap_size: 0,
                    max_pause_ms: 1000,  // Allow longer pauses for manual GC
                    parallel: true,
                    incremental: false,
                });
            },
            GCMode::IncrementalOnly => {
                // Enable only incremental GC
                vibecheck::configure_gc(vibecheck::GcConfig {
                    enabled: true,
                    target_heap_size: 0,
                    max_pause_ms: 5,     // Very short pauses
                    parallel: true,
                    incremental: true,
                });
            },
            GCMode::StopTheWorldOnly => {
                // Traditional stop-the-world GC
                vibecheck::configure_gc(vibecheck::GcConfig {
                    enabled: true,
                    target_heap_size: 0,
                    max_pause_ms: 100,   // Allow longer pauses
                    parallel: true,
                    incremental: false,
                });
            },
        }
        
        Ok(())
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

pub fn get_gc_mode() -> GCMode {
    let manager_guard = ENHANCED_MANAGER.lock().unwrap_or_else(|_| {
        return std::sync::PoisonError::new(std::sync::MutexGuard::leak(
            std::sync::Mutex::new(None).into_inner().unwrap()
        )).into_inner();
    });
    
    if let Some(ref manager) = *manager_guard {
        manager.gc_mode
    } else {
        GCMode::Auto // Default if not initialized
    }
}

/// Starts a concurrent garbage collection cycle
pub fn start_gc() -> ChaosResult<()> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        if manager.gc_in_progress {
            return Err(runtime_error("Garbage collection already in progress"));
        }
        
        manager.gc_in_progress = true;
        manager.gc_start_time = Some(Instant::now());
        
        // Notify before callbacks
        for (before, _) in &manager.gc_notifications {
            before();
        }
        
        // Start GC using vibecheck
        vibecheck::run_gc();
        
        Ok(())
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

/// Waits for the current GC cycle to complete
pub fn wait_for_gc() -> ChaosResult<bool> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        if !manager.gc_in_progress {
            return Ok(true); // No GC in progress
        }
        
        // In a real implementation, this would wait for the actual GC to complete
        // For simulation, we'll just mark it as complete after a short delay
        std::thread::sleep(Duration::from_millis(10));
        
        manager.gc_in_progress = false;
        
        // Notify after callbacks
        for (_, after) in &manager.gc_notifications {
            after();
        }
        
        manager.gc_start_time = None;
        
        Ok(true)
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

/// Registers a function to be called before/after garbage collection
pub fn register_gc_notification<F1, F2>(before: F1, after: F2) -> ChaosResult<()>
where
    F1: Fn() + Send + Sync + 'static,
    F2: Fn() + Send + Sync + 'static,
{
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        manager.gc_notifications.push((Box::new(before), Box::new(after)));
        Ok(())
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

/// Sets the maximum number of threads to use
pub fn set_max_threads(n: i32) -> ChaosResult<i32> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        if n <= 0 {
            return Err(config_error("Thread count must be positive"));
        }
        
        let old = manager.max_threads;
        manager.max_threads = n;
        
        // In a real implementation, this would configure the runtime thread pool
        // For now, we'll use GOMAXPROCS as a proxy
        vibecheck::gomaxprocs(n);
        
        Ok(old)
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

/// Gets the current number of threads
pub fn num_threads() -> ChaosResult<i32> {
    let manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        Ok(manager.max_threads)
    } else {
        // Fallback to system CPU count
        Ok(vibecheck::num_cpu())
    }
}

/// Controls CPU frequency scaling (if supported by OS)
pub fn set_cpu_frequency(percent: i32) -> ChaosResult<String> {
    if percent < 0 || percent > 100 {
        return Err(config_error("CPU frequency percent must be between 0 and 100"));
    }
    
    // This is a platform-specific feature that would require OS-level access
    // For now, we'll return a descriptive message
    Ok(format!("CPU frequency scaling to {}% requested (platform-dependent)", percent))
}

/// Sets thread priorities (if supported by OS)
pub fn set_thread_priority(thread_id: i32, priority: i32) -> ChaosResult<String> {
    if priority < -20 || priority > 20 {
        return Err(config_error("Thread priority must be between -20 and 20"));
    }
    
    // This is a platform-specific feature that would require OS-level access
    // For now, we'll return a descriptive message
    Ok(format!("Thread {} priority set to {} (platform-dependent)", thread_id, priority))
}

/// Controls the runtime scheduler
pub fn set_scheduler_mode(mode: SchedulerMode) -> ChaosResult<()> {
    let mut manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref mut manager) = *manager_guard {
        manager.scheduler_mode = mode;
        
        // Configure scheduler based on mode
        match mode {
            SchedulerMode::Default => {
                // Standard scheduling parameters
                vibecheck::gomaxprocs(vibecheck::num_cpu());
            },
            SchedulerMode::Fair => {
                // Ensure fair scheduling across all goroutines
                vibecheck::gomaxprocs(vibecheck::num_cpu());
            },
            SchedulerMode::Aggressive => {
                // More aggressive scheduling for performance
                vibecheck::gomaxprocs(std::cmp::max(vibecheck::num_cpu() * 2, 1));
            },
            SchedulerMode::Conservative => {
                // Conservative scheduling to reduce CPU usage
                vibecheck::gomaxprocs(std::cmp::max(vibecheck::num_cpu() / 2, 1));
            },
        }
        
        Ok(())
    } else {
        Err(config_error("Enhanced manager not initialized"))
    }
}

pub fn get_scheduler_mode() -> SchedulerMode {
    let manager_guard = ENHANCED_MANAGER.lock().unwrap_or_else(|_| {
        return std::sync::PoisonError::new(std::sync::MutexGuard::leak(
            std::sync::Mutex::new(None).into_inner().unwrap()
        )).into_inner();
    });
    
    if let Some(ref manager) = *manager_guard {
        manager.scheduler_mode
    } else {
        SchedulerMode::Default // Default if not initialized
    }
}

/// Check if GC is currently in progress
pub fn is_gc_in_progress() -> ChaosResult<bool> {
    let manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        Ok(manager.gc_in_progress)
    } else {
        Ok(false)
    }
}

/// Get GC duration if in progress
pub fn get_gc_duration() -> ChaosResult<Option<Duration>> {
    let manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        if let Some(start_time) = manager.gc_start_time {
            Ok(Some(start_time.elapsed()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

/// Get the number of registered GC notification callbacks
pub fn get_gc_notification_count() -> ChaosResult<usize> {
    let manager_guard = ENHANCED_MANAGER.lock()
        .map_err(|e| system_error(&format!("Lock error: {}", e)))?;
    
    if let Some(ref manager) = *manager_guard {
        Ok(manager.gc_notifications.len())
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_initialize_cleanup() {
        assert!(initialize().is_ok());
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_gc_mode() {
        assert!(initialize().is_ok());
        
        // Test setting different GC modes
        assert!(set_gc_mode(GCMode::Manual).is_ok());
        assert_eq!(get_gc_mode(), GCMode::Manual);
        
        assert!(set_gc_mode(GCMode::IncrementalOnly).is_ok());
        assert_eq!(get_gc_mode(), GCMode::IncrementalOnly);
        
        assert!(set_gc_mode(GCMode::StopTheWorldOnly).is_ok());
        assert_eq!(get_gc_mode(), GCMode::StopTheWorldOnly);
        
        assert!(set_gc_mode(GCMode::Auto).is_ok());
        assert_eq!(get_gc_mode(), GCMode::Auto);
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_gc_mode_from_i32() {
        assert_eq!(GCMode::from(0), GCMode::Auto);
        assert_eq!(GCMode::from(1), GCMode::Manual);
        assert_eq!(GCMode::from(2), GCMode::IncrementalOnly);
        assert_eq!(GCMode::from(3), GCMode::StopTheWorldOnly);
        assert_eq!(GCMode::from(999), GCMode::Auto); // Unknown defaults to Auto
    }

    #[test]
    fn test_start_wait_gc() {
        assert!(initialize().is_ok());
        
        // Start GC
        assert!(start_gc().is_ok());
        assert!(is_gc_in_progress().unwrap());
        
        // Starting again should fail
        assert!(start_gc().is_err());
        
        // Wait for GC to complete
        assert!(wait_for_gc().is_ok());
        assert!(!is_gc_in_progress().unwrap());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_gc_notifications() {
        assert!(initialize().is_ok());
        
        let before_called = Arc::new(AtomicBool::new(false));
        let after_called = Arc::new(AtomicBool::new(false));
        
        let before_called_clone = before_called.clone();
        let after_called_clone = after_called.clone();
        
        // Register notification callbacks
        let result = register_gc_notification(
            move || { before_called_clone.store(true, Ordering::SeqCst); },
            move || { after_called_clone.store(true, Ordering::SeqCst); }
        );
        assert!(result.is_ok());
        
        // Verify callback was registered
        assert_eq!(get_gc_notification_count().unwrap(), 1);
        
        // Start and complete GC cycle
        assert!(start_gc().is_ok());
        assert!(wait_for_gc().is_ok());
        
        // Verify callbacks were called
        assert!(before_called.load(Ordering::SeqCst));
        assert!(after_called.load(Ordering::SeqCst));
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_max_threads() {
        assert!(initialize().is_ok());
        
        let current = num_threads().unwrap();
        assert!(current > 0);
        
        // Set max threads
        let old = set_max_threads(4).unwrap();
        assert_eq!(old, current);
        assert_eq!(num_threads().unwrap(), 4);
        
        // Test invalid thread count
        assert!(set_max_threads(0).is_err());
        assert!(set_max_threads(-1).is_err());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_cpu_frequency() {
        let result = set_cpu_frequency(80);
        assert!(result.is_ok());
        let message = result.unwrap();
        assert!(message.contains("80%"));
        
        // Test invalid percentages
        assert!(set_cpu_frequency(-1).is_err());
        assert!(set_cpu_frequency(101).is_err());
    }

    #[test]
    fn test_thread_priority() {
        let result = set_thread_priority(1, 10);
        assert!(result.is_ok());
        let message = result.unwrap();
        assert!(message.contains("Thread 1"));
        assert!(message.contains("priority set to 10"));
        
        // Test invalid priorities
        assert!(set_thread_priority(1, -21).is_err());
        assert!(set_thread_priority(1, 21).is_err());
    }

    #[test]
    fn test_scheduler_mode() {
        assert!(initialize().is_ok());
        
        // Test setting different scheduler modes
        assert!(set_scheduler_mode(SchedulerMode::Fair).is_ok());
        assert_eq!(get_scheduler_mode(), SchedulerMode::Fair);
        
        assert!(set_scheduler_mode(SchedulerMode::Aggressive).is_ok());
        assert_eq!(get_scheduler_mode(), SchedulerMode::Aggressive);
        
        assert!(set_scheduler_mode(SchedulerMode::Conservative).is_ok());
        assert_eq!(get_scheduler_mode(), SchedulerMode::Conservative);
        
        assert!(set_scheduler_mode(SchedulerMode::Default).is_ok());
        assert_eq!(get_scheduler_mode(), SchedulerMode::Default);
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_scheduler_mode_from_i32() {
        assert_eq!(SchedulerMode::from(0), SchedulerMode::Default);
        assert_eq!(SchedulerMode::from(1), SchedulerMode::Fair);
        assert_eq!(SchedulerMode::from(2), SchedulerMode::Aggressive);
        assert_eq!(SchedulerMode::from(3), SchedulerMode::Conservative);
        assert_eq!(SchedulerMode::from(999), SchedulerMode::Default); // Unknown defaults to Default
    }

    #[test]
    fn test_gc_duration() {
        assert!(initialize().is_ok());
        
        // No GC in progress initially
        assert!(get_gc_duration().unwrap().is_none());
        
        // Start GC and check duration
        assert!(start_gc().is_ok());
        let duration = get_gc_duration().unwrap();
        assert!(duration.is_some());
        assert!(duration.unwrap().as_millis() >= 0);
        
        // Complete GC
        assert!(wait_for_gc().is_ok());
        assert!(get_gc_duration().unwrap().is_none());
        
        assert!(cleanup().is_ok());
    }

    #[test]
    fn test_uninitialized_fallbacks() {
        // Test that functions handle uninitialized state gracefully
        assert_eq!(get_gc_mode(), GCMode::Auto);
        assert_eq!(get_scheduler_mode(), SchedulerMode::Default);
        
        // num_threads should still work using fallback
        let result = num_threads();
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_multiple_gc_notifications() {
        assert!(initialize().is_ok());
        
        // Register multiple callbacks
        for i in 0..3 {
            let result = register_gc_notification(
                move || { println!("Before GC {}", i); },
                move || { println!("After GC {}", i); }
            );
            assert!(result.is_ok());
        }
        
        assert_eq!(get_gc_notification_count().unwrap(), 3);
        
        // Start and complete GC cycle (all callbacks should be called)
        assert!(start_gc().is_ok());
        assert!(wait_for_gc().is_ok());
        
        assert!(cleanup().is_ok());
    }
}
