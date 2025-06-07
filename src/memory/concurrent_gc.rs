//! Concurrent garbage collector implementation
//!
//! This module enhances the garbage collector with concurrent collection capabilities,
//! allowing garbage collection to run in a separate thread without blocking the main program.

use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};

// Import tracing
use tracing::{debug, error, info, trace, warn, instrument};

use crate::memory::{GarbageCollector, Traceable, Visitor, Tag, ThreadSafeGc};
use crate::memory::gc::CollectionTrigger;
use crate::memory::mark_sweep::CollectionStats;

/// Configuration options for the concurrent garbage collector
#[derive(Debug, Clone)]
pub struct ConcurrentGcConfig {
    /// Interval between garbage collection cycles (in milliseconds)
    pub collection_interval_ms: u64,
    /// Maximum time the collector can run per cycle (in milliseconds)
    pub time_budget_ms: u64,
    /// Heap size threshold that triggers immediate collection
    pub heap_threshold_bytes: usize,
    /// Number of collector threads
    pub thread_count: usize,
    /// Maximum pause time for stop-the-world phases (in milliseconds)
    pub max_pause_ms: u64,
    /// Whether to log detailed statistics
    pub verbose_logging: bool,
}

impl Default for ConcurrentGcConfig {
    fn default() -> Self {
        Self {
            collection_interval_ms: 5000,   // 5 seconds between cycles
            time_budget_ms: 50,             // 50ms time budget per cycle
            heap_threshold_bytes: 1024 * 1024 * 10, // 10MB heap threshold
            thread_count: 1,                // Single collector thread
            max_pause_ms: 10,               // 10ms max pause time
            verbose_logging: false,         // No verbose logging by default
        }
    }
}

/// Status of the concurrent garbage collector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectorStatus {
    /// Collector is idle
    Idle,
    /// Collector is marking objects
    Marking,
    /// Collector is sweeping objects
    Sweeping,
    /// Collector is paused
    Paused,
    /// Collector has been shut down
    Shutdown,
}

/// Shared state for the concurrent garbage collector
#[derive(Debug)]
struct CollectorState {
    /// Current status of the collector
    status: CollectorStatus,
    /// Collection statistics
    stats: CollectionStats,
    /// Whether a collection has been requested
    collection_requested: bool,
    /// The last time a collection was completed
    last_collection: Option<Instant>,
    /// Collector configuration
    config: ConcurrentGcConfig,
}

/// A concurrent garbage collector that runs in a separate thread
#[derive(Debug)]
pub struct ConcurrentGarbageCollector {
    /// The underlying garbage collector
    gc: Arc<GarbageCollector>,
    /// Shared state protected by a mutex
    state: Arc<Mutex<CollectorState>>,
    /// Condvar for signaling the collector thread
    collector_signal: Arc<Condvar>,
    /// Whether the collector thread is running
    running: Arc<AtomicBool>,
    /// Handle for the collector thread
    collector_thread: Option<JoinHandle<()>>,
}

impl Clone for ConcurrentGarbageCollector {
    fn clone(&self) -> Self {
        // Create a new instance with the same configuration
        // but without copying the thread handle
        Self {
            gc: self.gc.clone(),
            state: self.state.clone(),
            collector_signal: self.collector_signal.clone(),
            running: self.running.clone(),
            collector_thread: None, // New instance needs its own thread
        }
    }
}

impl ConcurrentGarbageCollector {
    /// Create a new concurrent garbage collector with default configuration
    pub fn new(gc: Arc<GarbageCollector>) -> Self {
        Self::with_config(gc, ConcurrentGcConfig::default())
    }
    
    /// Create a new concurrent garbage collector with the specified configuration
    pub fn with_config(gc: Arc<GarbageCollector>, config: ConcurrentGcConfig) -> Self {
        debug!("Creating concurrent garbage collector with config: {:?}", config);
        
        let state = Arc::new(Mutex::new(CollectorState {
            status: CollectorStatus::Idle,
            stats: CollectionStats::default(),
            collection_requested: false,
            last_collection: None,
            config: config.clone(),
        }));
        
        let collector_signal = Arc::new(Condvar::new());
        let running = Arc::new(AtomicBool::new(false));
        
        let mut concurrent_gc = Self {
            gc,
            state,
            collector_signal,
            running,
            collector_thread: None,
        };
        
        // Start the collector thread
        concurrent_gc.start();
        
        concurrent_gc
    }
    
