//! Profile analysis for CURSED PGO system

use crate::error::{CursedError, Result};
use crate::optimization::pgo::profile_collector::ProfileData;
use crate::optimization::pgo::{
    OptimizationRecommendations, FunctionOptimizationRecommendation, 
    LoopOptimizationRecommendation, InliningRecommendation, 
    MemoryOptimizationRecommendation, FunctionOptimizationType,
    LoopOptimizationType, MemoryOptimizationType, RecommendationPriority
};
use std::collections::HashMap;
use std::time::Duration;

/// Configuration for profile analysis
#[derive(Debug, Clone)]
pub struct ProfileAnalysisConfig {
    pub hot_threshold: f64,
    pub cold_threshold: f64,
    pub inlining_threshold: f64,
    pub optimization_aggressiveness: f64,
    pub min_execution_count: u64,
    pub enable_loop_analysis: bool,
    pub enable_memory_analysis: bool,
}

impl Default for ProfileAnalysisConfig {
    fn default() -> Self {
        Self {
            hot_threshold: 0.1,       // 10% of total execution
            cold_threshold: 0.01,     // 1% of total execution
            inlining_threshold: 0.05, // 5% of total calls
            optimization_aggressiveness: 0.7,
            min_execution_count: 100,
            enable_loop_analysis: true,
            enable_memory_analysis: true,
        }
    }
}

/// Profile analyzer for optimization recommendations
pub struct ProfileAnalyzer {
    config: ProfileAnalysisConfig,
}

/// Analysis results
#[derive(Debug, Clone)]
pub struct ProfileAnalysis {
    pub hot_functions: Vec<HotFunction>,
    pub cold_functions: Vec<ColdFunction>,
    pub hot_paths: Vec<HotPath>,
    pub loop_patterns: Vec<LoopPattern>,
    pub memory_patterns: Vec<MemoryPattern>,
    pub call_graph: CallGraph,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    pub analysis_quality: f64,
    pub confidence_score: f64,
}

/// Hot function analysis
#[derive(Debug, Clone)]
pub struct HotFunction {
    pub name: String,
    pub execution_count: u64,
    pub execution_percentage: f64,
    pub average_execution_time: Duration,
    pub total_execution_time: Duration,
    pub optimization_potential: f64,
}

/// Cold function analysis
#[derive(Debug, Clone)]
pub struct ColdFunction {
    pub name: String,
    pub execution_count: u64,
    pub execution_percentage: f64,
    pub total_execution_time: Duration,
    pub size_estimate: u64,
}

/// Hot execution path
#[derive(Debug, Clone)]
pub struct HotPath {
    pub path_id: String,
    pub functions: Vec<String>,
    pub execution_count: u64,
    pub execution_percentage: f64,
    pub total_time: Duration,
}

/// Loop pattern analysis
#[derive(Debug, Clone)]
pub struct LoopPattern {
    pub loop_id: String,
    pub function_name: String,
    pub iteration_count: u64,
    pub execution_count: u64,
    pub unroll_potential: f64,
    pub vectorization_potential: f64,
}

/// Memory usage pattern
#[derive(Debug, Clone)]
pub struct MemoryPattern {
    pub pattern_id: String,
    pub function_name: String,
    pub allocation_count: u64,
    pub total_allocated: u64,
    pub access_pattern: String,
    pub locality_score: f64,
}

/// Call graph representation
#[derive(Debug, Clone)]
pub struct CallGraph {
    pub nodes: HashMap<String, CallNode>,
    pub edges: Vec<CallEdge>,
}

/// Call graph node
#[derive(Debug, Clone)]
pub struct CallNode {
    pub function_name: String,
    pub execution_count: u64,
    pub total_time: Duration,
    pub self_time: Duration,
}

/// Call graph edge
#[derive(Debug, Clone)]
pub struct CallEdge {
    pub caller: String,
    pub callee: String,
    pub call_count: u64,
    pub call_percentage: f64,
}

/// Optimization opportunity
#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    pub opportunity_id: String,
    pub opportunity_type: String,
    pub target_function: String,
    pub estimated_impact: f64,
    pub confidence: f64,
    pub priority: RecommendationPriority,
    pub description: String,
}

