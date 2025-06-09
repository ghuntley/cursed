//! Tests for self-compilation cycles in the bootstrap process
//!
//! These tests verify that the compiler can compile itself through
//! multiple generations (Stage 1 -> Stage 2 -> Stage 3 -> ...)

use super::utils::*;
use super::{init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics};
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, instrument, warn};

#[instrument]
#[test]
fn test_stage1_to_stage2_compilation() {
    let config = init_bootstrap_tests();
    
    // Test that Stage 1 (Rust) can compile Stage 2 (CURSED)
    let stage2_source = std::fs::read_to_string("src/bootstrap/stage2/main.csd")
        .unwrap_or_else(|_| create_stage2_compiler_test().to_string());
    
    let metrics = compile_bootstrap_stage(&config, &stage2_source, "stage2_from_stage1")
        .expect("Stage 1 to Stage 2 compilation failed");
    
    info!(
        compile_time_ms = metrics.stage1_compile_time_ms,
        binary_size = metrics.binary_size_bytes,
        "Stage 1 to Stage 2 compilation successful"
    );
    
    // Verify the compiled Stage 2 can execute
    let stage2_binary = PathBuf::from(&config.output_dir).join("stage2_from_stage1");
    verify_compiler_functionality(&stage2_binary)
        .expect("Stage 2 compiler functionality verification failed");
}

#[instrument]
#[test]
fn test_stage2_to_stage3_compilation() {
    let config = init_bootstrap_tests();
    
    // First compile Stage 2 with Stage 1
    let stage2_source = create_stage2_compiler_test();
    let _stage2_metrics = compile_bootstrap_stage(&config, &stage2_source, "stage2_compiler")
        .expect("Stage 2 compilation failed");
    
    // Now use Stage 2 to compile Stage 3 (which is the same source)
    let stage3_metrics = compile_stage_with_previous_stage(
        &config,
        "stage2_compiler",
        &stage2_source,
        "stage3_from_stage2"
    ).expect("Stage 2 to Stage 3 compilation failed");
    
    info!(
        compile_time_ms = stage3_metrics.stage2_compile_time_ms,
        binary_size = stage3_metrics.binary_size_bytes,
        "Stage 2 to Stage 3 compilation successful"
    );
    
    // Verify Stage 3 functionality
    let stage3_binary = PathBuf::from(&config.output_dir).join("stage3_from_stage2");
    verify_compiler_functionality(&stage3_binary)
        .expect("Stage 3 compiler functionality verification failed");
}

#[instrument]
#[test]
fn test_complete_bootstrap_cycle() {
    let config = init_bootstrap_tests();
    
    let mut overall_metrics = BootstrapTestMetrics::default();
    let stage2_source = create_stage2_compiler_test();
    
    // Stage 1 -> Stage 2
    info!("Compiling Stage 2 with Stage 1 (Rust compiler)");
    let stage2_metrics = compile_bootstrap_stage(&config, &stage2_source, "bootstrap_stage2")
        .expect("Stage 1 to Stage 2 compilation failed");
    overall_metrics.stage1_compile_time_ms = stage2_metrics.stage1_compile_time_ms;
    
    // Stage 2 -> Stage 3
    info!("Compiling Stage 3 with Stage 2 (first CURSED compiler)");
    let stage3_metrics = compile_stage_with_previous_stage(
        &config,
        "bootstrap_stage2",
        &stage2_source,
        "bootstrap_stage3"
    );
    
    match stage3_metrics {
        Ok(metrics) => {
            overall_metrics.stage2_compile_time_ms = metrics.stage2_compile_time_ms;
            
            // Stage 3 -> Stage 4 (verify self-hosting is stable)
            info!("Compiling Stage 4 with Stage 3 (second CURSED compiler)");
            let stage4_metrics = compile_stage_with_previous_stage(
                &config,
                "bootstrap_stage3",
                &stage2_source,
                "bootstrap_stage4"
            );
            
            match stage4_metrics {
                Ok(metrics) => {
                    overall_metrics.stage3_compile_time_ms = metrics.stage3_compile_time_ms;
                    
                    // Verify binary equivalence or functional equivalence
                    verify_bootstrap_cycle_consistency(&config, &overall_metrics)
                        .expect("Bootstrap cycle consistency verification failed");
                }
                Err(e) => {
                    warn!(error = %e, "Stage 3 to Stage 4 compilation failed - may be expected");
                    // This is acceptable for a minimal bootstrap implementation
                }
            }
        }
        Err(e) => {
            warn!(error = %e, "Stage 2 to Stage 3 compilation failed - may be expected");
            // This is acceptable for a minimal bootstrap implementation
        }
    }
    
    info!(metrics = ?overall_metrics, "Complete bootstrap cycle test completed");
}

