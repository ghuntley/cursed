//! Memory profiling and garbage collection monitoring

use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct MemoryProfiler {
    allocations: Vec<AllocationEvent>,
    gc_events: Vec<GcEvent>,
    leaks: Vec<MemoryLeak>,
    start_time: Instant,
    total_allocated: usize,
    total_freed: usize,
}

#[derive(Debug, Clone)]
pub struct AllocationEvent {
    pub timestamp: Instant,
    pub size: usize,
    pub location: String,
    pub object_type: String,
    pub allocation_id: u64,
}

#[derive(Debug, Clone)]
pub struct GcEvent {
    pub timestamp: Instant,
    pub gc_type: GcType,
    pub duration: Duration,
    pub bytes_collected: usize,
    pub objects_collected: usize,
    pub heap_size_before: usize,
    pub heap_size_after: usize,
}

#[derive(Debug, Clone)]
pub enum GcType {
    Minor,
    Major,
    Full,
    Incremental,
    Concurrent,
}

#[derive(Debug, Clone)]
pub struct MemoryLeak {
    pub allocation_id: u64,
    pub size: usize,
    pub location: String,
    pub age: Duration,
    pub object_type: String,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
            gc_events: Vec::new(),
            leaks: Vec::new(),
            start_time: Instant::now(),
            total_allocated: 0,
            total_freed: 0,
        }
    }

    pub fn record_allocation(&mut self, size: usize, location: String, object_type: String) -> u64 {
        let allocation_id = self.allocations.len() as u64;
        let event = AllocationEvent {
            timestamp: Instant::now(),
            size,
            location,
            object_type,
            allocation_id,
        };
        
        self.total_allocated += size;
        self.allocations.push(event);
        allocation_id
    }

    pub fn record_deallocation(&mut self, allocation_id: u64, size: usize) {
        self.total_freed += size;
        
        // Remove from active allocations if tracking
        self.allocations.retain(|alloc| alloc.allocation_id != allocation_id);
    }

    pub fn record_gc_event(&mut self, gc_type: GcType, duration: Duration, bytes_collected: usize, objects_collected: usize, heap_before: usize, heap_after: usize) {
        let event = GcEvent {
            timestamp: Instant::now(),
            gc_type,
            duration,
            bytes_collected,
            objects_collected,
            heap_size_before: heap_before,
            heap_size_after: heap_after,
        };
        
        self.gc_events.push(event);
        self.total_freed += bytes_collected;
    }

    pub fn detect_memory_leaks(&mut self, max_age: Duration) {
        let now = Instant::now();
        let threshold = now - max_age;
        
        for allocation in &self.allocations {
            if allocation.timestamp < threshold {
                let leak = MemoryLeak {
                    allocation_id: allocation.allocation_id,
                    size: allocation.size,
                    location: allocation.location.clone(),
                    age: now - allocation.timestamp,
                    object_type: allocation.object_type.clone(),
                };
                self.leaks.push(leak);
            }
        }
    }

    pub fn get_total_allocated(&self) -> usize {
        self.total_allocated
    }

    pub fn get_total_freed(&self) -> usize {
        self.total_freed
    }

    pub fn get_current_usage(&self) -> usize {
        self.total_allocated.saturating_sub(self.total_freed)
    }

    pub fn get_gc_statistics(&self) -> GcStatistics {
        let mut stats = GcStatistics::default();
        
        for event in &self.gc_events {
            stats.total_collections += 1;
            stats.total_collection_time += event.duration;
            stats.total_bytes_collected += event.bytes_collected;
            stats.total_objects_collected += event.objects_collected;
            
            match event.gc_type {
                GcType::Minor => stats.minor_collections += 1,
                GcType::Major => stats.major_collections += 1,
                GcType::Full => stats.full_collections += 1,
                GcType::Incremental => stats.incremental_collections += 1,
                GcType::Concurrent => stats.concurrent_collections += 1,
            }
        }
        
        if stats.total_collections > 0 {
            stats.average_collection_time = stats.total_collection_time / stats.total_collections as u32;
        }
        
        stats
    }

    pub fn get_memory_leaks(&self) -> &[MemoryLeak] {
        &self.leaks
    }

    pub fn get_allocation_histogram(&self) -> HashMap<String, AllocationStats> {
        let mut histogram = HashMap::new();
        
        for allocation in &self.allocations {
            let stats = histogram.entry(allocation.object_type.clone()).or_insert(AllocationStats::default());
            stats.count += 1;
            stats.total_size += allocation.size;
            stats.max_size = stats.max_size.max(allocation.size);
            stats.min_size = stats.min_size.min(allocation.size);
        }
        
        for stats in histogram.values_mut() {
            if stats.count > 0 {
                stats.average_size = stats.total_size / stats.count;
            }
        }
        
        histogram
    }

    pub fn generate_memory_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Memory Profiling Report ===\n");
        
        report.push_str(&format!("Total Allocated: {} bytes\n", self.total_allocated));
        report.push_str(&format!("Total Freed: {} bytes\n", self.total_freed));
        report.push_str(&format!("Current Usage: {} bytes\n", self.get_current_usage()));
        report.push_str(&format!("Active Allocations: {}\n", self.allocations.len()));
        report.push_str(&format!("Memory Leaks: {}\n", self.leaks.len()));
        
        let gc_stats = self.get_gc_statistics();
        report.push_str(&format!("GC Collections: {}\n", gc_stats.total_collections));
        report.push_str(&format!("GC Total Time: {:?}\n", gc_stats.total_collection_time));
        report.push_str(&format!("GC Average Time: {:?}\n", gc_stats.average_collection_time));
        
        if !self.leaks.is_empty() {
            report.push_str("\n=== Memory Leaks ===\n");
            for leak in &self.leaks {
                report.push_str(&format!(
                    "Leak ID {}: {} bytes, {} old, type: {}, location: {}\n",
                    leak.allocation_id,
                    leak.size,
                    format_duration(leak.age),
                    leak.object_type,
                    leak.location
                ));
            }
        }
        
        report
    }

    pub fn reset(&mut self) {
        self.allocations.clear();
        self.gc_events.clear();
        self.leaks.clear();
        self.start_time = Instant::now();
        self.total_allocated = 0;
        self.total_freed = 0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct GcStatistics {
    pub total_collections: usize,
    pub minor_collections: usize,
    pub major_collections: usize,
    pub full_collections: usize,
    pub incremental_collections: usize,
    pub concurrent_collections: usize,
    pub total_collection_time: Duration,
    pub average_collection_time: Duration,
    pub total_bytes_collected: usize,
    pub total_objects_collected: usize,
}

#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub count: usize,
    pub total_size: usize,
    pub average_size: usize,
    pub min_size: usize,
    pub max_size: usize,
}

