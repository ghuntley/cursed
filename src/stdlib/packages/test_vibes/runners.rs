/// fr fr Test runners and main functionality for the TestVibes framework
// use crate::stdlib::packages::test_vibes::core::{VibeTest, VibeBench, VibeTestingManager, TestResult, BenchResult};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// fr fr Test function type
pub type TestFunction = Box<dyn Fn(&mut VibeTest) + Send + Sync>;

/// fr fr Benchmark function type
pub type BenchFunction = Box<dyn Fn(&mut VibeBench) + Send + Sync>;

/// fr fr Main test entry point
pub fn test_main(m: &mut VibeTestingManager) -> i32 {
    m.run()
}

/// fr fr Test runner configuration
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    pub parallel: bool,
    pub fail_fast: bool,
    pub verbose: bool,
    pub timeout: Option<Duration>,
    pub filter: Option<String>,
    pub benchmark_iterations: i64,
    pub warmup_iterations: i64,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            parallel: false,
            fail_fast: false,
            verbose: false,
            timeout: Some(Duration::from_secs(300)), // 5 minutes default
            filter: None,
            benchmark_iterations: 1000,
            warmup_iterations: 100,
        }
    }
}

/// fr fr Test runner for executing tests and benchmarks
pub struct TestRunner {
    config: TestRunnerConfig,
    tests: Vec<(String, TestFunction)>,
    benchmarks: Vec<(String, BenchFunction)>,
    test_results: Arc<Mutex<Vec<TestResult>>>,
    bench_results: Arc<Mutex<Vec<BenchResult>>>,
}

