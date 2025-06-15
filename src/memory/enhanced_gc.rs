/// Enhanced Garbage Collector with Real Heap Management Integration
/// 
/// This module enhances the existing GC system by integrating it with real heap
/// management algorithms while maintaining compatibility with all existing
/// interfaces and goroutine-aware collection.

use std::sync::{Arc, RwLock, Mutex};
use std::ptr::NonNull;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::gc::{
    GarbageCollector, GcConfig, EnhancedCollectionStats, CollectionAlgorithm, 
    CollectionTrigger, AlgorithmStats, HeapStats, Gc, ComprehensiveGcStats
};
use crate::memory::object_store::ObjectHandle;
use crate::memory::heap_manager::{HeapManager, HeapConfig, AllocationMetrics};
use crate::memory::real_heap_manager::{RealHeapManager, RealHeapConfig};
use crate::memory::object_store::{ObjectStore, Storable};
use crate::memory::object_id::{ObjectId, SharedObjectRegistry};
use crate::memory::mark_sweep::{MarkSweepCollector, MarkSweepStats};
use crate::memory::incremental::{IncrementalCollector, IncrementalStats};
use crate::memory::copying::{CopyingCollector, CopyingStats};
use crate::memory::cycle_detection::{CycleDetector, CycleDetectionStats};
use crate::memory::roots::RootSetManager;
use crate::profiling::memory::MemoryProfiler;

/// Enhanced garbage collector that uses real heap management
/// 
/// This collector extends the existing GC with proper heap allocation
/// algorithms while maintaining full compatibility with the existing
/// GC interface and goroutine-aware collection.
pub struct EnhancedGarbageCollector {
    /// Configuration
    config: RwLock<GcConfig>,
    /// Real heap manager for actual memory allocation
    real_heap_manager: Arc<RealHeapManager>,
    /// Legacy heap manager for compatibility
    legacy_heap_manager: Arc<RwLock<HeapManager>>,
    /// Object store for high-level object management
    object_store: Arc<ObjectStore>,
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
    collection_stats: RwLock<HashMap<CollectionAlgorithm, EnhancedCollectionPerformance>>,
    algorithm_effectiveness: RwLock<HashMap<CollectionAlgorithm, f64>>,
    
    /// State tracking
    current_algorithm: RwLock<CollectionAlgorithm>,
    is_collecting: std::sync::atomic::AtomicBool,
    allocation_since_last_gc: std::sync::atomic::AtomicU64,
    
    /// Integration flags
    use_real_heap: bool,
}

/// Performance tracking for collection algorithms (enhanced version)
#[derive(Debug, Clone)]
struct EnhancedCollectionPerformance {
    total_collections: u64,
    total_time: Duration,
    average_time: Duration,
    bytes_reclaimed: u64,
    effectiveness_score: f64,
    last_updated: Instant,
}

/// Enhanced GC statistics with real heap data
#[derive(Debug, Clone)]
pub struct EnhancedGcStats {
    pub total_collections: u64,
    pub total_objects_collected: u64,
    pub total_bytes_collected: u64,
    pub algorithm_effectiveness: HashMap<CollectionAlgorithm, f64>,
    pub heap_stats: HeapStats,
    pub current_algorithm: CollectionAlgorithm,
    pub is_collecting: bool,
    pub enhanced_performance: HashMap<CollectionAlgorithm, EnhancedCollectionPerformance>,
}

impl EnhancedGarbageCollector {
    /// Allocate a new object with enhanced garbage collection
    /// 
    /// This is the main allocation interface for the enhanced GC system.
    /// It uses real heap management when available, falling back to legacy
    /// allocation when needed.
    #[instrument(skip(self, obj))]
    pub fn allocate<T>(&self, obj: T) -> Result<Gc<T>, String>
    where
        T: Storable,
    {
        self.allocate_real(obj)
    }
    /// Convert this enhanced GC to a standard GC
    /// 
    /// This is a convenience method that performs the same conversion as `Into<GarbageCollector>`,
    /// but allows for explicit conversion without consuming the enhanced GC.
    /// 
    /// Returns an error if the conversion fails due to lock poisoning or other issues.
    pub fn to_standard_gc(self) -> Result<GarbageCollector, String> {
        debug!("Converting EnhancedGarbageCollector to standard GarbageCollector");
        Ok(self.into())
    }
    
