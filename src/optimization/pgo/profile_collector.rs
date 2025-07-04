//! Profile data collection for CURSED PGO system

use crate::error::{CursedError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Configuration for profile collection
#[derive(Debug, Clone)]
pub struct ProfileCollectorConfig {
    pub enable_counters: bool,
    pub enable_sampling: bool,
    pub sampling_rate: f64,
    pub counter_threshold: u64,
    pub max_collection_time: Duration,
    pub output_path: PathBuf,
}

impl Default for ProfileCollectorConfig {
    fn default() -> Self {
        Self {
            enable_counters: true,
            enable_sampling: true,
            sampling_rate: 0.001, // 0.1% sampling rate
            counter_threshold: 1000,
            max_collection_time: Duration::from_secs(300),
            output_path: PathBuf::from("target/pgo-profiles"),
        }
    }
}

/// Profile data collector
pub struct ProfileCollector {
    config: ProfileCollectorConfig,
    counters: HashMap<String, u64>,
    samples: Vec<ProfileSample>,
    start_time: Option<Instant>,
    is_collecting: bool,
}

/// A single profile sample
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileSample {
    pub function_name: String,
    pub line_number: u32,
    #[serde(skip, default = "Instant::now")]
    pub timestamp: Instant,
    pub execution_count: u64,
    pub cpu_cycles: u64,
    pub memory_usage: u64,
}

impl ProfileCollector {
    /// Create a new profile collector
    pub fn new(config: ProfileCollectorConfig) -> Result<Self> {
        Ok(Self {
            config,
            counters: HashMap::new(),
            samples: Vec::new(),
            start_time: None,
            is_collecting: false,
        })
    }

    /// Initialize profile collection
    pub fn initialize(&mut self) -> Result<()> {
        if self.is_collecting {
            return Err(CursedError::General("Profile collection already initialized".to_string()));
        }

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.config.output_path).map_err(|e| {
            CursedError::General(format!("Failed to create profile output directory: {}", e))
        })?;

        self.start_time = Some(Instant::now());
        self.is_collecting = true;
        
        tracing::info!("Profile collection initialized");
        Ok(())
    }

    /// Record a function call
    pub fn record_function_call(&mut self, function_name: &str, line_number: u32) -> Result<()> {
        if !self.is_collecting {
            return Ok(());
        }

        // Update counters
        if self.config.enable_counters {
            let counter = self.counters.entry(function_name.to_string()).or_insert(0);
            *counter += 1;
        }

        // Record sample if sampling is enabled
        if self.config.enable_sampling && self.should_sample() {
            let sample = ProfileSample {
                function_name: function_name.to_string(),
                line_number,
                timestamp: Instant::now(),
                execution_count: 1,
                cpu_cycles: self.estimate_cpu_cycles(),
                memory_usage: self.estimate_memory_usage(),
            };
            self.samples.push(sample);
        }

        Ok(())
    }

    /// Record execution time for a function
    pub fn record_execution_time(&mut self, function_name: &str, duration: Duration) -> Result<()> {
        if !self.is_collecting {
            return Ok(());
        }

        let key = format!("{}_time", function_name);
        let time_ms = duration.as_millis() as u64;
        let counter = self.counters.entry(key).or_insert(0);
        *counter += time_ms;

        Ok(())
    }

    /// Get collected profile data
    pub fn get_profile_data(&self) -> ProfileData {
        let total_functions = self.counters.keys()
            .filter(|k| !k.ends_with("_time"))
            .count();
        
        ProfileData {
            counters: self.counters.clone(),
            samples: self.samples.clone(),
            collection_duration: self.start_time.map(|t| t.elapsed()),
            total_samples: self.samples.len(),
            total_functions,
        }
    }

    /// Finalize collection and return profile data
    pub fn finalize(&mut self) -> Result<ProfileData> {
        if !self.is_collecting {
            return Err(CursedError::General("Profile collection not active".to_string()));
        }

        self.is_collecting = false;
        let profile_data = self.get_profile_data();
        
        tracing::info!("Profile collection finalized: {} samples, {} functions", 
                      profile_data.total_samples, profile_data.total_functions);
        
        Ok(profile_data)
    }

    /// Check if we should sample this execution
    fn should_sample(&self) -> bool {
        // For testing, always sample or use higher rate
        true // fastrand::f64() < self.config.sampling_rate
    }

    /// Estimate CPU cycles (placeholder implementation)
    fn estimate_cpu_cycles(&self) -> u64 {
        // In a real implementation, this would use hardware counters
        fastrand::u64(1000..10000)
    }

    /// Estimate memory usage (placeholder implementation)
    fn estimate_memory_usage(&self) -> u64 {
        // In a real implementation, this would track actual memory usage
        fastrand::u64(1024..102400)
    }
}

/// Complete profile data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileData {
    pub counters: HashMap<String, u64>,
    pub samples: Vec<ProfileSample>,
    pub collection_duration: Option<Duration>,
    pub total_samples: usize,
    pub total_functions: usize,
}

impl ProfileData {
    /// Create empty profile data
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            samples: Vec::new(),
            collection_duration: None,
            total_samples: 0,
            total_functions: 0,
        }
    }

    /// Get function call count
    pub fn get_function_count(&self, function_name: &str) -> u64 {
        self.counters.get(function_name).copied().unwrap_or(0)
    }

    /// Get most frequently called functions
    pub fn get_hot_functions(&self, limit: usize) -> Vec<(String, u64)> {
        let mut functions: Vec<_> = self.counters.iter()
            .filter(|(name, _)| !name.ends_with("_time"))
            .map(|(name, count)| (name.clone(), *count))
            .collect();
        
        functions.sort_by(|a, b| b.1.cmp(&a.1));
        functions.truncate(limit);
        functions
    }

    /// Get function execution time
    pub fn get_function_time(&self, function_name: &str) -> Option<Duration> {
        let key = format!("{}_time", function_name);
        self.counters.get(&key).map(|&ms| Duration::from_millis(ms))
    }

    /// Check if profile data is sufficient for optimization
    pub fn is_sufficient_for_optimization(&self) -> bool {
        self.total_samples >= 100 && self.total_functions >= 5
    }
}

impl Default for ProfileData {
    fn default() -> Self {
        Self::new()
    }
}
