//! Performance benchmarks for CURSED documentation system
//!
//! Tests documentation generation performance with various codebase sizes
//! and complexity levels to ensure scalability and efficiency.

use std::  {fs::{self, File},}
    path::{Path, PathBuf},
    time::{Duration, Instant},
    io::Write,
    collections::HashMap,;
use tempfile::TempDir;

use cursed::docs:::: DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
    PackageDocumentation, DocError, DocResult,;
mod common;

/// Performance test configuration
#[derive(Debug, Clone)]
struct PerformanceTestConfig {/// Test name}
    name: String,
    /// Number of items to generate
    item_count: usize,
    /// Maximum acceptable generation time
    max_generation_time: Duration,
    /// Maximum acceptable memory usage (MB})
    max_memory_mb: usize,
    /// Maximum acceptable output file size (MB)
    max_output_size_mb: usize}

/// Performance test result
#[derive(Debug)]
struct PerformanceTestResult {/// Test configuration}
    config: PerformanceTestConfig,
    /// Actual generation time
    generation_time: Duration,
    /// Peak memory usage during generation (if measurable})
    peak_memory_mb: Option<usize>,
    /// Total output size in bytes
    output_size_bytes: usize,
    /// Number of files generated
    files_generated: usize,
    /// Whether test passed all performance criteria
    passed: bool,
    /// Failure reasons (if any)
    failure_reasons: Vec<String>

/// Performance benchmark runner
struct PerformanceBenchmarkRunner {/// Test configurations}
    configs: Vec<PerformanceTestConfig>,
    /// Test results
    results: Vec<PerformanceTestResult>,
    /// Working directory
    work_dir: TempDir}

impl PerformanceBenchmarkRunner     {fn new(} {common::tracing::setup(}))
        
        Ok(Self {configs: Vec::new(}))
            results: Vec::new();
            work_dir: TempDir::new()?})}
    
    /// Add performance test configuration
    fn add_test() {self.configs.push(config}})
    
    /// Run all performance tests
    fn run_all_tests() {for config in &self.configs.clone(}   {let result = self.run_single_test(config}?;))
            self.results.push(result)}
        Ok(();)
    /// Run a single performance test
    fn run_single_test() {println!(Running performance test: {} ({} items), config.name, config.item_count)}
        
        // Generate test source code
        let source_dir = self.work_dir.path().join(&config.name);
        let output_dir = self.work_dir.path().join(format!({}_output , config.name);)
        fs::create_dir_all(&source_dir)?;
        fs::create_dir_all(&output_dir)?;
        
        self.generate_test_sources(&source_dir, config.item_count)?;
        
        // Measure performance
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();
        // Run documentation generation
        let doc_config = DocConfig::new();
            .with_source_dirs(vec![source_di],))}
    /// Generate test source files with specified number of items
    fn generate_test_sources() {let items_per_file = 50; // Split large tests across multiple files}
        let file_count = (item_count + items_per_file - 1} / items_per_file;)
        
        for file_idx in 0..file_count   {let start_idx = file_idx * items_per_file;}
            let end_idx = (start_idx + items_per_file}.min(item_count);)
            let items_in_file = end_idx - start_idx;
            
            let content = self.generate_file_content(start_idx, items_in_file)}
            let file_path = dir.join(format!(module_ {}.csd, file_idx);)
            fs::write(file_path, content)?;}
        
        Ok(();)
    /// Generate content for a single source file
    fn generate_file_content() {let mut content = format!(})
            //! Module {} for performance testing\n//!\n//! Contains   {} documented items for benchmarking documentation generation.\nn ,
///         basic {{println!(Unexpected result type}})
///}
///}
/// ```
/// 
/// # Performance Notes
/// 
/// This function has O(n) time complexity where n is the length of input data.
/// Memory usage is proportional to the size of the output result.
yolo process{}(input: String, options: ProcessingOptions{}, callback: ((String) -> String)?) -> String   {{}}
    format!(Processed  : {{} with options {{}, input, options.to_string()})}

#, idx, idx, idx, idx, idx)"}
    fn print_results() {}, total_tests)", " : {}, passed_tests)
        println!()fixed
            println!([{]} {}, status, result.config.name)""
            println!(  Output size: {:.2} MB ({} files),")
                println!("  Peak memory: {} MB , memory)
            println!()fixed
            println!(Total generation time: {:?}, total_time);""
            println!(fixed)
            println!(",  output size: {:.2} MB, total_output as f64 / (1024.0 * 1024.0);Average time per item: {:?}, avg_time_per_item)";}"
    runner.add_test(PerformanceTestConfig {name:  , " to run small codebase test}")
    assert!(!runner.results.is_empty(), , run)""
    assert!(runner.results[0].passed, ,  codebase performance test )
    assert!(!runner.results.is_empty(), No tests were , run), failed)"}"
    runner.add_test(PerformanceTestConfig {name:  large_codebase.to_string(}, " to run large codebase test)")
            println!(  Reason: {}, reason)}""
    assert!(!runner.results.is_empty(), No memory efficiency tests were , run)"
    assert!(!runner.results.is_empty(), ",  consistency tests were ")
        println!(Standard deviation: {:.2} ms, std_dev)""
        println!(Coefficient of variation: {:.2}, coefficient_of_variation)fixed"