    /// Start the collector thread
    pub fn start(&mut self) {
        debug!("Starting collector thread");
        
        if self.running.load(Ordering::Acquire) {
            debug!("Collector thread already running");
            return;
        }
        
        // Set running to true
        self.running.store(true, Ordering::Release);
        
        // Clone the Arc's for the thread
        let gc = self.gc.clone();
        let state = self.state.clone();
        let collector_signal = self.collector_signal.clone();
        let running = self.running.clone();
        
        // Spawn the collector thread
        self.collector_thread = Some(thread::spawn(move || {
            Self::collector_thread_main(gc, state, collector_signal, running);
        }));
        
        debug!("Collector thread started");
    }
    
    /// Stop the collector thread
    pub fn stop(&mut self) -> bool {
        debug!("Stopping collector thread");
        
        if !self.running.load(Ordering::Acquire) {
            debug!("Collector thread not running");
            return true;
        }
        
        // Set running to false to signal the thread to exit
        self.running.store(false, Ordering::Release);
        
        // Wake up the collector thread if it's sleeping
        if let Ok(mut state) = self.state.lock() {
            state.status = CollectorStatus::Shutdown;
            state.collection_requested = true;
            self.collector_signal.notify_one();
        }
        
        // Join the collector thread
        if let Some(thread) = self.collector_thread.take() {
            // Try to join with a timeout
            match thread.join() {
                Ok(_) => {
                    debug!("Collector thread joined successfully");
                    true
                },
                Err(e) => {
                    error!("Failed to join collector thread: {:?}", e);
                    false
                }
            }
        } else {
            true
        }
    }
    
    /// Request a garbage collection cycle
    pub fn request_collection(&self) {
        debug!("Requesting garbage collection");
        
        if let Ok(mut state) = self.state.lock() {
            state.collection_requested = true;
            self.collector_signal.notify_one();
        }
    }
    
    /// Get the current status of the collector
    pub fn status(&self) -> CollectorStatus {
        if let Ok(state) = self.state.lock() {
            state.status
        } else {
            CollectorStatus::Idle
        }
    }
    
    /// Get the current statistics of the collector
    pub fn stats(&self) -> CollectionStats {
        if let Ok(state) = self.state.lock() {
            state.stats.clone()
        } else {
            CollectionStats::default()
        }
    }
    
    /// Update the collector configuration
    pub fn set_config(&self, config: ConcurrentGcConfig) {
        debug!("Updating collector configuration: {:?}", config);
        
        if let Ok(mut state) = self.state.lock() {
            state.config = config;
            self.collector_signal.notify_one();
        }
    }
    
    /// Get the collector configuration
    pub fn config(&self) -> ConcurrentGcConfig {
        if let Ok(state) = self.state.lock() {
            state.config.clone()
        } else {
            ConcurrentGcConfig::default()
        }
    }
    
    /// Allocate a new object with the underlying garbage collector
    pub fn allocate<T: Traceable + Clone + Send + Sync + 'static>(&self, value: T) -> ThreadSafeGc<T> {
        debug!("Allocating object with concurrent GC");
        
        // Allocate using the underlying GC
        let obj = self.gc.allocate_thread_safe(value);
        
        // Check if we need to trigger a collection
        self.check_threshold_and_trigger();
        