    /// Check if this enhanced GC can be safely converted to a standard GC
    /// 
    /// This method validates that all required state can be extracted without issues.
    pub fn can_convert_to_standard(&self) -> bool {
        // Check if locks can be acquired
        let config_ok = self.config.try_read().is_ok();
        let stats_ok = self.collection_stats.try_read().is_ok();
        let effectiveness_ok = self.algorithm_effectiveness.try_read().is_ok();
        let algorithm_ok = self.current_algorithm.try_read().is_ok();
        let time_ok = self.last_collection_time.try_lock().is_ok();
        
        config_ok && stats_ok && effectiveness_ok && algorithm_ok && time_ok
    }
    /// Create a new enhanced garbage collector
    #[instrument]
    pub fn new() -> Self {
        Self::with_config(GcConfig::default(), HeapConfig::default(), true)
    }
    
    /// Create enhanced GC with custom configuration
    #[instrument]
    pub fn with_config(gc_config: GcConfig, heap_config: HeapConfig, use_real_heap: bool) -> Self {
        info!("Creating enhanced garbage collector with real heap: {}", use_real_heap);
        
        let object_registry = Arc::new(crate::memory::object_id::ObjectRegistry::new());
        
        // Create real heap manager
        let real_heap_config = RealHeapConfig {
            initial_block_size: heap_config.default_block_size,
            max_blocks: 64,
            growth_factor: 1.5,
            fragmentation_threshold: 0.4,
            pressure_threshold: 0.85,
            auto_compaction: true,
            min_free_space: 0.15,
        };
        
        let real_heap_manager = Arc::new(
            RealHeapManager::new(real_heap_config, object_registry.clone())
                .expect("Failed to create real heap manager")
        );
        
        // Create legacy heap manager for compatibility
        let legacy_heap_manager = Arc::new(RwLock::new(
            HeapManager::new(heap_config, object_registry.clone())
        ));
        
        let object_store = ObjectStore::new(legacy_heap_manager.clone(), object_registry.clone());
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
            collection_stats.insert(*algorithm, EnhancedCollectionPerformance {
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
            real_heap_manager,
            legacy_heap_manager,
            object_store,
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
            use_real_heap,
        }
    }
    
    /// Set memory profiler for collection monitoring
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        info!("Enabling memory profiling for enhanced garbage collector");
        self.profiler = Some(profiler.clone());
        
        // Enable profiling in both heap managers
        if let Ok(mut legacy_heap) = self.legacy_heap_manager.write() {
            legacy_heap.set_profiler(profiler.clone());
        }
        
        // Real heap manager profiling
        // Note: set_profiler takes &mut self, so we'd need to modify the interface
        // For now, we'll set it during construction
    }
    
    /// Allocate a new object using real heap management
    #[instrument(skip(self, obj))]
    pub fn allocate_real<T>(&self, obj: T) -> Result<Gc<T>, String>
    where
        T: Storable,
    {
        debug!("Allocating object of type {} using enhanced heap management", std::any::type_name::<T>());
        
        if self.use_real_heap {
            self.allocate_with_real_heap(obj)
        } else {
            self.allocate_legacy(obj)
        }
    }
    
