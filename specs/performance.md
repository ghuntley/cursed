# Performance Specification

## Overview

The CURSED performance specification defines optimization guarantees, benchmark requirements, and performance characteristics for the compiler and runtime system. This specification ensures consistent, predictable performance across all CURSED implementations.

## Performance Targets

### Compilation Performance

#### Build Time Targets

```
┌─────────────────┬─────────────────┬─────────────────┬─────────────────┐
│   Project Size  │   Debug Build   │  Release Build  │  Incremental    │
├─────────────────┼─────────────────┼─────────────────┼─────────────────┤
│ Small (<1K LOC) │    < 100ms      │    < 500ms      │    < 50ms       │
│ Medium (1-10K)  │    < 1s         │    < 5s         │    < 200ms      │
│ Large (10-100K) │    < 10s        │    < 30s        │    < 1s         │
│ XL (>100K LOC)  │    < 60s        │    < 180s       │    < 5s         │
└─────────────────┴─────────────────┴─────────────────┴─────────────────┘
```

#### Memory Usage Targets

```rust
pub struct CompilationMemoryTargets {
    /// Maximum memory per 1K lines of code
    pub memory_per_kloc: usize, // 8MB
    /// Peak memory usage multiplier
    pub peak_multiplier: f64, // 2.5x
    /// Memory growth rate (linear)
    pub growth_rate: f64, // 1.2
}

impl Default for CompilationMemoryTargets {
    fn default() -> Self {
        Self {
            memory_per_kloc: 8 * 1024 * 1024, // 8MB per 1K LOC
            peak_multiplier: 2.5,
            growth_rate: 1.2,
        }
    }
}
```

### Runtime Performance

#### Execution Speed Targets

```
┌─────────────────────┬─────────────────┬─────────────────┬─────────────────┐
│    Operation Type   │   Interpreted   │     JIT         │    Native       │
├─────────────────────┼─────────────────┼─────────────────┼─────────────────┤
│ Arithmetic Ops      │   50M ops/sec   │   200M ops/sec  │   500M ops/sec  │
│ Function Calls      │   10M calls/sec │   50M calls/sec │   100M calls/sec│
│ Memory Allocation   │   1M allocs/sec │   5M allocs/sec │   10M allocs/sec│
│ String Operations   │   5M ops/sec    │   20M ops/sec   │   50M ops/sec   │
│ I/O Operations      │   100K ops/sec  │   500K ops/sec  │   1M ops/sec    │
└─────────────────────┴─────────────────┴─────────────────┴─────────────────┘
```

#### Memory Performance Targets

```rust
pub struct RuntimeMemoryTargets {
    /// GC pause time targets
    pub young_gc_pause: Duration,    // < 5ms
    pub old_gc_pause: Duration,      // < 50ms
    pub concurrent_gc_pause: Duration, // < 1ms
    
    /// Memory overhead targets
    pub gc_overhead: f64,            // < 5%
    pub object_overhead: usize,      // 16 bytes
    pub heap_utilization: f64,       // > 70%
    
    /// Allocation performance
    pub allocation_latency: Duration, // < 50ns small objects
    pub throughput: u64,             // > 10M allocations/sec
}
```

## Optimization Levels

