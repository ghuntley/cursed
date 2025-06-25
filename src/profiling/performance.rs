/// Performance Profiling Integration
/// 
/// Comprehensive performance monitoring system that tracks compilation phases,
/// measures compilation time, memory usage, and provides detailed performance reports.

use crate::error::{CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fmt::{self, Display};
use std::io::Write;

/// Compilation phase tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompilationPhase {
impl Display for CompilationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
impl Default for MemoryStats {
    fn default() -> Self {
        Self {
        }
    }
/// Performance metrics for a single phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMetrics {
impl Default for PhaseMetrics {
    fn default() -> Self {
        Self {
        }
    }
/// Performance report configuration
#[derive(Debug, Clone)]
pub struct ReportConfig {
impl Default for ReportConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Performance report output format
#[derive(Debug, Clone, PartialEq)]
pub enum ReportFormat {
/// Main performance monitor
pub struct PerformanceMonitor {
impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: ReportConfig) -> Self {
        Self {
        }
    }
    
    /// Start timing a compilation phase
    pub fn start_phase(&self, phase: CompilationPhase) -> Result<()> {
        // End current phase if one is running
        if let Some(current) = self.current_phase.lock().unwrap().as_ref() {
            self.end_phase(current.clone())?;
        // Start new phase
        *self.current_phase.lock().unwrap() = Some(phase.clone());
        *self.phase_start.lock().unwrap() = Some(Instant::now());
        
        // Initialize phase metrics
        let mut metrics = self.metrics.lock().unwrap();
        let memory_before = self.get_current_memory_stats();
        
        metrics.insert(phase.clone(), PhaseMetrics {
            ..Default::default()
        });
        
        Ok(())
    /// End timing a compilation phase
    pub fn end_phase(&self, phase: CompilationPhase) -> Result<()> {
        let phase_start = self.phase_start.lock().unwrap().take();
        let duration = phase_start.map(|start| start.elapsed()).unwrap_or_default();
        
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(phase_metrics) = metrics.get_mut(&phase) {
            phase_metrics.duration = duration;
            phase_metrics.memory_after = self.get_current_memory_stats();
            phase_metrics.end_time = SystemTime::now();
        *self.current_phase.lock().unwrap() = None;
        
        Ok(())
    /// Record file processing information
    pub fn record_file_processed(&self, phase: CompilationPhase, lines: usize) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(phase_metrics) = metrics.get_mut(&phase) {
            phase_metrics.files_processed += 1;
            phase_metrics.lines_processed += lines;
        }
    }
    
    /// Record error information
    pub fn record_error(&self, phase: CompilationPhase) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(phase_metrics) = metrics.get_mut(&phase) {
            phase_metrics.errors_encountered += 1;
        }
    }
    
    /// Update memory peak for current phase
    pub fn update_memory_peak(&self) {
        if let Some(current_phase) = self.current_phase.lock().unwrap().as_ref() {
            let current_memory = self.get_current_memory_stats();
            let mut metrics = self.metrics.lock().unwrap();
            
            if let Some(phase_metrics) = metrics.get_mut(current_phase) {
                if current_memory.current_memory_mb > phase_metrics.memory_peak.current_memory_mb {
                    phase_metrics.memory_peak = current_memory;
                }
            }
        }
    }
    
    /// Get current memory statistics
    fn get_current_memory_stats(&self) -> MemoryStats {
        // This is a simplified implementation - in a real system you'd use
        // platform-specific APIs to get actual memory usage
        MemoryStats {
            allocations: 0, // Would be tracked by memory allocator
        }
    }
    
    /// Get process memory usage in MB (simplified implementation)
    fn get_process_memory_mb(&self) -> f64 {
        // Platform-specific implementation would go here
        // For now, return a placeholder value
        #[cfg(target_os = "linux")]
        {
            self.get_linux_memory_mb()
        }
        #[cfg(not(target_os = "linux"))]
        {
            50.0 // Placeholder
        }
    }
    
    #[cfg(target_os = "linux")]
    fn get_linux_memory_mb(&self) -> f64 {
        use std::fs;
        
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.split("\n") {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<f64>() {
                            return kb / 1024.0; // Convert KB to MB
                        }
                    }
                }
            }
        }
        50.0 // Fallback
    /// Finalize monitoring and calculate total metrics
    pub fn finalize(&mut self) -> Result<()> {
        // End any currently running phase
        if let Some(current) = self.current_phase.lock().unwrap().as_ref() {
            self.end_phase(current.clone())?;
        // Calculate total metrics
        let total_duration = self.total_start.map(|start| start.elapsed()).unwrap_or_default();
        let mut total_files = 0;
        let mut total_lines = 0;
        let mut total_errors = 0;
        
        {
            let metrics = self.metrics.lock().unwrap();
            for phase_metrics in metrics.values() {
                total_files += phase_metrics.files_processed;
                total_lines += phase_metrics.lines_processed;
                total_errors += phase_metrics.errors_encountered;
            }
        }
        
        // Store total metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(CompilationPhase::Total, PhaseMetrics {
            ..Default::default()
        });
        
        Ok(())
    /// Generate performance report
    pub fn generate_report(&self) -> Result<String> {
        match self.config.format {
        }
    }
    
    /// Generate table format report
    fn generate_table_report(&self) -> Result<String> {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();
        
        output.push_str("📊 Compilation Performance Report\n");
        output.push_str("═══════════════════════════════════════════════════════════\n");
        
        if self.config.include_phases {
                "Phase", "Time", "Memory", "Files", "Lines"));
            output.push_str("───────────────────────────────────────────────────────────\n");
            
            let mut phases: Vec<_> = metrics.values().collect();
            if self.config.sort_by_duration {
                phases.sort_by(|a, b| b.duration.cmp(&a.duration));
            for phase_metrics in phases {
                if phase_metrics.phase == CompilationPhase::Total {
                    continue;
                let duration_str = format!("{:.2}s", phase_metrics.duration.as_secs_f64());
                let memory_str = if self.config.include_memory {
                    format!("{:.1}MB", phase_metrics.memory_peak.current_memory_mb)
                } else {
                    "-".to_string()
                
                ));
            output.push_str("───────────────────────────────────────────────────────────\n");
        // Total summary
        if let Some(total_metrics) = metrics.get(&CompilationPhase::Total) {
                total_metrics.duration.as_secs_f64()));
            if self.config.include_files {
                output.push_str(&format!("Files processed: {}\n", total_metrics.files_processed));
                output.push_str(&format!("Lines processed: {}\n", total_metrics.lines_processed));
            }
            if total_metrics.errors_encountered > 0 {
                output.push_str(&format!("Errors encountered: {}\n", total_metrics.errors_encountered));
            }
        }
        
        Ok(output)
    /// Generate JSON format report
    fn generate_json_report(&self) -> Result<String> {
        let metrics = self.metrics.lock().unwrap();
        serde_json::to_string_pretty(&*metrics)
            .map_err(|e| CursedError::General(format!("Failed to serialize metrics: {}", e)))
    /// Generate CSV format report
    fn generate_csv_report(&self) -> Result<String> {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();
        
        output.push_str("Phase,Duration(s),Memory(MB),Files,Lines,Errors\n");
        
        for phase_metrics in metrics.values() {
            ));
        Ok(output)
    /// Generate summary format report
    fn generate_summary_report(&self) -> Result<String> {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();
        
        if let Some(total_metrics) = metrics.get(&CompilationPhase::Total) {
                total_metrics.duration.as_secs_f64()));
            
            if total_metrics.files_processed > 0 {
                let lines_per_sec = total_metrics.lines_processed as f64 / total_metrics.duration.as_secs_f64();
                output.push_str(&format!("   {} files, {} lines ({:.0} lines/sec)\n",
                    lines_per_sec));
            if total_metrics.errors_encountered > 0 {
                output.push_str(&format!("   ⚠️  {} errors encountered\n", total_metrics.errors_encountered));
            }
        }
        
        Ok(output)
    /// Generate graph format report (ASCII art)
    fn generate_graph_report(&self) -> Result<String> {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();
        
        output.push_str("📈 Performance Graph (Duration)\n");
        output.push_str("═══════════════════════════════\n");
        
        let max_duration = metrics.values()
            .filter(|m| m.phase != CompilationPhase::Total)
            .map(|m| m.duration.as_secs_f64())
            .fold(0.0, f64::max);
        
        if max_duration > 0.0 {
            for phase_metrics in metrics.values() {
                if phase_metrics.phase == CompilationPhase::Total {
                    continue;
                let normalized = (phase_metrics.duration.as_secs_f64() / max_duration * 40.0) as usize;
                let bar = "█".repeat(normalized);
                
                ));
            }
        }
        
        Ok(output)
    /// Write report to file
    pub fn write_report_to_file(&self, path: &str) -> Result<()> {
        let report = self.generate_report()?;
        std::fs::write(path, report)
            .map_err(|e| CursedError::General(format!("Failed to write report to {}: {}", path, e)))
    /// Get specific phase metrics
    pub fn get_phase_metrics(&self, phase: CompilationPhase) -> Option<PhaseMetrics> {
        self.metrics.lock().unwrap().get(&phase).cloned()
    /// Get all metrics
    pub fn get_all_metrics(&self) -> HashMap<CompilationPhase, PhaseMetrics> {
        self.metrics.lock().unwrap().clone()
    /// Update report configuration
    pub fn update_config(&mut self, config: ReportConfig) {
        self.config = config;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance monitoring utilities
pub mod utils {
    use super::*;
    
    /// Create a scoped phase timer that automatically ends the phase when dropped
    pub struct ScopedPhaseTimer<'a> {
    impl<'a> ScopedPhaseTimer<'a> {
        pub fn new(monitor: &'a PerformanceMonitor, phase: CompilationPhase) -> Result<Self> {
            monitor.start_phase(phase.clone())?;
            Ok(Self { monitor, phase })
        }
    }
    
    impl<'a> Drop for ScopedPhaseTimer<'a> {
        fn drop(&mut self) {
            let _ = self.monitor.end_phase(self.phase.clone());
        }
    }
    
    /// Format duration for human reading
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs_f64();
        if secs < 1.0 {
            format!("{:.0}ms", secs * 1000.0)
        } else if secs < 60.0 {
            format!("{:.2}s", secs)
        } else {
            let mins = (secs / 60.0) as u64;
            let remaining_secs = secs % 60.0;
            format!("{}m{:.1}s", mins, remaining_secs)
        }
    }
    
    /// Format memory size for human reading
    pub fn format_memory(mb: f64) -> String {
        if mb < 1024.0 {
            format!("{:.1}MB", mb)
        } else {
            format!("{:.2}GB", mb / 1024.0)
        }
    }
