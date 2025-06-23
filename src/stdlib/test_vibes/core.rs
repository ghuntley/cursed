/// Core types for the TestVibes framework
/// 
/// This module implements the fundamental testing types:
/// - VibeTest: Main test execution context
/// - VibeBench: Benchmark execution context  
/// - VibeTestingManager: Test suite manager

use crate::stdlib::value::Value;
use crate::crate::stdlib::errors_simple::CursedError;
use super::{TestVibesResult, TestVibesError, test_failed, test_skipped};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::path::PathBuf;

/// Test execution state
#[derive(Debug, Clone, PartialEq)]
pub enum VibeTestState {
    Created,
    Running,
    Passed,
    Failed,
    Skipped,
    Panicked,
}

/// Benchmark execution state
#[derive(Debug, Clone, PartialEq)]
pub enum VibeBenchState {
    Created,
    Running,
    Completed,
    Failed,
}

/// Test result information
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub state: VibeTestState,
    pub message: Option<String>,
    pub duration: Duration,
    pub memory_used: Option<usize>,
}

/// Benchmark result information
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub state: VibeBenchState,
    pub iterations: usize,
    pub duration: Duration,
    pub ns_per_op: f64,
    pub bytes_per_op: Option<usize>,
    pub allocations: Option<usize>,
    pub custom_metrics: HashMap<String, f64>,
}

/// The core squad for a single test
#[derive(Debug)]
pub struct VibeTest {
    pub name: String,
    state: Arc<Mutex<VibeTestState>>,
    failed: Arc<Mutex<bool>>,
    skipped: Arc<Mutex<bool>>,
    logs: Arc<Mutex<Vec<String>>>,
    start_time: Instant,
    is_parallel: Arc<Mutex<bool>>,
    helpers: Arc<Mutex<Vec<String>>>,
    temp_dir: Arc<Mutex<Option<PathBuf>>>,
}

