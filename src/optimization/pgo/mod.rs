//! Profile-Guided Optimization (PGO) System for CURSED
//! 
//! This module provides a comprehensive Profile-Guided Optimization system that includes:
//! - Runtime profile data collection 
//! - Efficient profile data storage and management
//! - PGO-guided LLVM optimization passes
//! - CLI integration for profile management
//! - Performance analysis and validation

pub mod profile_collector;
pub mod profile_storage;
pub mod profile_analyzer;
pub mod pgo_passes;
pub mod profile_manager;
pub mod instrumentation;
pub mod optimization_integration;

pub use profile_collector::{
    ProfileCollector, ProfileCollectorConfig, ProfileData, 
    FunctionProfile, BranchProfile, LoopProfile, MemoryProfile,
    CallSiteProfile, ProfileEvent, ProfileEventType,
};

pub use profile_storage::{
    ProfileStorage, ProfileStorageConfig, ProfileDatabase,
    ProfileFormat, ProfileVersion, ProfileMetadata,
    StorageStatistics, ProfileMerger,
};

pub use profile_analyzer::{
    ProfileAnalyzer, ProfileAnalysisConfig, ProfileAnalysisResult,
    HotFunctionAnalysis, BranchPredictionAnalysis, LoopAnalysis,
    MemoryAccessAnalysis, OptimizationOpportunity, ProfileInsight,
};

pub use pgo_passes::{
    PgoPassManager, PgoPassConfig, PgoOptimizationPass,
    InliningPass, BranchLayoutPass, LoopOptimizationPass,
    CodeLayoutPass, PassExecutionResult, PassStatistics,
};

pub use profile_manager::{
    ProfileManager, ProfileManagerConfig, ProfileSession,
    ProfileCommand, ProfileOperationResult, ProfileValidation,
    ProfileCompatibility, ProfileMigration,
};

pub use instrumentation::{
    ProfileInstrumentation, InstrumentationConfig, InstrumentationPass,
    CounterInstrumentation, TimingInstrumentation, EdgeInstrumentation,
    InstrumentationStatistics, InstrumentationType,
};

pub use optimization_integration::{
    PgoOptimizationIntegrator, PgoIntegrationConfig, OptimizationStrategy,
    PerformanceMetrics, OptimizationResult, OptimizationEffectiveness,
    RegressionDetection, PerformanceValidation,
};

use crate::error::{Error, Result};
use std::path::Path;
use std::time::Duration;

/// Main PGO system coordinator
pub struct PgoSystem {
    /// Profile collector for runtime data gathering
    pub collector: ProfileCollector,
    /// Profile storage for data persistence
    pub storage: ProfileStorage,
    /// Profile analyzer for optimization insights
    pub analyzer: ProfileAnalyzer,
    /// PGO pass manager for optimization execution
    pub pass_manager: PgoPassManager,
    /// Profile manager for data lifecycle management
    pub profile_manager: ProfileManager,
    /// Instrumentation system for code generation
    pub instrumentation: ProfileInstrumentation,
    /// Integration with existing optimization pipeline
    pub integration: PgoOptimizationIntegrator,
    /// System configuration
    config: PgoSystemConfig,
}

/// Configuration for the PGO system
#[derive(Debug, Clone)]
pub struct PgoSystemConfig {
    /// Enable profile collection during execution
    pub enable_collection: bool,
    /// Enable profile-guided optimization
    pub enable_optimization: bool,
    /// Profile data storage directory
    pub profile_directory: String,
    /// Minimum profile data quality threshold
    pub quality_threshold: f64,
    /// Performance improvement target (percentage)
    pub performance_target: f64,
    /// Maximum profile data age before refresh
    pub max_profile_age: Duration,
    /// Enable profile validation
    pub enable_validation: bool,
    /// Profile format version
    pub profile_version: ProfileVersion,
    /// Enable multi-run profile merging
    pub enable_merging: bool,
    /// Optimization aggressiveness level
    pub optimization_level: OptimizationAggressiveness,
}

/// Optimization aggressiveness levels
#[derive(Debug, Clone, Copy)]
pub enum OptimizationAggressiveness {
    Conservative,  // Safe optimizations only
    Moderate,      // Balanced risk/reward
    Aggressive,    // Maximum performance, higher risk
    Experimental,  // Cutting-edge optimizations
}

