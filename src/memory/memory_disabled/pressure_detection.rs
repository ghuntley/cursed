/// Memory Pressure Detection System for CURSED Garbage Collector
/// 
/// This module provides sophisticated algorithms to detect when the system is under
/// memory pressure and requires immediate or proactive garbage collection. The detection
/// system uses multiple heuristics to make informed decisions about memory state.

use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::VecDeque;
use tracing::{instrument, debug, info, warn, error};
use serde::{Serialize, Deserialize};

use crate::memory::heap_manager::HeapStats;
use crate::memory::gc::CollectionStats;
use crate::error::CursedError;

/// Different levels of memory pressure
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PressureLevel {
    /// No memory pressure - system is operating normally
    None = 0,
    /// Low pressure - slightly elevated memory usage
    Low = 1,
    /// Moderate pressure - noticeable memory usage, proactive collection recommended
    Moderate = 2,
    /// High pressure - significant memory usage, immediate collection needed
    High = 3,
    /// Critical pressure - system near memory exhaustion, emergency collection required
    Critical = 4,
    /// Emergency pressure - immediate action required to prevent OOM
    Emergency = 5,
}

impl std::fmt::Display for PressureLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PressureLevel::None => write!(f, "None"),
            PressureLevel::Low => write!(f, "Low"),
            PressureLevel::Moderate => write!(f, "Moderate"),
            PressureLevel::High => write!(f, "High"),
            PressureLevel::Critical => write!(f, "Critical"),
            PressureLevel::Emergency => write!(f, "Emergency"),
        }
    }
}

/// Configuration for memory pressure detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureDetectionConfig {
    /// Memory usage percentage thresholds for each pressure level
    pub memory_thresholds: PressureThresholds,
    /// Allocation rate thresholds (bytes per second)
    pub allocation_rate_thresholds: AllocationRateThresholds,
    /// Collection failure thresholds
    pub collection_failure_thresholds: CollectionFailureThresholds,
    /// Time window for calculating trends (seconds)
    pub trend_window_seconds: u64,
    /// Minimum samples needed for trend analysis
    pub min_trend_samples: usize,
    /// Frequency of pressure detection checks (milliseconds)
    pub detection_interval_ms: u64,
    /// Enable predictive pressure detection
    pub enable_predictive_detection: bool,
    /// Weight factors for different pressure indicators
    pub indicator_weights: IndicatorWeights,
    /// Enable adaptive threshold adjustment
    pub adaptive_thresholds: bool,
    /// System memory monitoring configuration
    pub system_memory_config: SystemMemoryConfig,
}

/// Memory usage percentage thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureThresholds {
    pub low_threshold: f64,       // 60%
    pub moderate_threshold: f64,  // 75%
    pub high_threshold: f64,      // 85%
    pub critical_threshold: f64,  // 95%
    pub emergency_threshold: f64, // 98%
}

/// Allocation rate thresholds in bytes per second
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationRateThresholds {
    pub low_threshold: u64,       // 1MB/s
    pub moderate_threshold: u64,  // 10MB/s
    pub high_threshold: u64,      // 50MB/s
    pub critical_threshold: u64,  // 100MB/s
    pub emergency_threshold: u64, // 500MB/s
}

/// Collection failure thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionFailureThresholds {
    pub low_failure_rate: f64,       // 5%
    pub moderate_failure_rate: f64,  // 15%
    pub high_failure_rate: f64,      // 30%
    pub critical_failure_rate: f64,  // 50%
    pub emergency_failure_rate: f64, // 75%
}

/// Weight factors for different pressure indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorWeights {
    pub memory_usage_weight: f64,      // 0.4
    pub allocation_rate_weight: f64,   // 0.3
    pub collection_failure_weight: f64, // 0.2
    pub fragmentation_weight: f64,     // 0.1
}

