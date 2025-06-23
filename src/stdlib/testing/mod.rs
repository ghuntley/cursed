/// Comprehensive unit testing framework for the CURSED programming language
/// 
/// This module provides a complete testing infrastructure including:
/// - Test discovery and execution
/// - Comprehensive assertion framework
/// - Test attributes and macros
/// - Test runner with detailed reporting
/// - Performance measurement and statistics

pub mod attributes;
pub mod assertions;
pub mod discovery;
pub mod executor;
pub mod macros;
pub mod reporting;
pub mod runner;
pub mod stats;
pub mod framework;

// Core testing infrastructure re-exports
pub use framework::{
    TestFramework, TestFrameworkConfig, TestExecutionMode, TestFilterMode
};

// Test discovery and execution
pub use discovery::{
    TestDiscovery, TestFilter, DiscoveryConfig, TestInfo, TestMetadata
};
pub use executor::{
    TestExecutor, TestExecutorConfig, TestResult, TestStatus, TestFailure,
    ExecutionContext, TestTimeout, ParallelExecutor, SequentialExecutor
};

// Test runner and reporting
pub use runner::{
    TestRunner, TestRunnerConfig, RunnerResult, TestSuite, TestSuiteResult
};
pub use reporting::{
    TestReporter, ReportFormat, ReportConfig, TestReport, SummaryReport,
    ConsoleReporter, JsonReporter, XmlReporter, HtmlReporter
};

// Statistics and performance measurement
pub use stats::{
    TestStatistics, TestTiming, PerformanceStats, TestMetrics,
    ExecutionStats, TestBenchmark
};

// Memory stats from executor module
pub use executor::MemoryStats;

// Assertion framework
pub use assertions::{
    // Basic assertions
    assert_true, assert_false, assert_eq, assert_ne, assert_null, assert_not_null,
    
    // Numeric assertions
    assert_greater, assert_greater_equal, assert_less, assert_less_equal,
    assert_close_to, assert_between, assert_positive, assert_negative, assert_zero,
    
    // String assertions
    assert_contains, assert_not_contains, assert_starts_with, assert_ends_with,
    assert_matches_regex, assert_empty_string, assert_length,
    
    // Collection assertions
    assert_empty, assert_not_empty, assert_contains_element, assert_not_contains_element,
    assert_has_length, assert_all_true, assert_any_true, assert_none_true,
    
    // Error assertions
    assert_error, assert_no_error, assert_error_type, assert_error_message,
    assert_panic, assert_no_panic,
    
    // Advanced assertions
    assert_eventually, assert_within_timeout, assert_file_exists, assert_file_content,
    
    // Assertion result types
    AssertionResult, AssertionError, AssertionContext
};

// Test attributes and metadata
pub use attributes::{
    TestAttribute, TestAttributes, TestIgnore, TestExpectedPanic,
    // TestTimeout as AttributeTimeout, TestSetup, TestTeardown,
    parse_test_attributes, validate_test_attributes
};

// Test macros (re-exported for convenience)
pub use macros::{
    test_function, ignore_test, should_panic_test, timeout_test,
    setup_function, teardown_function, test_suite_macro
};

// Error handling for testing framework
use crate::crate::stdlib::errors_simple::CursedError;

/// Result type for testing operations
pub type TestFrameworkResult<(), Error>;

/// Error types specific to the testing framework
#[derive(Debug, Clone)]
pub enum TestError {
    /// Test discovery failed
    DiscoveryError(String),
    /// Test execution failed
    ExecutionError(String),
    /// Assertion failed
    AssertionError(String),
    /// Test timeout
    TimeoutError(String),
    /// Configuration error
    ConfigError(String),
    /// Report generation error
    ReportError(String),
    /// General testing framework error
    FrameworkError(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::DiscoveryError(msg) => write!(f, "Test discovery error: {}", msg),
            TestError::ExecutionError(msg) => write!(f, "Test execution error: {}", msg),
            TestError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
            TestError::TimeoutError(msg) => write!(f, "Test timeout: {}", msg),
            TestError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TestError::ReportError(msg) => write!(f, "Report generation error: {}", msg),
            TestError::FrameworkError(msg) => write!(f, "Framework error: {}", msg),
        }
    }
}

impl std::error::Error for TestError {}

/// Convert TestError to CursedError
impl From<TestError> for CursedError {
    fn from(err: TestError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

/// Helper function to create test discovery errors
pub fn discovery_error(message: &str) -> TestError {
    TestError::DiscoveryError(message.to_string())
}

/// Helper function to create test execution errors
pub fn execution_error(message: &str) -> TestError {
    TestError::ExecutionError(message.to_string())
}

/// Helper function to create assertion errors
pub fn assertion_error(message: &str) -> TestError {
    TestError::AssertionError(message.to_string())
}

/// Helper function to create timeout errors
pub fn timeout_error(message: &str) -> TestError {
    TestError::TimeoutError(message.to_string())
}

/// Helper function to create configuration errors
pub fn config_error(message: &str) -> TestError {
    TestError::ConfigError(message.to_string())
}

/// Helper function to create report generation errors
pub fn report_error(message: &str) -> TestError {
    TestError::ReportError(message.to_string())
}

/// Helper function to create general framework errors
pub fn framework_error(message: &str) -> TestError {
    TestError::FrameworkError(message.to_string())
}
