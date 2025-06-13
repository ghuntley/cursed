/// Optimization Configuration System
/// 
/// Provides comprehensive configuration options for all optimization passes
/// and runtime optimization features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization (-O0)
    None,
    /// Basic optimization (-O1)
    Basic,
    /// Standard optimization (-O2)
    Default,
    /// Aggressive optimization (-O3)
    Aggressive,
    /// Optimize for size (-Os)
    Size,
    /// Maximum size optimization (-Oz)
    MinSize,
    /// Profile-guided optimization
    ProfileGuided,
    /// Debug-friendly optimization
    Debug,
}

impl OptimizationLevel {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "0" | "o0" | "none" => Ok(OptimizationLevel::None),
            "1" | "o1" | "basic" => Ok(OptimizationLevel::Basic),
            "2" | "o2" | "default" => Ok(OptimizationLevel::Default),
            "3" | "o3" | "aggressive" => Ok(OptimizationLevel::Aggressive),
            "s" | "os" | "size" => Ok(OptimizationLevel::Size),
            "z" | "oz" | "minsize" => Ok(OptimizationLevel::MinSize),
            "pgo" | "profile-guided" => Ok(OptimizationLevel::ProfileGuided),
            "debug" => Ok(OptimizationLevel::Debug),
            _ => Err(format!("Unknown optimization level: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Basic => "O1",
            OptimizationLevel::Default => "O2",
            OptimizationLevel::Aggressive => "O3",
            OptimizationLevel::Size => "Os",
            OptimizationLevel::MinSize => "Oz",
            OptimizationLevel::ProfileGuided => "PGO",
            OptimizationLevel::Debug => "Og",
        }
    }

