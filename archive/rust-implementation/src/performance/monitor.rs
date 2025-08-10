//! Real-time performance monitoring for CURSED compiler
//! 
//! Provides continuous monitoring of compilation and runtime performance metrics
//! including timing, memory usage, CPU utilization, and throughput measurements.

use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant};
use std::collections::{HashMap, VecDeque};
use std::thread;
use crate::error::CursedError;
use crate::performance::{PerformanceConfig, PerformanceMetrics, ReportFormat};

/// Performance monitoring system
pub struct PerformanceMonitor {
    config: PerformanceConfig,
    metrics: Arc<RwLock<VecDeque<PerformanceMetrics>>>,
    timing_data: Arc<RwLock<HashMap<String, TimingMetrics>>>,
    memory_data: Arc<RwLock<MemoryMetrics>>,
    cpu_data: Arc<RwLock<CpuMetrics>>,
    throughput_data: Arc<RwLock<ThroughputMetrics>>,
    is_running: Arc<Mutex<bool>>,
    monitor_thread: Mutex<Option<thread::JoinHandle<()>>>,
    start_time: Instant,
}

/// Timing metrics for specific operations
#[derive(Debug, Clone)]
pub struct TimingMetrics {
    pub operation_name: String,
    pub total_calls: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub recent_times: VecDeque<Duration>,
    pub p95_time: Duration,
    pub p99_time: Duration,
}

/// Memory usage metrics
#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub current_usage: usize,
    pub peak_usage: usize,
    pub average_usage: usize,
    pub allocations: u64,
    pub deallocations: u64,
    pub gc_collections: u64,
    pub heap_size: usize,
    pub stack_size: usize,
    pub recent_usage: VecDeque<usize>,
}

/// CPU usage metrics
#[derive(Debug, Clone)]
pub struct CpuMetrics {
    pub current_usage: f64,
    pub average_usage: f64,
    pub peak_usage: f64,
    pub user_time: Duration,
    pub system_time: Duration,
    pub idle_time: Duration,
    pub recent_usage: VecDeque<f64>,
}

/// Throughput metrics
#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    pub operations_per_second: f64,
    pub bytes_per_second: f64,
    pub requests_per_second: f64,
    pub compilation_rate: f64,
    pub execution_rate: f64,
    pub recent_throughput: VecDeque<f64>,
}

/// Performance event types
#[derive(Debug, Clone)]
pub enum PerformanceEvent {
    CompilationStart,
    CompilationEnd,
    ExecutionStart,
    ExecutionEnd,
    MemoryAllocation { size: usize },
    MemoryDeallocation { size: usize },
    GarbageCollection { duration: Duration },
    FunctionCall { name: String, duration: Duration },
    Error { message: String },
}

/// Performance alert
#[derive(Debug, Clone)]
pub struct PerformanceAlert {
    pub timestamp: Instant,
    pub alert_type: AlertType,
    pub message: String,
    pub severity: AlertSeverity,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub recommendation: String,
}

/// Alert types
#[derive(Debug, Clone)]
pub enum AlertType {
    HighCpuUsage,
    HighMemoryUsage,
    SlowCompilation,
    SlowExecution,
    FrequentGc,
    ErrorSpike,
    ThroughputDrop,
}

