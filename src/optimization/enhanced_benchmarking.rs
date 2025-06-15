//! Enhanced Benchmarking System for CURSED Compiler
//! 
//! Comprehensive benchmarking infrastructure with statistical analysis,
//! regression detection, and performance trend tracking.

use crate::error::{Error, Result};
use crate::optimization::{OptimizationLevel, OptimizationConfig};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};
use tokio::time::timeout;

/// Enhanced benchmark runner with statistical analysis
#[derive(Debug)]
pub struct EnhancedBenchmarkRunner {
    /// Benchmark configuration
    config: BenchmarkConfig,
    /// Statistical analyzer
    statistics: StatisticalAnalyzer,
    /// Results database
    results_db: BenchmarkDatabase,
    /// Regression detector
    regression_detector: RegressionDetector,
}

/// Comprehensive benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of benchmark iterations
    pub iterations: usize,
    /// Number of warmup iterations
    pub warmup_iterations: usize,
    /// Timeout for each compilation
    pub timeout: Duration,
    /// Enable parallel benchmarking
    pub parallel: bool,
    /// Maximum concurrent benchmarks
    pub max_concurrent: usize,
    /// Statistical confidence level
    pub confidence_level: f64,
    /// Minimum detectable change
    pub min_detectable_change: f64,
    /// Enable memory profiling
    pub memory_profiling: bool,
    /// Enable CPU profiling
    pub cpu_profiling: bool,
    /// Environment stability checks
    pub stability_checks: bool,
    /// Output verbosity level
    pub verbosity: VerbosityLevel,
}

#[derive(Debug, Clone)]
pub enum VerbosityLevel {
    Quiet,
    Normal,
    Verbose,
    Debug,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            warmup_iterations: 3,
            timeout: Duration::from_secs(300),
            parallel: false,
            max_concurrent: num_cpus::get(),
            confidence_level: 0.95,
            min_detectable_change: 0.05, // 5%
            memory_profiling: false,
            cpu_profiling: false,
            stability_checks: true,
            verbosity: VerbosityLevel::Normal,
        }
    }
}

/// Comprehensive benchmark result with statistical analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedBenchmarkResult {
    /// Test metadata
    pub metadata: BenchmarkMetadata,
    /// Results by optimization level
    pub level_results: HashMap<OptimizationLevel, LevelBenchmarkResult>,
    /// Statistical summary
    pub statistical_summary: StatisticalSummary,
    /// Performance comparison
    pub performance_comparison: PerformanceComparison,
    /// Regression analysis
    pub regression_analysis: Option<RegressionAnalysis>,
    /// Environment information
    pub environment: EnvironmentInfo,
    /// Recommendations
    pub recommendations: Vec<BenchmarkRecommendation>,
}

/// Metadata about the benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    /// Benchmark timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Source file path
    pub source_file: PathBuf,
    /// Source file hash for consistency
    pub source_hash: String,
    /// Benchmark configuration used
    pub config_snapshot: BenchmarkConfigSnapshot,
    /// Total benchmark duration
    pub total_duration: Duration,
    /// Benchmark runner version
    pub runner_version: String,
}

/// Benchmark results for a specific optimization level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelBenchmarkResult {
    /// Optimization level
    pub level: OptimizationLevel,
    /// Individual iteration results
    pub iterations: Vec<IterationResult>,
    /// Statistical measurements
    pub statistics: LevelStatistics,
    /// Resource usage
    pub resource_usage: ResourceUsage,
    /// Compilation phases breakdown
    pub phase_breakdown: HashMap<String, Duration>,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
}

/// Single iteration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationResult {
    /// Iteration number
    pub iteration: usize,
    /// Compilation time
    pub compilation_time: Duration,
    /// Peak memory usage
    pub peak_memory: usize,
    /// Binary size
    pub binary_size: usize,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Detailed timing breakdown
    pub timing_breakdown: HashMap<String, Duration>,
}

/// Statistical measurements for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelStatistics {
    /// Mean compilation time
    pub mean_time: Duration,
    /// Median compilation time
    pub median_time: Duration,
    /// Standard deviation
    pub std_deviation: Duration,
    /// 95th percentile
    pub p95_time: Duration,
    /// 99th percentile
    pub p99_time: Duration,
    /// Coefficient of variation
    pub coefficient_of_variation: f64,
    /// Confidence interval
    pub confidence_interval: (Duration, Duration),
    /// Statistical significance indicators
    pub significance_indicators: SignificanceIndicators,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory statistics
    pub memory: MemoryStatistics,
    /// CPU statistics
    pub cpu: CpuStatistics,
    /// I/O statistics
    pub io: IoStatistics,
    /// Cache statistics
    pub cache: CacheStatistics,
}

