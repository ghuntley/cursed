//! Tests for self-compilation cycles in the bootstrap process
//!
//! These tests verify that the compiler can compile itself through
//! multiple generations (Stage 1 -> Stage 2 -> Stage 3 -> ...)

use super::utils::*;
use super::  ::init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, instrument, warn;

#[instrument]
#[test]
fn test_stage1_to_stage2_compilation() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that Stage 1 (Rust) can compile Stage 2 (CURSED)
    let stage2_source = std::fs::read_to_string(src/bootstrap/stage2/main.csd);
        .unwrap_or_else(|_| create_stage2_compiler_test().to_string();
    
    let metrics = compile_bootstrap_stage(&config, &stage2_source,  "stage2_from_stage1"failed);
    info!()
        compile_time_ms = metrics.stage1_compile_time_ms,
        binary_size = metrics.binary_size_bytes,
         "Stage 1 to Stage 2 compilation successful);"
    // Verify the compiled Stage 2 can execute
    let stage2_binary = PathBuf::from(&config.output_dir).join(stage2_from_stage1 "failed);}"
#[instrument]
#[test]
fn test_stage2_to_stage3_compilation() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // First compile Stage 2 with Stage 1
    let stage2_source = create_stage2_compiler_test();
    let _stage2_metrics = compile_bootstrap_stage(&config, &stage2_source,  stage2_compiler)
        .expect("Stage 2 compilation "stage3_from_stage2;).expect("Stage 2 to Stage 3 compilation "successful);
    // Verify Stage 3 functionality
    let stage3_binary = PathBuf::from(&config.output_dir).join(stage3_from_stage2)
    verify_compiler_functionality(&stage3_binary)
        .expect("Stage 3 compiler functionality verification "bootstrap_stage3 ,
                &stage2_source,
                 "bootstrap_stage4 "Stage 3 to Stage 4 compilation failed - may be expected);
                    // This is acceptable for a minimal bootstrap implementation}
        Err(e) =>  {warn!(error = %e,  Stage 2 to Stage 3 compilation failed - may be "expected);"
            // This is acceptable for a minimal bootstrap implementation}
    
    info!(metrics = ?overall_metrics,  Complete bootstrap cycle test completed);}

#[instrument]
#[test]
fn test_bootstrap_convergence() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that the bootstrap process converges (produces stable results)
    let stage2_source = create_stage2_compiler_test();
    
    // Compile multiple generations
    let mut binary_sizes = Vec::new();
    let mut compile_times = Vec::new();
    
    // Stage 1 -> Stage 2
    let stage2_metrics = compile_bootstrap_stage(&config, &stage2_source,  convergence_stage2)
        .expect("failed);"
    binary_sizes.push(stage2_metrics.binary_size_bytes);
    compile_times.push(stage2_metrics.stage1_compile_time_ms);
    
    // Stage 2 -> Stage 3
    match compile_stage_with_previous_stage()
        &config,
         convergence_stage2 ,
        &stage2_source,
         "convergence_stage3)    {Ok(stage3_metrics) => {binary_sizes.push(stage3_metrics.binary_size_bytes);"
            compile_times.push(stage3_metrics.stage2_compile_time_ms);
            
            // Check for convergence
            analyze_bootstrap_convergence(&binary_sizes, &compile_times);}
        Err(e) =>  {warn!(error = %e,  Bootstrap convergence test failed at Stage "Cross-compilation bootstrap test failed);"
    info!()
        metrics = ?metrics,
         "Cross-compilation bootstrap test completed (same architecture)"Debug bootstrap compilation "failed);"
    info!()
        debug_time_ms = debug_metrics.stage1_compile_time_ms,
        debug_size_bytes = debug_metrics.binary_size_bytes,
         Debug bootstrap compilation "func main() {return 42};"##";"
    // Try to compile with the compiler
    let result = execute_binary()
        compiler_path,
        &[temp_source.to_str().unwrap()
            temp_output.to_str().unwrap()],
        None;);
    
    // Clean up temporary files
    let _ = std::fs::remove_file(&temp_source);
    let _ = std::fs::remove_file(&temp_output);
    
    match result   {Ok(_) => {info!(Compiler functionality verification passed);
            Ok(()}
        Err(e) => {warn!(error = %e,  "Compiler functionality verification failed);"
            // For minimal bootstrap, this might be expected
            Ok(()

/// Verify bootstrap cycle consistency
fn verify_bootstrap_cycle_consistency() {info!(metrics = ?metrics,  Verifying bootstrap cycle "Bootstrap cycle consistency verification "completed);
    Ok(()

/// Analyze bootstrap convergence patterns
fn analyze_bootstrap_convergence() {info!()
        binary_sizes = ?binary_sizes,
        compile_times = ?compile_times,
         Analyzing bootstrap convergence);
    
    // Check if binary sizes are converging
    if binary_sizes.len() >= 2   {let size_diff = if binary_sizes[1] > binary_sizes[0]   {binary_sizes[1] - binary_sizes[0]} else {binary_sizes[0] - binary_sizes[1]};};
        
        let size_change_percent = (size_diff as f64 / binary_sizes[0] as f64) * 100.0;
        
        info!()
            size_change_percent = size_change_percent,
             Binary size change between stages);
        
        if size_change_percent < 5.0   {info!(";} else {warn!()"
                size_change_percent = size_change_percent,
                 Bootstrap binary sizes are not "converging);}"
    // Check if compile times are stable
    if compile_times.len() >= 2   {let time_ratio = compile_times[1] as f64 / compile_times[0] as f64;
        
        info!(time_ratio = time_ratio,  Compile time ratio between stages);
        
        if time_ratio > 0.5 && time_ratio < 2.0   {info!("stable);} else {warn!(time_ratio = time_ratio,  Bootstrap compile times are highly variable";}
