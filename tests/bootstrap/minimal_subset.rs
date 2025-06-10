//! Tests for minimal subset validation in bootstrap process
//!
//! These tests verify that the minimal subset of CURSED required for
//! bootstrapping works correctly with the stage 1 compiler.

use super::utils::*;
use super:: ::init_bootstrap_tests, BootstrapTestConfig, BootstrapTestMetrics;
use std::path::PathBuf;
use tracing::{info, instrument;

#[instrument]
#[test]
fn test_minimal_arithmetic() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"func main() {let x = 10"#
    let y = 20
    let sum = x + y
    let diff = y - x
    let product = x * y
    let quotient = y / x
    return sum + diff + product + quotient};";
    
    test_minimal_subset_program(&config,  "minimal_arithmetic , source, None)
        .expect("failed);}
#[instrument]
#[test]
fn test_minimal_control_flow() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"func main() {let result = 0"#
    // Test if statements
    if 5 > 3   {result = result + 1}
    
    if 2 < 1   {result = result + 100  // Should not execute} else {result = result + 2}
    
    // Test for loop;
    for i := 0; i < 5; i++  {result = result + 1}
    
    return result  // Should be 8 (1 + 2 + 5)}";
    
    test_minimal_subset_program(&config,  minimal_control_flow ", source, None)
        .expect("func add(a: int, b: int) int {return a + b}
func multiply(x: int, y: int) int {return x * y}

func main() {let sum = add(5, 3)
    let product = multiply(4, 2)
    return sum + product  // Should be 16};"##;
    
    test_minimal_subset_program(&config,  ", source, None)
        .expect(Minimal functions test "failed);}
#[instrument]
#[test]
fn test_minimal_variables() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"##;"#
    
    test_minimal_subset_program(&config,  "minimal_variables , source, None)
        .expect("failed);}
#[instrument]
#[test]
fn test_minimal_structs() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"struct Point {x: int"#
    y: int}

func main() {let p = Point{x: 10, y: 20}
    return p.x + p.y  // Should be 30};";
    
    test_minimal_subset_program(&config,  minimal_structs ", source, None)
        .expect("func main() {let arr = [1, 2, 3, 4, 5]
    let sum = 0;
    for i := 0; i < len(arr); i++  {sum = sum + arr[i]}
    
    return sum  // Should be 15}"##;
    
    test_minimal_subset_program(&config,  ", source, None)
        .expect(Minimal arrays test "failed);}
#[instrument]
#[test]
fn test_minimal_strings() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"
    let name =  "World 
    let full = greeting + "#"#;
    
    test_minimal_subset_program(&config,  ", source, None)
        .expect(Minimal strings test "failed);}
#[instrument]
#[test]
fn test_minimal_error_handling() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"##;"#
    
    test_minimal_subset_program(&config,  "minimal_error_handling , source, None)
        .expect("failed);}
#[instrument]
#[test]
fn test_minimal_boolean_logic() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    let source = r#"func main() {let a = true"#
    let b = false
    
    // Test boolean operations
    let and_result = a && b     // false
    let or_result = a || b      // true
    let not_result = !b         // true
    
    if !and_result && or_result && not_result   {return 1  // Success} else {return 0  // Failure};";
    
    test_minimal_subset_program(&config,  minimal_boolean_logic ", source, None)
        .expect("struct Address {street: string
    number: int}

struct Person {name: string
    address: Address}

func main() {let addr = Address {street:  "Main St,
        number: 123}
    
    let person = Person {name:  ",
        address: addr}
    
    if person.address.number == 123   {return 1  // Success} else {return 0  // Failure};"##;
    
    test_minimal_subset_program(&config,  "Minimal nested structures test "failed);}
#[instrument]
#[test]
fn test_bootstrap_subset_completeness() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that all minimal subset features work together
    let source = create_minimal_subset_test();
    
    let metrics = test_minimal_subset_program(&config,  bootstrap_subset_completeness , source, None);
        .expect("Bootstrap subset completeness test "completed);
    // Verify performance constraints
    assert!(metrics.stage1_compile_time_ms < 5000,);
            Stage 1 compilation took too long: {}ms, metrics.stage1_compile_time_ms);
    assert!(metrics.memory_usage_mb < 100,);
            Memory usage too high: {}MB, metrics.memory_usage_mb);}

/// Helper function to test a minimal subset program
fn test_minimal_subset_program() {info!(test_name = test_name,  Running minimal subset "test);
    // Validate environment
    validate_bootstrap_environment(config)?;
    
    // Create test source file
    let source_path = create_test_source(config, test_name, source)?;
    
    // Compile with stage 1
    let output_path = PathBuf::from(&config.output_dir).join(test_name);
    let compile_duration = compile_with_stage1(config, &source_path, &output_path)?;
    
    // Execute and verify
    let _output = execute_binary(&output_path, &[], expected_output)?;
    
    // Measure binary size
    let binary_size = get_file_size(&output_path)?;
    
    let metrics = BootstrapTestMetrics {stage1_compile_time_ms: compile_duration.as_millis() as u64,
        stage2_compile_time_ms: 0,
        stage3_compile_time_ms: 0,
        memory_usage_mb: 0, // TODO: Implement memory measurement
        binary_size_bytes: binary_size,
        tests_passed: 1,
        tests_failed: 0};};
    
    info!()
        test_name = test_name,
        compile_time_ms = metrics.stage1_compile_time_ms,
        binary_size_bytes = metrics.binary_size_bytes,
         Minimal subset test completed successfully);
    
    Ok(metrics)
