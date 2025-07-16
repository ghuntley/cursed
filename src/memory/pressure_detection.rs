//! Advanced Memory Pressure Detection System for CURSED
//!
//! This module provides sophisticated memory pressure detection and response
//! mechanisms to prevent out-of-memory conditions and optimize performance.

use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread;

use crate::error::CursedError;
use crate::runtime::stack::RuntimeStack;

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureLevel {
    /// Normal memory usage
    Normal,
    /// Elevated memory usage
    Elevated,
    /// High memory usage
    High,
    /// Critical memory usage
    Critical,
    /// Emergency memory usage
    Emergency,
}

/// Memory pressure configuration
#[derive(Debug, Clone)]
pub struct PressureConfig {
    /// Normal threshold (percentage of available memory)
    pub normal_threshold: f64,
    /// Elevated threshold
    pub elevated_threshold: f64,
    /// High threshold
    pub high_threshold: f64,
    /// Critical threshold
    pub critical_threshold: f64,
    /// Emergency threshold
    pub emergency_threshold: f64,
    /// Check interval
    pub check_interval: Duration,
    /// History size for trend analysis
    pub history_size: usize,
    /// Enable predictive detection
    pub enable_prediction: bool,
    /// Prediction window
    pub prediction_window: Duration,
}

impl Default for PressureConfig {
    fn default() -> Self {
        Self {
            normal_threshold: 0.6,      // 60%
            elevated_threshold: 0.75,   // 75%
            high_threshold: 0.85,       // 85%
            critical_threshold: 0.95,   // 95%
            emergency_threshold: 0.98,  // 98%
            check_interval: Duration::from_millis(100),
            history_size: 1000,
            enable_prediction: true,
            prediction_window: Duration::from_secs(5),
        }
    }
}

/// Memory pressure statistics
#[derive(Debug)]
pub struct PressureStats {
    /// Current memory usage (bytes)
    pub current_usage: usize,
    /// Total available memory (bytes)
    pub total_available: usize,
    /// Current pressure level
    pub current_level: PressureLevel,
    /// Time at current level
    pub time_at_level: Duration,
    /// Pressure trend
    pub trend: PressureTrend,
    /// Predicted pressure level
    pub predicted_level: Option<PressureLevel>,
    /// Time until predicted level
    pub prediction_time: Option<Duration>,
    /// Response actions triggered
    pub responses_triggered: u64,
    /// Memory freed by responses
    pub memory_freed: usize,
}

/// Memory pressure trend
#[derive(Debug, Clone, Copy)]
pub enum PressureTrend {
    /// Pressure is decreasing
    Decreasing,
    /// Pressure is stable
    Stable,
    /// Pressure is increasing
    Increasing,
    /// Pressure is rapidly increasing
    RapidlyIncreasing,
}

/// Memory pressure response action
#[derive(Debug, Clone)]
pub enum PressureResponse {
    /// Trigger garbage collection
    TriggerGC,
    /// Increase GC frequency
    IncreaseGCFrequency,
    /// Reduce allocation rate
    ReduceAllocationRate,
    /// Free cached data
    FreeCachedData,
    /// Notify application
    NotifyApplication,
    /// Emergency cleanup
    EmergencyCleanup,
}

/// Memory pressure event
#[derive(Debug, Clone)]
pub struct PressureEvent {
    /// Event timestamp
    pub timestamp: Instant,
    /// Previous pressure level
    pub previous_level: PressureLevel,
    /// New pressure level
    pub new_level: PressureLevel,
    /// Memory usage at time of event
    pub memory_usage: usize,
    /// Responses triggered
    pub responses: Vec<PressureResponse>,
    /// Duration of pressure state
    pub duration: Duration,
}

/// Memory pressure history entry
#[derive(Debug, Clone)]
pub struct PressureHistoryEntry {
    /// Timestamp
    pub timestamp: Instant,
    /// Memory usage
    pub usage: usize,
    /// Pressure level
    pub level: PressureLevel,
    /// Allocation rate
    pub allocation_rate: f64,
    /// Free memory
    pub free_memory: usize,
}

