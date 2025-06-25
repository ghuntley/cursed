/// Adaptive Garbage Collection System for CURSED
/// 
/// This module implements sophisticated adaptive garbage collection that monitors
/// memory pressure, allocation patterns, and system performance to automatically
/// adjust collection strategies for optimal performance and memory management.

use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::collections::{HashMap, VecDeque};
use tracing::{instrument, debug, info, warn, error};


use crate::memory::{
    gc::{GarbageCollector, CollectionAlgorithm, CollectionTrigger, GcConfig},
    pressure_detection::{MemoryPressureDetector, PressureLevel, PressureDetectionConfig},
    collection_triggers::{CollectionTriggerManager, TriggerType, TriggerConfig},
    heap_manager::{HeapStats, MemoryPressure, AllocationMetrics},
};
use crate::error_types::Error;

/// Adaptive collection strategy based on application behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdaptiveStrategy {
    /// Conservative strategy - low collection frequency, larger pause times
    Conservative,
    /// Balanced strategy - moderate collection frequency and pause times
    Balanced,
    /// Aggressive strategy - high collection frequency, shorter pause times
    Aggressive,
    /// Latency-sensitive strategy - minimizes pause times at all costs
    LatencySensitive,
    /// Throughput-optimized strategy - maximizes application throughput
    ThroughputOptimized,
    /// Memory-constrained strategy - optimizes for low memory usage
    MemoryConstrained,
}

/// Application behavior pattern detected by the adaptive system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorPattern {
    /// Steady allocation rate with consistent object lifetimes
    Steady,
    /// Burst allocation with quick deallocation (e.g., web requests)
    Bursty,
    /// Batch processing with large temporary allocations
    Batch,
    /// Long-lived objects with occasional cleanup
    Accumulative,
    /// Mixed pattern that doesn't fit other categories
    Mixed,
}

/// Memory threshold management configuration
#[derive(Debug, Clone)]
pub struct AdaptiveThresholds {
    /// Young generation collection threshold (adaptive)
    pub young_threshold: f64,
    /// Old generation collection threshold (adaptive)
    pub old_threshold: f64,
    /// Emergency collection threshold (adaptive)
    pub emergency_threshold: f64,
    /// Minimum threshold values (safety bounds)
    pub min_thresholds: ThresholdBounds,
    /// Maximum threshold values (safety bounds)
    pub max_thresholds: ThresholdBounds,
    /// Threshold adjustment factors
    pub adjustment_factors: AdjustmentFactors,
}

#[derive(Debug, Clone)]
pub struct ThresholdBounds {
    pub young_min: f64,
    pub young_max: f64,
    pub old_min: f64,
    pub old_max: f64,
    pub emergency_min: f64,
    pub emergency_max: f64,
}

#[derive(Debug, Clone)]
pub struct AdjustmentFactors {
    /// Factor for increasing thresholds when performance is good
    pub increase_factor: f64,
    /// Factor for decreasing thresholds when memory pressure is high
    pub decrease_factor: f64,
    /// Maximum adjustment per update cycle
    pub max_adjustment_per_cycle: f64,
    /// Smoothing factor for exponential moving averages
    pub smoothing_factor: f64,
}

/// Performance metrics tracking for adaptive decisions
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Average allocation rate (bytes per second)
    pub allocation_rate: f64,
    /// Average collection pause time
    pub average_pause_time: Duration,
    /// Collection frequency (collections per minute)
    pub collection_frequency: f64,
    /// Memory utilization efficiency (0.0 to 1.0)
    pub memory_efficiency: f64,
    /// Throughput impact of GC (percentage)
    pub throughput_impact: f64,
    /// Memory pressure trend
    pub pressure_trend: f64,
}

/// Allocation pattern analysis for behavior detection
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    /// Recent allocation sizes
    allocation_sizes: VecDeque<usize>,
    /// Recent allocation intervals
    allocation_intervals: VecDeque<Duration>,
    /// Object lifetime distribution
    lifetime_distribution: HashMap<u64, u64>, // lifetime_ms -> count
    /// Peak memory usage samples
    peak_usage_samples: VecDeque<usize>,
    /// Last pattern analysis time
    last_analysis: Instant,
}

impl AllocationPattern {
    fn new() -> Self {
        Self {
            allocation_sizes: VecDeque::with_capacity(1000),
            allocation_intervals: VecDeque::with_capacity(1000),
            lifetime_distribution: HashMap::new(),
            peak_usage_samples: VecDeque::with_capacity(100),
            last_analysis: Instant::now(),
        }
    }

    fn record_allocation(&mut self, size: usize) {
        let now = Instant::now();
        let interval = now.duration_since(self.last_analysis);
        
        self.allocation_sizes.push_back(size);
        self.allocation_intervals.push_back(interval);
        
        // Keep only recent samples
        if self.allocation_sizes.len() > 1000 {
            self.allocation_sizes.pop_front();
        }
        if self.allocation_intervals.len() > 1000 {
            self.allocation_intervals.pop_front();
        }
        
        self.last_analysis = now;
    }

