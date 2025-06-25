/// Profile Data Structures and Reporting for CURSED vibecheck
/// 
/// Provides robust data structures for profile collection, aggregation, and reporting
/// with thread-safe operations and multiple export formats.

use crate::error::CursedError;
// use crate::stdlib::vibecheck::{memory_profiler::MemoryStats, cpu_profiler::CpuProfile};
use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, SystemTime};
use std::fmt;
use std::thread;
use serde::{Serialize, Deserialize};

/// Comprehensive profiling data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    /// Profile metadata
    /// Memory profiling data
    /// CPU profiling data
    /// Custom metrics
/// Profile metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Profile session ID
    /// Profile start time
    /// Profile end time
    /// Total profiling duration
    /// Profiler version
    /// Target application/binary
    /// Host system information
    /// Profile configuration
/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system
    /// CPU architecture
    /// Number of CPU cores
    /// Total system memory
    /// Hostname
/// Profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    /// Memory profiling enabled
    /// CPU profiling enabled
    /// Sampling rate for CPU profiling
    /// Memory sample rate
    /// Maximum profile size
    /// Export formats enabled
/// Memory profiling data (simplified version of MemoryStats)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfileData {
    /// Current allocated bytes
    /// Peak allocated bytes
    /// Total allocated bytes
    /// Total freed bytes
    /// Active allocation count
    /// Memory fragmentation ratio
    /// Size distribution
    /// Thread allocation breakdown
    /// Memory leak information
/// CPU profiling data (simplified version of CpuProfile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileData {
    /// Total profiling duration
    /// Total samples collected
    /// Function call statistics
    /// Hot paths
    /// Performance bottlenecks
    /// Call graph summary
/// Thread allocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAllocation {
    /// Thread name or ID
    /// Total bytes allocated by thread
    /// Number of allocations
/// Memory leak information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakInfo {
    /// Allocation size
    /// Age of allocation
    /// Stack trace (truncated)
    /// Object type if known
/// Function statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    /// Function name
    /// Module name
    /// Total inclusive time
    /// Total exclusive time
    /// Call count
    /// Average time per call
/// Hot path information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotPathInfo {
    /// Call stack path
    /// Percentage of total execution time
    /// Number of samples
/// Performance bottleneck information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckInfo {
    /// Location (function or code)
    /// Type of bottleneck
    /// Performance impact percentage
    /// Optimization suggestion
/// Call graph summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphSummary {
    /// Total number of nodes
    /// Total number of edges
    /// Maximum call depth
    /// Top callers
    /// Top callees
/// Custom metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Integer value
    /// Floating point value
    /// String value
    /// Duration value
    /// Counter value
    /// Histogram