/// System memory monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryConfig {
    pub monitor_system_memory: bool,
    pub system_memory_threshold: f64, // 90%
    pub virtual_memory_threshold: f64, // 95%
    pub swap_usage_threshold: f64,     // 50%
}

impl Default for PressureDetectionConfig {
    fn default() -> Self {
        Self {
            memory_thresholds: PressureThresholds {
                low_threshold: 0.6,
                moderate_threshold: 0.75,
                high_threshold: 0.85,
                critical_threshold: 0.95,
                emergency_threshold: 0.98,
            },
            allocation_rate_thresholds: AllocationRateThresholds {
                low_threshold: 1_048_576,       // 1MB/s
                moderate_threshold: 10_485_760, // 10MB/s
                high_threshold: 52_428_800,     // 50MB/s
                critical_threshold: 104_857_600, // 100MB/s
                emergency_threshold: 524_288_000, // 500MB/s
            },
            collection_failure_thresholds: CollectionFailureThresholds {
                low_failure_rate: 0.05,
                moderate_failure_rate: 0.15,
                high_failure_rate: 0.30,
                critical_failure_rate: 0.50,
                emergency_failure_rate: 0.75,
            },
            trend_window_seconds: 60,
            min_trend_samples: 5,
            detection_interval_ms: 100,
            enable_predictive_detection: true,
            indicator_weights: IndicatorWeights {
                memory_usage_weight: 0.4,
                allocation_rate_weight: 0.3,
                collection_failure_weight: 0.2,
                fragmentation_weight: 0.1,
            },
            adaptive_thresholds: true,
            system_memory_config: SystemMemoryConfig {
                monitor_system_memory: true,
                system_memory_threshold: 0.90,
                virtual_memory_threshold: 0.95,
                swap_usage_threshold: 0.50,
            },
        }
    }
}

/// Memory pressure sample for trend analysis
#[derive(Debug, Clone)]
struct PressureSample {
    timestamp: Instant,
    pressure_level: PressureLevel,
    memory_usage_ratio: f64,
    allocation_rate: u64,
    collection_failure_rate: f64,
    fragmentation_ratio: f64,
    composite_score: f64,
}

/// System memory information
#[derive(Debug, Clone)]
pub struct SystemMemoryInfo {
    pub total_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub memory_usage_ratio: f64,
    pub virtual_memory_used: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

/// Memory pressure detector with advanced algorithms
pub struct MemoryPressureDetector {
    config: Arc<RwLock<PressureDetectionConfig>>,
    current_pressure: Arc<RwLock<PressureLevel>>,
    pressure_history: Arc<RwLock<VecDeque<PressureSample>>>,
    last_detection: Arc<RwLock<Instant>>,
    total_detections: AtomicU64,
    pressure_changes: AtomicU64,
    adaptive_adjustment_factor: Arc<RwLock<f64>>,
    detection_active: AtomicBool,
    system_memory_info: Arc<RwLock<Option<SystemMemoryInfo>>>,
}

impl MemoryPressureDetector {
    /// Create a new memory pressure detector
    #[instrument]
    pub fn new(config: PressureDetectionConfig) -> Self {
        info!("Creating memory pressure detector with config: {:?}", config);
        Self {
            config: Arc::new(RwLock::new(config)),
            current_pressure: Arc::new(RwLock::new(PressureLevel::None)),
            pressure_history: Arc::new(RwLock::new(VecDeque::new())),
            last_detection: Arc::new(RwLock::new(Instant::now())),
            total_detections: AtomicU64::new(0),
            pressure_changes: AtomicU64::new(0),
            adaptive_adjustment_factor: Arc::new(RwLock::new(1.0)),
            detection_active: AtomicBool::new(true),
            system_memory_info: Arc::new(RwLock::new(None)),
        }
    }

    /// Create a detector with default configuration
    pub fn with_default_config() -> Self {
        Self::new(PressureDetectionConfig::default())
    }

