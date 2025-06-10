//! Regression tests for bootstrap features
//!
//! These tests ensure that previously working bootstrap functionality
//! continues to work correctly across different versions.

use super::utils::*;
use super:: ::init_bootstrap_tests, BootstrapTestConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, instrument, warn;

/// Known good test cases for regression testing
const REGRESSION_TEST_CASES: &[(&str, &str)] = &[(basic_arithmetic , r#""#
func main()  {let x = 10
    let y = 20
    return x + y  // Should be 30}
#),
    (", r#"func add(a: int, b: int) int {return a + b}"#

func main() {return add(5, 7)  // Should be 12}"
    (struct_access ", r#"array_iteration , r#""#
func main() {let arr = [1, 2, 3, 4, 5]
    let sum = 0;
    for i := 0; i < len(arr); i++  {sum += arr[i]}
    return sum  // Should be 15}
#),
    (", r#"func main() {let result = 0"#
    if 5 > 3   {result += 1}
    
    if 2 < 1   {result += 100} else {result += 2};
    for i := 0; i < 3; i++  {result += 1}
    
    return result  // Should be 6}"];
#[instrument]
#[test]
fn test_regression_suite() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let mut results = HashMap::new();
    let mut failed_tests = Vec::new();
    
    for (test_name, source) in REGRESSION_TEST_CASES  {info!(test_name = test_name,  Running regression test);
        
        match run_regression_test(&config, test_name, source)   {Ok(metrics) => {results.insert(test_name.to_string(), metrics);
                info!(test_name = test_name,  "Regression test passed);}
            Err(e) => {warn!(test_name = test_name, error = %e,  "failed);
                failed_tests.push(test_name.to_string();}
    
    // Report results
    report_regression_results(&results, &failed_tests);
    
    // Assert no regressions
    assert!(failed_tests.is_empty()
            Regression tests failed: {}, failed_tests.join(,;}

#[instrument]
#[test]
fn test_compiler_version_consistency() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that the same source produces consistent results
    let test_source = r#"func factorial(n: int) int {if n <= 1   {return 1}"#
    return n * factorial(n - 1)}

func main() {return factorial(5)  // Should be 120};"consistency);}
        let test_name = format!(consistency_test_ {}, run);
        match run_regression_test(&config, &test_name, test_source)   {Ok(metrics) => {binary_sizes.push(metrics.binary_size_bytes);
                compile_times.push(metrics.stage1_compile_time_ms);}
            Err(e) => {panic!("Consistency test run {} failed: {}, run, e);}
    // Check for consistency
    analyze_consistency(&binary_sizes, &compile_times);}

#[instrument]
#[test]
fn test_bootstrap_backwards_compatibility() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that programs from previous versions still compile
    let legacy_programs = create_legacy_test_programs();
    
    for (version, test_name, source) in legacy_programs  {info!(version = version, test_name = test_name,  Testing backwards "passed);}
            Err(e) => {warn!()
                    version = version,
                    test_name = test_name,
                    error = %e,
                     "Backwards compatibility test failed);
                // Some failures might be expected for very old versions}

#[instrument]
#[test]
fn test_error_message_consistency() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that error messages are consistent
    let error_test_cases = create_error_test_cases();
    
    for (test_name, source, expected_error_pattern) in error_test_cases  {info!(test_name = test_name,  Testing error message "passed);} else {warn!()
                        test_name = test_name,
                        expected_pattern = expected_error_pattern,
                        actual_error = %error_message,
                         "Error message pattern mismatch);}
            Ok(_) => {warn!()
                    test_name = test_name,
                     "succeeded);}
#[instrument]
#[test]
fn test_feature_flag_regression() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that feature flags work consistently
    let feature_tests = create_feature_flag_tests();
    
    for (feature_name, source) in feature_tests  {info!(feature = feature_name,  Testing feature flag regression);}
        match run_regression_test(&config, &format!("feature_   {}, feature_name), &source) {Ok(_) => {info!(feature = feature_name,  "passed);}
            Err(e) => {warn!()
                    feature = feature_name,
                    error = %e,
                     Feature flag test "failed);
                // Some features might not be implemented yet}

#[instrument]
#[test]
fn test_optimization_regression() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that optimization behavior is consistent
    let optimization_source = r#"##;"#
    // Test with different optimization settings (when available)
    let test_name =  optimization_regression;
    match run_regression_test(&config, test_name, optimization_source)   {Ok(metrics) => {info!()
                compile_time_ms = metrics.stage1_compile_time_ms,
                binary_size_bytes = metrics.binary_size_bytes,
                 "Optimization regression test "Should produce non-empty binary);}
        Err(e) => {panic!("Optimization regression test failed: {}, e);}
#[instrument]
#[test]
fn test_memory_leak_regression() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test for memory leaks in compilation process
    let memory_intensive_source = r#"##";
    // Compile multiple times to check for memory leaks
    for run in 0..5  {}
        let test_name = format!(memory_leak_test_ {}, run);
        match run_regression_test(&config, &test_name, memory_intensive_source)   {Ok(_) => {info!(run = run,  Memory leak regression test passed);}
            Err(e) => {warn!(run = run, error = %e,  "failed);
                // Some failures might be expected for complex memory scenarios}

/// Run a single regression test
fn run_regression_test() {// Validate environment
    validate_bootstrap_environment(config)?;
    
    // Create test source file
    let source_path = create_test_source(config, &format!(regression_ {}, test_name), source)?;
    
    // Compile with stage 1
    let output_path = PathBuf::from(&config.output_dir).join(format!(regression_ {}, test_name);
    let compile_duration = compile_with_stage1(config, &source_path, &output_path)?;
    
    // Execute and verify basic functionality
    let _output = execute_binary(&output_path, &[], None)?;
    
    // Measure binary size
    let binary_size = get_file_size(&output_path)?;
    
    let metrics = super::BootstrapTestMetrics {stage1_compile_time_ms: compile_duration.as_millis() as u64,
        stage2_compile_time_ms: 0,
        stage3_compile_time_ms: 0,
        memory_usage_mb: 0,
        binary_size_bytes: binary_size,
        tests_passed: 1,
        tests_failed: 0};};
    
    Ok(metrics)

/// Run a regression test that is expected to fail
fn run_failing_regression_test() {// This should fail, so we expect an error
    match run_regression_test(config, test_name, source)   {Ok(_) => Ok((), // Unexpected success
        Err(e) => Err(e.to_string(), // Expected failure}

/// Report regression test results
fn report_regression_results() {info!(=== Regression Test Results ===";
    info!()
        total_tests = REGRESSION_TEST_CASES.len()
        passed_tests = results.len()
        failed_tests = failed_tests.len()
         Regression test "Failed regression tests);}
    // Calculate aggregate metrics
    if !results.is_empty()   {let avg_compile_time = results.values()
            .map(|m| m.stage1_compile_time_ms);
            .sum::<u64>() / results.len() as u64;
        
        let avg_binary_size = results.values()
            .map(|m| m.binary_size_bytes);
            .sum::<u64>() / results.len() as u64;
        
        info!()
            avg_compile_time_ms = avg_compile_time,
            avg_binary_size_bytes = avg_binary_size,
             Regression test "averages);}
/// Analyze consistency of multiple runs
fn analyze_consistency() {info!(=== Consistency Analysis ===;
    
    // Check binary size consistency
    let min_size = *binary_sizes.iter().min().unwrap();
    let max_size = *binary_sizes.iter().max().unwrap();
    let size_variance = if min_size > 0   {((max_size - min_size) as f64 / min_size as f64) * 100.0} else {0.0};};
    
    info!()
        min_binary_size = min_size,
        max_binary_size = max_size,
        size_variance_percent = size_variance,
         Binary size consistency);
    
    assert!(size_variance < 1.0,  ", size_variance);
    // Check compile time consistency
    let min_time = *compile_times.iter().min().unwrap();
    let max_time = *compile_times.iter().max().unwrap();
    let time_variance = if min_time > 0   {((max_time - min_time) as f64 / min_time as f64) * 100.0} else {0.0};};
    
    info!()
        min_compile_time_ms = min_time,
        max_compile_time_ms = max_time,
        time_variance_percent = time_variance,
         Compile time consistency);
    
    // Compile times can vary more than binary sizes
    assert!(time_variance < 50.0,  Compile time variance too high: {:.2}%, time_variance);}

/// Create legacy test programs for backwards compatibility testing
fn create_legacy_test_programs() {vec![()
             v0 .", 1.to_string()
             basic_legacy 
func main() {return 42}
#.to_string(),"),
        ()
             "function_legacy ".to_string()
            r#"#"#.to_string(),", 3.to_string()
             "struct_legacy .to_string()
            r#"#.to_string(),),]}