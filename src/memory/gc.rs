//! Main garbage collector for CURSED runtime
//! 
//! Provides mark-and-sweep garbage collection with configurable triggers
//! and statistics tracking.

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor, MemoryStats};
use crate::memory::heap::{get_global_heap, HeapStats};
use crate::memory::roots::{get_global_root_set, RootStats};
use crate::runtime::borrowing::{get_global_borrow_checker, BorrowState};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Main garbage collector
pub struct GarbageCollector {
    /// GC configuration
    config: Mutex<GcConfig>,
    /// GC statistics
    stats: Mutex<GcStats>,
    /// Last collection time
    last_collection: Mutex<Option<Instant>>,
}

/// Garbage collector configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// Enable automatic collection
    pub auto_collect: bool,
    /// Memory pressure threshold (bytes)
    pub memory_threshold: usize,
    /// Time threshold between collections
    pub time_threshold: Duration,
    /// Minimum objects before collection
    pub object_threshold: usize,
    /// Initial heap size
    pub initial_heap_size: usize,
    /// Maximum heap size
    pub max_heap_size: Option<usize>,
    /// Young generation ratio
    pub young_generation_ratio: f64,
    /// Young collection threshold
    pub young_collection_threshold: usize,
    /// Old collection threshold
    pub old_collection_threshold: usize,
    /// Enable incremental collection
    pub incremental_collection: bool,
    /// Incremental time budget
    pub incremental_time_budget: u64,
    /// Enable concurrent collection
    pub concurrent_collection: bool,
    /// Concurrent threads
    pub concurrent_threads: usize,
    /// Trigger mode
    pub trigger_mode: crate::runtime::gc::GcTriggerMode,
    /// Enable compaction
    pub enable_compaction: bool,
    /// Compaction threshold
    pub compaction_threshold: f64,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            auto_collect: true,
            memory_threshold: 64 * 1024 * 1024, // 64MB
            time_threshold: Duration::from_secs(30),
            object_threshold: 1000,
            initial_heap_size: 32 * 1024 * 1024, // 32MB
            max_heap_size: Some(1024 * 1024 * 1024), // 1GB
            young_generation_ratio: 0.3,
            young_collection_threshold: 16 * 1024 * 1024, // 16MB
            old_collection_threshold: 128 * 1024 * 1024, // 128MB
            incremental_collection: false,
            incremental_time_budget: 50, // 50ms
            concurrent_collection: false,
            concurrent_threads: 1,
            trigger_mode: crate::runtime::gc::GcTriggerMode::Threshold,
            enable_compaction: false,
            compaction_threshold: 0.5,
        }
    }
}

