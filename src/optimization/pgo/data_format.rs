/// Profile Data Format Definitions
/// 
/// Defines the structure and format of profile data collected during execution.
/// Supports both LLVM's profdata format and custom CURSED profile formats.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Main profile data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    /// Function execution counts
    pub function_counts: HashMap<String, u64>,
    
    /// Basic block execution counts
    pub basic_block_counts: HashMap<String, u64>,
    
    /// Edge execution counts (control flow edges)
    pub edge_counts: HashMap<String, u64>,
    
    /// Total execution time of the profiled program
    pub total_execution_time: Duration,
    
    /// Hot functions identified during analysis
    pub hot_functions: Vec<HotFunction>,
    
    /// Cold functions identified during analysis
    pub cold_functions: Vec<String>,
    
    /// Value profiles for indirect calls and other dynamic values
    pub value_profiles: HashMap<String, u64>,
    
    /// Sampling rate used during collection (samples per second)
    pub sampling_rate: u32,
    
    /// Timestamp when profile data was collected
    pub collection_timestamp: SystemTime,
    
    /// Additional metadata
    pub metadata: ProfileMetadata,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self {
            function_counts: HashMap::new(),
            basic_block_counts: HashMap::new(),
            edge_counts: HashMap::new(),
            total_execution_time: Duration::ZERO,
            hot_functions: Vec::new(),
            cold_functions: Vec::new(),
            value_profiles: HashMap::new(),
            sampling_rate: 0,
            collection_timestamp: SystemTime::now(),
            metadata: ProfileMetadata::default(),
        }
    }
}

/// Information about frequently executed functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunction {
    /// Function name
    pub name: String,
    
    /// Number of times the function was executed
    pub execution_count: u64,
    
    /// Total time spent in this function
    pub total_time: Duration,
    
    /// Average time per execution
    pub average_time: Duration,
    
    /// Percentage of total execution time
    pub time_percentage: f64,
    
    /// Optimization priority based on profile analysis
    pub optimization_priority: OptimizationPriority,
    
    /// Call sites and their frequencies
    pub call_sites: HashMap<String, u64>,
    
    /// Number of direct and indirect calls
    pub call_count: u64,
    
    /// Average function size in instructions/lines
    pub average_size: u32,
    
    /// Whether the function has vectorizable loops
    pub has_vectorizable_loops: bool,
    
    /// Memory access patterns
    pub memory_access_pattern: MemoryAccessPattern,
    
    /// Branch prediction accuracy
    pub branch_prediction_accuracy: f64,
    
    /// Cache miss rate
    pub cache_miss_rate: f64,
    
    /// Optimization potential based on analysis
    pub optimization_potential: OptimizationPotential,
}

/// Priority levels for optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Memory access patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryAccessPattern {
    Sequential,
    Random,
    Strided,
    Irregular,
    Unknown,
}

/// Optimization potential assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationPotential {
    High,
    Medium,
    Low,
}

/// Profile metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Version of the profile format
    pub format_version: String,
    
    /// Target architecture
    pub target_architecture: String,
    
    /// Compiler version used
    pub compiler_version: String,
    
    /// Optimization level used during profiling
    pub optimization_level: String,
    
    /// Program arguments during profiling
    pub program_arguments: Vec<String>,
    
    /// Environment variables
    pub environment_variables: HashMap<String, String>,
    
    /// Profiling tool used
    pub profiling_tool: String,
    
    /// Collection duration
    pub collection_duration: Duration,
    
    /// Number of processes profiled
    pub process_count: u32,
    
    /// Number of threads profiled
    pub thread_count: u32,
    
    /// Total memory usage during profiling
    pub memory_usage_mb: u64,
    
    /// CPU utilization during profiling
    pub cpu_utilization_percent: f64,
}

impl Default for ProfileMetadata {
    fn default() -> Self {
        Self {
            format_version: "1.0".to_string(),
            target_architecture: std::env::consts::ARCH.to_string(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            optimization_level: "O2".to_string(),
            program_arguments: Vec::new(),
            environment_variables: HashMap::new(),
            profiling_tool: "cursed-pgo".to_string(),
            collection_duration: Duration::ZERO,
            process_count: 1,
            thread_count: 1,
            memory_usage_mb: 0,
            cpu_utilization_percent: 0.0,
        }
    }
}

/// Loop profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopProfile {
    /// Function containing the loop
    pub function_name: String,
    
    /// Loop identifier
    pub loop_id: String,
    
    /// Average number of iterations
    pub average_iteration_count: f64,
    
    /// Total number of iterations across all executions
    pub total_iteration_count: u64,
    
    /// Number of times the loop was executed
    pub execution_count: u64,
    
    /// Average time per iteration
    pub average_iteration_time: Duration,
    
    /// Whether the loop is vectorizable
    pub is_vectorizable: bool,
    
