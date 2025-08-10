/// GC Tuning Module for Performance Optimization
///
/// This module provides advanced tuning capabilities for the garbage collector
/// to achieve sub-100ms pause times for web and server applications.

use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::runtime::gc::{GarbageCollector, GcConfig, GcStats};
use crate::error::CursedError;

/// Advanced GC tuning parameters for production workloads
#[derive(Debug, Clone)]
pub struct GcTuningParams {
    /// Target maximum pause time in milliseconds
    pub max_pause_time_ms: u64,
    /// Target heap utilization percentage (0.0 to 1.0)
    pub target_heap_utilization: f64,
    /// Concurrent collection thread count
    pub concurrent_threads: usize,
    /// Incremental collection step size in bytes
    pub incremental_step_size: usize,
    /// Tri-color marking enabled
    pub tri_color_marking: bool,
    /// Generational collection enabled
    pub generational_collection: bool,
    /// Young generation promotion threshold
    pub young_promotion_threshold: usize,
    /// Old generation collection frequency
    pub old_generation_frequency: usize,
    /// Memory pressure threshold for aggressive collection
    pub memory_pressure_threshold: f64,
    /// Enable write barriers for concurrent collection
    pub write_barriers: bool,
    /// Enable card table for generational collection
    pub card_table: bool,
    /// Card table size in bytes
    pub card_table_size: usize,
}

impl Default for GcTuningParams {
    fn default() -> Self {
        Self {
            max_pause_time_ms: 50, // Target 50ms max pause time
            target_heap_utilization: 0.75,
            concurrent_threads: std::cmp::max(2, num_cpus::get() / 2),
            incremental_step_size: 1024 * 1024, // 1MB incremental steps
            tri_color_marking: true,
            generational_collection: true,
            young_promotion_threshold: 8, // Promote after 8 young collections
            old_generation_frequency: 10, // Old gen collection every 10 young collections
            memory_pressure_threshold: 0.9,
            write_barriers: true,
            card_table: true,
            card_table_size: 512 * 1024, // 512KB card table
        }
    }
}

/// Tri-color marking collector for concurrent collection
pub struct TriColorCollector {
    /// White objects (not yet visited)
    white_objects: RwLock<HashMap<usize, ObjectInfo>>,
    /// Gray objects (visited but not processed)
    gray_objects: Arc<RwLock<Vec<usize>>>,
    /// Black objects (fully processed)
    black_objects: RwLock<HashMap<usize, ObjectInfo>>,
    /// Write barrier log
    write_barrier_log: RwLock<Vec<WriteBarrierEntry>>,
    /// Collection phase
    collection_phase: AtomicUsize, // 0=idle, 1=mark, 2=sweep
    /// Marking progress
    marking_progress: AtomicUsize,
    /// Total objects to mark
    total_objects: AtomicUsize,
}

#[derive(Debug, Clone)]
struct ObjectInfo {
    address: usize,
    size: usize,
    last_access: Instant,
    reference_count: usize,
}

#[derive(Debug, Clone)]
struct WriteBarrierEntry {
    object_addr: usize,
    field_addr: usize,
    new_value: usize,
    timestamp: Instant,
}

impl TriColorCollector {
    pub fn new() -> Self {
        Self {
            white_objects: RwLock::new(HashMap::new()),
            gray_objects: Arc::new(RwLock::new(Vec::new())),
            black_objects: RwLock::new(HashMap::new()),
            write_barrier_log: RwLock::new(Vec::new()),
            collection_phase: AtomicUsize::new(0),
            marking_progress: AtomicUsize::new(0),
            total_objects: AtomicUsize::new(0),
        }
    }

