/// Performance Profiling Infrastructure for CURSED Compiler
/// 
/// Provides comprehensive performance monitoring, hot path detection, and optimization
/// opportunity identification with thread-safe data collection.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

use crate::error::{Error, Result};

/// Performance metrics for a single function or code region
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Function or region name
    pub name: String,
    /// Total execution count
    pub execution_count: u64,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Average execution time
    pub average_execution_time: Duration,
    /// Minimum execution time
    pub min_execution_time: Duration,
    /// Maximum execution time
    pub max_execution_time: Duration,
    /// Memory allocations during execution
    pub memory_allocations: u64,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Cache misses (if available)
    pub cache_misses: u64,
    /// Branch mispredictions (if available)
    pub branch_mispredictions: u64,
    /// Is this a hot path?
    pub is_hot_path: bool,
    /// Optimization opportunities identified
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

impl PerformanceMetrics {
    pub fn new(name: String) -> Self {
        Self {
            name,
            execution_count: 0,
            total_execution_time: Duration::default(),
            average_execution_time: Duration::default(),
            min_execution_time: Duration::MAX,
            max_execution_time: Duration::default(),
            memory_allocations: 0,
            peak_memory_usage: 0,
            cache_misses: 0,
            branch_mispredictions: 0,
            is_hot_path: false,
            optimization_opportunities: Vec::new(),
        }
    }

    /// Update metrics with a new execution sample
    pub fn update(&mut self, execution_time: Duration, memory_allocated: u64, memory_peak: usize) {
        self.execution_count += 1;
        self.total_execution_time += execution_time;
        self.memory_allocations += memory_allocated;
        self.peak_memory_usage = self.peak_memory_usage.max(memory_peak);

        if execution_time < self.min_execution_time {
            self.min_execution_time = execution_time;
        }
        if execution_time > self.max_execution_time {
            self.max_execution_time = execution_time;
        }

        self.average_execution_time = self.total_execution_time / self.execution_count as u32;
        
        // Determine if this is a hot path (executed frequently or takes significant time)
        self.is_hot_path = self.execution_count > 100 || 
                          self.total_execution_time > Duration::from_millis(1000);
    }

    /// Calculate execution frequency (executions per second)
    pub fn execution_frequency(&self) -> f64 {
        if self.total_execution_time.is_zero() {
            return 0.0;
        }
        self.execution_count as f64 / self.total_execution_time.as_secs_f64()
    }

    /// Calculate relative performance cost
    pub fn performance_cost(&self) -> f64 {
        self.total_execution_time.as_nanos() as f64 * self.execution_count as f64
    }
}

/// Types of optimization opportunities
#[derive(Debug, Clone)]
pub enum OptimizationOpportunity {
    /// Function should be inlined
    InlineCandidate { 
        call_count: u64, 
        function_size: usize 
    },
    /// Loop can be optimized
    LoopOptimization { 
        loop_id: String, 
        iteration_count: u64 
    },
    /// Memory layout can be improved
    MemoryLayout { 
        allocation_count: u64, 
        fragmentation_ratio: f64 
    },
    /// Code can be vectorized
    Vectorization { 
        loop_id: String, 
        data_parallelism: usize 
    },
    /// Dead code elimination opportunity
    DeadCodeElimination { 
        unused_functions: Vec<String> 
    },
    /// Constant folding opportunity
    ConstantFolding { 
        constant_expressions: u64 
    },
}

/// Compilation performance metrics
#[derive(Debug, Clone)]
pub struct CompilationMetrics {
    /// Total compilation time
    pub total_compilation_time: Duration,
    /// Parsing time
    pub parsing_time: Duration,
    /// Type checking time
    pub type_checking_time: Duration,
    /// LLVM IR generation time
    pub llvm_generation_time: Duration,
    /// LLVM optimization time
    pub llvm_optimization_time: Duration,
    /// Linking time
    pub linking_time: Duration,
    /// Memory used during compilation
    pub peak_memory_usage: usize,
    /// Number of compiled functions
    pub compiled_functions: usize,
    /// Lines of code compiled
    pub lines_of_code: usize,
}

