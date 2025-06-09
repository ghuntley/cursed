// Core profiling infrastructure and configuration

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

/// Main profiler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// Profiling modes to enable
    pub modes: Vec<ProfilerMode>,
    /// Sampling frequency for CPU profiling (Hz)
    pub cpu_sampling_frequency: u64,
    /// Memory allocation tracking threshold (bytes)
    pub memory_tracking_threshold: usize,
    /// Maximum number of stack frames to capture
    pub max_stack_depth: usize,
    /// Enable goroutine tracking
    pub track_goroutines: bool,
    /// Enable I/O operation tracking
    pub track_io_operations: bool,
    /// Output directory for profiling data
    pub output_directory: String,
    /// Maximum profiling session duration
    pub max_session_duration: Duration,
    /// Profiling data format (JSON, binary, flame graph)
    pub output_format: OutputFormat,
    /// Performance regression detection threshold
    pub regression_threshold: f64,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            modes: vec![ProfilerMode::Cpu, ProfilerMode::Memory],
            cpu_sampling_frequency: 100,
            memory_tracking_threshold: 1024,
            max_stack_depth: 64,
            track_goroutines: true,
            track_io_operations: true,
            output_directory: "profiling_output".to_string(),
            max_session_duration: Duration::from_secs(300),
            output_format: OutputFormat::Json,
            regression_threshold: 0.1, // 10% performance regression
        }
    }
}

/// Profiling modes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfilerMode {
    /// CPU profiling with call stack sampling
    Cpu,
    /// Memory allocation and usage tracking
    Memory,
    /// Goroutine and channel concurrency profiling
    Concurrency,
    /// I/O operation profiling
    Io,
    /// Custom profiling mode
    Custom(String),
}

/// Output formats for profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Binary,
    FlameGraph,
    Csv,
    Html,
}

/// Main profiler coordinator
pub struct CursedProfiler {
    config: ProfilerConfig,
    session: Option<ProfilingSession>,
    data_collectors: HashMap<ProfilerMode, Box<dyn DataCollector>>,
    stats: Arc<RwLock<ProfilerStats>>,
}

impl std::fmt::Debug for CursedProfiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CursedProfiler")
            .field("config", &self.config)
            .field("session", &self.session)
            .field("data_collectors", &format!("{} collectors", self.data_collectors.len()))
            .finish()
    }
}

impl CursedProfiler {
    #[instrument]
    pub fn new(config: ProfilerConfig) -> Self {
        info!("Initializing CURSED profiler with config: {:?}", config);
        
        let mut profiler = Self {
            config: config.clone(),
            session: None,
            data_collectors: HashMap::new(),
            stats: Arc::new(RwLock::new(ProfilerStats::default())),
        };
        
        profiler.initialize_collectors();
        profiler
    }
    
    #[instrument(skip(self))]
    fn initialize_collectors(&mut self) {
        for mode in &self.config.modes {
            match mode {
                ProfilerMode::Cpu => {
                    self.data_collectors.insert(
                        mode.clone(),
                        Box::new(crate::profiling::cpu::CpuProfiler::new(
                            self.config.cpu_sampling_frequency,
                            self.config.max_stack_depth,
                        )),
                    );
                }
                ProfilerMode::Memory => {
                    self.data_collectors.insert(
                        mode.clone(),
                        Box::new(crate::profiling::memory::MemoryProfiler::new(
                            self.config.memory_tracking_threshold,
                        )),
                    );
                }
                ProfilerMode::Concurrency => {
                    if self.config.track_goroutines {
                        self.data_collectors.insert(
                            mode.clone(),
                            Box::new(crate::profiling::concurrency::ConcurrencyProfiler::new()),
                        );
                    }
                }
                ProfilerMode::Io => {
                    if self.config.track_io_operations {
                        self.data_collectors.insert(
                            mode.clone(),
                            Box::new(crate::profiling::io::IoProfiler::new()),
                        );
                    }
                }
                ProfilerMode::Custom(name) => {
                    warn!("Custom profiling mode '{}' not implemented", name);
                }
            }
        }
        
        info!("Initialized {} data collectors", self.data_collectors.len());
    }
    
    #[instrument(skip(self))]
    pub fn start_session(&mut self, session_name: String) -> Result<(), ProfilerError> {
        if self.session.is_some() {
            return Err(ProfilerError::SessionAlreadyActive);
        }
        
        let session = ProfilingSession::new(session_name, self.config.clone());
        info!("Starting profiling session: {}", session.name);
        
        // Start all data collectors
        for (mode, collector) in &mut self.data_collectors {
            if let Err(e) = collector.start_collection() {
                error!("Failed to start collector for mode {:?}: {}", mode, e);
                return Err(ProfilerError::CollectorStartFailed(mode.clone()));
            }
        }
        
        self.session = Some(session);
        self.update_stats(|stats| stats.sessions_started += 1);
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn stop_session(&mut self) -> Result<ProfileData, ProfilerError> {
        let session = self.session.take()
            .ok_or(ProfilerError::NoActiveSession)?;
        
        info!("Stopping profiling session: {}", session.name);
        
        let mut profile_data = ProfileData::new(session.name.clone());
        
        // Stop all data collectors and gather data
        for (mode, collector) in &mut self.data_collectors {
            match collector.stop_collection() {
                Ok(data) => {
                    profile_data.add_mode_data(mode.clone(), data);
                }
                Err(e) => {
                    error!("Failed to stop collector for mode {:?}: {}", mode, e);
                    return Err(ProfilerError::CollectorStopFailed(mode.clone()));
                }
            }
        }
        
        profile_data.session_duration = session.start_time.elapsed();
        profile_data.timestamp = SystemTime::now();
        
        self.update_stats(|stats| {
            stats.sessions_completed += 1;
            stats.total_profiling_time += profile_data.session_duration;
        });
        
        info!("Profiling session completed in {:?}", profile_data.session_duration);
        Ok(profile_data)
    }
    
    #[instrument(skip(self))]
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    }
    
