/// Adaptive Optimization System
/// 
/// This module provides adaptive optimization features that learn
/// from execution patterns and optimize accordingly.

use crate::error::{CursedError, Result};
use crate::optimization::{OptimizationConfig, PerformanceMetrics};

use std::collections::{HashMap, BTreeMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, RwLock};
use serde::{Serialize, Deserialize};
use tracing::{debug, info, instrument, warn, error};

/// Optimization suggestion with confidence rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// Name/ID of the optimization
    pub name: String,
    /// Type of optimization suggested
    pub optimization_type: OptimizationType,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Cost of applying optimization
    pub cost: OptimizationCost,
    /// Context where this applies
    pub context: String,
    /// Reasoning for the suggestion
    pub reasoning: String,
}

/// Types of optimizations that can be suggested
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationType {
    /// Function inlining
    Inlining,
    /// Loop unrolling
    LoopUnrolling,
    /// Dead code elimination
    DeadCodeElimination,
    /// Constant propagation
    ConstantPropagation,
    /// Vector operations
    Vectorization,
    /// Memory layout optimization
    MemoryLayout,
    /// Profile-guided optimization
    ProfileGuided,
    /// Custom optimization
    Custom(String),
}

/// Cost associated with applying an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCost {
    /// Compilation time increase
    pub compile_time_cost: Duration,
    /// Memory usage increase during compilation
    pub memory_cost: u64,
    /// Risk level (0.0 to 1.0)
    pub risk: f64,
}

/// Configuration for adaptive optimization
#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    /// Minimum execution count before adapting
    pub min_execution_count: u64,
    /// Window size for performance history
    pub history_window_size: usize,
    /// Learning rate for adaptation (0.0 to 1.0)
    pub learning_rate: f64,
    /// Confidence threshold for making optimization decisions
    pub confidence_threshold: f64,
    /// Maximum optimization level to use
    pub max_optimization_level: u32,
    /// Enable experimental optimizations
    pub enable_experimental: bool,
    /// Adaptation frequency (how often to review optimizations)
    pub adaptation_frequency: Duration,
    /// Performance improvement threshold for optimization
    pub improvement_threshold: f64,
    /// Regression detection threshold
    pub regression_threshold: f64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            min_execution_count: 100,
            history_window_size: 1000,
            learning_rate: 0.1,
            confidence_threshold: 0.8,
            max_optimization_level: 3,
            enable_experimental: false,
            adaptation_frequency: Duration::from_secs(60),
            improvement_threshold: 0.05, // 5%
            regression_threshold: 0.1,   // 10%
        }
    }
}

/// Execution profile for a function or code region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfile {
    /// Function or region name
    pub name: String,
    /// Execution count
    pub execution_count: u64,
    /// Total execution time
    pub total_time: Duration,
    /// Average execution time
    pub average_time: Duration,
    /// Performance history (recent execution times)
    pub performance_history: VecDeque<Duration>,
    /// Memory usage profile
    pub memory_usage: MemoryProfile,
    /// Call frequency (calls per second)
    pub call_frequency: f64,
    /// Hotness score (0.0 to 1.0)
    pub hotness_score: f64,
    /// Current optimization level applied
    pub current_optimization_level: u32,
    /// Optimization history
    pub optimization_history: Vec<OptimizationEvent>,
    /// Last profiling update
    pub last_update: SystemTime,
}

impl ExecutionProfile {
    /// Create a new execution profile
    pub fn new(name: String) -> Self {
        Self {
            name,
            execution_count: 0,
            total_time: Duration::default(),
            average_time: Duration::default(),
            performance_history: VecDeque::new(),
            memory_usage: MemoryProfile::default(),
            call_frequency: 0.0,
            hotness_score: 0.0,
            current_optimization_level: 0,
            optimization_history: Vec::new(),
            last_update: SystemTime::now(),
        }
    }
    
    /// Update profile with new execution data
    pub fn update(&mut self, execution_time: Duration, memory_used: u64, window_size: usize) {
        self.execution_count += 1;
        self.total_time += execution_time;
        self.average_time = self.total_time / self.execution_count as u32;
        
        // Update performance history with sliding window
        self.performance_history.push_back(execution_time);
        if self.performance_history.len() > window_size {
            self.performance_history.pop_front();
        }
        
        // Update memory profile
        self.memory_usage.update(memory_used);
        
        // Calculate call frequency
        let time_since_last = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap_or(Duration::from_secs(1));
        
        if time_since_last.as_secs() > 0 {
            self.call_frequency = 1.0 / time_since_last.as_secs_f64();
        }
        
        // Update hotness score
        self.update_hotness_score();
        
        self.last_update = SystemTime::now();
    }
    
