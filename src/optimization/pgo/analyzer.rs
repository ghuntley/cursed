/// Profile Data Analyzer
/// 
/// Analyzes collected profile data to identify optimization opportunities,
/// hot paths, cold code, and generates optimization recommendations.

use crate::error::{Error, Result};
use crate::optimization::pgo::{
    PgoConfig, ProfileData, ProfileAnalysis, HotFunction, LoopProfile,
    BranchProfile, MemoryProfile, OptimizationRecommendation, CallSiteProfile,
    ValueProfile, OptimizationPriority, MemoryAccessPattern, OptimizationPotential
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use tracing::{info, debug, warn, instrument};

/// Profile data analyzer
#[derive(Debug)]
pub struct ProfileAnalyzer {
    config: PgoConfig,
    analysis_cache: HashMap<String, ProfileAnalysis>,
    hot_function_cache: HashMap<String, Vec<HotFunction>>,
    optimization_recommendations_cache: HashMap<String, Vec<OptimizationRecommendation>>,
}

impl ProfileAnalyzer {
    /// Create a new profile analyzer
    #[instrument]
    pub fn new(config: PgoConfig) -> Result<Self> {
        info!("Creating profile analyzer");
        
        Ok(Self {
            config,
            analysis_cache: HashMap::new(),
            hot_function_cache: HashMap::new(),
            optimization_recommendations_cache: HashMap::new(),
        })
    }

    /// Analyze profile data and generate comprehensive analysis
    #[instrument(skip(self, profile_data))]
    pub fn analyze_profile_data(&mut self, profile_data: &ProfileData) -> Result<ProfileAnalysis> {
        info!("Analyzing profile data with {} functions", profile_data.function_counts.len());

        // Check cache first
        let cache_key = self.generate_cache_key(profile_data);
        if let Some(cached_analysis) = self.analysis_cache.get(&cache_key) {
            debug!("Returning cached analysis");
            return Ok(cached_analysis.clone());
        }

        // Perform comprehensive analysis
        let hot_functions = self.analyze_hot_functions(profile_data)?;
        let cold_functions = self.analyze_cold_functions(profile_data)?;
        let loop_profiles = self.analyze_loops(profile_data)?;
        let branch_profiles = self.analyze_branches(profile_data)?;
        let memory_profiles = self.analyze_memory_patterns(profile_data)?;
        let call_graph = self.build_call_graph(profile_data)?;
        let critical_path = self.analyze_critical_path(profile_data, &call_graph)?;
        let recommendations = self.generate_optimization_recommendations(
            &hot_functions, &cold_functions, &loop_profiles
        )?;

        let analysis = ProfileAnalysis {
            hot_functions,
            cold_functions,
            loop_profiles,
            branch_profiles,
            memory_profiles,
            total_execution_time: profile_data.total_execution_time,
            indirect_call_count: self.count_indirect_calls(profile_data),
            call_graph,
            critical_path,
            recommendations,
        };

        // Cache the analysis
        self.analysis_cache.insert(cache_key, analysis.clone());

        info!("Analysis complete: {} hot functions, {} cold functions, {} recommendations",
              analysis.hot_functions.len(),
              analysis.cold_functions.len(),
              analysis.recommendations.len());

        Ok(analysis)
    }

    /// Analyze hot functions based on execution frequency and time
    #[instrument(skip(self, profile_data))]
    fn analyze_hot_functions(&self, profile_data: &ProfileData) -> Result<Vec<HotFunction>> {
        let total_executions = profile_data.total_function_executions();
        let total_time = profile_data.total_execution_time;
        
        if total_executions == 0 {
            return Ok(Vec::new());
        }

        let mut hot_functions = Vec::new();
        let hot_threshold = (total_executions as f64 * self.config.hot_function_threshold) as u64;

        for (function_name, &execution_count) in &profile_data.function_counts {
            if execution_count >= hot_threshold || execution_count >= self.config.min_execution_count {
                let time_percentage = if total_time.as_nanos() > 0 {
                    // Estimate time percentage based on execution count
                    (execution_count as f64 / total_executions as f64) * 100.0
                } else {
                    0.0
                };

                // Estimate function characteristics
                let estimated_size = self.estimate_function_size(function_name);
                let has_vectorizable_loops = self.has_vectorizable_loops(function_name, profile_data);
                let memory_pattern = self.analyze_function_memory_pattern(function_name, profile_data);
                let optimization_potential = self.assess_optimization_potential(
                    function_name, execution_count, estimated_size, &memory_pattern
                );

                let hot_function = HotFunction {
                    name: function_name.clone(),
                    execution_count,
                    total_time: Duration::from_millis((time_percentage * total_time.as_millis() as f64 / 100.0) as u64),
                    average_time: Duration::from_nanos(
                        (time_percentage * total_time.as_nanos() as f64 / 100.0 / execution_count as f64) as u64
                    ),
                    time_percentage,
                    optimization_priority: self.determine_optimization_priority(execution_count, time_percentage),
                    call_sites: self.extract_call_sites(function_name, profile_data),
                    call_count: execution_count, // Simplified assumption
                    average_size: estimated_size,
                    has_vectorizable_loops,
                    memory_access_pattern: memory_pattern,
                    branch_prediction_accuracy: self.estimate_branch_prediction_accuracy(function_name, profile_data),
                    cache_miss_rate: self.estimate_cache_miss_rate(function_name, profile_data),
                    optimization_potential,
                };

                hot_functions.push(hot_function);
            }
        }

        // Sort by execution count and time percentage
        hot_functions.sort_by(|a, b| {
            b.execution_count.cmp(&a.execution_count)
                .then(b.time_percentage.partial_cmp(&a.time_percentage).unwrap_or(std::cmp::Ordering::Equal))
        });

        debug!("Identified {} hot functions", hot_functions.len());
        Ok(hot_functions)
    }

    /// Analyze cold functions for size optimization
    #[instrument(skip(self, profile_data))]
    fn analyze_cold_functions(&self, profile_data: &ProfileData) -> Result<Vec<String>> {
        let total_executions = profile_data.total_function_executions();
        let cold_threshold = (total_executions as f64 * self.config.cold_function_threshold) as u64;

        let cold_functions: Vec<String> = profile_data.function_counts
            .iter()
            .filter(|(_, &count)| count < cold_threshold && count > 0)
            .map(|(name, _)| name.clone())
            .collect();

        debug!("Identified {} cold functions", cold_functions.len());
        Ok(cold_functions)
    }

    /// Analyze loop execution patterns
    #[instrument(skip(self, profile_data))]
    fn analyze_loops(&self, profile_data: &ProfileData) -> Result<Vec<LoopProfile>> {
        let mut loop_profiles = Vec::new();

        // Extract loop information from basic block and edge data
        for (edge_id, &count) in &profile_data.edge_counts {
            if self.is_loop_edge(edge_id) {
                let (function_name, loop_id) = self.parse_loop_edge_id(edge_id);
                
                // Estimate loop characteristics
                let average_iteration_count = self.estimate_loop_iterations(&function_name, &loop_id, count);
                let is_vectorizable = self.is_loop_vectorizable(&function_name, &loop_id);
                let has_dependencies = self.has_loop_dependencies(&function_name, &loop_id);
                let memory_pattern = self.analyze_loop_memory_pattern(&function_name, &loop_id);

                let loop_profile = LoopProfile {
                    function_name: function_name.clone(),
                    loop_id: loop_id.clone(),
                    average_iteration_count,
                    total_iteration_count: count,
                    execution_count: profile_data.function_counts.get(&function_name).copied().unwrap_or(0),
                    average_iteration_time: Duration::from_nanos(1000), // Placeholder
                    is_vectorizable,
                    has_dependencies,
                    memory_pattern,
                    optimization_potential: if average_iteration_count > 100.0 && is_vectorizable {
                        OptimizationPotential::High
                    } else if average_iteration_count > 10.0 {
                        OptimizationPotential::Medium
                    } else {
                        OptimizationPotential::Low
                    },
                };

                loop_profiles.push(loop_profile);
            }
        }

        debug!("Analyzed {} loops", loop_profiles.len());
        Ok(loop_profiles)
    }

    /// Analyze branch execution patterns
    #[instrument(skip(self, profile_data))]
    fn analyze_branches(&self, profile_data: &ProfileData) -> Result<Vec<BranchProfile>> {
        let mut branch_profiles = Vec::new();

        // Extract branch information from edge data
        for (edge_id, &count) in &profile_data.edge_counts {
            if self.is_branch_edge(edge_id) {
                let (function_name, branch_id, is_taken) = self.parse_branch_edge_id(edge_id);
                
                // Find corresponding branch edges
                let taken_count = if is_taken { count } else { 0 };
                let not_taken_count = if !is_taken { count } else { 0 };
                
                // Calculate prediction accuracy (simplified)
                let total_count = taken_count + not_taken_count;
                let prediction_accuracy = if total_count > 0 {
                    std::cmp::max(taken_count, not_taken_count) as f64 / total_count as f64
                } else {
                    1.0
                };

                let branch_profile = BranchProfile {
                    function_name,
                    branch_id,
                    taken_count,
                    not_taken_count,
                    prediction_accuracy,
                    is_critical: total_count > 1000 && prediction_accuracy < 0.8,
                };

                branch_profiles.push(branch_profile);
            }
        }

        debug!("Analyzed {} branches", branch_profiles.len());
        Ok(branch_profiles)
    }

    /// Analyze memory access patterns
    #[instrument(skip(self, profile_data))]
    fn analyze_memory_patterns(&self, profile_data: &ProfileData) -> Result<Vec<MemoryProfile>> {
        let mut memory_profiles = Vec::new();

        for (function_name, &execution_count) in &profile_data.function_counts {
            // Estimate memory characteristics based on function patterns
            let estimated_allocations = execution_count * self.estimate_allocations_per_call(function_name);
            let access_pattern = self.analyze_function_memory_pattern(function_name, profile_data);
            let cache_hit_rate = 1.0 - self.estimate_cache_miss_rate(function_name, profile_data);

            let memory_profile = MemoryProfile {
                function_name: function_name.clone(),
                allocation_count: estimated_allocations,
                deallocation_count: estimated_allocations, // Assume balanced
                peak_memory_usage: estimated_allocations * 1024, // Estimate 1KB per allocation
                average_allocation_size: 1024,
                cache_hit_rate,
                access_pattern,
                page_faults: estimated_allocations / 1000, // Estimate
            };

            memory_profiles.push(memory_profile);
        }

        debug!("Analyzed memory patterns for {} functions", memory_profiles.len());
        Ok(memory_profiles)
    }

    /// Build function call graph
    #[instrument(skip(self, profile_data))]
    fn build_call_graph(&self, profile_data: &ProfileData) -> Result<HashMap<String, Vec<String>>> {
        let mut call_graph = HashMap::new();

        // Extract call relationships from edge data and function names
        for function_name in profile_data.function_counts.keys() {
            let callees = self.extract_function_callees(function_name, profile_data);
            if !callees.is_empty() {
                call_graph.insert(function_name.clone(), callees);
            }
        }

        debug!("Built call graph with {} nodes", call_graph.len());
        Ok(call_graph)
    }

    /// Analyze critical execution path
    #[instrument(skip(self, profile_data, call_graph))]
    fn analyze_critical_path(
        &self,
        profile_data: &ProfileData,
        call_graph: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<String>> {
        // Find the path through the most executed functions
        let mut critical_path = Vec::new();
        let mut visited = HashSet::new();
        let mut current_function = profile_data.most_executed_function()
            .map(|(name, _)| name.clone());

        while let Some(function) = current_function {
            if visited.contains(&function) {
                break; // Avoid infinite loops
            }

            critical_path.push(function.clone());
            visited.insert(function.clone());

            // Find the most executed callee
            current_function = call_graph.get(&function)
                .and_then(|callees| {
                    callees.iter()
                        .filter_map(|callee| {
                            profile_data.function_counts.get(callee)
                                .map(|&count| (callee.clone(), count))
                        })
                        .max_by_key(|(_, count)| *count)
                        .map(|(name, _)| name)
                });
        }

        debug!("Critical path contains {} functions", critical_path.len());
        Ok(critical_path)
    }

    /// Generate optimization recommendations
    #[instrument(skip(self, hot_functions, cold_functions, loop_profiles))]
    fn generate_optimization_recommendations(
        &self,
        hot_functions: &[HotFunction],
        cold_functions: &[String],
        loop_profiles: &[LoopProfile],
    ) -> Result<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Function inlining recommendations
        for hot_function in hot_functions {
            if hot_function.average_size < 50 && hot_function.call_count > 100 {
                recommendations.push(OptimizationRecommendation {
                    target: hot_function.name.clone(),
                    optimization_type: "function_inlining".to_string(),
                    expected_improvement: 15.0,
                    confidence: 0.8,
                    priority: OptimizationPriority::High,
                    explanation: format!(
                        "Function {} is small ({} instructions) and frequently called ({} times). Inlining could eliminate call overhead.",
                        hot_function.name, hot_function.average_size, hot_function.call_count
                    ),
                    compiler_flags: vec![
                        "-finline-functions".to_string(),
                        format!("-finline-limit={}", hot_function.average_size * 2),
                    ],
                });
            }
        }

        // Loop optimization recommendations
        for loop_profile in loop_profiles {
            if loop_profile.average_iteration_count > 10.0 && loop_profile.is_vectorizable {
                recommendations.push(OptimizationRecommendation {
                    target: format!("{}::{}", loop_profile.function_name, loop_profile.loop_id),
                    optimization_type: "loop_vectorization".to_string(),
                    expected_improvement: 25.0,
                    confidence: 0.9,
                    priority: OptimizationPriority::High,
                    explanation: format!(
                        "Loop in {} has high iteration count ({:.1}) and is vectorizable. SIMD optimization could provide significant speedup.",
                        loop_profile.function_name, loop_profile.average_iteration_count
                    ),
                    compiler_flags: vec![
                        "-fvectorize".to_string(),
                        "-fslp-vectorize".to_string(),
                    ],
                });
            }

            if loop_profile.average_iteration_count > 5.0 && !loop_profile.has_dependencies {
                recommendations.push(OptimizationRecommendation {
                    target: format!("{}::{}", loop_profile.function_name, loop_profile.loop_id),
                    optimization_type: "loop_unrolling".to_string(),
                    expected_improvement: 12.0,
                    confidence: 0.7,
                    priority: OptimizationPriority::Medium,
                    explanation: format!(
                        "Loop in {} can be unrolled to reduce branch overhead. No data dependencies detected.",
                        loop_profile.function_name
                    ),
                    compiler_flags: vec![
                        "-funroll-loops".to_string(),
                        "-funroll-all-loops".to_string(),
                    ],
                });
            }
        }

        // Cold function size optimization
        for cold_function in cold_functions {
            recommendations.push(OptimizationRecommendation {
                target: cold_function.clone(),
                optimization_type: "size_optimization".to_string(),
                expected_improvement: 5.0,
                confidence: 0.6,
                priority: OptimizationPriority::Low,
                explanation: format!(
                    "Function {} is rarely executed. Optimize for size to reduce binary bloat.",
                    cold_function
                ),
                compiler_flags: vec![
                    "-Os".to_string(),
                    "-ffunction-sections".to_string(),
                ],
            });
        }

        // Sort recommendations by priority and expected improvement
        recommendations.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
                .then(b.expected_improvement.partial_cmp(&a.expected_improvement).unwrap_or(std::cmp::Ordering::Equal))
        });

        info!("Generated {} optimization recommendations", recommendations.len());
        Ok(recommendations)
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: PgoConfig) -> Result<()> {
        self.config = new_config;
        // Clear caches when configuration changes
        self.analysis_cache.clear();
        self.hot_function_cache.clear();
        self.optimization_recommendations_cache.clear();
        Ok(())
    }

    // Helper methods for analysis
    
    fn generate_cache_key(&self, profile_data: &ProfileData) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        profile_data.function_counts.len().hash(&mut hasher);
        profile_data.total_execution_time.hash(&mut hasher);
        
        format!("analysis_{}", hasher.finish())
    }

    fn estimate_function_size(&self, function_name: &str) -> u32 {
        // Heuristic based on function name patterns
        match function_name {
            name if name.contains("main") => 100,
            name if name.contains("init") => 50,
            name if name.contains("helper") => 25,
            name if name.contains("test") => 75,
            name if name.len() > 20 => 80,
            _ => 40,
        }
    }

    fn has_vectorizable_loops(&self, function_name: &str, _profile_data: &ProfileData) -> bool {
        // Heuristic based on function name
        function_name.contains("compute") || 
        function_name.contains("process") || 
        function_name.contains("transform") ||
        function_name.contains("math")
    }

    fn analyze_function_memory_pattern(&self, function_name: &str, _profile_data: &ProfileData) -> MemoryAccessPattern {
        // Heuristic based on function name
        if function_name.contains("sort") || function_name.contains("search") {
            MemoryAccessPattern::Sequential
        } else if function_name.contains("hash") || function_name.contains("map") {
            MemoryAccessPattern::Random
        } else if function_name.contains("matrix") || function_name.contains("array") {
            MemoryAccessPattern::Strided
        } else {
            MemoryAccessPattern::Unknown
        }
    }

    fn assess_optimization_potential(
        &self,
        _function_name: &str,
        execution_count: u64,
        function_size: u32,
        memory_pattern: &MemoryAccessPattern,
    ) -> OptimizationPotential {
        let mut score = 0;

        if execution_count > 10000 { score += 3; }
        else if execution_count > 1000 { score += 2; }
        else if execution_count > 100 { score += 1; }

        if function_size > 100 { score += 2; }
        else if function_size > 50 { score += 1; }

        match memory_pattern {
            MemoryAccessPattern::Sequential => score += 2,
            MemoryAccessPattern::Strided => score += 1,
            MemoryAccessPattern::Random => score -= 1,
            _ => {}
        }

        match score {
            6.. => OptimizationPotential::High,
            3..=5 => OptimizationPotential::Medium,
            _ => OptimizationPotential::Low,
        }
    }

    fn determine_optimization_priority(&self, execution_count: u64, time_percentage: f64) -> OptimizationPriority {
        if execution_count > 50000 || time_percentage > 20.0 {
            OptimizationPriority::Critical
        } else if execution_count > 10000 || time_percentage > 10.0 {
            OptimizationPriority::High
        } else if execution_count > 1000 || time_percentage > 5.0 {
            OptimizationPriority::Medium
        } else {
            OptimizationPriority::Low
        }
    }

    fn extract_call_sites(&self, function_name: &str, profile_data: &ProfileData) -> HashMap<String, u64> {
        let mut call_sites = HashMap::new();
        
        // Look for edge data that represents calls to this function
        for (edge_id, &count) in &profile_data.edge_counts {
            if edge_id.contains(&format!("call_{}", function_name)) {
                // Extract caller from edge ID
                if let Some(caller) = edge_id.split("call_").next() {
                    call_sites.insert(caller.to_string(), count);
                }
            }
        }
        
        call_sites
    }

    fn estimate_branch_prediction_accuracy(&self, function_name: &str, _profile_data: &ProfileData) -> f64 {
        // Heuristic based on function complexity
        if function_name.contains("simple") || function_name.contains("linear") {
            0.95
        } else if function_name.contains("complex") || function_name.contains("branch") {
            0.75
        } else {
            0.85
        }
    }

    fn estimate_cache_miss_rate(&self, function_name: &str, _profile_data: &ProfileData) -> f64 {
        // Heuristic based on memory access patterns
        if function_name.contains("random") || function_name.contains("sparse") {
            0.15
        } else if function_name.contains("sequential") || function_name.contains("local") {
            0.02
        } else {
            0.05
        }
    }

    fn count_indirect_calls(&self, profile_data: &ProfileData) -> u64 {
        profile_data.value_profiles.values().sum()
    }

    fn is_loop_edge(&self, edge_id: &str) -> bool {
        edge_id.contains("loop_") || edge_id.contains("_loop") || edge_id.contains("backedge")
    }

    fn parse_loop_edge_id(&self, edge_id: &str) -> (String, String) {
        // Parse edge ID format like "function_name:loop_id"
        if let Some((function, loop_part)) = edge_id.split_once(':') {
            (function.to_string(), loop_part.to_string())
        } else {
            ("unknown".to_string(), edge_id.to_string())
        }
    }

    fn estimate_loop_iterations(&self, _function_name: &str, _loop_id: &str, total_count: u64) -> f64 {
        // Estimate average iterations based on total count
        std::cmp::max(1, total_count / 100) as f64
    }

    fn is_loop_vectorizable(&self, function_name: &str, _loop_id: &str) -> bool {
        function_name.contains("compute") || function_name.contains("math") || function_name.contains("simd")
    }

    fn has_loop_dependencies(&self, function_name: &str, _loop_id: &str) -> bool {
        function_name.contains("dependency") || function_name.contains("recursive")
    }

    fn analyze_loop_memory_pattern(&self, function_name: &str, _loop_id: &str) -> MemoryAccessPattern {
        self.analyze_function_memory_pattern(function_name, &ProfileData::default())
    }

    fn is_branch_edge(&self, edge_id: &str) -> bool {
        edge_id.contains("branch_") || edge_id.contains("_branch") || edge_id.contains("cond")
    }

    fn parse_branch_edge_id(&self, edge_id: &str) -> (String, String, bool) {
        // Parse edge ID format like "function_name:branch_id:taken"
        let parts: Vec<&str> = edge_id.split(':').collect();
        if parts.len() >= 3 {
            (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2] == "taken"
            )
        } else {
            ("unknown".to_string(), edge_id.to_string(), true)
        }
    }

    fn estimate_allocations_per_call(&self, function_name: &str) -> u64 {
        if function_name.contains("alloc") || function_name.contains("new") {
            10
        } else if function_name.contains("create") || function_name.contains("build") {
            5
        } else {
            1
        }
    }

    fn extract_function_callees(&self, function_name: &str, profile_data: &ProfileData) -> Vec<String> {
        let mut callees = Vec::new();
        
        // Look for edges that represent calls from this function
        for edge_id in profile_data.edge_counts.keys() {
            if edge_id.starts_with(&format!("{}_call_", function_name)) {
                if let Some(callee) = edge_id.strip_prefix(&format!("{}_call_", function_name)) {
                    callees.push(callee.to_string());
                }
            }
        }
        
        callees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let config = PgoConfig::default();
        let analyzer = ProfileAnalyzer::new(config);
        assert!(analyzer.is_ok());
    }

    #[test]
    fn test_hot_function_analysis() {
        let config = PgoConfig::default();
        let mut analyzer = ProfileAnalyzer::new(config).unwrap();
        
        let mut profile_data = ProfileData::default();
        profile_data.add_function_execution("hot_function".to_string(), 10000);
        profile_data.add_function_execution("cold_function".to_string(), 10);
        
        let hot_functions = analyzer.analyze_hot_functions(&profile_data).unwrap();
        assert!(!hot_functions.is_empty());
        assert_eq!(hot_functions[0].name, "hot_function");
    }

    #[test]
    fn test_optimization_recommendations() {
        let config = PgoConfig::default();
        let mut analyzer = ProfileAnalyzer::new(config).unwrap();
        
        let hot_functions = vec![
            HotFunction {
                name: "small_hot_function".to_string(),
                execution_count: 10000,
                average_size: 30,
                call_count: 10000,
                optimization_potential: OptimizationPotential::High,
                time_percentage: 25.0,
                total_time: Duration::from_millis(100),
                average_time: Duration::from_nanos(10000),
                optimization_priority: OptimizationPriority::High,
                call_sites: HashMap::new(),
                has_vectorizable_loops: false,
                memory_access_pattern: MemoryAccessPattern::Sequential,
                branch_prediction_accuracy: 0.9,
                cache_miss_rate: 0.02,
            }
        ];
        
        let recommendations = analyzer.generate_optimization_recommendations(
            &hot_functions, &[], &[]
        ).unwrap();
        
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].optimization_type.contains("inlining"));
    }
}
