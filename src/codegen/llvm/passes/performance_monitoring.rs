/// Performance Monitoring and Analysis for Optimization Passes
/// 
/// Provides comprehensive performance tracking, analysis, and reporting
/// for optimization passes to measure effectiveness and guide optimization decisions.

use super::{PassResult, PassStatistics, OptimizationLevel};
use crate::error::{Error, Result};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, instrument, warn};

/// Performance monitor for tracking optimization effectiveness
pub struct PerformanceMonitor {
    metrics: HashMap<String, OptimizationMetrics>,
    execution_history: VecDeque<PassExecutionStats>,
    baseline_metrics: Option<BaselineMetrics>,
    monitoring_config: MonitoringConfig,
    analysis_engine: AnalysisEngine,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            metrics: HashMap::new(),
            execution_history: VecDeque::with_capacity(config.history_size),
            baseline_metrics: None,
            monitoring_config: config,
            analysis_engine: AnalysisEngine::new(),
        }
    }
    
    /// Record pass execution metrics
    #[instrument(skip(self, pass_result))]
    pub fn record_pass_execution(
        &mut self,
        pass_name: &str,
        pass_result: &PassResult,
        before_metrics: &CodeMetrics,
        after_metrics: &CodeMetrics,
    ) -> Result<()> {
        debug!("Recording performance metrics for pass: {}", pass_name);
        
        let execution_stats = PassExecutionStats {
            pass_name: pass_name.to_string(),
            timestamp: SystemTime::now(),
            execution_time: pass_result.execution_time,
            memory_usage: pass_result.memory_usage,
            optimizations_applied: pass_result.instructions_eliminated +
                                  pass_result.functions_inlined +
                                  pass_result.loops_unrolled +
                                  pass_result.constants_folded,
            before_metrics: before_metrics.clone(),
            after_metrics: after_metrics.clone(),
            effectiveness_score: pass_result.effectiveness_score(),
            changed: pass_result.changed,
            errors: pass_result.errors.len(),
        };
        
        // Update per-pass metrics
        let metrics = self.metrics.entry(pass_name.to_string()).or_insert_with(OptimizationMetrics::new);
        metrics.update(&execution_stats);
        
        // Add to execution history
        self.execution_history.push_back(execution_stats);
        if self.execution_history.len() > self.monitoring_config.history_size {
            self.execution_history.pop_front();
        }
        
        // Analyze performance trends
        if self.monitoring_config.enable_trend_analysis {
            self.analysis_engine.analyze_trends(pass_name, &self.execution_history);
        }
        
        debug!("Recorded metrics for pass {}: {:.2} effectiveness, {:?} execution time",
               pass_name, execution_stats.effectiveness_score, execution_stats.execution_time);
        
        Ok(())
    }
    
    /// Set baseline metrics for comparison
    pub fn set_baseline(&mut self, metrics: BaselineMetrics) {
        info!("Setting baseline metrics for performance comparison");
        self.baseline_metrics = Some(metrics);
    }
    
    /// Get optimization metrics for a specific pass
    pub fn get_pass_metrics(&self, pass_name: &str) -> Option<&OptimizationMetrics> {
        self.metrics.get(pass_name)
    }
    
    /// Get all tracked optimization metrics
    pub fn get_all_metrics(&self) -> &HashMap<String, OptimizationMetrics> {
        &self.metrics
    }
    
    /// Generate performance report
    #[instrument(skip(self))]
    pub fn generate_report(&self) -> Result<PerformanceReport> {
        info!("Generating performance report");
        
        let mut report = PerformanceReport::default();
        report.timestamp = SystemTime::now();
        report.total_passes_monitored = self.metrics.len();
        report.total_executions = self.execution_history.len();
        
        // Calculate overall statistics
        let mut total_execution_time = Duration::from_secs(0);
        let mut total_optimizations = 0;
        let mut total_effectiveness = 0.0;
        
        for stats in &self.execution_history {
            total_execution_time += stats.execution_time;
            total_optimizations += stats.optimizations_applied;
            total_effectiveness += stats.effectiveness_score;
        }
        
        if !self.execution_history.is_empty() {
            report.average_execution_time = total_execution_time / self.execution_history.len() as u32;
            report.average_effectiveness = total_effectiveness / self.execution_history.len() as f64;
        }
        
        report.total_optimizations_applied = total_optimizations;
        
        // Analyze pass performance
        for (pass_name, metrics) in &self.metrics {
            let pass_analysis = self.analyze_pass_performance(pass_name, metrics);
            report.pass_analyses.insert(pass_name.clone(), pass_analysis);
        }
        
        // Generate recommendations
        report.recommendations = self.generate_recommendations();
        
        // Compare with baseline if available
        if let Some(ref baseline) = self.baseline_metrics {
            report.baseline_comparison = Some(self.compare_with_baseline(baseline));
        }
        
        // Trend analysis
        if self.monitoring_config.enable_trend_analysis {
            report.trends = self.analysis_engine.get_trend_summary();
        }
        
        info!("Generated performance report with {} pass analyses", report.pass_analyses.len());
        Ok(report)
    }
    
    /// Analyze performance of a specific pass
    fn analyze_pass_performance(&self, pass_name: &str, metrics: &OptimizationMetrics) -> PassPerformanceAnalysis {
        let mut analysis = PassPerformanceAnalysis::default();
        
        analysis.total_executions = metrics.execution_count;
        analysis.average_execution_time = metrics.average_execution_time;
        analysis.total_optimizations = metrics.total_optimizations_applied;
        analysis.average_effectiveness = metrics.average_effectiveness;
        analysis.success_rate = metrics.success_rate;
        
        // Analyze efficiency
        if metrics.average_execution_time.as_millis() > 0 {
            analysis.optimizations_per_ms = metrics.total_optimizations_applied as f64 
                / (metrics.average_execution_time.as_millis() as f64 * metrics.execution_count as f64);
        }
        
        // Categorize performance
        analysis.performance_category = self.categorize_pass_performance(metrics);
        
        // Generate specific recommendations
        analysis.recommendations = self.generate_pass_recommendations(pass_name, metrics);
        
        analysis
    }
    
    /// Categorize pass performance
    fn categorize_pass_performance(&self, metrics: &OptimizationMetrics) -> PerformanceCategory {
        // Use heuristics to categorize performance
        if metrics.average_effectiveness > 10.0 && metrics.average_execution_time < Duration::from_millis(100) {
            PerformanceCategory::Excellent
        } else if metrics.average_effectiveness > 5.0 && metrics.average_execution_time < Duration::from_millis(500) {
            PerformanceCategory::Good
        } else if metrics.average_effectiveness > 1.0 && metrics.average_execution_time < Duration::from_secs(2) {
            PerformanceCategory::Fair
        } else {
            PerformanceCategory::Poor
        }
    }
    
    /// Generate recommendations for optimization strategy
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Find most effective passes
        let mut pass_effectiveness: Vec<_> = self.metrics
            .iter()
            .map(|(name, metrics)| (name, metrics.average_effectiveness))
            .collect();
        pass_effectiveness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        if let Some((best_pass, effectiveness)) = pass_effectiveness.first() {
            if *effectiveness > 5.0 {
                recommendations.push(format!("Pass '{}' shows excellent performance (effectiveness: {:.2}). Consider running it earlier in the pipeline.", best_pass, effectiveness));
            }
        }
        
        // Find slowest passes
        let mut pass_times: Vec<_> = self.metrics
            .iter()
            .map(|(name, metrics)| (name, metrics.average_execution_time))
            .collect();
        pass_times.sort_by(|a, b| b.1.cmp(&a.1));
        
        if let Some((slowest_pass, time)) = pass_times.first() {
            if time.as_millis() > 1000 {
                recommendations.push(format!("Pass '{}' has high execution time ({:?}). Consider optimizing or running conditionally.", slowest_pass, time));
            }
        }
        
        // Check for ineffective passes
        for (pass_name, metrics) in &self.metrics {
            if metrics.average_effectiveness < 0.5 && metrics.execution_count > 5 {
                recommendations.push(format!("Pass '{}' shows low effectiveness ({:.2}). Consider removing or reconfiguring.", pass_name, metrics.average_effectiveness));
            }
        }
        
        // Overall optimization level recommendations
        let overall_effectiveness: f64 = self.metrics.values()
            .map(|m| m.average_effectiveness)
            .sum::<f64>() / self.metrics.len() as f64;
            
        if overall_effectiveness < 2.0 {
            recommendations.push("Overall optimization effectiveness is low. Consider increasing optimization level or enabling more aggressive passes.".to_string());
        } else if overall_effectiveness > 10.0 {
            recommendations.push("Excellent optimization effectiveness. Current configuration is performing well.".to_string());
        }
        
        recommendations
    }
    
    /// Generate recommendations for a specific pass
    fn generate_pass_recommendations(&self, pass_name: &str, metrics: &OptimizationMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if metrics.average_effectiveness < 1.0 {
            recommendations.push("Low effectiveness - consider disabling or reconfiguring".to_string());
        }
        
        if metrics.average_execution_time > Duration::from_secs(1) {
            recommendations.push("High execution time - consider conditional execution".to_string());
        }
        
        if metrics.success_rate < 0.9 {
            recommendations.push("Low success rate - investigate error conditions".to_string());
        }
        
        if metrics.code_size_reduction_percentage > 10.0 {
            recommendations.push("Excellent code size reduction - maintain current configuration".to_string());
        }
        
        recommendations
    }
    
    /// Compare current performance with baseline
    fn compare_with_baseline(&self, baseline: &BaselineMetrics) -> BaselineComparison {
        let mut comparison = BaselineComparison::default();
        
        // Calculate current metrics
        let current_total_time: Duration = self.execution_history
            .iter()
            .map(|stats| stats.execution_time)
            .sum();
            
        let current_optimizations: usize = self.execution_history
            .iter()
            .map(|stats| stats.optimizations_applied)
            .sum();
        
        // Compare execution time
        if current_total_time < baseline.total_execution_time {
            comparison.execution_time_change = -(baseline.total_execution_time.as_millis() as i64 - current_total_time.as_millis() as i64);
            comparison.execution_time_improvement = true;
        } else {
            comparison.execution_time_change = current_total_time.as_millis() as i64 - baseline.total_execution_time.as_millis() as i64;
            comparison.execution_time_improvement = false;
        }
        
        // Compare optimizations applied
        comparison.optimization_change = current_optimizations as i64 - baseline.total_optimizations as i64;
        comparison.optimization_improvement = comparison.optimization_change > 0;
        
        // Calculate improvement percentage
        if baseline.total_execution_time.as_millis() > 0 {
            comparison.improvement_percentage = 
                (comparison.execution_time_change as f64 / baseline.total_execution_time.as_millis() as f64) * 100.0;
        }
        
        comparison
    }
    
    /// Reset monitoring data
    pub fn reset(&mut self) {
        info!("Resetting performance monitoring data");
        self.metrics.clear();
        self.execution_history.clear();
        self.baseline_metrics = None;
        self.analysis_engine.reset();
    }
    
    /// Get recent execution statistics
    pub fn get_recent_executions(&self, count: usize) -> Vec<&PassExecutionStats> {
        self.execution_history
            .iter()
            .rev()
            .take(count)
            .collect()
    }
    
    /// Check if a pass is performing below threshold
    pub fn is_underperforming(&self, pass_name: &str) -> bool {
        if let Some(metrics) = self.metrics.get(pass_name) {
            metrics.average_effectiveness < self.monitoring_config.effectiveness_threshold &&
            metrics.execution_count >= self.monitoring_config.min_executions_for_analysis
        } else {
            false
        }
    }
}