    fn record_peak_usage(&mut self, usage: usize) {
        self.peak_usage_samples.push_back(usage);
        if self.peak_usage_samples.len() > 100 {
            self.peak_usage_samples.pop_front();
        }
    }

    fn analyze_behavior(&self) -> BehaviorPattern {
        if self.allocation_sizes.len() < 50 {
            return BehaviorPattern::Mixed;
        }

        // Calculate allocation rate variance
        let avg_interval: f64 = self.allocation_intervals.iter()
            .map(|d| d.as_secs_f64())
            .sum::<f64>() / self.allocation_intervals.len() as f64;
        
        let interval_variance: f64 = self.allocation_intervals.iter()
            .map(|d| {
                let diff = d.as_secs_f64() - avg_interval;
                diff * diff
            })
            .sum::<f64>() / self.allocation_intervals.len() as f64;

        // Calculate size variance
        let avg_size: f64 = self.allocation_sizes.iter()
            .map(|&s| s as f64)
            .sum::<f64>() / self.allocation_sizes.len() as f64;
        
        let size_variance: f64 = self.allocation_sizes.iter()
            .map(|&s| {
                let diff = s as f64 - avg_size;
                diff * diff
            })
            .sum::<f64>() / self.allocation_sizes.len() as f64;

        // Determine pattern based on variance characteristics
        match (interval_variance, size_variance) {
            (iv, sv) if iv < avg_interval * avg_interval * 0.1 && sv < avg_size * avg_size * 0.1 => {
                BehaviorPattern::Steady
            },
            (iv, _) if iv > avg_interval * avg_interval * 2.0 => {
                BehaviorPattern::Bursty
            },
            (_, sv) if sv > avg_size * avg_size * 5.0 => {
                BehaviorPattern::Batch
            },
            _ => {
                // Check for accumulative pattern by looking at peak usage trend
                if self.peak_usage_samples.len() >= 10 {
                    let recent_peaks: f64 = self.peak_usage_samples.iter()
                        .rev().take(5).map(|&u| u as f64).sum::<f64>() / 5.0;
                    let older_peaks: f64 = self.peak_usage_samples.iter()
                        .rev().skip(5).take(5).map(|&u| u as f64).sum::<f64>() / 5.0;
                    
                    if recent_peaks > older_peaks * 1.5 {
                        BehaviorPattern::Accumulative
                    } else {
                        BehaviorPattern::Mixed
                    }
                } else {
                    BehaviorPattern::Mixed
                }
            }
        }
    }
}

/// Configuration for adaptive garbage collection
#[derive(Debug, Clone)]
pub struct AdaptiveGcConfig {
    /// Base GC configuration
    pub base_gc_config: GcConfig,
    /// Pressure detection configuration
    pub pressure_config: PressureDetectionConfig,
    /// Trigger configuration
    pub trigger_config: TriggerConfig,
    /// Adaptive thresholds
    pub adaptive_thresholds: AdaptiveThresholds,
    /// Target performance metrics
    pub target_metrics: TargetMetrics,
    /// Adaptation parameters
    pub adaptation_params: AdaptationParameters,
}

#[derive(Debug, Clone)]
pub struct TargetMetrics {
    /// Target maximum pause time
    pub max_pause_time: Duration,
    /// Target memory utilization
    pub target_utilization: f64,
    /// Target collection frequency (collections per minute)
    pub target_collection_frequency: f64,
    /// Acceptable throughput impact (percentage)
    pub max_throughput_impact: f64,
}

#[derive(Debug, Clone)]
pub struct AdaptationParameters {
    /// How quickly to adapt to changes (0.0 to 1.0)
    pub adaptation_speed: f64,
    /// Minimum samples needed before adaptation
    pub min_samples_for_adaptation: usize,
    /// Pattern analysis window (seconds)
    pub pattern_analysis_window: u64,
    /// Performance evaluation interval
    pub evaluation_interval: Duration,
    /// Enable automatic strategy switching
    pub auto_strategy_switching: bool,
    /// Strategy evaluation threshold
    pub strategy_switch_threshold: f64,
}

impl Default for AdaptiveGcConfig {
    fn default() -> Self {
        Self {
            base_gc_config: GcConfig::default(),
            pressure_config: PressureDetectionConfig::default(),
            trigger_config: TriggerConfig::default(),
            adaptive_thresholds: AdaptiveThresholds {
                young_threshold: 0.75,
                old_threshold: 0.85,
                emergency_threshold: 0.95,
                min_thresholds: ThresholdBounds {
                    young_min: 0.50,
                    young_max: 0.90,
                    old_min: 0.70,
                    old_max: 0.95,
                    emergency_min: 0.90,
                    emergency_max: 0.99,
                },
                max_thresholds: ThresholdBounds {
                    young_min: 0.50,
                    young_max: 0.90,
                    old_min: 0.70,
                    old_max: 0.95,
                    emergency_min: 0.90,
                    emergency_max: 0.99,
                },
                adjustment_factors: AdjustmentFactors {
                    increase_factor: 1.05,
                    decrease_factor: 0.95,
                    max_adjustment_per_cycle: 0.05,
                    smoothing_factor: 0.1,
                },
            },
            target_metrics: TargetMetrics {
                max_pause_time: Duration::from_millis(10),
                target_utilization: 0.80,
                target_collection_frequency: 6.0, // 6 collections per minute
                max_throughput_impact: 5.0, // 5% maximum impact
            },
            adaptation_params: AdaptationParameters {
                adaptation_speed: 0.1,
                min_samples_for_adaptation: 10,
                pattern_analysis_window: 60,
                evaluation_interval: Duration::from_secs(30),
                auto_strategy_switching: true,
                strategy_switch_threshold: 0.15, // 15% performance difference
            },
        }
    }
}

