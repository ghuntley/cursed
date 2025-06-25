/// Machine Learning-Guided Optimization System for CURSED
/// 
/// This module provides ML-driven optimization decision making for the CURSED compiler.
/// It includes feature extraction, model training, prediction, data collection, and
/// continuous learning capabilities.

pub mod feature_extraction;
pub mod model_training;
pub mod prediction;
pub mod data_collection;
pub mod continuous_learning;

// Re-export main types for convenience
pub use feature_extraction::{FeatureExtractor, FeatureVector, CursedSpecificFeatures};
pub use model_training::{ModelTrainer, TrainingConfig, ModelType};
pub use prediction::{OptimizationPredictor, PredictionResult, PredictionConfig};
pub use data_collection::{PerformanceDataCollector, CompilationMetrics, RuntimeMetrics};
pub use continuous_learning::{ContinuousLearningEngine, LearningConfig, UpdateTrigger as ModelUpdateTrigger};

use crate::error::{CursedError, Result};

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn, instrument};

/// ML-guided optimization coordinator
#[derive(Debug)]
pub struct MLOptimizationCoordinator {
/// Configuration for ML optimization system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLOptimizationConfig {
/// Optimization strategy recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategy {
// Import canonical OptimizationLevel from optimization_config
pub use crate::common_types::optimization_level::OptimizationLevel as MLOptimizationLevel;

// Alias for compatibility in this module
pub type OptimizationLevel = crate::common::optimization_level::OptimizationLevel;

/// Optimization passes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPass {
    // Function-level optimizations
    
    // Loop optimizations
    
    // Memory optimizations
    
