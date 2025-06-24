use crate::error::Error;
// Advanced performance analysis algorithms and insights

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use crate::profiling::core::{ProfileData, ProfilerError};
use crate::profiling::cpu::CpuProfileData;
use crate::profiling::memory::MemoryProfileData;
use crate::profiling::concurrency::ConcurrencyProfileData;

/// Advanced performance analyzer
#[derive(Debug)]
pub struct PerformanceAnalyzer {
    config: AnalysisConfig,
}

impl PerformanceAnalyzer {
    pub fn new(config: AnalysisConfig) -> Self {
        Self { config }
    }
    
    #[instrument(skip(self, profile_data))]
    pub fn analyze_performance(&self, profile_data: &ProfileData) -> Result<(), Error> {
        info!("Running comprehensive performance analysis");
        
        let mut insights = PerformanceInsights::new(profile_data.session_name.clone());
        
        // CPU Analysis
        if let Some(cpu_data) = self.extract_cpu_data(profile_data)? {
            insights.cpu_insights = Some(self.analyze_cpu_performance(&cpu_data)?);
        }
        
        // Memory Analysis
        if let Some(memory_data) = self.extract_memory_data(profile_data)? {
            insights.memory_insights = Some(self.analyze_memory_performance(&memory_data)?);
        }
        
        // Concurrency Analysis
        if let Some(concurrency_data) = self.extract_concurrency_data(profile_data)? {
            insights.concurrency_insights = Some(self.analyze_concurrency_performance(&concurrency_data)?);
        }
        
        // Cross-cutting analysis
        insights.optimization_opportunities = self.identify_optimization_opportunities(&insights);
        insights.bottlenecks = self.identify_bottlenecks(&insights);
        insights.performance_score = self.calculate_performance_score(&insights);
        
        Ok(insights)
    }
    
    #[instrument(skip(self, cpu_data))]
    fn analyze_cpu_performance(&self, cpu_data: &CpuProfileData) -> Result<(), Error> {
        let hot_functions = cpu_data.get_hot_functions(self.config.max_hot_functions);
        let call_graph = cpu_data.get_call_graph();
        
        let cpu_efficiency = self.calculate_cpu_efficiency(cpu_data);
        let function_complexity = self.analyze_function_complexity(&hot_functions);
        let call_patterns = self.analyze_call_patterns(&call_graph);
        let optimization_potential = self.estimate_cpu_optimization_potential(&hot_functions);
        
        // Convert hot functions to insights before moving hot_functions
        let hot_function_insights: Vec<HotFunctionInsight> = hot_functions.iter()
            .map(|(name, stats)| HotFunctionInsight {
                name: (*name).clone(),
                sample_count: stats.sample_count,
                percentage: stats.percentage(cpu_data.sample_count),
                optimization_potential: self.calculate_function_optimization_potential(stats),
                complexity_score: self.calculate_complexity_score(name),
            })
            .collect();
        
        let recommendations = self.generate_cpu_recommendations(&hot_functions, &call_graph);
        
        Ok(CpuInsights {
            hot_functions: hot_function_insights,
            cpu_efficiency,
            function_complexity,
            call_patterns,
            optimization_potential,
            recommendations,
        })
    }
    
    #[instrument(skip(self, memory_data))]
    fn analyze_memory_performance(&self, memory_data: &MemoryProfileData) -> Result<(), Error> {
        let allocation_analysis = memory_data.analyze_patterns();
        let memory_leaks = memory_data.detect_leaks();
        
        let memory_efficiency = self.calculate_memory_efficiency(memory_data);
        let allocation_patterns = self.analyze_allocation_patterns(&allocation_analysis);
        let gc_analysis = self.analyze_gc_performance(memory_data);
        let fragmentation_analysis = self.analyze_memory_fragmentation(&allocation_analysis);
        
        Ok(MemoryInsights {
            memory_efficiency,
            allocation_patterns,
            gc_analysis,
            fragmentation_analysis,
            leak_severity: self.classify_leak_severity(&memory_leaks),
            optimization_potential: self.estimate_memory_optimization_potential(&allocation_analysis),
            recommendations: self.generate_memory_recommendations(&allocation_analysis, &memory_leaks),
        })
    }
    