/// Adaptive garbage collection manager
pub struct AdaptiveGarbageCollector {
    /// Base garbage collector
    gc: Arc<GarbageCollector>,
    /// Memory pressure detector
    pressure_detector: Arc<MemoryPressureDetector>,
    /// Collection trigger manager
    trigger_manager: Arc<CollectionTriggerManager>,
    /// Configuration
    config: RwLock<AdaptiveGcConfig>,
    /// Current strategy
    current_strategy: RwLock<AdaptiveStrategy>,
    /// Current behavior pattern
    current_pattern: RwLock<BehaviorPattern>,
    /// Performance metrics
    performance_metrics: RwLock<PerformanceMetrics>,
    /// Allocation pattern analyzer
    allocation_pattern: Mutex<AllocationPattern>,
    /// Adaptive thresholds
    adaptive_thresholds: RwLock<AdaptiveThresholds>,
    /// Collection statistics
    collection_count: AtomicU64,
    bytes_allocated_since_last_gc: AtomicU64,
    objects_allocated_since_last_gc: AtomicU64,
    last_collection_time: Mutex<Option<Instant>>,
    last_evaluation_time: Mutex<Instant>,
    /// Strategy performance tracking
    strategy_performance: RwLock<HashMap<AdaptiveStrategy, PerformanceMetrics>>,
    /// Adaptation active flag
    adaptation_active: AtomicBool,
}

impl AdaptiveGarbageCollector {
    /// Create a new adaptive garbage collector
    #[instrument]
    pub fn new(config: AdaptiveGcConfig) -> Result<Self, String> {
        info!("Creating adaptive garbage collector with config: {:?}", config);
        
        let gc = Arc::new(GarbageCollector::with_config(
            config.base_gc_config.clone(),
            crate::memory::heap_manager::HeapConfig::default()
        ));
        
        let pressure_detector = Arc::new(MemoryPressureDetector::new(config.pressure_config.clone()));
        let trigger_manager = Arc::new(CollectionTriggerManager::with_config(config.trigger_config.clone()));
        
        Ok(Self {
            gc,
            pressure_detector,
            trigger_manager,
            config: RwLock::new(config.clone()),
            current_strategy: RwLock::new(AdaptiveStrategy::Balanced),
            current_pattern: RwLock::new(BehaviorPattern::Mixed),
            performance_metrics: RwLock::new(PerformanceMetrics::default()),
            allocation_pattern: Mutex::new(AllocationPattern::new()),
            adaptive_thresholds: RwLock::new(config.adaptive_thresholds),
            collection_count: AtomicU64::new(0),
            bytes_allocated_since_last_gc: AtomicU64::new(0),
            objects_allocated_since_last_gc: AtomicU64::new(0),
            last_collection_time: Mutex::new(None),
            last_evaluation_time: Mutex::new(Instant::now()),
            strategy_performance: RwLock::new(HashMap::new()),
            adaptation_active: AtomicBool::new(true),
        })
    }

    /// Create with default configuration
    pub fn with_default_config() -> Result<Self, String> {
        Self::new(AdaptiveGcConfig::default())
    }

    /// Allocate an object with adaptive tracking
    #[instrument(skip(self, obj))]
    pub fn allocate<T>(&self, obj: T) -> Result<crate::memory::gc::Gc<T>, String>
    where
        T: crate::memory::object_store::Storable,
    {
        let size = std::mem::size_of::<T>();
        
        // Record allocation pattern
        {
            let mut pattern = self.allocation_pattern.lock()
                .map_err(|_| "Failed to acquire allocation pattern lock")?;
            pattern.record_allocation(size);
        }
        
        // Update allocation tracking
        self.bytes_allocated_since_last_gc.fetch_add(size as u64, Ordering::Relaxed);
        self.objects_allocated_since_last_gc.fetch_add(1, Ordering::Relaxed);
        
        // Notify allocation to trigger manager
        self.trigger_manager.update_allocation_tracking(size, 1)?;
        
        // Check if collection should be triggered
        self.check_and_trigger_collection()?;
        
        // Allocate through base GC
        self.gc.allocate(obj)
    }

