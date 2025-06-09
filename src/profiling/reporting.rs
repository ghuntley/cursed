// Performance reporting and visualization generation

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::profiling::core::{ProfileData, ProfilerError};
use crate::profiling::cpu::{CpuProfileData, FlameGraph};
use crate::profiling::memory::MemoryProfileData;
use crate::profiling::concurrency::ConcurrencyProfileData;
use crate::profiling::benchmarking::BenchmarkResults;

/// Performance report generator
#[derive(Debug)]
pub struct ReportGenerator {
    config: ReportConfig,
}

impl ReportGenerator {
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }
    
    #[instrument(skip(self, profile_data))]
    pub fn generate_report(&self, profile_data: &ProfileData) -> Result<PerformanceReport, ProfilerError> {
        info!("Generating performance report for session: {}", profile_data.session_name);
        
        let mut report = PerformanceReport::new(
            profile_data.session_name.clone(),
            self.config.clone(),
        );
        
        // Generate sections based on available data
        if let Some(cpu_data) = self.extract_cpu_data(profile_data)? {
            report.cpu_analysis = Some(self.generate_cpu_analysis(&cpu_data)?);
        }
        
        if let Some(memory_data) = self.extract_memory_data(profile_data)? {
            report.memory_analysis = Some(self.generate_memory_analysis(&memory_data)?);
        }
        
        if let Some(concurrency_data) = self.extract_concurrency_data(profile_data)? {
            report.concurrency_analysis = Some(self.generate_concurrency_analysis(&concurrency_data)?);
        }
        
        // Generate summary
        report.summary = self.generate_summary(&report);
        
        info!("Performance report generated successfully");
        Ok(report)
    }
    
    #[instrument(skip(self, benchmark_results))]
    pub fn generate_benchmark_report(&self, benchmark_results: &BenchmarkResults) -> Result<BenchmarkReport, ProfilerError> {
        info!("Generating benchmark report for suite: {}", benchmark_results.suite_name);
        
        let report = BenchmarkReport {
            suite_name: benchmark_results.suite_name.clone(),
            timestamp: benchmark_results.timestamp,
            summary: benchmark_results.summary.clone(),
            results: benchmark_results.results.clone(),
            regression_analysis: benchmark_results.regression_analysis.clone(),
            performance_insights: self.generate_performance_insights(benchmark_results),
            recommendations: self.generate_recommendations(benchmark_results),
        };
        
        Ok(report)
    }
    
    #[instrument(skip(self, report))]
    pub fn export_html(&self, report: &PerformanceReport, output_path: &str) -> Result<(), ProfilerError> {
        let html = self.generate_html_report(report)?;
        std::fs::write(output_path, html).map_err(ProfilerError::IoError)?;
        info!("HTML report exported to: {}", output_path);
        Ok(())
    }
    
    #[instrument(skip(self, report))]
    pub fn export_markdown(&self, report: &PerformanceReport, output_path: &str) -> Result<(), ProfilerError> {
        let markdown = self.generate_markdown_report(report)?;
        std::fs::write(output_path, markdown).map_err(ProfilerError::IoError)?;
        info!("Markdown report exported to: {}", output_path);
        Ok(())
    }
    
    #[instrument(skip(self, report))]
    pub fn export_json(&self, report: &PerformanceReport, output_path: &str) -> Result<(), ProfilerError> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        std::fs::write(output_path, json).map_err(ProfilerError::IoError)?;
        info!("JSON report exported to: {}", output_path);
        Ok(())
    }
    
    fn extract_cpu_data(&self, profile_data: &ProfileData) -> Result<Option<CpuProfileData>, ProfilerError> {
        if let Some(data) = profile_data.get_mode_data(&crate::profiling::core::ProfilerMode::Cpu) {
            let cpu_data: CpuProfileData = bincode::deserialize(data)
                .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
            Ok(Some(cpu_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_memory_data(&self, profile_data: &ProfileData) -> Result<Option<MemoryProfileData>, ProfilerError> {
        if let Some(data) = profile_data.get_mode_data(&crate::profiling::core::ProfilerMode::Memory) {
            let memory_data: MemoryProfileData = bincode::deserialize(data)
                .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
            Ok(Some(memory_data))
        } else {
            Ok(None)
        }
    }
    
    fn extract_concurrency_data(&self, _profile_data: &ProfileData) -> Result<Option<ConcurrencyProfileData>, ProfilerError> {
        // TODO: Re-enable when deserialization is properly implemented
        // For now, return None as concurrency data cannot be deserialized due to Instant fields
        Ok(None)
    }
    
    fn generate_cpu_analysis(&self, cpu_data: &CpuProfileData) -> Result<CpuAnalysisReport, ProfilerError> {
        let hot_functions = cpu_data.get_hot_functions(self.config.max_functions);
        let call_graph = cpu_data.get_call_graph();
        let flame_graph = FlameGraph::from_cpu_profile(cpu_data)?;
        
        Ok(CpuAnalysisReport {
            total_samples: cpu_data.sample_count,
            total_duration: cpu_data.total_duration,
            hot_functions: hot_functions.into_iter()
                .map(|(name, stats)| HotFunction {
                    name: name.clone(),
                    sample_count: stats.sample_count,
                    percentage: stats.percentage(cpu_data.sample_count),
                    exclusive_time: stats.exclusive_time,
                    inclusive_time: stats.inclusive_time,
                })
                .collect(),
            call_graph_summary: CallGraphSummary {
                total_functions: call_graph.nodes.len(),
                total_edges: call_graph.edges.len(),
                max_depth: self.calculate_max_call_depth(&call_graph),
            },
            flame_graph: if self.config.include_flame_graphs {
                Some(flame_graph)
            } else {
                None
            },
            performance_insights: self.generate_cpu_insights(cpu_data),
        })
    }
    
    fn generate_memory_analysis(&self, memory_data: &MemoryProfileData) -> Result<MemoryAnalysisReport, ProfilerError> {
        let current_usage = memory_data.calculate_current_usage();
        let allocation_analysis = memory_data.analyze_patterns();
        let memory_leaks = memory_data.detect_leaks();
        
        Ok(MemoryAnalysisReport {
            current_usage,
            allocation_analysis,
            memory_leaks: memory_leaks.into_iter()
                .take(self.config.max_memory_leaks)
                .collect(),
            gc_performance: self.analyze_gc_performance(memory_data),
            memory_insights: self.generate_memory_insights(memory_data),
        })
    }
    
    fn generate_concurrency_analysis(&self, concurrency_data: &ConcurrencyProfileData) -> Result<ConcurrencyAnalysisReport, ProfilerError> {
        let goroutine_timeline = concurrency_data.generate_goroutine_timeline();
        let channel_analysis = concurrency_data.analyze_channels();
        let deadlocks = concurrency_data.detect_deadlocks();
        let scheduler_analysis = concurrency_data.analyze_scheduler();
        
        Ok(ConcurrencyAnalysisReport {
            goroutine_timeline: goroutine_timeline.into_iter()
                .take(self.config.max_goroutines)
                .collect(),
            channel_analysis,
            deadlock_detections: deadlocks,
            scheduler_analysis,
            concurrency_insights: self.generate_concurrency_insights(concurrency_data),
        })
    }
    
    fn generate_summary(&self, report: &PerformanceReport) -> PerformanceSummary {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Analyze CPU performance
        if let Some(cpu_analysis) = &report.cpu_analysis {
            if cpu_analysis.hot_functions.len() > 0 {
                let top_function = &cpu_analysis.hot_functions[0];
                if top_function.percentage > 30.0 {
                    issues.push(format!(
                        "Function '{}' consumes {:.1}% of CPU time",
                        top_function.name, top_function.percentage
                    ));
                    recommendations.push(
                        "Consider optimizing the most CPU-intensive function".to_string()
                    );
                }
            }
        }
        
        // Analyze memory performance
        if let Some(memory_analysis) = &report.memory_analysis {
            if !memory_analysis.memory_leaks.is_empty() {
                issues.push(format!(
                    "{} potential memory leaks detected",
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
                    "{} potential deadlocks detected",
                    concurrency_analysis.deadlock_detections.len()
                ));
                recommendations.push(
                    "Review goroutine synchronization and channel usage".to_string()
                );
            }
        }
        
        let overall_score = self.calculate_performance_score(report);
        
        PerformanceSummary {
            overall_score,
            key_issues: issues,
            recommendations,
            execution_time: report.cpu_analysis
                .as_ref()
                .map(|cpu| cpu.total_duration)
                .unwrap_or_default(),
            memory_usage: report.memory_analysis
                .as_ref()
                .map(|mem| mem.current_usage.allocated_bytes)
                .unwrap_or(0),
            concurrency_utilization: report.concurrency_analysis
                .as_ref()
                .map(|conc| conc.scheduler_analysis.scheduler_efficiency)
                .unwrap_or(0.0),
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
        }
        
        // Deduct points for memory issues
        if let Some(memory_analysis) = &report.memory_analysis {
            let leak_penalty = (memory_analysis.memory_leaks.len() as f64) * 5.0;
            score -= leak_penalty.min(30.0);
        }
        
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
    }
    
    fn calculate_max_call_depth(&self, call_graph: &crate::profiling::cpu::CallGraph) -> usize {
        // Simplified depth calculation
        call_graph.edges.len()
    }
    
    fn generate_cpu_insights(&self, _cpu_data: &CpuProfileData) -> Vec<String> {
        vec![
            "Consider function inlining for frequently called small functions".to_string(),
            "Look for opportunities to parallelize CPU-intensive operations".to_string(),
        ]
    }
    
    fn generate_memory_insights(&self, _memory_data: &MemoryProfileData) -> Vec<String> {
        vec![
            "Consider object pooling for frequently allocated objects".to_string(),
            "Review garbage collection frequency and tuning parameters".to_string(),
        ]
    }
    
    fn generate_concurrency_insights(&self, _concurrency_data: &ConcurrencyProfileData) -> Vec<String> {
        vec![
            "Consider reducing goroutine creation overhead".to_string(),
            "Review channel buffer sizes for optimal throughput".to_string(),
        ]
    }
    
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
        };
        
        let total_bytes_collected: usize = memory_data.gc_events
            .iter()
            .map(|event| event.bytes_collected)
            .sum();
        
        GcPerformanceAnalysis {
            total_collections,
            total_gc_time,
            average_pause_time,
            total_bytes_collected,
            gc_efficiency: if total_gc_time.as_millis() > 0 {
                total_bytes_collected as f64 / total_gc_time.as_millis() as f64
            } else {
                0.0
            },
        }
    }
    
    fn generate_performance_insights(&self, _benchmark_results: &BenchmarkResults) -> Vec<String> {
        vec![
            "Consider using benchmark-driven optimization".to_string(),
            "Monitor performance trends over time".to_string(),
        ]
    }
    
    fn generate_recommendations(&self, _benchmark_results: &BenchmarkResults) -> Vec<String> {
        vec![
            "Set up automated performance regression testing".to_string(),
            "Establish performance budgets for critical operations".to_string(),
        ]
    }
    
    fn generate_html_report(&self, report: &PerformanceReport) -> Result<String, ProfilerError> {
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
            }
            
            html.push_str("</table>");
            html.push_str("</section>");
        }
        
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
            }
            
            html.push_str("</section>");
        }
        
        html.push_str("</body></html>");
        
        Ok(html)
    }
    
    fn generate_markdown_report(&self, report: &PerformanceReport) -> Result<String, ProfilerError> {
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
        }
        
        if !report.summary.recommendations.is_empty() {
            md.push_str("### Recommendations\n\n");
            for rec in &report.summary.recommendations {
                md.push_str(&format!("- {}\n", rec));
            }
            md.push_str("\n");
        }
        
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
                        "| {} | {:.1}% | {} |\n",
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
                "- **Current Usage:** {} bytes\n",
                memory_analysis.current_usage.allocated_bytes
            ));
            md.push_str(&format!(
                "- **Active Allocations:** {}\n\n",
                memory_analysis.current_usage.active_allocations
            ));
            
            if !memory_analysis.memory_leaks.is_empty() {
                md.push_str("### Memory Leaks\n\n");
                for leak in &memory_analysis.memory_leaks {
                    md.push_str(&format!(
                        "- **Address:** 0x{:x}, **Size:** {} bytes, **Age:** {:?}\n",
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
    pub include_flame_graphs: bool,
    pub include_call_graphs: bool,
    pub max_functions: usize,
    pub max_memory_leaks: usize,
    pub max_goroutines: usize,
    pub performance_threshold: f64,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            include_flame_graphs: true,
            include_call_graphs: true,
            max_functions: 20,
            max_memory_leaks: 10,
            max_goroutines: 50,
            performance_threshold: 10.0,
        }
    }
}

/// Complete performance report
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceReport {
    pub session_name: String,
    pub timestamp: std::time::SystemTime,
    pub config: ReportConfig,
    pub summary: PerformanceSummary,
    pub cpu_analysis: Option<CpuAnalysisReport>,
    #[serde(skip)]
    pub memory_analysis: Option<MemoryAnalysisReport>,
    #[serde(skip)]
    pub concurrency_analysis: Option<ConcurrencyAnalysisReport>,
}

impl PerformanceReport {
    pub fn new(session_name: String, config: ReportConfig) -> Self {
        Self {
            session_name,
            timestamp: std::time::SystemTime::now(),
            config,
            summary: PerformanceSummary::default(),
            cpu_analysis: None,
            memory_analysis: None,
            concurrency_analysis: None,
        }
    }
}

/// Performance summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub overall_score: f64,
    pub key_issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub execution_time: std::time::Duration,
    pub memory_usage: usize,
    pub concurrency_utilization: f64,
}

