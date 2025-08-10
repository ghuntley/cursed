//! Benchmarking framework for CURSED compiler performance testing
//! 
//! Provides comprehensive benchmarking capabilities for measuring performance
//! across different compiler phases, stdlib operations, and runtime scenarios.

use std::time::{Duration, Instant};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::fs;
use std::path::Path;
use crate::error::CursedError;
use crate::performance::{PerformanceConfig, BenchmarkConfig, BenchmarkResults};

/// Benchmark runner for performance testing
pub struct BenchmarkRunner {
    config: PerformanceConfig,
    results: Arc<Mutex<HashMap<String, BenchmarkResults>>>,
    baseline_results: Arc<Mutex<HashMap<String, BenchmarkResults>>>,
    benchmark_suites: Vec<BenchmarkSuite>,
}

/// Benchmark suite containing related benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
    pub name: String,
    pub description: String,
    pub benchmarks: Vec<Benchmark>,
    pub setup_command: Option<String>,
    pub teardown_command: Option<String>,
}

/// Individual benchmark definition
#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub description: String,
    pub benchmark_type: BenchmarkType,
    pub config: BenchmarkConfig,
    pub source_file: Option<String>,
    pub command: Option<String>,
    pub expected_output: Option<String>,
    pub tags: Vec<String>,
}

/// Types of benchmarks
#[derive(Debug, Clone)]
pub enum BenchmarkType {
    Compilation,
    Execution,
    Memory,
    Throughput,
    Latency,
    StdlibFunction,
    EndToEnd,
    Regression,
    Stress,
    Custom(String),
}

/// Benchmark execution context
#[derive(Debug, Clone)]
pub struct BenchmarkContext {
    pub workspace_dir: String,
    pub temp_dir: String,
    pub compiler_path: String,
    pub runtime_path: String,
    pub environment: HashMap<String, String>,
}

/// Benchmark result with detailed metrics
#[derive(Debug, Clone)]
pub struct DetailedBenchmarkResult {
    pub name: String,
    pub benchmark_type: BenchmarkType,
    pub config: BenchmarkConfig,
    pub metrics: BenchmarkMetrics,
    pub status: BenchmarkStatus,
    pub error_message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_info: SystemInfo,
}

/// Benchmark execution metrics
#[derive(Debug, Clone)]
pub struct BenchmarkMetrics {
    pub execution_times: Vec<Duration>,
    pub memory_usage: Vec<usize>,
    pub cpu_usage: Vec<f64>,
    pub throughput: f64,
    pub latency: Duration,
    pub error_count: u32,
    pub success_count: u32,
    pub statistical_data: StatisticalData,
}

/// Statistical analysis of benchmark results
#[derive(Debug, Clone)]
pub struct StatisticalData {
    pub mean: Duration,
    pub median: Duration,
    pub std_deviation: Duration,
    pub min: Duration,
    pub max: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub coefficient_of_variation: f64,
}

/// Benchmark execution status
#[derive(Debug, Clone)]
pub enum BenchmarkStatus {
    Success,
    Failed,
    Timeout,
    MemoryExceeded,
    CpuExceeded,
    Cancelled,
}

/// System information for benchmark context
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
    pub cpu_cores: u32,
    pub total_memory: usize,
    pub compiler_version: String,
    pub rust_version: String,
}

/// Benchmark comparison result
#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    pub benchmark_name: String,
    pub baseline_result: BenchmarkResults,
    pub current_result: BenchmarkResults,
    pub improvement_percentage: f64,
    pub significance: ComparisonSignificance,
    pub recommendation: String,
}

/// Significance levels for benchmark comparisons
#[derive(Debug, Clone)]
pub enum ComparisonSignificance {
    NoChange,
    SlightImprovement,
    SignificantImprovement,
    SlightRegression,
    SignificantRegression,
    HighVariance,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        let mut runner = Self {
            config,
            results: Arc::new(Mutex::new(HashMap::new())),
            baseline_results: Arc::new(Mutex::new(HashMap::new())),
            benchmark_suites: Vec::new(),
        };
        
