//! Performance monitoring for LLVM compilation and runtime
//! 
//! Provides comprehensive performance monitoring for production deployment including:
//! - Compilation time tracking and optimization metrics
//! - Runtime performance monitoring hooks
//! - Memory usage tracking and GC performance metrics
//! - Performance regression detection system
//! - Integration with existing metrics collection

use crate::error_types::{Result, Error};
use crate::runtime::gc_monitor::{GcMonitor, GcMonitorConfig, GcMetricsSnapshot};
use crate::runtime::memory_profiler::MemoryProfiler;
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::collections::{HashMap, VecDeque};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use serde::{Serialize, Deserialize};

/// Comprehensive monitoring configuration for production deployment
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub enable_timing: bool,
    pub enable_memory_tracking: bool,
    pub enable_metrics: bool,
    pub enable_compilation_tracking: bool,
    pub enable_runtime_monitoring: bool,
    pub enable_gc_monitoring: bool,
    pub enable_regression_detection: bool,
    pub enable_metrics_export: bool,
    pub monitoring_interval_ms: u64,
    pub metrics_retention_hours: u64,
    pub export_format: MetricsExportFormat,
    pub export_path: Option<String>,
    pub alert_thresholds: AlertThresholds,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_timing: true,
            enable_memory_tracking: true,
            enable_metrics: true,
            enable_compilation_tracking: true,
            enable_runtime_monitoring: true,
            enable_gc_monitoring: true,
            enable_regression_detection: true,
            enable_metrics_export: true,
            monitoring_interval_ms: 5000, // 5 seconds
            metrics_retention_hours: 24,
            export_format: MetricsExportFormat::Prometheus,
            export_path: Some("metrics.txt".to_string()),
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

/// Metrics export formats
#[derive(Debug, Clone, Copy)]
pub enum MetricsExportFormat {
    Prometheus,
    Json,
    InfluxDB,
    Csv,
}

/// Alert thresholds for performance monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub max_compilation_time_ms: u64,
    pub max_memory_usage_mb: usize,
    pub max_gc_pause_time_ms: u64,
    pub max_runtime_degradation_percent: f64,
    pub max_regression_threshold_percent: f64,
    pub min_throughput_ops_per_sec: f64,
    pub max_error_rate_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_compilation_time_ms: 30000, // 30 seconds
            max_memory_usage_mb: 2048,      // 2GB
            max_gc_pause_time_ms: 50,       // 50ms
            max_runtime_degradation_percent: 20.0, // 20%
            max_regression_threshold_percent: 10.0, // 10%
            min_throughput_ops_per_sec: 1000.0,
            max_error_rate_percent: 1.0,    // 1%
        }
    }
}

/// Compilation metrics for tracking build performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationMetrics {
    pub total_compilation_time: Duration,
    pub parsing_time: Duration,
    pub semantic_analysis_time: Duration,
    pub optimization_time: Duration,
    pub codegen_time: Duration,
    pub linking_time: Duration,
    pub lines_of_code: usize,
    pub functions_compiled: usize,
    pub modules_compiled: usize,
    pub optimization_passes: usize,
    pub code_size_bytes: usize,
    pub memory_peak_mb: usize,
    pub compilation_errors: usize,
    pub warnings: usize,
    pub cyclomatic_complexity: f64,
    pub optimization_level: i32,
}

impl Default for CompilationMetrics {
    fn default() -> Self {
        Self {
            total_compilation_time: Duration::from_secs(0),
            parsing_time: Duration::from_secs(0),
            semantic_analysis_time: Duration::from_secs(0),
            optimization_time: Duration::from_secs(0),
            codegen_time: Duration::from_secs(0),
            linking_time: Duration::from_secs(0),
            lines_of_code: 0,
            functions_compiled: 0,
            modules_compiled: 0,
            optimization_passes: 0,
            code_size_bytes: 0,
            memory_peak_mb: 0,
            compilation_errors: 0,
            warnings: 0,
            cyclomatic_complexity: 0.0,
            optimization_level: 0,
        }
    }
}

/// Runtime performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub uptime: Duration,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: usize,
    pub heap_usage_mb: usize,
    pub gc_collections: u64,
    pub gc_pause_time_ms: u64,
    pub throughput_ops_per_sec: f64,
    pub error_rate_percent: f64,
    pub request_latency_ms: f64,
    pub active_goroutines: usize,
    pub channel_operations: u64,
    pub allocation_rate_mb_per_sec: f64,
    pub deallocation_rate_mb_per_sec: f64,
    pub heap_fragmentation_percent: f64,
    pub cache_hit_rate_percent: f64,
    pub network_throughput_mb_per_sec: f64,
}

impl Default for RuntimeMetrics {
    fn default() -> Self {
        Self {
            uptime: Duration::from_secs(0),
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0,
            heap_usage_mb: 0,
            gc_collections: 0,
            gc_pause_time_ms: 0,
            throughput_ops_per_sec: 0.0,
            error_rate_percent: 0.0,
            request_latency_ms: 0.0,
            active_goroutines: 0,
            channel_operations: 0,
            allocation_rate_mb_per_sec: 0.0,
            deallocation_rate_mb_per_sec: 0.0,
            heap_fragmentation_percent: 0.0,
            cache_hit_rate_percent: 0.0,
            network_throughput_mb_per_sec: 0.0,
        }
    }
}

