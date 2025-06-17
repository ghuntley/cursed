/// CURSED Test Runner System
/// 
/// Comprehensive testing framework for the CURSED programming language
/// providing test discovery, execution, reporting, and assertion utilities.

pub mod discovery;
pub mod execution;
pub mod framework;
pub mod reporting;
pub mod runner;
pub mod assertions;
pub mod fixtures;

// Re-export main types
pub use runner::{TestRunner, TestRunnerConfig, TestRunnerBuilder};
pub use execution::{TestExecutor, TestStatus, TestExecutionContext};
pub use execution::TestResult as ExecutionTestResult; // Rename to avoid conflict
pub use discovery::{TestDiscovery, TestFile, TestFunction, TestSuite};
pub use framework::{TestFramework, TestContext, TestEnvironment};
pub use reporting::{TestReporter, ReportFormat, TestReport, TestStats};
pub use assertions::{Assert, AssertionError};
pub use assertions::AssertionResult as AssertionsResult; // Rename to avoid conflict  
pub use fixtures::{TestFixture, FixtureManager, TestData};

use crate::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

/// Main result type for test operations
pub type TestResult<T> = Result<T, TestError>;

/// Comprehensive test error types
#[derive(Debug, Clone)]
pub enum TestError {
    /// Test discovery failed
    Discovery(String),
    /// Test execution failed
    Execution(String),
    /// Test compilation failed
    Compilation(String),
    /// Test assertion failed
    Assertion(String),
    /// Test timeout
    Timeout(String),
    /// Resource limit exceeded
    ResourceLimit(String),
    /// Test framework error
    Framework(String),
    /// File I/O error during testing
    Io(String),
    /// Configuration error
    Config(String),
    /// General test error
    General(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Discovery(msg) => write!(f, "Test discovery error: {}", msg),
            TestError::Execution(msg) => write!(f, "Test execution error: {}", msg),
            TestError::Compilation(msg) => write!(f, "Test compilation error: {}", msg),
            TestError::Assertion(msg) => write!(f, "Test assertion error: {}", msg),
            TestError::Timeout(msg) => write!(f, "Test timeout: {}", msg),
            TestError::ResourceLimit(msg) => write!(f, "Resource limit exceeded: {}", msg),
            TestError::Framework(msg) => write!(f, "Test framework error: {}", msg),
            TestError::Io(msg) => write!(f, "Test I/O error: {}", msg),
            TestError::Config(msg) => write!(f, "Test configuration error: {}", msg),
            TestError::General(msg) => write!(f, "Test error: {}", msg),
        }
    }
}

impl std::error::Error for TestError {}

impl From<Error> for TestError {
    fn from(err: Error) -> Self {
        TestError::General(err.to_string())
    }
}

impl From<std::io::Error> for TestError {
    fn from(err: std::io::Error) -> Self {
        TestError::Io(err.to_string())
    }
}

/// Test configuration for filtering and execution
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Test file patterns to include
    pub include_patterns: Vec<String>,
    /// Test file patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Test function name patterns to match
    pub test_patterns: Vec<String>,
    /// Maximum number of parallel test threads
    pub max_parallel_tests: usize,
    /// Test execution timeout in seconds
    pub timeout_seconds: u64,
    /// Enable verbose output
    pub verbose: bool,
    /// Fail fast on first test failure
    pub fail_fast: bool,
    /// Enable test coverage collection
    pub coverage: bool,
    /// Working directory for test execution
    pub working_directory: PathBuf,
    /// Additional environment variables for tests
    pub environment: HashMap<String, String>,
    /// Test data directory
    pub test_data_dir: Option<PathBuf>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            include_patterns: vec!["**/*_test.csd".to_string(), "**/test_*.csd".to_string()],
            exclude_patterns: vec!["target/**".to_string(), ".git/**".to_string()],
            test_patterns: vec![],
            max_parallel_tests: num_cpus::get(),
            timeout_seconds: 30,
            verbose: false,
            fail_fast: false,
            coverage: false,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            environment: HashMap::new(),
            test_data_dir: None,
        }
    }
}

/// Main entry point for running tests
pub async fn run_tests(config: TestConfig) -> TestResult<TestReport> {
    let mut runner = TestRunnerBuilder::new()
        .with_config(config)
        .build()?;
    
    runner.run_all_tests().await
}

/// Run tests in a specific directory
pub async fn run_tests_in_directory(dir: &str, pattern: Option<&str>) -> TestResult<TestReport> {
    let mut config = TestConfig::default();
    config.working_directory = PathBuf::from(dir);
    
    if let Some(pat) = pattern {
        config.test_patterns.push(pat.to_string());
    }
    
    run_tests(config).await
}

/// Run a specific test file
pub async fn run_test_file(file_path: &str) -> TestResult<TestReport> {
    let mut config = TestConfig::default();
    config.include_patterns = Vec::from([file_path.to_string()]);
    
    run_tests(config).await
}