/// CPU analysis report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuAnalysisReport {
    pub total_samples: u64,
    pub total_duration: std::time::Duration,
    pub hot_functions: Vec<HotFunction>,
    pub call_graph_summary: CallGraphSummary,
    pub flame_graph: Option<FlameGraph>,
    pub performance_insights: Vec<String>,
}

/// Hot function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunction {
    pub name: String,
    pub sample_count: u64,
    pub percentage: f64,
    pub exclusive_time: std::time::Duration,
    pub inclusive_time: std::time::Duration,
}

/// Call graph summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphSummary {
    pub total_functions: usize,
    pub total_edges: usize,
    pub max_depth: usize,
}

/// Memory analysis report section
#[derive(Debug, Clone, Serialize)]
pub struct MemoryAnalysisReport {
    pub current_usage: crate::profiling::memory::MemoryUsageSnapshot,
    pub allocation_analysis: crate::profiling::memory::AllocationAnalysis,
    #[serde(skip)]
    pub memory_leaks: Vec<crate::profiling::memory::MemoryLeak>,
    pub gc_performance: GcPerformanceAnalysis,
    pub memory_insights: Vec<String>,
}

/// GC performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcPerformanceAnalysis {
    pub total_collections: u64,
    pub total_gc_time: std::time::Duration,
    pub average_pause_time: std::time::Duration,
    pub total_bytes_collected: usize,
    pub gc_efficiency: f64,
}

