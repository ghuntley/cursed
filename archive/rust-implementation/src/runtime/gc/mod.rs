// GC module with enhanced finalizer support
// Integrates the new finalizer queue with existing collectors

pub mod finalizer_queue;

use crate::runtime::concurrent_gc::ConcurrentGC;
use finalizer_queue::{FinalizerQueue, FinalizerConfig, FinalizerPriority};
use std::sync::{Arc, Mutex};
use log::{debug, info, warn};

/// Enhanced GC wrapper that integrates finalizer queue with existing collectors
pub struct EnhancedGC {
    finalizer_queue: Arc<Mutex<FinalizerQueue>>,
    concurrent_gc: Option<ConcurrentGC>,
}

impl EnhancedGC {
    /// Create new enhanced GC with finalizer support
    pub fn new() -> Self {
        let config = FinalizerConfig {
            max_queue_size: 5000,
            max_worker_threads: 2,
            execution_timeout_ms: 500,
            max_retry_attempts: 3,
            batch_size: 25,
            enable_parallel_execution: true,
            enable_priority_scheduling: true,
        };
        
        let finalizer_queue = Arc::new(Mutex::new(FinalizerQueue::new(config)));
        
        Self {
            finalizer_queue,
            concurrent_gc: None,
        }
    }
    
    /// Initialize with concurrent GC backend
    pub fn with_concurrent_gc(mut self, concurrent_gc: ConcurrentGC) -> Self {
        self.concurrent_gc = Some(concurrent_gc);
        self
    }
    
    /// Register object for finalization during GC
    pub fn register_finalizer(
        &self,
        object_ptr: *mut u8,
        object_size: usize,
        object_type: u32,
        cleanup_fn: impl Fn(*mut u8) -> Result<(), String> + Send + Sync + 'static,
        priority: FinalizerPriority,
    ) -> Result<(), String> {
        let finalizer = Box::new(cleanup_fn);
        
        self.finalizer_queue
            .lock()
            .unwrap()
            .register_finalizer(object_ptr, object_size, object_type, finalizer, priority)
            .map_err(|e| format!("Failed to register finalizer: {:?}", e))
    }
    
    /// Trigger garbage collection with finalizer integration
    pub fn collect(&self) -> Result<usize, String> {
        debug!("Starting GC collection with finalizer coordination");
        
        // Pause finalizer execution during collection
        {
            let queue_guard = self.finalizer_queue.lock().unwrap();
            queue_guard.pause_for_gc();
        }
        
        let result = if let Some(ref concurrent_gc) = self.concurrent_gc {
            // Use concurrent collector
            concurrent_gc.collect_garbage()
                .map_err(|e| format!("Concurrent GC failed: {}", e))
        } else {
            // Fallback to basic collection
            warn!("No concurrent GC backend available, using basic collection");
            Ok(0)
        };
        
        // Resume finalizer execution
        {
            let queue_guard = self.finalizer_queue.lock().unwrap();
            queue_guard.resume_after_gc();
        }
        
        match result {
            Ok(bytes_freed) => {
                info!("GC collection completed, freed {} bytes", bytes_freed);
                Ok(bytes_freed)
            }
            Err(e) => {
                warn!("GC collection failed: {}", e);
                Err(e)
            }
        }
    }
    
    /// Get finalizer queue statistics
    pub fn get_finalizer_stats(&self) -> String {
        let queue_guard = self.finalizer_queue.lock().unwrap();
        let stats = queue_guard.get_stats();
        let queue_sizes = queue_guard.get_queue_sizes();
        
        format!(
            "Finalizer Stats: registered={}, finalized={}, failed={}, queue_sizes={:?}",
            stats.objects_registered.load(std::sync::atomic::Ordering::Relaxed),
            stats.objects_finalized.load(std::sync::atomic::Ordering::Relaxed),
            stats.objects_failed.load(std::sync::atomic::Ordering::Relaxed),
            queue_sizes
        )
    }
    
    /// Shutdown GC and finalizer queue
    pub fn shutdown(&mut self) {
        info!("Shutting down enhanced GC");
        
        // Flush remaining finalizers
        {
            let mut queue_guard = self.finalizer_queue.lock().unwrap();
            if let Err(e) = queue_guard.flush_all(std::time::Duration::from_secs(2)) {
                warn!("Failed to flush all finalizers during shutdown: {:?}", e);
            }
            queue_guard.shutdown(std::time::Duration::from_secs(3));
        }
    }
}

