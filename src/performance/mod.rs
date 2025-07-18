//! Comprehensive performance monitoring system for CURSED compiler
//! 
//! This module provides enterprise-grade performance monitoring capabilities including:
//! - Runtime performance tracking
//! - Profiling tools and analysis
//! - Benchmarking framework
//! - Performance regression detection
//! - Real-time performance visualization
//! - Production monitoring hooks

pub mod monitor;
pub mod profiler;
pub mod benchmark;
pub mod regression;
pub mod visualization;
pub mod reporting;
pub mod analysis;
pub mod hooks;

use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::error::CursedError;

pub use monitor::PerformanceMonitor;
pub use profiler::PerformanceProfiler;
pub use benchmark::BenchmarkRunner;
pub use regression::RegressionDetector;
pub use visualization::PerformanceVisualizer;
pub use reporting::PerformanceReporter;
pub use analysis::PerformanceAnalyzer;
pub use hooks::PerformanceHooks;

/// Configuration for the performance monitoring system
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_monitoring: bool,
    pub enable_profiling: bool,
    pub enable_benchmarking: bool,
    pub enable_regression_detection: bool,
    pub enable_visualization: bool,
    pub sampling_rate: f64,
    pub buffer_size: usize,
    pub flush_interval: Duration,
    pub output_dir: String,
    pub report_format: ReportFormat,
    pub performance_threshold: f64,
    pub memory_threshold: usize,
    pub cpu_threshold: f64,
}

/// Performance report formats
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Text,
    Json,
    Html,
    Csv,
    Markdown,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            enable_profiling: true,
            enable_benchmarking: true,
            enable_regression_detection: true,
            enable_visualization: true,
            sampling_rate: 1.0,
            buffer_size: 100000,
            flush_interval: Duration::from_secs(5),
            output_dir: "./performance_reports".to_string(),
            report_format: ReportFormat::Html,
            performance_threshold: 0.05, // 5% regression threshold
            memory_threshold: 1024 * 1024 * 100, // 100MB threshold
            cpu_threshold: 80.0, // 80% CPU threshold
        }
    }
}

/// Main performance monitoring system
pub struct PerformanceSystem {
    config: PerformanceConfig,
    monitor: Arc<PerformanceMonitor>,
    profiler: Arc<PerformanceProfiler>,
    benchmark_runner: Arc<BenchmarkRunner>,
    regression_detector: Arc<RegressionDetector>,
    visualizer: Arc<PerformanceVisualizer>,
    reporter: Arc<PerformanceReporter>,
    analyzer: Arc<PerformanceAnalyzer>,
    hooks: Arc<PerformanceHooks>,
    start_time: Instant,
}

impl PerformanceSystem {
    /// Create a new performance monitoring system
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        let monitor = Arc::new(PerformanceMonitor::new(config.clone())?);
        let profiler = Arc::new(PerformanceProfiler::new(config.clone())?);
        let benchmark_runner = Arc::new(BenchmarkRunner::new(config.clone())?);
        let regression_detector = Arc::new(RegressionDetector::new(config.clone())?);
        let visualizer = Arc::new(PerformanceVisualizer::new(config.clone())?);
        let reporter = Arc::new(PerformanceReporter::new(config.clone())?);
        let analyzer = Arc::new(PerformanceAnalyzer::new(config.clone())?);
        let hooks = Arc::new(PerformanceHooks::new(config.clone())?);

        Ok(Self {
            config,
            monitor,
            profiler,
            benchmark_runner,
            regression_detector,
            visualizer,
            reporter,
            analyzer,
            hooks,
            start_time: Instant::now(),
        })
    }

    /// Start the performance monitoring system
    pub fn start(&self) -> Result<(), CursedError> {
        if self.config.enable_monitoring {
            self.monitor.start()?;
        }
        
        if self.config.enable_profiling {
            self.profiler.start()?;
        }
        
        if self.config.enable_regression_detection {
            self.regression_detector.start()?;
        }
        
        if self.config.enable_visualization {
            self.visualizer.start()?;
        }
        
        self.hooks.start()?;
        
        Ok(())
    }

    /// Stop the performance monitoring system
    pub fn stop(&self) -> Result<(), CursedError> {
        self.hooks.stop()?;
        self.visualizer.stop()?;
        self.regression_detector.stop()?;
        self.profiler.stop()?;
        self.monitor.stop()?;
        
        Ok(())
    }

    /// Run benchmarks
    pub fn run_benchmarks(&self, benchmark_config: &BenchmarkConfig) -> Result<BenchmarkResults, CursedError> {
        self.benchmark_runner.run_benchmarks(benchmark_config)
    }

    /// Generate performance report
    pub fn generate_report(&self) -> Result<String, CursedError> {
        self.reporter.generate_comprehensive_report(&self.analyzer.analyze()?)
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> Result<PerformanceMetrics, CursedError> {
        self.monitor.get_current_metrics()
    }

    /// Detect performance regressions
    pub fn detect_regressions(&self) -> Result<Vec<RegressionAlert>, CursedError> {
        self.regression_detector.detect_regressions()
    }

    /// Visualize performance data
    pub fn create_visualization(&self, data: &PerformanceData) -> Result<String, CursedError> {
        self.visualizer.create_visualization(data)
    }

    /// Get system uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get configuration
    pub fn get_config(&self) -> &PerformanceConfig {
        &self.config
    }
}

