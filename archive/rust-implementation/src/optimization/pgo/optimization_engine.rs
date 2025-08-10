//! Profile-Guided Optimization Engine for CURSED compiler
//! 
//! This module implements the core optimization algorithms that use profile data
//! to make intelligent optimization decisions, including hot function identification,
//! branch prediction optimization, and memory access pattern optimization.

use crate::error::{CursedError, Result};
use crate::optimization::pgo::{
    ProfileData, ProfileAnalysis, OptimizationRecommendations,
    OptimizationType, FunctionOptimizationRecommendation, 
    LoopOptimizationRecommendation, InliningRecommendation,
    MemoryOptimizationRecommendation, RecommendationPriority,
    FunctionOptimizationType, LoopOptimizationType, MemoryOptimizationType
};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Hot function identification engine
#[derive(Debug, Clone)]
pub struct HotFunctionEngine {
    pub hot_threshold: f64,
    pub cold_threshold: f64,
    pub min_execution_count: u64,
    pub aggressive_threshold: f64,
}

impl Default for HotFunctionEngine {
    fn default() -> Self {
        Self {
            hot_threshold: 0.05,      // 5% of total execution time
            cold_threshold: 0.001,    // 0.1% of total execution time
            min_execution_count: 50,  // Minimum calls to consider
            aggressive_threshold: 0.15, // 15% for aggressive optimization
        }
    }
}

impl HotFunctionEngine {
    /// Identify hot functions from profile data
    pub fn identify_hot_functions(&self, profile_data: &ProfileData) -> Result<Vec<HotFunctionInfo>> {
        let total_execution_time = self.calculate_total_execution_time(profile_data);
        let mut hot_functions = Vec::new();

        for (function_name, &execution_count) in &profile_data.function_counts {
            if execution_count < self.min_execution_count {
                continue;
            }

            let execution_time = profile_data.get_function_time(function_name)
                .unwrap_or_else(|| Duration::from_millis(execution_count));
            
            let time_percentage = execution_time.as_nanos() as f64 / total_execution_time.as_nanos() as f64;
            
            if time_percentage >= self.hot_threshold {
                let optimization_level = if time_percentage >= self.aggressive_threshold {
                    OptimizationLevel::Aggressive
                } else {
                    OptimizationLevel::Moderate
                };

                let hot_function = HotFunctionInfo {
                    name: function_name.clone(),
                    execution_count,
                    execution_time,
                    time_percentage,
                    optimization_level,
                    optimization_potential: self.calculate_optimization_potential(time_percentage),
                    inlining_candidate: self.is_inlining_candidate(function_name, execution_count, &execution_time),
                    specialization_candidate: self.is_specialization_candidate(time_percentage),
                };
                hot_functions.push(hot_function);
            }
        }

        // Sort by execution time percentage (hottest first)
        hot_functions.sort_by(|a, b| b.time_percentage.partial_cmp(&a.time_percentage).unwrap());
        
        tracing::info!("Identified {} hot functions", hot_functions.len());
        Ok(hot_functions)
    }

    /// Calculate total execution time across all functions
    fn calculate_total_execution_time(&self, profile_data: &ProfileData) -> Duration {
        profile_data.function_counts.iter()
            .map(|(name, &count)| {
                profile_data.get_function_time(name)
                    .unwrap_or_else(|| Duration::from_millis(count))
            })
            .fold(Duration::ZERO, |acc, time| acc + time)
    }

    /// Calculate optimization potential based on execution time percentage
    fn calculate_optimization_potential(&self, time_percentage: f64) -> f64 {
        // Logarithmic scaling: more hot = more potential
        (time_percentage / self.hot_threshold).log2().max(1.0) * 0.3
    }

    /// Determine if function is a good inlining candidate
    fn is_inlining_candidate(&self, function_name: &str, execution_count: u64, execution_time: &Duration) -> bool {
        // Small functions with high call frequency are good inlining candidates
        let avg_execution_time = execution_time.as_nanos() / execution_count as u128;
        let is_small = avg_execution_time < 100_000; // Less than 0.1ms average
        let is_frequent = execution_count > 1000;
        let not_recursive = !function_name.contains("recursive");
        
        is_small && is_frequent && not_recursive
    }

