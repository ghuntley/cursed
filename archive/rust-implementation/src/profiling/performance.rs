//! Performance monitoring and compilation phase tracking

use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    phases: HashMap<CompilationPhase, PhaseMetrics>,
    start_time: Option<Instant>,
    config: ReportConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationPhase {
    Lexing,
    Parsing,
    TypeChecking,
    Optimization,
    CodeGeneration,
    Linking,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct PhaseMetrics {
    pub start_time: Instant,
    pub duration: Duration,
    pub memory_usage: usize,
    pub iterations: u64,
}

#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub format: ReportFormat,
    pub include_memory: bool,
    pub include_timing: bool,
    pub detailed: bool,
}

#[derive(Debug, Clone)]
pub enum ReportFormat {
    Text,
    Json,
    Csv,
    Html,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            phases: HashMap::new(),
            start_time: None,
            config: ReportConfig::default(),
        }
    }

    pub fn with_config(config: ReportConfig) -> Self {
        Self {
            phases: HashMap::new(),
            start_time: None,
            config,
        }
    }

    pub fn start_monitoring(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn start_phase(&mut self, phase: CompilationPhase) {
        let metrics = PhaseMetrics {
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            memory_usage: 0,
            iterations: 0,
        };
        self.phases.insert(phase, metrics);
    }

    pub fn end_phase(&mut self, phase: CompilationPhase) -> Result<Duration, CursedError> {
        if let Some(metrics) = self.phases.get_mut(&phase) {
            metrics.duration = metrics.start_time.elapsed();
            Ok(metrics.duration)
        } else {
            Err(CursedError::runtime_error(&format!("Phase {:?} was not started", phase)))
        }
    }

    pub fn record_memory_usage(&mut self, phase: CompilationPhase, memory: usize) {
        if let Some(metrics) = self.phases.get_mut(&phase) {
            metrics.memory_usage = memory;
        }
    }

    pub fn increment_iterations(&mut self, phase: CompilationPhase) {
        if let Some(metrics) = self.phases.get_mut(&phase) {
            metrics.iterations += 1;
        }
    }

    pub fn get_phase_duration(&self, phase: &CompilationPhase) -> Option<Duration> {
        self.phases.get(phase).map(|m| m.duration)
    }

    pub fn get_total_duration(&self) -> Duration {
        self.phases.values().map(|m| m.duration).sum()
    }

    pub fn get_phase_report(&self) -> HashMap<CompilationPhase, PhaseMetrics> {
        self.phases.clone()
    }

    pub fn generate_report(&self) -> String {
        match self.config.format {
            ReportFormat::Text => self.generate_text_report(),
            ReportFormat::Json => self.generate_json_report(),
            ReportFormat::Csv => self.generate_csv_report(),
            ReportFormat::Html => self.generate_html_report(),
        }
    }

    fn generate_text_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Performance Report ===\n");
        
        for (phase, metrics) in &self.phases {
            report.push_str(&format!("{:?}: {:?}", phase, metrics.duration));
            if self.config.include_memory {
                report.push_str(&format!(" (Memory: {} bytes)", metrics.memory_usage));
            }
            report.push('\n');
        }
        
        report.push_str(&format!("Total: {:?}\n", self.get_total_duration()));
        report
    }

    fn generate_json_report(&self) -> String {
        // Simplified JSON generation
        let mut json = String::from("{\n");
        json.push_str("  \"phases\": {\n");
        
        let mut first = true;
        for (phase, metrics) in &self.phases {
            if !first {
                json.push_str(",\n");
            }
            json.push_str(&format!(
                "    \"{:?}\": {{ \"duration_ms\": {}, \"memory_bytes\": {}, \"iterations\": {} }}",
                phase,
                metrics.duration.as_millis(),
                metrics.memory_usage,
                metrics.iterations
            ));
            first = false;
        }
        
        json.push_str("\n  },\n");
        json.push_str(&format!("  \"total_duration_ms\": {}\n", self.get_total_duration().as_millis()));
        json.push_str("}\n");
        json
    }

    fn generate_csv_report(&self) -> String {
        let mut csv = String::from("Phase,Duration(ms),Memory(bytes),Iterations\n");
        for (phase, metrics) in &self.phases {
            csv.push_str(&format!(
                "{:?},{},{},{}\n",
                phase,
                metrics.duration.as_millis(),
                metrics.memory_usage,
                metrics.iterations
            ));
        }
        csv
    }

    fn generate_html_report(&self) -> String {
        let mut html = String::from("<html><head><title>Performance Report</title></head><body>\n");
        html.push_str("<h1>Performance Report</h1>\n");
        html.push_str("<table border='1'>\n");
        html.push_str("<tr><th>Phase</th><th>Duration</th><th>Memory</th><th>Iterations</th></tr>\n");
        
        for (phase, metrics) in &self.phases {
            html.push_str(&format!(
                "<tr><td>{:?}</td><td>{:?}</td><td>{} bytes</td><td>{}</td></tr>\n",
                phase, metrics.duration, metrics.memory_usage, metrics.iterations
            ));
        }
        
        html.push_str("</table>\n");
        html.push_str(&format!("<p>Total Duration: {:?}</p>\n", self.get_total_duration()));
        html.push_str("</body></html>\n");
        html
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Text,
            include_memory: true,
            include_timing: true,
            detailed: false,
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitoring() {
        let mut monitor = PerformanceMonitor::new();
        monitor.start_monitoring();
        
        monitor.start_phase(CompilationPhase::Lexing);
        std::thread::sleep(Duration::from_millis(1));
        monitor.end_phase(CompilationPhase::Lexing).unwrap();
        
        assert!(monitor.get_phase_duration(&CompilationPhase::Lexing).is_some());
        assert!(monitor.get_total_duration() > Duration::new(0, 0));
    }

    #[test]
    fn test_report_generation() {
        let mut monitor = PerformanceMonitor::new();
        monitor.start_phase(CompilationPhase::Parsing);
        monitor.end_phase(CompilationPhase::Parsing).unwrap();
        
        let report = monitor.generate_report();
        assert!(report.contains("Parsing"));
        assert!(report.contains("Performance Report"));
    }
}