impl Default for AllocationStats {
    fn default() -> Self {
        Self {
            count: 0,
            total_size: 0,
            average_size: 0,
            min_size: usize::MAX,
            max_size: 0,
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new();
        
        let id1 = profiler.record_allocation(1024, "test.rs:10".to_string(), "String".to_string());
        let id2 = profiler.record_allocation(512, "test.rs:20".to_string(), "Vec".to_string());
        
        assert_eq!(profiler.get_total_allocated(), 1536);
        assert_eq!(profiler.get_current_usage(), 1536);
        
        profiler.record_deallocation(id1, 1024);
        assert_eq!(profiler.get_current_usage(), 512);
        
        profiler.record_gc_event(GcType::Minor, Duration::from_millis(10), 256, 5, 1000, 744);
        let stats = profiler.get_gc_statistics();
        assert_eq!(stats.total_collections, 1);
        assert_eq!(stats.minor_collections, 1);
    }

    #[test]
    fn test_leak_detection() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_allocation(1024, "test.rs:10".to_string(), "String".to_string());
        
        // Simulate old allocation
        std::thread::sleep(Duration::from_millis(1));
        profiler.detect_memory_leaks(Duration::from_nanos(1));
        
        assert!(!profiler.get_memory_leaks().is_empty());
    }
}