/// Quality metrics for generated code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Binary size statistics
    pub binary_size: SizeStatistics,
    /// Optimization effectiveness
    pub optimization_effectiveness: f64,
    /// Code quality score
    pub code_quality_score: f64,
    /// Performance characteristics
    pub performance_characteristics: PerformanceCharacteristics,
}

/// Statistical summary across all levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSummary {
    /// Best performing level
    pub best_level: OptimizationLevel,
    /// Worst performing level
    pub worst_level: OptimizationLevel,
    /// Performance spread
    pub performance_spread: f64,
    /// Statistical confidence in results
    pub overall_confidence: f64,
    /// Variance explained by optimization level
    pub optimization_variance: f64,
    /// Power analysis results
    pub power_analysis: PowerAnalysis,
}

/// Performance comparison between levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// Pairwise comparisons
    pub pairwise_comparisons: HashMap<String, ComparisonResult>,
    /// Ranking by performance
    pub performance_ranking: Vec<(OptimizationLevel, f64)>,
    /// Speed improvements
    pub speed_improvements: HashMap<OptimizationLevel, f64>,
    /// Trade-off analysis
    pub tradeoff_analysis: TradeoffAnalysis,
}

/// Regression analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    /// Regression detected
    pub regression_detected: bool,
    /// Severity of regression
    pub severity: RegressionSeverity,
    /// Affected optimization levels
    pub affected_levels: Vec<OptimizationLevel>,
    /// Performance degradation percentage
    pub degradation_percentage: f64,
    /// Confidence in regression detection
    pub detection_confidence: f64,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
}

/// Environment information for reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Operating system
    pub os: String,
    /// CPU information
    pub cpu_info: CpuInfo,
    /// Memory information
    pub memory_info: MemoryInfo,
    /// Compiler version
    pub compiler_version: String,
    /// LLVM version
    pub llvm_version: String,
    /// System load during benchmark
    pub system_load: SystemLoad,
    /// Environment variables
    pub environment_variables: HashMap<String, String>,
}

/// Benchmark recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: u8,
    /// Description
    pub description: String,
    /// Specific action to take
    pub action: String,
    /// Expected impact
    pub expected_impact: f64,
}

