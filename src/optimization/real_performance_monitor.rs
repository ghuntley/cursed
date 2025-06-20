/// Real-time Performance Monitoring System
/// 
/// Provides comprehensive performance tracking during compilation and optimization
/// with measurable metrics, trend analysis, and adaptive optimization suggestions.

use crate::error::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::{Duration, Instant, SystemTime};
use sysinfo::{System, Process, Cpu};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

/// Real-time performance monitor with measurable metrics
pub struct RealPerformanceMonitor {
    start_time: Instant,
    system_info: Arc<Mutex<System>>,
    metrics: Arc<Mutex<PerformanceMetrics>>,
    counters: PerformanceCounters,
    config: MonitoringConfig,
    trend_analyzer: TrendAnalyzer,
    adaptive_optimizer: AdaptiveOptimizer,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Sampling interval for system metrics
    pub sampling_interval: Duration,
    /// Number of samples to keep for trend analysis
    pub sample_history_size: usize,
    /// CPU usage threshold for optimization warnings
    pub cpu_threshold: f32,
    /// Memory usage threshold (percentage)
    pub memory_threshold: f32,
    /// Enable detailed instruction counting
    pub detailed_instruction_tracking: bool,
    /// Enable cache performance monitoring
    pub cache_monitoring: bool,
    /// Enable adaptive optimization suggestions
    pub adaptive_optimization: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            sampling_interval: Duration::from_millis(100),
            sample_history_size: 1000,
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            detailed_instruction_tracking: true,
            cache_monitoring: true,
            adaptive_optimization: true,
        }
    }
}