    #[instrument(skip(self, concurrency_data))]
    fn analyze_concurrency_performance(&self, concurrency_data: &ConcurrencyProfileData) -> Result<(), Error> {
        let timeline = concurrency_data.generate_goroutine_timeline();
        let channel_analysis = concurrency_data.analyze_channels();
        let deadlocks = concurrency_data.detect_deadlocks();
        let scheduler_analysis = concurrency_data.analyze_scheduler();
        
        let parallelism_efficiency = self.calculate_parallelism_efficiency(&timeline);
        let channel_efficiency = self.analyze_channel_efficiency(&channel_analysis);
        let contention_analysis = self.analyze_contention_patterns(concurrency_data);
        let scalability_analysis = self.analyze_scalability_potential(&timeline);
        
        Ok(ConcurrencyInsights {
            parallelism_efficiency,
            channel_efficiency,
            contention_analysis,
            scalability_analysis,
            deadlock_risk: self.assess_deadlock_risk(&deadlocks),
            scheduler_efficiency: scheduler_analysis.scheduler_efficiency,
            optimization_potential: self.estimate_concurrency_optimization_potential(&timeline, &channel_analysis),
            recommendations: self.generate_concurrency_recommendations(&timeline, &channel_analysis, &deadlocks),
        })
    }
    
    fn calculate_cpu_efficiency(&self, cpu_data: &CpuProfileData) -> f64 {
        if cpu_data.sample_count == 0 {
            return 0.0;
        }
        
        let hot_functions = cpu_data.get_hot_functions(5);
        let top_function_percentage = hot_functions.first()
            .map(|(_, stats)| stats.percentage(cpu_data.sample_count))
            .unwrap_or(0.0);
        
        // Efficiency decreases if one function dominates
        if top_function_percentage > 80.0 {
            0.2
        } else if top_function_percentage > 50.0 {
            0.5
        } else {
            1.0 - (top_function_percentage / 100.0)
        }
    }
    
    fn analyze_function_complexity(&self, hot_functions: &[(&String, &crate::profiling::cpu::FunctionStats)]) -> ComplexityAnalysis {
        let mut high_complexity_functions = Vec::new();
        let mut total_complexity = 0.0;
        
        for (name, stats) in hot_functions {
            let complexity = self.calculate_complexity_score(name);
            total_complexity += complexity;
            
            if complexity > self.config.complexity_threshold {
                high_complexity_functions.push(ComplexFunction {
                    name: name.to_string(),
                    complexity_score: complexity,
                    sample_count: stats.sample_count,
                });
            }
        }
        
        ComplexityAnalysis {
            average_complexity: if !hot_functions.is_empty() {
                total_complexity / hot_functions.len() as f64
            } else {
                0.0
            },
            high_complexity_functions,
            complexity_distribution: self.calculate_complexity_distribution(hot_functions),
        }
    }
    
    fn analyze_call_patterns(&self, call_graph: &crate::profiling::cpu::CallGraph) -> CallPatternAnalysis {
        let mut recursive_functions = Vec::new();
        let mut deep_call_chains = Vec::new();
        let mut frequent_calls = Vec::new();
        
        // Analyze call patterns
        for (caller, callees) in &call_graph.edges {
            for (callee, count) in callees {
                // Check for recursion
                if caller == callee {
                    recursive_functions.push(caller.clone());
                }
                
                // Check for frequent calls
                if *count > self.config.frequent_call_threshold {
                    frequent_calls.push(FrequentCall {
                        caller: caller.clone(),
                        callee: callee.clone(),
                        call_count: *count,
                    });
                }
            }
        }
        
        CallPatternAnalysis {
            recursive_functions,
            deep_call_chains,
            frequent_calls,
            call_graph_density: self.calculate_call_graph_density(call_graph),
        }
    }
    
    fn calculate_memory_efficiency(&self, memory_data: &MemoryProfileData) -> f64 {
        let analysis = memory_data.analyze_patterns();
        let current_usage = memory_data.calculate_current_usage();
        
        // Efficiency based on allocation/deallocation ratio and fragmentation
        let allocation_rate = analysis.allocation_rate;
        let peak_usage = analysis.peak_memory_usage;
        
        if peak_usage == 0 {
            return 1.0;
        }
        
        let efficiency = (current_usage.allocated_bytes as f64 / peak_usage as f64).min(1.0);
        efficiency
    }
    
    fn analyze_allocation_patterns(&self, analysis: &crate::profiling::memory::AllocationAnalysis) -> AllocationPatternAnalysis {
        let mut dominant_allocators = Vec::new();
        let mut size_distribution = Vec::new();
        
        // Find dominant allocating functions
        for (function, stats) in &analysis.function_allocations {
            if stats.total_bytes > analysis.peak_memory_usage / 10 {
                dominant_allocators.push(DominantAllocator {
                    function: function.clone(),
                    total_bytes: stats.total_bytes,
                    allocation_count: stats.allocation_count,
                    average_size: stats.average_size,
                });
            }
        }
        
        // Analyze size distribution
        for (size, count) in &analysis.size_histogram {
            size_distribution.push(SizeDistribution {
                size_bucket: *size,
                allocation_count: *count,
            });
        }
        
        AllocationPatternAnalysis {
            dominant_allocators,
            size_distribution,
            allocation_rate: analysis.allocation_rate,
            fragmentation_risk: self.assess_fragmentation_risk(&analysis.size_histogram),
        }
    }
    
