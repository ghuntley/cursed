/// Garbage Collection Monitor
/// 
/// Provides real-time monitoring, alerting, and diagnostics for the garbage collector
/// to ensure optimal performance in production environments.

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use std::thread::{self, JoinHandle};
use std::fs::OpenOptions;
use std::io::Write;

use crate::runtime::gc::{GarbageCollector, GcStats, GcState};
use crate::runtime::concurrent_gc::{ConcurrentGarbageCollector, ConcurrentStats};
use crate::runtime::memory::{MemoryManager, MemoryStats};
use crate::runtime::memory_profiler::{MemoryProfiler, ProfilingStats};
use crate::runtime::heap_optimizer::{HeapOptimizer, HeapStats};
use crate::error::CursedError;

/// GC monitoring configuration
#[derive(Debug, Clone)]
pub struct GcMonitorConfig {
    /// Enable real-time monitoring
    pub real_time_monitoring: bool,
    /// Monitoring interval in milliseconds
    pub monitoring_interval_ms: u64,
    /// Enable alerting
    pub alerting: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
    /// Enable logging
    pub logging: bool,
    /// Log file path
    pub log_file: Option<String>,
    /// Maximum log size in bytes
    pub max_log_size: usize,
    /// Enable metrics collection
    pub metrics_collection: bool,
    /// Metrics retention period
    pub metrics_retention: Duration,
    /// Enable performance analysis
    pub performance_analysis: bool,
    /// Enable trend analysis
    pub trend_analysis: bool,
    /// Enable automatic tuning recommendations
    pub auto_tuning: bool,
}

impl Default for GcMonitorConfig {
    fn default() -> Self {
        Self {
            real_time_monitoring: true,
            monitoring_interval_ms: 1000, // 1 second
            alerting: true,
            alert_thresholds: AlertThresholds::default(),
            logging: true,
            log_file: Some("gc_monitor.log".to_string()),
            max_log_size: 100 * 1024 * 1024, // 100MB
            metrics_collection: true,
            metrics_retention: Duration::from_secs(24 * 60 * 60), // 24 hours
            performance_analysis: true,
            trend_analysis: true,
            auto_tuning: true,
        }
    }
}

/// Alert thresholds for GC monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Maximum pause time in milliseconds
    pub max_pause_time_ms: u64,
    /// Maximum GC overhead percentage
    pub max_gc_overhead: f64,
    /// Maximum heap utilization percentage
    pub max_heap_utilization: f64,
    /// Maximum allocation rate in bytes per second
    pub max_allocation_rate: f64,
    /// Maximum memory leak detection threshold
    pub max_leak_threshold: usize,
    /// Maximum fragmentation percentage
    pub max_fragmentation: f64,
    /// Maximum collection frequency per minute
    pub max_collection_frequency: f64,
    /// Minimum concurrent efficiency percentage
    pub min_concurrent_efficiency: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_pause_time_ms: 50, // 50ms
            max_gc_overhead: 0.1, // 10%
            max_heap_utilization: 0.9, // 90%
            max_allocation_rate: 1_000_000_000.0, // 1GB/s
            max_leak_threshold: 1000, // 1000 leaked objects
            max_fragmentation: 0.3, // 30%
            max_collection_frequency: 60.0, // 60 collections per minute
            min_concurrent_efficiency: 0.8, // 80%
        }
    }
}

/// GC monitoring event
#[derive(Debug, Clone)]
pub struct GcEvent {
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event type
    pub event_type: GcEventType,
    /// Event severity
    pub severity: EventSeverity,
    /// Event message
    pub message: String,
    /// Associated metrics
    pub metrics: HashMap<String, f64>,
}

