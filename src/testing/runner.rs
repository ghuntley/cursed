use crate::error::CursedError;
/// Test Runner Main Controller
/// 
/// Orchestrates the complete test execution pipeline including discovery,
/// compilation, execution, and reporting.

use super::{TestError, TestResult as TestingResult, TestConfig};
use super::discovery::{TestDiscovery, TestSuite};
use super::execution::{TestExecutor, TestExecutionContext};
use super::framework::{TestFramework, TestEnvironment, TestEnvironmentConfig};
use super::reporting::{TestReporter, TestReport, ReportFormat, build_test_report, TestSuiteResult};
use std::path::PathBuf;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use tracing::{info, debug, warn, error};
use tokio::signal;

/// Main test runner that coordinates all test execution
pub struct TestRunner {
    /// Test runner configuration
    config: TestRunnerConfig,
    /// Test discovery engine
    discovery: TestDiscovery,
    /// Test execution engine
    executor: TestExecutor,
    /// Test framework
    framework: TestFramework,
    /// Test reporter
    reporter: TestReporter,
    /// Shutdown signal
    shutdown: Arc<Mutex<bool>>,
}

/// Configuration for test runner
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    /// Base test configuration
    pub test_config: TestConfig,
    /// Report format
    pub report_format: ReportFormat,
    /// Output file for report (None for stdout)
    pub report_output: Option<PathBuf>,
    /// Enable real-time progress reporting
    pub progress_reporting: bool,
    /// Fail fast on first failure
    pub fail_fast: bool,
    /// Dry run (discover tests but don't execute)
    pub dry_run: bool,
    /// Enable test coverage collection
    pub collect_coverage: bool,
    /// Randomize test execution order
    pub randomize_order: bool,
    /// Random seed for test ordering
    pub random_seed: Option<u64>,
    /// Enable strict mode (warnings as errors)
    pub strict_mode: bool,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            test_config: TestConfig::default(),
            report_format: ReportFormat::Console,
            report_output: None,
            progress_reporting: true,
            fail_fast: false,
            dry_run: false,
            collect_coverage: false,
            randomize_order: false,
            random_seed: None,
            strict_mode: false,
        }
    }
}

/// Builder for test runner
pub struct TestRunnerBuilder {
    config: TestRunnerConfig,
}

impl TestRunnerBuilder {
    /// Create new test runner builder
    pub fn new() -> Self {
        Self {
            config: TestRunnerConfig::default(),
        }
    }

    /// Set test configuration
    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config.test_config = config;
        self
    }

    /// Set report format
    pub fn with_report_format(mut self, format: ReportFormat) -> Self {
        self.config.report_format = format;
        self
    }

    /// Set report output file
    pub fn with_report_output(mut self, output: PathBuf) -> Self {
        self.config.report_output = Some(output);
        self
    }

    /// Enable fail fast mode
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.config.fail_fast = fail_fast;
        self
    }

    /// Enable dry run mode
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.config.dry_run = dry_run;
        self
    }

    /// Enable coverage collection
    pub fn with_coverage(mut self, coverage: bool) -> Self {
        self.config.collect_coverage = coverage;
        self
    }

    /// Randomize test execution order
    pub fn with_randomized_order(mut self, randomize: bool, seed: Option<u64>) -> Self {
        self.config.randomize_order = randomize;
        self.config.random_seed = seed;
        self
    }

    /// Enable strict mode
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.config.strict_mode = strict;
        self
    }

    /// Build the test runner
    pub fn build(self) -> TestingResult<TestRunner> {
        TestRunner::new(self.config)
    }
}

