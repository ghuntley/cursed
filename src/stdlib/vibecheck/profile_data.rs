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
    pub metadata: ProfileMetadata,
    /// Memory profiling data
    pub memory: Option<MemoryProfileData>,
    /// CPU profiling data
    pub cpu: Option<CpuProfileData>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, MetricValue>,
}

/// Profile metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Profile session ID
    pub session_id: String,
    /// Profile start time
    pub start_time: SystemTime,
    /// Profile end time
    pub end_time: Option<SystemTime>,
    /// Total profiling duration
    pub duration: Option<Duration>,
    /// Profiler version
    pub version: String,
    /// Target application/binary
    pub target: String,
    /// Host system information
    pub system_info: SystemInfo,
    /// Profile configuration
    pub config: ProfileConfig,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system
    pub os: String,
    /// CPU architecture
    pub arch: String,
    /// Number of CPU cores
    pub cpu_cores: u32,
    /// Total system memory
    pub total_memory: u64,
    /// Hostname
    pub hostname: String,
}

/// Profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    /// Memory profiling enabled
    pub memory_profiling: bool,
    /// CPU profiling enabled
    pub cpu_profiling: bool,
    /// Sampling rate for CPU profiling
    pub cpu_sample_rate: u32,
    /// Memory sample rate
    pub memory_sample_rate: u32,
    /// Maximum profile size
    pub max_profile_size: usize,
    /// Export formats enabled
    pub export_formats: Vec<String>,
}

/// Memory profiling data (simplified version of MemoryStats)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfileData {
    /// Current allocated bytes
    pub current_allocated: u64,
    /// Peak allocated bytes
    pub peak_allocated: u64,
    /// Total allocated bytes
    pub total_allocated: u64,
    /// Total freed bytes
    pub total_freed: u64,
    /// Active allocation count
    pub active_allocations: usize,
    /// Memory fragmentation ratio
    pub fragmentation_ratio: f64,
    /// Size distribution
    pub size_distribution: BTreeMap<String, u64>,
    /// Thread allocation breakdown
    pub thread_allocations: Vec<ThreadAllocation>,
    /// Memory leak information
    pub potential_leaks: Vec<LeakInfo>,
}

/// CPU profiling data (simplified version of CpuProfile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileData {
    /// Total profiling duration
    pub profiling_duration: Duration,
    /// Total samples collected
    pub total_samples: u64,
    /// Function call statistics
    pub function_stats: Vec<FunctionStats>,
    /// Hot paths
    pub hot_paths: Vec<HotPathInfo>,
    /// Performance bottlenecks
    pub bottlenecks: Vec<BottleneckInfo>,
    /// Call graph summary
    pub call_graph_summary: CallGraphSummary,
}

/// Thread allocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAllocation {
    /// Thread name or ID
    pub thread_name: String,
    /// Total bytes allocated by thread
    pub allocated_bytes: u64,
    /// Number of allocations
    pub allocation_count: u64,
}

/// Memory leak information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakInfo {
    /// Allocation size
    pub size: usize,
    /// Age of allocation
    pub age_seconds: u64,
    /// Stack trace (truncated)
    pub stack_trace: Option<String>,
    /// Object type if known
    pub object_type: Option<String>,
}

/// Function statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    /// Function name
    pub name: String,
    /// Module name
    pub module: String,
    /// Total inclusive time
    pub inclusive_time_ms: f64,
    /// Total exclusive time
    pub exclusive_time_ms: f64,
    /// Call count
    pub call_count: u64,
    /// Average time per call
    pub avg_time_ms: f64,
}

/// Hot path information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotPathInfo {
    /// Call stack path
    pub path: Vec<String>,
    /// Percentage of total execution time
    pub percentage: f64,
    /// Number of samples
    pub sample_count: u64,
}

/// Performance bottleneck information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckInfo {
    /// Location (function or code)
    pub location: String,
    /// Type of bottleneck
    pub bottleneck_type: String,
    /// Performance impact percentage
    pub impact: f64,
    /// Optimization suggestion
    pub suggestion: String,
}

/// Call graph summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphSummary {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Total number of edges
    pub total_edges: usize,
    /// Maximum call depth
    pub max_depth: usize,
    /// Top callers
    pub top_callers: Vec<(String, u64)>,
    /// Top callees
    pub top_callees: Vec<(String, u64)>,
}

/// Custom metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
    /// String value
    String(String),
    /// Duration value
    Duration(Duration),
    /// Counter value
    Counter(u64),
    /// Histogram
    Histogram(Vec<(f64, u64)>),
}