### Compilation Optimization Levels

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    /// O0 - No optimization
    None,
    /// O1 - Minimal optimization
    Less,
    /// O2 - Standard optimization (default)
    Default,
    /// O3 - Maximum optimization
    Aggressive,
    /// Os - Size optimization
    Size,
    /// Oz - Aggressive size optimization
    SizeAggressive,
    /// Production-level optimization
    Production,
}
```

#### Optimization Level Specifications

##### O0 (None)
```rust
pub struct O0Config {
    /// Compilation time target: fastest possible
    pub compile_time_target: Duration, // < 100ms per 1K LOC
    /// Debug info: maximum
    pub debug_info: DebugLevel,        // Full
    /// Optimizations: disabled
    pub enable_optimizations: bool,    // false
    /// Inline: disabled
    pub enable_inlining: bool,         // false
}
```

##### O1 (Less)
```rust
pub struct O1Config {
    /// Compilation time: fast
    pub compile_time_target: Duration, // < 200ms per 1K LOC
    /// Basic optimizations only
    pub optimizations: Vec<OptimizationPass>,
    /// Limited inlining
    pub inline_threshold: usize,       // 10 instructions
    /// Dead code elimination
    pub dead_code_elimination: bool,   // true
}
```

##### O2 (Default)
```rust
pub struct O2Config {
    /// Compilation time: balanced
    pub compile_time_target: Duration, // < 500ms per 1K LOC
    /// Standard optimization passes
    pub optimization_passes: Vec<OptimizationPass>,
    /// Moderate inlining
    pub inline_threshold: usize,       // 50 instructions
    /// Loop optimizations
    pub loop_optimizations: bool,      // true
    /// Constant folding
    pub constant_folding: bool,        // true
}
```

##### O3 (Aggressive)
```rust
pub struct O3Config {
    /// Compilation time: longer acceptable
    pub compile_time_target: Duration, // < 2s per 1K LOC
    /// All optimization passes
    pub optimization_passes: Vec<OptimizationPass>,
    /// Aggressive inlining
    pub inline_threshold: usize,       // 200 instructions
    /// Advanced optimizations
    pub vectorization: bool,           // true
    pub loop_unrolling: bool,          // true
    pub interprocedural: bool,         // true
}
```

#### Performance Guarantees by Level

```rust
pub struct OptimizationGuarantees {
    pub level: OptimizationLevel,
    pub performance_multiplier: f64,  // vs O0
    pub code_size_multiplier: f64,    // vs O0
    pub compile_time_multiplier: f64, // vs O0
}

const OPTIMIZATION_GUARANTEES: &[OptimizationGuarantees] = &[
    OptimizationGuarantees {
        level: OptimizationLevel::None,
        performance_multiplier: 1.0,
        code_size_multiplier: 1.0,
        compile_time_multiplier: 1.0,
    },
    OptimizationGuarantees {
        level: OptimizationLevel::Less,
        performance_multiplier: 1.5,
        code_size_multiplier: 0.95,
        compile_time_multiplier: 1.2,
    },
    OptimizationGuarantees {
        level: OptimizationLevel::Default,
        performance_multiplier: 2.5,
        code_size_multiplier: 0.85,
        compile_time_multiplier: 2.0,
    },
    OptimizationGuarantees {
        level: OptimizationLevel::Aggressive,
        performance_multiplier: 4.0,
        code_size_multiplier: 0.75,
        compile_time_multiplier: 5.0,
    },
];
```

## Performance Monitoring

### Compilation Performance Monitoring

```rust
pub struct CompilationPerformanceMonitor {
    /// Phase timing
    phases: HashMap<CompilationPhase, PhaseMetrics>,
    /// Memory tracking
    memory_tracker: MemoryTracker,
    /// Optimization statistics
    optimization_stats: OptimizationStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilationPhase {
    Lexing,
    Parsing,
    SemanticAnalysis,
    TypeChecking,
    Optimization,
    CodeGeneration,
    Linking,
}

pub struct PhaseMetrics {
    /// Total time spent in phase
    pub total_time: Duration,
    /// Number of invocations
    pub invocation_count: u64,
    /// Peak memory usage during phase
    pub peak_memory: usize,
    /// Lines of code processed
    pub loc_processed: u64,
}

impl CompilationPerformanceMonitor {
    pub fn start_phase(&mut self, phase: CompilationPhase) -> PhaseGuard {
        let start_time = Instant::now();
        let start_memory = self.memory_tracker.current_usage();
        
        PhaseGuard {
            phase,
            start_time,
            start_memory,
            monitor: self,
        }
    }
    
    pub fn get_performance_report(&self) -> CompilationReport {
        let total_time: Duration = self.phases.values()
            .map(|metrics| metrics.total_time)
            .sum();
        
        let peak_memory = self.phases.values()
            .map(|metrics| metrics.peak_memory)
            .max()
            .unwrap_or(0);
        
        CompilationReport {
            total_compilation_time: total_time,
            peak_memory_usage: peak_memory,
            phase_breakdown: self.phases.clone(),
            optimization_statistics: self.optimization_stats.clone(),
        }
    }
}

pub struct PhaseGuard<'a> {
    phase: CompilationPhase,
    start_time: Instant,
    start_memory: usize,
    monitor: &'a mut CompilationPerformanceMonitor,
}

