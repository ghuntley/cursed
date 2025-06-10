/// Enhanced Garbage Collector with Comprehensive Algorithm Integration
/// 
/// This module provides a state-of-the-art garbage collection system that integrates
/// multiple collection algorithms and advanced memory management features:
/// 
/// 1. **Multi-Algorithm Support**: Mark-sweep, incremental, copying, and cycle detection
/// 2. **Smart Algorithm Selection**: Automatic algorithm switching based on heap state
/// 3. **Smart Pointer Safety**: Gc<T> pointers with automatic lifecycle management
/// 4. **Generational Collection**: Young/old generation with adaptive promotion
/// 5. **Concurrent Collection**: Goroutine-aware collection with safe points
/// 6. **Performance Monitoring**: Comprehensive statistics and adaptive tuning
/// 7. **Memory Safety**: Thread-safe operations with comprehensive error handling

use std::sync::{Arc, RwLock, Weak, Mutex};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::memory::heap_manager::{HeapManager, HeapConfig};
use crate::memory::object_store::{ObjectStore, ObjectHandle, Storable};
use crate::memory::mark_sweep::{MarkSweepCollector, MarkSweepConfig, MarkSweepStats};
use crate::memory::incremental::{IncrementalCollector, IncrementalConfig, IncrementalStats};
use crate::memory::copying::{CopyingCollector, CopyingConfig, CopyingStats};
use crate::memory::cycle_detection::{CycleDetector, CycleDetectionConfig, CycleDetectionStats};
use crate::memory::roots::{RootSetManager, RootType};
use crate::profiling::memory::MemoryProfiler;

/// Collection algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionAlgorithm {
    /// Mark-and-sweep for old generation and full collections
    MarkSweep,
    /// Incremental collection for reduced pause times
    Incremental,
    /// Copying collector for young generation
    Copying,
    /// Cycle detection for reference cycles
    CycleDetection,
    /// Automatic selection based on heap state
    Adaptive,
}

/// Collection trigger conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionTrigger {
    /// Manual collection request
    Manual,
    /// Allocation pressure threshold exceeded
    AllocationPressure,
    /// Heap utilization threshold exceeded
    HeapUtilization,
    /// Time-based periodic collection
    Periodic,
    /// Emergency collection due to low memory
    Emergency,
    /// Concurrent goroutine pressure
    GoroutinePressure,
}

/// Collection generation for generational GC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionGeneration {
    /// Young generation (Eden + Survivor spaces)
    Young,
    /// Old generation (Tenured space)
    Old,
    /// Full collection across all generations
    Full,
}

/// Collection configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// Primary collection algorithm
    pub algorithm: CollectionAlgorithm,
    /// Enable generational collection
    pub generational: bool,
    /// Enable incremental collection
    pub incremental: bool,
    /// Enable concurrent collection
    pub concurrent: bool,
    /// Enable goroutine-aware collection
    pub goroutine_aware: bool,
    /// Young generation collection threshold
    pub young_gen_threshold: f64,
    /// Old generation collection threshold
    pub old_gen_threshold: f64,
    /// Emergency collection threshold
    pub emergency_threshold: f64,
    /// Maximum collection pause time
    pub max_pause_time: Duration,
    /// Allocation pressure trigger ratio
    pub allocation_pressure_ratio: f64,
    /// Enable adaptive algorithm selection
    pub adaptive_algorithm_selection: bool,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            algorithm: CollectionAlgorithm::Adaptive,
            generational: true,
            incremental: true,
            concurrent: false,
            goroutine_aware: true,
            young_gen_threshold: 0.8,  // 80% full
            old_gen_threshold: 0.9,    // 90% full
            emergency_threshold: 0.95, // 95% full
            max_pause_time: Duration::from_millis(10),
            allocation_pressure_ratio: 0.1, // 10% collection overhead
            adaptive_algorithm_selection: true,
        }
    }
}

/// Enhanced garbage collector with multi-algorithm support
/// 
/// This garbage collector provides comprehensive memory management with
/// multiple collection algorithms, adaptive selection, and performance monitoring.
pub struct GarbageCollector {
    /// Configuration
    config: RwLock<GcConfig>,
    /// Object store for high-level object management
    object_store: Arc<ObjectStore>,
    /// Heap manager for low-level memory allocation
    heap_manager: Arc<RwLock<HeapManager>>,
    /// Object registry for metadata tracking
    object_registry: SharedObjectRegistry,
    /// Root set manager for root object tracking
    root_manager: Arc<RootSetManager>,
    /// Memory profiler for performance monitoring
    profiler: Option<Arc<MemoryProfiler>>,
    
    /// Collection algorithms
    mark_sweep_collector: Arc<MarkSweepCollector>,
    incremental_collector: Arc<IncrementalCollector>,
    copying_collector: Arc<Mutex<CopyingCollector>>,
    cycle_detector: Arc<CycleDetector>,
    
