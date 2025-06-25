use crate::error::CursedError;
/// CursedError types for the profiler subsystem
use std::fmt;

/// Result type for profiler operations
pub type ProfilerResult<T> = std::result::Result<T, ProfilerError>;

/// Profiler-specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum ProfilerError {
    /// Profiler is not initialized
    /// Profiler is already running
    /// Profiler is not running
    /// Invalid configuration
    /// Sampling failed
    /// Memory allocation failed
    /// I/O error during profiling
    /// Serialization error
    /// Runtime integration error
    /// Benchmark error
    /// Metrics collection error
    /// General profiler error
// impl fmt::Display for ProfilerError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ProfilerError::NotInitialized => write!(f, "Profiler is not initialized"),
//             ProfilerError::AlreadyRunning => write!(f, "Profiler is already running"),
//             ProfilerError::NotRunning => write!(f, "Profiler is not running"),
//             ProfilerError::InvalidConfig(msg) => write!(f, "Invalid profiler configuration: {}", msg),
//             ProfilerError::SamplingFailed(msg) => write!(f, "Profiling sampling failed: {}", msg),
//             ProfilerError::AllocationFailed => write!(f, "Memory allocation failed during profiling"),
//             ProfilerError::IoError(msg) => write!(f, "I/O error during profiling: {}", msg),
//             ProfilerError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
//             ProfilerError::RuntimeError(msg) => write!(f, "Runtime integration error: {}", msg),
//             ProfilerError::BenchmarkError(msg) => write!(f, "Benchmark error: {}", msg),
//             ProfilerError::MetricsError(msg) => write!(f, "Metrics collection error: {}", msg),
//             ProfilerError::General(msg) => write!(f, "Profiler error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ProfilerError {}
// 
// impl From<std::io::Error> for ProfilerError {
//     fn from(err: std::io::Error) -> Self {
//         ProfilerError::IoError(err.to_string())
//     }
// }

// impl From<serde_json::Error> for ProfilerError {
//     fn from(err: serde_json::Error) -> Self {
//         ProfilerError::SerializationError(err.to_string())
//     }
// }

/// Create a profiler error with a custom message
pub fn profiler_error(msg: &str) -> ProfilerError {
    ProfilerError::General(msg.to_string())
/// Create an invalid configuration error
pub fn invalid_config_error(msg: &str) -> ProfilerError {
    ProfilerError::InvalidConfig(msg.to_string())
/// Create a sampling failed error
pub fn sampling_failed_error(msg: &str) -> ProfilerError {
    ProfilerError::SamplingFailed(msg.to_string())
/// Create a runtime integration error
pub fn runtime_error(msg: &str) -> ProfilerError {
    ProfilerError::RuntimeError(msg.to_string())
/// Create a benchmark error
pub fn benchmark_error(msg: &str) -> ProfilerError {
    ProfilerError::BenchmarkError(msg.to_string())
/// Create a metrics error
pub fn metrics_error(msg: &str) -> ProfilerError {
    ProfilerError::MetricsError(msg.to_string())
