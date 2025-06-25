// Garbage Collection types and statistics for CURSED memory management

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Collection algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollectionAlgorithm {
    MarkAndSweep,
    Copying,
    Generational,
    Incremental,
}

/// Triggers for garbage collection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollectionTrigger {
    Manual,
    MemoryPressure,
    AllocationThreshold,
    TimeThreshold,
    Emergency,
}

/// Basic collection statistics
#[derive(Debug, Clone, Default)]
pub struct CollectionStats {
    pub collections: usize,
    pub bytes_collected: usize,
    pub total_time: Duration,
    pub last_collection: Option<Instant>,
}

/// Enhanced collection statistics with detailed metrics
#[derive(Debug, Clone, Default)]
pub struct EnhancedCollectionStats {
    pub basic: CollectionStats,
    pub by_algorithm: HashMap<CollectionAlgorithm, CollectionStats>,
    pub by_trigger: HashMap<CollectionTrigger, usize>,
    pub peak_memory: usize,
    pub average_pause_time: Duration,
}

/// Heap statistics
#[derive(Debug, Clone, Default)]
pub struct HeapStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub fragmentation: f64,
}

/// Algorithm-specific statistics
#[derive(Debug, Clone, Default)]
pub struct AlgorithmStats {
    pub algorithm: CollectionAlgorithm,
    pub efficiency: f64,
    pub pause_times: Vec<Duration>,
    pub memory_recovered: usize,
}

/// Comprehensive GC statistics
#[derive(Debug, Clone, Default)]
pub struct ComprehensiveGcStats {
    pub enhanced: EnhancedCollectionStats,
    pub heap: HeapStats,
    pub algorithms: Vec<AlgorithmStats>,
    pub uptime: Duration,
}

/// Generic GC statistics type alias
pub type GcStats = CollectionStats;

/// Weak garbage collected pointer stub
#[derive(Debug, Clone)]
pub struct WeakGc<T> {
    inner: Option<T>,
}

impl<T> WeakGc<T> {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn upgrade(&self) -> Option<&T> {
        self.inner.as_ref()
    }
}

impl<T> Default for WeakGc<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection statistics tracker
#[derive(Debug, Default)]
pub struct StatsTracker {
    collections: AtomicUsize,
    bytes_collected: AtomicUsize,
    total_objects: AtomicUsize,
}

impl StatsTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_collection(&self, bytes: usize) {
        self.collections.fetch_add(1, Ordering::Relaxed);
        self.bytes_collected.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> CollectionStats {
        CollectionStats {
            collections: self.collections.load(Ordering::Relaxed),
            bytes_collected: self.bytes_collected.load(Ordering::Relaxed),
            total_time: Duration::default(),
            last_collection: None,
        }
    }
}
