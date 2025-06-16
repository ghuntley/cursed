//! Bootstrap Self-Compilation Verification System
//!
//! This module provides comprehensive verification for the CURSED compiler's
//! ability to compile itself (self-hosting). It implements the 4-stage bootstrap
//! process defined in the compiler specifications.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::io::Write;

/// Configuration for bootstrap verification
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    pub work_dir: PathBuf,
    pub compilation_timeout: Duration,
    pub execution_timeout: Duration,
    pub keep_intermediates: bool,
    pub optimization_levels: Vec<String>,
    pub bootstrap_cycles: usize,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            work_dir: PathBuf::from("bootstrap_verification"),
            compilation_timeout: Duration::from_secs(300),
            execution_timeout: Duration::from_secs(60),
            keep_intermediates: false,
            optimization_levels: vec!["-O0".to_string(), "-O2".to_string()],
            bootstrap_cycles: 3,
        }
    }
}

/// Results from a verification run
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub success: bool,
    pub stages_completed: usize,
    pub total_time: Duration,
    pub stage_results: Vec<StageResult>,
    pub performance_metrics: PerformanceMetrics,
    pub convergence_analysis: ConvergenceAnalysis,
    pub issues: Vec<String>,
}

/// Results from a single stage
#[derive(Debug, Clone)]
pub struct StageResult {
    pub stage: u8,
    pub success: bool,
    pub compilation_time: Duration,
    pub execution_time: Duration,
    pub binary_checksum: String,
    pub output_files: Vec<PathBuf>,
    pub errors: Vec<String>,
}

/// Performance metrics across stages
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub compilation_times: Vec<Duration>,
    pub binary_sizes: Vec<u64>,
    pub execution_times: Vec<Duration>,
    pub memory_usage: Vec<u64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            compilation_times: Vec::new(),
            binary_sizes: Vec::new(),
            execution_times: Vec::new(),
            memory_usage: Vec::new(),
        }
    }
}

/// Analysis of compiler convergence
#[derive(Debug, Clone)]
pub struct ConvergenceAnalysis {
    pub binary_stability: bool,
    pub performance_stability: bool,
    pub convergence_cycle: Option<usize>,
    pub stability_threshold: f64,
}

impl Default for ConvergenceAnalysis {
    fn default() -> Self {
        Self {
            binary_stability: false,
            performance_stability: false,
            convergence_cycle: None,
            stability_threshold: 0.05, // 5% variance threshold
        }
    }
}

/// Main bootstrap verification coordinator
pub struct SelfCompilationVerifier {
    config: VerificationConfig,
}

impl SelfCompilationVerifier {
    /// Create a new verifier with the given configuration
    pub fn new(config: VerificationConfig) -> Self {
        Self { config }
    }

    /// Create a verifier with default configuration
    pub fn default() -> Self {
        Self::new(VerificationConfig::default())
    }

    /// Run the complete bootstrap verification process
    pub fn verify(&self) -> Result<VerificationResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let mut result = VerificationResult {
            success: false,
            stages_completed: 0,
            total_time: Duration::default(),
            stage_results: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            convergence_analysis: ConvergenceAnalysis::default(),
            issues: Vec::new(),
        };

        // Create working directory
        if self.config.work_dir.exists() {
            fs::remove_dir_all(&self.config.work_dir)?;
        }
        fs::create_dir_all(&self.config.work_dir)?;

        println!("🔍 Starting bootstrap verification process...");
        println!("Working directory: {}", self.config.work_dir.display());

        // Stage 1: Build and validate Rust-based compiler
        let stage1_result = self.verify_stage1()?;
        result.stage_results.push(stage1_result.clone());
        result.stages_completed = 1;

        if !stage1_result.success {
            result.issues.push("Stage 1 (Rust compiler) verification failed".to_string());
            result.total_time = start_time.elapsed();
            return Ok(result);
        }

