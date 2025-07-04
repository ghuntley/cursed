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
}

impl Default for BuildAnalyticsConfig {
    fn default() -> Self {
        Self {
            enable_timing: true,
            enable_memory_tracking: true,
            output_format: "json".to_string(),
        }
    }
}

impl BuildAnalytics {
    pub fn new(config: BuildAnalyticsConfig) -> Self {
        Self {
            config,
            events: Vec::new(),
        }
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
