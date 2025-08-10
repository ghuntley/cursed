//! Memory pressure detection for GC optimization
//! 
//! This module implements memory pressure detection and automatic GC tuning
//! to optimize compilation performance under memory constraints.

use crate::error::{CursedError, Result};
use crate::runtime::gc::{GcStats, GcConfiguration};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryPressureLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Memory pressure detector
pub struct MemoryPressureDetector {
    config: MemoryPressureConfig,
    pressure_history: VecDeque<MemoryPressureSample>,
    last_gc_stats: Option<GcStats>,
    optimization_adjustments: OptimizationAdjustments,
}

/// Configuration for memory pressure detection
#[derive(Debug, Clone)]
pub struct MemoryPressureConfig {
    pub memory_threshold_mb: usize,
    pub pressure_window_samples: usize,
    pub gc_frequency_threshold: f64,
    pub allocation_rate_threshold: f64,
    pub enable_adaptive_optimization: bool,
    pub enable_size_optimization_under_pressure: bool,
}

/// Memory pressure sample
#[derive(Debug, Clone)]
pub struct MemoryPressureSample {
    pub timestamp: Instant,
    pub allocated_memory_mb: f64,
    pub available_memory_mb: f64,
    pub gc_frequency_hz: f64,
    pub allocation_rate_mb_per_sec: f64,
    pub pressure_level: MemoryPressureLevel,
}

/// Optimization adjustments based on memory pressure
#[derive(Debug, Clone)]
pub struct OptimizationAdjustments {
    pub reduce_inlining_threshold: bool,
    pub enable_aggressive_dce: bool,
    pub reduce_loop_unrolling: bool,
    pub enable_size_optimization: bool,
    pub reduce_template_instantiation: bool,
    pub enable_memory_pool_optimization: bool,
}

impl MemoryPressureDetector {
    /// Create a new memory pressure detector
    pub fn new(config: MemoryPressureConfig) -> Self {
        Self {
            config,
            pressure_history: VecDeque::with_capacity(config.pressure_window_samples),
            last_gc_stats: None,
            optimization_adjustments: OptimizationAdjustments::default(),
        }
    }
    
    /// Sample current memory pressure
    pub fn sample_memory_pressure(&mut self) -> Result<MemoryPressureSample> {
        let timestamp = Instant::now();
        
        // Get system memory info
        let (allocated_mb, available_mb) = self.get_memory_info()?;
        
        // Calculate GC frequency and allocation rate
        let gc_frequency = self.calculate_gc_frequency()?;
        let allocation_rate = self.calculate_allocation_rate(allocated_mb)?;
        
        // Determine pressure level
        let pressure_level = self.determine_pressure_level(
            allocated_mb,
            available_mb,
            gc_frequency,
            allocation_rate,
        );
        
        let sample = MemoryPressureSample {
            timestamp,
            allocated_memory_mb: allocated_mb,
            available_memory_mb: available_mb,
            gc_frequency_hz: gc_frequency,
            allocation_rate_mb_per_sec: allocation_rate,
            pressure_level,
        };
        
        // Add to history
        if self.pressure_history.len() >= self.config.pressure_window_samples {
            self.pressure_history.pop_front();
        }
        self.pressure_history.push_back(sample.clone());
        
        // Update optimization adjustments
        self.update_optimization_adjustments();
        
        Ok(sample)
    }
    
    /// Get current memory pressure level
    pub fn get_current_pressure_level(&self) -> MemoryPressureLevel {
        self.pressure_history
            .back()
            .map(|sample| sample.pressure_level)
            .unwrap_or(MemoryPressureLevel::Low)
    }
    
    /// Get optimization adjustments based on memory pressure
    pub fn get_optimization_adjustments(&self) -> &OptimizationAdjustments {
        &self.optimization_adjustments
    }
    
    /// Check if memory pressure requires immediate action
    pub fn requires_immediate_action(&self) -> bool {
        matches!(self.get_current_pressure_level(), MemoryPressureLevel::Critical)
    }
    