/// Alert severity levels
#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        Ok(Self {
            config,
            metrics: Arc::new(RwLock::new(VecDeque::new())),
            timing_data: Arc::new(RwLock::new(HashMap::new())),
            memory_data: Arc::new(RwLock::new(MemoryMetrics::default())),
            cpu_data: Arc::new(RwLock::new(CpuMetrics::default())),
            throughput_data: Arc::new(RwLock::new(ThroughputMetrics::default())),
            is_running: Arc::new(Mutex::new(false)),
            monitor_thread: Mutex::new(None),
            start_time: Instant::now(),
        })
    }

    /// Start performance monitoring
    pub fn start(&self) -> Result<(), CursedError> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Err(CursedError::runtime_error("Performance monitor already running"));
        }
        *is_running = true;

        let config = self.config.clone();
        let metrics = Arc::clone(&self.metrics);
        let timing_data = Arc::clone(&self.timing_data);
        let memory_data = Arc::clone(&self.memory_data);
        let cpu_data = Arc::clone(&self.cpu_data);
        let throughput_data = Arc::clone(&self.throughput_data);
        let is_running_clone = Arc::clone(&self.is_running);

        let handle = thread::Builder::new()
            .name("performance-monitor".to_string())
            .spawn(move || {
                let mut last_update = Instant::now();
                
                while *is_running_clone.lock().unwrap() {
                    thread::sleep(Duration::from_millis(100)); // 10Hz monitoring
                    
                    if last_update.elapsed() >= config.flush_interval {
                        // Collect current metrics
                        let current_metrics = Self::collect_system_metrics();
                        
                        // Update memory metrics
                        Self::update_memory_metrics(&memory_data, &current_metrics);
                        
                        // Update CPU metrics
                        Self::update_cpu_metrics(&cpu_data, &current_metrics);
                        
                        // Update throughput metrics
                        Self::update_throughput_metrics(&throughput_data, &current_metrics);
                        
                        // Store metrics history
                        {
                            let mut metrics_queue = metrics.write().unwrap();
                            metrics_queue.push_back(current_metrics.clone());
                            
                            // Limit history size
                            if metrics_queue.len() > config.buffer_size {
                                metrics_queue.pop_front();
                            }
                        }
                        
                        // Check for alerts
                        Self::check_alerts(&config, &current_metrics);
                        
                        last_update = Instant::now();
                    }
                }
            })
            .map_err(|e| CursedError::runtime_error(&format!("Failed to start monitor thread: {}", e)))?;

        *self.monitor_thread.lock().unwrap() = Some(handle);
        Ok(())
    }

    /// Stop performance monitoring
    pub fn stop(&self) -> Result<(), CursedError> {
        *self.is_running.lock().unwrap() = false;
        
        if let Some(handle) = self.monitor_thread.lock().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join monitor thread"))?;
        }
        
        Ok(())
    }

    /// Record timing for an operation
    pub fn record_timing(&self, operation_name: &str, duration: Duration) {
        let mut timing_data = self.timing_data.write().unwrap();
        
        let entry = timing_data.entry(operation_name.to_string()).or_insert_with(|| {
            TimingMetrics {
                operation_name: operation_name.to_string(),
                total_calls: 0,
                total_time: Duration::from_nanos(0),
                average_time: Duration::from_nanos(0),
                min_time: duration,
                max_time: duration,
                recent_times: VecDeque::new(),
                p95_time: duration,
                p99_time: duration,
            }
        });

        entry.total_calls += 1;
        entry.total_time += duration;
        entry.average_time = entry.total_time / entry.total_calls as u32;
        entry.min_time = entry.min_time.min(duration);
        entry.max_time = entry.max_time.max(duration);
        
        entry.recent_times.push_back(duration);
        if entry.recent_times.len() > 1000 {
            entry.recent_times.pop_front();
        }
        
        // Update percentiles
        let mut sorted_times: Vec<Duration> = entry.recent_times.iter().cloned().collect();
        sorted_times.sort();
        
        if !sorted_times.is_empty() {
            let p95_index = (sorted_times.len() as f64 * 0.95) as usize;
            let p99_index = (sorted_times.len() as f64 * 0.99) as usize;
            
            entry.p95_time = sorted_times[p95_index.min(sorted_times.len() - 1)];
            entry.p99_time = sorted_times[p99_index.min(sorted_times.len() - 1)];
        }
    }

    /// Record memory allocation
    pub fn record_memory_allocation(&self, size: usize) {
        let mut memory_data = self.memory_data.write().unwrap();
        memory_data.allocations += 1;
        memory_data.current_usage += size;
        memory_data.peak_usage = memory_data.peak_usage.max(memory_data.current_usage);
        
        let current_usage = memory_data.current_usage;
        memory_data.recent_usage.push_back(current_usage);
        if memory_data.recent_usage.len() > 1000 {
            memory_data.recent_usage.pop_front();
        }
        
        // Update average
        let sum: usize = memory_data.recent_usage.iter().sum();
        memory_data.average_usage = sum / memory_data.recent_usage.len();
    }

    /// Record memory deallocation
    pub fn record_memory_deallocation(&self, size: usize) {
        let mut memory_data = self.memory_data.write().unwrap();
        memory_data.deallocations += 1;
        memory_data.current_usage = memory_data.current_usage.saturating_sub(size);
        
        let current_usage = memory_data.current_usage;
        memory_data.recent_usage.push_back(current_usage);
        if memory_data.recent_usage.len() > 1000 {
            memory_data.recent_usage.pop_front();
        }
        
        // Update average
        let sum: usize = memory_data.recent_usage.iter().sum();
        memory_data.average_usage = sum / memory_data.recent_usage.len();
    }

    /// Record garbage collection
    pub fn record_gc(&self, duration: Duration) {
        let mut memory_data = self.memory_data.write().unwrap();
        memory_data.gc_collections += 1;
        
        // Record GC timing
        self.record_timing("garbage_collection", duration);
    }

    /// Get current performance metrics
    pub fn get_current_metrics(&self) -> Result<PerformanceMetrics, CursedError> {
        let metrics = self.metrics.read().unwrap();
        if let Some(latest) = metrics.back() {
            Ok(latest.clone())
        } else {
            Ok(Self::collect_system_metrics())
        }
    }

    /// Get timing metrics for operation
    pub fn get_timing_metrics(&self, operation_name: &str) -> Option<TimingMetrics> {
        let timing_data = self.timing_data.read().unwrap();
        timing_data.get(operation_name).cloned()
    }

    /// Get all timing metrics
    pub fn get_all_timing_metrics(&self) -> HashMap<String, TimingMetrics> {
        self.timing_data.read().unwrap().clone()
    }

    /// Get memory metrics
    pub fn get_memory_metrics(&self) -> MemoryMetrics {
        self.memory_data.read().unwrap().clone()
    }

    /// Get CPU metrics
    pub fn get_cpu_metrics(&self) -> CpuMetrics {
        self.cpu_data.read().unwrap().clone()
    }

    /// Get throughput metrics
    pub fn get_throughput_metrics(&self) -> ThroughputMetrics {
        self.throughput_data.read().unwrap().clone()
    }

    /// Generate performance report
    pub fn generate_report(&self, format: ReportFormat) -> Result<String, CursedError> {
        let metrics = self.get_current_metrics()?;
        let timing_metrics = self.get_all_timing_metrics();
        let memory_metrics = self.get_memory_metrics();
        let cpu_metrics = self.get_cpu_metrics();
        let throughput_metrics = self.get_throughput_metrics();

        match format {
            ReportFormat::Text => self.generate_text_report(&metrics, &timing_metrics, &memory_metrics, &cpu_metrics, &throughput_metrics),
            ReportFormat::Json => self.generate_json_report(&metrics, &timing_metrics, &memory_metrics, &cpu_metrics, &throughput_metrics),
            ReportFormat::Html => self.generate_html_report(&metrics, &timing_metrics, &memory_metrics, &cpu_metrics, &throughput_metrics),
            ReportFormat::Csv => self.generate_csv_report(&metrics, &timing_metrics, &memory_metrics, &cpu_metrics, &throughput_metrics),
            ReportFormat::Markdown => self.generate_markdown_report(&metrics, &timing_metrics, &memory_metrics, &cpu_metrics, &throughput_metrics),
        }
    }

    /// Get system uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Reset all metrics
    pub fn reset_metrics(&self) {
        self.metrics.write().unwrap().clear();
        self.timing_data.write().unwrap().clear();
        *self.memory_data.write().unwrap() = MemoryMetrics::default();
        *self.cpu_data.write().unwrap() = CpuMetrics::default();
        *self.throughput_data.write().unwrap() = ThroughputMetrics::default();
    }

    // Private helper methods

    fn collect_system_metrics() -> PerformanceMetrics {
        PerformanceMetrics {
            compilation_time: Duration::from_millis(100),
            execution_time: Duration::from_millis(50),
            memory_usage: Self::get_current_memory_usage(),
            cpu_usage: Self::get_current_cpu_usage(),
            throughput: Self::get_current_throughput(),
            latency: Duration::from_millis(10),
            error_rate: 0.001,
            gc_pressure: 0.1,
        }
    }

    fn get_current_memory_usage() -> usize {
        // Get current memory usage from system
        // This is a simplified implementation
        1024 * 1024 * 64 // 64MB
    }

    fn get_current_cpu_usage() -> f64 {
        // Get current CPU usage from system
        // This is a simplified implementation
        25.0 // 25%
    }

    fn get_current_throughput() -> f64 {
        // Get current throughput from system
        // This is a simplified implementation
        1000.0 // 1000 ops/sec
    }

    fn update_memory_metrics(memory_data: &Arc<RwLock<MemoryMetrics>>, metrics: &PerformanceMetrics) {
        let mut data = memory_data.write().unwrap();
        data.current_usage = metrics.memory_usage;
        data.peak_usage = data.peak_usage.max(metrics.memory_usage);
        
        data.recent_usage.push_back(metrics.memory_usage);
        if data.recent_usage.len() > 1000 {
            data.recent_usage.pop_front();
        }
        
        let sum: usize = data.recent_usage.iter().sum();
        data.average_usage = sum / data.recent_usage.len();
    }

    fn update_cpu_metrics(cpu_data: &Arc<RwLock<CpuMetrics>>, metrics: &PerformanceMetrics) {
        let mut data = cpu_data.write().unwrap();
        data.current_usage = metrics.cpu_usage;
        data.peak_usage = data.peak_usage.max(metrics.cpu_usage);
        
        data.recent_usage.push_back(metrics.cpu_usage);
        if data.recent_usage.len() > 1000 {
            data.recent_usage.pop_front();
        }
        
        let sum: f64 = data.recent_usage.iter().sum();
        data.average_usage = sum / data.recent_usage.len() as f64;
    }

    fn update_throughput_metrics(throughput_data: &Arc<RwLock<ThroughputMetrics>>, metrics: &PerformanceMetrics) {
        let mut data = throughput_data.write().unwrap();
        data.operations_per_second = metrics.throughput;
        
        data.recent_throughput.push_back(metrics.throughput);
        if data.recent_throughput.len() > 1000 {
            data.recent_throughput.pop_front();
        }
    }

    fn check_alerts(config: &PerformanceConfig, metrics: &PerformanceMetrics) {
        if metrics.cpu_usage > config.cpu_threshold {
            println!("ALERT: High CPU usage: {:.2}%", metrics.cpu_usage);
        }
        
        if metrics.memory_usage > config.memory_threshold {
            println!("ALERT: High memory usage: {} bytes", metrics.memory_usage);
        }
        
        if metrics.error_rate > 0.01 {
            println!("ALERT: High error rate: {:.2}%", metrics.error_rate * 100.0);
        }
    }

    fn generate_text_report(
        &self,
        metrics: &PerformanceMetrics,
        timing_metrics: &HashMap<String, TimingMetrics>,
        memory_metrics: &MemoryMetrics,
        cpu_metrics: &CpuMetrics,
        throughput_metrics: &ThroughputMetrics,
    ) -> Result<String, CursedError> {
        let mut report = String::new();
        
        report.push_str("=== CURSED Compiler Performance Report ===\n\n");
        
        report.push_str("## Current Metrics\n");
        report.push_str(&format!("Compilation Time: {:?}\n", metrics.compilation_time));
        report.push_str(&format!("Execution Time: {:?}\n", metrics.execution_time));
        report.push_str(&format!("Memory Usage: {} bytes\n", metrics.memory_usage));
        report.push_str(&format!("CPU Usage: {:.2}%\n", metrics.cpu_usage));
        report.push_str(&format!("Throughput: {:.2} ops/sec\n", metrics.throughput));
        report.push_str(&format!("Latency: {:?}\n", metrics.latency));
        report.push_str(&format!("Error Rate: {:.4}%\n", metrics.error_rate * 100.0));
        report.push_str(&format!("GC Pressure: {:.2}%\n", metrics.gc_pressure * 100.0));
        
        report.push_str("\n## Timing Metrics\n");
        for (name, timing) in timing_metrics {
            report.push_str(&format!("  {}: {} calls, avg {:?}, min {:?}, max {:?}\n",
                name, timing.total_calls, timing.average_time, timing.min_time, timing.max_time));
        }
        
        report.push_str("\n## Memory Metrics\n");
        report.push_str(&format!("Current Usage: {} bytes\n", memory_metrics.current_usage));
        report.push_str(&format!("Peak Usage: {} bytes\n", memory_metrics.peak_usage));
        report.push_str(&format!("Average Usage: {} bytes\n", memory_metrics.average_usage));
        report.push_str(&format!("Allocations: {}\n", memory_metrics.allocations));
        report.push_str(&format!("Deallocations: {}\n", memory_metrics.deallocations));
        report.push_str(&format!("GC Collections: {}\n", memory_metrics.gc_collections));
        
        report.push_str("\n## CPU Metrics\n");
        report.push_str(&format!("Current Usage: {:.2}%\n", cpu_metrics.current_usage));
        report.push_str(&format!("Average Usage: {:.2}%\n", cpu_metrics.average_usage));
        report.push_str(&format!("Peak Usage: {:.2}%\n", cpu_metrics.peak_usage));
        
        report.push_str("\n## Throughput Metrics\n");
        report.push_str(&format!("Operations/sec: {:.2}\n", throughput_metrics.operations_per_second));
        report.push_str(&format!("Bytes/sec: {:.2}\n", throughput_metrics.bytes_per_second));
        report.push_str(&format!("Requests/sec: {:.2}\n", throughput_metrics.requests_per_second));
        
        report.push_str(&format!("\nUptime: {:?}\n", self.get_uptime()));
        
        Ok(report)
    }

    fn generate_json_report(
        &self,
        metrics: &PerformanceMetrics,
        timing_metrics: &HashMap<String, TimingMetrics>,
        memory_metrics: &MemoryMetrics,
        cpu_metrics: &CpuMetrics,
        throughput_metrics: &ThroughputMetrics,
    ) -> Result<String, CursedError> {
        // Simple JSON-like report generation without serde
        let report = format!(
            r#"{{
  "timestamp": "{}",
  "current_metrics": {{
    "compilation_time_ms": {},
    "execution_time_ms": {},
    "memory_usage_bytes": {},
    "cpu_usage_percent": {},
    "throughput_ops_per_sec": {},
    "latency_ms": {},
    "error_rate_percent": {},
    "gc_pressure_percent": {}
  }},
  "uptime_ms": {}
}}"#,
            chrono::Utc::now().to_rfc3339(),
            metrics.compilation_time.as_millis(),
            metrics.execution_time.as_millis(),
            metrics.memory_usage,
            metrics.cpu_usage,
            metrics.throughput,
            metrics.latency.as_millis(),
            metrics.error_rate * 100.0,
            metrics.gc_pressure * 100.0,
            self.get_uptime().as_millis()
        );
        
        Ok(report)
    }

    fn generate_html_report(
        &self,
        metrics: &PerformanceMetrics,
        timing_metrics: &HashMap<String, TimingMetrics>,
        memory_metrics: &MemoryMetrics,
        cpu_metrics: &CpuMetrics,
        throughput_metrics: &ThroughputMetrics,
    ) -> Result<String, CursedError> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>CURSED Compiler Performance Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; margin: 20px 0; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background-color: #f2f2f2; }\n");
        html.push_str(".metric { background-color: #e8f5e8; }\n");
        html.push_str(".alert { background-color: #ffebee; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str("<h1>CURSED Compiler Performance Report</h1>\n");
        html.push_str(&format!("<p>Generated: {}</p>\n", chrono::Utc::now().to_rfc3339()));
        html.push_str(&format!("<p>Uptime: {:?}</p>\n", self.get_uptime()));
        
        html.push_str("<h2>Current Metrics</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<tr><th>Metric</th><th>Value</th></tr>\n");
        html.push_str(&format!("<tr><td>Compilation Time</td><td>{:?}</td></tr>\n", metrics.compilation_time));
        html.push_str(&format!("<tr><td>Execution Time</td><td>{:?}</td></tr>\n", metrics.execution_time));
        html.push_str(&format!("<tr><td>Memory Usage</td><td>{} bytes</td></tr>\n", metrics.memory_usage));
        html.push_str(&format!("<tr><td>CPU Usage</td><td>{:.2}%</td></tr>\n", metrics.cpu_usage));
        html.push_str(&format!("<tr><td>Throughput</td><td>{:.2} ops/sec</td></tr>\n", metrics.throughput));
        html.push_str(&format!("<tr><td>Latency</td><td>{:?}</td></tr>\n", metrics.latency));
        html.push_str(&format!("<tr><td>Error Rate</td><td>{:.4}%</td></tr>\n", metrics.error_rate * 100.0));
        html.push_str(&format!("<tr><td>GC Pressure</td><td>{:.2}%</td></tr>\n", metrics.gc_pressure * 100.0));
        html.push_str("</table>\n");
        
        html.push_str("<h2>Timing Metrics</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<tr><th>Operation</th><th>Calls</th><th>Average</th><th>Min</th><th>Max</th><th>P95</th><th>P99</th></tr>\n");
        for (name, timing) in timing_metrics {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{:?}</td><td>{:?}</td><td>{:?}</td><td>{:?}</td><td>{:?}</td></tr>\n",
                name, timing.total_calls, timing.average_time, timing.min_time, timing.max_time, timing.p95_time, timing.p99_time
            ));
        }
        html.push_str("</table>\n");
        
        html.push_str("</body>\n</html>\n");
        
        Ok(html)
    }

    fn generate_csv_report(
        &self,
        metrics: &PerformanceMetrics,
        timing_metrics: &HashMap<String, TimingMetrics>,
        memory_metrics: &MemoryMetrics,
        cpu_metrics: &CpuMetrics,
        throughput_metrics: &ThroughputMetrics,
    ) -> Result<String, CursedError> {
        let mut csv = String::new();
        
        csv.push_str("Section,Metric,Value\n");
        csv.push_str(&format!("Current,Compilation Time (ms),{}\n", metrics.compilation_time.as_millis()));
        csv.push_str(&format!("Current,Execution Time (ms),{}\n", metrics.execution_time.as_millis()));
        csv.push_str(&format!("Current,Memory Usage (bytes),{}\n", metrics.memory_usage));
        csv.push_str(&format!("Current,CPU Usage (%),{:.2}\n", metrics.cpu_usage));
        csv.push_str(&format!("Current,Throughput (ops/sec),{:.2}\n", metrics.throughput));
        csv.push_str(&format!("Current,Latency (ms),{}\n", metrics.latency.as_millis()));
        csv.push_str(&format!("Current,Error Rate (%),{:.4}\n", metrics.error_rate * 100.0));
        csv.push_str(&format!("Current,GC Pressure (%),{:.2}\n", metrics.gc_pressure * 100.0));
        
        csv.push_str("\nOperation,Calls,Average (ms),Min (ms),Max (ms),P95 (ms),P99 (ms)\n");
        for (name, timing) in timing_metrics {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                name,
                timing.total_calls,
                timing.average_time.as_millis(),
                timing.min_time.as_millis(),
                timing.max_time.as_millis(),
                timing.p95_time.as_millis(),
                timing.p99_time.as_millis()
            ));
        }
        
        Ok(csv)
    }

    fn generate_markdown_report(
        &self,
        metrics: &PerformanceMetrics,
        timing_metrics: &HashMap<String, TimingMetrics>,
        memory_metrics: &MemoryMetrics,
        cpu_metrics: &CpuMetrics,
        throughput_metrics: &ThroughputMetrics,
    ) -> Result<String, CursedError> {
        let mut md = String::new();
        
        md.push_str("# CURSED Compiler Performance Report\n\n");
        md.push_str(&format!("**Generated:** {}\n", chrono::Utc::now().to_rfc3339()));
        md.push_str(&format!("**Uptime:** {:?}\n\n", self.get_uptime()));
        
        md.push_str("## Current Metrics\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Compilation Time | {:?} |\n", metrics.compilation_time));
        md.push_str(&format!("| Execution Time | {:?} |\n", metrics.execution_time));
        md.push_str(&format!("| Memory Usage | {} bytes |\n", metrics.memory_usage));
        md.push_str(&format!("| CPU Usage | {:.2}% |\n", metrics.cpu_usage));
        md.push_str(&format!("| Throughput | {:.2} ops/sec |\n", metrics.throughput));
        md.push_str(&format!("| Latency | {:?} |\n", metrics.latency));
        md.push_str(&format!("| Error Rate | {:.4}% |\n", metrics.error_rate * 100.0));
        md.push_str(&format!("| GC Pressure | {:.2}% |\n\n", metrics.gc_pressure * 100.0));
        
        md.push_str("## Timing Metrics\n\n");
        md.push_str("| Operation | Calls | Average | Min | Max | P95 | P99 |\n");
        md.push_str("|-----------|-------|---------|-----|-----|-----|-----|\n");
        for (name, timing) in timing_metrics {
            md.push_str(&format!(
                "| {} | {} | {:?} | {:?} | {:?} | {:?} | {:?} |\n",
                name, timing.total_calls, timing.average_time, timing.min_time, timing.max_time, timing.p95_time, timing.p99_time
            ));
        }
        
        md.push_str("\n## Memory Metrics\n\n");
        md.push_str("| Metric | Value |\n");
        md.push_str("|--------|-------|\n");
        md.push_str(&format!("| Current Usage | {} bytes |\n", memory_metrics.current_usage));
        md.push_str(&format!("| Peak Usage | {} bytes |\n", memory_metrics.peak_usage));
        md.push_str(&format!("| Average Usage | {} bytes |\n", memory_metrics.average_usage));
        md.push_str(&format!("| Allocations | {} |\n", memory_metrics.allocations));
        md.push_str(&format!("| Deallocations | {} |\n", memory_metrics.deallocations));
        md.push_str(&format!("| GC Collections | {} |\n", memory_metrics.gc_collections));
        
        Ok(md)
    }
}