impl CompilationMetrics {
    pub fn new() -> Self {
        Self {
            total_compilation_time: Duration::default(),
            parsing_time: Duration::default(),
            type_checking_time: Duration::default(),
            llvm_generation_time: Duration::default(),
            llvm_optimization_time: Duration::default(),
            linking_time: Duration::default(),
            peak_memory_usage: 0,
            compiled_functions: 0,
            lines_of_code: 0,
        }
    }

    /// Calculate compilation throughput (lines per second)
    pub fn compilation_throughput(&self) -> f64 {
        if self.total_compilation_time.is_zero() {
            return 0.0;
        }
        self.lines_of_code as f64 / self.total_compilation_time.as_secs_f64()
    }
}

/// Runtime performance metrics
#[derive(Debug, Clone)]
pub struct RuntimeMetrics {
    /// Garbage collection metrics
    pub gc_metrics: GcMetrics,
    /// Memory usage statistics
    pub memory_usage: MemoryUsageStats,
    /// Thread performance
    pub thread_metrics: HashMap<String, ThreadMetrics>,
}

#[derive(Debug, Clone)]
pub struct GcMetrics {
    pub total_collections: u64,
    pub total_gc_time: Duration,
    pub average_gc_time: Duration,
    pub memory_freed: usize,
    pub memory_peak: usize,
}

#[derive(Debug, Clone)]
pub struct MemoryUsageStats {
    pub heap_size: usize,
    pub heap_used: usize,
    pub heap_free: usize,
    pub stack_size: usize,
    pub allocations_per_second: f64,
}

#[derive(Debug, Clone)]
pub struct ThreadMetrics {
    pub cpu_time: Duration,
    pub context_switches: u64,
    pub cache_misses: u64,
}

/// Main performance profiler
pub struct PerformanceProfiler {
    /// Configuration
    config: ProfilerConfig,
    /// Function-level performance metrics
    function_metrics: Arc<RwLock<HashMap<String, PerformanceMetrics>>>,
    /// Compilation metrics
    compilation_metrics: Arc<Mutex<CompilationMetrics>>,
    /// Runtime metrics
    runtime_metrics: Arc<Mutex<RuntimeMetrics>>,
    /// Active profiling sessions
    active_sessions: Arc<Mutex<HashMap<String, ProfilingSession>>>,
    /// Hot path detector
    hot_path_detector: Arc<Mutex<HotPathDetector>>,
}

#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    /// Enable detailed function profiling
    pub enable_function_profiling: bool,
    /// Enable compilation profiling
    pub enable_compilation_profiling: bool,
    /// Enable runtime profiling
    pub enable_runtime_profiling: bool,
    /// Hot path detection threshold
    pub hot_path_threshold: u64,
    /// Maximum number of tracked functions
    pub max_tracked_functions: usize,
    /// Profiling sample rate (0.0 to 1.0)
    pub sample_rate: f64,
    /// Enable hardware performance counters
    pub enable_hardware_counters: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enable_function_profiling: true,
            enable_compilation_profiling: true,
            enable_runtime_profiling: true,
            hot_path_threshold: 100,
            max_tracked_functions: 10000,
            sample_rate: 1.0,
            enable_hardware_counters: false,
        }
    }
}

/// Active profiling session
#[derive(Debug)]
pub struct ProfilingSession {
    name: String,
    start_time: Instant,
    memory_start: usize,
    memory_allocations: u64,
}

impl ProfilingSession {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_time: Instant::now(),
            memory_start: 0, // Would integrate with memory allocator
            memory_allocations: 0,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Hot path detection and analysis
#[derive(Debug)]
pub struct HotPathDetector {
    /// Execution frequency tracking
    execution_counts: HashMap<String, u64>,
    /// Time-based hot path detection
    time_based_hotness: HashMap<String, Duration>,
    /// Hot path threshold
    threshold: u64,
}

impl HotPathDetector {
    pub fn new(threshold: u64) -> Self {
        Self {
            execution_counts: HashMap::new(),
            time_based_hotness: HashMap::new(),
            threshold,
        }
    }

    /// Record function execution
    pub fn record_execution(&mut self, function_name: &str, execution_time: Duration) {
        *self.execution_counts.entry(function_name.to_string()).or_insert(0) += 1;
        *self.time_based_hotness.entry(function_name.to_string()).or_insert(Duration::default()) += execution_time;
    }

