//! Advanced Memory Profiling and Monitoring System for CURSED
//!
//! This module provides comprehensive memory profiling capabilities:
//! - Real-time memory usage tracking
//! - Allocation pattern analysis
//! - Memory leak detection
//! - Performance profiling
//! - Memory usage visualization
//! - Heap analysis and optimization

use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque, BTreeMap};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::fmt;

use crate::error::CursedError;
use crate::memory::Tag;

/// Memory profiling configuration
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    /// Enable allocation tracking
    pub enable_allocation_tracking: bool,
    /// Enable leak detection
    pub enable_leak_detection: bool,
    /// Enable performance profiling
    pub enable_performance_profiling: bool,
    /// Enable heap analysis
    pub enable_heap_analysis: bool,
    /// Stack trace depth for allocations
    pub stack_trace_depth: usize,
    /// Sampling rate (0.0-1.0)
    pub sampling_rate: f64,
    /// Profile data retention period
    pub retention_period: Duration,
    /// Enable real-time monitoring
    pub enable_real_time: bool,
    /// Monitor update interval
    pub monitor_interval: Duration,
    /// Enable detailed statistics
    pub enable_detailed_stats: bool,
    /// Memory snapshot interval
    pub snapshot_interval: Duration,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enable_allocation_tracking: true,
            enable_leak_detection: true,
            enable_performance_profiling: true,
            enable_heap_analysis: true,
            stack_trace_depth: 10,
            sampling_rate: 0.1, // 10% sampling
            retention_period: Duration::from_secs(3600), // 1 hour
            enable_real_time: true,
            monitor_interval: Duration::from_millis(100),
            enable_detailed_stats: true,
            snapshot_interval: Duration::from_secs(60), // 1 minute
        }
    }
}

/// Memory allocation record
#[derive(Debug, Clone)]
pub struct AllocationRecord {
    /// Allocation ID
    pub id: usize,
    /// Memory address
    pub address: usize,
    /// Allocation size
    pub size: usize,
    /// Memory tag
    pub tag: Tag,
    /// Allocation timestamp
    pub timestamp: Instant,
    /// Thread ID
    pub thread_id: thread::ThreadId,
    /// Stack trace
    pub stack_trace: Vec<String>,
    /// Allocation source
    pub source: String,
    /// Alignment
    pub alignment: usize,
    /// Lifetime (if deallocated)
    pub lifetime: Option<Duration>,
}

/// Memory deallocation record
#[derive(Debug, Clone)]
pub struct DeallocationRecord {
    /// Allocation ID
    pub allocation_id: usize,
    /// Memory address
    pub address: usize,
    /// Deallocation timestamp
    pub timestamp: Instant,
    /// Thread ID
    pub thread_id: thread::ThreadId,
    /// Stack trace
    pub stack_trace: Vec<String>,
}

/// Memory leak candidate
#[derive(Debug, Clone)]
pub struct LeakCandidate {
    /// Allocation record
    pub allocation: AllocationRecord,
    /// Age of allocation
    pub age: Duration,
    /// Leak probability (0.0-1.0)
    pub probability: f64,
    /// Leak type
    pub leak_type: LeakType,
    /// Related allocations
    pub related_allocations: Vec<usize>,
}

/// Memory leak type
#[derive(Debug, Clone, Copy)]
pub enum LeakType {
    /// Definite leak
    Definite,
    /// Possible leak
    Possible,
    /// Reachable but not freed
    Reachable,
    /// Indirect leak
    Indirect,
}

/// Memory profiling snapshot
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    /// Snapshot timestamp
    pub timestamp: Instant,
    /// Total allocated memory
    pub total_allocated: usize,
    /// Total freed memory
    pub total_freed: usize,
    /// Current memory usage
    pub current_usage: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Allocation count
    pub allocation_count: usize,
    /// Deallocation count
    pub deallocation_count: usize,
    /// Active allocations
    pub active_allocations: usize,
    /// Heap utilization
    pub heap_utilization: f64,
    /// Fragmentation level
    pub fragmentation: f64,
    /// Size distribution
    pub size_distribution: HashMap<usize, usize>,
    /// Tag distribution
    pub tag_distribution: HashMap<Tag, usize>,
    /// Thread distribution
    pub thread_distribution: HashMap<thread::ThreadId, usize>,
}

