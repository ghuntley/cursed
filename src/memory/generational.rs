/// Generational Garbage Collection System
/// 
/// This module provides a comprehensive generational garbage collection system
/// that combines different collection strategies for optimal performance across
/// different object lifetimes and allocation patterns.

use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::memory::heap_manager::{HeapManager, HeapStats};
use crate::memory::roots::{RootSetManager, RootType};
use crate::memory::collection_triggers::{CollectionTriggerManager, TriggerType, TriggerReason};
use crate::memory::cycle_detection::{CycleDetector, CycleDetectionConfig};
use crate::memory::mark_sweep::{MarkSweepCollector, MarkSweepConfig};
use crate::memory::copying::{CopyingCollector, CopyingConfig};
use crate::memory::incremental::{IncrementalCollector, IncrementalConfig};

/// Generation types in the generational collector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Generation {
    /// Young generation for newly allocated objects
    Young,
    /// Old generation for long-lived objects
    Old,
    /// Permanent generation for class metadata (optional)
    Permanent,
}

/// Collection strategy for different scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionStrategy {
    /// Young generation collection only
    YoungOnly,
    /// Old generation collection only
    OldOnly,
    /// Full collection of all generations
    Full,
    /// Incremental collection
    Incremental,
    /// Emergency collection when memory is critically low
    Emergency,
}

/// Configuration for generational collection
#[derive(Debug, Clone)]
pub struct GenerationalConfig {
    /// Young generation configuration
    pub young_config: CopyingConfig,
    /// Old generation configuration
    pub old_config: MarkSweepConfig,
    /// Incremental collection configuration
    pub incremental_config: IncrementalConfig,
    /// Cycle detection configuration
    pub cycle_detection_config: CycleDetectionConfig,
    /// Young generation size as fraction of total heap
    pub young_generation_ratio: f64,
    /// Object age threshold for promotion
    pub promotion_age_threshold: u8,
    /// Enable adaptive generation sizing
    pub adaptive_sizing: bool,
    /// Enable concurrent collection
    pub concurrent_collection: bool,
    /// Enable incremental collection
    pub incremental_collection: bool,
    /// Enable cycle detection
    pub cycle_detection: bool,
    /// Write barrier overhead threshold
    pub write_barrier_threshold: f64,
}

impl Default for GenerationalConfig {
    fn default() -> Self {
        Self {
            young_config: CopyingConfig::default(),
            old_config: MarkSweepConfig::default(),
            incremental_config: IncrementalConfig::default(),
            cycle_detection_config: CycleDetectionConfig::default(),
            young_generation_ratio: 0.33, // 1/3 of heap for young generation
            promotion_age_threshold: 3,
            adaptive_sizing: true,
            concurrent_collection: false,
            incremental_collection: true,
            cycle_detection: true,
            write_barrier_threshold: 0.05, // 5% overhead
        }
    }
}

/// Statistics from generational collection
#[derive(Debug, Clone)]
pub struct GenerationalStats {
    pub total_collections: u64,
    pub young_collections: u64,
    pub old_collections: u64,
    pub full_collections: u64,
    pub incremental_collections: u64,
    pub total_collection_time: Duration,
    pub young_collection_time: Duration,
    pub old_collection_time: Duration,
    pub objects_promoted: u64,
    pub bytes_promoted: u64,
    pub promotion_rate: f64,
    pub young_generation_size: usize,
    pub old_generation_size: usize,
    pub heap_utilization: f64,
    pub collection_efficiency: f64, // bytes reclaimed per collection time
    pub write_barrier_overhead: f64,
    pub cycles_detected: u64,
    pub cycles_collected: u64,
}

/// Object generation tracking
#[derive(Debug, Clone)]
struct ObjectGenerationInfo {
    generation: Generation,
    age: u8,
    promotion_candidate: bool,
    size: usize,
    allocated_at: Instant,
}

/// Cross-generational reference tracking
#[derive(Debug, Clone)]
struct CrossGenerationalReference {
    from_object: ObjectId,
    from_generation: Generation,
    to_object: ObjectId,
    to_generation: Generation,
    field_offset: usize,
}

/// Main generational garbage collector
pub struct GenerationalCollector {
    config: RwLock<GenerationalConfig>,
    
    /// Core components
    object_registry: SharedObjectRegistry,
    heap_manager: Option<Arc<RwLock<HeapManager>>>,
    root_manager: Arc<RootSetManager>,
    trigger_manager: Arc<CollectionTriggerManager>,
    