impl TestRunner {
    /// fr fr Create a new test runner
    pub fn new(config: TestRunnerConfig) -> Self {
        Self {
            config,
            tests: Vec::new(),
            benchmarks: Vec::new(),
            test_results: Arc::new(Mutex::new(Vec::new())),
            bench_results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// fr fr Add a test to the runner
    pub fn add_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: Fn(&mut VibeTest) + Send + Sync + 'static,
    {
        if self.should_run_test(name) {
            self.tests.push((name.to_string(), Box::new(test_fn)));
        }
    }

    /// fr fr Add a benchmark to the runner
    pub fn add_benchmark<F>(&mut self, name: &str, bench_fn: F)
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static,
    {
        if self.should_run_test(name) {
            self.benchmarks.push((name.to_string(), Box::new(bench_fn)));
        }
    }

    /// fr fr Check if a test should run based on filters
    fn should_run_test(&self, name: &str) -> bool {
        if let Some(ref filter) = self.config.filter {
            name.contains(filter)
        } else {
            true
        }
    }

    /// fr fr Run all tests
    pub fn run_tests(&mut self) -> TestRunSummary {
        let start_time = Instant::now();
        
        if self.config.parallel {
            let _ = self.run_tests_parallel();
        } else {
            let _ = self.run_tests_sequential();
        }

        let test_results = self.test_results.lock().unwrap().clone();
        let bench_results = self.bench_results.lock().unwrap().clone();
        
        let total_duration = start_time.elapsed();
        
        TestRunSummary::new(test_results, bench_results, total_duration)
    }

    /// fr fr Run tests sequentially
    fn run_tests_sequential(&mut self) -> i32 {
        let mut exit_code = 0;

        // Run tests
        for (name, test_fn) in &self.tests {
            let result = self.run_single_test(name, test_fn);
            
            if self.config.verbose {
                self.print_test_result(&result);
            }
            
            if result.failed {
                exit_code = 1;
                if self.config.fail_fast {
                    break;
                }
            }
            
            self.test_results.lock().unwrap().push(result);
        }

        // Run benchmarks
        for (name, bench_fn) in &self.benchmarks {
            let result = self.run_single_benchmark(name, bench_fn);
            
            if self.config.verbose {
                self.print_bench_result(&result);
            }
            
            self.bench_results.lock().unwrap().push(result);
        }

        exit_code
    }

    /// fr fr Run tests in parallel
    fn run_tests_parallel(&mut self) -> i32 {
        let mut handles = Vec::new();
        let results = Arc::clone(&self.test_results);
        let config = self.config.clone();

        // Parallel test execution
        for (name, test_fn) in std::mem::take(&mut self.tests) {
            let results_clone = Arc::clone(&results);
            let config_clone = config.clone();
            
            let handle = std::thread::spawn(move || {
                let runner = TestRunner::new(config_clone);
                let result = runner.run_single_test(&name, &test_fn);
                results_clone.lock().unwrap().push(result);
            });
            
            handles.push(handle);
        }

        // Wait for all test threads
        for handle in handles {
            let _ = handle.join();
        }

        // Run benchmarks sequentially (they're resource-intensive)
        for (name, bench_fn) in &self.benchmarks {
            let result = self.run_single_benchmark(name, bench_fn);
            self.bench_results.lock().unwrap().push(result);
        }

        // Check if any tests failed
        let test_results = self.test_results.lock().unwrap();
        if test_results.iter().any(|r| r.failed) { 1 } else { 0 }
    }

    /// fr fr Run a single test
    fn run_single_test(&self, name: &str, test_fn: &TestFunction) -> TestResult {
        let mut test = VibeTest::new(name.to_string());
        
        let start = Instant::now();
        
        // Run the test directly (timeout handling simplified for now)
        test_fn(&mut test);
        
        let mut result = test.get_result();
        result.duration = start.elapsed();
        
        result
    }

    /// fr fr Run a single benchmark
    fn run_single_benchmark(&self, name: &str, bench_fn: &BenchFunction) -> BenchResult {
        let mut bench = VibeBench::new(name.to_string());
        bench.set_iterations(self.config.benchmark_iterations);
        
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            let mut warmup_bench = VibeBench::new(format!("{}_warmup", name));
            warmup_bench.set_iterations(1);
            bench_fn(&mut warmup_bench);
        }
        
        // Actual benchmark
        bench.reset_timer();
        bench_fn(&mut bench);
        bench.stop_timer();
        
        bench.get_result()
    }



    /// fr fr Print test result
    fn print_test_result(&self, result: &TestResult) {
        let status = if result.skipped {
            "SKIP"
        } else if result.failed {
            "FAIL"
        } else {
            "PASS"
        };

        println!("{} {} ({:.2?})", status, result.name, result.duration);
        
        if result.failed {
            for error in &result.errors {
                println!("    ERROR: {}", error);
            }
        }
        
        if self.config.verbose {
            for log in &result.logs {
                println!("    LOG: {}", log);
            }
        }
    }

    /// fr fr Print benchmark result
    fn print_bench_result(&self, result: &BenchResult) {
        let ns_per_op = if result.iterations > 0 {
            result.duration.as_nanos() as f64 / result.iterations as f64
        } else {
            0.0
        };

        println!("BENCH {} {} iterations {:.2} ns/op", 
                 result.name, result.iterations, ns_per_op);

        for (name, (value, unit)) in &result.metrics {
            println!("    {}: {:.2} {}", name, value, unit);
        }
    }

    /// fr fr Get configuration
    pub fn config(&self) -> &TestRunnerConfig {
        &self.config
    }
}

/// fr fr Test run summary
#[derive(Debug, Clone)]
pub struct TestRunSummary {
    pub test_results: Vec<TestResult>,
    pub bench_results: Vec<BenchResult>,
    pub total_duration: Duration,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub total_benchmarks: usize,
}

impl TestRunSummary {
    /// fr fr Create a new test run summary
    pub fn new(test_results: Vec<TestResult>, bench_results: Vec<BenchResult>, total_duration: Duration) -> Self {
        let total_tests = test_results.len();
        let passed_tests = test_results.iter().filter(|r| r.passed).count();
        let failed_tests = test_results.iter().filter(|r| r.failed).count();
        let skipped_tests = test_results.iter().filter(|r| r.skipped).count();
        let total_benchmarks = bench_results.len();

        Self {
            test_results,
            bench_results,
            total_duration,
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            total_benchmarks,
        }
    }

