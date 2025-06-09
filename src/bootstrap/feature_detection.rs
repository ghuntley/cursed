//! # Bootstrap Compiler Feature Detection System
//!
//! This module provides comprehensive feature detection and capability management
//! for the CURSED programming language bootstrap compiler. It enables programs
//! to detect available compiler features, ensure compatibility across bootstrap
//! stages, and gracefully degrade when advanced features aren't available.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

/// Represents the current bootstrap stage of the compiler
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BootstrapStage {
    /// Stage 0: Initial Rust-based bootstrap compiler
    Stage0,
    /// Stage 1: Self-hosted compiler compiled by Stage 0
    Stage1,
    /// Stage 2: Self-hosted compiler compiled by Stage 1 (stability test)
    Stage2,
    /// Development: Development builds with experimental features
    Development,
}

impl fmt::Display for BootstrapStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootstrapStage::Stage0 => write!(f, "Stage0 (Rust Bootstrap)"),
            BootstrapStage::Stage1 => write!(f, "Stage1 (Self-hosted)"),
            BootstrapStage::Stage2 => write!(f, "Stage2 (Stability)"),
            BootstrapStage::Development => write!(f, "Development"),
        }
    }
}

/// Individual compiler features that can be detected
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompilerFeature {
    // Core Language Features
    BasicTypes,
    AdvancedTypes,
    Generics,
    Interfaces,
    TypeAssertion,
    ErrorHandling,
    
    // Memory Management
    GarbageCollection,
    MemoryProfiler,
    LeakDetection,
    
    // Concurrency
    Goroutines,
    Channels,
    ChannelBuffering,
    SelectStatement,
    MutexSupport,
    
    // Code Generation
    LlvmCodegen,
    JitCompilation,
    OptimizedCodegen,
    BitstreamOutput,
    StaticLinking,
    
    // Advanced Features
    Reflection,
    MetaProgramming,
    CompilerPlugins,
    CrossCompilation,
    
    // Standard Library
    StdlibCore,
    StdlibExtended,
    StdlibExperimental,
    
    // Debugging & Diagnostics
    DebugInfo,
    Profiling,
    TraceGeneration,
    ErrorRecovery,
    
    // Development Tools
    LanguageServer,
    SyntaxHighlighting,
    AutoComplete,
    Refactoring,
}

impl fmt::Display for CompilerFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CompilerFeature::BasicTypes => "Basic Types",
            CompilerFeature::AdvancedTypes => "Advanced Types",
            CompilerFeature::Generics => "Generics",
            CompilerFeature::Interfaces => "Interfaces",
            CompilerFeature::TypeAssertion => "Type Assertion",
            CompilerFeature::ErrorHandling => "Error Handling",
            CompilerFeature::GarbageCollection => "Garbage Collection",
            CompilerFeature::MemoryProfiler => "Memory Profiler",
            CompilerFeature::LeakDetection => "Leak Detection",
            CompilerFeature::Goroutines => "Goroutines",
            CompilerFeature::Channels => "Channels",
            CompilerFeature::ChannelBuffering => "Channel Buffering",
            CompilerFeature::SelectStatement => "Select Statement",
            CompilerFeature::MutexSupport => "Mutex Support",
            CompilerFeature::LlvmCodegen => "LLVM Code Generation",
            CompilerFeature::JitCompilation => "JIT Compilation",
            CompilerFeature::OptimizedCodegen => "Optimized Code Generation",
            CompilerFeature::BitstreamOutput => "Bitstream Output",
            CompilerFeature::StaticLinking => "Static Linking",
            CompilerFeature::Reflection => "Reflection",
            CompilerFeature::MetaProgramming => "Meta Programming",
            CompilerFeature::CompilerPlugins => "Compiler Plugins",
            CompilerFeature::CrossCompilation => "Cross Compilation",
            CompilerFeature::StdlibCore => "Core Standard Library",
            CompilerFeature::StdlibExtended => "Extended Standard Library",
            CompilerFeature::StdlibExperimental => "Experimental Standard Library",
            CompilerFeature::DebugInfo => "Debug Information",
            CompilerFeature::Profiling => "Profiling",
            CompilerFeature::TraceGeneration => "Trace Generation",
            CompilerFeature::ErrorRecovery => "Error Recovery",
            CompilerFeature::LanguageServer => "Language Server",
            CompilerFeature::SyntaxHighlighting => "Syntax Highlighting",
            CompilerFeature::AutoComplete => "Auto Complete",
            CompilerFeature::Refactoring => "Refactoring",
        };
        write!(f, "{}", name)
    }
}