    /// Get the relative optimization aggressiveness (0.0 to 1.0)
    pub fn aggressiveness(&self) -> f32 {
        match self {
            OptimizationLevel::None => 0.0,
            OptimizationLevel::Debug => 0.1,
            OptimizationLevel::Basic => 0.3,
            OptimizationLevel::Default => 0.6,
            OptimizationLevel::Size => 0.7,
            OptimizationLevel::MinSize => 0.8,
            OptimizationLevel::Aggressive => 0.9,
            OptimizationLevel::ProfileGuided => 1.0,
        }
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}

/// Configuration for compiler optimization passes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassConfig {
    /// Dead code elimination
    pub dead_code_elimination: bool,
    /// Constant propagation and folding
    pub constant_propagation: bool,
    /// Loop optimizations
    pub loop_optimization: LoopOptimizationConfig,
    /// Function inlining
    pub inlining: InliningConfig,
    /// Register allocation optimization
    pub register_allocation: RegisterAllocationConfig,
    /// Control flow optimization
    pub control_flow_optimization: bool,
    /// Tail call optimization
    pub tail_call_optimization: bool,
    /// Vector optimization
    pub vectorization: VectorizationConfig,
    /// Memory optimization
    pub memory_optimization: MemoryOptimizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopOptimizationConfig {
    /// Enable loop unrolling
    pub unrolling: bool,
    /// Maximum unroll count
    pub max_unroll_count: u32,
    /// Loop invariant code motion
    pub invariant_code_motion: bool,
    /// Loop fusion
    pub loop_fusion: bool,
    /// Loop interchange
    pub loop_interchange: bool,
    /// Strength reduction
    pub strength_reduction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InliningConfig {
    /// Enable function inlining
    pub enabled: bool,
    /// Maximum function size for inlining (in IR instructions)
    pub max_inline_size: u32,
    /// Maximum inlining depth
    pub max_inline_depth: u32,
    /// Aggressive inlining for hot functions
    pub aggressive_hot_inlining: bool,
    /// Size threshold for always inlining
    pub always_inline_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAllocationConfig {
    /// Register allocation algorithm
    pub algorithm: RegisterAllocationAlgorithm,
    /// Enable register coalescing
    pub coalescing: bool,
    /// Enable register rematerialization
    pub rematerialization: bool,
    /// Spill code optimization
    pub spill_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegisterAllocationAlgorithm {
    /// Linear scan allocation
    LinearScan,
    /// Graph coloring allocation
    GraphColoring,
    /// Greedy allocation
    Greedy,
    /// Advanced allocation with machine learning
    MLGuided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizationConfig {
    /// Enable auto-vectorization
    pub enabled: bool,
    /// Loop vectorization
    pub loop_vectorization: bool,
    /// SLP (superword-level parallelism) vectorization
    pub slp_vectorization: bool,
    /// Target vector width
    pub vector_width: Option<u32>,
    /// Minimum trip count for vectorization
    pub min_trip_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimizationConfig {
    /// Memory layout optimization
    pub layout_optimization: bool,
    /// Structure padding optimization
    pub padding_optimization: bool,
    /// Cache-friendly data structure transformation
    pub cache_optimization: bool,
    /// Memory prefetching hints
    pub prefetching: bool,
    /// Array-of-structures to structure-of-arrays transformation
    pub aos_to_soa: bool,
}

/// Runtime optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeOptimizationConfig {
    /// JIT compilation settings
    pub jit: JitOptimizationConfig,
    /// Profile-guided optimization
    pub pgo: PgoConfig,
    /// Memory management optimization
    pub memory_management: MemoryManagementConfig,
    /// Garbage collection optimization
    pub gc_optimization: GcOptimizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitOptimizationConfig {
    /// Enable JIT compilation
    pub enabled: bool,
    /// Hot function threshold (execution count)
    pub hot_function_threshold: u64,
    /// Compilation tier strategy
    pub tier_strategy: TierStrategy,
    /// Background compilation
    pub background_compilation: bool,
    /// Deoptimization support
    pub deoptimization: bool,
    /// OSR (On-Stack Replacement) support
    pub osr_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TierStrategy {
    /// Single tier with optimization
    SingleTier,
    /// Two-tier: interpreter + optimized
    TwoTier,
    /// Three-tier: interpreter + quick + optimized
    ThreeTier,
    /// Adaptive tiering
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgoConfig {
    /// Enable profile-guided optimization
    pub enabled: bool,
    /// Profile data collection
    pub profile_collection: bool,
    /// Use existing profile data
    pub use_profile_data: bool,
    /// Profile data file path
    pub profile_data_path: Option<String>,
    /// Training runs before optimization
    pub training_runs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryManagementConfig {
    /// Memory pool optimization
    pub pool_optimization: bool,
    /// Memory prefetching
    pub prefetching: bool,
    /// Cache-aware allocation
    pub cache_aware_allocation: bool,
    /// Memory compaction
    pub compaction: bool,
    /// NUMA awareness
    pub numa_awareness: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcOptimizationConfig {
    /// Parallel garbage collection
    pub parallel_gc: bool,
    /// Incremental garbage collection
    pub incremental_gc: bool,
    /// Generational garbage collection
    pub generational_gc: bool,
    /// Concurrent garbage collection
    pub concurrent_gc: bool,
    /// GC algorithm selection
    pub algorithm: GcAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GcAlgorithm {
    /// Mark and sweep
    MarkAndSweep,
    /// Copying collector
    Copying,
    /// Mark compact
    MarkCompact,
    /// Generational
    Generational,
    /// Concurrent mark sweep
    ConcurrentMarkSweep,
    /// G1-style collector
    G1,
}

/// Build system optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOptimizationConfig {
    /// Parallel compilation settings
    pub parallel_compilation: ParallelCompilationConfig,
    /// Incremental compilation
    pub incremental_compilation: IncrementalCompilationConfig,
    /// Link-time optimization
    pub lto: LtoConfig,
    /// Debug information optimization
    pub debug_info: DebugInfoConfig,
    /// Caching strategies
    pub caching: CachingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCompilationConfig {
    /// Enable parallel compilation
    pub enabled: bool,
    /// Number of parallel jobs (0 = auto-detect)
    pub job_count: u32,
    /// Dependency graph optimization
    pub dependency_optimization: bool,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Static load balancing
    Static,
    /// Dynamic work stealing
    WorkStealing,
    /// Priority-based scheduling
    Priority,
    /// Machine learning guided
    MLGuided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalCompilationConfig {
    /// Enable incremental compilation
    pub enabled: bool,
    /// Dependency tracking granularity
    pub dependency_granularity: DependencyGranularity,
    /// Change detection strategy
    pub change_detection: ChangeDetectionStrategy,
    /// Cache invalidation strategy
    pub cache_invalidation: CacheInvalidationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyGranularity {
    /// File-level dependencies
    File,
    /// Function-level dependencies
    Function,
    /// Statement-level dependencies
    Statement,
    /// Expression-level dependencies
    Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeDetectionStrategy {
    /// Timestamp-based detection
    Timestamp,
    /// Content hash-based detection
    ContentHash,
    /// Hybrid approach
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheInvalidationStrategy {
    /// Conservative invalidation
    Conservative,
    /// Aggressive invalidation
    Aggressive,
    /// Smart invalidation with dependency analysis
    Smart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtoConfig {
    /// Enable link-time optimization
    pub enabled: bool,
    /// LTO mode
    pub mode: LtoMode,
    /// Cross-module optimization
    pub cross_module_optimization: bool,
    /// Whole program optimization
    pub whole_program_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LtoMode {
    /// Thin LTO
    Thin,
    /// Full LTO
    Full,
    /// Fat LTO (both thin and full objects)
    Fat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfoConfig {
    /// Debug info optimization level
    pub optimization_level: DebugInfoLevel,
    /// Compress debug info
    pub compression: bool,
    /// Split debug info
    pub split_debug_info: bool,
    /// Debug info linking optimization
    pub linking_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugInfoLevel {
    /// No debug info
    None,
    /// Line tables only
    LineTablesOnly,
    /// Basic debug info
    Basic,
    /// Full debug info
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Enable compilation caching
    pub enabled: bool,
    /// Cache directory
    pub cache_directory: Option<String>,
    /// Cache size limit (in MB)
    pub size_limit: Option<u64>,
    /// Cache eviction strategy
    pub eviction_strategy: CacheEvictionStrategy,
    /// Distributed caching
    pub distributed_caching: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEvictionStrategy {
    /// Least recently used
    LRU,
    /// Least frequently used
    LFU,
    /// Time-based eviction
    TimeToLive,
    /// Size-based eviction
    SizeBased,
}

/// Profiling and monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    /// CPU profiling
    pub cpu_profiling: CpuProfilingConfig,
    /// Memory profiling
    pub memory_profiling: MemoryProfilingConfig,
    /// Performance counters
    pub performance_counters: PerformanceCountersConfig,
    /// Benchmark framework
    pub benchmarking: BenchmarkingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfilingConfig {
    /// Enable CPU profiling
    pub enabled: bool,
    /// Sampling rate (samples per second)
    pub sampling_rate: u32,
    /// Stack trace depth
    pub stack_depth: u32,
    /// Profile hot functions only
    pub hot_functions_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfilingConfig {
    /// Enable memory profiling
    pub enabled: bool,
    /// Track allocations
    pub track_allocations: bool,
    /// Track deallocations
    pub track_deallocations: bool,
    /// Memory leak detection
    pub leak_detection: bool,
    /// Heap analysis
    pub heap_analysis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCountersConfig {
    /// Enable performance counters
    pub enabled: bool,
    /// Hardware counters
    pub hardware_counters: Vec<String>,
    /// Software counters
    pub software_counters: Vec<String>,
    /// Counter sampling interval
    pub sampling_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingConfig {
    /// Enable benchmarking
    pub enabled: bool,
    /// Benchmark iterations
    pub iterations: u32,
    /// Warmup iterations
    pub warmup_iterations: u32,
    /// Statistical analysis
    pub statistical_analysis: bool,
    /// Performance regression detection
    pub regression_detection: bool,
}

/// Main optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Overall optimization level
    pub level: OptimizationLevel,
    /// Enable runtime optimizations
    pub enable_runtime_optimizations: bool,
    /// Enable profiling and monitoring
    pub enable_profiling: bool,
    /// Maximum optimization time budget
    pub time_budget: Duration,
    /// Custom optimization flags
    pub custom_flags: HashMap<String, String>,
    
    /// Specific configuration sections
    pub compiler_passes: PassConfig,
    pub runtime_optimization: RuntimeOptimizationConfig,
    pub build_optimization: BuildOptimizationConfig,
    pub profiling: ProfilingConfig,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            level: OptimizationLevel::Default,
            enable_runtime_optimizations: true,
            enable_profiling: false,
            time_budget: Duration::from_secs(60),
            custom_flags: HashMap::new(),
            compiler_passes: PassConfig::default(),
            runtime_optimization: RuntimeOptimizationConfig::default(),
            build_optimization: BuildOptimizationConfig::default(),
            profiling: ProfilingConfig::default(),
        }
    }
}

// Default implementations for all config structs
impl Default for PassConfig {
    fn default() -> Self {
        Self {
            dead_code_elimination: true,
            constant_propagation: true,
            loop_optimization: LoopOptimizationConfig::default(),
            inlining: InliningConfig::default(),
            register_allocation: RegisterAllocationConfig::default(),
            control_flow_optimization: true,
            tail_call_optimization: true,
            vectorization: VectorizationConfig::default(),
            memory_optimization: MemoryOptimizationConfig::default(),
        }
    }
}

impl Default for LoopOptimizationConfig {
    fn default() -> Self {
        Self {
            unrolling: true,
            max_unroll_count: 8,
            invariant_code_motion: true,
            loop_fusion: true,
            loop_interchange: false,
            strength_reduction: true,
        }
    }
}

impl Default for InliningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_inline_size: 100,
            max_inline_depth: 5,
            aggressive_hot_inlining: true,
            always_inline_threshold: 10,
        }
    }
}

impl Default for RegisterAllocationConfig {
    fn default() -> Self {
        Self {
            algorithm: RegisterAllocationAlgorithm::GraphColoring,
            coalescing: true,
            rematerialization: true,
            spill_optimization: true,
        }
    }
}

impl Default for VectorizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            loop_vectorization: true,
            slp_vectorization: true,
            vector_width: None,
            min_trip_count: 4,
        }
    }
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            layout_optimization: true,
            padding_optimization: true,
            cache_optimization: true,
            prefetching: false,
            aos_to_soa: false,
        }
    }
}

impl Default for RuntimeOptimizationConfig {
    fn default() -> Self {
        Self {
            jit: JitOptimizationConfig::default(),
            pgo: PgoConfig::default(),
            memory_management: MemoryManagementConfig::default(),
            gc_optimization: GcOptimizationConfig::default(),
        }
    }
}

impl Default for JitOptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hot_function_threshold: 100,
            tier_strategy: TierStrategy::TwoTier,
            background_compilation: true,
            deoptimization: true,
            osr_support: false,
        }
    }
}

impl Default for PgoConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            profile_collection: false,
            use_profile_data: false,
            profile_data_path: None,
            training_runs: 3,
        }
    }
}

impl Default for MemoryManagementConfig {
    fn default() -> Self {
        Self {
            pool_optimization: true,
            prefetching: false,
            cache_aware_allocation: true,
            compaction: true,
            numa_awareness: false,
        }
    }
}

impl Default for GcOptimizationConfig {
    fn default() -> Self {
        Self {
            parallel_gc: true,
            incremental_gc: true,
            generational_gc: true,
            concurrent_gc: false,
            algorithm: GcAlgorithm::Generational,
        }
    }
}

impl Default for BuildOptimizationConfig {
    fn default() -> Self {
        Self {
            parallel_compilation: ParallelCompilationConfig::default(),
            incremental_compilation: IncrementalCompilationConfig::default(),
            lto: LtoConfig::default(),
            debug_info: DebugInfoConfig::default(),
            caching: CachingConfig::default(),
        }
    }
}

impl Default for ParallelCompilationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            job_count: 0, // Auto-detect
            dependency_optimization: true,
            load_balancing: LoadBalancingStrategy::WorkStealing,
        }
    }
}

impl Default for IncrementalCompilationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dependency_granularity: DependencyGranularity::Function,
            change_detection: ChangeDetectionStrategy::ContentHash,
            cache_invalidation: CacheInvalidationStrategy::Smart,
        }
    }
}

