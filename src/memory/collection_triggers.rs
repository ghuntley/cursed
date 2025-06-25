/// Collection Trigger System for Garbage Collection
/// 
/// This module implements sophisticated heuristics and triggers for determining
/// when garbage collection should be performed. It includes allocation-based,
/// time-based, and pressure-based triggers for different collection strategies.

use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant};
use tracing::{instrument, debug, info, warn};

use crate::memory::heap_manager::{HeapStats, MemoryPressure, AllocationMetrics};
use crate::error_types::Error;

/// Types of collection triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerType {
    /// Young generation collection trigger
    YoungGeneration,
    /// Old generation collection trigger
    OldGeneration,
    /// Full heap collection trigger
    FullCollection,
    /// Emergency collection when memory is critically low
    Emergency,
    /// Incremental collection step
    Incremental,
}

/// Reasons why collection was triggered
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerReason {
    /// Allocation pressure reached threshold
    AllocationPressure { utilization: f64, threshold: f64 },
    /// Time-based trigger fired
    TimeBased { elapsed: Duration, interval: Duration },
    /// Object count exceeded threshold
    ObjectCount { count: usize, threshold: usize },
    /// Fragmentation exceeded threshold
    Fragmentation { fragmentation: f64, threshold: f64 },
    /// External request for collection
    External { reason: String },
    /// Emergency low memory situation
    Emergency { available_bytes: usize },
    /// Promotional pressure from young to old generation
    PromotionalPressure { promotion_rate: f64 },
}

/// Configuration for collection triggers
#[derive(Debug, Clone)]
pub struct TriggerConfig {
    /// Young generation allocation threshold (0.0 to 1.0)
    pub young_allocation_threshold: f64,
    /// Old generation allocation threshold (0.0 to 1.0)
    pub old_allocation_threshold: f64,
    /// Full collection allocation threshold (0.0 to 1.0)
    pub full_allocation_threshold: f64,
    /// Emergency collection threshold (0.0 to 1.0)
    pub emergency_threshold: f64,
    /// Fragmentation threshold for triggering collection
    pub fragmentation_threshold: f64,
    /// Time interval for time-based collection
    pub time_based_interval: Option<Duration>,
    /// Object count threshold for collection
    pub object_count_threshold: Option<usize>,
    /// Promotion rate threshold (young to old generation)
    pub promotion_rate_threshold: f64,
    /// Enable adaptive threshold adjustment
    pub adaptive_thresholds: bool,
    /// Enable predictive triggering based on allocation patterns
    pub predictive_triggering: bool,
}

impl Default for TriggerConfig {
    fn default() -> Self {
        Self {
            young_allocation_threshold: 0.75,    // Trigger young GC at 75% full
            old_allocation_threshold: 0.85,      // Trigger old GC at 85% full
            full_allocation_threshold: 0.90,     // Trigger full GC at 90% full
            emergency_threshold: 0.95,           // Emergency GC at 95% full
            fragmentation_threshold: 0.30,       // Trigger on 30% fragmentation
            time_based_interval: Some(Duration::from_secs(10)),
            object_count_threshold: Some(10000),
            promotion_rate_threshold: 0.20,      // Trigger if 20% promotion rate
            adaptive_thresholds: true,
            predictive_triggering: true,
        }
    }
}

/// Statistics about collection triggers
#[derive(Debug, Clone, Default)]
pub struct TriggerStats {
    pub total_triggers: u64,
    pub triggers_by_type: std::collections::HashMap<TriggerType, u64>,
    pub triggers_by_reason: std::collections::HashMap<String, u64>,
    pub false_triggers: u64,
    pub emergency_triggers: u64,
    pub average_trigger_interval: Duration,
    pub last_trigger_time: Option<Instant>,
}

/// Collection trigger history for analysis
#[derive(Debug, Clone)]
pub struct TriggerEvent {
    pub trigger_type: TriggerType,
    pub reason: TriggerReason,
    pub timestamp: Instant,
    pub heap_stats_before: HeapStats,
    pub collection_needed: bool,
}

/// Allocation tracking for predictive triggers
#[derive(Debug, Clone)]
struct AllocationTracker {
    allocation_rate: f64,      // bytes per second
    object_creation_rate: f64, // objects per second
    last_update: Instant,
    total_allocated: usize,
    total_objects: usize,
    samples: Vec<(Instant, usize, usize)>,
}