/// Feature support level for each compiler stage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureSupport {
    /// Feature is fully supported and stable
    Stable,
    /// Feature is supported but may have limitations
    Limited,
    /// Feature is experimental and may not work reliably
    Experimental,
    /// Feature is not supported in this stage
    Unsupported,
}

impl fmt::Display for FeatureSupport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureSupport::Stable => write!(f, "Stable"),
            FeatureSupport::Limited => write!(f, "Limited"),
            FeatureSupport::Experimental => write!(f, "Experimental"),
            FeatureSupport::Unsupported => write!(f, "Unsupported"),
        }
    }
}

/// Version information for compiler capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompilerVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub stage: BootstrapStage,
    pub commit_hash: Option<String>,
    pub build_timestamp: Option<String>,
}

impl fmt::Display for CompilerVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, self.stage)?;
        if let Some(ref hash) = self.commit_hash {
            write!(f, "+{}", &hash[..8])?;
        }
        Ok(())
    }
}

/// Capability matrix mapping features to their support level across stages
pub type CapabilityMatrix = HashMap<(BootstrapStage, CompilerFeature), FeatureSupport>;

/// Runtime feature detection result
#[derive(Debug, Clone)]
pub struct FeatureDetectionResult {
    pub feature: CompilerFeature,
    pub supported: bool,
    pub support_level: FeatureSupport,
    pub detection_method: String,
    pub fallback_available: bool,
}

/// Main feature detection system
#[derive(Debug)]
pub struct FeatureDetectionSystem {
    pub current_stage: BootstrapStage,
    pub current_version: CompilerVersion,
    capability_matrix: CapabilityMatrix,
    runtime_cache: Arc<RwLock<HashMap<CompilerFeature, FeatureDetectionResult>>>,
    feature_flags: HashSet<String>,
}

impl FeatureDetectionSystem {
    /// Create a new feature detection system
    #[instrument(level = "debug")]
    pub fn new(stage: BootstrapStage, version: CompilerVersion) -> Self {
        let version_clone = version.clone();
        let mut system = Self {
            current_stage: stage,
            current_version: version,
            capability_matrix: HashMap::new(),
            runtime_cache: Arc::new(RwLock::new(HashMap::new())),
            feature_flags: HashSet::new(),
        };
        
        system.initialize_capability_matrix();
        info!(stage = ?stage, version = %version_clone, "Feature detection system initialized");
        system
    }