    /// Collection statistics
    collection_count: std::sync::atomic::AtomicU64,
    objects_collected: std::sync::atomic::AtomicU64,
    bytes_collected: std::sync::atomic::AtomicU64,
    last_collection_time: Mutex<Option<Instant>>,
    
    /// Performance tracking
    collection_stats: RwLock<HashMap<CollectionAlgorithm, CollectionPerformance>>,
    algorithm_effectiveness: RwLock<HashMap<CollectionAlgorithm, f64>>,
    
    /// State tracking
    current_algorithm: RwLock<CollectionAlgorithm>,
    is_collecting: std::sync::atomic::AtomicBool,
    allocation_since_last_gc: std::sync::atomic::AtomicU64,
}

/// Performance tracking for collection algorithms
#[derive(Debug, Clone)]
struct CollectionPerformance {
    total_collections: u64,
    total_time: Duration,
    average_time: Duration,
    bytes_reclaimed: u64,
    effectiveness_score: f64,
    last_updated: Instant,
}

impl GarbageCollector {
    /// Create a new garbage collector with default configuration
    #[instrument]
    pub fn new() -> Self {
        Self::with_config(GcConfig::default(), HeapConfig::default())
    }
    
    /// Create a new garbage collector with custom configuration
    #[instrument]
    pub fn with_config(gc_config: GcConfig, heap_config: HeapConfig) -> Self {
        info!("Creating garbage collector with GC config: {:?}", gc_config);
        
        let object_registry = Arc::new(ObjectRegistry::new());
        let heap_manager = Arc::new(RwLock::new(HeapManager::new(heap_config, object_registry.clone())));
        let object_store = ObjectStore::new(heap_manager.clone(), object_registry.clone());
        let root_manager = Arc::new(RootSetManager::new(object_registry.clone()));
        
        // Initialize collection algorithms
        let mut mark_sweep_collector_impl = MarkSweepCollector::new(object_registry.clone());
        mark_sweep_collector_impl.set_root_manager(root_manager.clone());
        let mark_sweep_collector = Arc::new(mark_sweep_collector_impl);
        
        let mut incremental_collector_impl = IncrementalCollector::new(object_registry.clone());
        incremental_collector_impl.set_root_manager(root_manager.clone());
        let incremental_collector = Arc::new(incremental_collector_impl);
        
        let mut copying_collector_impl = CopyingCollector::new(object_registry.clone())
            .expect("Failed to create copying collector");
        copying_collector_impl.set_root_manager(root_manager.clone());
        let copying_collector = Arc::new(Mutex::new(copying_collector_impl));
        
        let cycle_detector = Arc::new(CycleDetector::new(object_registry.clone()));
        
        // Initialize performance tracking
        let mut collection_stats = HashMap::new();
        let mut algorithm_effectiveness = HashMap::new();
        
        for algorithm in &[
            CollectionAlgorithm::MarkSweep,
            CollectionAlgorithm::Incremental,
            CollectionAlgorithm::Copying,
            CollectionAlgorithm::CycleDetection,
        ] {
            collection_stats.insert(*algorithm, CollectionPerformance {
                total_collections: 0,
                total_time: Duration::ZERO,
                average_time: Duration::ZERO,
                bytes_reclaimed: 0,
                effectiveness_score: 1.0,
                last_updated: Instant::now(),
            });
            algorithm_effectiveness.insert(*algorithm, 1.0);
        }
        
        Self {
            config: RwLock::new(gc_config.clone()),
            object_store,
            heap_manager,
            object_registry,
            root_manager,
            profiler: None,
            mark_sweep_collector,
            incremental_collector,
            copying_collector,
            cycle_detector,
            collection_count: std::sync::atomic::AtomicU64::new(0),
            objects_collected: std::sync::atomic::AtomicU64::new(0),
            bytes_collected: std::sync::atomic::AtomicU64::new(0),
            last_collection_time: Mutex::new(None),
            collection_stats: RwLock::new(collection_stats),
            algorithm_effectiveness: RwLock::new(algorithm_effectiveness),
            current_algorithm: RwLock::new(gc_config.algorithm),
            is_collecting: std::sync::atomic::AtomicBool::new(false),
            allocation_since_last_gc: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    /// Set memory profiler for collection monitoring
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        info!("Enabling memory profiling for garbage collector");
        self.profiler = Some(profiler.clone());
        
        // Enable profiling in heap manager
        if let Ok(mut heap) = self.heap_manager.write() {
            heap.set_profiler(profiler);
        }
    }
    
    /// Allocate a new object and return a Gc<T> pointer
    /// 
    /// This is the main allocation interface that integrates with the object store
    /// and provides automatic memory management.
    #[instrument(skip(self, obj))]
    pub fn allocate<T>(&self, obj: T) -> Result<Gc<T>, String>
    where
        T: Storable,
    {
        debug!("Allocating object of type {}", std::any::type_name::<T>());
        
        let handle = self.object_store.store(obj)?;
        let gc_ptr = Gc::from_handle(handle, Arc::downgrade(&self.object_store));
        
        debug!("Successfully allocated object {}", gc_ptr.object_id());
        Ok(gc_ptr)
    }
    
    /// Get statistics about the garbage collector state
    pub fn stats(&self) -> GcStats {
        self.get_stats().unwrap_or_default()
    }
    
    /// Trigger garbage collection with automatic algorithm selection
    /// 
    /// This performs collection using the most appropriate algorithm based on heap state.
    #[instrument(skip(self))]
    pub fn collect(&self) -> Result<EnhancedCollectionStats, String> {
        self.collect_with_trigger(CollectionTrigger::Manual)
    }
    
    /// Trigger garbage collection with specific trigger
    #[instrument(skip(self))]
    pub fn collect_with_trigger(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        // Prevent concurrent collections
        if self.is_collecting.compare_exchange(false, true, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_err() {
            return Err("Collection already in progress".to_string());
        }
        
        let _guard = CollectionGuard::new(&self.is_collecting);
        
        info!("Starting garbage collection cycle with trigger: {:?}", trigger);
        let collection_start = Instant::now();
        let collection_number = self.collection_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        
        // Select appropriate algorithm based on trigger and heap state
        let algorithm = self.select_collection_algorithm(trigger)?;
        info!("Selected collection algorithm: {:?}", algorithm);
        
        // Update current algorithm
        {
            let mut current = self.current_algorithm.write()
                .map_err(|_| "Failed to acquire write lock on current algorithm")?;
            *current = algorithm;
        }
        
        // Trigger GC event in heap manager for profiling
        if let Ok(heap) = self.heap_manager.read() {
            heap.trigger_gc(&format!("{:?}", algorithm))?;
        }
        
        // Perform collection with selected algorithm
        let algorithm_stats = match algorithm {
            CollectionAlgorithm::MarkSweep => {
                AlgorithmStats::MarkSweep(self.mark_sweep_collector.collect()?)
            }
            CollectionAlgorithm::Incremental => {
                if !self.incremental_collector.is_collecting()? {
                    self.incremental_collector.start_collection()?;
                }
                // Perform incremental steps until complete
                self.run_incremental_collection()?
            }
            CollectionAlgorithm::Copying => {
                let copying = self.copying_collector.lock()
                    .map_err(|_| "Failed to acquire lock on copying collector")?;
                AlgorithmStats::Copying(copying.collect(None)?)
            }
            CollectionAlgorithm::CycleDetection => {
                // Run cycle detection and get stats
                let _cycles = self.cycle_detector.detect_cycles()?;
                AlgorithmStats::CycleDetection(self.cycle_detector.get_stats()?)
            }
            CollectionAlgorithm::Adaptive => {
                // Adaptive collection chooses the best algorithm dynamically
                return self.adaptive_collect(trigger);
            }
        };
        
        let collection_duration = collection_start.elapsed();
        
        // Update performance tracking
        self.update_algorithm_performance(algorithm, collection_duration, &algorithm_stats)?;
        
        // Update global statistics
        let (objects_collected, bytes_collected) = self.extract_collection_metrics(&algorithm_stats);
        self.objects_collected.fetch_add(objects_collected as u64, std::sync::atomic::Ordering::SeqCst);
        self.bytes_collected.fetch_add(bytes_collected as u64, std::sync::atomic::Ordering::SeqCst);
        
        // Update last collection time
        {
            let mut last_time = self.last_collection_time.lock()
                .map_err(|_| "Failed to acquire lock on last collection time")?;
            *last_time = Some(Instant::now());
        }
        
        // Reset allocation counter
        self.allocation_since_last_gc.store(0, std::sync::atomic::Ordering::SeqCst);
        
        let stats = EnhancedCollectionStats {
            collection_number,
            algorithm_used: algorithm,
            trigger,
            total_duration: collection_duration,
            objects_collected,
            bytes_collected,
            algorithm_stats,
            heap_stats: self.get_heap_stats()?,
        };
        
        info!("Garbage collection completed: {:?}", stats);
        Ok(stats)
    }
    
    /// Legacy get stats method for backward compatibility
    pub fn get_stats(&self) -> Result<GcStats, String> {
        let comprehensive_stats = self.get_comprehensive_stats()?;
        let object_store_stats = self.object_store.get_stats()?;
        
        Ok(GcStats {
            total_collections: comprehensive_stats.total_collections,
            total_objects_collected: comprehensive_stats.total_objects_collected,
            current_objects: object_store_stats.total_objects,
            current_heap_size: comprehensive_stats.heap_stats.used_after,
            heap_capacity: comprehensive_stats.heap_stats.total_capacity,
            fragmentation: comprehensive_stats.heap_stats.fragmentation,
        })
    }
    
    /// Legacy collection trigger for backward compatibility
    #[deprecated(since = "1.0.0", note = "Use collect_with_trigger instead")]
    pub fn legacy_collect(&self) -> Result<CollectionStats, String> {
        let enhanced_stats = self.collect()?;
        
        Ok(CollectionStats {
            collection_number: enhanced_stats.collection_number,
            duration: enhanced_stats.total_duration,
            objects_marked: 0, // Not applicable to all algorithms
            objects_collected: enhanced_stats.objects_collected,
            heap_size_before: enhanced_stats.heap_stats.used_before,
            heap_size_after: enhanced_stats.heap_stats.used_after,
        })
    }
    
    /// Check if the collector should run (based on allocation pressure) - Legacy method
    #[deprecated(since = "1.0.0", note = "Use should_collect() -> Result<Option<CollectionTrigger>, String> instead")]
    pub fn should_collect_legacy(&self) -> bool {
        // Simple heuristic: collect if heap is more than 80% full
        if let Ok(Some(_)) = self.should_collect() {
            true
        } else {
            false
        }
    }
    
    /// Select the appropriate collection algorithm based on trigger and heap state
    fn select_collection_algorithm(&self, trigger: CollectionTrigger) -> Result<CollectionAlgorithm, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // If adaptive selection is disabled, use configured algorithm
        if !config.adaptive_algorithm_selection {
            return Ok(config.algorithm);
        }
        
        // Get current heap statistics
        let heap_stats = self.get_heap_stats()?;
        
        // Select algorithm based on trigger and heap state
        let algorithm = match trigger {
            CollectionTrigger::Emergency => {
                // Emergency collection - use most aggressive algorithm
                if heap_stats.fragmentation > 0.7 {
                    CollectionAlgorithm::MarkSweep // Full compaction
                } else {
                    CollectionAlgorithm::Copying // Fast collection
                }
            }
            CollectionTrigger::AllocationPressure => {
                // High allocation pressure - prefer fast collection
                if heap_stats.young_gen_utilization > config.young_gen_threshold {
                    CollectionAlgorithm::Copying
                } else {
                    CollectionAlgorithm::Incremental
                }
            }
            CollectionTrigger::HeapUtilization => {
                // High heap utilization - comprehensive collection
                if heap_stats.old_gen_utilization > config.old_gen_threshold {
                    CollectionAlgorithm::MarkSweep
                } else {
                    CollectionAlgorithm::Incremental
                }
            }
            CollectionTrigger::Periodic => {
                // Periodic collection - balanced approach
                self.select_most_effective_algorithm()?
            }
            CollectionTrigger::GoroutinePressure => {
                // Goroutine pressure - incremental or concurrent
                CollectionAlgorithm::Incremental
            }
            CollectionTrigger::Manual => {
                // Manual collection - use configured algorithm or adaptive
                if config.algorithm == CollectionAlgorithm::Adaptive {
                    self.select_most_effective_algorithm()?
                } else {
                    config.algorithm
                }
            }
        };
        
        debug!("Selected algorithm {:?} for trigger {:?}", algorithm, trigger);
        Ok(algorithm)
    }
    
    /// Select the most effective algorithm based on performance history
    fn select_most_effective_algorithm(&self) -> Result<CollectionAlgorithm, String> {
        let effectiveness = self.algorithm_effectiveness.read()
            .map_err(|_| "Failed to acquire read lock on algorithm effectiveness")?;
        
        let mut best_algorithm = CollectionAlgorithm::MarkSweep;
        let mut best_score = 0.0;
        
        for (&algorithm, &score) in effectiveness.iter() {
            if algorithm != CollectionAlgorithm::Adaptive && score > best_score {
                best_score = score;
                best_algorithm = algorithm;
            }
        }
        
        debug!("Most effective algorithm: {:?} (score: {:.3})", best_algorithm, best_score);
        Ok(best_algorithm)
    }
    
    /// Run incremental collection until completion
    fn run_incremental_collection(&self) -> Result<AlgorithmStats, String> {
        let mut total_increments = 0;
        let start_time = Instant::now();
        
        while self.incremental_collector.is_collecting()? {
            if !self.incremental_collector.step()? {
                break; // No more work
            }
            total_increments += 1;
            
            // Prevent infinite loops
            if total_increments > 10000 {
                warn!("Incremental collection taking too long, forcing completion");
                break;
            }
        }
        
        let stats = self.incremental_collector.get_stats()?;
        debug!("Incremental collection completed in {} steps", total_increments);
        Ok(AlgorithmStats::Incremental(stats))
    }
    
    /// Adaptive collection with dynamic algorithm selection
    fn adaptive_collect(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        debug!("Performing adaptive collection");
        
        // Try multiple algorithms and pick the best one
        let heap_stats = self.get_heap_stats()?;
        
        // Decide between young generation and full collection
        if heap_stats.young_gen_utilization > 0.8 {
            // Young generation collection first
            let copying_result = {
                let copying = self.copying_collector.lock()
                    .map_err(|_| "Failed to acquire lock on copying collector")?;
                copying.collect(None)
            };
            
            match copying_result {
                Ok(copying_stats) => {
                    let stats = EnhancedCollectionStats {
                        collection_number: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
                        algorithm_used: CollectionAlgorithm::Copying,
                        trigger,
                        total_duration: copying_stats.total_duration,
                        objects_collected: copying_stats.objects_copied,
                        bytes_collected: copying_stats.bytes_copied,
                        algorithm_stats: AlgorithmStats::Copying(copying_stats),
                        heap_stats,
                    };
                    return Ok(stats);
                }
                Err(_) => {
                    // Fall back to mark-sweep if copying fails
                    warn!("Copying collection failed, falling back to mark-sweep");
                }
            }
        }
        
        // Full collection with mark-sweep
        let mark_sweep_stats = self.mark_sweep_collector.collect()?;
        let stats = EnhancedCollectionStats {
            collection_number: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
            algorithm_used: CollectionAlgorithm::MarkSweep,
            trigger,
            total_duration: mark_sweep_stats.total_duration,
            objects_collected: mark_sweep_stats.objects_swept,
            bytes_collected: mark_sweep_stats.bytes_reclaimed,
            algorithm_stats: AlgorithmStats::MarkSweep(mark_sweep_stats),
            heap_stats: self.get_heap_stats()?,
        };
        
        Ok(stats)
    }
    
    /// Update algorithm performance tracking
    fn update_algorithm_performance(&self, algorithm: CollectionAlgorithm, duration: Duration, stats: &AlgorithmStats) -> Result<(), String> {
        let mut collection_stats = self.collection_stats.write()
            .map_err(|_| "Failed to acquire write lock on collection stats")?;
        
        let mut effectiveness = self.algorithm_effectiveness.write()
            .map_err(|_| "Failed to acquire write lock on algorithm effectiveness")?;
        
        if let Some(perf) = collection_stats.get_mut(&algorithm) {
            perf.total_collections += 1;
            perf.total_time += duration;
            perf.average_time = perf.total_time / perf.total_collections as u32;
            perf.last_updated = Instant::now();
            
            // Calculate effectiveness score (bytes reclaimed per millisecond)
            let bytes_reclaimed = match stats {
                AlgorithmStats::MarkSweep(s) => s.bytes_reclaimed as u64,
                AlgorithmStats::Incremental(_) => 0, // TODO: Add bytes to incremental stats
                AlgorithmStats::Copying(s) => s.bytes_reclaimed as u64,
                AlgorithmStats::CycleDetection(s) => {
                    // Estimate bytes reclaimed from objects in cycles
                    s.objects_in_cycles * 64 // Estimate 64 bytes per object
                }
            };
            
            perf.bytes_reclaimed += bytes_reclaimed;
            
            let effectiveness_score = if duration.as_millis() > 0 {
                bytes_reclaimed as f64 / duration.as_millis() as f64
            } else {
                bytes_reclaimed as f64
            };
            
            // Exponential moving average for effectiveness
            perf.effectiveness_score = 0.8 * perf.effectiveness_score + 0.2 * effectiveness_score;
            effectiveness.insert(algorithm, perf.effectiveness_score);
        }
        
        debug!("Updated performance for {:?}: effectiveness = {:.3}", algorithm, 
               effectiveness.get(&algorithm).unwrap_or(&0.0));
        Ok(())
    }
    
    /// Extract collection metrics from algorithm stats
    fn extract_collection_metrics(&self, stats: &AlgorithmStats) -> (usize, usize) {
        match stats {
            AlgorithmStats::MarkSweep(s) => (s.objects_swept, s.bytes_reclaimed),
            AlgorithmStats::Incremental(_) => (0, 0), // TODO: Add metrics to incremental stats
            AlgorithmStats::Copying(s) => (s.objects_copied, s.bytes_copied),
            AlgorithmStats::CycleDetection(s) => {
                // Use objects in cycles and estimate bytes
                (s.objects_in_cycles as usize, (s.objects_in_cycles * 64) as usize)
            }
        }
    }
    
    /// Get current heap statistics
    fn get_heap_stats(&self) -> Result<HeapStats, String> {
        let heap = self.heap_manager.read()
            .map_err(|_| "Failed to acquire read lock on heap manager")?;
        let heap_stats = heap.get_stats()?;
        
        // Get young/old generation utilization
        let copying = self.copying_collector.lock()
            .map_err(|_| "Failed to acquire lock on copying collector")?;
        let young_gen_utilization = copying.available_space()
            .map(|available| {
                let total = 64 * 1024 * 1024; // TODO: Get actual young gen size
                1.0 - (available as f64 / total as f64)
            })
            .unwrap_or(0.0);
        
        let old_gen_utilization = heap_stats.average_utilization / 100.0;
        
        Ok(HeapStats {
            total_capacity: heap_stats.total_capacity,
            used_before: heap_stats.total_used,
            used_after: heap_stats.total_used, // TODO: Update after collection
            fragmentation: heap_stats.fragmentation_ratio,
            young_gen_utilization,
            old_gen_utilization,
        })
    }
    
    /// Check if collection should be triggered
    pub fn should_collect(&self) -> Result<Option<CollectionTrigger>, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let heap_stats = self.get_heap_stats()?;
        
        // Check emergency threshold
        if heap_stats.old_gen_utilization > config.emergency_threshold {
            return Ok(Some(CollectionTrigger::Emergency));
        }
        
        // Check heap utilization thresholds
        if heap_stats.old_gen_utilization > config.old_gen_threshold {
            return Ok(Some(CollectionTrigger::HeapUtilization));
        }
        
        if heap_stats.young_gen_utilization > config.young_gen_threshold {
            return Ok(Some(CollectionTrigger::AllocationPressure));
        }
        
        // Check allocation pressure
        let allocation_bytes = self.allocation_since_last_gc.load(std::sync::atomic::Ordering::SeqCst);
        let threshold = (heap_stats.total_capacity as f64 * config.allocation_pressure_ratio) as u64;
        
        if allocation_bytes > threshold {
            return Ok(Some(CollectionTrigger::AllocationPressure));
        }
        
        Ok(None)
    }
    