    /// Check if collection should be triggered and trigger if necessary
    fn check_and_trigger_collection(&self) -> Result<(), String> {
        // Get current heap stats
        let heap_stats = self.get_heap_stats()?;
        
        // Check memory pressure
        let pressure_level = self.pressure_detector.detect_pressure(&heap_stats, None)?;
        
        // Determine if collection should be triggered
        let should_collect = match pressure_level {
            PressureLevel::Emergency => true,
            PressureLevel::Critical => true,
            PressureLevel::High => true,
            PressureLevel::Moderate => {
                // Check allocation-based triggers
                self.trigger_manager.should_trigger_collection(&heap_stats)?
                    .map(|(trigger_type, _)| {
                        matches!(trigger_type, TriggerType::YoungGeneration | TriggerType::OldGeneration)
                    })
                    .unwrap_or(false)
            },
            _ => {
                // Check standard triggers
                self.trigger_manager.should_trigger_collection(&heap_stats)?
                    .is_some()
            }
        };

        if should_collect {
            let trigger = match pressure_level {
                PressureLevel::Emergency => CollectionTrigger::Emergency,
                PressureLevel::Critical | PressureLevel::High => CollectionTrigger::HeapUtilization,
                _ => CollectionTrigger::AllocationPressure,
            };
            
            self.perform_adaptive_collection(trigger)?;
        }

        Ok(())
    }

    /// Perform collection with adaptive algorithm selection
    fn perform_adaptive_collection(&self, trigger: CollectionTrigger) -> Result<(), String> {
        let start_time = Instant::now();
        
        // Select optimal collection algorithm based on current strategy and pattern
        let algorithm = self.select_optimal_algorithm(trigger)?;
        
        // Update GC configuration for this collection
        self.update_gc_configuration_for_collection(algorithm)?;
        
        // Perform collection
        let collection_stats = self.gc.collect_with_trigger(trigger)?;
        
        let collection_duration = start_time.elapsed();
        
        // Update performance metrics
        self.update_performance_metrics(collection_duration, &collection_stats)?;
        
        // Record collection statistics
        self.collection_count.fetch_add(1, Ordering::Relaxed);
        self.bytes_allocated_since_last_gc.store(0, Ordering::Relaxed);
        self.objects_allocated_since_last_gc.store(0, Ordering::Relaxed);
        
        {
            let mut last_time = self.last_collection_time.lock()
                .map_err(|_| "Failed to acquire last collection time lock")?;
            *last_time = Some(Instant::now());
        }

        // Evaluate and adapt if necessary
        self.evaluate_and_adapt()?;

        info!(
            algorithm = ?algorithm,
            trigger = ?trigger,
            duration_ms = collection_duration.as_millis(),
            objects_collected = collection_stats.objects_collected,
            bytes_collected = collection_stats.bytes_collected,
            "Adaptive collection completed"
        );

        Ok(())
    }

    /// Select optimal collection algorithm based on current state
    fn select_optimal_algorithm(&self, trigger: CollectionTrigger) -> Result<CollectionAlgorithm, String> {
        let strategy = *self.current_strategy.read()
            .map_err(|_| "Failed to read current strategy")?;
        let pattern = *self.current_pattern.read()
            .map_err(|_| "Failed to read current pattern")?;

        let algorithm = match (strategy, pattern, trigger) {
            // Emergency situations - always use fastest algorithm
            (_, _, CollectionTrigger::Emergency) => CollectionAlgorithm::Copying,
            
            // Latency-sensitive strategy
            (AdaptiveStrategy::LatencySensitive, _, _) => CollectionAlgorithm::Incremental,
            
            // Throughput-optimized strategy
            (AdaptiveStrategy::ThroughputOptimized, BehaviorPattern::Steady, _) => {
                CollectionAlgorithm::MarkSweep
            },
            (AdaptiveStrategy::ThroughputOptimized, BehaviorPattern::Bursty, _) => {
                CollectionAlgorithm::Copying
            },
            (AdaptiveStrategy::ThroughputOptimized, _, _) => {
                CollectionAlgorithm::MarkSweep
            },
            
            // Memory-constrained strategy
            (AdaptiveStrategy::MemoryConstrained, _, _) => CollectionAlgorithm::MarkSweep,
            
            // Aggressive strategy - frequent but fast collection
            (AdaptiveStrategy::Aggressive, BehaviorPattern::Bursty, _) => {
                CollectionAlgorithm::Copying
            },
            (AdaptiveStrategy::Aggressive, _, _) => CollectionAlgorithm::Incremental,
            
            // Conservative strategy - infrequent but thorough collection
            (AdaptiveStrategy::Conservative, _, _) => CollectionAlgorithm::MarkSweep,
            
            // Balanced strategy - adaptive selection
            (AdaptiveStrategy::Balanced, pattern, trigger) => {
                match (pattern, trigger) {
                    (BehaviorPattern::Bursty, _) => CollectionAlgorithm::Copying,
                    (BehaviorPattern::Batch, _) => CollectionAlgorithm::MarkSweep,
                    (BehaviorPattern::Accumulative, _) => CollectionAlgorithm::MarkSweep,
                    (_, CollectionTrigger::AllocationPressure) => CollectionAlgorithm::Copying,
                    (_, CollectionTrigger::HeapUtilization) => CollectionAlgorithm::MarkSweep,
                    _ => CollectionAlgorithm::Incremental,
                }
            },
        };

        debug!(
            strategy = ?strategy,
            pattern = ?pattern,
            trigger = ?trigger,
            selected_algorithm = ?algorithm,
            "Selected collection algorithm"
        );

        Ok(algorithm)
    }

