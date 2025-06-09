//! Bootstrap Compiler Testing Module
//!
//! Comprehensive test suite for the CURSED language bootstrap compiler system.
//! Tests the entire bootstrap pipeline from minimal subset compilation to 
//! full self-hosting capabilities.

pub mod minimal_subset;
pub mod stage2_compiler;
pub mod self_compilation;
pub mod performance_benchmarks;
pub mod regression_tests;
pub mod ci_integration;
pub mod memory_usage;
pub mod utils;

/// Bootstrap test configuration
#[derive(Debug, Clone)]
pub struct BootstrapTestConfig {
    pub stage1_binary: String,
    pub test_data_dir: String,
    pub output_dir: String,
    pub timeout_seconds: u64,
    pub enable_debug: bool,
    pub enable_benchmarks: bool,
}

impl Default for BootstrapTestConfig {
    fn default() -> Self {
        Self {
            stage1_binary: "./target/debug/cursed".to_string(),
            test_data_dir: "./tests/bootstrap/test_data".to_string(),
            output_dir: "./tests/bootstrap/output".to_string(),
            timeout_seconds: 30,
            enable_debug: false,
            enable_benchmarks: true,
        }
    }
}

/// Bootstrap test result metrics
#[derive(Debug, Default)]
pub struct BootstrapTestMetrics {
    pub stage1_compile_time_ms: u64,
    pub stage2_compile_time_ms: u64,
    pub stage3_compile_time_ms: u64,
    pub memory_usage_mb: u64,
    pub binary_size_bytes: u64,
    pub tests_passed: usize,
    pub tests_failed: usize,
}

/// Initialize bootstrap test environment
pub fn init_bootstrap_tests() -> BootstrapTestConfig {
    crate::common::init_tracing();
    
    let config = BootstrapTestConfig::default();
    
    // Create test directories
    std::fs::create_dir_all(&config.test_data_dir).unwrap();
    std::fs::create_dir_all(&config.output_dir).unwrap();
    
    tracing::info!("Bootstrap test environment initialized");
    config
}
