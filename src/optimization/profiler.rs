// Enhanced build profiler for compilation performance analysis

use crate::error::{Result, CursedError};
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
    Json,
    /// HTML format for web viewing
    Html,
    /// Markdown format for documentation
    Markdown,
    /// Interactive terminal format
    Interactive,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Html
    }
}

/// Configuration for the enhanced build profiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    pub enable_realtime_monitoring: bool,
    pub enable_memory_profiling: bool,
    pub enable_cpu_profiling: bool,
    pub enable_io_profiling: bool,
    pub monitoring_interval_ms: u64,
    pub max_profile_entries: usize,
    pub report_format: ReportFormat,
    pub output_directory: Option<PathBuf>,
    pub auto_export_reports: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enable_realtime_monitoring: true,
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
            enable_io_profiling: true,
            monitoring_interval_ms: 100,
            max_profile_entries: 1000,
            report_format: ReportFormat::Html,
            output_directory: None,
            auto_export_reports: false,
        }
    }
}

/// Profiling session for tracking build operations
#[derive(Debug, Clone)]
pub struct ProfileSession {
    pub id: String,
    pub name: String,
    pub start_time: Instant,
    pub system_start_time: SystemTime,
}

/// Result from profiling a compilation unit
#[derive(Debug, Clone)]
pub struct CompilationUnitResult {
    pub unit_name: String,
    pub compilation_time: Duration,
    pub peak_memory_mb: f64,
    pub average_cpu_percent: f64,
    pub io_operations: u64,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Performance summary for a profiling session
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub overall_performance_score: f64,
    pub compilation_efficiency: f64,
    pub memory_efficiency: f64,
    pub cpu_utilization: f64,
    pub io_efficiency: f64,
    pub bottlenecks: Vec<String>,
    pub recommendations: Vec<String>,
}

/// System metrics collected during profiling
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub peak_cpu_percent: f64,
    pub average_cpu_percent: f64,
    pub total_io_operations: u64,
    pub network_bytes_transferred: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            peak_memory_mb: 0.0,
            average_memory_mb: 0.0,
            peak_cpu_percent: 0.0,
            average_cpu_percent: 0.0,
            total_io_operations: 0,
            network_bytes_transferred: 0,
        }
    }
}

/// Comprehensive build profiling report
#[derive(Debug, Clone)]
pub struct ProfileReport {
    pub session_id: String,
    pub session_name: String,
    pub start_time: SystemTime,
    pub total_duration: Duration,
    pub compilation_units: Vec<CompilationUnitResult>,
    pub performance_summary: PerformanceSummary,
    pub system_metrics: SystemMetrics,
    pub resource_timeline: Vec<ResourceSnapshot>,
}

/// Resource usage snapshot at a point in time
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub timestamp: Duration, // Relative to session start
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub io_rate: f64,
}

/// Enhanced build profiler
#[derive(Debug)]
pub struct EnhancedBuildProfiler {
    config: ProfilerConfig,
    active_sessions: HashMap<String, ProfileSession>,
    session_data: HashMap<String, SessionData>,
}