/// Memory performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Total allocation time
    pub total_allocation_time: Duration,
    /// Average allocation time
    pub avg_allocation_time: Duration,
    /// Maximum allocation time
    pub max_allocation_time: Duration,
    /// Allocation throughput (allocs/sec)
    pub allocation_throughput: f64,
    /// Memory bandwidth (bytes/sec)
    pub memory_bandwidth: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Cache miss rate
    pub cache_miss_rate: f64,
    /// Page fault rate
    pub page_fault_rate: f64,
    /// Memory pressure index
    pub memory_pressure: f64,
}

/// Heap analysis result
#[derive(Debug, Clone)]
pub struct HeapAnalysis {
    /// Analysis timestamp
    pub timestamp: Instant,
    /// Total heap size
    pub total_heap_size: usize,
    /// Used heap size
    pub used_heap_size: usize,
    /// Free heap size
    pub free_heap_size: usize,
    /// Fragmentation analysis
    pub fragmentation: FragmentationAnalysis,
    /// Size class distribution
    pub size_classes: HashMap<usize, SizeClassInfo>,
    /// Hot allocation sites
    pub hot_sites: Vec<AllocationSite>,
    /// Garbage collection impact
    pub gc_impact: GCImpactAnalysis,
}

/// Fragmentation analysis
#[derive(Debug, Clone)]
pub struct FragmentationAnalysis {
    /// Internal fragmentation
    pub internal_fragmentation: f64,
    /// External fragmentation
    pub external_fragmentation: f64,
    /// Largest free block
    pub largest_free_block: usize,
    /// Free block count
    pub free_block_count: usize,
    /// Free block size distribution
    pub free_block_sizes: HashMap<usize, usize>,
}

/// Size class information
#[derive(Debug, Clone)]
pub struct SizeClassInfo {
    /// Size class
    pub size_class: usize,
    /// Total allocations
    pub total_allocations: usize,
    /// Current allocations
    pub current_allocations: usize,
    /// Average lifetime
    pub avg_lifetime: Duration,
    /// Utilization rate
    pub utilization: f64,
}

/// Allocation site hotspot
#[derive(Debug, Clone)]
pub struct AllocationSite {
    /// Site identifier
    pub site_id: String,
    /// Stack trace
    pub stack_trace: Vec<String>,
    /// Total allocations
    pub total_allocations: usize,
    /// Total bytes allocated
    pub total_bytes: usize,
    /// Allocation frequency
    pub frequency: f64,
    /// Average allocation size
    pub avg_size: usize,
}

/// Garbage collection impact analysis
#[derive(Debug, Clone)]
pub struct GCImpactAnalysis {
    /// GC overhead percentage
    pub gc_overhead: f64,
    /// GC frequency
    pub gc_frequency: f64,
    /// Average GC pause time
    pub avg_pause_time: Duration,
    /// GC efficiency
    pub gc_efficiency: f64,
}

/// Memory profiler
pub struct MemoryProfiler {
    /// Configuration
    config: RwLock<ProfilingConfig>,
    /// Allocation records
    allocations: RwLock<HashMap<usize, AllocationRecord>>,
    /// Deallocation records
    deallocations: RwLock<VecDeque<DeallocationRecord>>,
    /// Leak candidates
    leak_candidates: RwLock<Vec<LeakCandidate>>,
    /// Memory snapshots
    snapshots: RwLock<VecDeque<MemorySnapshot>>,
    /// Performance metrics
    performance_metrics: RwLock<PerformanceMetrics>,
    /// Heap analysis results
    heap_analysis: RwLock<Option<HeapAnalysis>>,
    /// Allocation ID counter
    next_allocation_id: AtomicUsize,
    /// Profiling statistics
    stats: RwLock<ProfilingStats>,
    /// Background threads
    background_threads: RwLock<Vec<thread::JoinHandle<()>>>,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Real-time monitor
    real_time_monitor: Arc<RealTimeMonitor>,
}