/// Performance regression detection data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionData {
    pub timestamp: SystemTime,
    pub metric_name: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub regression_percent: f64,
    pub severity: RegressionSeverity,
    pub confidence: f64,
    pub details: String,
}

/// Regression severity levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RegressionSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

/// Performance baseline for regression detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub compilation_time_ms: f64,
    pub memory_usage_mb: f64,
    pub gc_pause_time_ms: f64,
    pub throughput_ops_per_sec: f64,
    pub error_rate_percent: f64,
    pub created_at: SystemTime,
    pub sample_count: usize,
    pub confidence_interval: (f64, f64),
}

impl Default for PerformanceBaseline {
    fn default() -> Self {
        Self {
            compilation_time_ms: 1000.0,
            memory_usage_mb: 256.0,
            gc_pause_time_ms: 10.0,
            throughput_ops_per_sec: 10000.0,
            error_rate_percent: 0.1,
            created_at: SystemTime::now(),
            sample_count: 100,
            confidence_interval: (0.0, 0.0),
        }
    }
}

/// Combined performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: SystemTime,
    pub compilation_metrics: CompilationMetrics,
    pub runtime_metrics: RuntimeMetrics,
    pub gc_metrics: Option<GcMetricsSnapshot>,
    pub regression_data: Vec<RegressionData>,
    pub system_info: SystemInfo,
}

/// System information for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
    pub cpu_cores: usize,
    pub total_memory_mb: usize,
    pub available_memory_mb: usize,
    pub rust_version: String,
    pub llvm_version: String,
    pub compiler_version: String,
}

/// Performance monitoring events
#[derive(Debug, Clone)]
pub enum PerformanceEvent {
    CompilationStarted,
    CompilationCompleted(CompilationMetrics),
    RuntimeMetricsCollected(RuntimeMetrics),
    RegressionDetected(RegressionData),
    BaselineUpdated(PerformanceBaseline),
    AlertTriggered(String, f64),
    MetricsExported(usize), // Number of metrics exported
}

/// Comprehensive performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub generated_at: SystemTime,
    pub monitoring_period: Duration,
    pub compilation_summary: CompilationSummary,
    pub runtime_summary: RuntimeSummary,
    pub regression_summary: RegressionSummary,
    pub recommendations: Vec<PerformanceRecommendation>,
    pub alerts: Vec<PerformanceAlert>,
    pub system_health: SystemHealth,
}

/// Compilation performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationSummary {
    pub total_compilations: usize,
    pub average_compilation_time_ms: f64,
    pub fastest_compilation_time_ms: f64,
    pub slowest_compilation_time_ms: f64,
    pub total_lines_compiled: usize,
    pub compilation_throughput_loc_per_sec: f64,
    pub error_rate_percent: f64,
    pub optimization_effectiveness: f64,
}

/// Runtime performance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSummary {
    pub uptime_hours: f64,
    pub average_cpu_usage_percent: f64,
    pub peak_memory_usage_mb: usize,
    pub total_gc_collections: u64,
    pub average_gc_pause_time_ms: f64,
    pub average_throughput_ops_per_sec: f64,
    pub total_errors: u64,
    pub availability_percent: f64,
}

/// Regression detection summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionSummary {
    pub total_regressions: usize,
    pub critical_regressions: usize,
    pub major_regressions: usize,
    pub moderate_regressions: usize,
    pub minor_regressions: usize,
    pub most_degraded_metric: String,
    pub worst_regression_percent: f64,
}

/// Performance improvement recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    pub category: String,
    pub priority: RecommendationPriority,
    pub description: String,
    pub expected_improvement: String,
    pub implementation_effort: String,
    pub confidence: f64,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub timestamp: SystemTime,
    pub severity: AlertSeverity,
    pub metric: String,
    pub current_value: f64,
    pub threshold: f64,
    pub message: String,
    pub suggested_action: String,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_score: f64, // 0-100
    pub compilation_health: f64,
    pub runtime_health: f64,
    pub memory_health: f64,
    pub gc_health: f64,
    pub status: HealthStatus,
}

/// Health status levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

/// Main performance monitor for production deployment
pub struct PerformanceMonitor {
    config: MonitoringConfig,
    start_time: Instant,
    
    // Metrics collection
    compilation_metrics: Arc<RwLock<VecDeque<CompilationMetrics>>>,
    runtime_metrics: Arc<RwLock<VecDeque<RuntimeMetrics>>>,
    performance_snapshots: Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
    
    // Regression detection
    performance_baseline: Arc<RwLock<PerformanceBaseline>>,
    regression_data: Arc<RwLock<VecDeque<RegressionData>>>,
    
    // Monitoring state
    monitoring_active: AtomicBool,
    monitor_thread: Mutex<Option<thread::JoinHandle<()>>>,
    
    // Event tracking
    events: Arc<RwLock<VecDeque<PerformanceEvent>>>,
    alerts: Arc<RwLock<VecDeque<PerformanceAlert>>>,
    