impl<'a> Drop for PhaseGuard<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        let current_memory = self.monitor.memory_tracker.current_usage();
        let peak_memory = self.monitor.memory_tracker.peak_usage_since(self.start_memory);
        
        let metrics = self.monitor.phases.entry(self.phase)
            .or_insert_with(|| PhaseMetrics {
                total_time: Duration::ZERO,
                invocation_count: 0,
                peak_memory: 0,
                loc_processed: 0,
            });
        
        metrics.total_time += duration;
        metrics.invocation_count += 1;
        metrics.peak_memory = metrics.peak_memory.max(peak_memory);
    }
}
```

### Runtime Performance Monitoring

```rust
pub struct RuntimePerformanceMonitor {
    /// Execution statistics
    execution_stats: ExecutionStats,
    /// Memory statistics
    memory_stats: MemoryStats,
    /// GC statistics
    gc_stats: GcStats,
    /// Function call statistics
    call_stats: HashMap<String, CallStats>,
}

pub struct ExecutionStats {
    /// Total instructions executed
    pub instructions_executed: AtomicU64,
    /// Function calls made
    pub function_calls: AtomicU64,
    /// Total execution time
    pub execution_time: AtomicU64, // nanoseconds
    /// Hot functions (most called)
    pub hot_functions: Vec<String>,
}

pub struct CallStats {
    /// Number of calls
    pub call_count: u64,
    /// Total time spent in function
    pub total_time: Duration,
    /// Average time per call
    pub average_time: Duration,
    /// Maximum time for a single call
    pub max_time: Duration,
    /// Minimum time for a single call
    pub min_time: Duration,
}

impl RuntimePerformanceMonitor {
    pub fn record_function_call(&self, function_name: &str, duration: Duration) {
        let mut call_stats = self.call_stats.entry(function_name.to_string())
            .or_insert_with(|| CallStats {
                call_count: 0,
                total_time: Duration::ZERO,
                average_time: Duration::ZERO,
                max_time: Duration::ZERO,
                min_time: Duration::MAX,
            });
        
        call_stats.call_count += 1;
        call_stats.total_time += duration;
        call_stats.average_time = call_stats.total_time / call_stats.call_count as u32;
        call_stats.max_time = call_stats.max_time.max(duration);
        call_stats.min_time = call_stats.min_time.min(duration);
        
        self.execution_stats.function_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let total_calls = self.execution_stats.function_calls.load(Ordering::Relaxed);
        let total_time = Duration::from_nanos(
            self.execution_stats.execution_time.load(Ordering::Relaxed)
        );
        
        // Find hottest functions
        let mut hot_functions: Vec<_> = self.call_stats.iter().collect();
        hot_functions.sort_by_key(|(_, stats)| stats.call_count);
        hot_functions.reverse();
        hot_functions.truncate(10);
        
        PerformanceSummary {
            total_function_calls: total_calls,
            total_execution_time: total_time,
            average_call_time: total_time / total_calls as u32,
            hot_functions: hot_functions.into_iter()
                .map(|(name, stats)| (name.clone(), stats.clone()))
                .collect(),
            memory_usage: self.memory_stats.clone(),
            gc_statistics: self.gc_stats.clone(),
        }
    }
}
```

## Benchmark Requirements

### Compilation Benchmarks

#### Build Time Benchmarks

```rust
pub struct CompilationBenchmarks {
    /// Small project benchmark
    pub small_project: BenchmarkSuite,
    /// Medium project benchmark
    pub medium_project: BenchmarkSuite,
    /// Large project benchmark
    pub large_project: BenchmarkSuite,
    /// Incremental build benchmark
    pub incremental_build: BenchmarkSuite,
}

pub struct BenchmarkSuite {
    /// Project characteristics
    pub project_size: ProjectSize,
    /// Test cases
    pub test_cases: Vec<BenchmarkCase>,
    /// Performance targets
    pub targets: PerformanceTargets,
}