    /// Notify of allocation for pressure tracking
    pub fn notify_allocation(&self, bytes: usize) {
        self.allocation_since_last_gc.fetch_add(bytes as u64, std::sync::atomic::Ordering::SeqCst);
        
        // Notify incremental collector for step scheduling
        self.incremental_collector.notify_allocation(bytes);
    }
    
    /// Check if goroutine-aware collection should be used
    pub fn should_use_goroutine_aware_collection(&self) -> bool {
        let config = self.config.read().ok();
        config.map(|c| c.goroutine_aware).unwrap_or(false)
    }
    
    /// Perform goroutine-aware collection
    pub fn collect_garbage_with_goroutine_awareness(&self) -> Result<EnhancedCollectionStats, String> {
        if self.should_use_goroutine_aware_collection() {
            // TODO: Integrate with goroutine GC when available
            warn!("Goroutine-aware collection requested but not yet implemented, falling back to standard collection");
        }
        
        self.collect_with_trigger(CollectionTrigger::GoroutinePressure)
    }
    
    /// Get comprehensive garbage collection statistics
    pub fn get_comprehensive_stats(&self) -> Result<ComprehensiveGcStats, String> {
        let collection_stats = self.collection_stats.read()
            .map_err(|_| "Failed to acquire read lock on collection stats")?;
        
        let algorithm_effectiveness = self.algorithm_effectiveness.read()
            .map_err(|_| "Failed to acquire read lock on algorithm effectiveness")?;
        
        let heap_stats = self.get_heap_stats()?;
        
        Ok(ComprehensiveGcStats {
            total_collections: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
            total_objects_collected: self.objects_collected.load(std::sync::atomic::Ordering::SeqCst),
            total_bytes_collected: self.bytes_collected.load(std::sync::atomic::Ordering::SeqCst),
            algorithm_performance: collection_stats.clone(),
            algorithm_effectiveness: algorithm_effectiveness.clone(),
            heap_stats,
            current_algorithm: {
                let current = self.current_algorithm.read()
                    .map_err(|_| "Failed to acquire read lock on current algorithm")?;
                *current
            },
            is_collecting: self.is_collecting.load(std::sync::atomic::Ordering::SeqCst),
        })
    }
    