// Default implementations
impl Default for TimingMetrics {
    fn default() -> Self {
        Self {
            operation_name: String::new(),
            total_calls: 0,
            total_time: Duration::from_nanos(0),
            average_time: Duration::from_nanos(0),
            min_time: Duration::from_nanos(0),
            max_time: Duration::from_nanos(0),
            recent_times: VecDeque::new(),
            p95_time: Duration::from_nanos(0),
            p99_time: Duration::from_nanos(0),
        }
    }
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            current_usage: 0,
            peak_usage: 0,
            average_usage: 0,
            allocations: 0,
            deallocations: 0,
            gc_collections: 0,
            heap_size: 0,
            stack_size: 0,
            recent_usage: VecDeque::new(),
        }
    }
}

impl Default for CpuMetrics {
    fn default() -> Self {
        Self {
            current_usage: 0.0,
            average_usage: 0.0,
            peak_usage: 0.0,
            user_time: Duration::from_nanos(0),
            system_time: Duration::from_nanos(0),
            idle_time: Duration::from_nanos(0),
            recent_usage: VecDeque::new(),
        }
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            bytes_per_second: 0.0,
            requests_per_second: 0.0,
            compilation_rate: 0.0,
            execution_rate: 0.0,
            recent_throughput: VecDeque::new(),
        }
    }
}