/// Profile report configuration
#[derive(Debug, Clone)]
pub struct ProfileReportConfig {
    /// Include memory statistics
    pub include_memory: bool,
    /// Include CPU statistics
    pub include_cpu: bool,
    /// Include call graph
    pub include_call_graph: bool,
    /// Include hot paths
    pub include_hot_paths: bool,
    /// Include bottlenecks
    pub include_bottlenecks: bool,
    /// Maximum items in lists
    pub max_list_items: usize,
    /// Report format
    pub format: ReportFormat,
    /// Include system information
    pub include_system_info: bool,
}

impl Default for ProfileReportConfig {
    fn default() -> Self {
        Self {
            include_memory: true,
            include_cpu: true,
            include_call_graph: true,
            include_hot_paths: true,
            include_bottlenecks: true,
            max_list_items: 20,
            format: ReportFormat::Text,
            include_system_info: true,
        }
    }
}

/// Report output format
#[derive(Debug, Clone)]
pub enum ReportFormat {
    /// Plain text format
    Text,
    /// JSON format
    Json,
    /// HTML format
    Html,
    /// Markdown format
    Markdown,
    /// CSV format for data analysis
    Csv,
}

/// Profile report generator
pub struct ProfileReport {
    data: ProfileData,
    config: ProfileReportConfig,
}

impl ProfileData {
    /// Create a new empty profile data
    pub fn new(session_id: String, target: String) -> Self {
        let system_info = SystemInfo::current();
        
        Self {
            metadata: ProfileMetadata {
                session_id,
                start_time: SystemTime::now(),
                end_time: None,
                duration: None,
                version: env!("CARGO_PKG_VERSION").to_string(),
                target,
                system_info,
                config: ProfileConfig::default(),
            },
            memory: None,
            cpu: None,
            custom_metrics: HashMap::new(),
        }
    }

    /// Set memory profiling data
    pub fn set_memory_data(&mut self, memory_stats: &MemoryStats) {
        self.memory = Some(MemoryProfileData::from_memory_stats(memory_stats));
        self.metadata.config.memory_profiling = true;
    }

    /// Set CPU profiling data
    pub fn set_cpu_data(&mut self, cpu_profile: &CpuProfile) {
        self.cpu = Some(CpuProfileData::from_cpu_profile(cpu_profile));
        self.metadata.config.cpu_profiling = true;
    }

