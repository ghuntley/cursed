/// LLVM Optimization Performance Monitor
/// 
/// Provides comprehensive performance monitoring and analysis for optimization
/// processes, including compilation speed, memory usage, code quality metrics,
/// and regression detection.

use crate::error::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tracing::{debug, info, warn, instrument, span, Level};

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub enable_compilation_timing: bool,
    pub enable_memory_tracking: bool,
    pub enable_code_quality_metrics: bool,
    pub enable_regression_detection: bool,
    pub enable_baseline_comparison: bool,
    pub sample_rate: f64, // 0.0 to 1.0
    pub history_retention_days: u32,
    pub performance_baseline_path: Option<PathBuf>,
    pub report_generation_interval: Duration,
    pub alert_thresholds: AlertThresholds,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_compilation_timing: true,
            enable_memory_tracking: true,
            enable_code_quality_metrics: true,
            enable_regression_detection: true,
            enable_baseline_comparison: false,
            sample_rate: 1.0,
            history_retention_days: 30,
            performance_baseline_path: None,
            report_generation_interval: Duration::from_secs(3600), // 1 hour
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

/// Alert threshold configuration
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub compilation_time_regression_percent: f64,
    pub memory_usage_increase_percent: f64,
    pub code_size_increase_percent: f64,
    pub performance_degradation_percent: f64,
    pub pass_failure_rate_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            compilation_time_regression_percent: 20.0,
            memory_usage_increase_percent: 50.0,
            code_size_increase_percent: 10.0,
            performance_degradation_percent: 5.0,
            pass_failure_rate_percent: 5.0,
        }
    }
}

/// Code quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub instruction_count: usize,
    pub function_count: usize,
    pub basic_block_count: usize,
    pub call_instruction_count: usize,
    pub loop_count: usize,
    pub branch_count: usize,
    pub load_store_count: usize,
    pub complexity_score: f64,
    pub estimated_cache_performance: f64,
    pub vectorization_opportunities: usize,
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            instruction_count: 0,
            function_count: 0,
            basic_block_count: 0,
            call_instruction_count: 0,
            loop_count: 0,
            branch_count: 0,
            load_store_count: 0,
            complexity_score: 0.0,
            estimated_cache_performance: 1.0,
            vectorization_opportunities: 0,
        }
    }
}

/// Performance measurement sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    pub timestamp: u64,
    pub module_name: String,
    pub optimization_level: String,
    pub compilation_time: Duration,
    pub memory_peak_usage: usize,
    pub memory_average_usage: usize,
    pub code_metrics_before: CodeMetrics,
    pub code_metrics_after: CodeMetrics,
    pub passes_executed: usize,
    pub passes_successful: usize,
    pub estimated_runtime_improvement: f64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// Baseline performance metrics for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    pub average_compilation_time: Duration,
    pub average_memory_usage: usize,
    pub average_code_size_reduction: f64,
    pub average_performance_improvement: f64,
    pub typical_pass_success_rate: f64,
    pub code_quality_baseline: CodeMetrics,
    pub last_updated: u64,
    pub sample_count: usize,
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            average_compilation_time: Duration::from_secs(1),
            average_memory_usage: 100 * 1024 * 1024, // 100MB
            average_code_size_reduction: 0.05, // 5%
            average_performance_improvement: 1.2, // 20%
            typical_pass_success_rate: 0.95, // 95%
            code_quality_baseline: CodeMetrics::default(),
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            sample_count: 0,
        }
    }
}

/// Performance analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub generation_time: u64,
    pub analysis_period: Duration,
    pub total_samples: usize,
    pub compilation_performance: CompilationPerformanceAnalysis,
    pub memory_analysis: MemoryUsageAnalysis,
    pub code_quality_analysis: CodeQualityAnalysis,
    pub regression_analysis: RegressionAnalysis,
    pub recommendations: Vec<String>,
    pub alerts: Vec<PerformanceAlert>,
}

/// Compilation performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationPerformanceAnalysis {
    pub average_compilation_time: Duration,
    pub median_compilation_time: Duration,
    pub p95_compilation_time: Duration,
    pub p99_compilation_time: Duration,
    pub fastest_compilation: Duration,
    pub slowest_compilation: Duration,
    pub compilation_time_trend: Vec<(u64, Duration)>,
    pub pass_performance_breakdown: HashMap<String, Duration>,
}

