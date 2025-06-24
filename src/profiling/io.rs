use crate::error::Error;
// I/O profiling for file and network operations

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

use crate::profiling::core::{DataCollector, CollectorStats, ProfilerError};

/// I/O profiler for file and network operations
#[derive(Debug)]
pub struct IoProfiler {
    collecting: Arc<Mutex<bool>>,
    data: Arc<RwLock<IoProfileData>>,
    stats: Arc<RwLock<CollectorStats>>,
}

impl IoProfiler {
    pub fn new() -> Self {
        Self {
            collecting: Arc::new(Mutex::new(false)),
            data: Arc::new(RwLock::new(IoProfileData::new())),
            stats: Arc::new(RwLock::new(CollectorStats::default())),
        }
    }
    
    #[instrument(skip(self))]
    pub fn track_file_operation(
        &self,
        operation: FileOperation,
        path: String,
        size: Option<usize>,
        duration: Duration,
    ) -> Result<(), Error> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = IoEvent {
            event_type: IoEventType::File(operation),
            resource: path,
            size,
            duration,
            timestamp: Instant::now(),
            thread_id: Self::get_current_thread_id(),
            status: IoStatus::Success,
            error_message: None,
        };
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        }
        
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_network_operation(
        &self,
        operation: NetworkOperation,
        address: String,
        size: Option<usize>,
        duration: Duration,
    ) -> Result<(), Error> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = IoEvent {
            event_type: IoEventType::Network(operation),
            resource: address,
            size,
            duration,
            timestamp: Instant::now(),
            thread_id: Self::get_current_thread_id(),
            status: IoStatus::Success,
            error_message: None,
        };
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        }
        
        self.update_stats(|stats| stats.data_points += 1);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn track_io_error(
        &self,
        operation_type: IoEventType,
        resource: String,
        error: String,
        duration: Duration,
    ) -> Result<(), Error> {
        if !self.is_collecting() {
            return Ok(());
        }
        
        let event = IoEvent {
            event_type: operation_type,
            resource,
            size: None,
            duration,
            timestamp: Instant::now(),
            thread_id: Self::get_current_thread_id(),
            status: IoStatus::Error,
            error_message: Some(error),
        };
        
        if let Ok(mut data) = self.data.write() {
            data.add_io_event(event);
        }
        
        self.update_stats(|stats| {
            stats.data_points += 1;
            stats.errors += 1;
        });
        
        Ok(())
    }
    
    fn get_current_thread_id() -> u64 {
        // Use a simple hash of the thread id since as_u64() is unstable
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    }
    
    fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut CollectorStats),
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
}

impl DataCollector for IoProfiler {
    #[instrument(skip(self))]
    fn start_collection(&mut self) -> Result<(), Error> {
        if self.is_collecting() {
            return Err(ProfilerError::ConfigError("I/O profiler already collecting".to_string()));
        }
        
        *self.collecting.lock().unwrap() = true;
        info!("Started I/O profiling");
        Ok(())
    }
    
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> Result<(), Error> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("I/O profiler not collecting".to_string()));
        }
        
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
            Err(e) => Err(ProfilerError::SerializationError(e.to_string())),
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    }
    
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// I/O profiling data collection
#[derive(Debug, Clone, Serialize)]
pub struct IoProfileData {
    #[serde(skip)]
    pub io_events: Vec<IoEvent>,
    pub start_time: SystemTime,
}

impl IoProfileData {
    pub fn new() -> Self {
        Self {
            io_events: Vec::new(),
            start_time: SystemTime::now(),
        }
    }
    
    pub fn add_io_event(&mut self, event: IoEvent) {
        self.io_events.push(event);
    }
    