    /// Get hot paths based on execution count
    pub fn get_hot_paths_by_count(&self) -> Vec<(String, u64)> {
        let mut hot_paths: Vec<_> = self.execution_counts
            .iter()
            .filter(|(_, &count)| count >= self.threshold)
            .map(|(name, &count)| (name.clone(), count))
            .collect();
        
        hot_paths.sort_by(|a, b| b.1.cmp(&a.1));
        hot_paths
    }

    /// Get hot paths based on total execution time
    pub fn get_hot_paths_by_time(&self) -> Vec<(String, Duration)> {
        let mut hot_paths: Vec<_> = self.time_based_hotness
            .iter()
            .filter(|(_, &time)| time > Duration::from_millis(100))
            .map(|(name, &time)| (name.clone(), time))
            .collect();
        
        hot_paths.sort_by(|a, b| b.1.cmp(&a.1));
        hot_paths
    }
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    #[instrument]
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        let profiler_config = ProfilerConfig {
            enable_function_profiling: config.enable_profiling,
            enable_compilation_profiling: config.enable_profiling,
            enable_runtime_profiling: config.enable_profiling,
            ..Default::default()
        };

        Ok(Self {
            config: profiler_config.clone(),
            function_metrics: Arc::new(RwLock::new(HashMap::new())),
            compilation_metrics: Arc::new(Mutex::new(CompilationMetrics::new())),
            runtime_metrics: Arc::new(Mutex::new(RuntimeMetrics {
                gc_metrics: GcMetrics {
                    total_collections: 0,
                    total_gc_time: Duration::default(),
                    average_gc_time: Duration::default(),
                    memory_freed: 0,
                    memory_peak: 0,
                },
                memory_usage: MemoryUsageStats {
                    heap_size: 0,
                    heap_used: 0,
                    heap_free: 0,
                    stack_size: 0,
                    allocations_per_second: 0.0,
                },
                thread_metrics: HashMap::new(),
            })),
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            hot_path_detector: Arc::new(Mutex::new(HotPathDetector::new(profiler_config.hot_path_threshold))),
        })
    }

    /// Start profiling a function or code region
    #[instrument(skip(self))]
    pub fn start_profiling(&self, name: &str) -> Result<String> {
        if !self.config.enable_function_profiling {
            return Ok(String::new());
        }

        let session_id = format!("{}_{}", name, Instant::now().elapsed().as_nanos());
        let session = ProfilingSession::new(name.to_string());
        
        self.active_sessions
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire profiling session lock".to_string()))?
            .insert(session_id.clone(), session);
        
        debug!("Started profiling session: {}", session_id);
        Ok(session_id)
    }

    /// End profiling session and record metrics
    #[instrument(skip(self))]
    pub fn end_profiling(&self, session_id: &str) -> Result<()> {
        if !self.config.enable_function_profiling {
            return Ok(());
        }

        let session = {
            let mut sessions = self.active_sessions
                .lock()
                .map_err(|_| Error::Runtime("Failed to acquire profiling session lock".to_string()))?;
            sessions.remove(session_id)
                .ok_or_else(|| Error::Runtime(format!("Profiling session not found: {}", session_id)))?
        };

        let execution_time = session.elapsed();
        let memory_allocated = session.memory_allocations;
        let memory_peak = session.memory_start; // Simplified

        // Update function metrics
        {
            let mut metrics = self.function_metrics
                .write()
                .map_err(|_| Error::Runtime("Failed to acquire function metrics lock".to_string()))?;
            
            let function_metrics = metrics
                .entry(session.name.clone())
                .or_insert_with(|| PerformanceMetrics::new(session.name.clone()));
            
            function_metrics.update(execution_time, memory_allocated, memory_peak);
        }

        // Update hot path detector
        {
            let mut detector = self.hot_path_detector
                .lock()
                .map_err(|_| Error::Runtime("Failed to acquire hot path detector lock".to_string()))?;
            detector.record_execution(&session.name, execution_time);
        }

        debug!("Ended profiling session: {} ({}μs)", session_id, execution_time.as_micros());
        Ok(())
    }

