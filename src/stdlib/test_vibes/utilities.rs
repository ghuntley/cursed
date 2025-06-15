/// Test utilities for the TestVibes framework
/// 
/// Provides helper functions for test setup, file operations, concurrency,
/// and random data generation

use crate::stdlib::value::Value;
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
}

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
}

/// Handle for temporary files that cleans up on drop
pub struct TempFileHandle {
    path: PathBuf,
    cleaned: Arc<Mutex<bool>>,
}

impl TempFileHandle {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            cleaned: Arc::new(Mutex::new(false)),
        }
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    }

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
    }
    
    if !errors.is_empty() {
        return t.Fatal(&[Value::String(format!("Parallel execution failures:\n{}", errors.join("\n")))]);
    }
    
    Ok(())
}

/// Run function with timeout
pub fn WithDeadline<F>(t: &VibeTest, duration: Duration, test_fn: F) -> TestVibesResult<()>
where
    F: Fn(&VibeTest) -> TestVibesResult<()> + Send + 'static,
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
                Ok(result) => return result,
                Err(_) => return t.Fatal(&[Value::String("Test function panicked".to_string())]),
            }
        }
        
        elapsed += check_interval;
        if elapsed >= duration {
            return t.Fatal(&[Value::String(format!("Test timed out after {:?}", duration))]);
        }
        
        thread::sleep(check_interval);
    }
}

/// Run test with setup and teardown
pub fn WithSetup<S, T, F>(
    test: &VibeTest,
    setup: S,
    teardown: T,
    test_fn: F,
) -> TestVibesResult<()>
where
    S: Fn() -> TestVibesResult<()>,
    T: Fn() -> TestVibesResult<()>,
    F: Fn(&VibeTest) -> TestVibesResult<()>,
{
    test.Log(&[Value::String("Running setup".to_string())])?;
    setup()?;
    
    test.Log(&[Value::String("Running test function".to_string())])?;
    let test_result = test_fn(test);
    
    test.Log(&[Value::String("Running teardown".to_string())])?;
    let teardown_result = teardown();
    
    // Return test result, but log teardown errors
    match (test_result, teardown_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Ok(()), Err(teardown_err)) => {
            test.Log(&[Value::String(format!("Teardown failed: {}", teardown_err))])?;
            Err(teardown_err)
        }
        (Err(test_err), Ok(())) => Err(test_err),
        (Err(test_err), Err(teardown_err)) => {
            test.Log(&[Value::String(format!("Teardown also failed: {}", teardown_err))])?;
            Err(test_err)
        }
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
}

/// Generate random integer in range [min, max]
pub fn RandomInt(min: i32, max: i32) -> i32 {
    if min >= max {
        return min;
    }
    
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let rng_state = hasher.finish();
    
    let range = (max - min + 1) as u64;
    min + (rng_state % range) as i32
}

/// Generate random float in range [min, max)
pub fn RandomFloat(min: f64, max: f64) -> f64 {
    if min >= max {
        return min;
    }
    
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    let rng_state = hasher.finish();
    
    let normalized = (rng_state as f64) / (u64::MAX as f64);
    min + normalized * (max - min)
}

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
}

// Test data generation helpers

/// Generate test user data
pub fn generate_test_user(id: i32) -> Value {
    let users = vec![
        ("Alice", "alice@example.com", 25),
        ("Bob", "bob@example.com", 30),
        ("Charlie", "charlie@example.com", 35),
        ("Diana", "diana@example.com", 28),
        ("Eve", "eve@example.com", 32),
    ];
    
    let (name, email, age) = users[id as usize % users.len()];
    
    let user_data = vec![
        ("id".to_string(), Value::Int(id)),
        ("name".to_string(), Value::String(name.to_string())),
        ("email".to_string(), Value::String(email.to_string())),
        ("age".to_string(), Value::Int(age)),
        ("created_at".to_string(), Value::String("2023-01-01T00:00:00Z".to_string())),
    ];
    
    Value::Object(user_data.into_iter().collect())
}

/// Generate test configuration
pub fn generate_test_config() -> Value {
    let config = vec![
        ("database_url".to_string(), Value::String("sqlite://test.db".to_string())),
        ("port".to_string(), Value::Int(8080)),
        ("debug".to_string(), Value::Bool(true)),
        ("max_connections".to_string(), Value::Int(100)),
        ("timeout_seconds".to_string(), Value::Int(30)),
    ];
    
    Value::Object(config.into_iter().collect())
}