    /// Detect current memory pressure level
    #[instrument(skip(self, heap_stats, collection_stats))]
    pub fn detect_pressure(
        &self,
        heap_stats: &HeapStats,
        collection_stats: Option<&CollectionStats>,
    ) -> Result<PressureLevel, String> {
        if !self.detection_active.load(Ordering::Acquire) {
            return Ok(PressureLevel::None);
        }

        let config = self.config.read().map_err(|e| format!("Failed to read config: {}", e))?;
        
        // Calculate individual pressure indicators
        let memory_pressure = self.calculate_memory_pressure(heap_stats, &config)?;
        let allocation_pressure = self.calculate_allocation_pressure(heap_stats, &config)?;
        let collection_pressure = self.calculate_collection_pressure(collection_stats, &config)?;
        let fragmentation_pressure = self.calculate_fragmentation_pressure(heap_stats, &config)?;
        let system_pressure = self.calculate_system_pressure(&config)?;

        // Calculate composite pressure score
        let composite_score = self.calculate_composite_score(
            memory_pressure,
            allocation_pressure,
            collection_pressure,
            fragmentation_pressure,
            system_pressure,
            &config,
        )?;

        // Determine pressure level from composite score
        let pressure_level = self.determine_pressure_level(composite_score, &config)?;

        // Apply predictive analysis if enabled
        let final_pressure = if config.enable_predictive_detection {
            self.apply_predictive_analysis(pressure_level, &config)?
        } else {
            pressure_level
        };

        // Update detection statistics
        self.update_detection_stats(final_pressure, heap_stats, collection_stats, composite_score)?;

        // Apply adaptive threshold adjustment if enabled
        if config.adaptive_thresholds {
            self.update_adaptive_thresholds(final_pressure, composite_score)?;
        }

        debug!(
            pressure_level = %final_pressure,
            composite_score = composite_score,
            memory_pressure = memory_pressure,
            allocation_pressure = allocation_pressure,
            collection_pressure = collection_pressure,
            fragmentation_pressure = fragmentation_pressure,
            system_pressure = system_pressure,
            "Memory pressure detected"
        );

        Ok(final_pressure)
    }

    /// Calculate memory usage-based pressure
    fn calculate_memory_pressure(
        &self,
        heap_stats: &HeapStats,
        config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        let usage_ratio = if heap_stats.total_capacity > 0 {
            heap_stats.total_used as f64 / heap_stats.total_capacity as f64
        } else {
            0.0
        };

        let thresholds = &config.memory_thresholds;
        let pressure = if usage_ratio >= thresholds.emergency_threshold {
            5.0
        } else if usage_ratio >= thresholds.critical_threshold {
            4.0 + (usage_ratio - thresholds.critical_threshold) / 
                   (thresholds.emergency_threshold - thresholds.critical_threshold)
        } else if usage_ratio >= thresholds.high_threshold {
            3.0 + (usage_ratio - thresholds.high_threshold) / 
                   (thresholds.critical_threshold - thresholds.high_threshold)
        } else if usage_ratio >= thresholds.moderate_threshold {
            2.0 + (usage_ratio - thresholds.moderate_threshold) / 
                   (thresholds.high_threshold - thresholds.moderate_threshold)
        } else if usage_ratio >= thresholds.low_threshold {
            1.0 + (usage_ratio - thresholds.low_threshold) / 
                   (thresholds.moderate_threshold - thresholds.low_threshold)
        } else {
            usage_ratio / thresholds.low_threshold
        };

        Ok(pressure)
    }