impl Default for LtoConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: LtoMode::Thin,
            cross_module_optimization: true,
            whole_program_optimization: false,
        }
    }
}

impl Default for DebugInfoConfig {
    fn default() -> Self {
        Self {
            optimization_level: DebugInfoLevel::Basic,
            compression: true,
            split_debug_info: false,
            linking_optimization: true,
        }
    }
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_directory: None,
            size_limit: Some(1024), // 1GB
            eviction_strategy: CacheEvictionStrategy::LRU,
            distributed_caching: false,
        }
    }
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            cpu_profiling: CpuProfilingConfig::default(),
            memory_profiling: MemoryProfilingConfig::default(),
            performance_counters: PerformanceCountersConfig::default(),
            benchmarking: BenchmarkingConfig::default(),
        }
    }
}

impl Default for CpuProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sampling_rate: 1000,
            stack_depth: 64,
            hot_functions_only: false,
        }
    }
}

impl Default for MemoryProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            track_allocations: true,
            track_deallocations: true,
            leak_detection: true,
            heap_analysis: false,
        }
    }
}

impl Default for PerformanceCountersConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            hardware_counters: vec![],
            software_counters: vec![],
            sampling_interval: Duration::from_millis(100),
        }
    }
}

impl Default for BenchmarkingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            iterations: 100,
            warmup_iterations: 10,
            statistical_analysis: true,
            regression_detection: true,
        }
    }
}