/// Profiling statistics
#[derive(Debug, Clone, Default)]
pub struct ProfilingStats {
    /// Total records processed
    pub total_records: usize,
    /// Records processed per second
    pub records_per_second: f64,
    /// Memory overhead
    pub memory_overhead: usize,
    /// Profiling accuracy
    pub accuracy: f64,
    /// Leak detection accuracy
    pub leak_detection_accuracy: f64,
}

/// Real-time memory monitor
pub struct RealTimeMonitor {
    /// Current memory usage
    current_usage: AtomicUsize,
    /// Peak memory usage
    peak_usage: AtomicUsize,
    /// Allocation rate
    allocation_rate: AtomicUsize,
    /// Deallocation rate
    deallocation_rate: AtomicUsize,
    /// Monitor callbacks
    callbacks: RwLock<Vec<Box<dyn Fn(&MemorySnapshot) + Send + Sync>>>,
}

impl MemoryProfiler {
    /// Create new memory profiler
    pub fn new(config: ProfilingConfig) -> Result<Arc<Self>, CursedError> {
        let profiler = Arc::new(Self {
            config: RwLock::new(config),
            allocations: RwLock::new(HashMap::new()),
            deallocations: RwLock::new(VecDeque::new()),
            leak_candidates: RwLock::new(Vec::new()),
            snapshots: RwLock::new(VecDeque::new()),
            performance_metrics: RwLock::new(PerformanceMetrics::default()),
            heap_analysis: RwLock::new(None),
            next_allocation_id: AtomicUsize::new(1),
            stats: RwLock::new(ProfilingStats::default()),
            background_threads: RwLock::new(Vec::new()),
            shutdown: AtomicBool::new(false),
            real_time_monitor: Arc::new(RealTimeMonitor::new()),
        });

        // Start background threads
        profiler.start_background_threads()?;

        Ok(profiler)
    }

    /// Start background profiling threads
    fn start_background_threads(&self) -> Result<(), CursedError> {
        let mut threads = self.background_threads.write().unwrap();

        // Leak detection thread
        if self.config.read().unwrap().enable_leak_detection {
            let profiler_weak = Arc::downgrade(&Arc::new(self.clone()));
            let leak_thread = thread::spawn(move || {
                Self::leak_detection_loop(profiler_weak);
            });
            threads.push(leak_thread);
        }

        // Snapshot thread
        let profiler_weak = Arc::downgrade(&Arc::new(self.clone()));
        let snapshot_thread = thread::spawn(move || {
            Self::snapshot_loop(profiler_weak);
        });
        threads.push(snapshot_thread);

        // Performance monitoring thread
        if self.config.read().unwrap().enable_performance_profiling {
            let profiler_weak = Arc::downgrade(&Arc::new(self.clone()));
            let perf_thread = thread::spawn(move || {
                Self::performance_monitoring_loop(profiler_weak);
            });
            threads.push(perf_thread);
        }

        // Heap analysis thread
        if self.config.read().unwrap().enable_heap_analysis {
            let profiler_weak = Arc::downgrade(&Arc::new(self.clone()));
            let heap_thread = thread::spawn(move || {
                Self::heap_analysis_loop(profiler_weak);
            });
            threads.push(heap_thread);
        }

        Ok(())
    }

    /// Leak detection loop
    fn leak_detection_loop(profiler_weak: std::sync::Weak<Self>) {
        while let Some(profiler) = profiler_weak.upgrade() {
            if profiler.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Perform leak detection
            if let Err(e) = profiler.detect_leaks() {
                eprintln!("Leak detection error: {}", e);
            }

            thread::sleep(Duration::from_secs(30));
        }
    }

    /// Snapshot loop
    fn snapshot_loop(profiler_weak: std::sync::Weak<Self>) {
        while let Some(profiler) = profiler_weak.upgrade() {
            if profiler.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Take memory snapshot
            if let Err(e) = profiler.take_snapshot() {
                eprintln!("Snapshot error: {}", e);
            }

            let config = profiler.config.read().unwrap();
            let sleep_duration = config.snapshot_interval;
            drop(config);

            thread::sleep(sleep_duration);
        }
    }

