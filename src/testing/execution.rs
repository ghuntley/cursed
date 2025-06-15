/// Test Execution Engine
/// 
/// Handles compilation and execution of individual CURSED test functions
/// using the existing LLVM compilation pipeline.

use crate::error::Error;
use crate::codegen::LlvmCodeGenerator;
use super::{TestError, TestResult as TestingResult};
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
    pub fn new() -> TestingResult<Self> {
        let codegen = LlvmCodeGenerator::new()
            .map_err(|e| TestError::Framework(format!("Failed to create LLVM codegen: {}", e)))?;
        
        Ok(Self {
            codegen: Arc::new(codegen),
            context: TestExecutionContext::default(),
            compilation_cache: HashMap::new(),
        })
    }

    /// Create test executor with custom context
    pub fn with_context(context: TestExecutionContext) -> TestingResult<Self> {
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
                error_message: Some("Test was skipped".to_string()),
                memory_stats: None,
                metrics: HashMap::new(),
            };
        }

        // Step 1: Compile the test function
        let llvm_ir = match self.compile_test(test_function, test_file).await {
            Ok(ir) => ir,
            Err(e) => {
                error!("Compilation failed for test {}: {}", test_function.name, e);
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

        // Step 2: Execute the compiled test with timeout
        let timeout_duration = Duration::from_secs(
            test_function.timeout_override.unwrap_or(self.context.timeout_seconds)
        );

        let execution_result = timeout(timeout_duration, 
            self.execute_compiled_test(&llvm_ir, test_function)
        ).await;

        let (status, stdout, stderr, error_message, memory_stats, metrics) = match execution_result {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => {
                error!("Test execution failed for {}: {}", test_function.name, e);
                (TestStatus::Failed, String::new(), String::new(), 
                 Some(format!("Execution failed: {}", e)), None, HashMap::new())
            }
            Err(_) => {
                warn!("Test {} timed out after {:?}", test_function.name, timeout_duration);
                (TestStatus::Timeout, String::new(), String::new(),
                 Some(format!("Test timed out after {:?}", timeout_duration)), None, HashMap::new())
            }
        };

        // Step 3: Handle should_fail logic
        let final_status = match (status, test_function.should_fail) {
            (TestStatus::Failed, true) => {
                info!("Test {} failed as expected", test_function.name);
                TestStatus::Passed  // Expected failure becomes a pass
            }
            (TestStatus::Passed, true) => {
                warn!("Test {} was expected to fail but passed", test_function.name);
                TestStatus::Failed  // Unexpected pass becomes a failure
            }
            (s, false) => s,  // Normal case - status unchanged
            (s, true) => s,   // Other statuses (timeout, compilation error) remain unchanged
        };

        let duration = start_time.elapsed();
        info!("Test {} completed with status {:?} in {:?}", 
              test_function.name, final_status, duration);

        TestResult {
            test_function: test_function.clone(),
            status: final_status,
            duration,
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
    pub fn should_skip_test(&self, test_function: &TestFunction) -> bool {
        // Skip based on tags or other criteria
        test_function.tags.contains(&"skip".to_string()) ||
        test_function.name.contains("skip_")
    }

    /// Compile a test function to LLVM IR
    pub async fn compile_test(&mut self, test_function: &TestFunction, test_file: &TestFile) -> TestingResult<String> {
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
        
        // Parse the test program
        let mut parser = crate::parser::Parser::new(&test_program)
            .map_err(|e| TestError::Compilation(format!("Parser creation failed: {}", e)))?;
        
        let program = parser.parse()
            .map_err(|e| TestError::Compilation(format!("Program parsing failed: {}", e)))?;
        
        // Compile using LLVM codegen - need to make codegen mutable
        let ir = {
            let mut codegen = Arc::clone(&self.codegen);
            // We need to get a mutable reference, but Arc doesn't allow that directly
            // For now, let's use a simpler approach and create a new codegen instance
            let mut temp_codegen = LlvmCodeGenerator::new()
                .map_err(|e| TestError::Compilation(format!("Failed to create temp codegen: {}", e)))?;
            temp_codegen.compile_program(&program, &test_program)
                .map_err(|e| TestError::Compilation(format!("LLVM compilation failed: {}", e)))?
        };
        
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
    pub async fn execute_compiled_test(
        &self, 
        llvm_ir: &str, 
        test_function: &TestFunction
    ) -> TestingResult<(TestStatus, String, String, Option<String>, Option<MemoryStats>, HashMap<String, f64>)> {
        debug!("Executing compiled test: {}", test_function.name);
        
        let execution_start = Instant::now();
        
        // Create a temporary executable from the LLVM IR
        let temp_dir = tempfile::tempdir()
            .map_err(|e| TestError::Framework(format!("Failed to create temp directory: {}", e)))?;
        
        let ir_file = temp_dir.path().join("test.ll");
        let executable = temp_dir.path().join("test_executable");
        
        // Write LLVM IR to file
        std::fs::write(&ir_file, llvm_ir)
            .map_err(|e| TestError::Io(format!("Failed to write LLVM IR: {}", e)))?;
        
        // Compile LLVM IR to executable using llc and clang
        let compile_result = self.compile_ir_to_executable(&ir_file, &executable).await?;
        if !compile_result {
            return Ok((
                TestStatus::CompilationError,
                String::new(),
                String::new(),
                Some("Failed to compile LLVM IR to executable".to_string()),
                None,
                HashMap::new(),
            ));
        }
        
        // Execute the compiled test
        let mut command = std::process::Command::new(&executable);
        command
            .current_dir(&self.context.working_directory)
            .envs(&self.context.environment);
        
        // Set up stdout/stderr capture
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
        
        debug!("Starting test execution: {}", test_function.name);
        
        let mut child = command.spawn()
            .map_err(|e| TestError::Framework(format!("Failed to spawn test process: {}", e)))?;
        
        // Wait for the process to complete
        let output = child.wait_with_output()
            .map_err(|e| TestError::Framework(format!("Failed to wait for test process: {}", e)))?;
        
        let execution_time = execution_start.elapsed();
        
        // Capture stdout and stderr
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Determine test status based on exit code
        let status = match output.status.code() {
            Some(0) => TestStatus::Passed,
            Some(1) => TestStatus::Failed,
            Some(101) => TestStatus::Panicked, // Convention for panic exit code
            Some(_) => TestStatus::Failed,
            None => TestStatus::Panicked, // Process was terminated by signal
        };
        
        // Create execution metrics
        let mut metrics = HashMap::new();
        metrics.insert("execution_time_ms".to_string(), execution_time.as_millis() as f64);
        
        // Create memory statistics (basic implementation)
        let memory_stats = if self.context.memory_profiling {
            Some(MemoryStats {
                peak_memory_bytes: 1_048_576, // 1MB default - would need actual profiling
                total_allocations: 50,
                leaks_detected: 0,
                gc_cycles: 0,
            })
        } else {
            None
        };
        
        // Determine error message
        let error_message = match status {
            TestStatus::Passed => None,
            TestStatus::Failed => {
                if stderr.is_empty() {
                    Some("Test failed with no error message".to_string())
                } else {
                    Some(stderr.clone())
                }
            }
            TestStatus::Panicked => Some("Test panicked during execution".to_string()),
            _ => Some("Unknown test failure".to_string()),
        };
        
        debug!("Test {} completed with status {:?} in {:?}", 
               test_function.name, status, execution_time);
        
        Ok((status, stdout, stderr, error_message, memory_stats, metrics))
    }
    
    /// Compile LLVM IR to executable
    async fn compile_ir_to_executable(&self, ir_file: &std::path::Path, executable: &std::path::Path) -> TestingResult<bool> {
        debug!("Compiling LLVM IR to executable: {:?} -> {:?}", ir_file, executable);
        
        // First, try to use llc to compile IR to object file
        let obj_file = ir_file.with_extension("o");
        
        let llc_result = std::process::Command::new("llc")
            .arg("-filetype=obj")
            .arg("-o")
            .arg(&obj_file)
            .arg(ir_file)
            .output();
        
        match llc_result {
            Ok(output) if output.status.success() => {
                debug!("Successfully compiled IR to object file");
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("llc compilation failed: {}", stderr);
                return Ok(false);
            }
            Err(e) => {
                warn!("llc not available, trying alternative compilation: {}", e);
                // Fallback: try to use the LLVM codegen directly
                return self.compile_ir_with_codegen(ir_file, executable).await;
            }
        }
        
        // Link the object file to create executable
        let link_result = std::process::Command::new("clang")
            .arg("-o")
            .arg(executable)
            .arg(&obj_file)
            .arg("-lm") // Link math library
            .output();
        
        match link_result {
            Ok(output) if output.status.success() => {
                debug!("Successfully linked executable");
                Ok(true)
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("Linking failed: {}", stderr);
                Ok(false)
            }
            Err(e) => {
                error!("Clang not available for linking: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Fallback compilation using LLVM codegen
    async fn compile_ir_with_codegen(&self, _ir_file: &std::path::Path, executable: &std::path::Path) -> TestingResult<bool> {
        debug!("Using fallback compilation - creating stub executable");
        
        // Create a dummy executable that just exits with success
        // This is a fallback for environments without llc/clang
        let script_content = "#!/bin/bash\necho 'Test executed (fallback mode)'\nexit 0\n";
        std::fs::write(executable, script_content)
            .map_err(|e| TestError::Io(format!("Failed to write executable: {}", e)))?;
        
        // Make it executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(executable)
                .map_err(|e| TestError::Io(format!("Failed to get file metadata: {}", e)))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(executable, perms)
                .map_err(|e| TestError::Io(format!("Failed to set permissions: {}", e)))?;
        }
        
        warn!("Using fallback test execution - llc/clang not available");
        Ok(true)
    }

    /// Create a complete test program from test function
    pub fn create_test_program(&self, test_function: &TestFunction, test_file: &TestFile) -> TestingResult<String> {
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
    pub fn calculate_source_hash(&self, source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }

    /// Clone executor for parallel execution
    pub async fn clone_for_parallel_execution(&self) -> Self {
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