/// Configuration for performance monitoring
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub history_size: usize,
    pub enable_trend_analysis: bool,
    pub effectiveness_threshold: f64,
    pub min_executions_for_analysis: usize,
    pub enable_baseline_comparison: bool,
    pub report_generation_interval: Duration,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            history_size: 1000,
            enable_trend_analysis: true,
            effectiveness_threshold: 1.0,
            min_executions_for_analysis: 5,
            enable_baseline_comparison: true,
            report_generation_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Optimization metrics for a specific pass
#[derive(Debug, Clone)]
pub struct OptimizationMetrics {
    pub execution_count: usize,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub total_optimizations_applied: usize,
    pub average_effectiveness: f64,
    pub success_rate: f64,
    pub code_size_reduction_percentage: f64,
    pub memory_usage_reduction_percentage: f64,
    pub last_execution: Option<SystemTime>,
}

impl OptimizationMetrics {
    /// Create new optimization metrics
    pub fn new() -> Self {
        Self {
            execution_count: 0,
            total_execution_time: Duration::from_secs(0),
            average_execution_time: Duration::from_secs(0),
            total_optimizations_applied: 0,
            average_effectiveness: 0.0,
            success_rate: 0.0,
            code_size_reduction_percentage: 0.0,
            memory_usage_reduction_percentage: 0.0,
            last_execution: None,
        }
    }
    