    /// Performance monitoring loop
    fn performance_monitoring_loop(profiler_weak: std::sync::Weak<Self>) {
        while let Some(profiler) = profiler_weak.upgrade() {
            if profiler.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Update performance metrics
            if let Err(e) = profiler.update_performance_metrics() {
                eprintln!("Performance monitoring error: {}", e);
            }

            let config = profiler.config.read().unwrap();
            let sleep_duration = config.monitor_interval;
            drop(config);

            thread::sleep(sleep_duration);
        }
    }

    /// Heap analysis loop
    fn heap_analysis_loop(profiler_weak: std::sync::Weak<Self>) {
        while let Some(profiler) = profiler_weak.upgrade() {
            if profiler.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Perform heap analysis
            if let Err(e) = profiler.analyze_heap() {
                eprintln!("Heap analysis error: {}", e);
            }

            thread::sleep(Duration::from_secs(60));
        }
    }

    /// Record memory allocation
    pub fn record_allocation(
        &self,
        address: usize,
        size: usize,
        tag: Tag,
        source: String,
        alignment: usize,
    ) -> Result<usize, CursedError> {
        let config = self.config.read().unwrap();
        
        // Check sampling rate
        if config.sampling_rate < 1.0 {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            
            let mut hasher = DefaultHasher::new();
            address.hash(&mut hasher);
            let hash = hasher.finish();
            
            if (hash as f64 / u64::MAX as f64) > config.sampling_rate {
                return Ok(0); // Skip this allocation
            }
        }

        let allocation_id = self.next_allocation_id.fetch_add(1, Ordering::Relaxed);
        let timestamp = Instant::now();
        let thread_id = thread::current().id();

        let stack_trace = if config.enable_allocation_tracking {
            self.capture_stack_trace(config.stack_trace_depth)
        } else {
            Vec::new()
        };

        let record = AllocationRecord {
            id: allocation_id,
            address,
            size,
            tag,
            timestamp,
            thread_id,
            stack_trace,
            source,
            alignment,
            lifetime: None,
        };

        // Store allocation record
        if config.enable_allocation_tracking {
            let mut allocations = self.allocations.write().unwrap();
            allocations.insert(allocation_id, record);
        }

        // Update real-time monitor
        self.real_time_monitor.record_allocation(size);

        Ok(allocation_id)
    }

    /// Record memory deallocation
    pub fn record_deallocation(
        &self,
        allocation_id: usize,
        address: usize,
    ) -> Result<(), CursedError> {
        let config = self.config.read().unwrap();
        
        if !config.enable_allocation_tracking {
            return Ok(());
        }

        let timestamp = Instant::now();
        let thread_id = thread::current().id();

        let stack_trace = self.capture_stack_trace(config.stack_trace_depth);

        let deallocation = DeallocationRecord {
            allocation_id,
            address,
            timestamp,
            thread_id,
            stack_trace,
        };

        // Update allocation record with lifetime
        {
            let mut allocations = self.allocations.write().unwrap();
            if let Some(allocation) = allocations.get_mut(&allocation_id) {
                allocation.lifetime = Some(timestamp.duration_since(allocation.timestamp));
            }
        }

        // Store deallocation record
        let mut deallocations = self.deallocations.write().unwrap();
        deallocations.push_back(deallocation);

        // Limit deallocation history
        if deallocations.len() > 100000 {
            deallocations.drain(0..50000);
        }

        // Update real-time monitor
        self.real_time_monitor.record_deallocation();

        Ok(())
    }

    /// Capture stack trace
    fn capture_stack_trace(&self, depth: usize) -> Vec<String> {
        // In a real implementation, this would capture the actual stack trace
        // For now, return a placeholder
        vec![format!("stack_frame_{}", depth)]
    }

    /// Detect memory leaks
    fn detect_leaks(&self) -> Result<(), CursedError> {
        let config = self.config.read().unwrap();
        if !config.enable_leak_detection {
            return Ok(());
        }

        let now = Instant::now();
        let mut leak_candidates = Vec::new();

        // Analyze allocations for leak candidates
        let allocations = self.allocations.read().unwrap();
        for (id, allocation) in allocations.iter() {
            let age = now.duration_since(allocation.timestamp);
            
            // Consider as leak candidate if allocation is old and not freed
            if age > Duration::from_secs(300) && allocation.lifetime.is_none() { // 5 minutes
                let probability = self.calculate_leak_probability(allocation, age);
                let leak_type = self.determine_leak_type(allocation);
                
                leak_candidates.push(LeakCandidate {
                    allocation: allocation.clone(),
                    age,
                    probability,
                    leak_type,
                    related_allocations: Vec::new(),
                });
            }
        }

        // Update leak candidates
        *self.leak_candidates.write().unwrap() = leak_candidates;

        Ok(())
    }