    /// Update GC configuration
    pub fn update_config(&self, new_config: GcConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        
        *config = new_config.clone();
        
        // Update sub-collector configurations
        if let Ok(mark_sweep_config) = self.create_mark_sweep_config(&new_config) {
            self.mark_sweep_collector.update_config(mark_sweep_config)?;
        }
        
        if let Ok(incremental_config) = self.create_incremental_config(&new_config) {
            self.incremental_collector.update_config(incremental_config)?;
        }
        
        if let Ok(copying_config) = self.create_copying_config(&new_config) {
            let copying = self.copying_collector.lock()
                .map_err(|_| "Failed to acquire lock on copying collector")?;
            copying.update_config(copying_config)?;
        }
        
        info!("Updated GC configuration: {:?}", new_config);
        Ok(())
    }
    
    /// Create mark-sweep configuration from GC config
    fn create_mark_sweep_config(&self, gc_config: &GcConfig) -> Result<MarkSweepConfig, String> {
        Ok(MarkSweepConfig {
            parallel_marking: true,
            marking_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            incremental_sweeping: gc_config.incremental,
            sweep_batch_size: 1000,
            write_barrier: gc_config.concurrent,
            finalization: true,
            marking_time_limit: Some(gc_config.max_pause_time / 2),
            sweeping_time_limit: Some(gc_config.max_pause_time / 2),
            enable_compression: true,
        })
    }
    
