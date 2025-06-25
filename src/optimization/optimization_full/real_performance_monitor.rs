/// Real-time Performance Monitoring System
/// 
/// Provides comprehensive performance tracking during compilation and optimization
/// with measurable metrics, trend analysis, and adaptive optimization suggestions.

use crate::error::{CursedError, Result};

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::{Duration, Instant, SystemTime};
use sysinfo::{System, Process, Cpu};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

/// Real-time performance monitor with measurable metrics
pub struct RealPerformanceMonitor {
/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Sampling interval for system metrics
    /// Number of samples to keep for trend analysis
    /// CPU usage threshold for optimization warnings
    /// Memory usage threshold (percentage)
    /// Enable detailed instruction counting
    /// Enable cache performance monitoring
    /// Enable adaptive optimization suggestions
impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Comprehensive performance metrics with real measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Compilation phase timings
    /// Total compilation time
    /// Memory usage statistics
    /// CPU usage statistics
    /// Instruction processing statistics
    /// Cache performance metrics
    /// Optimization effectiveness metrics
    /// Performance trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStatistics {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionStatistics {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEffectiveness {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
/// Atomic performance counters for thread-safe updates
struct PerformanceCounters {
impl Default for PerformanceCounters {
    fn default() -> Self {
        Self {
        }
    }
/// Trend analyzer for performance metrics
pub struct TrendAnalyzer {
#[derive(Debug, Clone)]
struct PerformanceSample {
struct TrendCalculations {
/// Simple linear regression for trend analysis
struct LinearRegression {
impl LinearRegression {
    fn new() -> Self {
        Self {
        }
    }

    fn add_sample(&mut self, x: f64, y: f64) {
        self.samples.push((x, y));
        if self.samples.len() > 100 {
            self.samples.remove(0);
        }
        self.calculate_trend();
    fn calculate_trend(&mut self) {
        if self.samples.len() < 2 {
            return;
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
    fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
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
/// Adaptive optimizer that suggests optimizations based on performance data
pub struct AdaptiveOptimizer {
#[derive(Debug, Clone)]
struct OptimizationResult {
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationStrategy {
impl RealPerformanceMonitor {
    /// Create new performance monitor
    #[instrument(skip(config))]
    pub fn new(config: MonitoringConfig) -> Result<Self> {
        info!("Initializing real-time performance monitor");
        
        let system_info = Arc::new(Mutex::new(System::new_all()));
        let metrics = Arc::new(Mutex::new(PerformanceMetrics::default()));
        let counters = PerformanceCounters::default();
        
        let trend_analyzer = TrendAnalyzer {
            trend_calculations: TrendCalculations {

        let adaptive_optimizer = AdaptiveOptimizer {
            effectiveness_threshold: 1.2, // 20% improvement threshold

        Ok(Self {
        })
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
    /// Record instruction processing metrics
    #[instrument(skip(self))]
    pub fn record_instruction_processing(&self, count: u64, pass_name: &str) {
        self.counters.instructions_processed.fetch_add(count, Ordering::Relaxed);
        
        if pass_name.contains("optimization") {
            self.counters.optimization_passes.fetch_add(1, Ordering::Relaxed);
        debug!("Recorded {} instructions processed in pass: {}", count, pass_name);
    /// Record cache performance
    #[instrument(skip(self))]
    pub fn record_cache_performance(&self, hits: u64, misses: u64) {
        self.counters.cache_hits.fetch_add(hits, Ordering::Relaxed);
        self.counters.cache_misses.fetch_add(misses, Ordering::Relaxed);
        
        let hit_rate = hits as f64 / (hits + misses) as f64;
        debug!("Cache performance: hit rate {:.2}%", hit_rate * 100.0);
    /// Record optimization phase timing
    #[instrument(skip(self))]
    pub fn record_phase_timing(&self, phase_name: &str, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.phase_timings.insert(phase_name.to_string(), duration);
        
        info!("Phase '{}' completed in {:?}", phase_name, duration);
    /// Get current performance metrics
    pub fn get_current_metrics(&self) -> Result<PerformanceMetrics> {
        self.update_calculated_metrics()?;
        Ok(self.metrics.lock().unwrap().clone())
    /// Get optimization recommendations
    #[instrument(skip(self))]
    pub fn get_optimization_recommendations(&mut self) -> Result<Vec<OptimizationRecommendation>> {
        let metrics = self.get_current_metrics()?;
        let mut recommendations = Vec::new();

        // CPU usage recommendations
        if metrics.cpu_stats.average_cpu_usage > self.config.cpu_threshold {
            recommendations.push(OptimizationRecommendation {
                description: format!(
                    metrics.cpu_stats.average_cpu_usage
            });
        // Memory usage recommendations
        if metrics.memory_stats.current_memory_usage_mb > self.config.memory_threshold as f64 * 1024.0 {
            recommendations.push(OptimizationRecommendation {
            });
        // Cache performance recommendations
        if metrics.cache_stats.cache_hit_rate < 0.7 {
            recommendations.push(OptimizationRecommendation {
                description: format!(
                    metrics.cache_stats.cache_hit_rate * 100.0
            });
        // Instruction processing efficiency
        if metrics.instruction_stats.instructions_per_second < 10000.0 {
            recommendations.push(OptimizationRecommendation {
            });
        // Trend-based recommendations
        self.add_trend_based_recommendations(&mut recommendations)?;

        info!("Generated {} optimization recommendations", recommendations.len());
        Ok(recommendations)
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
        Ok(())
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
    /// Background system metrics update
    fn update_system_metrics_background(
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
    /// Add trend-based recommendations
    fn add_trend_based_recommendations(&mut self, recommendations: &mut Vec<OptimizationRecommendation>) -> Result<()> {
        // Add current sample to trend analysis
        let current_time = self.start_time.elapsed().as_secs_f64();
        let metrics = self.metrics.lock().unwrap();
        
        let sample = PerformanceSample {

        self.trend_analyzer.sample_history.push_back(sample.clone());
        if self.trend_analyzer.sample_history.len() > self.config.sample_history_size {
            self.trend_analyzer.sample_history.pop_front();
        // Update trend calculations
        self.trend_analyzer.trend_calculations.cpu_trend.add_sample(current_time, sample.cpu_usage as f64);
        self.trend_analyzer.trend_calculations.memory_trend.add_sample(current_time, sample.memory_usage);
        self.trend_analyzer.trend_calculations.performance_trend.add_sample(current_time, sample.optimization_effectiveness);

        // Generate trend-based recommendations
        match self.trend_analyzer.trend_calculations.cpu_trend.trend_direction() {
            TrendDirection::Degrading(confidence) if confidence > 0.7 => {
                recommendations.push(OptimizationRecommendation {
                });
            }
            _ => {}
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
#[derive(Debug, Clone)]
pub enum RecommendationCategory {
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
#[derive(Debug, Clone)]
pub enum ImplementationCost {
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            memory_stats: MemoryStatistics {
            cpu_stats: CpuStatistics {
            instruction_stats: InstructionStatistics {
            cache_stats: CacheStatistics {
            optimization_effectiveness: OptimizationEffectiveness {
            performance_trends: PerformanceTrends {
        }
    }