/// Types of GC events
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GcEventType {
    /// Collection started
    CollectionStarted,
    /// Collection completed
    CollectionCompleted,
    /// Pause time threshold exceeded
    PauseTimeAlert,
    /// GC overhead threshold exceeded
    GcOverheadAlert,
    /// Heap utilization threshold exceeded
    HeapUtilizationAlert,
    /// Memory leak detected
    MemoryLeakAlert,
    /// Fragmentation threshold exceeded
    FragmentationAlert,
    /// Collection frequency threshold exceeded
    CollectionFrequencyAlert,
    /// Concurrent efficiency below threshold
    ConcurrentEfficiencyAlert,
    /// Performance degradation detected
    PerformanceDegradation,
    /// Tuning recommendation
    TuningRecommendation,
    /// System information
    SystemInfo,
}

/// Event severity levels
#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum EventSeverity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// GC metrics snapshot
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GcMetricsSnapshot {
    /// Timestamp
    pub timestamp: SystemTime,
    /// GC statistics
    pub gc_stats: GcStats,
    /// Concurrent GC statistics
    pub concurrent_stats: Option<ConcurrentStats>,
    /// Memory statistics
    pub memory_stats: Option<MemoryStats>,
    /// Profiling statistics
    #[serde(skip)]
    pub profiling_stats: Option<ProfilingStats>,
    /// Heap statistics
    pub heap_stats: Option<HeapStats>,
    /// System metrics
    pub system_metrics: SystemMetrics,
}

/// System metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Available memory in bytes
    pub available_memory: usize,
    /// Total memory in bytes
    pub total_memory: usize,
    /// System load average
    pub load_average: f64,
    /// Number of threads
    pub thread_count: usize,
}

/// Performance trend data
#[derive(Debug, Clone)]
pub struct PerformanceTrend {
    /// Metric name
    pub metric_name: String,
    /// Data points
    pub data_points: VecDeque<(SystemTime, f64)>,
    /// Trend direction
    pub trend_direction: TrendDirection,
    /// Trend strength
    pub trend_strength: f64,
    /// Predicted next value
    pub predicted_value: f64,
    /// Confidence interval
    pub confidence_interval: (f64, f64),
}

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrendDirection {
    /// Increasing trend
    Increasing,
    /// Decreasing trend
    Decreasing,
    /// Stable trend
    Stable,
    /// Volatile trend
    Volatile,
}

/// Tuning recommendation
#[derive(Debug, Clone)]
pub struct TuningRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Description
    pub description: String,
    /// Expected impact
    pub expected_impact: String,
    /// Configuration changes
    pub config_changes: HashMap<String, String>,
    /// Confidence score
    pub confidence: f64,
}

/// Types of tuning recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationType {
    /// Increase heap size
    IncreaseHeapSize,
    /// Decrease heap size
    DecreaseHeapSize,
    /// Adjust collection frequency
    AdjustCollectionFrequency,
    /// Enable concurrent collection
    EnableConcurrentCollection,
    /// Adjust concurrent threads
    AdjustConcurrentThreads,
    /// Enable incremental collection
    EnableIncrementalCollection,
    /// Adjust incremental step size
    AdjustIncrementalStepSize,
    /// Enable compaction
    EnableCompaction,
    /// Disable compaction
    DisableCompaction,
    /// Adjust generational ratios
    AdjustGenerationalRatios,
    /// Enable write barriers
    EnableWriteBarriers,
    /// Optimize allocation strategy
    OptimizeAllocationStrategy,
}

/// Recommendation priority
#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum RecommendationPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// GC monitor
pub struct GcMonitor {
    /// Configuration
    config: GcMonitorConfig,
    /// Garbage collector reference
    gc_ref: Option<Arc<GarbageCollector>>,
    /// Concurrent GC reference
    concurrent_gc_ref: Option<Arc<ConcurrentGarbageCollector>>,
    /// Memory manager reference
    memory_manager_ref: Option<Arc<dyn MemoryManager>>,
    /// Memory profiler reference
    profiler_ref: Option<Arc<MemoryProfiler>>,
    /// Heap optimizer reference
    heap_optimizer_ref: Option<Arc<HeapOptimizer>>,
    /// Running flag
    running: AtomicBool,
    /// Monitoring thread
    monitor_thread: Mutex<Option<JoinHandle<()>>>,
    /// Event queue
    event_queue: Arc<Mutex<VecDeque<GcEvent>>>,
    /// Metrics history
    metrics_history: Arc<RwLock<VecDeque<GcMetricsSnapshot>>>,
    /// Performance trends
    performance_trends: Arc<RwLock<HashMap<String, PerformanceTrend>>>,
    /// Tuning recommendations
    tuning_recommendations: Arc<RwLock<Vec<TuningRecommendation>>>,
    /// Alert callbacks
    alert_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&GcEvent) + Send + Sync>>>>,
    /// Log file handle
    log_file: Arc<Mutex<Option<std::fs::File>>>,
    /// Event counters
    event_counters: Arc<RwLock<HashMap<GcEventType, AtomicU64>>>,
}

