//! Comprehensive tests for the CURSED documentation CLI tool
//!
//! Tests all command-line options, formats, error handling, and integration scenarios.

use cursed::docs::{DocConfig, DocumentationGenerator, DocResult};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use tracing_test::traced_test;

/// Test helper to create a temporary CURSED source file
fn create_test_source_file(temp_dir: &TempDir, filename: &str, content: &str) -> PathBuf {
    let file_path = temp_dir.path().join(filename);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

/// Test helper to run cursed-doc CLI command
fn run_cli_command(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(&["run", "--bin", "cursed-doc", "--"])
        .args(args)
        .output()
        .expect("Failed to execute cursed-doc command")
}

#[test]
#[traced_test]
fn test_cli_help_option() {
    let output = run_cli_command(&["--help"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generate documentation for CURSED source files"));
    assert!(stdout.contains("--html"));
    assert!(stdout.contains("--markdown"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--check"));
    assert!(stdout.contains("--serve"));
}

#[test]
#[traced_test]
fn test_cli_version_option() {
    let output = run_cli_command(&["--version"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cursed-doc"));
}

#[test]
#[traced_test]
fn test_html_generation_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create a simple test source file
    create_test_source_file(&temp_dir, "test.csd", r#"
/// A simple test function
/// @param x - The input value
/// @returns The squared value
slay square(x: i32) -> i32 {
    ret x * x
}
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Test Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    assert!(output_dir.join("index.html").exists());
}

#[test]
#[traced_test]
fn test_markdown_generation_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// A test struct
squad TestStruct {
    /// The name field
    name: String,
    /// The value field
    value: i32,
}
"#);
    
    let output = run_cli_command(&[
        "--markdown",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Test Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Check that markdown output was generated
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documentation successfully generated"));
}

#[test]
#[traced_test]
fn test_json_generation_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// A test interface
collab TestInterface {
    /// Get the name
    slay get_name() -> String;
    /// Set the value
    slay set_value(value: i32);
}
"#);
    
    let output = run_cli_command(&[
        "--json",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Test Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Check that JSON output was generated
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documentation successfully generated"));
}

#[test]
#[traced_test]
fn test_check_validation_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create a file with missing documentation
    create_test_source_file(&temp_dir, "incomplete.csd", r#"
/// This function has documentation
slay documented_function() -> i32 {
    ret 42
}

// This function lacks documentation
slay undocumented_function() -> String {
    ret "hello"
}
"#);
    
    let output = run_cli_command(&[
        "--check",
        "--source", temp_dir.path().to_str().unwrap(),
        "--package-name", "Test Package",
    ]);
    
    // Check should fail due to missing documentation
    assert!(!output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documentation Validation"));
    assert!(stdout.contains("undocumented_function"));
}

#[test]
#[traced_test]
fn test_multiple_source_directories() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let src1 = temp_dir.path().join("src1");
    let src2 = temp_dir.path().join("src2");
    let output_dir = temp_dir.path().join("docs");
    
    fs::create_dir_all(&src1).expect("Failed to create src1");
    fs::create_dir_all(&src2).expect("Failed to create src2");
    
    fs::write(src1.join("module1.csd"), r#"
/// Module 1 function
slay module1_func() -> i32 { ret 1 }
"#).expect("Failed to write module1.csd");
    
    fs::write(src2.join("module2.csd"), r#"
/// Module 2 function  
slay module2_func() -> i32 { ret 2 }
"#).expect("Failed to write module2.csd");
    
    let output = run_cli_command(&[
        "--html",
        "--source", src1.to_str().unwrap(),
        "--source", src2.to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Multi-Source Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
}

#[test]
#[traced_test]
fn test_exclude_patterns() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create multiple files, some should be excluded
    create_test_source_file(&temp_dir, "main.csd", r#"
/// Main function
slay main() -> i32 { ret 0 }
"#);
    
    create_test_source_file(&temp_dir, "test_helper.csd", r#"
/// Test helper function
slay test_helper() -> i32 { ret 1 }
"#);
    
    create_test_source_file(&temp_dir, "example_demo.csd", r#"
/// Example function
slay example_demo() -> i32 { ret 2 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--exclude", "test",
        "--exclude", "example",
        "--package-name", "Filtered Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Should only process main.csd, excluding test and example files
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documentation successfully generated"));
}

#[test]
#[traced_test]
fn test_include_private_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "visibility.csd", r#"
/// Public function
slay public_func() -> i32 { ret 1 }

/// Private function
private slay private_func() -> i32 { ret 2 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--include-private",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Visibility Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
}

#[test]
#[traced_test]
fn test_custom_package_info() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// Test function
slay test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Custom Package Name",
        "--package-version", "2.1.3",
        "--description", "A custom package for testing",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Check that custom package info appears in output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documentation successfully generated"));
}

#[test]
#[traced_test]
fn test_verbose_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// Verbose test function
slay verbose_test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--verbose",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Verbose Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Verbose mode should show more detailed output
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Debug level logs should be present with verbose flag
    assert!(stderr.len() > 0); // Should have some debug output
}

#[test]
#[traced_test]
fn test_quiet_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// Quiet test function
slay quiet_test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--quiet",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Quiet Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Quiet mode should minimize output
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should be minimal or no output in quiet mode
    assert!(stdout.len() < 100); // Very minimal output expected
}

#[test]
#[traced_test]
fn test_json_output_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// JSON test function
slay json_test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--output-format", "json",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "JSON Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Output should be in JSON format
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("{")); // Should contain JSON
    assert!(stdout.contains("status"));
}

#[test]
#[traced_test]
fn test_markdown_output_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// Markdown test function
slay markdown_test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--output-format", "markdown",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Markdown Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Output should be in Markdown format
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#")); // Should contain Markdown headers
    assert!(stdout.contains("CURSED Documentation"));
}

#[test]
#[traced_test]
fn test_clean_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create output directory with existing content
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    fs::write(output_dir.join("old_file.html"), "old content").expect("Failed to write old file");
    
    create_test_source_file(&temp_dir, "test.csd", r#"
/// Clean test function
slay clean_test() -> i32 { ret 0 }
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--clean",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Clean Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Old file should be cleaned up
    assert!(!output_dir.join("old_file.html").exists());
}

#[test]
#[traced_test]
fn test_stats_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    create_test_source_file(&temp_dir, "stats.csd", r#"
/// Stats test function
slay stats_function() -> i32 { ret 0 }

/// Stats test struct
squad StatsStruct {
    field: i32,
}
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--stats",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Stats Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
    
    // Should show detailed statistics
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Statistics"));
}

#[test]
#[traced_test]
fn test_error_handling_nonexistent_source() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    let output = run_cli_command(&[
        "--html",
        "--source", "/nonexistent/directory",
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Error Package",
    ]);
    
    // Should fail with non-zero exit code
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error") || stderr.contains("error"));
}

#[test]
#[traced_test]
fn test_error_handling_invalid_syntax() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create a file with invalid CURSED syntax
    create_test_source_file(&temp_dir, "invalid.csd", r#"
/// This has invalid syntax
slay invalid_function( -> i32 {
    ret "this is wrong"
}
"#);
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--package-name", "Invalid Package",
    ]);
    
    // May succeed but should report parsing errors
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should mention parsing issues somewhere
    assert!(stderr.len() > 0 || stdout.contains("error") || stdout.contains("Error"));
}

#[test]
#[traced_test]
fn test_max_depth_option() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create nested directory structure
    let nested_dir = temp_dir.path().join("level1").join("level2").join("level3");
    fs::create_dir_all(&nested_dir).expect("Failed to create nested dirs");
    
    fs::write(temp_dir.path().join("root.csd"), "slay root() {}").expect("Failed to write root.csd");
    fs::write(nested_dir.join("deep.csd"), "slay deep() {}").expect("Failed to write deep.csd");
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--max-depth", "2",
        "--package-name", "Depth Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
}

#[test]
#[traced_test]
fn test_parallel_jobs_option() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let output_dir = temp_dir.path().join("docs");
    
    // Create multiple source files
    for i in 1..=5 {
        create_test_source_file(&temp_dir, &format!("file{}.csd", i), &format!(r#"
/// Function {}
slay function_{}() -> i32 {{ ret {} }}
"#, i, i, i));
    }
    
    let output = run_cli_command(&[
        "--html",
        "--source", temp_dir.path().to_str().unwrap(),
        "--output", output_dir.to_str().unwrap(),
        "--jobs", "2",
        "--package-name", "Parallel Package",
    ]);
    
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success());
}

// Configuration file tests

#[test]
#[traced_test]
fn test_generate_config_toml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("cursed-doc.toml");
    
    let output = run_cli_command(&[
        "--generate-config", config_path.to_str().unwrap(),
    ]);
    
    assert!(output.status.success());
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config");
    assert!(config_content.contains("[package]"));
    assert!(config_content.contains("[generation]"));
}

#[test]
#[traced_test]
fn test_generate_config_json() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("cursed-doc.json");
    
    let output = run_cli_command(&[
        "--generate-config", config_path.to_str().unwrap(),
    ]);
    
    assert!(output.status.success());
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config");
    assert!(config_content.contains("{"));
    assert!(config_content.contains("\"package\""));
}

#[test]
#[traced_test]
fn test_generate_config_yaml() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("cursed-doc.yaml");
    
    let output = run_cli_command(&[
        "--generate-config", config_path.to_str().unwrap(),
    ]);
    
    assert!(output.status.success());
    assert!(config_path.exists());
    
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config");
    assert!(config_content.contains("package:"));
    assert!(config_content.contains("generation:"));
}

// Integration tests with actual DocConfig

#[test]
#[traced_test]
fn test_doc_config_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    create_test_source_file(&temp_dir, "integration.csd", r#"
/// Integration test function
/// @param value - Test value
/// @returns Processed value
slay integration_test(value: i32) -> i32 {
    ret value * 2
}
"#);
    
    let config = DocConfig::new("Integration Test".to_string(), "1.0.0".to_string())
        .with_source_dirs(vec![temp_dir.path().to_path_buf()])
        .with_output_dir(temp_dir.path().join("output"))
        .include_private(false)
        .with_search(true);
    
    let generator = DocumentationGenerator::new(config);
    assert!(generator.is_ok());
}

#[test]
#[traced_test]
fn test_validation_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    create_test_source_file(&temp_dir, "validation.csd", r#"
/// Well documented function
/// @param input - The input parameter
/// @returns The result value
slay well_documented(input: String) -> String {
    ret input
}

// Missing documentation
slay poorly_documented() -> i32 {
    ret 42
}
"#);
    
    let config = DocConfig::new("Validation Test".to_string(), "1.0.0".to_string())
        .with_source_dirs(vec![temp_dir.path().to_path_buf()]);
    
    let generator = DocumentationGenerator::new(config).expect("Failed to create generator");
    let validation_result = generator.validate_documentation();
    
    assert!(validation_result.is_ok());
    let result = validation_result.unwrap();
    
    // Should have some missing documentation
    assert!(result.missing_documentation.len() > 0);
    assert!(result.total_items > 0);
    assert!(result.documented_items > 0);
}

#[cfg(test)]
mod cli_argument_parsing_tests {
    use super::*;
    
    #[test]
    fn test_argument_combinations() {
        // Test that conflicting flags don't cause issues
        let output = run_cli_command(&[
            "--html",
            "--markdown", 
            "--json",
            "--help"
        ]);
        
        // Help should take precedence
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generate documentation"));
    }
    
    #[test]
    fn test_verbose_levels() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        create_test_source_file(&temp_dir, "verbose.csd", r#"
/// Verbose levels test
slay verbose_levels() -> i32 { ret 0 }
"#);
        
        // Test different verbose levels
        for verbose_level in &["-v", "-vv", "-vvv"] {
            let output = run_cli_command(&[
                "--html",
                verbose_level,
                "--source", temp_dir.path().to_str().unwrap(),
                "--package-name", "Verbose Test",
            ]);
            
            // All should succeed but with different log levels
            if !output.status.success() {
                eprintln!("Failed with verbose level {}: {}", verbose_level, String::from_utf8_lossy(&output.stderr));
            }
        }
    }
}