    /// Record compilation metrics
    #[instrument(skip(self))]
    pub fn record_compilation_phase(&self, phase: CompilationPhase, duration: Duration) -> Result<()> {
        if !self.config.enable_compilation_profiling {
            return Ok(());
        }

        let mut metrics = self.compilation_metrics
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire compilation metrics lock".to_string()))?;

        match phase {
            CompilationPhase::Parsing => metrics.parsing_time += duration,
            CompilationPhase::TypeChecking => metrics.type_checking_time += duration,
            CompilationPhase::LlvmGeneration => metrics.llvm_generation_time += duration,
            CompilationPhase::LlvmOptimization => metrics.llvm_optimization_time += duration,
            CompilationPhase::Linking => metrics.linking_time += duration,
        }

        metrics.total_compilation_time += duration;
        
        debug!("Recorded compilation phase {:?}: {}μs", phase, duration.as_micros());
        Ok(())
    }

    /// Get function performance metrics
    pub fn get_function_metrics(&self, function_name: &str) -> Result<Option<PerformanceMetrics>> {
        let metrics = self.function_metrics
            .read()
            .map_err(|_| Error::Runtime("Failed to acquire function metrics lock".to_string()))?;
        
        Ok(metrics.get(function_name).cloned())
    }

    /// Get all hot paths
    pub fn get_hot_paths(&self) -> Result<Vec<(String, PerformanceMetrics)>> {
        let metrics = self.function_metrics
            .read()
            .map_err(|_| Error::Runtime("Failed to acquire function metrics lock".to_string()))?;
        
        let hot_paths: Vec<_> = metrics
            .iter()
            .filter(|(_, metrics)| metrics.is_hot_path)
            .map(|(name, metrics)| (name.clone(), metrics.clone()))
            .collect();
        
        Ok(hot_paths)
    }

    /// Get compilation performance summary
    pub fn get_compilation_summary(&self) -> Result<CompilationMetrics> {
        let metrics = self.compilation_metrics
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire compilation metrics lock".to_string()))?;
        
        Ok(metrics.clone())
    }

    /// Generate optimization recommendations
    #[instrument(skip(self))]
    pub fn generate_optimization_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();
        
        let hot_paths = self.get_hot_paths()?;
        
        for (function_name, metrics) in hot_paths {
            // Recommend inlining for frequently called small functions
            if metrics.execution_count > 1000 && metrics.average_execution_time < Duration::from_micros(10) {
                recommendations.push(OptimizationRecommendation {
                    function_name: function_name.clone(),
                    optimization_type: OptimizationType::Inlining,
                    priority: OptimizationPriority::High,
                    estimated_benefit: EstimatedBenefit::ExecutionTime(metrics.total_execution_time / 10),
                });
            }
            
            // Recommend loop optimization for functions with high iteration counts
            if metrics.execution_count > 500 && metrics.average_execution_time > Duration::from_millis(1) {
                recommendations.push(OptimizationRecommendation {
                    function_name: function_name.clone(),
                    optimization_type: OptimizationType::LoopOptimization,
                    priority: OptimizationPriority::Medium,
                    estimated_benefit: EstimatedBenefit::ExecutionTime(metrics.total_execution_time / 5),
                });
            }
            
            // Recommend memory optimization for memory-intensive functions
            if metrics.memory_allocations > 10000 {
                recommendations.push(OptimizationRecommendation {
                    function_name: function_name.clone(),
                    optimization_type: OptimizationType::MemoryLayout,
                    priority: OptimizationPriority::Medium,
                    estimated_benefit: EstimatedBenefit::MemoryReduction(metrics.peak_memory_usage / 4),
                });
            }
        }
        