/// Memory pressure detector
pub struct PressureDetector {
    /// Configuration
    config: RwLock<PressureConfig>,
    /// Current pressure stats
    stats: RwLock<PressureStats>,
    /// Pressure history
    history: RwLock<VecDeque<PressureHistoryEntry>>,
    /// Event history
    events: RwLock<VecDeque<PressureEvent>>,
    /// Response handlers
    handlers: RwLock<HashMap<PressureLevel, Vec<Box<dyn Fn(PressureLevel) + Send + Sync>>>>,
    /// Monitoring thread
    monitor_thread: RwLock<Option<thread::JoinHandle<()>>>,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Stack manager for memory info
    stack_manager: Arc<RuntimeStack>,
    /// Current memory usage
    current_usage: AtomicUsize,
    /// Last check time
    last_check: RwLock<Instant>,
    /// Pressure predictor
    predictor: Arc<PressurePredictor>,
}

/// Pressure predictor for proactive memory management
pub struct PressurePredictor {
    /// Historical data for prediction
    prediction_data: RwLock<VecDeque<PredictionDataPoint>>,
    /// Prediction model
    model: RwLock<PredictionModel>,
    /// Prediction accuracy tracker
    accuracy_tracker: RwLock<AccuracyTracker>,
}

/// Data point for pressure prediction
#[derive(Debug, Clone)]
pub struct PredictionDataPoint {
    /// Timestamp
    pub timestamp: Instant,
    /// Memory usage
    pub usage: usize,
    /// Allocation rate
    pub allocation_rate: f64,
    /// GC frequency
    pub gc_frequency: f64,
    /// System load
    pub system_load: f64,
    /// Pressure level
    pub pressure_level: PressureLevel,
}

/// Simple prediction model
#[derive(Debug, Clone)]
pub struct PredictionModel {
    /// Trend coefficient
    pub trend_coeff: f64,
    /// Allocation rate coefficient
    pub allocation_coeff: f64,
    /// GC frequency coefficient
    pub gc_coeff: f64,
    /// System load coefficient
    pub load_coeff: f64,
    /// Bias term
    pub bias: f64,
}

/// Accuracy tracking for predictions
#[derive(Debug, Clone)]
pub struct AccuracyTracker {
    /// Prediction history
    pub predictions: VecDeque<(PressureLevel, PressureLevel)>, // (predicted, actual)
    /// Current accuracy
    pub accuracy: f64,
    /// Accuracy history
    pub accuracy_history: VecDeque<f64>,
}

impl PressureDetector {
    /// Create new pressure detector
    pub fn new(config: PressureConfig, stack_manager: Arc<RuntimeStack>) -> Result<Arc<Self>, CursedError> {
        let detector = Arc::new(Self {
            config: RwLock::new(config),
            stats: RwLock::new(PressureStats::default()),
            history: RwLock::new(VecDeque::new()),
            events: RwLock::new(VecDeque::new()),
            handlers: RwLock::new(HashMap::new()),
            monitor_thread: RwLock::new(None),
            shutdown: AtomicBool::new(false),
            stack_manager,
            current_usage: AtomicUsize::new(0),
            last_check: RwLock::new(Instant::now()),
            predictor: Arc::new(PressurePredictor::new()),
        });

        // Start monitoring thread
        let detector_weak = Arc::downgrade(&detector);
        let monitor_handle = thread::spawn(move || {
            Self::monitor_loop(detector_weak);
        });

        *detector.monitor_thread.write().unwrap() = Some(monitor_handle);

        Ok(detector)
    }

    /// Main monitoring loop
    fn monitor_loop(detector_weak: std::sync::Weak<Self>) {
        while let Some(detector) = detector_weak.upgrade() {
            if detector.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Check memory pressure
            if let Err(e) = detector.check_pressure() {
                eprintln!("Pressure check error: {}", e);
            }

            // Sleep for check interval
            let config = detector.config.read().unwrap();
            let sleep_duration = config.check_interval;
            drop(config);

            thread::sleep(sleep_duration);
        }
    }

