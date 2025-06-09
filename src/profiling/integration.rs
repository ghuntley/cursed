// Build system integration and CI/CD support for profiling

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

use crate::profiling::core::{ProfilerConfig, CursedProfiler, ProfilerError};
use crate::profiling::benchmarking::{BenchmarkSuite, BenchmarkConfig, BenchmarkResults};
use crate::profiling::reporting::{ReportGenerator, ReportConfig};

/// Build system integration for automated profiling
#[derive(Debug)]
pub struct BuildIntegration {
    config: BuildConfig,
    profiler: Option<CursedProfiler>,
}

impl BuildIntegration {
    pub fn new(config: BuildConfig) -> Self {
        Self {
            config,
            profiler: None,
        }
    }
    
    #[instrument(skip(self))]
    pub fn setup_profiling_build(&mut self) -> Result<(), ProfilerError> {
        info!("Setting up profiling build integration");
        
        let profiler_config = ProfilerConfig {
            modes: self.config.default_profiling_modes.clone(),
            cpu_sampling_frequency: self.config.cpu_sampling_frequency,
            memory_tracking_threshold: self.config.memory_threshold,
            max_stack_depth: self.config.max_stack_depth,
            track_goroutines: self.config.enable_concurrency_profiling,
            track_io_operations: self.config.enable_io_profiling,
            output_directory: self.config.output_directory.to_string_lossy().to_string(),
            max_session_duration: self.config.max_session_duration,
            output_format: self.config.output_format.clone(),
            regression_threshold: self.config.regression_threshold,
        };
        
        self.profiler = Some(CursedProfiler::new(profiler_config));
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn profile_build(&mut self, target: &str) -> Result<BuildProfileResult, ProfilerError> {
        info!("Starting profiled build for target: {}", target);
        
        let session_name = format!("build_{}_{}", target, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        
        if let Some(profiler) = &mut self.profiler {
            profiler.start_session(session_name.clone())?;
        }
        
        let build_start = std::time::Instant::now();
        
        // Execute build command
        let build_result = self.execute_build_command(target)?;
        
        let build_duration = build_start.elapsed();
        
        // Stop profiling
        let profile_data = if let Some(profiler) = &mut self.profiler {
            Some(profiler.stop_session()?)
        } else {
            None
        };
        
        let result = BuildProfileResult {
            target: target.to_string(),
            session_name,
            build_duration,
            build_success: build_result.success,
            build_output: build_result.output,
            profile_data,
            artifacts: self.collect_build_artifacts(target)?,
        };
        
        info!("Build profiling completed for target: {} in {:?}", target, build_duration);
        Ok(result)
    }
    
    #[instrument(skip(self))]
    pub fn run_performance_tests(&self) -> Result<PerformanceTestResults, ProfilerError> {
        info!("Running performance tests");
        
        let mut test_results = HashMap::new();
        
        for test_config in &self.config.performance_tests {
            info!("Running performance test: {}", test_config.name);
            
            let mut suite = BenchmarkSuite::new(
                test_config.name.clone(),
                test_config.benchmark_config.clone(),
            );
            
            // Load baseline if available
            if let Some(baseline_path) = &test_config.baseline_path {
                if baseline_path.exists() {
                    suite.load_baseline(&baseline_path.to_string_lossy())?;
                }
            }
            
            // Add benchmarks from test configuration
            for benchmark_path in &test_config.benchmark_files {
                self.load_benchmarks_from_file(&mut suite, benchmark_path)?;
            }
            
            let results = suite.run_all()?;
            test_results.insert(test_config.name.clone(), results);
        }
        
        let overall_pass = self.evaluate_overall_performance(&test_results);
        
        Ok(PerformanceTestResults {
            timestamp: std::time::SystemTime::now(),
            test_results,
            overall_pass,
        })
    }
    
    #[instrument(skip(self))]
    pub fn detect_performance_regressions(&self, current: &PerformanceTestResults) -> Result<RegressionReport, ProfilerError> {
        info!("Detecting performance regressions");
        
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (test_name, results) in &current.test_results {
            if let Some(analysis) = &results.regression_analysis {
                for regression in &analysis.regressions {
                    regressions.push(DetectedRegression {
                        test_name: test_name.clone(),
                        benchmark_name: regression.benchmark_name.clone(),
                        change: regression.change_type.clone(),
                        severity: regression.severity.clone(),
                    });
                }
                
                for improvement in &analysis.improvements {
                    improvements.push(DetectedImprovement {
                        test_name: test_name.clone(),
                        benchmark_name: improvement.benchmark_name.clone(),
                        change: improvement.change_type.clone(),
                    });
                }
            }
        }
        
        let critical_regressions = regressions.iter()
            .filter(|r| r.severity == crate::profiling::benchmarking::RegressionSeverity::Critical)
            .count();
        
        Ok(RegressionReport {
            timestamp: std::time::SystemTime::now(),
            regressions,
            improvements,
            critical_count: critical_regressions,
            should_fail_build: critical_regressions > 0 && self.config.fail_on_regression,
        })
    }
    
    #[instrument(skip(self))]
    pub fn generate_ci_report(&self, results: &PerformanceTestResults) -> Result<CiReport, ProfilerError> {
        info!("Generating CI/CD report");
        
        let regression_report = self.detect_performance_regressions(results)?;
        
        let mut status = CiStatus::Success;
        let mut messages = Vec::new();
        
        if regression_report.should_fail_build {
            status = CiStatus::Failure;
            messages.push(format!(
                "Build failed due to {} critical performance regressions",
                regression_report.critical_count
            ));
        } else if !regression_report.regressions.is_empty() {
            status = CiStatus::Warning;
            messages.push(format!(
                "Performance regressions detected: {}",
                regression_report.regressions.len()
            ));
        }
        
        if !regression_report.improvements.is_empty() {
            messages.push(format!(
                "Performance improvements detected: {}",
                regression_report.improvements.len()
            ));
        }
        
        let report = CiReport {
            status,
            messages,
            regression_report,
            performance_summary: self.generate_performance_summary(results),
            artifacts: self.collect_ci_artifacts(results)?,
        };
        
        Ok(report)
    }
    
    fn execute_build_command(&self, target: &str) -> Result<BuildCommandResult, ProfilerError> {
        let mut command = Command::new(&self.config.build_command);
        command.args(&self.config.build_args);
        command.arg(target);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        let output = command.output().map_err(ProfilerError::IoError)?;
        
        Ok(BuildCommandResult {
            success: output.status.success(),
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            error: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
    
    fn collect_build_artifacts(&self, target: &str) -> Result<Vec<BuildArtifact>, ProfilerError> {
        let mut artifacts = Vec::new();
        
        // Collect binary artifacts
        let target_path = self.config.output_directory.join(target);
        if target_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&target_path) {
                artifacts.push(BuildArtifact {
                    name: target.to_string(),
                    path: target_path,
                    size: metadata.len(),
                    artifact_type: ArtifactType::Binary,
                });
            }
        }
        
        // Collect profiling data artifacts
        let profiling_dir = self.config.output_directory.join("profiling");
        if profiling_dir.exists() {
            for entry in std::fs::read_dir(profiling_dir).map_err(ProfilerError::IoError)? {
                let entry = entry.map_err(ProfilerError::IoError)?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        artifacts.push(BuildArtifact {
                            name: path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            path,
                            size: metadata.len(),
                            artifact_type: ArtifactType::ProfilingData,
                        });
                    }
                }
            }
        }
        
        Ok(artifacts)
    }
    
    fn load_benchmarks_from_file(&self, suite: &mut BenchmarkSuite, _path: &Path) -> Result<(), ProfilerError> {
        // In a real implementation, this would parse benchmark files
        // and add them to the suite
        warn!("Benchmark file loading not yet implemented");
        Ok(())
    }
    
    fn evaluate_overall_performance(&self, test_results: &HashMap<String, BenchmarkResults>) -> bool {
        for results in test_results.values() {
            if let Some(analysis) = &results.regression_analysis {
                if analysis.has_critical_regressions() {
                    return false;
                }
            }
        }
        true
    }
    
    fn generate_performance_summary(&self, results: &PerformanceTestResults) -> PerformanceSummary {
        let total_tests = results.test_results.len();
        let total_benchmarks: usize = results.test_results.values()
            .map(|r| r.results.len())
            .sum();
        
        let total_regressions: usize = results.test_results.values()
            .filter_map(|r| r.regression_analysis.as_ref())
            .map(|a| a.regressions.len())
            .sum();
        
        let total_improvements: usize = results.test_results.values()
            .filter_map(|r| r.regression_analysis.as_ref())
            .map(|a| a.improvements.len())
            .sum();
        
        PerformanceSummary {
            total_tests,
            total_benchmarks,
            total_regressions,
            total_improvements,
            overall_pass: results.overall_pass,
        }
    }
    
    fn collect_ci_artifacts(&self, _results: &PerformanceTestResults) -> Result<Vec<CiArtifact>, ProfilerError> {
        let mut artifacts = Vec::new();
        
        // Collect HTML reports
        let reports_dir = self.config.output_directory.join("reports");
        if reports_dir.exists() {
            for entry in std::fs::read_dir(reports_dir).map_err(ProfilerError::IoError)? {
                let entry = entry.map_err(ProfilerError::IoError)?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("html") {
                    artifacts.push(CiArtifact {
                        name: path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        path,
                        artifact_type: CiArtifactType::Report,
                        public: true,
                    });
                }
            }
        }
        
        Ok(artifacts)
    }
}

/// Build integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub build_command: String,
    pub build_args: Vec<String>,
    pub output_directory: PathBuf,
    pub enable_profiling: bool,
    pub default_profiling_modes: Vec<crate::profiling::core::ProfilerMode>,
    pub cpu_sampling_frequency: u64,
    pub memory_threshold: usize,
    pub max_stack_depth: usize,
    pub enable_concurrency_profiling: bool,
    pub enable_io_profiling: bool,
    pub max_session_duration: std::time::Duration,
    pub output_format: crate::profiling::core::OutputFormat,
    pub regression_threshold: f64,
    pub fail_on_regression: bool,
    pub performance_tests: Vec<PerformanceTestConfig>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            build_command: "cargo".to_string(),
            build_args: vec!["build".to_string(), "--release".to_string()],
            output_directory: PathBuf::from("target/cursed"),
            enable_profiling: true,
            default_profiling_modes: vec![
                crate::profiling::core::ProfilerMode::Cpu,
                crate::profiling::core::ProfilerMode::Memory,
            ],
            cpu_sampling_frequency: 100,
            memory_threshold: 1024,
            max_stack_depth: 64,
            enable_concurrency_profiling: true,
            enable_io_profiling: true,
            max_session_duration: std::time::Duration::from_secs(600),
            output_format: crate::profiling::core::OutputFormat::Json,
            regression_threshold: 10.0,
            fail_on_regression: true,
            performance_tests: Vec::new(),
        }
    }
}