/// Memory usage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageAnalysis {
    pub average_peak_memory: usize,
    pub median_peak_memory: usize,
    pub maximum_peak_memory: usize,
    pub memory_efficiency_score: f64,
    pub memory_usage_trend: Vec<(u64, usize)>,
    pub memory_leak_detection: Vec<String>,
}

/// Code quality analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityAnalysis {
    pub average_code_size_reduction: f64,
    pub average_performance_improvement: f64,
    pub instruction_reduction_rate: f64,
    pub vectorization_success_rate: f64,
    pub optimization_effectiveness: HashMap<String, f64>,
    pub code_complexity_trends: Vec<(u64, f64)>,
}

/// Regression analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    pub compilation_time_regression: Option<f64>,
    pub memory_usage_regression: Option<f64>,
    pub code_quality_regression: Option<f64>,
    pub performance_regression: Option<f64>,
    pub detected_regressions: Vec<String>,
    pub regression_confidence: f64,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub metric_value: f64,
    pub threshold_value: f64,
    pub timestamp: u64,
    pub recommendations: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Performance monitor
pub struct PerformanceMonitor {
    config: MonitoringConfig,
    samples: Arc<Mutex<VecDeque<PerformanceSample>>>,
    baseline_metrics: Arc<RwLock<BaselineMetrics>>,
    current_metrics: Arc<Mutex<CodeMetrics>>,
    memory_tracker: Arc<Mutex<MemoryTracker>>,
    alert_history: Arc<Mutex<Vec<PerformanceAlert>>>,
    last_report_time: Arc<Mutex<Instant>>,
}

/// Memory usage tracker
#[derive(Debug)]
struct MemoryTracker {
    peak_usage: usize,
    current_usage: usize,
    allocations: usize,
    deallocations: usize,
    tracking_start: Instant,
}

impl MemoryTracker {
    fn new() -> Self {
        Self {
            peak_usage: 0,
            current_usage: 0,
            allocations: 0,
            deallocations: 0,
            tracking_start: Instant::now(),
        }
    }
    
    fn record_allocation(&mut self, size: usize) {
        self.current_usage += size;
        self.allocations += 1;
        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
    }
    
    fn record_deallocation(&mut self, size: usize) {
        self.current_usage = self.current_usage.saturating_sub(size);
        self.deallocations += 1;
    }
    