/// Comprehensive performance metrics with real measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Compilation phase timings
    pub phase_timings: HashMap<String, Duration>,
    /// Total compilation time
    pub total_compilation_time: Duration,
    /// Memory usage statistics
    pub memory_stats: MemoryStatistics,
    /// CPU usage statistics
    pub cpu_stats: CpuStatistics,
    /// Instruction processing statistics
    pub instruction_stats: InstructionStatistics,
    /// Cache performance metrics
    pub cache_stats: CacheStatistics,
    /// Optimization effectiveness metrics
    pub optimization_effectiveness: OptimizationEffectiveness,
    /// Performance trends
    pub performance_trends: PerformanceTrends,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    pub peak_memory_usage_mb: f64,
    pub current_memory_usage_mb: f64,
    pub memory_efficiency: f64,
    pub allocation_rate: f64,
    pub gc_frequency: f64,
    pub memory_fragmentation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStatistics {
    pub average_cpu_usage: f32,
    pub peak_cpu_usage: f32,
    pub cpu_efficiency: f64,
    pub core_utilization: Vec<f32>,
    pub context_switches: u64,
    pub cpu_cycles_per_instruction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionStatistics {
    pub instructions_processed: u64,
    pub instructions_per_second: f64,
    pub optimization_passes_applied: u64,
    pub instructions_eliminated: u64,
    pub instruction_reduction_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub cache_hit_rate: f64,
    pub cache_miss_rate: f64,
    pub cache_efficiency: f64,
    pub instruction_cache_misses: u64,
    pub data_cache_misses: u64,
    pub tlb_misses: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEffectiveness {
    pub runtime_improvement_factor: f64,
    pub code_size_reduction_factor: f64,
    pub compilation_speedup: f64,
    pub optimization_success_rate: f64,
    pub performance_per_watt: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    pub compilation_time_trend: TrendDirection,
    pub memory_usage_trend: TrendDirection,
    pub optimization_effectiveness_trend: TrendDirection,
    pub cpu_efficiency_trend: TrendDirection,
    pub predicted_performance_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving(f64),
    Stable(f64),
    Degrading(f64),
}

/// Atomic performance counters for thread-safe updates
struct PerformanceCounters {
    instructions_processed: AtomicU64,
    optimization_passes: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    allocations: AtomicUsize,
    context_switches: AtomicU64,
}

impl Default for PerformanceCounters {
    fn default() -> Self {
        Self {
            instructions_processed: AtomicU64::new(0),
            optimization_passes: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            allocations: AtomicUsize::new(0),
            context_switches: AtomicU64::new(0),
        }
    }
}

/// Trend analyzer for performance metrics
pub struct TrendAnalyzer {
    sample_history: VecDeque<PerformanceSample>,
    trend_calculations: TrendCalculations,
}

#[derive(Debug, Clone)]
struct PerformanceSample {
    timestamp: Instant,
    cpu_usage: f32,
    memory_usage: f64,
    instruction_rate: f64,
    optimization_effectiveness: f64,
}

struct TrendCalculations {
    cpu_trend: LinearRegression,
    memory_trend: LinearRegression,
    performance_trend: LinearRegression,
}

/// Simple linear regression for trend analysis
struct LinearRegression {
    samples: Vec<(f64, f64)>,
    slope: f64,
    intercept: f64,
    r_squared: f64,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            samples: Vec::new(),
            slope: 0.0,
            intercept: 0.0,
            r_squared: 0.0,
        }
    }

    fn add_sample(&mut self, x: f64, y: f64) {
        self.samples.push((x, y));
        if self.samples.len() > 100 {
            self.samples.remove(0);
        }
        self.calculate_trend();
    }

    fn calculate_trend(&mut self) {
        if self.samples.len() < 2 {
            return;
        }

        let n = self.samples.len() as f64;
        let sum_x: f64 = self.samples.iter().map(|(x, _)| x).sum();
        let sum_y: f64 = self.samples.iter().map(|(_, y)| y).sum();
        let sum_xy: f64 = self.samples.iter().map(|(x, y)| x * y).sum();
        let sum_x2: f64 = self.samples.iter().map(|(x, _)| x * x).sum();

        let mean_x = sum_x / n;
        let mean_y = sum_y / n;

        self.slope = (sum_xy - n * mean_x * mean_y) / (sum_x2 - n * mean_x * mean_x);
        self.intercept = mean_y - self.slope * mean_x;

        // Calculate R-squared
        let ss_tot: f64 = self.samples.iter()
            .map(|(_, y)| (y - mean_y).powi(2))
            .sum();
        let ss_res: f64 = self.samples.iter()
            .map(|(x, y)| (y - (self.slope * x + self.intercept)).powi(2))
            .sum();

        self.r_squared = if ss_tot > 0.0 { 1.0 - (ss_res / ss_tot) } else { 0.0 };
    }

    fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    fn trend_direction(&self) -> TrendDirection {
        let confidence = self.r_squared.abs();
        if confidence < 0.1 {
            TrendDirection::Stable(confidence)
        } else if self.slope > 0.01 {
            TrendDirection::Improving(confidence)
        } else if self.slope < -0.01 {
            TrendDirection::Degrading(confidence)
        } else {
            TrendDirection::Stable(confidence)
        }
    }
}

/// Adaptive optimizer that suggests optimizations based on performance data
pub struct AdaptiveOptimizer {
    optimization_history: HashMap<String, OptimizationResult>,
    current_strategy: OptimizationStrategy,
    effectiveness_threshold: f64,
}

#[derive(Debug, Clone)]
struct OptimizationResult {
    strategy: OptimizationStrategy,
    performance_improvement: f64,
    resource_cost: f64,
    success_count: usize,
    failure_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationStrategy {
    Aggressive,
    Balanced,
    Conservative,
    Adaptive,
}

impl RealPerformanceMonitor {
    /// Create new performance monitor
    #[instrument(skip(config))]
    pub fn new(config: MonitoringConfig) -> Result<Self> {
        info!("Initializing real-time performance monitor");
        
        let system_info = Arc::new(Mutex::new(System::new_all()));
        let metrics = Arc::new(Mutex::new(PerformanceMetrics::default()));
        let counters = PerformanceCounters::default();
        
        let trend_analyzer = TrendAnalyzer {
            sample_history: VecDeque::with_capacity(config.sample_history_size),
            trend_calculations: TrendCalculations {
                cpu_trend: LinearRegression::new(),
                memory_trend: LinearRegression::new(),
                performance_trend: LinearRegression::new(),
            },
        };

        let adaptive_optimizer = AdaptiveOptimizer {
            optimization_history: HashMap::new(),
            current_strategy: OptimizationStrategy::Balanced,
            effectiveness_threshold: 1.2, // 20% improvement threshold
        };

        Ok(Self {
            start_time: Instant::now(),
            system_info,
            metrics,
            counters,
            config,
            trend_analyzer,
            adaptive_optimizer,
        })
    }

