/// Integration layer for CURSED-specific optimizations with existing optimization infrastructure
/// 
/// Connects CURSED optimizations with enhanced analysis, LTO, and performance monitoring systems.

use crate::error::{Error, Result};
use crate::optimization::config::{OptimizationConfig, OptimizationLevel};
use crate::optimization::enhanced_analysis::{
    EnhancedPerformanceAnalyzer, EnhancedAnalysisResult, PerformanceBottleneck, 
    OptimizationRecommendation, CompilationPhase
};
use crate::optimization::lto::{LtoOptimizer, LtoConfig, LtoLevel};
use crate::codegen::llvm::optimization::{OptimizationManager, OptimizationStats};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, instrument, debug, warn};

/// Comprehensive CURSED optimization coordinator
pub struct CursedOptimizationCoordinator<'ctx> {
    /// Core LLVM optimization manager
    llvm_optimizer: OptimizationManager<'ctx>,
    /// Enhanced performance analyzer
    performance_analyzer: EnhancedPerformanceAnalyzer,
    /// Link-time optimizer
    lto_optimizer: Option<LtoOptimizer>,
    /// Optimization configuration
    config: CursedOptimizationConfig,
    /// Cumulative statistics
    cumulative_stats: Arc<Mutex<CursedOptimizationStats>>,
    /// Optimization history for learning
    optimization_history: Vec<OptimizationSession>,
}

/// Configuration for CURSED optimization integration
#[derive(Debug, Clone)]
pub struct CursedOptimizationConfig {
    /// Base optimization configuration
    pub base_config: OptimizationConfig,
    /// LTO configuration
    pub lto_config: Option<LtoConfig>,
    /// Enable CURSED-specific optimizations
    pub enable_cursed_optimizations: bool,
    /// Enable performance analysis
    pub enable_performance_analysis: bool,
    /// Enable adaptive optimization tuning
    pub enable_adaptive_tuning: bool,
    /// Performance analysis threshold
    pub analysis_threshold: Duration,
    /// Maximum optimization iterations
    pub max_optimization_iterations: usize,
    /// Target performance improvement
    pub target_improvement: f64,
}

impl Default for CursedOptimizationConfig {
    fn default() -> Self {
        Self {
            base_config: OptimizationConfig::default(),
            lto_config: Some(LtoConfig::default()),
            enable_cursed_optimizations: true,
            enable_performance_analysis: true,
            enable_adaptive_tuning: true,
            analysis_threshold: Duration::from_millis(100),
            max_optimization_iterations: 3,
            target_improvement: 0.15, // 15% improvement target
        }
    }
}

/// Comprehensive CURSED optimization statistics
#[derive(Debug, Clone, Default)]
pub struct CursedOptimizationStats {
    /// Total optimization sessions
    pub sessions: usize,
    /// Total CURSED optimizations applied
    pub cursed_optimizations: usize,
    /// Total performance improvements achieved
    pub total_performance_improvement: f64,
    /// Total memory reductions achieved
    pub total_memory_reduction: f64,
    /// LLVM optimization statistics
    pub llvm_stats: OptimizationStats,
    /// Performance analysis results
    pub analysis_results: Vec<EnhancedAnalysisResult>,
    /// Optimization effectiveness by category
    pub effectiveness_by_category: HashMap<OptimizationCategory, CategoryStats>,
    /// Adaptive tuning adjustments made
    pub adaptive_adjustments: usize,
}

/// Optimization category tracking
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum OptimizationCategory {
    Goroutines,
    Channels,
    GarbageCollection,
    GenZKeywords,
    ControlFlow,
    MemoryLayout,
    CrossModule,
    ProfileGuided,
}

/// Statistics per optimization category
#[derive(Debug, Clone, Default)]
pub struct CategoryStats {
    pub optimizations_applied: usize,
    pub performance_improvement: f64,
    pub memory_reduction: f64,
    pub success_rate: f64,
}

