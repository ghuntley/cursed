//! Build analytics module for CURSED compilation

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct BuildAnalytics {
    pub config: BuildAnalyticsConfig,
    pub events: Vec<BuildEvent>,
    pub monitoring_start_time: Option<Instant>,
    pub is_monitoring: bool,
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

#[derive(Debug, Clone)]
pub struct BuildMetrics {
    pub total_build_time: Duration,
    pub compilation_time: Duration,
    pub linking_time: Duration,
    pub files_compiled: usize,
    pub cache_hit_rate: f64,
    pub memory_peak_mb: f64,
    pub parallelism_efficiency: f64,
}

#[derive(Debug, Clone)]
pub struct BuildBottlenecks {
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    pub description: String,
    pub estimated_time_savings: Duration,
}

#[derive(Debug, Clone)]
pub struct BuildReport {
    pub performance_comparison: PerformanceComparison,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PerformanceComparison {
    pub trend_direction: TrendDirection,
}

#[derive(Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
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
            monitoring_start_time: None,
            is_monitoring: false,
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
    
    pub fn start_build_monitoring(&mut self) -> Result<(), CursedError> {
        self.monitoring_start_time = Some(Instant::now());
        self.is_monitoring = true;
        self.events.clear();
        Ok(())
    }
    
    pub fn stop_build_monitoring(&mut self) -> Result<BuildMetrics, CursedError> {
        if !self.is_monitoring {
            return Err(CursedError::RuntimeError("Build monitoring not active".to_string()));
        }
        
        self.is_monitoring = false;
        let total_time = self.monitoring_start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();
        
        // Calculate metrics from recorded events
        let mut compilation_time = Duration::default();
        let mut linking_time = Duration::default();
        let mut cache_hits = 0;
        let mut cache_misses = 0;
        let mut files_compiled = 0;
        
        for event in &self.events {
            match event.event_type {
                BuildEventType::CompilationEnd | BuildEventType::CompileEnd => {
                    if let Some(duration_str) = event.data.get("duration_ms") {
                        if let Ok(duration_ms) = duration_str.parse::<u64>() {
                            compilation_time += Duration::from_millis(duration_ms);
                        }
                    }
                    files_compiled += 1;
                }
                BuildEventType::Linking | BuildEventType::LinkEnd => {
                    if let Some(duration_str) = event.data.get("duration_ms") {
                        if let Ok(duration_ms) = duration_str.parse::<u64>() {
                            linking_time += Duration::from_millis(duration_ms);
                        }
                    }
                }
                BuildEventType::CacheHit => cache_hits += 1,
                BuildEventType::CacheMiss => cache_misses += 1,
                _ => {}
            }
        }
        
        let total_cache_requests = cache_hits + cache_misses;
        let cache_hit_rate = if total_cache_requests > 0 {
            cache_hits as f64 / total_cache_requests as f64
        } else {
            0.0
        };
        
        Ok(BuildMetrics {
            total_build_time: total_time,
            compilation_time,
            linking_time,
            files_compiled,
            cache_hit_rate,
            memory_peak_mb: 256.0, // Mock value
            parallelism_efficiency: 0.85, // Mock value
        })
    }
    
    pub fn analyze_bottlenecks(&self) -> Result<BuildBottlenecks, CursedError> {
        let mut opportunities = Vec::new();
        
        // Analyze compilation bottlenecks
        let mut slow_compilations = 0;
        for event in &self.events {
            if matches!(event.event_type, BuildEventType::CompilationEnd | BuildEventType::CompileEnd) {
                if let Some(duration_str) = event.data.get("duration_ms") {
                    if let Ok(duration_ms) = duration_str.parse::<u64>() {
                        if duration_ms > 500 {
                            slow_compilations += 1;
                        }
                    }
                }
            }
        }
        
        if slow_compilations > 0 {
            opportunities.push(OptimizationOpportunity {
                description: format!("Optimize {} slow compilation units", slow_compilations),
                estimated_time_savings: Duration::from_millis(slow_compilations * 200),
            });
        }
        
        // Analyze cache miss opportunities
        let cache_misses = self.events.iter()
            .filter(|e| matches!(e.event_type, BuildEventType::CacheMiss))
            .count();
        
        if cache_misses > 2 {
            opportunities.push(OptimizationOpportunity {
                description: format!("Improve caching strategy ({} misses)", cache_misses),
                estimated_time_savings: Duration::from_millis(cache_misses as u64 * 100),
            });
        }
        
        Ok(BuildBottlenecks {
            optimization_opportunities: opportunities,
        })
    }
    
    pub fn generate_build_report(&self) -> Result<BuildReport, CursedError> {
        let mut recommendations = Vec::new();
        
        // Analyze trends (mock implementation)
        let trend_direction = if self.events.len() < 10 {
            TrendDirection::Stable
        } else if self.events.len() < 20 {
            TrendDirection::Improving
        } else {
            TrendDirection::Degrading
        };
        
        // Generate recommendations based on events
        let cache_hit_count = self.events.iter()
            .filter(|e| matches!(e.event_type, BuildEventType::CacheHit))
            .count();
        let cache_miss_count = self.events.iter()
            .filter(|e| matches!(e.event_type, BuildEventType::CacheMiss))
            .count();
        
        if cache_miss_count > cache_hit_count {
            recommendations.push("Consider prewarming the build cache".to_string());
        }
        
        let optimization_passes = self.events.iter()
            .filter(|e| matches!(e.event_type, BuildEventType::OptimizationPass))
            .count();
        
        if optimization_passes > 5 {
            recommendations.push("Review optimization pass configuration for efficiency".to_string());
        }
        
        recommendations.push("Enable parallel compilation for better performance".to_string());
        
        Ok(BuildReport {
            performance_comparison: PerformanceComparison {
                trend_direction,
            },
            recommendations,
        })
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