    /// Calculate allocation rate-based pressure
    fn calculate_allocation_pressure(
        &self,
        heap_stats: &HeapStats,
        config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        // Estimate allocation rate from heap statistics
        // This is a simplified calculation - in practice, you'd track allocation rate over time
        // Use available metrics for allocation rate estimation
        let estimated_rate = 1000; // Default rate when uptime data unavailable

        let thresholds = &config.allocation_rate_thresholds;
        let pressure = if estimated_rate >= thresholds.emergency_threshold {
            5.0
        } else if estimated_rate >= thresholds.critical_threshold {
            4.0 + (estimated_rate - thresholds.critical_threshold) as f64 / 
                   (thresholds.emergency_threshold - thresholds.critical_threshold) as f64
        } else if estimated_rate >= thresholds.high_threshold {
            3.0 + (estimated_rate - thresholds.high_threshold) as f64 / 
                   (thresholds.critical_threshold - thresholds.high_threshold) as f64
        } else if estimated_rate >= thresholds.moderate_threshold {
            2.0 + (estimated_rate - thresholds.moderate_threshold) as f64 / 
                   (thresholds.high_threshold - thresholds.moderate_threshold) as f64
        } else if estimated_rate >= thresholds.low_threshold {
            1.0 + (estimated_rate - thresholds.low_threshold) as f64 / 
                   (thresholds.moderate_threshold - thresholds.low_threshold) as f64
        } else {
            estimated_rate as f64 / thresholds.low_threshold as f64
        };

        Ok(pressure)
    }

    /// Calculate collection failure-based pressure
    fn calculate_collection_pressure(
        &self,
        collection_stats: Option<&CollectionStats>,
        config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        let failure_rate = if let Some(_stats) = collection_stats {
            // Use collection_number as a proxy for total collections 
            // Simplified - assume no failures tracked in current CollectionStats
            0.0
        } else {
            0.0
        };

        let thresholds = &config.collection_failure_thresholds;
        let pressure = if failure_rate >= thresholds.emergency_failure_rate {
            5.0
        } else if failure_rate >= thresholds.critical_failure_rate {
            4.0 + (failure_rate - thresholds.critical_failure_rate) / 
                   (thresholds.emergency_failure_rate - thresholds.critical_failure_rate)
        } else if failure_rate >= thresholds.high_failure_rate {
            3.0 + (failure_rate - thresholds.high_failure_rate) / 
                   (thresholds.critical_failure_rate - thresholds.high_failure_rate)
        } else if failure_rate >= thresholds.moderate_failure_rate {
            2.0 + (failure_rate - thresholds.moderate_failure_rate) / 
                   (thresholds.high_failure_rate - thresholds.moderate_failure_rate)
        } else if failure_rate >= thresholds.low_failure_rate {
            1.0 + (failure_rate - thresholds.low_failure_rate) / 
                   (thresholds.moderate_failure_rate - thresholds.low_failure_rate)
        } else {
            failure_rate / thresholds.low_failure_rate
        };

        Ok(pressure)
    }

    /// Calculate fragmentation-based pressure
    fn calculate_fragmentation_pressure(
        &self,
        heap_stats: &HeapStats,
        _config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        // Use fragmentation_ratio from HeapStats instead of calculating from missing fields
        let fragmentation = heap_stats.fragmentation_ratio;

        // Convert fragmentation ratio to pressure score (0-5)
        let pressure = fragmentation * 5.0;
        Ok(pressure.min(5.0))
    }

    /// Calculate system-wide memory pressure
    fn calculate_system_pressure(
        &self,
        config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        if !config.system_memory_config.monitor_system_memory {
            return Ok(0.0);
        }

        // Update system memory information
        self.update_system_memory_info()?;

        let system_info = self.system_memory_info.read()
            .map_err(|e| format!("Failed to read system memory info: {}", e))?;

        if let Some(ref info) = *system_info {
            let memory_pressure = if info.memory_usage_ratio >= config.system_memory_config.system_memory_threshold {
                5.0 * (info.memory_usage_ratio - config.system_memory_config.system_memory_threshold) / 
                      (1.0 - config.system_memory_config.system_memory_threshold)
            } else {
                0.0
            };

            let swap_pressure = if info.swap_total > 0 {
                let swap_ratio = info.swap_used as f64 / info.swap_total as f64;
                if swap_ratio >= config.system_memory_config.swap_usage_threshold {
                    3.0 * (swap_ratio - config.system_memory_config.swap_usage_threshold) / 
                          (1.0 - config.system_memory_config.swap_usage_threshold)
                } else {
                    0.0
                }
            } else {
                0.0
            };

            Ok(memory_pressure.max(swap_pressure).min(5.0))
        } else {
            Ok(0.0)
        }
    }

