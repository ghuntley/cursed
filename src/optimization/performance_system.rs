use crate::error::CursedError;
// Performance system for comprehensive optimization management
//
// This module provides a unified interface for performance monitoring,
// optimization coordination, and system resource management.

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
    /// Compilation speed optimization
    /// Profile-guided optimization
    /// System configuration
    /// Performance history
/// Configuration for the performance system
#[derive(Debug, Clone)]
pub struct PerformanceSystemConfig {
    /// Enable automatic optimization
    /// Maximum parallel compilation jobs
    /// Performance monitoring interval
    /// Performance history retention limit
    /// Enable profile-guided optimization
    /// Optimization aggressiveness level
/// Optimization aggressiveness levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceOptimizationLevel {
    /// Conservative optimization (fast compilation)
    /// Balanced optimization (default)
    /// Aggressive optimization (slower compilation, better runtime)
    /// Maximum optimization (slowest compilation, best runtime)
/// Snapshot of performance metrics at a specific time
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    /// Timestamp of the snapshot
    /// Compilation statistics at this time
    /// System resource usage
    /// Resource utilization statistics
    /// Overall performance score (0-100)
/// Status of the performance system
#[derive(Debug, Clone)]
pub struct PerformanceStatus {
    /// Current compilation throughput (compilations per second)
    /// Average compilation time
    /// System resource utilization percentage
    /// Cache effectiveness percentage
    /// Overall system health score (0-100)
    /// Active optimization recommendations
impl PerformanceSystem {
    /// Create a new performance system with default configuration
    pub fn new() -> crate::error::Result<()> {
        let config = PerformanceSystemConfig::default();
        Self::with_config(config)
    /// Create a new performance system with custom configuration
    pub fn with_config(config: PerformanceSystemConfig) -> crate::error::Result<()> {
        let pgo_config = if config.enable_pgo {
            PgoSystemConfig::default()
        } else {
            PgoSystemConfig {
                ..PgoSystemConfig::default()
            }

        Ok(Self {
        })
    /// Start performance monitoring
    pub fn start_monitoring(&self) -> Result<(), String> {
        // In a real implementation, this would start background monitoring threads
        self.record_performance_snapshot()?;
        Ok(())
    /// Stop performance monitoring
    pub fn stop_monitoring(&self) -> Result<(), String> {
        // In a real implementation, this would stop background monitoring threads
        Ok(())
    /// Record a performance snapshot
    pub fn record_performance_snapshot(&self) -> Result<(), String> {
        let metrics = self.metrics_collector.lock().map_err(|e| format!("Lock error: {}", e))?;
        let summary = metrics.get_summary();
        
        let snapshot = PerformanceSnapshot {
            compilation_stats: CompilationStatistics {
                total_size_bytes: 0, // Would be populated in real implementation
            system_stats: SystemStatistics {
                available_memory_bytes: summary.peak_memory * 4, // Estimate
                load_average: summary.current_cpu_usage / 100.0 * 4.0,
            resource_stats: ResourceStatistics {

        let mut history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        history.push(snapshot);

        // Limit history size
        if history.len() > self.config.history_retention_limit {
            history.remove(0);
        Ok(())
    /// Get current performance status
    pub fn get_performance_status(&self) -> Result<PerformanceStatus, String> {
        let metrics = self.metrics_collector.lock().map_err(|e| format!("Lock error: {}", e))?;
        let summary = metrics.get_summary();
        
        let history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        
        let compilation_throughput = if summary.total_time.as_secs() > 0 {
            summary.total_compilations as f64 / summary.total_time.as_secs_f64()
        } else {
            0.0

        let resource_utilization = (summary.current_cpu_usage + summary.peak_memory as f64 / 1024.0 / 1024.0 / 1024.0 * 25.0) / 2.0;
        
        let health_score = self.calculate_health_score(&summary);
        let recommendations = self.generate_recommendations(&summary);

        Ok(PerformanceStatus {
        })
    /// Optimize system performance based on current metrics
    pub fn optimize_performance(&self) -> Result<Vec<String>, String> {
        let mut optimizations_applied = Vec::new();

        // Get current performance status
        let status = self.get_performance_status()?;

        // Apply optimizations based on current state
        if status.cache_effectiveness < 50.0 {
            optimizations_applied.push("Increased cache size and retention time".to_string());
        if status.resource_utilization > 80.0 {
            optimizations_applied.push("Reduced parallel compilation jobs to prevent resource exhaustion".to_string());
        if status.compilation_throughput < 1.0 {
            optimizations_applied.push("Enabled aggressive compilation caching".to_string());
        // Record the optimization event
        self.record_performance_snapshot()?;

        Ok(optimizations_applied)
    /// Generate performance recommendations
    fn generate_recommendations(&self, summary: &super::metrics::MetricsSummary) -> Vec<String> {
        let mut recommendations = Vec::new();

        if summary.cache_hit_rate < 0.5 {
            recommendations.push("Consider increasing cache size or improving cache key generation".to_string());
        if summary.current_cpu_usage > 80.0 {
            recommendations.push("System CPU usage is high - consider reducing parallel compilation jobs".to_string());
        if summary.average_time.as_millis() > 1000 {
            recommendations.push("Average compilation time is high - enable more aggressive caching".to_string());
        if summary.success_rate < 0.9 {
            recommendations.push("Compilation success rate is low - check for system resource issues".to_string());
        recommendations
    /// Calculate overall performance score (0-100)
    fn calculate_performance_score(&self, summary: &super::metrics::MetricsSummary) -> f64 {
        let success_score = summary.success_rate * 30.0;
        let speed_score = if summary.average_time.as_millis() > 0 {
            (1000.0 / summary.average_time.as_millis() as f64).min(1.0) * 25.0
        } else {
            25.0
        let cache_score = summary.cache_hit_rate * 25.0;
        let resource_score = ((100.0 - summary.current_cpu_usage) / 100.0) * 20.0;

        success_score + speed_score + cache_score + resource_score
    /// Calculate system health score
    fn calculate_health_score(&self, summary: &super::metrics::MetricsSummary) -> f64 {
        // Simple health calculation based on multiple factors
        let success_factor = summary.success_rate * 40.0;
        let performance_factor = if summary.average_time.as_millis() < 500 { 30.0 } else { 15.0 };
        let resource_factor = if summary.current_cpu_usage < 70.0 { 30.0 } else { 15.0 };

        success_factor + performance_factor + resource_factor
    /// Calculate parallel efficiency
    fn calculate_parallel_efficiency(&self) -> f64 {
        // Simplified calculation - in real implementation would measure actual speedup
        let theoretical_max = self.config.max_parallel_jobs as f64;
        let efficiency_factor = 0.8; // Assume 80% efficiency
        efficiency_factor * (theoretical_max / theoretical_max.max(1.0))
    /// Get performance history
    pub fn get_performance_history(&self) -> Result<Vec<PerformanceSnapshot>, String> {
        let history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        Ok(history.clone())
    /// Clear performance history
    pub fn clear_performance_history(&self) -> Result<(), String> {
        let mut history = self.performance_history.lock().map_err(|e| format!("Lock error: {}", e))?;
        history.clear();
        Ok(())
    /// Update system configuration
    pub fn update_config(&mut self, config: PerformanceSystemConfig) {
        self.config = config;
    /// Get current configuration
    pub fn get_config(&self) -> &PerformanceSystemConfig {
        &self.config
    }
}

impl Default for PerformanceSystemConfig {
    fn default() -> Self {
        Self {
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
    /// Compilation is in progress
    /// Compilation completed successfully
    /// Compilation failed
    /// Compilation was cancelled
impl std::fmt::Display for CompilationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
// Additional types required by performance_optimization_system.rs

/// Performance monitoring level for granular control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceMonitoringLevel {
    /// Minimal monitoring with low overhead
    /// Basic monitoring with standard metrics
    /// Detailed monitoring with comprehensive metrics
    /// Verbose monitoring with all available metrics
/// Configuration for parallel compilation
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Maximum number of parallel jobs
    /// Enable parallel compilation
    /// Thread pool size for compilation
/// Configuration for compilation caching
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable compilation caching
    /// Cache directory path
    /// Maximum cache size in MB
    /// Cache TTL in seconds
/// Performance metrics for compilation
#[derive(Debug, Clone)]
pub struct CompilationPerformanceMetrics {
    /// Total compilation time
    /// Memory usage during compilation
    /// CPU utilization percentage
    /// Cache hit rate
    /// Number of files compiled
/// Adaptive decision for optimization
#[derive(Debug, Clone)]
pub struct AdaptiveDecision {
    /// Type of decision made
    /// Confidence level (0.0 to 1.0)
    /// Reasoning for the decision
/// Types of adaptive decisions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptiveDecisionType {
    /// Enable optimization
    /// Disable optimization
    /// Adjust optimization level
    /// Enable caching
    /// Adjust parallel jobs
/// Performance recommendation
#[derive(Debug, Clone)]
pub struct PerformanceRecommendation {
    /// Type of recommendation
    /// Priority level (1-10, 10 being highest)
    /// Description of the recommendation
    /// Expected performance impact
/// Types of performance recommendations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationType {
    /// Optimize compilation speed
    /// Optimize runtime performance
    /// Reduce memory usage
    /// Enable caching
    /// Adjust parallel configuration
/// Optimization session tracking
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    /// Unique session ID
    /// Session start time
    /// Session configuration
    /// Performance metrics collected during session
    /// Decisions made during session
    /// Recommendations generated
impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: "target/cursed-cache".to_string(),
            max_cache_size_mb: 1024, // 1GB
            cache_ttl_seconds: 3600, // 1 hour
        }
    }
impl Default for PerformanceMonitoringLevel {
    fn default() -> Self {
        Self::Basic
    }
}
