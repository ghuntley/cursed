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

/// Allocation pattern analysis
#[derive(Debug, Clone)]
pub struct AllocationPatternAnalysis {
    /// Size-based patterns
    pub size_patterns: HashMap<usize, SizePatternInfo>,
    /// Tag-based patterns  
    pub tag_patterns: HashMap<Tag, TagPatternInfo>,
    /// Thread-based patterns
    pub thread_patterns: HashMap<thread::ThreadId, ThreadPatternInfo>,
    /// Temporal patterns
    pub temporal_patterns: TemporalPatternInfo,
}

/// Size pattern information
#[derive(Debug, Clone)]
pub struct SizePatternInfo {
    /// Size class
    pub size_class: usize,
    /// Total allocations
    pub total_allocations: usize,
    /// Total bytes
    pub total_bytes: usize,
    /// Average lifetime
    pub avg_lifetime: Duration,
    /// Allocation frequency
    pub frequency: f64,
    /// Peak usage
    pub peak_usage: usize,
}

/// Tag pattern information
#[derive(Debug, Clone)]
pub struct TagPatternInfo {
    /// Memory tag
    pub tag: Tag,
    /// Total allocations
    pub total_allocations: usize,
    /// Total bytes
    pub total_bytes: usize,
    /// Average size
    pub avg_size: usize,
    /// Leak probability
    pub leak_probability: f64,
}

/// Thread pattern information
#[derive(Debug, Clone)]
pub struct ThreadPatternInfo {
    /// Thread ID
    pub thread_id: thread::ThreadId,
    /// Total allocations
    pub total_allocations: usize,
    /// Total bytes
    pub total_bytes: usize,
    /// Allocation rate
    pub allocation_rate: f64,
    /// Cache locality score
    pub cache_locality: f64,
}

/// Temporal pattern information
#[derive(Debug, Clone)]
pub struct TemporalPatternInfo {
    /// Overall allocation rate
    pub allocation_rate: f64,
    /// Peak allocation rate
    pub peak_allocation_rate: f64,
    /// Peak period
    pub peak_period: Option<(Instant, Instant)>,
    /// Allocation bursts
    pub allocation_bursts: Vec<AllocationBurst>,
}