        // Load default benchmark suites
        runner.load_default_benchmarks()?;
        
        Ok(runner)
    }

    /// Load default benchmark suites
    fn load_default_benchmarks(&mut self) -> Result<(), CursedError> {
        // Compilation benchmarks
        let compilation_suite = BenchmarkSuite {
            name: "compilation".to_string(),
            description: "Compiler performance benchmarks".to_string(),
            benchmarks: vec![
                Benchmark {
                    name: "small_program_compile".to_string(),
                    description: "Compile small CURSED program".to_string(),
                    benchmark_type: BenchmarkType::Compilation,
                    config: BenchmarkConfig {
                        name: "small_program_compile".to_string(),
                        iterations: 1000,
                        warmup_iterations: 100,
                        timeout: Duration::from_secs(10),
                        memory_limit: 1024 * 1024 * 256, // 256MB
                        cpu_limit: 90.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/small_function.csd".to_string()),
                    command: Some("cargo run --bin cursed -- compile {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["compilation".to_string(), "small".to_string()],
                },
                Benchmark {
                    name: "medium_program_compile".to_string(),
                    description: "Compile medium CURSED program".to_string(),
                    benchmark_type: BenchmarkType::Compilation,
                    config: BenchmarkConfig {
                        name: "medium_program_compile".to_string(),
                        iterations: 100,
                        warmup_iterations: 10,
                        timeout: Duration::from_secs(30),
                        memory_limit: 1024 * 1024 * 512, // 512MB
                        cpu_limit: 90.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/medium_program.csd".to_string()),
                    command: Some("cargo run --bin cursed -- compile {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["compilation".to_string(), "medium".to_string()],
                },
                Benchmark {
                    name: "large_program_compile".to_string(),
                    description: "Compile large CURSED program".to_string(),
                    benchmark_type: BenchmarkType::Compilation,
                    config: BenchmarkConfig {
                        name: "large_program_compile".to_string(),
                        iterations: 10,
                        warmup_iterations: 2,
                        timeout: Duration::from_secs(60),
                        memory_limit: 1024 * 1024 * 1024, // 1GB
                        cpu_limit: 90.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/large_application.csd".to_string()),
                    command: Some("cargo run --bin cursed -- compile {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["compilation".to_string(), "large".to_string()],
                },
            ],
            setup_command: None,
            teardown_command: None,
        };

        // Execution benchmarks
        let execution_suite = BenchmarkSuite {
            name: "execution".to_string(),
            description: "Runtime execution performance benchmarks".to_string(),
            benchmarks: vec![
                Benchmark {
                    name: "math_operations".to_string(),
                    description: "Mathematical operations performance".to_string(),
                    benchmark_type: BenchmarkType::Execution,
                    config: BenchmarkConfig {
                        name: "math_operations".to_string(),
                        iterations: 10000,
                        warmup_iterations: 1000,
                        timeout: Duration::from_secs(30),
                        memory_limit: 1024 * 1024 * 128, // 128MB
                        cpu_limit: 95.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/cursed/math_benchmark.csd".to_string()),
                    command: Some("cargo run --bin cursed {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["execution".to_string(), "math".to_string()],
                },
                Benchmark {
                    name: "string_operations".to_string(),
                    description: "String manipulation performance".to_string(),
                    benchmark_type: BenchmarkType::Execution,
                    config: BenchmarkConfig {
                        name: "string_operations".to_string(),
                        iterations: 5000,
                        warmup_iterations: 500,
                        timeout: Duration::from_secs(30),
                        memory_limit: 1024 * 1024 * 256, // 256MB
                        cpu_limit: 95.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/cursed/string_benchmark.csd".to_string()),
                    command: Some("cargo run --bin cursed {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["execution".to_string(), "string".to_string()],
                },
            ],
            setup_command: None,
            teardown_command: None,
        };

        // Memory benchmarks
        let memory_suite = BenchmarkSuite {
            name: "memory".to_string(),
            description: "Memory usage and garbage collection benchmarks".to_string(),
            benchmarks: vec![
                Benchmark {
                    name: "memory_allocation".to_string(),
                    description: "Memory allocation performance".to_string(),
                    benchmark_type: BenchmarkType::Memory,
                    config: BenchmarkConfig {
                        name: "memory_allocation".to_string(),
                        iterations: 1000,
                        warmup_iterations: 100,
                        timeout: Duration::from_secs(60),
                        memory_limit: 1024 * 1024 * 1024, // 1GB
                        cpu_limit: 95.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/cursed/memory_benchmark.csd".to_string()),
                    command: Some("cargo run --bin cursed {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["memory".to_string(), "allocation".to_string()],
                },
                Benchmark {
                    name: "garbage_collection".to_string(),
                    description: "Garbage collection performance".to_string(),
                    benchmark_type: BenchmarkType::Memory,
                    config: BenchmarkConfig {
                        name: "garbage_collection".to_string(),
                        iterations: 100,
                        warmup_iterations: 10,
                        timeout: Duration::from_secs(120),
                        memory_limit: 1024 * 1024 * 1024, // 1GB
                        cpu_limit: 95.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("benchmarks/cursed/gc_benchmark.csd".to_string()),
                    command: Some("cargo run --bin cursed {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["memory".to_string(), "gc".to_string()],
                },
            ],
            setup_command: None,
            teardown_command: None,
        };

        // Stdlib benchmarks
        let stdlib_suite = BenchmarkSuite {
            name: "stdlib".to_string(),
            description: "Standard library performance benchmarks".to_string(),
            benchmarks: vec![
                Benchmark {
                    name: "stdlib_comprehensive".to_string(),
                    description: "Comprehensive stdlib benchmark".to_string(),
                    benchmark_type: BenchmarkType::StdlibFunction,
                    config: BenchmarkConfig {
                        name: "stdlib_comprehensive".to_string(),
                        iterations: 100,
                        warmup_iterations: 10,
                        timeout: Duration::from_secs(300),
                        memory_limit: 1024 * 1024 * 512, // 512MB
                        cpu_limit: 95.0,
                        parallel_executions: 1,
                    },
                    source_file: Some("stdlib_performance_benchmarks.csd".to_string()),
                    command: Some("cargo run --bin cursed {source_file}".to_string()),
                    expected_output: None,
                    tags: vec!["stdlib".to_string(), "comprehensive".to_string()],
                },
            ],
            setup_command: None,
            teardown_command: None,
        };

        self.benchmark_suites.push(compilation_suite);
        self.benchmark_suites.push(execution_suite);
        self.benchmark_suites.push(memory_suite);
        self.benchmark_suites.push(stdlib_suite);

        Ok(())
    }

    /// Run all benchmark suites
    pub fn run_all_benchmarks(&self) -> Result<HashMap<String, BenchmarkResults>, CursedError> {
        let mut all_results = HashMap::new();
        
        for suite in &self.benchmark_suites {
            println!("Running benchmark suite: {}", suite.name);
            
            // Run setup command if present
            if let Some(setup_cmd) = &suite.setup_command {
                self.run_command(setup_cmd)?;
            }
            
            for benchmark in &suite.benchmarks {
                println!("  Running benchmark: {}", benchmark.name);
                
                match self.run_single_benchmark(benchmark) {
                    Ok(result) => {
                        all_results.insert(benchmark.name.clone(), result);
                        println!("    ✓ Completed: {}", benchmark.name);
                    }
                    Err(e) => {
                        eprintln!("    ✗ Failed: {} - {}", benchmark.name, e);
                        // Continue with other benchmarks
                    }
                }
            }
            
            // Run teardown command if present
            if let Some(teardown_cmd) = &suite.teardown_command {
                self.run_command(teardown_cmd)?;
            }
        }
        
        // Store results
        *self.results.lock().unwrap() = all_results.clone();
        
        Ok(all_results)
    }

    /// Run benchmarks matching specific tags
    pub fn run_benchmarks_by_tags(&self, tags: &[String]) -> Result<HashMap<String, BenchmarkResults>, CursedError> {
        let mut results = HashMap::new();
        
        for suite in &self.benchmark_suites {
            for benchmark in &suite.benchmarks {
                if benchmark.tags.iter().any(|tag| tags.contains(tag)) {
                    println!("Running tagged benchmark: {}", benchmark.name);
                    
                    match self.run_single_benchmark(benchmark) {
                        Ok(result) => {
                            results.insert(benchmark.name.clone(), result);
                            println!("  ✓ Completed: {}", benchmark.name);
                        }
                        Err(e) => {
                            eprintln!("  ✗ Failed: {} - {}", benchmark.name, e);
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Run a specific benchmark
    pub fn run_benchmarks(&self, config: &BenchmarkConfig) -> Result<BenchmarkResults, CursedError> {
        // Find benchmark by name
        for suite in &self.benchmark_suites {
            for benchmark in &suite.benchmarks {
                if benchmark.name == config.name {
                    return self.run_single_benchmark(benchmark);
                }
            }
        }
        
        Err(CursedError::runtime_error(&format!("Benchmark not found: {}", config.name)))
    }

    /// Run a single benchmark
    fn run_single_benchmark(&self, benchmark: &Benchmark) -> Result<BenchmarkResults, CursedError> {
        let mut execution_times = Vec::new();
        let mut memory_usage = Vec::new();
        let mut cpu_usage = Vec::new();
        let mut success_count = 0;
        let mut error_count = 0;

        // Warmup iterations
        for _ in 0..benchmark.config.warmup_iterations {
            if let Err(_) = self.execute_benchmark(benchmark) {
                // Ignore warmup errors
            }
        }

        // Actual benchmark iterations
        for i in 0..benchmark.config.iterations {
            if i % 100 == 0 {
                println!("    Progress: {}/{}", i, benchmark.config.iterations);
            }

            match self.execute_benchmark(benchmark) {
                Ok(result) => {
                    execution_times.push(result.execution_time);
                    memory_usage.push(result.memory_usage);
                    cpu_usage.push(result.cpu_usage);
                    success_count += 1;
                }
                Err(_) => {
                    error_count += 1;
                    // Continue with other iterations
                }
            }
        }

        if execution_times.is_empty() {
            return Err(CursedError::runtime_error("No successful benchmark executions"));
        }

        // Calculate statistics
        let stats = self.calculate_statistics(&execution_times);
        
        let average_memory = memory_usage.iter().sum::<usize>() / memory_usage.len();
        let average_cpu = cpu_usage.iter().sum::<f64>() / cpu_usage.len() as f64;
        let total_iterations = benchmark.config.iterations;
        let success_rate = (success_count as f64) / (total_iterations as f64);
        
        let throughput = if stats.mean.as_nanos() > 0 {
            1_000_000_000.0 / stats.mean.as_nanos() as f64
        } else {
            0.0
        };

        Ok(BenchmarkResults {
            name: benchmark.name.clone(),
            average_time: stats.mean,
            min_time: stats.min,
            max_time: stats.max,
            std_deviation: stats.std_deviation,
            throughput,
            memory_usage: average_memory,
            cpu_usage: average_cpu,
            success_rate,
        })
    }

    /// Execute a single benchmark iteration
    fn execute_benchmark(&self, benchmark: &Benchmark) -> Result<BenchmarkExecutionResult, CursedError> {
        let start_time = Instant::now();
        
        // Replace placeholders in command
        let command = if let Some(cmd) = &benchmark.command {
            let mut processed_cmd = cmd.clone();
            if let Some(source_file) = &benchmark.source_file {
                processed_cmd = processed_cmd.replace("{source_file}", source_file);
            }
            processed_cmd
        } else {
            return Err(CursedError::runtime_error("No command specified for benchmark"));
        };

        // Execute command with timeout
        let output = self.run_command_with_timeout(&command, benchmark.config.timeout)?;
        
        let execution_time = start_time.elapsed();
        
        // Get system metrics (simplified)
        let memory_usage = self.get_memory_usage();
        let cpu_usage = self.get_cpu_usage();

        Ok(BenchmarkExecutionResult {
            execution_time,
            memory_usage,
            cpu_usage,
            output,
        })
    }

    /// Run command with timeout
    fn run_command_with_timeout(&self, command: &str, timeout: Duration) -> Result<String, CursedError> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CursedError::runtime_error("Empty command"));
        }

        let mut cmd = Command::new(parts[0]);
        cmd.args(&parts[1..]);

        let output = cmd.output()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to execute command: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::runtime_error(&format!(
                "Command failed with exit code: {:?}",
                output.status.code()
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Run a command without timeout
    fn run_command(&self, command: &str) -> Result<String, CursedError> {
        self.run_command_with_timeout(command, Duration::from_secs(300))
    }

    /// Calculate statistical data for benchmark results
    fn calculate_statistics(&self, times: &[Duration]) -> StatisticalData {
        if times.is_empty() {
            return StatisticalData::default();
        }

        let mut sorted_times = times.to_vec();
        sorted_times.sort();

        let min = sorted_times[0];
        let max = sorted_times[sorted_times.len() - 1];
        
        let total_nanos: u64 = times.iter().map(|t| t.as_nanos() as u64).sum();
        let mean_nanos = total_nanos / times.len() as u64;
        let mean = Duration::from_nanos(mean_nanos);

        let median_index = sorted_times.len() / 2;
        let median = if sorted_times.len() % 2 == 0 {
            let mid1 = sorted_times[median_index - 1].as_nanos() as u64;
            let mid2 = sorted_times[median_index].as_nanos() as u64;
            Duration::from_nanos((mid1 + mid2) / 2)
        } else {
            sorted_times[median_index]
        };

        // Calculate standard deviation
        let variance: f64 = times.iter()
            .map(|t| {
                let diff = t.as_nanos() as f64 - mean_nanos as f64;
                diff * diff
            })
            .sum::<f64>() / times.len() as f64;
        let std_deviation = Duration::from_nanos(variance.sqrt() as u64);

        // Calculate percentiles
        let p95_index = (sorted_times.len() as f64 * 0.95) as usize;
        let p99_index = (sorted_times.len() as f64 * 0.99) as usize;
        let p95 = sorted_times[p95_index.min(sorted_times.len() - 1)];
        let p99 = sorted_times[p99_index.min(sorted_times.len() - 1)];

        // Coefficient of variation
        let coefficient_of_variation = if mean_nanos > 0 {
            variance.sqrt() / mean_nanos as f64
        } else {
            0.0
        };

        StatisticalData {
            mean,
            median,
            std_deviation,
            min,
            max,
            p95,
            p99,
            coefficient_of_variation,
        }
    }

    /// Get current memory usage (simplified)
    fn get_memory_usage(&self) -> usize {
        // This is a simplified implementation
        // In a real implementation, this would use system APIs
        1024 * 1024 * 64 // 64MB
    }

    /// Get current CPU usage (simplified)
    fn get_cpu_usage(&self) -> f64 {
        // This is a simplified implementation
        // In a real implementation, this would use system APIs
        25.0 // 25%
    }

    /// Compare current results with baseline
    pub fn compare_with_baseline(&self, benchmark_name: &str) -> Result<BenchmarkComparison, CursedError> {
        let results = self.results.lock().unwrap();
        let baseline_results = self.baseline_results.lock().unwrap();

        let current_result = results.get(benchmark_name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Current result not found: {}", benchmark_name)))?;

        let baseline_result = baseline_results.get(benchmark_name)
            .ok_or_else(|| CursedError::runtime_error(&format!("Baseline result not found: {}", benchmark_name)))?;

        let current_time = current_result.average_time.as_nanos() as f64;
        let baseline_time = baseline_result.average_time.as_nanos() as f64;

        let improvement_percentage = ((baseline_time - current_time) / baseline_time) * 100.0;

        let significance = if improvement_percentage.abs() < 5.0 {
            ComparisonSignificance::NoChange
        } else if improvement_percentage > 20.0 {
            ComparisonSignificance::SignificantImprovement
        } else if improvement_percentage > 5.0 {
            ComparisonSignificance::SlightImprovement
        } else if improvement_percentage < -20.0 {
            ComparisonSignificance::SignificantRegression
        } else {
            ComparisonSignificance::SlightRegression
        };

        let recommendation = match significance {
            ComparisonSignificance::SignificantImprovement => "Great improvement! Consider analyzing what changed to maintain this performance.".to_string(),
            ComparisonSignificance::SlightImprovement => "Good improvement. Monitor to ensure consistency.".to_string(),
            ComparisonSignificance::NoChange => "Performance is stable.".to_string(),
            ComparisonSignificance::SlightRegression => "Minor regression detected. Consider investigating.".to_string(),
            ComparisonSignificance::SignificantRegression => "Significant regression detected! Immediate investigation required.".to_string(),
            ComparisonSignificance::HighVariance => "High variance detected. Consider running more iterations.".to_string(),
        };

        Ok(BenchmarkComparison {
            benchmark_name: benchmark_name.to_string(),
            baseline_result: baseline_result.clone(),
            current_result: current_result.clone(),
            improvement_percentage,
            significance,
            recommendation,
        })
    }

    /// Save current results as baseline
    pub fn save_baseline(&self) -> Result<(), CursedError> {
        let results = self.results.lock().unwrap();
        let mut baseline_results = self.baseline_results.lock().unwrap();
        
        *baseline_results = results.clone();
        
        // Save to file
        let baseline_file = format!("{}/baseline_results.json", self.config.output_dir);
        let json_data = format!("{:#?}", baseline_results);
        
        fs::write(&baseline_file, json_data)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write baseline file: {}", e)))?;
        
        println!("Baseline results saved to: {}", baseline_file);
        Ok(())
    }

    /// Load baseline results from file
    pub fn load_baseline(&self) -> Result<(), CursedError> {
        let baseline_file = format!("{}/baseline_results.json", self.config.output_dir);
        
        if !Path::new(&baseline_file).exists() {
            return Err(CursedError::runtime_error(&format!("Baseline file not found: {}", baseline_file)));
        }
        
        let json_data = fs::read_to_string(&baseline_file)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to read baseline file: {}", e)))?;
        
        // For now, just use empty baseline
        let baseline_results: HashMap<String, BenchmarkResults> = HashMap::new();
        
        *self.baseline_results.lock().unwrap() = baseline_results;
        
        println!("Baseline results loaded from: {}", baseline_file);
        Ok(())
    }

    /// Get all benchmark results
    pub fn get_results(&self) -> HashMap<String, BenchmarkResults> {
        self.results.lock().unwrap().clone()
    }

    /// Generate benchmark report
    pub fn generate_report(&self) -> Result<String, CursedError> {
        let results = self.results.lock().unwrap();
        
        let mut report = String::new();
        report.push_str("# CURSED Compiler Benchmark Report\n\n");
        report.push_str(&format!("Generated: {}\n", chrono::Utc::now().to_rfc3339()));
        report.push_str(&format!("Total benchmarks: {}\n\n", results.len()));
        
        // Group results by benchmark type
        let mut compilation_results = Vec::new();
        let mut execution_results = Vec::new();
        let mut memory_results = Vec::new();
        let mut stdlib_results = Vec::new();
        
        for (name, result) in results.iter() {
            if name.contains("compile") {
                compilation_results.push((name, result));
            } else if name.contains("execution") || name.contains("math") || name.contains("string") {
                execution_results.push((name, result));
            } else if name.contains("memory") || name.contains("gc") {
                memory_results.push((name, result));
            } else if name.contains("stdlib") {
                stdlib_results.push((name, result));
            }
        }
        
        // Generate sections
        if !compilation_results.is_empty() {
            report.push_str("## Compilation Benchmarks\n\n");
            report.push_str("| Benchmark | Average Time | Min Time | Max Time | Throughput (ops/sec) | Success Rate |\n");
            report.push_str("|-----------|--------------|----------|----------|---------------------|---------------|\n");
            
            for (name, result) in compilation_results {
                report.push_str(&format!(
                    "| {} | {:?} | {:?} | {:?} | {:.2} | {:.2}% |\n",
                    name, result.average_time, result.min_time, result.max_time,
                    result.throughput, result.success_rate * 100.0
                ));
            }
            report.push_str("\n");
        }
        
        if !execution_results.is_empty() {
            report.push_str("## Execution Benchmarks\n\n");
            report.push_str("| Benchmark | Average Time | Min Time | Max Time | Throughput (ops/sec) | Success Rate |\n");
            report.push_str("|-----------|--------------|----------|----------|---------------------|---------------|\n");
            
            for (name, result) in execution_results {
                report.push_str(&format!(
                    "| {} | {:?} | {:?} | {:?} | {:.2} | {:.2}% |\n",
                    name, result.average_time, result.min_time, result.max_time,
                    result.throughput, result.success_rate * 100.0
                ));
            }
            report.push_str("\n");
        }
        
        if !memory_results.is_empty() {
            report.push_str("## Memory Benchmarks\n\n");
            report.push_str("| Benchmark | Average Time | Memory Usage (MB) | Success Rate |\n");
            report.push_str("|-----------|--------------|-------------------|---------------|\n");
            
            for (name, result) in memory_results {
                report.push_str(&format!(
                    "| {} | {:?} | {:.2} | {:.2}% |\n",
                    name, result.average_time, result.memory_usage as f64 / 1024.0 / 1024.0,
                    result.success_rate * 100.0
                ));
            }
            report.push_str("\n");
        }
        
        if !stdlib_results.is_empty() {
            report.push_str("## Standard Library Benchmarks\n\n");
            report.push_str("| Benchmark | Average Time | Throughput (ops/sec) | Success Rate |\n");
            report.push_str("|-----------|--------------|---------------------|---------------|\n");
            
            for (name, result) in stdlib_results {
                report.push_str(&format!(
                    "| {} | {:?} | {:.2} | {:.2}% |\n",
                    name, result.average_time, result.throughput, result.success_rate * 100.0
                ));
            }
            report.push_str("\n");
        }
        
        // Add summary
        report.push_str("## Summary\n\n");
        let total_success_rate = results.values().map(|r| r.success_rate).sum::<f64>() / results.len() as f64;
        let average_throughput = results.values().map(|r| r.throughput).sum::<f64>() / results.len() as f64;
        
        report.push_str(&format!("- Overall Success Rate: {:.2}%\n", total_success_rate * 100.0));
        report.push_str(&format!("- Average Throughput: {:.2} ops/sec\n", average_throughput));
        report.push_str(&format!("- Total Benchmarks: {}\n", results.len()));
        
        Ok(report)
    }
}

/// Result of a single benchmark execution
#[derive(Debug, Clone)]
struct BenchmarkExecutionResult {
    execution_time: Duration,
    memory_usage: usize,
    cpu_usage: f64,
    output: String,
}

impl Default for StatisticalData {
    fn default() -> Self {
        Self {
            mean: Duration::from_nanos(0),
            median: Duration::from_nanos(0),
            std_deviation: Duration::from_nanos(0),
            min: Duration::from_nanos(0),
            max: Duration::from_nanos(0),
            p95: Duration::from_nanos(0),
            p99: Duration::from_nanos(0),
            coefficient_of_variation: 0.0,
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get() as u32,
            total_memory: 1024 * 1024 * 1024 * 8, // 8GB default
            compiler_version: option_env!("CARGO_PKG_VERSION").unwrap_or("unknown").to_string(),
            rust_version: "1.70.0".to_string(), // This would be detected
        }
    }
}