/// Configuration utility functions
impl OptimizationConfig {
    /// Create configuration for development builds
    pub fn for_development() -> Self {
        Self {
            level: OptimizationLevel::Debug,
            enable_runtime_optimizations: false,
            enable_profiling: true,
            time_budget: Duration::from_secs(10),
            ..Default::default()
        }
    }

    /// Create configuration for release builds
    pub fn for_release() -> Self {
        Self {
            level: OptimizationLevel::Aggressive,
            enable_runtime_optimizations: true,
            enable_profiling: false,
            time_budget: Duration::from_secs(300),
            ..Default::default()
        }
    }

    /// Create configuration for size-optimized builds
    pub fn for_size() -> Self {
        Self {
            level: OptimizationLevel::MinSize,
            enable_runtime_optimizations: false,
            enable_profiling: false,
            time_budget: Duration::from_secs(180),
            ..Default::default()
        }
    }

    /// Create configuration for profile-guided optimization
    pub fn for_pgo() -> Self {
        let mut config = Self::for_release();
        config.level = OptimizationLevel::ProfileGuided;
        config.runtime_optimization.pgo.enabled = true;
        config.enable_profiling = true;
        config
    }

    /// Validate configuration consistency
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check time budget
        if self.time_budget < Duration::from_secs(1) {
            errors.push("Time budget too small (minimum 1 second)".to_string());
        }