    /// Generation-specific collectors
    young_collector: Arc<CopyingCollector>,
    old_collector: Arc<MarkSweepCollector>,
    incremental_collector: Arc<IncrementalCollector>,
    cycle_detector: Arc<CycleDetector>,
    
    /// Object tracking
    object_generations: RwLock<HashMap<ObjectId, ObjectGenerationInfo>>,
    cross_gen_references: RwLock<Vec<CrossGenerationalReference>>,
    
    /// Statistics and monitoring
    stats: RwLock<GenerationalStats>,
    collection_counter: std::sync::atomic::AtomicU64,
    last_collection_time: Mutex<Option<Instant>>,
    
    /// Background collection thread
    background_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
    should_stop: std::sync::atomic::AtomicBool,
}

impl GenerationalCollector {
    /// Create a new generational collector
    pub fn new(object_registry: SharedObjectRegistry) -> Result<Self, String> {
        Self::with_config(object_registry, GenerationalConfig::default())
    }
    
    /// Create a new generational collector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: GenerationalConfig) -> Result<Self, String> {
        info!("Creating generational collector with config: {:?}", config);
        
        // Create root manager
        let root_manager = Arc::new(RootSetManager::new(object_registry.clone()));
        
        // Create trigger manager
        let trigger_manager = Arc::new(CollectionTriggerManager::new());
        
        // Create generation-specific collectors
        let young_collector = Arc::new(CopyingCollector::with_config(
            object_registry.clone(),
            config.young_config.clone(),
        )?);
        
        let old_collector = Arc::new(MarkSweepCollector::with_config(
            object_registry.clone(),
            config.old_config.clone(),
        ));
        
        let incremental_collector = Arc::new(IncrementalCollector::with_config(
            object_registry.clone(),
            config.incremental_config.clone(),
        ));
        
        let cycle_detector = Arc::new(CycleDetector::with_config(
            object_registry.clone(),
            config.cycle_detection_config.clone(),
        ));
        