/// Performance metrics data structure
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub compilation_time: Duration,
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub throughput: f64,
    pub latency: Duration,
    pub error_rate: f64,
    pub gc_pressure: f64,
}

/// Performance data for visualization
#[derive(Debug, Clone)]
pub struct PerformanceData {
    pub metrics: Vec<PerformanceMetrics>,
    pub timestamps: Vec<Instant>,
    pub labels: Vec<String>,
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub name: String,
    pub iterations: u32,
    pub warmup_iterations: u32,
    pub timeout: Duration,
    pub memory_limit: usize,
    pub cpu_limit: f64,
    pub parallel_executions: u32,
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub name: String,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub std_deviation: Duration,
    pub throughput: f64,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub success_rate: f64,
}

/// Regression alert
#[derive(Debug, Clone)]
pub struct RegressionAlert {
    pub metric: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub regression_percentage: f64,
    pub severity: RegressionSeverity,
    pub recommendation: String,
}

/// Regression severity levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegressionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            name: "default_benchmark".to_string(),
            iterations: 1000,
            warmup_iterations: 100,
            timeout: Duration::from_secs(60),
            memory_limit: 1024 * 1024 * 512, // 512MB
            cpu_limit: 80.0, // 80%
            parallel_executions: 1,
        }
    }
}

/// Global performance system instance
static mut GLOBAL_PERFORMANCE_SYSTEM: Option<Arc<PerformanceSystem>> = None;
static PERFORMANCE_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global performance system
pub fn init_global_performance_system(config: PerformanceConfig) -> Result<(), CursedError> {
    PERFORMANCE_INIT.call_once(|| {
        match PerformanceSystem::new(config) {
            Ok(system) => {
                unsafe {
                    GLOBAL_PERFORMANCE_SYSTEM = Some(Arc::new(system));
                }
            }
            Err(e) => {
                eprintln!("Failed to initialize performance system: {}", e);
            }
        }
    });
    Ok(())
}

/// Get global performance system instance
pub fn get_global_performance_system() -> Option<Arc<PerformanceSystem>> {
    unsafe {
        GLOBAL_PERFORMANCE_SYSTEM.clone()
    }
}

/// Convenience macro for performance monitoring
#[macro_export]
macro_rules! perf_monitor {
    ($name:expr, $body:expr) => {
        {
            use crate::performance::get_global_performance_system;
            use std::time::Instant;
            
            let start = Instant::now();
            let result = $body;
            let duration = start.elapsed();
            
            if let Some(system) = get_global_performance_system() {
                system.monitor.record_timing($name, duration);
            }
            
            result
        }
    };
}

/// Convenience macro for benchmark execution
#[macro_export]
macro_rules! benchmark {
    ($name:expr, $iterations:expr, $body:expr) => {
        {
            use crate::performance::{get_global_performance_system, BenchmarkConfig};
            use std::time::Instant;
            
            let mut config = BenchmarkConfig::default();
            config.name = $name.to_string();
            config.iterations = $iterations;
            
            if let Some(system) = get_global_performance_system() {
                system.run_benchmarks(&config).unwrap_or_else(|e| {
                    eprintln!("Benchmark failed: {}", e);
                    Default::default()
                })
            } else {
                Default::default()
            }
        }
    };
}

impl Default for BenchmarkResults {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            average_time: Duration::from_nanos(0),
            min_time: Duration::from_nanos(0),
            max_time: Duration::from_nanos(0),
            std_deviation: Duration::from_nanos(0),
            throughput: 0.0,
            memory_usage: 0,
            cpu_usage: 0.0,
            success_rate: 0.0,
        }
    }
}