    pub fn analyze_performance(&self) -> IoAnalysis {
        let mut file_stats = IoTypeStats::default();
        let mut network_stats = IoTypeStats::default();
        let mut operation_patterns = Vec::new();
        let mut resource_usage: HashMap<String, ResourceUsage> = HashMap::new();
        
        for event in &self.io_events {
            let stats = match &event.event_type {
                IoEventType::File(_) => &mut file_stats,
                IoEventType::Network(_) => &mut network_stats,
            };
            
            stats.total_operations += 1;
            stats.total_duration += event.duration;
            
            if event.status == IoStatus::Success {
                stats.successful_operations += 1;
                if let Some(size) = event.size {
                    stats.total_bytes += size;
                }
            } else {
                stats.failed_operations += 1;
            }
            
            if event.duration > stats.max_duration {
                stats.max_duration = event.duration;
            }
            
            if stats.min_duration.is_zero() || event.duration < stats.min_duration {
                stats.min_duration = event.duration;
            }
            
            // Track resource usage
            let usage = resource_usage
                .entry(event.resource.clone())
                .or_insert_with(ResourceUsage::default);
            usage.access_count += 1;
            usage.total_time += event.duration;
            if let Some(size) = event.size {
                usage.total_bytes += size;
            }
            
            // Track operation patterns
            operation_patterns.push(IoOperationPattern {
                timestamp: event.timestamp,
                operation_type: event.event_type.clone(),
                resource: event.resource.clone(),
                duration: event.duration,
                size: event.size,
                success: event.status == IoStatus::Success,
            });
        }
        
        // Calculate averages
        if file_stats.total_operations > 0 {
            file_stats.average_duration = file_stats.total_duration / file_stats.total_operations as u32;
        }
        
        if network_stats.total_operations > 0 {
            network_stats.average_duration = network_stats.total_duration / network_stats.total_operations as u32;
        }
        
        let total_io_time = file_stats.total_duration + network_stats.total_duration;
        let io_efficiency = self.calculate_io_efficiency(&file_stats, &network_stats);
        
        IoAnalysis {
            file_stats,
            network_stats,
            operation_patterns,
            resource_usage,
            total_io_time,
            io_efficiency,
        }
    }
    
    pub fn detect_bottlenecks(&self) -> Vec<IoBottleneck> {
        let mut bottlenecks = Vec::new();
        let analysis = self.analyze_performance();
        
        // Detect slow file operations
        if analysis.file_stats.average_duration > Duration::from_millis(100) {
            bottlenecks.push(IoBottleneck {
                bottleneck_type: BottleneckType::SlowFileIo,
                description: format!(
                    "File operations are averaging {:?}, which may be slow",
                    analysis.file_stats.average_duration
                ),
                severity: self.calculate_severity(analysis.file_stats.average_duration),
                affected_resources: analysis.resource_usage
                    .iter()
                    .filter(|(_, usage)| usage.average_time() > Duration::from_millis(100))
                    .map(|(resource, _)| resource.clone())
                    .collect(),
                recommendations: vec![
                    "Consider using async I/O operations".to_string(),
                    "Check if file caching can be improved".to_string(),
                ],
            });
        }
        
        // Detect slow network operations
        if analysis.network_stats.average_duration > Duration::from_millis(500) {
            bottlenecks.push(IoBottleneck {
                bottleneck_type: BottleneckType::SlowNetworkIo,
                description: format!(
                    "Network operations are averaging {:?}, which may indicate network issues",
                    analysis.network_stats.average_duration
                ),
                severity: self.calculate_severity(analysis.network_stats.average_duration),
                affected_resources: analysis.resource_usage
                    .iter()
                    .filter(|(_, usage)| usage.average_time() > Duration::from_millis(500))
                    .map(|(resource, _)| resource.clone())
                    .collect(),
                recommendations: vec![
                    "Consider connection pooling".to_string(),
                    "Review network timeout settings".to_string(),
                    "Check for network latency issues".to_string(),
                ],
            });
        }
        
        // Detect high error rates
        let total_operations = analysis.file_stats.total_operations + analysis.network_stats.total_operations;
        let total_failures = analysis.file_stats.failed_operations + analysis.network_stats.failed_operations;
        
        if total_operations > 0 {
            let error_rate = (total_failures as f64 / total_operations as f64) * 100.0;
            if error_rate > 5.0 {
                bottlenecks.push(IoBottleneck {
                    bottleneck_type: BottleneckType::HighErrorRate,
                    description: format!("I/O error rate is {:.1}%, which is concerning", error_rate),
                    severity: if error_rate > 20.0 {
                        BottleneckSeverity::Critical
                    } else if error_rate > 10.0 {
                        BottleneckSeverity::High
                    } else {
                        BottleneckSeverity::Medium
                    },
                    affected_resources: Vec::new(),
                    recommendations: vec![
                        "Review error handling and retry logic".to_string(),
                        "Check resource availability and permissions".to_string(),
                    ],
                });
            }
        }
        
        // Sort by severity
        bottlenecks.sort_by(|a, b| b.severity.cmp(&a.severity));
        bottlenecks
    }
    
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
            0..=50 => BottleneckSeverity::Low,
            51..=200 => BottleneckSeverity::Medium,
            201..=1000 => BottleneckSeverity::High,
            _ => BottleneckSeverity::Critical,
        }
    }
}