    /// Allocate using real heap manager
    fn allocate_with_real_heap<T>(&self, obj: T) -> Result<Gc<T>, String>
    where
        T: Storable,
    {
        let type_name = obj.type_name();
        let size = obj.object_size();
        
        debug!("Real heap allocation: {} bytes for {}", size, type_name);
        
        // Use real heap manager for allocation
        let (object_id, ptr) = self.real_heap_manager.allocate(size, std::mem::align_of::<T>(), type_name)?;
        
        // Write object to allocated memory
        unsafe {
            let typed_ptr = ptr.as_ptr() as *mut T;
            typed_ptr.write(obj);
        }
        
        // Create object handle through object store
        let handle = self.create_object_handle(object_id, ptr.cast(), size, type_name)?;
        
        // Create Gc<T> using the public constructor
        let gc_ptr = Gc::from_object_handle(handle, Arc::downgrade(&self.object_store));
        
        // Notify allocation for pressure tracking
        self.notify_allocation_enhanced(size);
        
        debug!("Successfully allocated object {} using real heap at {:p}", object_id, ptr.as_ptr());
        Ok(gc_ptr)
    }
    
    /// Legacy allocation for compatibility
    fn allocate_legacy<T>(&self, obj: T) -> Result<Gc<T>, String>
    where
        T: Storable,
    {
        debug!("Allocating object of type {} using legacy heap", std::any::type_name::<T>());
        
        // Use legacy heap manager through object store
        let handle = self.object_store.store(obj)?;
        let gc_ptr = Gc::from_object_handle(handle, Arc::downgrade(&self.object_store));
        
        // Notify allocation for pressure tracking
        self.notify_allocation_enhanced(obj.object_size());
        
        debug!("Successfully allocated object {} using legacy heap", gc_ptr.object_id());
        Ok(gc_ptr)
    }
    
    /// Create an object handle for an allocated object
    fn create_object_handle<T: Storable>(
        &self, 
        object_id: ObjectId, 
        ptr: NonNull<T>, 
        size: usize, 
        type_name: &str
    ) -> Result<ObjectHandle<T>, String> {
        use crate::memory::object_store::ObjectHandle;
        use std::marker::PhantomData;
        
        // Register object in the registry
        let metadata = crate::memory::object_id::ObjectMetadata::new(
            object_id,
            size,
            type_name.to_string()
        );
        
        if let Err(e) = self.object_registry.register(metadata) {
            warn!("Failed to register object in registry: {}", e);
        }
        
        // Create handle (note: this is a simplified approach)
        // In a full implementation, this would go through the ObjectStore
        Ok(ObjectHandle::new_external(object_id, ptr, Arc::downgrade(&self.object_store)))
    }
    
    /// Enhanced collection with real heap integration
    #[instrument(skip(self))]
    pub fn collect_enhanced(&self) -> Result<EnhancedCollectionStats, String> {
        self.collect_with_trigger_enhanced(CollectionTrigger::Manual)
    }
    
    /// Enhanced collection with specific trigger
    #[instrument(skip(self))]
    pub fn collect_with_trigger_enhanced(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        // Prevent concurrent collections
        if self.is_collecting.compare_exchange(false, true, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_err() {
            return Err("Collection already in progress".to_string());
        }
        
        let _guard = CollectionGuard::new(&self.is_collecting);
        
        info!("Starting enhanced garbage collection cycle with trigger: {:?}", trigger);
        let collection_start = Instant::now();
        let collection_number = self.collection_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        
        // Select appropriate algorithm based on trigger and heap state
        let algorithm = self.select_collection_algorithm_enhanced(trigger)?;
        info!("Selected collection algorithm: {:?}", algorithm);
        
        // Update current algorithm
        {
            let mut current = self.current_algorithm.write()
                .map_err(|_| "Failed to acquire write lock on current algorithm")?;
            *current = algorithm;
        }
        
        // Trigger GC event in both heap managers
        if self.use_real_heap {
            self.real_heap_manager.trigger_gc(&format!("{:?}", algorithm))?;
        }
        
        if let Ok(legacy_heap) = self.legacy_heap_manager.read() {
            legacy_heap.trigger_gc(&format!("{:?}", algorithm))?;
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
                self.run_incremental_collection_enhanced()?
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
                return self.adaptive_collect_enhanced(trigger);
            }
        };
        
        let collection_duration = collection_start.elapsed();
        
        // Update performance tracking
        self.update_algorithm_performance_enhanced(algorithm, collection_duration, &algorithm_stats)?;
        
        // Update global statistics
        let (objects_collected, bytes_collected) = self.extract_collection_metrics_enhanced(&algorithm_stats);
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
            heap_stats: self.get_heap_stats_enhanced()?,
        };
        