    /// Determine if function is a good specialization candidate
    fn is_specialization_candidate(&self, time_percentage: f64) -> bool {
        time_percentage >= self.aggressive_threshold
    }
}

/// Branch prediction optimization engine
#[derive(Debug, Clone)]
pub struct BranchPredictionEngine {
    pub prediction_threshold: f64,
    pub misprediction_penalty: f64,
    pub enable_profile_guided_prediction: bool,
}

impl Default for BranchPredictionEngine {
    fn default() -> Self {
        Self {
            prediction_threshold: 0.8,    // 80% prediction accuracy threshold
            misprediction_penalty: 10.0,  // Cycles penalty for misprediction
            enable_profile_guided_prediction: true,
        }
    }
}

impl BranchPredictionEngine {
    /// Analyze branch patterns and generate optimization recommendations
    pub fn analyze_branch_patterns(&self, profile_data: &ProfileData) -> Result<Vec<BranchOptimization>> {
        let mut optimizations = Vec::new();

        // Analyze basic block transitions (simplified implementation)
        for (block_name, &execution_count) in &profile_data.basic_block_counts {
            if execution_count < 100 {
                continue; // Skip rarely executed blocks
            }

            // Find related edge counts to determine branch patterns
            let related_edges: Vec<_> = profile_data.edge_counts.iter()
                .filter(|(edge_name, _)| edge_name.starts_with(block_name))
                .collect();

            if related_edges.len() >= 2 {
                let total_edge_count: u64 = related_edges.iter().map(|(_, &count)| count).sum();
                
                for (edge_name, &edge_count) in related_edges {
                    let branch_probability = edge_count as f64 / total_edge_count as f64;
                    
                    if branch_probability > self.prediction_threshold || branch_probability < (1.0 - self.prediction_threshold) {
                        let optimization = BranchOptimization {
                            block_name: block_name.clone(),
                            edge_name: edge_name.clone(),
                            branch_probability,
                            is_predictable: true,
                            optimization_type: if branch_probability > 0.5 {
                                BranchOptimizationType::LikelyBranch
                            } else {
                                BranchOptimizationType::UnlikelyBranch
                            },
                            estimated_speedup: self.calculate_branch_speedup(branch_probability),
                        };
                        optimizations.push(optimization);
                    }
                }
            }
        }

        tracing::info!("Identified {} branch optimization opportunities", optimizations.len());
        Ok(optimizations)
    }

    /// Calculate expected speedup from branch prediction optimization
    fn calculate_branch_speedup(&self, probability: f64) -> f64 {
        let accuracy = probability.max(1.0 - probability);
        let misprediction_rate = 1.0 - accuracy;
        let current_penalty = misprediction_rate * self.misprediction_penalty;
        let optimized_penalty = (misprediction_rate * 0.5) * self.misprediction_penalty;
        
        (current_penalty - optimized_penalty) / current_penalty
    }
}

/// Memory access pattern optimization engine
#[derive(Debug, Clone)]
pub struct MemoryOptimizationEngine {
    pub cache_line_size: usize,
    pub locality_threshold: f64,
    pub prefetch_distance: usize,
}

impl Default for MemoryOptimizationEngine {
    fn default() -> Self {
        Self {
            cache_line_size: 64,      // Typical cache line size
            locality_threshold: 0.7,  // 70% locality required for optimization
            prefetch_distance: 4,     // Prefetch 4 cache lines ahead
        }
    }
}