    /// Whether the loop has data dependencies
    pub has_dependencies: bool,
    
    /// Memory access pattern within the loop
    pub memory_pattern: MemoryAccessPattern,
    
    /// Potential for optimization
    pub optimization_potential: OptimizationPotential,
}

/// Branch profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProfile {
    /// Function containing the branch
    pub function_name: String,
    
    /// Branch location identifier
    pub branch_id: String,
    
    /// Number of times branch was taken
    pub taken_count: u64,
    
    /// Number of times branch was not taken
    pub not_taken_count: u64,
    
    /// Branch prediction accuracy
    pub prediction_accuracy: f64,
    
    /// Whether this is a critical branch for optimization
    pub is_critical: bool,
}

/// Memory profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Function name
    pub function_name: String,
    
    /// Total memory allocations
    pub allocation_count: u64,
    
    /// Total memory deallocations
    pub deallocation_count: u64,
    
    /// Peak memory usage
    pub peak_memory_usage: u64,
    
    /// Average allocation size
    pub average_allocation_size: usize,
    
    /// Cache hit rate
    pub cache_hit_rate: f64,
    
    /// Memory access pattern
    pub access_pattern: MemoryAccessPattern,
    
    /// Number of page faults
    pub page_faults: u64,
}

/// Comprehensive profile analysis result
#[derive(Debug, Clone)]
pub struct ProfileAnalysis {
    /// Hot functions analysis
    pub hot_functions: Vec<HotFunction>,
    
    /// Cold functions that could be optimized for size
    pub cold_functions: Vec<String>,
    
    /// Loop analysis results
    pub loop_profiles: Vec<LoopProfile>,
    
    /// Branch analysis results
    pub branch_profiles: Vec<BranchProfile>,
    
    /// Memory analysis results
    pub memory_profiles: Vec<MemoryProfile>,
    
    /// Total execution time
    pub total_execution_time: Duration,
    
    /// Number of indirect calls
    pub indirect_call_count: u64,
    
    /// Function call graph
    pub call_graph: HashMap<String, Vec<String>>,
    
    /// Critical path analysis
    pub critical_path: Vec<String>,
    
    /// Optimization recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Target function or code region
    pub target: String,
    
    /// Type of optimization recommended
    pub optimization_type: String,
    
    /// Expected performance improvement percentage
    pub expected_improvement: f64,
    
    /// Confidence level in the recommendation
    pub confidence: f64,
    
    /// Priority of applying this optimization
    pub priority: OptimizationPriority,
    
    /// Detailed explanation
    pub explanation: String,
    
    /// Compiler flags to enable this optimization
    pub compiler_flags: Vec<String>,
}

/// Function call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSiteProfile {
    /// Caller function
    pub caller: String,
    
    /// Callee function
    pub callee: String,
    
    /// Number of calls
    pub call_count: u64,
    
    /// Total time spent in calls
    pub total_call_time: Duration,
    
    /// Average call overhead
    pub average_call_overhead: Duration,
    
    /// Whether this is a hot call site for inlining
    pub is_inline_candidate: bool,
}

/// Value profile for indirect calls and values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueProfile {
    /// Call site or value site identifier
    pub site_id: String,
    
    /// Map of values to their frequencies
    pub value_counts: HashMap<u64, u64>,
    
    /// Total number of observations
    pub total_count: u64,
    
    /// Most frequent value
    pub most_frequent_value: Option<u64>,
    
    /// Frequency of the most common value
    pub most_frequent_count: u64,
    
    /// Whether this site is a candidate for specialization
    pub is_specialization_candidate: bool,
}

impl ProfileData {
    /// Create a new empty profile data
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add function execution data
    pub fn add_function_execution(&mut self, function_name: String, count: u64) {
        *self.function_counts.entry(function_name).or_insert(0) += count;
    }
    
    /// Add basic block execution data
    pub fn add_basic_block_execution(&mut self, block_id: String, count: u64) {
        *self.basic_block_counts.entry(block_id).or_insert(0) += count;
    }
    
    /// Add edge execution data
    pub fn add_edge_execution(&mut self, edge_id: String, count: u64) {
        *self.edge_counts.entry(edge_id).or_insert(0) += count;
    }
    
    /// Get total function executions
    pub fn total_function_executions(&self) -> u64 {
        self.function_counts.values().sum()
    }
    
    /// Get most executed function
    pub fn most_executed_function(&self) -> Option<(&String, &u64)> {
        self.function_counts.iter().max_by_key(|(_, &count)| count)
    }
    
