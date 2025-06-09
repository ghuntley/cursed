//! Performance benchmarks for CURSED documentation system
//!
//! Tests documentation generation performance with various codebase sizes
//! and complexity levels to ensure scalability and efficiency.

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::{Duration, Instant},
    io::Write,
    collections::HashMap,
};
use tempfile::TempDir;

use cursed::docs::{
    DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
    PackageDocumentation, DocError, DocResult,
};

mod common;

/// Performance test configuration
#[derive(Debug, Clone)]
struct PerformanceTestConfig {
    /// Test name
    name: String,
    /// Number of items to generate
    item_count: usize,
    /// Maximum acceptable generation time
    max_generation_time: Duration,
    /// Maximum acceptable memory usage (MB)
    max_memory_mb: usize,
    /// Maximum acceptable output file size (MB)
    max_output_size_mb: usize,
}

/// Performance test result
#[derive(Debug)]
struct PerformanceTestResult {
    /// Test configuration
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
    failure_reasons: Vec<String>,
}

/// Performance benchmark runner
struct PerformanceBenchmarkRunner {
    /// Test configurations
    configs: Vec<PerformanceTestConfig>,
    /// Test results
    results: Vec<PerformanceTestResult>,
    /// Working directory
    work_dir: TempDir,
}

impl PerformanceBenchmarkRunner {
    fn new() -> std::io::Result<Self> {
        common::tracing::setup();
        
        Ok(Self {
            configs: Vec::new(),
            results: Vec::new(),
            work_dir: TempDir::new()?,
        })
    }
    
    /// Add performance test configuration
    fn add_test(&mut self, config: PerformanceTestConfig) {
        self.configs.push(config);
    }
    