impl MemoryOptimizationEngine {
    /// Analyze memory access patterns and generate optimization recommendations
    pub fn analyze_memory_patterns(&self, profile_data: &ProfileData) -> Result<Vec<MemoryOptimization>> {
        let mut optimizations = Vec::new();

        // Analyze function-level memory usage patterns
        for (function_name, &execution_count) in &profile_data.function_counts {
            if execution_count < 50 {
                continue; // Skip infrequently called functions
            }

            let memory_usage = self.estimate_memory_usage(function_name, execution_count);
            let access_pattern = self.analyze_access_pattern(function_name);
            let locality_score = self.calculate_locality_score(&access_pattern);

            if locality_score < self.locality_threshold {
                let optimization = MemoryOptimization {
                    function_name: function_name.clone(),
                    access_pattern: access_pattern.clone(),
                    locality_score,
                    memory_usage,
                    optimization_type: self.determine_memory_optimization_type(&access_pattern, locality_score),
                    estimated_improvement: self.calculate_memory_improvement(locality_score),
                    recommendations: self.generate_memory_recommendations(&access_pattern, locality_score),
                };
                optimizations.push(optimization);
            }
        }

        tracing::info!("Identified {} memory optimization opportunities", optimizations.len());
        Ok(optimizations)
    }

    /// Estimate memory usage for a function
    fn estimate_memory_usage(&self, function_name: &str, execution_count: u64) -> MemoryUsage {
        // Simplified estimation based on function name patterns
        let estimated_allocations = if function_name.contains("alloc") || function_name.contains("new") {
            execution_count * 2
        } else if function_name.contains("process") || function_name.contains("parse") {
            execution_count / 2
        } else {
            execution_count / 10
        };

        let estimated_bytes_per_call = if function_name.contains("large") || function_name.contains("buffer") {
            8192
        } else if function_name.contains("small") || function_name.contains("util") {
            256
        } else {
            1024
        };

        MemoryUsage {
            total_allocations: estimated_allocations,
            total_bytes: estimated_allocations * estimated_bytes_per_call,
            peak_usage: estimated_bytes_per_call * 2,
            average_usage: estimated_bytes_per_call,
        }
    }

    /// Analyze memory access pattern for a function
    fn analyze_access_pattern(&self, function_name: &str) -> AccessPattern {
        // Simplified pattern analysis based on function name
        if function_name.contains("sequential") || function_name.contains("iterate") {
            AccessPattern::Sequential
        } else if function_name.contains("random") || function_name.contains("hash") {
            AccessPattern::Random
        } else if function_name.contains("stride") || function_name.contains("step") {
            AccessPattern::Strided { stride: 8 }
        } else {
            AccessPattern::Mixed
        }
    }

    /// Calculate memory locality score
    fn calculate_locality_score(&self, pattern: &AccessPattern) -> f64 {
        match pattern {
            AccessPattern::Sequential => 0.95,
            AccessPattern::Strided { stride } => {
                if *stride <= self.cache_line_size {
                    0.8
                } else {
                    0.4
                }
            }
            AccessPattern::Random => 0.2,
            AccessPattern::Mixed => 0.6,
        }
    }

    /// Determine appropriate memory optimization type
    fn determine_memory_optimization_type(&self, pattern: &AccessPattern, locality_score: f64) -> MemoryOptimizationType {
        if locality_score < 0.3 {
            MemoryOptimizationType::Prefetching
        } else if locality_score < 0.6 {
            MemoryOptimizationType::Locality
        } else {
            MemoryOptimizationType::Allocation
        }
    }

    /// Calculate expected memory performance improvement
    fn calculate_memory_improvement(&self, locality_score: f64) -> f64 {
        // Improvement potential is higher for worse locality
        (1.0 - locality_score) * 0.4
    }

    /// Generate specific memory optimization recommendations
    fn generate_memory_recommendations(&self, pattern: &AccessPattern, locality_score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();

        if locality_score < 0.3 {
            recommendations.push("Add software prefetching".to_string());
            recommendations.push("Consider data structure reorganization".to_string());
        }

        if locality_score < 0.6 {
            recommendations.push("Improve data locality".to_string());
            recommendations.push("Consider loop tiling".to_string());
        }

        match pattern {
            AccessPattern::Random => {
                recommendations.push("Consider hash table optimization".to_string());
                recommendations.push("Add cache-friendly data structures".to_string());
            }
            AccessPattern::Strided { stride } if *stride > self.cache_line_size => {
                recommendations.push("Optimize stride pattern".to_string());
                recommendations.push("Consider array-of-structures to structure-of-arrays transformation".to_string());
            }
            _ => {}
        }

        recommendations
    }
}

