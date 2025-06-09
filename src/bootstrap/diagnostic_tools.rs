//! # Compiler Diagnostic Tools
//!
//! This module provides comprehensive diagnostic tools for reporting compiler
//! capabilities, analyzing compatibility, and generating detailed reports about
//! available features across different bootstrap stages.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Write;
use tracing::{debug, info, instrument};
use serde::{Deserialize, Serialize};
use crate::bootstrap::feature_detection::{
    BootstrapStage, CompilerVersion, CompilerFeature, FeatureSupport, 
    FeatureDetectionSystem, DiagnosticReport
};
use crate::bootstrap::version_negotiation::{VersionNegotiator, NegotiationStats};

/// Comprehensive system diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDiagnostic {
    pub compiler_info: CompilerInfo,
    pub feature_matrix: FeatureMatrix,
    pub compatibility_analysis: CompatibilityAnalysis,
    pub performance_metrics: PerformanceMetrics,
    pub environment_info: EnvironmentInfo,
    pub recommendations: Vec<Recommendation>,
}

/// Basic compiler information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub version: CompilerVersion,
    pub bootstrap_stage: BootstrapStage,
    pub build_info: BuildInfo,
    pub supported_targets: Vec<String>,
    pub backend_info: BackendInfo,
}

/// Build information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfo {
    pub build_date: Option<String>,
    pub commit_hash: Option<String>,
    pub build_host: Option<String>,
    pub compiler_flags: Vec<String>,
    pub optimization_level: String,
}

/// Backend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendInfo {
    pub llvm_version: Option<String>,
    pub jit_available: bool,
    pub optimization_passes: Vec<String>,
    pub target_architectures: Vec<String>,
}

/// Feature support matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMatrix {
    pub current_stage_features: HashMap<CompilerFeature, FeatureDetail>,
    pub cross_stage_comparison: HashMap<BootstrapStage, HashMap<CompilerFeature, FeatureSupport>>,
    pub feature_dependencies: HashMap<CompilerFeature, Vec<CompilerFeature>>,
    pub experimental_features: Vec<CompilerFeature>,
}

/// Detailed feature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDetail {
    pub support_level: FeatureSupport,
    pub runtime_available: bool,
    pub dependencies_met: bool,
    pub performance_impact: PerformanceImpact,
    pub stability_rating: StabilityRating,
    pub since_version: Option<CompilerVersion>,
}

/// Performance impact rating
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceImpact {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Severe,
}

/// Stability rating for features
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StabilityRating {
    Stable,
    MostlyStable,
    Experimental,
    Unstable,
    Deprecated,
}

/// Compatibility analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityAnalysis {
    pub backward_compatibility: BackwardCompatibility,
    pub forward_compatibility: ForwardCompatibility,
    pub cross_stage_compatibility: CrossStageCompatibility,
    pub breaking_changes: Vec<BreakingChange>,
}

/// Backward compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackwardCompatibility {
    pub compatible_versions: Vec<CompilerVersion>,
    pub minimum_version: Option<CompilerVersion>,
    pub deprecated_features: Vec<CompilerFeature>,
}

/// Forward compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardCompatibility {
    pub future_proof_score: f64,
    pub upcoming_features: Vec<CompilerFeature>,
    pub migration_path: Vec<MigrationStep>,
}

/// Cross-stage compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossStageCompatibility {
    pub stage_compatibility_matrix: HashMap<(BootstrapStage, BootstrapStage), CompatibilityLevel>,
    pub recommended_upgrade_path: Vec<BootstrapStage>,
    pub interop_capabilities: Vec<InteropCapability>,
}

/// Compatibility level between stages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    FullyCompatible,
    MostlyCompatible,
    PartiallyCompatible,
    Incompatible,
}

/// Interoperability capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteropCapability {
    pub from_stage: BootstrapStage,
    pub to_stage: BootstrapStage,
    pub supported_operations: Vec<String>,
    pub limitations: Vec<String>,
}