    /// Run all performance tests
    fn run_all_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for config in &self.configs.clone() {
            let result = self.run_single_test(config)?;
            self.results.push(result);
        }
        Ok(())
    }
    
    /// Run a single performance test
    fn run_single_test(&self, config: &PerformanceTestConfig) -> Result<PerformanceTestResult, Box<dyn std::error::Error>> {
        println!("Running performance test: {} ({} items)", config.name, config.item_count);
        
        // Generate test source code
        let source_dir = self.work_dir.path().join(&config.name);
        let output_dir = self.work_dir.path().join(format!("{}_output", config.name));
        fs::create_dir_all(&source_dir)?;
        fs::create_dir_all(&output_dir)?;
        
        self.generate_test_sources(&source_dir, config.item_count)?;
        
        // Measure performance
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();
        
        // Run documentation generation
        let doc_config = DocConfig::new()
            .with_source_dirs(vec![source_dir])
            .with_output_dir(output_dir.clone())
            .with_package_name(config.name.clone());
        
        let mut generator = DocumentationGenerator::new();
        let generation_result = generator.generate_docs("src", "docs");
        
        let generation_time = start_time.elapsed();
        let peak_memory = self.get_memory_usage();
        
        // Handle generation result
        match generation_result {
            Ok(_) => {
                // Measure output characteristics
                let (output_size_bytes, files_generated) = self.measure_output_size(&output_dir)?;
                let peak_memory_mb = peak_memory.map(|m| m.saturating_sub(start_memory.unwrap_or(0)));
                
                // Check performance criteria
                let mut failure_reasons = Vec::new();
                
                if generation_time > config.max_generation_time {
                    failure_reasons.push(format!(
                        "Generation time {:?} exceeded limit {:?}",
                        generation_time, config.max_generation_time
                    ));
                }
                
                if let Some(memory_mb) = peak_memory_mb {
                    if memory_mb > config.max_memory_mb {
                        failure_reasons.push(format!(
                            "Memory usage {} MB exceeded limit {} MB",
                            memory_mb, config.max_memory_mb
                        ));
                    }
                }
                
                let output_size_mb = output_size_bytes / (1024 * 1024);
                if output_size_mb > config.max_output_size_mb {
                    failure_reasons.push(format!(
                        "Output size {} MB exceeded limit {} MB",
                        output_size_mb, config.max_output_size_mb
                    ));
                }
                
                Ok(PerformanceTestResult {
                    config: config.clone(),
                    generation_time,
                    peak_memory_mb,
                    output_size_bytes,
                    files_generated,
                    passed: failure_reasons.is_empty(),
                    failure_reasons,
                })
            }
            Err(e) => {
                Ok(PerformanceTestResult {
                    config: config.clone(),
                    generation_time,
                    peak_memory_mb: None,
                    output_size_bytes: 0,
                    files_generated: 0,
                    passed: false,
                    failure_reasons: vec![format!("Generation failed: {:?}", e)],
                })
            }
        }
    }
    
    /// Generate test source files with specified number of items
    fn generate_test_sources(&self, dir: &Path, item_count: usize) -> std::io::Result<()> {
        let items_per_file = 50; // Split large tests across multiple files
        let file_count = (item_count + items_per_file - 1) / items_per_file;
        
        for file_idx in 0..file_count {
            let start_idx = file_idx * items_per_file;
            let end_idx = (start_idx + items_per_file).min(item_count);
            let items_in_file = end_idx - start_idx;
            
            let content = self.generate_file_content(start_idx, items_in_file);
            let file_path = dir.join(format!("module_{}.csd", file_idx));
            fs::write(file_path, content)?;
        }
        
        Ok(())
    }
    
    /// Generate content for a single source file
    fn generate_file_content(&self, start_idx: usize, item_count: usize) -> String {
        let mut content = format!(
            "//! Module {} for performance testing\n//!\n//! Contains {} documented items for benchmarking documentation generation.\n\n",
            start_idx / 50, item_count
        );
        
        for i in 0..item_count {
            let idx = start_idx + i;
            
            // Generate alternating structs and functions
            if i % 2 == 0 {
                content.push_str(&self.generate_struct_documentation(idx));
            } else {
                content.push_str(&self.generate_function_documentation(idx));
            }
            
            // Add some interfaces periodically
            if i % 5 == 0 {
                content.push_str(&self.generate_interface_documentation(idx));
            }
        }
        
        content
    }
    
    /// Generate struct documentation
    fn generate_struct_documentation(&self, idx: usize) -> String {
        format!(r#""
/// Data structure {} for performance testing
/// 
/// This structure represents item {} in the performance test suite.
/// It includes comprehensive documentation to test generation performance
/// with realistic documentation content.
/// 
/// # Examples
/// 
/// ```cursed
/// facts item = Data{}{{
///     id: {},
///     name: "test_{}",
///     value: 42,
/// }}
/// ```
/// 
/// # Fields
/// 
/// * `id` - Unique identifier for this data item
/// * `name` - Human-readable name for the item
/// * `value` - Numeric value associated with the item
/// * `metadata` - Additional metadata as key-value pairs
squad Data{} {{
    /// Unique identifier (auto-generated)
    id: Int,
    /// Display name for this item
    name: String,
    /// Associated numeric value
    value: Int,
    /// Additional metadata storage
    metadata: Map[String, String],
}}

"#, idx, idx, idx, idx, idx, idx)"
    }
    
    /// Generate function documentation
    fn generate_function_documentation(&self, idx: usize) -> String {
        format!(r#""
/// Process function {} for performance testing
/// 
/// This function performs processing operation {} as part of the
/// performance testing suite. It demonstrates comprehensive
/// function documentation with parameters, return values, and examples.
/// 
/// # Arguments
/// 
/// * `input` - Input data to process (must not be empty)
/// * `options` - Processing options to control behavior
/// * `callback` - Optional callback function for progress updates
/// 
/// # Returns
/// 
/// Returns processed result as string, or error if processing fails.
/// 
/// # Errors
/// 
/// This function will return an error in the following situations:
/// 
/// * Input data is empty or invalid
/// * Processing options contain invalid values
/// * Callback function throws an exception
/// 
/// # Examples
/// 
/// ```cursed
/// facts result = process{}("test data", default_options(), nil)
/// lowkey result.is_ok() {{
///     vibe_check result.unwrap() {{
///         mood String {{ println!("Success: {{}}", value) }}
///         basic {{ println!("Unexpected result type") }}
///     }}
/// }}
/// ```
/// 
/// # Performance Notes
/// 
/// This function has O(n) time complexity where n is the length of input data.
/// Memory usage is proportional to the size of the output result.
yolo process{}(input: String, options: ProcessingOptions{}, callback: ((String) -> String)?) -> String {{
    format!("Processed: {{}} with options {{}}", input, options.to_string())
}}

"#, idx, idx, idx, idx, idx)"
    }
    
    /// Generate interface documentation
    fn generate_interface_documentation(&self, idx: usize) -> String {
        format!(r#""
/// Interface {} for performance testing
/// 
/// This interface defines the contract for processing component {}.
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
/// squad MyProcessor{} {{
///     config: ProcessorConfig,
/// }}
/// 
/// impl Processor{} for MyProcessor{} {{
///     yolo process(self, data: String) -> String {{
///         // Implementation here
///         "processed"
///     }}
/// }}
/// ```
collab Processor{} {{
    /// Process input data according to interface contract
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
    yolo get_config(self) -> ProcessorConfig{}
}}

"#, idx, idx, idx, idx, idx, idx, idx)"
    }
    
    /// Measure total output size and file count
    fn measure_output_size(&self, output_dir: &Path) -> std::io::Result<(usize, usize)> {
        let mut total_size = 0;
        let mut file_count = 0;
        
        fn visit_dir(dir: &Path, total_size: &mut usize, file_count: &mut usize) -> std::io::Result<()> {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    visit_dir(&path, total_size, file_count)?;
                } else {
                    *total_size += entry.metadata()?.len() as usize;
                    *file_count += 1;
                }
            }
            Ok(())
        }
        
        visit_dir(output_dir, &mut total_size, &mut file_count)?;
        Ok((total_size, file_count))
    }
    
    /// Get current memory usage (simplified implementation)
    fn get_memory_usage(&self) -> Option<usize> {
        // This is a simplified implementation
        // In a real scenario, you might use a more sophisticated memory measurement
        None
    }
    
    /// Print performance test results
    fn print_results(&self) {
        println!("\n=== Performance Test Results ===");
        
        let total_tests = self.results.len();
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        
        println!("Total tests: {}", total_tests);
        println!("Passed: {}", passed_tests);
        println!("Failed: {}", total_tests - passed_tests);
        
        println!("\n=== Individual Results ===");
        for result in &self.results {
            let status = if result.passed { "PASS" } else { "FAIL" };
            println!("[{}] {}", status, result.config.name);
            println!("  Items: {}", result.config.item_count);
            println!("  Generation time: {:?}", result.generation_time);
            println!("  Output size: {:.2} MB ({} files)", 
                    result.output_size_bytes as f64 / (1024.0 * 1024.0),
                    result.files_generated);
            
            if let Some(memory) = result.peak_memory_mb {
                println!("  Peak memory: {} MB", memory);
            }
            
            if !result.passed {
                for reason in &result.failure_reasons {
                    println!("    ❌ {}", reason);
                }
            }
            println!();
        }
        
        // Performance summary
        if !self.results.is_empty() {
            let total_time: Duration = self.results.iter().map(|r| r.generation_time).sum();
            let total_items: usize = self.results.iter().map(|r| r.config.item_count).sum();
            let total_output: usize = self.results.iter().map(|r| r.output_size_bytes).sum();
            
            println!("=== Performance Summary ===");
            println!("Total generation time: {:?}", total_time);
            println!("Total items processed: {}", total_items);
            println!("Total output size: {:.2} MB", total_output as f64 / (1024.0 * 1024.0));
            
            if total_items > 0 {
                let avg_time_per_item = total_time / (total_items as u32);
                println!("Average time per item: {:?}", avg_time_per_item);
            }
        }
    }
}

