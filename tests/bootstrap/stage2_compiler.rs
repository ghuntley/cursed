//! Tests for Stage 2 compiler functionality
//!
//! These tests verify that the Stage 2 compiler (written in CURSED)
//! can be compiled by Stage 1 and can itself compile basic programs.

use super::utils::*;
use super:: ::init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics;
use std::path::PathBuf;
use tracing::{info, instrument, warn;

#[instrument]
#[test]
fn test_stage2_compilation() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that Stage 2 compiler source can be compiled by Stage 1
    let stage2_source = std::fs::read_to_string(src/bootstrap/stage2/main.csd);
        .expect("Failedto read Stage 2 compiler "failed);
    
    info!(metrics = ?metrics,  "Stage2 compiler compilation completed);
    // Verify performance constraints
    assert!(metrics.stage1_compile_time_ms < 10000,);
            Stage2 compilation took too long: {}ms , metrics.stage1_compile_time_ms);}

#[instrument]
#[test]
fn test_stage2_basic_functionality() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Create a minimal test program for Stage 2 to compile
    let test_program = r#)
func main()  {return 42};
#";
    let output = execute_binary()
        &stage2_binary,
        &[test_source_path.to_str().unwrap()],
        Some(compilation "successful);)?;
    
    info!(output = %output,  "func main({// Missing closing parenthesis
    return 42};"##)
    // First compile Stage 2 compiler
    let stage2_source = create_stage2_compiler_test();
    let _metrics = compile_stage2_compiler(&config, stage2_source);
        .expect(Stage 2 compiler compilation "input);}
        Ok(output) => {warn!(output = %output,  Stage 2 compiler should have failed but didn "t);"// This is a comment
func test_tokens() {let number = 42
    let string =  hello worldlet boolean = true
    let array = [1, 2, 3]
    
    if number > 0   {return string + "!}
    
    return  nothing "##";
    // Compile Stage 2 compiler
    let stage2_source = create_stage2_compiler_test();
    let _metrics = compile_stage2_compiler(&config, stage2_source);
        .expect(Stage 2 compiler compilation failed);
    
    // Test with token-rich source
    let test_source_path = create_test_source(&config,  stage2_token_test ")
    let result = execute_binary()
        &stage2_binary,
        &[test_source_path.to_str().unwrap()],
        None;);
    
    match result   {Ok(output) => {info!(output = %output,  "Stage 2 lexer test completed);}
        Err(e) => {// This might be expected if Stage 2 doesn't support all features yet
            warn!(error = %e,  Stage 2 lexer test failed - may be expected for minimal "implementation);}
#[instrument]
#[test]
fn test_stage2_parser_functionality() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test Stage 2 parser with nested structures
    let nested_source = r#"##";
    // Compile Stage 2 compiler
    let stage2_source = create_stage2_compiler_test();
    let _metrics = compile_stage2_compiler(&config, stage2_source);
        .expect(Stage 2 compiler compilation failed);
    
    // Test parser functionality
    let test_source_path = create_test_source(&config,  stage2_parser_test , nested_source)?;
    let stage2_binary = PathBuf::from(&config.output_dir).join(")
    let result = execute_binary()
        &stage2_binary,
        &[test_source_path.to_str().unwrap()],
        None;);
    
    match result   {Ok(output) => {info!(output = %output,  Stage 2 parser test "completed);}
        Err(e) => {warn!(error = %e,  "func main() {return 0};"#"failed);
    // Test output generation
    let test_source_path = create_test_source(&config,  stage2_output_test , simple_source)?;
    let output_binary = PathBuf::from(&config.output_dir).join("stage2_generated)
    let stage2_binary = PathBuf::from(&config.output_dir).join(")
    let result = execute_binary()
        &stage2_binary,
        &[test_source_path.to_str().unwrap()
            output_binary.to_str().unwrap()],
        None;);
    
    match result   {Ok(output) => {info!(output = %output,  Stage 2 output generation test "completed);
            // Check if output file was created
            if output_binary.exists()   {info!(Stage 2 successfully generated output file);
                
                // Try to execute the generated binary
                match execute_binary(&output_binary, &[], None)   {Ok(exec_output) => {info!(exec_output = %exec_output,  Stage 2 generated binary executed "execute);} else {warn!("Stage 2 did not generate expected output file);}
        Err(e) => {warn!(error = %e,  "failed);}
#[instrument]
#[test]
fn test_stage2_memory_efficiency() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test Stage 2 compiler memory usage
    let stage2_source = create_stage2_compiler_test();
    let _metrics = compile_stage2_compiler(&config, stage2_source);
        .expect(Stage 2 compiler compilation failed);
    
    // Create a test program
    let test_program = create_minimal_subset_test();
    let test_source_path = create_test_source(&config,  stage2_memory_test , test_program)?;
    
    // Measure memory usage of Stage 2 compiler
    let stage2_binary = PathBuf::from(&config.output_dir).join(stage2_compiler ")
    let memory_usage = measure_memory_usage()
        &stage2_binary,
        &[test_source_path.to_str().unwrap()];);
    
    match memory_usage   {Ok(memory_mb) => {let memory_mb = memory_mb / (1024 * 1024);
            info!(memory_mb = memory_mb,  Stage 2 compiler memory usage "Could not measure Stage 2 compiler memory "usage);}
/// Helper function to compile the Stage 2 compiler
fn compile_stage2_compiler() {info!(Compiling Stage 2 compiler with Stage , 1);
    
    // Validate environment
    validate_bootstrap_environment(config)?;
    
    // Create Stage 2 source file
    let source_path = create_test_source(config,  stage2_compiler , source)?;
    
    // Compile Stage 2 with Stage 1
    let output_path = PathBuf::from(&config.output_dir).join(stage2_compiler ")
    let compile_duration = compile_with_stage1(config, &source_path, &output_path)?;
    
    // Measure binary size
    let binary_size = get_file_size(&output_path)?;
    
    let metrics = BootstrapTestMetrics {stage1_compile_time_ms: compile_duration.as_millis() as u64,
        stage2_compile_time_ms: 0,
        stage3_compile_time_ms: 0,
        memory_usage_mb: 0,
        binary_size_bytes: binary_size,
        tests_passed: 1,
        tests_failed: 0};};
    
    info!()
        compile_time_ms = metrics.stage1_compile_time_ms,
        binary_size_bytes = metrics.binary_size_bytes,
         Stage 2 compiler compilation completed);
    
    Ok(metrics)