    /// Update GC configuration for the selected algorithm
    fn update_gc_configuration_for_collection(&self, algorithm: CollectionAlgorithm) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to read config")?;
        
        let adaptive_thresholds = self.adaptive_thresholds.read()
            .map_err(|_| "Failed to read adaptive thresholds")?;

        let mut gc_config = config.base_gc_config.clone();
        
        // Update algorithm
        gc_config.algorithm = algorithm;
        
        // Update thresholds based on adaptive values
        gc_config.young_gen_threshold = adaptive_thresholds.young_threshold;
        gc_config.old_gen_threshold = adaptive_thresholds.old_threshold;
        gc_config.emergency_threshold = adaptive_thresholds.emergency_threshold;
        
        // Update other parameters based on strategy
        let strategy = *self.current_strategy.read()
            .map_err(|_| "Failed to read current strategy")?;
        
        match strategy {
            AdaptiveStrategy::LatencySensitive => {
                gc_config.max_pause_time = Duration::from_millis(5);
                gc_config.incremental = true;
                gc_config.concurrent = true;
            },
            AdaptiveStrategy::ThroughputOptimized => {
                gc_config.max_pause_time = Duration::from_millis(50);
                gc_config.incremental = false;
                gc_config.concurrent = false;
            },
            AdaptiveStrategy::MemoryConstrained => {
                gc_config.generational = true;
                gc_config.incremental = true;
            },
            AdaptiveStrategy::Aggressive => {
                gc_config.allocation_pressure_ratio = 0.05; // Trigger more frequently
            },
            AdaptiveStrategy::Conservative => {
                gc_config.allocation_pressure_ratio = 0.20; // Trigger less frequently
            },
            AdaptiveStrategy::Balanced => {
                // Use default values
            },
        }