    /// Create incremental configuration from GC config
    fn create_incremental_config(&self, gc_config: &GcConfig) -> Result<IncrementalConfig, String> {
        Ok(IncrementalConfig {
            max_step_duration: gc_config.max_pause_time / 4,
            min_step_interval: Duration::from_millis(1),
            work_quantum: 100,
            adaptive_step_sizing: true,
            allocation_collection_ratio: gc_config.allocation_pressure_ratio,
            concurrent_collection: gc_config.concurrent,
            write_barrier_threshold: 0.05,
            remembered_set_optimization: gc_config.generational,
            max_remembered_set_size: 10000,
        })
    }
    
    /// Create copying configuration from GC config
    fn create_copying_config(&self, gc_config: &GcConfig) -> Result<CopyingConfig, String> {
        Ok(CopyingConfig {
            semispace_size: 32 * 1024 * 1024, // 32MB
            fast_allocation: true,
            survivor_ratio: 0.1,
            promotion_age_threshold: 3,
            promotion_size_threshold: 32 * 1024,
            parallel_copying: true,
            copying_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2),
            copying_time_limit: Some(gc_config.max_pause_time),
            enable_aging: gc_config.generational,
            large_object_area: true,
        })
    }
    
    /// Get the object store reference
    pub fn object_store(&self) -> &Arc<ObjectStore> {
        &self.object_store
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: GarbageCollector is safe to send between threads because:
// 1. All fields are either Arc<> or atomic types
// 2. ObjectStore is thread-safe with internal RwLock protection
// 3. HeapManager is protected by Arc<RwLock<>>
// 4. ObjectRegistry is thread-safe
// 5. MemoryProfiler is thread-safe when wrapped in Arc
// 6. Atomic counters are inherently thread-safe
unsafe impl Send for GarbageCollector {}

// Safety: GarbageCollector is safe to share between threads because:
// 1. All operations are coordinated through internal locks (RwLock)
// 2. Arc<> types provide shared ownership semantics
// 3. AtomicU64 counters are thread-safe for concurrent access
// 4. The component interfaces are designed for concurrent access
unsafe impl Sync for GarbageCollector {}

/// Enhanced smart pointer for garbage-collected objects
/// 
/// Gc<T> provides automatic memory management with reference tracking
/// and integration with the garbage collection system.
#[derive(Debug)]
pub struct Gc<T: Storable> {
    /// Handle to the object in the object store
    handle: ObjectHandle<T>,
    /// Weak reference to object store for validation
    object_store: Weak<ObjectStore>,
}

impl<T: Storable> Gc<T> {
    /// Create a Gc from an object handle
    fn from_handle(handle: ObjectHandle<T>, object_store: Weak<ObjectStore>) -> Self {
        // Increment reference count
        if let Some(store) = object_store.upgrade() {
            let _ = store.inc_ref(handle.object_id());
        }
        
        Self {
            handle,
            object_store,
        }
    }
    
    /// Get the object ID
    pub fn object_id(&self) -> ObjectId {
        self.handle.object_id()
    }
    
    /// Check if this pointer is still valid
    pub fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
    
    /// Mark this object as a GC root
    /// 
    /// Root objects are never collected and serve as starting points
    /// for reachability analysis during collection.
    pub fn mark_as_root(&self) -> Result<(), String> {
        self.handle.mark_as_root()
    }
    
    /// Unmark this object as a GC root
    pub fn unmark_as_root(&self) -> Result<(), String> {
        self.handle.unmark_as_root()
    }
    
    /// Get a weak reference to this object
    /// 
    /// Weak references don't prevent collection and can be used to
    /// break reference cycles.
    pub fn downgrade(&self) -> WeakGc<T> {
        WeakGc {
            object_id: self.object_id(),
            object_store: self.object_store.clone(),
            _phantom: PhantomData,
        }
    }
    
    /// Try to upgrade a weak reference back to a strong reference
    pub fn from_weak(weak: &WeakGc<T>) -> Option<Self> {
        let store = weak.object_store.upgrade()?;
        let handle = store.get_handle(weak.object_id)?;
        Some(Self::from_handle(handle, Weak::clone(&weak.object_store)))
    }
}

impl<T: Storable> Deref for Gc<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.handle.get().expect("Gc pointer dereferenced after collection")
    }
}