/// Concurrency analysis report section
#[derive(Debug, Clone, Serialize)]
pub struct ConcurrencyAnalysisReport {
    pub goroutine_timeline: Vec<crate::profiling::concurrency::GoroutineTimeline>,
    #[serde(skip)]
    pub channel_analysis: crate::profiling::concurrency::ChannelAnalysis,
    pub deadlock_detections: Vec<crate::profiling::concurrency::DeadlockDetection>,
    pub scheduler_analysis: crate::profiling::concurrency::SchedulerAnalysis,
    pub concurrency_insights: Vec<String>,
}

/// Benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub suite_name: String,
    pub timestamp: std::time::SystemTime,
    pub summary: crate::profiling::benchmarking::BenchmarkSummary,
    pub results: HashMap<String, crate::profiling::benchmarking::BenchmarkResult>,
    pub regression_analysis: Option<crate::profiling::benchmarking::RegressionAnalysis>,
    pub performance_insights: Vec<String>,
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_report_generator_creation() {
        let config = ReportConfig::default();
        let generator = ReportGenerator::new(config);
        assert!(generator.config.include_flame_graphs);
    }
    
    #[test]
    fn test_performance_score_calculation() {
        let config = ReportConfig::default();
        let generator = ReportGenerator::new(config);
        
        let report = PerformanceReport::new("test".to_string(), ReportConfig::default());
        let score = generator.calculate_performance_score(&report);
        
        assert_eq!(score, 100.0); // No issues, perfect score
    }
    
    #[test]
    fn test_markdown_generation() {
        let config = ReportConfig::default();
        let generator = ReportGenerator::new(config.clone());
        
        let report = PerformanceReport::new("test_session".to_string(), config);
        let markdown = generator.generate_markdown_report(&report).unwrap();
        
        assert!(markdown.contains("# Performance Report: test_session"));
        assert!(markdown.contains("## Summary"));
    }
}