// Supporting types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfigSnapshot {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub timeout_seconds: u64,
    pub parallel: bool,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificanceIndicators {
    pub p_value: f64,
    pub effect_size: f64,
    pub statistical_power: f64,
    pub sample_size_adequacy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    pub peak_usage: usize,
    pub average_usage: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
    pub fragmentation_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStatistics {
    pub user_time: Duration,
    pub system_time: Duration,
    pub utilization_percentage: f64,
    pub context_switches: usize,
    pub cache_misses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStatistics {
    pub bytes_read: usize,
    pub bytes_written: usize,
    pub read_operations: usize,
    pub write_operations: usize,
    pub io_wait_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub l1_hits: usize,
    pub l1_misses: usize,
    pub l2_hits: usize,
    pub l2_misses: usize,
    pub l3_hits: usize,
    pub l3_misses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeStatistics {
    pub mean_size: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub size_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    pub scalability_factor: f64,
    pub optimization_efficiency: f64,
    pub resource_efficiency: f64,
    pub predictability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerAnalysis {
    pub statistical_power: f64,
    pub effect_size: f64,
    pub required_sample_size: usize,
    pub observed_sample_size: usize,
    pub power_adequate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub faster_level: OptimizationLevel,
    pub speed_difference: f64,
    pub statistical_significance: bool,
    pub confidence_interval: (f64, f64),
    pub p_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeoffAnalysis {
    pub compile_time_vs_runtime: HashMap<OptimizationLevel, (f64, f64)>,
    pub memory_vs_speed: HashMap<OptimizationLevel, (f64, f64)>,
    pub size_vs_speed: HashMap<OptimizationLevel, (f64, f64)>,
    pub pareto_optimal_levels: Vec<OptimizationLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionSeverity {
    Minor,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
    pub frequency_mhz: u32,
    pub cache_sizes: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_ram: usize,
    pub available_ram: usize,
    pub swap_total: usize,
    pub swap_available: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoad {
    pub load_average_1min: f64,
    pub load_average_5min: f64,
    pub load_average_15min: f64,
    pub cpu_usage_percentage: f64,
    pub memory_usage_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    OptimizationLevel,
    CompilerFlag,
    Environment,
    Hardware,
    CodeStructure,
}

/// Statistical analyzer for benchmark results
#[derive(Debug)]
pub struct StatisticalAnalyzer {
    /// Confidence level for statistical tests
    confidence_level: f64,
}

/// Benchmark results database
#[derive(Debug)]
pub struct BenchmarkDatabase {
    /// Historical results
    results: Vec<EnhancedBenchmarkResult>,
    /// Database file path
    db_path: Option<PathBuf>,
}

/// Regression detector
#[derive(Debug)]
pub struct RegressionDetector {
    /// Sensitivity threshold
    sensitivity: f64,
    /// Minimum number of historical results needed
    min_history: usize,
}

impl EnhancedBenchmarkRunner {
    /// Create a new enhanced benchmark runner
    pub fn new() -> Self {
        Self::with_config(BenchmarkConfig::default())
    }

    /// Create runner with custom configuration
    pub fn with_config(config: BenchmarkConfig) -> Self {
        Self {
            statistics: StatisticalAnalyzer::new(config.confidence_level),
            regression_detector: RegressionDetector::new(config.min_detectable_change),
            results_db: BenchmarkDatabase::new(),
            config,
        }
    }

    /// Run comprehensive benchmark on a source file
    #[instrument(skip(self, source))]
    pub async fn benchmark_comprehensive(
        &mut self,
        source: &str,
        file_path: &Path,
        levels: &[OptimizationLevel],
    ) -> Result<EnhancedBenchmarkResult> {
        info!("Starting comprehensive benchmark for {}", file_path.display());
        
        let benchmark_start = Instant::now();

        // Validate environment stability
        if self.config.stability_checks {
            self.check_environment_stability().await?;
        }

        // Create metadata
        let metadata = self.create_metadata(source, file_path, benchmark_start)?;

        // Collect environment information
        let environment = self.collect_environment_info().await?;

        // Run benchmarks for each optimization level
        let mut level_results = HashMap::new();

        for &level in levels {
            match self.config.verbosity {
                VerbosityLevel::Normal | VerbosityLevel::Verbose => {
                    info!("Benchmarking optimization level: {:?}", level);
                }
                _ => {}
            }

            let level_result = self.benchmark_level(source, file_path, level).await?;
            level_results.insert(level, level_result);
        }

        // Perform statistical analysis
        let statistical_summary = self.statistics.analyze_across_levels(&level_results)?;

        // Generate performance comparison
        let performance_comparison = self.generate_performance_comparison(&level_results)?;

        // Check for regressions
        let regression_analysis = self.check_for_regressions(&level_results).await?;

        // Generate recommendations
        let recommendations = self.generate_recommendations(&level_results, &statistical_summary)?;

        let result = EnhancedBenchmarkResult {
            metadata,
            level_results,
            statistical_summary,
            performance_comparison,
            regression_analysis,
            environment,
            recommendations,
        };

        // Store result in database
        self.results_db.store_result(&result)?;

        let total_duration = benchmark_start.elapsed();
        info!("Comprehensive benchmark completed in {:?}", total_duration);

        Ok(result)
    }

    /// Benchmark a specific optimization level
    async fn benchmark_level(
        &self,
        source: &str,
        file_path: &Path,
        level: OptimizationLevel,
    ) -> Result<LevelBenchmarkResult> {
        let mut iterations = Vec::new();
        let mut phase_breakdown = HashMap::new();

        // Warmup iterations
        for i in 0..self.config.warmup_iterations {
            match self.config.verbosity {
                VerbosityLevel::Debug => {
                    debug!("Warmup iteration {} for {:?}", i + 1, level);
                }
                _ => {}
            }
            let _ = self.run_single_iteration(source, file_path, level, true).await;
        }

        // Actual benchmark iterations
        for i in 0..self.config.iterations {
            match self.config.verbosity {
                VerbosityLevel::Verbose | VerbosityLevel::Debug => {
                    debug!("Benchmark iteration {} for {:?}", i + 1, level);
                }
                _ => {}
            }

            let iteration_result = self.run_single_iteration(source, file_path, level, false).await?;
            
            // Accumulate phase breakdown
            for (phase, duration) in &iteration_result.timing_breakdown {
                let total = phase_breakdown.entry(phase.clone()).or_insert(Duration::ZERO);
                *total += *duration;
            }

            iterations.push(iteration_result);
        }

        // Calculate average phase breakdown
        for duration in phase_breakdown.values_mut() {
            *duration /= self.config.iterations as u32;
        }

        // Calculate statistics
        let statistics = self.statistics.calculate_level_statistics(&iterations)?;

        // Calculate resource usage
        let resource_usage = self.calculate_resource_usage(&iterations)?;

        // Calculate quality metrics
        let quality_metrics = self.calculate_quality_metrics(&iterations)?;

        Ok(LevelBenchmarkResult {
            level,
            iterations,
            statistics,
            resource_usage,
            phase_breakdown,
            quality_metrics,
        })
    }

    /// Run a single benchmark iteration
    async fn run_single_iteration(
        &self,
        source: &str,
        file_path: &Path,
        level: OptimizationLevel,
        is_warmup: bool,
    ) -> Result<IterationResult> {
        let iteration_start = Instant::now();
        
        // Create optimization configuration
        let mut opt_config = OptimizationConfig::default();
        opt_config.optimization_level = level;

        // Simulate compilation with timing
        let compilation_result = timeout(
            self.config.timeout,
            self.simulate_compilation(source, file_path, &opt_config)
        ).await;

        let compilation_time = iteration_start.elapsed();

        match compilation_result {
            Ok(Ok(sim_result)) => {
                Ok(IterationResult {
                    iteration: 0, // Will be set by caller
                    compilation_time,
                    peak_memory: sim_result.peak_memory,
                    binary_size: sim_result.binary_size,
                    success: true,
                    error_message: None,
                    timing_breakdown: sim_result.timing_breakdown,
                })
            }
            Ok(Err(e)) => {
                Ok(IterationResult {
                    iteration: 0,
                    compilation_time,
                    peak_memory: 0,
                    binary_size: 0,
                    success: false,
                    error_message: Some(e.to_string()),
                    timing_breakdown: HashMap::new(),
                })
            }
            Err(_) => {
                // Timeout
                Ok(IterationResult {
                    iteration: 0,
                    compilation_time: self.config.timeout,
                    peak_memory: 0,
                    binary_size: 0,
                    success: false,
                    error_message: Some("Compilation timeout".to_string()),
                    timing_breakdown: HashMap::new(),
                })
            }
        }
    }

    /// Simulate compilation process
    async fn simulate_compilation(
        &self,
        source: &str,
        _file_path: &Path,
        opt_config: &OptimizationConfig,
    ) -> Result<CompilationSimulationResult> {
        // Simulate compilation phases with realistic timing
        let mut timing_breakdown = HashMap::new();
        
        // Base timings scaled by source complexity
        let complexity_factor = (source.len() as f64 / 1000.0).max(0.1);
        let opt_factor = match opt_config.optimization_level {
            OptimizationLevel::O0 => 0.5,
            OptimizationLevel::O1 => 0.8,
            OptimizationLevel::O2 => 1.0,
            OptimizationLevel::O3 => 1.5,
            OptimizationLevel::Os => 0.9,
            OptimizationLevel::Oz => 1.1,
        };

        // Simulate each phase
        let phases = vec![
            ("lexing", Duration::from_millis(5)),
            ("parsing", Duration::from_millis(20)),
            ("semantic_analysis", Duration::from_millis(40)),
            ("type_checking", Duration::from_millis(30)),
            ("ir_generation", Duration::from_millis(25)),
            ("llvm_optimization", Duration::from_millis(100)),
            ("code_generation", Duration::from_millis(15)),
            ("linking", Duration::from_millis(10)),
        ];

        let mut total_time = Duration::ZERO;
        for (phase, base_time) in phases {
            let phase_time = Duration::from_nanos(
                (base_time.as_nanos() as f64 * complexity_factor * opt_factor) as u64
            );
            timing_breakdown.insert(phase.to_string(), phase_time);
            total_time += phase_time;

            // Simulate actual work
            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        // Simulate memory usage
        let base_memory = source.len() * 3;
        let peak_memory = (base_memory as f64 * complexity_factor * opt_factor) as usize;

        // Simulate binary size
        let base_size = source.len() / 2;
        let size_factor = match opt_config.optimization_level {
            OptimizationLevel::O0 => 1.5,
            OptimizationLevel::O1 => 1.2,
            OptimizationLevel::O2 => 1.0,
            OptimizationLevel::O3 => 0.9,
            OptimizationLevel::Os => 0.7,
            OptimizationLevel::Oz => 0.6,
        };
        let binary_size = (base_size as f64 * size_factor) as usize;

        Ok(CompilationSimulationResult {
            timing_breakdown,
            peak_memory,
            binary_size,
        })
    }

    /// Check environment stability
    async fn check_environment_stability(&self) -> Result<()> {
        // Simulate stability checks
        info!("Checking environment stability...");
        
        // Check system load
        let load = self.get_system_load().await?;
        if load.cpu_usage_percentage > 80.0 {
            warn!("High CPU usage detected: {:.1}%", load.cpu_usage_percentage);
        }
        
        if load.memory_usage_percentage > 90.0 {
            return Err(Error::general("System memory usage too high for stable benchmarking"));
        }

        Ok(())
    }

    /// Create benchmark metadata
    fn create_metadata(
        &self,
        source: &str,
        file_path: &Path,
        start_time: Instant,
    ) -> Result<BenchmarkMetadata> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        let source_hash = format!("{:x}", hasher.finish());

        Ok(BenchmarkMetadata {
            timestamp: chrono::Utc::now(),
            source_file: file_path.to_path_buf(),
            source_hash,
            config_snapshot: BenchmarkConfigSnapshot {
                iterations: self.config.iterations,
                warmup_iterations: self.config.warmup_iterations,
                timeout_seconds: self.config.timeout.as_secs(),
                parallel: self.config.parallel,
                confidence_level: self.config.confidence_level,
            },
            total_duration: Duration::ZERO, // Will be updated
            runner_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    /// Collect environment information
    async fn collect_environment_info(&self) -> Result<EnvironmentInfo> {
        let cpu_info = CpuInfo {
            model: "Simulated CPU".to_string(),
            cores: num_cpus::get(),
            threads: num_cpus::get(),
            frequency_mhz: 3000,
            cache_sizes: vec![32768, 262144, 8388608], // L1, L2, L3
        };

        let memory_info = MemoryInfo {
            total_ram: 16 * 1024 * 1024 * 1024, // 16GB
            available_ram: 8 * 1024 * 1024 * 1024, // 8GB
            swap_total: 4 * 1024 * 1024 * 1024, // 4GB
            swap_available: 4 * 1024 * 1024 * 1024, // 4GB
        };

        let system_load = self.get_system_load().await?;

        Ok(EnvironmentInfo {
            os: std::env::consts::OS.to_string(),
            cpu_info,
            memory_info,
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            llvm_version: "15.0.0".to_string(), // Simulated
            system_load,
            environment_variables: std::env::vars().collect(),
        })
    }

    /// Get current system load
    async fn get_system_load(&self) -> Result<SystemLoad> {
        // Simulate system load measurements
        Ok(SystemLoad {
            load_average_1min: 1.5,
            load_average_5min: 1.2,
            load_average_15min: 1.0,
            cpu_usage_percentage: 25.0,
            memory_usage_percentage: 60.0,
        })
    }

    /// Calculate resource usage statistics
    fn calculate_resource_usage(&self, iterations: &[IterationResult]) -> Result<ResourceUsage> {
        let successful_iterations: Vec<_> = iterations.iter().filter(|i| i.success).collect();
        
        if successful_iterations.is_empty() {
            return Ok(ResourceUsage {
                memory: MemoryStatistics {
                    peak_usage: 0,
                    average_usage: 0,
                    allocation_count: 0,
                    deallocation_count: 0,
                    fragmentation_ratio: 0.0,
                },
                cpu: CpuStatistics {
                    user_time: Duration::ZERO,
                    system_time: Duration::ZERO,
                    utilization_percentage: 0.0,
                    context_switches: 0,
                    cache_misses: 0,
                },
                io: IoStatistics {
                    bytes_read: 0,
                    bytes_written: 0,
                    read_operations: 0,
                    write_operations: 0,
                    io_wait_time: Duration::ZERO,
                },
                cache: CacheStatistics {
                    l1_hits: 0,
                    l1_misses: 0,
                    l2_hits: 0,
                    l2_misses: 0,
                    l3_hits: 0,
                    l3_misses: 0,
                },
            });
        }

        let peak_memory = successful_iterations.iter().map(|i| i.peak_memory).max().unwrap_or(0);
        let avg_memory = successful_iterations.iter().map(|i| i.peak_memory).sum::<usize>() / successful_iterations.len();

        Ok(ResourceUsage {
            memory: MemoryStatistics {
                peak_usage: peak_memory,
                average_usage: avg_memory,
                allocation_count: 100, // Simulated
                deallocation_count: 95, // Simulated
                fragmentation_ratio: 0.1, // Simulated
            },
            cpu: CpuStatistics {
                user_time: Duration::from_millis(500), // Simulated
                system_time: Duration::from_millis(100), // Simulated
                utilization_percentage: 75.0, // Simulated
                context_switches: 50, // Simulated
                cache_misses: 1000, // Simulated
            },
            io: IoStatistics {
                bytes_read: 10000, // Simulated
                bytes_written: 5000, // Simulated
                read_operations: 10, // Simulated
                write_operations: 5, // Simulated
                io_wait_time: Duration::from_millis(10), // Simulated
            },
            cache: CacheStatistics {
                l1_hits: 10000, // Simulated
                l1_misses: 1000, // Simulated
                l2_hits: 5000, // Simulated
                l2_misses: 500, // Simulated
                l3_hits: 2000, // Simulated
                l3_misses: 200, // Simulated
            },
        })
    }

    /// Calculate quality metrics
    fn calculate_quality_metrics(&self, iterations: &[IterationResult]) -> Result<QualityMetrics> {
        let successful_iterations: Vec<_> = iterations.iter().filter(|i| i.success).collect();
        
        if successful_iterations.is_empty() {
            return Ok(QualityMetrics {
                binary_size: SizeStatistics {
                    mean_size: 0,
                    min_size: 0,
                    max_size: 0,
                    size_variance: 0.0,
                },
                optimization_effectiveness: 0.0,
                code_quality_score: 0.0,
                performance_characteristics: PerformanceCharacteristics {
                    scalability_factor: 0.0,
                    optimization_efficiency: 0.0,
                    resource_efficiency: 0.0,
                    predictability_score: 0.0,
                },
            });
        }

        let sizes: Vec<usize> = successful_iterations.iter().map(|i| i.binary_size).collect();
        let mean_size = sizes.iter().sum::<usize>() / sizes.len();
        let min_size = sizes.iter().min().copied().unwrap_or(0);
        let max_size = sizes.iter().max().copied().unwrap_or(0);
        
        let size_variance = if sizes.len() > 1 {
            let mean_f64 = mean_size as f64;
            sizes.iter()
                .map(|&s| (s as f64 - mean_f64).powi(2))
                .sum::<f64>() / (sizes.len() - 1) as f64
        } else {
            0.0
        };

        Ok(QualityMetrics {
            binary_size: SizeStatistics {
                mean_size,
                min_size,
                max_size,
                size_variance,
            },
            optimization_effectiveness: 0.8, // Simulated
            code_quality_score: 0.85, // Simulated
            performance_characteristics: PerformanceCharacteristics {
                scalability_factor: 0.9, // Simulated
                optimization_efficiency: 0.8, // Simulated
                resource_efficiency: 0.7, // Simulated
                predictability_score: 0.85, // Simulated
            },
        })
    }

    /// Generate performance comparison
    fn generate_performance_comparison(
        &self,
        level_results: &HashMap<OptimizationLevel, LevelBenchmarkResult>,
    ) -> Result<PerformanceComparison> {
        let mut pairwise_comparisons = HashMap::new();
        let mut performance_ranking = Vec::new();
        let mut speed_improvements = HashMap::new();

        // Calculate performance ranking
        let mut levels_by_speed: Vec<_> = level_results
            .iter()
            .map(|(&level, result)| (level, result.statistics.mean_time.as_secs_f64()))
            .collect();
        
        levels_by_speed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        for (level, time) in &levels_by_speed {
            performance_ranking.push((*level, *time));
        }

        // Calculate speed improvements relative to O0
        if let Some(o0_result) = level_results.get(&OptimizationLevel::O0) {
            let baseline_time = o0_result.statistics.mean_time.as_secs_f64();
            
            for (&level, result) in level_results {
                let level_time = result.statistics.mean_time.as_secs_f64();
                let improvement = (baseline_time - level_time) / baseline_time;
                speed_improvements.insert(level, improvement);
            }
        }

        // Generate pairwise comparisons
        let levels: Vec<_> = level_results.keys().collect();
        for i in 0..levels.len() {
            for j in (i + 1)..levels.len() {
                let level1 = levels[i];
                let level2 = levels[j];
                
                let result1 = &level_results[level1];
                let result2 = &level_results[level2];
                
                let time1 = result1.statistics.mean_time.as_secs_f64();
                let time2 = result2.statistics.mean_time.as_secs_f64();
                
                let (faster_level, speed_diff) = if time1 < time2 {
                    (*level1, (time2 - time1) / time2)
                } else {
                    (*level2, (time1 - time2) / time1)
                };

                let comparison = ComparisonResult {
                    faster_level,
                    speed_difference: speed_diff,
                    statistical_significance: speed_diff > 0.05, // 5% threshold
                    confidence_interval: (-0.1, 0.1), // Simulated
                    p_value: 0.01, // Simulated
                };

                let key = format!("{:?}_vs_{:?}", level1, level2);
                pairwise_comparisons.insert(key, comparison);
            }
        }

        let tradeoff_analysis = self.analyze_tradeoffs(level_results)?;

        Ok(PerformanceComparison {
            pairwise_comparisons,
            performance_ranking,
            speed_improvements,
            tradeoff_analysis,
        })
    }

    /// Analyze performance tradeoffs
    fn analyze_tradeoffs(
        &self,
        level_results: &HashMap<OptimizationLevel, LevelBenchmarkResult>,
    ) -> Result<TradeoffAnalysis> {
        let mut compile_time_vs_runtime = HashMap::new();
        let mut memory_vs_speed = HashMap::new();
        let mut size_vs_speed = HashMap::new();
        let mut pareto_optimal_levels = Vec::new();

        for (&level, result) in level_results {
            let compile_time = result.statistics.mean_time.as_secs_f64();
            let binary_size = result.quality_metrics.binary_size.mean_size as f64;
            let memory_usage = result.resource_usage.memory.peak_usage as f64;
            
            // Simulate runtime performance (inversely related to compile time for optimization)
            let runtime_perf = match level {
                OptimizationLevel::O0 => 1.0,
                OptimizationLevel::O1 => 0.9,
                OptimizationLevel::O2 => 0.8,
                OptimizationLevel::O3 => 0.75,
                OptimizationLevel::Os => 0.85,
                OptimizationLevel::Oz => 0.82,
            };

            compile_time_vs_runtime.insert(level, (compile_time, runtime_perf));
            memory_vs_speed.insert(level, (memory_usage, 1.0 / runtime_perf));
            size_vs_speed.insert(level, (binary_size, 1.0 / runtime_perf));

            // Simple Pareto optimality check (could be more sophisticated)
            if matches!(level, OptimizationLevel::O2 | OptimizationLevel::O3 | OptimizationLevel::Os) {
                pareto_optimal_levels.push(level);
            }
        }

        Ok(TradeoffAnalysis {
            compile_time_vs_runtime,
            memory_vs_speed,
            size_vs_speed,
            pareto_optimal_levels,
        })
    }

    /// Check for performance regressions
    async fn check_for_regressions(
        &self,
        _level_results: &HashMap<OptimizationLevel, LevelBenchmarkResult>,
    ) -> Result<Option<RegressionAnalysis>> {
        // Placeholder for regression detection
        Ok(None)
    }

    /// Generate benchmark recommendations
    fn generate_recommendations(
        &self,
        level_results: &HashMap<OptimizationLevel, LevelBenchmarkResult>,
        statistical_summary: &StatisticalSummary,
    ) -> Result<Vec<BenchmarkRecommendation>> {
        let mut recommendations = Vec::new();

        // Recommend best optimization level
        recommendations.push(BenchmarkRecommendation {
            recommendation_type: RecommendationType::OptimizationLevel,
            priority: 9,
            description: format!("Use {:?} for best performance", statistical_summary.best_level),
            action: format!("Set optimization level to {:?}", statistical_summary.best_level),
            expected_impact: 0.2, // 20% improvement
        });

        // Check for high variance
        if statistical_summary.overall_confidence < 0.8 {
            recommendations.push(BenchmarkRecommendation {
                recommendation_type: RecommendationType::Environment,
                priority: 7,
                description: "High variance detected in benchmark results".to_string(),
                action: "Increase iteration count or check system stability".to_string(),
                expected_impact: 0.1,
            });
        }

        // Memory usage recommendations
        for (&level, result) in level_results {
            if result.resource_usage.memory.peak_usage > 1024 * 1024 * 100 { // 100MB
                recommendations.push(BenchmarkRecommendation {
                    recommendation_type: RecommendationType::CompilerFlag,
                    priority: 5,
                    description: format!("High memory usage detected with {:?}", level),
                    action: "Consider using memory optimization flags".to_string(),
                    expected_impact: 0.15,
                });
            }
        }

        // Sort by priority
        recommendations.sort_by_key(|r| std::cmp::Reverse(r.priority));

        Ok(recommendations)
    }
}

/// Compilation simulation result
#[derive(Debug)]
struct CompilationSimulationResult {
    timing_breakdown: HashMap<String, Duration>,
    peak_memory: usize,
    binary_size: usize,
}

impl StatisticalAnalyzer {
    pub fn new(confidence_level: f64) -> Self {
        Self { confidence_level }
    }

    pub fn calculate_level_statistics(&self, iterations: &[IterationResult]) -> Result<LevelStatistics> {
        let successful_iterations: Vec<_> = iterations.iter().filter(|i| i.success).collect();
        
        if successful_iterations.is_empty() {
            return Err(Error::general("No successful iterations for statistical analysis"));
        }

        let mut times: Vec<Duration> = successful_iterations.iter().map(|i| i.compilation_time).collect();
        times.sort();

        let mean_time = Duration::from_nanos(
            times.iter().map(|t| t.as_nanos()).sum::<u128>() / times.len() as u128
        );

        let median_time = if times.len() % 2 == 0 {
            let mid1 = times[times.len() / 2 - 1];
            let mid2 = times[times.len() / 2];
            Duration::from_nanos((mid1.as_nanos() + mid2.as_nanos()) / 2)
        } else {
            times[times.len() / 2]
        };

        // Calculate standard deviation
        let mean_nanos = mean_time.as_nanos() as f64;
        let variance: f64 = times.iter()
            .map(|t| (t.as_nanos() as f64 - mean_nanos).powi(2))
            .sum::<f64>() / (times.len() - 1) as f64;
        let std_deviation = Duration::from_nanos(variance.sqrt() as u64);

        // Calculate percentiles
        let p95_index = (0.95 * times.len() as f64) as usize;
        let p99_index = (0.99 * times.len() as f64) as usize;
        let p95_time = times.get(p95_index.min(times.len() - 1)).copied().unwrap_or(median_time);
        let p99_time = times.get(p99_index.min(times.len() - 1)).copied().unwrap_or(median_time);

        // Coefficient of variation
        let cv = if mean_time.as_nanos() > 0 {
            std_deviation.as_nanos() as f64 / mean_time.as_nanos() as f64
        } else {
            0.0
        };

        // Confidence interval (simplified)
        let margin_of_error = std_deviation.as_nanos() as f64 / (times.len() as f64).sqrt();
        let ci_lower = Duration::from_nanos((mean_time.as_nanos() as f64 - margin_of_error) as u64);
        let ci_upper = Duration::from_nanos((mean_time.as_nanos() as f64 + margin_of_error) as u64);

        Ok(LevelStatistics {
            mean_time,
            median_time,
            std_deviation,
            p95_time,
            p99_time,
            coefficient_of_variation: cv,
            confidence_interval: (ci_lower, ci_upper),
            significance_indicators: SignificanceIndicators {
                p_value: 0.05, // Placeholder
                effect_size: 0.5, // Placeholder
                statistical_power: 0.8, // Placeholder
                sample_size_adequacy: times.len() >= 5,
            },
        })
    }

    pub fn analyze_across_levels(
        &self,
        level_results: &HashMap<OptimizationLevel, LevelBenchmarkResult>,
    ) -> Result<StatisticalSummary> {
        if level_results.is_empty() {
            return Err(Error::general("No level results for statistical analysis"));
        }

        // Find best and worst performing levels
        let mut levels_by_performance: Vec<_> = level_results
            .iter()
            .map(|(&level, result)| (level, result.statistics.mean_time))
            .collect();
        
        levels_by_performance.sort_by_key(|(_, time)| *time);
        
        let best_level = levels_by_performance[0].0;
        let worst_level = levels_by_performance[levels_by_performance.len() - 1].0;

        // Calculate performance spread
        let best_time = levels_by_performance[0].1.as_secs_f64();
        let worst_time = levels_by_performance[levels_by_performance.len() - 1].1.as_secs_f64();
        let performance_spread = (worst_time - best_time) / best_time;

        // Overall confidence (simplified)
        let avg_cv: f64 = level_results.values()
            .map(|r| r.statistics.coefficient_of_variation)
            .sum::<f64>() / level_results.len() as f64;
        let overall_confidence = (1.0 - avg_cv).max(0.0).min(1.0);

        Ok(StatisticalSummary {
            best_level,
            worst_level,
            performance_spread,
            overall_confidence,
            optimization_variance: 0.3, // Placeholder
            power_analysis: PowerAnalysis {
                statistical_power: 0.8,
                effect_size: 0.5,
                required_sample_size: 10,
                observed_sample_size: level_results.values().next().map(|r| r.iterations.len()).unwrap_or(0),
                power_adequate: true,
            },
        })
    }
}

impl BenchmarkDatabase {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            db_path: None,
        }
    }

    pub fn store_result(&mut self, result: &EnhancedBenchmarkResult) -> Result<()> {
        self.results.push(result.clone());
        
        // Keep only recent results (last 50)
        if self.results.len() > 50 {
            self.results.remove(0);
        }

        Ok(())
    }
}

impl RegressionDetector {
    pub fn new(sensitivity: f64) -> Self {
        Self {
            sensitivity,
            min_history: 3,
        }
    }
}

impl Default for EnhancedBenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enhanced_benchmark_runner() {
        let mut runner = EnhancedBenchmarkRunner::new();
        let source = r#"
            slay main() {
                facts x = 42;
                println("Hello, world!");
            }
        "#;

        let levels = vec![OptimizationLevel::O0, OptimizationLevel::O2];
        let result = runner.benchmark_comprehensive(
            source,
            Path::new("test.csd"),
            &levels,
        ).await;

        assert!(result.is_ok());
        let benchmark_result = result.unwrap();
        
        assert_eq!(benchmark_result.level_results.len(), 2);
        assert!(benchmark_result.level_results.contains_key(&OptimizationLevel::O0));
        assert!(benchmark_result.level_results.contains_key(&OptimizationLevel::O2));
    }

    #[test]
    fn test_statistical_analyzer() {
        let analyzer = StatisticalAnalyzer::new(0.95);
        
        let iterations = vec![
            IterationResult {
                iteration: 1,
                compilation_time: Duration::from_millis(100),
                peak_memory: 1000,
                binary_size: 500,
                success: true,
                error_message: None,
                timing_breakdown: HashMap::new(),
            },
            IterationResult {
                iteration: 2,
                compilation_time: Duration::from_millis(110),
                peak_memory: 1100,
                binary_size: 510,
                success: true,
                error_message: None,
                timing_breakdown: HashMap::new(),
            },
        ];

        let result = analyzer.calculate_level_statistics(&iterations);
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        assert!(stats.mean_time.as_millis() > 0);
        assert!(stats.coefficient_of_variation >= 0.0);
    }

    #[test]
    fn test_benchmark_config() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.iterations, 10);
        assert_eq!(config.warmup_iterations, 3);
        assert_eq!(config.confidence_level, 0.95);
    }
}
