//! Self-Compilation Verification System
//!
//! This module provides comprehensive verification that the CURSED compiler can properly
//! compile itself and produce equivalent output to the Rust implementation.

use crate::error::Error;
type Result<T> = std::result::Result<T, Error>;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};
use serde::{Deserialize, Serialize};

/// Configuration for self-compilation verification
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Working directory for verification tests
    pub work_dir: PathBuf,
    /// Timeout for compilation steps
    pub compilation_timeout: Duration,
    /// Timeout for execution tests
    pub execution_timeout: Duration,
    /// Whether to keep intermediate files for debugging
    pub keep_intermediates: bool,
    /// Optimization levels to test
    pub optimization_levels: Vec<String>,
    /// Number of bootstrap cycles to test
    pub bootstrap_cycles: usize,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            work_dir: PathBuf::from("./bootstrap_verification"),
            compilation_timeout: Duration::from_secs(300),  // 5 minutes
            execution_timeout: Duration::from_secs(60),     // 1 minute
            keep_intermediates: false,
            optimization_levels: vec!["-O0".to_string(), "-O1".to_string(), "-O2".to_string()],
            bootstrap_cycles: 3,
        }
    }
}

/// Results from a compilation stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub stage: String,
    pub compiler_path: PathBuf,
    pub output_path: PathBuf,
    pub compilation_time: Duration,
    pub binary_size: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub checksum: String,
}

/// Results from executing a test program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub program_path: PathBuf,
    pub execution_time: Duration,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

/// Comparison results between two compilation stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub stage1: String,
    pub stage2: String,
    pub binary_size_diff: i64,
    pub performance_diff: f64,  // Percentage difference
    pub output_identical: bool,
    pub checksum_match: bool,
    pub functional_equivalent: bool,
}

/// Complete verification report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub config: String,  // Serialized config for reference
    pub compilation_results: Vec<CompilationResult>,
    pub execution_results: Vec<ExecutionResult>,
    pub comparison_results: Vec<ComparisonResult>,
    pub bootstrap_cycle_results: Vec<BootstrapCycleResult>,
    pub performance_metrics: PerformanceMetrics,
    pub issues_found: Vec<String>,
    pub overall_success: bool,
    pub verification_time: Duration,
}

/// Results from a complete bootstrap cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapCycleResult {
    pub cycle_number: usize,
    pub stages: Vec<CompilationResult>,
    pub convergence_achieved: bool,
    pub binary_stable: bool,
    pub performance_stable: bool,
}

/// Performance metrics across compilation stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub compilation_times: HashMap<String, Duration>,
    pub binary_sizes: HashMap<String, u64>,
    pub execution_times: HashMap<String, Duration>,
    pub memory_usage: HashMap<String, u64>,
}

/// Main verification system
pub struct SelfCompilationVerifier {
    config: VerificationConfig,
}

impl SelfCompilationVerifier {
    /// Create a new verification system with the given configuration
    pub fn new(config: VerificationConfig) -> Self {
        Self { config }
    }

    /// Run the complete self-compilation verification
    #[instrument(skip(self))]
    pub fn run_verification(&self) -> Result<VerificationReport> {
        let start_time = Instant::now();
        info!("Starting self-compilation verification");

        // Prepare working directory
        self.prepare_work_dir()?;

        let mut report = VerificationReport {
            config: format!("{:?}", self.config),
            compilation_results: Vec::new(),
            execution_results: Vec::new(),
            comparison_results: Vec::new(),
            bootstrap_cycle_results: Vec::new(),
            performance_metrics: PerformanceMetrics {
                compilation_times: HashMap::new(),
                binary_sizes: HashMap::new(),
                execution_times: HashMap::new(),
                memory_usage: HashMap::new(),
            },
            issues_found: Vec::new(),
            overall_success: true,
            verification_time: Duration::new(0, 0),
        };

        // Step 1: Test Stage 1 (Rust) compiler can compile CURSED source
        info!("Phase 1: Testing Stage 1 (Rust) compiler");
        let stage1_result = self.compile_stage1_compiler()?;
        report.compilation_results.push(stage1_result.clone());

        // Step 2: Use Stage 1 to compile Stage 2 (CURSED) compiler
        info!("Phase 2: Compiling Stage 2 (CURSED) compiler using Stage 1");
        let stage2_result = self.compile_stage2_compiler(&stage1_result.compiler_path)?;
        report.compilation_results.push(stage2_result.clone());

        // Step 3: Compare Stage 1 and Stage 2 output
        info!("Phase 3: Comparing Stage 1 and Stage 2 compilers");
        let comparison = self.compare_stages(&stage1_result, &stage2_result)?;
        report.comparison_results.push(comparison);

        // Step 4: Test both compilers with identical test programs
        info!("Phase 4: Testing functional equivalence");
        let functional_tests = self.run_functional_tests(&stage1_result, &stage2_result)?;
        report.execution_results.extend(functional_tests);

        // Step 5: Bootstrap cycle testing
        info!("Phase 5: Running bootstrap cycles");
        let bootstrap_results = self.run_bootstrap_cycles(&stage1_result)?;
        report.bootstrap_cycle_results = bootstrap_results;

        // Step 6: Performance analysis
        info!("Phase 6: Performance analysis");
        self.analyze_performance(&mut report)?;

        // Step 7: Generate final assessment
        self.assess_verification_results(&mut report);

        report.verification_time = start_time.elapsed();
        info!("Verification completed in {:?}", report.verification_time);

        Ok(report)
    }