pub struct BenchmarkCase {
    /// Test name
    pub name: String,
    /// Source code
    pub source: String,
    /// Expected output
    pub expected_output: String,
    /// Performance requirements
    pub requirements: PerformanceRequirements,
}

pub struct PerformanceRequirements {
    /// Maximum compilation time
    pub max_compile_time: Duration,
    /// Maximum memory usage
    pub max_memory_usage: usize,
    /// Minimum execution speed (for runtime benchmarks)
    pub min_execution_speed: Option<f64>, // operations per second
}

impl CompilationBenchmarks {
    pub fn run_all_benchmarks(&self) -> BenchmarkResults {
        let mut results = BenchmarkResults::new();
        
        // Run small project benchmarks
        results.small_project = self.run_benchmark_suite(&self.small_project);
        
        // Run medium project benchmarks
        results.medium_project = self.run_benchmark_suite(&self.medium_project);
        
        // Run large project benchmarks
        results.large_project = self.run_benchmark_suite(&self.large_project);
        
        // Run incremental build benchmarks
        results.incremental_build = self.run_benchmark_suite(&self.incremental_build);
        
        results
    }
    
    fn run_benchmark_suite(&self, suite: &BenchmarkSuite) -> SuiteResults {
        let mut suite_results = SuiteResults::new();
        
        for test_case in &suite.test_cases {
            let case_result = self.run_benchmark_case(test_case);
            suite_results.add_case_result(case_result);
        }
        
        suite_results
    }
    
    fn run_benchmark_case(&self, case: &BenchmarkCase) -> CaseResult {
        let start_time = Instant::now();
        let start_memory = get_memory_usage();
        
        // Compile the source code
        let compilation_result = compile_source(&case.source);
        
        let compile_time = start_time.elapsed();
        let peak_memory = get_peak_memory_usage() - start_memory;
        
        // Verify compilation succeeded
        assert!(compilation_result.is_ok(), "Compilation failed for case: {}", case.name);
        
        // Check performance requirements
        let meets_time_requirement = compile_time <= case.requirements.max_compile_time;
        let meets_memory_requirement = peak_memory <= case.requirements.max_memory_usage;
        
        CaseResult {
            name: case.name.clone(),
            compile_time,
            memory_usage: peak_memory,
            meets_requirements: meets_time_requirement && meets_memory_requirement,
            compilation_success: compilation_result.is_ok(),
        }
    }
}
```

#### Runtime Benchmarks

```rust
pub struct RuntimeBenchmarks {
    /// Arithmetic operation benchmarks
    pub arithmetic: ArithmeticBenchmarks,
    /// Function call benchmarks
    pub function_calls: FunctionCallBenchmarks,
    /// Memory allocation benchmarks
    pub memory_allocation: MemoryBenchmarks,
    /// String operation benchmarks
    pub string_operations: StringBenchmarks,
    /// I/O operation benchmarks
    pub io_operations: IoBenchmarks,
}

pub struct ArithmeticBenchmarks;

impl ArithmeticBenchmarks {
    pub fn run_integer_arithmetic(&self) -> BenchmarkResult {
        let iterations = 10_000_000;
        let start = Instant::now();
        
        let mut result = 0i64;
        for i in 0..iterations {
            result += i;
            result *= 2;
            result /= 3;
            result -= 1;
        }
        
        let duration = start.elapsed();
        let ops_per_second = (iterations * 4) as f64 / duration.as_secs_f64();
        
        BenchmarkResult {
            name: "Integer Arithmetic".to_string(),
            operations_per_second: ops_per_second,
            total_time: duration,
            iterations,
            meets_target: ops_per_second >= 50_000_000.0, // 50M ops/sec target
        }
    }
    
