//! Incremental garbage collection for CURSED runtime
//! 
//! Implements incremental GC that can pause and resume collection
//! to reduce latency spikes in the runtime.

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor};
use crate::memory::heap::{ObjectId, get_global_heap};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Incremental garbage collector state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncrementalState {
    /// Not collecting
    Idle,
    /// Marking phase in progress
    Marking,
    /// Sweeping phase in progress
    Sweeping,
    /// Collection completed
    Completed,
}

/// Incremental garbage collector
pub struct IncrementalCollector {
    /// Current collection state
    state: Mutex<IncrementalState>,
    /// Work queue for marking
    mark_queue: Mutex<VecDeque<ObjectId>>,
    /// Objects being swept
    sweep_queue: Mutex<VecDeque<ObjectId>>,
    /// Collection statistics
    stats: Mutex<IncrementalStats>,
    /// Configuration
    config: Mutex<IncrementalConfig>,
    /// Current collection start time
    collection_start: Mutex<Option<Instant>>,
}

/// Configuration for incremental collection
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Maximum time to spend in one increment (milliseconds)
    pub time_slice_ms: u64,
    /// Maximum objects to process in one increment
    pub objects_per_slice: usize,
    /// Minimum time between collection starts
    pub collection_interval_ms: u64,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            time_slice_ms: 5, // 5ms slices
            objects_per_slice: 100,
            collection_interval_ms: 1000, // 1 second
        }
    }
}

/// Statistics for incremental collection
#[derive(Debug, Clone, Default)]
pub struct IncrementalStats {
    pub total_collections: u64,
    pub total_increments: u64,
    pub total_collection_time_ms: u64,
    pub average_increment_time_ms: f64,
    pub objects_marked: u64,
    pub objects_swept: u64,
    pub current_collection_increments: u32,
}

impl IncrementalCollector {
    /// Create a new incremental collector
    pub fn new() -> Self {
        Self {
            state: Mutex::new(IncrementalState::Idle),
            mark_queue: Mutex::new(VecDeque::new()),
            sweep_queue: Mutex::new(VecDeque::new()),
            stats: Mutex::new(IncrementalStats::default()),
            config: Mutex::new(IncrementalConfig::default()),
            collection_start: Mutex::new(None),
        }
    }

