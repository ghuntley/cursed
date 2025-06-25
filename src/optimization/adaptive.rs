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
    /// Type of optimization suggested
    /// Confidence level (0.0 to 1.0)
    /// Expected performance improvement
    /// Cost of applying optimization
    /// Context where this applies
    /// Reasoning for the suggestion
/// Types of optimizations that can be suggested
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationType {
    /// Function inlining
    /// Loop unrolling
    /// Dead code elimination
    /// Constant propagation
    /// Vector operations
    /// Memory layout optimization
    /// Profile-guided optimization
    /// Custom optimization
/// Cost associated with applying an optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCost {
    /// Compilation time increase
    /// Memory usage increase during compilation
    /// Risk level (0.0 to 1.0)
/// Configuration for adaptive optimization
#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    /// Minimum execution count before adapting
    /// Window size for performance history
    /// Learning rate for adaptation (0.0 to 1.0)
    /// Confidence threshold for making optimization decisions
    /// Maximum optimization level to use
    /// Enable experimental optimizations
    /// Adaptation frequency (how often to review optimizations)
    /// Performance improvement threshold for optimization
    /// Regression detection threshold
impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            improvement_threshold: 0.05, // 5%
            regression_threshold: 0.1,   // 10%
        }
    }
/// Execution profile for a function or code region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfile {
    /// Function or region name
    /// Execution count
    /// Total execution time
    /// Average execution time
    /// Performance history (recent execution times)
    /// Memory usage profile
    /// Call frequency (calls per second)
    /// Hotness score (0.0 to 1.0)
    /// Current optimization level applied
    /// Optimization history
    /// Last profiling update