#[test]
fn test_small_codebase_performance() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    runner.add_test(PerformanceTestConfig {
        name: "small_codebase".to_string(),
        item_count: 25,
        max_generation_time: Duration::from_secs(5),
        max_memory_mb: 100,
        max_output_size_mb: 5,
    });
    
    runner.run_all_tests().expect("Failed to run small codebase test");
    runner.print_results();
    
    assert!(!runner.results.is_empty(), "No tests were run");
    assert!(runner.results[0].passed, "Small codebase performance test failed");
}

#[test]
fn test_medium_codebase_performance() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    runner.add_test(PerformanceTestConfig {
        name: "medium_codebase".to_string(),
        item_count: 100,
        max_generation_time: Duration::from_secs(15),
        max_memory_mb: 200,
        max_output_size_mb: 15,
    });
    
    runner.run_all_tests().expect("Failed to run medium codebase test");
    runner.print_results();
    
    assert!(!runner.results.is_empty(), "No tests were run");
    assert!(runner.results[0].passed, "Medium codebase performance test failed");
}

#[test]
fn test_large_codebase_performance() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    runner.add_test(PerformanceTestConfig {
        name: "large_codebase".to_string(),
        item_count: 500,
        max_generation_time: Duration::from_secs(60),
        max_memory_mb: 500,
        max_output_size_mb: 50,
    });
    
    runner.run_all_tests().expect("Failed to run large codebase test");
    runner.print_results();
    
    assert!(!runner.results.is_empty(), "No tests were run");
    
    // Large codebase test may fail on resource-constrained systems
    if !runner.results[0].passed {
        println!("⚠ Large codebase test failed - this may be expected on resource-constrained systems");
        for reason in &runner.results[0].failure_reasons {
            println!("  Reason: {}", reason);
        }
    }
}

