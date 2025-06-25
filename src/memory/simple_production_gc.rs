/// Simplified Production Garbage Collector
/// 
/// This is a simplified but complete production-ready garbage collector that
/// integrates with the existing CURSED memory management system without
/// requiring extensive API changes.

use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{
// };
use crate::error::CursedError;

/// Simplified production GC configuration
#[derive(Debug, Clone)]
pub struct SimpleProductionGcConfig {
    /// Base GC configuration
    /// Heap configuration
    /// Enable automatic collection
    /// Background collection interval
    /// Emergency collection threshold
impl Default for SimpleProductionGcConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Statistics for the simple production GC
#[derive(Debug, Clone)]
pub struct SimpleProductionStats {
    /// Total allocations
    /// Total collections
    /// Total collection time
    /// Memory pressure triggers
    /// Manual triggers
    /// Current heap usage
    /// Peak heap usage
/// Simplified production garbage collector
/// 
/// This provides a production-ready garbage collector that integrates
/// with existing CURSED memory management components without requiring
/// extensive API changes.
pub struct SimpleProductionGarbageCollector {
    /// Main garbage collector
    /// Heap manager
    /// Configuration
    /// Object registry
    /// Statistics
    /// Background thread handle
    /// Shutdown flag
    /// Total allocations counter
    /// Total collections counter
    /// Peak heap usage
impl SimpleProductionGarbageCollector {
    /// Create a new simple production garbage collector
    #[instrument]
    pub fn new(config: SimpleProductionGcConfig) -> Result<Self, String> {
        info!("Creating simple production garbage collector");
        
        // Create object registry
        let object_registry = Arc::new(ObjectRegistry::new());
        
        // Create heap manager
        let heap_manager = Arc::new(RwLock::new(
            HeapManager::new(config.heap_config.clone(), object_registry.clone())
        ));
        
        // Create main garbage collector
        let gc = Arc::new(GarbageCollector::with_config(
            config.heap_config.clone()
        ));
        
        let shutdown = Arc::new(AtomicBool::new(false));
        
        let collector = Self {
            stats: Arc::new(Mutex::new(SimpleProductionStats {
        
        // Start background collection if enabled
        if config.enable_auto_collection {
            collector.start_background_collection()?;
        info!("Simple production garbage collector created successfully");
        Ok(collector)
    /// Allocate an object
    #[instrument(skip(self, obj))]
    pub fn allocate<T>(&self, obj: T) -> Result<crate::memory::gc::Gc<T>, String>
    where
    {
        let allocation_size = std::mem::size_of::<T>();
        debug!("Allocating object of size {} bytes", allocation_size);
        
        // Check if we should collect before allocation
        if self.should_collect_before_allocation()? {
            info!("Triggering collection before allocation");
            let _ = self.collect();
        // Perform allocation
        let result = self.gc.allocate(obj);
        
        match &result {
            Ok(_) => {
                // Update allocation statistics
                self.allocation_count.fetch_add(1, Ordering::Relaxed);
                
                // Update heap usage statistics
                if let Ok(heap_stats) = self.get_heap_stats() {
                    let current_usage = heap_stats.total_used as u64;
                    let peak = self.peak_heap_usage.load(Ordering::Relaxed);
                    if current_usage > peak {
                        self.peak_heap_usage.store(current_usage, Ordering::Relaxed);
                    // Update statistics
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.total_allocations = self.allocation_count.load(Ordering::Relaxed);
                        stats.current_heap_usage = heap_stats.total_used;
                        stats.peak_heap_usage = self.peak_heap_usage.load(Ordering::Relaxed) as usize;
                    }
                }
                
                debug!("Successfully allocated object");
            }
            Err(e) => {
                warn!("Allocation failed: {}", e);
                
                // Try emergency collection
                if let Err(collection_err) = self.collect() {
                    error!("Emergency collection failed: {}", collection_err);
                    return Err(format!("Allocation failed and emergency collection failed: {}", e));
                warn!("Performed emergency collection due to allocation failure");
            }
        }
        
        result
    /// Trigger garbage collection
    #[instrument(skip(self))]
    pub fn collect(&self) -> Result<crate::memory::gc::EnhancedCollectionStats, String> {
        info!("Starting garbage collection");
        let start_time = Instant::now();
        
        let result = self.gc.collect();
        
        match &result {
            Ok(stats) => {
                let collection_duration = start_time.elapsed();
                
                // Update statistics
                self.collection_count.fetch_add(1, Ordering::Relaxed);
                
                if let Ok(mut production_stats) = self.stats.lock() {
                    production_stats.total_collections = self.collection_count.load(Ordering::Relaxed);
                    production_stats.total_collection_time += collection_duration;
                    production_stats.manual_triggers += 1;
                      stats.objects_collected, stats.bytes_collected, collection_duration);
            }
            Err(e) => {
                error!("Garbage collection failed: {}", e);
            }
        }
        
        result
    /// Check if collection should be triggered before allocation
    fn should_collect_before_allocation(&self) -> Result<bool, String> {
        // Simple heuristic: collect if heap utilization is high
        let heap_stats = self.get_heap_stats()?;
        let config = self.config.read().map_err(|_| "Failed to read config")?;
        
        let utilization = if heap_stats.total_capacity > 0 {
            heap_stats.total_used as f64 / heap_stats.total_capacity as f64
        } else {
            0.0
        
        Ok(utilization > config.emergency_threshold)
    /// Start background collection thread
    fn start_background_collection(&self) -> Result<(), String> {
        let gc = self.gc.clone();
        let heap_manager = self.heap_manager.clone();
        let shutdown = self.shutdown.clone();
        let config = self.config.clone();
        let stats = self.stats.clone();
        
        let handle = std::thread::Builder::new()
            .name("simple-gc-background".to_string())
            .spawn(move || {
                info!("Background garbage collection thread started");
                
                while !shutdown.load(Ordering::Acquire) {
                    // Get collection interval from config
                    let interval = if let Ok(cfg) = config.read() {
                        cfg.background_collection_interval
                    } else {
                        Duration::from_millis(500)
                    
                    std::thread::sleep(interval);
                    
                    // Check if collection is needed
                    if let Ok(heap_stats) = Self::get_heap_stats_static(&heap_manager) {
                        let utilization = if heap_stats.total_capacity > 0 {
                            heap_stats.total_used as f64 / heap_stats.total_capacity as f64
                        } else {
                            0.0
                        
                        // Trigger collection if utilization is high
                        if utilization > 0.8 {
                            info!("Background collection triggered due to {:.1}% heap utilization", utilization * 100.0);
                            
                            if let Ok(collection_stats) = gc.collect() {
                                // Update statistics
                                if let Ok(mut production_stats) = stats.lock() {
                                    production_stats.pressure_triggers += 1;
                                    production_stats.total_collections += 1;
                                       collection_stats.objects_collected);
                            } else {
                                warn!("Background collection failed");
                            }
                        }
                    }
                }
                
                info!("Background garbage collection thread stopped");
            })
            .map_err(|e| format!("Failed to start background collection thread: {}", e))?;
        
        *self.background_thread.lock().unwrap() = Some(handle);
        Ok(())
    /// Get heap statistics
    fn get_heap_stats(&self) -> Result<crate::memory::heap_manager::HeapStats, String> {
        Self::get_heap_stats_static(&self.heap_manager)
    /// Static version of get_heap_stats for background thread
    fn get_heap_stats_static(heap_manager: &Arc<RwLock<HeapManager>>) -> Result<crate::memory::heap_manager::HeapStats, String> {
        let heap = heap_manager.read().map_err(|_| "Failed to read heap manager")?;
        heap.get_stats()
    /// Get production statistics
    pub fn get_stats(&self) -> Result<SimpleProductionStats, String> {
        let stats = self.stats.lock().map_err(|_| "Failed to lock stats")?;
        Ok(stats.clone())
    /// Enable or disable auto collection
    pub fn set_auto_collection(&self, enabled: bool) -> Result<(), String> {
        if enabled && self.background_thread.lock().unwrap().is_none() {
            self.start_background_collection()?;
        info!("Auto collection {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    /// Force a collection cycle
    pub fn force_collection(&self) -> Result<crate::memory::gc::EnhancedCollectionStats, String> {
        info!("Forcing garbage collection");
        self.collect()
    /// Get the underlying garbage collector
    pub fn gc(&self) -> &Arc<GarbageCollector> {
        &self.gc
    /// Get object registry
    pub fn object_registry(&self) -> &SharedObjectRegistry {
        &self.object_registry
    /// Get current memory usage
    pub fn memory_usage(&self) -> Result<f64, String> {
        let heap_stats = self.get_heap_stats()?;
        let utilization = if heap_stats.total_capacity > 0 {
            heap_stats.total_used as f64 / heap_stats.total_capacity as f64
        } else {
            0.0
        Ok(utilization)
    }
}

impl Drop for SimpleProductionGarbageCollector {
    fn drop(&mut self) {
        info!("Shutting down simple production garbage collector");
        
        // Signal shutdown
        self.shutdown.store(true, Ordering::Release);
        
        // Wait for background thread to finish
        if let Ok(mut thread_handle) = self.background_thread.lock() {
            if let Some(handle) = thread_handle.take() {
                if let Err(e) = handle.join() {
                    error!("Background collection thread panicked: {:?}", e);
                }
            }
        // Perform final collection
        if let Err(e) = self.collect() {
            warn!("Final garbage collection failed: {}", e);
        info!("Simple production garbage collector shutdown complete");
    }
}

// Safety: The same safety reasoning as the full production GC applies
unsafe impl Send for SimpleProductionGarbageCollector {}
unsafe impl Sync for SimpleProductionGarbageCollector {}

