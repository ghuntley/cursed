/// fr fr TestVibes testing framework for CURSED language
/// Provides comprehensive testing and benchmarking capabilities with Gen Z energy

// Core modules
pub mod core;
pub mod assertions;
pub mod fixtures;
pub mod mocking;
pub mod benchmarks;
pub mod utilities;
pub mod matchers;
pub mod runners;

// Re-export core types for easy access
pub use core::{VibeTest, VibeBench, VibeTestingManager, TestResult, BenchResult};
pub use assertions::*;
pub use fixtures::{FixtureVibe, TestCase};
pub use mocking::{MockVibe, Expectation, Stub};
pub use benchmarks::{Benchmark, BenchmarkMemory, BenchmarkParallel};
pub use utilities::{TempFile, TempDir, parallel, with_deadline, with_setup, random_string, random_int, random_float, random_bytes};
pub use matchers::*;
pub use runners::{test_main, TestRunner};

/// fr fr Initialize the test_vibes package and register it with stdlib
pub fn init_test_vibes() {
        // TODO: implement
    }
    // Register built-in testing functions with the dot registry
    register_builtin_functions();
/// fr fr Register built-in testing functions with the CURSED stdlib
fn register_builtin_functions() {
        // TODO: implement
    }
    // This will integrate with the existing dot registry system
    // Common testing functions that would be available globally:
    // - test_vibes.run() - run tests
    // - test_vibes.bench() - run benchmarks
    // - test_vibes.assert() - basic assertion
    // - test_vibes.mock() - create mock object
    
    println!("🧪 test_vibes package initialized - ready to test with good vibes!");
/// fr fr Quick test setup for common use cases
pub fn quick_test(name: &str) -> VibeTest {
    VibeTest::new(name.to_string())
/// fr fr Quick benchmark setup
pub fn quick_bench(name: &str) -> VibeBench {
    VibeBench::new(name.to_string())
/// fr fr Create test suite with multiple tests
pub fn test_suite(name: &str, tests: Vec<fn(&mut VibeTest)>) -> TestResult {
    let mut suite_result = TestResult::new();
    
    for (i, test_fn) in tests.iter().enumerate() {
        let mut test = VibeTest::new(format!("{}_{}", name, i));
        test_fn(&mut test);
        suite_result.add_test_result(test.get_result());
    suite_result