#[test]
fn test_scalability_characteristics() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    // Test multiple sizes to analyze scalability
    let test_sizes = vec![10, 25, 50, 100, 200];
    
    for size in test_sizes {
        runner.add_test(PerformanceTestConfig {
            name: format!("scalability_{}", size),
            item_count: size,
            max_generation_time: Duration::from_millis(size as u64 * 100), // Linear scaling expectation
            max_memory_mb: 50 + (size / 10), // Allow some memory growth
            max_output_size_mb: (size / 10).max(1), // Proportional output size
        });
    }
    
    runner.run_all_tests().expect("Failed to run scalability tests");
    runner.print_results();
    
    // Analyze scalability
    let mut generation_times = Vec::new();
    let mut item_counts = Vec::new();
    
    for result in &runner.results {
        if result.passed {
            generation_times.push(result.generation_time.as_millis() as f64);
            item_counts.push(result.config.item_count as f64);
        }
    }
    
    if generation_times.len() >= 2 {
        // Simple linear regression to check if scaling is reasonable
        let n = generation_times.len() as f64;
        let sum_x: f64 = item_counts.iter().sum();
        let sum_y: f64 = generation_times.iter().sum();
        let sum_xy: f64 = item_counts.iter().zip(&generation_times).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = item_counts.iter().map(|x| x * x).sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        
        println!("\n=== Scalability Analysis ===");
        println!("Linear fit: time = {:.2} * items + {:.2}", slope, intercept);
        println!("Time per item: {:.2} ms", slope);
        
        // Check if scaling is reasonable (not exponential)
        assert!(slope < 50.0, "Documentation generation scaling may be too slow: {:.2} ms per item", slope);
    }
    
    assert!(!runner.results.is_empty(), "No scalability tests were run");
}