    fn reset(&mut self) {
        self.peak_usage = 0;
        self.current_usage = 0;
        self.allocations = 0;
        self.deallocations = 0;
        self.tracking_start = Instant::now();
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    #[instrument(skip(config))]
    pub fn new(config: MonitoringConfig) -> Self {
        info!("Creating performance monitor with {} day retention", config.history_retention_days);
        
        let monitor = Self {
            config,
            samples: Arc::new(Mutex::new(VecDeque::new())),
            baseline_metrics: Arc::new(RwLock::new(BaselineMetrics::default())),
            current_metrics: Arc::new(Mutex::new(CodeMetrics::default())),
            memory_tracker: Arc::new(Mutex::new(MemoryTracker::new())),
            alert_history: Arc::new(Mutex::new(Vec::new())),
            last_report_time: Arc::new(Mutex::new(Instant::now())),
        };
        
        // Load baseline metrics if available
        if let Some(ref baseline_path) = monitor.config.performance_baseline_path {
            if let Err(e) = monitor.load_baseline_metrics(baseline_path) {
                warn!("Failed to load baseline metrics: {}", e);
            }
        }
        
        monitor
    }
    
    /// Start monitoring a compilation session
    #[instrument(skip(self))]
    pub fn start_compilation_monitoring(&self, module_name: &str) -> CompilationSession {
        debug!("Starting compilation monitoring for module: {}", module_name);
        
        // Reset memory tracker
        if let Ok(mut tracker) = self.memory_tracker.lock() {
            tracker.reset();
        }
        
        CompilationSession {
            module_name: module_name.to_string(),
            start_time: Instant::now(),
            monitor: self,
        }
    }
    
    /// Record a performance sample
    #[instrument(skip(self, sample))]
    pub fn record_sample(&self, sample: PerformanceSample) {
        // Apply sampling rate
        if fastrand::f64() > self.config.sample_rate {
            return;
        }
        
        debug!("Recording performance sample for module: {}", sample.module_name);
        
        if let Ok(mut samples) = self.samples.lock() {
            samples.push_back(sample.clone());
            
            // Maintain history retention
            let retention_cutoff = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .saturating_sub(self.config.history_retention_days as u64 * 24 * 3600);
            
            while let Some(front) = samples.front() {
                if front.timestamp < retention_cutoff {
                    samples.pop_front();
                } else {
                    break;
                }
            }
        }
        
        // Update baseline metrics
        self.update_baseline_metrics(&sample);
        
        // Check for regressions and alerts
        if self.config.enable_regression_detection {
            self.check_for_regressions(&sample);
        }
        
        // Generate periodic reports
        if let Ok(mut last_report) = self.last_report_time.lock() {
            if last_report.elapsed() > self.config.report_generation_interval {
                if let Ok(report) = self.generate_performance_report() {
                    info!("Generated performance report with {} samples", report.total_samples);
                }
                *last_report = Instant::now();
            }
        }
    }
    
    /// Update baseline metrics with new sample
    fn update_baseline_metrics(&self, sample: &PerformanceSample) {
        if let Ok(mut baseline) = self.baseline_metrics.write() {
            let alpha = 0.1; // Exponential moving average factor
            
            // Update compilation time (convert to f64 for calculation)
            let new_time_secs = sample.compilation_time.as_secs_f64();
            let old_time_secs = baseline.average_compilation_time.as_secs_f64();
            let updated_time_secs = old_time_secs + alpha * (new_time_secs - old_time_secs);
            baseline.average_compilation_time = Duration::from_secs_f64(updated_time_secs);
            
            // Update memory usage
            baseline.average_memory_usage = 
                (baseline.average_memory_usage as f64 + alpha * (sample.memory_peak_usage as f64 - baseline.average_memory_usage as f64)) as usize;
            
            // Update performance improvement
            baseline.average_performance_improvement += 
                alpha * (sample.estimated_runtime_improvement - baseline.average_performance_improvement);
            
            // Update pass success rate
            let current_success_rate = sample.passes_successful as f64 / sample.passes_executed as f64;
            baseline.typical_pass_success_rate += 
                alpha * (current_success_rate - baseline.typical_pass_success_rate);
            
            baseline.sample_count += 1;
            baseline.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    }
    
    /// Check for performance regressions
    fn check_for_regressions(&self, sample: &PerformanceSample) {
        if let Ok(baseline) = self.baseline_metrics.read() {
            let mut alerts = Vec::new();
            
            // Check compilation time regression
            let time_increase_percent = (sample.compilation_time.as_secs_f64() - baseline.average_compilation_time.as_secs_f64()) 
                / baseline.average_compilation_time.as_secs_f64() * 100.0;
            
            if time_increase_percent > self.config.alert_thresholds.compilation_time_regression_percent {
                alerts.push(PerformanceAlert {
                    alert_type: "Compilation Time Regression".to_string(),
                    severity: if time_increase_percent > 50.0 { AlertSeverity::Critical } else { AlertSeverity::Warning },
                    message: format!("Compilation time increased by {:.1}%", time_increase_percent),
                    metric_value: time_increase_percent,
                    threshold_value: self.config.alert_thresholds.compilation_time_regression_percent,
                    timestamp: sample.timestamp,
                    recommendations: vec![
                        "Consider reducing optimization level".to_string(),
                        "Check for inefficient passes".to_string(),
                        "Review module complexity".to_string(),
                    ],
                });
            }
            
            // Check memory usage regression
            let memory_increase_percent = (sample.memory_peak_usage as f64 - baseline.average_memory_usage as f64) 
                / baseline.average_memory_usage as f64 * 100.0;
            
            if memory_increase_percent > self.config.alert_thresholds.memory_usage_increase_percent {
                alerts.push(PerformanceAlert {
                    alert_type: "Memory Usage Increase".to_string(),
                    severity: if memory_increase_percent > 100.0 { AlertSeverity::Critical } else { AlertSeverity::Warning },
                    message: format!("Memory usage increased by {:.1}%", memory_increase_percent),
                    metric_value: memory_increase_percent,
                    threshold_value: self.config.alert_thresholds.memory_usage_increase_percent,
                    timestamp: sample.timestamp,
                    recommendations: vec![
                        "Check for memory leaks".to_string(),
                        "Review caching strategies".to_string(),
                        "Consider incremental compilation".to_string(),
                    ],
                });
            }
            
            // Check performance degradation
            let performance_degradation = baseline.average_performance_improvement - sample.estimated_runtime_improvement;
            let degradation_percent = performance_degradation / baseline.average_performance_improvement * 100.0;
            
            if degradation_percent > self.config.alert_thresholds.performance_degradation_percent {
                alerts.push(PerformanceAlert {
                    alert_type: "Performance Degradation".to_string(),
                    severity: if degradation_percent > 15.0 { AlertSeverity::Critical } else { AlertSeverity::Warning },
                    message: format!("Optimization effectiveness decreased by {:.1}%", degradation_percent),
                    metric_value: degradation_percent,
                    threshold_value: self.config.alert_thresholds.performance_degradation_percent,
                    timestamp: sample.timestamp,
                    recommendations: vec![
                        "Review optimization pass selection".to_string(),
                        "Check for conflicting optimizations".to_string(),
                        "Analyze code complexity changes".to_string(),
                    ],
                });
            }
            
            // Record alerts
            if !alerts.is_empty() {
                if let Ok(mut alert_history) = self.alert_history.lock() {
                    for alert in alerts {
                        warn!("Performance alert: {} - {}", alert.alert_type, alert.message);
                        alert_history.push(alert);
                        
                        // Keep only last 1000 alerts
                        if alert_history.len() > 1000 {
                            alert_history.remove(0);
                        }
                    }
                }
            }
        }
    }
    
    /// Generate comprehensive performance report
    #[instrument(skip(self))]
    pub fn generate_performance_report(&self) -> Result<PerformanceReport> {
        let _span = span!(Level::INFO, "generate_performance_report").entered();
        
        let samples = self.samples.lock()
            .map_err(|_| Error::General("Failed to lock samples".to_string()))?;
        
        if samples.is_empty() {
            return Err(Error::General("No performance samples available".to_string()));
        }
        
        let total_samples = samples.len();
        let analysis_period = Duration::from_secs(
            samples.back().unwrap().timestamp - samples.front().unwrap().timestamp
        );
        
        // Generate compilation performance analysis
        let compilation_performance = self.analyze_compilation_performance(&samples);
        
        // Generate memory analysis
        let memory_analysis = self.analyze_memory_usage(&samples);
        
        // Generate code quality analysis
        let code_quality_analysis = self.analyze_code_quality(&samples);
        
        // Generate regression analysis
        let regression_analysis = self.analyze_regressions(&samples);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&compilation_performance, &memory_analysis, &code_quality_analysis);
        
        // Get recent alerts
        let alerts = self.alert_history.lock()
            .map_err(|_| Error::General("Failed to lock alert history".to_string()))?
            .iter()
            .rev()
            .take(50)
            .cloned()
            .collect();
        
        let report = PerformanceReport {
            generation_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            analysis_period,
            total_samples,
            compilation_performance,
            memory_analysis,
            code_quality_analysis,
            regression_analysis,
            recommendations,
            alerts,
        };
        
        info!("Generated performance report covering {} samples over {:?}", total_samples, analysis_period);
        
        Ok(report)
    }
    
    /// Analyze compilation performance trends
    fn analyze_compilation_performance(&self, samples: &VecDeque<PerformanceSample>) -> CompilationPerformanceAnalysis {
        let mut times: Vec<Duration> = samples.iter().map(|s| s.compilation_time).collect();
        times.sort();
        
        let average = times.iter().sum::<Duration>() / times.len() as u32;
        let median = times[times.len() / 2];
        let p95 = times[(times.len() as f64 * 0.95) as usize];
        let p99 = times[(times.len() as f64 * 0.99) as usize];
        let fastest = times[0];
        let slowest = times[times.len() - 1];
        
        let compilation_time_trend: Vec<(u64, Duration)> = samples.iter()
            .map(|s| (s.timestamp, s.compilation_time))
            .collect();
        
        // Analyze pass performance (simplified)
        let mut pass_performance = HashMap::new();
        for sample in samples {
            let avg_pass_time = sample.compilation_time / sample.passes_executed as u32;
            pass_performance.insert("average_pass_time".to_string(), avg_pass_time);
        }
        
        CompilationPerformanceAnalysis {
            average_compilation_time: average,
            median_compilation_time: median,
            p95_compilation_time: p95,
            p99_compilation_time: p99,
            fastest_compilation: fastest,
            slowest_compilation: slowest,
            compilation_time_trend,
            pass_performance_breakdown: pass_performance,
        }
    }
    
    /// Analyze memory usage patterns
    fn analyze_memory_usage(&self, samples: &VecDeque<PerformanceSample>) -> MemoryUsageAnalysis {
        let mut peak_memories: Vec<usize> = samples.iter().map(|s| s.memory_peak_usage).collect();
        peak_memories.sort();
        
        let average_peak = peak_memories.iter().sum::<usize>() / peak_memories.len();
        let median_peak = peak_memories[peak_memories.len() / 2];
        let maximum_peak = peak_memories[peak_memories.len() - 1];
        
        // Calculate memory efficiency (simplified metric)
        let memory_efficiency_score = samples.iter()
            .map(|s| s.code_metrics_after.instruction_count as f64 / s.memory_peak_usage as f64)
            .sum::<f64>() / samples.len() as f64;
        
        let memory_usage_trend: Vec<(u64, usize)> = samples.iter()
            .map(|s| (s.timestamp, s.memory_peak_usage))
            .collect();
        
        // Simple memory leak detection (look for consistently increasing usage)
        let mut memory_leak_detection = Vec::new();
        let window_size = 10;
        if samples.len() >= window_size {
            let recent_avg = samples.iter().rev().take(window_size)
                .map(|s| s.memory_peak_usage).sum::<usize>() / window_size;
            let older_avg = samples.iter().rev().skip(window_size).take(window_size)
                .map(|s| s.memory_peak_usage).sum::<usize>() / window_size;
            
            if recent_avg > older_avg && (recent_avg - older_avg) as f64 / older_avg as f64 > 0.2 {
                memory_leak_detection.push("Potential memory leak detected in recent compilations".to_string());
            }
        }
        
        MemoryUsageAnalysis {
            average_peak_memory: average_peak,
            median_peak_memory: median_peak,
            maximum_peak_memory: maximum_peak,
            memory_efficiency_score,
            memory_usage_trend,
            memory_leak_detection,
        }
    }
    
    /// Analyze code quality metrics
    fn analyze_code_quality(&self, samples: &VecDeque<PerformanceSample>) -> CodeQualityAnalysis {
        let code_size_reductions: Vec<f64> = samples.iter()
            .filter_map(|s| {
                if s.code_metrics_before.instruction_count > 0 {
                    Some((s.code_metrics_before.instruction_count as f64 - s.code_metrics_after.instruction_count as f64) 
                         / s.code_metrics_before.instruction_count as f64)
                } else {
                    None
                }
            })
            .collect();
        
        let average_code_size_reduction = if !code_size_reductions.is_empty() {
            code_size_reductions.iter().sum::<f64>() / code_size_reductions.len() as f64
        } else {
            0.0
        };
        
        let average_performance_improvement = samples.iter()
            .map(|s| s.estimated_runtime_improvement)
            .sum::<f64>() / samples.len() as f64;
        
        let instruction_reductions: Vec<f64> = samples.iter()
            .filter_map(|s| {
                if s.code_metrics_before.instruction_count > 0 {
                    Some((s.code_metrics_before.instruction_count as f64 - s.code_metrics_after.instruction_count as f64)
                         / s.code_metrics_before.instruction_count as f64)
                } else {
                    None
                }
            })
            .collect();
        
        let instruction_reduction_rate = if !instruction_reductions.is_empty() {
            instruction_reductions.iter().sum::<f64>() / instruction_reductions.len() as f64
        } else {
            0.0
        };
        
        // Calculate vectorization success rate (simplified)
        let vectorization_success_rate = samples.iter()
            .filter_map(|s| {
                if s.code_metrics_before.vectorization_opportunities > 0 {
                    Some((s.code_metrics_before.vectorization_opportunities as f64 - s.code_metrics_after.vectorization_opportunities as f64)
                         / s.code_metrics_before.vectorization_opportunities as f64)
                } else {
                    None
                }
            })
            .sum::<f64>() / samples.len() as f64;
        
        let mut optimization_effectiveness = HashMap::new();
        optimization_effectiveness.insert("overall".to_string(), average_performance_improvement);
        optimization_effectiveness.insert("code_size".to_string(), average_code_size_reduction);
        optimization_effectiveness.insert("instruction_reduction".to_string(), instruction_reduction_rate);
        
        let code_complexity_trends: Vec<(u64, f64)> = samples.iter()
            .map(|s| (s.timestamp, s.code_metrics_after.complexity_score))
            .collect();
        
        CodeQualityAnalysis {
            average_code_size_reduction,
            average_performance_improvement,
            instruction_reduction_rate,
            vectorization_success_rate,
            optimization_effectiveness,
            code_complexity_trends,
        }
    }
    
    /// Analyze performance regressions
    fn analyze_regressions(&self, samples: &VecDeque<PerformanceSample>) -> RegressionAnalysis {
        if samples.len() < 10 {
            return RegressionAnalysis {
                compilation_time_regression: None,
                memory_usage_regression: None,
                code_quality_regression: None,
                performance_regression: None,
                detected_regressions: Vec::new(),
                regression_confidence: 0.0,
            };
        }
        
        let mid_point = samples.len() / 2;
        let recent_samples = &samples.as_slices().0[mid_point..];
        let older_samples = &samples.as_slices().0[..mid_point];
        
        // Calculate regression metrics
        let recent_avg_time = recent_samples.iter().map(|s| s.compilation_time.as_secs_f64()).sum::<f64>() / recent_samples.len() as f64;
        let older_avg_time = older_samples.iter().map(|s| s.compilation_time.as_secs_f64()).sum::<f64>() / older_samples.len() as f64;
        let time_regression = (recent_avg_time - older_avg_time) / older_avg_time;
        
        let recent_avg_memory = recent_samples.iter().map(|s| s.memory_peak_usage).sum::<usize>() / recent_samples.len();
        let older_avg_memory = older_samples.iter().map(|s| s.memory_peak_usage).sum::<usize>() / older_samples.len();
        let memory_regression = (recent_avg_memory as f64 - older_avg_memory as f64) / older_avg_memory as f64;
        
        let recent_avg_perf = recent_samples.iter().map(|s| s.estimated_runtime_improvement).sum::<f64>() / recent_samples.len() as f64;
        let older_avg_perf = older_samples.iter().map(|s| s.estimated_runtime_improvement).sum::<f64>() / older_samples.len() as f64;
        let performance_regression = (older_avg_perf - recent_avg_perf) / older_avg_perf;
        
        let mut detected_regressions = Vec::new();
        if time_regression > 0.1 {
            detected_regressions.push(format!("Compilation time increased by {:.1}%", time_regression * 100.0));
        }
        if memory_regression > 0.2 {
            detected_regressions.push(format!("Memory usage increased by {:.1}%", memory_regression * 100.0));
        }
        if performance_regression > 0.05 {
            detected_regressions.push(format!("Performance decreased by {:.1}%", performance_regression * 100.0));
        }
        
        let regression_confidence = if detected_regressions.is_empty() { 0.0 } else { 0.8 }; // Simplified confidence
        
        RegressionAnalysis {
            compilation_time_regression: if time_regression.abs() > 0.05 { Some(time_regression) } else { None },
            memory_usage_regression: if memory_regression.abs() > 0.1 { Some(memory_regression) } else { None },
            code_quality_regression: None, // TODO: Implement
            performance_regression: if performance_regression.abs() > 0.03 { Some(performance_regression) } else { None },
            detected_regressions,
            regression_confidence,
        }
    }
    
    /// Generate optimization recommendations
    fn generate_recommendations(&self, 
                               compilation: &CompilationPerformanceAnalysis,
                               memory: &MemoryUsageAnalysis,
                               code_quality: &CodeQualityAnalysis) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Compilation time recommendations
        if compilation.average_compilation_time > Duration::from_secs(10) {
            recommendations.push("Consider enabling incremental compilation for large modules".to_string());
            recommendations.push("Evaluate reducing optimization level for development builds".to_string());
        }
        
        if compilation.p99_compilation_time > compilation.average_compilation_time * 3 {
            recommendations.push("Investigate outlier compilation times - may indicate problematic modules".to_string());
        }
        
        // Memory recommendations
        if memory.average_peak_memory > 1024 * 1024 * 1024 { // 1GB
            recommendations.push("High memory usage detected - consider enabling optimization caching".to_string());
            recommendations.push("Review parallel compilation settings to reduce memory pressure".to_string());
        }
        
        if memory.memory_efficiency_score < 0.1 {
            recommendations.push("Low memory efficiency - investigate memory allocation patterns".to_string());
        }
        
        // Code quality recommendations
        if code_quality.average_code_size_reduction < 0.02 {
            recommendations.push("Low code size reduction - consider enabling more aggressive optimizations".to_string());
        }
        
        if code_quality.vectorization_success_rate < 0.3 {
            recommendations.push("Low vectorization rate - review loop structures and enable vectorization passes".to_string());
        }
        
        if code_quality.average_performance_improvement < 1.15 {
            recommendations.push("Consider profile-guided optimization for better performance improvements".to_string());
        }
        
        recommendations
    }
    
