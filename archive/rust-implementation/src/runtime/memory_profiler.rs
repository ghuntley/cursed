/// Memory Profiler for CURSED Runtime
/// 
/// Provides comprehensive memory profiling and leak detection capabilities
/// for production applications.

use std::sync::{Arc, Mutex, RwLock};
use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::runtime::gc::{GarbageCollector, GcStats, HeapObject, ObjectMetadata};
use crate::runtime::stack::{RuntimeStack, StackId};
use crate::error::CursedError;

/// Memory profiling configuration
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    /// Enable allocation tracking
    pub track_allocations: bool,
    /// Enable leak detection
    pub leak_detection: bool,
    /// Leak detection threshold in seconds
    pub leak_threshold_seconds: u64,
    /// Maximum number of allocation traces to keep
    pub max_allocation_traces: usize,
    /// Enable stack trace collection
    pub collect_stack_traces: bool,
    /// Profiling sampling rate (1 = every allocation, 10 = every 10th allocation)
    pub sampling_rate: usize,
    /// Enable heap fragmentation analysis
    pub fragmentation_analysis: bool,
    /// Enable generational analysis
    pub generational_analysis: bool,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            track_allocations: true,
            leak_detection: true,
            leak_threshold_seconds: 300, // 5 minutes
            max_allocation_traces: 100_000,
            collect_stack_traces: true,
            sampling_rate: 1,
            fragmentation_analysis: true,
            generational_analysis: true,
        }
    }
}

/// Allocation information for profiling
#[derive(Debug, Clone, serde::Serialize)]
pub struct AllocationInfo {
    /// Object address
    pub address: usize,
    /// Object size
    pub size: usize,
    /// Allocation timestamp
    #[serde(skip)]
    pub timestamp: Instant,
    /// Stack trace at allocation
    pub stack_trace: Option<Vec<String>>,
    /// Object tag/type
    #[serde(skip)]
    pub object_tag: crate::memory::Tag,
    /// Thread ID that allocated
    #[serde(skip)]
    pub thread_id: Option<thread::ThreadId>,
    /// Stack ID (for goroutine allocations)
    pub stack_id: Option<StackId>,
    /// Still alive flag
    pub is_alive: bool,
}

impl Default for AllocationInfo {
    fn default() -> Self {
        Self {
            address: 0,
            size: 0,
            timestamp: Instant::now(),
            stack_trace: None,
            object_tag: crate::memory::Tag::Object,
            thread_id: None,
            stack_id: None,
            is_alive: false,
        }
    }
}

/// Memory leak information
#[derive(Debug, Clone, serde::Serialize)]
pub struct LeakInfo {
    /// Allocation information
    pub allocation: AllocationInfo,
    /// Age of the allocation
    pub age: Duration,
    /// Suspected leak type
    pub leak_type: LeakType,
    /// Number of similar leaks
    pub similar_count: usize,
    /// Total size of similar leaks
    pub total_size: usize,
}

/// Types of memory leaks
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum LeakType {
    /// Long-lived object that should have been collected
    LongLived,
    /// Growing data structure that's not being cleaned up
    Growing,
    /// Circular reference preventing collection
    CircularReference,
    /// Global reference preventing collection
    GlobalReference,
    /// Stack reference preventing collection
    StackReference,
    /// Unknown leak type
    Unknown,
}

/// Memory profiling statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfilingStats {
    /// Total allocations tracked
    pub total_allocations: u64,
    /// Total deallocations tracked
    pub total_deallocations: u64,
    /// Current live allocations
    pub live_allocations: u64,
    /// Total bytes allocated
    pub total_bytes_allocated: u64,
    /// Total bytes deallocated
    pub total_bytes_deallocated: u64,
    /// Current bytes in use
    pub current_bytes: u64,
    /// Peak memory usage
    pub peak_memory: u64,
    /// Detected leaks
    pub detected_leaks: Vec<LeakInfo>,
    /// Allocation rate (allocations per second)
    pub allocation_rate: f64,
    /// Deallocation rate (deallocations per second)
    pub deallocation_rate: f64,
    /// Average object lifetime
    pub avg_object_lifetime: Duration,
    /// Heap fragmentation percentage
    pub heap_fragmentation: f64,
}

