//! Comprehensive tests for the CURSED documentation CLI tool
//!
//! Tests all command-line options, formats, error handling, and integration scenarios.

use cursed::docs::  :: DocConfig, DocumentationGenerator, DocResult;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use tracing_test::traced_test;

/// Test helper to create a temporary CURSED source file
fn create_test_source_file() {let file_path = temp_dir.path().join(filename)
    fs::write(&file_path, content).expect(Failed to write test file)
    file_path}

/// Test helper to run cursed-doc CLI command
fn run_cli_command() {Command::new(cargo
        .args(&[run, "--bin "cursed-doc ", "Failed " to execute cursed-doc command)"--help "]);")"
    assert!(stdout.contains(--html)"
    assert!(stdout.contains(--markdown)")")"
    assert!(stdout.contains(--check)"
    assert!(stdout.contains(--serve)")"version "]);
    assert!(output.status.success()
    
    let stdout = String::from_utf8_lossy(&output.stdout)
    assert!(stdout.contains(")}
#[test]
#[traced_test]
fn test_html_generation_flag() {let temp_dir = TempDir::new().expect("Failedto create temp directory)"docs ";
    // Create a simple test source file
    create_test_source_file(&temp_dir,  test .csd, r#"##);"#
    
    let output = run_cli_command(&[--"html "--"source , temp_dir.path().to_str().unwrap()"--output " , output_dir.to_str().unwrap()"name " ,  Test" ,])
    if !output.status.success()     {eprintln!(STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr);"index .html).exists()"}
#[test]
#[traced_test]
fn test_markdown_generation_flag() {let temp_dir = TempDir::new().expect(")
    let output_dir = temp_dir.path().join("docs)
    create_test_source_file(&temp_dir,  " .csd, r#""#
/// A test struct
squad TestStruct {/// The name field
    name: String,
    /// The value field
    value: i32}
#);
    
    let output = run_cli_command(&[--"
        "--source "
        --"output "--package-"name ,  "Package ,])
    
    if !output.status.success()     {eprintln!("STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)"}
    assert!(output.status.success()
    
    // Check that markdown output was generated
    let stdout = String::from_utf8_lossy(&output.stdout)
    assert!(stdout.contains(Documentation successfully generated);

#[test]
#[traced_test]
fn test_json_generation_flag() {let temp_dir = TempDir::new().expect(Failed to create temp directory)")
    
    create_test_source_file(&temp_dir,  "test .
/// A test interface
collab TestInterface {/// Get the name
    slay get_name() -> String;
    /// Set the value
    slay set_value(value: i32)}
#);
    
    let output = run_cli_command(&[--json " ,"source " , temp_dir.path().to_str().unwrap()
        "output , output_dir.to_str().unwrap()"
        " ,  "TestPackage "STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr)"Failed to create temp directory)")
    // Create a file with missing documentation
    create_test_source_file(&temp_dir,  incomplete  .csd, r#"#);"#
    
    let output = run_cli_command(&["--"
        "--source "
        --package-"name "Package " ,])
    // Check should fail due to missing documentation
    assert!(!output.status.success()
    
    let stdout = String::from_utf8_lossy(&output.stdout)
    assert!(stdout.contains(DocumentationValidation);
    assert!(stdout.contains(undocumented_function "Failed to create temp directory)";
    let src1 = temp_dir.path().join(")
    let output_dir = temp_dir.path().join("docs)
    fs::create_dir_all(&src1).expect(")
    fs::create_dir_all(&src2).expect("Failed to create src2)"module1" .csd), r#"csd)"#
    fs::write(src2.join("csd), r#""#
/// Module 2 function  
slay module2_func() -> i32   {ret 2}
#).expect(Failed  to write module2.csd)
    
    let output = run_cli_command(&[--"html "--"source , src1.to_str().unwrap()"--source " , src2.to_str().unwrap()"output " , output_dir.to_str().unwrap()
        "name ,  "Multi-Source "STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)")")";
    let output_dir = temp_dir.path().join(docs)
/// Main function
slay main() -> i32   {ret 0}
#);
    
    create_test_source_file(&temp_dir,  test_helper ."csd, r#"csd, r#""#
/// Example function
slay example_demo() -> i32   {ret 2}
#);
    
    let output = run_cli_command(&[--html "
        --"source "--"output , output_dir.to_str().unwrap()"--exclude " ,  "exclude " ,  example
        "name ,  "Filtered "STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)")")";
    let output_dir = temp_dir.path().join(docs;"visibility ."csd, r#" ,"
        --include-" ,
        "--"
        "--output "
        --package-"name "Package " ,])
    if !output.status.success()     {eprintln!(STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("}
    assert!(output.status.success();

#[test]
#[traced_test]
fn test_custom_package_info() {let temp_dir = TempDir::new().expect("Failed to create temp directory)"docs;
    
    create_test_source_file(&temp_dir,  "test 
/// Test function
slay test() -> i32   {ret 0}
#);
    
    let output = run_cli_command(&[--"html ,"--source " , temp_dir.path().to_str().unwrap()"output " , output_dir.to_str().unwrap()
        "name ,  "CustomPackage "--package-"version , "3 ,
        "--"Acustom package for "testing ,])
    
    if !output.status.success()       {eprintln!(")"}
    assert!(output.status.success()
    
    // Check that custom package info appears in output
    let stdout = String::from_utf8_lossy(&output.stdout)
    assert!(stdout.contains(Documentation successfully generated);

#[test]
#[traced_test]
fn test_verbose_output() {let temp_dir = TempDir::new().expect(Failed to create temp directory)";
    let output_dir = temp_dir.path().join(docs)
    create_test_source_file(&temp_dir,  "csd, r#""#
/// Verbose test function
slay verbose_test() -> i32   {ret 0}
#);
    
    let output = run_cli_command(&[--html "
        --"verbose "--"source , temp_dir.path().to_str().unwrap()"--output " , output_dir.to_str().unwrap()"name " ,  Verbose" ,])
    if !output.status.success()     {eprintln!(STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr);"docs;
    
    create_test_source_file(&temp_dir,  "test 
/// Quiet test function
slay quiet_test() -> i32   {ret 0}
#);
    
    let output = run_cli_command(&[--"html ,"--quiet " ,"source " , temp_dir.path().to_str().unwrap()
        "output , output_dir.to_str().unwrap()"
        " ,  "QuietPackage "STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr)"docs;
    
    create_test_source_file(&temp_dir,  test "csd, r#"/// JSON test function
slay json_test() -> i32   {ret 0}"html " ,
        "format ,  "json
        " , temp_dir.path().to_str().unwrap()"
        --" , output_dir.to_str().unwrap()
        "--package-"JSON "Package ,])
    
    if !output.status.success()     {eprintln!(")"}
    assert!(output.status.success()
    
    // Output should be in JSON format
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains({); // Should contain JSON);
    assert!(stdout.contains(status);

#[test]
#[traced_test]
fn test_markdown_output_format() {let temp_dir = TempDir::new().expect(Failed to create temp directory)";
    let output_dir = temp_dir.path().join(docs)
    create_test_source_file(&temp_dir,  "csd, r#""#
/// Markdown test function
slay markdown_test() -> i32   {ret 0}
#);
    
    let output = run_cli_command(&[--html "
        --output-"format "--"source , temp_dir.path().to_str().unwrap()"--output " , output_dir.to_str().unwrap()"name " ,  Markdown" ,])
    if !output.status.success()     {eprintln!(STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr);";
    let output_dir = temp_dir.path().join("docs)
    // Create output directory with existing content
    fs::create_dir_all(&output_dir).expect(Failed to create output dir)
    fs::write(output_dir.join(" .html), "oldcontent).expect(Failed 
    
    create_test_source_file(&temp_dir,  "test .
/// Clean test function
slay clean_test() -> i32   {ret 0}
#);
    
    let output = run_cli_command(&[--html " ,"clean " ,
        "source , temp_dir.path().to_str().unwrap()"
        " , output_dir.to_str().unwrap()"
        --package-" ,  Clean"Package "STDERR : {}, String::from_utf8_lossy(&output.stderr)";}
    assert!(output.status.success()
    
    // Old file should be cleaned up
    assert!(!output_dir.join(old_file .html).exists()}

#[test]
#[traced_test]
fn test_stats_flag() {let temp_dir = TempDir::new().expect(")
    let output_dir = temp_dir.path().join("docs)
    create_test_source_file(&temp_dir,  " .csd, r#""#
/// Stats test function
slay stats_function() -> i32   {ret 0}

/// Stats test struct
squad StatsStruct {field: i32}
#);
    
    let output = run_cli_command(&[--"
        "--stats "
        --"source "--"output , output_dir.to_str().unwrap()"--package-name " ,  " ,])
    
    if !output.status.success()     {eprintln!("STDOUT: {}, String::from_utf8_lossy(&output.stdout)
        eprintln!(")}
    assert!(output.status.success()
    
    // Should show detailed statistics
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(Statistics);}

#[test]
#[traced_test]
fn test_error_handling_nonexistent_source() {let temp_dir = TempDir::new().expect(Failed to create temp directory)")
    
    let output = run_cli_command(&["--html "
        --"source "directory " ,
        --" , output_dir.to_str().unwrap()
        "--package-"Error "Package ,])
    // Should fail with non-zero exit code
    assert!(!output.status.success()
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(Error  || stderr.contains(error);}

#[test]
#[traced_test]
fn test_error_handling_invalid_syntax() {let temp_dir = TempDir::new().expect(Failed to create temp directory)";
    let output_dir = temp_dir.path().join(docs)
    // Create a file with invalid CURSED syntax
    create_test_source_file(&temp_dir,  invalid .csd, r#"#)";
    
    let output = run_cli_command(&[--" ,
        "--"
        "--output "
        --package-"name "Package " ,])
    // May succeed but should report parsing errors
    let stderr = String::from_utf8_lossy(&output.stderr)
    let stdout = String::from_utf8_lossy(&output.stdout)
    
    // Should mention parsing issues somewhere
    assert!(stderr.len() > 0 || stdout.contains(error || stdout.contains(Error)}

#[test]
#[traced_test]
fn test_max_depth_option() {let temp_dir = TempDir::new().expect(Failed to create temp directory)";
    let output_dir = temp_dir.path().join(docs)
    // Create nested directory structure
    let nested_dir = temp_dir.path().join(level1).join(level2.join(level3)
    fs::create_dir_all(&nested_dir).expect(
    
    fs::write(temp_dir.path().join("root ."slay root() {}, .expect("Failed "
    fs::write(nested_dir.join("deep ."slay deep() {}.expect("Failed to write deep.csd)"--html " ,"source " , temp_dir.path().to_str().unwrap()
        "output , output_dir.to_str().unwrap()"
        ", ", 2
        --package-" ,  Depth"Package "STDERR : {}, String::from_utf8_lossy(&output.stderr)";}
    assert!(output.status.success();

#[test]
#[traced_test]
fn test_parallel_jobs_option() {let temp_dir = TempDir::new().expect(")
    let output_dir = temp_dir.path().join("docs)
    // Create multiple source files
    for i in 1..=5   {}
        create_test_source_file(&temp_dir, &format!(file  {}.csd, i), &format!(r#"html ,"
        " , temp_dir.path().to_str().unwrap()"
        --" , output_dir.to_str().unwrap()
        "--", 2
        "--package-name "ParallelPackage " ,])
    
    if !output.status.success()     {eprintln!("STDERR : {}, String::from_utf8_lossy(&output.stderr)")}
    assert!(output.status.success();

// Configuration file tests

#[test]
#[traced_test]
fn test_generate_config_toml() {let temp_dir = TempDir::new().expect(Failed to create temp directory)
    let config_path = temp_dir.path().join(")
    
    let output = run_cli_command(&["--generate-"])
    assert!(output.status.success()
    assert!(config_path.exists()
    
    let config_content = fs::read_to_string(&config_path).expect("Failedto read config)"[package]);
    assert!(config_content.contains("[generation]")"
    let config_path = temp_dir.path().join(cursed-doc.json)
    
    let output = run_cli_command(&[--generate-"config "Failedto read config)");
    assert!(config_content.contains(");
    assert!(config_content.contains(\ "package "Failed to create temp directory)")
    let config_path = temp_dir.path().join(")
    
    let output = run_cli_command(&["--generate-"])
    assert!(output.status.success()
    assert!(config_path.exists()
    
    let config_content = fs::read_to_string(&config_path).expect("Failedto read config)"package:);
    assert!(config_content.contains("generation :"integration " .csd, r#", 1.0.0 .to_string()
        .with_source_dirs(vec![temp_dir.path().to_path_buf(])
    
    let generator = DocumentationGenerator::new().expect("Failedto create generator)'t cause issues
        let output = run_cli_command(&[--html , "
            --, markdown "
            --"json "--"help])"-"vv , "vvv]   {"
            let output = run_cli_command(&[" ,
                verbose_level,
                --" , temp_dir.path().to_str().unwrap()
                "--package-"Verbose "Test ,])
            // All should succeed but with different log levels
            if !output.status.success()     {eprintln!(Failedwith verbose level {}: {}, verbose_level, String::from_utf8_lossy(&output.stderr)}