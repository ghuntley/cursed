//! Memory allocation profiler for CURSED language
//!
//! This module provides tools for tracking and profiling memory allocations,
//! helping to identify hot code paths and optimize memory usage patterns.

use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::any::TypeId;
use tracing::{debug, info, warn};
use std::backtrace::Backtrace;

/// Allocation event information
#[derive(Debug, Clone)]
pub struct AllocationEvent {
    /// Type name of the allocated object
    pub type_name: String,
    /// Size of the allocation in bytes
    pub size: usize,
    /// Timestamp when the allocation occurred
    pub timestamp: Instant,
    /// Stack trace at the allocation point (if available)
    pub stack_trace: Option<Vec<String>>,
    /// Source location of the allocation (if available)
    pub location: Option<String>,
}

/// Allocation statistics for a specific type
#[derive(Debug, Clone, Default)]
pub struct TypeAllocationStats {
    /// Total count of allocations for this type
    pub count: usize,
    /// Total bytes allocated for this type
    pub total_bytes: usize,
    /// Maximum single allocation size for this type
    pub max_allocation: usize,
    /// Average allocation size for this type
    pub avg_allocation: f64,
    /// First seen timestamp
    pub first_seen: Option<Instant>,
    /// Last seen timestamp
    pub last_seen: Option<Instant>,
    /// Common allocation sites
    pub allocation_sites: HashMap<String, usize>,
}

/// Hot spot information for allocation frequency
#[derive(Debug, Clone)]
pub struct HotSpot {
    /// Location or type of the hot spot
    pub identifier: String,
    /// Number of allocations at this hot spot
    pub allocation_count: usize,
    /// Total memory allocated at this hot spot
    pub total_bytes: usize,
    /// Rate of allocations per second
    pub allocation_rate: f64,
}

/// Allocation pattern information
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    /// Pattern description
    pub description: String,
    /// Allocation sequence information
    pub sequence: Vec<String>,
    /// Frequency of this pattern
    pub frequency: usize,
    /// Total memory impact of this pattern
    pub memory_impact: usize,
    /// Optimization suggestions
    pub suggestions: Vec<String>,
}

/// Global memory allocation profiler
///
/// This struct keeps track of memory allocations across the program,
/// collecting statistics and identifying patterns to help optimize memory usage.
pub struct AllocationProfiler {
    /// All recorded allocation events
    events: Arc<Mutex<Vec<AllocationEvent>>>,
    /// Statistics by type
    type_stats: Arc<RwLock<HashMap<TypeId, TypeAllocationStats>>>,
    /// Type name mapping
    type_names: Arc<RwLock<HashMap<TypeId, String>>>,
    /// Start time of profiling
    start_time: Instant,
    /// Whether to capture stack traces (can be expensive)
    capture_stack_traces: bool,
    /// Whether detailed profiling is enabled
    enabled: bool,
    /// Maximum events to store
    max_events: usize,
    /// Sampling rate (1.0 = all allocations, 0.1 = 10% of allocations)
    sampling_rate: f64,
}

impl Default for AllocationProfiler {
    fn default() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            type_stats: Arc::new(RwLock::new(HashMap::new())),
            type_names: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            capture_stack_traces: false,
            enabled: false,
            max_events: 10000,
            sampling_rate: 1.0,
        }
    }
}

impl AllocationProfiler {
    /// Create a new allocation profiler
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Enable the profiler
    pub fn enable(&mut self) {
        self.enabled = true;
        info!("Memory allocation profiler enabled");
    }
    
    /// Disable the profiler
    pub fn disable(&mut self) {
        self.enabled = false;
        info!("Memory allocation profiler disabled");
    }
    
    /// Set whether to capture stack traces
    pub fn set_capture_stack_traces(&mut self, capture: bool) {
        self.capture_stack_traces = capture;
        info!("Stack trace capture {}", if capture { "enabled" } else { "disabled" });
    }
    
    /// Set the maximum number of events to store
    pub fn set_max_events(&mut self, max_events: usize) {
        self.max_events = max_events;
    }
    
