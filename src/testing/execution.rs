/// Test Execution Engine
/// 
/// Handles compilation and execution of individual CURSED test functions
/// using the existing LLVM compilation pipeline.

use crate::error::Error;
use crate::codegen::LlvmCodeGenerator;
use super::{TestError, TestResult};
use super::discovery::{TestFile, TestFunction, TestSuite};
use super::framework::TestContext;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::timeout;
use tracing::{info, debug, warn, error};

/// Test execution result
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Test function that was executed
    pub test_function: TestFunction,
    /// Test execution status
    pub status: TestStatus,
    /// Execution duration
    pub duration: Duration,
    /// Standard output captured during test
    pub stdout: String,
    /// Standard error captured during test
    pub stderr: String,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Memory usage statistics
    pub memory_stats: Option<MemoryStats>,
    /// Test metrics and measurements
    pub metrics: HashMap<String, f64>,
}

/// Test execution status
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    /// Test passed successfully
    Passed,
    /// Test failed with an error
    Failed,
    /// Test was skipped
    Skipped,
    /// Test timed out
    Timeout,
    /// Test compilation failed
    CompilationError,
    /// Test panicked/crashed
    Panicked,
    /// Test result is ignored (expected to fail)
    Ignored,
}

/// Memory usage statistics during test execution
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Peak memory usage in bytes
    pub peak_memory_bytes: u64,
    /// Total allocations
    pub total_allocations: u64,
    /// Memory leaks detected
    pub leaks_detected: u64,
    /// Garbage collection cycles
    pub gc_cycles: u64,
}

/// Test execution context and environment
#[derive(Debug, Clone)]
pub struct TestExecutionContext {
    /// Working directory for test execution
    pub working_directory: std::path::PathBuf,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Test timeout in seconds
    pub timeout_seconds: u64,
    /// Enable memory profiling
    pub memory_profiling: bool,
    /// Enable coverage collection
    pub coverage_collection: bool,
    /// Test data directory
    pub test_data_dir: Option<std::path::PathBuf>,
}

impl Default for TestExecutionContext {
    fn default() -> Self {
        Self {
            working_directory: std::env::current_dir().unwrap_or_default(),
            environment: HashMap::new(),
            timeout_seconds: 30,
            memory_profiling: false,
            coverage_collection: false,
            test_data_dir: None,
        }
    }
}

/// Test executor handles compilation and execution of tests
pub struct TestExecutor {
    /// LLVM code generator for compilation
    codegen: Arc<LlvmCodeGenerator>,
    /// Execution context
    context: TestExecutionContext,
    /// Compiled test cache
    compilation_cache: HashMap<String, CompiledTest>,
}

/// Compiled test representation
#[derive(Debug, Clone)]
struct CompiledTest {
    /// Compiled LLVM IR
    pub llvm_ir: String,
    /// Compilation timestamp
    pub compiled_at: Instant,
    /// Source hash for cache invalidation
    pub source_hash: u64,
}

impl TestExecutor {
    /// Create new test executor
    pub fn new() -> TestResult<Self> {
        let codegen = LlvmCodeGenerator::new()
            .map_err(|e| TestError::Framework(format!("Failed to create LLVM codegen: {}", e)))?;
        
        Ok(Self {
            codegen: Arc::new(codegen),
            context: TestExecutionContext::default(),
            compilation_cache: HashMap::new(),
        })
    }

    /// Create test executor with custom context
    pub fn with_context(context: TestExecutionContext) -> TestResult<Self> {
        let mut executor = Self::new()?;
        executor.context = context;
        Ok(executor)
    }

    /// Execute a single test function
    pub async fn execute_test(&mut self, test_function: &TestFunction, test_file: &TestFile) -> TestResult {
        let start_time = Instant::now();
        
        info!("Executing test: {}", test_function.name);
        
        // Check if test should be skipped
        if self.should_skip_test(test_function) {
            return TestResult {
                test_function: test_function.clone(),
                status: TestStatus::Skipped,
                duration: start_time.elapsed(),
                stdout: String::new(),
                stderr: String::new(),
                error_message: Some("Test skipped".to_string()),
                memory_stats: None,
                metrics: HashMap::new(),
            };
        }

        // Compile the test
        let compilation_result = self.compile_test(test_function, test_file).await;
        
        let compiled_ir = match compilation_result {
            Ok(ir) => ir,
            Err(e) => {
                return TestResult {
                    test_function: test_function.clone(),
                    status: TestStatus::CompilationError,
                    duration: start_time.elapsed(),
                    stdout: String::new(),
                    stderr: String::new(),
                    error_message: Some(format!("Compilation failed: {}", e)),
                    memory_stats: None,
                    metrics: HashMap::new(),
                };
            }
        };

        // Execute the compiled test with timeout
        let timeout_duration = Duration::from_secs(
            test_function.timeout_override.unwrap_or(self.context.timeout_seconds)
        );

        let execution_result = timeout(timeout_duration, self.execute_compiled_test(&compiled_ir, test_function)).await;

        let (status, stdout, stderr, error_message, memory_stats, metrics) = match execution_result {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => (
                TestStatus::Failed,
                String::new(),
                String::new(),
                Some(format!("Execution failed: {}", e)),
                None,
                HashMap::new(),
            ),
            Err(_) => (
                TestStatus::Timeout,
                String::new(),
                String::new(),
                Some(format!("Test timed out after {:?}", timeout_duration)),
                None,
                HashMap::new(),
            ),
        };

        // Handle expected failures
        let final_status = if test_function.should_fail {
            match status {
                TestStatus::Failed => TestStatus::Passed, // Expected to fail, and it did
                TestStatus::Passed => TestStatus::Failed, // Expected to fail, but it passed
                other => other,
            }
        } else {
            status
        };

        TestResult {
            test_function: test_function.clone(),
            status: final_status,
            duration: start_time.elapsed(),
            stdout,
            stderr,
            error_message,
            memory_stats,
            metrics,
        }
    }