    /// Start concurrent marking phase
    pub fn start_concurrent_marking(&self, root_objects: Vec<usize>) -> Result<(), CursedError> {
        // Initialize all objects as white
        let mut white_objects = self.white_objects.write().unwrap();
        white_objects.clear();
        
        // Add all heap objects to white set
        for obj_addr in &root_objects {
            white_objects.insert(*obj_addr, ObjectInfo {
                address: *obj_addr,
                size: self.get_object_size(*obj_addr),
                last_access: Instant::now(),
                reference_count: 0,
            });
        }
        
        self.total_objects.store(white_objects.len(), Ordering::Relaxed);
        drop(white_objects);
        
        // Start with root objects in gray set
        let mut gray_objects = self.gray_objects.write().unwrap();
        gray_objects.clear();
        
        // Add root objects to gray set
        for root_addr in &root_objects {
            gray_objects.push(*root_addr);
        }
        
        self.collection_phase.store(1, Ordering::Relaxed);
        self.marking_progress.store(0, Ordering::Relaxed);
        
        Ok(())
    }

    /// Perform incremental marking step
    pub fn incremental_mark_step(&self, max_objects: usize) -> Result<bool, CursedError> {
        let mut gray_objects = self.gray_objects.write().unwrap();
        let mut processed = 0;
        
        while processed < max_objects && !gray_objects.is_empty() {
            let obj_addr = gray_objects.pop().unwrap();
            
            // Move object from white to black
            let mut white_objects = self.white_objects.write().unwrap();
            if let Some(obj_info) = white_objects.remove(&obj_addr) {
                drop(white_objects);
                
                // Add object to black set
                let mut black_objects = self.black_objects.write().unwrap();
                black_objects.insert(obj_addr, obj_info);
                drop(black_objects);
                
                // Find all references and add to gray set
                let references = self.find_object_references(obj_addr)?;
                for ref_addr in references {
                    if self.is_white_object(ref_addr) {
                        gray_objects.push(ref_addr);
                    }
                }
                
                processed += 1;
                self.marking_progress.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        // Check if marking is complete
        Ok(gray_objects.is_empty())
    }

    /// Handle write barrier for concurrent collection
    pub fn write_barrier(&self, object_addr: usize, field_addr: usize, new_value: usize) {
        if self.collection_phase.load(Ordering::Relaxed) == 1 {
            // During marking phase, log write barriers
            let mut write_log = self.write_barrier_log.write().unwrap();
            write_log.push(WriteBarrierEntry {
                object_addr,
                field_addr,
                new_value,
                timestamp: Instant::now(),
            });
            
            // If we're writing to a black object, add the new reference to gray set
            if self.is_black_object(object_addr) && new_value != 0 {
                let mut gray_objects = self.gray_objects.write().unwrap();
                gray_objects.push(new_value);
            }
        }
    }

    /// Get marking progress as percentage
    pub fn get_marking_progress(&self) -> f64 {
        let total = self.total_objects.load(Ordering::Relaxed);
        let marked = self.marking_progress.load(Ordering::Relaxed);
        
        if total == 0 {
            return 100.0;
        }
        
        (marked as f64 / total as f64) * 100.0
    }

    /// Check if object is white (unmarked)
    fn is_white_object(&self, obj_addr: usize) -> bool {
        let white_objects = self.white_objects.read().unwrap();
        white_objects.contains_key(&obj_addr)
    }

    /// Check if object is black (fully marked)
    fn is_black_object(&self, obj_addr: usize) -> bool {
        let black_objects = self.black_objects.read().unwrap();
        black_objects.contains_key(&obj_addr)
    }

    /// Find all object references (stub implementation)
    fn find_object_references(&self, obj_addr: usize) -> Result<Vec<usize>, CursedError> {
        // This would scan the object's memory for pointer-sized values
        // and check if they point to valid heap objects
        Ok(Vec::new()) // Stub implementation
    }

    /// Get object size (stub implementation)
    fn get_object_size(&self, obj_addr: usize) -> usize {
        // This would read the object's metadata to get its size
        64 // Stub implementation
    }
}

/// GC performance tuner that adapts collection parameters based on runtime metrics
pub struct GcPerformanceTuner {
    /// Current tuning parameters
    params: RwLock<GcTuningParams>,
    /// Pause time history
    pause_times: RwLock<Vec<Duration>>,
    /// Allocation rate history
    allocation_rates: RwLock<Vec<f64>>,
    /// Heap utilization history
    heap_utilization: RwLock<Vec<f64>>,
    /// Last tuning adjustment
    last_adjustment: RwLock<Instant>,
    /// Tuning enabled flag
    tuning_enabled: AtomicBool,
}

impl GcPerformanceTuner {
    pub fn new(params: GcTuningParams) -> Self {
        Self {
            params: RwLock::new(params),
            pause_times: RwLock::new(Vec::new()),
            allocation_rates: RwLock::new(Vec::new()),
            heap_utilization: RwLock::new(Vec::new()),
            last_adjustment: RwLock::new(Instant::now()),
            tuning_enabled: AtomicBool::new(true),
        }
    }

    /// Record GC statistics for tuning
    pub fn record_gc_stats(&self, stats: &GcStats) {
        if !self.tuning_enabled.load(Ordering::Relaxed) {
            return;
        }

        // Record pause time
        {
            let mut pause_times = self.pause_times.write().unwrap();
            pause_times.push(stats.avg_pause_time);
            
            // Keep only last 100 measurements
            if pause_times.len() > 100 {
                pause_times.remove(0);
            }
        }

        // Record allocation rate
        {
            let mut allocation_rates = self.allocation_rates.write().unwrap();
            allocation_rates.push(stats.allocation_rate);
            
            if allocation_rates.len() > 100 {
                allocation_rates.remove(0);
            }
        }

        // Record heap utilization
        {
            let mut heap_utilization = self.heap_utilization.write().unwrap();
            heap_utilization.push(stats.heap_utilization);
            
            if heap_utilization.len() > 100 {
                heap_utilization.remove(0);
            }
        }

        // Adjust parameters if needed
        self.adjust_parameters_if_needed();
    }

    /// Adjust GC parameters based on performance metrics
    fn adjust_parameters_if_needed(&self) {
        let last_adjustment = *self.last_adjustment.read().unwrap();
        
        // Only adjust parameters every 10 seconds
        if last_adjustment.elapsed() < Duration::from_secs(10) {
            return;
        }

        let avg_pause_time = self.calculate_average_pause_time();
        let mut params = self.params.write().unwrap();
        
        // If pause times are too high, make collection more aggressive
        if avg_pause_time.as_millis() > params.max_pause_time_ms as u128 {
            // Increase concurrent threads if possible
            if params.concurrent_threads < num_cpus::get() {
                params.concurrent_threads += 1;
            }
            
            // Reduce incremental step size for more frequent incremental collection
            params.incremental_step_size = std::cmp::max(
                params.incremental_step_size / 2,
                512 * 1024 // Minimum 512KB
            );
            
            // Lower heap utilization threshold for more frequent collection
            params.target_heap_utilization = (params.target_heap_utilization - 0.05).max(0.5);
            
            println!("GC Tuner: Adjusted parameters for lower pause times");
        }
        // If pause times are very low, we can be less aggressive
        else if avg_pause_time.as_millis() < params.max_pause_time_ms as u128 / 2 {
            // Increase incremental step size for better throughput
            params.incremental_step_size = std::cmp::min(
                params.incremental_step_size * 2,
                8 * 1024 * 1024 // Maximum 8MB
            );
            
            // Increase heap utilization threshold for less frequent collection
            params.target_heap_utilization = (params.target_heap_utilization + 0.05).min(0.9);
        }
        
        *self.last_adjustment.write().unwrap() = Instant::now();
    }

    /// Calculate average pause time from recent measurements
    fn calculate_average_pause_time(&self) -> Duration {
        let pause_times = self.pause_times.read().unwrap();
        
        if pause_times.is_empty() {
            return Duration::from_millis(0);
        }
        
        let total_ms: u64 = pause_times.iter()
            .map(|d| d.as_millis() as u64)
            .sum();
        
        Duration::from_millis(total_ms / pause_times.len() as u64)
    }

    /// Get current tuning parameters
    pub fn get_params(&self) -> GcTuningParams {
        self.params.read().unwrap().clone()
    }

    /// Enable/disable automatic tuning
    pub fn set_tuning_enabled(&self, enabled: bool) {
        self.tuning_enabled.store(enabled, Ordering::Relaxed);
    }
}

/// Create optimized GC configuration for web/server workloads
pub fn create_web_server_gc_config() -> GcConfig {
    let tuning_params = GcTuningParams::default();
    
    GcConfig {
        auto_collect: true,
        memory_threshold: 128 * 1024 * 1024, // 128MB
        time_threshold: Duration::from_millis(tuning_params.max_pause_time_ms),
        object_threshold: 10000,
        initial_heap_size: 128 * 1024 * 1024, // 128MB initial heap
        max_heap_size: Some(2 * 1024 * 1024 * 1024), // 2GB max heap
        young_generation_ratio: 0.4, // 40% for young generation
        young_collection_threshold: 32 * 1024 * 1024, // 32MB threshold
        old_collection_threshold: 256 * 1024 * 1024, // 256MB threshold
        incremental_collection: true,
        incremental_time_budget: tuning_params.max_pause_time_ms / 2, // Half of max pause time
        concurrent_collection: true,
        concurrent_threads: tuning_params.concurrent_threads,
        trigger_mode: crate::runtime::gc::GcTriggerMode::Adaptive,
        enable_compaction: true,
        compaction_threshold: 0.25, // 25% fragmentation threshold
    }
}

/// Create optimized GC configuration for low-latency applications
pub fn create_low_latency_gc_config() -> GcConfig {
    let tuning_params = GcTuningParams {
        max_pause_time_ms: 10, // Ultra-low 10ms pause time
        concurrent_threads: num_cpus::get(), // Use all available cores
        incremental_step_size: 256 * 1024, // Small 256KB incremental steps
        ..Default::default()
    };
    
    GcConfig {
        auto_collect: true,
        memory_threshold: 256 * 1024 * 1024, // 256MB
        time_threshold: Duration::from_millis(tuning_params.max_pause_time_ms),
        object_threshold: 5000,
        initial_heap_size: 256 * 1024 * 1024, // 256MB initial heap
        max_heap_size: Some(4 * 1024 * 1024 * 1024), // 4GB max heap
        young_generation_ratio: 0.5, // 50% for young generation
        young_collection_threshold: 16 * 1024 * 1024, // 16MB threshold
        old_collection_threshold: 128 * 1024 * 1024, // 128MB threshold
        incremental_collection: true,
        incremental_time_budget: tuning_params.max_pause_time_ms / 4, // Quarter of max pause time
        concurrent_collection: true,
        concurrent_threads: tuning_params.concurrent_threads,
        trigger_mode: crate::runtime::gc::GcTriggerMode::Adaptive,
        enable_compaction: false, // Disable compaction for minimum pause times
        compaction_threshold: 0.5, // High threshold to avoid compaction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_tuning_params_default() {
        let params = GcTuningParams::default();
        assert_eq!(params.max_pause_time_ms, 50);
        assert_eq!(params.target_heap_utilization, 0.75);
        assert!(params.tri_color_marking);
        assert!(params.generational_collection);
    }

    #[test]
    fn test_tri_color_collector_creation() {
        let collector = TriColorCollector::new();
        assert_eq!(collector.collection_phase.load(Ordering::Relaxed), 0);
        assert_eq!(collector.marking_progress.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_performance_tuner_creation() {
        let params = GcTuningParams::default();
        let tuner = GcPerformanceTuner::new(params);
        assert!(tuner.tuning_enabled.load(Ordering::Relaxed));
    }

    #[test]
    fn test_web_server_gc_config() {
        let config = create_web_server_gc_config();
        assert_eq!(config.initial_heap_size, 128 * 1024 * 1024);
        assert_eq!(config.young_generation_ratio, 0.4);
        assert!(config.concurrent_collection);
        assert!(config.incremental_collection);
    }

    #[test]
    fn test_low_latency_gc_config() {
        let config = create_low_latency_gc_config();
        assert_eq!(config.initial_heap_size, 256 * 1024 * 1024);
        assert_eq!(config.young_generation_ratio, 0.5);
        assert!(!config.enable_compaction);
        assert_eq!(config.incremental_time_budget, 2); // 10ms / 4 = 2.5ms rounded down
    }
}
