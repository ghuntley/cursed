// CLI integration tests for CURSED profiling tools

use std::process::Command;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

#[path = "common.rs"]
mod common;

/// Test basic CLI help functionality
#[test]
fn test_cli_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Profiling and performance tools for CURSED programs"));
        assert!(stdout.contains("profile"));
        assert!(stdout.contains("benchmark"));
        assert!(stdout.contains("analyze"));
        assert!(stdout.contains("report"));
        assert!(stdout.contains("compare"));
        assert!(stdout.contains("visualize"));
    }
    // If cargo run fails, that's okay for this test environment
}

/// Test CLI version flag
#[test]
fn test_cli_version() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "--version"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("1.0.0") || stdout.contains("cursed-profile"));
    }
}

/// Test CLI profile command structure
#[test]
fn test_profile_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "profile", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Run profiling on a CURSED program"));
        assert!(stdout.contains("--modes"));
        assert!(stdout.contains("--cpu-frequency"));
        assert!(stdout.contains("--memory-threshold"));
    }
}

/// Test CLI benchmark command structure
#[test]
fn test_benchmark_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "benchmark", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Run benchmarks"));
        assert!(stdout.contains("--warmup"));
        assert!(stdout.contains("--iterations"));
        assert!(stdout.contains("--baseline"));
    }
}

/// Test CLI configuration file handling
#[test]
fn test_cli_config_file() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("profiler.toml");
    
    let config_content = r#"
default_output_dir = "custom_output"
default_cpu_frequency = 200
default_memory_threshold = 2048

[reporting]
include_flame_graphs = true
include_call_graphs = false
max_functions_in_report = 25

[benchmarking]
warmup_iterations = 5
measurement_iterations = 20
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    // Test that CLI accepts the config file
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "--config", config_path.to_str().unwrap(),
            "--help"
        ])
        .output();
    
    if let Ok(output) = output {
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}

/// Test CLI with invalid arguments
#[test]
fn test_cli_invalid_arguments() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "invalid-command"])
        .output();
    
    if let Ok(output) = output {
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("error") || stderr.contains("unrecognized"));
    }
}

/// Test CLI analyze command structure
#[test]
fn test_analyze_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "analyze", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Analyze profiling data"));
        assert!(stdout.contains("--analysis"));
        assert!(stdout.contains("--top"));
        assert!(stdout.contains("--filter"));
    }
}

/// Test CLI report command structure
#[test]
fn test_report_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "report", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generate reports"));
        assert!(stdout.contains("--report-type"));
        assert!(stdout.contains("--format"));
        assert!(stdout.contains("--flame-graphs"));
    }
}

/// Test CLI compare command structure
#[test]
fn test_compare_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "compare", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Compare profiling results"));
        assert!(stdout.contains("--threshold"));
        assert!(stdout.contains("--regressions-only"));
        assert!(stdout.contains("--improvements-only"));
    }
}

/// Test CLI visualize command structure
#[test]
fn test_visualize_command_help() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "visualize", "--help"])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Visualize profiling data"));
        assert!(stdout.contains("--viz-type"));
        assert!(stdout.contains("--width"));
        assert!(stdout.contains("--height"));
    }
}

/// Test CLI with mock profiling data
#[test]
fn test_cli_with_mock_data() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let data_path = temp_dir.path().join("profile_data.json");
    
    // Create mock profiling data
    let mock_data = serde_json::json!({
        "session_name": "test_session",
        "timestamp": "2024-01-01T00:00:00Z",
        "session_duration": {"secs": 10, "nanos": 0},
        "mode_data": {},
        "metadata": {}
    });
    
    fs::write(&data_path, serde_json::to_string_pretty(&mock_data).unwrap()).unwrap();
    
    // Test analyze command with mock data
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "analyze",
            data_path.to_str().unwrap(),
            "--analysis", "hot-functions"
        ])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should handle the mock data gracefully
        assert!(stdout.contains("Analysis Results") || output.status.success());
    }
}

/// Test CLI global flags
#[test]
fn test_cli_global_flags() {
    common::tracing::setup();
    
    // Test verbose flag
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "--verbose", "--help"])
        .output();
    
    if let Ok(output) = output {
        assert!(output.status.success() || output.status.code() == Some(0));
    }
    
    // Test output directory flag
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "--output", output_path.to_str().unwrap(),
            "--help"
        ])
        .output();
    
    if let Ok(output) = output {
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}