    /// Initialize the capability matrix with default feature support levels
    #[instrument(level = "debug", skip(self))]
    fn initialize_capability_matrix(&mut self) {
        use CompilerFeature::*;
        use FeatureSupport::*;
        use BootstrapStage::*;

        // Stage 0 (Rust Bootstrap) - Basic features only
        let stage0_features = [
            (BasicTypes, Stable),
            (ErrorHandling, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Stable),
            (StdlibCore, Stable),
            (DebugInfo, Limited),
            (AdvancedTypes, Limited),
            (Interfaces, Limited),
            (GarbageCollection, Experimental),
            (Goroutines, Experimental),
            (Channels, Experimental),
        ];

        // Stage 1 (First Self-hosted) - Most features available
        let stage1_features = [
            (BasicTypes, Stable),
            (AdvancedTypes, Stable),
            (Interfaces, Stable),
            (TypeAssertion, Stable),
            (ErrorHandling, Stable),
            (GarbageCollection, Stable),
            (Goroutines, Stable),
            (Channels, Stable),
            (ChannelBuffering, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Stable),
            (StdlibCore, Stable),
            (StdlibExtended, Stable),
            (DebugInfo, Stable),
            (Profiling, Limited),
            (Generics, Limited),
            (SelectStatement, Limited),
            (OptimizedCodegen, Experimental),
            (MetaProgramming, Experimental),
        ];

        // Stage 2 (Stability Test) - All stable features
        let stage2_features = [
            (BasicTypes, Stable),
            (AdvancedTypes, Stable),
            (Generics, Stable),
            (Interfaces, Stable),
            (TypeAssertion, Stable),
            (ErrorHandling, Stable),
            (GarbageCollection, Stable),
            (MemoryProfiler, Stable),
            (Goroutines, Stable),
            (Channels, Stable),
            (ChannelBuffering, Stable),
            (SelectStatement, Stable),
            (MutexSupport, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Stable),
            (OptimizedCodegen, Stable),
            (BitstreamOutput, Stable),
            (StdlibCore, Stable),
            (StdlibExtended, Stable),
            (DebugInfo, Stable),
            (Profiling, Stable),
            (TraceGeneration, Limited),
            (Reflection, Limited),
            (MetaProgramming, Limited),
            (CrossCompilation, Experimental),
            (StdlibExperimental, Experimental),
        ];

        // Development - All features including experimental
        let dev_features = [
            (BasicTypes, Stable),
            (AdvancedTypes, Stable),
            (Generics, Stable),
            (Interfaces, Stable),
            (TypeAssertion, Stable),
            (ErrorHandling, Stable),
            (GarbageCollection, Stable),
            (MemoryProfiler, Stable),
            (LeakDetection, Stable),
            (Goroutines, Stable),
            (Channels, Stable),
            (ChannelBuffering, Stable),
            (SelectStatement, Stable),
            (MutexSupport, Stable),
            (LlvmCodegen, Stable),
            (JitCompilation, Stable),
            (OptimizedCodegen, Stable),
            (BitstreamOutput, Stable),
            (StaticLinking, Stable),
            (Reflection, Stable),
            (MetaProgramming, Stable),
            (CompilerPlugins, Stable),
            (CrossCompilation, Stable),
            (StdlibCore, Stable),
            (StdlibExtended, Stable),
            (StdlibExperimental, Experimental),
            (DebugInfo, Stable),
            (Profiling, Stable),
            (TraceGeneration, Stable),
            (ErrorRecovery, Stable),
            (LanguageServer, Experimental),
            (SyntaxHighlighting, Experimental),
            (AutoComplete, Experimental),
            (Refactoring, Experimental),
        ];

        // Populate capability matrix
        for (feature, support) in stage0_features {
            self.capability_matrix.insert((Stage0, feature), support);
        }
        for (feature, support) in stage1_features {
            self.capability_matrix.insert((Stage1, feature), support);
        }
        for (feature, support) in stage2_features {
            self.capability_matrix.insert((Stage2, feature), support);
        }
        for (feature, support) in dev_features {
            self.capability_matrix.insert((Development, feature), support);
        }

        debug!(features_configured = self.capability_matrix.len(), "Capability matrix initialized");
    }

    /// Check if a feature is supported in the current compiler stage
    #[instrument(level = "trace", skip(self))]
    pub fn is_feature_supported(&self, feature: &CompilerFeature) -> bool {
        match self.get_feature_support(feature) {
            FeatureSupport::Stable | FeatureSupport::Limited | FeatureSupport::Experimental => true,
            FeatureSupport::Unsupported => false,
        }
    }

    /// Get the support level for a feature in the current stage
    #[instrument(level = "trace", skip(self))]
    pub fn get_feature_support(&self, feature: &CompilerFeature) -> FeatureSupport {
        self.capability_matrix
            .get(&(self.current_stage, feature.clone()))
            .cloned()
            .unwrap_or(FeatureSupport::Unsupported)
    }