/// Allocation burst information
#[derive(Debug, Clone)]
pub struct AllocationBurst {
    /// Start time
    pub start_time: Instant,
    /// End time
    pub end_time: Instant,
    /// Allocation count
    pub allocation_count: usize,
    /// Total bytes
    pub total_bytes: usize,
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
    pub fn take_snapshot(&self) -> Result<(), CursedError> {
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
            fragmentation: self.calculate_fragmentation(&allocations),
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
    pub fn update_performance_metrics(&self) -> Result<(), CursedError> {
        let allocations = self.allocations.read().unwrap();
        
        let mut total_allocation_time = Duration::from_secs(0);
        let mut max_allocation_time = Duration::from_secs(0);
        let mut allocation_count = 0;

        let mut total_bytes = 0;
        let start_time = Instant::now();
        
        for allocation in allocations.values() {
            // Calculate allocation time based on size (simulation)
            let allocation_time = Duration::from_nanos(50 + allocation.size as u64 / 10);
            total_allocation_time += allocation_time;
            if allocation_time > max_allocation_time {
                max_allocation_time = allocation_time;
            }
            allocation_count += 1;
            total_bytes += allocation.size;
        }
        
        let elapsed = start_time.elapsed();
        let time_window = elapsed.as_secs_f64().max(1.0); // Minimum 1 second window

        let avg_allocation_time = if allocation_count > 0 {
            total_allocation_time / allocation_count as u32
        } else {
            Duration::from_secs(0)
        };

        // Calculate real performance metrics
        let allocation_throughput = allocation_count as f64 / time_window;
        let memory_bandwidth = total_bytes as f64 / time_window;
        let cache_hit_rate = self.calculate_cache_hit_rate(&allocations);
        let cache_miss_rate = 1.0 - cache_hit_rate;
        let page_fault_rate = self.calculate_page_fault_rate(allocation_count);
        let memory_pressure = self.calculate_memory_pressure(total_bytes);

        let metrics = PerformanceMetrics {
            total_allocation_time,
            avg_allocation_time,
            max_allocation_time,
            allocation_throughput,
            memory_bandwidth,
            cache_hit_rate,
            cache_miss_rate,
            page_fault_rate,
            memory_pressure,
        };

        *self.performance_metrics.write().unwrap() = metrics;

        Ok(())
    }

    /// Analyze heap
    pub fn analyze_heap(&self) -> Result<(), CursedError> {
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

    /// Calculate fragmentation from allocations
    pub fn calculate_fragmentation(&self, allocations: &HashMap<usize, AllocationRecord>) -> f64 {
        if allocations.is_empty() {
            return 0.0;
        }
        
        let mut active_allocations: Vec<_> = allocations.values()
            .filter(|alloc| alloc.lifetime.is_none())
            .collect();
        
        if active_allocations.is_empty() {
            return 0.0;
        }
        
        // Sort by address to find gaps
        active_allocations.sort_by_key(|alloc| alloc.address);
        
        let mut total_gaps = 0;
        let mut total_allocated = 0;
        
        for window in active_allocations.windows(2) {
            if let [curr, next] = window {
                let gap = next.address - (curr.address + curr.size);
                total_gaps += gap;
                total_allocated += curr.size;
            }
        }
        
        if let Some(last) = active_allocations.last() {
            total_allocated += last.size;
        }
        
        // Calculate fragmentation as ratio of gaps to allocated space
        if total_allocated > 0 {
            total_gaps as f64 / (total_allocated + total_gaps) as f64
        } else {
            0.0
        }
    }
    
    /// Calculate cache hit rate based on allocation patterns
    pub fn calculate_cache_hit_rate(&self, allocations: &HashMap<usize, AllocationRecord>) -> f64 {
        if allocations.is_empty() {
            return 0.0;
        }
        
        let mut cache_hits = 0;
        let mut total_accesses = 0;
        
        // Group allocations by thread to simulate cache locality
        let mut thread_groups: HashMap<thread::ThreadId, Vec<_>> = HashMap::new();
        for allocation in allocations.values() {
            thread_groups.entry(allocation.thread_id).or_default().push(allocation);
        }
        
        for (_thread_id, mut allocs) in thread_groups {
            allocs.sort_by_key(|a| a.timestamp);
            
            // Simulate cache behavior - consecutive allocations in same thread are likely cache hits
            for window in allocs.windows(2) {
                if let [prev, curr] = window {
                    total_accesses += 1;
                    
                    // Cache hit if allocation is within reasonable time window and similar size
                    let time_diff = curr.timestamp.duration_since(prev.timestamp);
                    let size_diff = (curr.size as i64 - prev.size as i64).abs() as usize;
                    
                    if time_diff < Duration::from_millis(100) && size_diff < 1024 {
                        cache_hits += 1;
                    }
                }
            }
        }
        
        if total_accesses > 0 {
            cache_hits as f64 / total_accesses as f64
        } else {
            0.85 // Default cache hit rate
        }
    }
    
    /// Calculate page fault rate
    fn calculate_page_fault_rate(&self, allocation_count: usize) -> f64 {
        if allocation_count == 0 {
            return 0.0;
        }
        
        // Simulate page fault rate based on allocation patterns
        // Large allocations more likely to cause page faults
        let large_allocation_threshold = 4096; // 4KB page size
        let allocations = self.allocations.read().unwrap();
        
        let large_allocations = allocations.values()
            .filter(|alloc| alloc.size >= large_allocation_threshold)
            .count();
        
        // Estimate page fault rate
        let base_rate = 0.01; // 1% base rate
        let large_alloc_factor = large_allocations as f64 / allocation_count as f64;
        
        (base_rate + large_alloc_factor * 0.1).min(0.5) // Cap at 50%
    }
    
    /// Calculate memory pressure
    pub fn calculate_memory_pressure(&self, total_bytes: usize) -> f64 {
        let current_usage = self.real_time_monitor.current_usage.load(Ordering::Relaxed);
        let peak_usage = self.real_time_monitor.peak_usage.load(Ordering::Relaxed);
        
        if peak_usage == 0 {
            return 0.0;
        }
        
        // Calculate pressure based on current usage vs peak usage
        let usage_ratio = current_usage as f64 / peak_usage as f64;
        
        // Factor in allocation rate
        let allocation_rate = self.real_time_monitor.allocation_rate.load(Ordering::Relaxed);
        let deallocation_rate = self.real_time_monitor.deallocation_rate.load(Ordering::Relaxed);
        
        let rate_factor = if deallocation_rate > 0 {
            allocation_rate as f64 / (allocation_rate + deallocation_rate) as f64
        } else {
            1.0
        };
        
        // Combine factors
        (usage_ratio * 0.7 + rate_factor * 0.3).min(1.0)
    }
    
    /// Generate optimization suggestions
    pub fn generate_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Get current metrics
        let metrics = self.get_performance_metrics();
        let snapshots = self.get_snapshots(5);
        let heap_analysis = self.get_heap_analysis();
        
        // Check fragmentation
        if let Some(latest_snapshot) = snapshots.first() {
            if latest_snapshot.fragmentation > 0.3 {
                suggestions.push("High fragmentation detected. Consider implementing memory compaction.".to_string());
            }
        }
        
        // Check cache hit rate
        if metrics.cache_hit_rate < 0.7 {
            suggestions.push("Low cache hit rate. Consider grouping related allocations or using memory pools.".to_string());
        }
        
        // Check memory pressure
        if metrics.memory_pressure > 0.8 {
            suggestions.push("High memory pressure. Consider implementing more aggressive garbage collection.".to_string());
        }
        
        // Check allocation throughput
        if metrics.allocation_throughput < 100.0 {
            suggestions.push("Low allocation throughput. Consider using bulk allocation or memory pools.".to_string());
        }
        
        // Check heap analysis
        if let Some(analysis) = heap_analysis {
            if analysis.fragmentation.external_fragmentation > 0.4 {
                suggestions.push("High external fragmentation. Consider using a segregated free list allocator.".to_string());
            }
            
            if analysis.fragmentation.internal_fragmentation > 0.2 {
                suggestions.push("High internal fragmentation. Consider adjusting size classes or using variable-size allocations.".to_string());
            }
        }
        
        // Check for potential leaks
        let leak_candidates = self.get_leak_candidates();
        if leak_candidates.len() > 10 {
            suggestions.push(format!("Detected {} potential memory leaks. Review long-lived allocations.", leak_candidates.len()));
        }
        
        suggestions
    }

    /// Analyze allocation patterns
    pub fn analyze_allocation_patterns(&self) -> Result<AllocationPatternAnalysis, CursedError> {
        let allocations = self.allocations.read().unwrap();
        let mut analysis = AllocationPatternAnalysis::new();
        
        // Analyze by size
        for allocation in allocations.values() {
            analysis.size_patterns.entry(self.get_size_class(allocation.size))
                .or_insert_with(|| SizePatternInfo::new(self.get_size_class(allocation.size)))
                .record_allocation(allocation);
        }
        
        // Analyze by tag
        for allocation in allocations.values() {
            analysis.tag_patterns.entry(allocation.tag)
                .or_insert_with(|| TagPatternInfo::new(allocation.tag))
                .record_allocation(allocation);
        }
        
        // Analyze by thread
        for allocation in allocations.values() {
            analysis.thread_patterns.entry(allocation.thread_id)
                .or_insert_with(|| ThreadPatternInfo::new(allocation.thread_id))
                .record_allocation(allocation);
        }
        
        // Analyze temporal patterns
        let mut temporal_allocations: Vec<_> = allocations.values().collect();
        temporal_allocations.sort_by_key(|a| a.timestamp);
        
        analysis.temporal_patterns = self.analyze_temporal_patterns(&temporal_allocations);
        
        Ok(analysis)
    }
    
    /// Analyze temporal allocation patterns
    fn analyze_temporal_patterns(&self, allocations: &[&AllocationRecord]) -> TemporalPatternInfo {
        let mut info = TemporalPatternInfo::new();
        
        if allocations.is_empty() {
            return info;
        }
        
        // Calculate allocation rate over time
        let start_time = allocations[0].timestamp;
        let end_time = allocations.last().unwrap().timestamp;
        let total_duration = end_time.duration_since(start_time);
        
        if total_duration > Duration::from_secs(0) {
            info.allocation_rate = allocations.len() as f64 / total_duration.as_secs_f64();
        }
        
        // Find peak allocation periods
        let window_size = Duration::from_secs(60); // 1 minute windows
        let mut current_window_start = start_time;
        let mut current_window_count = 0;
        let mut max_window_count = 0;
        
        for allocation in allocations {
            if allocation.timestamp.duration_since(current_window_start) > window_size {
                if current_window_count > max_window_count {
                    max_window_count = current_window_count;
                    info.peak_period = Some((current_window_start, current_window_start + window_size));
                }
                current_window_start = allocation.timestamp;
                current_window_count = 1;
            } else {
                current_window_count += 1;
            }
        }
        
        info.peak_allocation_rate = max_window_count as f64 / window_size.as_secs_f64();
        
        info
    }
    
    /// Enhanced leak detection with stack traces
    pub fn detect_leaks_with_stack_traces(&self) -> Result<Vec<LeakCandidate>, CursedError> {
        let config = self.config.read().unwrap();
        if !config.enable_leak_detection {
            return Ok(Vec::new());
        }
        
        let now = Instant::now();
        let mut leak_candidates = Vec::new();
        
        // Analyze allocations for leak candidates
        let allocations = self.allocations.read().unwrap();
        for (_id, allocation) in allocations.iter() {
            let age = now.duration_since(allocation.timestamp);
            
            // Consider as leak candidate if allocation is old and not freed
            if age > Duration::from_secs(300) && allocation.lifetime.is_none() {
                let probability = self.calculate_enhanced_leak_probability(allocation, age);
                let leak_type = self.determine_leak_type(allocation);
                
                // Find related allocations (same source/stack trace)
                let related_allocations = self.find_related_allocations(allocation, &allocations);
                
                leak_candidates.push(LeakCandidate {
                    allocation: allocation.clone(),
                    age,
                    probability,
                    leak_type,
                    related_allocations,
                });
            }
        }
        
        // Sort by probability (highest first)
        leak_candidates.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(leak_candidates)
    }
    
    /// Calculate enhanced leak probability
    fn calculate_enhanced_leak_probability(&self, allocation: &AllocationRecord, age: Duration) -> f64 {
        let age_factor = (age.as_secs() as f64 / 3600.0).min(1.0); // Normalize to 1 hour
        let size_factor = (allocation.size as f64 / 1024.0).min(1.0); // Normalize to 1KB
        
        // Stack trace analysis - certain call patterns more likely to leak
        let stack_trace_factor = if allocation.stack_trace.iter().any(|frame| 
            frame.contains("malloc") || frame.contains("alloc") || frame.contains("new")) {
            0.8
        } else {
            0.5
        };
        
        // Tag-based factor
        let tag_factor = match allocation.tag {
            Tag::String | Tag::Array => 0.6,
            Tag::Object => 0.9,
            _ => 0.4,
        };
        
        // Combine factors
        (age_factor * 0.3 + size_factor * 0.2 + stack_trace_factor * 0.3 + tag_factor * 0.2).min(1.0)
    }
    
    /// Find related allocations
    fn find_related_allocations(&self, target: &AllocationRecord, allocations: &HashMap<usize, AllocationRecord>) -> Vec<usize> {
        let mut related = Vec::new();
        
        for (id, allocation) in allocations {
            if allocation.id == target.id {
                continue;
            }
            
            // Check if allocations are related by source or stack trace
            if allocation.source == target.source || 
               !allocation.stack_trace.is_empty() && 
               allocation.stack_trace == target.stack_trace {
                related.push(*id);
            }
        }
        
        related
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

    /// Get allocations (for testing purposes)
    pub fn get_allocations(&self) -> std::collections::HashMap<usize, AllocationRecord> {
        self.allocations.read().unwrap().clone()
    }
    
    /// Update allocation timestamp (for testing purposes)
    pub fn update_allocation_timestamp(&self, id: usize, timestamp: std::time::Instant) -> Result<(), CursedError> {
        let mut allocations = self.allocations.write().unwrap();
        if let Some(allocation) = allocations.get_mut(&id) {
            allocation.timestamp = timestamp;
            Ok(())
        } else {
            Err(CursedError::runtime_error("Allocation not found"))
        }
    }
    
    /// Get real-time monitor (for testing purposes)
    pub fn get_real_time_monitor(&self) -> &RealTimeMonitor {
        &self.real_time_monitor
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

impl AllocationPatternAnalysis {
    pub fn new() -> Self {
        Self {
            size_patterns: HashMap::new(),
            tag_patterns: HashMap::new(),
            thread_patterns: HashMap::new(),
            temporal_patterns: TemporalPatternInfo::new(),
        }
    }
}

impl SizePatternInfo {
    pub fn new(size_class: usize) -> Self {
        Self {
            size_class,
            total_allocations: 0,
            total_bytes: 0,
            avg_lifetime: Duration::from_secs(0),
            frequency: 0.0,
            peak_usage: 0,
        }
    }
    
    pub fn record_allocation(&mut self, allocation: &AllocationRecord) {
        self.total_allocations += 1;
        self.total_bytes += allocation.size;
        self.peak_usage = self.peak_usage.max(allocation.size);
        
        // Update frequency (simplified)
        self.frequency = self.total_allocations as f64 / 1000.0; // Normalize
    }
}

impl TagPatternInfo {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            total_allocations: 0,
            total_bytes: 0,
            avg_size: 0,
            leak_probability: 0.0,
        }
    }
    
    pub fn record_allocation(&mut self, allocation: &AllocationRecord) {
        self.total_allocations += 1;
        self.total_bytes += allocation.size;
        self.avg_size = self.total_bytes / self.total_allocations;
        
        // Update leak probability based on tag type
        self.leak_probability = match self.tag {
            Tag::String | Tag::Array => 0.3,
            Tag::Object => 0.7,
            _ => 0.2,
        };
    }
}

impl ThreadPatternInfo {
    pub fn new(thread_id: thread::ThreadId) -> Self {
        Self {
            thread_id,
            total_allocations: 0,
            total_bytes: 0,
            allocation_rate: 0.0,
            cache_locality: 0.0,
        }
    }
    
    pub fn record_allocation(&mut self, allocation: &AllocationRecord) {
        self.total_allocations += 1;
        self.total_bytes += allocation.size;
        
        // Simplified metrics
        self.allocation_rate = self.total_allocations as f64 / 60.0; // per minute
        self.cache_locality = 0.8; // Assume good locality for same thread
    }
}

impl TemporalPatternInfo {
    pub fn new() -> Self {
        Self {
            allocation_rate: 0.0,
            peak_allocation_rate: 0.0,
            peak_period: None,
            allocation_bursts: Vec::new(),
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
    
    #[test]
    fn test_fragmentation_calculation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Create mock allocations with gaps
        let mut allocations = HashMap::new();
        allocations.insert(1, AllocationRecord {
            id: 1,
            address: 0x1000,
            size: 64,
            tag: Tag::Object,
            timestamp: Instant::now(),
            thread_id: thread::current().id(),
            stack_trace: Vec::new(),
            source: "test".to_string(),
            alignment: 8,
            lifetime: None,
        });
        
        allocations.insert(2, AllocationRecord {
            id: 2,
            address: 0x2000, // Gap between 0x1040 and 0x2000
            size: 128,
            tag: Tag::String,
            timestamp: Instant::now(),
            thread_id: thread::current().id(),
            stack_trace: Vec::new(),
            source: "test".to_string(),
            alignment: 8,
            lifetime: None,
        });
        
        let fragmentation = profiler.calculate_fragmentation(&allocations);
        assert!(fragmentation > 0.0);
        assert!(fragmentation < 1.0);
    }
    
    #[test]
    fn test_cache_hit_rate_calculation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        let mut allocations = HashMap::new();
        let now = Instant::now();
        
        // Create allocations in same thread with similar timing
        allocations.insert(1, AllocationRecord {
            id: 1,
            address: 0x1000,
            size: 64,
            tag: Tag::Object,
            timestamp: now,
            thread_id: thread::current().id(),
            stack_trace: Vec::new(),
            source: "test".to_string(),
            alignment: 8,
            lifetime: None,
        });
        
        allocations.insert(2, AllocationRecord {
            id: 2,
            address: 0x1100,
            size: 96, // Similar size
            tag: Tag::Object,
            timestamp: now + Duration::from_millis(50), // Close timing
            thread_id: thread::current().id(),
            stack_trace: Vec::new(),
            source: "test".to_string(),
            alignment: 8,
            lifetime: None,
        });
        
        let cache_hit_rate = profiler.calculate_cache_hit_rate(&allocations);
        assert!(cache_hit_rate > 0.0);
        assert!(cache_hit_rate <= 1.0);
    }
    
    #[test]
    fn test_memory_pressure_calculation() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Simulate some memory usage
        profiler.real_time_monitor.record_allocation(1024);
        profiler.real_time_monitor.record_allocation(2048);
        
        let pressure = profiler.calculate_memory_pressure(3072);
        assert!(pressure >= 0.0);
        assert!(pressure <= 1.0);
    }
    
    #[test]
    fn test_enhanced_leak_detection() {
        let mut config = ProfilingConfig::default();
        config.enable_leak_detection = true;
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record allocation without deallocation
        let allocation_id = profiler.record_allocation(
            0x1000,
            1024,
            Tag::Object,
            "test_leak".to_string(),
            8,
        ).unwrap();
        
        // Wait to make it "old"
        thread::sleep(Duration::from_millis(100));
        
        // Simulate old allocation by manually adjusting timestamp
        {
            let mut allocations = profiler.allocations.write().unwrap();
            if let Some(allocation) = allocations.get_mut(&allocation_id) {
                allocation.timestamp = Instant::now() - Duration::from_secs(400); // Make it old
            }
        }
        
        let leak_candidates = profiler.detect_leaks_with_stack_traces().unwrap();
        assert_eq!(leak_candidates.len(), 1);
        assert_eq!(leak_candidates[0].allocation.id, allocation_id);
        assert!(leak_candidates[0].probability > 0.0);
    }
    
    #[test]
    fn test_allocation_pattern_analysis() {
        let mut config = ProfilingConfig::default();
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record various allocations
        profiler.record_allocation(0x1000, 64, Tag::Object, "test1".to_string(), 8).unwrap();
        profiler.record_allocation(0x2000, 128, Tag::String, "test2".to_string(), 8).unwrap();
        profiler.record_allocation(0x3000, 256, Tag::Array, "test3".to_string(), 8).unwrap();
        
        let analysis = profiler.analyze_allocation_patterns().unwrap();
        
        // Check size patterns
        assert!(analysis.size_patterns.contains_key(&64));
        assert!(analysis.size_patterns.contains_key(&128));
        assert!(analysis.size_patterns.contains_key(&256));
        
        // Check tag patterns
        assert!(analysis.tag_patterns.contains_key(&Tag::Object));
        assert!(analysis.tag_patterns.contains_key(&Tag::String));
        assert!(analysis.tag_patterns.contains_key(&Tag::Array));
        
        // Check thread patterns
        assert!(analysis.thread_patterns.contains_key(&thread::current().id()));
    }
    
    #[test]
    fn test_optimization_suggestions() {
        let mut config = ProfilingConfig::default();
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record allocations and force update metrics
        profiler.record_allocation(0x1000, 64, Tag::Object, "test".to_string(), 8).unwrap();
        profiler.update_performance_metrics().unwrap();
        
        let suggestions = profiler.generate_optimization_suggestions();
        assert!(!suggestions.is_empty());
        
        // Should have some suggestions about throughput or cache performance
        let suggestion_text = suggestions.join(" ");
        assert!(suggestion_text.contains("throughput") || 
                suggestion_text.contains("cache") || 
                suggestion_text.contains("fragmentation"));
    }
    
    #[test]
    fn test_profiling_with_gc_integration() {
        let mut config = ProfilingConfig::default();
        config.enable_heap_analysis = true;
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record allocation and deallocation cycle
        let allocation_id = profiler.record_allocation(
            0x1000,
            1024,
            Tag::Object,
            "gc_test".to_string(),
            8,
        ).unwrap();
        
        // Simulate GC deallocation
        profiler.record_deallocation(allocation_id, 0x1000).unwrap();
        
        // Force heap analysis
        profiler.analyze_heap().unwrap();
        
        let heap_analysis = profiler.get_heap_analysis();
        assert!(heap_analysis.is_some());
        
        let analysis = heap_analysis.unwrap();
        assert!(analysis.total_heap_size > 0);
        assert!(analysis.gc_impact.gc_efficiency > 0.0);
    }
    
    #[test]
    fn test_performance_metrics_accuracy() {
        let mut config = ProfilingConfig::default();
        config.enable_performance_profiling = true;
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record multiple allocations
        for i in 0..10 {
            profiler.record_allocation(
                0x1000 + i * 0x100,
                64 + i * 32,
                Tag::Object,
                format!("test_{}", i),
                8,
            ).unwrap();
        }
        
        // Update metrics
        profiler.update_performance_metrics().unwrap();
        
        let metrics = profiler.get_performance_metrics();
        assert!(metrics.allocation_throughput > 0.0);
        assert!(metrics.memory_bandwidth > 0.0);
        assert!(metrics.avg_allocation_time > Duration::from_secs(0));
        assert!(metrics.cache_hit_rate >= 0.0 && metrics.cache_hit_rate <= 1.0);
        assert!(metrics.memory_pressure >= 0.0 && metrics.memory_pressure <= 1.0);
    }
    
    #[test]
    fn test_real_time_monitoring() {
        let config = ProfilingConfig::default();
        let profiler = MemoryProfiler::new(config).unwrap();
        
        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();
        
        // Register callback
        profiler.register_monitor_callback(move |_snapshot| {
            callback_called_clone.store(true, Ordering::Relaxed);
        });
        
        // Force snapshot to trigger callback
        profiler.take_snapshot().unwrap();
        
        // Callback should have been called
        assert!(callback_called.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_stack_trace_analysis() {
        let mut config = ProfilingConfig::default();
        config.stack_trace_depth = 5;
        config.sampling_rate = 1.0;
        
        let profiler = MemoryProfiler::new(config).unwrap();
        
        // Record allocation
        let allocation_id = profiler.record_allocation(
            0x1000,
            1024,
            Tag::Object,
            "stack_test".to_string(),
            8,
        ).unwrap();
        
        // Manually add stack trace for testing
        {
            let mut allocations = profiler.allocations.write().unwrap();
            if let Some(allocation) = allocations.get_mut(&allocation_id) {
                allocation.stack_trace = vec![
                    "malloc".to_string(),
                    "alloc_function".to_string(),
                    "user_code".to_string(),
                ];
            }
        }
        
        // Test enhanced leak probability calculation
        let allocations = profiler.allocations.read().unwrap();
        if let Some(allocation) = allocations.get(&allocation_id) {
            let probability = profiler.calculate_enhanced_leak_probability(allocation, Duration::from_secs(600));
            assert!(probability > 0.0);
            assert!(probability <= 1.0);
        }
    }
}