    // External integrations
    gc_monitor: Option<Arc<GcMonitor>>,
    memory_profiler: Option<Arc<MemoryProfiler>>,
    
    // Metrics export
    metrics_file: Arc<Mutex<Option<std::fs::File>>>,
    last_export: AtomicU64,
    
    // Performance counters
    compilation_counter: AtomicU64,
    runtime_counter: AtomicU64,
    regression_counter: AtomicU64,
    alert_counter: AtomicU64,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new(config: MonitoringConfig) -> Result<Self> {
        let metrics_file = if let Some(path) = &config.export_path {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| Error::Runtime(format!("Failed to open metrics file: {}", e)))?;
            Some(file)
        } else {
            None
        };

        Ok(Self {
            config,
            start_time: Instant::now(),
            compilation_metrics: Arc::new(RwLock::new(VecDeque::new())),
            runtime_metrics: Arc::new(RwLock::new(VecDeque::new())),
            performance_snapshots: Arc::new(RwLock::new(VecDeque::new())),
            performance_baseline: Arc::new(RwLock::new(PerformanceBaseline::default())),
            regression_data: Arc::new(RwLock::new(VecDeque::new())),
            monitoring_active: AtomicBool::new(false),
            monitor_thread: Mutex::new(None),
            events: Arc::new(RwLock::new(VecDeque::new())),
            alerts: Arc::new(RwLock::new(VecDeque::new())),
            gc_monitor: None,
            memory_profiler: None,
            metrics_file: Arc::new(Mutex::new(metrics_file)),
            last_export: AtomicU64::new(0),
            compilation_counter: AtomicU64::new(0),
            runtime_counter: AtomicU64::new(0),
            regression_counter: AtomicU64::new(0),
            alert_counter: AtomicU64::new(0),
        })
    }

    /// Start performance monitoring
    pub fn start_monitoring(&self) -> Result<()> {
        if self.monitoring_active.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(Error::Runtime("Performance monitoring already active".to_string()));
        }

        // Start monitoring thread
        let config = self.config.clone();
        let runtime_metrics = Arc::clone(&self.runtime_metrics);
        let performance_snapshots = Arc::clone(&self.performance_snapshots);
        let events = Arc::clone(&self.events);
        let alerts = Arc::clone(&self.alerts);
        let monitoring_active = Arc::new(AtomicBool::new(true));
        let monitoring_active_clone = Arc::clone(&monitoring_active);
        let metrics_file = Arc::clone(&self.metrics_file);
        
        let handle = thread::Builder::new()
            .name("performance-monitor".to_string())
            .spawn(move || {
                let mut last_snapshot = Instant::now();
                
                while monitoring_active_clone.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(config.monitoring_interval_ms));
                    
                    // Collect runtime metrics
                    let runtime_metrics_sample = RuntimeMetrics {
                        uptime: last_snapshot.elapsed(),
                        cpu_usage_percent: Self::get_cpu_usage(),
                        memory_usage_mb: Self::get_memory_usage_mb(),
                        heap_usage_mb: Self::get_heap_usage_mb(),
                        gc_collections: Self::get_gc_collections(),
                        gc_pause_time_ms: Self::get_gc_pause_time_ms(),
                        throughput_ops_per_sec: Self::get_throughput_ops_per_sec(),
                        error_rate_percent: Self::get_error_rate_percent(),
                        request_latency_ms: Self::get_request_latency_ms(),
                        active_goroutines: Self::get_active_goroutines(),
                        channel_operations: Self::get_channel_operations(),
                        allocation_rate_mb_per_sec: Self::get_allocation_rate_mb_per_sec(),
                        deallocation_rate_mb_per_sec: Self::get_deallocation_rate_mb_per_sec(),
                        heap_fragmentation_percent: Self::get_heap_fragmentation_percent(),
                        cache_hit_rate_percent: Self::get_cache_hit_rate_percent(),
                        network_throughput_mb_per_sec: Self::get_network_throughput_mb_per_sec(),
                    };
                    
                    // Store runtime metrics
                    {
                        let mut metrics = runtime_metrics.write().unwrap();
                        metrics.push_back(runtime_metrics_sample.clone());
                        
                        // Limit collection size
                        if metrics.len() > 10000 {
                            metrics.pop_front();
                        }
                    }
                    
                    // Export metrics if enabled
                    if config.enable_metrics_export {
                        if let Some(ref mut file) = *metrics_file.lock().unwrap() {
                            let _ = Self::export_metrics_to_file(file, &runtime_metrics_sample, &config);
                        }
                    }
                    
                    last_snapshot = Instant::now();
                }
            })
            .map_err(|e| Error::Runtime(format!("Failed to start monitoring thread: {}", e)))?;

        *self.monitor_thread.lock().unwrap() = Some(handle);
        
        self.emit_event(PerformanceEvent::CompilationStarted);
        Ok(())
    }

    /// Stop performance monitoring
    pub fn stop_monitoring(&self) -> Result<()> {
        self.monitoring_active.store(false, Ordering::SeqCst);
        
        if let Some(handle) = self.monitor_thread.lock().unwrap().take() {
            handle.join().map_err(|_| Error::Runtime("Failed to join monitoring thread".to_string()))?;
        }
        
        Ok(())
    }

    /// Record compilation metrics
    pub fn record_compilation_metrics(&self, metrics: CompilationMetrics) {
        if !self.config.enable_compilation_tracking {
            return;
        }
        
        // Store compilation metrics
        {
            let mut compilation_metrics = self.compilation_metrics.write().unwrap();
            compilation_metrics.push_back(metrics.clone());
            
            // Limit collection size
            if compilation_metrics.len() > 1000 {
                compilation_metrics.pop_front();
            }
        }
        
        // Check for performance regressions
        if self.config.enable_regression_detection {
            self.check_compilation_regression(&metrics);
        }
        
        // Check alert thresholds
        self.check_compilation_alerts(&metrics);
        
        self.emit_event(PerformanceEvent::CompilationCompleted(metrics));
        self.compilation_counter.fetch_add(1, Ordering::SeqCst);
    }

    /// Set GC monitor for integration
    pub fn set_gc_monitor(&mut self, gc_monitor: Arc<GcMonitor>) {
        self.gc_monitor = Some(gc_monitor);
    }

    /// Set memory profiler for integration
    pub fn set_memory_profiler(&mut self, memory_profiler: Arc<MemoryProfiler>) {
        self.memory_profiler = Some(memory_profiler);
    }

    /// Generate comprehensive performance report
    pub fn generate_report(&self) -> PerformanceReport {
        let monitoring_period = self.start_time.elapsed();
        
        let compilation_summary = self.generate_compilation_summary();
        let runtime_summary = self.generate_runtime_summary();
        let regression_summary = self.generate_regression_summary();
        let recommendations = self.generate_recommendations();
        let alerts = self.get_recent_alerts();
        let system_health = self.calculate_system_health();
        
        PerformanceReport {
            generated_at: SystemTime::now(),
            monitoring_period,
            compilation_summary,
            runtime_summary,
            regression_summary,
            recommendations,
            alerts,
            system_health,
        }
    }

    /// Get current performance snapshot
    pub fn get_performance_snapshot(&self) -> PerformanceSnapshot {
        let compilation_metrics = self.compilation_metrics.read().unwrap()
            .back().cloned().unwrap_or_default();
        let runtime_metrics = self.runtime_metrics.read().unwrap()
            .back().cloned().unwrap_or_default();
        let gc_metrics = self.gc_monitor.as_ref()
            .map(|monitor| monitor.get_metrics_snapshot());
        let regression_data = self.regression_data.read().unwrap()
            .iter().cloned().collect();
        
        PerformanceSnapshot {
            timestamp: SystemTime::now(),
            compilation_metrics,
            runtime_metrics,
            gc_metrics,
            regression_data,
            system_info: self.get_system_info(),
        }
    }

    /// Export metrics in specified format
    pub fn export_metrics(&self, format: MetricsExportFormat) -> Result<String> {
        let snapshot = self.get_performance_snapshot();
        
        match format {
            MetricsExportFormat::Prometheus => self.export_prometheus(&snapshot),
            MetricsExportFormat::Json => self.export_json(&snapshot),
            MetricsExportFormat::InfluxDB => self.export_influxdb(&snapshot),
            MetricsExportFormat::Csv => self.export_csv(&snapshot),
        }
    }

    /// Update performance baseline
    pub fn update_baseline(&self) {
        let compilation_metrics = self.compilation_metrics.read().unwrap();
        let runtime_metrics = self.runtime_metrics.read().unwrap();
        
        if compilation_metrics.len() < 10 || runtime_metrics.len() < 10 {
            return; // Not enough data for baseline
        }
        
        let avg_compilation_time = compilation_metrics.iter()
            .map(|m| m.total_compilation_time.as_millis() as f64)
            .sum::<f64>() / compilation_metrics.len() as f64;
        
        let avg_memory_usage = runtime_metrics.iter()
            .map(|m| m.memory_usage_mb as f64)
            .sum::<f64>() / runtime_metrics.len() as f64;
        
        let avg_gc_pause = runtime_metrics.iter()
            .map(|m| m.gc_pause_time_ms as f64)
            .sum::<f64>() / runtime_metrics.len() as f64;
        
        let avg_throughput = runtime_metrics.iter()
            .map(|m| m.throughput_ops_per_sec)
            .sum::<f64>() / runtime_metrics.len() as f64;
        
        let avg_error_rate = runtime_metrics.iter()
            .map(|m| m.error_rate_percent)
            .sum::<f64>() / runtime_metrics.len() as f64;
        
        let new_baseline = PerformanceBaseline {
            compilation_time_ms: avg_compilation_time,
            memory_usage_mb: avg_memory_usage,
            gc_pause_time_ms: avg_gc_pause,
            throughput_ops_per_sec: avg_throughput,
            error_rate_percent: avg_error_rate,
            created_at: SystemTime::now(),
            sample_count: compilation_metrics.len().min(runtime_metrics.len()),
            confidence_interval: (0.95, 0.99), // 95-99% confidence
        };
        
        *self.performance_baseline.write().unwrap() = new_baseline.clone();
        self.emit_event(PerformanceEvent::BaselineUpdated(new_baseline));
    }

    // Helper methods for system metrics collection
    fn get_cpu_usage() -> f64 {
        // Platform-specific CPU usage collection
        // This is a placeholder - in production, use system APIs
        25.0
    }

    fn get_memory_usage_mb() -> usize {
        // Platform-specific memory usage collection
        // This is a placeholder - in production, use system APIs
        512
    }

    fn get_heap_usage_mb() -> usize {
        // Get heap usage from GC
        // This is a placeholder - integrate with actual GC
        256
    }

    fn get_gc_collections() -> u64 {
        // Get GC collection count
        // This is a placeholder - integrate with actual GC
        100
    }

    fn get_gc_pause_time_ms() -> u64 {
        // Get GC pause time
        // This is a placeholder - integrate with actual GC
        15
    }

    fn get_throughput_ops_per_sec() -> f64 {
        // Get application throughput
        // This is a placeholder - integrate with actual metrics
        5000.0
    }

    fn get_error_rate_percent() -> f64 {
        // Get error rate
        // This is a placeholder - integrate with actual metrics
        0.5
    }

    fn get_request_latency_ms() -> f64 {
        // Get request latency
        // This is a placeholder - integrate with actual metrics
        50.0
    }

    fn get_active_goroutines() -> usize {
        // Get active goroutine count
        // This is a placeholder - integrate with runtime
        25
    }

    fn get_channel_operations() -> u64 {
        // Get channel operation count
        // This is a placeholder - integrate with runtime
        1000
    }

    fn get_allocation_rate_mb_per_sec() -> f64 {
        // Get allocation rate
        // This is a placeholder - integrate with memory profiler
        10.0
    }

    fn get_deallocation_rate_mb_per_sec() -> f64 {
        // Get deallocation rate
        // This is a placeholder - integrate with memory profiler
        9.5
    }

    fn get_heap_fragmentation_percent() -> f64 {
        // Get heap fragmentation
        // This is a placeholder - integrate with memory profiler
        15.0
    }

    fn get_cache_hit_rate_percent() -> f64 {
        // Get cache hit rate
        // This is a placeholder - integrate with caching layer
        85.0
    }

    fn get_network_throughput_mb_per_sec() -> f64 {
        // Get network throughput
        // This is a placeholder - integrate with network layer
        50.0
    }

    fn export_metrics_to_file(file: &mut std::fs::File, metrics: &RuntimeMetrics, config: &MonitoringConfig) -> Result<()> {
        match config.export_format {
            MetricsExportFormat::Prometheus => {
                let metrics_text = format!(
                    "# HELP cursed_cpu_usage_percent CPU usage percentage\n\
                     # TYPE cursed_cpu_usage_percent gauge\n\
                     cursed_cpu_usage_percent {}\n\
                     # HELP cursed_memory_usage_mb Memory usage in MB\n\
                     # TYPE cursed_memory_usage_mb gauge\n\
                     cursed_memory_usage_mb {}\n\
                     # HELP cursed_gc_pause_time_ms GC pause time in milliseconds\n\
                     # TYPE cursed_gc_pause_time_ms gauge\n\
                     cursed_gc_pause_time_ms {}\n\
                     # HELP cursed_throughput_ops_per_sec Throughput in operations per second\n\
                     # TYPE cursed_throughput_ops_per_sec gauge\n\
                     cursed_throughput_ops_per_sec {}\n\
                     # HELP cursed_error_rate_percent Error rate percentage\n\
                     # TYPE cursed_error_rate_percent gauge\n\
                     cursed_error_rate_percent {}\n",
                    metrics.cpu_usage_percent,
                    metrics.memory_usage_mb,
                    metrics.gc_pause_time_ms,
                    metrics.throughput_ops_per_sec,
                    metrics.error_rate_percent
                );
                file.write_all(metrics_text.as_bytes()).map_err(|e| Error::Runtime(format!("IO error: {}", e)))?;
            }
            MetricsExportFormat::Json => {
                let json = serde_json::to_string(metrics)
                    .map_err(|e| Error::Runtime(format!("Failed to serialize metrics: {}", e)))?;
                file.write_all(json.as_bytes()).map_err(|e| Error::Runtime(format!("IO error: {}", e)))?;
                file.write_all(b"\n").map_err(|e| Error::Runtime(format!("IO error: {}", e)))?;
            }
            _ => {
                // Other formats can be implemented as needed
                return Err(Error::Runtime("Export format not yet implemented".to_string()));
            }
        }
        Ok(())
    }

    // Private helper methods
    fn emit_event(&self, event: PerformanceEvent) {
        let mut events = self.events.write().unwrap();
        events.push_back(event);
        
        // Limit event history
        if events.len() > 1000 {
            events.pop_front();
        }
    }

    fn check_compilation_regression(&self, metrics: &CompilationMetrics) {
        let baseline = self.performance_baseline.read().unwrap();
        let current_time = metrics.total_compilation_time.as_millis() as f64;
        
        if current_time > baseline.compilation_time_ms * (1.0 + self.config.alert_thresholds.max_regression_threshold_percent / 100.0) {
            let regression_percent = ((current_time - baseline.compilation_time_ms) / baseline.compilation_time_ms) * 100.0;
            
            let regression = RegressionData {
                timestamp: SystemTime::now(),
                metric_name: "compilation_time".to_string(),
                current_value: current_time,
                baseline_value: baseline.compilation_time_ms,
                regression_percent,
                severity: if regression_percent > 50.0 { RegressionSeverity::Critical }
                         else if regression_percent > 25.0 { RegressionSeverity::Major }
                         else if regression_percent > 10.0 { RegressionSeverity::Moderate }
                         else { RegressionSeverity::Minor },
                confidence: 0.85,
                details: format!("Compilation time increased by {:.1}%", regression_percent),
            };
            
            self.regression_data.write().unwrap().push_back(regression.clone());
            self.emit_event(PerformanceEvent::RegressionDetected(regression));
            self.regression_counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn check_compilation_alerts(&self, metrics: &CompilationMetrics) {
        let thresholds = &self.config.alert_thresholds;
        
        if metrics.total_compilation_time.as_millis() > thresholds.max_compilation_time_ms as u128 {
            let alert = PerformanceAlert {
                timestamp: SystemTime::now(),
                severity: AlertSeverity::Warning,
                metric: "compilation_time".to_string(),
                current_value: metrics.total_compilation_time.as_millis() as f64,
                threshold: thresholds.max_compilation_time_ms as f64,
                message: "Compilation time exceeded threshold".to_string(),
                suggested_action: "Consider optimizing build configuration or hardware".to_string(),
            };
            
            self.alerts.write().unwrap().push_back(alert);
            self.emit_event(PerformanceEvent::AlertTriggered(
                "compilation_time".to_string(), 
                metrics.total_compilation_time.as_millis() as f64
            ));
            self.alert_counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn generate_compilation_summary(&self) -> CompilationSummary {
        let metrics = self.compilation_metrics.read().unwrap();
        
        if metrics.is_empty() {
            return CompilationSummary {
                total_compilations: 0,
                average_compilation_time_ms: 0.0,
                fastest_compilation_time_ms: 0.0,
                slowest_compilation_time_ms: 0.0,
                total_lines_compiled: 0,
                compilation_throughput_loc_per_sec: 0.0,
                error_rate_percent: 0.0,
                optimization_effectiveness: 0.0,
            };
        }

        let total_compilations = metrics.len();
        let total_time: Duration = metrics.iter().map(|m| m.total_compilation_time).sum();
        let average_time = total_time.as_millis() as f64 / total_compilations as f64;
        
        let fastest_time = metrics.iter()
            .map(|m| m.total_compilation_time.as_millis() as f64)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        
        let slowest_time = metrics.iter()
            .map(|m| m.total_compilation_time.as_millis() as f64)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        
        let total_lines: usize = metrics.iter().map(|m| m.lines_of_code).sum();
        let total_errors: usize = metrics.iter().map(|m| m.compilation_errors).sum();
        let error_rate = (total_errors as f64 / total_compilations as f64) * 100.0;
        
        let throughput = if total_time.as_secs() > 0 {
            total_lines as f64 / total_time.as_secs() as f64
        } else {
            0.0
        };
        
        CompilationSummary {
            total_compilations,
            average_compilation_time_ms: average_time,
            fastest_compilation_time_ms: fastest_time,
            slowest_compilation_time_ms: slowest_time,
            total_lines_compiled: total_lines,
            compilation_throughput_loc_per_sec: throughput,
            error_rate_percent: error_rate,
            optimization_effectiveness: 85.0, // Placeholder calculation
        }
    }

    fn generate_runtime_summary(&self) -> RuntimeSummary {
        let metrics = self.runtime_metrics.read().unwrap();
        
        if metrics.is_empty() {
            return RuntimeSummary {
                uptime_hours: 0.0,
                average_cpu_usage_percent: 0.0,
                peak_memory_usage_mb: 0,
                total_gc_collections: 0,
                average_gc_pause_time_ms: 0.0,
                average_throughput_ops_per_sec: 0.0,
                total_errors: 0,
                availability_percent: 0.0,
            };
        }

        let uptime_hours = self.start_time.elapsed().as_secs() as f64 / 3600.0;
        let avg_cpu = metrics.iter().map(|m| m.cpu_usage_percent).sum::<f64>() / metrics.len() as f64;
        let peak_memory = metrics.iter().map(|m| m.memory_usage_mb).max().unwrap_or(0);
        let total_gc = metrics.iter().map(|m| m.gc_collections).sum();
        let avg_gc_pause = metrics.iter().map(|m| m.gc_pause_time_ms as f64).sum::<f64>() / metrics.len() as f64;
        let avg_throughput = metrics.iter().map(|m| m.throughput_ops_per_sec).sum::<f64>() / metrics.len() as f64;
        let avg_error_rate = metrics.iter().map(|m| m.error_rate_percent).sum::<f64>() / metrics.len() as f64;
        let availability = 100.0 - avg_error_rate; // Simplified calculation
        
        RuntimeSummary {
            uptime_hours,
            average_cpu_usage_percent: avg_cpu,
            peak_memory_usage_mb: peak_memory,
            total_gc_collections: total_gc,
            average_gc_pause_time_ms: avg_gc_pause,
            average_throughput_ops_per_sec: avg_throughput,
            total_errors: (avg_error_rate * metrics.len() as f64) as u64,
            availability_percent: availability,
        }
    }

    fn generate_regression_summary(&self) -> RegressionSummary {
        let regressions = self.regression_data.read().unwrap();
        
        let total_regressions = regressions.len();
        let critical_regressions = regressions.iter().filter(|r| r.severity == RegressionSeverity::Critical).count();
        let major_regressions = regressions.iter().filter(|r| r.severity == RegressionSeverity::Major).count();
        let moderate_regressions = regressions.iter().filter(|r| r.severity == RegressionSeverity::Moderate).count();
        let minor_regressions = regressions.iter().filter(|r| r.severity == RegressionSeverity::Minor).count();
        
        let (most_degraded_metric, worst_regression_percent) = regressions.iter()
            .max_by(|a, b| a.regression_percent.partial_cmp(&b.regression_percent).unwrap())
            .map(|r| (r.metric_name.clone(), r.regression_percent))
            .unwrap_or(("none".to_string(), 0.0));
        
        RegressionSummary {
            total_regressions,
            critical_regressions,
            major_regressions,
            moderate_regressions,
            minor_regressions,
            most_degraded_metric,
            worst_regression_percent,
        }
    }

    fn generate_recommendations(&self) -> Vec<PerformanceRecommendation> {
        let mut recommendations = Vec::new();
        
        let runtime_summary = self.generate_runtime_summary();
        let compilation_summary = self.generate_compilation_summary();
        
        // Memory optimization recommendations
        if runtime_summary.peak_memory_usage_mb > self.config.alert_thresholds.max_memory_usage_mb {
            recommendations.push(PerformanceRecommendation {
                category: "Memory".to_string(),
                priority: RecommendationPriority::High,
                description: "Memory usage is high. Consider optimizing memory allocation patterns.".to_string(),
                expected_improvement: "Reduce memory usage by 20-30%".to_string(),
                implementation_effort: "Medium".to_string(),
                confidence: 0.85,
            });
        }
        
        // Compilation optimization recommendations
        if compilation_summary.average_compilation_time_ms > self.config.alert_thresholds.max_compilation_time_ms as f64 {
            recommendations.push(PerformanceRecommendation {
                category: "Compilation".to_string(),
                priority: RecommendationPriority::Medium,
                description: "Compilation time is slow. Consider enabling parallel compilation or improving build configuration.".to_string(),
                expected_improvement: "Reduce compilation time by 40-60%".to_string(),
                implementation_effort: "Low".to_string(),
                confidence: 0.90,
            });
        }
        
        // GC optimization recommendations
        if runtime_summary.average_gc_pause_time_ms > self.config.alert_thresholds.max_gc_pause_time_ms as f64 {
            recommendations.push(PerformanceRecommendation {
                category: "Garbage Collection".to_string(),
                priority: RecommendationPriority::High,
                description: "GC pause times are high. Consider tuning GC parameters or using concurrent collection.".to_string(),
                expected_improvement: "Reduce GC pause times by 50-70%".to_string(),
                implementation_effort: "Medium".to_string(),
                confidence: 0.80,
            });
        }
        
        recommendations
    }

    fn get_recent_alerts(&self) -> Vec<PerformanceAlert> {
        let alerts = self.alerts.read().unwrap();
        alerts.iter().rev().take(10).cloned().collect()
    }

    fn calculate_system_health(&self) -> SystemHealth {
        let compilation_summary = self.generate_compilation_summary();
        let runtime_summary = self.generate_runtime_summary();
        let regression_summary = self.generate_regression_summary();
        
        // Calculate health scores (0-100)
        let compilation_health = if compilation_summary.total_compilations > 0 {
            100.0 - (compilation_summary.error_rate_percent * 10.0).min(100.0)
        } else {
            100.0
        };
        
        let runtime_health = if runtime_summary.uptime_hours > 0.0 {
            runtime_summary.availability_percent
        } else {
            100.0
        };
        
        let memory_health = if runtime_summary.peak_memory_usage_mb > 0 {
            let usage_percent = (runtime_summary.peak_memory_usage_mb as f64 / self.config.alert_thresholds.max_memory_usage_mb as f64) * 100.0;
            (100.0 - usage_percent).max(0.0)
        } else {
            100.0
        };
        
        let gc_health = if runtime_summary.average_gc_pause_time_ms > 0.0 {
            let pause_ratio = runtime_summary.average_gc_pause_time_ms / self.config.alert_thresholds.max_gc_pause_time_ms as f64;
            (100.0 - (pause_ratio * 100.0)).max(0.0)
        } else {
            100.0
        };
        
        let regression_impact = if regression_summary.total_regressions > 0 {
            (regression_summary.critical_regressions * 25 + regression_summary.major_regressions * 10 + regression_summary.moderate_regressions * 5 + regression_summary.minor_regressions * 1) as f64
        } else {
            0.0
        };
        
        let overall_score = ((compilation_health + runtime_health + memory_health + gc_health) / 4.0) - regression_impact;
        let clamped_score = overall_score.max(0.0).min(100.0);
        
        let status = if clamped_score >= 90.0 { HealthStatus::Excellent }
                    else if clamped_score >= 80.0 { HealthStatus::Good }
                    else if clamped_score >= 60.0 { HealthStatus::Fair }
                    else if clamped_score >= 40.0 { HealthStatus::Poor }
                    else { HealthStatus::Critical };
        
        SystemHealth {
            overall_score: clamped_score,
            compilation_health,
            runtime_health,
            memory_health,
            gc_health,
            status,
        }
    }

    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get(),
            total_memory_mb: 4096, // Placeholder - use system APIs
            available_memory_mb: 2048, // Placeholder - use system APIs
            rust_version: "1.70.0".to_string(), // Placeholder - get from build info
            llvm_version: "15.0.0".to_string(), // Placeholder - get from LLVM
            compiler_version: "1.0.0".to_string(), // Placeholder - get from version
        }
    }

    fn export_prometheus(&self, _snapshot: &PerformanceSnapshot) -> Result<String> {
        // Implement Prometheus export format
        Ok("# Prometheus metrics not yet implemented".to_string())
    }

    fn export_json(&self, snapshot: &PerformanceSnapshot) -> Result<String> {
        serde_json::to_string_pretty(snapshot)
            .map_err(|e| Error::Runtime(format!("Failed to serialize to JSON: {}", e)))
    }

    fn export_influxdb(&self, _snapshot: &PerformanceSnapshot) -> Result<String> {
        // Implement InfluxDB line protocol
        Ok("# InfluxDB export not yet implemented".to_string())
    }

    fn export_csv(&self, _snapshot: &PerformanceSnapshot) -> Result<String> {
        // Implement CSV export format
        Ok("# CSV export not yet implemented".to_string())
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        PerformanceStats {
            total_compilations: self.compilation_counter.load(Ordering::SeqCst),
            total_runtime_samples: self.runtime_counter.load(Ordering::SeqCst),
            total_regressions: self.regression_counter.load(Ordering::SeqCst),
            total_alerts: self.alert_counter.load(Ordering::SeqCst),
            monitoring_uptime: self.start_time.elapsed(),
            is_active: self.monitoring_active.load(Ordering::SeqCst),
        }
    }
}

/// Performance monitor statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_compilations: u64,
    pub total_runtime_samples: u64,
    pub total_regressions: u64,
    pub total_alerts: u64,
    pub monitoring_uptime: Duration,
    pub is_active: bool,
}