        // Check PGO configuration
        if self.level == OptimizationLevel::ProfileGuided 
           && !self.runtime_optimization.pgo.enabled {
            errors.push("PGO optimization level requires PGO to be enabled".to_string());
        }

        // Check JIT configuration
        if self.runtime_optimization.jit.enabled 
           && !self.enable_runtime_optimizations {
            errors.push("JIT compilation requires runtime optimizations to be enabled".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Estimate optimization time
    pub fn estimate_optimization_time(&self) -> Duration {
        let base_time = Duration::from_secs(1);
        let multiplier = match self.level {
            OptimizationLevel::None => 0.1,
            OptimizationLevel::Debug => 0.2,
            OptimizationLevel::Basic => 0.5,
            OptimizationLevel::Default => 1.0,
            OptimizationLevel::Size => 1.5,
            OptimizationLevel::MinSize => 2.0,
            OptimizationLevel::Aggressive => 3.0,
            OptimizationLevel::ProfileGuided => 5.0,
        };

        Duration::from_secs_f64(base_time.as_secs_f64() * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_level_from_str() {
        assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
        assert_eq!(OptimizationLevel::from_str("O3").unwrap(), OptimizationLevel::Aggressive);
        assert_eq!(OptimizationLevel::from_str("pgo").unwrap(), OptimizationLevel::ProfileGuided);
        assert!(OptimizationLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_optimization_config_presets() {
        let dev_config = OptimizationConfig::for_development();
        assert_eq!(dev_config.level, OptimizationLevel::Debug);
        assert!(!dev_config.enable_runtime_optimizations);
        assert!(dev_config.enable_profiling);

        let release_config = OptimizationConfig::for_release();
        assert_eq!(release_config.level, OptimizationLevel::Aggressive);
        assert!(release_config.enable_runtime_optimizations);

        let size_config = OptimizationConfig::for_size();
        assert_eq!(size_config.level, OptimizationLevel::MinSize);

        let pgo_config = OptimizationConfig::for_pgo();
        assert_eq!(pgo_config.level, OptimizationLevel::ProfileGuided);
        assert!(pgo_config.runtime_optimization.pgo.enabled);
    }

    #[test]
    fn test_config_validation() {
        let mut config = OptimizationConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid time budget
        config.time_budget = Duration::from_millis(500);
        assert!(config.validate().is_err());

        config.time_budget = Duration::from_secs(60);
        
        // Test PGO inconsistency
        config.level = OptimizationLevel::ProfileGuided;
        config.runtime_optimization.pgo.enabled = false;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_optimization_time_estimation() {
        let config = OptimizationConfig::for_development();
        let estimated_time = config.estimate_optimization_time();
        assert!(estimated_time < Duration::from_secs(1));

        let aggressive_config = OptimizationConfig::for_release();
        let aggressive_time = aggressive_config.estimate_optimization_time();
        assert!(aggressive_time > Duration::from_secs(2));
    }
}