impl AllocationTracker {
    fn new() -> Self {
        Self {
            allocation_rate: 0.0,
            object_creation_rate: 0.0,
            last_update: Instant::now(),
            total_allocated: 0,
            total_objects: 0,
            samples: Vec::new(),
        }
    }
    
    fn update(&mut self, bytes_allocated: usize, objects_allocated: usize) {
        let now = Instant::now();
        let duration = now.duration_since(self.last_update);
        
        if duration >= Duration::from_millis(100) { // Update every 100ms
            let duration_secs = duration.as_secs_f64();
            self.allocation_rate = bytes_allocated as f64 / duration_secs;
            self.object_creation_rate = objects_allocated as f64 / duration_secs;
            
            self.total_allocated += bytes_allocated;
            self.total_objects += objects_allocated;
            
            // Keep last 60 samples (6 seconds at 100ms intervals)
            self.samples.push((now, bytes_allocated, objects_allocated));
            if self.samples.len() > 60 {
                self.samples.remove(0);
            }
            
            self.last_update = now;
        }
    }
    
    fn predict_time_to_threshold(&self, current_utilization: f64, threshold: f64, heap_size: usize) -> Option<Duration> {
        if self.allocation_rate <= 0.0 {
            return None;
        }
        
        let available_bytes = heap_size as f64 * (threshold - current_utilization);
        if available_bytes <= 0.0 {
            return Some(Duration::from_secs(0));
        }
        
        let time_to_threshold = available_bytes / self.allocation_rate;
        Some(Duration::from_secs_f64(time_to_threshold))
    }
}

/// Main collection trigger manager
pub struct CollectionTriggerManager {
    config: RwLock<TriggerConfig>,
    stats: RwLock<TriggerStats>,
    allocation_tracker: Mutex<AllocationTracker>,
    trigger_history: Mutex<Vec<TriggerEvent>>,
    last_collection_times: RwLock<std::collections::HashMap<TriggerType, Instant>>,
    adaptive_thresholds: Mutex<std::collections::HashMap<TriggerType, f64>>,
}

impl CollectionTriggerManager {
    /// Create a new trigger manager with default configuration
    pub fn new() -> Self {
        Self::with_config(TriggerConfig::default())
    }
    
    /// Create a new trigger manager with custom configuration
    #[instrument(skip(config))]
    pub fn with_config(config: TriggerConfig) -> Self {
        info!("Creating collection trigger manager with config: {:?}", config);
        
        Self {
            config: RwLock::new(config),
            stats: RwLock::new(TriggerStats::default()),
            allocation_tracker: Mutex::new(AllocationTracker::new()),
            trigger_history: Mutex::new(Vec::new()),
            last_collection_times: RwLock::new(std::collections::HashMap::new()),
            adaptive_thresholds: Mutex::new(std::collections::HashMap::new()),
        }
    }
    
    /// Check if collection should be triggered based on current heap state
    #[instrument(skip(self, heap_stats))]
    pub fn should_trigger_collection(&self, heap_stats: &HeapStats) -> Result<Option<(TriggerType, TriggerReason)>, String> {
        // Check emergency condition first
        if heap_stats.average_utilization >= self.get_threshold(TriggerType::Emergency)? {
            let reason = TriggerReason::Emergency {
                available_bytes: heap_stats.total_capacity - heap_stats.total_used,
            };
            return Ok(Some((TriggerType::Emergency, reason)));
        }
        
        // Check full collection triggers
        if let Some(reason) = self.check_full_collection_triggers(heap_stats)? {
            return Ok(Some((TriggerType::FullCollection, reason)));
        }
        
        // Check old generation triggers
        if let Some(reason) = self.check_old_generation_triggers(heap_stats)? {
            return Ok(Some((TriggerType::OldGeneration, reason)));
        }
        
        // Check young generation triggers
        if let Some(reason) = self.check_young_generation_triggers(heap_stats)? {
            return Ok(Some((TriggerType::YoungGeneration, reason)));
        }
        
        // Check incremental collection triggers
        if let Some(reason) = self.check_incremental_triggers(heap_stats)? {
            return Ok(Some((TriggerType::Incremental, reason)));
        }
        
        Ok(None)
    }
    
    /// Check triggers for full collection
    fn check_full_collection_triggers(&self, heap_stats: &HeapStats) -> Result<Option<TriggerReason>, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Allocation pressure
        let threshold = self.get_threshold(TriggerType::FullCollection)?;
        if heap_stats.average_utilization >= threshold {
            return Ok(Some(TriggerReason::AllocationPressure {
                utilization: heap_stats.average_utilization,
                threshold,
            }));
        }
        
