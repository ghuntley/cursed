/// fr fr Test utilities and helper functions for the TestVibes framework
// use crate::stdlib::packages::test_vibes::core::VibeTest;
use crate::error::CursedError;
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir_all};
use std::io::Write;
use rand::Rng;

/// fr fr Temporary file for testing
pub struct TempFile {
    path: PathBuf,
    file: Option<File>,
}

impl TempFile {
    /// fr fr Create a new temporary file
    pub fn new(t: &mut VibeTest, pattern: &str) -> crate::error::Result<()> {
        let temp_dir = std::env::temp_dir();
        let file_name = format!("{}_{}", pattern, generate_random_suffix());
        let path = temp_dir.join(file_name);
        
        let file = File::create(&path)?;
        let temp_file = TempFile {
            path: path.clone(),
            file: Some(file),
        };
        
        t.log(&[&format!("Created temporary file: {}", path.display())]);
        
        Ok((temp_file, path.to_string_lossy().to_string()))
    }

    /// fr fr Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// fr fr Write content to the file
    pub fn write_all(&mut self, content: &[u8]) -> crate::error::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_all(content)?;
            file.flush()?;
        }
        Ok(())
    }

    /// fr fr Write string content to the file
    pub fn write_string(&mut self, content: &str) -> crate::error::Result<()> {
        self.write_all(content.as_bytes())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Clean up the temporary file
        let _ = std::fs::remove_file(&self.path);
    }
}