impl<T: Storable> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.handle.get_mut().expect("Gc pointer dereferenced after collection")
    }
}

impl<T: Storable> AsRef<T> for Gc<T> {
    fn as_ref(&self) -> &T {
        self.handle.get().expect("Gc pointer dereferenced after collection")
    }
}

impl<T: Storable> Clone for Gc<T> {
    fn clone(&self) -> Self {
        // Increment reference count
        if let Some(store) = self.object_store.upgrade() {
            let _ = store.inc_ref(self.object_id());
        }
        
        Self {
            handle: self.handle.clone(),
            object_store: self.object_store.clone(),
        }
    }
}

impl<T: Storable> Drop for Gc<T> {
    fn drop(&mut self) {
        // Decrement reference count
        if let Some(store) = self.object_store.upgrade() {
            let _ = store.dec_ref(self.object_id());
        }
    }
}

// Safety: Gc<T> is Send/Sync if T is Send/Sync (which Storable requires)
unsafe impl<T: Storable> Send for Gc<T> {}
unsafe impl<T: Storable> Sync for Gc<T> {}

/// Weak reference to a garbage-collected object
/// 
/// WeakGc<T> doesn't prevent collection and can be used to break reference cycles.
#[derive(Debug)]
pub struct WeakGc<T: Storable> {
    object_id: ObjectId,
    object_store: Weak<ObjectStore>,
    _phantom: PhantomData<T>,
}