impl VibeTest {
    /// Create a new test instance
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Arc::new(Mutex::new(VibeTestState::Created)),
            failed: Arc::new(Mutex::new(false)),
            skipped: Arc::new(Mutex::new(false)),
            logs: Arc::new(Mutex::new(Vec::new())),
            start_time: Instant::now(),
            is_parallel: Arc::new(Mutex::new(false)),
            helpers: Arc::new(Mutex::new(Vec::new())),
            temp_dir: Arc::new(Mutex::new(None)),
        }
    }

    /// Print error message and mark test as failed
    pub fn Error(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.Log(&[Value::String(format!("ERROR: {}", message))])?;
        self.Fail()
    }

    /// Print formatted error message and mark test as failed
    pub fn Errorf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Error(&[Value::String(message)])
    }

    /// Mark test as failed
    pub fn Fail(&self) -> TestVibesResult<()> {
        *self.failed.lock().unwrap() = true;
        *self.state.lock().unwrap() = VibeTestState::Failed;
        Ok(())
    }

    /// Mark test as failed and stop execution immediately
    pub fn FailNow(&self) -> TestVibesResult<()> {
        self.Fail()?;
        Err(test_failed("Test failed and stopped").into())
    }

    /// Check if test has failed
    pub fn Failed(&self) -> bool {
        *self.failed.lock().unwrap()
    }

    /// Print fatal error message and stop execution
    pub fn Fatal(&self, args: &[Value]) -> TestVibesResult<()> {
        self.Error(args)?;
        self.FailNow()
    }

    /// Print formatted fatal error message and stop execution
    pub fn Fatalf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Fatal(&[Value::String(message)])
    }

    /// Mark calling function as a test helper
    pub fn Helper(&self) -> TestVibesResult<()> {
        // In a real implementation, this would use stack inspection
        // For now, we'll record a generic helper marker
        self.helpers.lock().unwrap().push("helper".to_string());
        Ok(())
    }

    /// Log a message
    pub fn Log(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.logs.lock().unwrap().push(message);
        Ok(())
    }

    /// Log a formatted message
    pub fn Logf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Log(&[Value::String(message)])
    }

    /// Get test name
    pub fn Name(&self) -> String {
        self.name.clone()
    }

    /// Signal that test can be run in parallel
    pub fn Parallel(&self) -> TestVibesResult<()> {
        *self.is_parallel.lock().unwrap() = true;
        Ok(())
    }

    /// Skip test with message
    pub fn Skip(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.Log(&[Value::String(format!("SKIP: {}", message))])?;
        self.SkipNow()
    }

    /// Skip test immediately
    pub fn SkipNow(&self) -> TestVibesResult<()> {
        *self.skipped.lock().unwrap() = true;
        *self.state.lock().unwrap() = VibeTestState::Skipped;
        Err(test_skipped("Test skipped").into())
    }

    /// Skip test with formatted message
    pub fn Skipf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Skip(&[Value::String(message)])
    }

    /// Check if test was skipped
    pub fn Skipped(&self) -> bool {
        *self.skipped.lock().unwrap()
    }

    /// Get temporary directory for test
    pub fn TempDir(&self) -> TestVibesResult<String> {
        let mut temp_dir = self.temp_dir.lock().unwrap();
        if temp_dir.is_none() {
            let dir = std::env::temp_dir().join(format!("test_{}", self.name));
            std::fs::create_dir_all(&dir).map_err(|e| {
                CursedError::Runtime(format!("Failed to create temp dir: {}", e))
            })?;
            *temp_dir = Some(dir);
        }
        Ok(temp_dir.as_ref().unwrap().to_string_lossy().to_string())
    }

    /// Signal test passed with good vibes
    pub fn PassVibe(&self) -> TestVibesResult<()> {
        *self.state.lock().unwrap() = VibeTestState::Passed;
        self.Log(&[Value::String("✨ Test passed with good vibes! ✨".to_string())])
    }

    /// Signal test failed with bad vibes
    pub fn FailVibe(&self, message: &str) -> TestVibesResult<()> {
        self.Log(&[Value::String(format!("💀 Bad vibes: {} 💀", message))])?;
        self.Fail()
    }

    /// Get current test state
    pub fn get_state(&self) -> VibeTestState {
        self.state.lock().unwrap().clone()
    }

    /// Get test logs
    pub fn get_logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }

    /// Check if test is set to run in parallel
    pub fn is_parallel(&self) -> bool {
        *self.is_parallel.lock().unwrap()
    }

    /// Get test duration
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// The core squad for a single benchmark
#[derive(Debug)]
pub struct VibeBench {
    pub name: String,
    pub N: usize, // Number of iterations
    state: Arc<Mutex<VibeBenchState>>,
    failed: Arc<Mutex<bool>>,
    skipped: Arc<Mutex<bool>>,
    logs: Arc<Mutex<Vec<String>>>,
    start_time: Option<Instant>,
    timer_start: Option<Instant>,
    total_duration: Duration,
    bytes_per_op: Option<usize>,
    custom_metrics: Arc<Mutex<HashMap<String, f64>>>,
    parallelism: usize,
}

