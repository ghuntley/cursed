use crate::error::CursedError;
// Memory profiling for allocation tracking and analysis

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

// use crate::profiling::core::{DataCollector, CollectorStats, ProfilerError};

/// Memory profiler for allocation tracking and analysis
#[derive(Debug)]
pub struct MemoryProfiler {
impl MemoryProfiler {
    pub fn new(tracking_threshold: usize) -> Self {
        Self {
        }
    }
    
    #[instrument(skip(self))]
    pub fn track_allocation(
    ) -> crate::error::Result<()> {
        if !self.is_collecting() || size < self.tracking_threshold {
            return Ok(());
        let allocation = AllocationEvent {
        
        if let Ok(mut data) = self.data.write() {
            data.add_allocation_event(allocation);
        self.update_stats(|stats| {
            stats.data_points += 1;
        });
        
        Ok(())
    #[instrument(skip(self))]
    pub fn track_deallocation(
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        let deallocation = AllocationEvent {
            size: 0, // Size not available for deallocations
        
        if let Ok(mut data) = self.data.write() {
            data.add_allocation_event(deallocation);
        self.update_stats(|stats| {
            stats.data_points += 1;
        });
        
        Ok(())
    #[instrument(skip(self))]
    pub fn track_gc_event(&self, gc_event: GcEvent) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        if let Ok(mut data) = self.data.write() {
            data.add_gc_event(gc_event);
        Ok(())
    fn get_current_thread_id() -> u64 {
        // Use a simple hash of the thread id since as_u64() is unstable
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    fn update_stats<F>(&self, updater: F)
    where
    {
        if let Ok(mut stats) = self.stats.write() {
            updater(&mut stats);
        }
    }
    
    pub fn get_memory_usage_snapshot(&self) -> MemoryUsageSnapshot {
        if let Ok(data) = self.data.read() {
            data.calculate_current_usage()
        } else {
            MemoryUsageSnapshot::default()
        }
    }
    
    pub fn analyze_allocation_patterns(&self) -> AllocationAnalysis {
        if let Ok(data) = self.data.read() {
            data.analyze_patterns()
        } else {
            AllocationAnalysis::default()
        }
    }
    
    pub fn detect_memory_leaks(&self) -> Vec<MemoryLeak> {
        if let Ok(data) = self.data.read() {
            data.detect_leaks()
        } else {
            Vec::new()
        }
    }
impl DataCollector for MemoryProfiler {
    #[instrument(skip(self))]
    fn start_collection(&mut self) -> crate::error::Result<()> {
        if self.is_collecting() {
            return Err(ProfilerError::ConfigError("Memory profiler already collecting".to_string()));
        *self.collecting.lock().unwrap() = true;
        self.allocation_hooks.install()?;
        
        info!("Started memory profiling with threshold {} bytes", self.tracking_threshold);
        Ok(())
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("Memory profiler not collecting".to_string()));
        *self.collecting.lock().unwrap() = false;
        self.allocation_hooks.uninstall()?;
        
        let profile_data = self.data.read().unwrap().clone();
        match bincode::serialize(&profile_data) {
            Ok(data) => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.bytes_collected = data.len() as u64;
                }
                      profile_data.allocation_events.len());
                Ok(data)
            }
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// Memory profiling data collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfileData {
impl MemoryProfileData {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_allocation_event(&mut self, event: AllocationEvent) {
        self.allocation_events.push(event);
    pub fn add_gc_event(&mut self, event: GcEvent) {
        self.gc_events.push(event);
    pub fn add_heap_snapshot(&mut self, snapshot: HeapSnapshot) {
        self.heap_snapshots.push(snapshot);
    pub fn calculate_current_usage(&self) -> MemoryUsageSnapshot {
        let mut allocated_bytes = 0;
        let mut allocated_objects: u64 = 0;
        let mut active_allocations: HashMap<u64, usize> = HashMap::new();
        
        for event in &self.allocation_events {
            match event.event_type {
                AllocationEventType::Allocate => {
                    active_allocations.insert(event.address, event.size);
                    allocated_bytes += event.size;
                    allocated_objects += 1;
                }
                AllocationEventType::Deallocate => {
                    if let Some(size) = active_allocations.remove(&event.address) {
                        allocated_bytes = allocated_bytes.saturating_sub(size);
                        allocated_objects = allocated_objects.saturating_sub(1);
                    }
                }
                AllocationEventType::Realloc => {
                    if let Some(old_size) = active_allocations.get_mut(&event.address) {
                        allocated_bytes = allocated_bytes.saturating_sub(*old_size);
                        *old_size = event.size;
                        allocated_bytes += event.size;
                    }
                }
            }
        }
        
        MemoryUsageSnapshot {
        }
    }
    
    pub fn analyze_patterns(&self) -> AllocationAnalysis {
        let mut size_histogram: HashMap<usize, u64> = HashMap::new();
        let mut function_allocations: HashMap<String, AllocationStats> = HashMap::new();
        let mut temporal_patterns = Vec::new();
        
        for event in &self.allocation_events {
            if let AllocationEventType::Allocate = event.event_type {
                // Size histogram
                let size_bucket = Self::size_bucket(event.size);
                *size_histogram.entry(size_bucket).or_default() += 1;
                
                // Function allocation patterns
                if let Some(function) = event.stack_trace.first() {
                    let stats = function_allocations
                        .entry(function.clone())
                        .or_default();
                    stats.allocation_count += 1;
                    stats.total_bytes += event.size;
                    stats.average_size = stats.total_bytes / stats.allocation_count;
                // Temporal patterns
                temporal_patterns.push(TemporalAllocation {
                    cumulative_size: 0, // Will be calculated
                });
            }
        }
        
        // Calculate cumulative sizes
        let mut cumulative = 0;
        for pattern in &mut temporal_patterns {
            cumulative += pattern.size;
            pattern.cumulative_size = cumulative;
        AllocationAnalysis {
        }
    }
    
    fn size_bucket(size: usize) -> usize {
        // Create size buckets: 1-16, 17-32, 33-64, etc.
        if size == 0 { return 0; }
        let bucket = (size - 1).next_power_of_two();
        std::cmp::min(bucket, 1024 * 1024) // Cap at 1MB bucket
    fn calculate_allocation_rate(&self) -> f64 {
        if self.allocation_events.is_empty() {
            return 0.0;
        let duration = self.start_time.elapsed().unwrap_or(Duration::from_secs(1));
        let allocations = self.allocation_events
            .iter()
            .filter(|e| matches!(e.event_type, AllocationEventType::Allocate))
            .count();
        
        allocations as f64 / duration.as_secs_f64()
    pub fn detect_leaks(&self) -> Vec<MemoryLeak> {
        let mut active_allocations: HashMap<u64, &AllocationEvent> = HashMap::new();
        let mut leaks = Vec::new();
        
        // Track allocations and deallocations
        for event in &self.allocation_events {
            match event.event_type {
                AllocationEventType::Allocate => {
                    active_allocations.insert(event.address, event);
                }
                AllocationEventType::Deallocate => {
                    active_allocations.remove(&event.address);
                }
                AllocationEventType::Realloc => {
                    // Realloc is both dealloc of old and alloc of new
                    active_allocations.insert(event.address, event);
                }
            }
        // Remaining allocations are potential leaks
        let now = SystemTime::now();
        for (address, allocation) in active_allocations {
            let age = now.duration_since(self.start_time)
                .unwrap_or(Duration::from_secs(0));
            
            // Consider allocations older than 10 seconds as potential leaks
            if age > Duration::from_secs(10) {
                leaks.push(MemoryLeak {
                });
            }
        }
        
        leaks.sort_by(|a, b| b.size.cmp(&a.size)); // Sort by size descending
        leaks
    }
}

/// Individual allocation/deallocation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationEvent {
    #[serde(skip, default = "Instant::now")]
/// Type of allocation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationEventType {
/// Garbage collection event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcEvent {
    #[serde(skip, default = "Instant::now")]
/// Types of garbage collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GcType {
/// Heap snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapSnapshot {
/// Current memory usage snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageSnapshot {
    #[serde(default = "SystemTime::now")]
impl Default for MemoryUsageSnapshot {
    fn default() -> Self {
        Self {
        }
    }
/// Allocation pattern analysis results
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocationAnalysis {
    #[serde(skip)]
    pub allocation_rate: f64, // allocations per second
/// Allocation statistics per function
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllocationStats {
/// Temporal allocation pattern
#[derive(Debug, Clone, Serialize)]
pub struct TemporalAllocation {
    #[serde(skip)]
/// Detected memory leak
#[derive(Debug, Clone, Serialize)]
pub struct MemoryLeak {
    #[serde(skip)]
impl MemoryLeak {
    pub fn severity(&self) -> LeakSeverity {
        match self.size {
        }
    }
/// Memory leak severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeakSeverity {
/// Allocation hook system for instrumenting memory operations
#[derive(Debug)]
struct AllocationHooks {
impl AllocationHooks {
    fn new() -> Self {
        Self { installed: false }
    }
    
    fn install(&mut self) -> crate::error::Result<()> {
        if self.installed {
            return Ok(());
        // In a real implementation, this would:
        // - Hook into the system allocator (malloc/free)
        // - Instrument CURSED's garbage collector
        // - Set up stack trace capturing
        // - Install allocation callbacks
        
        self.installed = true;
        debug!("Memory allocation hooks installed");
        Ok(())
    fn uninstall(&mut self) -> crate::error::Result<()> {
        if !self.installed {
            return Ok(());
        // In a real implementation, this would:
        // - Remove allocator hooks
        // - Uninstall callbacks
        // - Clean up instrumentation
        
        self.installed = false;
        debug!("Memory allocation hooks uninstalled");
        Ok(())
    }
}