    /// Update metrics with new execution statistics
    pub fn update(&mut self, stats: &PassExecutionStats) {
        self.execution_count += 1;
        self.total_execution_time += stats.execution_time;
        self.average_execution_time = self.total_execution_time / self.execution_count as u32;
        self.total_optimizations_applied += stats.optimizations_applied;
        
        // Update effectiveness (running average)
        self.average_effectiveness = 
            (self.average_effectiveness * (self.execution_count - 1) as f64 + stats.effectiveness_score) 
            / self.execution_count as f64;
        
        // Update success rate
        let successful_executions = if stats.errors == 0 { 1 } else { 0 };
        self.success_rate = 
            (self.success_rate * (self.execution_count - 1) as f64 + successful_executions as f64) 
            / self.execution_count as f64;
        
        // Calculate code size reduction
        if stats.before_metrics.code_size > 0 {
            let reduction = (stats.before_metrics.code_size.saturating_sub(stats.after_metrics.code_size)) as f64;
            let percentage = (reduction / stats.before_metrics.code_size as f64) * 100.0;
            self.code_size_reduction_percentage = 
                (self.code_size_reduction_percentage * (self.execution_count - 1) as f64 + percentage) 
                / self.execution_count as f64;
        }
        
        self.last_execution = Some(stats.timestamp);
    }
}

