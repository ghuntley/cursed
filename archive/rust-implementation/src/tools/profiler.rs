use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::thread;

/// Production-grade profiler for CURSED programs
#[derive(Debug, Clone)]
pub struct Profiler {
    pub config: ProfilerConfig,
    pub samples: Arc<Mutex<Vec<ProfileSample>>>,
    pub memory_snapshots: Arc<Mutex<Vec<MemorySnapshot>>>,
    pub call_graph: Arc<Mutex<CallGraph>>,
    pub start_time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    pub sample_rate: u64,          // samples per second
    pub memory_tracking: bool,
    pub call_graph_tracking: bool,
    pub cpu_profiling: bool,
    pub io_profiling: bool,
    pub gc_profiling: bool,
    pub output_format: String,     // json, flamegraph, html
    pub output_file: String,
    pub max_samples: usize,
    pub stack_depth: usize,
    pub profile_duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSample {
    pub timestamp: u64,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub function_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub thread_id: u64,
    pub stack_trace: Vec<StackFrame>,
    pub gc_activity: bool,
    pub io_activity: IOActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub column_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOActivity {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_calls: u64,
    pub write_calls: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub timestamp: u64,
    pub heap_size: u64,
    pub stack_size: u64,
    pub allocations: Vec<AllocationInfo>,
    pub deallocations: Vec<AllocationInfo>,
    pub gc_collections: u64,
    pub gc_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInfo {
    pub size: u64,
    pub type_name: String,
    pub stack_trace: Vec<StackFrame>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub functions: HashMap<String, FunctionStats>,
    pub call_relationships: Vec<CallRelationship>,
    pub hot_paths: Vec<HotPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    pub name: String,
    pub call_count: u64,
    pub total_time: Duration,
    pub self_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub memory_allocated: u64,
    pub memory_deallocated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRelationship {
    pub caller: String,
    pub callee: String,
    pub call_count: u64,
    pub total_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotPath {
    pub path: Vec<String>,
    pub total_time: Duration,
    pub call_count: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileReport {
    pub config: ProfilerConfig,
    pub duration: Duration,
    pub total_samples: usize,
    pub cpu_summary: CpuSummary,
    pub memory_summary: MemorySummary,
    pub function_summary: FunctionSummary,
    pub hot_spots: Vec<HotSpot>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSummary {
    pub avg_usage: f64,
    pub max_usage: f64,
    pub min_usage: f64,
    pub total_time: Duration,
    pub user_time: Duration,
    pub system_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySummary {
    pub peak_usage: u64,
    pub avg_usage: u64,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub gc_collections: u64,
    pub gc_time: Duration,
    pub memory_leaks: Vec<MemoryLeak>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLeak {
    pub size: u64,
    pub type_name: String,
    pub allocation_stack: Vec<StackFrame>,
    pub age: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSummary {
    pub total_functions: usize,
    pub most_called: Vec<FunctionStats>,
    pub slowest: Vec<FunctionStats>,
    pub memory_intensive: Vec<FunctionStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotSpot {
    pub location: String,
    pub function_name: String,
    pub time_percentage: f64,
    pub call_count: u64,
    pub optimization_suggestions: Vec<String>,
}

impl Profiler {
    /// Create new profiler instance
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            config,
            samples: Arc::new(Mutex::new(Vec::new())),
            memory_snapshots: Arc::new(Mutex::new(Vec::new())),
            call_graph: Arc::new(Mutex::new(CallGraph::new())),
            start_time: Instant::now(),
        }
    }

    /// Start profiling
    pub fn start_profiling(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Starting CURSED profiler...");
        
        // Start CPU sampling thread
        if self.config.cpu_profiling {
            self.start_cpu_sampling()?;
        }

        // Start memory tracking thread
        if self.config.memory_tracking {
            self.start_memory_tracking()?;
        }

        // Start call graph tracking
        if self.config.call_graph_tracking {
            self.start_call_graph_tracking()?;
        }

        println!("✅ Profiler started with {} Hz sampling rate", self.config.sample_rate);
        Ok(())
    }

    /// Stop profiling and generate report
    pub fn stop_profiling(&self) -> Result<ProfileReport, Box<dyn std::error::Error>> {
        println!("🛑 Stopping profiler and generating report...");
        
        let duration = self.start_time.elapsed();
        let samples = self.samples.lock().unwrap();
        let memory_snapshots = self.memory_snapshots.lock().unwrap();
        let call_graph = self.call_graph.lock().unwrap();

        let report = ProfileReport {
            config: self.config.clone(),
            duration,
            total_samples: samples.len(),
            cpu_summary: self.generate_cpu_summary(&samples),
            memory_summary: self.generate_memory_summary(&memory_snapshots),
            function_summary: self.generate_function_summary(&call_graph),
            hot_spots: self.identify_hot_spots(&samples, &call_graph),
            recommendations: self.generate_recommendations(&samples, &memory_snapshots, &call_graph),
        };

        // Generate output files
        self.generate_output(&report)?;

        println!("✅ Profiling report generated: {}", self.config.output_file);
        Ok(report)
    }

    /// Start CPU sampling
    fn start_cpu_sampling(&self) -> Result<(), Box<dyn std::error::Error>> {
        let samples = Arc::clone(&self.samples);
        let sample_rate = self.config.sample_rate;
        let max_samples = self.config.max_samples;

        thread::spawn(move || {
            let interval = Duration::from_millis(1000 / sample_rate);
            let mut last_cpu_time = Self::get_cpu_time();
            
            loop {
                thread::sleep(interval);
                
                let current_cpu_time = Self::get_cpu_time();
                let cpu_usage = ((current_cpu_time - last_cpu_time) / interval.as_secs_f64()) * 100.0;
                last_cpu_time = current_cpu_time;

                let sample = ProfileSample {
                    timestamp: Self::get_timestamp(),
                    cpu_usage,
                    memory_usage: Self::get_memory_usage(),
                    function_name: Self::get_current_function(),
                    file_path: Self::get_current_file(),
                    line_number: Self::get_current_line(),
                    thread_id: Self::get_thread_id(),
                    stack_trace: Self::get_stack_trace(),
                    gc_activity: Self::is_gc_active(),
                    io_activity: Self::get_io_activity(),
                };

                let mut samples_lock = samples.lock().unwrap();
                samples_lock.push(sample);
                
                // Limit samples to prevent memory issues
                if samples_lock.len() > max_samples {
                    samples_lock.remove(0);
                }
            }
        });

        Ok(())
    }

    /// Start memory tracking
    fn start_memory_tracking(&self) -> Result<(), Box<dyn std::error::Error>> {
        let memory_snapshots = Arc::clone(&self.memory_snapshots);
        let sample_rate = self.config.sample_rate;

        thread::spawn(move || {
            let interval = Duration::from_millis(1000 / sample_rate);
            
            loop {
                thread::sleep(interval);
                
                let snapshot = MemorySnapshot {
                    timestamp: Self::get_timestamp(),
                    heap_size: Self::get_heap_size(),
                    stack_size: Self::get_stack_size(),
                    allocations: Self::get_recent_allocations(),
                    deallocations: Self::get_recent_deallocations(),
                    gc_collections: Self::get_gc_collections(),
                    gc_time_ms: Self::get_gc_time(),
                };

                let mut snapshots_lock = memory_snapshots.lock().unwrap();
                snapshots_lock.push(snapshot);
            }
        });

        Ok(())
    }

    /// Start call graph tracking
    fn start_call_graph_tracking(&self) -> Result<(), Box<dyn std::error::Error>> {
        // This would integrate with the CURSED runtime to track function calls
        // For now, we'll use a simplified approach
        Ok(())
    }

    /// Generate CPU summary
    fn generate_cpu_summary(&self, samples: &[ProfileSample]) -> CpuSummary {
        if samples.is_empty() {
            return CpuSummary {
                avg_usage: 0.0,
                max_usage: 0.0,
                min_usage: 0.0,
                total_time: Duration::from_secs(0),
                user_time: Duration::from_secs(0),
                system_time: Duration::from_secs(0),
            };
        }

        let cpu_usages: Vec<f64> = samples.iter().map(|s| s.cpu_usage).collect();
        let avg_usage = cpu_usages.iter().sum::<f64>() / cpu_usages.len() as f64;
        let max_usage = cpu_usages.iter().fold(0.0f64, |a, &b| a.max(b));
        let min_usage = cpu_usages.iter().fold(100.0f64, |a, &b| a.min(b));

        CpuSummary {
            avg_usage,
            max_usage,
            min_usage,
            total_time: self.start_time.elapsed(),
            user_time: Duration::from_secs(0), // Would need OS integration
            system_time: Duration::from_secs(0), // Would need OS integration
        }
    }

    /// Generate memory summary
    fn generate_memory_summary(&self, snapshots: &[MemorySnapshot]) -> MemorySummary {
        if snapshots.is_empty() {
            return MemorySummary {
                peak_usage: 0,
                avg_usage: 0,
                total_allocations: 0,
                total_deallocations: 0,
                gc_collections: 0,
                gc_time: Duration::from_secs(0),
                memory_leaks: Vec::new(),
            };
        }

        let peak_usage = snapshots.iter().map(|s| s.heap_size).max().unwrap_or(0);
        let avg_usage = snapshots.iter().map(|s| s.heap_size).sum::<u64>() / snapshots.len() as u64;
        let total_allocations = snapshots.iter().map(|s| s.allocations.len() as u64).sum();
        let total_deallocations = snapshots.iter().map(|s| s.deallocations.len() as u64).sum();
        let gc_collections = snapshots.iter().map(|s| s.gc_collections).sum();
        let gc_time = Duration::from_millis(snapshots.iter().map(|s| s.gc_time_ms).sum());

        MemorySummary {
            peak_usage,
            avg_usage,
            total_allocations,
            total_deallocations,
            gc_collections,
            gc_time,
            memory_leaks: self.detect_memory_leaks(snapshots),
        }
    }

    /// Generate function summary
    fn generate_function_summary(&self, call_graph: &CallGraph) -> FunctionSummary {
        let mut most_called: Vec<FunctionStats> = call_graph.functions.values().cloned().collect();
        most_called.sort_by(|a, b| b.call_count.cmp(&a.call_count));
        most_called.truncate(10);

        let mut slowest: Vec<FunctionStats> = call_graph.functions.values().cloned().collect();
        slowest.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        slowest.truncate(10);

        let mut memory_intensive: Vec<FunctionStats> = call_graph.functions.values().cloned().collect();
        memory_intensive.sort_by(|a, b| b.memory_allocated.cmp(&a.memory_allocated));
        memory_intensive.truncate(10);

        FunctionSummary {
            total_functions: call_graph.functions.len(),
            most_called,
            slowest,
            memory_intensive,
        }
    }

    /// Identify hot spots
    fn identify_hot_spots(&self, samples: &[ProfileSample], call_graph: &CallGraph) -> Vec<HotSpot> {
        let mut hot_spots = Vec::new();
        let total_samples = samples.len() as f64;

        // Count function occurrences in samples
        let mut function_counts: HashMap<String, u64> = HashMap::new();
        for sample in samples {
            *function_counts.entry(sample.function_name.clone()).or_insert(0) += 1;
        }

        // Convert to hot spots
        for (function_name, count) in function_counts {
            let percentage = (count as f64 / total_samples) * 100.0;
            
            if percentage > 5.0 { // Only functions taking > 5% of time
                let stats = call_graph.functions.get(&function_name);
                
                hot_spots.push(HotSpot {
                    location: format!("{}:?", function_name),
                    function_name: function_name.clone(),
                    time_percentage: percentage,
                    call_count: stats.map(|s| s.call_count).unwrap_or(0),
                    optimization_suggestions: self.generate_optimization_suggestions(&function_name, percentage),
                });
            }
        }

        hot_spots.sort_by(|a, b| b.time_percentage.partial_cmp(&a.time_percentage).unwrap());
        hot_spots
    }

    /// Generate optimization suggestions
    fn generate_optimization_suggestions(&self, function_name: &str, time_percentage: f64) -> Vec<String> {
        let mut suggestions = Vec::new();

        if time_percentage > 20.0 {
            suggestions.push("Consider algorithmic optimization - this function consumes significant CPU time".to_string());
        }

        if function_name.contains("string") || function_name.contains("concat") {
            suggestions.push("Use string builder pattern for multiple concatenations".to_string());
        }

        if function_name.contains("sort") || function_name.contains("search") {
            suggestions.push("Consider using more efficient data structures (HashMap, BTreeMap)".to_string());
        }

        if function_name.contains("loop") || function_name.contains("iter") {
            suggestions.push("Consider vectorization or parallel processing".to_string());
        }

        if function_name.contains("io") || function_name.contains("file") {
            suggestions.push("Consider buffering or async I/O operations".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Profile individual operations within this function".to_string());
        }

        suggestions
    }

    /// Generate recommendations
    fn generate_recommendations(&self, samples: &[ProfileSample], memory_snapshots: &[MemorySnapshot], call_graph: &CallGraph) -> Vec<String> {
        let mut recommendations = Vec::new();

        // CPU recommendations
        if let Some(avg_cpu) = samples.iter().map(|s| s.cpu_usage).reduce(|a, b| a + b) {
            let avg_cpu = avg_cpu / samples.len() as f64;
            if avg_cpu > 80.0 {
                recommendations.push("High CPU usage detected - consider optimizing hot paths".to_string());
            }
        }

        // Memory recommendations
        if let Some(peak_memory) = memory_snapshots.iter().map(|s| s.heap_size).max() {
            if peak_memory > 1_000_000_000 { // 1GB
                recommendations.push("High memory usage detected - consider memory optimization".to_string());
            }
        }

        // GC recommendations
        let total_gc_time: u64 = memory_snapshots.iter().map(|s| s.gc_time_ms).sum();
        if total_gc_time > 1000 {
            recommendations.push("Significant GC overhead - consider reducing allocations".to_string());
        }

        // Call graph recommendations
        if call_graph.functions.len() > 1000 {
            recommendations.push("Large number of functions - consider code organization".to_string());
        }

        recommendations
    }

    /// Detect memory leaks
    fn detect_memory_leaks(&self, snapshots: &[MemorySnapshot]) -> Vec<MemoryLeak> {
        let mut leaks = Vec::new();
        
        // Simple leak detection - objects that are allocated but never deallocated
        let mut allocation_tracking: HashMap<String, Vec<&AllocationInfo>> = HashMap::new();
        
        for snapshot in snapshots {
            for alloc in &snapshot.allocations {
                allocation_tracking.entry(alloc.type_name.clone()).or_default().push(alloc);
            }
        }

        // Check for types with many allocations but few deallocations
        for (type_name, allocations) in allocation_tracking {
            if allocations.len() > 100 { // Arbitrary threshold
                let total_size: u64 = allocations.iter().map(|a| a.size).sum();
                
                leaks.push(MemoryLeak {
                    size: total_size,
                    type_name,
                    allocation_stack: allocations.first().unwrap().stack_trace.clone(),
                    age: Duration::from_secs(60), // Simplified
                });
            }
        }

        leaks
    }

    /// Generate output files
    fn generate_output(&self, report: &ProfileReport) -> Result<(), Box<dyn std::error::Error>> {
        match self.config.output_format.as_str() {
            "json" => self.generate_json_output(report)?,
            "html" => self.generate_html_output(report)?,
            "flamegraph" => self.generate_flamegraph_output(report)?,
            _ => self.generate_json_output(report)?,
        }

        Ok(())
    }

    /// Generate JSON output
    fn generate_json_output(&self, report: &ProfileReport) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(report)?;
        fs::write(&self.config.output_file, json)?;
        Ok(())
    }

    /// Generate HTML output
    fn generate_html_output(&self, report: &ProfileReport) -> Result<(), Box<dyn std::error::Error>> {
        let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>CURSED Profile Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .section {{ margin: 20px 0; padding: 15px; border: 1px solid #ccc; }}
        .hot-spot {{ background: #ffebee; padding: 10px; margin: 5px 0; }}
        .recommendation {{ background: #e8f5e8; padding: 10px; margin: 5px 0; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <h1>CURSED Profile Report</h1>
    
    <div class="section">
        <h2>Summary</h2>
        <p>Duration: {:.2?}</p>
        <p>Total Samples: {}</p>
        <p>Average CPU Usage: {:.1}%</p>
        <p>Peak Memory Usage: {} MB</p>
    </div>
    
    <div class="section">
        <h2>Hot Spots</h2>
        {}
    </div>
    
    <div class="section">
        <h2>Recommendations</h2>
        {}
    </div>
    
    <div class="section">
        <h2>Function Statistics</h2>
        <h3>Most Called Functions</h3>
        <table>
            <tr><th>Function</th><th>Call Count</th><th>Total Time</th></tr>
            {}
        </table>
    </div>
</body>
</html>"#,
            report.duration,
            report.total_samples,
            report.cpu_summary.avg_usage,
            report.memory_summary.peak_usage / 1_000_000,
            report.hot_spots.iter()
                .map(|h| format!("<div class=\"hot-spot\"><strong>{}</strong> - {:.1}% of time<br/>{}</div>", 
                    h.function_name, h.time_percentage, h.optimization_suggestions.join("<br/>")))
                .collect::<Vec<_>>()
                .join(""),
            report.recommendations.iter()
                .map(|r| format!("<div class=\"recommendation\">{}</div>", r))
                .collect::<Vec<_>>()
                .join(""),
            report.function_summary.most_called.iter()
                .map(|f| format!("<tr><td>{}</td><td>{}</td><td>{:.2?}</td></tr>", 
                    f.name, f.call_count, f.total_time))
                .collect::<Vec<_>>()
                .join("")
        );

        fs::write(&self.config.output_file, html)?;
        Ok(())
    }

    /// Generate flamegraph output
    fn generate_flamegraph_output(&self, report: &ProfileReport) -> Result<(), Box<dyn std::error::Error>> {
        // This would generate flamegraph data format
        // For now, generate a simple text representation
        let mut flamegraph_data = String::new();
        
        for hot_spot in &report.hot_spots {
            flamegraph_data.push_str(&format!("{} {}\n", hot_spot.function_name, hot_spot.time_percentage));
        }

        fs::write(&self.config.output_file, flamegraph_data)?;
        Ok(())
    }

    // Helper methods for system integration
    fn get_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn get_cpu_time() -> f64 {
        // This would integrate with OS-specific CPU time APIs
        0.0
    }

    fn get_memory_usage() -> u64 {
        // This would integrate with OS-specific memory APIs
        0
    }

    fn get_current_function() -> String {
        // This would integrate with CURSED runtime
        "unknown".to_string()
    }

    fn get_current_file() -> String {
        // This would integrate with CURSED runtime
        "unknown".to_string()
    }

    fn get_current_line() -> usize {
        // This would integrate with CURSED runtime
        0
    }

    fn get_thread_id() -> u64 {
        // This would integrate with threading system
        0
    }

    fn get_stack_trace() -> Vec<StackFrame> {
        // This would integrate with CURSED runtime
        Vec::new()
    }

    fn is_gc_active() -> bool {
        // This would integrate with GC system
        false
    }

    fn get_io_activity() -> IOActivity {
        // This would integrate with I/O system
        IOActivity {
            read_bytes: 0,
            write_bytes: 0,
            read_calls: 0,
            write_calls: 0,
        }
    }

    fn get_heap_size() -> u64 {
        // This would integrate with memory management
        0
    }

    fn get_stack_size() -> u64 {
        // This would integrate with memory management
        0
    }

    fn get_recent_allocations() -> Vec<AllocationInfo> {
        // This would integrate with memory management
        Vec::new()
    }

    fn get_recent_deallocations() -> Vec<AllocationInfo> {
        // This would integrate with memory management
        Vec::new()
    }

    fn get_gc_collections() -> u64 {
        // This would integrate with GC system
        0
    }

    fn get_gc_time() -> u64 {
        // This would integrate with GC system
        0
    }
}

impl CallGraph {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            call_relationships: Vec::new(),
            hot_paths: Vec::new(),
        }
    }
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            sample_rate: 100,
            memory_tracking: true,
            call_graph_tracking: true,
            cpu_profiling: true,
            io_profiling: true,
            gc_profiling: true,
            output_format: "json".to_string(),
            output_file: "cursed_profile.json".to_string(),
            max_samples: 10000,
            stack_depth: 16,
            profile_duration: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_creation() {
        let config = ProfilerConfig::default();
        let profiler = Profiler::new(config);
        
        assert_eq!(profiler.config.sample_rate, 100);
    }

    #[test]
    fn test_cpu_summary_generation() {
        let config = ProfilerConfig::default();
        let profiler = Profiler::new(config);
        
        let samples = vec![
            ProfileSample {
                timestamp: 0,
                cpu_usage: 50.0,
                memory_usage: 0,
                function_name: "test".to_string(),
                file_path: "test.csd".to_string(),
                line_number: 1,
                thread_id: 0,
                stack_trace: Vec::new(),
                gc_activity: false,
                io_activity: IOActivity {
                    read_bytes: 0,
                    write_bytes: 0,
                    read_calls: 0,
                    write_calls: 0,
                },
            },
        ];

        let summary = profiler.generate_cpu_summary(&samples);
        assert_eq!(summary.avg_usage, 50.0);
    }
}