impl Default for PgoSystemConfig {
    fn default() -> Self {
        Self {
            enable_collection: true,
            enable_optimization: true,
            profile_directory: "target/pgo-profiles".to_string(),
            quality_threshold: 0.8, // 80% confidence threshold
            performance_target: 15.0, // 15% improvement target
            max_profile_age: Duration::from_secs(7 * 24 * 3600), // 1 week
            enable_validation: true,
            profile_version: ProfileVersion::V1_0,
            enable_merging: true,
            optimization_level: OptimizationAggressiveness::Moderate,
        }
    }
}

impl PgoSystem {
    /// Create new PGO system with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(PgoSystemConfig::default())
    }

    /// Create new PGO system with custom configuration
    pub fn with_config(config: PgoSystemConfig) -> Result<Self> {
        // Initialize collector
        let collector_config = ProfileCollectorConfig::from_pgo_config(&config);
        let collector = ProfileCollector::new(collector_config)?;

        // Initialize storage
        let storage_config = ProfileStorageConfig::from_pgo_config(&config);
        let storage = ProfileStorage::new(storage_config)?;

        // Initialize analyzer
        let analyzer_config = ProfileAnalysisConfig::from_pgo_config(&config);
        let analyzer = ProfileAnalyzer::new(analyzer_config)?;

        // Initialize pass manager
        let pass_config = PgoPassConfig::from_pgo_config(&config);
        let pass_manager = PgoPassManager::new(pass_config)?;

        // Initialize profile manager
        let manager_config = ProfileManagerConfig::from_pgo_config(&config);
        let profile_manager = ProfileManager::new(manager_config)?;

        // Initialize instrumentation
        let instrumentation_config = InstrumentationConfig::from_pgo_config(&config);
        let instrumentation = ProfileInstrumentation::new(instrumentation_config)?;

        // Initialize integration
        let integration_config = PgoIntegrationConfig::from_pgo_config(&config);
        let integration = PgoOptimizationIntegrator::new(integration_config)?;

        Ok(Self {
            collector,
            storage,
            analyzer,
            pass_manager,
            profile_manager,
            instrumentation,
            integration,
            config,
        })
    }

    /// Initialize PGO system for profile collection
    pub fn initialize_collection(&mut self, output_path: &Path) -> Result<()> {
        if !self.config.enable_collection {
            return Ok(());
        }

        // Initialize storage
        self.storage.initialize(output_path)?;
        
        // Configure collector
        self.collector.initialize()?;
        
        // Setup instrumentation
        self.instrumentation.prepare_for_collection()?;
        
        tracing::info!("PGO profile collection initialized at: {}", output_path.display());
        Ok(())
    }

    /// Initialize PGO system for optimization
    pub fn initialize_optimization(&mut self, profile_path: &Path) -> Result<()> {
        if !self.config.enable_optimization {
            return Ok(());
        }

        // Load profile data
        let profile_data = self.storage.load_profile(profile_path)?;
        
        // Validate profile quality
        if self.config.enable_validation {
            self.profile_manager.validate_profile(&profile_data)?;
        }
        
        // Analyze profile data
        let analysis = self.analyzer.analyze_profile(&profile_data)?;
        
        // Configure optimization passes
        self.pass_manager.configure_passes(&analysis)?;
        
        // Setup integration
        self.integration.initialize_with_profile(&profile_data, &analysis)?;
        
        tracing::info!("PGO optimization initialized with profile: {}", profile_path.display());
        Ok(())
    }

    /// Collect runtime profile data
    pub fn collect_profile_data(&mut self, execution_context: &ExecutionContext) -> Result<ProfileData> {
        if !self.config.enable_collection {
            return Err(Error::Other("Profile collection is disabled".to_string()));
        }

        self.collector.collect_execution_profile(execution_context)
    }

    /// Store profile data to persistent storage
    pub fn store_profile_data(&mut self, profile_data: &ProfileData) -> Result<()> {
        self.storage.store_profile(profile_data)
    }

    /// Optimize code using collected profile data
    pub fn optimize_with_profile(&mut self, module: &inkwell::module::Module) -> Result<OptimizationResult> {
        if !self.config.enable_optimization {
            return Err(Error::Other("PGO optimization is disabled".to_string()));
        }

        self.integration.optimize_module(module)
    }

    /// Get optimization recommendations based on profile analysis
    pub fn get_optimization_recommendations(&self, profile_data: &ProfileData) -> Result<Vec<OptimizationOpportunity>> {
        let analysis = self.analyzer.analyze_profile(profile_data)?;
        Ok(analysis.optimization_opportunities)
    }

    /// Validate optimization effectiveness
    pub fn validate_optimization_effectiveness(
        &self,
        baseline_metrics: &PerformanceMetrics,
        optimized_metrics: &PerformanceMetrics,
    ) -> Result<PerformanceValidation> {
        self.integration.validate_performance_improvement(baseline_metrics, optimized_metrics)
    }

    /// Get PGO system statistics
    pub fn get_system_statistics(&self) -> PgoSystemStatistics {
        PgoSystemStatistics {
            collection_stats: self.collector.get_statistics(),
            storage_stats: self.storage.get_statistics(),
            analysis_stats: self.analyzer.get_statistics(),
            pass_stats: self.pass_manager.get_statistics(),
            optimization_stats: self.integration.get_statistics(),
            profile_count: self.profile_manager.get_profile_count(),
            total_optimization_time: self.integration.get_total_optimization_time(),
            average_performance_improvement: self.integration.get_average_performance_improvement(),
        }
    }
}