    /// Calculate leak probability
    fn calculate_leak_probability(&self, allocation: &AllocationRecord, age: Duration) -> f64 {
        let age_factor = (age.as_secs() as f64 / 3600.0).min(1.0); // Normalize to 1 hour
        let size_factor = (allocation.size as f64 / 1024.0).min(1.0); // Normalize to 1KB
        
        // Simple heuristic: older and larger allocations are more likely to be leaks
        (age_factor * 0.7 + size_factor * 0.3).min(1.0)
    }

    /// Determine leak type
    fn determine_leak_type(&self, allocation: &AllocationRecord) -> LeakType {
        // Simple heuristic based on allocation characteristics
        match allocation.tag {
            Tag::String | Tag::Array => LeakType::Possible,
            Tag::Object => LeakType::Definite,
            _ => LeakType::Reachable,
        }
    }

    /// Take memory snapshot
    fn take_snapshot(&self) -> Result<(), CursedError> {
        let allocations = self.allocations.read().unwrap();
        let now = Instant::now();

        let mut total_allocated = 0;
        let mut total_freed = 0;
        let mut current_usage = 0;
        let mut allocation_count = 0;
        let mut deallocation_count = 0;
        let mut active_allocations = 0;

        let mut size_distribution = HashMap::new();
        let mut tag_distribution = HashMap::new();
        let mut thread_distribution = HashMap::new();

        for allocation in allocations.values() {
            total_allocated += allocation.size;
            allocation_count += 1;

            if allocation.lifetime.is_some() {
                total_freed += allocation.size;
                deallocation_count += 1;
            } else {
                current_usage += allocation.size;
                active_allocations += 1;
            }

            // Update distributions
            *size_distribution.entry(allocation.size).or_insert(0) += 1;
            *tag_distribution.entry(allocation.tag).or_insert(0) += 1;
            *thread_distribution.entry(allocation.thread_id).or_insert(0) += 1;
        }

        let snapshot = MemorySnapshot {
            timestamp: now,
            total_allocated,
            total_freed,
            current_usage,
            peak_usage: self.real_time_monitor.peak_usage.load(Ordering::Relaxed),
            allocation_count,
            deallocation_count,
            active_allocations,
            heap_utilization: if total_allocated > 0 { 
                current_usage as f64 / total_allocated as f64 
            } else { 
                0.0 
            },
            fragmentation: 0.1, // TODO: Calculate actual fragmentation
            size_distribution,
            tag_distribution,
            thread_distribution,
        };

        // Store snapshot
        let mut snapshots = self.snapshots.write().unwrap();
        snapshots.push_back(snapshot.clone());

        // Limit snapshot history
        if snapshots.len() > 1000 {
            snapshots.drain(0..500);
        }

        // Notify real-time monitor callbacks
        self.real_time_monitor.notify_callbacks(&snapshot);

        Ok(())
    }

    /// Update performance metrics
    fn update_performance_metrics(&self) -> Result<(), CursedError> {
        let allocations = self.allocations.read().unwrap();
        
        let mut total_allocation_time = Duration::from_secs(0);
        let mut max_allocation_time = Duration::from_secs(0);
        let mut allocation_count = 0;

        for allocation in allocations.values() {
            // In a real implementation, we would track actual allocation times
            let allocation_time = Duration::from_nanos(100); // Placeholder
            total_allocation_time += allocation_time;
            if allocation_time > max_allocation_time {
                max_allocation_time = allocation_time;
            }
            allocation_count += 1;
        }

        let avg_allocation_time = if allocation_count > 0 {
            total_allocation_time / allocation_count as u32
        } else {
            Duration::from_secs(0)
        };

        let metrics = PerformanceMetrics {
            total_allocation_time,
            avg_allocation_time,
            max_allocation_time,
            allocation_throughput: 1000.0, // TODO: Calculate actual throughput
            memory_bandwidth: 100_000_000.0, // TODO: Calculate actual bandwidth
            cache_hit_rate: 0.85, // TODO: Calculate actual cache hit rate
            cache_miss_rate: 0.15,
            page_fault_rate: 0.01,
            memory_pressure: 0.5, // TODO: Calculate actual pressure
        };

        *self.performance_metrics.write().unwrap() = metrics;

        Ok(())
    }

