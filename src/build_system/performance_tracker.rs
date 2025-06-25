/// Build Performance Tracking and Analytics
/// 
/// Provides comprehensive performance tracking, compilation metrics,
/// and build time analysis for the CURSED build system.

use crate::error::{CursedError, Result};

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn, error};
use serde::{Serialize, Deserialize};

/// Comprehensive build performance tracker
pub struct BuildPerformanceTracker {
impl BuildPerformanceTracker {
    /// Create a new performance tracker
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
        }
    }
    
    /// Start tracking a new build
    #[instrument(skip(self))]
    pub fn start_build(&self, build_id: String, target: String) -> Result<()> {
        let mut current = self.current_build.lock().unwrap();
        
        if current.is_some() {
            warn!("Starting new build while another is in progress");
        *current = Some(CurrentBuildTracking {
        });
        
        self.resource_monitor.start_monitoring()?;
        
        info!("Started build tracking for build: {}", build_id);
        Ok(())
    /// End build tracking and record results
    #[instrument(skip(self))]
    pub fn end_build(&self) -> Result<BuildPerformanceReport> {
        let mut current = self.current_build.lock().unwrap();
        
        let tracking = current.take()
            .ok_or_else(|| CursedError::Internal("No build currently being tracked".to_string()))?;
        
        let total_duration = tracking.start_time.elapsed();
        
        // Stop resource monitoring
        let resource_stats = self.resource_monitor.stop_monitoring()?;
        
        // Create build record
        let build_record = BuildRecord {
            cache_hit_rate: if tracking.cache_hits + tracking.cache_misses > 0 {
                tracking.cache_hits as f64 / (tracking.cache_hits + tracking.cache_misses) as f64
            } else {
                0.0
            compilation_speed: if total_duration.as_secs_f64() > 0.0 {
                tracking.lines_compiled as f64 / total_duration.as_secs_f64()
            } else {
                0.0
        
        // Store build record
        {
            let mut history = self.build_history.write().unwrap();
            history.push_back(build_record.clone());
            
            // Maintain history size limit
            if history.len() > self.config.max_build_history {
                history.pop_front();
            }
        }
        
        // Check for performance regressions
        let regression_analysis = self.regression_detector.analyze(&build_record)?;
        
        // Generate performance report
        let report = BuildPerformanceReport {
        
              tracking.build_id, total_duration);
        
        Ok(report)
    /// Record the start of a build phase
    #[instrument(skip(self))]
    pub fn start_phase(&self, phase_name: String) -> Result<()> {
        let mut current = self.current_build.lock().unwrap();
        
        if let Some(ref mut tracking) = current.as_mut() {
            tracking.phases.insert(phase_name.clone(), PhaseTracking {
            });
            
            debug!("Started tracking phase: {}", phase_name);
        } else {
            warn!("Attempted to start phase without active build tracking");
        Ok(())
    /// Record the end of a build phase
    #[instrument(skip(self))]
    pub fn end_phase(&self, phase_name: String) -> Result<PhaseMetrics> {
        let mut current = self.current_build.lock().unwrap();
        
        if let Some(ref mut tracking) = current.as_mut() {
            if let Some(phase_tracking) = tracking.phases.get_mut(&phase_name) {
                let duration = phase_tracking.start_time.elapsed();
                phase_tracking.duration = Some(duration);
                
                let metrics = PhaseMetrics {
                    throughput: if duration.as_secs_f64() > 0.0 {
                        phase_tracking.files_processed as f64 / duration.as_secs_f64()
                    } else {
                        0.0
                
                // Store in phase metrics history
                {
                    let mut phase_metrics = self.phase_metrics.write().unwrap();
                    phase_metrics.insert(phase_name.clone(), metrics.clone());
                debug!("Completed tracking phase: {} (duration: {:?})", phase_name, duration);
                return Ok(metrics);
            }
        }
        
        Err(CursedError::Internal(format!("Phase not found or no active build: {}", phase_name)))
    /// Record file compilation
    #[instrument(skip(self))]
    pub fn record_file_compilation(
    ) -> Result<()> {
        // Update current build tracking
        {
            let mut current = self.current_build.lock().unwrap();
            if let Some(ref mut tracking) = current.as_mut() {
                tracking.files_compiled += 1;
                tracking.lines_compiled += lines_of_code;
                
                if !success {
                    tracking.errors += 1;
                // Update current phase if active
                if let Some((_, phase)) = tracking.phases.iter_mut()
                    .find(|(_, p)| p.duration.is_none()) {
                    phase.files_processed += 1;
                    if !success {
                        phase.errors += 1;
                    }
                }
            }
        }
        
        // Update file compilation metrics
        {
            let mut file_metrics = self.file_compilation_metrics.write().unwrap();
            let metrics = file_metrics.entry(file_path.clone()).or_insert_with(|| {
                FileCompilationMetrics {
                }
            });
            
            metrics.compilation_history.push_back(CompilationRecord {
                cache_hit: false, // Would be determined by cache system
                optimization_level: "O2".to_string(), // Would come from build config
            });
            
            // Maintain history size
            if metrics.compilation_history.len() > self.config.max_file_history {
                metrics.compilation_history.pop_front();
            // Update averages
            metrics.compilation_count += 1;
            let total_time: Duration = metrics.compilation_history.iter()
                .map(|r| r.compilation_time)
                .sum();
            metrics.average_compilation_time = total_time / metrics.compilation_history.len() as u32;
               file_path, compilation_time, lines_of_code);
        
        Ok(())
    /// Record cache hit/miss
    pub fn record_cache_event(&self, hit: bool) -> Result<()> {
        let mut current = self.current_build.lock().unwrap();
        if let Some(ref mut tracking) = current.as_mut() {
            if hit {
                tracking.cache_hits += 1;
            } else {
                tracking.cache_misses += 1;
            }
        }
        Ok(())
    /// Record dependency resolution
    pub fn record_dependency_resolution(&self, count: usize) -> Result<()> {
        let mut current = self.current_build.lock().unwrap();
        if let Some(ref mut tracking) = current.as_mut() {
            tracking.dependencies_resolved += count;
        }
        Ok(())
    /// Generate comprehensive performance statistics
    pub fn get_performance_statistics(&self) -> Result<PerformanceStatistics> {
        let history = self.build_history.read().unwrap();
        let file_metrics = self.file_compilation_metrics.read().unwrap();
        let phase_metrics = self.phase_metrics.read().unwrap();
        
        if history.is_empty() {
            return Ok(PerformanceStatistics::default());
        let recent_builds: Vec<_> = history.iter().rev().take(10).collect();
        
        let average_build_time = recent_builds.iter()
            .map(|b| b.total_duration)
            .sum::<Duration>() / recent_builds.len() as u32;
        
        let average_compilation_speed = recent_builds.iter()
            .map(|b| b.compilation_speed)
            .sum::<f64>() / recent_builds.len() as f64;
        
        let average_cache_hit_rate = recent_builds.iter()
            .map(|b| b.cache_hit_rate)
            .sum::<f64>() / recent_builds.len() as f64;
        
        let build_success_rate = recent_builds.iter()
            .filter(|b| b.success)
            .count() as f64 / recent_builds.len() as f64;
        
        Ok(PerformanceStatistics {
        })
    /// Calculate resource efficiency
    fn calculate_resource_efficiency(
    ) -> f64 {
        // Base efficiency calculation
        let cpu_efficiency = if resource_stats.peak_cpu_percent > 0.0 {
            resource_stats.average_cpu_percent / resource_stats.peak_cpu_percent
        } else {
            0.0
        
        let memory_efficiency = if resource_stats.peak_memory_mb > 0.0 {
            resource_stats.average_memory_mb / resource_stats.peak_memory_mb
        } else {
            0.0
        
        let parallel_efficiency = if tracking.parallel_jobs > 1 {
            // Estimate parallel efficiency based on actual vs theoretical speedup
            let theoretical_speedup = tracking.parallel_jobs as f64;
            let actual_efficiency = (tracking.files_compiled as f64 / tracking.start_time.elapsed().as_secs_f64()) 
                / (tracking.files_compiled as f64 / (tracking.start_time.elapsed().as_secs_f64() * tracking.parallel_jobs as f64));
            (actual_efficiency / theoretical_speedup).min(1.0)
        } else {
            1.0
        
        // Weighted average
        (cpu_efficiency * 0.4 + memory_efficiency * 0.3 + parallel_efficiency * 0.3).max(0.0).min(1.0)
    /// Get compilation breakdown by file type, size, etc.
    fn get_compilation_breakdown(&self) -> CompilationBreakdown {
        let file_metrics = self.file_compilation_metrics.read().unwrap();
        
        let mut by_extension = HashMap::new();
        let mut by_size_category = HashMap::new();
        let mut total_compilation_time = Duration::from_secs(0);
        
        for metrics in file_metrics.values() {
            // Group by file extension
            if let Some(ext) = metrics.file_path.extension().and_then(|e| e.to_str()) {
                let entry = by_extension.entry(ext.to_string()).or_insert_with(|| CompilationCategoryStats {
                });
                
                entry.file_count += 1;
                entry.total_time += metrics.average_compilation_time;
                entry.total_lines += metrics.compilation_history.iter()
                    .map(|r| r.lines_of_code)
                    .sum::<usize>();
            // Group by complexity/size
            let category = if metrics.complexity_score < 100.0 {
                "simple"
            } else if metrics.complexity_score < 500.0 {
                "medium"
            } else if metrics.complexity_score < 1000.0 {
                "complex"
            } else {
                "very_complex"
            
            let entry = by_size_category.entry(category.to_string()).or_insert_with(|| CompilationCategoryStats {
            });
            
            entry.file_count += 1;
            entry.total_time += metrics.average_compilation_time;
            total_compilation_time += metrics.average_compilation_time;
        // Calculate averages
        for stats in by_extension.values_mut() {
            if stats.file_count > 0 {
                stats.average_time = stats.total_time / stats.file_count as u32;
            }
        }
        
        for stats in by_size_category.values_mut() {
            if stats.file_count > 0 {
                stats.average_time = stats.total_time / stats.file_count as u32;
            }
        }
        
        CompilationBreakdown {
        }
    }
    
    /// Analyze phase performance
    fn analyze_phases(&self) -> PhaseAnalysis {
        let phase_metrics = self.phase_metrics.read().unwrap();
        let history = self.build_history.read().unwrap();
        
        let mut phase_summary = HashMap::new();
        let mut bottlenecks = Vec::new();
        
        // Analyze each phase
        for (phase_name, metrics) in phase_metrics.iter() {
            phase_summary.insert(phase_name.clone(), PhaseSummary {
                success_rate: if metrics.errors > 0 { 0.8 } else { 1.0 }, // Simplified calculation
            });
            
            // Identify bottlenecks (phases taking > 20% of total build time)
            if let Some(recent_build) = history.back() {
                let phase_percentage = metrics.duration.as_secs_f64() / recent_build.total_duration.as_secs_f64();
                if phase_percentage > 0.2 {
                    bottlenecks.push(PhaseBottleneck {
                    });
                }
            }
        PhaseAnalysis {
        }
    }
    
    /// Generate optimization suggestions
    fn generate_optimization_suggestions(&self) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();
        let history = self.build_history.read().unwrap();
        
        if let Some(recent_build) = history.back() {
            // Cache hit rate suggestions
            if recent_build.cache_hit_rate < 0.7 {
                suggestions.push(OptimizationSuggestion {
                    description: format!(
                        recent_build.cache_hit_rate * 100.0
                    estimated_improvement: Duration::from_secs(
                        (recent_build.total_duration.as_secs() as f64 * 0.3) as u64
                });
            // Parallelization suggestions
            if recent_build.parallel_jobs == 1 && recent_build.files_compiled > 10 {
                suggestions.push(OptimizationSuggestion {
                    estimated_improvement: Duration::from_secs(
                        (recent_build.total_duration.as_secs() as f64 * 0.5) as u64
                });
            // Memory suggestions
            if recent_build.peak_memory_mb > 4000.0 {
                suggestions.push(OptimizationSuggestion {
                    description: format!(
                        recent_build.peak_memory_mb
                });
            }
        }
        
        suggestions
    /// Generate historical comparison
    fn generate_historical_comparison(&self) -> HistoricalComparison {
        let history = self.build_history.read().unwrap();
        
        if history.len() < 2 {
            return HistoricalComparison::default();
        let recent = history.back().unwrap();
        let baseline = if history.len() >= 10 {
            // Use average of builds 5-10 ago as baseline
            let baseline_builds: Vec<_> = history.iter().rev().skip(5).take(5).collect();
            let avg_time = baseline_builds.iter()
                .map(|b| b.total_duration)
                .sum::<Duration>() / baseline_builds.len() as u32;
            avg_time
        } else {
            history.front().unwrap().total_duration
        
        let time_change = if baseline.as_secs() > 0 {
            ((recent.total_duration.as_secs_f64() - baseline.as_secs_f64()) / baseline.as_secs_f64()) * 100.0
        } else {
            0.0
        
        HistoricalComparison {
            performance_trend: if time_change < -5.0 {
                PerformanceTrend::Improving
            } else if time_change > 5.0 {
                PerformanceTrend::Degrading
            } else {
                PerformanceTrend::Stable
        }
    }
    
    // Helper methods
    
    fn get_most_compiled_files(&self, file_metrics: &HashMap<PathBuf, FileCompilationMetrics>) -> Vec<PathBuf> {
        let mut files: Vec<_> = file_metrics.iter().collect();
        files.sort_by_key(|(_, metrics)| std::cmp::Reverse(metrics.compilation_count));
        files.into_iter().take(10).map(|(path, _)| path.clone()).collect()
    fn get_slowest_files(&self, file_metrics: &HashMap<PathBuf, FileCompilationMetrics>) -> Vec<PathBuf> {
        let mut files: Vec<_> = file_metrics.iter().collect();
        files.sort_by_key(|(_, metrics)| std::cmp::Reverse(metrics.average_compilation_time));
        files.into_iter().take(10).map(|(path, _)| path.clone()).collect()
    fn get_phase_performance_summary(&self, phase_metrics: &HashMap<String, PhaseMetrics>) -> HashMap<String, f64> {
        phase_metrics.iter()
            .map(|(name, metrics)| (name.clone(), metrics.throughput))
            .collect()
    fn get_resource_utilization_summary(&self, builds: &[&BuildRecord]) -> ResourceUtilizationSummary {
        if builds.is_empty() {
            return ResourceUtilizationSummary::default();
        let avg_cpu = builds.iter().map(|b| b.average_cpu_percent).sum::<f64>() / builds.len() as f64;
        let avg_memory = builds.iter().map(|b| b.average_memory_mb).sum::<f64>() / builds.len() as f64;
        let peak_cpu = builds.iter().map(|b| b.peak_cpu_percent).fold(0.0, |a, b| a.max(*b));
        let peak_memory = builds.iter().map(|b| b.peak_memory_mb).fold(0.0, |a, b| a.max(*b));
        
        ResourceUtilizationSummary {
            efficiency_score: (avg_cpu / peak_cpu.max(1.0) + avg_memory / peak_memory.max(1.0)) / 2.0,
        }
    }
    
    fn suggest_phase_optimizations(&self, phase_name: &str, metrics: &PhaseMetrics) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match phase_name {
            "parsing" => {
                if metrics.throughput < 100.0 {
                    suggestions.push("Consider parallel parsing of independent files".to_string());
                }
            }
            "type_checking" => {
                if metrics.duration > Duration::from_secs(10) {
                    suggestions.push("Incremental type checking could reduce build times".to_string());
                }
            }
            "optimization" => {
                if metrics.duration > Duration::from_secs(30) {
                    suggestions.push("Consider reducing optimization level for debug builds".to_string());
                }
            }
            "linking" => {
                if metrics.duration > Duration::from_secs(5) {
                    suggestions.push("LTO linking can be slow; consider disabling for debug builds".to_string());
                }
            }
            _ => {}
        }
        
        suggestions
    fn identify_critical_path(&self) -> Vec<String> {
        // Simplified critical path identification
        let phase_metrics = self.phase_metrics.read().unwrap();
        let mut phases: Vec<_> = phase_metrics.iter().collect();
        phases.sort_by_key(|(_, metrics)| std::cmp::Reverse(metrics.duration));
        phases.into_iter().take(3).map(|(name, _)| name.clone()).collect()
    fn identify_parallelization_opportunities(&self) -> Vec<String> {
        let mut opportunities = Vec::new();
        let phase_metrics = self.phase_metrics.read().unwrap();
        
        // Identify phases that could benefit from parallelization
        for (phase_name, metrics) in phase_metrics.iter() {
            if metrics.duration > Duration::from_secs(5) && metrics.files_processed > 10 {
                                          phase_name, metrics.files_processed));
            }
        }
        
        opportunities
    }
}

// Data structures

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
        }
    }
#[derive(Debug, Clone)]
pub struct CurrentBuildTracking {
#[derive(Debug, Clone)]
pub struct PhaseTracking {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRecord {
    pub compilation_speed: f64, // lines per second
#[derive(Debug, Clone)]
pub struct FileCompilationMetrics {
#[derive(Debug, Clone)]
pub struct CompilationRecord {
#[derive(Debug, Clone)]
pub struct PhaseMetrics {
    pub throughput: f64, // files per second
#[derive(Debug, Clone, Default)]
pub struct PerformanceStatistics {
#[derive(Debug, Clone)]
pub struct BuildPerformanceReport {
#[derive(Debug, Clone)]
pub struct CompilationBreakdown {
#[derive(Debug, Clone)]
pub struct CompilationCategoryStats {
#[derive(Debug, Clone)]
pub struct PhaseAnalysis {
#[derive(Debug, Clone)]
pub struct PhaseSummary {
#[derive(Debug, Clone)]
pub struct PhaseBottleneck {
#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
#[derive(Debug, Clone)]
pub enum EffortLevel {
#[derive(Debug, Clone, Default)]
pub struct HistoricalComparison {
#[derive(Debug, Clone, Default)]
pub enum PerformanceTrend {
    #[default]
#[derive(Debug, Clone, Default)]
pub struct ResourceUtilizationSummary {
// Resource monitoring and regression detection would be implemented as separate modules
// with platform-specific implementations

/// Resource monitoring for system metrics
pub struct ResourceMonitor {
    // Implementation would include platform-specific monitoring
impl ResourceMonitor {
    pub fn new(config: PerformanceConfig) -> Self {
        Self { config }
    }
    
    pub fn start_monitoring(&self) -> Result<()> {
        // Start background monitoring thread
        Ok(())
    pub fn stop_monitoring(&self) -> Result<ResourceStatistics> {
        // Stop monitoring and return collected stats
        Ok(ResourceStatistics {
        })
    pub fn get_current_memory_mb(&self) -> f64 {
        // Get current memory usage
        512.0 // Placeholder
    pub fn get_current_cpu_percent(&self) -> f64 {
        // Get current CPU usage
        45.0 // Placeholder
    }
}

#[derive(Debug, Clone)]
pub struct ResourceStatistics {
/// Regression detection for performance monitoring
pub struct RegressionDetector {
impl RegressionDetector {
    pub fn new(config: PerformanceConfig) -> Self {
        Self { config }
    }
    
    pub fn analyze(&self, build_record: &BuildRecord) -> Result<RegressionAnalysis> {
        // Implement regression detection logic
        Ok(RegressionAnalysis {
        })
    }
}

#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
#[derive(Debug, Clone)]
pub enum RegressionSeverity {
}