    /// Load baseline metrics from file
    fn load_baseline_metrics(&self, path: &PathBuf) -> Result<()> {
        info!("Loading baseline metrics from: {:?}", path);
        
        if !path.exists() {
            warn!("Baseline metrics file does not exist: {:?}", path);
            return Ok(());
        }
        
        match std::fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<BaselineMetrics>(&content) {
                    Ok(loaded_baseline) => {
                        if let Ok(mut baseline) = self.baseline_metrics.write() {
                            *baseline = loaded_baseline;
                            info!("Successfully loaded baseline metrics with {} samples", baseline.sample_count);
                        } else {
                            return Err(Error::General("Failed to acquire baseline metrics lock".to_string()));
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse baseline metrics JSON: {}", e);
                        return Err(Error::General(format!("Invalid baseline metrics format: {}", e)));
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read baseline metrics file: {}", e);
                return Err(Error::General(format!("File read error: {}", e)));
            }
        }
        
        Ok(())
    }
    
    /// Save baseline metrics to file
    pub fn save_baseline_metrics(&self, path: &PathBuf) -> Result<()> {
        info!("Saving baseline metrics to: {:?}", path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| Error::General(format!("Failed to create directory: {}", e)))?;
            }
        }
        
        let baseline = self.baseline_metrics.read()
            .map_err(|_| Error::General("Failed to acquire baseline metrics lock".to_string()))?;
        