    /// Calculate hotness score based on execution frequency and time
    fn update_hotness_score(&mut self) {
        // Hotness is based on execution count, call frequency, and total time
        let execution_factor = (self.execution_count as f64).ln().max(1.0) / 10.0;
        let frequency_factor = self.call_frequency.min(10.0) / 10.0;
        let time_factor = self.total_time.as_secs_f64() / 60.0; // Normalize to minutes
        
        self.hotness_score = (execution_factor + frequency_factor + time_factor.min(1.0)) / 3.0;
        self.hotness_score = self.hotness_score.min(1.0);
    }
    
    /// Check if function is hot (frequently executed)
    pub fn is_hot(&self, threshold: f64) -> bool {
        self.hotness_score >= threshold
    }
    
    /// Get recent performance trend
    pub fn get_performance_trend(&self) -> PerformanceTrend {
        if self.performance_history.len() < 10 {
            return PerformanceTrend::Insufficient;
        }
        
        let recent_half = self.performance_history.len() / 2;
        let older: Vec<_> = self.performance_history.iter().take(recent_half).collect();
        let newer: Vec<_> = self.performance_history.iter().skip(recent_half).collect();
        
        let older_avg = older.iter().map(|d| d.as_nanos()).sum::<u128>() / older.len() as u128;
        let newer_avg = newer.iter().map(|d| d.as_nanos()).sum::<u128>() / newer.len() as u128;
        
        let change_ratio = (newer_avg as f64 - older_avg as f64) / older_avg as f64;
        
        if change_ratio < -0.05 {
            PerformanceTrend::Improving
        } else if change_ratio > 0.05 {
            PerformanceTrend::Degrading
        } else {
            PerformanceTrend::Stable
        }
    }
}

/// Memory usage profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Total memory allocated
    pub total_allocated: u64,
    /// Peak memory usage
    pub peak_usage: u64,
    /// Average memory usage
    pub average_usage: u64,
    /// Memory allocation count
    pub allocation_count: u64,
}

impl MemoryProfile {
    /// Update memory profile
    pub fn update(&mut self, memory_used: u64) {
        self.total_allocated += memory_used;
        self.peak_usage = self.peak_usage.max(memory_used);
        self.allocation_count += 1;
        self.average_usage = self.total_allocated / self.allocation_count;
    }
}

/// Performance trend analysis
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceTrend {
    /// Performance is improving
    Improving,
    /// Performance is stable
    Stable,
    /// Performance is degrading
    Degrading,
    /// Insufficient data for analysis
    Insufficient,
}

/// Optimization event in the history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEvent {
    /// Timestamp of the optimization
    pub timestamp: SystemTime,
    /// Optimization level applied
    pub optimization_level: u32,
    /// Performance before optimization
    pub performance_before: Duration,
    /// Performance after optimization
    pub performance_after: Option<Duration>,
    /// Optimization strategy used
    pub strategy: OptimizationStrategy,
    /// Success of the optimization
    pub success: Option<bool>,
}

/// Optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Increase optimization level
    IncreaseLevel,
    /// Decrease optimization level (due to regression)
    DecreaseLevel,
    /// Apply specific optimization pass
    SpecificPass(String),
    /// Enable experimental optimization
    Experimental(String),
    /// Disable optimization (performance regression)
    Disable,
}