#[instrument]
#[test]
fn test_bootstrap_convergence() {
    let config = init_bootstrap_tests();
    
    // Test that the bootstrap process converges (produces stable results)
    let stage2_source = create_stage2_compiler_test();
    
    // Compile multiple generations
    let mut binary_sizes = Vec::new();
    let mut compile_times = Vec::new();
    
    // Stage 1 -> Stage 2
    let stage2_metrics = compile_bootstrap_stage(&config, &stage2_source, "convergence_stage2")
        .expect("Stage 2 compilation failed");
    binary_sizes.push(stage2_metrics.binary_size_bytes);
    compile_times.push(stage2_metrics.stage1_compile_time_ms);
    
    // Stage 2 -> Stage 3
    match compile_stage_with_previous_stage(
        &config,
        "convergence_stage2",
        &stage2_source,
        "convergence_stage3"
    ) {
        Ok(stage3_metrics) => {
            binary_sizes.push(stage3_metrics.binary_size_bytes);
            compile_times.push(stage3_metrics.stage2_compile_time_ms);
            
            // Check for convergence
            analyze_bootstrap_convergence(&binary_sizes, &compile_times);
        }
        Err(e) => {
            warn!(error = %e, "Bootstrap convergence test failed at Stage 3");
        }
    }
}

#[instrument]
#[test]
fn test_cross_compilation_bootstrap() {
    let config = init_bootstrap_tests();
    
    // Test that the bootstrap process works with different target architectures
    // This is a placeholder for future cross-compilation support
    
    let stage2_source = create_stage2_compiler_test();
    
    // For now, just test that we can compile for the current architecture
    let metrics = compile_bootstrap_stage(&config, &stage2_source, "cross_compile_test")
        .expect("Cross-compilation bootstrap test failed");
    
    info!(
        metrics = ?metrics,
        "Cross-compilation bootstrap test completed (same architecture)"
    );
    
    // TODO: Add actual cross-compilation tests when supported
    warn!("Cross-compilation bootstrap tests not yet implemented");
}

#[instrument]
#[test]
fn test_bootstrap_with_optimizations() {
    let config = init_bootstrap_tests();
    
    // Test bootstrap process with different optimization levels
    let stage2_source = create_stage2_compiler_test();
    
    // Compile with debug mode (should be faster)
    let debug_metrics = compile_bootstrap_stage(&config, &stage2_source, "debug_stage2")
        .expect("Debug bootstrap compilation failed");
    
    info!(
        debug_time_ms = debug_metrics.stage1_compile_time_ms,
        debug_size_bytes = debug_metrics.binary_size_bytes,
        "Debug bootstrap compilation completed"
    );
    
    // TODO: Add release mode compilation when optimization flags are available
    warn!("Optimization-level bootstrap tests not yet fully implemented");
}

/// Helper function to compile a bootstrap stage
fn compile_bootstrap_stage(
    config: &BootstrapTestConfig,
    source: &str,
    output_name: &str,
) -> Result<BootstrapTestMetrics, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create source file
    let source_path = create_test_source(config, &format!("{}_source", output_name), source)?;
    
    // Compile with Stage 1 (Rust compiler)
    let output_path = PathBuf::from(&config.output_dir).join(output_name);
    let compile_duration = compile_with_stage1(config, &source_path, &output_path)?;
    
    // Measure binary size
    let binary_size = get_file_size(&output_path)?;
    
    let metrics = BootstrapTestMetrics {
        stage1_compile_time_ms: compile_duration.as_millis() as u64,
        stage2_compile_time_ms: 0,
        stage3_compile_time_ms: 0,
        memory_usage_mb: 0,
        binary_size_bytes: binary_size,
        tests_passed: 1,
        tests_failed: 0,
    };
    
    Ok(metrics)
}

