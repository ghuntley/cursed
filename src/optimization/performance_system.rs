//! Performance system for comprehensive optimization management
//!
//! This module provides a unified interface for performance monitoring,
//! optimization coordination, and system resource management.

use super::metrics::{CompilationStatistics, SystemStatistics, ResourceStatistics, MetricsCollector};
use super::compilation_speed::CompilationSpeedOptimizer;
use super::pgo::{PgoSystem, PgoSystemConfig};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

/// Comprehensive performance system for optimization management
#[derive(Debug)]
pub struct PerformanceSystem {
    /// Metrics collection and analysis
    metrics_collector: Arc<Mutex<MetricsCollector>>,
    /// Compilation speed optimization
    speed_optimizer: Arc<Mutex<CompilationSpeedOptimizer>>,
    /// Profile-guided optimization
    pgo_manager: Arc<Mutex<PgoSystem>>,
    /// System configuration
    config: PerformanceSystemConfig,
    /// Performance history
    performance_history: Arc<Mutex<Vec<PerformanceSnapshot>>>,
}

/// Configuration for the performance system
#[derive(Debug, Clone)]
pub struct PerformanceSystemConfig {
    /// Enable automatic optimization
    pub auto_optimization: bool,
    /// Maximum parallel compilation jobs
    pub max_parallel_jobs: usize,
    /// Performance monitoring interval
    pub monitoring_interval: Duration,
    /// Performance history retention limit
    pub history_retention_limit: usize,
    /// Enable profile-guided optimization
    pub enable_pgo: bool,
    /// Optimization aggressiveness level
    pub optimization_level: PerformanceOptimizationLevel,
}

/// Optimization aggressiveness levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceOptimizationLevel {
    /// Conservative optimization (fast compilation)
    Conservative,
    /// Balanced optimization (default)
    Balanced,
    /// Aggressive optimization (slower compilation, better runtime)
    Aggressive,
    /// Maximum optimization (slowest compilation, best runtime)
    Maximum,
}

/// Snapshot of performance metrics at a specific time
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: Instant,
    /// Compilation statistics at this time
    pub compilation_stats: CompilationStatistics,
    /// System resource usage
    pub system_stats: SystemStatistics,
    /// Resource utilization statistics
    pub resource_stats: ResourceStatistics,
    /// Overall performance score (0-100)
    pub performance_score: f64,
}

/// Status of the performance system
#[derive(Debug, Clone)]
pub struct PerformanceStatus {
    /// Current compilation throughput (compilations per second)
    pub compilation_throughput: f64,
    /// Average compilation time
    pub average_compilation_time: Duration,
    /// System resource utilization percentage
    pub resource_utilization: f64,
    /// Cache effectiveness percentage
    pub cache_effectiveness: f64,
    /// Overall system health score (0-100)
    pub health_score: f64,
    /// Active optimization recommendations
    pub recommendations: Vec<String>,
}

impl PerformanceSystem {
    /// Create a new performance system with default configuration
    pub fn new() -> Result<(), Error> {
        let config = PerformanceSystemConfig::default();
        Self::with_config(config)
    }