    fn calculate_parallelism_efficiency(&self, timeline: &[crate::profiling::concurrency::GoroutineTimeline]) -> f64 {
        if timeline.is_empty() {
            return 0.0;
        }
        
        let active_goroutines = timeline.len();
        let total_yields: u64 = timeline.iter().map(|g| g.yield_count).sum();
        let total_blocks: u64 = timeline.iter().map(|g| g.block_count).sum();
        
        // Efficiency decreases with excessive yielding and blocking
        let yield_penalty = (total_yields as f64 / active_goroutines as f64) * 0.01;
        let block_penalty = (total_blocks as f64 / active_goroutines as f64) * 0.02;
        
        (1.0 - yield_penalty - block_penalty).max(0.0).min(1.0)
    }
    
    fn identify_optimization_opportunities(&self, insights: &PerformanceInsights) -> Vec<OptimizationOpportunity> {
        let mut opportunities = Vec::new();
        
        // CPU optimization opportunities
        if let Some(cpu_insights) = &insights.cpu_insights {
            if cpu_insights.cpu_efficiency < 0.7 {
                opportunities.push(OptimizationOpportunity {
                    category: OptimizationCategory::Cpu,
                    priority: Priority::High,
                    description: "CPU efficiency is low due to hotspot functions".to_string(),
                    estimated_impact: ImpactEstimate::High,
                    effort_required: EffortLevel::Medium,
                    recommendations: vec![
                        "Profile and optimize the most CPU-intensive functions".to_string(),
                        "Consider algorithmic improvements".to_string(),
                    ],
                });
            }
        }
        
        // Memory optimization opportunities
        if let Some(memory_insights) = &insights.memory_insights {
            if memory_insights.memory_efficiency < 0.8 {
                opportunities.push(OptimizationOpportunity {
                    category: OptimizationCategory::Memory,
                    priority: Priority::Medium,
                    description: "Memory usage could be optimized".to_string(),
                    estimated_impact: ImpactEstimate::Medium,
                    effort_required: EffortLevel::Low,
                    recommendations: vec![
                        "Review allocation patterns".to_string(),
                        "Consider object pooling".to_string(),
                    ],
                });
            }
        }
        
        // Concurrency optimization opportunities
        if let Some(concurrency_insights) = &insights.concurrency_insights {
            if concurrency_insights.parallelism_efficiency < 0.7 {
                opportunities.push(OptimizationOpportunity {
                    category: OptimizationCategory::Concurrency,
                    priority: Priority::High,
                    description: "Parallelism efficiency could be improved".to_string(),
                    estimated_impact: ImpactEstimate::High,
                    effort_required: EffortLevel::High,
                    recommendations: vec![
                        "Reduce goroutine contention".to_string(),
                        "Optimize channel usage patterns".to_string(),
                    ],
                });
            }
        }
        
        opportunities.sort_by(|a, b| b.priority.cmp(&a.priority));
        opportunities
    }
    
    fn identify_bottlenecks(&self, insights: &PerformanceInsights) -> Vec<PerformanceBottleneck> {
        let mut bottlenecks = Vec::new();
        
        // Identify CPU bottlenecks
        if let Some(cpu_insights) = &insights.cpu_insights {
            for hot_function in &cpu_insights.hot_functions {
                if hot_function.percentage > 30.0 {
                    bottlenecks.push(PerformanceBottleneck {
                        bottleneck_type: BottleneckType::CpuHotspot,
                        severity: if hot_function.percentage > 50.0 {
                            Severity::Critical
                        } else {
                            Severity::High
                        },
                        description: format!(
                            "Function '{}' consumes {:.1}% of CPU time",
                            hot_function.name, hot_function.percentage
                        ),
                        affected_component: hot_function.name.clone(),
                        resolution_suggestions: vec![
                            "Profile function internals".to_string(),
                            "Consider algorithmic optimization".to_string(),
                        ],
                    });
                }
            }
        }
        
        bottlenecks.sort_by(|a, b| b.severity.cmp(&a.severity));
        bottlenecks
    }
    