    /// Get recommended GC configuration based on pressure
    pub fn get_recommended_gc_config(&self) -> GcConfiguration {
        let pressure = self.get_current_pressure_level();
        
        match pressure {
            MemoryPressureLevel::Low => GcConfiguration {
                heap_size_mb: 512,
                gc_threshold: 0.8,
                nursery_size_mb: 64,
                enable_concurrent_gc: true,
                enable_incremental_gc: false,
            },
            MemoryPressureLevel::Medium => GcConfiguration {
                heap_size_mb: 256,
                gc_threshold: 0.7,
                nursery_size_mb: 32,
                enable_concurrent_gc: true,
                enable_incremental_gc: true,
            },
            MemoryPressureLevel::High => GcConfiguration {
                heap_size_mb: 128,
                gc_threshold: 0.6,
                nursery_size_mb: 16,
                enable_concurrent_gc: true,
                enable_incremental_gc: true,
            },
            MemoryPressureLevel::Critical => GcConfiguration {
                heap_size_mb: 64,
                gc_threshold: 0.5,
                nursery_size_mb: 8,
                enable_concurrent_gc: false,
                enable_incremental_gc: true,
            },
        }
    }
    
    /// Get memory usage statistics
    pub fn get_memory_statistics(&self) -> MemoryStatistics {
        let current_pressure = self.get_current_pressure_level();
        let average_allocation_rate = self.calculate_average_allocation_rate();
        let average_gc_frequency = self.calculate_average_gc_frequency();
        let peak_memory_usage = self.calculate_peak_memory_usage();
        
        MemoryStatistics {
            current_pressure_level: current_pressure,
            average_allocation_rate_mb_per_sec: average_allocation_rate,
            average_gc_frequency_hz: average_gc_frequency,
            peak_memory_usage_mb: peak_memory_usage,
            samples_collected: self.pressure_history.len(),
            optimization_adjustments_active: self.optimization_adjustments.has_adjustments(),
        }
    }
    
    // Private helper methods
    
    fn get_memory_info(&self) -> Result<(f64, f64)> {
        // Get system memory information
        // This is a simplified implementation - in practice would use system APIs
        let allocated_mb = 128.0; // Placeholder - would read from OS
        let available_mb = 1024.0; // Placeholder - would read from OS
        Ok((allocated_mb, available_mb))
    }
    
    fn calculate_gc_frequency(&mut self) -> Result<f64> {
        // Calculate GC frequency based on recent GC events
        // This would integrate with the actual GC system
        let gc_frequency = 2.5; // Placeholder - would calculate from GC stats
        Ok(gc_frequency)
    }
    
    fn calculate_allocation_rate(&self, current_allocated: f64) -> Result<f64> {
        if let Some(previous_sample) = self.pressure_history.back() {
            let time_diff = previous_sample.timestamp.elapsed().as_secs_f64();
            if time_diff > 0.0 {
                let memory_diff = current_allocated - previous_sample.allocated_memory_mb;
                return Ok(memory_diff / time_diff);
            }
        }
        Ok(0.0)
    }
    
    fn determine_pressure_level(
        &self,
        allocated_mb: f64,
        available_mb: f64,
        gc_frequency: f64,
        allocation_rate: f64,
    ) -> MemoryPressureLevel {
        let memory_usage_ratio = allocated_mb / (allocated_mb + available_mb);
        
        if memory_usage_ratio > 0.9 || gc_frequency > 10.0 {
            MemoryPressureLevel::Critical
        } else if memory_usage_ratio > 0.8 || gc_frequency > 5.0 {
            MemoryPressureLevel::High
        } else if memory_usage_ratio > 0.6 || gc_frequency > 2.0 {
            MemoryPressureLevel::Medium
        } else {
            MemoryPressureLevel::Low
        }
    }
    
    fn update_optimization_adjustments(&mut self) {
        let pressure = self.get_current_pressure_level();
        
        self.optimization_adjustments = match pressure {
            MemoryPressureLevel::Low => OptimizationAdjustments::default(),
            MemoryPressureLevel::Medium => OptimizationAdjustments {
                reduce_inlining_threshold: true,
                enable_aggressive_dce: true,
                reduce_loop_unrolling: false,
                enable_size_optimization: false,
                reduce_template_instantiation: false,
                enable_memory_pool_optimization: true,
            },
            MemoryPressureLevel::High => OptimizationAdjustments {
                reduce_inlining_threshold: true,
                enable_aggressive_dce: true,
                reduce_loop_unrolling: true,
                enable_size_optimization: true,
                reduce_template_instantiation: true,
                enable_memory_pool_optimization: true,
            },
            MemoryPressureLevel::Critical => OptimizationAdjustments {
                reduce_inlining_threshold: true,
                enable_aggressive_dce: true,
                reduce_loop_unrolling: true,
                enable_size_optimization: true,
                reduce_template_instantiation: true,
                enable_memory_pool_optimization: true,
            },
        };
    }
    