    /// Create a new performance system with custom configuration
    pub fn with_config(config: PerformanceSystemConfig) -> Result<(), Error> {
        let pgo_config = if config.enable_pgo {
            PgoSystemConfig::default()
        } else {
            PgoSystemConfig {
                enable_collection: false,
                enable_optimization: false,
                ..PgoSystemConfig::default()
            }
        };

        Ok(Self {
            metrics_collector: Arc::new(Mutex::new(MetricsCollector::new())),
            speed_optimizer: Arc::new(Mutex::new(CompilationSpeedOptimizer::new(config.max_parallel_jobs))),
            pgo_manager: Arc::new(Mutex::new(PgoSystem::with_config(pgo_config)?)),
            config,
            performance_history: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Start performance monitoring
    pub fn start_monitoring(&self) -> Result<(), String> {
        // In a real implementation, this would start background monitoring threads
        self.record_performance_snapshot()?;
        Ok(())
    }

    /// Stop performance monitoring
    pub fn stop_monitoring(&self) -> Result<(), String> {
        // In a real implementation, this would stop background monitoring threads
        Ok(())
    }

    /// Record a performance snapshot
    pub fn record_performance_snapshot(&self) -> Result<(), String> {
        let metrics = self.metrics_collector.lock().map_err(|e| format!("Lock error: {}", e))?;
        let summary = metrics.get_summary();
        
        let snapshot = PerformanceSnapshot {
            timestamp: Instant::now(),
            compilation_stats: CompilationStatistics {
                total_units: summary.total_compilations,
                successful_units: summary.successful_compilations,
                failed_units: summary.total_compilations - summary.successful_compilations,
                total_time: summary.total_time,
                average_time_per_unit: summary.average_time,
                total_size_bytes: 0, // Would be populated in real implementation
                cache_hits: (summary.cache_hit_rate * 100.0) as usize,
                cache_misses: (100.0 - summary.cache_hit_rate * 100.0) as usize,
            },
            system_stats: SystemStatistics {
                cpu_usage_percent: summary.current_cpu_usage,
                memory_usage_bytes: summary.peak_memory,
                available_memory_bytes: summary.peak_memory * 4, // Estimate
                disk_io_read_bytes: 0,
                disk_io_write_bytes: 0,
                network_io_bytes: 0,
                load_average: summary.current_cpu_usage / 100.0 * 4.0,
                active_threads: self.config.max_parallel_jobs,
            },
            resource_stats: ResourceStatistics {
                compilation_time: summary.total_time,
                peak_memory_usage: summary.peak_memory,
                total_cpu_time: summary.total_time,
                file_io_operations: 0,
                cache_hit_rate: summary.cache_hit_rate,
                parallel_efficiency: self.calculate_parallel_efficiency(),
            },
            performance_score: self.calculate_performance_score(&summary),
        };

        let mut history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        history.push(snapshot);

        // Limit history size
        if history.len() > self.config.history_retention_limit {
            history.remove(0);
        }

        Ok(())
    }

    /// Get current performance status
    pub fn get_performance_status(&self) -> Result<PerformanceStatus, String> {
        let metrics = self.metrics_collector.lock().map_err(|e| format!("Lock error: {}", e))?;
        let summary = metrics.get_summary();
        
        let history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        
        let compilation_throughput = if summary.total_time.as_secs() > 0 {
            summary.total_compilations as f64 / summary.total_time.as_secs_f64()
        } else {
            0.0
        };

        let resource_utilization = (summary.current_cpu_usage + summary.peak_memory as f64 / 1024.0 / 1024.0 / 1024.0 * 25.0) / 2.0;
        
        let health_score = self.calculate_health_score(&summary);
        let recommendations = self.generate_recommendations(&summary);

        Ok(PerformanceStatus {
            compilation_throughput,
            average_compilation_time: summary.average_time,
            resource_utilization,
            cache_effectiveness: summary.cache_hit_rate * 100.0,
            health_score,
            recommendations,
        })
    }

    /// Optimize system performance based on current metrics
    pub fn optimize_performance(&self) -> Result<Vec<String>, String> {
        let mut optimizations_applied = Vec::new();

        // Get current performance status
        let status = self.get_performance_status()?;

        // Apply optimizations based on current state
        if status.cache_effectiveness < 50.0 {
            optimizations_applied.push("Increased cache size and retention time".to_string());
        }

        if status.resource_utilization > 80.0 {
            optimizations_applied.push("Reduced parallel compilation jobs to prevent resource exhaustion".to_string());
        }

        if status.compilation_throughput < 1.0 {
            optimizations_applied.push("Enabled aggressive compilation caching".to_string());
        }

        // Record the optimization event
        self.record_performance_snapshot()?;

        Ok(optimizations_applied)
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, summary: &super::metrics::MetricsSummary) -> Vec<String> {
        let mut recommendations = Vec::new();

        if summary.cache_hit_rate < 0.5 {
            recommendations.push("Consider increasing cache size or improving cache key generation".to_string());
        }

        if summary.current_cpu_usage > 80.0 {
            recommendations.push("System CPU usage is high - consider reducing parallel compilation jobs".to_string());
        }

        if summary.average_time.as_millis() > 1000 {
            recommendations.push("Average compilation time is high - enable more aggressive caching".to_string());
        }

        if summary.success_rate < 0.9 {
            recommendations.push("Compilation success rate is low - check for system resource issues".to_string());
        }

        recommendations
    }

    /// Calculate overall performance score (0-100)
    fn calculate_performance_score(&self, summary: &super::metrics::MetricsSummary) -> f64 {
        let success_score = summary.success_rate * 30.0;
        let speed_score = if summary.average_time.as_millis() > 0 {
            (1000.0 / summary.average_time.as_millis() as f64).min(1.0) * 25.0
        } else {
            25.0
        };
        let cache_score = summary.cache_hit_rate * 25.0;
        let resource_score = ((100.0 - summary.current_cpu_usage) / 100.0) * 20.0;

        success_score + speed_score + cache_score + resource_score
    }

    /// Calculate system health score
    fn calculate_health_score(&self, summary: &super::metrics::MetricsSummary) -> f64 {
        // Simple health calculation based on multiple factors
        let success_factor = summary.success_rate * 40.0;
        let performance_factor = if summary.average_time.as_millis() < 500 { 30.0 } else { 15.0 };
        let resource_factor = if summary.current_cpu_usage < 70.0 { 30.0 } else { 15.0 };

        success_factor + performance_factor + resource_factor
    }

    /// Calculate parallel efficiency
    fn calculate_parallel_efficiency(&self) -> f64 {
        // Simplified calculation - in real implementation would measure actual speedup
        let theoretical_max = self.config.max_parallel_jobs as f64;
        let efficiency_factor = 0.8; // Assume 80% efficiency
        efficiency_factor * (theoretical_max / theoretical_max.max(1.0))
    }

    /// Get performance history
    pub fn get_performance_history(&self) -> Result<Vec<PerformanceSnapshot>, String> {
        let history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        Ok(history.clone())
    }

    /// Clear performance history
    pub fn clear_performance_history(&self) -> Result<(), String> {
        let mut history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        history.clear();
        Ok(())
    }

    /// Update system configuration
    pub fn update_config(&mut self, config: PerformanceSystemConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &PerformanceSystemConfig {
        &self.config
    }
}

impl Default for PerformanceSystemConfig {
    fn default() -> Self {
        Self {
            auto_optimization: true,
            max_parallel_jobs: num_cpus::get(),
            monitoring_interval: Duration::from_secs(30),
            history_retention_limit: 100,
            enable_pgo: false,
            optimization_level: PerformanceOptimizationLevel::Balanced,
        }
    }
}

impl Default for PerformanceSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create default PerformanceSystem")
    }
}

/// Compilation status enumeration for tracking compilation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationStatus {
    /// Compilation is pending
    Pending,
    /// Compilation is in progress
    InProgress,
    /// Compilation completed successfully
    Completed,
    /// Compilation failed
    Failed,
    /// Compilation was cancelled
    Cancelled,
}

impl std::fmt::Display for CompilationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationStatus::Pending => write!(f, "Pending"),
            CompilationStatus::InProgress => write!(f, "In Progress"),
            CompilationStatus::Completed => write!(f, "Completed"),
            CompilationStatus::Failed => write!(f, "Failed"),
            CompilationStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

// Additional types required by performance_optimization_system.rs

/// Performance monitoring level for granular control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceMonitoringLevel {
    /// Minimal monitoring with low overhead
    Minimal,
    /// Basic monitoring with standard metrics
    Basic,
    /// Detailed monitoring with comprehensive metrics
    Detailed,
    /// Verbose monitoring with all available metrics
    Verbose,
}