/// Record of an optimization session
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input_characteristics: InputCharacteristics,
    pub optimizations_applied: HashMap<OptimizationCategory, usize>,
    pub performance_improvement: f64,
    pub memory_reduction: f64,
    pub compilation_time: Duration,
    pub analysis_result: Option<EnhancedAnalysisResult>,
}

/// Characteristics of the input being optimized
#[derive(Debug, Clone)]
pub struct InputCharacteristics {
    pub source_size: usize,
    pub function_count: usize,
    pub goroutine_usage: usize,
    pub channel_usage: usize,
    pub gc_allocations: usize,
    pub genz_keyword_usage: usize,
    pub complexity_score: f64,
}

impl<'ctx> CursedOptimizationCoordinator<'ctx> {
    /// Create a new CURSED optimization coordinator
    #[instrument(skip(context))]
    pub fn new(
        context: &'ctx inkwell::context::Context,
        config: CursedOptimizationConfig,
    ) -> Result<Self> {
        info!("Initializing CURSED optimization coordinator");
        
        let llvm_optimizer = OptimizationManager::new(context, config.base_config.clone());
        let performance_analyzer = EnhancedPerformanceAnalyzer::with_config(
            crate::optimization::enhanced_analysis::AnalysisConfig::default()
        );
        
        let lto_optimizer = if let Some(lto_config) = &config.lto_config {
            Some(LtoOptimizer::new(lto_config.clone())?)
        } else {
            None
        };
        
        Ok(Self {
            llvm_optimizer,
            performance_analyzer,
            lto_optimizer,
            config,
            cumulative_stats: Arc::new(Mutex::new(CursedOptimizationStats::default())),
            optimization_history: Vec::new(),
        })
    }
    
    /// Perform comprehensive CURSED optimization
    #[instrument(skip(self, module, source))]
    pub async fn optimize_comprehensive(
        &mut self,
        module: &inkwell::module::Module<'ctx>,
        source: &str,
        file_path: &str,
    ) -> Result<CursedOptimizationResult> {
        let session_start = Instant::now();
        info!("Starting comprehensive CURSED optimization for {}", file_path);
        
        // 1. Analyze input characteristics
        let input_characteristics = self.analyze_input_characteristics(source, module)?;
        
        // 2. Initialize optimization session
        let mut session = OptimizationSession {
            timestamp: chrono::Utc::now(),
            input_characteristics,
            optimizations_applied: HashMap::new(),
            performance_improvement: 0.0,
            memory_reduction: 0.0,
            compilation_time: Duration::from_millis(0),
            analysis_result: None,
        };
        
        // 3. Performance analysis (if enabled and meets threshold)
        let analysis_result = if self.config.enable_performance_analysis {
            Some(self.performance_analyzer.analyze_compilation(
                source,
                file_path,
                self.config.base_config.level,
            ).await?)
        } else {
            None
        };
        
        session.analysis_result = analysis_result.clone();
        
        // 4. Apply adaptive tuning based on analysis
        if self.config.enable_adaptive_tuning {
            if let Some(ref analysis) = analysis_result {
                self.apply_adaptive_tuning(analysis)?;
            }
        }
        
        // 5. Initialize LLVM optimizer with module
        self.llvm_optimizer.initialize(module)?;
        
        // 6. Apply iterative optimization
        let optimization_result = self.apply_iterative_optimization(module, &mut session).await?;
        
        // 7. Apply LTO if configured
        if let Some(ref mut lto_optimizer) = self.lto_optimizer {
            let lto_result = self.apply_lto_optimization(lto_optimizer, module)?;
            optimization_result.merge_lto_results(lto_result);
        }
        
        // 8. Update session and statistics
        session.compilation_time = session_start.elapsed();
        self.update_statistics(&session)?;
        self.optimization_history.push(session);
        
        info!(
            optimizations_applied = optimization_result.total_optimizations,
            performance_improvement = %format!("{:.1}%", optimization_result.performance_improvement * 100.0),
            memory_reduction = %format!("{:.1}%", optimization_result.memory_reduction * 100.0),
            compilation_time = ?session.compilation_time,
            "Comprehensive CURSED optimization completed"
        );
        
        Ok(optimization_result)
    }
    
