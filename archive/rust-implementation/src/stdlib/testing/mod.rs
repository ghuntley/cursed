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
// };

// Test discovery and execution
pub use discovery::{
    TestDiscovery, TestFilter, DiscoveryConfig, TestInfo, TestMetadata
// };
pub use executor::{
    ExecutionContext, TestTimeout, ParallelExecutor, SequentialExecutor
// };

// Test runner and reporting
pub use runner::{
    TestRunner, TestRunnerConfig, RunnerResult, TestSuite, TestSuiteResult
// };
pub use reporting::{
    ConsoleReporter, JsonReporter, XmlReporter, HtmlReporter
// };

// Statistics and performance measurement
pub use stats::{
    ExecutionStats, TestBenchmark
// };

// Memory stats from executor module
pub use executor::MemoryStats;

// Assertion framework
pub use assertions::{
    // Basic assertions
    
    // Numeric assertions
    
    // String assertions
    
    // Collection assertions
    
    // CursedError assertions
    
    // Advanced assertions
    
    // Assertion result types
    AssertionResult, AssertionError, AssertionContext
// };

// Test attributes and metadata
pub use attributes::{
    // TestTimeout as AttributeTimeout, TestSetup, TestTeardown,
    parse_test_attributes, validate_test_attributes
// };

// Test macros (re-exported for convenience)
pub use macros::{
    setup_function, teardown_function, test_suite_macro
// };

// CursedError handling for testing framework
use crate::error::CursedError;

/// Result type for testing operations
pub type TestFrameworkResult<T> = std::result::Result<T, TestError>;

/// CursedError types specific to the testing framework
#[derive(Debug, Clone)]
pub enum TestError {
    /// Test discovery failed
    /// Test execution failed
    /// Assertion failed
    /// Test timeout
    /// Configuration error
    /// Report generation error
    /// General testing framework error
// impl std::fmt::Display for TestError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TestError::DiscoveryError(msg) => write!(f, "Test discovery error: {}", msg),
//             TestError::ExecutionError(msg) => write!(f, "Test execution error: {}", msg),
//             TestError::AssertionError(msg) => write!(f, "Assertion failed: {}", msg),
//             TestError::TimeoutError(msg) => write!(f, "Test timeout: {}", msg),
//             TestError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
//             TestError::ReportError(msg) => write!(f, "Report generation error: {}", msg),
//             TestError::FrameworkError(msg) => write!(f, "Framework error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for TestError {}
// 
/// Convert TestError to CursedError
// impl From<TestError> for CursedError {
//     fn from(err: TestError) -> Self {
//         CursedError::Runtime(err.to_string())
//     }
// }

/// Helper function to create test discovery errors
pub fn discovery_error(message: &str) -> TestError {
    TestError::DiscoveryError(message.to_string())
/// Helper function to create test execution errors
pub fn execution_error(message: &str) -> TestError {
    TestError::ExecutionError(message.to_string())
/// Helper function to create assertion errors
pub fn assertion_error(message: &str) -> TestError {
    TestError::AssertionError(message.to_string())
/// Helper function to create timeout errors
pub fn timeout_error(message: &str) -> TestError {
    TestError::TimeoutError(message.to_string())
/// Helper function to create configuration errors
pub fn config_error(message: &str) -> TestError {
    TestError::ConfigError(message.to_string())
/// Helper function to create report generation errors
pub fn report_error(message: &str) -> TestError {
    TestError::ReportError(message.to_string())
/// Helper function to create general framework errors
pub fn framework_error(message: &str) -> TestError {
    TestError::FrameworkError(message.to_string())
}