/// Code metrics for before/after comparison
#[derive(Debug, Clone)]
pub struct CodeMetrics {
    pub code_size: usize,
    pub instruction_count: usize,
    pub function_count: usize,
    pub basic_block_count: usize,
    pub memory_allocations: usize,
    pub complexity_score: f64,
}

impl Default for CodeMetrics {
    fn default() -> Self {
        Self {
            code_size: 0,
            instruction_count: 0,
            function_count: 0,
            basic_block_count: 0,
            memory_allocations: 0,
            complexity_score: 0.0,
        }
    }
}

/// Statistics for a single pass execution
#[derive(Debug, Clone)]
pub struct PassExecutionStats {
    pub pass_name: String,
    pub timestamp: SystemTime,
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub optimizations_applied: usize,
    pub before_metrics: CodeMetrics,
    pub after_metrics: CodeMetrics,
    pub effectiveness_score: f64,
    pub changed: bool,
    pub errors: usize,
}

/// Baseline metrics for comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub total_execution_time: Duration,
    pub total_optimizations: usize,
    pub code_size: usize,
    pub compilation_timestamp: SystemTime,
}

/// Performance analysis engine
#[derive(Debug)]
pub struct AnalysisEngine {
    trends: HashMap<String, TrendData>,
}

impl AnalysisEngine {
    /// Create new analysis engine
    pub fn new() -> Self {
        Self {
            trends: HashMap::new(),
        }
    }
    