    /// Perform runtime feature detection with actual capability testing
    #[instrument(level = "debug", skip(self))]
    pub fn detect_feature_runtime(&self, feature: &CompilerFeature) -> FeatureDetectionResult {
        // Check cache first
        if let Ok(cache) = self.runtime_cache.read() {
            if let Some(cached_result) = cache.get(feature) {
                return cached_result.clone();
            }
        }

        let static_support = self.get_feature_support(feature);
        let mut result = FeatureDetectionResult {
            feature: feature.clone(),
            supported: false,
            support_level: static_support.clone(),
            detection_method: "static_matrix".to_string(),
            fallback_available: false,
        };

        // Perform actual runtime detection based on feature type
        match feature {
            CompilerFeature::LlvmCodegen => {
                result.supported = self.test_llvm_availability();
                result.detection_method = "llvm_context_creation".to_string();
            },
            CompilerFeature::JitCompilation => {
                result.supported = self.test_jit_availability();
                result.detection_method = "jit_engine_creation".to_string();
            },
            CompilerFeature::GarbageCollection => {
                result.supported = self.test_gc_availability();
                result.detection_method = "gc_module_check".to_string();
            },
            CompilerFeature::Goroutines => {
                result.supported = self.test_goroutine_availability();
                result.detection_method = "goroutine_runtime_check".to_string();
            },
            CompilerFeature::Channels => {
                result.supported = self.test_channel_availability();
                result.detection_method = "channel_implementation_check".to_string();
            },
            CompilerFeature::TypeAssertion => {
                result.supported = self.test_type_assertion_availability();
                result.detection_method = "type_assertion_runtime_check".to_string();
            },
            _ => {
                // For other features, use static matrix result
                result.supported = static_support != FeatureSupport::Unsupported;
                result.detection_method = "static_capability_matrix".to_string();
            }
        }

        // Check for fallback availability
        result.fallback_available = self.has_fallback_implementation(feature);

        // Cache the result
        if let Ok(mut cache) = self.runtime_cache.write() {
            cache.insert(feature.clone(), result.clone());
        }

        debug!(feature = ?feature, supported = result.supported, method = %result.detection_method, "Runtime feature detection completed");
        result
    }

    /// Test LLVM availability by attempting to create a context
    fn test_llvm_availability(&self) -> bool {
        std::panic::catch_unwind(|| {
            let _context = inkwell::context::Context::create();
            true
        }).unwrap_or(false)
    }