    pub fn run_floating_arithmetic(&self) -> BenchmarkResult {
        let iterations = 10_000_000;
        let start = Instant::now();
        
        let mut result = 0.0f64;
        for i in 0..iterations {
            result += i as f64;
            result *= 2.0;
            result /= 3.0;
            result -= 1.0;
        }
        
        let duration = start.elapsed();
        let ops_per_second = (iterations * 4) as f64 / duration.as_secs_f64();
        
        BenchmarkResult {
            name: "Floating Point Arithmetic".to_string(),
            operations_per_second: ops_per_second,
            total_time: duration,
            iterations,
            meets_target: ops_per_second >= 25_000_000.0, // 25M ops/sec target
        }
    }
}

pub struct MemoryBenchmarks;

impl MemoryBenchmarks {
    pub fn run_allocation_benchmark(&self) -> BenchmarkResult {
        let iterations = 1_000_000;
        let start = Instant::now();
        
        let mut allocations = Vec::new();
        for _ in 0..iterations {
            let allocation = vec![0u8; 64]; // 64-byte allocation
            allocations.push(allocation);
        }
        
        let duration = start.elapsed();
        let allocs_per_second = iterations as f64 / duration.as_secs_f64();
        
        BenchmarkResult {
            name: "Memory Allocation".to_string(),
            operations_per_second: allocs_per_second,
            total_time: duration,
            iterations,
            meets_target: allocs_per_second >= 1_000_000.0, // 1M allocs/sec target
        }
    }
    
    pub fn run_gc_benchmark(&self) -> GcBenchmarkResult {
        let gc = create_test_gc();
        let allocation_size = 1024;
        let iterations = 100_000;
        
        let start = Instant::now();
        
        // Allocate objects that will trigger GC
        for _ in 0..iterations {
            let _ = gc.allocate(allocation_size);
        }
        
        let total_time = start.elapsed();
        let gc_stats = gc.get_stats();
        
        GcBenchmarkResult {
            total_allocation_time: total_time,
            gc_count: gc_stats.collection_count,
            total_gc_time: gc_stats.total_gc_time,
            average_gc_pause: gc_stats.average_pause_time,
            max_gc_pause: gc_stats.max_pause_time,
            meets_pause_target: gc_stats.max_pause_time < Duration::from_millis(50),
        }
    }
}
```

### Performance Regression Testing

```rust
pub struct PerformanceRegressionSuite {
    /// Historical performance data
    baseline_data: HashMap<String, BaselineMetrics>,
    /// Current benchmark results
    current_results: HashMap<String, BenchmarkResult>,
    /// Regression thresholds
    thresholds: RegressionThresholds,
}

pub struct RegressionThresholds {
    /// Maximum allowed performance regression (as percentage)
    pub max_regression_percent: f64, // 10.0 = 10% regression allowed
    /// Minimum improvement to report (as percentage)
    pub min_improvement_percent: f64, // 5.0 = report 5%+ improvements
}

impl PerformanceRegressionSuite {
    pub fn check_for_regressions(&self) -> RegressionReport {
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (benchmark_name, current) in &self.current_results {
            if let Some(baseline) = self.baseline_data.get(benchmark_name) {
                let performance_change = self.calculate_performance_change(baseline, current);
                
                if performance_change < -self.thresholds.max_regression_percent {
                    regressions.push(RegressionItem {
                        benchmark: benchmark_name.clone(),
                        baseline_performance: baseline.operations_per_second,
                        current_performance: current.operations_per_second,
                        performance_change,
                    });
                } else if performance_change > self.thresholds.min_improvement_percent {
                    improvements.push(ImprovementItem {
                        benchmark: benchmark_name.clone(),
                        baseline_performance: baseline.operations_per_second,
                        current_performance: current.operations_per_second,
                        performance_change,
                    });
                }
            }
        }
        
        RegressionReport {
            regressions,
            improvements,
            total_benchmarks: self.current_results.len(),
            passed: self.current_results.len() - regressions.len(),
        }
    }
    
    fn calculate_performance_change(&self, baseline: &BaselineMetrics, current: &BenchmarkResult) -> f64 {
        let baseline_perf = baseline.operations_per_second;
        let current_perf = current.operations_per_second;
        
        ((current_perf - baseline_perf) / baseline_perf) * 100.0
    }
}
```

## Optimization Implementation

### LLVM Optimization Pipeline

```rust
pub struct LlvmOptimizationPipeline {
    /// Function pass manager
    function_pass_manager: FunctionPassManager,
    /// Module pass manager
    module_pass_manager: ModulePassManager,
    /// Optimization level
    optimization_level: OptimizationLevel,
}