    /// Analyze trends for a pass
    pub fn analyze_trends(&mut self, pass_name: &str, history: &VecDeque<PassExecutionStats>) {
        let pass_history: Vec<_> = history
            .iter()
            .filter(|stats| stats.pass_name == pass_name)
            .collect();
            
        if pass_history.len() < 3 {
            return; // Need at least 3 data points for trend analysis
        }
        
        let mut trend_data = TrendData::default();
        
        // Analyze effectiveness trend
        let effectiveness_values: Vec<f64> = pass_history
            .iter()
            .map(|stats| stats.effectiveness_score)
            .collect();
        trend_data.effectiveness_trend = self.calculate_trend(&effectiveness_values);
        
        // Analyze execution time trend
        let time_values: Vec<f64> = pass_history
            .iter()
            .map(|stats| stats.execution_time.as_millis() as f64)
            .collect();
        trend_data.execution_time_trend = self.calculate_trend(&time_values);
        
        self.trends.insert(pass_name.to_string(), trend_data);
    }
    
    /// Calculate trend direction and magnitude
    fn calculate_trend(&self, values: &[f64]) -> TrendDirection {
        if values.len() < 2 {
            return TrendDirection::Stable;
        }
        
        let first_half = &values[..values.len() / 2];
        let second_half = &values[values.len() / 2..];
        
        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;
        
        let change_percentage = ((second_avg - first_avg) / first_avg) * 100.0;
        
        if change_percentage > 10.0 {
            TrendDirection::Improving
        } else if change_percentage < -10.0 {
            TrendDirection::Degrading
        } else {
            TrendDirection::Stable
        }
    }
    
    /// Get trend summary
    pub fn get_trend_summary(&self) -> HashMap<String, TrendData> {
        self.trends.clone()
    }
    
    /// Reset trend data
    pub fn reset(&mut self) {
        self.trends.clear();
    }
}

/// Trend data for a pass
#[derive(Debug, Clone, Default)]
pub struct TrendData {
    pub effectiveness_trend: TrendDirection,
    pub execution_time_trend: TrendDirection,
}

/// Trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
}

impl Default for TrendDirection {
    fn default() -> Self {
        TrendDirection::Stable
    }
}

/// Performance category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceCategory {
    Excellent,
    Good,
    Fair,
    Poor,
}

/// Comprehensive performance report
#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub timestamp: SystemTime,
    pub total_passes_monitored: usize,
    pub total_executions: usize,
    pub average_execution_time: Duration,
    pub average_effectiveness: f64,
    pub total_optimizations_applied: usize,
    pub pass_analyses: HashMap<String, PassPerformanceAnalysis>,
    pub recommendations: Vec<String>,
    pub baseline_comparison: Option<BaselineComparison>,
    pub trends: HashMap<String, TrendData>,
}

/// Performance analysis for a single pass
#[derive(Debug, Default)]
pub struct PassPerformanceAnalysis {
    pub total_executions: usize,
    pub average_execution_time: Duration,
    pub total_optimizations: usize,
    pub average_effectiveness: f64,
    pub success_rate: f64,
    pub optimizations_per_ms: f64,
    pub performance_category: PerformanceCategory,
    pub recommendations: Vec<String>,
}

impl Default for PerformanceCategory {
    fn default() -> Self {
        PerformanceCategory::Fair
    }
}

