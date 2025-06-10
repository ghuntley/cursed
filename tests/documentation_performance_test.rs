//! Performance benchmarks for CURSED documentation system
//!
//! Tests documentation generation performance with various codebase sizes
//! and complexity levels to ensure scalability and efficiency.

use std::  {fs::{self, File},
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
struct PerformanceTestConfig {/// Test name
    name: String,
    /// Number of items to generate
    item_count: usize,
    /// Maximum acceptable generation time
    max_generation_time: Duration,
    /// Maximum acceptable memory usage (MB)
    max_memory_mb: usize,
    /// Maximum acceptable output file size (MB)
    max_output_size_mb: usize}

/// Performance test result
#[derive(Debug)]
struct PerformanceTestResult {/// Test configuration
    config: PerformanceTestConfig,
    /// Actual generation time
    generation_time: Duration,
    /// Peak memory usage during generation (if measurable)
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
struct PerformanceBenchmarkRunner {/// Test configurations
    configs: Vec<PerformanceTestConfig>,
    /// Test results
    results: Vec<PerformanceTestResult>,
    /// Working directory
    work_dir: TempDir}

impl PerformanceBenchmarkRunner     {fn new() {common::tracing::setup()
        
        Ok(Self {configs: Vec::new()
            results: Vec::new()
            work_dir: TempDir::new()?})}
    
    /// Add performance test configuration
    fn add_test() {self.configs.push(config)}
    
    /// Run all performance tests
    fn run_all_tests() {for config in &self.configs.clone()   {let result = self.run_single_test(config)?;
            self.results.push(result)}
        Ok(()
    
    /// Run a single performance test
    fn run_single_test() {println!(Running performance test: {} ({} items), config.name, config.item_count)
        
        // Generate test source code
        let source_dir = self.work_dir.path().join(&config.name)
        let output_dir = self.work_dir.path().join(format!({}_output , config.name);
        fs::create_dir_all(&source_dir)?;
        fs::create_dir_all(&output_dir)?;
        
        self.generate_test_sources(&source_dir, config.item_count)?;
        
        // Measure performance
        let start_time = Instant::now()
        let start_memory = self.get_memory_usage()
        
        // Run documentation generation
        let doc_config = DocConfig::new()
            .with_source_dirs(vec![source_di],})}
    /// Generate test source files with specified number of items
    fn generate_test_sources() {let items_per_file = 50; // Split large tests across multiple files
        let file_count = (item_count + items_per_file - 1) / items_per_file;
        
        for file_idx in 0..file_count   {let start_idx = file_idx * items_per_file;
            let end_idx = (start_idx + items_per_file).min(item_count);
            let items_in_file = end_idx - start_idx;
            
            let content = self.generate_file_content(start_idx, items_in_file)}
            let file_path = dir.join(format!(module_ {}.csd, file_idx);
            fs::write(file_path, content)?;}
        
        Ok(()
    
    /// Generate content for a single source file
    fn generate_file_content() {let mut content = format!(}
            //! Module {} for performance testing\n//!\n//! Contains   {} documented items for benchmarking documentation generation.\nn ,
///         basic {{println!(Unexpected result type)}
///}
///}
/// ```
/// 
/// # Performance Notes
/// 
/// This function has O(n) time complexity where n is the length of input data.
/// Memory usage is proportional to the size of the output result.
yolo process{}(input: String, options: ProcessingOptions{}, callback: ((String) -> String)?) -> String   {{}
    format!(Processed  : {{} with options {{}, input, options.to_string()}

#, idx, idx, idx, idx, idx)"}
    /// Generate interface documentation
    fn generate_interface_documentation() {format!(r#}
/// Interface {} for performance testing
/// 
/// This interface defines the contract for processing component   {}.
/// It includes method specifications with comprehensive documentation
/// to test interface documentation generation performance.
/// 
/// # Implementation Requirements
/// 
/// Implementing types must provide efficient implementations of all methods.
/// Performance characteristics should be documented in implementation comments.
/// 
/// # Examples
/// 
/// ```cursed
/// squad MyProcessor{} {{///     config: ProcessorConfig,
///}
/// 
/// impl Processor    {} for MyProcessor  {} {{///     yolo process(self, data: String) -> String   {{///         // Implementation here
///          processed}
///}
///}
/// ```
collab Processor{} {{/// Process input data according to interface contract
    /// 
    /// # Arguments
    /// * `data` - Input data to process
    /// 
    /// # Returns
    /// Processed output string
    /// 
    /// # Performance
    /// Implementations should complete processing in O(n) time
    yolo process(self, data: String) -> String
    
    /// Validate input data before processing
    /// 
    /// # Arguments
    /// * `data` - Data to validate
    /// 
    /// # Returns
    /// True if data is valid for processing
    yolo validate(self, data: String) -> Bool
    
    /// Get processor configuration information
    /// 
    /// # Returns
    /// Current processor configuration
    yolo get_config(self) -> ProcessorConfig        {}

#, idx, idx, idx, idx, idx, idx, idx)}
    
    /// Measure total output size and file count
    fn measure_output_size() {let mut total_size = 0;
        let mut file_count = 0;
        
        fn visit_dir() {for entry in fs::read_dir(dir)?   {let entry = entry?;
                let path = entry.path()
                
                if path.is_dir()     {;
                    visit_dir(&path, total_size, file_count)?;} else {*total_size += entry.metadata()?.len() as usize;
                    *file_count += 1;}
            Ok(()
        
        visit_dir(output_dir, &mut total_size, &mut file_count)?;
        Ok((total_size, file_count)
    
    /// Get current memory usage (simplified implementation)
    fn get_memory_usage() {// This is a simplified implementation
        // In a real scenario, you might use a more sophisticated memory measurement
        None}
    
    /// Print performance test results
    fn print_results() {}, total_tests)"Passed : {}, passed_tests)")
        println!(");
        println!("\n=== Individual Results ==="FAIL}
            println!("[{}] {}, status, result.config.name)
            println!("  Output size: {:.2} MB ({} files), 
                    result.output_size_bytes as f64 / (1024.0 * 1024.0),
                    result.files_generated)
            
            if let Some(memory) = result.peak_memory_mb     {}
                println!("  Peak memory: {} MB , memory)"    ❌ {}, reason)}
            println!()}
        
        // Performance summary
        if !self.results.is_empty()     {let total_time: Duration = self.results.iter().map(|r| r.generation_time).sum()
            let total_items: usize = self.results.iter().map(|r| r.config.item_count).sum()
            let total_output: usize = self.results.iter().map(|r| r.output_size_bytes).sum();
            println!(=== Performance Summary ===;
            println!(Total generation time: {:?}, total_time);"
            println!("
            println!("Total output size: {:.2} MB, total_output as f64 / (1024.0 * 1024.0);"Average time per item: {:?}, avg_time_per_item)";}
#[test]
fn test_small_codebase_performance() {// common::tracing::init_tracing!()
    let mut runner = PerformanceBenchmarkRunner::new().expect(Failed to create runner)
    
    runner.add_test(PerformanceTestConfig {name:  "Failed to run small codebase test)")
    runner.print_results()
    
    assert!(!runner.results.is_empty(), ", run)
    assert!(runner.results[0].passed, "Small codebase performance test "medium_codebase.to_string()
        item_count: 100,
        max_generation_time: Duration::from_secs(15),
        max_memory_mb: 200,
        max_output_size_mb: 15})
    
    runner.run_all_tests().expect(Failed to run medium codebase test)
    runner.print_results()
    
    assert!(!runner.results.is_empty(), No tests were ", run)", failed)"}
#[test]
fn test_large_codebase_performance() {// common::tracing::init_tracing!()
    let mut runner = PerformanceBenchmarkRunner::new().expect(Failed to create runner)
    
    runner.add_test(PerformanceTestConfig {name:  large_codebase.to_string()"Failed to run large codebase test)
    runner.print_results()
    
    assert!(!runner.results.is_empty(), 
    
    // Large codebase test may fail on resource-constrained systems
    if !runner.results[0].passed     {println!(⚠ Large codebase test failed - this may be expected on resource-constrained systems);
        for reason in &runner.results[0].failure_reasons   {}
            println!("  Reason: {}, reason)}
#[test]
fn test_scalability_characteristics() {// common::tracing::init_tracing!()
    let mut runner = PerformanceBenchmarkRunner::new().expect(Failed to create runner)
    
    // Test multiple sizes to analyze scalability
    let test_sizes = vec![10, 25, 50, 100, 20]
fn test_memory_efficiency() {// common::tracing::init_tracing!()
    let mut runner = PerformanceBenchmarkRunner::new().expect(Failed to create runner)
    
    // Test with progressively larger codebases to check memory usage
    runner.add_test(PerformanceTestConfig {name:  memory_efficiency.to_string()
        item_count: 150,
        max_generation_time: Duration::from_secs(30),
        max_memory_mb: 300, // Generous limit for memory efficiency test
        max_output_size_mb: 25})
    
    runner.run_all_tests().expect(Failed to run memory efficiency test)
    runner.print_results()
    
    assert!(!runner.results.is_empty(), No memory efficiency tests were ", run)")
    runner.print_results()
    
    assert!(!runner.results.is_empty(), "No consistency tests were ")"
        println!(Standard deviation: {:.2} ms, std_dev)"
        println!(Coefficient of variation: {:.2}, coefficient_of_variation)")
        
        // Check that generation time is reasonably consistent
        assert!(coefficient_of_variation < 0.5, Generation time is too inconsistent: CV = {:.2}, , coefficient_of_variation);}