    /// Set the sampling rate
    pub fn set_sampling_rate(&mut self, rate: f64) {
        self.sampling_rate = rate.clamp(0.0, 1.0);
    }
    
    /// Reset the profiler
    pub fn reset(&mut self) {
        if let Ok(mut events) = self.events.lock() {
            events.clear();
        }
        if let Ok(mut stats) = self.type_stats.write() {
            stats.clear();
        }
        self.start_time = Instant::now();
        info!("Memory allocation profiler reset");
    }
    
    /// Record an allocation event
    pub fn record_allocation<T: 'static>(&self, size: usize, location: Option<String>) {
        if !self.enabled {
            return;
        }
        
        // Apply sampling rate
        if self.sampling_rate < 1.0 && rand::random::<f64>() > self.sampling_rate {
            return;
        }
        
        let type_id = TypeId::of::<T>();
        let type_name = std::any::type_name::<T>().to_string();
        
        // Update type name mapping
        if let Ok(mut type_names) = self.type_names.write() {
            type_names.insert(type_id, type_name.clone());
        }
        
        // Capture stack trace if enabled
        let stack_trace = if self.capture_stack_traces {
            Some(self.capture_backtrace())
        } else {
            None
        };
        
        // Create allocation event
        let event = AllocationEvent {
            type_name: type_name.clone(),
            size,
            timestamp: Instant::now(),
            stack_trace,
            location: location.clone(),
        };
        
        // Update type statistics
        if let Ok(mut stats) = self.type_stats.write() {
            let type_stat = stats.entry(type_id).or_insert_with(|| TypeAllocationStats::default());
            type_stat.count += 1;
            type_stat.total_bytes += size;
            type_stat.max_allocation = type_stat.max_allocation.max(size);
            type_stat.avg_allocation = type_stat.total_bytes as f64 / type_stat.count as f64;
            
            if type_stat.first_seen.is_none() {
                type_stat.first_seen = Some(event.timestamp);
            }
            type_stat.last_seen = Some(event.timestamp);
            
            if let Some(loc) = &location {
                *type_stat.allocation_sites.entry(loc.clone()).or_insert(0) += 1;
            }
        }
        