impl GcMonitor {
    /// Create new GC monitor
    pub fn new(config: GcMonitorConfig) -> Result<Self, CursedError> {
        let log_file = if let Some(log_path) = &config.log_file {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to open log file: {}", e)))?;
            Some(file)
        } else {
            None
        };

        Ok(Self {
            config,
            gc_ref: None,
            concurrent_gc_ref: None,
            memory_manager_ref: None,
            profiler_ref: None,
            heap_optimizer_ref: None,
            running: AtomicBool::new(false),
            monitor_thread: Mutex::new(None),
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            metrics_history: Arc::new(RwLock::new(VecDeque::new())),
            performance_trends: Arc::new(RwLock::new(HashMap::new())),
            tuning_recommendations: Arc::new(RwLock::new(Vec::new())),
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
            log_file: Arc::new(Mutex::new(log_file)),
            event_counters: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start monitoring
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("GC monitor already running"));
        }

        // Start monitoring thread
        let event_queue = Arc::clone(&self.event_queue);
        let config = self.config.clone();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);
        
        let handle = thread::Builder::new()
            .name("gc-monitor".to_string())
            .spawn(move || {
                // Simple monitoring loop that doesn't require raw pointers
                while running_clone.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(config.monitoring_interval_ms));
                    
                    // Emit periodic event
                    let mut queue = event_queue.lock().unwrap();
                    queue.push_back(GcEvent {
                        timestamp: SystemTime::now(),
                        event_type: GcEventType::SystemInfo,
                        severity: EventSeverity::Info,
                        message: "GC monitoring tick".to_string(),
                        metrics: HashMap::new(),
                    });
                    
                    // Limit queue size
                    if queue.len() > 1000 {
                        queue.pop_front();
                    }
                }
            })
            .map_err(|e| CursedError::runtime_error(&format!("Failed to start monitoring thread: {}", e)))?;

        *self.monitor_thread.lock().unwrap() = Some(handle);

        self.emit_event(GcEvent {
            timestamp: SystemTime::now(),
            event_type: GcEventType::SystemInfo,
            severity: EventSeverity::Info,
            message: "GC monitoring started".to_string(),
            metrics: HashMap::new(),
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&self) -> Result<(), CursedError> {
        self.running.store(false, Ordering::Relaxed);

        // Wait for monitoring thread to finish
        if let Some(handle) = self.monitor_thread.lock().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join monitoring thread"))?;
        }

        self.emit_event(GcEvent {
            timestamp: SystemTime::now(),
            event_type: GcEventType::SystemInfo,
            severity: EventSeverity::Info,
            message: "GC monitoring stopped".to_string(),
            metrics: HashMap::new(),
        });

        Ok(())
    }

    /// Set garbage collector reference
    pub fn set_gc_ref(&mut self, gc: Arc<GarbageCollector>) {
        self.gc_ref = Some(gc);
    }

    /// Set concurrent GC reference
    pub fn set_concurrent_gc_ref(&mut self, concurrent_gc: Arc<ConcurrentGarbageCollector>) {
        self.concurrent_gc_ref = Some(concurrent_gc);
    }

    /// Set memory manager reference
    pub fn set_memory_manager_ref(&mut self, memory_manager: Arc<dyn MemoryManager>) {
        self.memory_manager_ref = Some(memory_manager);
    }

    /// Set profiler reference
    pub fn set_profiler_ref(&mut self, profiler: Arc<MemoryProfiler>) {
        self.profiler_ref = Some(profiler);
    }

    /// Set heap optimizer reference
    pub fn set_heap_optimizer_ref(&mut self, heap_optimizer: Arc<HeapOptimizer>) {
        self.heap_optimizer_ref = Some(heap_optimizer);
    }

    /// Register alert callback
    pub fn register_alert_callback<F>(&self, callback: F)
    where
        F: Fn(&GcEvent) + Send + Sync + 'static,
    {
        let mut callbacks = self.alert_callbacks.write().unwrap();
        callbacks.push(Box::new(callback));
    }

    /// Get current metrics snapshot
    pub fn get_metrics_snapshot(&self) -> GcMetricsSnapshot {
        let gc_stats = self.gc_ref.as_ref()
            .and_then(|gc| gc.get_stats().ok())
            .unwrap_or_default();
        let concurrent_stats = self.concurrent_gc_ref.as_ref().map(|cgc| cgc.get_stats());
        let memory_stats = self.memory_manager_ref.as_ref().and_then(|mm| mm.get_stats());
        let profiling_stats = self.profiler_ref.as_ref().map(|p| p.get_stats());
        let heap_stats = self.heap_optimizer_ref.as_ref().map(|ho| ho.get_stats());

        GcMetricsSnapshot {
            timestamp: SystemTime::now(),
            gc_stats: crate::memory::gc::GcStats {
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
            },
            concurrent_stats,
            memory_stats,
            profiling_stats,
            heap_stats,
            system_metrics: self.collect_system_metrics(),
        }
    }

    /// Get performance trends
    pub fn get_performance_trends(&self) -> HashMap<String, PerformanceTrend> {
        self.performance_trends.read().unwrap().clone()
    }

    /// Get tuning recommendations
    pub fn get_tuning_recommendations(&self) -> Vec<TuningRecommendation> {
        self.tuning_recommendations.read().unwrap().clone()
    }

    /// Get event history
    pub fn get_event_history(&self) -> Vec<GcEvent> {
        self.event_queue.lock().unwrap().iter().cloned().collect()
    }

    /// Generate monitoring report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=== GC Monitoring Report ===\n\n");
        
        // Current snapshot
        let snapshot = self.get_metrics_snapshot();
        report.push_str(&format!("Report generated at: {:?}\n\n", snapshot.timestamp));
        
        // GC Statistics
        report.push_str("GC Statistics:\n");
        report.push_str(&format!("  Total Collections: {}\n", snapshot.gc_stats.total_collections));
        report.push_str(&format!("  Average Pause Time: {:.2}ms\n", snapshot.gc_stats.avg_pause_time.as_millis()));
        report.push_str(&format!("  Max Pause Time: {:.2}ms\n", snapshot.gc_stats.max_pause_time.as_millis()));
        report.push_str(&format!("  GC Overhead: {:.2}%\n", snapshot.gc_stats.gc_overhead * 100.0));
        report.push_str(&format!("  Heap Utilization: {:.2}%\n", snapshot.gc_stats.heap_utilization * 100.0));
        
        // Concurrent GC Statistics
        if let Some(concurrent_stats) = &snapshot.concurrent_stats {
            report.push_str("\nConcurrent GC Statistics:\n");
            report.push_str(&format!("  Concurrent Collections: {}\n", concurrent_stats.total_concurrent_collections));
            report.push_str(&format!("  Concurrent Marking Efficiency: {:.2}%\n", concurrent_stats.concurrent_marking_efficiency * 100.0));
            report.push_str(&format!("  Write Barrier Overhead: {:.2}%\n", concurrent_stats.write_barrier_overhead * 100.0));
        }
        
        // Memory Statistics
        if let Some(memory_stats) = &snapshot.memory_stats {
            report.push_str("\nMemory Statistics:\n");
            report.push_str(&format!("  Heap Allocations: {}\n", memory_stats.heap_allocations));
            report.push_str(&format!("  Current Heap Usage: {} bytes\n", memory_stats.heap_usage));
            report.push_str(&format!("  Peak Heap Usage: {} bytes\n", memory_stats.peak_heap_usage));
            report.push_str(&format!("  Pressure Level: {:.2}%\n", memory_stats.pressure_level * 100.0));
        }
        
        // Profiling Statistics
        if let Some(profiling_stats) = &snapshot.profiling_stats {
            report.push_str("\nProfiling Statistics:\n");
            report.push_str(&format!("  Live Allocations: {}\n", profiling_stats.live_allocations));
            report.push_str(&format!("  Detected Leaks: {}\n", profiling_stats.detected_leaks.len()));
            report.push_str(&format!("  Average Object Lifetime: {:.2}s\n", profiling_stats.avg_object_lifetime.as_secs_f64()));
            report.push_str(&format!("  Heap Fragmentation: {:.2}%\n", profiling_stats.heap_fragmentation));
        }
        
        // System Metrics
        report.push_str("\nSystem Metrics:\n");
        report.push_str(&format!("  CPU Usage: {:.2}%\n", snapshot.system_metrics.cpu_usage));
        report.push_str(&format!("  Available Memory: {} bytes\n", snapshot.system_metrics.available_memory));
        report.push_str(&format!("  Total Memory: {} bytes\n", snapshot.system_metrics.total_memory));
        report.push_str(&format!("  Load Average: {:.2}\n", snapshot.system_metrics.load_average));
        report.push_str(&format!("  Thread Count: {}\n", snapshot.system_metrics.thread_count));
        
        // Tuning Recommendations
        let recommendations = self.get_tuning_recommendations();
        if !recommendations.is_empty() {
            report.push_str("\nTuning Recommendations:\n");
            for (i, rec) in recommendations.iter().enumerate().take(5) {
                report.push_str(&format!("  {}. {:?} (Priority: {:?}, Confidence: {:.2}%)\n", 
                    i + 1, rec.recommendation_type, rec.priority, rec.confidence * 100.0));
                report.push_str(&format!("     {}\n", rec.description));
            }
        }
        
        // Recent Events
        let events = self.get_event_history();
        let recent_events: Vec<_> = events.iter().rev().take(10).collect();
        if !recent_events.is_empty() {
            report.push_str("\nRecent Events:\n");
            for event in recent_events {
                report.push_str(&format!("  {:?}: {:?} - {}\n", 
                    event.severity, event.event_type, event.message));
            }
        }
        
        report
    }

    /// Main monitoring loop
    fn monitoring_loop(&self) {
        let interval = Duration::from_millis(self.config.monitoring_interval_ms);
        let start_time = std::time::Instant::now();
        let max_runtime = Duration::from_secs(300); // Maximum 5 minutes
        
        while self.running.load(Ordering::Relaxed) {
            // Safety timeout to prevent infinite loops
            if start_time.elapsed() > max_runtime {
                println!("GC monitoring loop timed out after 5 minutes, stopping");
                break;
            }
            
            // Collect metrics
            let snapshot = self.get_metrics_snapshot();
            
            // Store metrics history
            self.store_metrics_snapshot(snapshot.clone());
            
            // Check alert thresholds
            self.check_alert_thresholds(&snapshot);
            
            // Update performance trends
            self.update_performance_trends(&snapshot);
            
            // Generate tuning recommendations
            if self.config.auto_tuning {
                self.generate_tuning_recommendations(&snapshot);
            }
            
            // Clean up old data
            self.cleanup_old_data();
            
            thread::sleep(interval);
        }
    }

    /// Store metrics snapshot in history
    fn store_metrics_snapshot(&self, snapshot: GcMetricsSnapshot) {
        let mut history = self.metrics_history.write().unwrap();
        history.push_back(snapshot);
        
        // Limit history size based on retention period
        let retention_cutoff = SystemTime::now() - self.config.metrics_retention;
        while let Some(front) = history.front() {
            if front.timestamp < retention_cutoff {
                history.pop_front();
            } else {
                break;
            }
        }
    }

    /// Check alert thresholds
    fn check_alert_thresholds(&self, snapshot: &GcMetricsSnapshot) {
        let thresholds = &self.config.alert_thresholds;
        
        // Check pause time
        if snapshot.gc_stats.max_pause_time.as_millis() > thresholds.max_pause_time_ms as u128 {
            self.emit_event(GcEvent {
                timestamp: SystemTime::now(),
                event_type: GcEventType::PauseTimeAlert,
                severity: EventSeverity::Warning,
                message: format!("GC pause time exceeded threshold: {}ms > {}ms", 
                    snapshot.gc_stats.max_pause_time.as_millis(), thresholds.max_pause_time_ms),
                metrics: HashMap::new(),
            });
        }
        
        // Check GC overhead
        if snapshot.gc_stats.gc_overhead > thresholds.max_gc_overhead {
            self.emit_event(GcEvent {
                timestamp: SystemTime::now(),
                event_type: GcEventType::GcOverheadAlert,
                severity: EventSeverity::Warning,
                message: format!("GC overhead exceeded threshold: {:.2}% > {:.2}%", 
                    snapshot.gc_stats.gc_overhead * 100.0, thresholds.max_gc_overhead * 100.0),
                metrics: HashMap::new(),
            });
        }
        
        // Check heap utilization
        if snapshot.gc_stats.heap_utilization > thresholds.max_heap_utilization {
            self.emit_event(GcEvent {
                timestamp: SystemTime::now(),
                event_type: GcEventType::HeapUtilizationAlert,
                severity: EventSeverity::Warning,
                message: format!("Heap utilization exceeded threshold: {:.2}% > {:.2}%", 
                    snapshot.gc_stats.heap_utilization * 100.0, thresholds.max_heap_utilization * 100.0),
                metrics: HashMap::new(),
            });
        }
        
        // Check concurrent efficiency
        if let Some(concurrent_stats) = &snapshot.concurrent_stats {
            if concurrent_stats.concurrent_marking_efficiency < thresholds.min_concurrent_efficiency {
                self.emit_event(GcEvent {
                    timestamp: SystemTime::now(),
                    event_type: GcEventType::ConcurrentEfficiencyAlert,
                    severity: EventSeverity::Warning,
                    message: format!("Concurrent marking efficiency below threshold: {:.2}% < {:.2}%", 
                        concurrent_stats.concurrent_marking_efficiency * 100.0, thresholds.min_concurrent_efficiency * 100.0),
                    metrics: HashMap::new(),
                });
            }
        }
    }

    /// Update performance trends
    fn update_performance_trends(&self, snapshot: &GcMetricsSnapshot) {
        let mut trends = self.performance_trends.write().unwrap();
        let timestamp = snapshot.timestamp;
        
        // Update GC metrics trends
        self.update_trend(&mut trends, "avg_pause_time", timestamp, snapshot.gc_stats.avg_pause_time.as_millis() as f64);
        self.update_trend(&mut trends, "gc_overhead", timestamp, snapshot.gc_stats.gc_overhead);
        self.update_trend(&mut trends, "heap_utilization", timestamp, snapshot.gc_stats.heap_utilization);
        self.update_trend(&mut trends, "allocation_rate", timestamp, snapshot.gc_stats.allocation_rate);
        
        // Update system metrics trends
        self.update_trend(&mut trends, "cpu_usage", timestamp, snapshot.system_metrics.cpu_usage);
        self.update_trend(&mut trends, "memory_usage", timestamp, 
            (snapshot.system_metrics.total_memory - snapshot.system_metrics.available_memory) as f64);
    }

    /// Update individual trend
    fn update_trend(&self, trends: &mut HashMap<String, PerformanceTrend>, 
                   metric_name: &str, timestamp: SystemTime, value: f64) {
        let trend = trends.entry(metric_name.to_string()).or_insert_with(|| {
            PerformanceTrend {
                metric_name: metric_name.to_string(),
                data_points: VecDeque::new(),
                trend_direction: TrendDirection::Stable,
                trend_strength: 0.0,
                predicted_value: value,
                confidence_interval: (value, value),
            }
        });
        
        trend.data_points.push_back((timestamp, value));
        
        // Keep only last 100 data points
        if trend.data_points.len() > 100 {
            trend.data_points.pop_front();
        }
        
        // Calculate trend direction and strength
        if trend.data_points.len() >= 5 {
            let recent_values: Vec<f64> = trend.data_points.iter().rev().take(5).map(|(_, v)| *v).collect();
            let slope = self.calculate_trend_slope(&recent_values);
            
            trend.trend_direction = if slope > 0.05 {
                TrendDirection::Increasing
            } else if slope < -0.05 {
                TrendDirection::Decreasing
            } else {
                TrendDirection::Stable
            };
            
            trend.trend_strength = slope.abs();
            trend.predicted_value = value + slope;
        }
    }

    /// Calculate trend slope
    fn calculate_trend_slope(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let n = values.len() as f64;
        let sum_x = (0..values.len()).map(|i| i as f64).sum::<f64>();
        let sum_y = values.iter().sum::<f64>();
        let sum_xy = values.iter().enumerate().map(|(i, &y)| i as f64 * y).sum::<f64>();
        let sum_x2 = (0..values.len()).map(|i| (i as f64) * (i as f64)).sum::<f64>();
        
        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator == 0.0 {
            return 0.0;
        }
        
        (n * sum_xy - sum_x * sum_y) / denominator
    }

    /// Generate tuning recommendations
    fn generate_tuning_recommendations(&self, snapshot: &GcMetricsSnapshot) {
        let mut recommendations = self.tuning_recommendations.write().unwrap();
        recommendations.clear();
        
        // Check pause time
        if snapshot.gc_stats.avg_pause_time.as_millis() > 20 {
            recommendations.push(TuningRecommendation {
                recommendation_type: RecommendationType::EnableConcurrentCollection,
                priority: RecommendationPriority::High,
                description: "Enable concurrent collection to reduce pause times".to_string(),
                expected_impact: "Reduce pause times by 50-80%".to_string(),
                config_changes: HashMap::from([
                    ("concurrent_collection".to_string(), "true".to_string()),
                    ("concurrent_threads".to_string(), "4".to_string()),
                ]),
                confidence: 0.85,
            });
        }
        
        // Check GC overhead
        if snapshot.gc_stats.gc_overhead > 0.05 {
            recommendations.push(TuningRecommendation {
                recommendation_type: RecommendationType::IncreaseHeapSize,
                priority: RecommendationPriority::Medium,
                description: "Increase heap size to reduce GC frequency".to_string(),
                expected_impact: "Reduce GC overhead by 30-50%".to_string(),
                config_changes: HashMap::from([
                    ("initial_heap_size".to_string(), "256MB".to_string()),
                    ("max_heap_size".to_string(), "2GB".to_string()),
                ]),
                confidence: 0.75,
            });
        }
        
        // Check heap utilization
        if snapshot.gc_stats.heap_utilization > 0.85 {
            recommendations.push(TuningRecommendation {
                recommendation_type: RecommendationType::IncreaseHeapSize,
                priority: RecommendationPriority::High,
                description: "Heap utilization is high, consider increasing heap size".to_string(),
                expected_impact: "Improve application performance and reduce GC pressure".to_string(),
                config_changes: HashMap::from([
                    ("max_heap_size".to_string(), "4GB".to_string()),
                ]),
                confidence: 0.90,
            });
        }
    }

    /// Emit monitoring event
    fn emit_event(&self, event: GcEvent) {
        // Add to event queue
        {
            let mut queue = self.event_queue.lock().unwrap();
            queue.push_back(event.clone());
            
            // Limit queue size
            if queue.len() > 1000 {
                queue.pop_front();
            }
        }
        
        // Update event counters
        {
            let mut counters = self.event_counters.write().unwrap();
            counters.entry(event.event_type.clone())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(1, Ordering::Relaxed);
        }
        
        // Log event
        if self.config.logging {
            self.log_event(&event);
        }
        
        // Call alert callbacks
        if self.config.alerting && event.severity >= EventSeverity::Warning {
            let callbacks = self.alert_callbacks.read().unwrap();
            for callback in callbacks.iter() {
                callback(&event);
            }
        }
    }

    /// Log event to file
    fn log_event(&self, event: &GcEvent) {
        if let Some(ref mut file) = *self.log_file.lock().unwrap() {
            let log_line = format!("{:?} [{:?}] {:?}: {}\n", 
                event.timestamp, event.severity, event.event_type, event.message);
            
            if let Err(e) = file.write_all(log_line.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }

    /// Collect system metrics
    fn collect_system_metrics(&self) -> SystemMetrics {
        // Simple system metrics collection
        // In a real implementation, this would use system APIs
        SystemMetrics {
            cpu_usage: 0.0, // Placeholder
            available_memory: 1024 * 1024 * 1024, // 1GB placeholder
            total_memory: 4 * 1024 * 1024 * 1024, // 4GB placeholder
            load_average: 0.0, // Placeholder
            thread_count: 10, // Placeholder
        }
    }

    /// Cleanup old data
    fn cleanup_old_data(&self) {
        // Metrics history is cleaned up in store_metrics_snapshot
        
        // Clean up event queue
        let mut queue = self.event_queue.lock().unwrap();
        let cutoff_time = SystemTime::now() - Duration::from_secs(3600); // Keep 1 hour of events
        
        while let Some(front) = queue.front() {
            if front.timestamp < cutoff_time {
                queue.pop_front();
            } else {
                break;
            }
        }
    }

    /// Clone for thread (simplified)
    fn clone_for_thread(&self) -> Self {
        // This is a simplified clone for the monitoring thread
        // In a real implementation, you'd need proper Arc/Rc handling
        Self {
            config: self.config.clone(),
            gc_ref: self.gc_ref.clone(),
            concurrent_gc_ref: self.concurrent_gc_ref.clone(),
            memory_manager_ref: self.memory_manager_ref.clone(),
            profiler_ref: self.profiler_ref.clone(),
            heap_optimizer_ref: self.heap_optimizer_ref.clone(),
            running: AtomicBool::new(true),
            monitor_thread: Mutex::new(None),
            event_queue: Arc::clone(&self.event_queue),
            metrics_history: Arc::clone(&self.metrics_history),
            performance_trends: Arc::clone(&self.performance_trends),
            tuning_recommendations: Arc::clone(&self.tuning_recommendations),
            alert_callbacks: Arc::clone(&self.alert_callbacks),
            log_file: Arc::clone(&self.log_file),
            event_counters: Arc::clone(&self.event_counters),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_monitor_creation() {
        let config = GcMonitorConfig::default();
        let monitor = GcMonitor::new(config).unwrap();
        
        assert!(!monitor.running.load(Ordering::Relaxed));
    }

    #[test]
    fn test_alert_thresholds() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.max_pause_time_ms, 50);
        assert_eq!(thresholds.max_gc_overhead, 0.1);
        assert_eq!(thresholds.max_heap_utilization, 0.9);
    }

    #[test]
    fn test_event_creation() {
        let event = GcEvent {
            timestamp: SystemTime::now(),
            event_type: GcEventType::CollectionStarted,
            severity: EventSeverity::Info,
            message: "Test event".to_string(),
            metrics: HashMap::new(),
        };
        
        assert_eq!(event.event_type, GcEventType::CollectionStarted);
        assert_eq!(event.severity, EventSeverity::Info);
    }

    #[test]
    fn test_tuning_recommendation() {
        let recommendation = TuningRecommendation {
            recommendation_type: RecommendationType::IncreaseHeapSize,
            priority: RecommendationPriority::High,
            description: "Test recommendation".to_string(),
            expected_impact: "Test impact".to_string(),
            config_changes: HashMap::new(),
            confidence: 0.8,
        };
        
        assert_eq!(recommendation.recommendation_type, RecommendationType::IncreaseHeapSize);
        assert_eq!(recommendation.priority, RecommendationPriority::High);
        assert_eq!(recommendation.confidence, 0.8);
    }
}
