/// Production-Ready Garbage Collector for CURSED
/// 
/// This module provides a comprehensive production-ready garbage collection system
/// that integrates all the existing GC components into a cohesive, high-performance
/// memory management solution. This is the main entry point for GC operations.
/// 
/// Key features:
/// - Real allocation/deallocation with multiple algorithms
/// - Generational and incremental collection
/// - Memory pressure detection and automatic collection
/// - Integration with goroutine system for thread safety
/// - Comprehensive monitoring and statistics
/// - Production-grade error handling and recovery

use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::time::{Duration, Instant, SystemTime};
use std::collections::HashMap;
use std::ptr::NonNull;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{
    gc::{GarbageCollector, GcConfig, EnhancedCollectionStats, CollectionTrigger, CollectionAlgorithm},
    heap::{Heap, HeapConfiguration},
    heap_manager::{HeapManager, HeapConfig, HeapStats},
    allocator::{Allocator, AllocationResult},
    pressure_detection::{MemoryPressureDetector, PressureDetectionConfig, PressureLevel},
    mark_sweep::{MarkSweepCollector, MarkSweepConfig},
    incremental::{IncrementalCollector, IncrementalConfig},
    copying::{CopyingCollector, CopyingConfig},
    generational::{GenerationalCollector, GenerationalConfig},
    object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry},
    object_store::{ObjectStore, Storable},
    roots::{RootSetManager},
};
use crate::error::Error;
use crate::profiling::memory::MemoryProfiler;

/// Production-ready garbage collector configuration
#[derive(Debug, Clone)]
pub struct ProductionGcConfig {
    /// Base GC configuration
    pub gc_config: GcConfig,
    /// Heap configuration
    pub heap_config: HeapConfig,
    /// Pressure detection configuration
    pub pressure_config: PressureDetectionConfig,
    /// Memory profiling enabled
    pub enable_profiling: bool,
    /// Automatic collection enabled
    pub enable_auto_collection: bool,
    /// Collection thread count
    pub collection_threads: usize,
    /// Maximum heap size in bytes
    pub max_heap_size: usize,
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    /// Emergency collection threshold
    pub emergency_threshold: f64,
    /// Background collection interval
    pub background_collection_interval: Duration,
    /// Enable statistics tracking
    pub enable_statistics: bool,
    /// Enable goroutine-aware collection
    pub enable_goroutine_awareness: bool,
}

impl Default for ProductionGcConfig {
    fn default() -> Self {
        Self {
            gc_config: GcConfig::default(),
            heap_config: HeapConfig::default(),
            pressure_config: PressureDetectionConfig::default(),
            enable_profiling: true,
            enable_auto_collection: true,
            collection_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            max_heap_size: 1024 * 1024 * 1024, // 1GB
            initial_heap_size: 64 * 1024 * 1024, // 64MB
            emergency_threshold: 0.95,
            background_collection_interval: Duration::from_millis(500),
            enable_statistics: true,
            enable_goroutine_awareness: true,
        }
    }
}

/// Production garbage collector with comprehensive memory management
/// 
/// This is the main production garbage collector that provides:
/// - Multiple collection algorithms with automatic selection
/// - Real memory allocation and deallocation
/// - Memory pressure monitoring and automatic collection
/// - Thread-safe operations for concurrent execution
/// - Comprehensive monitoring and debugging support
pub struct ProductionGarbageCollector {
    /// Main garbage collector engine
    gc: Arc<GarbageCollector>,
    /// Heap manager for low-level memory operations
    heap_manager: Arc<RwLock<HeapManager>>,
    /// Memory pressure detector
    pressure_detector: Arc<MemoryPressureDetector>,
    /// Memory profiler (optional)
    profiler: Option<Arc<MemoryProfiler>>,
    /// Configuration
    config: Arc<RwLock<ProductionGcConfig>>,
    /// Object registry
    object_registry: SharedObjectRegistry,
    /// Background collection thread handle
    background_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
    /// Collection statistics
    stats: Arc<Mutex<ProductionGcStats>>,
    /// Auto collection enabled flag
    auto_collection_enabled: AtomicBool,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Total allocations
    total_allocations: AtomicU64,
    /// Total deallocations
    total_deallocations: AtomicU64,
    /// Total collections
    total_collections: AtomicU64,
    /// Last collection time
    last_collection: Mutex<Option<Instant>>,
    /// Creation time
    creation_time: SystemTime,
}