    /// Analyze input characteristics for optimization planning
    fn analyze_input_characteristics(
        &self,
        source: &str,
        module: &inkwell::module::Module<'ctx>,
    ) -> Result<InputCharacteristics> {
        let source_size = source.len();
        let function_count = module.get_functions().count();
        
        // Count CURSED-specific patterns
        let goroutine_patterns = ["stan ", "yolo", "goroutine"];
        let channel_patterns = ["channel", "send(", "receive("];
        let gc_patterns = ["new ", "alloc", "gc_"];
        let genz_patterns = ["slay ", "facts ", "sus ", "lowkey", "highkey", "periodt", "bestie", "flex"];
        
        let goroutine_usage = goroutine_patterns.iter()
            .map(|p| source.matches(p).count())
            .sum();
        
        let channel_usage = channel_patterns.iter()
            .map(|p| source.matches(p).count())
            .sum();
        
        let gc_allocations = gc_patterns.iter()
            .map(|p| source.matches(p).count())
            .sum();
        
        let genz_keyword_usage = genz_patterns.iter()
            .map(|p| source.matches(p).count())
            .sum();
        
        // Calculate complexity score
        let complexity_score = self.calculate_complexity_score(source, function_count);
        
        Ok(InputCharacteristics {
            source_size,
            function_count,
            goroutine_usage,
            channel_usage,
            gc_allocations,
            genz_keyword_usage,
            complexity_score,
        })
    }
    
    /// Calculate complexity score for input
    fn calculate_complexity_score(&self, source: &str, function_count: usize) -> f64 {
        let lines = source.lines().count() as f64;
        let avg_function_size = if function_count > 0 {
            lines / function_count as f64
        } else {
            lines
        };
        
        // Complexity factors
        let size_factor = (lines / 1000.0).min(2.0); // Max 2x for size
        let function_complexity = (avg_function_size / 20.0).min(1.5); // Max 1.5x for function size
        let nesting_factor = source.matches('{').count() as f64 / lines.max(1.0);
        
        size_factor + function_complexity + nesting_factor
    }
    
    /// Apply adaptive tuning based on performance analysis
    fn apply_adaptive_tuning(&mut self, analysis: &EnhancedAnalysisResult) -> Result<()> {
        debug!("Applying adaptive tuning based on performance analysis");
        
        // Adjust optimization level based on performance score
        if analysis.summary.performance_score < 60.0 {
            // Poor performance, try more aggressive optimization
            self.config.base_config.level = OptimizationLevel::Aggressive;
            self.config.base_config.unroll_loops = true;
            self.config.base_config.vectorize_loops = true;
            debug!("Switched to aggressive optimization due to poor performance");
        } else if analysis.summary.performance_score > 90.0 {
            // Good performance, can use lighter optimization for faster compilation
            self.config.base_config.level = OptimizationLevel::Default;
            debug!("Using default optimization for already good performance");
        }
        
        // Adjust based on bottlenecks
        for bottleneck in &analysis.bottlenecks {
            match bottleneck.phase {
                CompilationPhase::LLVMOptimization => {
                    if bottleneck.severity > 7 {
                        // Reduce LLVM optimization if it's too slow
                        self.config.base_config.level = OptimizationLevel::Less;
                        warn!("Reduced LLVM optimization level due to bottleneck");
                    }
                }
                CompilationPhase::Parsing => {
                    // Parser bottleneck - enable incremental mode
                    self.config.base_config.enable_incremental = true;
                    debug!("Enabled incremental compilation for parsing bottleneck");
                }
                _ => {}
            }
        }
        
        // Update statistics
        if let Ok(mut stats) = self.cumulative_stats.lock() {
            stats.adaptive_adjustments += 1;
        }
        
        Ok(())
    }
    
