// Real time savings calculations for optimization passes

use crate::error::{CursedError, Result};
use crate::optimization::metrics::CompilationUnit;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument};

/// Time savings analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSavingsAnalysis {
    /// Total time saved from all optimizations
    /// Time saved from parallel compilation
    /// Time saved from incremental compilation
    /// Time saved from caching
    /// Time saved from LLVM optimizations
    /// Time saved from dependency optimization
    /// Breakdown by optimization type
    /// Overall efficiency improvement percentage
    /// Compilation speed improvement (units per second)
/// Savings from a specific optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSavings {
    /// Name of the optimization
    /// Time saved by this optimization
    /// Number of compilation units affected
    /// Average time saved per unit
    /// Confidence level in the measurement
    /// Additional metadata
/// Configuration for time savings calculation
#[derive(Debug, Clone)]
pub struct TimeSavingsConfig {
    /// Baseline compilation time per unit (used for estimates)
    /// Cache lookup time
    /// Incremental analysis time
    /// Parallel scheduling overhead
    /// LLVM optimization overhead
    /// Whether to include statistical confidence intervals
impl Default for TimeSavingsConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Time savings calculator
pub struct TimeSavingsCalculator {
/// Compilation measurement data
#[derive(Debug, Clone)]
pub struct CompilationMeasurement {
    /// Timestamp of the measurement
    /// Total compilation time
    /// Number of units compiled
    /// Units from cache
    /// Units from incremental
    /// Parallel efficiency
    /// LLVM optimization time
    /// Cache lookup time
    /// Incremental analysis time
    /// Dependency analysis time
/// Compilation timing context
#[derive(Debug, Clone)]
pub struct CompilationTimingContext {
    /// Start time of compilation
    /// Individual unit timings
    /// Optimization pass timings
    /// Cache operation timings
    /// Parallel compilation metrics
/// Timing information for a compilation unit
#[derive(Debug, Clone)]
pub struct UnitTiming {
    /// Unit name
    /// Start time
    /// End time
    /// Whether this unit was from cache
    /// Whether this unit was from incremental compilation
    /// Individual optimization pass times
/// Cache operation timings
#[derive(Debug, Clone, Default)]
pub struct CacheTimings {
    /// Total cache lookup time
    /// Total cache store time
    /// Number of cache hits
    /// Number of cache misses
/// Parallel compilation metrics
#[derive(Debug, Clone, Default)]
pub struct ParallelMetrics {
    /// Number of worker threads
    /// Thread utilization percentages
    /// Work stealing statistics
    /// Synchronization overhead
impl TimeSavingsCalculator {
    /// Create a new time savings calculator
    pub fn new(config: TimeSavingsConfig) -> Self {
        Self {
        }
    }

    /// Start measuring compilation timing
    #[instrument(skip(self))]
    pub fn start_measurement(&mut self) -> CompilationTimingContext {
        debug!("Starting compilation timing measurement");
        CompilationTimingContext {
        }
    }

