//! Benchmark result reporters

use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing::{debug, error, info, warn};

use super::harness::{BenchmarkResult, BenchmarkResults};
use super::metrics::{Metric, MetricType, MetricValue};

/// Trait for reporting benchmark results
pub trait BenchmarkReporter: Send + Sync {
    /// Report benchmark results
    fn report(&self, results: &BenchmarkResults);
}

/// Console reporter for benchmark results
pub struct ConsoleReporter {
    /// Whether to include detailed metrics
    pub verbose: bool,
}

impl ConsoleReporter {
    /// Create a new console reporter
    pub fn new() -> Self {
        Self { verbose: false }
    }
    
    /// Create a new verbose console reporter
    pub fn verbose() -> Self {
        Self { verbose: true }
    }
}

impl BenchmarkReporter for ConsoleReporter {
    fn report(&self, results: &BenchmarkResults) {
        println!("\n=== Benchmark Results: {} ===", results.suite_name);
        println!("Timestamp: {:?}", results.timestamp);
        println!("{:-^80}", "");
        println!("{:<30} {:>12} {:>12} {:>12}", "Benchmark", "Avg Time", "Min Time", "Max Time");
        println!("{:-^80}", "");
        
        for result in &results.results {
            println!(
                "{:<30} {:>12?} {:>12?} {:>12?}",
                result.name,
                result.avg_time,
                result.min_time,
                result.max_time
            );
            
            if self.verbose {
                // Group metrics by type
                let mut timing_metrics = Vec::new();
                let mut memory_metrics = Vec::new();
                let mut gc_metrics = Vec::new();
                let mut throughput_metrics = Vec::new();
                let mut other_metrics = Vec::new();
                
                for metric in &result.metrics {
                    match metric.metric_type() {
                        MetricType::Timing => timing_metrics.push(metric),
                        MetricType::Memory => memory_metrics.push(metric),
                        MetricType::GarbageCollection => gc_metrics.push(metric),
                        MetricType::Throughput => throughput_metrics.push(metric),
                        _ => other_metrics.push(metric),
                    }
                }
                
                // Print metrics if any exist
                if !timing_metrics.is_empty() {
                    println!("  Timing Metrics:");
                    for metric in timing_metrics {
                        println!("    {}: {} {}", metric.name(), metric.value(), metric.unit());
                    }
                }
                
                if !memory_metrics.is_empty() {
                    println!("  Memory Metrics:");
                    for metric in memory_metrics {
                        println!("    {}: {} {}", metric.name(), metric.value(), metric.unit());
                    }
                }
                
                if !gc_metrics.is_empty() {
                    println!("  GC Metrics:");
                    for metric in gc_metrics {
                        println!("    {}: {} {}", metric.name(), metric.value(), metric.unit());
                    }
                }
                
                if !throughput_metrics.is_empty() {
                    println!("  Throughput Metrics:");
                    for metric in throughput_metrics {
                        println!("    {}: {} {}/s", metric.name(), metric.value(), metric.unit());
                    }
                }
                
                if !other_metrics.is_empty() {
                    println!("  Other Metrics:");
                    for metric in other_metrics {
                        println!("    {}: {} {}", metric.name(), metric.value(), metric.unit());
                    }
                }
            }
        }
        
        println!("{:-^80}", "");
    }
}

/// JSON reporter for benchmark results
pub struct JsonReporter {
    /// File path to write JSON results
    pub file_path: String,
}

impl JsonReporter {
    /// Create a new JSON reporter
    pub fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}

impl BenchmarkReporter for JsonReporter {
    fn report(&self, results: &BenchmarkResults) {
        // Simple JSON serialization
        let mut json = String::new();
        json.push_str("{");
        json.push_str(&format!("\"suite_name\": \"{}\",", results.suite_name));
        json.push_str(&format!("\"timestamp\": \"{:?}\",", results.timestamp));
        json.push_str("\"results\": [");
        
        for (i, result) in results.results.iter().enumerate() {
            if i > 0 {
                json.push_str(",");
            }
            json.push_str("{");
            json.push_str(&format!("\"name\": \"{}\",", result.name));
            json.push_str(&format!("\"avg_time_ns\": {},", result.avg_time.as_nanos()));
            json.push_str(&format!("\"min_time_ns\": {},", result.min_time.as_nanos()));
            json.push_str(&format!("\"max_time_ns\": {},", result.max_time.as_nanos()));
            json.push_str("\"metrics\": [");
            
            for (j, metric) in result.metrics.iter().enumerate() {
                if j > 0 {
                    json.push_str(",");
                }
                json.push_str("{");
                json.push_str(&format!("\"name\": \"{}\",", metric.name()));
                json.push_str(&format!("\"type\": \"{}\",", metric.metric_type()));
                json.push_str(&format!("\"unit\": \"{}\",", metric.unit()));
                
                match metric.value() {
                    MetricValue::Duration(d) => json.push_str(&format!("\"value\": {}", d.as_nanos())),
                    MetricValue::Integer(i) => json.push_str(&format!("\"value\": {}", i)),
                    MetricValue::UInteger(u) => json.push_str(&format!("\"value\": {}", u)),
                    MetricValue::Float(f) => json.push_str(&format!("\"value\": {}", f)),
                    MetricValue::Boolean(b) => json.push_str(&format!("\"value\": {}", b)),
                    MetricValue::String(s) => json.push_str(&format!("\"value\": \"{}\"", s)),
                }
                
                json.push_str("}");
            }
            
            json.push_str("]");
            json.push_str("}");
        }
        
        json.push_str("]");
        json.push_str("}");
        
        // Write to file
        match File::create(&self.file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json.as_bytes()) {
                    error!("Failed to write JSON report: {}", e);
                } else {
                    info!("JSON report written to {}", self.file_path);
                }
            }
            Err(e) => {
                error!("Failed to create JSON report file: {}", e);
            }
        }
    }
}

/// CSV reporter for benchmark results
pub struct CsvReporter {
    /// File path to write CSV results
    pub file_path: String,
}

impl CsvReporter {
    /// Create a new CSV reporter
    pub fn new(file_path: &str) -> Self {
        Self { file_path: file_path.to_string() }
    }
}

impl BenchmarkReporter for CsvReporter {
    fn report(&self, results: &BenchmarkResults) {
        // Create CSV header
        let mut csv = String::new();
        csv.push_str("Language,Algorithm,Avg Time (ns),Min Time (ns),Max Time (ns)");
        
        // We don't need separate columns for each metric in the simplified format
        // Just ensure we have the standard columns needed
        
        // No additional columns needed in the simplified format
        csv.push_str("\n");
        
        // Add results
        for result in &results.results {
            // Split benchmark name into language and algorithm
            let parts: Vec<&str> = result.name.split('_').collect();
            let (language, algorithm) = if parts.len() >= 2 {
                (parts[0].to_string(), parts[1..].join("_"))
            } else {
                (result.name.clone(), String::new())
            };
            
            csv.push_str(&format!("{},{},{},{},{}",
                language,
                algorithm,
                result.avg_time.as_nanos(),
                result.min_time.as_nanos(),
                result.max_time.as_nanos()
            ));
            
            // We don't need to add metric values in the simplified format
            
            csv.push_str("\n");
        }
        
        // Write to file
        match File::create(&self.file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(csv.as_bytes()) {
                    error!("Failed to write CSV report: {}", e);
                } else {
                    info!("CSV report written to {}", self.file_path);
                }
            }
            Err(e) => {
                error!("Failed to create CSV report file: {}", e);
            }
        }
    }
}