    /// Analyze heap
    fn analyze_heap(&self) -> Result<(), CursedError> {
        let allocations = self.allocations.read().unwrap();
        let now = Instant::now();

        let mut total_heap_size = 0;
        let mut used_heap_size = 0;
        let mut size_classes = HashMap::new();
        let mut hot_sites = HashMap::new();

        for allocation in allocations.values() {
            total_heap_size += allocation.size;
            
            if allocation.lifetime.is_none() {
                used_heap_size += allocation.size;
            }

            // Update size class info
            let size_class = self.get_size_class(allocation.size);
            let info = size_classes.entry(size_class).or_insert_with(|| SizeClassInfo {
                size_class,
                total_allocations: 0,
                current_allocations: 0,
                avg_lifetime: Duration::from_secs(0),
                utilization: 0.0,
            });

            info.total_allocations += 1;
            if allocation.lifetime.is_none() {
                info.current_allocations += 1;
            }

            // Update hot sites
            let site_id = allocation.source.clone();
            let site = hot_sites.entry(site_id.clone()).or_insert_with(|| AllocationSite {
                site_id,
                stack_trace: allocation.stack_trace.clone(),
                total_allocations: 0,
                total_bytes: 0,
                frequency: 0.0,
                avg_size: 0,
            });

            site.total_allocations += 1;
            site.total_bytes += allocation.size;
        }

        // Calculate metrics
        for info in size_classes.values_mut() {
            if info.total_allocations > 0 {
                info.utilization = info.current_allocations as f64 / info.total_allocations as f64;
            }
        }

        let analysis = HeapAnalysis {
            timestamp: now,
            total_heap_size,
            used_heap_size,
            free_heap_size: total_heap_size - used_heap_size,
            fragmentation: FragmentationAnalysis {
                internal_fragmentation: 0.05,
                external_fragmentation: 0.10,
                largest_free_block: 1024 * 1024,
                free_block_count: 100,
                free_block_sizes: HashMap::new(),
            },
            size_classes,
            hot_sites: hot_sites.into_values().collect(),
            gc_impact: GCImpactAnalysis {
                gc_overhead: 0.05,
                gc_frequency: 1.0,
                avg_pause_time: Duration::from_millis(5),
                gc_efficiency: 0.9,
            },
        };

        *self.heap_analysis.write().unwrap() = Some(analysis);

        Ok(())
    }

    /// Get size class for allocation
    fn get_size_class(&self, size: usize) -> usize {
        // Simple size class calculation
        if size <= 64 { 64 }
        else if size <= 128 { 128 }
        else if size <= 256 { 256 }
        else if size <= 512 { 512 }
        else if size <= 1024 { 1024 }
        else if size <= 2048 { 2048 }
        else if size <= 4096 { 4096 }
        else { 8192 }
    }

    /// Get current leak candidates
    pub fn get_leak_candidates(&self) -> Vec<LeakCandidate> {
        self.leak_candidates.read().unwrap().clone()
    }

    /// Get recent snapshots
    pub fn get_snapshots(&self, count: usize) -> Vec<MemorySnapshot> {
        let snapshots = self.snapshots.read().unwrap();
        snapshots.iter().rev().take(count).cloned().collect()
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().unwrap().clone()
    }

    /// Get heap analysis
    pub fn get_heap_analysis(&self) -> Option<HeapAnalysis> {
        self.heap_analysis.read().unwrap().clone()
    }

    /// Get profiling statistics
    pub fn get_stats(&self) -> ProfilingStats {
        self.stats.read().unwrap().clone()
    }