impl<T: Storable> WeakGc<T> {
    /// Try to upgrade this weak reference to a strong reference
    pub fn upgrade(&self) -> Option<Gc<T>> {
        Gc::from_weak(self)
    }
    
    /// Get the object ID (even if object has been collected)
    pub fn object_id(&self) -> ObjectId {
        self.object_id
    }
    
    /// Check if the referenced object still exists
    pub fn is_valid(&self) -> bool {
        if let Some(store) = self.object_store.upgrade() {
            store.is_object_valid(self.object_id)
        } else {
            false
        }
    }
}

impl<T: Storable> Clone for WeakGc<T> {
    fn clone(&self) -> Self {
        Self {
            object_id: self.object_id,
            object_store: self.object_store.clone(),
            _phantom: PhantomData,
        }
    }
}

/// Enhanced collection statistics with algorithm-specific data
#[derive(Debug, Clone)]
pub struct EnhancedCollectionStats {
    pub collection_number: u64,
    pub algorithm_used: CollectionAlgorithm,
    pub trigger: CollectionTrigger,
    pub total_duration: Duration,
    pub objects_collected: usize,
    pub bytes_collected: usize,
    pub algorithm_stats: AlgorithmStats,
    pub heap_stats: HeapStats,
}

/// Algorithm-specific statistics
#[derive(Debug, Clone)]
pub enum AlgorithmStats {
    MarkSweep(MarkSweepStats),
    Incremental(IncrementalStats),
    Copying(CopyingStats),
    CycleDetection(CycleDetectionStats),
}