/// Individual I/O event
#[derive(Debug, Clone, Serialize)]
pub struct IoEvent {
    pub event_type: IoEventType,
    pub resource: String,
    pub size: Option<usize>,
    pub duration: Duration,
    #[serde(skip)]
    pub timestamp: Instant,
    pub thread_id: u64,
    pub status: IoStatus,
    pub error_message: Option<String>,
}

/// Types of I/O events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoEventType {
    File(FileOperation),
    Network(NetworkOperation),
}

/// File operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
    Read,
    Write,
    Open,
    Close,
    Seek,
    Flush,
    Sync,
    Delete,
    Rename,
    CreateDirectory,
    RemoveDirectory,
}

/// Network operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkOperation {
    TcpConnect,
    TcpRead,
    TcpWrite,
    TcpClose,
    UdpSend,
    UdpReceive,
    HttpRequest,
    HttpResponse,
    DnsLookup,
}

/// I/O operation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoStatus {
    Success,
    Error,
    Timeout,
    Canceled,
}

/// I/O performance analysis
#[derive(Debug, Clone, Default, Serialize)]
pub struct IoAnalysis {
    pub file_stats: IoTypeStats,
    pub network_stats: IoTypeStats,
    #[serde(skip)]
    pub operation_patterns: Vec<IoOperationPattern>,
    pub resource_usage: HashMap<String, ResourceUsage>,
    pub total_io_time: Duration,
    pub io_efficiency: f64,
}

/// Statistics for a type of I/O operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoTypeStats {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub total_duration: Duration,
    pub average_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub total_bytes: usize,
}

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
}

/// I/O operation pattern for analysis
#[derive(Debug, Clone, Serialize)]
pub struct IoOperationPattern {
    #[serde(skip)]
    pub timestamp: Instant,
    pub operation_type: IoEventType,
    pub resource: String,
    pub duration: Duration,
    pub size: Option<usize>,
    pub success: bool,
}

/// Resource usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub access_count: u64,
    pub total_time: Duration,
    pub total_bytes: usize,
}

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
}

/// I/O bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoBottleneck {
    pub bottleneck_type: BottleneckType,
    pub description: String,
    pub severity: BottleneckSeverity,
    pub affected_resources: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Types of I/O bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    SlowFileIo,
    SlowNetworkIo,
    HighErrorRate,
    ExcessiveOperations,
    LargeOperations,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_io_profiler_creation() {
        let profiler = IoProfiler::new();
        assert!(!profiler.is_collecting());
    }
    
    #[test]
    fn test_file_operation_tracking() {
        let profiler = IoProfiler::new();
        
        let result = profiler.track_file_operation(
            FileOperation::Read,
            "/test/file.txt".to_string(),
            Some(1024),
            Duration::from_millis(10),
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_network_operation_tracking() {
        let profiler = IoProfiler::new();
        
        let result = profiler.track_network_operation(
            NetworkOperation::HttpRequest,
            "http://example.com".to_string(),
            Some(512),
            Duration::from_millis(100),
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_io_analysis() {
        let mut data = IoProfileData::new();
        
        let event = IoEvent {
            event_type: IoEventType::File(FileOperation::Read),
            resource: "/test/file.txt".to_string(),
            size: Some(1024),
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            thread_id: 1,
            status: IoStatus::Success,
            error_message: None,
        };
        
        data.add_io_event(event);
        
        let analysis = data.analyze_performance();
        assert_eq!(analysis.file_stats.total_operations, 1);
        assert_eq!(analysis.file_stats.successful_operations, 1);
        assert_eq!(analysis.file_stats.total_bytes, 1024);
    }
    
    #[test]
    fn test_bottleneck_detection() {
        let mut data = IoProfileData::new();
        
        // Add a slow operation
        let slow_event = IoEvent {
            event_type: IoEventType::File(FileOperation::Read),
            resource: "/slow/file.txt".to_string(),
            size: Some(1024),
            duration: Duration::from_millis(200), // Slow operation
            timestamp: Instant::now(),
            thread_id: 1,
            status: IoStatus::Success,
            error_message: None,
        };
        
        data.add_io_event(slow_event);
        
        let bottlenecks = data.detect_bottlenecks();
        assert!(!bottlenecks.is_empty());
        assert!(matches!(bottlenecks[0].bottleneck_type, BottleneckType::SlowFileIo));
    }
    
    #[test]
    fn test_io_type_stats() {
        let mut stats = IoTypeStats::default();
        stats.total_operations = 10;
        stats.successful_operations = 8;
        
        assert_eq!(stats.success_rate(), 0.8);
    }
}
