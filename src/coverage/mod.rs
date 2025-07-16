/// Code Coverage Analysis for CURSED
/// 
/// Provides comprehensive code coverage collection and reporting
/// for both CURSED and Rust code during test execution.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::lexer::Position;

pub mod collector;
pub mod reporter;
pub mod instrumentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageData {
    pub files: HashMap<String, FileCoverage>,
    pub summary: CoverageSummary,
    pub timestamp: String,
    pub test_run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub path: String,
    pub lines: HashMap<u32, LineCoverage>,
    pub functions: HashMap<String, FunctionCoverage>,
    pub branches: HashMap<String, BranchCoverage>,
    pub total_lines: u32,
    pub covered_lines: u32,
    pub coverage_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineCoverage {
    pub line_number: u32,
    pub execution_count: u64,
    pub is_executable: bool,
    pub is_covered: bool,
    pub source_line: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCoverage {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub execution_count: u64,
    pub is_covered: bool,
    pub complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCoverage {
    pub line_number: u32,
    pub branch_id: String,
    pub condition: String,
    pub true_count: u64,
    pub false_count: u64,
    pub is_covered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageSummary {
    pub total_files: u32,
    pub total_lines: u32,
    pub covered_lines: u32,
    pub line_coverage_percentage: f64,
    pub total_functions: u32,
    pub covered_functions: u32,
    pub function_coverage_percentage: f64,
    pub total_branches: u32,
    pub covered_branches: u32,
    pub branch_coverage_percentage: f64,
}

/// Main coverage analyzer that orchestrates coverage collection and reporting
pub struct CoverageAnalyzer {
    config: CoverageConfig,
    collector: collector::CoverageCollector,
    reporter: reporter::CoverageReporter,
}

#[derive(Debug, Clone)]
pub struct CoverageConfig {
    pub output_dir: PathBuf,
    pub source_dirs: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub formats: Vec<OutputFormat>,
    pub min_coverage_threshold: f64,
    pub collect_branch_coverage: bool,
    pub collect_function_coverage: bool,
    pub enable_instrumentation: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Html,
    Xml,
    Lcov,
    Console,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("coverage"),
            source_dirs: vec![PathBuf::from("src"), PathBuf::from("stdlib")],
            exclude_patterns: vec![
                "*/target/*".to_string(),
                "*/tests/*".to_string(),
                "*_test.csd".to_string(),
                "test_*.csd".to_string(),
            ],
            include_patterns: vec!["*.rs".to_string(), "*.csd".to_string()],
            formats: vec![OutputFormat::Html, OutputFormat::Json],
            min_coverage_threshold: 80.0,
            collect_branch_coverage: true,
            collect_function_coverage: true,
            enable_instrumentation: true,
        }
    }
}

impl CoverageAnalyzer {
    pub fn new(config: CoverageConfig) -> io::Result<Self> {
        // Create output directory
        fs::create_dir_all(&config.output_dir)?;
        
        let collector = collector::CoverageCollector::new(config.clone())?;
        let reporter = reporter::CoverageReporter::new(config.clone())?;
        
        Ok(Self {
            config,
            collector,
            reporter,
        })
    }

    /// Start coverage collection for a test run
    pub async fn start_coverage(&mut self, test_run_id: &str) -> io::Result<()> {
        println!("🎯 Starting coverage collection for test run: {}", test_run_id);
        self.collector.start_collection(test_run_id).await
    }

    /// Stop coverage collection and generate reports
    pub async fn stop_coverage(&mut self) -> io::Result<CoverageData> {
        println!("📊 Stopping coverage collection and generating reports...");
        let coverage_data = self.collector.stop_collection().await?;
        
        // Generate reports in all configured formats
        for format in &self.config.formats {
            self.reporter.generate_report(&coverage_data, format).await?;
        }
        
        // Check coverage thresholds
        self.check_coverage_thresholds(&coverage_data)?;
        
        Ok(coverage_data)
    }

    /// Instrument source files for coverage collection
    pub fn instrument_source_files(&self) -> io::Result<()> {
        if !self.config.enable_instrumentation {
            return Ok(());
        }
        
        println!("🔧 Instrumenting source files for coverage...");
        instrumentation::instrument_cursed_files(&self.config.source_dirs, &self.config.output_dir)?;
        Ok(())
    }

    /// Check if coverage meets minimum thresholds
    fn check_coverage_thresholds(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let summary = &coverage_data.summary;
        
        println!("\n📈 Coverage Summary:");
        println!("  Lines: {:.2}% ({}/{})", 
                 summary.line_coverage_percentage, 
                 summary.covered_lines, 
                 summary.total_lines);
        println!("  Functions: {:.2}% ({}/{})", 
                 summary.function_coverage_percentage, 
                 summary.covered_functions, 
                 summary.total_functions);
        println!("  Branches: {:.2}% ({}/{})", 
                 summary.branch_coverage_percentage, 
                 summary.covered_branches, 
                 summary.total_branches);
        
        if summary.line_coverage_percentage < self.config.min_coverage_threshold {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Coverage threshold not met: {:.2}% < {:.2}%", 
                        summary.line_coverage_percentage, 
                        self.config.min_coverage_threshold)
            ));
        }
        
        println!("✅ Coverage threshold met: {:.2}%", summary.line_coverage_percentage);
        Ok(())
    }
}

/// CLI interface for coverage analysis
pub async fn run_coverage_analysis(
    test_command: &str,
    config: CoverageConfig,
) -> io::Result<CoverageData> {
    let mut analyzer = CoverageAnalyzer::new(config)?;
    
    // Generate unique test run ID
    let test_run_id = format!("test-{}", chrono::Utc::now().timestamp());
    
    // Instrument source files if needed
    analyzer.instrument_source_files()?;
    
    // Start coverage collection
    analyzer.start_coverage(&test_run_id).await?;
    
    // Execute tests with coverage
    let test_result = execute_tests_with_coverage(test_command).await?;
    
    // Stop coverage collection and generate reports
    let coverage_data = analyzer.stop_coverage().await?;
    
    if !test_result.success {
        eprintln!("⚠️  Some tests failed, but coverage report generated");
    }
    
    Ok(coverage_data)
}

#[derive(Debug)]
struct TestResult {
    success: bool,
    output: String,
    stderr: String,
}

async fn execute_tests_with_coverage(test_command: &str) -> io::Result<TestResult> {
    use std::process::Command;
    
    // Set environment variables for coverage collection
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(test_command)
       .env("CURSED_COVERAGE_ENABLED", "1")
       .env("CURSED_COVERAGE_OUTPUT", "coverage/raw");
    
    let output = cmd.output()?;
    
    Ok(TestResult {
        success: output.status.success(),
        output: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}