    // CURSED-specific optimizations
/// Cached performance data
#[derive(Debug, Clone)]
pub struct CachedPerformance {
impl Default for MLOptimizationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl MLOptimizationCoordinator {
    /// Create new ML optimization coordinator
    #[instrument]
    pub fn new(config: MLOptimizationConfig) -> Result<Self> {
        info!("Initializing ML optimization coordinator");
        
        let feature_extractor = FeatureExtractor::new(config.feature_config.clone())?;
        let model_trainer = ModelTrainer::new(config.training_config.clone())?;
        let predictor = OptimizationPredictor::new(config.prediction_config.clone())?;
        let data_collector = PerformanceDataCollector::new()?;
        let continuous_learner = ContinuousLearningEngine::new(config.learning_config.clone())?;
        
        Ok(Self {
        })
    /// Get optimization strategy recommendation for given code
    #[instrument(skip(self, source_code, context))]
    pub fn recommend_optimization_strategy(
    ) -> Result<OptimizationStrategy> {
        info!("Generating optimization strategy recommendation");
        
        // Check cache first
        let cache_key = self.generate_cache_key(source_code, context);
        if let Some(cached) = self.performance_cache.get_mut(&cache_key) {
            debug!("Using cached optimization strategy");
            cached.hit_count += 1;
            return Ok(cached.strategy.clone());
        // Extract features from source code
        let features = self.feature_extractor.extract_features(source_code, Some(context))?;
        
        // Get prediction from ML models
        let prediction = self.predictor.predict_optimization_strategy(&features)?;
        
        // Create optimization strategy
        let strategy = self.create_optimization_strategy(prediction, &features)?;
        
        // Cache the strategy
        self.cache_strategy(cache_key, &strategy);
        
        Ok(strategy)
    /// Record optimization outcome for learning
    #[instrument(skip(self, strategy, compilation_metrics))]
    pub fn record_optimization_outcome(
    ) -> Result<()> {
        info!("Recording optimization outcome for learning");
        
        // Collect performance data
        self.data_collector.record_compilation_data(
        )?;
        
        self.data_collector.record_runtime_data(
        )?;
        
        // Trigger continuous learning
        self.continuous_learner.process_new_data(
        )?;
        
        // Check if model updates are needed
        if self.continuous_learner.should_update_models()? {
            self.trigger_model_update()?;
        Ok(())
    /// Train ML models with collected data
    #[instrument(skip(self))]
    pub fn train_models(&mut self) -> Result<()> {
        info!("Training ML models");
        
        let training_data = self.data_collector.get_training_data()?;
        self.model_trainer.train_all_models(&training_data)?;
        
        // Update predictor with new models
        let trained_models = self.model_trainer.get_trained_models()?;
        self.predictor.update_models(trained_models)?;
        
        info!("Model training completed successfully");
        Ok(())
    /// Get performance statistics
    pub fn get_performance_statistics(&self) -> Result<PerformanceStatistics> {
        let model_accuracy = self.predictor.get_model_accuracy()?;
        let data_statistics = self.data_collector.get_statistics()?;
        let learning_statistics = self.continuous_learner.get_statistics()?;
        
        Ok(PerformanceStatistics {
        })
    /// Update configuration
    pub fn update_config(&mut self, config: MLOptimizationConfig) -> Result<()> {
        self.config = config.clone();
        self.feature_extractor.update_config(config.feature_config)?;
        self.model_trainer.update_config(config.training_config)?;
        self.predictor.update_config(config.prediction_config)?;
        self.continuous_learner.update_config(config.learning_config)?;
        Ok(())
    // Private helper methods
    
    fn generate_cache_key(&self, source_code: &str, context: &CompilationContext) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source_code.hash(&mut hasher);
        context.target_arch.hash(&mut hasher);
        context.optimization_goals.hash(&mut hasher);
        
        format!("opt_{}_{}", hasher.finish(), context.target_arch)
    fn create_optimization_strategy(
    ) -> Result<OptimizationStrategy> {
        let optimization_level = self.determine_optimization_level(&prediction, features)?;
        let enabled_passes = self.select_optimization_passes(&prediction, features)?;
        let pass_parameters = self.calculate_pass_parameters(&prediction, features)?;
        
        Ok(OptimizationStrategy {
        })
    fn determine_optimization_level(
    ) -> Result<OptimizationLevel> {
        // Analyze prediction and features to determine optimal level
        if features.performance_features.execution_frequency > 100.0 {
            // Hot code path - aggressive optimization
            Ok(OptimizationLevel::O3)
        } else if features.code_features.cyclomatic_complexity > 10.0 {
            // Complex code - balanced optimization
            Ok(OptimizationLevel::Speed)
        } else if features.function_features.size_in_bytes > 10000 {
            // Large functions - size optimization
            Ok(OptimizationLevel::Os)
        } else {
            // Default to speed optimization
            Ok(OptimizationLevel::Speed)
        }
    }
    
    fn select_optimization_passes(
    ) -> Result<Vec<OptimizationPass>> {
        let mut passes = Vec::new();
        
        // Always include basic optimizations
        passes.push(OptimizationPass::DeadCodeElimination);
        passes.push(OptimizationPass::ConstantPropagation);
        
        // Function inlining based on ML prediction
        if prediction.should_inline.unwrap_or(false) {
            let aggressiveness = prediction.inline_aggressiveness.unwrap_or(0.5);
            passes.push(OptimizationPass::Inlining { aggressiveness });
        // Loop optimizations
        if features.function_features.loop_count > 0 {
            if prediction.should_unroll_loops.unwrap_or(false) {
                let factor = prediction.unroll_factor.unwrap_or(4);
                passes.push(OptimizationPass::LoopUnrolling { factor });
            if prediction.should_vectorize.unwrap_or(false) {
                let width = prediction.vector_width.unwrap_or(4);
                passes.push(OptimizationPass::LoopVectorization { width });
            }
        }
        
        // CURSED-specific optimizations
        if features.cursed_features.goroutine_usage.goroutine_spawn_count > 0 {
            let target_size = features.cursed_features.goroutine_usage.stack_size_requirements;
            passes.push(OptimizationPass::GoroutineStackOptimization { target_size });
        if features.cursed_features.channel_usage.channel_count > 0 {
            let buffer_size = prediction.optimal_channel_buffer_size.unwrap_or(16);
            passes.push(OptimizationPass::ChannelBufferOptimization { buffer_size });
        if features.cursed_features.interface_complexity.interface_count > 0 {
            passes.push(OptimizationPass::InterfaceDevirtualization);
        if features.cursed_features.error_propagation_usage.question_mark_operator_usage > 0 {
            passes.push(OptimizationPass::ErrorPropagationOptimization);
        if features.cursed_features.gen_z_slang_patterns.slay_function_usage > 0 {
            let threshold = prediction.slang_inline_threshold.unwrap_or(0.7);
            passes.push(OptimizationPass::GenZSlangInlining { threshold });
        Ok(passes)
    fn calculate_pass_parameters(
    ) -> Result<HashMap<String, f64>> {
        let mut parameters = HashMap::new();
        
        // Calculate parameters based on code characteristics
                         prediction.inline_threshold.unwrap_or(100.0));
                         prediction.vectorization_threshold.unwrap_or(10.0));
                         prediction.unroll_benefit_ratio.unwrap_or(2.0));
        
        // CURSED-specific parameters
        if features.cursed_features.goroutine_usage.goroutine_spawn_count > 0 {
            parameters.insert("goroutine_stack_growth_factor".to_string(), 1.5);
            parameters.insert("goroutine_scheduling_quantum".to_string(), 10.0);
        Ok(parameters)
    fn generate_reasoning(&self, prediction: &PredictionResult, features: &FeatureVector) -> String {
        let mut reasoning = String::new();
        
        reasoning.push_str(&format!(
            features.function_features.call_count
        ));
        
        if features.cursed_features.goroutine_usage.goroutine_spawn_count > 0 {
            reasoning.push_str(&format!(
                features.cursed_features.goroutine_usage.goroutine_spawn_count
            ));
        if features.performance_features.execution_frequency > 50.0 {
            reasoning.push_str("High execution frequency detected, prioritizing speed optimizations. ");
        reasoning.push_str(&format!("ML confidence: {:.2}", prediction.confidence));
        
        reasoning
    fn cache_strategy(&mut self, key: String, strategy: &OptimizationStrategy) {
        if self.performance_cache.len() >= self.config.cache_size {
            // Simple LRU eviction
            if let Some(oldest_key) = self.performance_cache.keys().next().cloned() {
                self.performance_cache.remove(&oldest_key);
            }
        }
        
        self.performance_cache.insert(key, CachedPerformance {
        });
    fn calculate_cache_hit_rate(&self) -> f64 {
        let total_requests: usize = self.performance_cache.values()
            .map(|cached| cached.hit_count + 1)
            .sum();
        
        let cache_hits: usize = self.performance_cache.values()
            .map(|cached| cached.hit_count)
            .sum();
        
        if total_requests > 0 {
            (cache_hits as f64) / (total_requests as f64)
        } else {
            0.0
        }
    }
    
    fn trigger_model_update(&mut self) -> Result<()> {
        info!("Triggering ML model update");
        self.train_models()?;
        self.continuous_learner.mark_models_updated()?;
        Ok(())
    }
}

/// Compilation context for optimization decisions
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CompilationContext {
/// Optimization goals
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OptimizationGoal {
/// Resource constraints
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ResourceConstraints {
/// Usage patterns
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct UsagePatterns {
/// Deployment environment
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum DeploymentEnvironment {
/// Performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStatistics {
impl Default for CompilationContext {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            max_memory_usage: Some(4 * 1024 * 1024 * 1024), // 4GB
        }
    }
impl Default for UsagePatterns {
    fn default() -> Self {
        Self {
        }
    }
}