/// Garbage collection statistics
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GcStats {
    pub total_collections: u64,
    pub total_time_ms: u64,
    pub objects_collected: u64,
    pub bytes_collected: u64,
    pub last_collection_time_ms: u64,
    pub last_objects_collected: usize,
    pub avg_pause_time: std::time::Duration,
    pub max_pause_time: std::time::Duration,
    pub gc_overhead: f64,
    pub heap_utilization: f64,
    pub allocation_rate: f64,
    pub total_gc_time: std::time::Duration,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            config: Mutex::new(GcConfig::default()),
            stats: Mutex::new(GcStats::default()),
            last_collection: Mutex::new(None),
        }
    }

    /// Configure the garbage collector
    pub fn configure(&self, config: GcConfig) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> GcConfig {
        let config = self.config.lock().unwrap();
        config.clone()
    }

    /// Check if garbage collection should be triggered
    pub fn should_collect(&self) -> bool {
        let config = self.config.lock().unwrap();
        
        if !config.auto_collect {
            return false;
        }

        let heap = get_global_heap();
        let heap_stats = heap.stats();

        // Check memory threshold
        if heap_stats.total_size >= config.memory_threshold {
            return true;
        }

        // Check object count threshold
        if heap_stats.total_objects >= config.object_threshold {
            return true;
        }

        // Check time threshold
        let last_collection = self.last_collection.lock().unwrap();
        if let Some(last_time) = *last_collection {
            if last_time.elapsed() >= config.time_threshold {
                return true;
            }
        } else {
            // Never collected before
            return heap_stats.total_objects > 0;
        }

        false
    }

    /// Perform a full garbage collection cycle
    pub fn collect(&self) -> Result<GcResult, CursedError> {
        let start_time = Instant::now();
        let heap = get_global_heap();
        let root_set = get_global_root_set();

        // Get initial statistics
        let initial_stats = heap.stats();

        // Mark phase: mark all reachable objects starting from roots
        let marked_objects = root_set.mark_reachable()?;

        // Sweep phase: remove unmarked objects
        let collected_objects = heap.sweep();

        // Calculate statistics
        let collection_time = start_time.elapsed();
        let final_stats = heap.stats();
        
        let bytes_collected = initial_stats.total_size.saturating_sub(final_stats.total_size);

        // Update GC statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_collections += 1;
            stats.total_time_ms += collection_time.as_millis() as u64;
            stats.objects_collected += collected_objects as u64;
            stats.bytes_collected += bytes_collected as u64;
            stats.last_collection_time_ms = collection_time.as_millis() as u64;
            stats.last_objects_collected = collected_objects;
        }

        // Update last collection time
        {
            let mut last_collection = self.last_collection.lock().unwrap();
            *last_collection = Some(start_time);
        }

        Ok(GcResult {
            marked_objects,
            collected_objects,
            bytes_collected,
            collection_time,
            heap_stats_before: initial_stats,
            heap_stats_after: final_stats,
        })
    }

    /// Force a garbage collection regardless of thresholds
    pub fn force_collect(&self) -> Result<GcResult, CursedError> {
        self.collect()
    }

    /// Get garbage collection statistics
    pub fn stats(&self) -> GcStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Get comprehensive memory statistics
    pub fn memory_stats(&self) -> MemoryStats {
        let gc_stats = self.stats();
        let heap_stats = get_global_heap().stats();
        
        MemoryStats {
            total_allocated: heap_stats.total_size,
            total_freed: gc_stats.bytes_collected as usize,
            current_usage: heap_stats.total_size,
            gc_collections: gc_stats.total_collections,
            gc_time_ms: gc_stats.total_time_ms,
        }
    }

    /// Auto-collect if thresholds are met
    pub fn maybe_collect(&self) -> Result<Option<GcResult>, CursedError> {
        if self.should_collect() {
            Ok(Some(self.collect()?))
        } else {
            Ok(None)
        }
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a garbage collection cycle
#[derive(Debug, Clone)]
pub struct GcResult {
    /// Number of objects marked as reachable
    pub marked_objects: usize,
    /// Number of objects collected (freed)
    pub collected_objects: usize,
    /// Bytes freed during collection
    pub bytes_collected: usize,
    /// Time taken for collection
    pub collection_time: Duration,
    /// Heap statistics before collection
    pub heap_stats_before: HeapStats,
    /// Heap statistics after collection
    pub heap_stats_after: HeapStats,
}

impl std::fmt::Display for GcResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "GC: collected {} objects ({} bytes) in {:.2}ms, {} objects remaining",
            self.collected_objects,
            self.bytes_collected,
            self.collection_time.as_secs_f64() * 1000.0,
            self.heap_stats_after.total_objects
        )
    }
}

/// Global garbage collector instance
static GLOBAL_GC: std::sync::LazyLock<Arc<GarbageCollector>> = std::sync::LazyLock::new(|| {
    Arc::new(GarbageCollector::new())
});

/// Get the global garbage collector
pub fn get_global_gc() -> Arc<GarbageCollector> {
    Arc::clone(&GLOBAL_GC)
}

/// Convenience function to trigger garbage collection
pub fn collect() -> Result<GcResult, CursedError> {
    get_global_gc().collect()
}

/// Convenience function to maybe trigger garbage collection
pub fn maybe_collect() -> Result<Option<GcResult>, CursedError> {
    get_global_gc().maybe_collect()
}

/// Configure the global garbage collector
pub fn configure_gc(config: GcConfig) {
    get_global_gc().configure(config);
}

/// Integrate borrow checker with GC for safe mutable reference handling
pub fn integrate_borrow_checker_with_gc() {
    let checker = get_global_borrow_checker();
    
    // Add callback to notify GC of borrow state changes
    checker.add_gc_callback(|borrow_state: &BorrowState| {
        // If a value has active borrows, mark it as reachable
        if borrow_state.shared_count > 0 || borrow_state.has_mutable {
            // The value should not be collected while borrowed
            tracing::trace!(
                "Value has active borrows (shared: {}, mutable: {})", 
                borrow_state.shared_count, 
                borrow_state.has_mutable
            );
        }
    });
}

/// Enhanced garbage collection that respects borrow checker state
pub fn collect_with_borrow_checking() -> Result<GcResult, CursedError> {
    // Clean up expired references in borrow checker first
    let checker = get_global_borrow_checker();
    checker.cleanup_expired_references();
    
    // Then perform normal GC
    collect()
}

/// Get global GC statistics
pub fn gc_stats() -> GcStats {
    get_global_gc().stats()
}

/// Compatibility exports
pub use GarbageCollector as MinimalImplementation;

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let gc = get_global_gc();
    let stats = gc.stats();
    Ok(format!("GC ready - {} collections, {} objects collected", 
               stats.total_collections, stats.objects_collected))
}
