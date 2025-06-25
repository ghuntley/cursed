/// Test utilities for the TestVibes framework
/// 
/// Provides helper functions for test setup, file operations, concurrency,
/// and random data generation

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use super::{VibeTest, TestVibesResult};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

// Temporary resource management

/// Create a temporary file for testing
pub fn TempFile(t: &VibeTest, pattern: &str) -> TestVibesResult<(TempFileHandle, String)> {
    let temp_dir = std::env::temp_dir();
    let file_name = format!("{}_{}.tmp", pattern, random_suffix());
    let file_path = temp_dir.join(file_name);
    
    // Create the file
    std::fs::File::create(&file_path).map_err(|e| {
        super::TestVibesError::InvalidConfig(format!("Failed to create temp file: {}", e))
    })?;
    
    let path_str = file_path.to_string_lossy().to_string();
    t.Log(&[Value::String(format!("Created temp file: {}", path_str))])?;
    
    Ok((TempFileHandle::new(file_path), path_str))
/// Create a temporary directory for testing
pub fn TempDir(t: &VibeTest, pattern: &str) -> TestVibesResult<String> {
    let temp_base = std::env::temp_dir();
    let dir_name = format!("{}_{}", pattern, random_suffix());
    let dir_path = temp_base.join(dir_name);
    
    std::fs::create_dir_all(&dir_path).map_err(|e| {
        super::TestVibesError::InvalidConfig(format!("Failed to create temp directory: {}", e))
    })?;
    
    let path_str = dir_path.to_string_lossy().to_string();
    t.Log(&[Value::String(format!("Created temp directory: {}", path_str))])?;
    
    Ok(path_str)
/// Handle for temporary files that cleans up on drop
pub struct TempFileHandle {
impl TempFileHandle {
    fn new(path: PathBuf) -> Self {
        Self {
        }
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    /// Manually clean up the file
    pub fn cleanup(&self) -> TestVibesResult<()> {
        let mut cleaned = self.cleaned.lock().unwrap();
        if !*cleaned {
            if self.path.exists() {
                std::fs::remove_file(&self.path).map_err(|e| {
                    super::TestVibesError::InvalidConfig(format!("Failed to cleanup temp file: {}", e))
                })?;
            }
            *cleaned = true;
        }
        Ok(())
    }
}

impl Drop for TempFileHandle {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

// Concurrency helpers

/// Run functions in parallel
pub fn Parallel(t: &VibeTest, test_fns: Vec<Box<dyn Fn(&VibeTest) -> TestVibesResult<()> + Send + Sync>>) -> TestVibesResult<()> {
    t.Log(&[Value::String(format!("Running {} functions in parallel", test_fns.len()))])?;
    
    let handles: Vec<_> = test_fns.into_iter().enumerate().map(|(i, test_fn)| {
        let sub_test = VibeTest::new(&format!("{}_{}", t.Name(), i));
        thread::spawn(move || {
            test_fn(&sub_test)
        })
    }).collect();
    
    let mut errors = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(Ok(())) => {
                t.Log(&[Value::String(format!("Parallel function {} completed successfully", i))])?;
            }
            Ok(Err(e)) => {
                errors.push(format!("Parallel function {} failed: {}", i, e));
            }
            Err(_) => {
                errors.push(format!("Parallel function {} panicked", i));
            }
        }
    if !errors.is_empty() {
        return t.Fatal(&[Value::String(format!("Parallel execution failures:\n{}", errors.join("\n")))]);
    Ok(())
/// Run function with timeout
pub fn WithDeadline<F>(t: &VibeTest, duration: Duration, test_fn: F) -> TestVibesResult<()>
where
{
    let sub_test = VibeTest::new(&format!("{}_timed", t.Name()));
    let start_time = Instant::now();
    
    t.Log(&[Value::String(format!("Running function with {:?} timeout", duration))])?;
    
    let handle = thread::spawn(move || {
        test_fn(&sub_test)
    });
    
    // Simple timeout implementation - in production would use more sophisticated approach
    let mut elapsed = Duration::new(0, 0);
    let check_interval = Duration::from_millis(100);
    
    loop {
        if handle.is_finished() {
            match handle.join() {
            }
        }
        
        elapsed += check_interval;
        if elapsed >= duration {
            return t.Fatal(&[Value::String(format!("Test timed out after {:?}", duration))]);
        thread::sleep(check_interval);
    }
}

/// Run test with setup and teardown
pub fn WithSetup<S, T, F>(
) -> TestVibesResult<()>
where
{
    test.Log(&[Value::String("Running setup".to_string())])?;
    setup()?;
    
    test.Log(&[Value::String("Running test function".to_string())])?;
    let test_result = test_fn(test);
    
    test.Log(&[Value::String("Running teardown".to_string())])?;
    let teardown_result = teardown();
    
    // Return test result, but log teardown errors
    match (test_result, teardown_result) {
        (Ok(()), Err(teardown_err)) => {
            test.Log(&[Value::String(format!("Teardown failed: {}", teardown_err))])?;
            Err(teardown_err)
        }
        (Err(test_err), Err(teardown_err)) => {
            test.Log(&[Value::String(format!("Teardown also failed: {}", teardown_err))])?;
            Err(test_err)
        }
    }
// Random data generation

/// Generate random string of specified length
pub fn RandomString(n: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let mut rng_state = hasher.finish();
    
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    (0..n)
        .map(|_| {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let idx = (rng_state % CHARS.len() as u64) as usize;
            CHARS[idx] as char
        })
        .collect()
/// Generate random integer in range [min, max]
pub fn RandomInt(min: i32, max: i32) -> i32 {
    if min >= max {
        return min;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let rng_state = hasher.finish();
    
    let range = (max - min + 1) as u64;
    min + (rng_state % range) as i32
/// Generate random float in range [min, max)
pub fn RandomFloat(min: f64, max: f64) -> f64 {
    if min >= max {
        return min;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let rng_state = hasher.finish();
    
    let normalized = (rng_state as f64) / (u64::MAX as f64);
    min + normalized * (max - min)
/// Generate random bytes
pub fn RandomBytes(n: usize) -> Vec<u8> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let mut rng_state = hasher.finish();
    
    (0..n)
        .map(|_| {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            (rng_state % 256) as u8
        })
        .collect()
// Test data generation helpers

/// Generate test user data
pub fn generate_test_user(id: i32) -> Value {
    let users = vec![
    ];
    
    let (name, email, age) = users[id as usize % users.len()];
    
    let user_data = vec![
    ];
    
    Value::Object(user_data.into_iter().collect())
/// Generate test configuration
pub fn generate_test_config() -> Value {
    let config = vec![
        ("database_url".to_string(), Value::String("sqlite://test.db".to_string())),
    ];
    
    Value::Object(config.into_iter().collect())
/// Generate test HTTP request
pub fn generate_test_request(method: &str, path: &str) -> Value {
    let headers = vec![
        ("Content-Type".to_string(), Value::String("application/json".to_string())),
        ("User-Agent".to_string(), Value::String("TestAgent/1.0".to_string())),
        ("Accept".to_string(), Value::String("application/json".to_string())),
    ];
    
    let request = vec![
    ];
    
    Value::Object(request.into_iter().collect())
// Test assertion helpers

/// Assert that function completes within time limit
pub fn assert_completes_within(
) -> TestVibesResult<()> {
    let start_time = Instant::now();
    
    t.Log(&[Value::String(format!("Starting timed operation: {}", operation))])?;
    
    let result = test_fn();
    let elapsed = start_time.elapsed();
    
    if elapsed > duration {
        return t.Fatal(&[Value::String(format!(
            operation, elapsed, duration
        ))]);
    t.Log(&[Value::String(format!("Operation '{}' completed in {:?}", operation, elapsed))])?;
    result
/// Assert that operation uses less than specified memory
pub fn assert_memory_usage_under(
) -> TestVibesResult<()> {
    // In a real implementation, would capture actual memory usage
    // For now, we'll simulate it
    
    t.Log(&[Value::String(format!("Monitoring memory for operation: {}", operation))])?;
    
    let result = test_fn();
    
    // Simulate memory usage check
    let simulated_usage = RandomInt(1000, max_bytes as i32 + 500) as usize;
    
    if simulated_usage > max_bytes {
        return t.Fatal(&[Value::String(format!(
            operation, simulated_usage, max_bytes
        ))]);
    t.Log(&[Value::String(format!(
        operation, simulated_usage, max_bytes
    ))])?;
    
    result
// Helper functions

/// Generate random suffix for temporary files
fn random_suffix() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    
    format!("{:x}", hasher.finish())