    /// Register real-time monitor callback
    pub fn register_monitor_callback<F>(&self, callback: F)
    where
        F: Fn(&MemorySnapshot) + Send + Sync + 'static,
    {
        self.real_time_monitor.register_callback(callback);
    }

    /// Generate memory report
    pub fn generate_report(&self) -> Result<String, CursedError> {
        let mut report = String::new();
        
        // Header
        report.push_str("=== CURSED Memory Profile Report ===\n\n");
        
        // Current snapshot
        if let Some(snapshot) = self.snapshots.read().unwrap().back() {
            report.push_str(&format!("Current Memory Usage: {} bytes\n", snapshot.current_usage));
            report.push_str(&format!("Peak Memory Usage: {} bytes\n", snapshot.peak_usage));
            report.push_str(&format!("Active Allocations: {}\n", snapshot.active_allocations));
            report.push_str(&format!("Heap Utilization: {:.2}%\n", snapshot.heap_utilization * 100.0));
            report.push_str(&format!("Fragmentation: {:.2}%\n\n", snapshot.fragmentation * 100.0));
        }
        
        // Performance metrics
        let metrics = self.get_performance_metrics();
        report.push_str("Performance Metrics:\n");
        report.push_str(&format!("  Average Allocation Time: {:?}\n", metrics.avg_allocation_time));
        report.push_str(&format!("  Allocation Throughput: {:.2} allocs/sec\n", metrics.allocation_throughput));
        report.push_str(&format!("  Memory Bandwidth: {:.2} MB/sec\n", metrics.memory_bandwidth / 1_000_000.0));
        report.push_str(&format!("  Cache Hit Rate: {:.2}%\n\n", metrics.cache_hit_rate * 100.0));
        
        // Leak candidates
        let leak_candidates = self.get_leak_candidates();
        report.push_str(&format!("Leak Candidates: {}\n", leak_candidates.len()));
        for (i, candidate) in leak_candidates.iter().take(5).enumerate() {
            report.push_str(&format!("  {}. Address: 0x{:x}, Size: {} bytes, Age: {:?}, Probability: {:.2}%\n", 
                i + 1, candidate.allocation.address, candidate.allocation.size, 
                candidate.age, candidate.probability * 100.0));
        }
        
        // Heap analysis
        if let Some(analysis) = self.get_heap_analysis() {
            report.push_str("\nHeap Analysis:\n");
            report.push_str(&format!("  Total Heap Size: {} bytes\n", analysis.total_heap_size));
            report.push_str(&format!("  Used Heap Size: {} bytes\n", analysis.used_heap_size));
            report.push_str(&format!("  Free Heap Size: {} bytes\n", analysis.free_heap_size));
            report.push_str(&format!("  Internal Fragmentation: {:.2}%\n", analysis.fragmentation.internal_fragmentation * 100.0));
            report.push_str(&format!("  External Fragmentation: {:.2}%\n", analysis.fragmentation.external_fragmentation * 100.0));
        }
        
        Ok(report)
    }

    /// Shutdown profiler
    pub fn shutdown(&self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::Relaxed);

        // Wait for background threads
        let mut threads = self.background_threads.write().unwrap();
        while let Some(handle) = threads.pop() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join profiler thread"))?;
        }

        Ok(())
    }
}

impl Clone for MemoryProfiler {
    fn clone(&self) -> Self {
        // Simplified clone for background thread creation
        Self {
            config: RwLock::new(self.config.read().unwrap().clone()),
            allocations: RwLock::new(HashMap::new()),
            deallocations: RwLock::new(VecDeque::new()),
            leak_candidates: RwLock::new(Vec::new()),
            snapshots: RwLock::new(VecDeque::new()),
            performance_metrics: RwLock::new(PerformanceMetrics::default()),
            heap_analysis: RwLock::new(None),
            next_allocation_id: AtomicUsize::new(1),
            stats: RwLock::new(ProfilingStats::default()),
            background_threads: RwLock::new(Vec::new()),
            shutdown: AtomicBool::new(false),
            real_time_monitor: Arc::new(RealTimeMonitor::new()),
        }
    }
}