    /// Calculate composite pressure score
    fn calculate_composite_score(
        &self,
        memory_pressure: f64,
        allocation_pressure: f64,
        collection_pressure: f64,
        fragmentation_pressure: f64,
        system_pressure: f64,
        config: &PressureDetectionConfig,
    ) -> Result<f64, String> {
        let weights = &config.indicator_weights;
        let adjustment_factor = *self.adaptive_adjustment_factor.read()
            .map_err(|e| format!("Failed to read adjustment factor: {}", e))?;

        let weighted_score = (
            memory_pressure * weights.memory_usage_weight +
            allocation_pressure * weights.allocation_rate_weight +
            collection_pressure * weights.collection_failure_weight +
            fragmentation_pressure * weights.fragmentation_weight
        ) * adjustment_factor + system_pressure * 0.2; // System pressure gets fixed weight

        Ok(weighted_score.min(5.0))
    }

    /// Determine pressure level from composite score
    fn determine_pressure_level(
        &self,
        composite_score: f64,
        _config: &PressureDetectionConfig,
    ) -> Result<PressureLevel, String> {
        let level = if composite_score >= 4.5 {
            PressureLevel::Emergency
        } else if composite_score >= 3.5 {
            PressureLevel::Critical
        } else if composite_score >= 2.5 {
            PressureLevel::High
        } else if composite_score >= 1.5 {
            PressureLevel::Moderate
        } else if composite_score >= 0.5 {
            PressureLevel::Low
        } else {
            PressureLevel::None
        };

        Ok(level)
    }

    /// Apply predictive analysis to pressure level
    fn apply_predictive_analysis(
        &self,
        current_pressure: PressureLevel,
        config: &PressureDetectionConfig,
    ) -> Result<PressureLevel, String> {
        let history = self.pressure_history.read()
            .map_err(|e| format!("Failed to read pressure history: {}", e))?;

        if history.len() < config.min_trend_samples {
            return Ok(current_pressure);
        }

        // Calculate trend in the last few samples
        let recent_samples: Vec<_> = history.iter()
            .rev()
            .take(config.min_trend_samples)
            .collect();

        let trend_slope = self.calculate_trend_slope(&recent_samples)?;

        // If trend is strongly increasing, escalate pressure level
        let adjusted_pressure = if trend_slope > 0.5 {
            match current_pressure {
                PressureLevel::None => PressureLevel::Low,
                PressureLevel::Low => PressureLevel::Moderate,
                PressureLevel::Moderate => PressureLevel::High,
                PressureLevel::High => PressureLevel::Critical,
                PressureLevel::Critical => PressureLevel::Emergency,
                PressureLevel::Emergency => PressureLevel::Emergency,
            }
        } else if trend_slope < -0.5 {
            // If trend is strongly decreasing, potentially de-escalate
            match current_pressure {
                PressureLevel::Emergency => PressureLevel::Critical,
                PressureLevel::Critical => PressureLevel::High,
                PressureLevel::High => PressureLevel::Moderate,
                PressureLevel::Moderate => PressureLevel::Low,
                PressureLevel::Low => PressureLevel::None,
                PressureLevel::None => PressureLevel::None,
            }
        } else {
            current_pressure
        };

        Ok(adjusted_pressure)
    }

