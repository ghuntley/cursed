// Optimization modules for CURSED
pub mod config;
pub mod optimization_manager;
pub mod real_llvm_passes;
pub mod simple_passes;
pub mod enhanced_llvm_passes_manager;
pub mod coordinator;
pub mod passes;
pub mod performance_monitor;
pub mod performance_system;
pub mod analysis;
pub mod cache;
pub mod types;
pub mod lto;
pub mod llvm_passes;
pub mod optimization_levels;
pub mod enhanced_llvm_optimization;
pub mod performance_analysis;
pub mod metrics;
pub mod compilation_speed;
pub mod pgo;
pub mod benchmarking;
pub mod incremental;
pub mod benchmarks;
pub mod parallel;
pub mod profiler;
pub mod comprehensive_performance_system;
pub mod distributed;

// Re-export key types
pub use config::{OptimizationConfig, OptimizationLevel};
pub use real_llvm_passes::RealLlvmPassManager;
pub use enhanced_llvm_passes_manager::EnhancedLlvmPassManager;
pub use coordinator::{OptimizationCoordinator, CoordinatorConfiguration as CoordinatorConfig};
pub use performance_monitor::PerformanceMonitor;
pub use types::{OptimizationStats, OptimizationResult};
pub use optimization_manager::{
    PerformanceMetrics, AdvancedOptimizationManager, OptimizationManager
};

// Stub types that examples are trying to import
#[derive(Debug, Clone)]
pub struct BaselineComparator {
    config: BaselineComparisonConfig,
}

#[derive(Debug, Clone)]
pub struct BaselineComparisonConfig {
    pub tolerance: f64,
    pub min_samples: usize,
    pub confidence_level: f64,
}

#[derive(Debug, Clone)]
pub struct BaselineMetadata {
    pub version: String,
    pub timestamp: std::time::SystemTime,
    pub environment: EnvironmentInfo,
}

#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub memory_gb: f64,
}

#[derive(Debug, Clone)]
pub struct TimeSavingsConfig {
    pub baseline_time: std::time::Duration,
    pub optimization_time: std::time::Duration,
    pub threshold: f64,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub timeout: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub duration: std::time::Duration,
    pub throughput: f64,
    pub memory_usage: usize,
    pub cpu_usage: f64,
}

#[derive(Debug, Clone)]
pub enum RegressionSeverity {
    Critical,
    Major,
    Minor,
    Warning,
}

impl BaselineComparator {
    pub fn new(config: BaselineComparisonConfig) -> Self {
        Self { config }
    }
}

impl Default for BaselineComparisonConfig {
    fn default() -> Self {
        Self {
            tolerance: 0.05,
            min_samples: 10,
            confidence_level: 0.95,
        }
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            warmup_iterations: 10,
            timeout: std::time::Duration::from_secs(60),
        }
    }
}