impl LlvmOptimizationPipeline {
    pub fn new(optimization_level: OptimizationLevel) -> Self {
        let mut pipeline = Self {
            function_pass_manager: FunctionPassManager::new(),
            module_pass_manager: ModulePassManager::new(),
            optimization_level,
        };
        
        pipeline.configure_passes();
        pipeline
    }
    
    fn configure_passes(&mut self) {
        match self.optimization_level {
            OptimizationLevel::None => {
                // No optimization passes
            }
            OptimizationLevel::Less => {
                self.add_basic_passes();
            }
            OptimizationLevel::Default => {
                self.add_basic_passes();
                self.add_standard_passes();
            }
            OptimizationLevel::Aggressive => {
                self.add_basic_passes();
                self.add_standard_passes();
                self.add_aggressive_passes();
            }
            OptimizationLevel::Size | OptimizationLevel::SizeAggressive => {
                self.add_size_optimization_passes();
            }
            OptimizationLevel::Production => {
                self.add_production_passes();
            }
        }
    }
    
    fn add_basic_passes(&mut self) {
        // Basic optimization passes
        self.function_pass_manager.add_dead_code_elimination_pass();
        self.function_pass_manager.add_constant_folding_pass();
        self.function_pass_manager.add_simple_alias_analysis_pass();
    }
    
    fn add_standard_passes(&mut self) {
        // Standard optimization passes
        self.function_pass_manager.add_instruction_combining_pass();
        self.function_pass_manager.add_cfg_simplification_pass();
        self.function_pass_manager.add_loop_rotate_pass();
        self.function_pass_manager.add_loop_unswitch_pass();
        self.function_pass_manager.add_sccp_pass(); // Sparse Conditional Constant Propagation
        
        // Module passes
        self.module_pass_manager.add_global_dce_pass();
        self.module_pass_manager.add_function_inlining_pass();
    }
    
    fn add_aggressive_passes(&mut self) {
        // Aggressive optimization passes
        self.function_pass_manager.add_loop_unroll_pass();
        self.function_pass_manager.add_vectorization_pass();
        self.function_pass_manager.add_gvn_pass(); // Global Value Numbering
        self.function_pass_manager.add_licm_pass(); // Loop Invariant Code Motion
        
        // Interprocedural optimization
        self.module_pass_manager.add_ipo_pass(); // Interprocedural Optimization
        self.module_pass_manager.add_argument_promotion_pass();
    }
    
    fn add_size_optimization_passes(&mut self) {
        // Size optimization specific passes
        self.function_pass_manager.add_dead_code_elimination_pass();
        self.function_pass_manager.add_cfg_simplification_pass();
        self.module_pass_manager.add_global_dce_pass();
        
        // Reduce inline threshold for size optimization
        self.module_pass_manager.add_function_inlining_pass_with_threshold(25);
    }
    
    fn add_production_passes(&mut self) {
        // All optimization passes for production
        self.add_basic_passes();
        self.add_standard_passes();
        self.add_aggressive_passes();
        
        // Additional production-specific optimizations
        self.module_pass_manager.add_lto_pass(); // Link Time Optimization
        self.function_pass_manager.add_profile_guided_optimization_pass();
    }
    
    pub fn optimize_module(&self, module: &mut LlvmModule) -> Result<OptimizationResult, OptimizationError> {
        let start_time = Instant::now();
        let initial_size = module.get_size();
        
        // Run function passes
        for function in module.get_functions() {
            self.function_pass_manager.run_on_function(function)?;
        }
        
        // Run module passes
        self.module_pass_manager.run_on_module(module)?;
        
        let optimization_time = start_time.elapsed();
        let final_size = module.get_size();
        
        Ok(OptimizationResult {
            optimization_time,
            initial_size,
            final_size,
            size_reduction: ((initial_size - final_size) as f64 / initial_size as f64) * 100.0,
            passes_applied: self.get_applied_passes(),
        })
    }
    