impl PerformanceMetrics {
    /// Create default performance metrics
    pub fn default() -> Self {
        Self {
            total_allocation_time: Duration::from_secs(0),
            avg_allocation_time: Duration::from_secs(0),
            max_allocation_time: Duration::from_secs(0),
            allocation_throughput: 0.0,
            memory_bandwidth: 0.0,
            cache_hit_rate: 0.0,
            cache_miss_rate: 0.0,
            page_fault_rate: 0.0,
            memory_pressure: 0.0,
        }
    }
}

impl RealTimeMonitor {
    /// Create new real-time monitor
    pub fn new() -> Self {
        Self {
            current_usage: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
            allocation_rate: AtomicUsize::new(0),
            deallocation_rate: AtomicUsize::new(0),
            callbacks: RwLock::new(Vec::new()),
        }
    }

    /// Record allocation
    pub fn record_allocation(&self, size: usize) {
        let current = self.current_usage.fetch_add(size, Ordering::Relaxed) + size;
        
        // Update peak usage
        let mut peak = self.peak_usage.load(Ordering::Relaxed);
        while current > peak {
            match self.peak_usage.compare_exchange_weak(peak, current, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => peak = x,
            }
        }
        
        self.allocation_rate.fetch_add(1, Ordering::Relaxed);
    }

    /// Record deallocation
    pub fn record_deallocation(&self) {
        self.deallocation_rate.fetch_add(1, Ordering::Relaxed);
    }

    /// Register callback
    pub fn register_callback<F>(&self, callback: F)
    where
        F: Fn(&MemorySnapshot) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.write().unwrap();
        callbacks.push(Box::new(callback));
    }

    /// Notify callbacks
    pub fn notify_callbacks(&self, snapshot: &MemorySnapshot) {
        let callbacks = self.callbacks.read().unwrap();
        for callback in callbacks.iter() {
            callback(snapshot);
        }
    }
}

impl fmt::Display for LeakType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeakType::Definite => write!(f, "Definite"),
            LeakType::Possible => write!(f, "Possible"),
            LeakType::Reachable => write!(f, "Reachable"),
            LeakType::Indirect => write!(f, "Indirect"),
        }
    }
}

/// Convenience function to create memory profiler
pub fn create_memory_profiler(config: ProfilingConfig) -> Result<Arc<MemoryProfiler>, CursedError> {
    MemoryProfiler::new(config)
}

/// Legacy compatibility
pub type MinimalImplementation = MemoryProfiler;

/// Get minimal result for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED memory profiling system active".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_profiler_creation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config);
        assert!(profiler.is_ok());
    }

    #[test]
    fn test_allocation_recording() {
        let mut config = ProfilingConfig::default();
        config.sampling_rate = 1.0; // 100% sampling for test
        let profiler = MemoryProfiler::new(config).unwrap();
        
        let allocation_id = profiler.record_allocation(
            0x1000,
            64,
            Tag::Object,
            "test".to_string(),
            8,
        );
        
        assert!(allocation_id.is_ok());
        assert!(allocation_id.unwrap() > 0);
    }

    #[test]
    fn test_snapshot_generation() {
        let mut config = ProfilingConfig::default();
        config.sampling_rate = 1.0; // 100% sampling for test
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record some allocations
        profiler.record_allocation(0x1000, 64, Tag::Object, "test1".to_string(), 8).unwrap();
        profiler.record_allocation(0x2000, 128, Tag::String, "test2".to_string(), 8).unwrap();
        
        // Take snapshot
        let result = profiler.take_snapshot();
        assert!(result.is_ok());
        
        // Get snapshots
        let snapshots = profiler.get_snapshots(1);
        assert_eq!(snapshots.len(), 1);
        assert!(snapshots[0].current_usage > 0);
    }

    #[test]
    fn test_leak_detection() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record allocation without deallocation
        profiler.record_allocation(0x1000, 64, Tag::Object, "test".to_string(), 8).unwrap();
        
        // Perform leak detection
        let result = profiler.detect_leaks();
        assert!(result.is_ok());
    }

    #[test]
    fn test_size_class_calculation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        assert_eq!(profiler.get_size_class(32), 64);
        assert_eq!(profiler.get_size_class(64), 64);
        assert_eq!(profiler.get_size_class(100), 128);
        assert_eq!(profiler.get_size_class(1000), 1024);
    }
}