/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestConfig {
    pub name: String,
    pub benchmark_files: Vec<PathBuf>,
    pub baseline_path: Option<PathBuf>,
    pub benchmark_config: BenchmarkConfig,
}

/// Build profiling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfileResult {
    pub target: String,
    pub session_name: String,
    pub build_duration: std::time::Duration,
    pub build_success: bool,
    pub build_output: String,
    pub profile_data: Option<crate::profiling::core::ProfileData>,
    pub artifacts: Vec<BuildArtifact>,
}

/// Build command execution result
#[derive(Debug, Clone)]
struct BuildCommandResult {
    success: bool,
    output: String,
    error: String,
}

/// Build artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub artifact_type: ArtifactType,
}

/// Types of build artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Binary,
    Library,
    ProfilingData,
    Report,
}

/// Performance test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestResults {
    pub timestamp: std::time::SystemTime,
    pub test_results: HashMap<String, BenchmarkResults>,
    pub overall_pass: bool,
}

/// Regression detection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
    pub timestamp: std::time::SystemTime,
    pub regressions: Vec<DetectedRegression>,
    pub improvements: Vec<DetectedImprovement>,
    pub critical_count: usize,
    pub should_fail_build: bool,
}

/// Detected performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedRegression {
    pub test_name: String,
    pub benchmark_name: String,
    pub change: crate::profiling::benchmarking::PerformanceChange,
    pub severity: crate::profiling::benchmarking::RegressionSeverity,
}