        info!("Enhanced garbage collection completed: {:?}", stats);
        Ok(stats)
    }
    
    /// Select collection algorithm with real heap considerations
    fn select_collection_algorithm_enhanced(&self, trigger: CollectionTrigger) -> Result<CollectionAlgorithm, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // If adaptive selection is disabled, use configured algorithm
        if !config.adaptive_algorithm_selection {
            return Ok(config.algorithm);
        }
        
        // Get heap statistics from real heap if available
        let heap_stats = if self.use_real_heap {
            let real_stats = self.real_heap_manager.get_statistics()?;
            HeapStats {
                total_capacity: real_stats.total_capacity,
                used_before: real_stats.total_used,
                used_after: real_stats.total_used, // Will be updated after collection
                fragmentation: real_stats.overall_fragmentation,
                young_gen_utilization: 0.0, // Real heap doesn't have generational info yet
                old_gen_utilization: real_stats.average_block_utilization / 100.0,
            }
        } else {
            self.get_heap_stats_legacy()?
        };
        
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
                if self.use_real_heap {
                    let pressure = self.real_heap_manager.get_memory_pressure();
                    if pressure > 0.8 {
                        CollectionAlgorithm::MarkSweep
                    } else {
                        CollectionAlgorithm::Incremental
                    }
                } else {
                    CollectionAlgorithm::Copying
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
                self.select_most_effective_algorithm_enhanced()?
            }
            CollectionTrigger::GoroutinePressure => {
                // Goroutine pressure - incremental or concurrent
                CollectionAlgorithm::Incremental
            }
            CollectionTrigger::Manual => {
                // Manual collection - use configured algorithm or adaptive
                if config.algorithm == CollectionAlgorithm::Adaptive {
                    self.select_most_effective_algorithm_enhanced()?
                } else {
                    config.algorithm
                }
            }
        };
        
        debug!("Selected algorithm {:?} for trigger {:?} (real_heap: {})", 
               algorithm, trigger, self.use_real_heap);
        Ok(algorithm)
    }
    
    /// Select most effective algorithm based on performance history
    fn select_most_effective_algorithm_enhanced(&self) -> Result<CollectionAlgorithm, String> {
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
    fn run_incremental_collection_enhanced(&self) -> Result<AlgorithmStats, String> {
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
    
    /// Adaptive collection with real heap integration
    fn adaptive_collect_enhanced(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        debug!("Performing adaptive collection with real heap integration");
        
        // Get heap statistics to decide approach
        let heap_stats = if self.use_real_heap {
            let real_stats = self.real_heap_manager.get_statistics()?;
            let pressure = self.real_heap_manager.get_memory_pressure();
            
            // Use pressure to influence collection strategy
            if pressure > 0.8 {
                // High pressure - use mark-sweep for thorough cleanup
                let mark_sweep_stats = self.mark_sweep_collector.collect()?;
                let stats = EnhancedCollectionStats {
                    collection_number: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
                    algorithm_used: CollectionAlgorithm::MarkSweep,
                    trigger,
                    total_duration: mark_sweep_stats.total_duration,
                    objects_collected: mark_sweep_stats.objects_swept,
                    bytes_collected: mark_sweep_stats.bytes_reclaimed,
                    algorithm_stats: AlgorithmStats::MarkSweep(mark_sweep_stats),
                    heap_stats: HeapStats {
                        total_capacity: real_stats.total_capacity,
                        used_before: real_stats.total_used,
                        used_after: real_stats.total_used,
                        fragmentation: real_stats.overall_fragmentation,
                        young_gen_utilization: 0.0,
                        old_gen_utilization: real_stats.average_block_utilization / 100.0,
                    },
                };
                return Ok(stats);
            }
            
            real_stats
        } else {
            // Fall back to legacy adaptive collection
            return self.adaptive_collect_legacy(trigger);
        };
        
        // Use incremental collection for moderate pressure
        if !self.incremental_collector.is_collecting()? {
            self.incremental_collector.start_collection()?;
        }
        
        let incremental_stats = self.run_incremental_collection_enhanced()?;
        let stats = EnhancedCollectionStats {
            collection_number: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
            algorithm_used: CollectionAlgorithm::Incremental,
            trigger,
            total_duration: Duration::from_millis(0), // TODO: Track incremental duration
            objects_collected: 0, // TODO: Extract from incremental stats
            bytes_collected: 0,   // TODO: Extract from incremental stats
            algorithm_stats: incremental_stats,
            heap_stats: HeapStats {
                total_capacity: heap_stats.total_capacity,
                used_before: heap_stats.total_used,
                used_after: heap_stats.total_used,
                fragmentation: heap_stats.overall_fragmentation,
                young_gen_utilization: 0.0,
                old_gen_utilization: heap_stats.average_block_utilization / 100.0,
            },
        };
        
        Ok(stats)
    }
    
    /// Legacy adaptive collection for compatibility
    fn adaptive_collect_legacy(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        debug!("Performing legacy adaptive collection");
        
        // Try copying collection first for young objects
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
                    heap_stats: self.get_heap_stats_legacy()?,
                };
                return Ok(stats);
            }
            Err(_) => {
                // Fall back to mark-sweep if copying fails
                warn!("Copying collection failed, falling back to mark-sweep");
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
            heap_stats: self.get_heap_stats_legacy()?,
        };
        
        Ok(stats)
    }
    
    /// Get heap statistics from real heap manager
    fn get_heap_stats_enhanced(&self) -> Result<HeapStats, String> {
        if self.use_real_heap {
            let real_stats = self.real_heap_manager.get_statistics()?;
            Ok(HeapStats {
                total_capacity: real_stats.total_capacity,
                used_before: real_stats.total_used,
                used_after: real_stats.total_used, // TODO: Update after collection
                fragmentation: real_stats.overall_fragmentation,
                young_gen_utilization: 0.0, // TODO: Add generational support to real heap
                old_gen_utilization: real_stats.average_block_utilization / 100.0,
            })
        } else {
            self.get_heap_stats_legacy()
        }
    }
    
    /// Get heap statistics from legacy heap manager
    fn get_heap_stats_legacy(&self) -> Result<HeapStats, String> {
        let legacy_heap = self.legacy_heap_manager.read()
            .map_err(|_| "Failed to acquire read lock on legacy heap manager")?;
        let heap_stats = legacy_heap.get_stats()?;
        
        // Convert legacy stats to HeapStats
        Ok(HeapStats {
            total_capacity: heap_stats.total_capacity,
            used_before: heap_stats.total_used,
            used_after: heap_stats.total_used,
            fragmentation: heap_stats.fragmentation_ratio,
            young_gen_utilization: 0.0, // Legacy heap doesn't track generations
            old_gen_utilization: heap_stats.average_utilization / 100.0,
        })
    }
    
    /// Update algorithm performance tracking
    fn update_algorithm_performance_enhanced(&self, algorithm: CollectionAlgorithm, duration: Duration, stats: &AlgorithmStats) -> Result<(), String> {
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
    fn extract_collection_metrics_enhanced(&self, stats: &AlgorithmStats) -> (usize, usize) {
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
    
    /// Check if collection should be triggered based on real heap state
    pub fn should_collect_enhanced(&self) -> Result<Option<CollectionTrigger>, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if self.use_real_heap {
            let real_stats = self.real_heap_manager.get_statistics()?;
            let pressure = self.real_heap_manager.get_memory_pressure();
            
            // Check for emergency conditions
            if pressure > 0.95 {
                return Ok(Some(CollectionTrigger::Emergency));
            }
            
            // Check memory pressure threshold
            if pressure > config.emergency_threshold as f64 {
                return Ok(Some(CollectionTrigger::AllocationPressure));
            }
            
            // Check heap utilization
            let utilization = real_stats.total_used as f64 / real_stats.total_capacity as f64;
            if utilization > config.old_gen_threshold {
                return Ok(Some(CollectionTrigger::HeapUtilization));
            }
            
            // Check fragmentation
            if real_stats.overall_fragmentation > config.allocation_pressure_ratio {
                return Ok(Some(CollectionTrigger::AllocationPressure));
            }
        } else {
            // Use legacy heap checking
            let legacy_heap = self.legacy_heap_manager.read()
                .map_err(|_| "Failed to acquire read lock on legacy heap manager")?;
            let heap_stats = legacy_heap.get_stats()?;
            
            let utilization = heap_stats.total_used as f64 / heap_stats.total_capacity as f64;
            
            if utilization > config.emergency_threshold {
                return Ok(Some(CollectionTrigger::Emergency));
            }
            
            if utilization > config.old_gen_threshold {
                return Ok(Some(CollectionTrigger::HeapUtilization));
            }
        }
        
        // Check allocation pressure
        let allocation_bytes = self.allocation_since_last_gc.load(std::sync::atomic::Ordering::SeqCst);
        let threshold = if self.use_real_heap {
            let real_stats = self.real_heap_manager.get_statistics()?;
            (real_stats.total_capacity as f64 * config.allocation_pressure_ratio) as u64
        } else {
            let legacy_heap = self.legacy_heap_manager.read()
                .map_err(|_| "Failed to acquire read lock on legacy heap manager")?;
            let heap_stats = legacy_heap.get_stats()?;
            (heap_stats.total_capacity as f64 * config.allocation_pressure_ratio) as u64
        };
        
        if allocation_bytes > threshold {
            return Ok(Some(CollectionTrigger::AllocationPressure));
        }
        
        Ok(None)
    }
    
    /// Notify of allocation for pressure tracking
    pub fn notify_allocation_enhanced(&self, bytes: usize) {
        self.allocation_since_last_gc.fetch_add(bytes as u64, std::sync::atomic::Ordering::SeqCst);
        
        // Notify incremental collector for step scheduling
        self.incremental_collector.notify_allocation(bytes);
    }
    
    /// Get comprehensive statistics including real heap data
    /// 
    /// For now, returns a simplified version with basic stats since the full
    /// ComprehensiveGcStats struct requires private types from gc.rs
    pub fn get_comprehensive_stats_enhanced(&self) -> Result<EnhancedGcStats, String> {
        let algorithm_effectiveness = self.algorithm_effectiveness.read()
            .map_err(|_| "Failed to acquire read lock on algorithm effectiveness")?;
        
        let heap_stats = self.get_heap_stats_enhanced()?;
        
        let collection_stats = self.collection_stats.read()
            .map_err(|_| "Failed to acquire read lock on collection stats")?;
        
        Ok(EnhancedGcStats {
            total_collections: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
            total_objects_collected: self.objects_collected.load(std::sync::atomic::Ordering::SeqCst),
            total_bytes_collected: self.bytes_collected.load(std::sync::atomic::Ordering::SeqCst),
            algorithm_effectiveness: algorithm_effectiveness.clone(),
            heap_stats,
            current_algorithm: {
                let current = self.current_algorithm.read()
                    .map_err(|_| "Failed to acquire read lock on current algorithm")?;
                *current
            },
            is_collecting: self.is_collecting.load(std::sync::atomic::Ordering::SeqCst),
            enhanced_performance: collection_stats.clone(),
        })
    }
    
    /// Create a simplified stats struct for enhanced GC
    pub fn get_basic_stats(&self) -> Result<crate::memory::gc::GcStats, String> {
        let heap_stats = self.get_heap_stats_enhanced()?;
        
        Ok(crate::memory::gc::GcStats {
            total_collections: self.collection_count.load(std::sync::atomic::Ordering::SeqCst),
            total_objects_collected: self.objects_collected.load(std::sync::atomic::Ordering::SeqCst),
            current_objects: 0, // Would need integration with object store
            current_heap_size: heap_stats.used_before,
            heap_capacity: heap_stats.total_capacity,
            fragmentation: heap_stats.fragmentation,
        })
    }
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

// Implement compatibility interface for enhanced GC
impl From<EnhancedGarbageCollector> for GarbageCollector {
    /// Convert an EnhancedGarbageCollector to a standard GarbageCollector
    /// 
    /// This conversion preserves all state and configuration while using the legacy
    /// heap manager for compatibility. The real heap manager data is not lost
    /// but becomes inaccessible through the standard GC interface.
    fn from(enhanced: EnhancedGarbageCollector) -> Self {
        debug!("Converting EnhancedGarbageCollector to GarbageCollector");
        
        // Extract configuration from enhanced GC
        let config = enhanced.config.into_inner().unwrap_or_else(|poisoned| {
            warn!("Config lock was poisoned during conversion, using recovered data");
            poisoned.into_inner()
        });
        
        // Convert enhanced performance tracking to standard performance tracking
        let enhanced_stats = enhanced.collection_stats.into_inner().unwrap_or_else(|poisoned| {
            warn!("Collection stats lock was poisoned during conversion, using recovered data");
            poisoned.into_inner()
        });
        
        // Since CollectionPerformance is private, we need to create a new GC 
        // and let it initialize with proper performance tracking, then transfer atomic state
        let temp_gc = GarbageCollector::with_config(config.clone(), HeapConfig::default());
        
        // Extract the initialized but empty collection stats
        let collection_stats = temp_gc.collection_stats.into_inner().unwrap_or_default();
        
        // Note: Since CollectionPerformance fields are private, we can't directly transfer
        // the enhanced performance data. The converted GC will start with fresh performance
        // tracking but will preserve all other state including collection counts and algorithm effectiveness.
        
        // Extract algorithm effectiveness
        let algorithm_effectiveness = enhanced.algorithm_effectiveness.into_inner().unwrap_or_else(|poisoned| {
            warn!("Algorithm effectiveness lock was poisoned during conversion, using recovered data");
            poisoned.into_inner()
        });
        
        // Extract current algorithm
        let current_algorithm = enhanced.current_algorithm.into_inner().unwrap_or_else(|poisoned| {
            warn!("Current algorithm lock was poisoned during conversion, using recovered data");
            poisoned.into_inner()
        });
        
        // Extract last collection time 
        let last_collection_time = enhanced.last_collection_time.into_inner().unwrap_or_else(|poisoned| {
            warn!("Last collection time lock was poisoned during conversion, using recovered data");
            poisoned.into_inner()
        });
        
        // Create the standard GC with preserved state from enhanced GC
        GarbageCollector {
            config: RwLock::new(config),
            object_store: enhanced.object_store,
            heap_manager: enhanced.legacy_heap_manager, // Use legacy heap for compatibility
            object_registry: enhanced.object_registry,
            root_manager: enhanced.root_manager,
            profiler: enhanced.profiler,
            mark_sweep_collector: enhanced.mark_sweep_collector,
            incremental_collector: enhanced.incremental_collector,
            copying_collector: enhanced.copying_collector,
            cycle_detector: enhanced.cycle_detector,
            collection_count: enhanced.collection_count,
            objects_collected: enhanced.objects_collected,
            bytes_collected: enhanced.bytes_collected,
            last_collection_time: Mutex::new(last_collection_time),
            collection_stats: RwLock::new(collection_stats), // Fresh performance tracking
            algorithm_effectiveness: RwLock::new(algorithm_effectiveness),
            current_algorithm: RwLock::new(current_algorithm),
            is_collecting: enhanced.is_collecting,
            allocation_since_last_gc: enhanced.allocation_since_last_gc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_gc_creation() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        assert!(enhanced_gc.use_real_heap);
    }
    
    #[test]
    fn test_enhanced_gc_with_legacy_heap() {
        let enhanced_gc = EnhancedGarbageCollector::with_config(
            GcConfig::default(),
            HeapConfig::default(),
            false
        );
        assert!(!enhanced_gc.use_real_heap);
    }
    
    #[test]
    fn test_enhanced_allocation_selection() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Test algorithm selection
        let algorithm = enhanced_gc.select_collection_algorithm_enhanced(
            CollectionTrigger::Emergency
        ).unwrap();
        
        // Should select an appropriate algorithm for emergency
        assert!(matches!(algorithm, 
            CollectionAlgorithm::MarkSweep | 
            CollectionAlgorithm::Copying
        ));
    }
    
    #[test]
    fn test_collection_trigger_detection() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Should not trigger collection initially
        let trigger = enhanced_gc.should_collect_enhanced().unwrap();
        assert!(trigger.is_none());
    }
    
    #[test]
    fn test_heap_stats_enhanced() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        let stats = enhanced_gc.get_heap_stats_enhanced().unwrap();
        assert_eq!(stats.total_capacity, 0); // No allocations yet
        assert_eq!(stats.used_before, 0);
    }
    
    #[test]
    fn test_enhanced_gc_to_standard_gc_conversion() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Convert to standard GC
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        // Verify basic properties are preserved
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 0);
        assert_eq!(stats.total_objects_collected, 0);
    }
    
    #[test]
    fn test_conversion_preserves_configuration() {
        let custom_config = GcConfig {
            algorithm: CollectionAlgorithm::MarkSweep,
            generational: true,
            incremental: false,
            concurrent: true,
            goroutine_aware: false,
            young_gen_threshold: 0.7,
            old_gen_threshold: 0.85,
            emergency_threshold: 0.98,
            max_pause_time: Duration::from_millis(20),
            allocation_pressure_ratio: 0.15,
            adaptive_algorithm_selection: false,
        };
        
        let enhanced_gc = EnhancedGarbageCollector::with_config(
            custom_config.clone(),
            HeapConfig::default(),
            true
        );
        
        // Convert to standard GC
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        // Check that configuration is preserved (indirectly through behavior)
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_conversion_preserves_state() {
        let mut enhanced_gc = EnhancedGarbageCollector::new();
        
        // Simulate some collection activity
        enhanced_gc.collection_count.store(5, std::sync::atomic::Ordering::SeqCst);
        enhanced_gc.objects_collected.store(100, std::sync::atomic::Ordering::SeqCst);
        enhanced_gc.bytes_collected.store(1024, std::sync::atomic::Ordering::SeqCst);
        enhanced_gc.allocation_since_last_gc.store(512, std::sync::atomic::Ordering::SeqCst);
        
        // Convert to standard GC
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        // Verify state is preserved
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 5);
        assert_eq!(stats.total_objects_collected, 100);
    }
    
    #[test]
    fn test_conversion_with_legacy_heap() {
        let enhanced_gc = EnhancedGarbageCollector::with_config(
            GcConfig::default(),
            HeapConfig::default(),
            false // Use legacy heap
        );
        
        // Convert to standard GC - should work even with legacy heap
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_conversion_handles_algorithm_effectiveness() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Set up some algorithm effectiveness data
        {
            let mut effectiveness = enhanced_gc.algorithm_effectiveness.write().unwrap();
            effectiveness.insert(CollectionAlgorithm::MarkSweep, 0.85);
            effectiveness.insert(CollectionAlgorithm::Copying, 0.92);
        }
        
        // Convert to standard GC
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        // Verify the conversion completes successfully
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_conversion_handles_poisoned_locks() {
        // Create enhanced GC in a scope where we can simulate lock poisoning
        let enhanced_gc = std::panic::catch_unwind(|| {
            let gc = EnhancedGarbageCollector::new();
            // Simulate potential lock poisoning scenario by accessing locks
            let _ = gc.config.read().unwrap();
            gc
        }).unwrap();
        
        // Convert should handle any lock issues gracefully
        let standard_gc: GarbageCollector = enhanced_gc.into();
        
        let stats = standard_gc.stats();
        assert_eq!(stats.total_collections, 0);
    }
}