/// Memory profiler
pub struct MemoryProfiler {
    /// Configuration
    config: ProfilingConfig,
    /// Active allocations
    allocations: RwLock<HashMap<usize, AllocationInfo>>,
    /// Allocation history
    allocation_history: RwLock<VecDeque<AllocationInfo>>,
    /// Profiling statistics
    stats: RwLock<ProfilingStats>,
    /// Leak detection thread handle
    leak_detector: Mutex<Option<thread::JoinHandle<()>>>,
    /// Running flag
    running: AtomicBool,
    /// Sampling counter
    sampling_counter: AtomicUsize,
    /// GC reference for heap analysis
    gc_ref: Option<Arc<GarbageCollector>>,
    /// Stack manager reference
    stack_manager: Option<Arc<RuntimeStack>>,
}

impl MemoryProfiler {
    /// Create new memory profiler with default config
    pub fn new() -> Self {
        Self::new_with_config(ProfilingConfig::default())
    }

    /// Create new memory profiler
    pub fn new_with_config(config: ProfilingConfig) -> Self {
        let stats = ProfilingStats {
            total_allocations: 0,
            total_deallocations: 0,
            live_allocations: 0,
            total_bytes_allocated: 0,
            total_bytes_deallocated: 0,
            current_bytes: 0,
            peak_memory: 0,
            detected_leaks: Vec::new(),
            allocation_rate: 0.0,
            deallocation_rate: 0.0,
            avg_object_lifetime: Duration::from_secs(0),
            heap_fragmentation: 0.0,
        };

        Self {
            config,
            allocations: RwLock::new(HashMap::new()),
            allocation_history: RwLock::new(VecDeque::new()),
            stats: RwLock::new(stats),
            leak_detector: Mutex::new(None),
            running: AtomicBool::new(false),
            sampling_counter: AtomicUsize::new(0),
            gc_ref: None,
            stack_manager: None,
        }
    }