    fn calculate_average_allocation_rate(&self) -> f64 {
        if self.pressure_history.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.pressure_history.iter()
            .map(|sample| sample.allocation_rate_mb_per_sec)
            .sum();
        sum / self.pressure_history.len() as f64
    }
    
    fn calculate_average_gc_frequency(&self) -> f64 {
        if self.pressure_history.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.pressure_history.iter()
            .map(|sample| sample.gc_frequency_hz)
            .sum();
        sum / self.pressure_history.len() as f64
    }
    
    fn calculate_peak_memory_usage(&self) -> f64 {
        self.pressure_history.iter()
            .map(|sample| sample.allocated_memory_mb)
            .fold(0.0, |max, current| max.max(current))
    }
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStatistics {
    pub current_pressure_level: MemoryPressureLevel,
    pub average_allocation_rate_mb_per_sec: f64,
    pub average_gc_frequency_hz: f64,
    pub peak_memory_usage_mb: f64,
    pub samples_collected: usize,
    pub optimization_adjustments_active: bool,
}

impl Default for MemoryPressureConfig {
    fn default() -> Self {
        Self {
            memory_threshold_mb: 1024,
            pressure_window_samples: 20,
            gc_frequency_threshold: 5.0,
            allocation_rate_threshold: 100.0,
            enable_adaptive_optimization: true,
            enable_size_optimization_under_pressure: true,
        }
    }
}

impl Default for OptimizationAdjustments {
    fn default() -> Self {
        Self {
            reduce_inlining_threshold: false,
            enable_aggressive_dce: false,
            reduce_loop_unrolling: false,
            enable_size_optimization: false,
            reduce_template_instantiation: false,
            enable_memory_pool_optimization: false,
        }
    }
}

impl OptimizationAdjustments {
    /// Check if any adjustments are active
    pub fn has_adjustments(&self) -> bool {
        self.reduce_inlining_threshold ||
        self.enable_aggressive_dce ||
        self.reduce_loop_unrolling ||
        self.enable_size_optimization ||
        self.reduce_template_instantiation ||
        self.enable_memory_pool_optimization
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pressure_detector_creation() {
        let config = MemoryPressureConfig::default();
        let detector = MemoryPressureDetector::new(config);
        assert_eq!(detector.get_current_pressure_level(), MemoryPressureLevel::Low);
    }

    #[test]
    fn test_optimization_adjustments_default() {
        let adjustments = OptimizationAdjustments::default();
        assert!(!adjustments.has_adjustments());
    }

    #[test]
    fn test_pressure_level_determination() {
        let config = MemoryPressureConfig::default();
        let detector = MemoryPressureDetector::new(config);
        
        // Test critical pressure level
        let level = detector.determine_pressure_level(900.0, 100.0, 15.0, 200.0);
        assert_eq!(level, MemoryPressureLevel::Critical);
        
        // Test low pressure level
        let level = detector.determine_pressure_level(200.0, 800.0, 1.0, 10.0);
        assert_eq!(level, MemoryPressureLevel::Low);
    }

    #[test]
    fn test_gc_config_recommendations() {
        let config = MemoryPressureConfig::default();
        let mut detector = MemoryPressureDetector::new(config);
        
        // Test critical pressure GC config
        detector.optimization_adjustments = OptimizationAdjustments {
            reduce_inlining_threshold: true,
            enable_aggressive_dce: true,
            reduce_loop_unrolling: true,
            enable_size_optimization: true,
            reduce_template_instantiation: true,
            enable_memory_pool_optimization: true,
        };
        
        let gc_config = detector.get_recommended_gc_config();
        assert!(gc_config.heap_size_mb <= 128); // Should recommend smaller heap under pressure
    }
}
