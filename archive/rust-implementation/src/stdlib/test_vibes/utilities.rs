//! Testing functionality for utilities

use crate::error::CursedError;
use rand::Rng;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Generate a random string for testing
pub fn random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate a random integer for testing
pub fn random_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Generate a random float for testing
pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Generate random bytes for testing
pub fn random_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u8>()).collect()
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
pub fn init_utilities() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (utilities) initialized");
    Ok(())
}

/// Test functionality
pub fn test_utilities() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
