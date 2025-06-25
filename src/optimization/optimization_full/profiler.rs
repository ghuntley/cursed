// Enhanced build profiler for compilation performance analysis

use crate::error::{CursedError, Result};
use crate::optimization::metrics::{CompilationUnit, ResourceStatistics};

use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use std::collections::HashMap;
use tracing::{info, debug, instrument};
use serde::{Deserialize, Serialize};

/// Report output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    /// JSON format for programmatic consumption
    /// HTML format for web viewing
    /// Markdown format for documentation
    /// Interactive terminal format
impl Default for ReportFormat {
    fn default() -> Self {
        Self::Html
    }
}

/// Configuration for the enhanced build profiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Profiling session for tracking build operations
#[derive(Debug, Clone)]
pub struct ProfileSession {
/// Result from profiling a compilation unit
#[derive(Debug, Clone)]
pub struct CompilationUnitResult {
/// Performance summary for a profiling session
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
/// System metrics collected during profiling
#[derive(Debug, Clone)]
pub struct SystemMetrics {
impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
        }
    }
/// Comprehensive build profiling report
#[derive(Debug, Clone)]
pub struct ProfileReport {
/// Resource usage snapshot at a point in time
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub timestamp: Duration, // Relative to session start
/// Enhanced build profiler
#[derive(Debug)]
pub struct EnhancedBuildProfiler {
/// Internal session data tracking
#[derive(Debug)]
struct SessionData {
impl EnhancedBuildProfiler {
    /// Create a new enhanced build profiler
    #[instrument]
    pub fn new(config: ProfilerConfig) -> Result<Self> {
        info!("Creating enhanced build profiler");
        
        // Validate configuration
        if config.monitoring_interval_ms == 0 {
            return Err(CursedError::optimization_error(
                "Monitoring interval must be greater than 0"
            ));
        if config.max_profile_entries == 0 {
            return Err(CursedError::optimization_error(
                "Max profile entries must be greater than 0"
            ));
        Ok(Self {
        })
    /// Start a new build profiling session
    #[instrument(skip(self))]
    pub fn start_build_session(&mut self, session_name: String) -> Result<ProfileSession> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        let session = ProfileSession {

        info!("Starting profiling session: {}", session.id);

        // Initialize session data
        let session_data = SessionData {

        self.active_sessions.insert(session.id.clone(), session.clone());
        self.session_data.insert(session.id.clone(), session_data);

        Ok(session)
    /// End a build profiling session and generate report
    #[instrument(skip(self))]
    pub fn end_build_session(&mut self, session: ProfileSession) -> Result<ProfileReport> {
        info!("Ending profiling session: {}", session.id);

        let session_data = self.session_data.remove(&session.id)
            .ok_or_else(|| CursedError::optimization_error(
                &format!("Session data not found: {}", session.id)
            ))?;

        self.active_sessions.remove(&session.id);

        let total_duration = session.start_time.elapsed();
        
        // Generate performance summary
        let performance_summary = self.generate_performance_summary(&session_data, total_duration);
        
        // Generate system metrics
        let system_metrics = self.generate_system_metrics(&session_data);

        let report = ProfileReport {

        // Auto-export if configured
        if self.config.auto_export_reports {
            if let Some(ref output_dir) = self.config.output_directory {
                    self.get_file_extension()
                ));
                let _ = self.export_report(&report, self.config.report_format.clone(), report_path);
            }
        }

        Ok(report)
    /// Profile a compilation unit
    #[instrument(skip(self, unit, session))]
    pub fn profile_compilation_unit(
    ) -> Result<CompilationUnitResult> {
        debug!("Profiling compilation unit: {}", unit.name);

        let session_data = self.session_data.get_mut(&session.id)
            .ok_or_else(|| CursedError::optimization_error(
                &format!("Session not found: {}", session.id)
            ))?;

        let start_time = Instant::now();
        let start_memory = self.get_current_memory_usage();
        let start_cpu = self.get_current_cpu_usage();

        // Simulate compilation work
        std::thread::sleep(Duration::from_millis(50));

        let compilation_time = start_time.elapsed();
        let peak_memory_mb = self.get_current_memory_usage().max(start_memory);
        let end_cpu = self.get_current_cpu_usage();
        let average_cpu_percent = (start_cpu + end_cpu) / 2.0;

        // Update peak memory for session
        session_data.peak_memory = session_data.peak_memory.max(peak_memory_mb);

        let result = CompilationUnitResult {

        session_data.compilation_units.push(result.clone());

        // Add resource snapshot
        if self.config.enable_realtime_monitoring {
            let snapshot = ResourceSnapshot {
                io_rate: 1000.0, // Simulated
            session_data.resource_snapshots.push(snapshot);
        Ok(result)
    /// Export a profiling report to a file
    #[instrument(skip(self, report))]
    pub fn export_report(
    ) -> Result<()> {
        info!("Exporting report to: {:?}", output_path);

        let content = match format {

        std::fs::write(&output_path, content).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write report: {}", e))
        })?;

        info!("Report exported successfully");
        Ok(())
    /// Generate performance summary from session data
    fn generate_performance_summary(
    ) -> PerformanceSummary {
        let unit_count = session_data.compilation_units.len();
        
        // Calculate compilation efficiency (units per second)
        let compilation_efficiency = if total_duration.as_secs() > 0 {
            unit_count as f64 / total_duration.as_secs() as f64
        } else {
            unit_count as f64

        // Calculate memory efficiency (inverse of peak memory usage)
        let memory_efficiency = if session_data.peak_memory > 0.0 {
            100.0 / session_data.peak_memory.max(1.0)
        } else {
            100.0

        // Calculate average CPU utilization
        let cpu_utilization = if !session_data.compilation_units.is_empty() {
            session_data.compilation_units.iter()
                .map(|unit| unit.average_cpu_percent)
                .sum::<f64>() / unit_count as f64
        } else {
            0.0

        // Calculate I/O efficiency
        let total_io = session_data.compilation_units.iter()
            .map(|unit| unit.io_operations)
            .sum::<u64>();
        let io_efficiency = if total_duration.as_millis() > 0 {
            total_io as f64 / total_duration.as_millis() as f64
        } else {
            0.0

        // Overall performance score (weighted combination)
        let overall_performance_score = (
            compilation_efficiency * 0.4 +
            memory_efficiency * 0.2 +
            cpu_utilization * 0.2 +
            io_efficiency * 0.2
        ).min(100.0);

        // Generate bottlenecks and recommendations
        let mut bottlenecks = Vec::new();
        let mut recommendations = Vec::new();

        if memory_efficiency < 50.0 {
            bottlenecks.push("High memory usage".to_string());
            recommendations.push("Consider reducing memory allocation or enabling memory optimizations".to_string());
        if cpu_utilization < 30.0 {
            bottlenecks.push("Low CPU utilization".to_string());
            recommendations.push("Enable parallel compilation to improve CPU utilization".to_string());
        if compilation_efficiency < 1.0 {
            bottlenecks.push("Slow compilation speed".to_string());
            recommendations.push("Enable incremental compilation and caching".to_string());
        PerformanceSummary {
        }
    }

    /// Generate system metrics from session data
    fn generate_system_metrics(&self, session_data: &SessionData) -> SystemMetrics {
        let unit_count = session_data.compilation_units.len();
        
        let peak_memory_mb = session_data.peak_memory;
        let average_memory_mb = if !session_data.compilation_units.is_empty() {
            session_data.compilation_units.iter()
                .map(|unit| unit.peak_memory_mb)
                .sum::<f64>() / unit_count as f64
        } else {
            0.0

        let peak_cpu_percent = session_data.compilation_units.iter()
            .map(|unit| unit.average_cpu_percent)
            .fold(0.0, f64::max);
        
        let average_cpu_percent = if !session_data.compilation_units.is_empty() {
            session_data.compilation_units.iter()
                .map(|unit| unit.average_cpu_percent)
                .sum::<f64>() / unit_count as f64
        } else {
            0.0

        let total_io_operations = session_data.compilation_units.iter()
            .map(|unit| unit.io_operations)
            .sum::<u64>();

        SystemMetrics {
            network_bytes_transferred: 0, // Not tracked in this simulation
        }
    }

    /// Generate JSON report content
    fn generate_json_report(&self, report: &ProfileReport) -> Result<String> {
        serde_json::to_string_pretty(report).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to serialize JSON report: {}", e))
        })
    /// Generate HTML report content
    fn generate_html_report(&self, report: &ProfileReport) -> Result<String> {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Build Profile Report - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background-color: #f0f0f0; padding: 10px; border-radius: 5px; }}
        .metric {{ margin: 10px 0; }}
        .warning {{ color: orange; }}
        .error {{ color: red; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Build Profile Report</h1>
        <p><strong>Session:</strong> {}</p>
        <p><strong>Duration:</strong> {:.2?}</p>
        <p><strong>Performance Score:</strong> {:.1}</p>
    </div>
    
    <h2>System Metrics</h2>
    <div class="metric">Peak Memory: {:.1} MB</div>
    <div class="metric">Average CPU: {:.1}%</div>
    <div class="metric">I/O Operations: {}</div>
    
    <h2>Compilation Units</h2>
    <table>
        <tr>
            <th>Unit Name</th>
            <th>Compilation Time</th>
            <th>Peak Memory (MB)</th>
            <th>CPU Usage (%)</th>
        </tr>
        {}
    </table>
</body>
</html>"#,
            report.compilation_units.iter()
                .map(|unit| format!(
                    "<tr><td>{}</td><td>{:.2?}</td><td>{:.1}</td><td>{:.1}</td></tr>",
                    unit.average_cpu_percent
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(html)
    /// Generate Markdown report content
    fn generate_markdown_report(&self, report: &ProfileReport) -> Result<String> {
        let markdown = format!(
**Session:** {}  
**Duration:** {:.2?}  
**Performance Score:** {:.1}  

## System Metrics

- **Peak Memory:** {:.1} MB
- **Average CPU:** {:.1}%
- **I/O Operations:** {}

## Compilation Units

| Unit Name | Compilation Time | Peak Memory (MB) | CPU Usage (%) |
|-----------|------------------|------------------|---------------|
## Performance Summary

- **Compilation Efficiency:** {:.2} units/sec
- **Memory Efficiency:** {:.1}%
- **CPU Utilization:** {:.1}%

## Recommendations

{}
            report.compilation_units.iter()
                .map(|unit| format!(
                    unit.average_cpu_percent
                ))
                .collect::<Vec<_>>()
            report.performance_summary.recommendations.iter()
                .map(|rec| format!("- {}", rec))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(markdown)
    /// Generate interactive report content
    fn generate_interactive_report(&self, report: &ProfileReport) -> Result<String> {
        // For now, just return a simple text format
        Ok(format!(
            report.performance_summary.overall_performance_score
        ))
    /// Get file extension for current report format
    fn get_file_extension(&self) -> &str {
        match self.config.report_format {
        }
    }

    /// Get current memory usage (simulated)
    fn get_current_memory_usage(&self) -> f64 {
        100.0 + (rand::random::<f64>() * 50.0)
    /// Get current CPU usage (simulated)
    fn get_current_cpu_usage(&self) -> f64 {
        20.0 + (rand::random::<f64>() * 30.0)
    /// Get current I/O operations (simulated)
    fn get_current_io_operations(&self) -> u64 {
        rand::random::<u64>() % 1000 + 100
    }
}

// Simple random number generation for simulation
mod rand {
    use std::cell::Cell;
    
    thread_local! {
        static RNG_STATE: Cell<u64> = Cell::new(1);
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        RNG_STATE.with(|state| {
            let current = state.get();
            let next = current.wrapping_mul(1103515245).wrapping_add(12345);
            state.set(next);
            T::from(next)
        })
    }
}