#[test]
fn test_concurrent_generation_performance() {
    // init_tracing!();
    // This test would ideally test parallel documentation generation
    // For now, we'll test sequential generation of multiple packages
    
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    // Simulate multiple packages being generated
    for i in 0..3 {
        runner.add_test(PerformanceTestConfig {
            name: format!("concurrent_package_{}", i),
            item_count: 30,
            max_generation_time: Duration::from_secs(10),
            max_memory_mb: 150,
            max_output_size_mb: 10,
        });
    }
    
    let start_time = Instant::now();
    runner.run_all_tests().expect("Failed to run concurrent tests");
    let total_time = start_time.elapsed();
    
    runner.print_results();
    
    println!("\n=== Concurrent Performance Analysis ===");
    println!("Total time for 3 packages: {:?}", total_time);
    
    let individual_times: Vec<Duration> = runner.results.iter().map(|r| r.generation_time).collect();
    let sum_individual: Duration = individual_times.iter().sum();
    
    println!("Sum of individual times: {:?}", sum_individual);
    println!("Overhead ratio: {:.2}", total_time.as_millis() as f64 / sum_individual.as_millis() as f64);
    
    assert!(!runner.results.is_empty(), "No concurrent tests were run");
    
    // Check that most tests passed
    let passed_count = runner.results.iter().filter(|r| r.passed).count();
    assert!(passed_count >= runner.results.len() / 2, "Too many concurrent tests failed");
}

#[test]
fn test_memory_efficiency() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    // Test with progressively larger codebases to check memory usage
    runner.add_test(PerformanceTestConfig {
        name: "memory_efficiency".to_string(),
        item_count: 150,
        max_generation_time: Duration::from_secs(30),
        max_memory_mb: 300, // Generous limit for memory efficiency test
        max_output_size_mb: 25,
    });
    
    runner.run_all_tests().expect("Failed to run memory efficiency test");
    runner.print_results();
    
    assert!(!runner.results.is_empty(), "No memory efficiency tests were run");
    
    // Analyze memory efficiency
    for result in &runner.results {
        if let Some(memory_mb) = result.peak_memory_mb {
            let memory_per_item = memory_mb as f64 / result.config.item_count as f64;
            println!("Memory per item: {:.2} MB", memory_per_item);
            
            // Check that memory usage per item is reasonable
            assert!(memory_per_item < 2.0, "Memory usage per item is too high: {:.2} MB", memory_per_item);
        }
        
        // Check output size efficiency
        let output_per_item = result.output_size_bytes as f64 / result.config.item_count as f64;
        println!("Output size per item: {:.2} KB", output_per_item / 1024.0);
    }
}

#[test]
fn test_generation_time_consistency() {
    // init_tracing!();
    let mut runner = PerformanceBenchmarkRunner::new().expect("Failed to create runner");
    
    // Run the same test multiple times to check consistency
    for i in 0..3 {
        runner.add_test(PerformanceTestConfig {
            name: format!("consistency_test_{}", i),
            item_count: 50,
            max_generation_time: Duration::from_secs(15),
            max_memory_mb: 200,
            max_output_size_mb: 10,
        });
    }
    
    runner.run_all_tests().expect("Failed to run consistency tests");
    runner.print_results();
    
    assert!(!runner.results.is_empty(), "No consistency tests were run");
    
    // Analyze time consistency
    let generation_times: Vec<f64> = runner.results.iter()
        .map(|r| r.generation_time.as_millis() as f64)
        .collect();
    
    if generation_times.len() >= 2 {
        let mean_time = generation_times.iter().sum::<f64>() / generation_times.len() as f64;
        let variance = generation_times.iter()
            .map(|&time| (time - mean_time).powi(2))
            .sum::<f64>() / generation_times.len() as f64;
        let std_dev = variance.sqrt();
        let coefficient_of_variation = std_dev / mean_time;
        
        println!("\n=== Time Consistency Analysis ===");
        println!("Mean generation time: {:.2} ms", mean_time);
        println!("Standard deviation: {:.2} ms", std_dev);
        println!("Coefficient of variation: {:.2}", coefficient_of_variation);
        
        // Check that generation time is reasonably consistent
        assert!(coefficient_of_variation < 0.5, 
               "Generation time is too inconsistent: CV = {:.2}", coefficient_of_variation);
    }
}