    /// Get functions above execution threshold
    pub fn get_hot_functions(&self, threshold_percentage: f64) -> Vec<String> {
        let total_executions = self.total_function_executions();
        let threshold = (total_executions as f64 * threshold_percentage / 100.0) as u64;
        
        self.function_counts
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get functions below execution threshold
    pub fn get_cold_functions(&self, threshold_percentage: f64) -> Vec<String> {
        let total_executions = self.total_function_executions();
        let threshold = (total_executions as f64 * threshold_percentage / 100.0) as u64;
        
        self.function_counts
            .iter()
            .filter(|(_, &count)| count < threshold)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Merge with another profile data
    pub fn merge(&mut self, other: ProfileData) {
        // Merge function counts
        for (function, count) in other.function_counts {
            *self.function_counts.entry(function).or_insert(0) += count;
        }
        
        // Merge basic block counts
        for (block, count) in other.basic_block_counts {
            *self.basic_block_counts.entry(block).or_insert(0) += count;
        }
        
        // Merge edge counts
        for (edge, count) in other.edge_counts {
            *self.edge_counts.entry(edge).or_insert(0) += count;
        }
        
        // Update execution time
        self.total_execution_time += other.total_execution_time;
        
        // Merge value profiles
        for (site, count) in other.value_profiles {
            *self.value_profiles.entry(site).or_insert(0) += count;
        }
    }
    
    /// Validate profile data consistency
    pub fn validate(&self) -> Result<(), String> {
        // Check for negative or invalid values
        for (function, &count) in &self.function_counts {
            if count == 0 {
                return Err(format!("Function {} has zero execution count", function));
            }
        }
        
        // Check metadata consistency
        if self.metadata.collection_duration > self.total_execution_time * 2 {
            return Err("Collection duration suspiciously longer than execution time".to_string());
        }
        
        Ok(())
    }
}

/// Serialize profile data to JSON
pub fn serialize_to_json(profile_data: &ProfileData) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(profile_data)
}

/// Deserialize profile data from JSON
pub fn deserialize_from_json(json_data: &str) -> Result<ProfileData, serde_json::Error> {
    serde_json::from_str(json_data)
}

/// Convert profile data to LLVM profdata format
pub fn to_llvm_profdata_format(profile_data: &ProfileData) -> String {
    let mut output = String::new();
    
    // Write header
    output.push_str("# LLVM Profile Data\n");
    output.push_str(&format!("# Generated by CURSED compiler\n"));
    output.push_str(&format!("# Collection time: {:?}\n", profile_data.collection_timestamp));
    output.push_str("\n");
    
    // Write function profiles
    for (function_name, &count) in &profile_data.function_counts {
        output.push_str(&format!("{}:{}\n", function_name, count));
    }
    
    output
}

/// Parse LLVM profdata format
pub fn from_llvm_profdata_format(data: &str) -> Result<ProfileData, String> {
    let mut profile_data = ProfileData::new();
    
    for line in data.lines() {
        // Skip comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        
        // Parse function:count format
        if let Some((function, count_str)) = line.split_once(':') {
            if let Ok(count) = count_str.parse::<u64>() {
                profile_data.function_counts.insert(function.to_string(), count);
            } else {
                return Err(format!("Invalid count format in line: {}", line));
            }
        } else {
            return Err(format!("Invalid line format: {}", line));
        }
    }
    
    Ok(profile_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_data_creation() {
        let profile_data = ProfileData::new();
        assert_eq!(profile_data.function_counts.len(), 0);
        assert_eq!(profile_data.total_execution_time, Duration::ZERO);
    }

    #[test]
    fn test_add_function_execution() {
        let mut profile_data = ProfileData::new();
        profile_data.add_function_execution("main".to_string(), 100);
        profile_data.add_function_execution("main".to_string(), 50);
        
        assert_eq!(profile_data.function_counts.get("main"), Some(&150));
    }

    #[test]
    fn test_hot_cold_functions() {
        let mut profile_data = ProfileData::new();
        profile_data.add_function_execution("hot_function".to_string(), 1000);
        profile_data.add_function_execution("cold_function".to_string(), 10);
        
        let hot_functions = profile_data.get_hot_functions(50.0); // Above 50%
        let cold_functions = profile_data.get_cold_functions(50.0); // Below 50%
        
        assert!(hot_functions.contains(&"hot_function".to_string()));
        assert!(cold_functions.contains(&"cold_function".to_string()));
    }

    #[test]
    fn test_serialization() {
        let profile_data = ProfileData::new();
        let json = serialize_to_json(&profile_data).unwrap();
        let deserialized = deserialize_from_json(&json).unwrap();
        
        assert_eq!(profile_data.function_counts.len(), deserialized.function_counts.len());
    }

    #[test]
    fn test_llvm_format_conversion() {
        let mut profile_data = ProfileData::new();
        profile_data.add_function_execution("test_func".to_string(), 42);
        
        let llvm_format = to_llvm_profdata_format(&profile_data);
        assert!(llvm_format.contains("test_func:42"));
        
        let parsed = from_llvm_profdata_format(&llvm_format).unwrap();
        assert_eq!(parsed.function_counts.get("test_func"), Some(&42));
    }
}