    fn get_applied_passes(&self) -> Vec<String> {
        let mut passes = self.function_pass_manager.get_pass_names();
        passes.extend(self.module_pass_manager.get_pass_names());
        passes
    }
}
```

### JIT Optimization

```rust
pub struct JitOptimizer {
    /// Hot function threshold
    hot_threshold: u64,
    /// Compilation tiers
    tiers: Vec<OptimizationTier>,
    /// Function call counters
    call_counters: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct OptimizationTier {
    /// Tier level (0 = interpreter, 1 = basic JIT, 2 = optimized JIT)
    pub level: u8,
    /// Call count threshold to reach this tier
    pub threshold: u64,
    /// Optimization level for this tier
    pub optimization_level: OptimizationLevel,
    /// Compilation time budget
    pub time_budget: Duration,
}

impl JitOptimizer {
    pub fn new() -> Self {
        Self {
            hot_threshold: 1000,
            tiers: vec![
                OptimizationTier {
                    level: 0,
                    threshold: 0,
                    optimization_level: OptimizationLevel::None,
                    time_budget: Duration::ZERO,
                },
                OptimizationTier {
                    level: 1,
                    threshold: 100,
                    optimization_level: OptimizationLevel::Less,
                    time_budget: Duration::from_millis(10),
                },
                OptimizationTier {
                    level: 2,
                    threshold: 1000,
                    optimization_level: OptimizationLevel::Default,
                    time_budget: Duration::from_millis(100),
                },
                OptimizationTier {
                    level: 3,
                    threshold: 10000,
                    optimization_level: OptimizationLevel::Aggressive,
                    time_budget: Duration::from_millis(1000),
                },
            ],
            call_counters: HashMap::new(),
        }
    }
    