impl ExecutionProfile {
    /// Create a new execution profile
    pub fn new(name: String) -> Self {
        Self {
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
        // Update memory profile
        self.memory_usage.update(memory_used);
        
        // Calculate call frequency
        let time_since_last = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap_or(Duration::from_secs(1));
        
        if time_since_last.as_secs() > 0 {
            self.call_frequency = 1.0 / time_since_last.as_secs_f64();
        // Update hotness score
        self.update_hotness_score();
        
        self.last_update = SystemTime::now();
    /// Calculate hotness score based on execution frequency and time
    fn update_hotness_score(&mut self) {
        // Hotness is based on execution count, call frequency, and total time
        let execution_factor = (self.execution_count as f64).ln().max(1.0) / 10.0;
        let frequency_factor = self.call_frequency.min(10.0) / 10.0;
        let time_factor = self.total_time.as_secs_f64() / 60.0; // Normalize to minutes
        
        self.hotness_score = (execution_factor + frequency_factor + time_factor.min(1.0)) / 3.0;
        self.hotness_score = self.hotness_score.min(1.0);
    /// Check if function is hot (frequently executed)
    pub fn is_hot(&self, threshold: f64) -> bool {
        self.hotness_score >= threshold
    /// Get recent performance trend
    pub fn get_performance_trend(&self) -> PerformanceTrend {
        if self.performance_history.len() < 10 {
            return PerformanceTrend::Insufficient;
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
/// Memory usage profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Total memory allocated
    /// Peak memory usage
    /// Average memory usage
    /// Memory allocation count
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
    /// Performance is stable
    /// Performance is degrading
    /// Insufficient data for analysis
/// Optimization event in the history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEvent {
    /// Timestamp of the optimization
    /// Optimization level applied
    /// Performance before optimization
    /// Performance after optimization
    /// Optimization strategy used
    /// Success of the optimization
/// Optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Increase optimization level
    /// Decrease optimization level (due to regression)
    /// Apply specific optimization pass
    /// Enable experimental optimization
    /// Disable optimization (performance regression)
/// Optimization feedback from execution
#[derive(Debug, Clone)]
pub struct OptimizationFeedback {
    /// Function or region name
    /// Execution time
    /// Memory usage
    /// Success indicator
    /// CursedError information if any
    /// Timestamp
/// Learning optimizer that adapts based on feedback
#[derive(Debug)]
pub struct LearningOptimizer {
    /// Configuration
    /// Execution profiles
    /// Optimization strategies and their success rates
    /// Global optimization trends
    /// Last adaptation time
/// Success rate tracking for optimization strategies
#[derive(Debug, Clone)]
pub struct SuccessRate {
    /// Total applications of this strategy
    /// Successful applications
    /// Average performance improvement
    /// Confidence in this strategy
impl SuccessRate {
    /// Create new success rate tracker
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Update with new result
    pub fn update(&mut self, success: bool, improvement: f64) {
        self.total_applications += 1;
        if success {
            self.successful_applications += 1;
        // Update average improvement using exponential moving average
        let alpha = 0.1;
        self.average_improvement = alpha * improvement + (1.0 - alpha) * self.average_improvement;
        
        // Update confidence based on sample size and success rate
        let success_rate = self.successful_applications as f64 / self.total_applications as f64;
        let sample_confidence = (self.total_applications as f64 / 100.0).min(1.0);
        self.confidence = success_rate * sample_confidence;
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_applications == 0 {
            0.0
        } else {
            self.successful_applications as f64 / self.total_applications as f64
        }
    }
/// Global optimization trends
#[derive(Debug, Default)]
pub struct GlobalTrends {
    /// Overall performance trend
    /// Total optimizations applied
    /// Successful optimizations
    /// Average performance improvement
    /// Hot functions count
impl LearningOptimizer {
    /// Create a new learning optimizer
    pub fn new(config: AdaptiveConfig) -> Self {
        Self {
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
    /// Get optimization recommendations
    #[instrument(skip(self))]
    pub fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        let profiles = self.profiles.read().unwrap();
        let mut recommendations = Vec::new();
        
        for (name, profile) in profiles.iter() {
            if profile.execution_count < self.config.min_execution_count {
                continue;
            let recommendation = self.analyze_profile(name, profile)?;
            if let Some(rec) = recommendation {
                recommendations.push(rec);
            }
        }
        
        // Sort by priority (hotness and potential improvement)
        recommendations.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(recommendations)
    /// Analyze a single profile for optimization opportunities
    fn analyze_profile(&self, name: &str, profile: &ExecutionProfile) -> Result<Option<OptimizationRecommendation>> {
        let trend = profile.get_performance_trend();
        let strategy_success = self.strategy_success.lock().unwrap();
        
        let recommendation = match trend {
            PerformanceTrend::Degrading => {
                // Performance is getting worse, suggest reducing optimization or specific fixes
                if profile.current_optimization_level > 0 {
                    Some(OptimizationRecommendation {
                        priority: profile.hotness_score * 0.8, // High priority for hot functions
                    })
                } else {
                    None
                }
            PerformanceTrend::Stable => {
                // Stable performance, consider increasing optimization if hot
                if profile.is_hot(0.6) && profile.current_optimization_level < self.config.max_optimization_level {
                    let increase_strategy = OptimizationStrategy::IncreaseLevel;
                    let success_rate = strategy_success.get(&increase_strategy)
                        .map(|sr| sr.confidence)
                        .unwrap_or(0.5);
                    
                    if success_rate >= self.config.confidence_threshold {
                        Some(OptimizationRecommendation {
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            PerformanceTrend::Improving => {
                // Performance is improving, maintain current strategy or try experimental
                if self.config.enable_experimental && profile.hotness_score > 0.8 {
                    Some(OptimizationRecommendation {
                        confidence: 0.4, // Lower confidence for experimental
                    })
                } else {
                    None
                }
        
        Ok(recommendation)
    /// Apply optimization feedback
    #[instrument(skip(self))]
    pub fn apply_optimization_feedback(
    ) -> Result<()> {
        // Update strategy success rates
        {
            let mut strategy_success = self.strategy_success.lock().unwrap();
            let success_rate = strategy_success.entry(strategy.clone())
                .or_insert_with(SuccessRate::new);
            success_rate.update(success, performance_improvement);
        // Update function profile
        {
            let mut profiles = self.profiles.write().unwrap();
            if let Some(profile) = profiles.get_mut(function_name) {
                let event = OptimizationEvent {
                    optimization_level: match &strategy {
                    performance_after: None, // Will be updated on next execution
                
                profile.optimization_history.push(event);
                
                // Update current optimization level
                match strategy {
                    OptimizationStrategy::IncreaseLevel if success => {
                        profile.current_optimization_level = (profile.current_optimization_level + 1)
                            .min(self.config.max_optimization_level);
                    OptimizationStrategy::DecreaseLevel => {
                        profile.current_optimization_level = profile.current_optimization_level.saturating_sub(1);
                    OptimizationStrategy::Disable => {
                        profile.current_optimization_level = 0;
                }
            }
        // Update global trends
        {
            let mut global_trends = self.global_trends.lock().unwrap();
            global_trends.total_optimizations += 1;
            if success {
                global_trends.successful_optimizations += 1;
            // Update average improvement using exponential moving average
            let alpha = 0.1;
            global_trends.average_improvement = alpha * performance_improvement + 
                (1.0 - alpha) * global_trends.average_improvement;
              function_name, strategy, success, performance_improvement * 100.0);
        
        Ok(())
    /// Check if adaptation should be performed
    pub fn should_adapt(&self) -> bool {
        let last_adaptation = *self.last_adaptation.lock().unwrap();
        SystemTime::now()
            .duration_since(last_adaptation)
            .unwrap_or(Duration::default()) >= self.config.adaptation_frequency
    /// Perform adaptive optimization
    #[instrument(skip(self))]
    pub fn adapt(&self) -> Result<AdaptationResult> {
        let recommendations = self.get_recommendations()?;
        let global_trends = self.global_trends.lock().unwrap().clone();
        
        *self.last_adaptation.lock().unwrap() = SystemTime::now();
        
        let adaptation_result = AdaptationResult {
        
        info!("Adaptive optimization completed: {} recommendations generated", recommendations.len());
        
        Ok(adaptation_result)
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
        }
    }
/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Function name
    /// Recommended optimization strategy
    /// Priority score (0.0 to 1.0)
    /// Confidence in recommendation (0.0 to 1.0)
    /// Expected performance improvement
    /// Reason for recommendation
/// Result of adaptive optimization
#[derive(Debug, Clone)]
pub struct AdaptationResult {
    /// Generated recommendations
    /// Global optimization trends
    /// Timestamp of adaptation
/// Summary of all profiles
#[derive(Debug, Clone)]
pub struct ProfilesSummary {
    /// Total number of functions
    /// Number of hot functions
    /// Number of optimized functions
    /// Total executions across all functions
    /// Total execution time
    /// Strategy success rates
    /// Global trends
/// Main adaptive optimizer
pub struct AdaptiveOptimizer {
    /// Configuration
    /// Learning optimizer
impl AdaptiveOptimizer {
    /// Create a new adaptive optimizer
    pub fn new(optimization_config: &OptimizationConfig) -> Result<Self> {
        let config = AdaptiveConfig::default();
        let learning_optimizer = LearningOptimizer::new(config.clone());
        
        Ok(Self {
        })
    /// Record execution feedback
    pub fn record_execution(&self, feedback: OptimizationFeedback) -> Result<()> {
        self.learning_optimizer.record_feedback(feedback)
    /// Get optimization recommendations
    pub fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        self.learning_optimizer.get_recommendations()
    /// Apply optimization result
    pub fn apply_optimization_result(
    ) -> Result<()> {
        self.learning_optimizer.apply_optimization_feedback(function_name, strategy, success, improvement)
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
    /// Get configuration
    pub fn config(&self) -> &AdaptiveConfig {
        &self.config
    }
}