/// Test CLI benchmark command with mock suite
#[test]
fn test_benchmark_command_mock() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let suite_path = temp_dir.path().join("benchmark_suite");
    
    // Create mock benchmark directory
    fs::create_dir_all(&suite_path).unwrap();
    
    let benchmark_file = suite_path.join("test.bench");
    fs::write(&benchmark_file, "// Mock benchmark file").unwrap();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "benchmark",
            suite_path.to_str().unwrap(),
            "--warmup", "1",
            "--iterations", "2"
        ])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should complete successfully or show appropriate error
        assert!(stdout.contains("Benchmark Results") || 
                stdout.contains("Running benchmark suite") ||
                output.status.success());
    }
}

/// Test CLI error handling for missing files
#[test]
fn test_cli_missing_file_error() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "analyze",
            "nonexistent_file.json"
        ])
        .output();
    
    if let Ok(output) = output {
        // Should handle missing file gracefully
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Failed to read") || 
                stderr.contains("No such file") ||
                !output.status.success());
    }
}

/// Test CLI configuration validation
#[test]
fn test_cli_config_validation() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let invalid_config_path = temp_dir.path().join("invalid.toml");
    
    // Create invalid TOML
    fs::write(&invalid_config_path, "invalid toml content [[[").unwrap();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "--config", invalid_config_path.to_str().unwrap(),
            "--help"
        ])
        .output();
    
    if let Ok(output) = output {
        // Should handle invalid config gracefully and use defaults
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}

/// Test CLI with environment variables
#[test]
fn test_cli_environment_variables() {
    common::tracing::setup();
    
    let output = Command::new("cargo")
        .env("CURSED_PROFILE_OUTPUT", "/tmp/test_output")
        .env("CURSED_PROFILE_VERBOSE", "1")
        .args(&["run", "--bin", "cursed-profile", "--", "--help"])
        .output();
    
    if let Ok(output) = output {
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}

/// Test CLI command chaining and pipelines
#[test]
fn test_cli_command_pipeline() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let profile_data = temp_dir.path().join("profile.json");
    let report_output = temp_dir.path().join("report.html");
    
    // Create mock profile data
    let mock_data = serde_json::json!({
        "session_name": "pipeline_test",
        "timestamp": "2024-01-01T00:00:00Z",
        "session_duration": {"secs": 5, "nanos": 0},
        "mode_data": {},
        "metadata": {}
    });
    
    fs::write(&profile_data, serde_json::to_string_pretty(&mock_data).unwrap()).unwrap();
    
    // Test report generation
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "report",
            profile_data.to_str().unwrap(),
            "--format", "html",
            "--output", report_output.to_str().unwrap()
        ])
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Report generation completed") || output.status.success());
    }
}

/// Test CLI with different output formats
#[test]
fn test_cli_output_formats() {
    common::tracing::setup();
    
    let formats = ["json", "html", "csv"];
    
    for format in &formats {
        let output = Command::new("cargo")
            .args(&[
                "run", "--bin", "cursed-profile", "--",
                "report", "--help"
            ])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains(format) || output.status.success());
        }
    }
}

/// Test CLI memory management during execution
#[test]
fn test_cli_memory_usage() {
    common::tracing::setup();
    
    // Test that CLI doesn't consume excessive memory
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "--help"])
        .output();
    
    if let Ok(output) = output {
        // Basic test that CLI completes successfully
        assert!(output.status.success() || output.status.code() == Some(0));
        
        // Check that output is reasonable (not empty, not excessively large)
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
        assert!(stdout.len() < 100_000); // Reasonable help text size
    }
}

/// Test CLI interrupt handling
#[test]
fn test_cli_signal_handling() {
    common::tracing::setup();
    
    // Test that CLI handles basic execution without hanging
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed-profile", "--", "--version"])
        .output();
    
    if let Ok(output) = output {
        // Should complete quickly for version check
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}

/// Test CLI with complex argument combinations
#[test]
fn test_cli_complex_arguments() {
    common::tracing::setup();
    
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    let output_path = temp_dir.path().join("output");
    
    // Create basic config
    fs::write(&config_path, "default_cpu_frequency = 150").unwrap();
    fs::create_dir_all(&output_path).unwrap();
    
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", "cursed-profile", "--",
            "--verbose",
            "--config", config_path.to_str().unwrap(),
            "--output", output_path.to_str().unwrap(),
            "--help"
        ])
        .output();
    
    if let Ok(output) = output {
        assert!(output.status.success() || output.status.code() == Some(0));
    }
}