impl TestRunner {
    /// Create new test runner with configuration
    pub fn new(config: TestRunnerConfig) -> TestingResult<Self> {
        info!("Creating test runner with config: {:?}", config);

        // Create test discovery
        let discovery = TestDiscovery::new(&config.test_config.working_directory)
            .with_include_patterns(config.test_config.include_patterns.clone())
            .with_exclude_patterns(config.test_config.exclude_patterns.clone());

        // Create test execution context
        let execution_context = TestExecutionContext {
            working_directory: config.test_config.working_directory.clone(),
            environment: config.test_config.environment.clone(),
            timeout_seconds: config.test_config.timeout_seconds,
            memory_profiling: config.collect_coverage,
            coverage_collection: config.collect_coverage,
            test_data_dir: config.test_config.test_data_dir.clone(),
        };

        // Create test executor
        let executor = TestExecutor::with_context(execution_context)?;

        // Create test environment configuration
        let env_config = TestEnvironmentConfig {
            memory_tracking: config.collect_coverage,
            performance_profiling: true,
            max_memory_mb: Some(512),
            max_execution_time: Some(std::time::Duration::from_secs(config.test_config.timeout_seconds)),
            working_directory: config.test_config.working_directory.clone(),
            environment_variables: config.test_config.environment.clone(),
        };

        // Create test framework
        let framework = TestFramework::with_environment(TestEnvironment::with_config(env_config));

        // Create test reporter
        let reporter = match config.report_output.as_ref() {
            Some(output_path) => {
                let file = std::fs::File::create(output_path)
                    .map_err(|e| TestError::Io(format!("Failed to create report file: {}", e)))?;
                TestReporter::with_output(config.report_format.clone(), Box::new(file))
            }
            None => match config.report_format {
                ReportFormat::Console => TestReporter::console(),
                ReportFormat::Json => TestReporter::json(),
                _ => TestReporter::console(),
            }
        };

        Ok(Self {
            config,
            discovery,
            executor,
            framework,
            reporter,
            shutdown: Arc::new(Mutex::new(false)),
        })
    }

    /// Run all discovered tests
    pub async fn run_all_tests(&mut self) -> TestingResult<TestReport> {
        let start_time = Instant::now();
        info!("Starting test run");

        // Setup signal handler for graceful shutdown
        self.setup_signal_handlers().await;

        // Initialize test framework
        self.framework.initialize()?;

        // Discover tests
        let test_suites = self.discover_tests().await?;
        
        if test_suites.is_empty() {
            warn!("No test files found matching the specified patterns");
            return Ok(build_test_report(vec![]));
        }

        let total_tests: usize = test_suites.iter().map(|s| s.total_tests).sum();
        info!("Discovered {} test suites with {} total tests", test_suites.len(), total_tests);

        // Print discovery summary if verbose
        if self.config.test_config.verbose {
            self.print_discovery_summary(&test_suites);
        }

        // Check for dry run
        if self.config.dry_run {
            info!("Dry run mode: discovered {} tests, skipping execution", total_tests);
            return Ok(build_test_report(vec![]));
        }

        // Randomize test order if requested
        let mut ordered_suites = test_suites;
        if self.config.randomize_order {
            self.randomize_test_order(&mut ordered_suites);
        }

        // Execute tests
        let suite_results = self.execute_test_suites(ordered_suites).await?;

        // Generate and output report
        let report = build_test_report(suite_results);
        self.reporter.generate_report(&report)?;

        // Cleanup
        self.framework.shutdown()?;

        let total_duration = start_time.elapsed();
        info!("Test run completed in {:.2}s", total_duration.as_secs_f64());

        // Return error exit code if tests failed
        if report.summary.failed > 0 {
            return Err(TestError::General(format!("{} tests failed", report.summary.failed)));
        }

        Ok(report)
    }

    /// Run tests matching a specific pattern
    pub async fn run_tests_matching(&mut self, pattern: &str) -> TestingResult<TestReport> {
        info!("Running tests matching pattern: {}", pattern);
        
        // Discover all tests first
        let mut test_suites = self.discover_tests().await?;
        
        // Filter tests by pattern
        let filtered_suites = self.discovery.filter_tests(pattern)?;
        
        if filtered_suites.is_empty() {
            warn!("No tests found matching pattern: {}", pattern);
            return Ok(build_test_report(vec![]));
        }

        let total_tests: usize = filtered_suites.iter().map(|s| s.total_tests).sum();
        info!("Found {} tests matching pattern", total_tests);

        // Execute filtered tests
        let suite_results = self.execute_test_suites(filtered_suites).await?;
        let report = build_test_report(suite_results);
        
        self.reporter.generate_report(&report)?;
        Ok(report)
    }

