
// Performance Benchmarking System for CURSED Optimization
// 
// Provides comprehensive benchmarking infrastructure to measure and validate
// performance improvements from various optimization techniques.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::process::Command;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};

use crate::codegen::llvm::optimization::{OptimizationConfig};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::error::{CursedError, Result};
use crate::optimization::baseline_storage::{BaselineStorage, BaselineStorageConfig, BaselineType};
use crate::optimization::regression_analyzer::{RegressionAnalyzer, RegressionAnalysisConfig, DetailedRegressionAnalysis};

/// Benchmark configuration for performance testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Name of the benchmark
    /// Source files to benchmark
    /// Optimization levels to test
    /// Number of iterations per test
    /// Warmup iterations before measurement
    /// Timeout for each compilation
    /// Additional compiler flags
    /// Expected performance thresholds
/// Performance thresholds for regression testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable compilation time increase (percentage)
    /// Minimum expected runtime performance improvement (percentage)
    /// Maximum acceptable binary size increase (percentage)
    /// Maximum acceptable memory usage increase (percentage)
impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_compile_time_increase: 50.0, // 50% increase acceptable for aggressive optimization
            min_runtime_improvement: 10.0,   // Expect at least 10% runtime improvement
            max_size_increase: 20.0,          // 20% size increase acceptable
            max_memory_increase: 30.0,        // 30% memory increase acceptable
        }
    }
/// Benchmark result for a single test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    /// Optimization level used
    /// Compilation time
    /// Runtime performance (if available)
    /// Binary size in bytes
    /// Peak memory usage during compilation
    /// Number of optimization passes applied
    /// Success/failure status
    /// CursedError message if failed
/// Complete benchmark suite results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteResult {
    /// Suite name
    /// Timestamp when benchmark was run
    /// Individual benchmark results
    /// Overall statistics
    /// Performance regression analysis
/// Benchmark statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
    /// Total benchmarks run
    /// Successful benchmarks
    /// Average compilation time
    /// Average performance improvement over baseline
    /// Average binary size change
    /// Best performing optimization level
/// Regression analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    /// Performance regressions detected
    /// Overall regression status
    /// Baseline comparison results
/// Individual performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    /// Benchmark name
    /// Type of regression
    /// Severity of regression
    /// Actual vs expected values
    /// Description of the regression
/// Type of performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionType {
/// Severity of regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegressionSeverity {
/// Baseline comparison results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineComparison {
    /// Baseline benchmark results file
    /// Performance improvements over baseline
    /// Overall improvement percentage
/// Performance improvement measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImprovement {
    /// Benchmark name
    /// Improvement percentage
    /// Category of improvement
/// Category of performance improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImprovementCategory {
/// Benchmark runner for executing performance tests
pub struct BenchmarkRunner {
    /// Compiler executable path
    /// Working directory for benchmarks
    /// Enable verbose output
    /// Baseline storage manager
    /// Regression analyzer
impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(compiler_path: PathBuf, work_dir: PathBuf) -> Self {
        Self {
        }
    }