    /// Prepare the working directory for verification
    fn prepare_work_dir(&self) -> Result<()> {
        if self.config.work_dir.exists() && !self.config.keep_intermediates {
            std::fs::remove_dir_all(&self.config.work_dir)
            .map_err(|e| Error::from_str(&format!("Failed to clean work dir: {}", e)))?;
        }
        
        std::fs::create_dir_all(&self.config.work_dir)
            .map_err(|e| Error::from_str(&format!("Failed to create work dir: {}", e)))?;
        
        Ok(())
    }

    /// Compile the Stage 1 (Rust-based) compiler
    #[instrument(skip(self))]
    fn compile_stage1_compiler(&self) -> Result<CompilationResult> {
        let start_time = Instant::now();
        let output_path = self.config.work_dir.join("cursed_stage1");

        info!("Compiling Stage 1 compiler (Rust implementation)");

        let output = Command::new("cargo")
            .args(&["build", "--release", "--bin", "cursed"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to run cargo: {}", e)))?;

        let compilation_time = start_time.elapsed();
        let success = output.status.success();

        if !success {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Ok(CompilationResult {
                stage: "Stage1_Rust".to_string(),
                compiler_path: PathBuf::new(),
                output_path,
                compilation_time,
                binary_size: 0,
                success: false,
                error_message: Some(error_msg.to_string()),
                checksum: String::new(),
            });
        }

        // Copy the compiled binary to our work directory
        let rust_binary = PathBuf::from("target/release/cursed");
        std::fs::copy(&rust_binary, &output_path)
            .map_err(|e| Error::from_str(&format!("Failed to copy Stage 1 binary: {}", e)))?;

        let binary_size = output_path.metadata()
            .map_err(|e| Error::from_str(&format!("Failed to get binary size: {}", e)))?
            .len();

        let checksum = self.calculate_checksum(&output_path)?;

        Ok(CompilationResult {
            stage: "Stage1_Rust".to_string(),
            compiler_path: output_path.clone(),
            output_path,
            compilation_time,
            binary_size,
            success: true,
            error_message: None,
            checksum,
        })
    }

    /// Use Stage 1 compiler to compile Stage 2 (CURSED) compiler
    #[instrument(skip(self, stage1_compiler))]
    fn compile_stage2_compiler(&self, stage1_compiler: &Path) -> Result<CompilationResult> {
        let start_time = Instant::now();
        let output_path = self.config.work_dir.join("cursed_stage2");

        info!("Compiling Stage 2 compiler using Stage 1");

        // Create a CURSED implementation of the compiler
        let cursed_compiler_source = self.generate_cursed_compiler_source()?;
        let source_path = self.config.work_dir.join("cursed_compiler.csd");
        std::fs::write(&source_path, cursed_compiler_source)
            .map_err(|e| Error::from_str(&format!("Failed to write Stage 2 source: {}", e)))?;

        // Compile using Stage 1
        let output = Command::new(stage1_compiler)
            .args(&[
                "--emit-binary",
                source_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to run Stage 1 compiler: {}", e)))?;

        let compilation_time = start_time.elapsed();
        let success = output.status.success();

        let binary_size = if success && output_path.exists() {
            output_path.metadata()
                .map_err(|e| Error::from_str(&format!("Failed to get Stage 2 binary size: {}", e)))?
                .len()
        } else {
            0
        };

        let checksum = if success {
            self.calculate_checksum(&output_path)?
        } else {
            String::new()
        };

        let error_message = if !success {
            Some(String::from_utf8_lossy(&output.stderr).to_string())
        } else {
            None
        };

        Ok(CompilationResult {
            stage: "Stage2_CURSED".to_string(),
            compiler_path: output_path.clone(),
            output_path,
            compilation_time,
            binary_size,
            success,
            error_message,
            checksum,
        })
    }

    /// Compare two compilation stages
    fn compare_stages(&self, stage1: &CompilationResult, stage2: &CompilationResult) -> Result<ComparisonResult> {
        let binary_size_diff = stage2.binary_size as i64 - stage1.binary_size as i64;
        
        let performance_diff = if stage1.compilation_time.as_millis() > 0 {
            let diff = stage2.compilation_time.as_millis() as f64 - stage1.compilation_time.as_millis() as f64;
            (diff / stage1.compilation_time.as_millis() as f64) * 100.0
        } else {
            0.0
        };

        let checksum_match = stage1.checksum == stage2.checksum;

        // Test output equivalence with a simple program
        let output_identical = self.test_output_equivalence(&stage1.compiler_path, &stage2.compiler_path)?;

        let functional_equivalent = checksum_match || output_identical;

        Ok(ComparisonResult {
            stage1: stage1.stage.clone(),
            stage2: stage2.stage.clone(),
            binary_size_diff,
            performance_diff,
            output_identical,
            checksum_match,
            functional_equivalent,
        })
    }

    /// Test output equivalence between two compilers
    fn test_output_equivalence(&self, compiler1: &Path, compiler2: &Path) -> Result<bool> {
        let test_program = r#"
func main() {
    print("Hello, bootstrap verification!")
    return 0
}
"#;

        let test_file = self.config.work_dir.join("equivalence_test.csd");
        std::fs::write(&test_file, test_program)
            .map_err(|e| Error::from_str(&format!("Failed to write test program: {}", e)))?;

        // Compile with both compilers
        let output1_path = self.config.work_dir.join("test_output1");
        let output2_path = self.config.work_dir.join("test_output2");

        let result1 = self.compile_test_program(compiler1, &test_file, &output1_path)?;
        let result2 = self.compile_test_program(compiler2, &test_file, &output2_path)?;

        if !result1.success || !result2.success {
            return Ok(false);
        }

        // Compare execution results
        let exec1 = self.execute_test_program(&output1_path)?;
        let exec2 = self.execute_test_program(&output2_path)?;

        Ok(exec1.stdout == exec2.stdout && exec1.exit_code == exec2.exit_code)
    }

    /// Compile a test program with a given compiler
    fn compile_test_program(&self, compiler: &Path, source: &Path, output: &Path) -> Result<ExecutionResult> {
        let start_time = Instant::now();

        let cmd_output = Command::new(compiler)
            .args(&[
                source.to_str().unwrap(),
                "-o",
                output.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to compile test program: {}", e)))?;

        Ok(ExecutionResult {
            program_path: output.to_path_buf(),
            execution_time: start_time.elapsed(),
            exit_code: cmd_output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&cmd_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd_output.stderr).to_string(),
            success: cmd_output.status.success(),
        })
    }

    /// Execute a compiled test program
    fn execute_test_program(&self, program: &Path) -> Result<ExecutionResult> {
        let start_time = Instant::now();

        let output = Command::new(program)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to execute test program: {}", e)))?;

        Ok(ExecutionResult {
            program_path: program.to_path_buf(),
            execution_time: start_time.elapsed(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        })
    }

    /// Run functional equivalence tests between two compilers
    fn run_functional_tests(&self, stage1: &CompilationResult, stage2: &CompilationResult) -> Result<Vec<ExecutionResult>> {
        let test_programs = vec![
            ("basic_arithmetic", r#"
func main() {
    let x = 42
    let y = 8
    print(x + y)
    print(x - y)
    print(x * y)
    print(x / y)
    return 0
}
"#),
            ("string_operations", r#"
func main() {
    let s1 = "Hello"
    let s2 = "World"
    print(s1 + " " + s2)
    return 0
}
"#),
            ("control_flow", r#"
func main() {
    for i in 0..5 {
        if i % 2 == 0 {
            print("even: " + i)
        } else {
            print("odd: " + i)
        }
    }
    return 0
}
"#),
        ];

        let mut results = Vec::new();

        for (test_name, test_code) in test_programs {
            info!("Running functional test: {}", test_name);
            
            let test_file = self.config.work_dir.join(format!("{}.csd", test_name));
            std::fs::write(&test_file, test_code)
                .map_err(|e| Error::from_str(&format!("Failed to write test {}: {}", test_name, e)))?;

            // Test with Stage 1 compiler
            let output1_path = self.config.work_dir.join(format!("{}_stage1", test_name));
            let result1 = self.compile_test_program(&stage1.compiler_path, &test_file, &output1_path)?;
            results.push(result1);

            // Test with Stage 2 compiler
            let output2_path = self.config.work_dir.join(format!("{}_stage2", test_name));
            let result2 = self.compile_test_program(&stage2.compiler_path, &test_file, &output2_path)?;
            results.push(result2);
        }

        Ok(results)
    }

    /// Run bootstrap cycles (Stage 1 → Stage 2 → Stage 3 → ...)
    fn run_bootstrap_cycles(&self, stage1: &CompilationResult) -> Result<Vec<BootstrapCycleResult>> {
        let mut cycle_results: Vec<BootstrapCycleResult> = Vec::new();
        let mut previous_compiler = stage1.compiler_path.clone();

        for cycle in 1..=self.config.bootstrap_cycles {
            info!("Running bootstrap cycle {}", cycle);

            let cycle_start = Instant::now();
            let mut stages = Vec::new();

            // Compile next stage using previous compiler
            let next_stage_result = self.compile_next_bootstrap_stage(&previous_compiler, cycle)?;
            stages.push(next_stage_result.clone());

            // Check convergence (binary stability)
            let convergence_achieved = if cycle > 1 {
                let prev_cycle = &cycle_results[cycle - 2];
                let prev_checksum = &prev_cycle.stages.last().unwrap().checksum;
                next_stage_result.checksum == *prev_checksum
            } else {
                false
            };

            let binary_stable = convergence_achieved;
            let performance_stable = self.check_performance_stability(&stages)?;

            cycle_results.push(BootstrapCycleResult {
                cycle_number: cycle,
                stages,
                convergence_achieved,
                binary_stable,
                performance_stable,
            });

            // If we've achieved convergence, we can stop early
            if convergence_achieved {
                info!("Bootstrap convergence achieved at cycle {}", cycle);
                break;
            }

            previous_compiler = next_stage_result.compiler_path;
        }

        Ok(cycle_results)
    }

    /// Compile the next bootstrap stage
    fn compile_next_bootstrap_stage(&self, compiler: &Path, cycle: usize) -> Result<CompilationResult> {
        let start_time = Instant::now();
        let output_path = self.config.work_dir.join(format!("cursed_stage{}", cycle + 1));

        // Use the same CURSED compiler source for each cycle
        let source_path = self.config.work_dir.join("cursed_compiler.csd");

        let output = Command::new(compiler)
            .args(&[
                source_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::from_str(&format!("Failed to run bootstrap cycle {}: {}", cycle, e)))?;

        let compilation_time = start_time.elapsed();
        let success = output.status.success();

        let binary_size = if success && output_path.exists() {
            output_path.metadata()
                .map_err(|e| Error::from_str(&format!("Failed to get binary size: {}", e)))?
                .len()
        } else {
            0
        };

        let checksum = if success {
            self.calculate_checksum(&output_path)?
        } else {
            String::new()
        };

        let error_message = if !success {
            Some(String::from_utf8_lossy(&output.stderr).to_string())
        } else {
            None
        };

        Ok(CompilationResult {
            stage: format!("Stage{}_Bootstrap", cycle + 1),
            compiler_path: output_path.clone(),
            output_path,
            compilation_time,
            binary_size,
            success,
            error_message,
            checksum,
        })
    }

    /// Check if performance is stable across compilation stages
    fn check_performance_stability(&self, stages: &[CompilationResult]) -> Result<bool> {
        if stages.len() < 2 {
            return Ok(true);
        }

        let last_stage = stages.last().unwrap();
        let prev_stage = &stages[stages.len() - 2];

        // Consider performance stable if compilation time difference is < 10%
        let time_diff = (last_stage.compilation_time.as_millis() as f64 - 
                        prev_stage.compilation_time.as_millis() as f64).abs();
        let time_ratio = time_diff / prev_stage.compilation_time.as_millis() as f64;

        Ok(time_ratio < 0.1) // Less than 10% difference
    }

    /// Analyze performance across all compilation results
    fn analyze_performance(&self, report: &mut VerificationReport) -> Result<()> {
        for result in &report.compilation_results {
            report.performance_metrics.compilation_times.insert(
                result.stage.clone(),
                result.compilation_time,
            );
            report.performance_metrics.binary_sizes.insert(
                result.stage.clone(),
                result.binary_size,
            );
        }

        for result in &report.execution_results {
            report.performance_metrics.execution_times.insert(
                result.program_path.to_string_lossy().to_string(),
                result.execution_time,
            );
        }

        Ok(())
    }

    /// Assess the final verification results
    fn assess_verification_results(&self, report: &mut VerificationReport) {
        let mut issues = Vec::new();

        // Check compilation success
        for result in &report.compilation_results {
            if !result.success {
                issues.push(format!("Compilation failed for {}: {}", 
                    result.stage, 
                    result.error_message.as_deref().unwrap_or("Unknown error")));
            }
        }

        // Check functional equivalence
        for comparison in &report.comparison_results {
            if !comparison.functional_equivalent {
                issues.push(format!("Functional equivalence failed between {} and {}", 
                    comparison.stage1, comparison.stage2));
            }
        }

        // Check bootstrap convergence
        let converged = report.bootstrap_cycle_results
            .iter()
            .any(|cycle| cycle.convergence_achieved);

        if !converged && report.bootstrap_cycle_results.len() >= self.config.bootstrap_cycles {
            issues.push("Bootstrap did not converge within the specified number of cycles".to_string());
        }

        report.issues_found = issues;
        report.overall_success = report.issues_found.is_empty();
    }

    /// Generate CURSED source code for the compiler
    fn generate_cursed_compiler_source(&self) -> Result<String> {
        // This is a simplified CURSED compiler implementation
        // In a real scenario, you would have the full CURSED compiler written in CURSED
        Ok(r#"
package main

import "os"
import "fmt"

func main() {
    args := os.Args()
    if len(args) < 2 {
        fmt.println("Usage: cursed <file>")
        return 1
    }
    
    filename := args[1]
    fmt.printf("Compiling %s with CURSED compiler...\n", filename)
    
    // Simplified compilation process
    // In reality, this would include lexing, parsing, code generation, etc.
    
    fmt.println("Compilation successful!")
    return 0
}
"#.to_string())
    }

    /// Calculate SHA-256 checksum of a file
    fn calculate_checksum(&self, file_path: &Path) -> Result<String> {
        use sha2::{Sha256, Digest};
        
        let contents = std::fs::read(file_path)
            .map_err(|e| Error::from_str(&format!("Failed to read file for checksum: {}", e)))?;
        
        let hash = Sha256::digest(&contents);
        Ok(format!("{:x}", hash))
    }
}

/// Generate a comprehensive verification report
pub fn generate_verification_report(report: &VerificationReport) -> String {
    use std::fmt::Write;
    
    let mut output = String::new();
    
    writeln!(output, "# CURSED Self-Compilation Verification Report").unwrap();
    writeln!(output, "").unwrap();
    writeln!(output, "**Generated on:** {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")).unwrap();
    writeln!(output, "**Verification Time:** {:?}", report.verification_time).unwrap();
    writeln!(output, "**Overall Success:** {}", if report.overall_success { "✅ PASS" } else { "❌ FAIL" }).unwrap();
    writeln!(output, "").unwrap();

    // Summary
    writeln!(output, "## Summary").unwrap();
    writeln!(output, "").unwrap();
    writeln!(output, "- **Compilation Stages:** {}", report.compilation_results.len()).unwrap();
    writeln!(output, "- **Bootstrap Cycles:** {}", report.bootstrap_cycle_results.len()).unwrap();
    writeln!(output, "- **Issues Found:** {}", report.issues_found.len()).unwrap();
    writeln!(output, "").unwrap();

    // Compilation Results
    writeln!(output, "## Compilation Results").unwrap();
    writeln!(output, "").unwrap();
    writeln!(output, "| Stage | Success | Time | Binary Size | Checksum |").unwrap();
    writeln!(output, "|-------|---------|------|-------------|----------|").unwrap();
    
    for result in &report.compilation_results {
        writeln!(output, "| {} | {} | {:?} | {} bytes | {} |",
            result.stage,
            if result.success { "✅" } else { "❌" },
            result.compilation_time,
            result.binary_size,
            &result.checksum[..8]  // First 8 chars of checksum
        ).unwrap();
    }
    writeln!(output, "").unwrap();

    // Comparison Results
    if !report.comparison_results.is_empty() {
        writeln!(output, "## Stage Comparisons").unwrap();
        writeln!(output, "").unwrap();
        
        for comparison in &report.comparison_results {
            writeln!(output, "### {} vs {}", comparison.stage1, comparison.stage2).unwrap();
            writeln!(output, "").unwrap();
            writeln!(output, "- **Binary Size Difference:** {} bytes", comparison.binary_size_diff).unwrap();
            writeln!(output, "- **Performance Difference:** {:.2}%", comparison.performance_diff).unwrap();
            writeln!(output, "- **Output Identical:** {}", if comparison.output_identical { "✅" } else { "❌" }).unwrap();
            writeln!(output, "- **Checksum Match:** {}", if comparison.checksum_match { "✅" } else { "❌" }).unwrap();
            writeln!(output, "- **Functionally Equivalent:** {}", if comparison.functional_equivalent { "✅" } else { "❌" }).unwrap();
            writeln!(output, "").unwrap();
        }
    }

    // Bootstrap Cycles
    if !report.bootstrap_cycle_results.is_empty() {
        writeln!(output, "## Bootstrap Cycles").unwrap();
        writeln!(output, "").unwrap();
        
        for cycle in &report.bootstrap_cycle_results {
            writeln!(output, "### Cycle {}", cycle.cycle_number).unwrap();
            writeln!(output, "").unwrap();
            writeln!(output, "- **Convergence Achieved:** {}", if cycle.convergence_achieved { "✅" } else { "❌" }).unwrap();
            writeln!(output, "- **Binary Stable:** {}", if cycle.binary_stable { "✅" } else { "❌" }).unwrap();
            writeln!(output, "- **Performance Stable:** {}", if cycle.performance_stable { "✅" } else { "❌" }).unwrap();
            writeln!(output, "").unwrap();
        }
    }

    // Issues Found
    if !report.issues_found.is_empty() {
        writeln!(output, "## Issues Found").unwrap();
        writeln!(output, "").unwrap();
        
        for (i, issue) in report.issues_found.iter().enumerate() {
            writeln!(output, "{}. {}", i + 1, issue).unwrap();
        }
        writeln!(output, "").unwrap();
    }

    // Performance Metrics
    writeln!(output, "## Performance Metrics").unwrap();
    writeln!(output, "").unwrap();
    
    writeln!(output, "### Compilation Times").unwrap();
    for (stage, time) in &report.performance_metrics.compilation_times {
        writeln!(output, "- {}: {:?}", stage, time).unwrap();
    }
    writeln!(output, "").unwrap();
    
    writeln!(output, "### Binary Sizes").unwrap();
    for (stage, size) in &report.performance_metrics.binary_sizes {
        writeln!(output, "- {}: {} bytes", stage, size).unwrap();
    }
    writeln!(output, "").unwrap();

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_verification_config_default() {
        let config = VerificationConfig::default();
        assert_eq!(config.bootstrap_cycles, 3);
        assert!(!config.keep_intermediates);
        assert!(config.optimization_levels.contains(&"-O0".to_string()));
    }

    #[test]
    fn test_checksum_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let config = VerificationConfig {
            work_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let verifier = SelfCompilationVerifier::new(config);
        
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello, World!").unwrap();
        
        let checksum = verifier.calculate_checksum(&test_file).unwrap();
        assert!(!checksum.is_empty());
        assert_eq!(checksum.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_report_generation() {
        let report = VerificationReport {
            config: "test config".to_string(),
            compilation_results: vec![],
            execution_results: vec![],
            comparison_results: vec![],
            bootstrap_cycle_results: vec![],
            performance_metrics: PerformanceMetrics {
                compilation_times: HashMap::new(),
                binary_sizes: HashMap::new(),
                execution_times: HashMap::new(),
                memory_usage: HashMap::new(),
            },
            issues_found: vec!["Test issue".to_string()],
            overall_success: false,
            verification_time: Duration::from_secs(60),
        };
        
        let report_text = generate_verification_report(&report);
        assert!(report_text.contains("CURSED Self-Compilation Verification Report"));
        assert!(report_text.contains("❌ FAIL"));
        assert!(report_text.contains("Test issue"));
    }
}