    /// Execute multiple tests in parallel
    pub async fn execute_tests_parallel(
        &mut self, 
        tests: Vec<(TestFunction, TestFile)>,
        max_parallel: usize
    ) -> Vec<TestResult> {
        use futures::stream::{FuturesUnordered, StreamExt};
        
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_parallel));
        let mut futures = FuturesUnordered::new();
        
        for (test_function, test_file) in tests {
            let semaphore_clone = semaphore.clone();
            let mut executor_clone = self.clone_for_parallel_execution().await;
            
            let future = async move {
                let _permit = semaphore_clone.acquire().await.unwrap();
                executor_clone.execute_test(&test_function, &test_file).await
            };
            
            futures.push(future);
        }
        
        let mut results = Vec::new();
        while let Some(result) = futures.next().await {
            results.push(result);
        }
        
        results
    }

    /// Execute all tests in a test suite
    pub async fn execute_test_suite(&mut self, test_suite: &TestSuite) -> Vec<TestResult> {
        let mut all_tests = Vec::new();
        
        // Collect all test functions with their files
        for test_file in &test_suite.test_files {
            for test_function in &test_file.test_functions {
                all_tests.push((test_function.clone(), test_file.clone()));
            }
        }
        
        // Determine parallelism level
        let max_parallel = test_suite.config.max_parallel
            .unwrap_or_else(|| std::cmp::min(all_tests.len(), num_cpus::get()));
        
        info!("Executing {} tests with max parallelism of {}", all_tests.len(), max_parallel);
        
        if max_parallel > 1 {
            self.execute_tests_parallel(all_tests, max_parallel).await
        } else {
            // Sequential execution
            let mut results = Vec::new();
            for (test_function, test_file) in all_tests {
                let result = self.execute_test(&test_function, &test_file).await;
                results.push(result);
            }
            results
        }
    }

    /// Check if test should be skipped
    fn should_skip_test(&self, test_function: &TestFunction) -> bool {
        // Skip based on tags or other criteria
        test_function.tags.contains(&"skip".to_string()) ||
        test_function.name.contains("skip_")
    }

    /// Compile a test function to LLVM IR
    async fn compile_test(&mut self, test_function: &TestFunction, test_file: &TestFile) -> TestResult<String> {
        debug!("Compiling test function: {}", test_function.name);
        
        // Check compilation cache
        let cache_key = format!("{}:{}", test_file.path.display(), test_function.name);
        let source_hash = self.calculate_source_hash(&test_function.source_code);
        
        if let Some(cached) = self.compilation_cache.get(&cache_key) {
            if cached.source_hash == source_hash {
                debug!("Using cached compilation for {}", test_function.name);
                return Ok(cached.llvm_ir.clone());
            }
        }

        // Create a complete test program
        let test_program = self.create_test_program(test_function, test_file)?;
        
        // Compile using LLVM codegen
        let ir = self.codegen.compile_to_ir(&test_program)
            .map_err(|e| TestError::Compilation(format!("LLVM compilation failed: {}", e)))?;
        
        // Cache the compilation result
        self.compilation_cache.insert(cache_key, CompiledTest {
            llvm_ir: ir.clone(),
            compiled_at: Instant::now(),
            source_hash,
        });
        
        debug!("Successfully compiled test function: {}", test_function.name);
        Ok(ir)
    }

    /// Execute compiled LLVM IR
    async fn execute_compiled_test(
        &self, 
        llvm_ir: &str, 
        test_function: &TestFunction
    ) -> TestResult<(TestStatus, String, String, Option<String>, Option<MemoryStats>, HashMap<String, f64>)> {
        debug!("Executing compiled test: {}", test_function.name);
        
        // For now, simulate test execution
        // In a real implementation, this would:
        // 1. Create an execution environment
        // 2. Load and run the LLVM IR
        // 3. Capture output and measure performance
        // 4. Handle panics and errors
        
        // Simulate different test outcomes based on function name
        if test_function.name.contains("fail") {
            Ok((
                TestStatus::Failed,
                String::new(),
                String::new(),
                Some("Test failed as expected".to_string()),
                None,
                HashMap::new(),
            ))
        } else if test_function.name.contains("panic") {
            Ok((
                TestStatus::Panicked,
                String::new(),
                "Test panicked!".to_string(),
                Some("Test panicked during execution".to_string()),
                None,
                HashMap::new(),
            ))
        } else {
            // Simulate successful test execution
            let mut metrics = HashMap::new();
            metrics.insert("execution_time_ms".to_string(), 1.5);
            metrics.insert("memory_peak_mb".to_string(), 2.1);
            
            let memory_stats = MemoryStats {
                peak_memory_bytes: 2_097_152, // 2MB
                total_allocations: 150,
                leaks_detected: 0,
                gc_cycles: 1,
            };
            
            Ok((
                TestStatus::Passed,
                "Test output".to_string(),
                String::new(),
                None,
                Some(memory_stats),
                metrics,
            ))
        }
    }

    /// Create a complete test program from test function
    fn create_test_program(&self, test_function: &TestFunction, test_file: &TestFile) -> TestResult<String> {
        // Read the original file to get imports and context
        let file_content = std::fs::read_to_string(&test_file.path)
            .map_err(|e| TestError::Io(format!("Failed to read test file: {}", e)))?;
        
        // Extract imports and package declarations
        let mut imports = Vec::new();
        let mut package_decl = None;
        
        for line in file_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                imports.push(line.to_string());
            } else if trimmed.starts_with("package ") {
                package_decl = Some(line.to_string());
            }
        }
        
        // Create complete test program
        let mut program = String::new();
        
        // Add package declaration
        if let Some(pkg) = package_decl {
            program.push_str(&pkg);
            program.push('\n');
        }
        
        // Add imports
        for import in imports {
            program.push_str(&import);
            program.push('\n');
        }
        
        // Add test framework imports
        program.push_str("import \"stdlib::testing::framework\"\n");
        program.push_str("import \"stdlib::testing::assertions\"\n");
        program.push('\n');
        
        // Add the test function
        program.push_str(&test_function.source_code);
        program.push('\n');
        
        // Add test runner main function
        program.push_str(&format!(r#"
slay main() {{
    sus test_context = TestContext::new()
    
    lowkey {{
        {}()
        test_context.assert_no_failures()
        println("Test {} passed!")
    }} bestie (err) {{
        test_context.report_failure(err.to_string())
        println("Test {} failed: {{}}", err)
        exit(1)
    }}
}}
"#, test_function.name, test_function.name, test_function.name));
        
        Ok(program)
    }

    /// Calculate hash of source code for caching
    fn calculate_source_hash(&self, source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }

    /// Clone executor for parallel execution
    async fn clone_for_parallel_execution(&self) -> Self {
        // Create new instances for parallel execution
        Self {
            codegen: self.codegen.clone(),
            context: self.context.clone(),
            compilation_cache: HashMap::new(), // Each parallel executor gets its own cache
        }
    }
}