        self.gc.update_config(gc_config)?;
        Ok(())
    }

    /// Update performance metrics after collection
    fn update_performance_metrics(
        &self, 
        collection_duration: Duration, 
        collection_stats: &crate::memory::gc::EnhancedCollectionStats
    ) -> Result<(), String> {
        let mut metrics = self.performance_metrics.write()
            .map_err(|_| "Failed to write performance metrics")?;

        // Update pause time (exponential moving average)
        let alpha = 0.1; // Smoothing factor
        if metrics.average_pause_time.is_zero() {
            metrics.average_pause_time = collection_duration;
        } else {
            let current_avg = metrics.average_pause_time.as_secs_f64();
            let new_duration = collection_duration.as_secs_f64();
            let new_avg = current_avg * (1.0 - alpha) + new_duration * alpha;
            metrics.average_pause_time = Duration::from_secs_f64(new_avg);
        }

        // Update allocation rate
        let bytes_allocated = self.bytes_allocated_since_last_gc.load(Ordering::Relaxed) as f64;
        let time_since_last = if let Some(last_time) = self.last_collection_time.lock()
            .map_err(|_| "Failed to acquire last collection time lock")?.as_ref() {
            Instant::now().duration_since(*last_time).as_secs_f64()
        } else {
            1.0 // Default to 1 second for first collection
        };
        
        if time_since_last > 0.0 {
            let current_rate = bytes_allocated / time_since_last;
            metrics.allocation_rate = metrics.allocation_rate * (1.0 - alpha) + current_rate * alpha;
        }

        // Update collection frequency
        let collection_count = self.collection_count.load(Ordering::Relaxed);
        if collection_count > 0 {
            // Collections per minute
            let uptime_minutes = Instant::now().duration_since(
                *self.last_evaluation_time.lock()
                    .map_err(|_| "Failed to acquire last evaluation time lock")?
            ).as_secs_f64() / 60.0;
            
            if uptime_minutes > 0.0 {
                metrics.collection_frequency = collection_count as f64 / uptime_minutes;
            }
        }

        // Update memory efficiency (bytes reclaimed / bytes allocated)
        if bytes_allocated > 0.0 {
            let efficiency = collection_stats.bytes_collected as f64 / bytes_allocated;
            metrics.memory_efficiency = metrics.memory_efficiency * (1.0 - alpha) + efficiency * alpha;
        }

        // Estimate throughput impact (collection time / total time)
        let throughput_impact = collection_duration.as_secs_f64() / time_since_last * 100.0;
        metrics.throughput_impact = metrics.throughput_impact * (1.0 - alpha) + throughput_impact * alpha;

        debug!(
            average_pause_time_ms = metrics.average_pause_time.as_millis(),
            allocation_rate_mb_s = metrics.allocation_rate / (1024.0 * 1024.0),
            collection_frequency = metrics.collection_frequency,
            memory_efficiency = metrics.memory_efficiency,
            throughput_impact = metrics.throughput_impact,
            "Updated performance metrics"
        );

        Ok(())
    }

    /// Evaluate current performance and adapt strategy if necessary
    fn evaluate_and_adapt(&self) -> Result<(), String> {
        if !self.adaptation_active.load(Ordering::Acquire) {
            return Ok(());
        }

        let config = self.config.read()
            .map_err(|_| "Failed to read config")?;
        
        let mut last_eval = self.last_evaluation_time.lock()
            .map_err(|_| "Failed to acquire last evaluation time lock")?;
        
        let now = Instant::now();
        if now.duration_since(*last_eval) < config.adaptation_params.evaluation_interval {
            return Ok(());
        }
        *last_eval = now;
        drop(last_eval);

        // Analyze allocation patterns
        self.analyze_allocation_patterns()?;
        
        // Adapt thresholds based on performance
        self.adapt_thresholds()?;
        
        // Evaluate strategy performance
        self.evaluate_strategy_performance()?;
        
        // Switch strategy if beneficial
        if config.adaptation_params.auto_strategy_switching {
            self.consider_strategy_switch()?;
        }

        Ok(())
    }

    /// Analyze allocation patterns and update behavior pattern
    fn analyze_allocation_patterns(&self) -> Result<(), String> {
        let pattern = {
            let allocation_pattern = self.allocation_pattern.lock()
                .map_err(|_| "Failed to acquire allocation pattern lock")?;
            allocation_pattern.analyze_behavior()
        };

        let mut current_pattern = self.current_pattern.write()
            .map_err(|_| "Failed to write current pattern")?;
        
        if *current_pattern != pattern {
            info!(
                old_pattern = ?*current_pattern,
                new_pattern = ?pattern,
                "Detected behavior pattern change"
            );
            *current_pattern = pattern;
        }

        Ok(())
    }

    /// Adapt collection thresholds based on performance metrics
    fn adapt_thresholds(&self) -> Result<(), String> {
        let metrics = self.performance_metrics.read()
            .map_err(|_| "Failed to read performance metrics")?;
        
        let config = self.config.read()
            .map_err(|_| "Failed to read config")?;
        
        let mut thresholds = self.adaptive_thresholds.write()
            .map_err(|_| "Failed to write adaptive thresholds")?;

        let target_metrics = &config.target_metrics;
        
        // Extract adjustment factors to avoid borrowing conflicts
        let increase_factor = thresholds.adjustment_factors.increase_factor;
        let decrease_factor = thresholds.adjustment_factors.decrease_factor;
        let max_adjustment = thresholds.adjustment_factors.max_adjustment_per_cycle;
        
        // Extract bounds to avoid borrowing conflicts
        let young_min = thresholds.min_thresholds.young_min;
        let young_max = thresholds.max_thresholds.young_max;
        let old_min = thresholds.min_thresholds.old_min;
        let old_max = thresholds.max_thresholds.old_max;
        let emergency_max = thresholds.max_thresholds.emergency_max;

        // Adapt young generation threshold
        if metrics.collection_frequency > target_metrics.target_collection_frequency * 1.2 {
            // Too frequent collections - increase threshold
            let adjustment = increase_factor.min(1.0 + max_adjustment);
            thresholds.young_threshold = (thresholds.young_threshold * adjustment).min(young_max);
        } else if metrics.collection_frequency < target_metrics.target_collection_frequency * 0.8 {
            // Too infrequent collections - decrease threshold  
            let adjustment = decrease_factor.max(1.0 - max_adjustment);
            thresholds.young_threshold = (thresholds.young_threshold * adjustment).max(young_min);
        }

        // Adapt old generation threshold based on memory efficiency
        if metrics.memory_efficiency < 0.5 {
            // Poor memory efficiency - decrease threshold for more frequent collection
            thresholds.old_threshold = (thresholds.old_threshold * decrease_factor).max(old_min);
        } else if metrics.memory_efficiency > 0.8 {
            // Good memory efficiency - can afford higher threshold
            thresholds.old_threshold = (thresholds.old_threshold * increase_factor).min(old_max);
        }

        // Adapt emergency threshold based on throughput impact
        if metrics.throughput_impact > target_metrics.max_throughput_impact {
            // High throughput impact - increase emergency threshold to reduce emergency collections
            thresholds.emergency_threshold = (thresholds.emergency_threshold * increase_factor).min(emergency_max);
        }

        debug!(
            young_threshold = thresholds.young_threshold,
            old_threshold = thresholds.old_threshold,
            emergency_threshold = thresholds.emergency_threshold,
            "Adapted collection thresholds"
        );

        Ok(())
    }

    /// Evaluate current strategy performance
    fn evaluate_strategy_performance(&self) -> Result<(), String> {
        let current_strategy = *self.current_strategy.read()
            .map_err(|_| "Failed to read current strategy")?;
        
        let current_metrics = self.performance_metrics.read()
            .map_err(|_| "Failed to read performance metrics")?
            .clone();

        let mut strategy_performance = self.strategy_performance.write()
            .map_err(|_| "Failed to write strategy performance")?;

        strategy_performance.insert(current_strategy, current_metrics);

        Ok(())
    }

    /// Consider switching to a better strategy
    fn consider_strategy_switch(&self) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to read config")?;
        
        let current_strategy = *self.current_strategy.read()
            .map_err(|_| "Failed to read current strategy")?;
        
        let strategy_performance = self.strategy_performance.read()
            .map_err(|_| "Failed to read strategy performance")?;

        if strategy_performance.len() < 2 {
            return Ok(()); // Need at least 2 strategies to compare
        }

        // Calculate performance score for each strategy
        let mut best_strategy = current_strategy;
        let mut best_score = self.calculate_performance_score(
            strategy_performance.get(&current_strategy).unwrap(),
            &config.target_metrics
        );

        for (&strategy, metrics) in strategy_performance.iter() {
            if strategy != current_strategy {
                let score = self.calculate_performance_score(metrics, &config.target_metrics);
                if score > best_score + config.adaptation_params.strategy_switch_threshold {
                    best_score = score;
                    best_strategy = strategy;
                }
            }
        }

        if best_strategy != current_strategy {
            info!(
                old_strategy = ?current_strategy,
                new_strategy = ?best_strategy,
                performance_improvement = best_score - self.calculate_performance_score(
                    strategy_performance.get(&current_strategy).unwrap(),
                    &config.target_metrics
                ),
                "Switching adaptive strategy"
            );

            let mut current = self.current_strategy.write()
                .map_err(|_| "Failed to write current strategy")?;
            *current = best_strategy;
        }

        Ok(())
    }

    /// Calculate a performance score for a set of metrics
    fn calculate_performance_score(&self, metrics: &PerformanceMetrics, targets: &TargetMetrics) -> f64 {
        let mut score = 0.0;

        // Pause time score (lower is better)
        let pause_ratio = targets.max_pause_time.as_secs_f64() / metrics.average_pause_time.as_secs_f64();
        score += pause_ratio.min(2.0) * 0.3; // Max 2.0 score, weight 0.3

        // Memory efficiency score (higher is better)
        score += metrics.memory_efficiency * 0.25; // Weight 0.25

        // Collection frequency score (closer to target is better)
        let freq_ratio = if metrics.collection_frequency > targets.target_collection_frequency {
            targets.target_collection_frequency / metrics.collection_frequency
        } else {
            metrics.collection_frequency / targets.target_collection_frequency
        };
        score += freq_ratio * 0.2; // Weight 0.2

        // Throughput impact score (lower is better)
        let throughput_score = if metrics.throughput_impact > 0.0 {
            (targets.max_throughput_impact / metrics.throughput_impact).min(2.0)
        } else {
            2.0
        };
        score += throughput_score * 0.25; // Weight 0.25

        score
    }

    /// Get current heap statistics
    fn get_heap_stats(&self) -> Result<crate::memory::heap_manager::HeapStats, String> {
        // Get heap stats from heap manager through object store
        let object_store = self.gc.object_store();
        let stats = object_store.get_stats()?;
        
        // Convert to heap manager HeapStats format
        Ok(crate::memory::heap_manager::HeapStats {
            total_blocks: 1, // Simplified
            total_capacity: stats.total_objects * 1024, // Estimate
            total_used: stats.total_objects * 512, // Estimate
            total_free: stats.total_objects * 512, // Estimate  
            average_utilization: 0.5, // Estimate
            active_objects: stats.total_objects,
            object_registry_count: stats.total_objects,
            fragmentation_ratio: 0.1, // Default
            memory_pressure: crate::memory::heap_manager::MemoryPressure::Low,
            metrics: crate::memory::heap_manager::AllocationMetrics::default(),
        })
    }

    /// Get current adaptive statistics
    pub fn get_adaptive_stats(&self) -> Result<AdaptiveGcStats, String> {
        let current_strategy = *self.current_strategy.read()
            .map_err(|_| "Failed to read current strategy")?;
        
        let current_pattern = *self.current_pattern.read()
            .map_err(|_| "Failed to read current pattern")?;
        
        let performance_metrics = self.performance_metrics.read()
            .map_err(|_| "Failed to read performance metrics")?
            .clone();
        
        let adaptive_thresholds = self.adaptive_thresholds.read()
            .map_err(|_| "Failed to read adaptive thresholds")?
            .clone();
        
        let strategy_performance = self.strategy_performance.read()
            .map_err(|_| "Failed to read strategy performance")?
            .clone();

        Ok(AdaptiveGcStats {
            current_strategy,
            current_pattern,
            performance_metrics,
            adaptive_thresholds,
            strategy_performance,
            collection_count: self.collection_count.load(Ordering::Acquire),
            bytes_allocated_since_last_gc: self.bytes_allocated_since_last_gc.load(Ordering::Acquire),
            objects_allocated_since_last_gc: self.objects_allocated_since_last_gc.load(Ordering::Acquire),
            adaptation_active: self.adaptation_active.load(Ordering::Acquire),
        })
    }

    /// Enable or disable adaptation
    pub fn set_adaptation_active(&self, active: bool) {
        self.adaptation_active.store(active, Ordering::Release);
        if active {
            info!("Adaptive garbage collection enabled");
        } else {
            info!("Adaptive garbage collection disabled");
        }
    }

    /// Update configuration
    pub fn update_config(&self, new_config: AdaptiveGcConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to write config")?;
        *config = new_config.clone();

        // Update sub-components
        self.pressure_detector.update_config(new_config.pressure_config)?;
        self.trigger_manager.update_config(new_config.trigger_config)?;
        self.gc.update_config(new_config.base_gc_config)?;

        let mut thresholds = self.adaptive_thresholds.write()
            .map_err(|_| "Failed to write adaptive thresholds")?;
        *thresholds = new_config.adaptive_thresholds;

        info!("Updated adaptive GC configuration");
        Ok(())
    }

    /// Get underlying garbage collector reference
    pub fn gc(&self) -> &Arc<GarbageCollector> {
        &self.gc
    }

    /// Get pressure detector reference
    pub fn pressure_detector(&self) -> &Arc<MemoryPressureDetector> {
        &self.pressure_detector
    }

    /// Get trigger manager reference
    pub fn trigger_manager(&self) -> &Arc<CollectionTriggerManager> {
        &self.trigger_manager
    }
}

