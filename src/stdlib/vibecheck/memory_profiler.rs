/// Memory Profiler Implementation for CURSED vibecheck
/// 
/// Provides comprehensive memory profiling with heap analysis, allocation tracking,
/// memory leak detection, and stack trace collection for allocations.

use crate::error::CursedError;
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, Duration};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::thread;
use std::fmt;

/// Memory profiler configuration
#[derive(Debug, Clone)]
pub struct MemoryProfilerConfig {
    /// Enable stack trace collection for allocations
    pub stack_traces: bool,
    /// Sample rate for allocation tracking (1 = track all, 10 = track every 10th)
    pub sample_rate: u32,
    /// Minimum allocation size to track
    pub min_tracked_size: usize,
    /// Maximum number of allocation records to keep
    pub max_allocation_records: usize,
    /// Enable leak detection
    pub leak_detection: bool,
    /// Threshold for leak detection (objects alive longer than this)
    pub leak_threshold: Duration,
}

impl Default for MemoryProfilerConfig {
    fn default() -> Self {
        Self {
            stack_traces: true,
            sample_rate: 1,
            min_tracked_size: 64,
            max_allocation_records: 100_000,
            leak_detection: true,
            leak_threshold: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Memory allocation record
#[derive(Debug, Clone)]
pub struct AllocationRecord {
    /// Allocation size in bytes
    pub size: usize,
    /// Thread that performed the allocation
    pub thread_id: thread::ThreadId,
    /// Timestamp of allocation
    pub timestamp: SystemTime,
    /// Stack trace at allocation site
    pub stack_trace: Option<String>,
    /// Object type hint if available
    pub object_type: Option<String>,
    /// Whether this allocation is still active
    pub active: bool,
}

/// Memory leak information
#[derive(Debug, Clone)]
pub struct MemoryLeak {
    /// Allocation record
    pub allocation: AllocationRecord,
    /// How long the object has been alive
    pub age: Duration,
    /// Pointer address for tracking
    pub address: usize,
}

/// Heap analysis results
#[derive(Debug)]
pub struct HeapAnalysis {
    /// Total allocated bytes
    pub total_allocated: u64,
    /// Total freed bytes
    pub total_freed: u64,
    /// Current allocated bytes
    pub current_allocated: u64,
    /// Peak allocated bytes
    pub peak_allocated: u64,
    /// Number of active allocations
    pub active_allocations: usize,
    /// Allocation size distribution
    pub size_distribution: BTreeMap<usize, u64>,
    /// Allocations by thread
    pub thread_allocations: HashMap<thread::ThreadId, u64>,
    /// Memory fragmentation estimate
    pub fragmentation_ratio: f64,
}

/// Allocation pattern analysis
#[derive(Debug)]
pub struct AllocationPattern {
    /// Average allocation size
    pub avg_allocation_size: f64,
    /// Most common allocation sizes
    pub common_sizes: Vec<(usize, u64)>,
    /// Allocation frequency over time
    pub allocation_timeline: Vec<(SystemTime, usize)>,
    /// Hot allocation sites (by stack trace)
    pub hot_sites: Vec<(String, u64)>,
}

/// Memory profiler state
pub struct MemoryProfiler {
    config: MemoryProfilerConfig,
    allocations: Arc<RwLock<HashMap<usize, AllocationRecord>>>,
    allocation_history: Arc<Mutex<Vec<AllocationRecord>>>,
    heap_stats: Arc<Mutex<HeapAnalysis>>,
    leak_candidates: Arc<Mutex<Vec<MemoryLeak>>>,
    sample_counter: Arc<Mutex<u32>>,
}

impl MemoryProfiler {
    /// Create a new memory profiler with default configuration
    pub fn new() -> Self {
        Self::with_config(MemoryProfilerConfig::default())
    }

    /// Create a new memory profiler with custom configuration
    pub fn with_config(config: MemoryProfilerConfig) -> Self {
        Self {
            config,
            allocations: Arc::new(RwLock::new(HashMap::new())),
            allocation_history: Arc::new(Mutex::new(Vec::new())),
            heap_stats: Arc::new(Mutex::new(HeapAnalysis::new())),
            leak_candidates: Arc::new(Mutex::new(Vec::new())),
            sample_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Record a new allocation
    pub fn record_allocation(&self, address: usize, size: usize, object_type: Option<String>) -> crate::error::Result<()> {
        if size < self.config.min_tracked_size {
            return Ok(());
        }

        // Sample allocation based on sample rate
        let should_sample = {
            let mut counter = self.sample_counter.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock sample counter".to_string()))?;
            *counter += 1;
            *counter % self.config.sample_rate == 0
        };

        if !should_sample {
            return Ok(());
        }

        let stack_trace = if self.config.stack_traces {
            Some(self.capture_stack_trace())
        } else {
            None
        };

        let record = AllocationRecord {
            size,
            thread_id: thread::current().id(),
            timestamp: SystemTime::now(),
            stack_trace,
            object_type,
            active: true,
        };

        // Update active allocations
        {
            let mut allocations = self.allocations.write()
                .map_err(|_| CursedError::Runtime("Failed to lock allocations".to_string()))?;
            allocations.insert(address, record.clone());
        }

        // Update allocation history
        {
            let mut history = self.allocation_history.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock allocation history".to_string()))?;
            
            if history.len() >= self.config.max_allocation_records {
                history.remove(0); // Remove oldest record
            }
            history.push(record);
        }

        // Update heap statistics
        self.update_heap_stats(size as i64)?;

        Ok(())
    }

    /// Record a deallocation
    pub fn record_deallocation(&self, address: usize) -> crate::error::Result<()> {
        let size = {
            let mut allocations = self.allocations.write()
                .map_err(|_| CursedError::Runtime("Failed to lock allocations".to_string()))?;
            
            if let Some(mut record) = allocations.remove(&address) {
                record.active = false;
                record.size
            } else {
                return Ok(()); // Unknown allocation, possibly not sampled
            }
        };

        // Update heap statistics
        self.update_heap_stats(-(size as i64))?;

        Ok(())
    }

    /// Perform heap analysis
    pub fn analyze_heap(&self) -> crate::error::Result<()> {
        let heap_stats = self.heap_stats.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock heap stats".to_string()))?;
        
        let allocations = self.allocations.read()
            .map_err(|_| CursedError::Runtime("Failed to lock allocations".to_string()))?;

        let mut size_distribution = BTreeMap::new();
        let mut thread_allocations = HashMap::new();
        let mut total_size = 0u64;

        for record in allocations.values() {
            if record.active {
                *size_distribution.entry(record.size).or_insert(0) += 1;
                *thread_allocations.entry(record.thread_id).or_insert(0) += record.size as u64;
                total_size += record.size as u64;
            }
        }

        // Calculate fragmentation estimate (simplified)
        let fragmentation_ratio = if heap_stats.peak_allocated > 0 {
            1.0 - (heap_stats.current_allocated as f64 / heap_stats.peak_allocated as f64)
        } else {
            0.0
        };

        Ok(HeapAnalysis {
            total_allocated: heap_stats.total_allocated,
            total_freed: heap_stats.total_freed,
            current_allocated: heap_stats.current_allocated,
            peak_allocated: heap_stats.peak_allocated,
            active_allocations: allocations.len(),
            size_distribution,
            thread_allocations,
            fragmentation_ratio,
        })
    }

    /// Detect memory leaks
    pub fn detect_leaks(&self) -> crate::error::Result<()> {
        if !self.config.leak_detection {
            return Ok(Vec::new());
        }

        let now = SystemTime::now();
        let allocations = self.allocations.read()
            .map_err(|_| CursedError::Runtime("Failed to lock allocations".to_string()))?;

        let mut leaks = Vec::new();

        for (&address, record) in allocations.iter() {
            if record.active {
                if let Ok(age) = now.duration_since(record.timestamp) {
                    if age > self.config.leak_threshold {
                        leaks.push(MemoryLeak {
                            allocation: record.clone(),
                            age,
                            address,
                        });
                    }
                }
            }
        }

        // Sort by age (oldest first)
        leaks.sort_by(|a, b| b.age.cmp(&a.age));

        // Update leak candidates
        {
            let mut candidates = self.leak_candidates.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock leak candidates".to_string()))?;
            *candidates = leaks.clone();
        }

        Ok(leaks)
    }

    /// Analyze allocation patterns
    pub fn analyze_patterns(&self) -> crate::error::Result<()> {
        let history = self.allocation_history.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock allocation history".to_string()))?;

        if history.is_empty() {
            return Ok(AllocationPattern {
                avg_allocation_size: 0.0,
                common_sizes: Vec::new(),
                allocation_timeline: Vec::new(),
                hot_sites: Vec::new(),
            });
        }

        // Calculate average allocation size
        let total_size: usize = history.iter().map(|r| r.size).sum();
        let avg_allocation_size = total_size as f64 / history.len() as f64;

        // Find common allocation sizes
        let mut size_counts = HashMap::new();
        for record in history.iter() {
            *size_counts.entry(record.size).or_insert(0) += 1;
        }
        let mut common_sizes: Vec<_> = size_counts.into_iter().collect();
        common_sizes.sort_by(|a, b| b.1.cmp(&a.1));
        common_sizes.truncate(10); // Top 10

        // Create allocation timeline
        let mut allocation_timeline = Vec::new();
        for record in history.iter() {
            allocation_timeline.push((record.timestamp, record.size));
        }

        // Find hot allocation sites
        let mut site_counts = HashMap::new();
        for record in history.iter() {
            if let Some(ref trace) = record.stack_trace {
                // Extract function name from stack trace (simplified)
                let function = trace.split("\n").nth(2)
                    .unwrap_or("unknown")
                    .trim()
                    .to_string();
                *site_counts.entry(function).or_insert(0) += 1;
            }
        }
        let mut hot_sites: Vec<_> = site_counts.into_iter().collect();
        hot_sites.sort_by(|a, b| b.1.cmp(&a.1));
        hot_sites.truncate(10); // Top 10

        Ok(AllocationPattern {
            avg_allocation_size,
            common_sizes,
            allocation_timeline,
            hot_sites,
        })
    }

    /// Get memory usage statistics
    pub fn get_memory_stats(&self) -> crate::error::Result<()> {
        let heap_analysis = self.analyze_heap()?;
        let leaks = self.detect_leaks()?;
        let patterns = self.analyze_patterns()?;

        Ok(MemoryStats {
            heap_analysis,
            patterns,
            leak_count: leaks.len(),
            total_leaked_bytes: leaks.iter().map(|l| l.allocation.size).sum::<usize>() as u64,
        })
    }

    /// Clear all profiling data
    pub fn clear(&self) -> crate::error::Result<()> {
        {
            let mut allocations = self.allocations.write()
                .map_err(|_| CursedError::Runtime("Failed to lock allocations".to_string()))?;
            allocations.clear();
        }

        {
            let mut history = self.allocation_history.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock allocation history".to_string()))?;
            history.clear();
        }

        {
            let mut heap_stats = self.heap_stats.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock heap stats".to_string()))?;
            *heap_stats = HeapAnalysis::new();
        }

        {
            let mut leak_candidates = self.leak_candidates.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock leak candidates".to_string()))?;
            leak_candidates.clear();
        }

        {
            let mut counter = self.sample_counter.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock sample counter".to_string()))?;
            *counter = 0;
        }

        Ok(())
    }

    /// Update heap statistics
    fn update_heap_stats(&self, size_delta: i64) -> crate::error::Result<()> {
        let mut heap_stats = self.heap_stats.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock heap stats".to_string()))?;

        if size_delta > 0 {
            heap_stats.total_allocated += size_delta as u64;
            heap_stats.current_allocated += size_delta as u64;
            if heap_stats.current_allocated > heap_stats.peak_allocated {
                heap_stats.peak_allocated = heap_stats.current_allocated;
            }
        } else {
            heap_stats.total_freed += (-size_delta) as u64;
            heap_stats.current_allocated = heap_stats.current_allocated.saturating_sub((-size_delta) as u64);
        }

        Ok(())
    }

    /// Capture stack trace for allocation site
    fn capture_stack_trace(&self) -> String {
        let backtrace = Backtrace::capture();
        match backtrace.status() {
            BacktraceStatus::Captured => {
                format!("{}", backtrace)
            }
            _ => "Stack trace unavailable".to_string()
        }
    }
}

/// Combined memory statistics
#[derive(Debug)]
pub struct MemoryStats {
    pub heap_analysis: HeapAnalysis,
    pub patterns: AllocationPattern,
    pub leak_count: usize,
    pub total_leaked_bytes: u64,
}

impl HeapAnalysis {
    fn new() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
            current_allocated: 0,
            peak_allocated: 0,
            active_allocations: 0,
            size_distribution: BTreeMap::new(),
            thread_allocations: HashMap::new(),
            fragmentation_ratio: 0.0,
        }
    }
}

impl fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== CURSED Memory Profile ===")?;
        writeln!(f)?;
        writeln!(f, "Heap Analysis:")?;
        writeln!(f, "  Current allocated: {} bytes", self.heap_analysis.current_allocated)?;
        writeln!(f, "  Peak allocated: {} bytes", self.heap_analysis.peak_allocated)?;
        writeln!(f, "  Total allocated: {} bytes", self.heap_analysis.total_allocated)?;
        writeln!(f, "  Total freed: {} bytes", self.heap_analysis.total_freed)?;
        writeln!(f, "  Active allocations: {}", self.heap_analysis.active_allocations)?;
        writeln!(f, "  Fragmentation ratio: {:.2}%", self.heap_analysis.fragmentation_ratio * 100.0)?;
        writeln!(f)?;

        writeln!(f, "Allocation Patterns:")?;
        writeln!(f, "  Average allocation size: {:.2} bytes", self.patterns.avg_allocation_size)?;
        writeln!(f, "  Common allocation sizes:")?;
        for (size, count) in &self.patterns.common_sizes {
            writeln!(f, "    {} bytes: {} allocations", size, count)?;
        }
        writeln!(f)?;

        writeln!(f, "Memory Leaks:")?;
        writeln!(f, "  Potential leaks: {}", self.leak_count)?;
        writeln!(f, "  Total leaked bytes: {}", self.total_leaked_bytes)?;
        writeln!(f)?;

        writeln!(f, "Hot Allocation Sites:")?;
        for (site, count) in &self.patterns.hot_sites {
            writeln!(f, "  {}: {} allocations", site, count)?;
        }

        Ok(())
    }
}

/// Global memory profiler instance
static GLOBAL_MEMORY_PROFILER: std::sync::OnceLock<Arc<MemoryProfiler>> = std::sync::OnceLock::new();

/// Get or create the global memory profiler
pub fn get_memory_profiler() -> Arc<MemoryProfiler> {
    GLOBAL_MEMORY_PROFILER.get_or_init(|| {
        Arc::new(MemoryProfiler::new())
    }).clone()
}

/// Set custom memory profiler configuration
pub fn configure_memory_profiler(config: MemoryProfilerConfig) -> crate::error::Result<()> {
    let profiler = Arc::new(MemoryProfiler::with_config(config));
    GLOBAL_MEMORY_PROFILER.set(profiler)
        .map_err(|_| CursedError::Runtime("Memory profiler already configured".to_string()))?;
    Ok(())
}

/// Record allocation (called by allocator hooks)
pub fn profile_allocation(address: usize, size: usize, object_type: Option<String>) -> crate::error::Result<()> {
    let profiler = get_memory_profiler();
    profiler.record_allocation(address, size, object_type)
}

/// Record deallocation (called by allocator hooks)
pub fn profile_deallocation(address: usize) -> crate::error::Result<()> {
    let profiler = get_memory_profiler();
    profiler.record_deallocation(address)
}

/// Get current memory statistics
pub fn memory_profile() -> crate::error::Result<()> {
    let profiler = get_memory_profiler();
    profiler.get_memory_stats()
}

/// Detect memory leaks
pub fn detect_memory_leaks() -> crate::error::Result<()> {
    let profiler = get_memory_profiler();
    profiler.detect_leaks()
}

/// Clear profiling data
pub fn clear_memory_profile() -> crate::error::Result<()> {
    let profiler = get_memory_profiler();
    profiler.clear()
}