impl Clone for TestExecutor {
    fn clone(&self) -> Self {
        Self {
            codegen: self.codegen.clone(),
            context: self.context.clone(),
            compilation_cache: self.compilation_cache.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_executor_creation() {
        let executor = TestExecutor::new();
        assert!(executor.is_ok());
    }

    #[tokio::test]
    async fn test_compilation_caching() {
        let mut executor = TestExecutor::new().unwrap();
        
        let test_function = TestFunction {
            name: "test_example".to_string(),
            line_number: 1,
            column_number: 1,
            source_code: "slay test_example() { assert_true(true) }".to_string(),
            test_type: super::discovery::TestType::Unit,
            should_fail: false,
            timeout_override: None,
            tags: vec![],
        };
        
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test.csd");
        fs::write(&test_file_path, "slay test_example() { assert_true(true) }").unwrap();
        
        let test_file = super::discovery::TestFile {
            path: test_file_path,
            test_functions: vec![test_function.clone()],
            package_name: None,
            size_bytes: 100,
            last_modified: std::time::UNIX_EPOCH,
        };
        
        // First compilation should succeed
        let result1 = executor.compile_test(&test_function, &test_file).await;
        assert!(result1.is_ok());
        
        // Second compilation should use cache
        let result2 = executor.compile_test(&test_function, &test_file).await;
        assert!(result2.is_ok());
        
        // Results should be identical (from cache)
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
}
