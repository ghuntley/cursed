/// Configuration modules for CURSED Language
/// 
/// Provides configuration management for various subsystems including
/// JIT compilation, debugging, optimization, and runtime behavior.

pub mod jit_config;
pub mod types;

pub use jit_config::{
    JitConfig, JitEngineConfig, JitCompilationConfig, JitRuntimeConfig,
    JitMonitoringConfig, JitMemoryConfig, JitOptimizationConfig, JitDebugConfig,
    PerformanceThresholds, parse_optimization_level, duration_to_secs, secs_to_duration
};
pub use types::{
    CursedConfig, ProjectConfig, CompilationConfig, DebugConfig, RuntimeConfig,
    OutputConfig, SecurityContext, CryptoParameters, ParserConfig, ModParser,
    GcConfig, PanicConfig, ErrorConfig, DebugFormat, TracingLevel, GcAlgorithm,
    SecurityLevel, HashAlgorithm, EncryptionAlgorithm, OutputFormat,
    PanicOutputFormat, ErrorOutputFormat,
};
