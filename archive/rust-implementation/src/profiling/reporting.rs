//! Report generation for profiling data

use std::time::Duration;
use std::collections::HashMap;
use crate::error::CursedError;
use crate::profiling::core::ProfileData;
use crate::profiling::performance::{CompilationPhase, PhaseMetrics};
use crate::profiling::memory::{GcStatistics, AllocationStats};

#[derive(Debug, Clone)]
pub struct ReportGenerator {
    config: ReportConfiguration,
}

#[derive(Debug, Clone)]
pub struct ReportConfiguration {
    pub output_format: OutputFormat,
    pub include_memory: bool,
    pub include_performance: bool,
    pub include_gc_stats: bool,
    pub detailed_breakdown: bool,
    pub sort_by: SortCriteria,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Html,
    Markdown,
}

#[derive(Debug, Clone)]
pub enum SortCriteria {
    Duration,
    MemoryUsage,
    CallCount,
    Alphabetical,
}

#[derive(Debug, Clone)]
pub struct ComprehensiveReport {
    pub performance_data: HashMap<CompilationPhase, PhaseMetrics>,
    pub memory_stats: MemoryReport,
    pub gc_statistics: GcStatistics,
    pub allocation_histogram: HashMap<String, AllocationStats>,
    pub total_duration: Duration,
    pub peak_memory: usize,
}

#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub leak_count: usize,
    pub average_allocation_size: usize,
}