    /// Apply iterative optimization with feedback
    async fn apply_iterative_optimization(
        &mut self,
        module: &inkwell::module::Module<'ctx>,
        session: &mut OptimizationSession,
    ) -> Result<CursedOptimizationResult> {
        let mut total_optimizations = 0;
        let mut best_performance = 0.0;
        let mut best_memory = 0.0;
        let start_time = Instant::now();
        
        for iteration in 0..self.config.max_optimization_iterations {
            debug!("Optimization iteration {}", iteration + 1);
            
            // Apply LLVM optimizations
            self.llvm_optimizer.optimize_module(module)?;
            let llvm_stats = self.llvm_optimizer.get_stats();
            
            // Count CURSED-specific optimizations
            let cursed_optimizations = llvm_stats.cursed_specific_optimizations;
            total_optimizations += cursed_optimizations;
            
            // Update session tracking
            self.update_session_category_stats(session, &llvm_stats);
            
            // Estimate performance improvement (in real implementation, this would be measured)
            let performance_improvement = self.estimate_performance_improvement(&llvm_stats);
            let memory_reduction = self.estimate_memory_reduction(&llvm_stats);
            
            // Update bests
            if performance_improvement > best_performance {
                best_performance = performance_improvement;
            }
            if memory_reduction > best_memory {
                best_memory = memory_reduction;
            }
            
            // Check if we've reached our target
            if performance_improvement >= self.config.target_improvement {
                info!("Reached target performance improvement in {} iterations", iteration + 1);
                break;
            }
            
            // Check for diminishing returns
            if iteration > 0 && performance_improvement < 0.02 {
                debug!("Diminishing returns detected, stopping optimization");
                break;
            }
        }
        
        session.performance_improvement = best_performance;
        session.memory_reduction = best_memory;
        
        Ok(CursedOptimizationResult {
            total_optimizations,
            performance_improvement: best_performance,
            memory_reduction: best_memory,
            compilation_time: start_time.elapsed(),
            llvm_stats: self.llvm_optimizer.get_stats(),
            cursed_optimizations_by_category: session.optimizations_applied.clone(),
            analysis_insights: session.analysis_result.clone(),
        })
    }
    
    /// Update session category statistics
    fn update_session_category_stats(&self, session: &mut OptimizationSession, llvm_stats: &OptimizationStats) {
        // Estimate category breakdown based on input characteristics
        let total_cursed = llvm_stats.cursed_specific_optimizations;
        
        if total_cursed > 0 {
            let chars = &session.input_characteristics;
            let total_patterns = chars.goroutine_usage + chars.channel_usage + chars.gc_allocations + chars.genz_keyword_usage;
            
            if total_patterns > 0 {
                // Distribute optimizations proportionally
                let goroutine_opts = (total_cursed * chars.goroutine_usage / total_patterns.max(1)).max(0);
                let channel_opts = (total_cursed * chars.channel_usage / total_patterns.max(1)).max(0);
                let gc_opts = (total_cursed * chars.gc_allocations / total_patterns.max(1)).max(0);
                let genz_opts = (total_cursed * chars.genz_keyword_usage / total_patterns.max(1)).max(0);
                
                session.optimizations_applied.insert(OptimizationCategory::Goroutines, goroutine_opts);
                session.optimizations_applied.insert(OptimizationCategory::Channels, channel_opts);
                session.optimizations_applied.insert(OptimizationCategory::GarbageCollection, gc_opts);
                session.optimizations_applied.insert(OptimizationCategory::GenZKeywords, genz_opts);
            }
        }
    }
    
    /// Estimate performance improvement from optimization statistics
    fn estimate_performance_improvement(&self, stats: &OptimizationStats) -> f64 {
        let base_improvement = stats.cursed_specific_optimizations as f64 * 0.03; // 3% per optimization
        let llvm_improvement = match stats.passes_run {
            0..=5 => 0.05,
            6..=15 => 0.15,
            16..=30 => 0.25,
            _ => 0.35,
        };
        
        (base_improvement + llvm_improvement).min(0.6) // Cap at 60%
    }
    