        // Fragmentation
        if heap_stats.fragmentation_ratio >= config.fragmentation_threshold {
            return Ok(Some(TriggerReason::Fragmentation {
                fragmentation: heap_stats.fragmentation_ratio,
                threshold: config.fragmentation_threshold,
            }));
        }
        
        // Time-based trigger
        if let Some(interval) = config.time_based_interval {
            if let Some(reason) = self.check_time_based_trigger(TriggerType::FullCollection, interval)? {
                return Ok(Some(reason));
            }
        }
        
        Ok(None)
    }
    
    /// Check triggers for old generation collection
    fn check_old_generation_triggers(&self, heap_stats: &HeapStats) -> Result<Option<TriggerReason>, String> {
        let threshold = self.get_threshold(TriggerType::OldGeneration)?;
        
        // Allocation pressure on old generation
        // Note: This is simplified - in a real implementation you'd have separate
        // heap stats for young and old generations
        if heap_stats.average_utilization >= threshold {
            return Ok(Some(TriggerReason::AllocationPressure {
                utilization: heap_stats.average_utilization,
                threshold,
            }));
        }
        
        // Check promotion pressure
        if let Some(reason) = self.check_promotion_pressure()? {
            return Ok(Some(reason));
        }
        
        Ok(None)
    }
    
    /// Check triggers for young generation collection
    fn check_young_generation_triggers(&self, heap_stats: &HeapStats) -> Result<Option<TriggerReason>, String> {
        let threshold = self.get_threshold(TriggerType::YoungGeneration)?;
        
        // Young generation allocation pressure
        // Note: This is simplified - in a real implementation you'd have separate
        // heap stats for young generation
        if heap_stats.average_utilization >= threshold {
            return Ok(Some(TriggerReason::AllocationPressure {
                utilization: heap_stats.average_utilization,
                threshold,
            }));
        }
        
        // Predictive triggering
        if self.should_trigger_predictively(TriggerType::YoungGeneration, heap_stats)? {
            return Ok(Some(TriggerReason::AllocationPressure {
                utilization: heap_stats.average_utilization,
                threshold,
            }));
        }
        
        Ok(None)
    }
    
    /// Check triggers for incremental collection
    fn check_incremental_triggers(&self, _heap_stats: &HeapStats) -> Result<Option<TriggerReason>, String> {
        // Incremental triggers are typically time-based or work-based
        // For now, this is a placeholder
        Ok(None)
    }
    
    /// Check time-based trigger
    fn check_time_based_trigger(&self, trigger_type: TriggerType, interval: Duration) -> Result<Option<TriggerReason>, String> {
        let last_times = self.last_collection_times.read()
            .map_err(|_| "Failed to acquire read lock on last_collection_times")?;
        
        let last_time = last_times.get(&trigger_type).copied().unwrap_or_else(|| {
            Instant::now() - interval // First time, consider interval elapsed
        });
        
        let elapsed = Instant::now().duration_since(last_time);
        if elapsed >= interval {
            return Ok(Some(TriggerReason::TimeBased { elapsed, interval }));
        }
        
        Ok(None)
    }
    
    /// Check promotion pressure from young to old generation
    fn check_promotion_pressure(&self) -> Result<Option<TriggerReason>, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // TODO: Implement actual promotion rate tracking
        // For now, this is a placeholder
        let promotion_rate = 0.1; // 10% - this would be calculated from actual data
        
        if promotion_rate >= config.promotion_rate_threshold {
            return Ok(Some(TriggerReason::PromotionalPressure { promotion_rate }));
        }
        
        Ok(None)
    }
    
    /// Check if predictive triggering should fire
    fn should_trigger_predictively(&self, trigger_type: TriggerType, heap_stats: &HeapStats) -> Result<bool, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.predictive_triggering {
            return Ok(false);
        }
        
        let tracker = self.allocation_tracker.lock()
            .map_err(|_| "Failed to acquire lock on allocation tracker")?;
        
        let threshold = self.get_threshold(trigger_type)?;
        if let Some(time_to_threshold) = tracker.predict_time_to_threshold(
            heap_stats.average_utilization,
            threshold,
            heap_stats.total_capacity,
        ) {
            // Trigger if we'll hit threshold in next 5 seconds
            return Ok(time_to_threshold <= Duration::from_secs(5));
        }
        
        Ok(false)
    }
    
    /// Get the current threshold for a trigger type (may be adaptive)
    fn get_threshold(&self, trigger_type: TriggerType) -> Result<f64, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.adaptive_thresholds {
            let adaptive_thresholds = self.adaptive_thresholds.lock()
                .map_err(|_| "Failed to acquire lock on adaptive thresholds")?;
            
            if let Some(&adaptive_threshold) = adaptive_thresholds.get(&trigger_type) {
                return Ok(adaptive_threshold);
            }
        }
        
        let base_threshold = match trigger_type {
            TriggerType::YoungGeneration => config.young_allocation_threshold,
            TriggerType::OldGeneration => config.old_allocation_threshold,
            TriggerType::FullCollection => config.full_allocation_threshold,
            TriggerType::Emergency => config.emergency_threshold,
            TriggerType::Incremental => config.young_allocation_threshold * 0.5, // More frequent
        };
        
        Ok(base_threshold)
    }
    
    /// Record that a collection was triggered
    #[instrument(skip(self, heap_stats_before))]
    pub fn record_trigger(&self, trigger_type: TriggerType, reason: TriggerReason, heap_stats_before: HeapStats, collection_needed: bool) -> Result<(), String> {
        debug!("Recording trigger: {:?} with reason: {:?}", trigger_type, reason);
        
        let now = Instant::now();
        
        // Update last collection time
        {
            let mut last_times = self.last_collection_times.write()
                .map_err(|_| "Failed to acquire write lock on last_collection_times")?;
            last_times.insert(trigger_type, now);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.total_triggers += 1;
            *stats.triggers_by_type.entry(trigger_type).or_insert(0) += 1;
            *stats.triggers_by_reason.entry(format!("{:?}", reason)).or_insert(0) += 1;
            
            if !collection_needed {
                stats.false_triggers += 1;
            }
            
            if trigger_type == TriggerType::Emergency {
                stats.emergency_triggers += 1;
            }
            
            // Update average trigger interval
            if let Some(last_time) = stats.last_trigger_time {
                let interval = now.duration_since(last_time);
                let total_intervals = stats.total_triggers as f64;
                let current_avg = stats.average_trigger_interval.as_secs_f64();
                let new_avg = (current_avg * (total_intervals - 1.0) + interval.as_secs_f64()) / total_intervals;
                stats.average_trigger_interval = Duration::from_secs_f64(new_avg);
            }
            stats.last_trigger_time = Some(now);
        }
        
        // Record trigger event
        {
            let mut history = self.trigger_history.lock()
                .map_err(|_| "Failed to acquire lock on trigger history")?;
            
            let event = TriggerEvent {
                trigger_type,
                reason,
                timestamp: now,
                heap_stats_before: heap_stats_before.clone(),
                collection_needed,
            };
            
            history.push(event);
            
            // Keep only last 1000 events
            if history.len() > 1000 {
                history.remove(0);
            }
        }
        
        // Adjust adaptive thresholds if enabled
        if collection_needed {
            self.adjust_adaptive_thresholds(trigger_type, &heap_stats_before)?;
        }
        
        Ok(())
    }
    
    /// Update allocation tracking
    pub fn update_allocation_tracking(&self, bytes_allocated: usize, objects_allocated: usize) -> Result<(), String> {
        let mut tracker = self.allocation_tracker.lock()
            .map_err(|_| "Failed to acquire lock on allocation tracker")?;
        tracker.update(bytes_allocated, objects_allocated);
        Ok(())
    }
    
    /// Adjust adaptive thresholds based on collection performance
    fn adjust_adaptive_thresholds(&self, trigger_type: TriggerType, heap_stats: &HeapStats) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.adaptive_thresholds {
            return Ok(());
        }
        
        let mut adaptive_thresholds = self.adaptive_thresholds.lock()
            .map_err(|_| "Failed to acquire lock on adaptive thresholds")?;
        
        let current_threshold = self.get_threshold(trigger_type)?;
        
        // Simple adaptive adjustment: if collection was triggered too early
        // (low utilization), increase threshold slightly. If too late (high
        // utilization), decrease threshold slightly.
        let adjustment = match heap_stats.average_utilization {
            util if util < current_threshold - 0.1 => 0.02,  // Increase threshold
            util if util > current_threshold + 0.05 => -0.02, // Decrease threshold
            _ => 0.0, // No adjustment
        };
        
        if adjustment != 0.0 {
            let new_threshold = (current_threshold + adjustment).clamp(0.1, 0.95);
            adaptive_thresholds.insert(trigger_type, new_threshold);
            debug!("Adjusted threshold for {:?}: {:.2} -> {:.2}", trigger_type, current_threshold, new_threshold);
        }
        
        Ok(())
    }
    
    /// Request external collection trigger
    pub fn request_collection(&self, trigger_type: TriggerType, reason: String) -> Result<(), String> {
        let dummy_stats = HeapStats {
            total_blocks: 0,
            total_capacity: 0,
            total_used: 0,
            total_free: 0,
            average_utilization: 0.0,
            active_objects: 0,
            object_registry_count: 0,
            fragmentation_ratio: 0.0,
            memory_pressure: MemoryPressure::Low,
            metrics: AllocationMetrics::default(),
        };
        
        let external_reason = TriggerReason::External { reason };
        self.record_trigger(trigger_type, external_reason, dummy_stats, true)
    }
    
    /// Get trigger statistics
    pub fn get_stats(&self) -> Result<TriggerStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Get trigger history
    pub fn get_trigger_history(&self, limit: Option<usize>) -> Result<Vec<TriggerEvent>, String> {
        let history = self.trigger_history.lock()
            .map_err(|_| "Failed to acquire lock on trigger history")?;
        
        let events = if let Some(limit) = limit {
            history.iter().rev().take(limit).rev().cloned().collect()
        } else {
            history.clone()
        };
        
        Ok(events)
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: TriggerConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated trigger configuration: {:?}", config);
        Ok(())
    }
}

