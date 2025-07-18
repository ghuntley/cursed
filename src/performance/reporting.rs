//! Performance reporting module

use crate::error::CursedError;
use crate::performance::{PerformanceConfig, PerformanceMetrics};

pub struct PerformanceReporter {
    config: PerformanceConfig,
}

impl PerformanceReporter {
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        Ok(Self { config })
    }

    pub fn generate_comprehensive_report(&self, metrics: &PerformanceMetrics) -> Result<String, CursedError> {
        let mut report = String::new();
        
        report.push_str("# CURSED Compiler Performance Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().to_rfc3339()));
        
        report.push_str("## Performance Metrics\n");
        report.push_str(&format!("- Compilation Time: {:?}\n", metrics.compilation_time));
        report.push_str(&format!("- Execution Time: {:?}\n", metrics.execution_time));
        report.push_str(&format!("- Memory Usage: {} MB\n", metrics.memory_usage / 1024 / 1024));
        report.push_str(&format!("- CPU Usage: {:.2}%\n", metrics.cpu_usage));
        report.push_str(&format!("- Throughput: {:.2} ops/sec\n", metrics.throughput));
        report.push_str(&format!("- Latency: {:?}\n", metrics.latency));
        report.push_str(&format!("- Error Rate: {:.4}%\n", metrics.error_rate * 100.0));
        report.push_str(&format!("- GC Pressure: {:.2}%\n", metrics.gc_pressure * 100.0));
        
        Ok(report)
    }
}