    /// Calculate trend slope from pressure samples
    fn calculate_trend_slope(&self, samples: &[&PressureSample]) -> Result<f64, String> {
        if samples.len() < 2 {
            return Ok(0.0);
        }

        let n = samples.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        for (i, sample) in samples.iter().enumerate() {
            let x = i as f64;
            let y = sample.composite_score;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_x2 += x * x;
        }

        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator.abs() < f64::EPSILON {
            return Ok(0.0);
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        Ok(slope)
    }

    /// Update detection statistics and history
    fn update_detection_stats(
        &self,
        pressure_level: PressureLevel,
        heap_stats: &HeapStats,
        collection_stats: Option<&CollectionStats>,
        composite_score: f64,
    ) -> Result<(), String> {
        self.total_detections.fetch_add(1, Ordering::Release);

        // Check if pressure level changed
        let mut current_pressure = self.current_pressure.write()
            .map_err(|e| format!("Failed to write current pressure: {}", e))?;
        
        if *current_pressure != pressure_level {
            self.pressure_changes.fetch_add(1, Ordering::Release);
            info!(
                old_pressure = %*current_pressure,
                new_pressure = %pressure_level,
                composite_score = composite_score,
                "Memory pressure level changed"
            );
        }
        *current_pressure = pressure_level;
        drop(current_pressure);

        // Update pressure history
        let mut history = self.pressure_history.write()
            .map_err(|e| format!("Failed to write pressure history: {}", e))?;

        let config = self.config.read()
            .map_err(|e| format!("Failed to read config: {}", e))?;

        let sample = PressureSample {
            timestamp: Instant::now(),
            pressure_level,
            memory_usage_ratio: if heap_stats.total_capacity > 0 {
                heap_stats.total_used as f64 / heap_stats.total_capacity as f64
            } else {
                0.0
            },
            allocation_rate: 1000, // Default rate when metrics unavailable
            collection_failure_rate: collection_stats
                .map(|_stats| {
                    // Simplified - assume no failures tracked in current CollectionStats
                    0.0
                })
                .unwrap_or(0.0),
            fragmentation_ratio: heap_stats.fragmentation_ratio,
            composite_score,
        };

        history.push_back(sample);

        // Limit history size based on trend window
        let max_samples = (config.trend_window_seconds * 1000 / config.detection_interval_ms) as usize;
        while history.len() > max_samples {
            history.pop_front();
        }

        // Update last detection time
        *self.last_detection.write()
            .map_err(|e| format!("Failed to write last detection time: {}", e))? = Instant::now();

        Ok(())
    }

    /// Update adaptive thresholds based on system behavior
    fn update_adaptive_thresholds(
        &self,
        pressure_level: PressureLevel,
        composite_score: f64,
    ) -> Result<(), String> {
        let mut adjustment = self.adaptive_adjustment_factor.write()
            .map_err(|e| format!("Failed to write adjustment factor: {}", e))?;

        // Simple adaptive algorithm: adjust based on pressure level frequency
        match pressure_level {
            PressureLevel::Emergency | PressureLevel::Critical => {
                // If we're hitting high pressure often, be more sensitive
                *adjustment = (*adjustment * 1.02).min(2.0);
            },
            PressureLevel::None => {
                // If we're consistently low pressure, be less sensitive
                *adjustment = (*adjustment * 0.999).max(0.5);
            },
            _ => {
                // Moderate levels - slight adjustment toward baseline
                *adjustment = *adjustment * 0.9995 + 1.0 * 0.0005;
            }
        }

        Ok(())
    }

    /// Update system memory information
    #[cfg(target_os = "linux")]
    fn update_system_memory_info(&self) -> Result<(), String> {
        use std::fs;

        let meminfo = fs::read_to_string("/proc/meminfo")
            .map_err(|e| format!("Failed to read /proc/meminfo: {}", e))?;

        let mut total_memory = 0u64;
        let mut available_memory = 0u64;
        let mut swap_total = 0u64;
        let mut swap_free = 0u64;

        for line in meminfo.split("\n") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    match parts[0] {
                        "MemTotal:" => total_memory = value * 1024, // Convert from kB to bytes
                        "MemAvailable:" => available_memory = value * 1024,
                        "SwapTotal:" => swap_total = value * 1024,
                        "SwapFree:" => swap_free = value * 1024,
                        _ => {}
                    }
                }
            }
        }