    /// Record timing for a compilation unit
    pub fn record_unit_timing(
    ) {
        context.unit_timings.insert(unit.name.clone(), timing);
    /// Record timing for an optimization pass
    pub fn record_optimization_timing(
    ) {
        context.optimization_timings.insert(pass_name.to_string(), duration);
    /// Calculate actual time savings from compilation measurement
    #[instrument(skip(self, context))]
    pub fn calculate_time_savings(
    ) -> Result<TimeSavingsAnalysis> {
        info!("Calculating time savings for {} compiled units", units_compiled);

        let total_compilation_time = context.start_time.elapsed();
        
        // Calculate individual optimization savings
        let cache_savings = self.calculate_cache_savings(&context.cache_timings, units_from_cache);
        let incremental_savings = self.calculate_incremental_savings(units_from_incremental);
        let parallel_savings = self.calculate_parallel_savings(
        );
        let llvm_optimization_savings = self.calculate_llvm_optimization_savings(&context.optimization_timings);
        let dependency_optimization_savings = self.calculate_dependency_optimization_savings(&context.optimization_timings);

        // Calculate total savings
        let total_time_saved = cache_savings + incremental_savings + parallel_savings + 
                              llvm_optimization_savings + dependency_optimization_savings;

        // Create detailed breakdown
        let mut savings_breakdown = HashMap::new();
        
        savings_breakdown.insert("cache".to_string(), OptimizationSavings {
            avg_savings_per_unit: if units_from_cache > 0 {
                cache_savings / units_from_cache as u32
            } else {
                Duration::from_secs(0)
        });

        savings_breakdown.insert("incremental".to_string(), OptimizationSavings {
            avg_savings_per_unit: if units_from_incremental > 0 {
                incremental_savings / units_from_incremental as u32
            } else {
                Duration::from_secs(0)
            confidence_level: 0.9, // High confidence in incremental savings
        });

        savings_breakdown.insert("parallel".to_string(), OptimizationSavings {
            avg_savings_per_unit: if units_compiled > 0 {
                parallel_savings / units_compiled as u32
            } else {
                Duration::from_secs(0)
        });

        // Calculate efficiency improvement
        let baseline_time = self.calculate_baseline_compilation_time(units_compiled);
        let efficiency_improvement_percent = if baseline_time.as_secs_f64() > 0.0 {
            (total_time_saved.as_secs_f64() / baseline_time.as_secs_f64()) * 100.0
        } else {
            0.0

        // Calculate throughput improvement
        let baseline_throughput = units_compiled as f64 / baseline_time.as_secs_f64();
        let actual_throughput = units_compiled as f64 / total_compilation_time.as_secs_f64();
        let throughput_improvement = actual_throughput - baseline_throughput;

        // Record this measurement for historical analysis
        let measurement = CompilationMeasurement {
        self.measurement_history.push(measurement);

        // Keep only recent measurements
        if self.measurement_history.len() > 100 {
            self.measurement_history.drain(0..50);
        let analysis = TimeSavingsAnalysis {

        info!(
            "Time savings analysis completed"
        );

        Ok(analysis)
    /// Calculate cache savings based on cache hits and timing
    fn calculate_cache_savings(&self, cache_timings: &CacheTimings, units_from_cache: usize) -> Duration {
        if units_from_cache == 0 {
            return Duration::from_secs(0);
        // Time saved = (baseline compile time - cache lookup time) * units from cache
        let baseline_time_per_unit = self.config.baseline_compile_time_per_unit;
        let avg_lookup_time = if cache_timings.cache_hits > 0 {
            cache_timings.total_lookup_time / cache_timings.cache_hits as u32
        } else {
            self.config.cache_lookup_time

        let time_saved_per_unit = baseline_time_per_unit.saturating_sub(avg_lookup_time);
        time_saved_per_unit * units_from_cache as u32
    /// Calculate incremental compilation savings
    fn calculate_incremental_savings(&self, units_from_incremental: usize) -> Duration {
        if units_from_incremental == 0 {
            return Duration::from_secs(0);
        // Incremental compilation saves most of the compilation time minus analysis overhead
        let baseline_time_per_unit = self.config.baseline_compile_time_per_unit;
        let analysis_overhead = self.config.incremental_analysis_time;
        
        let time_saved_per_unit = baseline_time_per_unit.saturating_sub(analysis_overhead);
        time_saved_per_unit * units_from_incremental as u32
    /// Calculate parallel compilation savings
    fn calculate_parallel_savings(
    ) -> Duration {
        if units_compiled == 0 || parallel_efficiency <= 1.0 {
            return Duration::from_secs(0);
        // Calculate what sequential time would have been
        let baseline_sequential_time = self.config.baseline_compile_time_per_unit * units_compiled as u32;
        
        // Account for parallel scheduling overhead
        let overhead = self.config.parallel_scheduling_overhead + parallel_metrics.synchronization_overhead;
        let adjusted_actual_time = actual_time + overhead;
        
        baseline_sequential_time.saturating_sub(adjusted_actual_time)
    /// Calculate LLVM optimization savings
    fn calculate_llvm_optimization_savings(&self, optimization_timings: &HashMap<String, Duration>) -> Duration {
        // This represents time saved in runtime due to LLVM optimizations
        // We estimate this based on the optimization time invested
        let total_optimization_time: Duration = optimization_timings.values().sum();
        
        // Conservative estimate: every second of optimization time saves 2 seconds in runtime
        // This would need to be calibrated based on actual measurements
        total_optimization_time * 2
    /// Calculate dependency optimization savings
    fn calculate_dependency_optimization_savings(&self, optimization_timings: &HashMap<String, Duration>) -> Duration {
        // Time saved from dependency analysis and reordering
        optimization_timings.get("dependency_analysis")
            .map(|&time| time / 2) // Assume dependency analysis saves half its time investment
            .unwrap_or_else(|| Duration::from_secs(0))
    /// Calculate baseline compilation time without optimizations
    fn calculate_baseline_compilation_time(&self, units_compiled: usize) -> Duration {
        self.config.baseline_compile_time_per_unit * units_compiled as u32
    /// Calculate confidence level for cache savings
    fn calculate_cache_confidence(&self, cache_timings: &CacheTimings) -> f64 {
        let total_operations = cache_timings.cache_hits + cache_timings.cache_misses;
        if total_operations == 0 {
            return 0.0;
        // Higher confidence with more cache operations
        let sample_confidence = (total_operations.min(100) as f64 / 100.0) * 0.5 + 0.5;
        
        // Higher confidence with higher hit rate
        let hit_rate = cache_timings.cache_hits as f64 / total_operations as f64;
        let hit_rate_confidence = hit_rate * 0.3 + 0.7;
        
        sample_confidence * hit_rate_confidence
    /// Calculate confidence level for parallel savings
    fn calculate_parallel_confidence(&self, efficiency: f64) -> f64 {
        // Higher confidence with better parallel efficiency
        if efficiency <= 1.0 {
            0.0
        } else if efficiency >= 2.0 {
            0.95
        } else {
            0.5 + (efficiency - 1.0) * 0.45
        }
    }

    /// Create metadata for cache optimization
    fn create_cache_metadata(&self, cache_timings: &CacheTimings) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        metadata.insert("cache_hits".to_string(), cache_timings.cache_hits.to_string());
        metadata.insert("cache_misses".to_string(), cache_timings.cache_misses.to_string());
        
        let hit_rate = if cache_timings.cache_hits + cache_timings.cache_misses > 0 {
            (cache_timings.cache_hits as f64 / (cache_timings.cache_hits + cache_timings.cache_misses) as f64) * 100.0
        } else {
            0.0
        metadata.insert("hit_rate_percent".to_string(), format!("{:.1}", hit_rate));
        
                        cache_timings.total_lookup_time.as_millis().to_string());
        
        metadata
    /// Create metadata for parallel optimization
    fn create_parallel_metadata(&self, metrics: &ParallelMetrics, efficiency: f64) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        metadata.insert("worker_threads".to_string(), metrics.worker_threads.to_string());
        metadata.insert("parallel_efficiency".to_string(), format!("{:.2}", efficiency));
        metadata.insert("work_stealing_events".to_string(), metrics.work_stealing_events.to_string());
                       metrics.synchronization_overhead.as_millis().to_string());
        
        if !metrics.thread_utilizations.is_empty() {
            let avg_utilization = metrics.thread_utilizations.iter().sum::<f64>() / metrics.thread_utilizations.len() as f64;
            metadata.insert("avg_thread_utilization".to_string(), format!("{:.1}%", avg_utilization * 100.0));
        metadata
    /// Extract incremental analysis time from compilation context
    fn extract_incremental_analysis_time(&self, context: &CompilationTimingContext) -> Duration {
        // Extract incremental analysis time from various sources in the context
        
        // Check if there are specific incremental compilation timings
        let mut total_incremental_time = Duration::from_secs(0);
        
        // Look for incremental analysis in optimization timings
        if let Some(&incremental_time) = context.optimization_timings.get("incremental_analysis") {
            total_incremental_time += incremental_time;
        // Check for dependency analysis time (which is part of incremental compilation)
        if let Some(&dependency_time) = context.optimization_timings.get("dependency_analysis") {
            total_incremental_time += dependency_time;
        // Check for file change detection time
        if let Some(&change_detection_time) = context.optimization_timings.get("change_detection") {
            total_incremental_time += change_detection_time;
        // Check unit timings for incremental units
        let incremental_unit_time: Duration = context.unit_timings.values()
            .filter(|timing| timing.from_incremental)
            .filter_map(|timing| timing.end_time.map(|end| end.duration_since(timing.start_time)))
            .sum();
        
        total_incremental_time += incremental_unit_time;
        
        // If no specific incremental timings found, estimate based on configuration
        if total_incremental_time == Duration::from_secs(0) {
            // Check if any units were from incremental compilation
            let has_incremental_units = context.unit_timings.values()
                .any(|timing| timing.from_incremental);
            
            if has_incremental_units {
                // Use configured incremental analysis time as estimate
                return self.config.incremental_analysis_time;
            }
        }
        
        total_incremental_time
    /// Get historical trend analysis
    pub fn get_trend_analysis(&self) -> Option<TrendAnalysis> {
        if self.measurement_history.len() < 3 {
            return None;
        let recent_measurements = &self.measurement_history[self.measurement_history.len().saturating_sub(10)..];
        
        let avg_efficiency = recent_measurements.iter()
            .map(|m| {
                let baseline = self.calculate_baseline_compilation_time(m.units_compiled);
                if baseline.as_secs_f64() > 0.0 {
                    m.total_time.as_secs_f64() / baseline.as_secs_f64()
                } else {
                    1.0
                }
            })
            .sum::<f64>() / recent_measurements.len() as f64;

        let avg_parallel_efficiency = recent_measurements.iter()
            .map(|m| m.parallel_efficiency)
            .sum::<f64>() / recent_measurements.len() as f64;

        Some(TrendAnalysis {
        })
    }
}

/// Trend analysis for optimization performance
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    /// Average efficiency ratio (actual time / baseline time)
    /// Average parallel efficiency
    /// Number of measurements in the analysis
    /// Overall trend direction
/// Trend direction for performance
#[derive(Debug, Clone)]
pub enum TrendDirection {
