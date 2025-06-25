use crate::error::CursedError;
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
    /// Sampling frequency for CPU profiling (Hz)
    /// Memory allocation tracking threshold (bytes)
    /// Maximum number of stack frames to capture
    /// Enable goroutine tracking
    /// Enable I/O operation tracking
    /// Output directory for profiling data
    /// Maximum profiling session duration
    /// Profiling data format (JSON, binary, flame graph)
    /// Performance regression detection threshold
impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            regression_threshold: 0.1, // 10% performance regression
        }
    }
/// Profiling modes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfilerMode {
    /// CPU profiling with call stack sampling
    /// Memory allocation and usage tracking
    /// Goroutine and channel concurrency profiling
    /// I/O operation profiling
    /// Custom profiling mode
/// Output formats for profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
/// Main profiler coordinator
pub struct CursedProfiler {
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
        
        profiler.initialize_collectors();
        profiler
    #[instrument(skip(self))]
    fn initialize_collectors(&mut self) {
        for mode in &self.config.modes {
            match mode {
                ProfilerMode::Cpu => {
                    self.data_collectors.insert(
                        Box::new(crate::profiling::cpu::CpuProfiler::new(
                    );
                }
                ProfilerMode::Memory => {
                    self.data_collectors.insert(
                        Box::new(crate::profiling::memory::MemoryProfiler::new(
                    );
                }
                ProfilerMode::Concurrency => {
                    if self.config.track_goroutines {
                        self.data_collectors.insert(
                        );
                    }
                }
                ProfilerMode::Io => {
                    if self.config.track_io_operations {
                        self.data_collectors.insert(
                        );
                    }
                }
                ProfilerMode::Custom(name) => {
                    warn!("Custom profiling mode '{}' not implemented", name);
                }
            }
        info!("Initialized {} data collectors", self.data_collectors.len());
    #[instrument(skip(self))]
    pub fn start_session(&mut self, session_name: String) -> crate::error::Result<()> {
        if self.session.is_some() {
            return Err(ProfilerError::SessionAlreadyActive);
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
    #[instrument(skip(self))]
    pub fn stop_session(&mut self) -> crate::error::Result<()> {
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
        profile_data.session_duration = session.start_time.elapsed();
        profile_data.timestamp = SystemTime::now();
        
        self.update_stats(|stats| {
            stats.sessions_completed += 1;
            stats.total_profiling_time += profile_data.session_duration;
        });
        
        info!("Profiling session completed in {:?}", profile_data.session_duration);
        Ok(profile_data)
    #[instrument(skip(self))]
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    #[instrument(skip(self))]
    pub fn get_current_session(&self) -> Option<&ProfilingSession> {
        self.session.as_ref()
    #[instrument(skip(self))]
    pub fn get_stats(&self) -> ProfilerStats {
        self.stats.read().unwrap().clone()
    fn update_stats<F>(&self, updater: F)
    where
    {
        if let Ok(mut stats) = self.stats.write() {
            updater(&mut stats);
        }
    }
/// Active profiling session information
#[derive(Debug, Clone)]
pub struct ProfilingSession {
impl ProfilingSession {
    pub fn new(name: String, config: ProfilerConfig) -> Self {
        Self {
        }
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Collected profiling data from a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub mode_data: HashMap<ProfilerMode, Vec<u8>>, // Serialized data per mode
impl ProfileData {
    pub fn new(session_name: String) -> Self {
        Self {
        }
    }
    
    pub fn add_mode_data(&mut self, mode: ProfilerMode, data: Vec<u8>) {
        self.mode_data.insert(mode, data);
    pub fn get_mode_data(&self, mode: &ProfilerMode) -> Option<&[u8]> {
        self.mode_data.get(mode).map(|v| v.as_slice())
    }
}

/// Profiler statistics and metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfilerStats {
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
/// Data collector trait for different profiling modes
pub trait DataCollector: Send + Sync {
    fn start_collection(&mut self) -> crate::error::Result<()>;
    fn stop_collection(&mut self) -> crate::error::Result<()>;
    fn is_collecting(&self) -> bool;
    fn get_stats(&self) -> CollectorStats;
/// Statistics for individual data collectors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectorStats {
/// Profiler builder for easy configuration
#[derive(Debug)]
pub struct ProfilerBuilder {
impl ProfilerBuilder {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn with_modes(mut self, modes: Vec<ProfilerMode>) -> Self {
        self.config.modes = modes;
        self
    pub fn with_cpu_sampling(mut self, frequency: u64) -> Self {
        self.config.cpu_sampling_frequency = frequency;
        self
    pub fn with_memory_threshold(mut self, threshold: usize) -> Self {
        self.config.memory_tracking_threshold = threshold;
        self
    pub fn with_output_dir(mut self, dir: String) -> Self {
        self.config.output_directory = dir;
        self
    pub fn with_format(mut self, format: OutputFormat) -> Self {
        self.config.output_format = format;
        self
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
#[derive(Debug, thiserror::CursedError)]
pub enum ProfilerError {
    #[error("Profiling session is already active")]
    
    #[error("No active profiling session")]
    
    #[error("Failed to start collector for mode: {0:?}")]
    
    #[error("Failed to stop collector for mode: {0:?}")]
    
    #[error("I/O error: {0}")]
    
    #[error("Serialization error: {0}")]
    
    #[error("Configuration error: {0}")]
    
    #[error("Unsupported profiling mode: {0:?}")]