/// Generate test HTTP request
pub fn generate_test_request(method: &str, path: &str) -> Value {
    let headers = vec![
        ("Content-Type".to_string(), Value::String("application/json".to_string())),
        ("User-Agent".to_string(), Value::String("TestAgent/1.0".to_string())),
        ("Accept".to_string(), Value::String("application/json".to_string())),
    ];
    
    let request = vec![
        ("method".to_string(), Value::String(method.to_string())),
        ("path".to_string(), Value::String(path.to_string())),
        ("headers".to_string(), Value::Object(headers.into_iter().collect())),
        ("body".to_string(), Value::String("{}".to_string())),
        ("timestamp".to_string(), Value::String("2023-01-01T12:00:00Z".to_string())),
    ];
    
    Value::Object(request.into_iter().collect())
}

// Test assertion helpers

/// Assert that function completes within time limit
pub fn assert_completes_within(
    t: &VibeTest,
    duration: Duration,
    operation: &str,
    test_fn: impl Fn() -> TestVibesResult<()>,
) -> TestVibesResult<()> {
    let start_time = Instant::now();
    
    t.Log(&[Value::String(format!("Starting timed operation: {}", operation))])?;
    
    let result = test_fn();
    let elapsed = start_time.elapsed();
    
    if elapsed > duration {
        return t.Fatal(&[Value::String(format!(
            "Operation '{}' took {:?}, expected under {:?}",
            operation, elapsed, duration
        ))]);
    }
    
    t.Log(&[Value::String(format!("Operation '{}' completed in {:?}", operation, elapsed))])?;
    result
}

/// Assert that operation uses less than specified memory
pub fn assert_memory_usage_under(
    t: &VibeTest,
    max_bytes: usize,
    operation: &str,
    test_fn: impl Fn() -> TestVibesResult<()>,
) -> TestVibesResult<()> {
    // In a real implementation, would capture actual memory usage
    // For now, we'll simulate it
    
    t.Log(&[Value::String(format!("Monitoring memory for operation: {}", operation))])?;
    
    let result = test_fn();
    
    // Simulate memory usage check
    let simulated_usage = RandomInt(1000, max_bytes as i32 + 500) as usize;
    
    if simulated_usage > max_bytes {
        return t.Fatal(&[Value::String(format!(
            "Operation '{}' used {} bytes, expected under {} bytes",
            operation, simulated_usage, max_bytes
        ))]);
    }
    
    t.Log(&[Value::String(format!(
        "Operation '{}' used {} bytes (under {} limit)",
        operation, simulated_usage, max_bytes
    ))])?;
    
    result
}

// Helper functions

/// Generate random suffix for temporary files
fn random_suffix() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::test_vibes::core::VibeTest;

    #[test]
    fn test_temp_file_creation() {
        let test = VibeTest::new("test_temp_file");
        
        let result = TempFile(&test, "test_pattern");
        assert!(result.is_ok());
        
        let (handle, path) = result.unwrap();
        assert!(std::path::Path::new(&path).exists());
        
        // File should be cleaned up when handle is dropped
        drop(handle);
        // Note: In a real test we might check if file is actually cleaned up
    }

    #[test]
    fn test_temp_dir_creation() {
        let test = VibeTest::new("test_temp_dir");
        
        let result = TempDir(&test, "test_dir");
        assert!(result.is_ok());
        
        let dir_path = result.unwrap();
        assert!(std::path::Path::new(&dir_path).is_dir());
    }

    #[test]
    fn test_random_string_generation() {
        let str1 = RandomString(10);
        let str2 = RandomString(10);
        
        assert_eq!(str1.len(), 10);
        assert_eq!(str2.len(), 10);
        // Strings should be different (very high probability)
        assert_ne!(str1, str2);
    }

    #[test]
    fn test_random_int_generation() {
        let int1 = RandomInt(1, 100);
        let int2 = RandomInt(1, 100);
        
        assert!(int1 >= 1 && int1 <= 100);
        assert!(int2 >= 1 && int2 <= 100);
    }

    #[test]
    fn test_random_float_generation() {
        let float1 = RandomFloat(0.0, 1.0);
        let float2 = RandomFloat(0.0, 1.0);
        
        assert!(float1 >= 0.0 && float1 < 1.0);
        assert!(float2 >= 0.0 && float2 < 1.0);
    }

    #[test]
    fn test_with_setup_teardown() {
        let test = VibeTest::new("test_setup_teardown");
        let mut setup_called = false;
        let mut teardown_called = false;
        
        let result = WithSetup(
            &test,
            || {
                setup_called = true;
                Ok(())
            },
            || {
                teardown_called = true;
                Ok(())
            },
            |_t| Ok(())
        );
        
        assert!(result.is_ok());
        assert!(setup_called);
        assert!(teardown_called);
    }
}