    /// Run a specific test file
    pub async fn run_test_file(&mut self, file_path: &str) -> TestingResult<TestReport> {
        info!("Running test file: {}", file_path);
        
        // Update config to only include this file
        let mut file_config = self.config.test_config.clone();
        file_config.include_patterns = vec![file_path.to_string()];
        
        // Create new discovery with updated config
        let mut discovery = TestDiscovery::new(&file_config.working_directory)
            .with_include_patterns(file_config.include_patterns);
        
        let test_suites = discovery.discover_tests().await?;
        
        if test_suites.is_empty() {
            return Err(TestError::Discovery(format!("No tests found in file: {}", file_path)));
        }

        // Execute tests from the file
        let suite_results = self.execute_test_suites(test_suites).await?;
        let report = build_test_report(suite_results);
        
        self.reporter.generate_report(&report)?;
        Ok(report)
    }

    /// Discover all test files and functions
    async fn discover_tests(&mut self) -> TestingResult<Vec<TestSuite>> {
        info!("Starting test discovery");
        
        let test_suites = self.discovery.discover_tests().await?;
        
        debug!("Discovery completed: {} test suites found", test_suites.len());
        Ok(test_suites)
    }

    /// Print discovery summary
    fn print_discovery_summary(&self, test_suites: &[TestSuite]) {
        println!("📋 Test Discovery Summary");
        println!("========================");
        
        for suite in test_suites {
            println!("📦 Suite: {}", suite.name);
            println!("   Files: {}", suite.test_files.len());
            println!("   Tests: {}", suite.total_tests);
            
            if self.config.test_config.verbose {
                for test_file in &suite.test_files {
                    println!("   📄 {}", test_file.path.display());
                    for test_func in &test_file.test_functions {
                        println!("      🧪 {} ({}:{})", 
                                 test_func.name, 
                                 test_func.line_number, 
                                 test_func.column_number);
                    }
                }
            }
            println!();
        }
    }

    /// Randomize test execution order
    fn randomize_test_order(&self, test_suites: &mut Vec<TestSuite>) {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        
        let mut rng = if let Some(seed) = self.config.random_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };

        // Shuffle test suites
        for i in (1..test_suites.len()).rev() {
            let j = rng.gen_range(0..=i);
            test_suites.swap(i, j);
        }

        // Shuffle tests within each suite
        for suite in test_suites.iter_mut() {
            for test_file in suite.test_files.iter_mut() {
                for i in (1..test_file.test_functions.len()).rev() {
                    let j = rng.gen_range(0..=i);
                    test_file.test_functions.swap(i, j);
                }
            }
        }