    /// Start profiling
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Profiler already running"));
        }

        // Start leak detection thread if enabled
        if self.config.leak_detection {
            let profiler = unsafe { std::ptr::read(self) };
            let handle = thread::spawn(move || {
                profiler.leak_detection_loop();
            });
            *self.leak_detector.lock().unwrap() = Some(handle);
        }

        Ok(())
    }

    /// Stop profiling
    pub fn stop(&self) -> Result<(), CursedError> {
        self.running.store(false, Ordering::Relaxed);

        // Wait for leak detection thread
        if let Some(handle) = self.leak_detector.lock().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join leak detection thread"))?;
        }

        Ok(())
    }

    /// Record allocation
    pub fn record_allocation(
        &self,
        address: usize,
        size: usize,
        object_tag: crate::memory::Tag,
        stack_id: Option<StackId>,
    ) -> Result<(), CursedError> {
        if !self.config.track_allocations {
            return Ok(());
        }

        // Check sampling rate
        let counter = self.sampling_counter.fetch_add(1, Ordering::Relaxed);
        if counter % self.config.sampling_rate != 0 {
            return Ok(());
        }

        let stack_trace = if self.config.collect_stack_traces {
            Some(self.collect_stack_trace())
        } else {
            None
        };

        let allocation_info = AllocationInfo {
            address,
            size,
            timestamp: Instant::now(),
            stack_trace,
            object_tag,
            thread_id: Some(thread::current().id()),
            stack_id,
            is_alive: true,
        };

        // Add to active allocations
        {
            let mut allocations = self.allocations.write().unwrap();
            allocations.insert(address, allocation_info.clone());
        }

        // Add to history
        {
            let mut history = self.allocation_history.write().unwrap();
            history.push_back(allocation_info);

            // Limit history size
            if history.len() > self.config.max_allocation_traces {
                history.pop_front();
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_allocations += 1;
            stats.live_allocations += 1;
            stats.total_bytes_allocated += size as u64;
            stats.current_bytes += size as u64;

            if stats.current_bytes > stats.peak_memory {
                stats.peak_memory = stats.current_bytes;
            }
        }

        Ok(())
    }

    /// Record deallocation
    pub fn record_deallocation(&self, address: usize) -> Result<(), CursedError> {
        if !self.config.track_allocations {
            return Ok(());
        }

        let mut allocations = self.allocations.write().unwrap();
        if let Some(mut allocation_info) = allocations.remove(&address) {
            allocation_info.is_alive = false;

            // Update statistics
            {
                let mut stats = self.stats.write().unwrap();
                stats.total_deallocations += 1;
                stats.live_allocations = stats.live_allocations.saturating_sub(1);
                stats.total_bytes_deallocated += allocation_info.size as u64;
                stats.current_bytes = stats.current_bytes.saturating_sub(allocation_info.size as u64);

                // Update average object lifetime
                let lifetime = allocation_info.timestamp.elapsed();
                let total_lifetime = stats.avg_object_lifetime.as_secs_f64() * (stats.total_deallocations - 1) as f64;
                stats.avg_object_lifetime = Duration::from_secs_f64(
                    (total_lifetime + lifetime.as_secs_f64()) / stats.total_deallocations as f64
                );
            }
        }

        Ok(())
    }

    /// Get profiling statistics
    pub fn get_stats(&self) -> ProfilingStats {
        let mut stats = self.stats.read().unwrap().clone();
        
        // Update rates
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(
            current_time - Duration::from_secs(1)
        );
        
        if elapsed.as_secs() > 0 {
            stats.allocation_rate = stats.total_allocations as f64 / elapsed.as_secs_f64();
            stats.deallocation_rate = stats.total_deallocations as f64 / elapsed.as_secs_f64();
        }

        // Update heap fragmentation if GC is available
        if let Some(gc) = &self.gc_ref {
            let gc_stats = gc.get_stats();
            if let Ok(gc_stats) = gc_stats {
                // Convert GCStats to GcStats for fragmentation calculation
                let converted_stats = crate::memory::gc::GcStats {
                    total_collections: gc_stats.total_collections,
                    total_time_ms: gc_stats.average_collection_time.as_millis() as u64,
                    objects_collected: gc_stats.objects_swept,
                    bytes_collected: 0,
                    last_collection_time_ms: gc_stats.average_collection_time.as_millis() as u64,
                    last_objects_collected: gc_stats.objects_swept as usize,
                    avg_pause_time: gc_stats.average_collection_time,
                    max_pause_time: gc_stats.average_collection_time,
                    gc_overhead: 0.0,
                    heap_utilization: 0.0,
                    allocation_rate: 0.0,
                    total_gc_time: gc_stats.average_collection_time,
                };
                stats.heap_fragmentation = self.calculate_heap_fragmentation(&converted_stats);
            }
        }

        stats
    }

    /// Analyze for memory leaks
    pub fn analyze_leaks(&self) -> Result<Vec<LeakInfo>, CursedError> {
        let mut leaks = Vec::new();
        let threshold = Duration::from_secs(self.config.leak_threshold_seconds);
        let now = Instant::now();

        let allocations = self.allocations.read().unwrap();
        
        for (address, allocation_info) in allocations.iter() {
            let age = now.duration_since(allocation_info.timestamp);
            
            if age > threshold {
                let leak_type = self.classify_leak(allocation_info);
                let similar_count = self.count_similar_leaks(&allocations, allocation_info);
                let total_size = self.calculate_similar_leak_size(&allocations, allocation_info);
                
                let leak_info = LeakInfo {
                    allocation: allocation_info.clone(),
                    age,
                    leak_type,
                    similar_count,
                    total_size,
                };
                
                leaks.push(leak_info);
            }
        }

        // Sort leaks by size (largest first)
        leaks.sort_by(|a, b| b.total_size.cmp(&a.total_size));

        Ok(leaks)
    }

    /// Generate memory usage report
    pub fn generate_report(&self) -> Result<String, CursedError> {
        let stats = self.get_stats();
        let leaks = self.analyze_leaks()?;

        let mut report = String::new();
        
        report.push_str("=== Memory Profiling Report ===\n\n");
        
        // Statistics
        report.push_str(&format!("Total Allocations: {}\n", stats.total_allocations));
        report.push_str(&format!("Total Deallocations: {}\n", stats.total_deallocations));
        report.push_str(&format!("Live Allocations: {}\n", stats.live_allocations));
        report.push_str(&format!("Current Memory Usage: {} bytes\n", stats.current_bytes));
        report.push_str(&format!("Peak Memory Usage: {} bytes\n", stats.peak_memory));
        report.push_str(&format!("Allocation Rate: {:.2}/sec\n", stats.allocation_rate));
        report.push_str(&format!("Deallocation Rate: {:.2}/sec\n", stats.deallocation_rate));
        report.push_str(&format!("Average Object Lifetime: {:.2}s\n", stats.avg_object_lifetime.as_secs_f64()));
        report.push_str(&format!("Heap Fragmentation: {:.2}%\n", stats.heap_fragmentation));
        
        // Leaks
        if !leaks.is_empty() {
            report.push_str("\n=== Detected Memory Leaks ===\n");
            for (i, leak) in leaks.iter().enumerate().take(10) {
                report.push_str(&format!("{}. Address: 0x{:x}, Size: {} bytes, Age: {:.2}s, Type: {:?}\n",
                    i + 1, leak.allocation.address, leak.total_size, leak.age.as_secs_f64(), leak.leak_type));
                
                if let Some(stack_trace) = &leak.allocation.stack_trace {
                    report.push_str("   Stack Trace:\n");
                    for frame in stack_trace.iter().take(5) {
                        report.push_str(&format!("     {}\n", frame));
                    }
                }
            }
        }

        Ok(report)
    }

    /// Leak detection loop
    fn leak_detection_loop(&self) {
        let start_time = std::time::Instant::now();
        let max_runtime = Duration::from_secs(300); // Maximum 5 minutes
        
        while self.running.load(Ordering::Relaxed) {
            // Safety timeout to prevent infinite loops
            if start_time.elapsed() > max_runtime {
                println!("Leak detection loop timed out after 5 minutes, stopping");
                break;
            }
            
            if let Ok(leaks) = self.analyze_leaks() {
                if !leaks.is_empty() {
                    let mut stats = self.stats.write().unwrap();
                    stats.detected_leaks = leaks;
                }
            }

            // Use shorter sleep interval and check running flag more frequently
            for _ in 0..60 {
                if !self.running.load(Ordering::Relaxed) {
                    return;
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    /// Collect stack trace
    fn collect_stack_trace(&self) -> Vec<String> {
        // This is a simplified stack trace collection
        // In a real implementation, you'd use backtrace or similar
        vec![
            "cursed::runtime::allocate".to_string(),
            "cursed::main".to_string(),
        ]
    }

    /// Classify leak type
    fn classify_leak(&self, allocation_info: &AllocationInfo) -> LeakType {
        // Simple heuristic-based classification
        if allocation_info.size > 1024 * 1024 {
            LeakType::Growing
        } else if allocation_info.timestamp.elapsed() > Duration::from_secs(3600) {
            LeakType::LongLived
        } else if allocation_info.stack_id.is_some() {
            LeakType::StackReference
        } else {
            LeakType::Unknown
        }
    }

    /// Count similar leaks
    fn count_similar_leaks(&self, allocations: &HashMap<usize, AllocationInfo>, target: &AllocationInfo) -> usize {
        allocations.values()
            .filter(|alloc| {
                alloc.object_tag == target.object_tag && 
                alloc.size == target.size &&
                alloc.thread_id == target.thread_id
            })
            .count()
    }

    /// Calculate total size of similar leaks
    fn calculate_similar_leak_size(&self, allocations: &HashMap<usize, AllocationInfo>, target: &AllocationInfo) -> usize {
        allocations.values()
            .filter(|alloc| {
                alloc.object_tag == target.object_tag && 
                alloc.size == target.size &&
                alloc.thread_id == target.thread_id
            })
            .map(|alloc| alloc.size)
            .sum()
    }

    /// Calculate heap fragmentation
    fn calculate_heap_fragmentation(&self, gc_stats: &GcStats) -> f64 {
        // Simple fragmentation calculation based on GC statistics
        if gc_stats.total_collections == 0 {
            return 0.0;
        }

        // Use collection frequency as a proxy for fragmentation
        let collection_rate = gc_stats.total_collections as f64 / gc_stats.total_gc_time.as_secs_f64();
        (collection_rate * 100.0).min(100.0)
    }

    /// Set GC reference for heap analysis
    pub fn set_gc_ref(&mut self, gc: Arc<GarbageCollector>) {
        self.gc_ref = Some(gc);
    }

    /// Set stack manager reference
    pub fn set_stack_manager(&mut self, stack_manager: Arc<RuntimeStack>) {
        self.stack_manager = Some(stack_manager);
    }
}

/// Global memory profiler instance
static GLOBAL_PROFILER: Lazy<std::sync::Mutex<MemoryProfiler>> = Lazy::new(|| std::sync::Mutex::new(MemoryProfiler::new()));
static PROFILER_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global memory profiler
pub fn initialize_profiler(config: ProfilingConfig) -> Result<(), CursedError> {
    // Initialization is handled automatically by Lazy
    Ok(())
}

/// Get global memory profiler
pub fn get_profiler() -> std::sync::MutexGuard<'static, MemoryProfiler> {
    GLOBAL_PROFILER.lock().unwrap()
}

/// Record allocation using global profiler
pub fn record_allocation(
    address: usize,
    size: usize,
    object_tag: crate::memory::Tag,
    stack_id: Option<StackId>,
) -> Result<(), CursedError> {
    let mut profiler = get_profiler();
    profiler.record_allocation(address, size, object_tag, stack_id)?;
    Ok(())
}

/// Record deallocation using global profiler
pub fn record_deallocation(address: usize) -> Result<(), CursedError> {
    let mut profiler = get_profiler();
    profiler.record_deallocation(address)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Tag;

    #[test]
    fn test_profiler_creation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new();
        
        let stats = profiler.get_stats();
        assert_eq!(stats.total_allocations, 0);
        assert_eq!(stats.live_allocations, 0);
    }

    #[test]
    fn test_allocation_recording() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new();
        
        profiler.record_allocation(0x1000, 64, Tag::Object, None).unwrap();
        
        let stats = profiler.get_stats();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.live_allocations, 1);
        assert_eq!(stats.current_bytes, 64);
    }

    #[test]
    fn test_deallocation_recording() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new();
        
        profiler.record_allocation(0x1000, 64, Tag::Object, None).unwrap();
        profiler.record_deallocation(0x1000).unwrap();
        
        let stats = profiler.get_stats();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.total_deallocations, 1);
        assert_eq!(stats.live_allocations, 0);
        assert_eq!(stats.current_bytes, 0);
    }

    #[test]
    fn test_leak_detection() {
        let config = ProfilingConfig {
            leak_threshold_seconds: 0, // Immediate leak detection
            ..Default::default()
        };
        let profiler = MemoryProfiler::new();
        
        profiler.record_allocation(0x1000, 64, Tag::Object, None).unwrap();
        
        // Wait a bit to ensure leak detection triggers
        thread::sleep(Duration::from_millis(10));
        
        let leaks = profiler.analyze_leaks().unwrap();
        assert_eq!(leaks.len(), 1);
        assert_eq!(leaks[0].allocation.address, 0x1000);
    }

    #[test]
    fn test_report_generation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new();
        
        profiler.record_allocation(0x1000, 64, Tag::Object, None).unwrap();
        profiler.record_allocation(0x2000, 128, Tag::String, None).unwrap();
        
        let report = profiler.generate_report().unwrap();
        assert!(report.contains("Memory Profiling Report"));
        assert!(report.contains("Total Allocations: 2"));
        assert!(report.contains("Live Allocations: 2"));
    }
}
