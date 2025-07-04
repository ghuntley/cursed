//! Build analytics module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BuildAnalytics {
    pub config: BuildAnalyticsConfig,
    pub events: Vec<BuildEvent>,
}

#[derive(Debug, Clone)]
pub struct BuildAnalyticsConfig {
    pub enable_timing: bool,
    pub enable_memory_tracking: bool,
    pub output_format: String,
    pub analytics_data_path: std::path::PathBuf,
    pub enable_detailed_tracking: bool,
    pub enable_memory_profiling: bool,
    pub enable_cpu_profiling: bool,
    pub enable_trend_analysis: bool,
    pub enable_regression_detection: bool,
    pub regression_threshold_percent: f64,
    pub sampling_interval_ms: u64,
    pub report_generation_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct BuildEvent {
    pub event_type: BuildEventType,
    pub timestamp: std::time::Instant,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum BuildEventType {
    CompileStart,
    CompileEnd,
    OptimizationStart,
    OptimizationEnd,
    LinkStart,
    LinkEnd,
    DependencyResolution,
    CompilationStart,
    CompilationEnd,
    CacheHit,
    CacheMiss,
    OptimizationPass,
    Linking,
}

impl Default for BuildAnalyticsConfig {
    fn default() -> Self {
        Self {
            enable_timing: true,
            enable_memory_tracking: true,
            output_format: "json".to_string(),
            analytics_data_path: std::path::PathBuf::from("./analytics"),
            enable_detailed_tracking: false,
            enable_memory_profiling: false,
            enable_cpu_profiling: false,
            enable_trend_analysis: false,
            enable_regression_detection: false,
            regression_threshold_percent: 10.0,
            sampling_interval_ms: 1000,
            report_generation_enabled: false,
        }
    }
}

impl BuildAnalytics {
    pub fn new(config: BuildAnalyticsConfig) -> Result<Self, CursedError> {
        Ok(Self {
            config,
            events: Vec::new(),
        })
    }
    
    pub fn record_event(&mut self, event: BuildEvent) {
        self.events.push(event);
    }
    
    pub fn get_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("total_events".to_string(), self.events.len().to_string());
        summary
    }
}

pub fn create_build_event(event_type: BuildEventType, data: HashMap<String, String>) -> BuildEvent {
    BuildEvent {
        event_type,
        timestamp: std::time::Instant::now(),
        data,
    }
}

pub fn create_build_event_with_duration(event_type: BuildEventType, duration: Duration) -> BuildEvent {
    let mut data = HashMap::new();
    data.insert("duration_ms".to_string(), duration.as_millis().to_string());
    BuildEvent {
        event_type,
        timestamp: std::time::Instant::now(),
        data,
    }
}