    /// Check current memory pressure
    fn check_pressure(&self) -> Result<(), CursedError> {
        let now = Instant::now();
        let last_check = *self.last_check.read().unwrap();

        // Get current memory usage
        let current_usage = self.get_current_memory_usage();
        let total_available = self.get_total_available_memory();

        // Calculate pressure level
        let pressure_level = self.calculate_pressure_level(current_usage, total_available);

        // Update statistics
        let previous_level = {
            let mut stats = self.stats.write().unwrap();
            let previous_level = stats.current_level;
            
            stats.current_usage = current_usage;
            stats.total_available = total_available;
            stats.current_level = pressure_level;
            
            if previous_level == pressure_level {
                stats.time_at_level += now.duration_since(last_check);
            } else {
                stats.time_at_level = Duration::from_secs(0);
            }

            // Calculate trend
            stats.trend = self.calculate_trend();

            previous_level
        };

        // Add to history
        self.add_to_history(PressureHistoryEntry {
            timestamp: now,
            usage: current_usage,
            level: pressure_level,
            allocation_rate: self.calculate_allocation_rate(),
            free_memory: total_available - current_usage,
        });

        // Update predictor
        self.update_predictor(current_usage, pressure_level);

        // Handle pressure level changes
        if previous_level != pressure_level {
            self.handle_pressure_change(previous_level, pressure_level, current_usage)?;
        }

        // Trigger responses based on pressure level
        self.trigger_responses(pressure_level);

        *self.last_check.write().unwrap() = now;

        Ok(())
    }

    /// Get current memory usage
    fn get_current_memory_usage(&self) -> usize {
        // In a real implementation, this would query the actual memory usage
        // For now, return the tracked usage
        self.current_usage.load(Ordering::Relaxed)
    }

    /// Get total available memory
    fn get_total_available_memory(&self) -> usize {
        // In a real implementation, this would query system memory
        // For now, return a reasonable default
        2 * 1024 * 1024 * 1024 // 2GB
    }

    /// Calculate pressure level based on usage
    fn calculate_pressure_level(&self, usage: usize, total: usize) -> PressureLevel {
        let ratio = usage as f64 / total as f64;
        let config = self.config.read().unwrap();

        if ratio >= config.emergency_threshold {
            PressureLevel::Emergency
        } else if ratio >= config.critical_threshold {
            PressureLevel::Critical
        } else if ratio >= config.high_threshold {
            PressureLevel::High
        } else if ratio >= config.elevated_threshold {
            PressureLevel::Elevated
        } else {
            PressureLevel::Normal
        }
    }

    /// Calculate memory pressure trend
    fn calculate_trend(&self) -> PressureTrend {
        let history = self.history.read().unwrap();
        if history.len() < 3 {
            return PressureTrend::Stable;
        }

        let recent_entries: Vec<_> = history.iter().rev().take(10).collect();
        let mut increasing = 0;
        let mut decreasing = 0;

        for i in 1..recent_entries.len() {
            if recent_entries[i-1].usage > recent_entries[i].usage {
                increasing += 1;
            } else if recent_entries[i-1].usage < recent_entries[i].usage {
                decreasing += 1;
            }
        }

        if increasing > decreasing * 2 {
            PressureTrend::RapidlyIncreasing
        } else if increasing > decreasing {
            PressureTrend::Increasing
        } else if decreasing > increasing {
            PressureTrend::Decreasing
        } else {
            PressureTrend::Stable
        }
    }

    /// Calculate allocation rate
    fn calculate_allocation_rate(&self) -> f64 {
        let history = self.history.read().unwrap();
        if history.len() < 2 {
            return 0.0;
        }

        let recent = history.back().unwrap();
        let previous = history.get(history.len() - 2).unwrap();

        let time_diff = recent.timestamp.duration_since(previous.timestamp).as_secs_f64();
        if time_diff > 0.0 {
            (recent.usage as f64 - previous.usage as f64) / time_diff
        } else {
            0.0
        }
    }

    /// Add entry to history
    fn add_to_history(&self, entry: PressureHistoryEntry) {
        let mut history = self.history.write().unwrap();
        history.push_back(entry);

        let config = self.config.read().unwrap();
        if history.len() > config.history_size {
            history.pop_front();
        }
    }