/// Optimization feedback from execution
#[derive(Debug, Clone)]
pub struct OptimizationFeedback {
    /// Function or region name
    pub name: String,
    /// Execution time
    pub execution_time: Duration,
    /// Memory usage
    pub memory_usage: u64,
    /// Success indicator
    pub success: bool,
    /// CursedError information if any
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Learning optimizer that adapts based on feedback
#[derive(Debug)]
pub struct LearningOptimizer {
    /// Configuration
    config: AdaptiveConfig,
    /// Execution profiles
    profiles: Arc<RwLock<HashMap<String, ExecutionProfile>>>,
    /// Optimization strategies and their success rates
    strategy_success: Arc<Mutex<HashMap<OptimizationStrategy, SuccessRate>>>,
    /// Global optimization trends
    global_trends: Arc<Mutex<GlobalTrends>>,
    /// Last adaptation time
    last_adaptation: Arc<Mutex<SystemTime>>,
}

/// Success rate tracking for optimization strategies
#[derive(Debug, Clone)]
pub struct SuccessRate {
    /// Total applications of this strategy
    pub total_applications: u64,
    /// Successful applications
    pub successful_applications: u64,
    /// Average performance improvement
    pub average_improvement: f64,
    /// Confidence in this strategy
    pub confidence: f64,
}

impl SuccessRate {
    /// Create new success rate tracker
    pub fn new() -> Self {
        Self {
            total_applications: 0,
            successful_applications: 0,
            average_improvement: 0.0,
            confidence: 0.0,
        }
    }
    
    /// Update with new result
    pub fn update(&mut self, success: bool, improvement: f64) {
        self.total_applications += 1;
        if success {
            self.successful_applications += 1;
        }
        
        // Update average improvement using exponential moving average
        let alpha = 0.1;
        self.average_improvement = alpha * improvement + (1.0 - alpha) * self.average_improvement;
        
        // Update confidence based on sample size and success rate
        let success_rate = self.successful_applications as f64 / self.total_applications as f64;
        let sample_confidence = (self.total_applications as f64 / 100.0).min(1.0);
        self.confidence = success_rate * sample_confidence;
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_applications == 0 {
            0.0
        } else {
            self.successful_applications as f64 / self.total_applications as f64
        }
    }
}

/// Global optimization trends
#[derive(Debug, Default)]
pub struct GlobalTrends {
    /// Overall performance trend
    pub performance_trend: PerformanceTrend,
    /// Total optimizations applied
    pub total_optimizations: u64,
    /// Successful optimizations
    pub successful_optimizations: u64,
    /// Average performance improvement
    pub average_improvement: f64,
    /// Hot functions count
    pub hot_functions_count: usize,
}

impl LearningOptimizer {
    /// Create a new learning optimizer
    pub fn new(config: AdaptiveConfig) -> Self {
        Self {
            config,
            profiles: Arc::new(RwLock::new(HashMap::new())),
            strategy_success: Arc::new(Mutex::new(HashMap::new())),
            global_trends: Arc::new(Mutex::new(GlobalTrends::default())),
            last_adaptation: Arc::new(Mutex::new(SystemTime::now())),
        }
    }
    
    /// Record execution feedback
    #[instrument(skip(self))]
    pub fn record_feedback(&self, feedback: OptimizationFeedback) -> Result<()> {
        let mut profiles = self.profiles.write().unwrap();
        
        let profile = profiles.entry(feedback.name.clone())
            .or_insert_with(|| ExecutionProfile::new(feedback.name.clone()));
        
        profile.update(feedback.execution_time, feedback.memory_usage, self.config.history_window_size);
        
        debug!("Recorded feedback for {}: {:?}", feedback.name, feedback.execution_time);
        Ok(())
    }
    
    /// Get optimization recommendations
    #[instrument(skip(self))]
    pub fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        let profiles = self.profiles.read().unwrap();
        let mut recommendations = Vec::new();
        
        for (name, profile) in profiles.iter() {
            if profile.execution_count < self.config.min_execution_count {
                continue;
            }
            
            let recommendation = self.analyze_profile(name, profile)?;
            if let Some(rec) = recommendation {
                recommendations.push(rec);
            }
        }
        
        // Sort by priority (hotness and potential improvement)
        recommendations.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(recommendations)
    }
    