        // Sort by priority and estimated benefit
        recommendations.sort_by(|a, b| {
            let priority_cmp = b.priority.cmp(&a.priority);
            if priority_cmp != std::cmp::Ordering::Equal {
                priority_cmp
            } else {
                b.estimated_benefit.value().partial_cmp(&a.estimated_benefit.value()).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        
        info!("Generated {} optimization recommendations", recommendations.len());
        Ok(recommendations)
    }

    /// Reset all metrics
    pub fn reset(&self) -> Result<()> {
        self.function_metrics
            .write()
            .map_err(|_| Error::Runtime("Failed to acquire function metrics lock".to_string()))?
            .clear();
        
        *self.compilation_metrics
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire compilation metrics lock".to_string()))? = CompilationMetrics::new();
        
        self.active_sessions
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire profiling session lock".to_string()))?
            .clear();
        
        info!("Reset all performance metrics");
        Ok(())
    }
}

/// Compilation phases for performance tracking
#[derive(Debug, Clone, Copy)]
pub enum CompilationPhase {
    Parsing,
    TypeChecking,
    LlvmGeneration,
    LlvmOptimization,
    Linking,
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub function_name: String,
    pub optimization_type: OptimizationType,
    pub priority: OptimizationPriority,
    pub estimated_benefit: EstimatedBenefit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum OptimizationType {
    Inlining,
    LoopOptimization,
    MemoryLayout,
    Vectorization,
    DeadCodeElimination,
    ConstantFolding,
}

#[derive(Debug, Clone)]
pub enum EstimatedBenefit {
    ExecutionTime(Duration),
    MemoryReduction(usize),
    CompilationTime(Duration),
}

impl EstimatedBenefit {
    pub fn value(&self) -> f64 {
        match self {
            EstimatedBenefit::ExecutionTime(duration) => duration.as_nanos() as f64,
            EstimatedBenefit::MemoryReduction(bytes) => *bytes as f64,
            EstimatedBenefit::CompilationTime(duration) => duration.as_nanos() as f64,
        }
    }
}

/// Compilation profiler for tracking build performance
pub struct CompilationProfiler {
    profiler: Arc<PerformanceProfiler>,
}

impl CompilationProfiler {
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        Ok(Self {
            profiler: Arc::new(PerformanceProfiler::new(config)?),
        })
    }

    /// Profile a compilation phase
    pub fn profile_phase<F, R>(&self, phase: CompilationPhase, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R>,
    {
        let start = Instant::now();
        let result = f()?;
        let duration = start.elapsed();
        
        self.profiler.record_compilation_phase(phase, duration)?;
        Ok(result)
    }

    /// Get compilation summary
    pub fn get_summary(&self) -> Result<CompilationMetrics> {
        self.profiler.get_compilation_summary()
    }
}

/// Runtime profiler for tracking execution performance
pub struct RuntimeProfiler {
    profiler: Arc<PerformanceProfiler>,
}

impl RuntimeProfiler {
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        Ok(Self {
            profiler: Arc::new(PerformanceProfiler::new(config)?),
        })
    }

    /// Profile a function execution
    pub fn profile_function<F, R>(&self, function_name: &str, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R>,
    {
        let session_id = self.profiler.start_profiling(function_name)?;
        let result = f();
        self.profiler.end_profiling(&session_id)?;
        result
    }

    /// Get hot paths
    pub fn get_hot_paths(&self) -> Result<Vec<(String, PerformanceMetrics)>> {
        self.profiler.get_hot_paths()
    }

    /// Get optimization recommendations
    pub fn get_optimization_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        self.profiler.generate_optimization_recommendations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new("test_function".to_string());
        
        // Update with sample data
        metrics.update(Duration::from_millis(10), 100, 1024);
        metrics.update(Duration::from_millis(20), 200, 2048);
        
        assert_eq!(metrics.execution_count, 2);
        assert_eq!(metrics.total_execution_time, Duration::from_millis(30));
        assert_eq!(metrics.average_execution_time, Duration::from_millis(15));
        assert_eq!(metrics.memory_allocations, 300);
        assert_eq!(metrics.peak_memory_usage, 2048);
    }

    #[test]
    fn test_profiler_session() {
        let config = super::super::OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        let session_id = profiler.start_profiling("test_function").unwrap();
        thread::sleep(Duration::from_millis(10));
        profiler.end_profiling(&session_id).unwrap();
        
        let metrics = profiler.get_function_metrics("test_function").unwrap();
        assert!(metrics.is_some());
        
        let metrics = metrics.unwrap();
        assert_eq!(metrics.execution_count, 1);
        assert!(metrics.total_execution_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_hot_path_detection() {
        let mut detector = HotPathDetector::new(5);
        
        // Record multiple executions
        for _ in 0..10 {
            detector.record_execution("hot_function", Duration::from_millis(1));
        }
        
        for _ in 0..3 {
            detector.record_execution("cold_function", Duration::from_millis(1));
        }
        
        let hot_paths = detector.get_hot_paths_by_count();
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].0, "hot_function");
        assert_eq!(hot_paths[0].1, 10);
    }
}