/// Statistics for adaptive garbage collection
#[derive(Debug, Clone)]
pub struct AdaptiveGcStats {
    pub current_strategy: AdaptiveStrategy,
    pub current_pattern: BehaviorPattern,
    pub performance_metrics: PerformanceMetrics,
    pub adaptive_thresholds: AdaptiveThresholds,
    pub strategy_performance: HashMap<AdaptiveStrategy, PerformanceMetrics>,
    pub collection_count: u64,
    pub bytes_allocated_since_last_gc: u64,
    pub objects_allocated_since_last_gc: u64,
    pub adaptation_active: bool,
}

// Safety: AdaptiveGarbageCollector is thread-safe through its use of Arc, RwLock, Mutex, and atomic types
unsafe impl Send for AdaptiveGarbageCollector {}
unsafe impl Sync for AdaptiveGarbageCollector {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_gc_creation() {
        let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        let stats = adaptive_gc.get_adaptive_stats().unwrap();
        
        assert_eq!(stats.current_strategy, AdaptiveStrategy::Balanced);
        assert_eq!(stats.current_pattern, BehaviorPattern::Mixed);
        assert!(stats.adaptation_active);
    }

    #[test]
    fn test_allocation_pattern_analysis() {
        let mut pattern = AllocationPattern::new();
        
        // Simulate steady allocation pattern
        for _ in 0..100 {
            pattern.record_allocation(1024); // Consistent size
            std::thread::sleep(std::time::Duration::from_millis(10)); // Consistent timing
        }
        
        let behavior = pattern.analyze_behavior();
        assert_eq!(behavior, BehaviorPattern::Steady);
    }