/// Function inlining decision engine
#[derive(Debug, Clone)]
pub struct InliningEngine {
    pub size_threshold: usize,
    pub call_frequency_threshold: u64,
    pub depth_limit: usize,
    pub aggressive_inlining: bool,
}

impl Default for InliningEngine {
    fn default() -> Self {
        Self {
            size_threshold: 300,        // Maximum function size for inlining
            call_frequency_threshold: 100, // Minimum call frequency
            depth_limit: 5,             // Maximum inlining depth
            aggressive_inlining: false,
        }
    }
}

impl InliningEngine {
    /// Make inlining decisions based on profile data
    pub fn make_inlining_decisions(&self, profile_data: &ProfileData, hot_functions: &[HotFunctionInfo]) -> Result<Vec<InliningDecision>> {
        let mut decisions = Vec::new();

        // Build call graph from profile data
        let call_graph = self.build_call_graph(profile_data)?;

        for hot_function in hot_functions {
            if !hot_function.inlining_candidate {
                continue;
            }

            // Find callers of this hot function
            let callers = call_graph.get_callers(&hot_function.name);
            
            for caller in callers {
                let call_frequency = call_graph.get_call_frequency(caller, &hot_function.name);
                
                if call_frequency >= self.call_frequency_threshold {
                    let should_inline = self.should_inline(
                        caller,
                        &hot_function.name,
                        call_frequency,
                        &hot_function,
                        &call_graph
                    );

                    let decision = InliningDecision {
                        caller_function: caller.clone(),
                        callee_function: hot_function.name.clone(),
                        should_inline,
                        call_frequency,
                        estimated_size_increase: self.estimate_size_increase(&hot_function.name),
                        estimated_performance_gain: self.estimate_performance_gain(call_frequency, &hot_function),
                        reasoning: self.explain_decision(should_inline, call_frequency, &hot_function),
                    };
                    decisions.push(decision);
                }
            }
        }

        tracing::info!("Made {} inlining decisions", decisions.len());
        Ok(decisions)
    }

    /// Build call graph from profile data
    fn build_call_graph(&self, profile_data: &ProfileData) -> Result<CallGraph> {
        let mut call_graph = CallGraph::new();

        // Add edges from profile data
        for (edge_name, &call_count) in &profile_data.edge_counts {
            if let Some((caller, callee)) = self.parse_edge_name(edge_name) {
                call_graph.add_edge(caller, callee, call_count);
            }
        }

        Ok(call_graph)
    }

    /// Parse edge name to extract caller and callee
    fn parse_edge_name(&self, edge_name: &str) -> Option<(String, String)> {
        // Simplified parsing - assumes format "caller->callee"
        if let Some(arrow_pos) = edge_name.find("->") {
            let caller = edge_name[..arrow_pos].to_string();
            let callee = edge_name[arrow_pos + 2..].to_string();
            Some((caller, callee))
        } else {
            None
        }
    }

    /// Determine if function should be inlined
    fn should_inline(
        &self,
        caller: &str,
        callee: &str,
        call_frequency: u64,
        hot_function: &HotFunctionInfo,
        call_graph: &CallGraph
    ) -> bool {
        // Don't inline if it would create cycles
        if call_graph.would_create_cycle(caller, callee) {
            return false;
        }

        // Don't inline very large functions unless aggressive inlining is enabled
        let estimated_size = self.estimate_function_size(callee);
        if estimated_size > self.size_threshold && !self.aggressive_inlining {
            return false;
        }

        // Don't inline if we're already at the depth limit
        let current_depth = call_graph.get_inlining_depth(caller);
        if current_depth >= self.depth_limit {
            return false;
        }

        // Inline if call frequency is high enough and function is hot
        call_frequency >= self.call_frequency_threshold && hot_function.optimization_potential > 0.1
    }

    /// Estimate function size (simplified)
    fn estimate_function_size(&self, function_name: &str) -> usize {
        // Very simplified size estimation based on name length and patterns
        let base_size = function_name.len() * 10;
        
        if function_name.contains("complex") || function_name.contains("large") {
            base_size * 5
        } else if function_name.contains("simple") || function_name.contains("small") {
            base_size / 2
        } else {
            base_size
        }
    }