    fn calculate_performance_score(&self, insights: &PerformanceInsights) -> f64 {
        let mut score = 100.0;
        
        if let Some(cpu_insights) = &insights.cpu_insights {
            score *= cpu_insights.cpu_efficiency;
        }
        
        if let Some(memory_insights) = &insights.memory_insights {
            score *= memory_insights.memory_efficiency;
        }
        
        if let Some(concurrency_insights) = &insights.concurrency_insights {
            score *= concurrency_insights.parallelism_efficiency;
        }
        
        score
    }
    
    // Helper methods with simplified implementations
    fn extract_cpu_data(&self, profile_data: &ProfileData) -> Result<(), Error> {
        // Simplified - in real implementation would deserialize from profile_data
        Ok(None)
    }
    
    fn extract_memory_data(&self, profile_data: &ProfileData) -> Result<(), Error> {
        // Simplified - in real implementation would deserialize from profile_data
        Ok(None)
    }
    
    fn extract_concurrency_data(&self, profile_data: &ProfileData) -> Result<(), Error> {
        // Simplified - in real implementation would deserialize from profile_data
        Ok(None)
    }
    
    fn calculate_function_optimization_potential(&self, _stats: &crate::profiling::cpu::FunctionStats) -> f64 {
        0.5 // Placeholder
    }
    
    fn calculate_complexity_score(&self, _name: &str) -> f64 {
        0.5 // Placeholder - would analyze function complexity
    }
    
    fn estimate_cpu_optimization_potential(&self, _hot_functions: &[(&String, &crate::profiling::cpu::FunctionStats)]) -> f64 {
        0.3 // Placeholder
    }
    
    fn generate_cpu_recommendations(&self, _hot_functions: &[(&String, &crate::profiling::cpu::FunctionStats)], _call_graph: &crate::profiling::cpu::CallGraph) -> Vec<String> {
        Vec::from(["Optimize hot functions".to_string()])
    }
    
    fn analyze_gc_performance(&self, _memory_data: &MemoryProfileData) -> GcAnalysis {
        GcAnalysis::default()
    }
    
    fn analyze_memory_fragmentation(&self, _analysis: &crate::profiling::memory::AllocationAnalysis) -> FragmentationAnalysis {
        FragmentationAnalysis::default()
    }
    
    fn classify_leak_severity(&self, _leaks: &[crate::profiling::memory::MemoryLeak]) -> LeakSeverity {
        LeakSeverity::Low
    }
    
    fn estimate_memory_optimization_potential(&self, _analysis: &crate::profiling::memory::AllocationAnalysis) -> f64 {
        0.2
    }
    
    fn generate_memory_recommendations(&self, _analysis: &crate::profiling::memory::AllocationAnalysis, _leaks: &[crate::profiling::memory::MemoryLeak]) -> Vec<String> {
        Vec::from(["Review allocation patterns".to_string()])
    }
    
    fn analyze_channel_efficiency(&self, _analysis: &crate::profiling::concurrency::ChannelAnalysis) -> f64 {
        0.8
    }
    
    fn analyze_contention_patterns(&self, _data: &ConcurrencyProfileData) -> ContentionAnalysis {
        ContentionAnalysis::default()
    }
    
    fn analyze_scalability_potential(&self, _timeline: &[crate::profiling::concurrency::GoroutineTimeline]) -> ScalabilityAnalysis {
        ScalabilityAnalysis::default()
    }
    
    fn assess_deadlock_risk(&self, _deadlocks: &[crate::profiling::concurrency::DeadlockDetection]) -> DeadlockRisk {
        DeadlockRisk::Low
    }
    
    fn estimate_concurrency_optimization_potential(&self, _timeline: &[crate::profiling::concurrency::GoroutineTimeline], _analysis: &crate::profiling::concurrency::ChannelAnalysis) -> f64 {
        0.4
    }
    
    fn generate_concurrency_recommendations(&self, _timeline: &[crate::profiling::concurrency::GoroutineTimeline], _analysis: &crate::profiling::concurrency::ChannelAnalysis, _deadlocks: &[crate::profiling::concurrency::DeadlockDetection]) -> Vec<String> {
        Vec::from(["Optimize goroutine usage".to_string()])
    }
    
    fn calculate_complexity_distribution(&self, _functions: &[(&String, &crate::profiling::cpu::FunctionStats)]) -> ComplexityDistribution {
        ComplexityDistribution::default()
    }
    
    fn calculate_call_graph_density(&self, _call_graph: &crate::profiling::cpu::CallGraph) -> f64 {
        0.5
    }
    
    fn assess_fragmentation_risk(&self, _histogram: &HashMap<usize, u64>) -> FragmentationRisk {
        FragmentationRisk::Low
    }
}