/// Detected performance improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedImprovement {
    pub test_name: String,
    pub benchmark_name: String,
    pub change: crate::profiling::benchmarking::PerformanceChange,
}

/// CI/CD integration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiReport {
    pub status: CiStatus,
    pub messages: Vec<String>,
    pub regression_report: RegressionReport,
    pub performance_summary: PerformanceSummary,
    pub artifacts: Vec<CiArtifact>,
}

/// CI build status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
    Success,
    Warning,
    Failure,
}

/// Performance summary for CI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_tests: usize,
    pub total_benchmarks: usize,
    pub total_regressions: usize,
    pub total_improvements: usize,
    pub overall_pass: bool,
}

/// CI artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiArtifact {
    pub name: String,
    pub path: PathBuf,
    pub artifact_type: CiArtifactType,
    pub public: bool,
}

/// Types of CI artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CiArtifactType {
    Report,
    ProfilingData,
    Logs,
}

/// GitHub Actions integration
#[derive(Debug)]
pub struct GitHubActionsIntegration {
    config: BuildConfig,
}

impl GitHubActionsIntegration {
    pub fn new(config: BuildConfig) -> Self {
        Self { config }
    }
    
    #[instrument(skip(self))]
    pub fn generate_workflow(&self) -> String {
        format!(r#"
name: Performance Testing

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  performance-test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup CURSED
      run: |
        # Install CURSED compiler and tools
        curl -sSL https://install.cursed.dev | sh
        echo "$HOME/.cursed/bin" >> $GITHUB_PATH
    
    - name: Run Performance Tests
      run: |
        cursed-profile benchmark benchmarks/ \
          --baseline benchmarks/baseline.json \
          --regression-threshold {} \
          --output performance-results.json
    
    - name: Generate Performance Report
      run: |
        cursed-profile report performance-results.json \
          --format html \
          --output performance-report.html \
          --flame-graphs \
          --memory-analysis \
          --concurrency-analysis
    
    - name: Upload Performance Results
      uses: actions/upload-artifact@v3
      with:
        name: performance-results
        path: |
          performance-results.json
          performance-report.html
    
    - name: Comment PR with Results
      if: github.event_name == 'pull_request'
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const results = JSON.parse(fs.readFileSync('performance-results.json'));
          
          let comment = '## Performance Test Results\n\n';
          
          if (results.regression_analysis && results.regression_analysis.regressions.length > 0) {{
            comment += '⚠️ **Performance Regressions Detected**\n\n';
            for (const regression of results.regression_analysis.regressions) {{
              comment += `- ${{regression.benchmark_name}}: ${{regression.change_type}}\n`;
            }}
            comment += '\n';
          }}
          
          if (results.regression_analysis && results.regression_analysis.improvements.length > 0) {{
            comment += '✅ **Performance Improvements**\n\n';
            for (const improvement of results.regression_analysis.improvements) {{
              comment += `- ${{improvement.benchmark_name}}: ${{improvement.change_type}}\n`;
            }}
            comment += '\n';
          }}
          
          comment += '[View detailed report](https://github.com/${{{{ github.repository }}}}/actions/runs/${{{{ github.run_id }}}})\n';
          
          github.rest.issues.createComment({{
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: comment
          }});
"#, self.config.regression_threshold)
    }
    
    pub fn setup_annotations(&self, report: &CiReport) -> Vec<String> {
        let mut annotations = Vec::new();
        
        for regression in &report.regression_report.regressions {
            let level = match regression.severity {
                crate::profiling::benchmarking::RegressionSeverity::Critical => "error",
                crate::profiling::benchmarking::RegressionSeverity::High => "warning",
                _ => "notice",
            };
            
            annotations.push(format!(
                "::{}::Performance regression in {}: {}",
                level, regression.benchmark_name, regression.change
            ));
        }
        
        annotations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_integration_creation() {
        let config = BuildConfig::default();
        let integration = BuildIntegration::new(config);
        assert!(integration.profiler.is_none());
    }
    
    #[test]
    fn test_build_config_default() {
        let config = BuildConfig::default();
        assert_eq!(config.build_command, "cargo");
        assert!(config.enable_profiling);
        assert_eq!(config.regression_threshold, 10.0);
    }
    
    #[test]
    fn test_ci_status() {
        let status = CiStatus::Success;
        assert_eq!(status, CiStatus::Success);
    }
    
    #[test]
    fn test_github_actions_integration() {
        let config = BuildConfig::default();
        let integration = GitHubActionsIntegration::new(config);
        let workflow = integration.generate_workflow();
        
        assert!(workflow.contains("Performance Testing"));
        assert!(workflow.contains("cursed-profile"));
    }
}
