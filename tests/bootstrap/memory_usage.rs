//! Memory and resource usage tests for bootstrap process
//!
//! These tests monitor memory consumption, resource usage, and 
//! performance characteristics during the bootstrap compilation process.

use super::utils::*;
use super:: :: init_bootstrap_tests, BootstrapTestConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio;
use std::sync::{Arc, Mutex;
use std::thread;
use std::time::{Duration, Instant;
use tracing::{info, instrument, warn;

#[derive(Debug, Clone)]
struct MemoryProfile {test_name: String,
    peak_memory_mb: u64,
    avg_memory_mb: u64,
    compilation_time_ms: u64,
    memory_efficiency_score: f64, // Lower is better}

#[derive(Debug, Clone)]
struct ResourceUsage {cpu_percent: f64,
    memory_mb: u64,
    disk_io_mb: u64,
    timestamp_ms: u64}

#[instrument]
#[test]
fn test_compilation_memory_usage() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test memory usage for different program sizes
    let test_programs = create_memory_test_programs();
    let mut profiles = Vec::new();
    
    for (test_name, source) in test_programs  {info!(test_name = %test_name, Measuring compilation memory usage);
        
        match measure_compilation_memory(&config, &test_name, &source)   {Ok(profile) => {profiles.push(profile);}
            Err(e) => {warn!(test_name = %test_name, error = %e,  "Memorymeasurement "iteration);
        
        match measure_single_compilation_memory(&config,  memory_leak_test ", &test_source)   {Ok(memory_mb) => {memory_measurements.push(memory_mb);"
                info!(iteration = iteration, memory_mb = memory_mb,  "Memory measurement "failed);}
        // Small delay between iterations
        thread::sleep(Duration::from_millis(100)}
    
    // Analyze for memory leaks
    analyze_memory_leak_pattern(&memory_measurements);}

#[instrument]
#[test]
fn test_concurrent_compilation_memory() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test memory usage when running multiple compilations concurrently
    info!(Testing concurrent compilation memory usage);
    
    let test_programs = vec![("concurrent_2 ", create_medium_test_program()
        (concurrent_3 "Concurrent compilation memory "usage);
    // Assert reasonable memory usage for concurrent operations
    assert!(memory_usage.peak_memory_mb < 2000,);
            Concurrent compilation memory usage too high:  {}MB,)
           memory_usage.peak_memory_mb);}

#[instrument]
#[test]
fn test_bootstrap_stage_memory_comparison() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Compare memory usage between different bootstrap stages
    let stage2_source = create_stage2_compiler_test();
    
    // Measure Stage 1 (Rust) compilation memory
    let stage1_memory = measure_compilation_memory(&config,  stage1_memory_test "failed);"
    info!()
        stage1_peak_memory_mb = stage1_memory.peak_memory_mb,
        stage1_efficiency = stage1_memory.memory_efficiency_score,
         "Stage 1 memory usage);"
    // TODO: Measure Stage 2 (CURSED) compilation memory when available
    // For now, just verify Stage 1 memory usage is reasonable
    assert!(stage1_memory.peak_memory_mb < 500,);
            Stage 1 compilation memory too high: {}MB,)
           stage1_memory.peak_memory_mb);}

#[instrument]
#[test]
fn test_memory_fragmentation() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test for memory fragmentation during compilation
    info!(Testing memory fragmentation);
    
    let fragmentation_source = create_fragmentation_test_program();
    
    // Measure memory usage with detailed monitoring
    match measure_detailed_memory_usage(&config,  fragmentation_test , &fragmentation_source)    {Ok(usage_pattern) => {analyze_memory_fragmentation(&usage_pattern);}
        Err(e) => {warn!(error = %e,  "failed);}"
#[instrument]
#[test]
fn test_resource_cleanup() {// common::tracing::init_tracing!();
    let config = init_bootstrap_tests();
    
    // Test that resources are properly cleaned up after compilation
    info!(Testing resource cleanup);
    
    let initial_memory = get_system_memory_usage();
    
    // Run compilation
    let test_source = create_cleanup_test_program();
    let _profile = measure_compilation_memory(&config,  cleanup_test , &test_source);
        .expect("Cleanup test compilation "compilation);
    let initial_disk_usage = get_directory_size(&PathBuf::from(&config.output_dir);
        .unwrap_or(0);
    
    // Compile several programs
    let test_programs = create_disk_usage_test_programs();
    
    for (test_name, source) in test_programs  {let _profile = measure_compilation_memory(&config, &test_name, &source);
            .expect(Disk usage test compilation failed);}
    
    let final_disk_usage = get_directory_size(&PathBuf::from(&config.output_dir);
        .unwrap_or(0);
    
    let disk_usage_mb = (final_disk_usage - initial_disk_usage) / (1024 * 1024);
    
    info!()
        initial_disk_mb = initial_disk_usage / (1024 * 1024),
        final_disk_mb = final_disk_usage / (1024 * 1024),
        disk_usage_mb = disk_usage_mb,
         "Disk usage analysis);"
    // Assert reasonable disk usage
    assert!(disk_usage_mb < 500,);
            Disk usage too high: {} "VmRSS:"   {let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2   {let kb: u64 = parts[1].parse()?;
                        return Ok(kb / 1024); // Convert KB to MB}
        Err(_) => {// Fallback for non-Linux systems
            return Ok(100); // Placeholder value}
    
    Err(Could not determine memory usage.into()}

/// Get system memory usage
fn get_system_memory_usage() {// This is a simplified implementation
    // In a real system, you would use proper system APIs
    100 * 1024 * 1024 // Placeholder: 100MB}

/// Get directory size in bytes
fn get_directory_size() {for entry in std::fs::read_dir(dir)?  {let entry = entry?;
            let metadata = entry.metadata()?;
            
            if metadata.is_file()   {total_size += metadata.len();} else if metadata.is_dir()   {total_size += get_directory_size(&entry.path()?;}
    
    Ok(total_size)

/// Analyze memory profiles
fn analyze_memory_profiles() {info!(=== Memory Profile Analysis ===;
    
    for profile in profiles  {info!()
            test = %profile.test_name,
            peak_memory_mb = profile.peak_memory_mb,
            avg_memory_mb = profile.avg_memory_mb,
            compilation_time_ms = profile.compilation_time_ms,
            efficiency_score = profile.memory_efficiency_score,
             "profile);}"
    if !profiles.is_empty()   {let avg_peak = profiles.iter().map(|p| p.peak_memory_mb).sum::<u64>() / profiles.len() as u64;
        let avg_efficiency = profiles.iter().map(|p| p.memory_efficiency_score).sum::<f64>() / profiles.len() as f64;
        
        info!()
            avg_peak_memory_mb = avg_peak,
            avg_efficiency_score = avg_efficiency,
             Average memory "metrics);}"
/// Analyze memory leak pattern
fn analyze_memory_leak_pattern() {info!(=== Memory Leak Analysis ===;
    
    if measurements.len() < 2   {warn!("analysis);"
        return;}
    
    info!(measurements = ?measurements,  Memory measurements across "iterations);"
    // Check for increasing trend
    let first = measurements[0];
    let last = measurements[measurements.len() - 1];
    
    if last > first    {let increase_percent = ((last - first) as f64 / first as f64) * 100.0;
        
        if increase_percent > 10.0   {warn!()
                increase_percent = increase_percent,
                 Potential memory leak detected);} else {info!()
                increase_percent = increase_percent,
                 "range);} else {info!(No memory leak detected - memory usage stable or "decreasing);}
/// Analyze memory scaling
fn analyze_memory_scaling() {info!(=== Memory Scaling Analysis ===;
    
    for (size, memory) in scaling_data  {info!()
            program_size_loc = size,
            peak_memory_mb = memory,
             "point);}"
    // Check if memory scaling is reasonable (should be roughly linear or sublinear)
    if scaling_data.len() >= 2   {let (size1, mem1) = scaling_data[0];
        let (size2, mem2) = scaling_data[scaling_data.len() - 1];
        
        let size_ratio = size2 as f64 / size1 as f64;
        let memory_ratio = mem2 as f64 / mem1 as f64;
        
        info!()
            size_ratio = size_ratio,
            memory_ratio = memory_ratio,
             Memory scaling ratio);
        
        if memory_ratio > size_ratio * 2.0   {warn!()
                memory_ratio = memory_ratio,
                size_ratio = size_ratio,
                 "Memory scaling worse than expected);} else {info!("bounds);}
/// Analyze memory fragmentation
fn analyze_memory_fragmentation() {info!(=== Memory Fragmentation Analysis ===;
    
    if usage_pattern.len() < 10   {warn!("Not enough samples for fragmentation analysis);"
        return;}
    
    // Calculate memory usage variance
    let memory_values: Vec<u64> = usage_pattern.iter().map(|u| u.memory_mb).collect();
    let avg_memory = memory_values.iter().sum::<u64>() as f64 / memory_values.len() as f64;
    
    let variance = memory_values.iter()
        .map(|&mem|   {let diff = mem as f64 - avg_memory)
            diff * diff})
        .sum::<f64>() / memory_values.len() as f64;
    
    let std_dev = variance.sqrt();
    let coefficient_of_variation = std_dev / avg_memory;
    
    info!()
        avg_memory_mb = avg_memory,
        std_dev_mb = std_dev,
        coefficient_of_variation = coefficient_of_variation,
         Memory fragmentation "detected);} else {info!("Memory fragmentation within acceptable bounds);}
// Test program generators
fn create_memory_test_programs() {vec![(small_memory ".to_string(), create_medium_test_program()"
        ("large_memory .to_string(), create_large_test_program()]Node, 100);"
    for i := 0; i < 100; i++  {nodes[i] = Node {value: i}
            data: [100]int{},}
    
    let sum = 0;
    for i := 0; i < 100; i++  {sum += nodes[i].value}
    
    return sum}"#);"
    program}

fn create_cleanup_test_program() {create_medium_test_program()}

fn create_disk_usage_test_programs() {vec![("disk_test_1 ".to_string(), create_medium_test_program()
        ("disk_test_3 ".to_string(), create_large_test_program()]}