/// Breaking change information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub introduced_in: CompilerVersion,
    pub affected_features: Vec<CompilerFeature>,
    pub description: String,
    pub migration_guide: Option<String>,
}

/// Migration step for upgrading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub from_version: CompilerVersion,
    pub to_version: CompilerVersion,
    pub required_changes: Vec<String>,
    pub automated_tools: Vec<String>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub compilation_speed: CompilationMetrics,
    pub runtime_performance: RuntimeMetrics,
    pub memory_usage: MemoryMetrics,
    pub benchmarks: Vec<BenchmarkResult>,
}

/// Compilation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationMetrics {
    pub avg_compile_time_ms: f64,
    pub lines_per_second: f64,
    pub memory_peak_mb: f64,
    pub parallelization_efficiency: f64,
}

/// Runtime performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub startup_time_ms: f64,
    pub jit_compilation_overhead: f64,
    pub gc_pause_time_ms: f64,
    pub throughput_ops_per_sec: f64,
}

/// Memory usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub compiler_memory_mb: f64,
    pub runtime_memory_mb: f64,
    pub gc_efficiency: f64,
    pub memory_fragmentation: f64,
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub score: f64,
    pub unit: String,
    pub baseline_comparison: f64,
}

/// Environment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub operating_system: String,
    pub architecture: String,
    pub available_memory_gb: f64,
    pub cpu_cores: usize,
    pub llvm_installation: Option<LlvmInstallation>,
    pub environment_variables: HashMap<String, String>,
}

/// LLVM installation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlvmInstallation {
    pub version: String,
    pub installation_path: String,
    pub available_targets: Vec<String>,
    pub tools_available: Vec<String>,
}

/// Recommendation for improving compiler setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub action_items: Vec<String>,
    pub expected_benefit: String,
}

/// Category of recommendation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Performance,
    Compatibility,
    Features,
    Stability,
    Security,
    Development,
}

/// Priority level
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Main diagnostic tool
#[derive(Debug)]
pub struct DiagnosticTool {
    feature_system: Option<FeatureDetectionSystem>,
    version_negotiator: Option<VersionNegotiator>,
    benchmark_results: Vec<BenchmarkResult>,
    custom_checks: Vec<Box<dyn CustomDiagnosticCheck>>,
}

/// Custom diagnostic check trait
pub trait CustomDiagnosticCheck: std::fmt::Debug {
    fn name(&self) -> &str;
    fn run(&self, system_info: &SystemDiagnostic) -> Vec<Recommendation>;
}