/// Statistics for the entire PGO system
#[derive(Debug, Clone)]
pub struct PgoSystemStatistics {
    pub collection_stats: profile_collector::CollectionStatistics,
    pub storage_stats: StorageStatistics,
    pub analysis_stats: profile_analyzer::AnalysisStatistics,
    pub pass_stats: PassStatistics,
    pub optimization_stats: optimization_integration::OptimizationStatistics,
    pub profile_count: usize,
    pub total_optimization_time: Duration,
    pub average_performance_improvement: f64,
}

/// Execution context for profile collection
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Program command line arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env_vars: std::collections::HashMap<String, String>,
    /// Working directory
    pub working_dir: std::path::PathBuf,
    /// Input files or data
    pub input_data: Option<Vec<u8>>,
    /// Expected output for validation
    pub expected_output: Option<String>,
    /// Execution timeout
    pub timeout: Option<Duration>,
    /// Custom execution metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            env_vars: std::collections::HashMap::new(),
            working_dir: std::env::current_dir().unwrap_or_default(),
            input_data: None,
            expected_output: None,
            timeout: Some(Duration::from_secs(300)), // 5 minute default timeout
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl Default for PgoSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create default PGO system")
    }
}

/// Error types specific to PGO operations
#[derive(Debug, Clone)]
pub enum PgoError {
    /// Profile collection failed
    CollectionFailed(String),
    /// Profile storage failed
    StorageFailed(String),
    /// Profile analysis failed
    AnalysisFailed(String),
    /// Optimization failed
    OptimizationFailed(String),
    /// Profile validation failed
    ValidationFailed(String),
    /// Profile format incompatible
    IncompatibleFormat { expected: String, found: String },
    /// Insufficient profile data quality
    InsufficientQuality { actual: f64, required: f64 },
    /// Profile data too old
    ProfileTooOld { age: Duration, max_age: Duration },
}

impl std::fmt::Display for PgoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgoError::CollectionFailed(msg) => write!(f, "Profile collection failed: {}", msg),
            PgoError::StorageFailed(msg) => write!(f, "Profile storage failed: {}", msg),
            PgoError::AnalysisFailed(msg) => write!(f, "Profile analysis failed: {}", msg),
            PgoError::OptimizationFailed(msg) => write!(f, "PGO optimization failed: {}", msg),
            PgoError::ValidationFailed(msg) => write!(f, "Profile validation failed: {}", msg),
            PgoError::IncompatibleFormat { expected, found } => {
                write!(f, "Incompatible profile format: expected {}, found {}", expected, found)
            }
            PgoError::InsufficientQuality { actual, required } => {
                write!(f, "Insufficient profile quality: {:.2}% < {:.2}%", actual * 100.0, required * 100.0)
            }
            PgoError::ProfileTooOld { age, max_age } => {
                write!(f, "Profile too old: {:?} > {:?}", age, max_age)
            }
        }
    }
}

impl std::error::Error for PgoError {}