impl Drop for EnhancedGC {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Helper functions for common finalizer patterns
pub mod helpers {
    use super::*;
    use finalizer_queue::gc_integration::*;
    
    /// Register channel for cleanup
    pub fn register_channel_finalizer(
        gc: &EnhancedGC,
        channel_ptr: *mut u8,
        channel_size: usize,
    ) -> Result<(), String> {
        gc.register_finalizer(
            channel_ptr,
            channel_size,
            1, // Channel type tag
            |ptr| {
                debug!("Cleaning up channel at {:p}", ptr);
                // Channel-specific cleanup
                Ok(())
            },
            FinalizerPriority::High,
        )
    }
    
    /// Register function for cleanup
    pub fn register_function_finalizer(
        gc: &EnhancedGC,
        function_ptr: *mut u8,
        function_size: usize,
    ) -> Result<(), String> {
        gc.register_finalizer(
            function_ptr,
            function_size,
            2, // Function type tag
            |ptr| {
                debug!("Cleaning up function at {:p}", ptr);
                // Function-specific cleanup
                Ok(())
            },
            FinalizerPriority::Normal,
        )
    }
    
    /// Register file handle for cleanup
    pub fn register_file_finalizer(
        gc: &EnhancedGC,
        file_ptr: *mut u8,
        file_size: usize,
        fd: i32,
    ) -> Result<(), String> {
        gc.register_finalizer(
            file_ptr,
            file_size,
            3, // File type tag
            move |ptr| {
                debug!("Cleaning up file handle {} at {:p}", fd, ptr);
                unsafe {
                    libc::close(fd);
                }
                Ok(())
            },
            FinalizerPriority::Critical,
        )
    }
    
    /// Register custom object with user-defined cleanup
    pub fn register_custom_finalizer<F>(
        gc: &EnhancedGC,
        object_ptr: *mut u8,
        object_size: usize,
        object_type: u32,
        cleanup_fn: F,
        priority: FinalizerPriority,
    ) -> Result<(), String>
    where
        F: Fn(*mut u8) -> Result<(), String> + Send + Sync + 'static,
    {
        gc.register_finalizer(
            object_ptr,
            object_size,
            object_type,
            cleanup_fn,
            priority,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    use std::thread;
    
    #[test]
    fn test_enhanced_gc_basic() {
        let gc = EnhancedGC::new();
        
        let cleanup_called = Arc::new(AtomicUsize::new(0));
        let cleanup_called_clone = cleanup_called.clone();
        
        let dummy_ptr = Box::into_raw(Box::new(42u8));
        
        gc.register_finalizer(
            dummy_ptr,
            1,
            0,
            move |_ptr| {
                cleanup_called_clone.fetch_add(1, Ordering::Relaxed);
                Ok(())
            },
            FinalizerPriority::Normal,
        ).unwrap();
        
        // Trigger collection
        gc.collect().unwrap();
        
        // Wait for finalizer to execute
        thread::sleep(Duration::from_millis(100));
        
        assert_eq!(cleanup_called.load(Ordering::Relaxed), 1);
        
        // Cleanup
        unsafe { Box::from_raw(dummy_ptr); }
    }
    
    #[test]
    fn test_finalizer_coordination_with_gc() {
        let gc = EnhancedGC::new();
        
        // Register multiple finalizers
        for i in 0..10 {
            let dummy_ptr = Box::into_raw(Box::new(i as u8));
            
            gc.register_finalizer(
                dummy_ptr,
                1,
                i as u32,
                move |ptr| {
                    debug!("Finalizing object {} at {:p}", i, ptr);
                    Ok(())
                },
                if i % 2 == 0 { FinalizerPriority::High } else { FinalizerPriority::Normal },
            ).unwrap();
            
            // Cleanup immediately since we don't use the pointer
            unsafe { Box::from_raw(dummy_ptr); }
        }
        
        // Multiple GC cycles
        for _ in 0..3 {
            gc.collect().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        
        // Check stats
        let stats = gc.get_finalizer_stats();
        assert!(stats.contains("registered=10"));
    }
}