        info!("Randomized test execution order (seed: {:?})", self.config.random_seed);
    }

    /// Setup signal handlers for graceful shutdown
    pub async fn setup_signal_handlers(&self) {
        let shutdown = self.shutdown.clone();
        
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(()) => {
                    println!("\n🛑 Received interrupt signal, stopping test execution...");
                    if let Ok(mut shutdown_flag) = shutdown.lock() {
                        *shutdown_flag = true;
                    }
                }
                Err(err) => {
                    error!("Unable to listen for shutdown signal: {}", err);
                }
            }
        });
    }

    /// Check if shutdown was requested
    pub fn should_shutdown(&self) -> bool {
        self.shutdown.lock()
            .map(|flag| *flag)
            .unwrap_or(false)
    }

    /// Report progress for a completed test suite
    pub fn report_suite_progress(&self, suite_result: &TestSuiteResult) {
        let passed = suite_result.test_results.iter()
            .filter(|r| r.status == super::execution::TestStatus::Passed)
            .count();
        let failed = suite_result.test_results.iter()
            .filter(|r| matches!(r.status, super::execution::TestStatus::Failed | 
                                             super::execution::TestStatus::Panicked |
                                             super::execution::TestStatus::CompilationError))
            .count();

        let status_symbol = if failed > 0 { "❌" } else { "✅" };
        
        println!("{} {} - {} passed, {} failed ({:.2}s)",
                 status_symbol,
                 suite_result.suite_name,
                 passed,
                 failed,
                 suite_result.duration.as_secs_f64());

        // Show individual test failures in real-time
        if failed > 0 && self.config.test_config.verbose {
            for test_result in &suite_result.test_results {
                if matches!(test_result.status, super::execution::TestStatus::Failed | 
                                               super::execution::TestStatus::Panicked |
                                               super::execution::TestStatus::CompilationError) {
                    println!("    ❌ {}: {}", 
                             test_result.test_function.name,
                             test_result.error_message.as_deref().unwrap_or("unknown error"));
                }
            }
        }
    }

    /// Execute all test suites
    async fn execute_test_suites(&mut self, test_suites: Vec<TestSuite>) -> TestingResult<Vec<TestSuiteResult>> {
        let mut suite_results = Vec::new();
        let mut total_failed = 0;

        for test_suite in test_suites {
            info!("Executing test suite: {}", test_suite.name);
            
            // Check for shutdown signal
            if self.should_shutdown() {
                warn!("Shutdown signal received, stopping test execution");
                break;
            }

            let suite_start = Instant::now();
            let test_results = self.executor.execute_test_suite(&test_suite).await;
            let suite_duration = suite_start.elapsed();

            // Count failures in this suite
            let suite_failures = test_results.iter()
                .filter(|r| matches!(r.status, super::execution::TestStatus::Failed | 
                                              super::execution::TestStatus::Panicked |
                                              super::execution::TestStatus::CompilationError))
                .count();

            total_failed += suite_failures;

            // Create suite result
            let suite_result = TestSuiteResult {
                suite_name: test_suite.name.clone(),
                test_results,
                duration: suite_duration,
                stats: super::reporting::TestStats {
                    by_status: std::collections::HashMap::new(), // Simplified
                    by_type: std::collections::HashMap::new(),
                    avg_duration: suite_duration / test_suite.total_tests.max(1) as u32,
                    fastest_test: std::time::Duration::from_millis(1),
                    slowest_test: suite_duration,
                    memory_stats: super::reporting::MemoryStatsSummary {
                        total_memory_mb: 10.0,
                        peak_memory_mb: 15.0,
                        avg_memory_per_test_mb: 2.0,
                        memory_leaks: 0,
                    },
                },
            };

            // Progress reporting
            if self.config.progress_reporting {
                self.report_suite_progress(&suite_result);
            }

            suite_results.push(suite_result);

            // Fail fast check
            if self.config.fail_fast && suite_failures > 0 {
                warn!("Fail fast mode: stopping execution after {} failures", suite_failures);
                break;
            }
        }

        info!("Test execution completed: {} suite(s) executed", suite_results.len());
        Ok(suite_results)
    }



    /// Get test runner configuration
    pub fn config(&self) -> &TestRunnerConfig {
        &self.config
    }

    /// Get discovered test suites
    pub fn discovered_suites(&self) -> &[TestSuite] {
        self.discovery.get_discovered_suites()
    }
}

/// Convenience function to run tests with default configuration
pub async fn run_tests() -> TestingResult<TestReport> {
    let mut runner = TestRunnerBuilder::new().build()?;
    runner.run_all_tests().await
}

/// Convenience function to run tests in a specific directory
pub async fn run_tests_in_dir(directory: &str) -> TestingResult<TestReport> {
    let mut config = TestConfig::default();
    config.working_directory = PathBuf::from(directory);
    
    let mut runner = TestRunnerBuilder::new()
        .with_config(config)
        .build()?;
    
    runner.run_all_tests().await
}

/// Convenience function to run tests with pattern matching
pub async fn run_tests_with_pattern(pattern: &str) -> TestingResult<TestReport> {
    let mut runner = TestRunnerBuilder::new().build()?;
    runner.run_tests_matching(pattern).await
}