        Ok(Self {
            config: RwLock::new(config),
            object_registry,
            heap_manager: None,
            root_manager,
            trigger_manager,
            young_collector,
            old_collector,
            incremental_collector,
            cycle_detector,
            object_generations: RwLock::new(HashMap::new()),
            cross_gen_references: RwLock::new(Vec::new()),
            stats: RwLock::new(GenerationalStats {
                total_collections: 0,
                young_collections: 0,
                old_collections: 0,
                full_collections: 0,
                incremental_collections: 0,
                total_collection_time: Duration::ZERO,
                young_collection_time: Duration::ZERO,
                old_collection_time: Duration::ZERO,
                objects_promoted: 0,
                bytes_promoted: 0,
                promotion_rate: 0.0,
                young_generation_size: 0,
                old_generation_size: 0,
                heap_utilization: 0.0,
                collection_efficiency: 0.0,
                write_barrier_overhead: 0.0,
                cycles_detected: 0,
                cycles_collected: 0,
            }),
            collection_counter: std::sync::atomic::AtomicU64::new(0),
            last_collection_time: Mutex::new(None),
            background_thread: Mutex::new(None),
            should_stop: std::sync::atomic::AtomicBool::new(false),
        })
    }
    
    /// Set heap manager
    pub fn set_heap_manager(&mut self, heap_manager: Arc<RwLock<HeapManager>>) {
        self.heap_manager = Some(heap_manager);
    }
    
    /// Allocate a new object
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize, align: usize) -> Result<Option<std::ptr::NonNull<u8>>, String> {
        debug!("Allocating object of size {} bytes", size);
        
        // Try allocating in young generation first
        if let Some(ptr) = self.young_collector.allocate(size, align)? {
            // Track allocation in young generation
            // TODO: Create ObjectId and track generation info
            debug!("Allocated in young generation");
            return Ok(Some(ptr));
        }
        
        // Young generation is full, trigger collection
        if self.should_collect_young()? {
            self.collect_young_generation()?;
            
            // Try allocating again after collection
            if let Some(ptr) = self.young_collector.allocate(size, align)? {
                debug!("Allocated in young generation after collection");
                return Ok(Some(ptr));
            }
        }
        
        // Allocate in old generation for large objects or when young gen is full
        debug!("Allocating in old generation");
        self.allocate_in_old_generation(size, align)
    }
    
    /// Allocate in old generation
    fn allocate_in_old_generation(&self, size: usize, align: usize) -> Result<Option<std::ptr::NonNull<u8>>, String> {
        // TODO: Implement old generation allocation
        // For now, return None to indicate allocation failed
        debug!("Old generation allocation not yet implemented");
        Ok(None)
    }
    
    /// Perform collection based on trigger analysis
    #[instrument(skip(self))]
    pub fn collect(&self) -> Result<GenerationalStats, String> {
        info!("Starting generational collection");
        
        // Check what type of collection is needed
        let collection_strategy = self.determine_collection_strategy()?;
        
        let collection_start = Instant::now();
        let collection_number = self.collection_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        
        let stats = match collection_strategy {
            CollectionStrategy::YoungOnly => self.collect_young_generation()?,
            CollectionStrategy::OldOnly => self.collect_old_generation()?,
            CollectionStrategy::Full => self.collect_full()?,
            CollectionStrategy::Incremental => self.collect_incremental()?,
            CollectionStrategy::Emergency => self.collect_emergency()?,
        };
        
        let total_duration = collection_start.elapsed();
        
        // Update overall statistics
        self.update_collection_statistics(collection_strategy, total_duration)?;
        
        // Record collection time
        {
            let mut last_time = self.last_collection_time.lock()
                .map_err(|_| "Failed to acquire lock on last collection time")?;
            *last_time = Some(Instant::now());
        }
        
        info!("Generational collection completed in {:?} using strategy {:?}", total_duration, collection_strategy);
        self.get_stats()
    }
    
    /// Determine the best collection strategy
    fn determine_collection_strategy(&self) -> Result<CollectionStrategy, String> {
        // Check for emergency conditions first
        if let Some(heap_manager) = &self.heap_manager {
            let heap_stats = {
                let heap = heap_manager.read()
                    .map_err(|_| "Failed to acquire read lock on heap manager")?;
                heap.get_stats()?
            };
            
            if let Some((trigger_type, _reason)) = self.trigger_manager.should_trigger_collection(&heap_stats)? {
                return Ok(match trigger_type {
                    TriggerType::Emergency => CollectionStrategy::Emergency,
                    TriggerType::FullCollection => CollectionStrategy::Full,
                    TriggerType::OldGeneration => CollectionStrategy::OldOnly,
                    TriggerType::YoungGeneration => CollectionStrategy::YoungOnly,
                    TriggerType::Incremental => CollectionStrategy::Incremental,
                });
            }
        }
        
        // Check young generation pressure
        if self.should_collect_young()? {
            return Ok(CollectionStrategy::YoungOnly);
        }
        
        // Check old generation pressure
        if self.should_collect_old()? {
            return Ok(CollectionStrategy::OldOnly);
        }
        
        // Check if incremental collection is beneficial
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.incremental_collection && self.incremental_collector.is_collecting()? {
            return Ok(CollectionStrategy::Incremental);
        }
        
        // Default to young generation collection
        Ok(CollectionStrategy::YoungOnly)
    }
    
    /// Collect young generation
    #[instrument(skip(self))]
    fn collect_young_generation(&self) -> Result<GenerationalStats, String> {
        info!("Collecting young generation");
        
        let collection_start = Instant::now();
        
        // Set up promotion callback
        let promotion_callback = Box::new(|object_id: ObjectId, object_data: &[u8]| -> Result<(), String> {
            // TODO: Actually promote object to old generation
            debug!("Promoting object {} to old generation", object_id);
            Ok(())
        });
        
        // Perform copying collection with promotion
        let copying_stats = self.young_collector.collect(Some(promotion_callback))?;
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.young_collections += 1;
            stats.total_collections += 1;
            stats.young_collection_time += collection_duration;
            stats.total_collection_time += collection_duration;
            stats.objects_promoted += copying_stats.objects_promoted as u64;
            stats.bytes_promoted += copying_stats.bytes_promoted as u64;
            
            if stats.young_collections > 0 {
                stats.promotion_rate = stats.objects_promoted as f64 / stats.young_collections as f64;
            }
        }
        
        info!("Young generation collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Collect old generation
    #[instrument(skip(self))]
    fn collect_old_generation(&self) -> Result<GenerationalStats, String> {
        info!("Collecting old generation");
        
        let collection_start = Instant::now();
        
        // Perform mark-and-sweep collection
        let mark_sweep_stats = self.old_collector.collect()?;
        
        // Run cycle detection if enabled
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let cycles_detected = if config.cycle_detection {
            let cycles = self.cycle_detector.detect_cycles()?;
            let cycle_count = cycles.len();
            
            if !cycles.is_empty() {
                let collected_cycles = self.cycle_detector.collect_cycles(&cycles)?;
                info!("Detected and collected {} cycles containing {} objects", cycle_count, collected_cycles);
            }
            
            cycle_count as u64
        } else {
            0
        };
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.old_collections += 1;
            stats.total_collections += 1;
            stats.old_collection_time += collection_duration;
            stats.total_collection_time += collection_duration;
            stats.cycles_detected += cycles_detected;
            
            // Update collection efficiency
            if collection_duration.as_secs_f64() > 0.0 {
                let efficiency = mark_sweep_stats.bytes_reclaimed as f64 / collection_duration.as_secs_f64();
                stats.collection_efficiency = (stats.collection_efficiency + efficiency) / 2.0;
            }
        }
        
        info!("Old generation collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Collect all generations (full collection)
    #[instrument(skip(self))]
    fn collect_full(&self) -> Result<GenerationalStats, String> {
        info!("Performing full collection");
        
        let collection_start = Instant::now();
        
        // Collect young generation first
        self.collect_young_generation()?;
        
        // Then collect old generation
        self.collect_old_generation()?;
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.full_collections += 1;
            // Note: individual generation collections already updated their counters
        }
        
        info!("Full collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Perform incremental collection step
    #[instrument(skip(self))]
    fn collect_incremental(&self) -> Result<GenerationalStats, String> {
        debug!("Performing incremental collection step");
        
        let collection_start = Instant::now();
        
        // Perform incremental step
        let work_performed = self.incremental_collector.step()?;
        
        let collection_duration = collection_start.elapsed();
        
        if work_performed {
            // Update statistics
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.incremental_collections += 1;
            stats.total_collection_time += collection_duration;
        }
        
        debug!("Incremental collection step completed in {:?}, work performed: {}", collection_duration, work_performed);
        self.get_stats()
    }
    
    /// Emergency collection when memory is critically low
    #[instrument(skip(self))]
    fn collect_emergency(&self) -> Result<GenerationalStats, String> {
        warn!("Performing emergency collection");
        
        let collection_start = Instant::now();
        
        // Perform aggressive full collection
        self.collect_full()?;
        
        // Force cycle detection and collection
        let cycles = self.cycle_detector.detect_cycles()?;
        if !cycles.is_empty() {
            self.cycle_detector.collect_cycles(&cycles)?;
        }
        
        let collection_duration = collection_start.elapsed();
        
        warn!("Emergency collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Check if young generation collection is needed
    fn should_collect_young(&self) -> Result<bool, String> {
        self.young_collector.should_collect()
    }
    
    /// Check if old generation collection is needed
    fn should_collect_old(&self) -> Result<bool, String> {
        // TODO: Implement old generation pressure checking
        // For now, use a simple heuristic
        Ok(false)
    }
    
    /// Write barrier for cross-generational references
    #[instrument(skip(self))]
    pub fn write_barrier(&self, object_id: ObjectId, field_offset: usize, old_value: Option<ObjectId>, new_value: ObjectId) -> Result<(), String> {
        debug!("Write barrier: object {} field {} = {}", object_id, field_offset, new_value);
        
        // Determine generations
        let from_generation = self.get_object_generation(object_id)?;
        let to_generation = self.get_object_generation(new_value)?;
        
        // Track cross-generational references
        if from_generation != to_generation {
            let cross_ref = CrossGenerationalReference {
                from_object: object_id,
                from_generation,
                to_object: new_value,
                to_generation,
                field_offset,
            };
            
            let mut cross_refs = self.cross_gen_references.write()
                .map_err(|_| "Failed to acquire write lock on cross-generational references")?;
            cross_refs.push(cross_ref);
            
            debug!("Recorded cross-generational reference: {:?} -> {:?}", from_generation, to_generation);
        }
        
        // Forward to incremental collector for write barrier processing
        self.incremental_collector.write_barrier(object_id, field_offset, old_value, new_value)?;
        
        Ok(())
    }
    
    /// Get the generation of an object
    fn get_object_generation(&self, object_id: ObjectId) -> Result<Generation, String> {
        let object_generations = self.object_generations.read()
            .map_err(|_| "Failed to acquire read lock on object generations")?;
        
        Ok(object_generations.get(&object_id)
            .map(|info| info.generation)
            .unwrap_or(Generation::Young)) // Default to young generation
    }
    
    /// Track object allocation in a specific generation
    pub fn track_object_allocation(&self, object_id: ObjectId, generation: Generation, size: usize) -> Result<(), String> {
        let mut object_generations = self.object_generations.write()
            .map_err(|_| "Failed to acquire write lock on object generations")?;
        
        let info = ObjectGenerationInfo {
            generation,
            age: 0,
            promotion_candidate: false,
            size,
            allocated_at: Instant::now(),
        };
        
        object_generations.insert(object_id, info);
        debug!("Tracked object {} allocation in {:?} generation", object_id, generation);
        Ok(())
    }
    
    /// Promote an object from young to old generation
    pub fn promote_object(&self, object_id: ObjectId) -> Result<(), String> {
        let mut object_generations = self.object_generations.write()
            .map_err(|_| "Failed to acquire write lock on object generations")?;
        
        if let Some(info) = object_generations.get_mut(&object_id) {
            info.generation = Generation::Old;
            info.age = 0; // Reset age in new generation
            info.promotion_candidate = false;
            
            debug!("Promoted object {} to old generation", object_id);
        }
        
        Ok(())
    }
    
    /// Update collection statistics
    fn update_collection_statistics(&self, strategy: CollectionStrategy, duration: Duration) -> Result<(), String> {
        // Update heap utilization and other derived statistics
        if let Some(heap_manager) = &self.heap_manager {
            let heap_stats = {
                let heap = heap_manager.read()
                    .map_err(|_| "Failed to acquire read lock on heap manager")?;
                heap.get_stats()?
            };
            
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.heap_utilization = heap_stats.average_utilization;
            
            // Estimate generation sizes (simplified)
            let config = self.config.read()
                .map_err(|_| "Failed to acquire read lock on config")?;
            
            stats.young_generation_size = (heap_stats.total_capacity as f64 * config.young_generation_ratio) as usize;
            stats.old_generation_size = heap_stats.total_capacity - stats.young_generation_size;
        }
        
        Ok(())
    }
    
    /// Start background collection if enabled
    pub fn start_background_collection(&self) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.concurrent_collection {
            return Ok(());
        }
        
        let mut thread_handle = self.background_thread.lock()
            .map_err(|_| "Failed to acquire lock on background thread")?;
        
        if thread_handle.is_some() {
            return Ok(());
        }
        
        // Start incremental collector background thread
        self.incremental_collector.start_collection()?;
        
        let collector_ref = unsafe { std::mem::transmute::<&GenerationalCollector, &'static GenerationalCollector>(self) };
        
        let handle = std::thread::spawn(move || {
            info!("Starting background generational collection thread");
            collector_ref.background_collection_loop();
            info!("Background generational collection thread stopped");
        });
        
        *thread_handle = Some(handle);
        Ok(())
    }
    
    /// Background collection loop
    fn background_collection_loop(&self) {
        while !self.should_stop.load(std::sync::atomic::Ordering::SeqCst) {
            // Perform incremental collection steps
            if let Err(e) = self.collect_incremental() {
                error!("Background incremental collection failed: {}", e);
            }
            
            // Check if full collection is needed periodically
            if let Ok(strategy) = self.determine_collection_strategy() {
                match strategy {
                    CollectionStrategy::Emergency | CollectionStrategy::Full => {
                        if let Err(e) = self.collect() {
                            error!("Background full collection failed: {}", e);
                        }
                    }
                    _ => {
                        // Continue with incremental collection
                    }
                }
            }
            
            // Sleep to avoid busy waiting
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Stop background collection
    pub fn stop_background_collection(&self) -> Result<(), String> {
        self.should_stop.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Stop incremental collector
        self.incremental_collector.stop_background_collection()?;
        
        let handle = {
            let mut thread_handle = self.background_thread.lock()
                .map_err(|_| "Failed to acquire lock on background thread")?;
            thread_handle.take()
        };
        
        if let Some(handle) = handle {
            if let Err(e) = handle.join() {
                error!("Failed to join background thread: {:?}", e);
            }
        }
        
        Ok(())
    }
    
    /// Notify of allocation for trigger management
    pub fn notify_allocation(&self, bytes: usize) {
        // Forward to trigger manager and incremental collector
        if let Err(e) = self.trigger_manager.update_allocation_tracking(bytes, 1) {
            error!("Failed to update allocation tracking: {}", e);
        }
        
        self.incremental_collector.notify_allocation(bytes);
    }
    
    /// Get generational collection statistics
    pub fn get_stats(&self) -> Result<GenerationalStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: GenerationalConfig) -> Result<(), String> {
        // Update individual collector configurations
        self.young_collector.update_config(new_config.young_config.clone())?;
        self.old_collector.update_config(new_config.old_config.clone())?;
        self.incremental_collector.update_config(new_config.incremental_config.clone())?;
        self.cycle_detector.update_config(new_config.cycle_detection_config.clone())?;
        
        // Update main configuration
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        
        info!("Updated generational collector configuration");
        Ok(())
    }
    
    /// Force a specific type of collection
    pub fn force_collection(&self, strategy: CollectionStrategy) -> Result<GenerationalStats, String> {
        info!("Forcing collection with strategy {:?}", strategy);
        
        match strategy {
            CollectionStrategy::YoungOnly => self.collect_young_generation(),
            CollectionStrategy::OldOnly => self.collect_old_generation(),
            CollectionStrategy::Full => self.collect_full(),
            CollectionStrategy::Incremental => self.collect_incremental(),
            CollectionStrategy::Emergency => self.collect_emergency(),
        }
    }
    
    /// Get object count by generation
    pub fn get_object_counts_by_generation(&self) -> Result<HashMap<Generation, usize>, String> {
        let object_generations = self.object_generations.read()
            .map_err(|_| "Failed to acquire read lock on object generations")?;
        
        let mut counts = HashMap::new();
        for info in object_generations.values() {
            *counts.entry(info.generation).or_insert(0) += 1;
        }
        
        Ok(counts)
    }
}

impl Drop for GenerationalCollector {
    fn drop(&mut self) {
        if let Err(e) = self.stop_background_collection() {
            error!("Failed to stop background collection during drop: {}", e);
        }
    }
}

// Safety: GenerationalCollector is thread-safe through its component's thread safety
unsafe impl Send for GenerationalCollector {}
unsafe impl Sync for GenerationalCollector {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    fn create_test_collector() -> Result<(GenerationalCollector, SharedObjectRegistry), String> {
        let registry = Arc::new(ObjectRegistry::new());
        let collector = GenerationalCollector::new(registry.clone())?;
        Ok((collector, registry))
    }
    
    #[test]
    fn test_collector_creation() {
        let (collector, _registry) = create_test_collector().unwrap();
        let stats = collector.get_stats().unwrap();
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_allocation_tracking() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let object_id = ObjectId::new(1);
        collector.track_object_allocation(object_id, Generation::Young, 64).unwrap();
        
        let counts = collector.get_object_counts_by_generation().unwrap();
        assert_eq!(counts.get(&Generation::Young), Some(&1));
    }
    
    #[test]
    fn test_object_promotion() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let object_id = ObjectId::new(2);
        collector.track_object_allocation(object_id, Generation::Young, 64).unwrap();
        collector.promote_object(object_id).unwrap();
        
        let generation = collector.get_object_generation(object_id).unwrap();
        assert_eq!(generation, Generation::Old);
    }
    
    #[test]
    fn test_write_barrier() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let from_object = ObjectId::new(3);
        let to_object = ObjectId::new(4);
        
        collector.track_object_allocation(from_object, Generation::Old, 64).unwrap();
        collector.track_object_allocation(to_object, Generation::Young, 32).unwrap();
        
        collector.write_barrier(from_object, 0, None, to_object).unwrap();
        
        // Should have recorded cross-generational reference
        let cross_refs = collector.cross_gen_references.read().unwrap();
        assert_eq!(cross_refs.len(), 1);
    }
    
    #[test]
    fn test_config_update() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let new_config = GenerationalConfig {
            young_generation_ratio: 0.5,
            promotion_age_threshold: 5,
            ..Default::default()
        };
        
        collector.update_config(new_config).unwrap();
    }
    
    #[test]
    fn test_force_collection() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        // Force a young generation collection
        let stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
        assert!(stats.young_collections > 0);
    }
}