/// Comprehensive statistics for production GC
#[derive(Debug, Clone)]
pub struct ProductionGcStats {
    /// Total runtime in seconds
    pub runtime_seconds: f64,
    /// Total allocations performed
    pub total_allocations: u64,
    /// Total deallocations performed
    pub total_deallocations: u64,
    /// Total garbage collections
    pub total_collections: u64,
    /// Total objects collected
    pub total_objects_collected: u64,
    /// Total bytes collected
    pub total_bytes_collected: u64,
    /// Current heap size
    pub current_heap_size: usize,
    /// Peak heap size
    pub peak_heap_size: usize,
    /// Current memory pressure
    pub current_pressure: PressureLevel,
    /// Average collection time
    pub average_collection_time: Duration,
    /// Total collection time
    pub total_collection_time: Duration,
    /// Collection algorithm usage
    pub algorithm_usage: HashMap<CollectionAlgorithm, u64>,
    /// Memory efficiency (allocation rate vs collection rate)
    pub memory_efficiency: f64,
    /// Fragmentation ratio
    pub fragmentation_ratio: f64,
    /// Auto collection triggers
    pub auto_collection_triggers: u64,
    /// Manual collection triggers
    pub manual_collection_triggers: u64,
    /// Emergency collection triggers
    pub emergency_collection_triggers: u64,
    /// Failed allocations
    pub failed_allocations: u64,
    /// Goroutine-aware collections
    pub goroutine_aware_collections: u64,
}

impl Default for ProductionGcStats {
    fn default() -> Self {
        Self {
            runtime_seconds: 0.0,
            total_allocations: 0,
            total_deallocations: 0,
            total_collections: 0,
            total_objects_collected: 0,
            total_bytes_collected: 0,
            current_heap_size: 0,
            peak_heap_size: 0,
            current_pressure: PressureLevel::None,
            average_collection_time: Duration::ZERO,
            total_collection_time: Duration::ZERO,
            algorithm_usage: HashMap::new(),
            memory_efficiency: 1.0,
            fragmentation_ratio: 0.0,
            auto_collection_triggers: 0,
            manual_collection_triggers: 0,
            emergency_collection_triggers: 0,
            failed_allocations: 0,
            goroutine_aware_collections: 0,
        }
    }
}

impl ProductionGarbageCollector {
    /// Create a new production garbage collector
    #[instrument]
    pub fn new(config: ProductionGcConfig) -> Result<Self, String> {
        info!("Creating production garbage collector");
        
        // Create object registry
        let object_registry = Arc::new(ObjectRegistry::new());
        
        // Create heap manager
        let heap_manager = Arc::new(RwLock::new(
            HeapManager::new(config.heap_config.clone(), object_registry.clone())
        ));
        
        // Create main garbage collector
        let gc = Arc::new(GarbageCollector::with_config(
            config.gc_config.clone(),
            config.heap_config.clone()
        ));
        
        // Create pressure detector
        let pressure_detector = Arc::new(MemoryPressureDetector::new(config.pressure_config.clone()));
        
        // Initialize profiler if enabled
        let profiler = if config.enable_profiling {
            let p = Arc::new(MemoryProfiler::new(1024)); // 1KB tracking threshold
            // Set profiler in heap manager
            if let Ok(mut heap) = heap_manager.write() {
                heap.set_profiler(p.clone());
            }
            Some(p)
        } else {
            None
        };
        
        let creation_time = SystemTime::now();
        let shutdown = Arc::new(AtomicBool::new(false));
        
        let collector = Self {
            gc,
            heap_manager,
            pressure_detector,
            profiler,
            config: Arc::new(RwLock::new(config.clone())),
            object_registry,
            background_thread: Mutex::new(None),
            stats: Arc::new(Mutex::new(ProductionGcStats::default())),
            auto_collection_enabled: AtomicBool::new(config.enable_auto_collection),
            shutdown: shutdown.clone(),
            total_allocations: AtomicU64::new(0),
            total_deallocations: AtomicU64::new(0),
            total_collections: AtomicU64::new(0),
            last_collection: Mutex::new(None),
            creation_time,
        };
        
        // Start background collection thread if auto collection is enabled
        if config.enable_auto_collection {
            collector.start_background_collection()?;
        }
        
        info!("Production garbage collector created successfully");
        Ok(collector)
    }
    