    #[test]
    fn test_performance_score_calculation() {
        let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        
        let metrics = PerformanceMetrics {
            allocation_rate: 1000000.0, // 1MB/s
            average_pause_time: Duration::from_millis(5),
            collection_frequency: 6.0,
            memory_efficiency: 0.8,
            throughput_impact: 3.0,
            pressure_trend: 0.0,
        };
        
        let targets = TargetMetrics {
            max_pause_time: Duration::from_millis(10),
            target_utilization: 0.80,
            target_collection_frequency: 6.0,
            max_throughput_impact: 5.0,
        };
        
        let score = adaptive_gc.calculate_performance_score(&metrics, &targets);
        assert!(score > 0.0);
        assert!(score <= 4.0); // Max possible score
    }

    #[test]
    fn test_threshold_adaptation() {
        let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        
        // Simulate high collection frequency requiring threshold adjustment
        {
            let mut metrics = adaptive_gc.performance_metrics.write().unwrap();
            metrics.collection_frequency = 12.0; // Double the target
        }
        
        let initial_threshold = adaptive_gc.adaptive_thresholds.read().unwrap().young_threshold;
        adaptive_gc.adapt_thresholds().unwrap();
        let new_threshold = adaptive_gc.adaptive_thresholds.read().unwrap().young_threshold;
        
        assert!(new_threshold > initial_threshold); // Should increase threshold to reduce frequency
    }

    #[test]
    fn test_strategy_selection() {
        let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        
        // Test emergency trigger always selects copying algorithm
        let algorithm = adaptive_gc.select_optimal_algorithm(CollectionTrigger::Emergency).unwrap();
        assert_eq!(algorithm, CollectionAlgorithm::Copying);
        
        // Test latency-sensitive strategy selects incremental
        {
            let mut strategy = adaptive_gc.current_strategy.write().unwrap();
            *strategy = AdaptiveStrategy::LatencySensitive;
        }
        
        let algorithm = adaptive_gc.select_optimal_algorithm(CollectionTrigger::AllocationPressure).unwrap();
        assert_eq!(algorithm, CollectionAlgorithm::Incremental);
    }

    #[test]
    fn test_adaptation_enable_disable() {
        let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap();
        
        assert!(adaptive_gc.adaptation_active.load(Ordering::Acquire));
        
        adaptive_gc.set_adaptation_active(false);
        assert!(!adaptive_gc.adaptation_active.load(Ordering::Acquire));
        
        adaptive_gc.set_adaptation_active(true);
        assert!(adaptive_gc.adaptation_active.load(Ordering::Acquire));
    }
}