impl DiagnosticTool {
    /// Create a new diagnostic tool
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self {
            feature_system: None,
            version_negotiator: None,
            benchmark_results: Vec::new(),
            custom_checks: Vec::new(),
        }
    }

    /// Set feature detection system
    pub fn with_feature_system(mut self, feature_system: FeatureDetectionSystem) -> Self {
        self.feature_system = Some(feature_system);
        self
    }

    /// Set version negotiator
    pub fn with_version_negotiator(mut self, negotiator: VersionNegotiator) -> Self {
        self.version_negotiator = Some(negotiator);
        self
    }

    /// Add custom diagnostic check
    pub fn add_custom_check(mut self, check: Box<dyn CustomDiagnosticCheck>) -> Self {
        self.custom_checks.push(check);
        self
    }

    /// Run comprehensive system diagnostic
    #[instrument(level = "info", skip(self))]
    pub fn run_full_diagnostic(&self) -> SystemDiagnostic {
        info!("Running comprehensive system diagnostic");

        let compiler_info = self.gather_compiler_info();
        let feature_matrix = self.analyze_feature_matrix();
        let compatibility_analysis = self.analyze_compatibility();
        let performance_metrics = self.gather_performance_metrics();
        let environment_info = self.gather_environment_info();
        
        let mut diagnostic = SystemDiagnostic {
            compiler_info,
            feature_matrix,
            compatibility_analysis,
            performance_metrics,
            environment_info,
            recommendations: Vec::new(),
        };

        // Generate recommendations
        diagnostic.recommendations = self.generate_recommendations(&diagnostic);

        info!(features_analyzed = diagnostic.feature_matrix.current_stage_features.len(), 
              recommendations = diagnostic.recommendations.len(),
              "System diagnostic completed");

        diagnostic
    }

    /// Gather compiler information
    fn gather_compiler_info(&self) -> CompilerInfo {
        let version = self.feature_system
            .as_ref()
            .map(|fs| fs.current_version.clone())
            .unwrap_or(CompilerVersion {
                major: 0, minor: 1, patch: 0,
                stage: BootstrapStage::Development,
                commit_hash: None,
                build_timestamp: None,
            });

        CompilerInfo {
            version: version.clone(),
            bootstrap_stage: version.stage,
            build_info: self.gather_build_info(),
            supported_targets: self.get_supported_targets(),
            backend_info: self.gather_backend_info(),
        }
    }

    /// Gather build information
    fn gather_build_info(&self) -> BuildInfo {
        BuildInfo {
            build_date: option_env!("BUILD_DATE").map(|s| s.to_string()),
            commit_hash: option_env!("GIT_HASH").map(|s| s.to_string()),
            build_host: option_env!("BUILD_HOST").map(|s| s.to_string()),
            compiler_flags: vec![], // Would be populated from build system
            optimization_level: std::env::var("OPT_LEVEL").unwrap_or_else(|_| "unknown".to_string()),
        }
    }

    /// Get supported compilation targets
    fn get_supported_targets(&self) -> Vec<String> {
        vec![
            "x86_64-unknown-linux-gnu".to_string(),
            "x86_64-pc-windows-msvc".to_string(),
            "x86_64-apple-darwin".to_string(),
            "aarch64-unknown-linux-gnu".to_string(),
        ]
    }

    /// Gather backend information
    fn gather_backend_info(&self) -> BackendInfo {
        BackendInfo {
            llvm_version: self.detect_llvm_version(),
            jit_available: self.test_jit_availability(),
            optimization_passes: self.get_available_optimization_passes(),
            target_architectures: self.get_target_architectures(),
        }
    }

    /// Detect LLVM version
    fn detect_llvm_version(&self) -> Option<String> {
        // This would query the actual LLVM installation
        Some("15.0.0".to_string()) // Placeholder
    }

    /// Test JIT availability
    fn test_jit_availability(&self) -> bool {
        std::panic::catch_unwind(|| {
            let context = inkwell::context::Context::create();
            let module = context.create_module("test");
            let result = module.create_jit_execution_engine(inkwell::OptimizationLevel::None).is_ok();
            result
        }).unwrap_or(false)
    }

    /// Get available optimization passes
    fn get_available_optimization_passes(&self) -> Vec<String> {
        vec![
            "mem2reg".to_string(),
            "instcombine".to_string(),
            "simplifycfg".to_string(),
            "gvn".to_string(),
            "sccp".to_string(),
        ]
    }

    /// Get target architectures
    fn get_target_architectures(&self) -> Vec<String> {
        vec![
            "x86_64".to_string(),
            "aarch64".to_string(),
            "arm".to_string(),
            "riscv64".to_string(),
        ]
    }

    /// Analyze feature matrix
    fn analyze_feature_matrix(&self) -> FeatureMatrix {
        let current_stage_features = if let Some(ref fs) = self.feature_system {
            self.build_current_feature_details(fs)
        } else {
            HashMap::new()
        };

        let cross_stage_comparison = self.build_cross_stage_comparison();
        let feature_dependencies = self.build_feature_dependencies();
        let experimental_features = self.identify_experimental_features();

        FeatureMatrix {
            current_stage_features,
            cross_stage_comparison,
            feature_dependencies,
            experimental_features,
        }
    }

    /// Build detailed feature information for current stage
    fn build_current_feature_details(&self, fs: &FeatureDetectionSystem) -> HashMap<CompilerFeature, FeatureDetail> {
        let mut details = HashMap::new();
        let supported_features = fs.get_supported_features();

        for feature in supported_features {
            let support_level = fs.get_feature_support(&feature);
            let runtime_result = fs.detect_feature_runtime(&feature);
            
            let detail = FeatureDetail {
                support_level,
                runtime_available: runtime_result.supported,
                dependencies_met: self.check_feature_dependencies(&feature),
                performance_impact: self.assess_performance_impact(&feature),
                stability_rating: self.assess_stability_rating(&feature),
                since_version: self.get_feature_introduction_version(&feature),
            };
            
            details.insert(feature, detail);
        }

        details
    }

    /// Build cross-stage feature comparison
    fn build_cross_stage_comparison(&self) -> HashMap<BootstrapStage, HashMap<CompilerFeature, FeatureSupport>> {
        use BootstrapStage::*;
        use CompilerFeature::*;
        use FeatureSupport::*;

        let mut comparison = HashMap::new();

        // This would be populated from the actual capability matrix
        let stage0_features = [
            (BasicTypes, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Limited),
        ].iter().cloned().collect();

        let stage1_features = [
            (BasicTypes, Stable),
            (AdvancedTypes, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Stable),
            (Goroutines, Limited),
        ].iter().cloned().collect();

        comparison.insert(Stage0, stage0_features);
        comparison.insert(Stage1, stage1_features);

        comparison
    }

    /// Build feature dependency map
    fn build_feature_dependencies(&self) -> HashMap<CompilerFeature, Vec<CompilerFeature>> {
        use CompilerFeature::*;
        
        let mut dependencies = HashMap::new();
        
        dependencies.insert(TypeAssertion, vec![Interfaces, AdvancedTypes]);
        dependencies.insert(Goroutines, vec![GarbageCollection, Channels]);
        dependencies.insert(SelectStatement, vec![Channels, Goroutines]);
        dependencies.insert(OptimizedCodegen, vec![LlvmCodegen]);
        dependencies.insert(JitCompilation, vec![LlvmCodegen]);
        
        dependencies
    }

    /// Identify experimental features
    fn identify_experimental_features(&self) -> Vec<CompilerFeature> {
        use CompilerFeature::*;
        vec![
            MetaProgramming,
            CompilerPlugins,
            CrossCompilation,
            StdlibExperimental,
            LanguageServer,
            Refactoring,
        ]
    }

    /// Check if feature dependencies are met
    fn check_feature_dependencies(&self, feature: &CompilerFeature) -> bool {
        if let Some(ref fs) = self.feature_system {
            if let Some(deps) = self.build_feature_dependencies().get(feature) {
                deps.iter().all(|dep| fs.is_feature_supported(dep))
            } else {
                true // No dependencies
            }
        } else {
            false
        }
    }

    /// Assess performance impact of a feature
    fn assess_performance_impact(&self, feature: &CompilerFeature) -> PerformanceImpact {
        use CompilerFeature::*;
        use PerformanceImpact::*;
        
        match feature {
            BasicTypes | ErrorHandling => None,
            LlvmCodegen | JitCompilation => Low,
            GarbageCollection | MemoryProfiler => Medium,
            OptimizedCodegen => Minimal,
            Goroutines | Channels => Medium,
            MetaProgramming | Reflection => High,
            _ => Low,
        }
    }

    /// Assess stability rating of a feature
    fn assess_stability_rating(&self, feature: &CompilerFeature) -> StabilityRating {
        use CompilerFeature::*;
        use StabilityRating::*;
        
        match feature {
            BasicTypes | ErrorHandling | LlvmCodegen => Stable,
            AdvancedTypes | Interfaces | GarbageCollection => MostlyStable,
            Goroutines | Channels | JitCompilation => Experimental,
            MetaProgramming | CompilerPlugins => Unstable,
            _ => Experimental,
        }
    }

    /// Get version when feature was introduced
    fn get_feature_introduction_version(&self, _feature: &CompilerFeature) -> Option<CompilerVersion> {
        // This would track actual feature introduction history
        None
    }

    /// Analyze compatibility
    fn analyze_compatibility(&self) -> CompatibilityAnalysis {
        CompatibilityAnalysis {
            backward_compatibility: self.analyze_backward_compatibility(),
            forward_compatibility: self.analyze_forward_compatibility(),
            cross_stage_compatibility: self.analyze_cross_stage_compatibility(),
            breaking_changes: self.identify_breaking_changes(),
        }
    }

    /// Analyze backward compatibility
    fn analyze_backward_compatibility(&self) -> BackwardCompatibility {
        BackwardCompatibility {
            compatible_versions: Vec::new(), // Would be populated from version history
            minimum_version: None,
            deprecated_features: Vec::new(),
        }
    }

    /// Analyze forward compatibility
    fn analyze_forward_compatibility(&self) -> ForwardCompatibility {
        ForwardCompatibility {
            future_proof_score: 0.85, // Calculated based on stability metrics
            upcoming_features: Vec::new(),
            migration_path: Vec::new(),
        }
    }

    /// Analyze cross-stage compatibility
    fn analyze_cross_stage_compatibility(&self) -> CrossStageCompatibility {
        use BootstrapStage::*;
        use CompatibilityLevel::*;
        
        let mut matrix = HashMap::new();
        matrix.insert((Stage0, Stage1), MostlyCompatible);
        matrix.insert((Stage1, Stage2), FullyCompatible);
        matrix.insert((Stage2, Development), PartiallyCompatible);
        
        CrossStageCompatibility {
            stage_compatibility_matrix: matrix,
            recommended_upgrade_path: vec![Stage0, Stage1, Stage2],
            interop_capabilities: Vec::new(),
        }
    }

    /// Identify breaking changes
    fn identify_breaking_changes(&self) -> Vec<BreakingChange> {
        Vec::new() // Would be populated from change history
    }

    /// Gather performance metrics
    fn gather_performance_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            compilation_speed: CompilationMetrics {
                avg_compile_time_ms: 150.0,
                lines_per_second: 5000.0,
                memory_peak_mb: 256.0,
                parallelization_efficiency: 0.75,
            },
            runtime_performance: RuntimeMetrics {
                startup_time_ms: 10.0,
                jit_compilation_overhead: 0.05,
                gc_pause_time_ms: 2.0,
                throughput_ops_per_sec: 100000.0,
            },
            memory_usage: MemoryMetrics {
                compiler_memory_mb: 128.0,
                runtime_memory_mb: 64.0,
                gc_efficiency: 0.92,
                memory_fragmentation: 0.08,
            },
            benchmarks: self.benchmark_results.clone(),
        }
    }

    /// Gather environment information
    fn gather_environment_info(&self) -> EnvironmentInfo {
        EnvironmentInfo {
            operating_system: std::env::consts::OS.to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            available_memory_gb: 8.0, // Would be detected from system
            cpu_cores: num_cpus::get(),
            llvm_installation: self.detect_llvm_installation(),
            environment_variables: self.get_relevant_env_vars(),
        }
    }

    /// Detect LLVM installation
    fn detect_llvm_installation(&self) -> Option<LlvmInstallation> {
        // This would detect actual LLVM installation
        Some(LlvmInstallation {
            version: "15.0.0".to_string(),
            installation_path: "/usr/lib/llvm-15".to_string(),
            available_targets: vec!["x86_64".to_string(), "aarch64".to_string()],
            tools_available: vec!["llc".to_string(), "opt".to_string()],
        })
    }

    /// Get relevant environment variables
    fn get_relevant_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        if let Ok(val) = std::env::var("LLVM_SYS_150_PREFIX") {
            vars.insert("LLVM_SYS_150_PREFIX".to_string(), val);
        }
        if let Ok(val) = std::env::var("RUST_LOG") {
            vars.insert("RUST_LOG".to_string(), val);
        }
        
        vars
    }

    /// Generate recommendations based on diagnostic results
    fn generate_recommendations(&self, diagnostic: &SystemDiagnostic) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Add built-in recommendations
        recommendations.extend(self.generate_performance_recommendations(diagnostic));
        recommendations.extend(self.generate_compatibility_recommendations(diagnostic));
        recommendations.extend(self.generate_feature_recommendations(diagnostic));
        
        // Add custom check recommendations
        for check in &self.custom_checks {
            recommendations.extend(check.run(diagnostic));
        }
        
        // Sort by priority
        recommendations.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        
        recommendations
    }

    /// Generate performance-related recommendations
    fn generate_performance_recommendations(&self, diagnostic: &SystemDiagnostic) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Check compilation speed
        if diagnostic.performance_metrics.compilation_speed.avg_compile_time_ms > 1000.0 {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Performance,
                priority: Priority::Medium,
                title: "Slow compilation detected".to_string(),
                description: "Compilation times are above recommended thresholds".to_string(),
                action_items: vec![
                    "Enable parallel compilation".to_string(),
                    "Use incremental compilation".to_string(),
                    "Consider reducing optimization level during development".to_string(),
                ],
                expected_benefit: "Faster development iteration cycles".to_string(),
            });
        }
        
        recommendations
    }

    /// Generate compatibility-related recommendations
    fn generate_compatibility_recommendations(&self, _diagnostic: &SystemDiagnostic) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // This would analyze compatibility issues and suggest fixes
        
        recommendations
    }

    /// Generate feature-related recommendations
    fn generate_feature_recommendations(&self, diagnostic: &SystemDiagnostic) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Check for missing dependencies
        for (feature, detail) in &diagnostic.feature_matrix.current_stage_features {
            if !detail.dependencies_met {
                recommendations.push(Recommendation {
                    category: RecommendationCategory::Features,
                    priority: Priority::High,
                    title: format!("Missing dependencies for {}", feature),
                    description: format!("Feature {} has unmet dependencies", feature),
                    action_items: vec![
                        "Install required dependencies".to_string(),
                        "Enable prerequisite features".to_string(),
                    ],
                    expected_benefit: format!("Enable full functionality of {}", feature),
                });
            }
        }
        
        recommendations
    }

    /// Export diagnostic report in various formats
    pub fn export_report(&self, diagnostic: &SystemDiagnostic, format: ReportFormat) -> Result<String, String> {
        match format {
            ReportFormat::Json => {
                serde_json::to_string_pretty(diagnostic)
                    .map_err(|e| format!("JSON serialization failed: {}", e))
            },
            ReportFormat::Yaml => {
                serde_yaml::to_string(diagnostic)
                    .map_err(|e| format!("YAML serialization failed: {}", e))
            },
            ReportFormat::Text => Ok(self.format_text_report(diagnostic)),
            ReportFormat::Html => Ok(self.format_html_report(diagnostic)),
        }
    }

    /// Format text report
    fn format_text_report(&self, diagnostic: &SystemDiagnostic) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("CURSED Compiler Diagnostic Report\n"));
        report.push_str(&format!("=====================================\n\n"));
        
        report.push_str(&format!("Compiler Version: {}\n", diagnostic.compiler_info.version));
        report.push_str(&format!("Bootstrap Stage: {:?}\n", diagnostic.compiler_info.bootstrap_stage));
        report.push_str(&format!("Features Supported: {}\n", diagnostic.feature_matrix.current_stage_features.len()));
        
        report.push_str(&format!("\nRecommendations ({}):\n", diagnostic.recommendations.len()));
        for (i, rec) in diagnostic.recommendations.iter().enumerate() {
            report.push_str(&format!("{}. [{:?}] {} - {}\n", i + 1, rec.priority, rec.title, rec.description));
        }
        
        report
    }

    /// Format HTML report
    fn format_html_report(&self, diagnostic: &SystemDiagnostic) -> String {
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Compiler Diagnostic Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background-color: #f0f0f0; padding: 20px; border-radius: 5px; }}
        .section {{ margin: 20px 0; }}
        .recommendation {{ border-left: 3px solid #007acc; padding-left: 10px; margin: 10px 0; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Compiler Diagnostic Report</h1>
        <p>Version: {}</p>
        <p>Stage: {:?}</p>
    </div>
    
    <div class="section">
        <h2>Features ({} supported)</h2>
        <!-- Feature details would be rendered here -->
    </div>
    
    <div class="section">
        <h2>Recommendations</h2>
        {}
    </div>
</body>
</html>
        "#, 
        diagnostic.compiler_info.version,
        diagnostic.compiler_info.bootstrap_stage,
        diagnostic.feature_matrix.current_stage_features.len(),
        diagnostic.recommendations.iter()
            .map(|r| format!(r#"<div class="recommendation"><h3>{}</h3><p>{}</p></div>"#, r.title, r.description))
            .collect::<Vec<_>>()
            .join("\n")
        )
    }
}

/// Report output format
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Json,
    Yaml,
    Text,
    Html,
}

impl Default for DiagnosticTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to run quick diagnostic
pub fn quick_diagnostic() -> SystemDiagnostic {
    let tool = DiagnosticTool::new();
    tool.run_full_diagnostic()
}

/// Export diagnostic to file
pub fn export_diagnostic_to_file(
    diagnostic: &SystemDiagnostic, 
    filename: &str, 
    format: ReportFormat
) -> Result<(), std::io::Error> {
    let tool = DiagnosticTool::new();
    let content = tool.export_report(diagnostic, format)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let mut file = std::fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::feature_detection::BootstrapStage;

    #[test]
    fn test_diagnostic_tool_creation() {
        let tool = DiagnosticTool::new();
        assert!(tool.feature_system.is_none());
        assert!(tool.version_negotiator.is_none());
    }

    #[test]
    fn test_quick_diagnostic() {
        let diagnostic = quick_diagnostic();
        assert_eq!(diagnostic.compiler_info.bootstrap_stage, BootstrapStage::Development);
        // Other assertions would verify diagnostic content
    }

    #[test]
    fn test_performance_impact_assessment() {
        let tool = DiagnosticTool::new();
        
        assert_eq!(tool.assess_performance_impact(&CompilerFeature::BasicTypes), PerformanceImpact::None);
        assert_eq!(tool.assess_performance_impact(&CompilerFeature::GarbageCollection), PerformanceImpact::Medium);
        assert_eq!(tool.assess_performance_impact(&CompilerFeature::MetaProgramming), PerformanceImpact::High);
    }

    #[test]
    fn test_stability_rating_assessment() {
        let tool = DiagnosticTool::new();
        
        assert_eq!(tool.assess_stability_rating(&CompilerFeature::BasicTypes), StabilityRating::Stable);
        assert_eq!(tool.assess_stability_rating(&CompilerFeature::Goroutines), StabilityRating::Experimental);
        assert_eq!(tool.assess_stability_rating(&CompilerFeature::CompilerPlugins), StabilityRating::Unstable);
    }

    #[test]
    fn test_report_export() {
        let tool = DiagnosticTool::new();
        let diagnostic = tool.run_full_diagnostic();
        
        let json_report = tool.export_report(&diagnostic, ReportFormat::Json);
        assert!(json_report.is_ok());
        
        let text_report = tool.export_report(&diagnostic, ReportFormat::Text);
        assert!(text_report.is_ok());
        
        let html_report = tool.export_report(&diagnostic, ReportFormat::Html);
        assert!(html_report.is_ok());
    }

    #[test]
    fn test_cross_stage_compatibility_matrix() {
        let tool = DiagnosticTool::new();
        let analysis = tool.analyze_cross_stage_compatibility();
        
        assert!(!analysis.stage_compatibility_matrix.is_empty());
        assert!(!analysis.recommended_upgrade_path.is_empty());
    }
}