        obj
    }
    
    /// Check if the heap threshold has been reached and trigger a collection if necessary
    fn check_threshold_and_trigger(&self) {
        // Get the current heap size
        let heap_size = self.gc.stats().total_size;
        
        // Get the threshold from config
        let threshold = if let Ok(state) = self.state.lock() {
            state.config.heap_threshold_bytes
        } else {
            return;
        };
        
        // If heap size exceeds threshold, request a collection
        if heap_size > threshold {
            debug!("Heap size {} exceeds threshold {}, triggering collection", heap_size, threshold);
            self.request_collection();
        }
    }
    
    /// Main function for the collector thread
    fn collector_thread_main(
        gc: Arc<GarbageCollector>,
        state: Arc<Mutex<CollectorState>>,
        collector_signal: Arc<Condvar>,
        running: Arc<AtomicBool>,
    ) {
        debug!("Collector thread started");
        
        while running.load(Ordering::Acquire) {
            // Lock the state
            let mut guard = match state.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    error!("Failed to lock collector state: {:?}", e);
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
            };
            
            // Check if a collection has been requested
            let should_collect = guard.collection_requested ||
                                 (guard.last_collection.is_none()) ||
                                 (guard.last_collection.map(|t| t.elapsed().as_millis() as u64 > guard.config.collection_interval_ms)
                                                      .unwrap_or(true));
            
            if should_collect {
                // Reset the collection requested flag
                guard.collection_requested = false;
                
                // Update status to marking
                guard.status = CollectorStatus::Marking;
                
                // Release the lock during collection
                drop(guard);
                
                // Run the garbage collection - measure performance
                let start_time = Instant::now();
                // Call collect_garbage() instead of collect_garbage_internal for better compatibility
                gc.collect_garbage();
                let elapsed = start_time.elapsed();
                
                // Update stats
                let mut guard = state.lock().unwrap();
                guard.stats.total_time_ms = elapsed.as_millis() as u64;
                guard.last_collection = Some(Instant::now());
                guard.status = CollectorStatus::Idle;
                
                if guard.config.verbose_logging {
                    let stats = gc.stats();
                    info!("Concurrent collection completed in {}ms, {} objects remain",
                          elapsed.as_millis(), stats.object_count);
                }
            } else {
                // No collection needed, wait for the next interval or signal
                let wait_time = if let Some(last_collection) = guard.last_collection {
                    let elapsed = last_collection.elapsed().as_millis() as u64;
                    if elapsed >= guard.config.collection_interval_ms {
                        0
                    } else {
                        guard.config.collection_interval_ms - elapsed
                    }
                } else {
                    guard.config.collection_interval_ms
                };
                
                // If we need to wait, use the condvar with a timeout
                if wait_time > 0 {
                    let timeout = Duration::from_millis(wait_time);
                    let _ = collector_signal.wait_timeout(guard, timeout);
                } else {
                    // No need to wait, just continue
                    drop(guard);
                }
            }
        }
        
        debug!("Collector thread exiting");
    }
}

impl Drop for ConcurrentGarbageCollector {
    fn drop(&mut self) {
        // Stop the collector thread when the ConcurrentGarbageCollector is dropped
        debug!("Dropping ConcurrentGarbageCollector");
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;
    use std::time::Duration;
    
    // A thread-safe test object
    #[derive(Debug, Clone)]
    struct TestObject {
        value: Arc<StdMutex<i32>>,
    }
    
    impl TestObject {
        fn new(value: i32) -> Self {
            Self {
                value: Arc::new(StdMutex::new(value)),
            }
        }
        
        fn get_value(&self) -> i32 {
            *self.value.lock().unwrap()
        }
        
        fn set_value(&self, value: i32) {
            *self.value.lock().unwrap() = value;
        }
    }
    
    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No references to trace
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    // Must be Send + Sync for ThreadSafeGc
    unsafe impl Send for TestObject {}
    unsafe impl Sync for TestObject {}
    
    #[test]
    fn test_concurrent_gc_basic() {
        // Create a basic GC
        let gc = Arc::new(GarbageCollector::new());
        
        // Create our concurrent GC with a custom configuration for testing
        let config = ConcurrentGcConfig {
            collection_interval_ms: 100,  // Short interval for testing
            time_budget_ms: 50,           // Short budget for testing
            heap_threshold_bytes: 1024,   // Small threshold to trigger collections
            thread_count: 1,              // Single collector thread
            max_pause_ms: 5,              // Short pause time
            verbose_logging: true,        // Enable verbose logging
        };
        
        let concurrent_gc = ConcurrentGarbageCollector::with_config(gc.clone(), config);
        
        // Allocate some objects
        let mut objects = Vec::new();
        for i in 0..10 {
            let obj = concurrent_gc.allocate(TestObject::new(i));
            objects.push(obj);
        }
        
        // Sleep to allow the collector to run
        thread::sleep(Duration::from_millis(300));
        
        // Ensure objects are still accessible
        for (i, obj) in objects.iter().enumerate() {
            if let Some(inner) = obj.inner() {
                assert_eq!(inner.get_value(), i as i32);
            } else {
                panic!("Object not accessible");
            }
        }
        
        // Force a collection
        concurrent_gc.request_collection();
        
        // Sleep to allow the collection to complete
        thread::sleep(Duration::from_millis(300));
        
        // Objects should still be accessible
        for (i, obj) in objects.iter().enumerate() {
            if let Some(inner) = obj.inner() {
                assert_eq!(inner.get_value(), i as i32);
            } else {
                panic!("Object not accessible after forced collection");
            }
        }
        
        // Drop half the objects
        objects.truncate(5);
        
        // Force another collection
        concurrent_gc.request_collection();
        
        // Sleep to allow the collection to complete
        thread::sleep(Duration::from_millis(300));
        
        // Remaining objects should still be accessible
        for (i, obj) in objects.iter().enumerate() {
            if let Some(inner) = obj.inner() {
                assert_eq!(inner.get_value(), i as i32);
            } else {
                panic!("Object not accessible after dropping half");
            }
        }
    }
}