    /// Configure the collector
    pub fn configure(&self, config: IncrementalConfig) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config;
    }

    /// Start a new incremental collection cycle
    pub fn start_collection(&self) -> Result<(), CursedError> {
        let mut state = self.state.lock().unwrap();
        
        match *state {
            IncrementalState::Idle | IncrementalState::Completed => {
                *state = IncrementalState::Marking;
                
                // Initialize marking queue with root objects
                let mut mark_queue = self.mark_queue.lock().unwrap();
                mark_queue.clear();
                
                let heap = get_global_heap();
                heap.unmark_all();
                
                // Add root objects to marking queue
                let root_set = crate::memory::roots::get_global_root_set();
                let roots = root_set.get_all_roots();
                for root_id in roots {
                    mark_queue.push_back(root_id);
                }
                
                // Record collection start time
                let mut collection_start = self.collection_start.lock().unwrap();
                *collection_start = Some(Instant::now());
                
                Ok(())
            }
            _ => Err(CursedError::RuntimeError(
                "Collection already in progress".to_string()
            )),
        }
    }

    /// Perform one increment of collection work
    pub fn do_increment(&self) -> Result<IncrementalResult, CursedError> {
        let config = self.config.lock().unwrap();
        let time_limit = Duration::from_millis(config.time_slice_ms);
        let object_limit = config.objects_per_slice;
        drop(config);
        
        let start_time = Instant::now();
        let mut objects_processed = 0;
        
        let current_state = {
            let state = self.state.lock().unwrap();
            *state
        };
        
        match current_state {
            IncrementalState::Marking => {
                self.do_marking_increment(time_limit, object_limit, &mut objects_processed)
            }
            IncrementalState::Sweeping => {
                self.do_sweeping_increment(time_limit, object_limit, &mut objects_processed)
            }
            IncrementalState::Idle => {
                Ok(IncrementalResult {
                    state: IncrementalState::Idle,
                    objects_processed: 0,
                    time_taken: Duration::from_nanos(0),
                    phase_completed: false,
                })
            }
            IncrementalState::Completed => {
                self.complete_collection()?;
                Ok(IncrementalResult {
                    state: IncrementalState::Completed,
                    objects_processed: 0,
                    time_taken: start_time.elapsed(),
                    phase_completed: true,
                })
            }
        }
    }

    /// Perform marking increment
    fn do_marking_increment(
        &self,
        time_limit: Duration,
        object_limit: usize,
        objects_processed: &mut usize,
    ) -> Result<IncrementalResult, CursedError> {
        let start_time = Instant::now();
        let heap = get_global_heap();
        let mut mark_queue = self.mark_queue.lock().unwrap();
        
        while *objects_processed < object_limit 
            && start_time.elapsed() < time_limit 
            && !mark_queue.is_empty() 
        {
            if let Some(object_id) = mark_queue.pop_front() {
                // Mark this object
                if heap.mark(object_id) {
                    // Object was newly marked, trace its references
                    // In practice, you'd add referenced objects to the queue
                    *objects_processed += 1;
                }
            }
        }
        
        let phase_completed = mark_queue.is_empty();
        
        if phase_completed {
            // Transition to sweeping phase
            let mut state = self.state.lock().unwrap();
            *state = IncrementalState::Sweeping;
            
            // Initialize sweep queue with all objects
            let mut sweep_queue = self.sweep_queue.lock().unwrap();
            sweep_queue.clear();
            let all_objects = heap.all_objects();
            for obj_id in all_objects {
                sweep_queue.push_back(obj_id);
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_increments += 1;
            stats.objects_marked += *objects_processed as u64;
            stats.current_collection_increments += 1;
        }
        
        Ok(IncrementalResult {
            state: if phase_completed { IncrementalState::Sweeping } else { IncrementalState::Marking },
            objects_processed: *objects_processed,
            time_taken: start_time.elapsed(),
            phase_completed,
        })
    }

    /// Perform sweeping increment
    fn do_sweeping_increment(
        &self,
        time_limit: Duration,
        object_limit: usize,
        objects_processed: &mut usize,
    ) -> Result<IncrementalResult, CursedError> {
        let start_time = Instant::now();
        let heap = get_global_heap();
        let mut sweep_queue = self.sweep_queue.lock().unwrap();
        let mut collected_objects = 0;
        
        while *objects_processed < object_limit 
            && start_time.elapsed() < time_limit 
            && !sweep_queue.is_empty() 
        {
            if let Some(_object_id) = sweep_queue.pop_front() {
                // In practice, you'd check if the object is marked
                // and remove it if not marked
                *objects_processed += 1;
                // For simplification, we're not actually removing objects here
            }
        }
        
        let phase_completed = sweep_queue.is_empty();
        
        if phase_completed {
            // Transition to completed state
            let mut state = self.state.lock().unwrap();
            *state = IncrementalState::Completed;
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_increments += 1;
            stats.objects_swept += *objects_processed as u64;
            stats.current_collection_increments += 1;
        }
        
        Ok(IncrementalResult {
            state: if phase_completed { IncrementalState::Completed } else { IncrementalState::Sweeping },
            objects_processed: *objects_processed,
            time_taken: start_time.elapsed(),
            phase_completed,
        })
    }

    /// Complete the current collection
    fn complete_collection(&self) -> Result<(), CursedError> {
        let collection_time = {
            let collection_start = self.collection_start.lock().unwrap();
            collection_start.map(|start| start.elapsed())
        };
        
        // Update final statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_collections += 1;
            if let Some(time) = collection_time {
                stats.total_collection_time_ms += time.as_millis() as u64;
            }
            
            // Calculate average increment time
            if stats.total_increments > 0 {
                stats.average_increment_time_ms = 
                    stats.total_collection_time_ms as f64 / stats.total_increments as f64;
            }
            
            stats.current_collection_increments = 0;
        }
        
        // Reset state to idle
        let mut state = self.state.lock().unwrap();
        *state = IncrementalState::Idle;
        
        let mut collection_start = self.collection_start.lock().unwrap();
        *collection_start = None;
        
        Ok(())
    }

    /// Get current collection state
    pub fn get_state(&self) -> IncrementalState {
        let state = self.state.lock().unwrap();
        *state
    }

    /// Check if collection is in progress
    pub fn is_collecting(&self) -> bool {
        let state = self.state.lock().unwrap();
        matches!(*state, IncrementalState::Marking | IncrementalState::Sweeping)
    }

    /// Get statistics
    pub fn stats(&self) -> IncrementalStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Run collection to completion with time slicing
    pub fn collect_incremental(&self) -> Result<Vec<IncrementalResult>, CursedError> {
        let mut results = Vec::new();
        
        // Start collection if not already started
        if self.get_state() == IncrementalState::Idle {
            self.start_collection()?;
        }
        
        // Run increments until completion
        loop {
            let result = self.do_increment()?;
            results.push(result.clone());
            
            if result.state == IncrementalState::Completed {
                break;
            }
            
            // Small yield between increments to allow other work
            std::thread::sleep(Duration::from_micros(100));
        }
        
        Ok(results)
    }
}

impl Default for IncrementalCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an incremental collection step
#[derive(Debug, Clone)]
pub struct IncrementalResult {
    pub state: IncrementalState,
    pub objects_processed: usize,
    pub time_taken: Duration,
    pub phase_completed: bool,
}

impl std::fmt::Display for IncrementalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Incremental GC ({:?}): processed {} objects in {:.2}ms{}",
            self.state,
            self.objects_processed,
            self.time_taken.as_secs_f64() * 1000.0,
            if self.phase_completed { " [PHASE COMPLETE]" } else { "" }
        )
    }
}

/// Global incremental collector
static GLOBAL_INCREMENTAL_GC: std::sync::LazyLock<Arc<IncrementalCollector>> = 
    std::sync::LazyLock::new(|| Arc::new(IncrementalCollector::new()));

/// Get the global incremental collector
pub fn get_global_incremental_gc() -> Arc<IncrementalCollector> {
    Arc::clone(&GLOBAL_INCREMENTAL_GC)
}

/// Compatibility exports
pub use IncrementalCollector as MinimalImplementation;

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let gc = get_global_incremental_gc();
    let stats = gc.stats();
    Ok(format!("Incremental GC ready - {} collections, {:.2}ms avg increment", 
               stats.total_collections, stats.average_increment_time_ms))
}
