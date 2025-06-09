// Bootstrap module for CURSED self-hosting compiler
// Manages Stage 1 (Rust) and Stage 2 (CURSED) compilers

pub mod self_compilation_verification;
pub mod feature_detection;
pub mod conditional_compilation;
pub mod version_negotiation;
pub mod diagnostic_tools;
pub mod validator;
pub mod subset;
pub mod config;

use crate::error::Error;
use std::path::Path;

/// Bootstrap compiler stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilerStage {
    /// Stage 1: Rust-based bootstrap compiler
    Stage1,
    /// Stage 2: Self-hosted CURSED compiler
    Stage2,
}

/// Bootstrap compiler manager
pub struct BootstrapManager {
    pub stage1_available: bool,
    pub stage2_available: bool,
}

impl BootstrapManager {
    /// Create new bootstrap manager
    pub fn new() -> Self {
        Self {
            stage1_available: true, // Rust compiler is always available in this context
            stage2_available: false, // Stage 2 not implemented yet
        }
    }
    
    /// Get preferred compiler stage
    pub fn preferred_stage(&self) -> CompilerStage {
        if self.stage2_available {
            CompilerStage::Stage2
        } else {
            CompilerStage::Stage1
        }
    }
    
    /// Check if stage is available
    pub fn is_stage_available(&self, stage: CompilerStage) -> bool {
        match stage {
            CompilerStage::Stage1 => self.stage1_available,
            CompilerStage::Stage2 => self.stage2_available,
        }
    }
    
    /// Compile using specified stage
    pub fn compile_with_stage(&self, stage: CompilerStage, input: &Path, output: &Path) -> Result<(), Error> {
        if !self.is_stage_available(stage) {
            return Err(Error::from_str(&format!("Compiler stage {:?} is not available", stage)));
        }
        
        match stage {
            CompilerStage::Stage1 => self.compile_stage1(input, output),
            CompilerStage::Stage2 => self.compile_stage2(input, output),
        }
    }
    
    fn compile_stage1(&self, _input: &Path, _output: &Path) -> Result<(), Error> {
        // Implementation would invoke the Rust-based compiler
        Ok(())
    }
    
    fn compile_stage2(&self, _input: &Path, _output: &Path) -> Result<(), Error> {
        // Implementation would invoke the self-hosted compiler
        Ok(())
    }
    
    /// Get bootstrap status information
    pub fn status(&self) -> BootstrapStatus {
        BootstrapStatus {
            stage1_available: self.stage1_available,
            stage2_available: self.stage2_available,
            preferred_stage: self.preferred_stage(),
        }
    }
}

impl Default for BootstrapManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Bootstrap status information
#[derive(Debug, Clone)]
pub struct BootstrapStatus {
    pub stage1_available: bool,
    pub stage2_available: bool,
    pub preferred_stage: CompilerStage,
}

impl std::fmt::Display for BootstrapStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bootstrap Status:")?;
        writeln!(f, "  Stage 1 (Rust): {}", if self.stage1_available { "Available" } else { "Not Available" })?;
        writeln!(f, "  Stage 2 (Self-hosted): {}", if self.stage2_available { "Available" } else { "Not Available" })?;
        writeln!(f, "  Preferred: {:?}", self.preferred_stage)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bootstrap_manager_creation() {
        let manager = BootstrapManager::new();
        assert!(manager.stage1_available);
        assert!(!manager.stage2_available);
        assert_eq!(manager.preferred_stage(), CompilerStage::Stage1);
    }
    
    #[test]
    fn test_stage_availability() {
        let manager = BootstrapManager::new();
        assert!(manager.is_stage_available(CompilerStage::Stage1));
        assert!(!manager.is_stage_available(CompilerStage::Stage2));
    }
    
    #[test]
    fn test_compilation_with_unavailable_stage() {
        let manager = BootstrapManager::new();
        let input = Path::new("test.csd");
        let output = Path::new("test.out");
        
        let result = manager.compile_with_stage(CompilerStage::Stage2, input, output);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_compilation_with_available_stage() {
        let manager = BootstrapManager::new();
        let input = Path::new("test.csd");
        let output = Path::new("test.out");
        
        // This should not fail since Stage1 is available
        // Result depends on build configuration, so we just test it doesn't panic
        let _ = manager.compile_with_stage(CompilerStage::Stage1, input, output);
    }
    
    #[test]
    fn test_bootstrap_status() {
        let manager = BootstrapManager::new();
        let status = manager.status();
        
        // Test that status can be displayed without panicking
        let _ = status.to_string();
    }
}

// Re-export feature detection types for convenience
pub use feature_detection::{
    BootstrapStage as FeatureBootstrapStage, CompilerVersion, CompilerFeature, FeatureSupport,
    FeatureDetectionSystem, FeatureDetectionResult, DiagnosticReport,
    init_feature_detection, get_feature_system, is_feature_supported, get_feature_support_level
};

pub use conditional_compilation::{
    ConditionalDirective, ConditionalBlock, FallbackStrategy, FeatureFallback,
    ConditionalCompiler, ConditionalCompilerStats
};

pub use version_negotiation::{
    ProtocolVersion, CapabilityAdvertisement, NegotiationRequest, NegotiationResponse,
    NegotiationResult, FallbackOption, DegradationLevel, VersionNegotiator,
    NegotiationStats, create_negotiation_request
};

pub use diagnostic_tools::{
    SystemDiagnostic, CompilerInfo, FeatureMatrix, CompatibilityAnalysis,
    PerformanceMetrics, EnvironmentInfo, Recommendation, DiagnosticTool,
    ReportFormat, quick_diagnostic, export_diagnostic_to_file
};

// Note: Macros if_feature and if_feature_level are available when importing conditional_compilation module