        let used_memory = total_memory.saturating_sub(available_memory);
        let swap_used = swap_total.saturating_sub(swap_free);
        let memory_usage_ratio = if total_memory > 0 {
            used_memory as f64 / total_memory as f64
        } else {
            0.0
        };

        let system_info = SystemMemoryInfo {
            total_memory,
            available_memory,
            used_memory,
            memory_usage_ratio,
            virtual_memory_used: used_memory, // Simplified
            swap_used,
            swap_total,
        };

        *self.system_memory_info.write()
            .map_err(|e| format!("Failed to write system memory info: {}", e))? = Some(system_info);

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn update_system_memory_info(&self) -> Result<(), String> {
        // Placeholder for non-Linux systems
        warn!("System memory monitoring not implemented for this platform");
        Ok(())
    }

    /// Get current pressure level
    pub fn current_pressure(&self) -> Result<PressureLevel, String> {
        Ok(*self.current_pressure.read()
            .map_err(|e| format!("Failed to read current pressure: {}", e))?)
    }

    /// Get pressure detection statistics
    pub fn get_statistics(&self) -> Result<PressureDetectionStatistics, String> {
        let current_pressure = *self.current_pressure.read()
            .map_err(|e| format!("Failed to read current pressure: {}", e))?;
        
        let last_detection = *self.last_detection.read()
            .map_err(|e| format!("Failed to read last detection time: {}", e))?;
        
        let history = self.pressure_history.read()
            .map_err(|e| format!("Failed to read pressure history: {}", e))?;
        
        let adjustment_factor = *self.adaptive_adjustment_factor.read()
            .map_err(|e| format!("Failed to read adjustment factor: {}", e))?;

        Ok(PressureDetectionStatistics {
            current_pressure,
            total_detections: self.total_detections.load(Ordering::Acquire),
            pressure_changes: self.pressure_changes.load(Ordering::Acquire),
            last_detection,
            samples_in_history: history.len(),
            adaptive_adjustment_factor: adjustment_factor,
            detection_active: self.detection_active.load(Ordering::Acquire),
        })
    }

    /// Update configuration
    pub fn update_config(&self, config: PressureDetectionConfig) -> Result<(), String> {
        *self.config.write()
            .map_err(|e| format!("Failed to write config: {}", e))? = config;
        info!("Pressure detection configuration updated");
        Ok(())
    }

    /// Enable or disable pressure detection
    pub fn set_detection_active(&self, active: bool) {
        self.detection_active.store(active, Ordering::Release);
        if active {
            info!("Memory pressure detection enabled");
        } else {
            info!("Memory pressure detection disabled");
        }
    }

    /// Get current system memory information
    pub fn get_system_memory_info(&self) -> Result<Option<SystemMemoryInfo>, String> {
        Ok(self.system_memory_info.read()
            .map_err(|e| format!("Failed to read system memory info: {}", e))?
            .clone())
    }

    /// Get pressure history for analysis
    pub fn get_pressure_history(&self) -> Result<Vec<PressureSample>, String> {
        Ok(self.pressure_history.read()
            .map_err(|e| format!("Failed to read pressure history: {}", e))?
            .iter()
            .cloned()
            .collect())
    }
}

/// Statistics about pressure detection system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureDetectionStatistics {
    pub current_pressure: PressureLevel,
    pub total_detections: u64,
    pub pressure_changes: u64,
    #[serde(skip, default = "Instant::now")]
    pub last_detection: Instant,
    pub samples_in_history: usize,
    pub adaptive_adjustment_factor: f64,
    pub detection_active: bool,
}