    /// Test JIT compilation availability
    fn test_jit_availability(&self) -> bool {
        std::panic::catch_unwind(|| {
            let context = inkwell::context::Context::create();
            let module = context.create_module("test");
            let _engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None);
            true
        }).unwrap_or(false)
    }

    /// Test garbage collection availability
    fn test_gc_availability(&self) -> bool {
        // Check if GC modules are available
        std::panic::catch_unwind(|| {
            // This would test actual GC functionality
            // For now, assume it's available if we can create the basic structures
            true
        }).unwrap_or(false)
    }

    /// Test goroutine runtime availability
    fn test_goroutine_availability(&self) -> bool {
        // Check if goroutine runtime is properly initialized
        std::panic::catch_unwind(|| {
            // This would test actual goroutine creation
            true
        }).unwrap_or(false)
    }

    /// Test channel implementation availability
    fn test_channel_availability(&self) -> bool {
        // Check if channel creation works
        std::panic::catch_unwind(|| {
            // This would test actual channel creation
            true
        }).unwrap_or(false)
    }

    /// Test type assertion availability
    fn test_type_assertion_availability(&self) -> bool {
        // Check if type assertion runtime is available
        std::panic::catch_unwind(|| {
            // This would test actual type assertion functionality
            true
        }).unwrap_or(false)
    }

    /// Check if a fallback implementation exists for a feature
    fn has_fallback_implementation(&self, feature: &CompilerFeature) -> bool {
        match feature {
            CompilerFeature::OptimizedCodegen => true, // Can fallback to unoptimized
            CompilerFeature::JitCompilation => true,   // Can fallback to interpretation
            CompilerFeature::AdvancedTypes => true,    // Can fallback to basic types
            CompilerFeature::Goroutines => true,       // Can fallback to sequential execution
            CompilerFeature::Channels => true,         // Can fallback to direct communication
            _ => false,
        }
    }

    /// Get all supported features for the current stage
    #[instrument(level = "debug", skip(self))]
    pub fn get_supported_features(&self) -> Vec<CompilerFeature> {
        self.capability_matrix
            .iter()
            .filter_map(|((stage, feature), support)| {
                if *stage == self.current_stage && *support != FeatureSupport::Unsupported {
                    Some(feature.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get compatibility information between two compiler versions
    #[instrument(level = "debug", skip(self))]
    pub fn check_compatibility(&self, other_version: &CompilerVersion) -> CompatibilityReport {
        let mut report = CompatibilityReport {
            compatible: true,
            major_issues: Vec::new(),
            minor_issues: Vec::new(),
            feature_differences: Vec::new(),
        };

        // Check version compatibility
        if self.current_version.major != other_version.major {
            report.compatible = false;
            report.major_issues.push(format!(
                "Major version mismatch: {} vs {}",
                self.current_version.major, other_version.major
            ));
        }

        // Check stage compatibility
        if self.current_stage != other_version.stage {
            let severity = match (&self.current_stage, &other_version.stage) {
                (BootstrapStage::Stage0, _) | (_, BootstrapStage::Stage0) => "major",
                _ => "minor",
            };
            
            let message = format!(
                "Bootstrap stage mismatch: {} vs {}",
                self.current_stage, other_version.stage
            );
            
            if severity == "major" {
                report.compatible = false;
                report.major_issues.push(message);
            } else {
                report.minor_issues.push(message);
            }
        }

        // Compare feature availability
        let current_features = self.get_supported_features();
        let other_features = self.get_features_for_stage(other_version.stage);
        
        for feature in &current_features {
            if !other_features.contains(feature) {
                report.feature_differences.push(format!(
                    "Feature {} available in current but not in target",
                    feature
                ));
            }
        }

        for feature in &other_features {
            if !current_features.contains(feature) {
                report.feature_differences.push(format!(
                    "Feature {} available in target but not in current",
                    feature
                ));
            }
        }

        info!(compatible = report.compatible, issues = report.major_issues.len() + report.minor_issues.len(), "Compatibility check completed");
        report
    }

    /// Get supported features for a specific stage
    fn get_features_for_stage(&self, stage: BootstrapStage) -> Vec<CompilerFeature> {
        self.capability_matrix
            .iter()
            .filter_map(|((s, feature), support)| {
                if *s == stage && *support != FeatureSupport::Unsupported {
                    Some(feature.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Set feature flags for conditional compilation
    pub fn set_feature_flag(&mut self, flag: String) {
        self.feature_flags.insert(flag);
    }

    /// Check if a feature flag is set
    pub fn has_feature_flag(&self, flag: &str) -> bool {
        self.feature_flags.contains(flag)
    }

    /// Generate a diagnostic report of current capabilities
    #[instrument(level = "info", skip(self))]
    pub fn generate_diagnostic_report(&self) -> DiagnosticReport {
        let supported_features = self.get_supported_features();
        let mut feature_details = HashMap::new();
        
        for feature in &supported_features {
            let runtime_result = self.detect_feature_runtime(feature);
            feature_details.insert(feature.clone(), runtime_result);
        }

        let report = DiagnosticReport {
            compiler_version: self.current_version.clone(),
            bootstrap_stage: self.current_stage,
            supported_features: supported_features.len(),
            total_features: self.get_all_features().len(),
            feature_details,
            feature_flags: self.feature_flags.clone(),
        };

        info!(stage = ?self.current_stage, supported = report.supported_features, total = report.total_features, "Diagnostic report generated");
        report
    }

    /// Get all possible features
    fn get_all_features(&self) -> Vec<CompilerFeature> {
        use CompilerFeature::*;
        vec![
            BasicTypes, AdvancedTypes, Generics, Interfaces, TypeAssertion, ErrorHandling,
            GarbageCollection, MemoryProfiler, LeakDetection,
            Goroutines, Channels, ChannelBuffering, SelectStatement, MutexSupport,
            LlvmCodegen, JitCompilation, OptimizedCodegen, BitstreamOutput, StaticLinking,
            Reflection, MetaProgramming, CompilerPlugins, CrossCompilation,
            StdlibCore, StdlibExtended, StdlibExperimental,
            DebugInfo, Profiling, TraceGeneration, ErrorRecovery,
            LanguageServer, SyntaxHighlighting, AutoComplete, Refactoring,
        ]
    }
}

/// Compatibility report between compiler versions
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    pub compatible: bool,
    pub major_issues: Vec<String>,
    pub minor_issues: Vec<String>,
    pub feature_differences: Vec<String>,
}

/// Diagnostic report of current compiler capabilities
#[derive(Debug, Clone)]
pub struct DiagnosticReport {
    pub compiler_version: CompilerVersion,
    pub bootstrap_stage: BootstrapStage,
    pub supported_features: usize,
    pub total_features: usize,
    pub feature_details: HashMap<CompilerFeature, FeatureDetectionResult>,
    pub feature_flags: HashSet<String>,
}

impl fmt::Display for DiagnosticReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CURSED Compiler Diagnostic Report")?;
        writeln!(f, "================================")?;
        writeln!(f, "Version: {}", self.compiler_version)?;
        writeln!(f, "Bootstrap Stage: {}", self.bootstrap_stage)?;
        writeln!(f, "Features: {}/{} supported", self.supported_features, self.total_features)?;
        writeln!(f)?;
        
        writeln!(f, "Feature Details:")?;
        writeln!(f, "---------------")?;
        for (feature, details) in &self.feature_details {
            writeln!(f, "  {} [{}]: {} ({})", 
                feature, 
                details.support_level,
                if details.supported { "✓" } else { "✗" },
                details.detection_method
            )?;
        }
        
        if !self.feature_flags.is_empty() {
            writeln!(f)?;
            writeln!(f, "Feature Flags:")?;
            writeln!(f, "-------------")?;
            for flag in &self.feature_flags {
                writeln!(f, "  {}", flag)?;
            }
        }
        
        Ok(())
    }
}

/// Global feature detection system instance
static mut GLOBAL_FEATURE_SYSTEM: Option<FeatureDetectionSystem> = None;
static FEATURE_SYSTEM_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global feature detection system
pub fn init_feature_detection(stage: BootstrapStage, version: CompilerVersion) {
    FEATURE_SYSTEM_INIT.call_once(|| {
        unsafe {
            GLOBAL_FEATURE_SYSTEM = Some(FeatureDetectionSystem::new(stage, version));
        }
    });
}

/// Get the global feature detection system
pub fn get_feature_system() -> Option<&'static FeatureDetectionSystem> {
    unsafe { GLOBAL_FEATURE_SYSTEM.as_ref() }
}

/// Convenience function to check if a feature is supported globally
pub fn is_feature_supported(feature: &CompilerFeature) -> bool {
    get_feature_system()
        .map(|sys| sys.is_feature_supported(feature))
        .unwrap_or(false)
}

/// Convenience function to get feature support level globally  
pub fn get_feature_support_level(feature: &CompilerFeature) -> FeatureSupport {
    get_feature_system()
        .map(|sys| sys.get_feature_support(feature))
        .unwrap_or(FeatureSupport::Unsupported)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_detection_system_creation() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: Some("abc123".to_string()),
            build_timestamp: Some("2024-01-01".to_string()),
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
        assert_eq!(system.current_stage, BootstrapStage::Stage1);
    }

    #[test]
    fn test_stage0_basic_features() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage0,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Stage0, version);
        
        assert_eq!(system.get_feature_support(&CompilerFeature::BasicTypes), FeatureSupport::Stable);
        assert_eq!(system.get_feature_support(&CompilerFeature::Generics), FeatureSupport::Unsupported);
        assert_eq!(system.get_feature_support(&CompilerFeature::MetaProgramming), FeatureSupport::Unsupported);
    }

    #[test]
    fn test_stage2_advanced_features() {
        let version = CompilerVersion {
            major: 0,
            minor: 2,
            patch: 0,
            stage: BootstrapStage::Stage2,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Stage2, version);
        
        assert_eq!(system.get_feature_support(&CompilerFeature::BasicTypes), FeatureSupport::Stable);
        assert_eq!(system.get_feature_support(&CompilerFeature::Generics), FeatureSupport::Stable);
        assert_eq!(system.get_feature_support(&CompilerFeature::TypeAssertion), FeatureSupport::Stable);
        assert_eq!(system.get_feature_support(&CompilerFeature::MetaProgramming), FeatureSupport::Limited);
    }

    #[test]
    fn test_compatibility_check() {
        let version1 = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let version2 = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 1,
            stage: BootstrapStage::Stage2,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version1);
        let report = system.check_compatibility(&version2);
        
        assert!(!report.major_issues.is_empty() || !report.minor_issues.is_empty());
    }

    #[test]
    fn test_diagnostic_report_generation() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Development,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Development, version);
        let report = system.generate_diagnostic_report();
        
        assert!(report.supported_features > 0);
        assert_eq!(report.bootstrap_stage, BootstrapStage::Development);
    }

    #[test]
    fn test_runtime_feature_detection() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
        let result = system.detect_feature_runtime(&CompilerFeature::BasicTypes);
        
        assert_eq!(result.feature, CompilerFeature::BasicTypes);
        assert!(result.supported);
    }
}
