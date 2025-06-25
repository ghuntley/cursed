use crate::error::CursedError;
// Build system integration and CI/CD support for profiling

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

// use crate::profiling::core::{ProfilerConfig, CursedProfiler, ProfilerError};
// use crate::profiling::benchmarking::{BenchmarkSuite, BenchmarkConfig, BenchmarkResults};
// use crate::profiling::reporting::{ReportGenerator, ReportConfig};

/// Build system integration for automated profiling
#[derive(Debug)]
pub struct BuildIntegration {
impl BuildIntegration {
    pub fn new(config: BuildConfig) -> Self {
        Self {
        }
    }
    
    #[instrument(skip(self))]
    pub fn setup_profiling_build(&mut self) -> crate::error::Result<()> {
        info!("Setting up profiling build integration");
        
        let profiler_config = ProfilerConfig {
        
        self.profiler = Some(CursedProfiler::new(profiler_config));
        Ok(())
    #[instrument(skip(self))]
    pub fn profile_build(&mut self, target: &str) -> crate::error::Result<()> {
        info!("Starting profiled build for target: {}", target);
        
        let session_name = format!("build_{}_{}", target, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        
        if let Some(profiler) = &mut self.profiler {
            profiler.start_session(session_name.clone())?;
        let build_start = std::time::Instant::now();
        
        // Execute build command
        let build_result = self.execute_build_command(target)?;
        
        let build_duration = build_start.elapsed();
        
        // Stop profiling
        let profile_data = if let Some(profiler) = &mut self.profiler {
            Some(profiler.stop_session()?)
        } else {
            None
        
        let result = BuildProfileResult {
        
        info!("Build profiling completed for target: {} in {:?}", target, build_duration);
        Ok(result)
    #[instrument(skip(self))]
    pub fn run_performance_tests(&self) -> crate::error::Result<()> {
        info!("Running performance tests");
        
        let mut test_results = HashMap::new();
        
        for test_config in &self.config.performance_tests {
            info!("Running performance test: {}", test_config.name);
            
            let mut suite = BenchmarkSuite::new(
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
            let results = suite.run_all()?;
            test_results.insert(test_config.name.clone(), results);
        let overall_pass = self.evaluate_overall_performance(&test_results);
        
        Ok(PerformanceTestResults {
        })
    #[instrument(skip(self))]
    pub fn detect_performance_regressions(&self, current: &PerformanceTestResults) -> crate::error::Result<()> {
        info!("Detecting performance regressions");
        
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (test_name, results) in &current.test_results {
            if let Some(analysis) = &results.regression_analysis {
                for regression in &analysis.regressions {
                    regressions.push(DetectedRegression {
                    });
                for improvement in &analysis.improvements {
                    improvements.push(DetectedImprovement {
                    });
                }
            }
        let critical_regressions = regressions.iter()
            .filter(|r| r.severity == crate::profiling::benchmarking::RegressionSeverity::Critical)
            .count();
        
        Ok(RegressionReport {
        })
    #[instrument(skip(self))]
    pub fn generate_ci_report(&self, results: &PerformanceTestResults) -> crate::error::Result<()> {
        info!("Generating CI/CD report");
        
        let regression_report = self.detect_performance_regressions(results)?;
        
        let mut status = CiStatus::Success;
        let mut messages = Vec::new();
        
        if regression_report.should_fail_build {
            status = CiStatus::Failure;
            messages.push(format!(
                regression_report.critical_count
            ));
        } else if !regression_report.regressions.is_empty() {
            status = CiStatus::Warning;
            messages.push(format!(
                regression_report.regressions.len()
            ));
        if !regression_report.improvements.is_empty() {
            messages.push(format!(
                regression_report.improvements.len()
            ));
        let report = CiReport {
        
        Ok(report)
    fn execute_build_command(&self, target: &str) -> crate::error::Result<()> {
        let mut command = Command::new(&self.config.build_command);
        command.args(&self.config.build_args);
        command.arg(target);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        let output = command.output().map_err(ProfilerError::IoError)?;
        
        Ok(BuildCommandResult {
        })
    fn collect_build_artifacts(&self, target: &str) -> crate::error::Result<()> {
        let mut artifacts = Vec::new();
        
        // Collect binary artifacts
        let target_path = self.config.output_directory.join(target);
        if target_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&target_path) {
                artifacts.push(BuildArtifact {
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
                        });
                    }
                }
            }
        }
        
        Ok(artifacts)
    fn load_benchmarks_from_file(&self, suite: &mut BenchmarkSuite, _path: &Path) -> crate::error::Result<()> {
        // In a real implementation, this would parse benchmark files
        // and add them to the suite
        warn!("Benchmark file loading not yet implemented");
        Ok(())
    fn evaluate_overall_performance(&self, test_results: &HashMap<String, BenchmarkResults>) -> bool {
        for results in test_results.values() {
            if let Some(analysis) = &results.regression_analysis {
                if analysis.has_critical_regressions() {
                    return false;
                }
            }
        }
        true
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
        }
    }
    
    fn collect_ci_artifacts(&self, _results: &PerformanceTestResults) -> crate::error::Result<()> {
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
                    });
                }
            }
        Ok(artifacts)
    }
}

/// Build integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_directory: PathBuf::from("target/cursed"),
            default_profiling_modes: vec![
        }
    }
/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestConfig {
/// Build profiling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfileResult {
/// Build command execution result
#[derive(Debug, Clone)]
struct BuildCommandResult {
/// Build artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
/// Types of build artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
/// Performance test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestResults {
/// Regression detection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionReport {
/// Detected performance regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedRegression {
/// Detected performance improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedImprovement {
/// CI/CD integration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiReport {
/// CI build status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
/// Performance summary for CI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
/// CI artifact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiArtifact {
/// Types of CI artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CiArtifactType {
/// GitHub Actions integration
#[derive(Debug)]
pub struct GitHubActionsIntegration {
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
          if (results.regression_analysis && results.regression_analysis.improvements.length > 0) {{
            comment += '✅ **Performance Improvements**\n\n';
            for (const improvement of results.regression_analysis.improvements) {{
              comment += `- ${{improvement.benchmark_name}}: ${{improvement.change_type}}\n`;
            }}
            comment += '\n';
          comment += '[View detailed report](https://github.com/${{{{ github.repository }}}}/actions/runs/${{{{ github.run_id }}}})\n';
          
          github.rest.issues.createComment({{
            body: comment
          }});
"#, self.config.regression_threshold)
    pub fn setup_annotations(&self, report: &CiReport) -> Vec<String> {
        let mut annotations = Vec::new();
        
        for regression in &report.regression_report.regressions {
            let level = match regression.severity {
            
            annotations.push(format!(
                level, regression.benchmark_name, regression.change
            ));
        annotations
    }
}