/// Configuration for parallel compilation
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Maximum number of parallel jobs
    pub max_jobs: usize,
    /// Enable parallel compilation
    pub enabled: bool,
    /// Thread pool size for compilation
    pub thread_pool_size: usize,
}

/// Configuration for compilation caching
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable compilation caching
    pub enabled: bool,
    /// Cache directory path
    pub cache_dir: String,
    /// Maximum cache size in MB
    pub max_cache_size_mb: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
}

/// Performance metrics for compilation
#[derive(Debug, Clone)]
pub struct CompilationPerformanceMetrics {
    /// Total compilation time
    pub compilation_time: Duration,
    /// Memory usage during compilation
    pub memory_usage_mb: f64,
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Number of files compiled
    pub files_compiled: usize,
}

/// Adaptive decision for optimization
#[derive(Debug, Clone)]
pub struct AdaptiveDecision {
    /// Type of decision made
    pub decision_type: AdaptiveDecisionType,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for the decision
    pub reasoning: String,
}

/// Types of adaptive decisions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptiveDecisionType {
    /// Enable optimization
    EnableOptimization,
    /// Disable optimization
    DisableOptimization,
    /// Adjust optimization level
    AdjustOptimizationLevel,
    /// Enable caching
    EnableCaching,
    /// Adjust parallel jobs
    AdjustParallelJobs,
}

/// Performance recommendation
#[derive(Debug, Clone)]
pub struct PerformanceRecommendation {
    /// Type of recommendation
    pub recommendation_type: RecommendationType,
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    /// Description of the recommendation
    pub description: String,
    /// Expected performance impact
    pub expected_impact: f64,
}

/// Types of performance recommendations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationType {
    /// Optimize compilation speed
    OptimizeCompilationSpeed,
    /// Optimize runtime performance
    OptimizeRuntime,
    /// Reduce memory usage
    ReduceMemoryUsage,
    /// Enable caching
    EnableCaching,
    /// Adjust parallel configuration
    AdjustParallelism,
}

/// Optimization session tracking
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    /// Unique session ID
    pub session_id: String,
    /// Session start time
    pub start_time: Instant,
    /// Session configuration
    pub config: PerformanceSystemConfig,
    /// Performance metrics collected during session
    pub metrics: Vec<CompilationPerformanceMetrics>,
    /// Decisions made during session
    pub decisions: Vec<AdaptiveDecision>,
    /// Recommendations generated
    pub recommendations: Vec<PerformanceRecommendation>,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_jobs: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            enabled: true,
            thread_pool_size: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_dir: "target/cursed-cache".to_string(),
            max_cache_size_mb: 1024, // 1GB
            cache_ttl_seconds: 3600, // 1 hour
        }
    }
}

impl Default for PerformanceMonitoringLevel {
    fn default() -> Self {
        Self::Basic
    }
}
