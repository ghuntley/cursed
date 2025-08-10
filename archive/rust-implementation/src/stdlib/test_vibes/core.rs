//! Testing functionality for core

use crate::error::CursedError;
use std::time::{Duration, Instant};

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Result type for benchmark operations
pub type BenchmarkResult = Result<Duration, CursedError>;

/// Test state for CURSED tests
#[derive(Debug, Clone)]
pub struct VibeTestState {
    pub name: String,
    pub passed: bool,
    pub failed: bool,
    pub skipped: bool,
    pub error_message: Option<String>,
}

/// Benchmark state for CURSED benchmarks
#[derive(Debug, Clone)]
pub struct VibeBenchState {
    pub name: String,
    pub iterations: u64,
    pub duration: Duration,
    pub bytes_per_second: Option<u64>,
}

/// A CURSED test instance
#[derive(Debug)]
pub struct VibeTest {
    pub name: String,
    pub state: VibeTestState,
    pub start_time: Option<Instant>,
    pub verbose: bool,
}

impl VibeTest {
    /// Create a new test
    pub fn new(name: String) -> Self {
        Self {
            state: VibeTestState {
                name: name.clone(),
                passed: false,
                failed: false,
                skipped: false,
                error_message: None,
            },
            name,
            start_time: None,
            verbose: false,
        }
    }
    
    /// Mark the test as failed
    pub fn fail(&mut self, message: &str) {
        self.state.failed = true;
        self.state.error_message = Some(message.to_string());
    }
    
    /// Mark the test as passed
    pub fn pass(&mut self) {
        self.state.passed = true;
        self.state.failed = false;
    }
    
    /// Skip the test
    pub fn skip(&mut self, message: &str) {
        self.state.skipped = true;
        self.state.error_message = Some(message.to_string());
    }
    
    /// Log a message
    pub fn log(&self, message: &str) {
        if self.verbose {
            println!("[{}] {}", self.name, message);
        }
    }
    
    /// Start timing
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    
    /// Get elapsed time
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }
    
    /// Log multiple messages
    pub fn log_multiple(&self, messages: &[&str]) {
        if self.verbose {
            for message in messages {
                println!("[{}] {}", self.name, message);
            }
        }
    }
    
    /// Mark the test as passed (alias)
    pub fn pass_vibe(&mut self) {
        self.pass();
    }
    
    /// Mark the test as failed (alias)
    pub fn fail_vibe(&mut self, message: &str) {
        self.fail(message);
    }
    
    /// Get test result
    pub fn get_result(&self) -> TestResult<()> {
        if self.state.failed {
            Err(CursedError::runtime_error(
                &self.state.error_message.clone()
                    .unwrap_or_else(|| "Test failed".to_string())
            ))
        } else if self.state.passed {
            Ok(())
        } else {
            Err(CursedError::runtime_error("Test not completed"))
        }
    }
}

/// A CURSED benchmark instance
#[derive(Debug)]
pub struct VibeBench {
    pub name: String,
    pub state: VibeBenchState,
    pub start_time: Option<Instant>,
    pub verbose: bool,
}

impl VibeBench {
    /// Create a new benchmark
    pub fn new(name: String) -> Self {
        Self {
            state: VibeBenchState {
                name: name.clone(),
                iterations: 0,
                duration: Duration::new(0, 0),
                bytes_per_second: None,
            },
            name,
            start_time: None,
            verbose: false,
        }
    }
    
    /// Start timing
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    
    /// Stop timing and record result
    pub fn stop(&mut self) -> Duration {
        if let Some(start) = self.start_time {
            let duration = start.elapsed();
            self.state.duration = duration;
            duration
        } else {
            Duration::new(0, 0)
        }
    }
    
    /// Set iteration count
    pub fn set_iterations(&mut self, iterations: u64) {
        self.state.iterations = iterations;
    }
    
    /// Set bytes per second
    pub fn set_bytes_per_second(&mut self, bytes_per_second: u64) {
        self.state.bytes_per_second = Some(bytes_per_second);
    }
    
    /// Log a message
    pub fn log(&self, message: &str) {
        if self.verbose {
            println!("[{}] {}", self.name, message);
        }
    }
}

/// Test operations handler
pub struct TestHandler {
    verbose: bool,
}

impl TestHandler {
    /// Create a new test handler
    pub fn new() -> Self {
        Self {
            verbose: false,
        }
    }
    
    /// Set verbose mode
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Assert equality
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left == right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} == {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} != {:?}", left, right)))
        }
    }
    
    /// Assert not equality
    pub fn assert_ne<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left != right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} != {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} == {:?}", left, right)))
        }
    }
    
    /// Assert true
    pub fn assert_true(&self, condition: bool) -> TestResult<()> {
        if condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is true");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is false"))
        }
    }
    
    /// Assert false
    pub fn assert_false(&self, condition: bool) -> TestResult<()> {
        if !condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is false");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is true"))
        }
    }
    
    /// Run a test
    pub fn run_test<F>(&self, name: &str, test_fn: F) -> TestResult<()>
    where
        F: FnOnce() -> TestResult<()>,
    {
        if self.verbose {
            println!("🧪 Running test: {}", name);
        }
        
        match test_fn() {
            Ok(()) => {
                if self.verbose {
                    println!("✅ Test passed: {}", name);
                }
                Ok(())
            }
            Err(e) => {
                println!("❌ Test failed: {}: {}", name, e);
                Err(e)
            }
        }
    }
}

impl Default for TestHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize test processing
pub fn init_core() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (core) initialized");
    Ok(())
}

/// Test functionality
pub fn test_core() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