/// fr fr Temporary directory for testing
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// fr fr Create a new temporary directory
    pub fn new(t: &mut VibeTest, pattern: &str) -> crate::error::Result<()> {
        let temp_base = std::env::temp_dir();
        let dir_name = format!("{}_{}", pattern, generate_random_suffix());
        let path = temp_base.join(dir_name);
        
        create_dir_all(&path)?;
        
        let temp_dir = TempDir { path: path.clone() };
        
        t.log(&[&format!("Created temporary directory: {}", path.display())]);
        
        Ok((temp_dir, path.to_string_lossy().to_string()))
    }

    /// fr fr Get the directory path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// fr fr Create a file in the temporary directory
    pub fn create_file(&self, name: &str) -> crate::error::Result<()> {
        let file_path = self.path.join(name);
        File::create(&file_path)?;
        Ok(file_path)
    }

    /// fr fr Create a subdirectory
    pub fn create_dir(&self, name: &str) -> crate::error::Result<()> {
        let dir_path = self.path.join(name);
        create_dir_all(&dir_path)?;
        Ok(dir_path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // Clean up the temporary directory
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// fr fr Convenience functions for temporary resources

/// fr fr Create a temporary file
pub fn temp_file(t: &mut VibeTest, pattern: &str) -> crate::error::Result<()> {
    TempFile::new(t, pattern)
}

/// fr fr Create a temporary directory
pub fn temp_dir(t: &mut VibeTest, pattern: &str) -> crate::error::Result<()> {
    TempDir::new(t, pattern)
}

/// fr fr Concurrency helpers

/// fr fr Run functions in parallel
pub fn parallel(t: &mut VibeTest, fns: Vec<Box<dyn Fn(&mut VibeTest) + Send>>) {
    let mut handles = Vec::new();
    
    for (i, test_fn) in fns.into_iter().enumerate() {
        let test_name = format!("{}::parallel_{}", t.name(), i);
        
        let handle = std::thread::spawn(move || {
            let mut parallel_test = VibeTest::new(test_name);
            test_fn(&mut parallel_test);
            parallel_test.get_result()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads and collect results
    for handle in handles {
        match handle.join() {
            Ok(result) => {
                if result.failed {
                    t.error(&["Parallel test failed"]);
                    for error in &result.errors {
                        t.error(&[error]);
                    }
                }
            }
            Err(_) => {
                t.error(&["Parallel test panicked"]);
            }
        }
    }
}

/// fr fr Run with timeout
pub fn with_deadline<F>(t: &mut VibeTest, deadline: Duration, test_fn: F) 
where
    F: Fn(&mut VibeTest) + Send + 'static,
{
    let test_name = format!("{}::with_deadline", t.name());
    let mut deadline_test = VibeTest::new(test_name);
    
    let start = Instant::now();
    
    // Create a channel for communication
    let (sender, receiver) = std::sync::mpsc::channel();
    
    // Spawn the test in a separate thread
    let handle = std::thread::spawn(move || {
        test_fn(&mut deadline_test);
        let _ = sender.send(deadline_test.get_result());
    });
    
    // Wait with timeout
    match receiver.recv_timeout(deadline) {
        Ok(result) => {
            // Test completed within deadline
            if result.failed {
                t.error(&["Test failed within deadline"]);
                for error in &result.errors {
                    t.error(&[error]);
                }
            }
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            // Test exceeded deadline
            t.error(&[&format!("Test exceeded deadline of {:?}", deadline)]);
            // Try to join the thread, but don't wait indefinitely
            let _ = handle.join();
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            t.error(&["Test thread disconnected unexpectedly"]);
        }
    }
    
    let elapsed = start.elapsed();
    t.log(&[&format!("Test completed in {:?}", elapsed)]);
}

/// fr fr Setup and teardown wrapper
pub fn with_setup(
    t: &mut VibeTest,
    setup: impl Fn(),
    teardown: impl Fn(),
    test_fn: impl Fn(&mut VibeTest),
) {
    // Setup phase
    t.log(&["Running setup"]);
    setup();
    
    // Run test
    test_fn(t);
    
    // Teardown phase (always run, even if test failed)
    t.log(&["Running teardown"]);
    teardown();
}

/// fr fr Random data generation

/// fr fr Generate a random string
pub fn random_string(n: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// fr fr Generate a random integer
pub fn random_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// fr fr Generate a random float
pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// fr fr Generate random bytes
pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

/// fr fr File system test utilities

/// fr fr Create a test file with content
pub fn create_test_file(path: &Path, content: &str) -> crate::error::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// fr fr Read file content as string
pub fn read_test_file(path: &Path) -> crate::error::Result<()> {
    std::fs::read_to_string(path)
}

/// fr fr Check if file exists
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// fr fr Check if directory exists
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// fr fr Environment test utilities

/// fr fr Set environment variable for test
pub fn with_env_var<F>(key: &str, value: &str, test_fn: F)
where
    F: Fn(),
{
    let original_value = std::env::var(key).ok();
    
    // Set the environment variable
    std::env::set_var(key, value);
    
    // Run the test
    test_fn();
    
    // Restore original value
    match original_value {
        Some(orig_val) => std::env::set_var(key, orig_val),
        None => std::env::remove_var(key),
    }
}

/// fr fr Remove environment variable for test
pub fn without_env_var<F>(key: &str, test_fn: F)
where
    F: Fn(),
{
    let original_value = std::env::var(key).ok();
    
    // Remove the environment variable
    std::env::remove_var(key);
    
    // Run the test
    test_fn();
    
    // Restore original value if it existed
    if let Some(orig_val) = original_value {
        std::env::set_var(key, orig_val);
    }
}

/// fr fr Timing utilities

/// fr fr Time a function execution
pub fn time_function<F, R>(func: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (result, duration)
}

/// fr fr Retry utilities

/// fr fr Retry a function with exponential backoff
pub fn retry_with_backoff<F, R, E>(
    mut func: F,
    max_attempts: usize,
    initial_delay: Duration,
) -> Result<R, E>
where
    F: FnMut() -> Result<R, E>,
{
    let mut attempts = 0;
    let mut delay = initial_delay;
    
    loop {
        attempts += 1;
        
        match func() {
            Ok(result) => return Ok(result),
            Err(err) => {
                if attempts >= max_attempts {
                    return Err(err);
                }
                
                std::thread::sleep(delay);
                delay = Duration::from_millis((delay.as_millis() as u64 * 2).min(30000));
            }
        }
    }
}

/// fr fr Test data builders

/// fr fr Build test data for various types
pub struct TestDataBuilder;

impl TestDataBuilder {
    /// fr fr Create a vector of test integers
    pub fn integers(count: usize) -> Vec<i32> {
        (0..count).map(|i| i as i32).collect()
    }

    /// fr fr Create a vector of test strings
    pub fn strings(count: usize) -> Vec<String> {
        (0..count).map(|i| format!("test_string_{}", i)).collect()
    }

    /// fr fr Create a hashmap of test data
    pub fn string_map(count: usize) -> std::collections::HashMap<String, String> {
        (0..count)
            .map(|i| (format!("key_{}", i), format!("value_{}", i)))
            .collect()
    }

    /// fr fr Create large test data
    pub fn large_string(size: usize) -> String {
        "X".repeat(size)
    }

    /// fr fr Create nested test data
    pub fn nested_vectors(depth: usize, width: usize) -> Vec<Vec<i32>> {
        (0..depth)
            .map(|_| Self::integers(width))
            .collect()
    }
}

/// fr fr Helper functions

/// fr fr Generate a random suffix for file names
fn generate_random_suffix() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:08x}", rng.gen::<u32>())
}

/// fr fr Assertion helpers

/// fr fr Eventually assert - retry until condition is true or timeout
pub fn eventually_assert<F>(
    t: &mut VibeTest,
    condition: F,
    timeout: Duration,
    message: &str,
) where
    F: Fn() -> bool,
{
    let start = Instant::now();
    let mut last_check = start;
    
    while start.elapsed() < timeout {
        if condition() {
            return; // Success
        }
        
        // Wait a bit before retrying
        let wait_time = Duration::from_millis(10);
        std::thread::sleep(wait_time);
        last_check = Instant::now();
    }
    
    // Timeout reached without success
    t.fail_vibe(&format!("Eventually assert failed: {} (timeout: {:?})", message, timeout));
}

/// fr fr Assert that a condition remains true for a duration
pub fn consistently_assert<F>(
    t: &mut VibeTest,
    condition: F,
    duration: Duration,
    message: &str,
) where
    F: Fn() -> bool,
{
    let start = Instant::now();
    
    while start.elapsed() < duration {
        if !condition() {
            t.fail_vibe(&format!("Consistently assert failed: {} (after {:?})", message, start.elapsed()));
            return;
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    // Success - condition remained true for the entire duration
}