/// Comparison with baseline metrics
#[derive(Debug, Default)]
pub struct BaselineComparison {
    pub execution_time_change: i64, // milliseconds
    pub execution_time_improvement: bool,
    pub optimization_change: i64,
    pub optimization_improvement: bool,
    pub improvement_percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor_creation() {
        let config = MonitoringConfig::default();
        let monitor = PerformanceMonitor::new(config);
        
        assert_eq!(monitor.metrics.len(), 0);
        assert_eq!(monitor.execution_history.len(), 0);
        assert!(monitor.baseline_metrics.is_none());
    }
    
    #[test]
    fn test_optimization_metrics_update() {
        let mut metrics = OptimizationMetrics::new();
        
        let stats = PassExecutionStats {
            pass_name: "test_pass".to_string(),
            timestamp: SystemTime::now(),
            execution_time: Duration::from_millis(100),
            memory_usage: 1024,
            optimizations_applied: 5,
            before_metrics: CodeMetrics { code_size: 1000, ..Default::default() },
            after_metrics: CodeMetrics { code_size: 900, ..Default::default() },
            effectiveness_score: 2.5,
            changed: true,
            errors: 0,
        };
        
        metrics.update(&stats);
        
        assert_eq!(metrics.execution_count, 1);
        assert_eq!(metrics.total_execution_time, Duration::from_millis(100));
        assert_eq!(metrics.average_effectiveness, 2.5);
        assert_eq!(metrics.success_rate, 1.0);
        assert_eq!(metrics.code_size_reduction_percentage, 10.0);
    }
    
    #[test]
    fn test_trend_analysis() {
        let mut engine = AnalysisEngine::new();
        let mut history = VecDeque::new();
        
        // Add some test data with improving trend
        for i in 0..5 {
            history.push_back(PassExecutionStats {
                pass_name: "test_pass".to_string(),
                timestamp: SystemTime::now(),
                execution_time: Duration::from_millis(100),
                memory_usage: 1024,
                optimizations_applied: 5,
                before_metrics: CodeMetrics::default(),
                after_metrics: CodeMetrics::default(),
                effectiveness_score: i as f64 + 1.0, // Improving trend
                changed: true,
                errors: 0,
            });
        }
        
        engine.analyze_trends("test_pass", &history);
        
        let trends = engine.get_trend_summary();
        assert!(trends.contains_key("test_pass"));
    }
    
    #[test]
    fn test_performance_categorization() {
        let monitor = PerformanceMonitor::new(MonitoringConfig::default());
        
        let mut excellent_metrics = OptimizationMetrics::new();
        excellent_metrics.average_effectiveness = 15.0;
        excellent_metrics.average_execution_time = Duration::from_millis(50);
        
        let category = monitor.categorize_pass_performance(&excellent_metrics);
        assert_eq!(category, PerformanceCategory::Excellent);
        
        let mut poor_metrics = OptimizationMetrics::new();
        poor_metrics.average_effectiveness = 0.1;
        poor_metrics.average_execution_time = Duration::from_secs(5);
        
        let category = monitor.categorize_pass_performance(&poor_metrics);
        assert_eq!(category, PerformanceCategory::Poor);
    }
    
    #[test]
    fn test_baseline_comparison() {
        let monitor = PerformanceMonitor::new(MonitoringConfig::default());
        
        let baseline = BaselineMetrics {
            total_execution_time: Duration::from_millis(1000),
            total_optimizations: 100,
            code_size: 10000,
            compilation_timestamp: SystemTime::now(),
        };
        
        let comparison = monitor.compare_with_baseline(&baseline);
        
        // With empty execution history, should show no change
        assert_eq!(comparison.execution_time_change, -1000);
        assert_eq!(comparison.optimization_change, -100);
    }
    
    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        
        assert_eq!(config.history_size, 1000);
        assert!(config.enable_trend_analysis);
        assert_eq!(config.effectiveness_threshold, 1.0);
        assert_eq!(config.min_executions_for_analysis, 5);
    }
}