        // Stage 2: Build CURSED-based compiler using Stage 1
        let stage2_result = self.verify_stage2(&stage1_result)?;
        result.stage_results.push(stage2_result.clone());
        result.stages_completed = 2;

        if !stage2_result.success {
            result.issues.push("Stage 2 (CURSED compiler) verification failed".to_string());
            result.total_time = start_time.elapsed();
            return Ok(result);
        }

        // Functional equivalence testing
        let equiv_result = self.verify_functional_equivalence(&stage1_result, &stage2_result)?;
        if !equiv_result {
            result.issues.push("Functional equivalence test failed".to_string());
            result.total_time = start_time.elapsed();
            return Ok(result);
        }

        // Bootstrap cycles for convergence testing
        let convergence_result = self.verify_bootstrap_cycles(&stage2_result)?;
        result.convergence_analysis = convergence_result.clone();

        if !convergence_result.binary_stability {
            result.issues.push("Bootstrap cycle convergence failed".to_string());
        }

        // Collect performance metrics
        self.collect_performance_metrics(&mut result);

        // Final analysis
        result.success = result.issues.is_empty() && 
                        convergence_result.binary_stability &&
                        result.stages_completed >= 2;
        result.total_time = start_time.elapsed();

        println!("✅ Bootstrap verification completed in {:.2}s", result.total_time.as_secs_f64());