// Analysis configuration and data structures...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub max_hot_functions: usize,
    pub complexity_threshold: f64,
    pub frequent_call_threshold: u64,
    pub enable_advanced_analysis: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_hot_functions: 20,
            complexity_threshold: 0.7,
            frequent_call_threshold: 100,
            enable_advanced_analysis: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsights {
    pub session_name: String,
    pub timestamp: std::time::SystemTime,
    pub performance_score: f64,
    pub cpu_insights: Option<CpuInsights>,
    pub memory_insights: Option<MemoryInsights>,
    pub concurrency_insights: Option<ConcurrencyInsights>,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    pub bottlenecks: Vec<PerformanceBottleneck>,
}

impl PerformanceInsights {
    pub fn new(session_name: String) -> Self {
        Self {
            session_name,
            timestamp: std::time::SystemTime::now(),
            performance_score: 0.0,
            cpu_insights: None,
            memory_insights: None,
            concurrency_insights: None,
            optimization_opportunities: Vec::new(),
            bottlenecks: Vec::new(),
        }
    }
}

// Additional data structures for comprehensive analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInsights {
    pub hot_functions: Vec<HotFunctionInsight>,
    pub cpu_efficiency: f64,
    pub function_complexity: ComplexityAnalysis,
    pub call_patterns: CallPatternAnalysis,
    pub optimization_potential: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInsights {
    pub memory_efficiency: f64,
    pub allocation_patterns: AllocationPatternAnalysis,
    pub gc_analysis: GcAnalysis,
    pub fragmentation_analysis: FragmentationAnalysis,
    pub leak_severity: LeakSeverity,
    pub optimization_potential: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyInsights {
    pub parallelism_efficiency: f64,
    pub channel_efficiency: f64,
    pub contention_analysis: ContentionAnalysis,
    pub scalability_analysis: ScalabilityAnalysis,
    pub deadlock_risk: DeadlockRisk,
    pub scheduler_efficiency: f64,
    pub optimization_potential: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub description: String,
    pub estimated_impact: ImpactEstimate,
    pub effort_required: EffortLevel,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Cpu,
    Memory,
    Concurrency,
    Io,
    Algorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactEstimate {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

// Placeholder implementations for comprehensive analysis structures
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    pub average_complexity: f64,
    pub high_complexity_functions: Vec<ComplexFunction>,
    pub complexity_distribution: ComplexityDistribution,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplexityDistribution {
    pub low: u32,
    pub medium: u32,
    pub high: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexFunction {
    pub name: String,
    pub complexity_score: f64,
    pub sample_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallPatternAnalysis {
    pub recursive_functions: Vec<String>,
    pub deep_call_chains: Vec<String>,
    pub frequent_calls: Vec<FrequentCall>,
    pub call_graph_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentCall {
    pub caller: String,
    pub callee: String,
    pub call_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunctionInsight {
    pub name: String,
    pub sample_count: u64,
    pub percentage: f64,
    pub optimization_potential: f64,
    pub complexity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationPatternAnalysis {
    pub dominant_allocators: Vec<DominantAllocator>,
    pub size_distribution: Vec<SizeDistribution>,
    pub allocation_rate: f64,
    pub fragmentation_risk: FragmentationRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominantAllocator {
    pub function: String,
    pub total_bytes: usize,
    pub allocation_count: usize,
    pub average_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeDistribution {
    pub size_bucket: usize,
    pub allocation_count: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GcAnalysis {
    pub efficiency: f64,
    pub pause_times: Vec<Duration>,
    pub collection_frequency: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FragmentationAnalysis {
    pub fragmentation_ratio: f64,
    pub largest_free_block: usize,
    pub free_block_count: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentionAnalysis {
    pub contention_points: Vec<String>,
    pub average_wait_time: Duration,
    pub contention_frequency: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScalabilityAnalysis {
    pub current_utilization: f64,
    pub theoretical_maximum: f64,
    pub scaling_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub bottleneck_type: BottleneckType,
    pub severity: Severity,
    pub description: String,
    pub affected_component: String,
    pub resolution_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    CpuHotspot,
    MemoryLeak,
    ConcurrencyContention,
    IoWait,
    AlgorithmicInefficiency,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeakSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlockRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FragmentationRisk {
    Low,
    Medium,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_analyzer_creation() {
        let config = AnalysisConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);
        assert_eq!(analyzer.config.max_hot_functions, 20);
    }
    
    #[test]
    fn test_performance_insights_creation() {
        let insights = PerformanceInsights::new("test_session".to_string());
        assert_eq!(insights.session_name, "test_session");
        assert_eq!(insights.performance_score, 0.0);
    }
    
    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }
    
    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }
}
