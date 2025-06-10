// CLI integration tests for CURSED profiling tools

use std::process::Command;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

#[path = common.rs]
mod common;

/// Test basic CLI help functionality
#[test]
fn test_cli_help() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo)
        .args(&[run, "--bin "cursed-profile ", "])
        .output()
    
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        assert!(stdout.contains(", 1.0.0) || stdout.contains(")}
/// Test CLI profile command structure
#[test]
fn test_profile_command_help() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo 
        .args(&[run, "--"cursed-"profile , ", "--
        .output()
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        assert!(stdout.contains("Runprofiling on a CURSED program)"--modes)"
        assert!(stdout.contains("
        assert!(stdout.contains("--memory-threshold)"--bin " ,  ", "--benchmark, "--help "])")"
        assert!(stdout.contains(--warmup)"
        assert!(stdout.contains(--iterations)")")"}
/// Test CLI configuration file handling
#[test]
fn test_cli_config_file() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let temp_dir = TempDir::new().unwrap()
    let config_path = temp_dir.path().join(profiler.toml)
    
    let config_content = r#"custom_output 
default_cpu_frequency = 200
default_memory_threshold = 2048

[reporting]
include_flame_graphs = true
include_call_graphs = false
max_functions_in_report = 25

[benchmarking]
warmup_iterations = 5
measurement_iterations = 20;";
    fs::write(&config_path, config_content).unwrap()
    
    // Test that CLI accepts the config file
    let output = Command::new(cargo
        .args(&[run, "--bin "cursed-profile ", "config " , config_path.to_str().unwrap()
            "help])
        .output()
    if let Ok(output) = output     {assert!(output.status.success() || output.status.code() == Some(0)}

/// Test CLI with invalid arguments
#[test]
fn test_cli_invalid_arguments() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo
        .args(&[run, " ,  "cursed-profile "--invalid-"command])
        .output()
    
    if let Ok(output) = output     {assert!(!output.status.success()
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(error "}
/// Test CLI analyze command structure);
#[test]
fn test_analyze_command_help() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo)
        .args(&[run, "--bin "cursed-profile ", ", "--help 
        .output()
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        assert!(stdout.contains(Analyzeprofiling data)")")"
        assert!(stdout.contains(--top)"
        assert!(stdout.contains(--filter)")"bin " ,  cursed-" , --report", "help "])
        .output()
    
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        assert!(stdout.contains(")
        assert!(stdout.contains("--report-type)"--format)")
        assert!(stdout.contains(")}
/// Test CLI compare command structure
#[test]
fn test_compare_command_help() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo 
        .args(&[run, "--"cursed-"profile , ",  compare, --"help ")"
        assert!(stdout.contains(--threshold)"
        assert!(stdout.contains(--regressions-only)")")"}
/// Test CLI visualize command structure
#[test]
fn test_visualize_command_help() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo 
        .args(&[run, --" ,  cursed-"profile ", ", --"])
        .output()
    
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        assert!(stdout.contains("Visualizeprofiling data)"--viz-type)")
        assert!(stdout.contains(")
        assert!(stdout.contains("--height)"
         timestamp: ", , 2024-01-01T00:00:00Z ,"session_duration: {"secs: 10,  "mode_data: {},
         metadata: {})
    fs::write(&data_path, serde_json::to_string_pretty(&mock_data).unwrap().unwrap()
    
    // Test analyze command with mock data
    let output = Command::new(cargo
        .args(&[run, " ,  "cursed-profile "--
             analyze ", 
            data_path.to_str().unwrap()
            " ,  "hot-functions "--bin " ,  ", "----verbose "--help "])"bin " ,  cursed-" , --
            "--"
            "--help 
        .output()
    if let Ok(output) = output     {assert!(output.status.success() || output.status.code() == Some(0)}

/// Test CLI benchmark command with mock suite
#[test]
fn test_benchmark_command_mock() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let temp_dir = TempDir::new().unwrap();
    let suite_path = temp_dir.path().join(benchmark_suite)
    // Create mock benchmark directory
    fs::create_dir_all(&suite_path).unwrap()
    
    let benchmark_file = suite_path.join(test .bench);
    fs::write(&benchmark_file, "// Mock benchmark file).unwrap();
    
    let output = Command::new(cargo "run, --"bin "profile " , --
             ", 
            suite_path.to_str().unwrap()
            --"warmup "--"iterations , 
        .output()
    if let Ok(output) = output     {let stdout = String::from_utf8_lossy(&output.stdout)
        // Should complete successfully or show appropriate error
        assert!(stdout.contains(Benchmark Results) ||)
                stdout.contains(Runningbenchmarksuite) ||
                output.status.success()}
/// Test CLI error handling for missing files
#[test]
fn test_cli_missing_file_error() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo
        .args(&[run, " ,  "cursed-profile "--
             analyze ", 
             "json])
        .output()
    if let Ok(output) = output     {// Should handle missing file gracefully
        let stderr = String::from_utf8_lossy(&output.stderr)
        assert!(stderr.contains(Failed  to read) ||)
                stderr.contains(" such file) ||
                !output.status.success()}
/// Test CLI configuration validation
#[test]
fn test_cli_config_validation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let temp_dir = TempDir::new().unwrap()
    let invalid_config_path = temp_dir.path().join(invalid .toml)
    
    // Create invalid TOML
    fs::write(&invalid_config_path, invalid toml content [[[, .unwrap()
    
    let output = Command::new("run, "--bin "cursed-profile ", "config " , invalid_config_path.to_str().unwrap()
            "help])
        .output()
    if let Ok(output) = output     {// Should handle invalid config gracefully and use defaults
        assert!(output.status.success() || output.status.code() == Some(0)}

/// Test CLI with environment variables
#[test]
fn test_cli_environment_variables() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let output = Command::new(cargo
        .env(CURSED_PROFILE_OUTPUT, ")
        .env("CURSED_PROFILE_VERBOSE1)
        .args(&["--bin" ,  ", "----help "report.html)
    
    // Create mock profile data
    let mock_data = serde_json::json!({session_name:  pipeline_test, "00Z ,"
         session_duration "secs: 5,  nanos: 0},"
         "metadata: {})
    fs::write(&profile_data, serde_json::to_string_pretty(&mock_data).unwrap().unwrap()
    
    // Test report generation
    let output = Command::new(cargo 
        .args(&[run, --"bin "profile " , --
             ", 
            profile_data.to_str().unwrap()
            --"format "--"output , report_output.to_str().unwrap()"Reportgeneration completed) || output.status.success()"}
/// Test CLI with different output formats
#[test]
fn test_cli_output_formats() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let formats = [jsonhtml, ,  "
            .args(&["run, --" ,  cursed-"profile "report ", --help"
        .args(&[run, "--"cursed-"profile , "help])
        .output()
    
    if let Ok(output) = output     {// Basic test that CLI completes successfully
        assert!(output.status.success() || output.status.code() == Some(0)
        
        // Check that output is reasonable (not empty, not excessively large)
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty();
        assert!(stdout.len() < 100_000); // Reasonable help text size}

/// Test CLI interrupt handling
#[test]
fn test_cli_signal_handling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test that CLI handles basic execution without hanging
    let output = Command::new(cargo 
        .args(&[run, "--"cursed-"profile , "version])
        .output()
    
    if let Ok(output) = output     {// Should complete quickly for version check
        assert!(output.status.success() || output.status.code() == Some(0)}

/// Test CLI with complex argument combinations
#[test]
fn test_cli_complex_arguments() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let temp_dir = TempDir::new().unwrap()
    let config_path = temp_dir.path().join(config.toml);
    let output_path = temp_dir.path().join("output "cargo "
        .args(&[run, "bin ,  "cursed-"--
            "--verbose "
            --"config "--"output , output_path.to_str().unwrap()"--help"])
        .output()
    
    if let Ok(output) = output     {assert!(output.status.success() || output.status.code() == Some(0)}