    /// Add custom metric
    pub fn add_custom_metric(&mut self, name: String, value: MetricValue) {
        self.custom_metrics.insert(name, value);
    }

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
            data: self.clone(),
            config,
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> crate::error::Result<()> {
        serde_json::to_string_pretty(self)
            .map_err(|e| CursedError::Runtime(format!("Failed to serialize to JSON: {}", e)))
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> crate::error::Result<()> {
        serde_json::from_str(json)
            .map_err(|e| CursedError::Runtime(format!("Failed to deserialize from JSON: {}", e)))
    }

    /// Merge with another profile data
    pub fn merge(&mut self, other: &ProfileData) -> crate::error::Result<()> {
        // Update metadata duration
        if let (Some(self_duration), Some(other_duration)) = (&self.metadata.duration, &other.metadata.duration) {
            self.metadata.duration = Some(*self_duration + *other_duration);
        }

        // Merge memory data
        if let (Some(ref mut self_memory), Some(ref other_memory)) = (&mut self.memory, &other.memory) {
            self_memory.merge(other_memory);
        } else if other.memory.is_some() {
            self.memory = other.memory.clone();
        }

        // Merge CPU data
        if let (Some(ref mut self_cpu), Some(ref other_cpu)) = (&mut self.cpu, &other.cpu) {
            self_cpu.merge(other_cpu);
        } else if other.cpu.is_some() {
            self.cpu = other.cpu.clone();
        }

        // Merge custom metrics
        for (key, value) in &other.custom_metrics {
            self.custom_metrics.insert(key.clone(), value.clone());
        }

        Ok(())
    }
}

impl SystemInfo {
    /// Get current system information
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get() as u32,
            total_memory: get_total_memory(),
            hostname: get_hostname(),
        }
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            memory_profiling: false,
            cpu_profiling: false,
            cpu_sample_rate: 100,
            memory_sample_rate: 1,
            max_profile_size: 100_000,
            export_formats: vec!["json".to_string(), "text".to_string()],
        }
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
                thread_name: format!("{:?}", thread_id),
                allocated_bytes: *bytes,
                allocation_count: 1, // Simplified
            })
            .collect();

        Self {
            current_allocated: stats.heap_analysis.current_allocated,
            peak_allocated: stats.heap_analysis.peak_allocated,
            total_allocated: stats.heap_analysis.total_allocated,
            total_freed: stats.heap_analysis.total_freed,
            active_allocations: stats.heap_analysis.active_allocations,
            fragmentation_ratio: stats.heap_analysis.fragmentation_ratio,
            size_distribution,
            thread_allocations,
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
        }
        
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
                name: node.function_name.clone(),
                module: "unknown".to_string(), // Would need to be extracted from call graph
                inclusive_time_ms: node.inclusive_time.as_secs_f64() * 1000.0,
                exclusive_time_ms: node.exclusive_time.as_secs_f64() * 1000.0,
                call_count: node.call_count,
                avg_time_ms: if node.call_count > 0 {
                    (node.inclusive_time.as_secs_f64() * 1000.0) / node.call_count as f64
                } else {
                    0.0
                },
            })
            .collect();

        let hot_paths = profile.hot_paths
            .iter()
            .map(|path| HotPathInfo {
                path: path.call_stack.clone(),
                percentage: path.percentage,
                sample_count: path.sample_count,
            })
            .collect();

        let bottlenecks = profile.bottlenecks
            .iter()
            .map(|bottleneck| BottleneckInfo {
                location: bottleneck.location.clone(),
                bottleneck_type: format!("{:?}", bottleneck.bottleneck_type),
                impact: bottleneck.impact,
                suggestion: bottleneck.suggestion.clone(),
            })
            .collect();

        let call_graph_summary = CallGraphSummary {
            total_nodes: profile.call_graph.len(),
            total_edges: profile.call_graph.values()
                .map(|node| node.callees.len())
                .sum(),
            max_depth: calculate_max_depth(&profile.function_calls),
            top_callers: Vec::new(), // Would be calculated from call graph
            top_callees: Vec::new(), // Would be calculated from call graph
        };

        Self {
            profiling_duration: profile.profiling_duration,
            total_samples: profile.total_samples,
            function_stats,
            hot_paths,
            bottlenecks,
            call_graph_summary,
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
            ReportFormat::Text => self.generate_text_report(),
            ReportFormat::Json => self.data.to_json(),
            ReportFormat::Html => self.generate_html_report(),
            ReportFormat::Markdown => self.generate_markdown_report(),
            ReportFormat::Csv => self.generate_csv_report(),
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
        }

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
                        report.push_str(&format!("  {}. {:.1}%: {}\n", 
                            i + 1, path.percentage, path.path.join(" -> ")));
                    }
                }

                if self.config.include_bottlenecks && !cpu.bottlenecks.is_empty() {
                    report.push_str("\nBottlenecks:\n");
                    for (i, bottleneck) in cpu.bottlenecks.iter().take(self.config.max_list_items).enumerate() {
                        report.push_str(&format!("  {}. {}: {:.1}% impact\n", 
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
        }

        Ok(report)
    }

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
        }

        if let Some(ref cpu) = self.data.cpu {
            html.push_str("<h2>CPU Profile</h2>");
            html.push_str("<table>");
            html.push_str("<tr><th>Metric</th><th>Value</th></tr>");
            html.push_str(&format!("<tr><td>Duration</td><td>{:.2}s</td></tr>", cpu.profiling_duration.as_secs_f64()));
            html.push_str(&format!("<tr><td>Total Samples</td><td>{}</td></tr>", cpu.total_samples));
            html.push_str("</table>");
        }
        
        html.push_str("</body></html>");
        
        Ok(html)
    }

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
        }

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
                    md.push_str(&format!("{}. **{:.1}%**: `{}`\n", 
                        i + 1, path.percentage, path.path.join(" -> ")));
                }
                md.push_str("\n");
            }
        }

        Ok(md)
    }

    /// Generate CSV report
    fn generate_csv_report(&self) -> crate::error::Result<()> {
        let mut csv = String::new();
        
        // CSV headers and data for function statistics
        if let Some(ref cpu) = self.data.cpu {
            csv.push_str("Function,Module,Inclusive Time (ms),Exclusive Time (ms),Call Count,Avg Time (ms)\n");
            
            for func in &cpu.function_stats {
                csv.push_str(&format!("{},{},{:.2},{:.2},{},{:.2}\n",
                    func.name, func.module, func.inclusive_time_ms, 
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
}

fn get_hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

// fn calculate_max_depth(function_calls: &[crate::stdlib::vibecheck::cpu_profiler::FunctionCall]) -> usize {
    function_calls.iter()
        .map(|call| calculate_call_depth(call, 0))
        .max()
        .unwrap_or(0)
}

// fn calculate_call_depth(call: &crate::stdlib::vibecheck::cpu_profiler::FunctionCall, current_depth: usize) -> usize {
    let max_child_depth = call.children.iter()
        .map(|child| calculate_call_depth(child, current_depth + 1))
        .max()
        .unwrap_or(current_depth);
    max_child_depth
}

fn format_metric_value(value: &MetricValue) -> String {
    match value {
        MetricValue::Integer(i) => i.to_string(),
        MetricValue::Float(f) => format!("{:.2}", f),
        MetricValue::String(s) => s.clone(),
        MetricValue::Duration(d) => format!("{:.2}s", d.as_secs_f64()),
        MetricValue::Counter(c) => c.to_string(),
        MetricValue::Histogram(h) => format!("{} buckets", h.len()),
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