impl VibeBench {
    /// Create a new benchmark instance
    pub fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            N: iterations,
            state: Arc::new(Mutex::new(VibeBenchState::Created)),
            failed: Arc::new(Mutex::new(false)),
            skipped: Arc::new(Mutex::new(false)),
            logs: Arc::new(Mutex::new(Vec::new())),
            start_time: None,
            timer_start: None,
            total_duration: Duration::new(0, 0),
            bytes_per_op: None,
            custom_metrics: Arc::new(Mutex::new(HashMap::new())),
            parallelism: 1,
        }
    }

    /// Print error message and mark benchmark as failed
    pub fn Error(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.Log(&[Value::String(format!("ERROR: {}", message))])?;
        self.Fail()
    }

    /// Print formatted error message and mark benchmark as failed
    pub fn Errorf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Error(&[Value::String(message)])
    }

    /// Mark benchmark as failed
    pub fn Fail(&self) -> TestVibesResult<()> {
        *self.failed.lock().unwrap() = true;
        *self.state.lock().unwrap() = VibeBenchState::Failed;
        Ok(())
    }

    /// Mark benchmark as failed and stop execution
    pub fn FailNow(&self) -> TestVibesResult<()> {
        self.Fail()?;
        Err(CursedError::Runtime("Benchmark failed and stopped".to_string()))
    }

    /// Check if benchmark has failed
    pub fn Failed(&self) -> bool {
        *self.failed.lock().unwrap()
    }

    /// Print fatal error and stop execution
    pub fn Fatal(&self, args: &[Value]) -> TestVibesResult<()> {
        self.Error(args)?;
        self.FailNow()
    }

    /// Print formatted fatal error and stop execution
    pub fn Fatalf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Fatal(&[Value::String(message)])
    }

    /// Mark calling function as helper
    pub fn Helper(&self) -> TestVibesResult<()> {
        // Benchmark helpers are noted but don't affect execution
        Ok(())
    }

    /// Log a message
    pub fn Log(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.logs.lock().unwrap().push(message);
        Ok(())
    }

    /// Log a formatted message
    pub fn Logf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Log(&[Value::String(message)])
    }

    /// Get benchmark name
    pub fn Name(&self) -> String {
        self.name.clone()
    }

    /// Skip benchmark with message
    pub fn Skip(&self, args: &[Value]) -> TestVibesResult<()> {
        let message = format_args_to_string(args);
        self.Log(&[Value::String(format!("SKIP: {}", message))])?;
        self.SkipNow()
    }

    /// Skip benchmark immediately
    pub fn SkipNow(&self) -> TestVibesResult<()> {
        *self.skipped.lock().unwrap() = true;
        Err(test_skipped("Benchmark skipped").into())
    }

    /// Skip benchmark with formatted message
    pub fn Skipf(&self, format: &str, args: &[Value]) -> TestVibesResult<()> {
        let message = format_string(format, args)?;
        self.Skip(&[Value::String(message)])
    }

    /// Check if benchmark was skipped
    pub fn Skipped(&self) -> bool {
        *self.skipped.lock().unwrap()
    }

    /// Reset the benchmark timer
    pub fn ResetTimer(&mut self) -> TestVibesResult<()> {
        self.start_time = Some(Instant::now());
        self.timer_start = Some(Instant::now());
        self.total_duration = Duration::new(0, 0);
        Ok(())
    }

    /// Start the benchmark timer
    pub fn StartTimer(&mut self) -> TestVibesResult<()> {
        self.timer_start = Some(Instant::now());
        Ok(())
    }

    /// Stop the benchmark timer
    pub fn StopTimer(&mut self) -> TestVibesResult<()> {
        if let Some(start) = self.timer_start.take() {
            self.total_duration += start.elapsed();
        }
        Ok(())
    }

    /// Report custom metric
    pub fn ReportMetric(&self, n: f64, unit: &str) -> TestVibesResult<()> {
        self.custom_metrics.lock().unwrap().insert(unit.to_string(), n);
        Ok(())
    }

    /// Set number of bytes processed per operation
    pub fn SetBytes(&mut self, n: i64) -> TestVibesResult<()> {
        if n >= 0 {
            self.bytes_per_op = Some(n as usize);
        }
        Ok(())
    }

    /// Set parallelism for benchmark
    pub fn SetParallelism(&mut self, p: usize) -> TestVibesResult<()> {
        self.parallelism = p;
        Ok(())
    }

    /// Get benchmark result
    pub fn result(&self) -> BenchmarkResult {
        let ns_per_op = if self.N > 0 {
            self.total_duration.as_nanos() as f64 / self.N as f64
        } else {
            0.0
        };

        BenchmarkResult {
            name: self.name.clone(),
            state: self.state.lock().unwrap().clone(),
            iterations: self.N,
            duration: self.total_duration,
            ns_per_op,
            bytes_per_op: self.bytes_per_op,
            allocations: None, // Would need runtime integration to track
            custom_metrics: self.custom_metrics.lock().unwrap().clone(),
        }
    }
}