        let json_content = serde_json::to_string_pretty(&*baseline)
            .map_err(|e| Error::General(format!("Failed to serialize baseline metrics: {}", e)))?;
        
        std::fs::write(path, json_content)
            .map_err(|e| Error::General(format!("Failed to write baseline metrics file: {}", e)))?;
        
        info!("Successfully saved baseline metrics with {} samples", baseline.sample_count);
        Ok(())
    }
    
    /// Export performance report to file
    pub fn export_performance_report(&self, path: &PathBuf) -> Result<()> {
        let report = self.generate_performance_report()?;
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| Error::General(format!("Failed to create directory: {}", e)))?;
            }
        }
        
        let json_content = serde_json::to_string_pretty(&report)
            .map_err(|e| Error::General(format!("Failed to serialize performance report: {}", e)))?;
        
        std::fs::write(path, json_content)
            .map_err(|e| Error::General(format!("Failed to write performance report: {}", e)))?;
        
        info!("Exported performance report to: {:?}", path);
        Ok(())
    }
    
    /// Import performance samples from file
    pub fn import_performance_samples(&self, path: &PathBuf) -> Result<usize> {
        info!("Importing performance samples from: {:?}", path);
        
        if !path.exists() {
            return Err(Error::General(format!("Import file does not exist: {:?}", path)));
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::General(format!("Failed to read import file: {}", e)))?;
        
        let imported_samples: Vec<PerformanceSample> = serde_json::from_str(&content)
            .map_err(|e| Error::General(format!("Failed to parse performance samples: {}", e)))?;
        
        let import_count = imported_samples.len();
        
        if let Ok(mut samples) = self.samples.lock() {
            for sample in imported_samples {
                samples.push_back(sample);
            }
            
            // Maintain retention policy
            let retention_cutoff = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .saturating_sub(self.config.history_retention_days as u64 * 24 * 3600);
            
            while let Some(front) = samples.front() {
                if front.timestamp < retention_cutoff {
                    samples.pop_front();
                } else {
                    break;
                }
            }
        }
        
        info!("Successfully imported {} performance samples", import_count);
        Ok(import_count)
    }
    
    /// Get current performance statistics
    pub fn get_current_statistics(&self) -> Result<(usize, Duration, f64)> {
        let samples = self.samples.lock()
            .map_err(|_| Error::General("Failed to lock samples".to_string()))?;
        
        if samples.is_empty() {
            return Ok((0, Duration::from_secs(0), 0.0));
        }
        
        let sample_count = samples.len();
        let total_time: Duration = samples.iter().map(|s| s.compilation_time).sum();
        let avg_improvement = samples.iter().map(|s| s.estimated_runtime_improvement).sum::<f64>() / sample_count as f64;
        
        Ok((sample_count, total_time, avg_improvement))
    }
    
    /// Clear all monitoring data
    pub fn clear_data(&self) {
        if let Ok(mut samples) = self.samples.lock() {
            samples.clear();
        }
        if let Ok(mut alerts) = self.alert_history.lock() {
            alerts.clear();
        }
        info!("Performance monitoring data cleared");
    }
    
    /// Print monitoring summary
    #[instrument(skip(self))]
    pub fn print_summary(&self) {
        if let Ok(samples) = self.samples.lock() {
            if samples.is_empty() {
                println!("📊 Performance Monitor: No data available");
                return;
            }
            
            let sample_count = samples.len();
            let avg_time = samples.iter().map(|s| s.compilation_time).sum::<Duration>() / sample_count as u32;
            let avg_memory = samples.iter().map(|s| s.memory_peak_usage).sum::<usize>() / sample_count;
            let avg_improvement = samples.iter().map(|s| s.estimated_runtime_improvement).sum::<f64>() / sample_count as f64;
            
            println!("📊 Performance Monitor Summary:");
            println!("   Samples collected: {}", sample_count);
            println!("   Average compilation time: {:?}", avg_time);
            println!("   Average memory usage: {} MB", avg_memory / 1024 / 1024);
            println!("   Average performance improvement: {:.1}%", (avg_improvement - 1.0) * 100.0);
            
            if let Ok(alerts) = self.alert_history.lock() {
                if !alerts.is_empty() {
                    println!("   Recent alerts: {}", alerts.len());
                }
            }
            
            if let Ok(baseline) = self.baseline_metrics.read() {
                println!("   Baseline samples: {}", baseline.sample_count);
            }
        }
    }
}

