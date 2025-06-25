use crate::error::CursedError;
// I/O profiling for file and network operations

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

// use crate::profiling::core::{DataCollector, CollectorStats, ProfilerError};

/// I/O profiler for file and network operations
#[derive(Debug)]
pub struct IoProfiler {
impl IoProfiler {
    pub fn new() -> Self {
        Self {
        }
    }
    
    #[instrument(skip(self))]
    pub fn track_file_operation(
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        let event = IoEvent {
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    #[instrument(skip(self))]
    pub fn track_network_operation(
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        let event = IoEvent {
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    #[instrument(skip(self))]
    pub fn track_io_error(
    ) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Ok(());
        let event = IoEvent {
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        self.update_stats(|stats| {
            stats.data_points += 1;
            stats.errors += 1;
        });
        
        Ok(())
    fn get_current_thread_id() -> u64 {
        // Use a simple hash of the thread id since as_u64() is unstable
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    fn update_stats<F>(&self, updater: F)
    where
    {
        if let Ok(mut stats) = self.stats.write() {
            updater(&mut stats);
        }
    }
    
    pub fn get_io_analysis(&self) -> IoAnalysis {
        if let Ok(data) = self.data.read() {
            data.analyze_performance()
        } else {
            IoAnalysis::default()
        }
    }
    
    pub fn detect_io_bottlenecks(&self) -> Vec<IoBottleneck> {
        if let Ok(data) = self.data.read() {
            data.detect_bottlenecks()
        } else {
            Vec::new()
        }
    }
impl DataCollector for IoProfiler {
    #[instrument(skip(self))]
    fn start_collection(&mut self) -> crate::error::Result<()> {
        if self.is_collecting() {
            return Err(ProfilerError::ConfigError("I/O profiler already collecting".to_string()));
        *self.collecting.lock().unwrap() = true;
        info!("Started I/O profiling");
        Ok(())
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("I/O profiler not collecting".to_string()));
        *self.collecting.lock().unwrap() = false;
        
        let profile_data = self.data.read().unwrap().clone();
        match bincode::serialize(&profile_data) {
            Ok(data) => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.bytes_collected = data.len() as u64;
                }
                info!("I/O profiling stopped, collected {} I/O events", profile_data.io_events.len());
                Ok(data)
            }
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// I/O profiling data collection
#[derive(Debug, Clone, Serialize)]
pub struct IoProfileData {
    #[serde(skip)]
impl IoProfileData {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_io_event(&mut self, event: IoEvent) {
        self.io_events.push(event);
    pub fn analyze_performance(&self) -> IoAnalysis {
        let mut file_stats = IoTypeStats::default();
        let mut network_stats = IoTypeStats::default();
        let mut operation_patterns = Vec::new();
        let mut resource_usage: HashMap<String, ResourceUsage> = HashMap::new();
        
        for event in &self.io_events {
            let stats = match &event.event_type {
            
            stats.total_operations += 1;
            stats.total_duration += event.duration;
            
            if event.status == IoStatus::Success {
                stats.successful_operations += 1;
                if let Some(size) = event.size {
                    stats.total_bytes += size;
                }
            } else {
                stats.failed_operations += 1;
            if event.duration > stats.max_duration {
                stats.max_duration = event.duration;
            if stats.min_duration.is_zero() || event.duration < stats.min_duration {
                stats.min_duration = event.duration;
            // Track resource usage
            let usage = resource_usage
                .entry(event.resource.clone())
                .or_insert_with(ResourceUsage::default);
            usage.access_count += 1;
            usage.total_time += event.duration;
            if let Some(size) = event.size {
                usage.total_bytes += size;
            // Track operation patterns
            operation_patterns.push(IoOperationPattern {
            });
        // Calculate averages
        if file_stats.total_operations > 0 {
            file_stats.average_duration = file_stats.total_duration / file_stats.total_operations as u32;
        if network_stats.total_operations > 0 {
            network_stats.average_duration = network_stats.total_duration / network_stats.total_operations as u32;
        let total_io_time = file_stats.total_duration + network_stats.total_duration;
        let io_efficiency = self.calculate_io_efficiency(&file_stats, &network_stats);
        
        IoAnalysis {
        }
    }
    
    pub fn detect_bottlenecks(&self) -> Vec<IoBottleneck> {
        let mut bottlenecks = Vec::new();
        let analysis = self.analyze_performance();
        
        // Detect slow file operations
        if analysis.file_stats.average_duration > Duration::from_millis(100) {
            bottlenecks.push(IoBottleneck {
                description: format!(
                    analysis.file_stats.average_duration
                affected_resources: analysis.resource_usage
                    .iter()
                    .filter(|(_, usage)| usage.average_time() > Duration::from_millis(100))
                    .map(|(resource, _)| resource.clone())
                recommendations: vec![
                    "Consider using async I/O operations".to_string(),
            });
        // Detect slow network operations
        if analysis.network_stats.average_duration > Duration::from_millis(500) {
            bottlenecks.push(IoBottleneck {
                description: format!(
                    analysis.network_stats.average_duration
                affected_resources: analysis.resource_usage
                    .iter()
                    .filter(|(_, usage)| usage.average_time() > Duration::from_millis(500))
                    .map(|(resource, _)| resource.clone())
                recommendations: vec![
            });
        // Detect high error rates
        let total_operations = analysis.file_stats.total_operations + analysis.network_stats.total_operations;
        let total_failures = analysis.file_stats.failed_operations + analysis.network_stats.failed_operations;
        
        if total_operations > 0 {
            let error_rate = (total_failures as f64 / total_operations as f64) * 100.0;
            if error_rate > 5.0 {
                bottlenecks.push(IoBottleneck {
                    description: format!("I/O error rate is {:.1}%, which is concerning", error_rate),
                    severity: if error_rate > 20.0 {
                        BottleneckSeverity::Critical
                    } else if error_rate > 10.0 {
                        BottleneckSeverity::High
                    } else {
                        BottleneckSeverity::Medium
                    recommendations: vec![
                });
            }
        }
        
        // Sort by severity
        bottlenecks.sort_by(|a, b| b.severity.cmp(&a.severity));
        bottlenecks
    fn calculate_io_efficiency(&self, file_stats: &IoTypeStats, network_stats: &IoTypeStats) -> f64 {
        let total_ops = file_stats.total_operations + network_stats.total_operations;
        let successful_ops = file_stats.successful_operations + network_stats.successful_operations;
        
        if total_ops == 0 {
            1.0
        } else {
            successful_ops as f64 / total_ops as f64
        }
    }
    
    fn calculate_severity(&self, duration: Duration) -> BottleneckSeverity {
        match duration.as_millis() {
        }
    }
/// Individual I/O event
#[derive(Debug, Clone, Serialize)]
pub struct IoEvent {
    #[serde(skip)]
/// Types of I/O events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoEventType {
/// File operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
/// Network operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkOperation {
/// I/O operation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoStatus {
/// I/O performance analysis
#[derive(Debug, Clone, Default, Serialize)]
pub struct IoAnalysis {
    #[serde(skip)]
/// Statistics for a type of I/O operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoTypeStats {
impl IoTypeStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            self.successful_operations as f64 / self.total_operations as f64
        }
    }
    
    pub fn throughput(&self) -> f64 {
        if self.total_duration.as_secs_f64() > 0.0 {
            self.total_bytes as f64 / self.total_duration.as_secs_f64()
        } else {
            0.0
        }
    }
/// I/O operation pattern for analysis
#[derive(Debug, Clone, Serialize)]
pub struct IoOperationPattern {
    #[serde(skip)]
/// Resource usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
impl ResourceUsage {
    pub fn average_time(&self) -> Duration {
        if self.access_count > 0 {
            self.total_time / self.access_count as u32
        } else {
            Duration::default()
        }
    }
    
    pub fn throughput(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            self.total_bytes as f64 / self.total_time.as_secs_f64()
        } else {
            0.0
        }
    }
/// I/O bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoBottleneck {
/// Types of I/O bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BottleneckSeverity {