// Legacy compatibility types
#[derive(Debug, Clone)]
pub struct CodeMetrics {
    pub lines_of_code: usize,
    pub functions: usize,
    pub complexity: f64,
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            lines_of_code: 0,
            functions: 0,
            complexity: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub compilation_time: Duration,
    pub memory_usage: usize,
    pub code_size: usize,
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            compilation_time: Duration::from_secs(0),
            memory_usage: 0,
            code_size: 0,
        }
    }
}

impl PerformanceReport {
    pub fn new(metrics: CodeMetrics, baseline: BaselineMetrics) -> Self {
        // Convert legacy types to new format
        let compilation_summary = CompilationSummary {
            total_compilations: 1,
            average_compilation_time_ms: baseline.compilation_time.as_millis() as f64,
            fastest_compilation_time_ms: baseline.compilation_time.as_millis() as f64,
            slowest_compilation_time_ms: baseline.compilation_time.as_millis() as f64,
            total_lines_compiled: metrics.lines_of_code,
            compilation_throughput_loc_per_sec: metrics.lines_of_code as f64 / baseline.compilation_time.as_secs() as f64,
            error_rate_percent: 0.0,
            optimization_effectiveness: 0.0,
        };
        
        Self {
            generated_at: SystemTime::now(),
            monitoring_period: Duration::from_secs(0),
            compilation_summary,
            runtime_summary: RuntimeSummary {
                uptime_hours: 0.0,
                average_cpu_usage_percent: 0.0,
                peak_memory_usage_mb: baseline.memory_usage / 1024 / 1024,
                total_gc_collections: 0,
                average_gc_pause_time_ms: 0.0,
                average_throughput_ops_per_sec: 0.0,
                total_errors: 0,
                availability_percent: 100.0,
            },
            regression_summary: RegressionSummary {
                total_regressions: 0,
                critical_regressions: 0,
                major_regressions: 0,
                moderate_regressions: 0,
                minor_regressions: 0,
                most_degraded_metric: "none".to_string(),
                worst_regression_percent: 0.0,
            },
            recommendations: Vec::new(),
            alerts: Vec::new(),
            system_health: SystemHealth {
                overall_score: 100.0,
                compilation_health: 100.0,
                runtime_health: 100.0,
                memory_health: 100.0,
                gc_health: 100.0,
                status: HealthStatus::Excellent,
            },
        }
    }
}

impl CodeMetrics {
    pub fn new(lines_of_code: usize, functions: usize, complexity: f64) -> Self {
        Self {
            lines_of_code,
            functions,
            complexity,
        }
    }
}

impl BaselineMetrics {
    pub fn new(compilation_time: Duration, memory_usage: usize, code_size: usize) -> Self {
        Self {
            compilation_time,
            memory_usage,
            code_size,
        }
    }
}