impl Default for CollectionTriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: CollectionTriggerManager is thread-safe through its use of RwLock and Mutex
unsafe impl Send for CollectionTriggerManager {}
unsafe impl Sync for CollectionTriggerManager {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_creation() {
        let manager = CollectionTriggerManager::new();
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_triggers, 0);
    }
    
    #[test]
    fn test_allocation_pressure_trigger() {
        let manager = CollectionTriggerManager::new();
        
        let heap_stats = HeapStats {
            total_blocks: 1,
            total_capacity: 1000,
            total_used: 900, // 90% utilization
            total_free: 100,
            average_utilization: 0.90,
            active_objects: 100,
            object_registry_count: 100,
            fragmentation_ratio: 0.1,
            memory_pressure: MemoryPressure::Low,
            metrics: AllocationMetrics::default(),
        };
        
        let result = manager.should_trigger_collection(&heap_stats).unwrap();
        assert!(result.is_some());
        
        if let Some((trigger_type, reason)) = result {
            assert_eq!(trigger_type, TriggerType::FullCollection);
            if let TriggerReason::AllocationPressure { utilization, .. } = reason {
                assert_eq!(utilization, 0.90);
            } else {
                panic!("Expected AllocationPressure reason");
            }
        }
    }
    
    #[test]
    fn test_emergency_trigger() {
        let manager = CollectionTriggerManager::new();
        
        let heap_stats = HeapStats {
            total_blocks: 1,
            total_capacity: 1000,
            total_used: 960, // 96% utilization
            total_free: 40,
            average_utilization: 0.96,
            active_objects: 100,
            object_registry_count: 100,
            fragmentation_ratio: 0.1,
            memory_pressure: MemoryPressure::High,
            metrics: AllocationMetrics::default(),
        };
        
        let result = manager.should_trigger_collection(&heap_stats).unwrap();
        assert!(result.is_some());
        
        if let Some((trigger_type, _)) = result {
            assert_eq!(trigger_type, TriggerType::Emergency);
        }
    }
    
    #[test]
    fn test_trigger_recording() {
        let manager = CollectionTriggerManager::new();
        
        let heap_stats = HeapStats {
            total_blocks: 1,
            total_capacity: 1000,
            total_used: 500,
            total_free: 500,
            average_utilization: 0.50,
            active_objects: 50,
            object_registry_count: 50,
            fragmentation_ratio: 0.1,
            memory_pressure: MemoryPressure::Low,
            metrics: AllocationMetrics::default(),
        };
        
        let reason = TriggerReason::External { reason: "test".to_string() };
        manager.record_trigger(TriggerType::YoungGeneration, reason, heap_stats, true).unwrap();
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_triggers, 1);
        assert_eq!(stats.triggers_by_type[&TriggerType::YoungGeneration], 1);
    }
    
    #[test]
    fn test_external_collection_request() {
        let manager = CollectionTriggerManager::new();
        
        manager.request_collection(TriggerType::FullCollection, "Manual request".to_string()).unwrap();
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_triggers, 1);
    }
}