/// Compilation session tracker
pub struct CompilationSession<'a> {
    module_name: String,
    start_time: Instant,
    monitor: &'a PerformanceMonitor,
}

impl<'a> CompilationSession<'a> {
    /// Record memory allocation
    pub fn record_allocation(&self, size: usize) {
        if let Ok(mut tracker) = self.monitor.memory_tracker.lock() {
            tracker.record_allocation(size);
        }
    }
    
    /// Record memory deallocation
    pub fn record_deallocation(&self, size: usize) {
        if let Ok(mut tracker) = self.monitor.memory_tracker.lock() {
            tracker.record_deallocation(size);
        }
    }
    
    /// Complete the compilation session
    pub fn complete(self, 
                   optimization_level: &str,
                   code_metrics_before: CodeMetrics,
                   code_metrics_after: CodeMetrics,
                   passes_executed: usize,
                   passes_successful: usize,
                   estimated_runtime_improvement: f64,
                   cache_hits: usize,
                   cache_misses: usize) {
        let compilation_time = self.start_time.elapsed();
        
        let (peak_memory, avg_memory) = if let Ok(tracker) = self.monitor.memory_tracker.lock() {
            (tracker.peak_usage, tracker.current_usage)
        } else {
            (0, 0)
        };
        
        let sample = PerformanceSample {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            module_name: self.module_name,
            optimization_level: optimization_level.to_string(),
            compilation_time,
            memory_peak_usage: peak_memory,
            memory_average_usage: avg_memory,
            code_metrics_before,
            code_metrics_after,
            passes_executed,
            passes_successful,
            estimated_runtime_improvement,
            cache_hits,
            cache_misses,
        };
        
        self.monitor.record_sample(sample);
    }
}