    /// Allocate an object in the managed heap
    #[instrument(skip(self, obj))]
    pub fn allocate<T>(&self, obj: T) -> Result<crate::memory::gc::Gc<T>, String>
    where
        T: Storable + Clone,
    {
        let allocation_size = std::mem::size_of::<T>();
        debug!("Allocating object of size {} bytes", allocation_size);
        
        // Check if we should trigger collection before allocation
        if self.should_collect_before_allocation(allocation_size)? {
            info!("Triggering preemptive collection before allocation");
            self.collect_with_trigger(CollectionTrigger::AllocationPressure)?;
        }
        
        // Clone the object before first allocation attempt for potential retry
        let obj_for_retry = obj.clone();
        
        // Perform the allocation
        let result = self.gc.allocate(obj);
        
        match result {
            Ok(gc_ptr) => {
                // Update statistics
                self.total_allocations.fetch_add(1, Ordering::Relaxed);
                
                // Notify pressure detector
                if let Ok(heap_stats) = self.get_heap_stats() {
                    let _ = self.pressure_detector.detect_pressure(&heap_stats, None);
                }
                
                // Profile the allocation
                if let Some(profiler) = &self.profiler {
                    let _ = profiler.track_allocation(
                        allocation_size,
                        gc_ptr.object_id().as_u64(),
                        vec![]
                    );
                }
                
                debug!("Successfully allocated object {}", gc_ptr.object_id());
                Ok(gc_ptr)
            }
            Err(e) => {
                warn!("Allocation failed: {}", e);
                
                // Try emergency collection
                if let Err(collection_err) = self.collect_with_trigger(CollectionTrigger::Emergency) {
                    error!("Emergency collection failed: {}", collection_err);
                }
                
                // Retry allocation once
                let retry_result = self.gc.allocate(obj_for_retry);
                if retry_result.is_ok() {
                    info!("Allocation succeeded after emergency collection");
                    self.total_allocations.fetch_add(1, Ordering::Relaxed);
                }
                
                retry_result.map_err(|retry_err| {
                    // Update failed allocation statistics
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.failed_allocations += 1;
                    }
                    format!("Allocation failed even after emergency collection: {}", retry_err)
                })
            }
        }
    }
    
    /// Trigger garbage collection
    #[instrument(skip(self))]
    pub fn collect(&self) -> Result<EnhancedCollectionStats, String> {
        self.collect_with_trigger(CollectionTrigger::Manual)
    }
    
    /// Trigger garbage collection with specific trigger
    #[instrument(skip(self))]
    pub fn collect_with_trigger(&self, trigger: CollectionTrigger) -> Result<EnhancedCollectionStats, String> {
        info!("Starting garbage collection with trigger: {:?}", trigger);
        let start_time = Instant::now();
        
        // Check if goroutine-aware collection should be used
        let result = if self.should_use_goroutine_aware_collection() {
            self.gc.collect_garbage_with_goroutine_awareness()
        } else {
            self.gc.collect_with_trigger(trigger)
        };
        
        match result {
            Ok(stats) => {
                let collection_duration = start_time.elapsed();
                
                // Update collection statistics
                self.total_collections.fetch_add(1, Ordering::Relaxed);
                *self.last_collection.lock().unwrap() = Some(Instant::now());
                
                // Update detailed statistics
                if let Ok(mut production_stats) = self.stats.lock() {
                    production_stats.total_collections += 1;
                    production_stats.total_objects_collected += stats.objects_collected as u64;
                    production_stats.total_bytes_collected += stats.bytes_collected as u64;
                    production_stats.total_collection_time += collection_duration;
                    production_stats.average_collection_time = 
                        production_stats.total_collection_time / production_stats.total_collections as u32;
                    
                    // Update algorithm usage
                    *production_stats.algorithm_usage.entry(stats.algorithm_used).or_insert(0) += 1;
                    
                    // Update trigger statistics
                    match trigger {
                        CollectionTrigger::Manual => production_stats.manual_collection_triggers += 1,
                        CollectionTrigger::Emergency => production_stats.emergency_collection_triggers += 1,
                        _ => production_stats.auto_collection_triggers += 1,
                    }
                    
                    if self.gc.should_use_goroutine_aware_collection() {
                        production_stats.goroutine_aware_collections += 1;
                    }
                }
                
                // Profile the collection
                if let Some(profiler) = &self.profiler {
                    let _ = profiler.track_gc_event(crate::profiling::memory::GcEvent {
                        gc_type: crate::profiling::memory::GcType::Major,
                        duration: collection_duration,
                        bytes_collected: stats.bytes_collected,
                        bytes_remaining: stats.heap_stats.used_after,
                        objects_collected: stats.objects_collected as u64,
                        objects_remaining: (stats.heap_stats.used_after / 64) as u64,
                        timestamp: std::time::Instant::now(),
                        trigger_reason: "automatic".to_string(),
                    });
                }
                
                info!(
                    "Garbage collection completed: {} objects, {} bytes collected in {:?}",
                    stats.objects_collected,
                    stats.bytes_collected,
                    collection_duration
                );
                
                Ok(stats)
            }
            Err(e) => {
                error!("Garbage collection failed: {}", e);
                Err(e)
            }
        }
    }
    
    /// Check if collection should be triggered before allocation
    fn should_collect_before_allocation(&self, allocation_size: usize) -> Result<bool, String> {
        // Check memory pressure
        let heap_stats = self.get_heap_stats()?;
        let pressure = self.pressure_detector.detect_pressure(&heap_stats, None)?;
        
        // Trigger collection for high pressure
        if pressure >= PressureLevel::High {
            return Ok(true);
        }
        
        // Check heap utilization
        let config = self.config.read().map_err(|_| "Failed to read config")?;
        let utilization = if heap_stats.total_capacity > 0 {
            heap_stats.total_used as f64 / heap_stats.total_capacity as f64
        } else {
            0.0
        };
        
        if utilization > config.emergency_threshold {
            return Ok(true);
        }
        
        // Check if allocation would exceed heap capacity
        if heap_stats.total_used + allocation_size > heap_stats.total_capacity {
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Check if goroutine-aware collection should be used
    fn should_use_goroutine_aware_collection(&self) -> bool {
        if let Ok(config) = self.config.read() {
            config.enable_goroutine_awareness && self.gc.should_use_goroutine_aware_collection()
        } else {
            false
        }
    }
    
    /// Start background collection thread
    fn start_background_collection(&self) -> Result<(), String> {
        let gc = self.gc.clone();
        let pressure_detector = self.pressure_detector.clone();
        let heap_manager = self.heap_manager.clone();
        let shutdown = self.shutdown.clone();
        let config = self.config.clone();
        let stats = self.stats.clone();
        
        let handle = std::thread::Builder::new()
            .name("gc-background".to_string())
            .spawn(move || {
                info!("Background garbage collection thread started");
                
                while !shutdown.load(Ordering::Acquire) {
                    // Get collection interval from config
                    let interval = if let Ok(cfg) = config.read() {
                        cfg.background_collection_interval
                    } else {
                        Duration::from_millis(500)
                    };
                    
                    std::thread::sleep(interval);
                    
                    // Check if collection is needed
                    if let Ok(heap_stats) = Self::get_heap_stats_static(&heap_manager) {
                        if let Ok(pressure) = pressure_detector.detect_pressure(&heap_stats, None) {
                            if pressure >= PressureLevel::Moderate {
                                info!("Background collection triggered due to {} pressure", pressure);
                                
                                let trigger = match pressure {
                                    PressureLevel::Emergency | PressureLevel::Critical => CollectionTrigger::Emergency,
                                    PressureLevel::High => CollectionTrigger::HeapUtilization,
                                    _ => CollectionTrigger::Periodic,
                                };
                                
                                if let Err(e) = gc.collect_with_trigger(trigger) {
                                    warn!("Background collection failed: {}", e);
                                }
                            }
                        }
                    }
                }
                
                info!("Background garbage collection thread stopped");
            })
            .map_err(|e| format!("Failed to start background collection thread: {}", e))?;
        
        *self.background_thread.lock().unwrap() = Some(handle);
        Ok(())
    }
    
    /// Get heap statistics
    fn get_heap_stats(&self) -> Result<HeapStats, String> {
        Self::get_heap_stats_static(&self.heap_manager)
    }
    
    /// Static version of get_heap_stats for background thread
    fn get_heap_stats_static(heap_manager: &Arc<RwLock<HeapManager>>) -> Result<HeapStats, String> {
        let heap = heap_manager.read().map_err(|_| "Failed to read heap manager")?;
        heap.get_stats()
    }
    
    /// Get comprehensive production statistics
    pub fn get_stats(&self) -> Result<ProductionGcStats, String> {
        let mut stats = self.stats.lock().map_err(|_| "Failed to lock stats")?;
        
        // Update runtime
        if let Ok(elapsed) = self.creation_time.elapsed() {
            stats.runtime_seconds = elapsed.as_secs_f64();
        }
        
        // Update current values
        stats.total_allocations = self.total_allocations.load(Ordering::Relaxed);
        stats.total_deallocations = self.total_deallocations.load(Ordering::Relaxed);
        stats.total_collections = self.total_collections.load(Ordering::Relaxed);
        
        // Update heap statistics
        if let Ok(heap_stats) = self.get_heap_stats() {
            stats.current_heap_size = heap_stats.total_used;
            if heap_stats.total_used > stats.peak_heap_size {
                stats.peak_heap_size = heap_stats.total_used;
            }
            stats.fragmentation_ratio = heap_stats.fragmentation_ratio;
        }
        
        // Update pressure
        if let Ok(pressure) = self.pressure_detector.current_pressure() {
            stats.current_pressure = pressure;
        }
        
        // Calculate memory efficiency
        if stats.total_allocations > 0 {
            stats.memory_efficiency = 
                (stats.total_allocations - stats.failed_allocations) as f64 / stats.total_allocations as f64;
        }
        
        Ok(stats.clone())
    }
    
    /// Enable or disable auto collection
    pub fn set_auto_collection(&self, enabled: bool) -> Result<(), String> {
        self.auto_collection_enabled.store(enabled, Ordering::Release);
        
        if enabled && self.background_thread.lock().unwrap().is_none() {
            self.start_background_collection()?;
        }
        
        info!("Auto collection {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: ProductionGcConfig) -> Result<(), String> {
        {
            let mut config = self.config.write().map_err(|_| "Failed to write config")?;
            *config = new_config.clone();
        }
        
        // Update sub-component configurations
        self.gc.update_config(new_config.gc_config)?;
        self.pressure_detector.update_config(new_config.pressure_config)?;
        
        info!("Production GC configuration updated");
        Ok(())
    }
    
    /// Force a full garbage collection cycle
    pub fn force_full_collection(&self) -> Result<EnhancedCollectionStats, String> {
        info!("Forcing full garbage collection");
        
        // Disable auto collection temporarily
        let auto_enabled = self.auto_collection_enabled.load(Ordering::Acquire);
        self.auto_collection_enabled.store(false, Ordering::Release);
        
        let result = self.collect_with_trigger(CollectionTrigger::Manual);
        
        // Restore auto collection setting
        self.auto_collection_enabled.store(auto_enabled, Ordering::Release);
        
        result
    }
    
    /// Get memory profiler if enabled
    pub fn get_profiler(&self) -> Option<Arc<MemoryProfiler>> {
        self.profiler.clone()
    }
    
    /// Check current memory pressure
    pub fn current_memory_pressure(&self) -> Result<PressureLevel, String> {
        let heap_stats = self.get_heap_stats()?;
        self.pressure_detector.detect_pressure(&heap_stats, None)
    }
    
    /// Get the underlying garbage collector for advanced operations
    pub fn gc(&self) -> &Arc<GarbageCollector> {
        &self.gc
    }
    
    /// Get object registry
    pub fn object_registry(&self) -> &SharedObjectRegistry {
        &self.object_registry
    }
}

impl Drop for ProductionGarbageCollector {
    fn drop(&mut self) {
        info!("Shutting down production garbage collector");
        
        // Signal shutdown
        self.shutdown.store(true, Ordering::Release);
        
        // Wait for background thread to finish
        if let Ok(mut thread_handle) = self.background_thread.lock() {
            if let Some(handle) = thread_handle.take() {
                if let Err(e) = handle.join() {
                    error!("Background collection thread panicked: {:?}", e);
                }
            }
        }
        
        // Perform final collection
        if let Err(e) = self.collect_with_trigger(CollectionTrigger::Manual) {
            warn!("Final garbage collection failed: {}", e);
        }
        
        info!("Production garbage collector shutdown complete");
    }
}

// Safety: ProductionGarbageCollector is safe to send between threads because:
// 1. All components are either Arc<> wrapped or atomic
// 2. The GarbageCollector itself is Send + Sync
// 3. Background thread coordination is handled properly
unsafe impl Send for ProductionGarbageCollector {}

// Safety: ProductionGarbageCollector is safe to share between threads because:
// 1. All operations are coordinated through internal locks
// 2. Arc<> provides shared ownership semantics
// 3. Atomic counters handle concurrent access safely
unsafe impl Sync for ProductionGarbageCollector {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_production_gc_creation() {
        let config = ProductionGcConfig::default();
        let gc = ProductionGarbageCollector::new(config).unwrap();
        
        let stats = gc.get_stats().unwrap();
        assert_eq!(stats.total_allocations, 0);
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_memory_pressure_detection() {
        let config = ProductionGcConfig::default();
        let gc = ProductionGarbageCollector::new(config).unwrap();
        
        let pressure = gc.current_memory_pressure().unwrap();
        assert_eq!(pressure, PressureLevel::None);
    }
    
    #[test]
    fn test_configuration_update() {
        let config = ProductionGcConfig::default();
        let gc = ProductionGarbageCollector::new(config).unwrap();
        
        let mut new_config = ProductionGcConfig::default();
        new_config.max_heap_size = 2048 * 1024 * 1024; // 2GB
        
        gc.update_config(new_config).unwrap();
    }
    
    #[test]
    fn test_auto_collection_toggle() {
        let mut config = ProductionGcConfig::default();
        config.enable_auto_collection = false;
        let gc = ProductionGarbageCollector::new(config).unwrap();
        
        gc.set_auto_collection(true).unwrap();
        gc.set_auto_collection(false).unwrap();
    }
}
