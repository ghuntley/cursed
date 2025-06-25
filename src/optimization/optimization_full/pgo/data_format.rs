use crate::error::CursedError;
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
    
    /// Basic block execution counts
    
    /// Edge execution counts (control flow edges)
    
    /// Total execution time of the profiled program
    
    /// Hot functions identified during analysis
    
    /// Cold functions identified during analysis
    
    /// Value profiles for indirect calls and other dynamic values
    
    /// Sampling rate used during collection (samples per second)
    
    /// Timestamp when profile data was collected
    
    /// Additional metadata
impl Default for ProfileData {
    fn default() -> Self {
        Self {
        }
    }
/// Information about frequently executed functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotFunction {
    /// Function name
    
    /// Number of times the function was executed
    
    /// Total time spent in this function
    
    /// Average time per execution
    
    /// Percentage of total execution time
    
    /// Optimization priority based on profile analysis
    
    /// Call sites and their frequencies
    
    /// Number of direct and indirect calls
    
    /// Average function size in instructions/lines
    
    /// Whether the function has vectorizable loops
    
    /// Memory access patterns
    
    /// Branch prediction accuracy
    
    /// Cache miss rate
    
    /// Optimization potential based on analysis
/// Priority levels for optimization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationPriority {
/// Memory access patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryAccessPattern {
/// Optimization potential assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationPotential {
/// Profile metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Version of the profile format
    
    /// Target architecture
    
    /// Compiler version used
    
    /// Optimization level used during profiling
    
    /// Program arguments during profiling
    
    /// Environment variables
    
    /// Profiling tool used
    
    /// Collection duration
    
    /// Number of processes profiled
    
    /// Number of threads profiled
    
    /// Total memory usage during profiling
    
    /// CPU utilization during profiling
impl Default for ProfileMetadata {
    fn default() -> Self {
        Self {
        }
    }
/// Loop profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopProfile {
    /// Function containing the loop
    
    /// Loop identifier
    
    /// Average number of iterations
    
    /// Total number of iterations across all executions
    
    /// Number of times the loop was executed
    
    /// Average time per iteration
    
    /// Whether the loop is vectorizable
    
    /// Whether the loop has data dependencies
    
    /// Memory access pattern within the loop
    
    /// Potential for optimization
/// Branch profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProfile {
    /// Function containing the branch
    
    /// Branch location identifier
    
    /// Number of times branch was taken
    
    /// Number of times branch was not taken
    
    /// Branch prediction accuracy
    
    /// Whether this is a critical branch for optimization
/// Memory profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Function name
    
    /// Total memory allocations
    
    /// Total memory deallocations
    
    /// Peak memory usage
    
    /// Average allocation size
    
    /// Cache hit rate
    
    /// Memory access pattern
    
    /// Number of page faults
/// Comprehensive profile analysis result
#[derive(Debug, Clone)]
pub struct ProfileAnalysis {
    /// Hot functions analysis
    
    /// Cold functions that could be optimized for size
    
    /// Loop analysis results
    
    /// Branch analysis results
    
    /// Memory analysis results
    
    /// Total execution time
    
    /// Number of indirect calls
    
    /// Function call graph
    
    /// Critical path analysis
    
    /// Optimization recommendations
/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Target function or code region
    
    /// Type of optimization recommended
    
    /// Expected performance improvement percentage
    
    /// Confidence level in the recommendation
    
    /// Priority of applying this optimization
    
    /// Detailed explanation
    
    /// Compiler flags to enable this optimization
/// Function call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSiteProfile {
    /// Caller function
    
    /// Callee function
    
    /// Number of calls
    
    /// Total time spent in calls
    
    /// Average call overhead
    
    /// Whether this is a hot call site for inlining
/// Value profile for indirect calls and values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueProfile {
    /// Call site or value site identifier
    
    /// Map of values to their frequencies
    
    /// Total number of observations
    
    /// Most frequent value
    
    /// Frequency of the most common value
    
    /// Whether this site is a candidate for specialization
impl ProfileData {
    /// Create a new empty profile data
    pub fn new() -> Self {
        Self::default()
    /// Add function execution data
    pub fn add_function_execution(&mut self, function_name: String, count: u64) {
        *self.function_counts.entry(function_name).or_insert(0) += count;
    /// Add basic block execution data
    pub fn add_basic_block_execution(&mut self, block_id: String, count: u64) {
        *self.basic_block_counts.entry(block_id).or_insert(0) += count;
    /// Add edge execution data
    pub fn add_edge_execution(&mut self, edge_id: String, count: u64) {
        *self.edge_counts.entry(edge_id).or_insert(0) += count;
    /// Get total function executions
    pub fn total_function_executions(&self) -> u64 {
        self.function_counts.values().sum()
    /// Get most executed function
    pub fn most_executed_function(&self) -> Option<(&String, &u64)> {
        self.function_counts.iter().max_by_key(|(_, &count)| count)
    /// Get functions above execution threshold
    pub fn get_hot_functions(&self, threshold_percentage: f64) -> Vec<String> {
        let total_executions = self.total_function_executions();
        let threshold = (total_executions as f64 * threshold_percentage / 100.0) as u64;
        
        self.function_counts
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(name, _)| name.clone())
            .collect()
    /// Get functions below execution threshold
    pub fn get_cold_functions(&self, threshold_percentage: f64) -> Vec<String> {
        let total_executions = self.total_function_executions();
        let threshold = (total_executions as f64 * threshold_percentage / 100.0) as u64;
        
        self.function_counts
            .iter()
            .filter(|(_, &count)| count < threshold)
            .map(|(name, _)| name.clone())
            .collect()
    /// Merge with another profile data
    pub fn merge(&mut self, other: ProfileData) {
        // Merge function counts
        for (function, count) in other.function_counts {
            *self.function_counts.entry(function).or_insert(0) += count;
        // Merge basic block counts
        for (block, count) in other.basic_block_counts {
            *self.basic_block_counts.entry(block).or_insert(0) += count;
        // Merge edge counts
        for (edge, count) in other.edge_counts {
            *self.edge_counts.entry(edge).or_insert(0) += count;
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
        Ok(())
    }
}

/// Serialize profile data to JSON
pub fn serialize_to_json(profile_data: &ProfileData) -> crate::error::Result<()> {
    serde_json::to_string_pretty(profile_data)
/// Deserialize profile data from JSON
pub fn deserialize_from_json(json_data: &str) -> crate::error::Result<()> {
    serde_json::from_str(json_data)
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
    output
/// Parse LLVM profdata format
pub fn from_llvm_profdata_format(data: &str) -> Result<ProfileData, String> {
    let mut profile_data = ProfileData::new();
    
    for line in data.split("\n") {
        // Skip comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
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