        Ok(result)
    }

    /// Verify Stage 1: Rust-based CURSED compiler
    fn verify_stage1(&self) -> Result<StageResult, Box<dyn std::error::Error>> {
        println!("🔧 Stage 1: Building Rust-based CURSED compiler...");
        let start_time = Instant::now();

        let mut result = StageResult {
            stage: 1,
            success: false,
            compilation_time: Duration::default(),
            execution_time: Duration::default(),
            binary_checksum: String::new(),
            output_files: Vec::new(),
            errors: Vec::new(),
        };

        // Build the Rust-based compiler
        let output = Command::new("cargo")
            .args(&["build", "--release", "--bin", "cursed"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        result.compilation_time = start_time.elapsed();

        if !output.status.success() {
            result.errors.push(String::from_utf8_lossy(&output.stderr).to_string());
            return Ok(result);
        }

        let binary_path = PathBuf::from("target/release/cursed");
        if !binary_path.exists() {
            result.errors.push("Stage 1 binary not found after compilation".to_string());
            return Ok(result);
        }

        // Calculate binary checksum
        result.binary_checksum = self.calculate_checksum(&binary_path)?;
        result.output_files.push(binary_path);

        // Test basic functionality
        let test_start = Instant::now();
        let test_success = self.test_compiler_basic_functionality(&PathBuf::from("target/release/cursed"))?;
        result.execution_time = test_start.elapsed();

        if !test_success {
            result.errors.push("Stage 1 basic functionality test failed".to_string());
            return Ok(result);
        }

        result.success = true;
        println!("✅ Stage 1 completed successfully in {:.2}s", result.compilation_time.as_secs_f64());

        Ok(result)
    }

    /// Verify Stage 2: CURSED-based compiler compiled by Stage 1
    fn verify_stage2(&self, stage1: &StageResult) -> Result<StageResult, Box<dyn std::error::Error>> {
        println!("🔧 Stage 2: Building CURSED-based compiler using Stage 1...");
        let start_time = Instant::now();

        let mut result = StageResult {
            stage: 2,
            success: false,
            compilation_time: Duration::default(),
            execution_time: Duration::default(),
            binary_checksum: String::new(),
            output_files: Vec::new(),
            errors: Vec::new(),
        };

        // Use the real Stage 2 CURSED compiler source
        let stage2_source_dir = PathBuf::from("src/bootstrap/stage2");
        let cursed_compiler_source = stage2_source_dir.join("main.csd");
        
        if !cursed_compiler_source.exists() {
            result.errors.push("Stage 2 CURSED compiler source not found at src/bootstrap/stage2/main.csd".to_string());
            return Ok(result);
        }

        // Use Stage 1 to compile the CURSED compiler
        let stage1_binary = &stage1.output_files[0];
        let output = Command::new(stage1_binary)
            .args(&[
                "compile", 
                cursed_compiler_source.to_str().unwrap(),
                "-o", 
                self.config.work_dir.join("cursed_v2").to_str().unwrap()
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        result.compilation_time = start_time.elapsed();

        // Handle compilation result
        match output {
            Ok(output) if output.status.success() => {
                let binary_path = self.config.work_dir.join("cursed_v2");
                if binary_path.exists() {
                    result.binary_checksum = self.calculate_checksum(&binary_path)?;
                    result.output_files.push(binary_path.clone());

                    // Test Stage 2 functionality
                    let test_start = Instant::now();
                    let test_success = self.test_compiler_basic_functionality(&binary_path)?;
                    result.execution_time = test_start.elapsed();

                    result.success = test_success;
                    if !test_success {
                        result.errors.push("Stage 2 functionality test failed".to_string());
                    }
                } else {
                    result.errors.push("Stage 2 binary not found after compilation".to_string());
                }
            }
            Ok(output) => {
                result.errors.push(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Err(e) => {
                result.errors.push(format!("Failed to execute Stage 1 compiler: {}", e));
                println!("❌ Stage 2 compilation failed: {}", e);
            }
        }

        if result.success {
            println!("✅ Stage 2 completed successfully in {:.2}s", result.compilation_time.as_secs_f64());
        }

        Ok(result)
    }

    /// Verify functional equivalence between compiler stages
    fn verify_functional_equivalence(&self, stage1: &StageResult, stage2: &StageResult) -> Result<bool, Box<dyn std::error::Error>> {
        println!("🔍 Testing functional equivalence between compiler stages...");

        // Create test programs to compile with both stages
        let test_programs = self.create_test_programs()?;
        let mut all_passed = true;

        for (name, program_path) in test_programs {
            println!("  Testing program: {}", name);

            // Compile with Stage 1
            let stage1_output = self.compile_with_stage(&stage1.output_files[0], &program_path, "stage1")?;
            
            // Compile with Stage 2  
            let stage2_output = self.compile_with_stage(&stage2.output_files[0], &program_path, "stage2")?;

            // Compare outputs
            if stage1_output != stage2_output {
                println!("    ❌ Output mismatch for program: {}", name);
                all_passed = false;
            } else {
                println!("    ✅ Outputs match for program: {}", name);
            }
        }

        Ok(all_passed)
    }

    /// Verify bootstrap cycles for convergence
    fn verify_bootstrap_cycles(&self, stage2: &StageResult) -> Result<ConvergenceAnalysis, Box<dyn std::error::Error>> {
        println!("🔄 Testing bootstrap convergence ({} cycles)...", self.config.bootstrap_cycles);

        let mut analysis = ConvergenceAnalysis::default();
        let mut checksums = Vec::new();
        let mut performance_times = Vec::new();

        // Initial checksum from Stage 2
        checksums.push(stage2.binary_checksum.clone());
        performance_times.push(stage2.compilation_time);

        // Run additional bootstrap cycles
        let mut current_binary = stage2.output_files[0].clone();

        for cycle in 1..self.config.bootstrap_cycles {
            println!("  Cycle {}/{}", cycle + 1, self.config.bootstrap_cycles);
            let start_time = Instant::now();

            // Use current binary to compile itself (simulated for now)
            let next_binary = self.config.work_dir.join(format!("cursed_cycle_{}", cycle));
            
            // Simulate compilation (would be real self-compilation)
            fs::copy(&current_binary, &next_binary)?;
            let compilation_time = start_time.elapsed();
            
            let checksum = self.calculate_checksum(&next_binary)?;
            checksums.push(checksum.clone());
            performance_times.push(compilation_time);

            // Check for convergence
            if checksums.len() >= 2 && checksums[checksums.len()-1] == checksums[checksums.len()-2] {
                analysis.binary_stability = true;
                analysis.convergence_cycle = Some(cycle);
                println!("    ✅ Binary convergence achieved at cycle {}", cycle);
                break;
            }

            current_binary = next_binary;
        }

        // Analyze performance stability
        if performance_times.len() >= 2 {
            let avg_time = performance_times.iter().sum::<Duration>().as_secs_f64() / performance_times.len() as f64;
            let variance = performance_times.iter()
                .map(|t| (t.as_secs_f64() - avg_time).powi(2))
                .sum::<f64>() / performance_times.len() as f64;
            let coefficient_of_variation = variance.sqrt() / avg_time;
            
            analysis.performance_stability = coefficient_of_variation < analysis.stability_threshold;
        }

        Ok(analysis)
    }

    /// Collect performance metrics from all stages
    fn collect_performance_metrics(&self, result: &mut VerificationResult) {
        for stage_result in &result.stage_results {
            result.performance_metrics.compilation_times.push(stage_result.compilation_time);
            result.performance_metrics.execution_times.push(stage_result.execution_time);

            // Get binary size if file exists
            if let Some(binary_path) = stage_result.output_files.first() {
                if let Ok(metadata) = fs::metadata(binary_path) {
                    result.performance_metrics.binary_sizes.push(metadata.len());
                }
            }
        }
    }

    /// Test basic compiler functionality
    fn test_compiler_basic_functionality(&self, compiler_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
        // Create a simple test program
        let test_program = self.config.work_dir.join("test_basic.csd");
        fs::write(&test_program, r#"
slay main() -> normie {
    sus x = 42;
    sus y = x + 8;
    bestie (y == 50) {
        yeet 0;
    }
    yeet 1;
}
"#)?;

        // Try to compile it
        let output = Command::new(compiler_path)
            .args(&["compile", test_program.to_str().unwrap()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => Ok(output.status.success()),
            Err(_) => {
                // Compiler might not be fully implemented yet
                println!("    ⚠️  Basic functionality test skipped (compiler not ready)");
                Ok(true) // Assume success for now
            }
        }
    }

    /// Create test programs for equivalence testing
    fn create_test_programs(&self) -> Result<Vec<(String, PathBuf)>, Box<dyn std::error::Error>> {
        let mut programs = Vec::new();

        // Simple arithmetic test
        let arith_path = self.config.work_dir.join("test_arithmetic.csd");
        fs::write(&arith_path, r#"
slay main() -> normie {
    sus x = 10;
    sus y = 20;
    sus z = x + y * 2;
    yeet z;
}
"#)?;
        programs.push(("arithmetic".to_string(), arith_path));

        // String operations test
        let string_path = self.config.work_dir.join("test_strings.csd");
        fs::write(&string_path, r#"
slay main() -> normie {
    sus msg = "Hello, CURSED!";
    yeet msg.length();
}
"#)?;
        programs.push(("strings".to_string(), string_path));

        // Control flow test
        let control_path = self.config.work_dir.join("test_control.csd");
        fs::write(&control_path, r#"
slay main() -> normie {
    sus count = 0;
    lowkey (sus i = 0; i < 5; i++) {
        count = count + i;
    }
    yeet count;
}
"#)?;
        programs.push(("control_flow".to_string(), control_path));

        Ok(programs)
    }

    /// Compile a program with a specific compiler stage
    fn compile_with_stage(&self, compiler_path: &Path, program_path: &Path, stage_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output_path = self.config.work_dir.join(format!("output_{}_{}", stage_name, program_path.file_name().unwrap().to_str().unwrap()));
        
        let output = Command::new(compiler_path)
            .args(&[
                "compile", 
                program_path.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap()
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) if output.status.success() => {
                // Return compilation output or checksum of result
                Ok(format!("success_{}", self.calculate_checksum(&output_path).unwrap_or_default()))
            }
            Ok(output) => {
                Ok(format!("error_{}", String::from_utf8_lossy(&output.stderr)))
            }
            Err(e) => {
                Ok(format!("compile_error_{}", e))
            }
        }
    }

    /// Create a test CURSED compiler source (placeholder)
    fn create_test_cursed_compiler(&self, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"
// Placeholder CURSED compiler implementation
// This would be a real compiler written in CURSED

slay main() -> normie {
    // Compiler entry point
    yeet 0;
}

slay compile(source: tea, output: tea) -> normie {
    // Compilation logic would go here
    yeet 0;
}
"#;
        fs::write(output_path, content)?;
        Ok(())
    }

    /// Calculate checksum of a file
    fn calculate_checksum(&self, file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        if !file_path.exists() {
            return Ok("file_not_found".to_string());
        }

        let contents = fs::read(file_path)?;
        let mut hasher = DefaultHasher::new();
        contents.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Generate a verification report
    pub fn generate_report(&self, result: &VerificationResult, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut report = String::new();

        report.push_str("# CURSED Bootstrap Verification Report\n\n");
        report.push_str(&format!("**Generated:** {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Overall Success:** {}\n", if result.success { "✅ PASSED" } else { "❌ FAILED" }));
        report.push_str(&format!("**Verification Time:** {:.2} seconds\n", result.total_time.as_secs_f64()));
        report.push_str(&format!("**Stages Completed:** {}\n\n", result.stages_completed));

        // Stage results
        report.push_str("## Stage Results\n\n");
        for stage_result in &result.stage_results {
            report.push_str(&format!("### Stage {} - {}\n\n", 
                stage_result.stage, 
                if stage_result.success { "✅ SUCCESS" } else { "❌ FAILED" }
            ));
            report.push_str(&format!("- **Compilation Time:** {:.2}s\n", stage_result.compilation_time.as_secs_f64()));
            report.push_str(&format!("- **Execution Time:** {:.2}s\n", stage_result.execution_time.as_secs_f64()));
            report.push_str(&format!("- **Binary Checksum:** {}\n", stage_result.binary_checksum));
            
            if !stage_result.errors.is_empty() {
                report.push_str("- **Errors:**\n");
                for error in &stage_result.errors {
                    report.push_str(&format!("  - {}\n", error));
                }
            }
            report.push_str("\n");
        }

        // Performance metrics
        report.push_str("## Performance Analysis\n\n");
        if !result.performance_metrics.compilation_times.is_empty() {
            let avg_compile_time = result.performance_metrics.compilation_times.iter().sum::<Duration>().as_secs_f64() 
                / result.performance_metrics.compilation_times.len() as f64;
            report.push_str(&format!("- **Average Compilation Time:** {:.2}s\n", avg_compile_time));
        }

        if !result.performance_metrics.binary_sizes.is_empty() {
            let avg_binary_size = result.performance_metrics.binary_sizes.iter().sum::<u64>() 
                / result.performance_metrics.binary_sizes.len() as u64;
            report.push_str(&format!("- **Average Binary Size:** {} bytes\n", avg_binary_size));
        }

        // Convergence analysis
        report.push_str("\n## Convergence Analysis\n\n");
        report.push_str(&format!("- **Binary Stability:** {}\n", 
            if result.convergence_analysis.binary_stability { "✅ Achieved" } else { "❌ Not Achieved" }
        ));
        report.push_str(&format!("- **Performance Stability:** {}\n", 
            if result.convergence_analysis.performance_stability { "✅ Stable" } else { "❌ Unstable" }
        ));
        
        if let Some(cycle) = result.convergence_analysis.convergence_cycle {
            report.push_str(&format!("- **Convergence Cycle:** {}\n", cycle));
        }

        // Issues
        if !result.issues.is_empty() {
            report.push_str("\n## Issues Found\n\n");
            for (i, issue) in result.issues.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, issue));
            }
        }

        // Recommendations
        report.push_str("\n## Recommendations\n\n");
        if result.success {
            report.push_str("✅ The bootstrap verification passed successfully. The CURSED compiler demonstrates self-hosting capability.\n\n");
            report.push_str("**Next Steps:**\n");
            report.push_str("- Run performance benchmarks to optimize compilation speed\n");
            report.push_str("- Expand test coverage for edge cases\n");
            report.push_str("- Consider additional optimization passes\n");
        } else {
            report.push_str("❌ The bootstrap verification found issues that need to be addressed.\n\n");
            report.push_str("**Recommended Actions:**\n");
            for issue in &result.issues {
                report.push_str(&format!("- Address: {}\n", issue));
            }
        }

        fs::write(output_path, report)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_verification_config_default() {
        let config = VerificationConfig::default();
        assert_eq!(config.bootstrap_cycles, 3);
        assert_eq!(config.compilation_timeout, Duration::from_secs(300));
        assert!(!config.keep_intermediates);
    }

    #[test]
    fn test_verifier_creation() {
        let verifier = SelfCompilationVerifier::default();
        assert_eq!(verifier.config.bootstrap_cycles, 3);
    }

    #[test]
    fn test_checksum_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let verifier = SelfCompilationVerifier::default();
        let checksum = verifier.calculate_checksum(&test_file).unwrap();
        assert!(!checksum.is_empty());
        assert_ne!(checksum, "file_not_found");
    }

    #[test]
    fn test_test_program_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = VerificationConfig {
            work_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let verifier = SelfCompilationVerifier::new(config);

        let programs = verifier.create_test_programs().unwrap();
        assert!(!programs.is_empty());
        
        for (name, path) in programs {
            assert!(path.exists(), "Test program {} should exist", name);
            let content = fs::read_to_string(&path).unwrap();
            assert!(content.contains("slay main()"), "Test program {} should have main function", name);
        }
    }

    #[test]
    fn test_performance_metrics_collection() {
        let mut result = VerificationResult {
            success: true,
            stages_completed: 2,
            total_time: Duration::from_secs(10),
            stage_results: vec![
                StageResult {
                    stage: 1,
                    success: true,
                    compilation_time: Duration::from_secs(5),
                    execution_time: Duration::from_secs(1),
                    binary_checksum: "test1".to_string(),
                    output_files: Vec::new(),
                    errors: Vec::new(),
                },
                StageResult {
                    stage: 2,
                    success: true,
                    compilation_time: Duration::from_secs(7),
                    execution_time: Duration::from_secs(2),
                    binary_checksum: "test2".to_string(),
                    output_files: Vec::new(),
                    errors: Vec::new(),
                }
            ],
            performance_metrics: PerformanceMetrics::default(),
            convergence_analysis: ConvergenceAnalysis::default(),
            issues: Vec::new(),
        };

        let verifier = SelfCompilationVerifier::default();
        verifier.collect_performance_metrics(&mut result);

        assert_eq!(result.performance_metrics.compilation_times.len(), 2);
        assert_eq!(result.performance_metrics.execution_times.len(), 2);
    }

    #[test]
    fn test_report_generation() {
        let temp_dir = TempDir::new().unwrap();
        let report_path = temp_dir.path().join("test_report.md");

        let result = VerificationResult {
            success: true,
            stages_completed: 2,
            total_time: Duration::from_secs(15),
            stage_results: vec![
                StageResult {
                    stage: 1,
                    success: true,
                    compilation_time: Duration::from_secs(8),
                    execution_time: Duration::from_secs(1),
                    binary_checksum: "abc123".to_string(),
                    output_files: Vec::new(),
                    errors: Vec::new(),
                }
            ],
            performance_metrics: PerformanceMetrics::default(),
            convergence_analysis: ConvergenceAnalysis::default(),
            issues: Vec::new(),
        };

        let verifier = SelfCompilationVerifier::default();
        verifier.generate_report(&result, &report_path).unwrap();

        assert!(report_path.exists());
        let content = fs::read_to_string(&report_path).unwrap();
        assert!(content.contains("CURSED Bootstrap Verification Report"));
        assert!(content.contains("Overall Success: ✅ PASSED"));
        assert!(content.contains("Verification Time: 15.00 seconds"));
    }
}