/// Test suite manager
#[derive(Debug)]
pub struct VibeTestingManager {
    tests: Vec<Box<dyn Fn(&mut VibeTest) -> TestVibesResult<()> + Send + Sync>>,
    benchmarks: Vec<Box<dyn Fn(&mut VibeBench) -> TestVibesResult<()> + Send + Sync>>,
    setup: Option<Box<dyn Fn() -> TestVibesResult<()> + Send + Sync>>,
    teardown: Option<Box<dyn Fn() -> TestVibesResult<()> + Send + Sync>>,
}

impl VibeTestingManager {
    /// Create new test manager
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            benchmarks: Vec::new(),
            setup: None,
            teardown: None,
        }
    }

    /// Run all tests and benchmarks
    pub fn Run(&self) -> i32 {
        let mut exit_code = 0;

        // Run setup if provided
        if let Some(ref setup) = self.setup {
            if let Err(e) = setup() {
                eprintln!("Setup failed: {}", e);
                return 1;
            }
        }

        // Run tests
        for (i, test_fn) in self.tests.iter().enumerate() {
            let mut test = VibeTest::new(&format!("test_{}", i));
            match test_fn(&mut test) {
                Ok(_) => {
                    if !test.Failed() && !test.Skipped() {
                        println!("✅ {}: PASS", test.Name());
                    } else if test.Skipped() {
                        println!("⏭️  {}: SKIP", test.Name());
                    } else {
                        println!("❌ {}: FAIL", test.Name());
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    println!("❌ {}: FAIL - {}", test.Name(), e);
                    exit_code = 1;
                }
            }

            // Print logs if any
            for log in test.get_logs() {
                println!("    {}", log);
            }
        }

        // Run benchmarks
        for (i, bench_fn) in self.benchmarks.iter().enumerate() {
            let mut bench = VibeBench::new(&format!("benchmark_{}", i), 1000);
            match bench_fn(&mut bench) {
                Ok(_) => {
                    let result = bench.result();
                    println!("🏃 {}: {} iterations, {:.2} ns/op", 
                             result.name, result.iterations, result.ns_per_op);
                }
                Err(e) => {
                    println!("❌ {}: FAIL - {}", bench.Name(), e);
                    exit_code = 1;
                }
            }
        }

        // Run teardown if provided
        if let Some(ref teardown) = self.teardown {
            if let Err(e) = teardown() {
                eprintln!("Teardown failed: {}", e);
                exit_code = 1;
            }
        }

        exit_code
    }
}

/// Entry point for test packages
pub type TestMain = fn(&mut VibeTestingManager);

// Helper functions

/// Format arguments to string
fn format_args_to_string(args: &[Value]) -> String {
    args.iter()
        .map(|arg| format!("{}", value_to_string(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format string with placeholder substitution
fn format_string(format: &str, args: &[Value]) -> TestVibesResult<String> {
    let mut result = format.to_string();
    let mut arg_index = 0;

    // Simple placeholder replacement - look for {} patterns
    while let Some(start) = result.find('{') {
        if let Some(end) = result[start..].find('}') {
            let end = start + end;
            let placeholder = &result[start..=end];
            
            if placeholder == "{}" {
                // Simple positional placeholder
                if arg_index < args.len() {
                    let replacement = value_to_string(&args[arg_index]);
                    result.replace_range(start..=end, &replacement);
                    arg_index += 1;
                } else {
                    return Err(CursedError::Runtime("Not enough arguments for format string".to_string()));
                }
            } else if placeholder.len() > 2 {
                // Check for indexed placeholder like {0}, {1}, etc.
                let index_str = &placeholder[1..placeholder.len()-1];
                if let Ok(index) = index_str.parse::<usize>() {
                    if index < args.len() {
                        let replacement = value_to_string(&args[index]);
                        result.replace_range(start..=end, &replacement);
                    } else {
                        return Err(CursedError::Runtime(format!("Argument index {} out of bounds", index)));
                    }
                } else {
                    // Unknown placeholder, skip
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(result)
}

/// Convert Value to string representation
fn value_to_string(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}