        // Store event if we haven't reached the maximum
        if let Ok(mut events) = self.events.lock() {
            if events.len() < self.max_events {
                events.push(event);
            }
        }
    }
    
    /// Capture a backtrace at the current location
    fn capture_backtrace(&self) -> Vec<String> {
        let backtrace = Backtrace::capture();
        // Convert backtrace to vector of strings, skip first few frames
        // which are this function and its callers within the profiling system
        backtrace.to_string().lines()
            .skip(3) // Skip this function and immediate callers
            .take(20) // Only keep the top 20 frames
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Get allocation statistics by type
    pub fn get_stats_by_type(&self) -> HashMap<String, TypeAllocationStats> {
        let mut result = HashMap::new();
        
        if let (Ok(stats), Ok(type_names)) = (self.type_stats.read(), self.type_names.read()) {
            for (type_id, stat) in stats.iter() {
                if let Some(type_name) = type_names.get(type_id) {
                    result.insert(type_name.clone(), stat.clone());
                }
            }
        }
        
        result
    }
    
    /// Get allocation hot spots
    pub fn get_hot_spots(&self, min_count: usize) -> Vec<HotSpot> {
        let mut hot_spots = Vec::new();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        
        // Get hot spots by type
        if let Ok(stats) = self.type_stats.read() {
            for (type_id, stat) in stats.iter() {
                if stat.count >= min_count {
                    if let Ok(type_names) = self.type_names.read() {
                        if let Some(type_name) = type_names.get(type_id) {
                            hot_spots.push(HotSpot {
                                identifier: type_name.clone(),
                                allocation_count: stat.count,
                                total_bytes: stat.total_bytes,
                                allocation_rate: stat.count as f64 / elapsed,
                            });
                        }
                    }
                }
                
                // Add hot spots by location
                for (location, count) in &stat.allocation_sites {
                    if *count >= min_count {
                        // Estimate bytes for this location
                        let location_bytes = (stat.total_bytes as f64 * (*count as f64 / stat.count as f64)) as usize;
                        
                        hot_spots.push(HotSpot {
                            identifier: location.clone(),
                            allocation_count: *count,
                            total_bytes: location_bytes,
                            allocation_rate: *count as f64 / elapsed,
                        });
                    }
                }
            }
        }
        
        // Sort by allocation count (descending)
        hot_spots.sort_by(|a, b| b.allocation_count.cmp(&a.allocation_count));
        
        hot_spots
    }
    
    /// Get allocation patterns
    pub fn get_allocation_patterns(&self) -> Vec<AllocationPattern> {
        let mut patterns = Vec::new();
        
        // This could be a complex analysis, but for simplicity we'll just look for
        // repeated allocations of the same type in sequence
        if let Ok(events) = self.events.lock() {
            if events.len() < 10 { // Need enough events to detect patterns
                return patterns;
            }
            
            // Look for sequences of same-type allocations
            let mut type_runs: HashMap<String, Vec<usize>> = HashMap::new();
            let mut current_type: Option<String> = None;
            let mut current_run = Vec::new();
            
            for (i, event) in events.iter().enumerate() {
                if let Some(ref t) = current_type {
                    if t == &event.type_name {
                        // Continue the run
                        current_run.push(i);
                    } else {
                        // End the run
                        if current_run.len() >= 3 {
                            type_runs.entry(t.clone())
                                .or_insert_with(Vec::new)
                                .push(current_run.len());
                        }
                        
                        // Start a new run
                        current_type = Some(event.type_name.clone());
                        current_run = vec![i];
                    }
                } else {
                    // Start a new run
                    current_type = Some(event.type_name.clone());
                    current_run = vec![i];
                }
            }
            
            // Process the last run
            if let Some(t) = current_type {
                if current_run.len() >= 3 {
                    type_runs.entry(t)
                        .or_insert_with(Vec::new)
                        .push(current_run.len());
                }
            }
            
            // Convert runs to patterns
            for (type_name, runs) in type_runs {
                let total_allocations = runs.iter().sum::<usize>();
                if total_allocations >= 10 { // Only report significant patterns
                    let avg_run_length = runs.iter().sum::<usize>() as f64 / runs.len() as f64;
                    
                    // Estimate memory impact
                    let mut memory_impact = 0;
                    if let Ok(stats) = self.type_stats.read() {
                        for (type_id, stat) in stats.iter() {
                            if let Ok(type_names) = self.type_names.read() {
                                if let Some(name) = type_names.get(type_id) {
                                    if name == &type_name {
                                        memory_impact = stat.avg_allocation as usize * total_allocations;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    // Generate optimization suggestions
                    let mut suggestions = Vec::new();
                    if avg_run_length >= 10.0 {
                        suggestions.push(format!("Consider using a pre-allocated buffer for '{}' instead of individual allocations", type_name));
                    }
                    if runs.len() >= 5 {
                        suggestions.push(format!("Look for loops allocating '{}' and consider moving allocations outside the loop", type_name));
                    }
                    suggestions.push(format!("Check if '{}' allocations can be pooled or reused", type_name));
                    
                    patterns.push(AllocationPattern {
                        description: format!("Repeated allocations of '{}' (avg run length: {:.1})", type_name, avg_run_length),
                        sequence: vec![type_name.clone()],
                        frequency: runs.len(),
                        memory_impact,
                        suggestions,
                    });
                }
            }
            
            // Sort patterns by memory impact
            patterns.sort_by(|a, b| b.memory_impact.cmp(&a.memory_impact));
        }
        
        patterns
    }
    
    /// Generate a report of allocation statistics
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("\n=== Memory Allocation Profile Report ===\n\n");
        
        // Overall statistics
        let elapsed = self.start_time.elapsed();
        report.push_str(&format!("Profile period: {:.2}s\n", elapsed.as_secs_f64()));
        
        let mut total_allocations = 0;
        let mut total_bytes = 0;
        
        if let Ok(stats) = self.type_stats.read() {
            for stat in stats.values() {
                total_allocations += stat.count;
                total_bytes += stat.total_bytes;
            }
        }
        
        report.push_str(&format!("Total allocations: {}\n", total_allocations));
        report.push_str(&format!("Total memory allocated: {} bytes ({:.2} MB)\n", 
                              total_bytes, total_bytes as f64 / (1024.0 * 1024.0)));
        
        if elapsed.as_secs_f64() > 0.0 {
            let alloc_rate = total_allocations as f64 / elapsed.as_secs_f64();
            let bytes_rate = total_bytes as f64 / elapsed.as_secs_f64();
            
            report.push_str(&format!("Allocation rate: {:.2} allocs/sec\n", alloc_rate));
            report.push_str(&format!("Memory allocation rate: {:.2} bytes/sec ({:.2} KB/sec)\n", 
                                  bytes_rate, bytes_rate / 1024.0));
        }
        
        report.push_str("\n--- Top Allocation Types ---\n");
        
        // Get stats by type and sort by total bytes
        let mut type_stats: Vec<(String, TypeAllocationStats)> = Vec::new();
        
        // Create a cloned copy of stats
        if let (Ok(stats), Ok(type_names)) = (self.type_stats.read(), self.type_names.read()) {
            for (type_id, stat) in stats.iter() {
                if let Some(type_name) = type_names.get(type_id) {
                    type_stats.push((type_name.clone(), stat.clone()));
                }
            }
        }
        
        type_stats.sort_by(|a, b| b.1.total_bytes.cmp(&a.1.total_bytes));
        
        for (i, (type_name, stat)) in type_stats.iter().take(10).enumerate() {
            report.push_str(&format!("\n{}. {}\n", i+1, type_name));
            report.push_str(&format!("   Count: {}\n", stat.count));
            report.push_str(&format!("   Total size: {} bytes ({:.2} KB)\n", 
                                  stat.total_bytes, stat.total_bytes as f64 / 1024.0));
            report.push_str(&format!("   Avg size: {:.2} bytes\n", stat.avg_allocation));
            
            if let (Some(first), Some(last)) = (stat.first_seen, stat.last_seen) {
                let duration = last.duration_since(first).as_secs_f64();
                if duration > 0.0 {
                    let rate = stat.count as f64 / duration;
                    report.push_str(&format!("   Allocation rate: {:.2} allocs/sec\n", rate));
                }
            }
            
            // List top allocation sites for this type
            let mut sites: Vec<(&String, &usize)> = stat.allocation_sites.iter().collect();
            sites.sort_by(|a, b| b.1.cmp(a.1));
            
            if !sites.is_empty() {
                report.push_str("   Top allocation sites:\n");
                for (site, count) in sites.iter().take(3) {
                    report.push_str(&format!("     - {} ({} allocs)\n", site, count));
                }
            }
        }
        
        // Hot spots
        report.push_str("\n--- Hot Spots ---\n");
        let hot_spots = self.get_hot_spots(10); // Get hot spots with at least 10 allocations
        
        for (i, hot_spot) in hot_spots.iter().take(10).enumerate() {
            report.push_str(&format!("\n{}. {}\n", i+1, hot_spot.identifier));
            report.push_str(&format!("   Allocations: {}\n", hot_spot.allocation_count));
            report.push_str(&format!("   Total size: {} bytes ({:.2} KB)\n", 
                                  hot_spot.total_bytes, hot_spot.total_bytes as f64 / 1024.0));
            report.push_str(&format!("   Rate: {:.2} allocs/sec\n", hot_spot.allocation_rate));
        }
        
        // Allocation patterns
        report.push_str("\n--- Allocation Patterns ---\n");
        let patterns = self.get_allocation_patterns();
        
        for (i, pattern) in patterns.iter().take(5).enumerate() {
            report.push_str(&format!("\n{}. {}\n", i+1, pattern.description));
            report.push_str(&format!("   Frequency: {} occurrences\n", pattern.frequency));
            report.push_str(&format!("   Memory impact: {} bytes ({:.2} KB)\n", 
                                  pattern.memory_impact, pattern.memory_impact as f64 / 1024.0));
            
            report.push_str("   Suggestions:\n");
            for suggestion in &pattern.suggestions {
                report.push_str(&format!("     - {}\n", suggestion));
            }
        }
        
        report.push_str("\n=== End of Report ===\n");
        
        report
    }
    
    /// Print the allocation report to stdout
    pub fn print_report(&self) {
        println!("{}", self.generate_report());
    }
}

// AllocationProfiler implementation is defined elsewhere

// Singleton instance of the allocation profiler
static ALLOCATION_PROFILER: once_cell::sync::Lazy<Mutex<AllocationProfiler>> =
    once_cell::sync::Lazy::new(|| Mutex::new(AllocationProfiler::new()));

/// Get the global allocation profiler
pub fn global_profiler() -> std::sync::MutexGuard<'static, AllocationProfiler> {
    ALLOCATION_PROFILER.lock().unwrap()
}

/// Enable the global allocation profiler
pub fn enable_profiling() {
    if let Ok(mut profiler) = ALLOCATION_PROFILER.lock() {
        profiler.enable();
    }
}

/// Disable the global allocation profiler
pub fn disable_profiling() {
    if let Ok(mut profiler) = ALLOCATION_PROFILER.lock() {
        profiler.disable();
    }
}

/// Reset the global allocation profiler
pub fn reset_profiling() {
    if let Ok(mut profiler) = ALLOCATION_PROFILER.lock() {
        profiler.reset();
    }
}

/// Generate and print a report from the global allocation profiler
pub fn print_profiling_report() {
    if let Ok(profiler) = ALLOCATION_PROFILER.lock() {
        profiler.print_report();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_allocation_profiler_basic() {
        let mut profiler = AllocationProfiler::new();
        profiler.enable();
        
        // Record some allocations
        profiler.record_allocation::<String>(10, Some("test_location_1".to_string()));
        profiler.record_allocation::<String>(20, Some("test_location_1".to_string()));
        profiler.record_allocation::<i32>(4, Some("test_location_2".to_string()));
        profiler.record_allocation::<Vec<u8>>(100, Some("test_location_3".to_string()));
        
        // Get stats
        let stats = profiler.get_stats_by_type();
        
        // Verify String stats
        assert!(stats.contains_key("alloc::string::String"));
        let string_stats = &stats["alloc::string::String"];
        assert_eq!(string_stats.count, 2);
        assert_eq!(string_stats.total_bytes, 30);
        assert_eq!(string_stats.max_allocation, 20);
        assert_eq!(string_stats.avg_allocation, 15.0);
        
        // Verify i32 stats
        assert!(stats.contains_key("i32"));
        let i32_stats = &stats["i32"];
        assert_eq!(i32_stats.count, 1);
        assert_eq!(i32_stats.total_bytes, 4);
        
        // Verify allocation site tracking
        assert!(string_stats.allocation_sites.contains_key("test_location_1"));
        assert_eq!(string_stats.allocation_sites["test_location_1"], 2);
        
        // Test hot spots
        let hot_spots = profiler.get_hot_spots(1);
        assert!(!hot_spots.is_empty());
        
        // Generate a report
        let report = profiler.generate_report();
        assert!(report.contains("Memory Allocation Profile Report"));
        assert!(report.contains("alloc::string::String"));
    }
    
    #[test]
    fn test_allocation_patterns() {
        let mut profiler = AllocationProfiler::new();
        profiler.enable();
        
        // Create a pattern of String allocations
        for _ in 0..20 {
            profiler.record_allocation::<String>(10, Some("pattern_test".to_string()));
        }
        
        // Mix in some other allocations
        profiler.record_allocation::<i32>(4, None);
        profiler.record_allocation::<i64>(8, None);
        
        // Continue the pattern
        for _ in 0..15 {
            profiler.record_allocation::<String>(15, Some("pattern_test".to_string()));
        }
        
        // Check for patterns
        let patterns = profiler.get_allocation_patterns();
        assert!(!patterns.is_empty());
        
        // The first pattern should be about String allocations
        if !patterns.is_empty() {
            assert!(patterns[0].description.contains("String"));
            assert!(patterns[0].memory_impact > 0);
            assert!(!patterns[0].suggestions.is_empty());
        }
    }
}