    /// Start performance monitoring
    #[instrument(skip(self))]
    pub async fn start_monitoring(&mut self) -> Result<()> {
        info!("Starting real-time performance monitoring");
        
        // Initialize system monitoring
        self.update_system_metrics()?;
        
        // Start background monitoring thread
        let system_info = Arc::clone(&self.system_info);
        let metrics = Arc::clone(&self.metrics);
        let sampling_interval = self.config.sampling_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(sampling_interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::update_system_metrics_background(&system_info, &metrics) {
                    warn!("Failed to update system metrics: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Record instruction processing metrics
    #[instrument(skip(self))]
    pub fn record_instruction_processing(&self, count: u64, pass_name: &str) {
        self.counters.instructions_processed.fetch_add(count, Ordering::Relaxed);
        
        if pass_name.contains("optimization") {
            self.counters.optimization_passes.fetch_add(1, Ordering::Relaxed);
        }
        
        debug!("Recorded {} instructions processed in pass: {}", count, pass_name);
    }

    /// Record cache performance
    #[instrument(skip(self))]
    pub fn record_cache_performance(&self, hits: u64, misses: u64) {
        self.counters.cache_hits.fetch_add(hits, Ordering::Relaxed);
        self.counters.cache_misses.fetch_add(misses, Ordering::Relaxed);
        
        let hit_rate = hits as f64 / (hits + misses) as f64;
        debug!("Cache performance: hit rate {:.2}%", hit_rate * 100.0);
    }

    /// Record optimization phase timing
    #[instrument(skip(self))]
    pub fn record_phase_timing(&self, phase_name: &str, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.phase_timings.insert(phase_name.to_string(), duration);
        
        info!("Phase '{}' completed in {:?}", phase_name, duration);
    }

    /// Get current performance metrics
    pub fn get_current_metrics(&self) -> Result<PerformanceMetrics> {
        self.update_calculated_metrics()?;
        Ok(self.metrics.lock().unwrap().clone())
    }

    /// Get optimization recommendations
    #[instrument(skip(self))]
    pub fn get_optimization_recommendations(&mut self) -> Result<Vec<OptimizationRecommendation>> {
        let metrics = self.get_current_metrics()?;
        let mut recommendations = Vec::new();

        // CPU usage recommendations
        if metrics.cpu_stats.average_cpu_usage > self.config.cpu_threshold {
            recommendations.push(OptimizationRecommendation {
                category: RecommendationCategory::CpuOptimization,
                priority: RecommendationPriority::High,
                description: format!(
                    "High CPU usage detected ({:.1}%). Consider reducing optimization level or enabling parallel compilation.",
                    metrics.cpu_stats.average_cpu_usage
                ),
                expected_improvement: 0.3,
                implementation_cost: ImplementationCost::Medium,
            });
        }

        // Memory usage recommendations
        if metrics.memory_stats.current_memory_usage_mb > self.config.memory_threshold as f64 * 1024.0 {
            recommendations.push(OptimizationRecommendation {
                category: RecommendationCategory::MemoryOptimization,
                priority: RecommendationPriority::High,
                description: "High memory usage detected. Consider enabling incremental compilation or reducing cache size.".to_string(),
                expected_improvement: 0.25,
                implementation_cost: ImplementationCost::Low,
            });
        }

        // Cache performance recommendations
        if metrics.cache_stats.cache_hit_rate < 0.7 {
            recommendations.push(OptimizationRecommendation {
                category: RecommendationCategory::CacheOptimization,
                priority: RecommendationPriority::Medium,
                description: format!(
                    "Low cache hit rate ({:.1}%). Consider adjusting cache size or improving data locality.",
                    metrics.cache_stats.cache_hit_rate * 100.0
                ),
                expected_improvement: 0.4,
                implementation_cost: ImplementationCost::Medium,
            });
        }

        // Instruction processing efficiency
        if metrics.instruction_stats.instructions_per_second < 10000.0 {
            recommendations.push(OptimizationRecommendation {
                category: RecommendationCategory::InstructionOptimization,
                priority: RecommendationPriority::Medium,
                description: "Low instruction processing rate detected. Consider enabling SIMD optimizations or reducing optimization complexity.".to_string(),
                expected_improvement: 0.5,
                implementation_cost: ImplementationCost::High,
            });
        }

        // Trend-based recommendations
        self.add_trend_based_recommendations(&mut recommendations)?;

        info!("Generated {} optimization recommendations", recommendations.len());
        Ok(recommendations)
    }

    /// Update calculated metrics based on current counters
    fn update_calculated_metrics(&self) -> Result<()> {
        let mut metrics = self.metrics.lock().unwrap();
        
        // Update compilation time
        metrics.total_compilation_time = self.start_time.elapsed();
        
        // Calculate instruction statistics
        let instructions_processed = self.counters.instructions_processed.load(Ordering::Relaxed);
        let elapsed_seconds = self.start_time.elapsed().as_secs_f64();
        
        metrics.instruction_stats.instructions_processed = instructions_processed;
        metrics.instruction_stats.instructions_per_second = 
            instructions_processed as f64 / elapsed_seconds.max(0.001);
        metrics.instruction_stats.optimization_passes_applied = 
            self.counters.optimization_passes.load(Ordering::Relaxed);

        // Calculate cache statistics
        let cache_hits = self.counters.cache_hits.load(Ordering::Relaxed);
        let cache_misses = self.counters.cache_misses.load(Ordering::Relaxed);
        let total_cache_accesses = cache_hits + cache_misses;
        
        if total_cache_accesses > 0 {
            metrics.cache_stats.cache_hit_rate = cache_hits as f64 / total_cache_accesses as f64;
            metrics.cache_stats.cache_miss_rate = cache_misses as f64 / total_cache_accesses as f64;
            metrics.cache_stats.cache_efficiency = metrics.cache_stats.cache_hit_rate;
        }

        Ok(())
    }

    /// Update system metrics (memory, CPU, etc.)
    fn update_system_metrics(&self) -> Result<()> {
        let mut system = self.system_info.lock().unwrap();
        system.refresh_all();
        
        let mut metrics = self.metrics.lock().unwrap();
        
        // Update memory statistics
        let total_memory = system.total_memory() as f64 / 1024.0 / 1024.0; // MB
        let used_memory = system.used_memory() as f64 / 1024.0 / 1024.0; // MB
        
        metrics.memory_stats.current_memory_usage_mb = used_memory;
        metrics.memory_stats.peak_memory_usage_mb = 
            metrics.memory_stats.peak_memory_usage_mb.max(used_memory);
        metrics.memory_stats.memory_efficiency = used_memory / total_memory;

        // Update CPU statistics
        let cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() 
            / system.cpus().len() as f32;
        
        metrics.cpu_stats.average_cpu_usage = cpu_usage;
        metrics.cpu_stats.peak_cpu_usage = metrics.cpu_stats.peak_cpu_usage.max(cpu_usage);
        metrics.cpu_stats.core_utilization = system.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        Ok(())
    }

    /// Background system metrics update
    fn update_system_metrics_background(
        system_info: &Arc<Mutex<System>>,
        metrics: &Arc<Mutex<PerformanceMetrics>>,
    ) -> Result<()> {
        let mut system = system_info.lock().unwrap();
        system.refresh_cpu();
        system.refresh_memory();
        
        let mut metrics = metrics.lock().unwrap();
        
        // Update real-time metrics
        let used_memory = system.used_memory() as f64 / 1024.0 / 1024.0;
        metrics.memory_stats.current_memory_usage_mb = used_memory;
        
        let cpu_usage: f32 = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() 
            / system.cpus().len() as f32;
        metrics.cpu_stats.average_cpu_usage = cpu_usage;

        Ok(())
    }

    /// Add trend-based recommendations
    fn add_trend_based_recommendations(&mut self, recommendations: &mut Vec<OptimizationRecommendation>) -> Result<()> {
        // Add current sample to trend analysis
        let current_time = self.start_time.elapsed().as_secs_f64();
        let metrics = self.metrics.lock().unwrap();
        
        let sample = PerformanceSample {
            timestamp: Instant::now(),
            cpu_usage: metrics.cpu_stats.average_cpu_usage,
            memory_usage: metrics.memory_stats.current_memory_usage_mb,
            instruction_rate: metrics.instruction_stats.instructions_per_second,
            optimization_effectiveness: metrics.optimization_effectiveness.runtime_improvement_factor,
        };

        self.trend_analyzer.sample_history.push_back(sample.clone());
        if self.trend_analyzer.sample_history.len() > self.config.sample_history_size {
            self.trend_analyzer.sample_history.pop_front();
        }

        // Update trend calculations
        self.trend_analyzer.trend_calculations.cpu_trend.add_sample(current_time, sample.cpu_usage as f64);
        self.trend_analyzer.trend_calculations.memory_trend.add_sample(current_time, sample.memory_usage);
        self.trend_analyzer.trend_calculations.performance_trend.add_sample(current_time, sample.optimization_effectiveness);

        // Generate trend-based recommendations
        match self.trend_analyzer.trend_calculations.cpu_trend.trend_direction() {
            TrendDirection::Degrading(confidence) if confidence > 0.7 => {
                recommendations.push(OptimizationRecommendation {
                    category: RecommendationCategory::TrendAnalysis,
                    priority: RecommendationPriority::Medium,
                    description: "CPU usage is trending upward. Consider proactive optimization to prevent performance degradation.".to_string(),
                    expected_improvement: 0.2,
                    implementation_cost: ImplementationCost::Medium,
                });
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub description: String,
    pub expected_improvement: f64,
    pub implementation_cost: ImplementationCost,
}

#[derive(Debug, Clone)]
pub enum RecommendationCategory {
    CpuOptimization,
    MemoryOptimization,
    CacheOptimization,
    InstructionOptimization,
    TrendAnalysis,
    AdaptiveStrategy,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum ImplementationCost {
    Low,
    Medium,
    High,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            phase_timings: HashMap::new(),
            total_compilation_time: Duration::from_secs(0),
            memory_stats: MemoryStatistics {
                peak_memory_usage_mb: 0.0,
                current_memory_usage_mb: 0.0,
                memory_efficiency: 0.0,
                allocation_rate: 0.0,
                gc_frequency: 0.0,
                memory_fragmentation: 0.0,
            },
            cpu_stats: CpuStatistics {
                average_cpu_usage: 0.0,
                peak_cpu_usage: 0.0,
                cpu_efficiency: 0.0,
                core_utilization: Vec::new(),
                context_switches: 0,
                cpu_cycles_per_instruction: 0.0,
            },
            instruction_stats: InstructionStatistics {
                instructions_processed: 0,
                instructions_per_second: 0.0,
                optimization_passes_applied: 0,
                instructions_eliminated: 0,
                instruction_reduction_ratio: 0.0,
            },
            cache_stats: CacheStatistics {
                cache_hit_rate: 0.0,
                cache_miss_rate: 0.0,
                cache_efficiency: 0.0,
                instruction_cache_misses: 0,
                data_cache_misses: 0,
                tlb_misses: 0,
            },
            optimization_effectiveness: OptimizationEffectiveness {
                runtime_improvement_factor: 1.0,
                code_size_reduction_factor: 1.0,
                compilation_speedup: 1.0,
                optimization_success_rate: 0.0,
                performance_per_watt: 0.0,
            },
            performance_trends: PerformanceTrends {
                compilation_time_trend: TrendDirection::Stable(0.0),
                memory_usage_trend: TrendDirection::Stable(0.0),
                optimization_effectiveness_trend: TrendDirection::Stable(0.0),
                cpu_efficiency_trend: TrendDirection::Stable(0.0),
                predicted_performance_gain: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let config = MonitoringConfig::default();
        let monitor = RealPerformanceMonitor::new(config);
        assert!(monitor.is_ok());
    }

    #[tokio::test]
    async fn test_instruction_recording() {
        let config = MonitoringConfig::default();
        let monitor = RealPerformanceMonitor::new(config).unwrap();
        
        monitor.record_instruction_processing(1000, "optimization_pass");
        
        let metrics = monitor.get_current_metrics().unwrap();
        assert_eq!(metrics.instruction_stats.instructions_processed, 1000);
    }

    #[tokio::test]
    async fn test_cache_performance_recording() {
        let config = MonitoringConfig::default();
        let monitor = RealPerformanceMonitor::new(config).unwrap();
        
        monitor.record_cache_performance(800, 200);
        
        let metrics = monitor.get_current_metrics().unwrap();
        assert_eq!(metrics.cache_stats.cache_hit_rate, 0.8);
    }

    #[test]
    fn test_linear_regression() {
        let mut regression = LinearRegression::new();
        
        // Add samples with increasing trend
        for i in 0..10 {
            regression.add_sample(i as f64, (i * 2) as f64);
        }
        
        assert!(regression.slope > 1.8); // Should be close to 2.0
        assert!(regression.r_squared > 0.9); // Should be highly correlated
        
        match regression.trend_direction() {
            TrendDirection::Improving(_) => {},
            _ => panic!("Expected improving trend"),
        }
    }

    #[tokio::test]
    async fn test_optimization_recommendations() {
        let config = MonitoringConfig {
            cpu_threshold: 50.0, // Low threshold for testing
            memory_threshold: 50.0,
            ..Default::default()
        };
        let mut monitor = RealPerformanceMonitor::new(config).unwrap();
        
        // Simulate high resource usage
        {
            let mut metrics = monitor.metrics.lock().unwrap();
            metrics.cpu_stats.average_cpu_usage = 80.0;
            metrics.memory_stats.current_memory_usage_mb = 2048.0;
            metrics.cache_stats.cache_hit_rate = 0.5;
        }
        
        let recommendations = monitor.get_optimization_recommendations().unwrap();
        assert!(recommendations.len() > 0);
        
        // Check that we get CPU optimization recommendation
        let has_cpu_recommendation = recommendations.iter()
            .any(|r| matches!(r.category, RecommendationCategory::CpuOptimization));
        assert!(has_cpu_recommendation);
    }

    #[test]
    fn test_trend_analysis() {
        let mut analyzer = TrendAnalyzer {
            sample_history: VecDeque::new(),
            trend_calculations: TrendCalculations {
                cpu_trend: LinearRegression::new(),
                memory_trend: LinearRegression::new(),
                performance_trend: LinearRegression::new(),
            },
        };

        // Add samples with degrading performance
        for i in 0..20 {
            analyzer.trend_calculations.performance_trend.add_sample(i as f64, 100.0 - i as f64);
        }

        match analyzer.trend_calculations.performance_trend.trend_direction() {
            TrendDirection::Degrading(_) => {},
            _ => panic!("Expected degrading trend"),
        }
    }
}