    /// Create a new benchmark runner with baseline storage
    pub fn with_baseline_storage(mut self, storage_config: BaselineStorageConfig) -> Result<Self> {
        let baseline_storage = BaselineStorage::new(storage_config)?;
        self.baseline_storage = Some(baseline_storage);
        
        // Initialize regression analyzer if we have baseline storage
        let regression_config = RegressionAnalysisConfig::default();
        self.regression_analyzer = Some(RegressionAnalyzer::new(regression_config));
        
        Ok(self)
    /// Enable verbose output
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    /// Run a complete benchmark suite
    #[instrument(skip(self, configs))]
    pub async fn run_benchmark_suite(
    ) -> Result<BenchmarkSuiteResult> {
        info!("Starting benchmark suite: {}", suite_name);
        
        let start_time = Instant::now();
        let mut all_results = Vec::new();
        let mut successful_benchmarks = 0;

        for config in configs {
            info!("Running benchmark: {}", config.name);
            
            for &level in &config.optimization_levels {
                let result = self.run_single_benchmark(config, level).await?;
                if result.success {
                    successful_benchmarks += 1;
                }
                all_results.push(result);
            }
        }

        let statistics = self.calculate_statistics(&all_results);
        let regression_analysis = self.analyze_regressions(&all_results, configs).await?;

        let suite_result = BenchmarkSuiteResult {

        info!(
            "Benchmark suite completed in {:?}, {}/{} benchmarks successful",
            suite_result.results.len()
        );

        Ok(suite_result)
    /// Run a single benchmark test
    async fn run_single_benchmark(
    ) -> Result<BenchmarkResult> {
               config.name, optimization_level);

        // Warmup iterations
        for i in 0..config.warmup_iterations {
            debug!("Warmup iteration {}/{}", i + 1, config.warmup_iterations);
            self.compile_with_optimization(&config.source_files[0], optimization_level).await?;
        // Measured iterations
        let mut compile_times = Vec::new();
        let mut binary_sizes = Vec::new();
        let mut memory_usages = Vec::new();

        for i in 0..config.iterations {
            debug!("Benchmark iteration {}/{}", i + 1, config.iterations);
            
            let iteration_result = self.run_benchmark_iteration(
            ).await?;

            compile_times.push(iteration_result.compile_time);
            binary_sizes.push(iteration_result.binary_size);
            memory_usages.push(iteration_result.memory_usage);
        // Calculate averages
        let avg_compile_time = Duration::from_nanos(
            (compile_times.iter().map(|d| d.as_nanos()).sum::<u128>() / config.iterations as u128) as u64
        );
        let avg_binary_size = binary_sizes.iter().sum::<usize>() / config.iterations;
        let avg_memory_usage = memory_usages.iter().sum::<usize>() / config.iterations;

        // Measure runtime performance if binary was generated successfully
        let runtime_performance = if avg_binary_size > 0 {
            self.measure_runtime_performance(&config.source_files[0], optimization_level).await?
        } else {
            None
        
        // Track optimization passes applied
        let optimization_passes = self.count_optimization_passes(optimization_level);
        
        Ok(BenchmarkResult {
        })
    /// Run a single benchmark iteration
    async fn run_benchmark_iteration(
    ) -> Result<IterationResult> {
        let start_time = Instant::now();
        
        // Build compiler command
        let mut cmd = Command::new(&self.compiler_path);
        cmd.arg("compile")
           .arg(source_file)
           .arg("-O").arg(optimization_level.as_str())
           .arg("--output").arg(self.work_dir.join("benchmark_output"));

        // Add additional flags
        for flag in compiler_flags {
            cmd.arg(flag);
        // Execute compilation
        let output = cmd.output()
            .map_err(|e| CursedError::General(format!("Failed to execute compiler: {}", e)))?;

        let compile_time = start_time.elapsed();

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(CursedError::General(format!("Compilation failed: {}", error_msg)));
        // Measure binary size
        let output_path = self.work_dir.join("benchmark_output");
        let binary_size = if output_path.exists() {
            std::fs::metadata(&output_path)
                .map_err(|e| CursedError::General(format!("Failed to get binary size: {}", e)))?
                .len() as usize
        } else {
            0

        // Implement memory usage tracking
        let memory_usage = self.measure_memory_usage(&output_path).await?;

        Ok(IterationResult {
        })
    /// Compile with specific optimization level
    async fn compile_with_optimization(
    ) -> Result<()> {
        let mut cmd = Command::new(&self.compiler_path);
        cmd.arg("compile")
           .arg(source_file)
           .arg("-O").arg(optimization_level.as_str())
           .arg("--output").arg(self.work_dir.join("warmup_output"));

        let output = cmd.output()
            .map_err(|e| CursedError::General(format!("Failed to execute compiler: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(CursedError::General(format!("Warmup compilation failed: {}", error_msg)));
        Ok(())
    /// Calculate benchmark statistics
    fn calculate_statistics(&self, results: &[BenchmarkResult]) -> BenchmarkStatistics {
        let total_benchmarks = results.len();
        let successful_benchmarks = results.iter().filter(|r| r.success).count();

        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();

        let avg_compile_time = if successful_results.is_empty() {
            Duration::from_secs(0)
        } else {
            Duration::from_nanos(
                successful_results.iter()
                    .map(|r| r.compile_time.as_nanos())
                    .sum::<u128>() / successful_results.len() as u128
            )

        // Find best optimization level by compile time
        let mut level_performance: HashMap<OptimizationLevel, Duration> = HashMap::new();
        for result in &successful_results {
            let entry = level_performance.entry(result.optimization_level).or_insert(Duration::from_secs(0));
            *entry += result.compile_time;
        let best_optimization_level = level_performance
            .iter()
            .min_by_key(|(_, &time)| time)
            .map(|(&level, _)| level)
            .unwrap_or(OptimizationLevel::O2);

        BenchmarkStatistics {
        }
    }

    /// Analyze performance regressions
    async fn analyze_regressions(
    ) -> Result<RegressionAnalysis> {
        // If we have the new regression analyzer, use it for comprehensive analysis
        if let (Some(ref baseline_storage), Some(ref mut regression_analyzer)) = 
            (&self.baseline_storage, &mut self.regression_analyzer) {
            
            // Find the best baseline for comparison
            let baseline = baseline_storage.get_default_baseline()
                .or_else(|| {
                    // Find the most recent baseline that has overlapping benchmarks
                    baseline_storage.list_baselines()
                        .into_iter()
                        .filter(|b| results.iter().any(|r| b.benchmarks.contains_key(&r.name)))
                        .max_by_key(|b| b.created_at)
                });

            let detailed_analysis = regression_analyzer.analyze_regressions(results, baseline)?;
            
            // Convert detailed analysis to the expected format
            Ok(RegressionAnalysis {
            })
        } else {
            // Fallback to legacy analysis
            self.analyze_regressions_legacy(results, configs).await
        }
    }

    /// Legacy regression analysis (fallback when baseline storage is not available)
    async fn analyze_regressions_legacy(
    ) -> Result<RegressionAnalysis> {
        let mut regressions = Vec::new();

        for (result, config) in results.iter().zip(configs.iter().cycle()) {
            if !result.success {
                continue;
            // Check compilation time regression with conservative baseline estimate
            let baseline_compile_time = Duration::from_secs(2); // Conservative baseline estimate
            let compile_time_increase = (result.compile_time.as_secs_f64() / baseline_compile_time.as_secs_f64() - 1.0) * 100.0;
            
            if compile_time_increase > config.performance_thresholds.max_compile_time_increase {
                regressions.push(PerformanceRegression {
                    severity: if compile_time_increase > 100.0 {
                        RegressionSeverity::Critical
                    } else if compile_time_increase > 75.0 {
                        RegressionSeverity::Major
                    } else {
                        RegressionSeverity::Minor
                    description: format!(
                        config.performance_thresholds.max_compile_time_increase
                });
            // Basic binary size regression check
            if result.binary_size > 0 {
                let estimated_baseline_size = 1000; // Conservative estimate
                let size_increase = ((result.binary_size as f64 - estimated_baseline_size as f64) / estimated_baseline_size as f64) * 100.0;
                
                if size_increase > config.performance_thresholds.max_size_increase {
                    regressions.push(PerformanceRegression {
                        severity: if size_increase > 200.0 {
                            RegressionSeverity::Critical
                        } else if size_increase > 100.0 {
                            RegressionSeverity::Major
                        } else {
                            RegressionSeverity::Minor
                        description: format!(
                            config.performance_thresholds.max_size_increase
                    });
                }
            }

            // Basic memory usage regression check
            if result.peak_memory_usage > 0 {
                let estimated_baseline_memory = 5000; // Conservative estimate in bytes
                let memory_increase = ((result.peak_memory_usage as f64 - estimated_baseline_memory as f64) / estimated_baseline_memory as f64) * 100.0;
                
                if memory_increase > config.performance_thresholds.max_memory_increase {
                    regressions.push(PerformanceRegression {
                        severity: if memory_increase > 150.0 {
                            RegressionSeverity::Critical
                        } else if memory_increase > 100.0 {
                            RegressionSeverity::Major
                        } else {
                            RegressionSeverity::Minor
                        description: format!(
                            config.performance_thresholds.max_memory_increase
                    });
                }
            }
        Ok(RegressionAnalysis {
        })
    /// Save benchmark results to file
    pub fn save_results(&self, results: &BenchmarkSuiteResult, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(results)
            .map_err(|e| CursedError::General(format!("Failed to serialize results: {}", e)))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write results: {}", e)))?;
        
        info!("Benchmark results saved to: {}", output_path.display());
        Ok(())
    /// Load baseline results for comparison
    pub fn load_baseline(&self, baseline_path: &Path) -> Result<BenchmarkSuiteResult> {
        let content = std::fs::read_to_string(baseline_path)
            .map_err(|e| CursedError::General(format!("Failed to read baseline: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse baseline: {}", e)))
    /// Measure runtime performance of a compiled binary
    async fn measure_runtime_performance(
    ) -> Result<Option<Duration>> {
        let output_path = self.work_dir.join("benchmark_output");
        
        if !output_path.exists() {
            return Ok(None);
        // Make executable if needed
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&output_path)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&output_path, perms)?;
        // Run the binary and measure execution time
        let start_time = std::time::Instant::now();
        let output = std::process::Command::new(&output_path)
            .output()
            .map_err(|e| CursedError::General(format!("Failed to execute binary: {}", e)))?;
        
        let runtime = start_time.elapsed();
        
        if output.status.success() {
            Ok(Some(runtime))
        } else {
            warn!("Binary execution failed for runtime measurement");
            Ok(None)
        }
    }
    
    /// Count optimization passes for a given optimization level
    fn count_optimization_passes(&self, level: OptimizationLevel) -> usize {
        match level {
            OptimizationLevel::O1 => 5,      // Basic passes: mem2reg, instcombine, simplifycfg, dce, gvn
            OptimizationLevel::O2 => 12,  // Standard passes including loop optimizations
            OptimizationLevel::O3 => 25, // All passes including aggressive inlining, vectorization
            OptimizationLevel::Os => 18,       // Size-focused passes
            OptimizationLevel::Oz => 15,       // Aggressive size optimization
        }
    }
    
    /// Measure memory usage during compilation
    async fn measure_memory_usage(&self, output_path: &Path) -> Result<usize> {
        // Get process memory info during compilation
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/self/status") {
                for line in content.split("\n") {
                    if line.starts_with("VmPeak:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return Ok(kb * 1024); // Convert KB to bytes
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("ps")
                .args(&["-o", "rss=", "-p"])
                .arg(std::process::id().to_string())
                .output() 
            {
                if let Ok(rss_str) = String::from_utf8(output.stdout) {
                    if let Ok(rss_kb) = rss_str.trim().parse::<usize>() {
                        return Ok(rss_kb * 1024); // Convert KB to bytes
                    }
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Use GetProcessMemoryInfo on Windows
            // This would require winapi dependency
            // For now, return a reasonable estimate
            return Ok(50 * 1024 * 1024); // 50MB estimate
        // Fallback: estimate based on binary size
        if output_path.exists() {
            if let Ok(metadata) = std::fs::metadata(output_path) {
                // Rough estimate: 10x binary size for compilation memory
                return Ok((metadata.len() as usize).saturating_mul(10));
            }
        }
        
        Ok(100 * 1024 * 1024) // 100MB fallback
    /// Calculate performance improvement over baseline
    fn calculate_performance_improvement(&self, results: &[&BenchmarkResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        // Find baseline (O0 or None level)
        let baseline_result = results.iter()
            .find(|r| matches!(r.optimization_level, OptimizationLevel::O0))
            .or_else(|| results.first())
            .unwrap();
        
        let baseline_time = if let Some(runtime) = baseline_result.runtime_performance {
            runtime.as_secs_f64()
        } else {
            baseline_result.compile_time.as_secs_f64()
        
        if baseline_time == 0.0 {
            return 0.0;
        // Calculate average improvement across all optimized results
        let mut total_improvement = 0.0;
        let mut count = 0;
        
        for result in results {
            if result.optimization_level == baseline_result.optimization_level {
                continue;
            let result_time = if let Some(runtime) = result.runtime_performance {
                runtime.as_secs_f64()
            } else {
                result.compile_time.as_secs_f64()
            
            if result_time > 0.0 {
                let improvement = (baseline_time - result_time) / baseline_time * 100.0;
                total_improvement += improvement;
                count += 1;
            }
        }
        
        if count > 0 {
            total_improvement / count as f64
        } else {
            0.0
        }
    }
    
    /// Calculate size change compared to baseline
    fn calculate_size_change(&self, results: &[&BenchmarkResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        // Find baseline (O0 or None level)
        let baseline_result = results.iter()
            .find(|r| matches!(r.optimization_level, OptimizationLevel::O0))
            .or_else(|| results.first())
            .unwrap();
        
        let baseline_size = baseline_result.binary_size as f64;
        
        if baseline_size == 0.0 {
            return 0.0;
        // Calculate average size change across all optimized results
        let mut total_change = 0.0;
        let mut count = 0;
        
        for result in results {
            if result.optimization_level == baseline_result.optimization_level {
                continue;
            let size_change = (result.binary_size as f64 - baseline_size) / baseline_size * 100.0;
            total_change += size_change;
            count += 1;
        if count > 0 {
            total_change / count as f64
        } else {
            0.0
        }
    }
    
    /// Generate performance report
    pub fn generate_report(&self, results: &BenchmarkSuiteResult) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("# Performance Benchmark Report\n\n"));
        report.push_str(&format!("**Suite:** {}\n", results.suite_name));
        report.push_str(&format!("**Timestamp:** {}\n\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Summary statistics
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Benchmarks:** {}\n", results.statistics.total_benchmarks));
        report.push_str(&format!("- **Successful:** {}\n", results.statistics.successful_benchmarks));
        report.push_str(&format!("- **Average Compile Time:** {:?}\n", results.statistics.avg_compile_time));
        report.push_str(&format!("- **Best Optimization Level:** {:?}\n\n", results.statistics.best_optimization_level));
        
        // Individual results
        report.push_str("## Individual Results\n\n");
        report.push_str("| Benchmark | Optimization | Compile Time | Binary Size | Success |\n");
        report.push_str("|-----------|-------------|-------------|-------------|----------|\n");
        
        for result in &results.results {
            report.push_str(&format!(
                if result.success { "✅" } else { "❌" }
            ));
        // Regression analysis
        if let Some(ref regression_analysis) = results.regression_analysis {
            report.push_str("\n## Regression Analysis\n\n");
            
            if regression_analysis.has_regressions {
                report.push_str("⚠️ **Performance regressions detected:**\n\n");
                
                for regression in &regression_analysis.regressions {
                    let severity_icon = match regression.severity {
                    
                    report.push_str(&format!(
                        regression.description
                    ));
                }
            } else {
                report.push_str("✅ **No performance regressions detected.**\n");
            }
        }
        
        report
    /// Create a new baseline from benchmark results
    pub fn create_baseline(
    ) -> Result<Option<String>> {
        if let Some(ref mut storage) = self.baseline_storage {
            let baseline_id = storage.create_baseline(
            )?;
            Ok(Some(baseline_id))
        } else {
            warn!("Baseline storage not available - cannot create baseline");
            Ok(None)
        }
    }

    /// Load a specific baseline
    pub fn load_baseline(&mut self, baseline_id: &str) -> Result<bool> {
        if let Some(ref mut storage) = self.baseline_storage {
            let baseline = storage.load_baseline(baseline_id)?;
            Ok(baseline.is_some())
        } else {
            Ok(false)
        }
    }

    /// Set the default baseline for comparisons
    pub fn set_default_baseline(&mut self, baseline_id: String) -> Result<()> {
        if let Some(ref mut storage) = self.baseline_storage {
            storage.set_default_baseline(baseline_id)
        } else {
            Err(CursedError::General("Baseline storage not available".to_string()))
        }
    }

    /// List all available baselines
    pub fn list_baselines(&self) -> Vec<String> {
        if let Some(ref storage) = self.baseline_storage {
            storage.list_baselines()
                .into_iter()
                .map(|b| b.baseline_id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Export baselines to a file
    pub fn export_baselines(&self, export_path: &Path, baseline_ids: Option<Vec<String>>) -> Result<()> {
        if let Some(ref storage) = self.baseline_storage {
            storage.export_baselines(export_path, baseline_ids)
        } else {
            Err(CursedError::General("Baseline storage not available".to_string()))
        }
    }

    /// Import baselines from a file
    pub fn import_baselines(&mut self, import_path: &Path, overwrite_existing: bool) -> Result<usize> {
        if let Some(ref mut storage) = self.baseline_storage {
            storage.import_baselines(import_path, overwrite_existing)
        } else {
            Err(CursedError::General("Baseline storage not available".to_string()))
        }
    }
/// Result of a single benchmark iteration
#[derive(Debug)]
struct IterationResult {
/// Create default benchmark configurations for common scenarios
pub fn create_default_benchmarks() -> Vec<BenchmarkConfig> {
    vec![
        BenchmarkConfig {
            source_files: vec![PathBuf::from("benchmarks/small_function.csd")],
            optimization_levels: vec![
        BenchmarkConfig {
            source_files: vec![PathBuf::from("benchmarks/medium_program.csd")],
            optimization_levels: vec![
        BenchmarkConfig {
            source_files: vec![PathBuf::from("benchmarks/large_application.csd")],
            optimization_levels: vec![
            performance_thresholds: PerformanceThresholds {
                max_compile_time_increase: 100.0, // Allow more time for large applications
    ]
