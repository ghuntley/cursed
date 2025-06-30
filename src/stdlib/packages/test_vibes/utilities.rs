//! Testing functionality for utilities

use crate::error::CursedError;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{Duration, Instant};
use rand::Rng;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Temporary file management for testing
#[derive(Debug)]
pub struct TempFile {
    pub path: PathBuf,
    pub auto_cleanup: bool,
}

impl TempFile {
    pub fn new() -> TestResult<Self> {
        let temp_dir = std::env::temp_dir();
        let filename = format!("cursed_test_{}", random_string(8));
        let path = temp_dir.join(filename);
        
        // Create empty file
        fs::File::create(&path)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create temp file: {}", e)))?;
        
        Ok(Self {
            path,
            auto_cleanup: true,
        })
    }

    pub fn with_content(content: &str) -> TestResult<Self> {
        let temp_file = Self::new()?;
        fs::write(&temp_file.path, content)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write temp file: {}", e)))?;
        Ok(temp_file)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read(&self) -> TestResult<String> {
        fs::read_to_string(&self.path)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to read temp file: {}", e)))
    }

    pub fn write(&self, content: &str) -> TestResult<()> {
        fs::write(&self.path, content)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write temp file: {}", e)))
    }

    pub fn cleanup(self) -> TestResult<()> {
        if self.path.exists() {
            fs::remove_file(&self.path)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to cleanup temp file: {}", e)))?;
        }
        Ok(())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if self.auto_cleanup && self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Temporary directory management for testing
#[derive(Debug)]
pub struct TempDir {
    pub path: PathBuf,
    pub auto_cleanup: bool,
}

impl TempDir {
    pub fn new() -> TestResult<Self> {
        let temp_dir = std::env::temp_dir();
        let dirname = format!("cursed_test_dir_{}", random_string(8));
        let path = temp_dir.join(dirname);
        
        fs::create_dir_all(&path)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create temp dir: {}", e)))?;
        
        Ok(Self {
            path,
            auto_cleanup: true,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn create_file(&self, name: &str, content: &str) -> TestResult<PathBuf> {
        let file_path = self.path.join(name);
        fs::write(&file_path, content)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create file in temp dir: {}", e)))?;
        Ok(file_path)
    }

    pub fn create_subdir(&self, name: &str) -> TestResult<PathBuf> {
        let dir_path = self.path.join(name);
        fs::create_dir_all(&dir_path)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create subdir in temp dir: {}", e)))?;
        Ok(dir_path)
    }

    pub fn cleanup(self) -> TestResult<()> {
        if self.path.exists() {
            fs::remove_dir_all(&self.path)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to cleanup temp dir: {}", e)))?;
        }
        Ok(())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.auto_cleanup && self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

/// Run a test function in parallel
pub fn parallel<F, T>(test_fn: F) -> TestResult<T>
where
    F: FnOnce() -> TestResult<T> + Send,
    T: Send,
{
    // In a real implementation, this would use std::thread or async runtime
    // For now, just run synchronously
    test_fn()
}

/// Run a test with a deadline
pub fn with_deadline<F, T>(duration: Duration, test_fn: F) -> TestResult<T>
where
    F: FnOnce() -> TestResult<T>,
{
    let start = Instant::now();
    let result = test_fn();
    let elapsed = start.elapsed();
    
    if elapsed > duration {
        return Err(CursedError::runtime_error(&format!(
            "Test exceeded deadline: {:?} > {:?}", elapsed, duration
        )));
    }
    
    result
}

/// Run a test with setup and teardown
pub fn with_setup<F, T>(setup: fn() -> TestResult<()>, teardown: fn() -> TestResult<()>, test_fn: F) -> TestResult<T>
where
    F: FnOnce() -> TestResult<T>,
{
    setup()?;
    let result = test_fn();
    teardown()?;
    result
}

/// Generate random string for test data
pub fn random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

/// Generate random integer for test data
pub fn random_int(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Generate random float for test data
pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Generate random bytes for test data
pub fn random_test_bytes(length: usize) -> Vec<u8> {
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