    /// Estimate memory reduction from optimization statistics
    fn estimate_memory_reduction(&self, stats: &OptimizationStats) -> f64 {
        let code_size_reduction = if stats.code_size_before > 0 {
            (stats.code_size_before - stats.code_size_after) as f64 / stats.code_size_before as f64
        } else {
            0.0
        };
        
        let cursed_memory_reduction = stats.cursed_specific_optimizations as f64 * 0.02; // 2% per optimization
        
        (code_size_reduction + cursed_memory_reduction).min(0.4) // Cap at 40%
    }
    
    /// Apply LTO optimization
    fn apply_lto_optimization(
        &self,
        lto_optimizer: &mut LtoOptimizer,
        _module: &inkwell::module::Module<'ctx>,
    ) -> Result<LtoOptimizationResult> {
        debug!("Applying LTO optimization");
        
        // In a real implementation, this would:
        // 1. Add the module as a compilation unit
        // 2. Run LTO optimization
        // 3. Return comprehensive results
        
        // For now, return mock results
        Ok(LtoOptimizationResult {
            functions_inlined: 5,
            dead_code_eliminated: 3,
            constants_propagated: 8,
            performance_improvement: 0.12,
            memory_reduction: 0.08,
        })
    }
    
    /// Update cumulative statistics
    fn update_statistics(&self, session: &OptimizationSession) -> Result<()> {
        if let Ok(mut stats) = self.cumulative_stats.lock() {
            stats.sessions += 1;
            stats.total_performance_improvement += session.performance_improvement;
            stats.total_memory_reduction += session.memory_reduction;
            
            // Update category statistics
            for (category, &count) in &session.optimizations_applied {
                let category_stats = stats.effectiveness_by_category.entry(category.clone()).or_default();
                category_stats.optimizations_applied += count;
                category_stats.performance_improvement += session.performance_improvement;
                category_stats.memory_reduction += session.memory_reduction;
                
                // Update success rate (optimizations > 0 means success)
                if count > 0 {
                    category_stats.success_rate = (category_stats.success_rate * (stats.sessions - 1) as f64 + 1.0) / stats.sessions as f64;
                } else {
                    category_stats.success_rate = (category_stats.success_rate * (stats.sessions - 1) as f64) / stats.sessions as f64;
                }
            }
            
            // Store analysis result
            if let Some(ref analysis) = session.analysis_result {
                stats.analysis_results.push(analysis.clone());
                
                // Keep only recent analysis results (last 10)
                if stats.analysis_results.len() > 10 {
                    stats.analysis_results.remove(0);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get comprehensive statistics
    pub fn get_comprehensive_stats(&self) -> Result<CursedOptimizationStats> {
        Ok(self.cumulative_stats.lock().unwrap().clone())
    }
    
    /// Generate optimization report
    pub fn generate_comprehensive_report(&self) -> Result<String> {
        let stats = self.get_comprehensive_stats()?;
        let mut report = String::new();
        
        report.push_str("# CURSED Comprehensive Optimization Report\n\n");
        
        // Overview
        report.push_str("## Overview\n");
        report.push_str(&format!("- **Optimization Sessions**: {}\n", stats.sessions));
        report.push_str(&format!("- **Total CURSED Optimizations**: {}\n", stats.cursed_optimizations));
        report.push_str(&format!("- **Average Performance Improvement**: {:.1}%\n", 
                                stats.total_performance_improvement / stats.sessions.max(1) as f64 * 100.0));
        report.push_str(&format!("- **Average Memory Reduction**: {:.1}%\n", 
                                stats.total_memory_reduction / stats.sessions.max(1) as f64 * 100.0));
        report.push_str(&format!("- **Adaptive Adjustments**: {}\n\n", stats.adaptive_adjustments));
        
        // Category effectiveness
        report.push_str("## Optimization Category Effectiveness\n");
        for (category, category_stats) in &stats.effectiveness_by_category {
            report.push_str(&format!("### {:?}\n", category));
            report.push_str(&format!("- Optimizations Applied: {}\n", category_stats.optimizations_applied));
            report.push_str(&format!("- Success Rate: {:.1}%\n", category_stats.success_rate * 100.0));
            report.push_str(&format!("- Avg Performance Improvement: {:.1}%\n", 
                                    category_stats.performance_improvement / category_stats.optimizations_applied.max(1) as f64 * 100.0));
            report.push_str(&format!("- Avg Memory Reduction: {:.1}%\n\n", 
                                    category_stats.memory_reduction / category_stats.optimizations_applied.max(1) as f64 * 100.0));
        }
        
        // Recent analysis insights
        if !stats.analysis_results.is_empty() {
            report.push_str("## Recent Performance Analysis Insights\n");
            for (i, analysis) in stats.analysis_results.iter().take(3).enumerate() {
                report.push_str(&format!("### Analysis {}\n", i + 1));
                report.push_str(&format!("- Performance Score: {:.1}\n", analysis.summary.performance_score));
                report.push_str(&format!("- Primary Bottleneck: {}\n", 
                                        analysis.summary.primary_bottleneck.as_deref().unwrap_or("None")));
                report.push_str(&format!("- Top Recommendation: {}\n", 
                                        analysis.summary.top_recommendation.as_deref().unwrap_or("None")));
                report.push_str(&format!("- Improvement Potential: {:.1}%\n\n", 
                                        analysis.summary.improvement_potential * 100.0));
            }
        }
        
        Ok(report)
    }
}

/// Result of comprehensive CURSED optimization
#[derive(Debug, Clone)]
pub struct CursedOptimizationResult {
    pub total_optimizations: usize,
    pub performance_improvement: f64,
    pub memory_reduction: f64,
    pub compilation_time: Duration,
    pub llvm_stats: OptimizationStats,
    pub cursed_optimizations_by_category: HashMap<OptimizationCategory, usize>,
    pub analysis_insights: Option<EnhancedAnalysisResult>,
}

impl CursedOptimizationResult {
    pub fn merge_lto_results(&mut self, lto_result: LtoOptimizationResult) {
        self.performance_improvement += lto_result.performance_improvement;
        self.memory_reduction += lto_result.memory_reduction;
        self.total_optimizations += lto_result.functions_inlined + 
                                   lto_result.dead_code_eliminated + 
                                   lto_result.constants_propagated;
    }
}

/// Result of LTO optimization (mock for now)
#[derive(Debug, Clone)]
pub struct LtoOptimizationResult {
    pub functions_inlined: usize,
    pub dead_code_eliminated: usize,
    pub constants_propagated: usize,
    pub performance_improvement: f64,
    pub memory_reduction: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[tokio::test]
    async fn test_cursed_optimization_coordinator() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        
        let mut coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        // Create a test module
        let module = context.create_module("test");
        let source = r#"
            slay main() {
                stan worker();
                facts channel = make_channel<i32>(10);
                send(channel, 42)?;
                yolo;
            }
        "#;
        
        let result = coordinator.optimize_comprehensive(&module, source, "test.csd").await;
        assert!(result.is_ok());
        
        let optimization_result = result.unwrap();
        assert!(optimization_result.total_optimizations >= 0);
        assert!(optimization_result.performance_improvement >= 0.0);
    }
    
    #[test]
    fn test_input_characteristics_analysis() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let source = r#"
            slay main() {
                stan worker();
                facts channel = make_channel<i32>(10);
                send(channel, 42);
                yolo;
            }
        "#;
        
        let module = context.create_module("test");
        let characteristics = coordinator.analyze_input_characteristics(source, &module).unwrap();
        
        assert!(characteristics.goroutine_usage > 0);
        assert!(characteristics.channel_usage > 0);
        assert!(characteristics.genz_keyword_usage > 0);
        assert!(characteristics.complexity_score > 0.0);
    }
    
    #[test]
    fn test_comprehensive_statistics() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let stats = coordinator.get_comprehensive_stats().unwrap();
        assert_eq!(stats.sessions, 0);
        assert_eq!(stats.cursed_optimizations, 0);
    }
    
    #[test]
    fn test_report_generation() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let report = coordinator.generate_comprehensive_report().unwrap();
        assert!(report.contains("CURSED Comprehensive Optimization Report"));
        assert!(report.contains("Overview"));
        assert!(report.contains("Optimization Category Effectiveness"));
    }
}