    /// Estimate size increase from inlining
    fn estimate_size_increase(&self, function_name: &str) -> usize {
        self.estimate_function_size(function_name) - 20 // Subtract call overhead
    }

    /// Estimate performance gain from inlining
    fn estimate_performance_gain(&self, call_frequency: u64, hot_function: &HotFunctionInfo) -> f64 {
        let call_overhead_savings = call_frequency as f64 * 0.001; // Small per-call savings
        let optimization_potential = hot_function.optimization_potential;
        
        call_overhead_savings + optimization_potential * 0.1
    }

    /// Explain inlining decision
    fn explain_decision(&self, should_inline: bool, call_frequency: u64, hot_function: &HotFunctionInfo) -> String {
        if should_inline {
            format!("Inline due to high call frequency ({}) and optimization potential ({:.1}%)", 
                   call_frequency, hot_function.optimization_potential * 100.0)
        } else {
            format!("Don't inline: insufficient benefit or constraints violated")
        }
    }
}

/// Supporting data structures

#[derive(Debug, Clone)]
pub struct HotFunctionInfo {
    pub name: String,
    pub execution_count: u64,
    pub execution_time: Duration,
    pub time_percentage: f64,
    pub optimization_level: OptimizationLevel,
    pub optimization_potential: f64,
    pub inlining_candidate: bool,
    pub specialization_candidate: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    Conservative,
    Moderate,
    Aggressive,
}

#[derive(Debug, Clone)]
pub struct BranchOptimization {
    pub block_name: String,
    pub edge_name: String,
    pub branch_probability: f64,
    pub is_predictable: bool,
    pub optimization_type: BranchOptimizationType,
    pub estimated_speedup: f64,
}

#[derive(Debug, Clone)]
pub enum BranchOptimizationType {
    LikelyBranch,
    UnlikelyBranch,
    PredictableBranch,
    UnpredictableBranch,
}

#[derive(Debug, Clone)]
pub struct MemoryOptimization {
    pub function_name: String,
    pub access_pattern: AccessPattern,
    pub locality_score: f64,
    pub memory_usage: MemoryUsage,
    pub optimization_type: MemoryOptimizationType,
    pub estimated_improvement: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum AccessPattern {
    Sequential,
    Random,
    Strided { stride: usize },
    Mixed,
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub total_allocations: u64,
    pub total_bytes: u64,
    pub peak_usage: u64,
    pub average_usage: u64,
}

#[derive(Debug, Clone)]
pub struct InliningDecision {
    pub caller_function: String,
    pub callee_function: String,
    pub should_inline: bool,
    pub call_frequency: u64,
    pub estimated_size_increase: usize,
    pub estimated_performance_gain: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub struct CallGraph {
    edges: HashMap<String, HashMap<String, u64>>,
    reverse_edges: HashMap<String, HashSet<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, caller: String, callee: String, call_count: u64) {
        self.edges.entry(caller.clone())
            .or_insert_with(HashMap::new)
            .insert(callee.clone(), call_count);
        
        self.reverse_edges.entry(callee)
            .or_insert_with(HashSet::new)
            .insert(caller);
    }

    pub fn get_callers(&self, function: &str) -> Vec<&String> {
        self.reverse_edges.get(function)
            .map(|callers| callers.iter().collect())
            .unwrap_or_default()
    }

    pub fn get_call_frequency(&self, caller: &str, callee: &str) -> u64 {
        self.edges.get(caller)
            .and_then(|callees| callees.get(callee))
            .copied()
            .unwrap_or(0)
    }

    pub fn would_create_cycle(&self, caller: &str, callee: &str) -> bool {
        // Simplified cycle detection
        self.has_path(callee, caller)
    }

    pub fn get_inlining_depth(&self, _function: &str) -> usize {
        // Simplified depth calculation
        0
    }

    fn has_path(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if visited.contains(current) {
                continue;
            }
            visited.insert(current);

            if let Some(callees) = self.edges.get(current) {
                for callee in callees.keys() {
                    if callee == to {
                        return true;
                    }
                    if !visited.contains(callee.as_str()) {
                        stack.push(callee);
                    }
                }
            }
        }

        false
    }
}