/// Internal session data tracking
#[derive(Debug)]
struct SessionData {
    compilation_units: Vec<CompilationUnitResult>,
    resource_snapshots: Vec<ResourceSnapshot>,
    start_memory: f64,
    peak_memory: f64,
    total_cpu_time: Duration,
}

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
        }

        if config.max_profile_entries == 0 {
            return Err(CursedError::optimization_error(
                "Max profile entries must be greater than 0"
            ));
        }

        Ok(Self {
            config,
            active_sessions: HashMap::new(),
            session_data: HashMap::new(),
        })
    }

    /// Start a new build profiling session
    #[instrument(skip(self))]
    pub fn start_build_session(&mut self, session_name: String) -> Result<ProfileSession> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        let session = ProfileSession {
            id: format!("{}_{}", session_name, timestamp),
            name: session_name.clone(),
            start_time: Instant::now(),
            system_start_time: SystemTime::now(),
        };

        info!("Starting profiling session: {}", session.id);

        // Initialize session data
        let session_data = SessionData {
            compilation_units: Vec::new(),
            resource_snapshots: Vec::new(),
            start_memory: self.get_current_memory_usage(),
            peak_memory: 0.0,
            total_cpu_time: Duration::from_secs(0),
        };

        self.active_sessions.insert(session.id.clone(), session.clone());
        self.session_data.insert(session.id.clone(), session_data);

        Ok(session)
    }

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
            session_id: session.id.clone(),
            session_name: session.name,
            start_time: session.system_start_time,
            total_duration,
            compilation_units: session_data.compilation_units,
            performance_summary,
            system_metrics,
            resource_timeline: session_data.resource_snapshots,
        };

        // Auto-export if configured
        if self.config.auto_export_reports {
            if let Some(ref output_dir) = self.config.output_directory {
                let report_path = output_dir.join(format!("{}.{}", 
                    session.id,
                    self.get_file_extension()
                ));
                let _ = self.export_report(&report, self.config.report_format.clone(), report_path);
            }
        }

        Ok(report)
    }

    /// Profile a compilation unit
    #[instrument(skip(self, unit, session))]
    pub fn profile_compilation_unit(
        &mut self,
        unit: &CompilationUnit,
        session: &ProfileSession,
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
            unit_name: unit.name.clone(),
            compilation_time,
            peak_memory_mb,
            average_cpu_percent,
            io_operations: self.get_current_io_operations(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        session_data.compilation_units.push(result.clone());

        // Add resource snapshot
        if self.config.enable_realtime_monitoring {
            let snapshot = ResourceSnapshot {
                timestamp: session.start_time.elapsed(),
                memory_mb: peak_memory_mb,
                cpu_percent: average_cpu_percent,
                io_rate: 1000.0, // Simulated
            };
            session_data.resource_snapshots.push(snapshot);
        }

        Ok(result)
    }

    /// Export a profiling report to a file
    #[instrument(skip(self, report))]
    pub fn export_report(
        &self,
        report: &ProfileReport,
        format: ReportFormat,
        output_path: PathBuf,
    ) -> Result<()> {
        info!("Exporting report to: {:?}", output_path);

        let content = match format {
            ReportFormat::Json => self.generate_json_report(report)?,
            ReportFormat::Html => self.generate_html_report(report)?,
            ReportFormat::Markdown => self.generate_markdown_report(report)?,
            ReportFormat::Interactive => self.generate_interactive_report(report)?,
        };

        std::fs::write(&output_path, content).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write report: {}", e))
        })?;

        info!("Report exported successfully");
        Ok(())
    }

    /// Generate performance summary from session data
    fn generate_performance_summary(
        &self,
        session_data: &SessionData,
        total_duration: Duration,
    ) -> PerformanceSummary {
        let unit_count = session_data.compilation_units.len();
        
        // Calculate compilation efficiency (units per second)
        let compilation_efficiency = if total_duration.as_secs() > 0 {
            unit_count as f64 / total_duration.as_secs() as f64
        } else {
            unit_count as f64
        };

        // Calculate memory efficiency (inverse of peak memory usage)
        let memory_efficiency = if session_data.peak_memory > 0.0 {
            100.0 / session_data.peak_memory.max(1.0)
        } else {
            100.0
        };

        // Calculate average CPU utilization
        let cpu_utilization = if !session_data.compilation_units.is_empty() {
            session_data.compilation_units.iter()
                .map(|unit| unit.average_cpu_percent)
                .sum::<f64>() / unit_count as f64
        } else {
            0.0
        };

        // Calculate I/O efficiency
        let total_io = session_data.compilation_units.iter()
            .map(|unit| unit.io_operations)
            .sum::<u64>();
        let io_efficiency = if total_duration.as_millis() > 0 {
            total_io as f64 / total_duration.as_millis() as f64
        } else {
            0.0
        };

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
        }

        if cpu_utilization < 30.0 {
            bottlenecks.push("Low CPU utilization".to_string());
            recommendations.push("Enable parallel compilation to improve CPU utilization".to_string());
        }

        if compilation_efficiency < 1.0 {
            bottlenecks.push("Slow compilation speed".to_string());
            recommendations.push("Enable incremental compilation and caching".to_string());
        }

        PerformanceSummary {
            overall_performance_score,
            compilation_efficiency,
            memory_efficiency,
            cpu_utilization,
            io_efficiency,
            bottlenecks,
            recommendations,
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
        };

        let peak_cpu_percent = session_data.compilation_units.iter()
            .map(|unit| unit.average_cpu_percent)
            .fold(0.0, f64::max);
        
        let average_cpu_percent = if !session_data.compilation_units.is_empty() {
            session_data.compilation_units.iter()
                .map(|unit| unit.average_cpu_percent)
                .sum::<f64>() / unit_count as f64
        } else {
            0.0
        };

        let total_io_operations = session_data.compilation_units.iter()
            .map(|unit| unit.io_operations)
            .sum::<u64>();

        SystemMetrics {
            peak_memory_mb,
            average_memory_mb,
            peak_cpu_percent,
            average_cpu_percent,
            total_io_operations,
            network_bytes_transferred: 0, // Not tracked in this simulation
        }
    }

    /// Generate JSON report content
    fn generate_json_report(&self, report: &ProfileReport) -> Result<String> {
        serde_json::to_string_pretty(report).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to serialize JSON report: {}", e))
        })
    }

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
            report.session_name,
            report.session_name,
            report.total_duration,
            report.performance_summary.overall_performance_score,
            report.system_metrics.peak_memory_mb,
            report.system_metrics.average_cpu_percent,
            report.system_metrics.total_io_operations,
            report.compilation_units.iter()
                .map(|unit| format!(
                    "<tr><td>{}</td><td>{:.2?}</td><td>{:.1}</td><td>{:.1}</td></tr>",
                    unit.unit_name,
                    unit.compilation_time,
                    unit.peak_memory_mb,
                    unit.average_cpu_percent
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(html)
    }

    /// Generate Markdown report content
    fn generate_markdown_report(&self, report: &ProfileReport) -> Result<String> {
        let markdown = format!(
            r#"# Build Profile Report - {}

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
{}

## Performance Summary

- **Compilation Efficiency:** {:.2} units/sec
- **Memory Efficiency:** {:.1}%
- **CPU Utilization:** {:.1}%

## Recommendations

{}
"#,
            report.session_name,
            report.session_name,
            report.total_duration,
            report.performance_summary.overall_performance_score,
            report.system_metrics.peak_memory_mb,
            report.system_metrics.average_cpu_percent,
            report.system_metrics.total_io_operations,
            report.compilation_units.iter()
                .map(|unit| format!(
                    "| {} | {:.2?} | {:.1} | {:.1} |",
                    unit.unit_name,
                    unit.compilation_time,
                    unit.peak_memory_mb,
                    unit.average_cpu_percent
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            report.performance_summary.compilation_efficiency,
            report.performance_summary.memory_efficiency,
            report.performance_summary.cpu_utilization,
            report.performance_summary.recommendations.iter()
                .map(|rec| format!("- {}", rec))
                .collect::<Vec<_>>()
                .join("\n")
        );

        Ok(markdown)
    }

    /// Generate interactive report content
    fn generate_interactive_report(&self, report: &ProfileReport) -> Result<String> {
        // For now, just return a simple text format
        Ok(format!(
            "=== Build Profile Report ===\nSession: {}\nDuration: {:.2?}\nScore: {:.1}\n",
            report.session_name,
            report.total_duration,
            report.performance_summary.overall_performance_score
        ))
    }

    /// Get file extension for current report format
    fn get_file_extension(&self) -> &str {
        match self.config.report_format {
            ReportFormat::Json => "json",
            ReportFormat::Html => "html",
            ReportFormat::Markdown => "md",
            ReportFormat::Interactive => "txt",
        }
    }

    /// Get current memory usage (simulated)
    fn get_current_memory_usage(&self) -> f64 {
        100.0 + (rand::random::<f64>() * 50.0)
    }

    /// Get current CPU usage (simulated)
    fn get_current_cpu_usage(&self) -> f64 {
        20.0 + (rand::random::<f64>() * 30.0)
    }

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
    }
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_creation() {
        let config = ProfilerConfig::default();
        let profiler = EnhancedBuildProfiler::new(config);
        assert!(profiler.is_ok());
    }

    #[test]
    fn test_session_lifecycle() {
        let config = ProfilerConfig::default();
        let mut profiler = EnhancedBuildProfiler::new(config).unwrap();
        
        let session = profiler.start_build_session("test_session".to_string());
        assert!(session.is_ok());
        
        let session = session.unwrap();
        assert_eq!(session.name, "test_session");
        
        let report = profiler.end_build_session(session);
        assert!(report.is_ok());
    }

    #[test]
    fn test_report_generation() {
        let config = ProfilerConfig::default();
        let mut profiler = EnhancedBuildProfiler::new(config).unwrap();
        
        let session = profiler.start_build_session("test".to_string()).unwrap();
        let report = profiler.end_build_session(session).unwrap();
        
        assert!(!report.session_id.is_empty());
        assert_eq!(report.session_name, "test");
        assert!(report.performance_summary.overall_performance_score >= 0.0);
    }
}