    #[instrument(skip(self))]
    pub fn get_current_session(&self) -> Option<&ProfilingSession> {
        self.session.as_ref()
    }
    
    #[instrument(skip(self))]
    pub fn get_stats(&self) -> ProfilerStats {
        self.stats.read().unwrap().clone()
    }
    
    fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut ProfilerStats),
    {
        if let Ok(mut stats) = self.stats.write() {
            updater(&mut stats);
        }
    }
}

/// Active profiling session information
#[derive(Debug, Clone)]
pub struct ProfilingSession {
    pub name: String,
    pub start_time: Instant,
    pub config: ProfilerConfig,
    pub metadata: HashMap<String, String>,
}

impl ProfilingSession {
    pub fn new(name: String, config: ProfilerConfig) -> Self {
        Self {
            name,
            start_time: Instant::now(),
            config,
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Collected profiling data from a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub session_name: String,
    pub timestamp: SystemTime,
    pub session_duration: Duration,
    pub mode_data: HashMap<ProfilerMode, Vec<u8>>, // Serialized data per mode
    pub metadata: HashMap<String, String>,
}

impl ProfileData {
    pub fn new(session_name: String) -> Self {
        Self {
            session_name,
            timestamp: SystemTime::now(),
            session_duration: Duration::default(),
            mode_data: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_mode_data(&mut self, mode: ProfilerMode, data: Vec<u8>) {
        self.mode_data.insert(mode, data);
    }
    
    pub fn get_mode_data(&self, mode: &ProfilerMode) -> Option<&[u8]> {
        self.mode_data.get(mode).map(|v| v.as_slice())
    }
}

/// Profiler statistics and metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfilerStats {
    pub sessions_started: u64,
    pub sessions_completed: u64,
    pub total_profiling_time: Duration,
    pub data_points_collected: u64,
    pub bytes_collected: u64,
    pub errors_encountered: u64,
}

impl ProfilerStats {
    pub fn success_rate(&self) -> f64 {
        if self.sessions_started == 0 {
            0.0
        } else {
            self.sessions_completed as f64 / self.sessions_started as f64
        }
    }
    
    pub fn average_session_duration(&self) -> Duration {
        if self.sessions_completed == 0 {
            Duration::default()
        } else {
            self.total_profiling_time / self.sessions_completed as u32
        }
    }
}

/// Data collector trait for different profiling modes
pub trait DataCollector: Send + Sync {
    fn start_collection(&mut self) -> Result<(), ProfilerError>;
    fn stop_collection(&mut self) -> Result<Vec<u8>, ProfilerError>;
    fn is_collecting(&self) -> bool;
    fn get_stats(&self) -> CollectorStats;
}

/// Statistics for individual data collectors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectorStats {
    pub data_points: u64,
    pub bytes_collected: u64,
    pub collection_time: Duration,
    pub errors: u64,
}

/// Profiler builder for easy configuration
#[derive(Debug)]
pub struct ProfilerBuilder {
    config: ProfilerConfig,
}

impl ProfilerBuilder {
    pub fn new() -> Self {
        Self {
            config: ProfilerConfig::default(),
        }
    }
    
    pub fn with_modes(mut self, modes: Vec<ProfilerMode>) -> Self {
        self.config.modes = modes;
        self
    }
    
    pub fn with_cpu_sampling(mut self, frequency: u64) -> Self {
        self.config.cpu_sampling_frequency = frequency;
        self
    }
    
    pub fn with_memory_threshold(mut self, threshold: usize) -> Self {
        self.config.memory_tracking_threshold = threshold;
        self
    }
    
    pub fn with_output_dir(mut self, dir: String) -> Self {
        self.config.output_directory = dir;
        self
    }
    
    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.config.output_format = format;
        self
    }
    
    pub fn build(self) -> CursedProfiler {
        CursedProfiler::new(self.config)
    }
}

impl Default for ProfilerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Profiler errors
#[derive(Debug, thiserror::Error)]
pub enum ProfilerError {
    #[error("Profiling session is already active")]
    SessionAlreadyActive,
    
    #[error("No active profiling session")]
    NoActiveSession,
    
    #[error("Failed to start collector for mode: {0:?}")]
    CollectorStartFailed(ProfilerMode),
    
    #[error("Failed to stop collector for mode: {0:?}")]
    CollectorStopFailed(ProfilerMode),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Unsupported profiling mode: {0:?}")]
    UnsupportedMode(ProfilerMode),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_profiler_builder() {
        let profiler = ProfilerBuilder::new()
            .with_modes(vec![ProfilerMode::Cpu, ProfilerMode::Memory])
            .with_cpu_sampling(200)
            .with_output_dir("test_output".to_string())
            .build();
        
        assert_eq!(profiler.config.cpu_sampling_frequency, 200);
        assert_eq!(profiler.config.output_directory, "test_output");
        assert_eq!(profiler.config.modes.len(), 2);
    }
    
    #[test]
    fn test_profiler_stats() {
        let mut stats = ProfilerStats::default();
        stats.sessions_started = 10;
        stats.sessions_completed = 8;
        
        assert_eq!(stats.success_rate(), 0.8);
    }
    
    #[test]
    fn test_profiling_session() {
        let config = ProfilerConfig::default();
        let mut session = ProfilingSession::new("test_session".to_string(), config);
        
        session.add_metadata("version".to_string(), "1.0.0".to_string());
        assert_eq!(session.metadata.get("version"), Some(&"1.0.0".to_string()));
    }
}