    /// Analyze a single profile for optimization opportunities
    fn analyze_profile(&self, name: &str, profile: &ExecutionProfile) -> Result<Option<OptimizationRecommendation>> {
        let trend = profile.get_performance_trend();
        let strategy_success = self.strategy_success.lock().unwrap();
        
        let recommendation = match trend {
            PerformanceTrend::Degrading => {
                // Performance is getting worse, suggest reducing optimization or specific fixes
                if profile.current_optimization_level > 0 {
                    Some(OptimizationRecommendation {
                        function_name: name.to_string(),
                        strategy: OptimizationStrategy::DecreaseLevel,
                        priority: profile.hotness_score * 0.8, // High priority for hot functions
                        confidence: 0.7,
                        expected_improvement: 0.1,
                        reason: "Performance degradation detected".to_string(),
                    })
                } else {
                    None
                }
            },
            PerformanceTrend::Stable => {
                // Stable performance, consider increasing optimization if hot
                if profile.is_hot(0.6) && profile.current_optimization_level < self.config.max_optimization_level {
                    let increase_strategy = OptimizationStrategy::IncreaseLevel;
                    let success_rate = strategy_success.get(&increase_strategy)
                        .map(|sr| sr.confidence)
                        .unwrap_or(0.5);
                    
                    if success_rate >= self.config.confidence_threshold {
                        Some(OptimizationRecommendation {
                            function_name: name.to_string(),
                            strategy: increase_strategy,
                            priority: profile.hotness_score * success_rate,
                            confidence: success_rate,
                            expected_improvement: 0.15,
                            reason: "Hot function with stable performance".to_string(),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            PerformanceTrend::Improving => {
                // Performance is improving, maintain current strategy or try experimental
                if self.config.enable_experimental && profile.hotness_score > 0.8 {
                    Some(OptimizationRecommendation {
                        function_name: name.to_string(),
                        strategy: OptimizationStrategy::Experimental("vectorization".to_string()),
                        priority: profile.hotness_score * 0.6,
                        confidence: 0.4, // Lower confidence for experimental
                        expected_improvement: 0.25,
                        reason: "Experimental optimization for very hot function".to_string(),
                    })
                } else {
                    None
                }
            },
            PerformanceTrend::Insufficient => None,
        };
        
        Ok(recommendation)
    }
    
    /// Apply optimization feedback
    #[instrument(skip(self))]
    pub fn apply_optimization_feedback(
        &self,
        function_name: &str,
        strategy: OptimizationStrategy,
        success: bool,
        performance_improvement: f64,
    ) -> Result<()> {
        // Update strategy success rates
        {
            let mut strategy_success = self.strategy_success.lock().unwrap();
            let success_rate = strategy_success.entry(strategy.clone())
                .or_insert_with(SuccessRate::new);
            success_rate.update(success, performance_improvement);
        }
        
        // Update function profile
        {
            let mut profiles = self.profiles.write().unwrap();
            if let Some(profile) = profiles.get_mut(function_name) {
                let event = OptimizationEvent {
                    timestamp: SystemTime::now(),
                    optimization_level: match &strategy {
                        OptimizationStrategy::IncreaseLevel => profile.current_optimization_level + 1,
                        OptimizationStrategy::DecreaseLevel => profile.current_optimization_level.saturating_sub(1),
                        _ => profile.current_optimization_level,
                    },
                    performance_before: profile.average_time,
                    performance_after: None, // Will be updated on next execution
                    strategy: strategy.clone(),
                    success: Some(success),
                };
                
                profile.optimization_history.push(event);
                
                // Update current optimization level
                match strategy {
                    OptimizationStrategy::IncreaseLevel if success => {
                        profile.current_optimization_level = (profile.current_optimization_level + 1)
                            .min(self.config.max_optimization_level);
                    },
                    OptimizationStrategy::DecreaseLevel => {
                        profile.current_optimization_level = profile.current_optimization_level.saturating_sub(1);
                    },
                    OptimizationStrategy::Disable => {
                        profile.current_optimization_level = 0;
                    },
                    _ => {},
                }
            }
        }
        
        // Update global trends
        {
            let mut global_trends = self.global_trends.lock().unwrap();
            global_trends.total_optimizations += 1;
            if success {
                global_trends.successful_optimizations += 1;
            }
            
            // Update average improvement using exponential moving average
            let alpha = 0.1;
            global_trends.average_improvement = alpha * performance_improvement + 
                (1.0 - alpha) * global_trends.average_improvement;
        }
        
        info!("Applied optimization feedback for {}: {:?}, success: {}, improvement: {:.2}%",
              function_name, strategy, success, performance_improvement * 100.0);
        
        Ok(())
    }
    
    /// Check if adaptation should be performed
    pub fn should_adapt(&self) -> bool {
        let last_adaptation = *self.last_adaptation.lock().unwrap();
        SystemTime::now()
            .duration_since(last_adaptation)
            .unwrap_or(Duration::default()) >= self.config.adaptation_frequency
    }
    
    /// Perform adaptive optimization
    #[instrument(skip(self))]
    pub fn adapt(&self) -> Result<AdaptationResult> {
        let recommendations = self.get_recommendations()?;
        let global_trends = self.global_trends.lock().unwrap().clone();
        
        *self.last_adaptation.lock().unwrap() = SystemTime::now();
        
        let adaptation_result = AdaptationResult {
            recommendations: recommendations.clone(),
            global_trends,
            adaptation_timestamp: SystemTime::now(),
        };
        
        info!("Adaptive optimization completed: {} recommendations generated", recommendations.len());
        
        Ok(adaptation_result)
    }
    
    /// Get current profiles summary
    pub fn get_profiles_summary(&self) -> ProfilesSummary {
        let profiles = self.profiles.read().unwrap();
        let strategy_success = self.strategy_success.lock().unwrap();
        let global_trends = self.global_trends.lock().unwrap();
        
        let total_functions = profiles.len();
        let hot_functions = profiles.values().filter(|p| p.is_hot(0.6)).count();
        let optimized_functions = profiles.values().filter(|p| p.current_optimization_level > 0).count();
        
        let total_executions: u64 = profiles.values().map(|p| p.execution_count).sum();
        let total_time: Duration = profiles.values().map(|p| p.total_time).sum();
        
        ProfilesSummary {
            total_functions,
            hot_functions,
            optimized_functions,
            total_executions,
            total_execution_time: total_time,
            strategy_success_rates: strategy_success.clone(),
            global_trends: global_trends.clone(),
        }
    }
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Function name
    pub function_name: String,
    /// Recommended optimization strategy
    pub strategy: OptimizationStrategy,
    /// Priority score (0.0 to 1.0)
    pub priority: f64,
    /// Confidence in recommendation (0.0 to 1.0)
    pub confidence: f64,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Reason for recommendation
    pub reason: String,
}

/// Result of adaptive optimization
#[derive(Debug, Clone)]
pub struct AdaptationResult {
    /// Generated recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
    /// Global optimization trends
    pub global_trends: GlobalTrends,
    /// Timestamp of adaptation
    pub adaptation_timestamp: SystemTime,
}

/// Summary of all profiles
#[derive(Debug, Clone)]
pub struct ProfilesSummary {
    /// Total number of functions
    pub total_functions: usize,
    /// Number of hot functions
    pub hot_functions: usize,
    /// Number of optimized functions
    pub optimized_functions: usize,
    /// Total executions across all functions
    pub total_executions: u64,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Strategy success rates
    pub strategy_success_rates: HashMap<OptimizationStrategy, SuccessRate>,
    /// Global trends
    pub global_trends: GlobalTrends,
}

/// Main adaptive optimizer
pub struct AdaptiveOptimizer {
    /// Configuration
    config: AdaptiveConfig,
    /// Learning optimizer
    learning_optimizer: LearningOptimizer,
}

impl AdaptiveOptimizer {
    /// Create a new adaptive optimizer
    pub fn new(optimization_config: &OptimizationConfig) -> Result<Self> {
        let config = AdaptiveConfig::default();
        let learning_optimizer = LearningOptimizer::new(config.clone());
        
        Ok(Self {
            config,
            learning_optimizer,
        })
    }
    
    /// Record execution feedback
    pub fn record_execution(&self, feedback: OptimizationFeedback) -> Result<()> {
        self.learning_optimizer.record_feedback(feedback)
    }
    
    /// Get optimization recommendations
    pub fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        self.learning_optimizer.get_recommendations()
    }
    
    /// Apply optimization result
    pub fn apply_optimization_result(
        &self,
        function_name: &str,
        strategy: OptimizationStrategy,
        success: bool,
        improvement: f64,
    ) -> Result<()> {
        self.learning_optimizer.apply_optimization_feedback(function_name, strategy, success, improvement)
    }
    
    /// Perform adaptation if needed
    pub fn adapt_if_needed(&self) -> Result<Option<AdaptationResult>> {
        if self.learning_optimizer.should_adapt() {
            Ok(Some(self.learning_optimizer.adapt()?))
        } else {
            Ok(None)
        }
    }
    
    /// Get profiles summary
    pub fn get_summary(&self) -> ProfilesSummary {
        self.learning_optimizer.get_profiles_summary()
    }
    
    /// Get configuration
    pub fn config(&self) -> &AdaptiveConfig {
        &self.config
    }
}