/// Profile report configuration
#[derive(Debug, Clone)]
pub struct ProfileReportConfig {
    /// Include memory statistics
    /// Include CPU statistics
    /// Include call graph
    /// Include hot paths
    /// Include bottlenecks
    /// Maximum items in lists
    /// Report format
    /// Include system information
impl Default for ProfileReportConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Report output format
#[derive(Debug, Clone)]
pub enum ReportFormat {
    /// Plain text format
    /// JSON format
    /// HTML format
    /// Markdown format
    /// CSV format for data analysis
/// Profile report generator
pub struct ProfileReport {
impl ProfileData {
    /// Create a new empty profile data
    pub fn new(session_id: String, target: String) -> Self {
        let system_info = SystemInfo::current();
        
        Self {
            metadata: ProfileMetadata {
        }
    }

    /// Set memory profiling data
    pub fn set_memory_data(&mut self, memory_stats: &MemoryStats) {
        self.memory = Some(MemoryProfileData::from_memory_stats(memory_stats));
        self.metadata.config.memory_profiling = true;
    /// Set CPU profiling data
    pub fn set_cpu_data(&mut self, cpu_profile: &CpuProfile) {
        self.cpu = Some(CpuProfileData::from_cpu_profile(cpu_profile));
        self.metadata.config.cpu_profiling = true;
    /// Add custom metric
    pub fn add_custom_metric(&mut self, name: String, value: MetricValue) {
        self.custom_metrics.insert(name, value);
    /// Finalize profile (set end time and duration)
    pub fn finalize(&mut self) {
        let end_time = SystemTime::now();
        self.metadata.end_time = Some(end_time);
        
        if let Ok(duration) = end_time.duration_since(self.metadata.start_time) {
            self.metadata.duration = Some(duration);
        }
    }

    /// Create profile report
    pub fn create_report(&self, config: ProfileReportConfig) -> ProfileReport {
        ProfileReport {
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> crate::error::Result<()> {
        serde_json::to_string_pretty(self)
            .map_err(|e| CursedError::Runtime(format!("Failed to serialize to JSON: {}", e)))
    /// Import from JSON
    pub fn from_json(json: &str) -> crate::error::Result<()> {
        serde_json::from_str(json)
            .map_err(|e| CursedError::Runtime(format!("Failed to deserialize from JSON: {}", e)))
    /// Merge with another profile data
    pub fn merge(&mut self, other: &ProfileData) -> crate::error::Result<()> {
        // Update metadata duration
        if let (Some(self_duration), Some(other_duration)) = (&self.metadata.duration, &other.metadata.duration) {
            self.metadata.duration = Some(*self_duration + *other_duration);
        // Merge memory data
        if let (Some(ref mut self_memory), Some(ref other_memory)) = (&mut self.memory, &other.memory) {
            self_memory.merge(other_memory);
        } else if other.memory.is_some() {
            self.memory = other.memory.clone();
        // Merge CPU data
        if let (Some(ref mut self_cpu), Some(ref other_cpu)) = (&mut self.cpu, &other.cpu) {
            self_cpu.merge(other_cpu);
        } else if other.cpu.is_some() {
            self.cpu = other.cpu.clone();
        // Merge custom metrics
        for (key, value) in &other.custom_metrics {
            self.custom_metrics.insert(key.clone(), value.clone());
        Ok(())
    }
}

impl SystemInfo {
    /// Get current system information
    pub fn current() -> Self {
        Self {
        }
    }
impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
        }
    }
impl MemoryProfileData {
    /// Convert from MemoryStats
    fn from_memory_stats(stats: &MemoryStats) -> Self {
        let size_distribution = stats.heap_analysis.size_distribution
            .iter()
            .map(|(size, count)| (size.to_string(), *count))
            .collect();

        let thread_allocations = stats.heap_analysis.thread_allocations
            .iter()
            .map(|(thread_id, bytes)| ThreadAllocation {
                allocation_count: 1, // Simplified
            })
            .collect();

        Self {
            potential_leaks: Vec::new(), // Would be populated from actual leak data
        }
    }

    /// Merge with another memory profile data
    fn merge(&mut self, other: &MemoryProfileData) {
        self.current_allocated += other.current_allocated;
        self.peak_allocated = self.peak_allocated.max(other.peak_allocated);
        self.total_allocated += other.total_allocated;
        self.total_freed += other.total_freed;
        self.active_allocations += other.active_allocations;
        
        // Merge size distribution
        for (size, count) in &other.size_distribution {
            *self.size_distribution.entry(size.clone()).or_insert(0) += count;
        // Merge thread allocations
        self.thread_allocations.extend(other.thread_allocations.iter().cloned());
        
        // Merge potential leaks
        self.potential_leaks.extend(other.potential_leaks.iter().cloned());
    }
}

impl CpuProfileData {
    /// Convert from CpuProfile
    fn from_cpu_profile(profile: &CpuProfile) -> Self {
        let function_stats = profile.call_graph
            .values()
            .map(|node| FunctionStats {
                module: "unknown".to_string(), // Would need to be extracted from call graph
                avg_time_ms: if node.call_count > 0 {
                    (node.inclusive_time.as_secs_f64() * 1000.0) / node.call_count as f64
                } else {
                    0.0
            })
            .collect();

        let hot_paths = profile.hot_paths
            .iter()
            .map(|path| HotPathInfo {
            })
            .collect();

        let bottlenecks = profile.bottlenecks
            .iter()
            .map(|bottleneck| BottleneckInfo {
            })
            .collect();

        let call_graph_summary = CallGraphSummary {
            total_edges: profile.call_graph.values()
                .map(|node| node.callees.len())
            top_callers: Vec::new(), // Would be calculated from call graph
            top_callees: Vec::new(), // Would be calculated from call graph

        Self {
        }
    }

    /// Merge with another CPU profile data
    fn merge(&mut self, other: &CpuProfileData) {
        self.profiling_duration += other.profiling_duration;
        self.total_samples += other.total_samples;
        
        // Merge function stats (simplified)
        self.function_stats.extend(other.function_stats.iter().cloned());
        
        // Merge hot paths
        self.hot_paths.extend(other.hot_paths.iter().cloned());
        
        // Merge bottlenecks
        self.bottlenecks.extend(other.bottlenecks.iter().cloned());
    }
}

impl ProfileReport {
    /// Generate report as string
    pub fn generate(&self) -> crate::error::Result<()> {
        match self.config.format {
        }
    }

    /// Generate text report
    fn generate_text_report(&self) -> crate::error::Result<()> {
        let mut report = String::new();
        
        report.push_str("=== CURSED Profiling Report ===\n\n");
        
        // Metadata
        if self.config.include_system_info {
            report.push_str(&format!("Session ID: {}\n", self.data.metadata.session_id));
            report.push_str(&format!("Target: {}\n", self.data.metadata.target));
            report.push_str(&format!("Start Time: {:?}\n", self.data.metadata.start_time));
            if let Some(duration) = self.data.metadata.duration {
                report.push_str(&format!("Duration: {:.2}s\n", duration.as_secs_f64()));
            }
            report.push_str(&format!("OS: {} ({})\n", self.data.metadata.system_info.os, self.data.metadata.system_info.arch));
            report.push_str(&format!("CPU Cores: {}\n", self.data.metadata.system_info.cpu_cores));
            report.push_str("\n");
        // Memory section
        if self.config.include_memory {
            if let Some(ref memory) = self.data.memory {
                report.push_str("=== Memory Profile ===\n");
                report.push_str(&format!("Current Allocated: {} bytes\n", memory.current_allocated));
                report.push_str(&format!("Peak Allocated: {} bytes\n", memory.peak_allocated));
                report.push_str(&format!("Total Allocated: {} bytes\n", memory.total_allocated));
                report.push_str(&format!("Fragmentation: {:.2}%\n", memory.fragmentation_ratio * 100.0));
                report.push_str(&format!("Active Allocations: {}\n", memory.active_allocations));
                
                if !memory.potential_leaks.is_empty() {
                    report.push_str(&format!("Potential Leaks: {}\n", memory.potential_leaks.len()));
                }
                report.push_str("\n");
            }
        }

        // CPU section
        if self.config.include_cpu {
            if let Some(ref cpu) = self.data.cpu {
                report.push_str("=== CPU Profile ===\n");
                report.push_str(&format!("Profiling Duration: {:.2}s\n", cpu.profiling_duration.as_secs_f64()));
                report.push_str(&format!("Total Samples: {}\n", cpu.total_samples));
                report.push_str(&format!("Functions Tracked: {}\n", cpu.function_stats.len()));
                
                if self.config.include_hot_paths && !cpu.hot_paths.is_empty() {
                    report.push_str("\nHot Paths:\n");
                    for (i, path) in cpu.hot_paths.iter().take(self.config.max_list_items).enumerate() {
                            i + 1, path.percentage, path.path.join(" -> ")));
                    }
                }

                if self.config.include_bottlenecks && !cpu.bottlenecks.is_empty() {
                    report.push_str("\nBottlenecks:\n");
                    for (i, bottleneck) in cpu.bottlenecks.iter().take(self.config.max_list_items).enumerate() {
                            i + 1, bottleneck.location, bottleneck.impact));
                        report.push_str(&format!("     {}\n", bottleneck.suggestion));
                    }
                }
                report.push_str("\n");
            }
        }

        // Custom metrics
        if !self.data.custom_metrics.is_empty() {
            report.push_str("=== Custom Metrics ===\n");
            for (name, value) in &self.data.custom_metrics {
                report.push_str(&format!("{}: {}\n", name, format_metric_value(value)));
            }
            report.push_str("\n");
        Ok(report)
    /// Generate HTML report
    fn generate_html_report(&self) -> crate::error::Result<()> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html><head><title>CURSED Profiling Report</title>");
        html.push_str("<style>");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }");
        html.push_str("h1, h2 { color: #333; }");
        html.push_str("table { border-collapse: collapse; width: 100%; margin: 10px 0; }");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }");
        html.push_str("th { background-color: #f2f2f2; }");
        html.push_str(".metric { margin: 5px 0; }");
        html.push_str("</style></head><body>");
        
        html.push_str("<h1>CURSED Profiling Report</h1>");
        
        // Add content similar to text report but with HTML formatting
        html.push_str(&format!("<p><strong>Session:</strong> {}</p>", self.data.metadata.session_id));
        html.push_str(&format!("<p><strong>Target:</strong> {}</p>", self.data.metadata.target));
        
        if let Some(ref memory) = self.data.memory {
            html.push_str("<h2>Memory Profile</h2>");
            html.push_str("<table>");
            html.push_str("<tr><th>Metric</th><th>Value</th></tr>");
            html.push_str(&format!("<tr><td>Current Allocated</td><td>{} bytes</td></tr>", memory.current_allocated));
            html.push_str(&format!("<tr><td>Peak Allocated</td><td>{} bytes</td></tr>", memory.peak_allocated));
            html.push_str("</table>");
        if let Some(ref cpu) = self.data.cpu {
            html.push_str("<h2>CPU Profile</h2>");
            html.push_str("<table>");
            html.push_str("<tr><th>Metric</th><th>Value</th></tr>");
            html.push_str(&format!("<tr><td>Duration</td><td>{:.2}s</td></tr>", cpu.profiling_duration.as_secs_f64()));
            html.push_str(&format!("<tr><td>Total Samples</td><td>{}</td></tr>", cpu.total_samples));
            html.push_str("</table>");
        html.push_str("</body></html>");
        
        Ok(html)
    /// Generate Markdown report
    fn generate_markdown_report(&self) -> crate::error::Result<()> {
        let mut md = String::new();
        
        md.push_str("# CURSED Profiling Report\n\n");
        
        md.push_str("## Session Information\n\n");
        md.push_str(&format!("- **Session ID**: {}\n", self.data.metadata.session_id));
        md.push_str(&format!("- **Target**: {}\n", self.data.metadata.target));
        if let Some(duration) = self.data.metadata.duration {
            md.push_str(&format!("- **Duration**: {:.2}s\n", duration.as_secs_f64()));
        }
        md.push_str("\n");

        if let Some(ref memory) = self.data.memory {
            md.push_str("## Memory Profile\n\n");
            md.push_str("| Metric | Value |\n");
            md.push_str("|--------|-------|\n");
            md.push_str(&format!("| Current Allocated | {} bytes |\n", memory.current_allocated));
            md.push_str(&format!("| Peak Allocated | {} bytes |\n", memory.peak_allocated));
            md.push_str(&format!("| Fragmentation | {:.2}% |\n", memory.fragmentation_ratio * 100.0));
            md.push_str("\n");
        if let Some(ref cpu) = self.data.cpu {
            md.push_str("## CPU Profile\n\n");
            md.push_str("| Metric | Value |\n");
            md.push_str("|--------|-------|\n");
            md.push_str(&format!("| Duration | {:.2}s |\n", cpu.profiling_duration.as_secs_f64()));
            md.push_str(&format!("| Total Samples | {} |\n", cpu.total_samples));
            md.push_str("\n");

            if !cpu.hot_paths.is_empty() {
                md.push_str("### Hot Paths\n\n");
                for (i, path) in cpu.hot_paths.iter().take(self.config.max_list_items).enumerate() {
                        i + 1, path.percentage, path.path.join(" -> ")));
                }
                md.push_str("\n");
            }
        }

        Ok(md)
    /// Generate CSV report
    fn generate_csv_report(&self) -> crate::error::Result<()> {
        let mut csv = String::new();
        
        // CSV headers and data for function statistics
        if let Some(ref cpu) = self.data.cpu {
            csv.push_str("Function,Module,Inclusive Time (ms),Exclusive Time (ms),Call Count,Avg Time (ms)\n");
            
            for func in &cpu.function_stats {
                    func.exclusive_time_ms, func.call_count, func.avg_time_ms));
            }
        }
        
        Ok(csv)
    }
}

/// Helper functions

fn get_total_memory() -> u64 {
    // Platform-specific implementation would be needed
    // For now, return a reasonable default
    8 * 1024 * 1024 * 1024 // 8GB
fn get_hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
// fn calculate_max_depth(function_calls: &[crate::stdlib::vibecheck::cpu_profiler::FunctionCall]) -> usize {
    function_calls.iter()
        .map(|call| calculate_call_depth(call, 0))
        .max()
        .unwrap_or(0)
// fn calculate_call_depth(call: &crate::stdlib::vibecheck::cpu_profiler::FunctionCall, current_depth: usize) -> usize {
    let max_child_depth = call.children.iter()
        .map(|child| calculate_call_depth(child, current_depth + 1))
        .max()
        .unwrap_or(current_depth);
    max_child_depth
fn format_metric_value(value: &MetricValue) -> String {
    match value {
    }
}

/// Simple num_cpus implementation for environments where it's not available
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

