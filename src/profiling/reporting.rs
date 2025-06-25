use crate::error::CursedError;
// Performance reporting and visualization generation

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

// use crate::profiling::core::{ProfileData, ProfilerError};
// use crate::profiling::cpu::{CpuProfileData, FlameGraph};
// use crate::profiling::memory::MemoryProfileData;
// use crate::profiling::concurrency::ConcurrencyProfileData;
// use crate::profiling::benchmarking::BenchmarkResults;

/// Performance report generator
#[derive(Debug)]
pub struct ReportGenerator {
impl ReportGenerator {
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }
    
    #[instrument(skip(self, profile_data))]
    pub fn generate_report(&self, profile_data: &ProfileData) -> crate::error::Result<()> {
        info!("Generating performance report for session: {}", profile_data.session_name);
        
        let mut report = PerformanceReport::new(
        );
        
        // Generate sections based on available data
        if let Some(cpu_data) = self.extract_cpu_data(profile_data)? {
            report.cpu_analysis = Some(self.generate_cpu_analysis(&cpu_data)?);
        if let Some(memory_data) = self.extract_memory_data(profile_data)? {
            report.memory_analysis = Some(self.generate_memory_analysis(&memory_data)?);
        if let Some(concurrency_data) = self.extract_concurrency_data(profile_data)? {
            report.concurrency_analysis = Some(self.generate_concurrency_analysis(&concurrency_data)?);
        // Generate summary
        report.summary = self.generate_summary(&report);
        
        info!("Performance report generated successfully");
        Ok(report)
    #[instrument(skip(self, benchmark_results))]
    pub fn generate_benchmark_report(&self, benchmark_results: &BenchmarkResults) -> crate::error::Result<()> {
        info!("Generating benchmark report for suite: {}", benchmark_results.suite_name);
        
        let report = BenchmarkReport {
        
        Ok(report)
    #[instrument(skip(self, report))]
    pub fn export_html(&self, report: &PerformanceReport, output_path: &str) -> crate::error::Result<()> {
        let html = self.generate_html_report(report)?;
        std::fs::write(output_path, html).map_err(ProfilerError::IoError)?;
        info!("HTML report exported to: {}", output_path);
        Ok(())
    #[instrument(skip(self, report))]
    pub fn export_markdown(&self, report: &PerformanceReport, output_path: &str) -> crate::error::Result<()> {
        let markdown = self.generate_markdown_report(report)?;
        std::fs::write(output_path, markdown).map_err(ProfilerError::IoError)?;
        info!("Markdown report exported to: {}", output_path);
        Ok(())
    #[instrument(skip(self, report))]
    pub fn export_json(&self, report: &PerformanceReport, output_path: &str) -> crate::error::Result<()> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        std::fs::write(output_path, json).map_err(ProfilerError::IoError)?;
        info!("JSON report exported to: {}", output_path);
        Ok(())
    /// Import a performance report from JSON file
    #[instrument(skip(self))]
    pub fn import_json(&self, input_path: &str) -> crate::error::Result<()> {
        let json = std::fs::read_to_string(input_path)
            .map_err(ProfilerError::IoError)?;
        let report: PerformanceReport = serde_json::from_str(&json)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        info!("JSON report imported from: {}", input_path);
        Ok(report)
    /// Import a benchmark report from JSON file
    #[instrument(skip(self))]
    pub fn import_benchmark_json(&self, input_path: &str) -> crate::error::Result<()> {
        let json = std::fs::read_to_string(input_path)
            .map_err(ProfilerError::IoError)?;
        let report: BenchmarkReport = serde_json::from_str(&json)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        info!("Benchmark report imported from: {}", input_path);
        Ok(report)
    /// Import a performance report from binary format
    #[instrument(skip(self))]
    pub fn import_binary(&self, input_path: &str) -> crate::error::Result<()> {
        let data = std::fs::read(input_path)
            .map_err(ProfilerError::IoError)?;
        let report: PerformanceReport = bincode::deserialize(&data)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        info!("Binary report imported from: {}", input_path);
        Ok(report)
    /// Export a performance report to binary format
    #[instrument(skip(self, report))]
    pub fn export_binary(&self, report: &PerformanceReport, output_path: &str) -> crate::error::Result<()> {
        let data = bincode::serialize(report)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        std::fs::write(output_path, data).map_err(ProfilerError::IoError)?;
        info!("Binary report exported to: {}", output_path);
        Ok(())
    fn extract_cpu_data(&self, profile_data: &ProfileData) -> crate::error::Result<()> {
        if let Some(data) = profile_data.get_mode_data(&crate::profiling::core::ProfilerMode::Cpu) {
            let cpu_data: CpuProfileData = bincode::deserialize(data)
                .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
            Ok(Some(cpu_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_memory_data(&self, profile_data: &ProfileData) -> crate::error::Result<()> {
        if let Some(data) = profile_data.get_mode_data(&crate::profiling::core::ProfilerMode::Memory) {
            let memory_data: MemoryProfileData = bincode::deserialize(data)
                .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
            Ok(Some(memory_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_concurrency_data(&self, profile_data: &ProfileData) -> crate::error::Result<()> {
        if let Some(data) = profile_data.get_mode_data(&crate::profiling::core::ProfilerMode::Concurrency) {
            let concurrency_data: ConcurrencyProfileData = bincode::deserialize(data)
                .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
            Ok(Some(concurrency_data))
        } else {
            Ok(None)
        }
    }
    
    fn generate_cpu_analysis(&self, cpu_data: &CpuProfileData) -> crate::error::Result<()> {
        let hot_functions = cpu_data.get_hot_functions(self.config.max_functions);
        let call_graph = cpu_data.get_call_graph();
        let flame_graph = FlameGraph::from_cpu_profile(cpu_data)?;
        
        Ok(CpuAnalysisReport {
            hot_functions: hot_functions.into_iter()
                .map(|(name, stats)| HotFunction {
                })
            call_graph_summary: CallGraphSummary {
            flame_graph: if self.config.include_flame_graphs {
                Some(flame_graph)
            } else {
                None
        })
    fn generate_memory_analysis(&self, memory_data: &MemoryProfileData) -> crate::error::Result<()> {
        let current_usage = memory_data.calculate_current_usage();
        let allocation_analysis = memory_data.analyze_patterns();
        let memory_leaks = memory_data.detect_leaks();
        
        Ok(MemoryAnalysisReport {
            memory_leaks: memory_leaks.iter()
                .take(self.config.max_memory_leaks)
                .map(SerializableMemoryLeak::from)
        })
    fn generate_concurrency_analysis(&self, concurrency_data: &ConcurrencyProfileData) -> crate::error::Result<()> {
        let goroutine_timeline = concurrency_data.generate_goroutine_timeline();
        let channel_analysis = concurrency_data.analyze_channels();
        let deadlocks = concurrency_data.detect_deadlocks();
        let scheduler_analysis = concurrency_data.analyze_scheduler();
        
        Ok(ConcurrencyAnalysisReport {
            goroutine_timeline: goroutine_timeline.iter()
                .take(self.config.max_goroutines)
                .map(SerializableGoroutineTimeline::from)
        })
    fn generate_summary(&self, report: &PerformanceReport) -> PerformanceSummary {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Analyze CPU performance
        if let Some(cpu_analysis) = &report.cpu_analysis {
            if cpu_analysis.hot_functions.len() > 0 {
                let top_function = &cpu_analysis.hot_functions[0];
                if top_function.percentage > 30.0 {
                    issues.push(format!(
                        top_function.name, top_function.percentage
                    ));
                    recommendations.push(
                        "Consider optimizing the most CPU-intensive function".to_string()
                    );
                }
            }
        // Analyze memory performance
        if let Some(memory_analysis) = &report.memory_analysis {
            if !memory_analysis.memory_leaks.is_empty() {
                issues.push(format!(
                    memory_analysis.memory_leaks.len()
                ));
                recommendations.push(
                    "Review memory leak analysis and fix allocation/deallocation patterns".to_string()
                );
            }
        }
        
        // Analyze concurrency performance
        if let Some(concurrency_analysis) = &report.concurrency_analysis {
            if !concurrency_analysis.deadlock_detections.is_empty() {
                issues.push(format!(
                    concurrency_analysis.deadlock_detections.len()
                ));
                recommendations.push(
                    "Review goroutine synchronization and channel usage".to_string()
                );
            }
        }
        
        let overall_score = self.calculate_performance_score(report);
        
        PerformanceSummary {
            execution_time: report.cpu_analysis
                .as_ref()
                .map(|cpu| cpu.total_duration)
            memory_usage: report.memory_analysis
                .as_ref()
                .map(|mem| mem.current_usage.allocated_bytes)
            concurrency_utilization: report.concurrency_analysis
                .as_ref()
                .map(|conc| conc.scheduler_analysis.scheduler_efficiency)
        }
    }
    
    fn calculate_performance_score(&self, report: &PerformanceReport) -> f64 {
        let mut score = 100.0;
        
        // Deduct points for CPU hotspots
        if let Some(cpu_analysis) = &report.cpu_analysis {
            if let Some(top_function) = cpu_analysis.hot_functions.first() {
                if top_function.percentage > 50.0 {
                    score -= 20.0;
                } else if top_function.percentage > 30.0 {
                    score -= 10.0;
                }
            }
        // Deduct points for memory issues
        if let Some(memory_analysis) = &report.memory_analysis {
            let leak_penalty = (memory_analysis.memory_leaks.len() as f64) * 5.0;
            score -= leak_penalty.min(30.0);
        // Deduct points for concurrency issues
        if let Some(concurrency_analysis) = &report.concurrency_analysis {
            let deadlock_penalty = (concurrency_analysis.deadlock_detections.len() as f64) * 15.0;
            score -= deadlock_penalty.min(40.0);
            
            let efficiency = concurrency_analysis.scheduler_analysis.scheduler_efficiency;
            if efficiency < 0.8 {
                score -= (0.8 - efficiency) * 50.0;
            }
        }
        
        score.max(0.0).min(100.0)
    fn calculate_max_call_depth(&self, call_graph: &crate::profiling::cpu::CallGraph) -> usize {
        // Simplified depth calculation
        call_graph.edges.len()
    fn generate_cpu_insights(&self, _cpu_data: &CpuProfileData) -> Vec<String> {
        vec![
        ]
    fn generate_memory_insights(&self, _memory_data: &MemoryProfileData) -> Vec<String> {
        vec![
        ]
    fn generate_concurrency_insights(&self, _concurrency_data: &ConcurrencyProfileData) -> Vec<String> {
        vec![
        ]
    fn analyze_gc_performance(&self, memory_data: &MemoryProfileData) -> GcPerformanceAnalysis {
        let total_collections = memory_data.gc_events.len() as u64;
        let total_gc_time: std::time::Duration = memory_data.gc_events
            .iter()
            .map(|event| event.duration)
            .sum();
        
        let average_pause_time = if total_collections > 0 {
            total_gc_time / total_collections as u32
        } else {
            std::time::Duration::default()
        
        let total_bytes_collected: usize = memory_data.gc_events
            .iter()
            .map(|event| event.bytes_collected)
            .sum();
        
        GcPerformanceAnalysis {
            gc_efficiency: if total_gc_time.as_millis() > 0 {
                total_bytes_collected as f64 / total_gc_time.as_millis() as f64
            } else {
                0.0
        }
    }
    
    fn generate_performance_insights(&self, _benchmark_results: &BenchmarkResults) -> Vec<String> {
        vec![
        ]
    fn generate_recommendations(&self, _benchmark_results: &BenchmarkResults) -> Vec<String> {
        vec![
        ]
    fn generate_html_report(&self, report: &PerformanceReport) -> crate::error::Result<()> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html><html><head>");
        html.push_str("<title>CURSED Performance Report</title>");
        html.push_str("<style>");
        html.push_str(include_str!("templates/report.css"));
        html.push_str("</style></head><body>");
        
        // Header
        html.push_str(&format!(
            "<h1>Performance Report: {}</h1>",
            report.session_name
        ));
        
        // Summary section
        html.push_str("<section class='summary'>");
        html.push_str("<h2>Summary</h2>");
        html.push_str(&format!(
            "<div class='score'>Overall Score: {:.1}/100</div>",
            report.summary.overall_score
        ));
        html.push_str("</section>");
        
        // CPU Analysis
        if let Some(cpu_analysis) = &report.cpu_analysis {
            html.push_str("<section class='cpu-analysis'>");
            html.push_str("<h2>CPU Analysis</h2>");
            html.push_str(&format!(
                "<p>Total Samples: {}</p>",
                cpu_analysis.total_samples
            ));
            
            html.push_str("<h3>Hot Functions</h3>");
            html.push_str("<table><tr><th>Function</th><th>Percentage</th><th>Samples</th></tr>");
            
            for func in &cpu_analysis.hot_functions {
                html.push_str(&format!(
                    "<tr><td>{}</td><td>{:.1}%</td><td>{}</td></tr>",
                    func.name, func.percentage, func.sample_count
                ));
            html.push_str("</table>");
            html.push_str("</section>");
        // Memory Analysis
        if let Some(memory_analysis) = &report.memory_analysis {
            html.push_str("<section class='memory-analysis'>");
            html.push_str("<h2>Memory Analysis</h2>");
            html.push_str(&format!(
                "<p>Current Usage: {} bytes</p>",
                memory_analysis.current_usage.allocated_bytes
            ));
            
            if !memory_analysis.memory_leaks.is_empty() {
                html.push_str("<h3>Memory Leaks</h3>");
                html.push_str("<ul>");
                for leak in &memory_analysis.memory_leaks {
                    html.push_str(&format!(
                        "<li>Address: 0x{:x}, Size: {} bytes, Age: {:?}</li>",
                        leak.address, leak.size, leak.age
                    ));
                }
                html.push_str("</ul>");
            html.push_str("</section>");
        html.push_str("</body></html>");
        
        Ok(html)
    fn generate_markdown_report(&self, report: &PerformanceReport) -> crate::error::Result<()> {
        let mut md = String::new();
        
        md.push_str(&format!("# Performance Report: {}\n\n", report.session_name));
        
        // Summary
        md.push_str("## Summary\n\n");
        md.push_str(&format!("**Overall Score:** {:.1}/100\n\n", report.summary.overall_score));
        
        if !report.summary.key_issues.is_empty() {
            md.push_str("### Key Issues\n\n");
            for issue in &report.summary.key_issues {
                md.push_str(&format!("- {}\n", issue));
            }
            md.push_str("\n");
        if !report.summary.recommendations.is_empty() {
            md.push_str("### Recommendations\n\n");
            for rec in &report.summary.recommendations {
                md.push_str(&format!("- {}\n", rec));
            }
            md.push_str("\n");
        // CPU Analysis
        if let Some(cpu_analysis) = &report.cpu_analysis {
            md.push_str("## CPU Analysis\n\n");
            md.push_str(&format!("- **Total Samples:** {}\n", cpu_analysis.total_samples));
            md.push_str(&format!("- **Total Duration:** {:?}\n\n", cpu_analysis.total_duration));
            
            if !cpu_analysis.hot_functions.is_empty() {
                md.push_str("### Hot Functions\n\n");
                md.push_str("| Function | Percentage | Samples |\n");
                md.push_str("|----------|------------|----------|\n");
                
                for func in &cpu_analysis.hot_functions {
                    md.push_str(&format!(
                        func.name, func.percentage, func.sample_count
                    ));
                }
                md.push_str("\n");
            }
        }
        
        // Memory Analysis
        if let Some(memory_analysis) = &report.memory_analysis {
            md.push_str("## Memory Analysis\n\n");
            md.push_str(&format!(
                memory_analysis.current_usage.allocated_bytes
            ));
            md.push_str(&format!(
                memory_analysis.current_usage.active_allocations
            ));
            
            if !memory_analysis.memory_leaks.is_empty() {
                md.push_str("### Memory Leaks\n\n");
                for leak in &memory_analysis.memory_leaks {
                    md.push_str(&format!(
                        leak.address, leak.size, leak.age
                    ));
                }
                md.push_str("\n");
            }
        }
        
        Ok(md)
    }
}

/// Report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
impl Default for ReportConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Complete performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
impl PerformanceReport {
    pub fn new(session_name: String, config: ReportConfig) -> Self {
        Self {
        }
    }
/// Performance summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceSummary {
/// CPU analysis report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuAnalysisReport {
/// Hot function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunction {
/// Call graph summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphSummary {
/// Memory analysis report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAnalysisReport {
/// GC performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcPerformanceAnalysis {
/// Concurrency analysis report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyAnalysisReport {
/// Benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
/// Serializable wrapper for memory leak data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMemoryLeak {
    pub allocation_timestamp_millis: u64, // Instant converted to milliseconds since start
impl From<&crate::profiling::memory::MemoryLeak> for SerializableMemoryLeak {
    fn from(leak: &crate::profiling::memory::MemoryLeak) -> Self {
        Self {
        }
    }
/// Serializable wrapper for goroutine timeline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGoroutineTimeline {
impl From<&crate::profiling::concurrency::GoroutineTimeline> for SerializableGoroutineTimeline {
    fn from(timeline: &crate::profiling::concurrency::GoroutineTimeline) -> Self {
        Self {
        }
    }
/// Serializable wrapper for goroutine events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGoroutineEvent {
impl From<&crate::profiling::concurrency::GoroutineEvent> for SerializableGoroutineEvent {
    fn from(event: &crate::profiling::concurrency::GoroutineEvent) -> Self {
        Self {
        }
    }
/// Serializable wrapper for state transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableStateTransition {
impl From<&crate::profiling::concurrency::StateTransition> for SerializableStateTransition {
    fn from(transition: &crate::profiling::concurrency::StateTransition) -> Self {
        Self {
        }
    }
/// Serializable wrapper for channel analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableChannelAnalysis {
impl From<&crate::profiling::concurrency::ChannelAnalysis> for SerializableChannelAnalysis {
    fn from(analysis: &crate::profiling::concurrency::ChannelAnalysis) -> Self {
        Self {
            channel_statistics: analysis.channel_statistics.iter()
                .map(|(k, v)| (k.clone(), SerializableChannelStats::from(v)))
        }
    }
/// Serializable wrapper for channel statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableChannelStats {
impl From<&crate::profiling::concurrency::ChannelStats> for SerializableChannelStats {
    fn from(stats: &crate::profiling::concurrency::ChannelStats) -> Self {
        Self {
        }
    }