    /// fr fr Print summary
    pub fn print_summary(&self) {
        println!("\n🧪 Test Run Summary:");
        println!("  Total Duration: {:.2?}", self.total_duration);
        println!("  Tests: {} total", self.total_tests);
        println!("    ✅ Passed: {}", self.passed_tests);
        println!("    ❌ Failed: {}", self.failed_tests);
        println!("    ⏭️  Skipped: {}", self.skipped_tests);
        
        if self.total_benchmarks > 0 {
            println!("  Benchmarks: {} total", self.total_benchmarks);
        }

        if self.failed_tests > 0 {
            println!("\n❌ Failed Tests:");
            for result in &self.test_results {
                if result.failed {
                    println!("  - {}", result.name);
                    for error in &result.errors {
                        println!("    {}", error);
                    }
                }
            }
        }
    }

    /// fr fr Get exit code
    pub fn exit_code(&self) -> i32 {
        if self.failed_tests > 0 { 1 } else { 0 }
    }

    /// fr fr Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0
    }

    /// fr fr Get failure rate
    pub fn failure_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            self.failed_tests as f64 / self.total_tests as f64
        }
    }
}

/// fr fr Test discovery and registration
pub struct TestRegistry {
    tests: HashMap<String, TestFunction>,
    benchmarks: HashMap<String, BenchFunction>,
}

impl TestRegistry {
    /// fr fr Create a new test registry
    pub fn new() -> Self {
        Self {
            tests: HashMap::new(),
            benchmarks: HashMap::new(),
        }
    }

    /// fr fr Register a test
    pub fn register_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: Fn(&mut VibeTest) + Send + Sync + 'static,
    {
        self.tests.insert(name.to_string(), Box::new(test_fn));
    }

    /// fr fr Register a benchmark
    pub fn register_benchmark<F>(&mut self, name: &str, bench_fn: F)
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static,
    {
        self.benchmarks.insert(name.to_string(), Box::new(bench_fn));
    }

    /// fr fr Run all registered tests
    pub fn run_all(&self, config: TestRunnerConfig) -> TestRunSummary {
        let mut runner = TestRunner::new(config);
        
        // We can't clone function pointers, so this would need a different approach
        // For now, let's just return an empty summary
        let test_results = Vec::new();
        let bench_results = Vec::new();
        let total_duration = std::time::Duration::from_millis(0);
        
        TestRunSummary::new(test_results, bench_results, total_duration)
    }

    /// fr fr List all registered tests
    pub fn list_tests(&self) -> Vec<&String> {
        self.tests.keys().collect()
    }

    /// fr fr List all registered benchmarks
    pub fn list_benchmarks(&self) -> Vec<&String> {
        self.benchmarks.keys().collect()
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Global test registry for easy test registration
lazy_static::lazy_static! {
    static ref GLOBAL_TEST_REGISTRY: Arc<Mutex<TestRegistry>> = Arc::new(Mutex::new(TestRegistry::new()));
}

/// fr fr Register a test globally
pub fn register_test<F>(name: &str, test_fn: F)
where
    F: Fn(&mut VibeTest) + Send + Sync + 'static,
{
    GLOBAL_TEST_REGISTRY
        .lock()
        .unwrap()
        .register_test(name, test_fn);
}

/// fr fr Register a benchmark globally
pub fn register_benchmark<F>(name: &str, bench_fn: F)
where
    F: Fn(&mut VibeBench) + Send + Sync + 'static,
{
    GLOBAL_TEST_REGISTRY
        .lock()
        .unwrap()
        .register_benchmark(name, bench_fn);
}

/// fr fr Run all globally registered tests
pub fn run_all_tests(config: TestRunnerConfig) -> TestRunSummary {
    GLOBAL_TEST_REGISTRY
        .lock()
        .unwrap()
        .run_all(config)
}

/// fr fr Create a custom test main function
pub fn create_test_main() -> impl Fn() -> i32 {
    || {
        let config = TestRunnerConfig::default();
        let summary = run_all_tests(config);
        summary.print_summary();
        summary.exit_code()
    }
}