impl ProfileAnalyzer {
    /// Create a new profile analyzer
    pub fn new(config: ProfileAnalysisConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Analyze profile data and generate optimization recommendations
    pub fn analyze_profile(&self, profile_data: &ProfileData) -> Result<ProfileAnalysis> {
        if !profile_data.is_sufficient_for_optimization() {
            return Err(CursedError::General("Insufficient profile data for analysis".to_string()));
        }

        let hot_functions = self.analyze_hot_functions(profile_data)?;
        let cold_functions = self.analyze_cold_functions(profile_data)?;
        let hot_paths = self.analyze_hot_paths(profile_data)?;
        let loop_patterns = if self.config.enable_loop_analysis {
            self.analyze_loop_patterns(profile_data)?
        } else {
            Vec::new()
        };
        let memory_patterns = if self.config.enable_memory_analysis {
            self.analyze_memory_patterns(profile_data)?
        } else {
            Vec::new()
        };
        let call_graph = self.build_call_graph(profile_data)?;
        let optimization_opportunities = self.identify_optimization_opportunities(&hot_functions, &cold_functions)?;
        
        let analysis_quality = self.calculate_analysis_quality(profile_data);
        let confidence_score = self.calculate_confidence_score(profile_data);

        Ok(ProfileAnalysis {
            hot_functions,
            cold_functions,
            hot_paths,
            loop_patterns,
            memory_patterns,
            call_graph,
            optimization_opportunities,
            analysis_quality,
            confidence_score,
        })
    }

    /// Generate optimization recommendations based on analysis
    pub fn generate_recommendations(&self, analysis: &ProfileAnalysis) -> Result<OptimizationRecommendations> {
        let mut recommendations = OptimizationRecommendations::new();

        // Function-level recommendations
        for hot_func in &analysis.hot_functions {
            if hot_func.optimization_potential > self.config.hot_threshold {
                let recommendation = FunctionOptimizationRecommendation {
                    function_name: hot_func.name.clone(),
                    recommendation_type: FunctionOptimizationType::Specialization,
                    priority: if hot_func.execution_percentage > 0.2 {
                        RecommendationPriority::High
                    } else {
                        RecommendationPriority::Medium
                    },
                    estimated_impact: hot_func.optimization_potential,
                    reasoning: format!("Hot function with {:.1}% execution time", 
                                     hot_func.execution_percentage * 100.0),
                };
                recommendations.add_function_recommendation(recommendation);
            }
        }

        // Loop-level recommendations
        for loop_pattern in &analysis.loop_patterns {
            if loop_pattern.unroll_potential > 0.5 {
                let recommendation = LoopOptimizationRecommendation {
                    loop_id: loop_pattern.loop_id.clone(),
                    optimization_type: LoopOptimizationType::Unrolling,
                    priority: RecommendationPriority::Medium,
                    estimated_speedup: loop_pattern.unroll_potential,
                    details: format!("Loop with {} iterations, {} executions", 
                                   loop_pattern.iteration_count, loop_pattern.execution_count),
                };
                recommendations.add_loop_recommendation(recommendation);
            }
        }

        // Inlining recommendations
        for edge in &analysis.call_graph.edges {
            if edge.call_percentage > self.config.inlining_threshold {
                let recommendation = InliningRecommendation {
                    caller_function: edge.caller.clone(),
                    callee_function: edge.callee.clone(),
                    should_inline: true,
                    confidence: edge.call_percentage,
                    justification: format!("Frequent call: {:.1}% of total calls", 
                                         edge.call_percentage * 100.0),
                };
                recommendations.add_inlining_recommendation(recommendation);
            }
        }

        // Memory optimization recommendations
        for memory_pattern in &analysis.memory_patterns {
            if memory_pattern.locality_score < 0.5 {
                let recommendation = MemoryOptimizationRecommendation {
                    location: memory_pattern.function_name.clone(),
                    optimization_type: MemoryOptimizationType::Locality,
                    priority: RecommendationPriority::Medium,
                    estimated_savings: (1.0 - memory_pattern.locality_score) * 100.0,
                    details: format!("Poor memory locality: {:.1}% efficiency", 
                                   memory_pattern.locality_score * 100.0),
                };
                recommendations.add_memory_recommendation(recommendation);
            }
        }

        // Set overall recommendation strategy
        recommendations.strategy_recommendation = self.determine_optimization_strategy(analysis);
        recommendations.expected_improvement = self.estimate_overall_improvement(analysis);
        recommendations.confidence_level = analysis.confidence_score;

        Ok(recommendations)
    }

    /// Analyze hot functions
    fn analyze_hot_functions(&self, profile_data: &ProfileData) -> Result<Vec<HotFunction>> {
        let total_execution: u64 = profile_data.counters.values().sum();
        let mut hot_functions = Vec::new();

        for (function_name, &count) in &profile_data.counters {
            if function_name.ends_with("_time") {
                continue;
            }

            let execution_percentage = count as f64 / total_execution as f64;
            
            if execution_percentage > self.config.hot_threshold && count >= self.config.min_execution_count {
                let total_time = profile_data.get_function_time(function_name)
                    .unwrap_or_else(|| Duration::from_millis(count)); // Fallback estimate
                let average_time = total_time / count as u32;
                
                let hot_function = HotFunction {
                    name: function_name.clone(),
                    execution_count: count,
                    execution_percentage,
                    average_execution_time: average_time,
                    total_execution_time: total_time,
                    optimization_potential: execution_percentage * self.config.optimization_aggressiveness,
                };
                hot_functions.push(hot_function);
            }
        }

        hot_functions.sort_by(|a, b| b.execution_percentage.partial_cmp(&a.execution_percentage).unwrap());
        Ok(hot_functions)
    }

    /// Analyze cold functions
    fn analyze_cold_functions(&self, profile_data: &ProfileData) -> Result<Vec<ColdFunction>> {
        let total_execution: u64 = profile_data.counters.values().sum();
        let mut cold_functions = Vec::new();

        for (function_name, &count) in &profile_data.counters {
            if function_name.ends_with("_time") {
                continue;
            }

            let execution_percentage = count as f64 / total_execution as f64;
            
            if execution_percentage < self.config.cold_threshold {
                let total_time = profile_data.get_function_time(function_name)
                    .unwrap_or_else(|| Duration::from_millis(count));
                
                let cold_function = ColdFunction {
                    name: function_name.clone(),
                    execution_count: count,
                    execution_percentage,
                    total_execution_time: total_time,
                    size_estimate: function_name.len() as u64 * 10, // Rough estimate
                };
                cold_functions.push(cold_function);
            }
        }

        Ok(cold_functions)
    }

    /// Analyze hot execution paths
    fn analyze_hot_paths(&self, profile_data: &ProfileData) -> Result<Vec<HotPath>> {
        // Simplified implementation - in practice this would analyze call sequences
        let mut hot_paths = Vec::new();
        let total_execution: u64 = profile_data.counters.values().sum();

        for (function_name, &count) in &profile_data.counters {
            if function_name.ends_with("_time") {
                continue;
            }

            let execution_percentage = count as f64 / total_execution as f64;
            
            if execution_percentage > self.config.hot_threshold {
                let total_time = profile_data.get_function_time(function_name)
                    .unwrap_or_else(|| Duration::from_millis(count));
                
                let hot_path = HotPath {
                    path_id: format!("path_{}", function_name),
                    functions: vec![function_name.clone()],
                    execution_count: count,
                    execution_percentage,
                    total_time,
                };
                hot_paths.push(hot_path);
            }
        }

        Ok(hot_paths)
    }

    /// Analyze loop patterns (simplified)
    fn analyze_loop_patterns(&self, _profile_data: &ProfileData) -> Result<Vec<LoopPattern>> {
        // Simplified implementation - would need actual loop profiling data
        Ok(vec![])
    }

    /// Analyze memory patterns (simplified)
    fn analyze_memory_patterns(&self, _profile_data: &ProfileData) -> Result<Vec<MemoryPattern>> {
        // Simplified implementation - would need actual memory profiling data
        Ok(vec![])
    }

    /// Build call graph from profile data
    fn build_call_graph(&self, profile_data: &ProfileData) -> Result<CallGraph> {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();

        // Build nodes
        for (function_name, &count) in &profile_data.counters {
            if function_name.ends_with("_time") {
                continue;
            }

            let total_time = profile_data.get_function_time(function_name)
                .unwrap_or_else(|| Duration::from_millis(count));
            
            let node = CallNode {
                function_name: function_name.clone(),
                execution_count: count,
                total_time,
                self_time: total_time, // Simplified - would need actual self time
            };
            nodes.insert(function_name.clone(), node);
        }

        // Build edges (simplified - would need actual call relationships)
        let total_calls: u64 = profile_data.counters.values().sum();
        for (function_name, &count) in &profile_data.counters {
            if function_name.ends_with("_time") {
                continue;
            }

            let call_percentage = count as f64 / total_calls as f64;
            if call_percentage > 0.01 { // Only significant calls
                let edge = CallEdge {
                    caller: "main".to_string(), // Simplified
                    callee: function_name.clone(),
                    call_count: count,
                    call_percentage,
                };
                edges.push(edge);
            }
        }

        Ok(CallGraph { nodes, edges })
    }

    /// Identify optimization opportunities
    fn identify_optimization_opportunities(&self, hot_functions: &[HotFunction], cold_functions: &[ColdFunction]) -> Result<Vec<OptimizationOpportunity>> {
        let mut opportunities = Vec::new();

        // Hot function optimization opportunities
        for hot_func in hot_functions {
            if hot_func.optimization_potential > 0.1 {
                let opportunity = OptimizationOpportunity {
                    opportunity_id: format!("opt_hot_{}", hot_func.name),
                    opportunity_type: "function_specialization".to_string(),
                    target_function: hot_func.name.clone(),
                    estimated_impact: hot_func.optimization_potential,
                    confidence: 0.8,
                    priority: RecommendationPriority::High,
                    description: format!("Specialize hot function with {:.1}% execution time", 
                                       hot_func.execution_percentage * 100.0),
                };
                opportunities.push(opportunity);
            }
        }

        // Cold function optimization opportunities
        for cold_func in cold_functions {
            if cold_func.size_estimate > 1000 {
                let opportunity = OptimizationOpportunity {
                    opportunity_id: format!("opt_cold_{}", cold_func.name),
                    opportunity_type: "function_outlining".to_string(),
                    target_function: cold_func.name.clone(),
                    estimated_impact: 0.02, // Small impact but reduces code size
                    confidence: 0.6,
                    priority: RecommendationPriority::Low,
                    description: format!("Outline cold function to reduce code size"),
                };
                opportunities.push(opportunity);
            }
        }

        Ok(opportunities)
    }

    /// Calculate analysis quality
    fn calculate_analysis_quality(&self, profile_data: &ProfileData) -> f64 {
        let sample_factor = (profile_data.total_samples as f64 / 1000.0).min(1.0);
        let function_factor = (profile_data.total_functions as f64 / 50.0).min(1.0);
        let duration_factor = profile_data.collection_duration
            .map(|d| (d.as_secs() as f64 / 60.0).min(1.0))
            .unwrap_or(0.5);
        
        (sample_factor + function_factor + duration_factor) / 3.0
    }

    /// Calculate confidence score
    fn calculate_confidence_score(&self, profile_data: &ProfileData) -> f64 {
        if profile_data.total_samples < 100 {
            0.3
        } else if profile_data.total_samples < 1000 {
            0.6
        } else {
            0.9
        }
    }

    /// Determine optimization strategy
    fn determine_optimization_strategy(&self, analysis: &ProfileAnalysis) -> String {
        if analysis.hot_functions.len() > 10 {
            "aggressive_optimization".to_string()
        } else if analysis.hot_functions.len() > 5 {
            "moderate_optimization".to_string()
        } else {
            "conservative_optimization".to_string()
        }
    }

    /// Estimate overall improvement
    fn estimate_overall_improvement(&self, analysis: &ProfileAnalysis) -> f64 {
        let hot_improvement: f64 = analysis.hot_functions.iter()
            .map(|f| f.optimization_potential)
            .sum();
        
        (hot_improvement * 0.6).min(0.5) // Cap at 50% improvement
    }
}

impl Default for ProfileAnalyzer {
    fn default() -> Self {
        Self::new(ProfileAnalysisConfig::default()).unwrap()
    }
}