    /// Update pressure predictor
    fn update_predictor(&self, current_usage: usize, pressure_level: PressureLevel) {
        let data_point = PredictionDataPoint {
            timestamp: Instant::now(),
            usage: current_usage,
            allocation_rate: self.calculate_allocation_rate(),
            gc_frequency: self.calculate_gc_frequency(),
            system_load: self.get_system_load(),
            pressure_level,
        };

        self.predictor.update_with_data_point(data_point);
    }

    /// Calculate GC frequency based on recent collections
    fn calculate_gc_frequency(&self) -> f64 {
        let stats = self.stats.read().unwrap();
        // Use responses triggered as a proxy for GC frequency
        stats.responses_triggered as f64 / 60.0 // Approximate frequency per minute
    }

    /// Get current system load (simplified implementation)
    fn get_system_load(&self) -> f64 {
        let stats = self.stats.read().unwrap();
        // Calculate load based on memory usage
        if stats.total_available > 0 {
            stats.current_usage as f64 / stats.total_available as f64
        } else {
            0.5 // Default moderate load
        }
    }

    /// Handle pressure level changes
    fn handle_pressure_change(
        &self,
        previous_level: PressureLevel,
        new_level: PressureLevel,
        current_usage: usize,
    ) -> Result<(), CursedError> {
        let event = PressureEvent {
            timestamp: Instant::now(),
            previous_level,
            new_level,
            memory_usage: current_usage,
            responses: Vec::new(),
            duration: Duration::from_secs(0),
        };

        let mut events = self.events.write().unwrap();
        events.push_back(event);

        // Limit event history
        if events.len() > 1000 {
            events.drain(0..500);
        }

        Ok(())
    }

    /// Trigger responses based on pressure level
    fn trigger_responses(&self, pressure_level: PressureLevel) {
        let handlers = self.handlers.read().unwrap();
        
        if let Some(level_handlers) = handlers.get(&pressure_level) {
            for handler in level_handlers {
                handler(pressure_level);
            }
        }

        // Update response statistics
        let mut stats = self.stats.write().unwrap();
        stats.responses_triggered += 1;
    }

    /// Register pressure response handler
    pub fn register_handler<F>(&self, level: PressureLevel, handler: F)
    where
        F: Fn(PressureLevel) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.entry(level).or_insert_with(Vec::new).push(Box::new(handler));
    }

    /// Update memory usage
    pub fn update_memory_usage(&self, usage: usize) {
        self.current_usage.store(usage, Ordering::Relaxed);
    }

    /// Get current pressure statistics
    pub fn get_stats(&self) -> PressureStats {
        self.stats.read().unwrap().clone()
    }

    /// Get pressure history
    pub fn get_history(&self) -> Vec<PressureHistoryEntry> {
        self.history.read().unwrap().iter().cloned().collect()
    }

    /// Get pressure events
    pub fn get_events(&self) -> Vec<PressureEvent> {
        self.events.read().unwrap().iter().cloned().collect()
    }

    /// Get pressure prediction
    pub fn get_prediction(&self) -> Option<(PressureLevel, Duration)> {
        self.predictor.predict_pressure()
    }

    /// Shutdown the pressure detector
    pub fn shutdown(&self) -> Result<(), CursedError> {
        self.shutdown.store(true, Ordering::Relaxed);

        // Wait for monitoring thread to finish
        if let Some(handle) = self.monitor_thread.write().unwrap().take() {
            handle.join().map_err(|_| CursedError::runtime_error("Failed to join monitor thread"))?;
        }

        Ok(())
    }
}

impl PressureStats {
    /// Create default pressure stats
    pub fn default() -> Self {
        Self {
            current_usage: 0,
            total_available: 0,
            current_level: PressureLevel::Normal,
            time_at_level: Duration::from_secs(0),
            trend: PressureTrend::Stable,
            predicted_level: None,
            prediction_time: None,
            responses_triggered: 0,
            memory_freed: 0,
        }
    }
}

impl Clone for PressureStats {
    fn clone(&self) -> Self {
        Self {
            current_usage: self.current_usage,
            total_available: self.total_available,
            current_level: self.current_level,
            time_at_level: self.time_at_level,
            trend: self.trend,
            predicted_level: self.predicted_level,
            prediction_time: self.prediction_time,
            responses_triggered: self.responses_triggered,
            memory_freed: self.memory_freed,
        }
    }
}