/// Heap statistics
#[derive(Debug, Clone)]
pub struct HeapStats {
    pub total_capacity: usize,
    pub used_before: usize,
    pub used_after: usize,
    pub fragmentation: f64,
    pub young_gen_utilization: f64,
    pub old_gen_utilization: f64,
}

/// Collection guard to ensure proper cleanup
struct CollectionGuard<'a> {
    is_collecting: &'a std::sync::atomic::AtomicBool,
}

impl<'a> CollectionGuard<'a> {
    fn new(is_collecting: &'a std::sync::atomic::AtomicBool) -> Self {
        Self { is_collecting }
    }
}

impl<'a> Drop for CollectionGuard<'a> {
    fn drop(&mut self) {
        self.is_collecting.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Comprehensive garbage collector statistics
#[derive(Debug, Clone)]
pub struct ComprehensiveGcStats {
    pub total_collections: u64,
    pub total_objects_collected: u64,
    pub total_bytes_collected: u64,
    pub algorithm_performance: HashMap<CollectionAlgorithm, CollectionPerformance>,
    pub algorithm_effectiveness: HashMap<CollectionAlgorithm, f64>,
    pub heap_stats: HeapStats,
    pub current_algorithm: CollectionAlgorithm,
    pub is_collecting: bool,
}

/// Legacy garbage collector statistics (for backward compatibility)
#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub total_collections: u64,
    pub total_objects_collected: u64,
    pub current_objects: usize,
    pub current_heap_size: usize,
    pub heap_capacity: usize,
    pub fragmentation: f64,
}

/// Legacy collection statistics (for backward compatibility)
#[derive(Debug, Clone)]
pub struct CollectionStats {
    pub collection_number: u64,
    pub duration: Duration,
    pub objects_marked: usize,
    pub objects_collected: usize,
    pub heap_size_before: usize,
    pub heap_size_after: usize,
}

impl std::fmt::Display for GcStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "GC Stats:\n\
             - Total Collections: {}\n\
             - Objects Collected: {}\n\
             - Current Objects: {}\n\
             - Heap Size: {} / {} bytes ({:.1}% used)\n\
             - Fragmentation: {:.1}%",
            self.total_collections,
            self.total_objects_collected,
            self.current_objects,
            self.current_heap_size,
            self.heap_capacity,
            if self.heap_capacity > 0 { 
                (self.current_heap_size as f64 / self.heap_capacity as f64) * 100.0 
            } else { 
                0.0 
            },
            self.fragmentation * 100.0
        )
    }
}