impl ReportGenerator {
    pub fn new(config: ReportConfiguration) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self {
            config: ReportConfiguration::default(),
        }
    }

    pub fn generate_comprehensive_report(&self, report_data: &ComprehensiveReport) -> Result<String, CursedError> {
        match self.config.output_format {
            OutputFormat::Text => Ok(self.generate_text_report(report_data)),
            OutputFormat::Json => Ok(self.generate_json_report(report_data)),
            OutputFormat::Csv => Ok(self.generate_csv_report(report_data)),
            OutputFormat::Html => Ok(self.generate_html_report(report_data)),
            OutputFormat::Markdown => Ok(self.generate_markdown_report(report_data)),
        }
    }

    pub fn generate_performance_summary(&self, performance_data: &HashMap<CompilationPhase, PhaseMetrics>) -> String {
        let mut summary = String::new();
        
        if self.config.output_format == OutputFormat::Text {
            summary.push_str("=== Performance Summary ===\n");
            
            let mut sorted_phases: Vec<_> = performance_data.iter().collect();
            match self.config.sort_by {
                SortCriteria::Duration => sorted_phases.sort_by(|a, b| b.1.duration.cmp(&a.1.duration)),
                SortCriteria::MemoryUsage => sorted_phases.sort_by(|a, b| b.1.memory_usage.cmp(&a.1.memory_usage)),
                SortCriteria::CallCount => sorted_phases.sort_by(|a, b| b.1.iterations.cmp(&a.1.iterations)),
                SortCriteria::Alphabetical => sorted_phases.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0))),
            }
            
            for (phase, metrics) in sorted_phases {
                summary.push_str(&format!(
                    "{:15} {:>8.2}ms {:>8} bytes {:>6} calls\n",
                    format!("{:?}", phase),
                    metrics.duration.as_secs_f64() * 1000.0,
                    metrics.memory_usage,
                    metrics.iterations
                ));
            }
        }
        
        summary
    }

    pub fn generate_memory_summary(&self, memory_report: &MemoryReport) -> String {
        let mut summary = String::new();
        
        if self.config.output_format == OutputFormat::Text {
            summary.push_str("=== Memory Summary ===\n");
            summary.push_str(&format!("Total Allocated: {} bytes\n", memory_report.total_allocated));
            summary.push_str(&format!("Total Freed:     {} bytes\n", memory_report.total_freed));
            summary.push_str(&format!("Current Usage:   {} bytes\n", memory_report.current_usage));
            summary.push_str(&format!("Peak Usage:      {} bytes\n", memory_report.peak_usage));
            summary.push_str(&format!("Memory Leaks:    {}\n", memory_report.leak_count));
            summary.push_str(&format!("Avg Alloc Size:  {} bytes\n", memory_report.average_allocation_size));
        }
        
        summary
    }

    fn generate_text_report(&self, report: &ComprehensiveReport) -> String {
        let mut output = String::new();
        
        output.push_str("==========================================\n");
        output.push_str("         CURSED PROFILING REPORT\n");
        output.push_str("==========================================\n\n");
        
        // Overall summary
        output.push_str(&format!("Total Compilation Time: {:.2}ms\n", report.total_duration.as_secs_f64() * 1000.0));
        output.push_str(&format!("Peak Memory Usage: {} bytes\n\n", report.peak_memory));
        
        if self.config.include_performance {
            output.push_str(&self.generate_performance_summary(&report.performance_data));
            output.push('\n');
        }
        
        if self.config.include_memory {
            output.push_str(&self.generate_memory_summary(&report.memory_stats));
            output.push('\n');
        }
        
        if self.config.include_gc_stats {
            output.push_str("=== Garbage Collection ===\n");
            output.push_str(&format!("Total Collections: {}\n", report.gc_statistics.total_collections));
            output.push_str(&format!("GC Time:          {:.2}ms\n", report.gc_statistics.total_collection_time.as_secs_f64() * 1000.0));
            output.push_str(&format!("Avg GC Time:      {:.2}ms\n", report.gc_statistics.average_collection_time.as_secs_f64() * 1000.0));
            output.push_str(&format!("Bytes Collected:  {}\n", report.gc_statistics.total_bytes_collected));
            output.push('\n');
        }
        
        if self.config.detailed_breakdown && !report.allocation_histogram.is_empty() {
            output.push_str("=== Allocation Breakdown ===\n");
            let mut sorted_types: Vec<_> = report.allocation_histogram.iter().collect();
            sorted_types.sort_by(|a, b| b.1.total_size.cmp(&a.1.total_size));
            
            for (type_name, stats) in sorted_types {
                output.push_str(&format!(
                    "{:20} {:>6} allocs {:>10} bytes avg:{:>6} bytes\n",
                    type_name,
                    stats.count,
                    stats.total_size,
                    stats.average_size
                ));
            }
        }
        
        output
    }

    fn generate_json_report(&self, report: &ComprehensiveReport) -> String {
        let mut json = String::from("{\n");
        json.push_str(&format!("  \"total_duration_ms\": {},\n", report.total_duration.as_millis()));
        json.push_str(&format!("  \"peak_memory_bytes\": {},\n", report.peak_memory));
        
        if self.config.include_performance {
            json.push_str("  \"performance\": {\n");
            let mut first = true;
            for (phase, metrics) in &report.performance_data {
                if !first { json.push_str(",\n"); }
                json.push_str(&format!(
                    "    \"{:?}\": {{\"duration_ms\": {}, \"memory_bytes\": {}, \"iterations\": {}}}",
                    phase, metrics.duration.as_millis(), metrics.memory_usage, metrics.iterations
                ));
                first = false;
            }
            json.push_str("\n  },\n");
        }
        
        if self.config.include_memory {
            json.push_str("  \"memory\": {\n");
            json.push_str(&format!("    \"total_allocated\": {},\n", report.memory_stats.total_allocated));
            json.push_str(&format!("    \"total_freed\": {},\n", report.memory_stats.total_freed));
            json.push_str(&format!("    \"current_usage\": {},\n", report.memory_stats.current_usage));
            json.push_str(&format!("    \"peak_usage\": {},\n", report.memory_stats.peak_usage));
            json.push_str(&format!("    \"leak_count\": {}\n", report.memory_stats.leak_count));
            json.push_str("  },\n");
        }
        
        if self.config.include_gc_stats {
            json.push_str("  \"gc_statistics\": {\n");
            json.push_str(&format!("    \"total_collections\": {},\n", report.gc_statistics.total_collections));
            json.push_str(&format!("    \"total_time_ms\": {},\n", report.gc_statistics.total_collection_time.as_millis()));
            json.push_str(&format!("    \"average_time_ms\": {},\n", report.gc_statistics.average_collection_time.as_millis()));
            json.push_str(&format!("    \"bytes_collected\": {}\n", report.gc_statistics.total_bytes_collected));
            json.push_str("  }\n");
        } else {
            // Remove trailing comma
            json.pop();
            json.pop();
            json.push('\n');
        }
        
        json.push_str("}\n");
        json
    }

    fn generate_csv_report(&self, report: &ComprehensiveReport) -> String {
        let mut csv = String::from("Metric,Value\n");
        csv.push_str(&format!("Total Duration (ms),{}\n", report.total_duration.as_millis()));
        csv.push_str(&format!("Peak Memory (bytes),{}\n", report.peak_memory));
        
        if self.config.include_performance {
            csv.push_str("\nPhase,Duration(ms),Memory(bytes),Iterations\n");
            for (phase, metrics) in &report.performance_data {
                csv.push_str(&format!(
                    "{:?},{},{},{}\n",
                    phase, metrics.duration.as_millis(), metrics.memory_usage, metrics.iterations
                ));
            }
        }
        
        csv
    }

    fn generate_html_report(&self, report: &ComprehensiveReport) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html><head>\n");
        html.push_str("<title>CURSED Profiling Report</title>\n");
        html.push_str("<style>body{font-family:Arial,sans-serif}table{border-collapse:collapse;width:100%}th,td{border:1px solid #ddd;padding:8px;text-align:left}th{background-color:#f2f2f2}</style>\n");
        html.push_str("</head><body>\n");
        html.push_str("<h1>CURSED Profiling Report</h1>\n");
        
        html.push_str("<h2>Summary</h2>\n");
        html.push_str(&format!("<p>Total Duration: {:.2}ms</p>\n", report.total_duration.as_secs_f64() * 1000.0));
        html.push_str(&format!("<p>Peak Memory: {} bytes</p>\n", report.peak_memory));
        
        if self.config.include_performance {
            html.push_str("<h2>Performance Breakdown</h2>\n");
            html.push_str("<table>\n<tr><th>Phase</th><th>Duration</th><th>Memory</th><th>Iterations</th></tr>\n");
            for (phase, metrics) in &report.performance_data {
                html.push_str(&format!(
                    "<tr><td>{:?}</td><td>{:.2}ms</td><td>{} bytes</td><td>{}</td></tr>\n",
                    phase, metrics.duration.as_secs_f64() * 1000.0, metrics.memory_usage, metrics.iterations
                ));
            }
            html.push_str("</table>\n");
        }
        
        html.push_str("</body></html>\n");
        html
    }

    fn generate_markdown_report(&self, report: &ComprehensiveReport) -> String {
        let mut md = String::from("# CURSED Profiling Report\n\n");
        
        md.push_str("## Summary\n\n");
        md.push_str(&format!("- **Total Duration**: {:.2}ms\n", report.total_duration.as_secs_f64() * 1000.0));
        md.push_str(&format!("- **Peak Memory**: {} bytes\n\n", report.peak_memory));
        
        if self.config.include_performance {
            md.push_str("## Performance Breakdown\n\n");
            md.push_str("| Phase | Duration | Memory | Iterations |\n");
            md.push_str("|-------|----------|--------|------------|\n");
            for (phase, metrics) in &report.performance_data {
                md.push_str(&format!(
                    "| {:?} | {:.2}ms | {} bytes | {} |\n",
                    phase, metrics.duration.as_secs_f64() * 1000.0, metrics.memory_usage, metrics.iterations
                ));
            }
            md.push('\n');
        }
        
        md
    }
}

impl Default for ReportConfiguration {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Text,
            include_memory: true,
            include_performance: true,
            include_gc_stats: true,
            detailed_breakdown: false,
            sort_by: SortCriteria::Duration,
        }
    }
}

impl PartialEq for OutputFormat {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (OutputFormat::Text, OutputFormat::Text) |
            (OutputFormat::Json, OutputFormat::Json) |
            (OutputFormat::Csv, OutputFormat::Csv) |
            (OutputFormat::Html, OutputFormat::Html) |
            (OutputFormat::Markdown, OutputFormat::Markdown)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_report_generation() {
        let generator = ReportGenerator::with_default_config();
        let mut performance_data = HashMap::new();
        
        let metrics = PhaseMetrics {
            start_time: std::time::Instant::now(),
            duration: Duration::from_millis(100),
            memory_usage: 1024,
            iterations: 5,
        };
        performance_data.insert(CompilationPhase::Parsing, metrics);
        
        let summary = generator.generate_performance_summary(&performance_data);
        assert!(summary.contains("Parsing"));
        assert!(summary.contains("Performance Summary"));
    }
}