impl PressurePredictor {
    /// Create new pressure predictor
    pub fn new() -> Self {
        Self {
            prediction_data: RwLock::new(VecDeque::new()),
            model: RwLock::new(PredictionModel::default()),
            accuracy_tracker: RwLock::new(AccuracyTracker::default()),
        }
    }

    /// Update with new data point
    pub fn update_with_data_point(&self, data_point: PredictionDataPoint) {
        let mut data = self.prediction_data.write().unwrap();
        data.push_back(data_point);

        // Limit data size
        if data.len() > 10000 {
            data.drain(0..5000);
        }
    }

    /// Predict future pressure level
    pub fn predict_pressure(&self) -> Option<(PressureLevel, Duration)> {
        let data = self.prediction_data.read().unwrap();
        if data.len() < 10 {
            return None;
        }

        let model = self.model.read().unwrap();
        let recent_data: Vec<_> = data.iter().rev().take(10).collect();

        // Simple trend-based prediction
        let trend = self.calculate_trend(&recent_data);
        let current_level = recent_data[0].pressure_level;

        let predicted_level = match (current_level, trend) {
            (PressureLevel::Normal, trend) if trend > 0.5 => PressureLevel::Elevated,
            (PressureLevel::Elevated, trend) if trend > 0.7 => PressureLevel::High,
            (PressureLevel::High, trend) if trend > 0.8 => PressureLevel::Critical,
            (PressureLevel::Critical, trend) if trend > 0.9 => PressureLevel::Emergency,
            _ => current_level,
        };

        let prediction_time = if predicted_level != current_level {
            Duration::from_secs(5) // Predict 5 seconds ahead
        } else {
            Duration::from_secs(0)
        };

        Some((predicted_level, prediction_time))
    }

    /// Calculate trend from recent data
    fn calculate_trend(&self, data: &[&PredictionDataPoint]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mut trend_sum = 0.0;
        for i in 1..data.len() {
            let usage_diff = data[i-1].usage as f64 - data[i].usage as f64;
            trend_sum += usage_diff;
        }

        trend_sum / (data.len() - 1) as f64
    }
}

impl PredictionModel {
    /// Create default prediction model
    pub fn default() -> Self {
        Self {
            trend_coeff: 0.3,
            allocation_coeff: 0.2,
            gc_coeff: -0.1,
            load_coeff: 0.1,
            bias: 0.0,
        }
    }
}

impl AccuracyTracker {
    /// Create default accuracy tracker
    pub fn default() -> Self {
        Self {
            predictions: VecDeque::new(),
            accuracy: 0.0,
            accuracy_history: VecDeque::new(),
        }
    }
}

/// Convenience function to create pressure detector
pub fn create_pressure_detector(
    config: PressureConfig,
    stack_manager: Arc<RuntimeStack>,
) -> Result<Arc<PressureDetector>, CursedError> {
    PressureDetector::new(config, stack_manager)
}

/// Legacy compatibility
pub type MinimalImplementation = PressureDetector;

/// Get minimal result for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED memory pressure detection system active".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::stack::RuntimeStack;

    #[test]
    fn test_pressure_detector_creation() {
        let stack = Arc::new(RuntimeStack::new());
        let detector = PressureDetector::new(PressureConfig::default(), stack);
        assert!(detector.is_ok());
    }

    #[test]
    fn test_pressure_level_calculation() {
        let stack = Arc::new(RuntimeStack::new());
        let detector = PressureDetector::new(PressureConfig::default(), stack).unwrap();
        
        let level = detector.calculate_pressure_level(800, 1000);
        assert_eq!(level, PressureLevel::Elevated);
    }

    #[test]
    fn test_pressure_predictor() {
        let predictor = PressurePredictor::new();
        let data_point = PredictionDataPoint {
            timestamp: Instant::now(),
            usage: 1000,
            allocation_rate: 100.0,
            gc_frequency: 0.1,
            system_load: 0.5,
            pressure_level: PressureLevel::Normal,
        };

        predictor.update_with_data_point(data_point);
        // Prediction should be None with insufficient data
        assert!(predictor.predict_pressure().is_none());
    }
}
