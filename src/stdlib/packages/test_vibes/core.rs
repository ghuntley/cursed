use crate::error::CursedError;
/// fr fr Core types for the TestVibes testing framework
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

/// fr fr The core squad for a single test
#[derive(Debug, Clone)]
pub struct VibeTest {
impl VibeTest {
    /// fr fr Create a new test with the given name
    pub fn new(name: String) -> Self {
        Self {
        }
    }

    /// fr fr CursedError methods
    pub fn error(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.errors.push(message);
        self.failed = true;
    pub fn errorf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.errors.push(message);
        self.failed = true;
    /// fr fr Failure methods
    pub fn fail(&mut self) {
        self.failed = true;
    pub fn fail_now(&mut self) {
        self.failed = true;
        // In a real implementation, this would exit the current test immediately
        panic!("Test failed immediately");
    pub fn failed(&self) -> bool {
        self.failed
    /// fr fr Fatal methods
    pub fn fatal(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.errors.push(message.clone());
        self.failed = true;
        panic!("Fatal test error: {}", message);
    pub fn fatalf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.errors.push(message.clone());
        self.failed = true;
        panic!("Fatal test error: {}", message);
    /// fr fr Helper method - marks the calling function as a test helper
    pub fn helper(&mut self) {
        self.helper = true;
    /// fr fr Logging methods
    pub fn log(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.logs.push(message);
    pub fn logf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.logs.push(message);
    /// fr fr Name method
    pub fn name(&self) -> &str {
        &self.name
    /// fr fr Parallel method - signals that test can be run in parallel
    pub fn parallel(&mut self) {
        self.parallel = true;
    /// fr fr Skip methods
    pub fn skip(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.logs.push(format!("SKIPPED: {}", message));
        self.skipped = true;
    pub fn skip_now(&mut self) {
        self.skipped = true;
        // In a real implementation, this would exit the current test immediately
    pub fn skipf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.logs.push(format!("SKIPPED: {}", message));
        self.skipped = true;
    pub fn skipped(&self) -> bool {
        self.skipped
    /// fr fr TempDir method - returns a temporary directory for the test
    pub fn temp_dir(&mut self) -> String {
        if self.temp_dir.is_none() {
            let temp_path = std::env::temp_dir().join(format!("cursed_test_{}", self.name));
            std::fs::create_dir_all(&temp_path).unwrap_or_default();
            self.temp_dir = Some(temp_path);
        self.temp_dir.as_ref().unwrap().to_string_lossy().to_string()
    /// fr fr Vibe methods - CURSED-specific test signaling
    pub fn pass_vibe(&mut self) {
        self.logs.push("✨ Test passed with good vibes!".to_string());
    pub fn fail_vibe(&mut self, message: &str) {
        self.errors.push(format!("💀 Bad vibes: {}", message));
        self.failed = true;
    /// fr fr Get test result for reporting
    pub fn get_result(&self) -> TestResult {
        let duration = self.start_time.map(|t| t.elapsed()).unwrap_or_default();
        
        TestResult {
        }
    }
/// fr fr The core squad for a single benchmark
#[derive(Debug, Clone)]
pub struct VibeBench {
    n: i64, // The number of iterations to run
    metrics: HashMap<String, (f64, String)>, // metric name -> (value, unit)
impl VibeBench {
    /// fr fr Create a new benchmark with the given name
    pub fn new(name: String) -> Self {
        Self {
        }
    }

    /// fr fr CursedError methods
    pub fn error(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.errors.push(message);
        self.failed = true;
    pub fn errorf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.errors.push(message);
        self.failed = true;
    /// fr fr Failure methods
    pub fn fail(&mut self) {
        self.failed = true;
    pub fn fail_now(&mut self) {
        self.failed = true;
        panic!("Benchmark failed immediately");
    pub fn failed(&self) -> bool {
        self.failed
    /// fr fr Fatal methods
    pub fn fatal(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.errors.push(message.clone());
        self.failed = true;
        panic!("Fatal benchmark error: {}", message);
    pub fn fatalf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.errors.push(message.clone());
        self.failed = true;
        panic!("Fatal benchmark error: {}", message);
    /// fr fr Helper method
    pub fn helper(&mut self) {
        // Mark as helper for call stack purposes
    /// fr fr Logging methods
    pub fn log(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.logs.push(message);
    pub fn logf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.logs.push(message);
    /// fr fr Name method
    pub fn name(&self) -> &str {
        &self.name
    /// fr fr Skip methods
    pub fn skip(&mut self, args: &[&str]) {
        let message = args.join(" ");
        self.logs.push(format!("SKIPPED: {}", message));
        self.skipped = true;
    pub fn skip_now(&mut self) {
        self.skipped = true;
    pub fn skipf(&mut self, format: &str, args: &[&str]) {
        let message = format_string(format, args);
        self.logs.push(format!("SKIPPED: {}", message));
        self.skipped = true;
    pub fn skipped(&self) -> bool {
        self.skipped
    /// fr fr Timer methods
    pub fn reset_timer(&mut self) {
        self.timer_start = Some(Instant::now());
        self.timer_running = true;
    pub fn start_timer(&mut self) {
        if self.timer_start.is_none() {
            self.timer_start = Some(Instant::now());
        }
        self.timer_running = true;
    pub fn stop_timer(&mut self) {
        self.timer_running = false;
    /// fr fr Metrics methods
    pub fn report_metric(&mut self, n: f64, unit: &str) {
        // Use a default metric name if not specified
        self.metrics.insert("custom".to_string(), (n, unit.to_string()));
    pub fn set_bytes(&mut self, n: i64) {
        self.bytes = Some(n);
    pub fn set_parallelism(&mut self, p: i32) {
        self.parallelism = Some(p);
    /// fr fr Get benchmark iterations
    pub fn iterations(&self) -> i64 {
        self.n
    pub fn set_iterations(&mut self, n: i64) {
        self.n = n;
    /// fr fr Get benchmark result for reporting
    pub fn get_result(&self) -> BenchResult {
        let duration = if let Some(start) = self.timer_start {
            if self.timer_running {
                start.elapsed()
            } else {
                Duration::default()
            }
        } else {
            Duration::default()

        BenchResult {
        }
    }
/// fr fr The main testing manager
pub struct VibeTestingManager {
impl VibeTestingManager {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn add_test<F>(&mut self, test_fn: F) 
    where 
        F: Fn(&mut VibeTest) + Send + Sync + 'static 
    {
        self.tests.push(Box::new(test_fn));
    pub fn add_benchmark<F>(&mut self, bench_fn: F)
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static
    {
        self.benchmarks.push(Box::new(bench_fn));
    /// fr fr Run all tests and benchmarks
    pub fn run(&mut self) -> i32 {
        let mut total_tests = 0;
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut skipped_tests = 0;

        // Run tests
        for (i, test_fn) in self.tests.iter().enumerate() {
            let mut test = VibeTest::new(format!("test_{}", i));
            test_fn(&mut test);
            
            total_tests += 1;
            if test.skipped() {
                skipped_tests += 1;
            } else if test.failed() {
                failed_tests += 1;
            } else {
                passed_tests += 1;
            let result = test.get_result();
            self.print_test_result(&result);
        // Run benchmarks
        for (i, bench_fn) in self.benchmarks.iter().enumerate() {
            let mut bench = VibeBench::new(format!("bench_{}", i));
            bench.set_iterations(self.config.benchmark_iterations);
            bench.start_timer();
            
            bench_fn(&mut bench);
            
            bench.stop_timer();
            let result = bench.get_result();
            self.print_bench_result(&result);
        // Print summary
        println!("\n🧪 Test Summary:");
        println!("  Total: {}", total_tests);
        println!("  Passed: {}", passed_tests);
        println!("  Failed: {}", failed_tests);
        println!("  Skipped: {}", skipped_tests);

        if failed_tests > 0 { 1 } else { 0 }
    }

    fn print_test_result(&self, result: &TestResult) {
        let status = if result.skipped {
            "SKIP"
        } else if result.failed {
            "FAIL"
        } else {
            "PASS"

        println!("{} {} ({:.2?})", status, result.name, result.duration);
        
        for error in &result.errors {
            println!("    ERROR: {}", error);
        if self.config.verbose {
            for log in &result.logs {
                println!("    LOG: {}", log);
            }
        }
    fn print_bench_result(&self, result: &BenchResult) {
        let ns_per_op = if result.iterations > 0 {
            result.duration.as_nanos() as f64 / result.iterations as f64
        } else {
            0.0

        println!("BENCH {} {} iterations {:.2} ns/op", 
                 result.name, result.iterations, ns_per_op);

        if let Some(bytes) = result.bytes_per_op {
            let mb_per_sec = if ns_per_op > 0.0 {
                (bytes as f64 * 1000.0) / ns_per_op
            } else {
                0.0
            println!("    {:.2} MB/s", mb_per_sec);
        for (name, (value, unit)) in &result.metrics {
            println!("    {}: {:.2} {}", name, value, unit);
        }
    }
impl Default for VibeTestingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
impl Default for TestConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr Test result for reporting
#[derive(Debug, Clone)]
pub struct TestResult {
impl TestResult {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn add_test_result(&mut self, result: TestResult) {
        // Aggregate results from multiple tests
        if result.failed {
            self.failed = true;
        }
        if result.skipped && !self.failed {
            self.skipped = true;
        }
        if !result.failed && !result.skipped {
            self.passed = true;
        self.duration += result.duration;
        self.errors.extend(result.errors);
        self.logs.extend(result.logs);
    }
}

impl Default for TestResult {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Benchmark result for reporting
#[derive(Debug, Clone)]
pub struct BenchResult {
/// fr fr Helper function for string formatting
fn format_string(format: &str, args: &[&str]) -> String {
    let mut result = format.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }
    result
