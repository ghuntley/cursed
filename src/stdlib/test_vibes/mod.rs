/// TestVibes - CURSED Testing Framework
/// 
/// A vibrant testing framework inspired by Go's testing package but with
/// Gen Z flavor and modern development practices for the CURSED language.
/// 
/// ## Features
/// - Expressive test and benchmark types (VibeTest, VibeBench)
/// - Comprehensive assertion framework with meaningful error messages
/// - Test fixtures and table-driven tests
/// - Mocking framework with expectations
/// - Parallel test execution support
/// - Benchmarking utilities with performance metrics
/// - Test utilities for temporary resources and random data

pub mod core;
pub mod assertions;
pub mod fixtures;
pub mod mocking;
pub mod table_driven;
pub mod benchmarking;
pub mod utilities;

// Core types re-exports
pub use core::{
    VibeTest, VibeBench, VibeTestingManager, TestMain,
    VibeTestState, VibeBenchState, TestResult, BenchmarkResult
};

// Assertion framework re-exports
pub use assertions::{
    // Basic assertions
    Assert, AssertEqual, AssertNotEqual, AssertNil, AssertNotNil,
    AssertTrue, AssertFalse,
    
    // Error assertions
    AssertError, AssertNoError, AssertErrorIs, AssertErrorContains,
    
    // Collection assertions
    AssertLen, AssertEmpty, AssertNotEmpty, AssertContains, AssertNotContains,
    
    // Numeric assertions
    AssertGreater, AssertGreaterOrEqual, AssertLess, AssertLessOrEqual,
    AssertZero, AssertNotZero,
    
    // String assertions
    AssertContainsSubtea, AssertHasPrefix, AssertHasSuffix, AssertMatchesRegex,
    
    // Type assertions
    AssertType, AssertImplements,
    
    // Shook (panic) assertions
    AssertShooks, AssertShooksWithValue, AssertNoShook
};

// Test fixtures
pub use fixtures::{
    FixtureVibe, NewFixtureVibe
};

// Table-driven tests
pub use table_driven::{
    TestCase, RunTestCases
};

// Mocking framework
pub use mocking::{
    MockVibe, Expectation, Stub
};

// Test utilities
pub use utilities::{
    TempFile, TempDir, Parallel, WithDeadline, WithSetup,
    RandomString, RandomInt, RandomFloat, RandomBytes
};

// Benchmarking utilities
pub use benchmarking::{
    Benchmark, BenchmarkMemory, BenchmarkParallel
};

// Error handling
use crate::stdlib::errors_simple::CursedError;

/// Result type for TestVibes operations
pub type TestVibesResult<T> = Result<T, CursedError>;

/// Error types specific to TestVibes
#[derive(Debug, Clone)]
pub enum TestVibesError {
    /// Test execution failed
    TestFailed(String),
    /// Test skipped
    TestSkipped(String),
    /// Benchmark failed
    BenchmarkFailed(String),
    /// Assertion failed
    AssertionFailed(String),
    /// Mock expectation not met
    ExpectationNotMet(String),
    /// Fixture setup/teardown failed
    FixtureFailed(String),
    /// Timeout exceeded
    TimeoutExceeded(String),
    /// Invalid test configuration
    InvalidConfig(String),
}

impl std::fmt::Display for TestVibesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestVibesError::TestFailed(msg) => write!(f, "Test failed: {}", msg),
            TestVibesError::TestSkipped(msg) => write!(f, "Test skipped: {}", msg),
            TestVibesError::BenchmarkFailed(msg) => write!(f, "Benchmark failed: {}", msg),
            TestVibesError::AssertionFailed(msg) => write!(f, "Assertion failed: {}", msg),
            TestVibesError::ExpectationNotMet(msg) => write!(f, "Mock expectation not met: {}", msg),
            TestVibesError::FixtureFailed(msg) => write!(f, "Fixture failed: {}", msg),
            TestVibesError::TimeoutExceeded(msg) => write!(f, "Timeout exceeded: {}", msg),
            TestVibesError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for TestVibesError {}

impl From<TestVibesError> for CursedError {
    fn from(err: TestVibesError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

// Helper functions for creating errors
pub fn test_failed(message: &str) -> TestVibesError {
    TestVibesError::TestFailed(message.to_string())
}

pub fn test_skipped(message: &str) -> TestVibesError {
    TestVibesError::TestSkipped(message.to_string())
}

pub fn assertion_failed(message: &str) -> TestVibesError {
    TestVibesError::AssertionFailed(message.to_string())
}

pub fn expectation_not_met(message: &str) -> TestVibesError {
    TestVibesError::ExpectationNotMet(message.to_string())
}

pub fn timeout_exceeded(message: &str) -> TestVibesError {
    TestVibesError::TimeoutExceeded(message.to_string())
}