    pub fn should_compile(&mut self, function_name: &str) -> Option<OptimizationTier> {
        let call_count = self.call_counters.entry(function_name.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        
        // Find the appropriate tier for this call count
        self.tiers.iter()
            .rev() // Start from highest tier
            .find(|tier| *call_count >= tier.threshold)
            .cloned()
    }
    
    pub fn optimize_hot_functions(&mut self) -> Vec<String> {
        let mut hot_functions = Vec::new();
        
        for (function_name, call_count) in &self.call_counters {
            if *call_count >= self.hot_threshold {
                hot_functions.push(function_name.clone());
            }
        }
        
        hot_functions.sort_by_key(|name| self.call_counters[name]);
        hot_functions.reverse(); // Most called first
        
        hot_functions
    }
}
```

## Testing and Validation

### Performance Tests

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_compilation_speed() {
        let source_code = include_str!("test_programs/medium_program.💀");
        let start = Instant::now();
        
        let result = compile_source(source_code);
        let compilation_time = start.elapsed();
        
        assert!(result.is_ok(), "Compilation should succeed");
        assert!(compilation_time < Duration::from_secs(5), 
               "Compilation should complete within 5 seconds for medium program");
    }
    
    #[test]
    fn test_arithmetic_performance() {
        let benchmarks = ArithmeticBenchmarks;
        let result = benchmarks.run_integer_arithmetic();
        
        assert!(result.meets_target, 
               "Integer arithmetic should meet performance target of 50M ops/sec");
        assert!(result.operations_per_second >= 50_000_000.0);
    }
    
    #[test]
    fn test_memory_allocation_performance() {
        let benchmarks = MemoryBenchmarks;
        let result = benchmarks.run_allocation_benchmark();
        
        assert!(result.meets_target,
               "Memory allocation should meet performance target of 1M allocs/sec");
        assert!(result.operations_per_second >= 1_000_000.0);
    }
    
    #[test]
    fn test_gc_pause_times() {
        let benchmarks = MemoryBenchmarks;
        let result = benchmarks.run_gc_benchmark();
        
        assert!(result.meets_pause_target,
               "GC pause times should be under 50ms");
        assert!(result.max_gc_pause < Duration::from_millis(50));
    }
    
    #[test]
    fn test_optimization_effectiveness() {
        let source = "
            sus x drip = 42;
            sus y drip = x + 1;
            sus z drip = y * 2;
            vibez.spill(z);
        ";
        
        // Compile with O0
        let o0_result = compile_with_optimization(source, OptimizationLevel::None);
        
        // Compile with O2
        let o2_result = compile_with_optimization(source, OptimizationLevel::Default);
        
        assert!(o0_result.is_ok() && o2_result.is_ok());
        
        // O2 should produce smaller or equal code size
        let o0_size = o0_result.unwrap().code_size;
        let o2_size = o2_result.unwrap().code_size;
        
        assert!(o2_size <= o0_size, "O2 optimization should not increase code size");
    }
}
```

### Continuous Performance Monitoring

```rust
pub struct ContinuousPerformanceMonitor {
    /// Performance history
    history: VecDeque<PerformanceSnapshot>,
    /// Alert thresholds
    thresholds: AlertThresholds,
    /// Notification system
    notifier: Box<dyn PerformanceNotifier>,
}

pub struct PerformanceSnapshot {
    pub timestamp: Instant,
    pub compilation_metrics: CompilationMetrics,
    pub runtime_metrics: RuntimeMetrics,
    pub memory_metrics: MemoryMetrics,
}

pub struct AlertThresholds {
    /// Maximum allowed compilation time regression
    pub compilation_time_regression: f64, // 20% = 0.2
    /// Maximum allowed runtime performance regression
    pub runtime_performance_regression: f64, // 10% = 0.1
    /// Maximum allowed memory usage increase
    pub memory_usage_increase: f64, // 15% = 0.15
}

impl ContinuousPerformanceMonitor {
    pub fn record_snapshot(&mut self, snapshot: PerformanceSnapshot) {
        self.history.push_back(snapshot);
        
        // Keep only recent history (last 100 snapshots)
        if self.history.len() > 100 {
            self.history.pop_front();
        }
        
        // Check for performance regressions
        self.check_for_alerts();
    }
    
    fn check_for_alerts(&self) {
        if let (Some(current), Some(baseline)) = (self.history.back(), self.get_baseline()) {
            // Check compilation time regression
            let compile_time_change = self.calculate_change(
                baseline.compilation_metrics.total_time.as_secs_f64(),
                current.compilation_metrics.total_time.as_secs_f64()
            );
            
            if compile_time_change > self.thresholds.compilation_time_regression {
                self.notifier.send_alert(Alert::CompilationTimeRegression {
                    baseline: baseline.compilation_metrics.total_time,
                    current: current.compilation_metrics.total_time,
                    regression_percent: compile_time_change * 100.0,
                });
            }
            
            // Check runtime performance regression
            let runtime_change = self.calculate_change(
                current.runtime_metrics.operations_per_second,
                baseline.runtime_metrics.operations_per_second
            );
            
            if runtime_change < -self.thresholds.runtime_performance_regression {
                self.notifier.send_alert(Alert::RuntimePerformanceRegression {
                    baseline: baseline.runtime_metrics.operations_per_second,
                    current: current.runtime_metrics.operations_per_second,
                    regression_percent: runtime_change.abs() * 100.0,
                });
            }
        }
    }
    
    fn get_baseline(&self) -> Option<&PerformanceSnapshot> {
        // Use average of last 10 snapshots as baseline
        if self.history.len() >= 10 {
            self.history.get(self.history.len() - 10)
        } else {
            self.history.front()
        }
    }
    
    fn calculate_change(&self, current: f64, baseline: f64) -> f64 {
        (current - baseline) / baseline
    }
}

pub trait PerformanceNotifier {
    fn send_alert(&self, alert: Alert);
}

#[derive(Debug)]
pub enum Alert {
    CompilationTimeRegression {
        baseline: Duration,
        current: Duration,
        regression_percent: f64,
    },
    RuntimePerformanceRegression {
        baseline: f64,
        current: f64,
        regression_percent: f64,
    },
    MemoryUsageIncrease {
        baseline: usize,
        current: usize,
        increase_percent: f64,
    },
}
```

This comprehensive performance specification ensures that CURSED maintains consistent, predictable performance characteristics across all optimization levels and execution modes, with robust monitoring and regression detection systems.