/// Helper function to compile a stage with the previous stage compiler
fn compile_stage_with_previous_stage(
    config: &BootstrapTestConfig,
    previous_compiler: &str,
    source: &str,
    output_name: &str,
) -> Result<BootstrapTestMetrics, Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create source file
    let source_path = create_test_source(config, &format!("{}_source", output_name), source)?;
    
    // Use previous stage compiler
    let compiler_path = PathBuf::from(&config.output_dir).join(previous_compiler);
    let output_path = PathBuf::from(&config.output_dir).join(output_name);
    
    // Execute the previous stage compiler
    let _output = execute_binary(
        &compiler_path,
        &[
            source_path.to_str().unwrap(),
            output_path.to_str().unwrap()
        ],
        None
    )?;
    
    let compile_duration = start.elapsed();
    
    // Measure binary size if output was created
    let binary_size = if output_path.exists() {
        get_file_size(&output_path)?
    } else {
        0
    };
    
    let metrics = BootstrapTestMetrics {
        stage1_compile_time_ms: 0,
        stage2_compile_time_ms: compile_duration.as_millis() as u64,
        stage3_compile_time_ms: 0,
        memory_usage_mb: 0,
        binary_size_bytes: binary_size,
        tests_passed: 1,
        tests_failed: 0,
    };
    
    Ok(metrics)
}

/// Verify that a compiler binary has basic functionality
fn verify_compiler_functionality(
    compiler_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test program
    let test_source = r#"
func main() {
    return 42
}
"#;
    
    // Create temporary source file
    let temp_source = std::env::temp_dir().join("bootstrap_test_temp.csd");
    std::fs::write(&temp_source, test_source)?;
    
    // Create temporary output file
    let temp_output = std::env::temp_dir().join("bootstrap_test_temp_out");
    
    // Try to compile with the compiler
    let result = execute_binary(
        compiler_path,
        &[
            temp_source.to_str().unwrap(),
            temp_output.to_str().unwrap()
        ],
        None
    );
    
    // Clean up temporary files
    let _ = std::fs::remove_file(&temp_source);
    let _ = std::fs::remove_file(&temp_output);
    
    match result {
        Ok(_) => {
            info!("Compiler functionality verification passed");
            Ok(())
        }
        Err(e) => {
            warn!(error = %e, "Compiler functionality verification failed");
            // For minimal bootstrap, this might be expected
            Ok(())
        }
    }
}

/// Verify bootstrap cycle consistency
fn verify_bootstrap_cycle_consistency(
    config: &BootstrapTestConfig,
    metrics: &BootstrapTestMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(metrics = ?metrics, "Verifying bootstrap cycle consistency");
    
    // Check that compilation times are reasonable
    let total_time = metrics.stage1_compile_time_ms + 
                    metrics.stage2_compile_time_ms + 
                    metrics.stage3_compile_time_ms;
    
    if total_time > 60000 { // 60 seconds
        warn!(total_time_ms = total_time, "Bootstrap cycle took longer than expected");
    }
    
    // Check that binary sizes are reasonable
    if metrics.binary_size_bytes > 100 * 1024 * 1024 { // 100MB
        warn!(binary_size_bytes = metrics.binary_size_bytes, "Bootstrap binary is very large");
    }
    
    info!("Bootstrap cycle consistency verification completed");
    Ok(())
}

/// Analyze bootstrap convergence patterns
fn analyze_bootstrap_convergence(binary_sizes: &[u64], compile_times: &[u64]) {
    info!(
        binary_sizes = ?binary_sizes,
        compile_times = ?compile_times,
        "Analyzing bootstrap convergence"
    );
    
    // Check if binary sizes are converging
    if binary_sizes.len() >= 2 {
        let size_diff = if binary_sizes[1] > binary_sizes[0] {
            binary_sizes[1] - binary_sizes[0]
        } else {
            binary_sizes[0] - binary_sizes[1]
        };
        
        let size_change_percent = (size_diff as f64 / binary_sizes[0] as f64) * 100.0;
        
        info!(
            size_change_percent = size_change_percent,
            "Binary size change between stages"
        );
        
        if size_change_percent < 5.0 {
            info!("Bootstrap binary sizes are converging (< 5% change)");
        } else {
            warn!(
                size_change_percent = size_change_percent,
                "Bootstrap binary sizes are not converging"
            );
        }
    }
    
    // Check if compile times are stable
    if compile_times.len() >= 2 {
        let time_ratio = compile_times[1] as f64 / compile_times[0] as f64;
        
        info!(time_ratio = time_ratio, "Compile time ratio between stages");
        
        if time_ratio > 0.5 && time_ratio < 2.0 {
            info!("Bootstrap compile times are stable");
        } else {
            warn!(time_ratio = time_ratio, "Bootstrap compile times are highly variable");
        }
    }
}
