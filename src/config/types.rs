use crate::error::CursedError;
/// Configuration types for the CURSED compiler
/// 
/// This module provides type definitions for various configuration
/// systems that are commonly referenced but may be missing.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Comprehensive configuration for the CURSED compiler
#[derive(Debug, Clone)]
pub struct CursedConfig {
    /// Project configuration
    pub project: ProjectConfig,
    /// Compilation settings
    pub compilation: CompilationConfig,
    /// Optimization settings
    pub optimization: crate::optimization::OptimizationConfig,
    /// Debug settings
    pub debug: DebugConfig,
    /// Runtime settings
    pub runtime: RuntimeConfig,
    /// Output settings
    pub output: OutputConfig,
}

impl CursedConfig {
    /// Create a new default configuration
    pub fn new() -> Self {
        Self {
            project: ProjectConfig::default(),
            compilation: CompilationConfig::default(),
            optimization: crate::optimization::OptimizationConfig::dev_config(),
            debug: DebugConfig::default(),
            runtime: RuntimeConfig::default(),
            output: OutputConfig::default(),
        }
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self {
            project: ProjectConfig::default(),
            compilation: CompilationConfig::development(),
            optimization: crate::optimization::OptimizationConfig::dev_config(),
            debug: DebugConfig::development(),
            runtime: RuntimeConfig::development(),
            output: OutputConfig::development(),
        }
    }

    /// Create a release configuration
    pub fn release() -> Self {
        Self {
            project: ProjectConfig::default(),
            compilation: CompilationConfig::release(),
            optimization: crate::optimization::OptimizationConfig::release_config(),
            debug: DebugConfig::release(),
            runtime: RuntimeConfig::release(),
            output: OutputConfig::release(),
        }
    }

    /// Load configuration from file
    pub fn load_from_file(path: &std::path::Path) -> crate::error::Result<()> {
        // Implementation would read from TOML/JSON file
        // For now, return default
        Ok(Self::new())
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::error::Result<()> {
        // Implementation would write to TOML/JSON file
        Ok(())
    }
}

impl Default for CursedConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Project configuration settings
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Project root directory
    pub root_dir: PathBuf,
    /// Source directories
    pub source_dirs: Vec<PathBuf>,
    /// Include directories
    pub include_dirs: Vec<PathBuf>,
    /// Library directories
    pub library_dirs: Vec<PathBuf>,
    /// Linked libraries
    pub libraries: Vec<String>,
    /// Build dependencies
    pub dependencies: HashMap<String, String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "cursed_project".to_string(),
            version: "0.1.0".to_string(),
            root_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            source_dirs: vec![PathBuf::from("src")],
            include_dirs: Vec::new(),
            library_dirs: Vec::new(),
            libraries: Vec::new(),
            dependencies: HashMap::new(),
        }
    }
}

/// Compilation configuration settings
#[derive(Debug, Clone)]
pub struct CompilationConfig {
    /// Target architecture
    pub target_arch: String,
    /// Target operating system
    pub target_os: String,
    /// Compilation timeout
    pub timeout: Duration,
    /// Maximum memory usage during compilation
    pub max_memory: Option<usize>,
    /// Number of parallel compilation jobs
    pub parallel_jobs: Option<usize>,
    /// Enable incremental compilation
    pub incremental: bool,
    /// Cache directory
    pub cache_dir: Option<PathBuf>,
    /// Temporary directory
    pub temp_dir: Option<PathBuf>,
    /// Additional compiler flags
    pub compiler_flags: Vec<String>,
    /// Linker flags
    pub linker_flags: Vec<String>,
}

impl CompilationConfig {
    /// Create development compilation configuration
    pub fn development() -> Self {
        Self {
            target_arch: "x86_64".to_string(),
            target_os: std::env::consts::OS.to_string(),
            timeout: Duration::from_secs(300), // 5 minutes
            max_memory: None,
            parallel_jobs: Some(4),
            incremental: true,
            cache_dir: Some(PathBuf::from("target/cache")),
            temp_dir: Some(PathBuf::from("target/tmp")),
            compiler_flags: vec!["-g".to_string(), "-O0".to_string()],
            linker_flags: Vec::new(),
        }
    }

    /// Create release compilation configuration
    pub fn release() -> Self {
        Self {
            target_arch: "x86_64".to_string(),
            target_os: std::env::consts::OS.to_string(),
            timeout: Duration::from_secs(1800), // 30 minutes
            max_memory: None,
            parallel_jobs: Some(8),
            incremental: false,
            cache_dir: Some(PathBuf::from("target/cache")),
            temp_dir: Some(PathBuf::from("target/tmp")),
            compiler_flags: vec!["-O3".to_string(), "-DNDEBUG".to_string()],
            linker_flags: vec!["-s".to_string()],
        }
    }
}

impl Default for CompilationConfig {
    fn default() -> Self {
        Self::development()
    }
}

/// Debug configuration settings
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Enable debug symbols
    pub debug_symbols: bool,
    /// Debug symbol format
    pub debug_format: DebugFormat,
    /// Enable source maps
    pub source_maps: bool,
    /// Enable runtime debugging
    pub runtime_debugging: bool,
    /// Enable profiling
    pub profiling: bool,
    /// Enable tracing
    pub tracing: bool,
    /// Tracing level
    pub tracing_level: TracingLevel,
    /// Debug output directory
    pub debug_output_dir: Option<PathBuf>,
}

impl DebugConfig {
    /// Create development debug configuration
    pub fn development() -> Self {
        Self {
            debug_symbols: true,
            debug_format: DebugFormat::Dwarf,
            source_maps: true,
            runtime_debugging: true,
            profiling: false,
            tracing: true,
            tracing_level: TracingLevel::Debug,
            debug_output_dir: Some(PathBuf::from("target/debug_info")),
        }
    }

    /// Create release debug configuration
    pub fn release() -> Self {
        Self {
            debug_symbols: false,
            debug_format: DebugFormat::None,
            source_maps: false,
            runtime_debugging: false,
            profiling: false,
            tracing: false,
            tracing_level: TracingLevel::CursedError,
            debug_output_dir: None,
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self::development()
    }
}

/// Debug symbol format
#[derive(Debug, Clone)]
pub enum DebugFormat {
    None,
    Dwarf,
    Pdb,
    Stabs,
}

/// Tracing level
#[derive(Debug, Clone)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    CursedError,
    Off,
}

/// Runtime configuration settings
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Enable garbage collection
    pub garbage_collection: bool,
    /// GC configuration
    pub gc_config: GcConfig,
    /// Stack size for goroutines
    pub goroutine_stack_size: usize,
    /// Maximum number of goroutines
    pub max_goroutines: Option<usize>,
    /// Channel buffer size
    pub default_channel_buffer_size: usize,
    /// Panic handling configuration
    pub panic_config: PanicConfig,
    /// CursedError handling configuration
    pub error_config: ErrorConfig,
}

impl RuntimeConfig {
    /// Create development runtime configuration
    pub fn development() -> Self {
        Self {
            garbage_collection: true,
            gc_config: GcConfig::development(),
            goroutine_stack_size: 64 * 1024, // 64KB
            max_goroutines: Some(10000),
            default_channel_buffer_size: 100,
            panic_config: PanicConfig::development(),
            error_config: ErrorConfig::development(),
        }
    }

    /// Create release runtime configuration
    pub fn release() -> Self {
        Self {
            garbage_collection: true,
            gc_config: GcConfig::release(),
            goroutine_stack_size: 32 * 1024, // 32KB
            max_goroutines: Some(100000),
            default_channel_buffer_size: 1000,
            panic_config: PanicConfig::release(),
            error_config: ErrorConfig::release(),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self::development()
    }
}

/// Garbage collection configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// GC algorithm to use
    pub algorithm: GcAlgorithm,
    /// Heap size limit
    pub max_heap_size: Option<usize>,
    /// GC trigger threshold
    pub gc_trigger_threshold: f64,
    /// Enable generational GC
    pub generational: bool,
    /// Enable concurrent GC
    pub concurrent: bool,
}

impl GcConfig {
    pub fn development() -> Self {
        Self {
            algorithm: GcAlgorithm::MarkAndSweep,
            max_heap_size: Some(1024 * 1024 * 1024), // 1GB
            gc_trigger_threshold: 0.8,
            generational: false,
            concurrent: false,
        }
    }

    pub fn release() -> Self {
        Self {
            algorithm: GcAlgorithm::Generational,
            max_heap_size: Some(4 * 1024 * 1024 * 1024), // 4GB
            gc_trigger_threshold: 0.9,
            generational: true,
            concurrent: true,
        }
    }
}

/// Garbage collection algorithm
#[derive(Debug, Clone)]
pub enum GcAlgorithm {
    None,
    MarkAndSweep,
    Copying,
    Generational,
    Incremental,
}

/// Panic handling configuration
#[derive(Debug, Clone)]
pub struct PanicConfig {
    /// Enable panic recovery
    pub recovery_enabled: bool,
    /// Maximum panic stack depth
    pub max_stack_depth: usize,
    /// Panic output format
    pub output_format: PanicOutputFormat,
    /// Enable panic hooks
    pub hooks_enabled: bool,
}

impl PanicConfig {
    pub fn development() -> Self {
        Self {
            recovery_enabled: true,
            max_stack_depth: 100,
            output_format: PanicOutputFormat::Detailed,
            hooks_enabled: true,
        }
    }

    pub fn release() -> Self {
        Self {
            recovery_enabled: false,
            max_stack_depth: 50,
            output_format: PanicOutputFormat::Minimal,
            hooks_enabled: false,
        }
    }
}

/// Panic output format
#[derive(Debug, Clone)]
pub enum PanicOutputFormat {
    Minimal,
    Standard,
    Detailed,
    Json,
}

/// CursedError handling configuration
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    /// Enable error propagation
    pub propagation_enabled: bool,
    /// Maximum error chain depth
    pub max_chain_depth: usize,
    /// Enable error context
    pub context_enabled: bool,
    /// CursedError output format
    pub output_format: ErrorOutputFormat,
}

impl ErrorConfig {
    pub fn development() -> Self {
        Self {
            propagation_enabled: true,
            max_chain_depth: 50,
            context_enabled: true,
            output_format: ErrorOutputFormat::Detailed,
        }
    }

    pub fn release() -> Self {
        Self {
            propagation_enabled: true,
            max_chain_depth: 20,
            context_enabled: false,
            output_format: ErrorOutputFormat::Standard,
        }
    }
}

/// CursedError output format
#[derive(Debug, Clone)]
pub enum ErrorOutputFormat {
    Minimal,
    Standard,
    Detailed,
    Json,
}

/// Output configuration settings
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Output directory
    pub output_dir: PathBuf,
    /// Output format
    pub format: OutputFormat,
    /// Binary name
    pub binary_name: Option<String>,
    /// Enable optimization for size
    pub optimize_for_size: bool,
    /// Enable link-time optimization
    pub link_time_optimization: bool,
    /// Strip symbols
    pub strip_symbols: bool,
}

impl OutputConfig {
    pub fn development() -> Self {
        Self {
            output_dir: PathBuf::from("target/debug"),
            format: OutputFormat::Executable,
            binary_name: None,
            optimize_for_size: false,
            link_time_optimization: false,
            strip_symbols: false,
        }
    }

    pub fn release() -> Self {
        Self {
            output_dir: PathBuf::from("target/release"),
            format: OutputFormat::Executable,
            binary_name: None,
            optimize_for_size: true,
            link_time_optimization: true,
            strip_symbols: true,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self::development()
    }
}

/// Output format
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Executable,
    Library,
    SharedLibrary,
    StaticLibrary,
    Object,
    LLVMIR,
    Assembly,
}

/// Security context for cryptographic operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Cryptographic parameters
    pub crypto_params: CryptoParameters,
    /// Security level
    pub security_level: SecurityLevel,
    /// Enable secure allocations
    pub secure_allocations: bool,
    /// Enable secure erasure
    pub secure_erasure: bool,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            crypto_params: CryptoParameters::default(),
            security_level: SecurityLevel::Standard,
            secure_allocations: false,
            secure_erasure: false,
        }
    }

    pub fn high_security() -> Self {
        Self {
            crypto_params: CryptoParameters::high_security(),
            security_level: SecurityLevel::High,
            secure_allocations: true,
            secure_erasure: true,
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Cryptographic parameters
#[derive(Debug, Clone)]
pub struct CryptoParameters {
    /// Key size for symmetric encryption
    pub symmetric_key_size: usize,
    /// Key size for asymmetric encryption
    pub asymmetric_key_size: usize,
    /// Hash algorithm
    pub hash_algorithm: HashAlgorithm,
    /// Encryption algorithm
    pub encryption_algorithm: EncryptionAlgorithm,
}

impl CryptoParameters {
    pub fn default() -> Self {
        Self {
            symmetric_key_size: 256,
            asymmetric_key_size: 2048,
            hash_algorithm: HashAlgorithm::Sha256,
            encryption_algorithm: EncryptionAlgorithm::Aes256,
        }
    }

    pub fn high_security() -> Self {
        Self {
            symmetric_key_size: 256,
            asymmetric_key_size: 4096,
            hash_algorithm: HashAlgorithm::Sha512,
            encryption_algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
        }
    }
}

/// Security level
#[derive(Debug, Clone)]
pub enum SecurityLevel {
    Low,
    Standard,
    High,
    Maximum,
}

/// Hash algorithm
#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
    Argon2,
}

/// Encryption algorithm
#[derive(Debug, Clone)]
pub enum EncryptionAlgorithm {
    Aes256,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
    /// Enable recovery from parse errors
    pub error_recovery: bool,
    /// Enable syntax extensions
    pub syntax_extensions: bool,
    /// Strict mode parsing
    pub strict_mode: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_recursion_depth: 1000,
            error_recovery: true,
            syntax_extensions: true,
            strict_mode: false,
        }
    }
}

/// Cryptographic parameters configuration
impl Default for CryptoParameters {
    fn default() -> Self {
        Self {
            hash_algorithm: "SHA-256".to_string(),
            kdf_iterations: 100_000,
            salt_length: 32,
            key_length: 256,
            use_hardware_acceleration: true,
            secure_rng: true,
        }
    }
}

/// Security context configuration
impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            security_level: 3,
            require_secure_env: false,
            use_tee: false,
            memory_protection: true,
            audit_logging: true,
            crypto: CryptoParameters::default(),
        }
    }
}

/// Module parser configuration
#[derive(Debug, Clone)]
pub struct ModParser {
    /// Parser configuration
    pub config: ParserConfig,
    /// Module search paths
    pub search_paths: Vec<PathBuf>,
    /// Cache parsed modules
    pub cache_modules: bool,
    /// Module cache directory
    pub cache_dir: Option<PathBuf>,
}

impl ModParser {
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
            search_paths: vec![PathBuf::from(".")],
            cache_modules: true,
            cache_dir: Some(PathBuf::from("target/module_cache")),
        }
    }

    pub fn with_config(config: ParserConfig) -> Self {
        Self {
            config,
            search_paths: vec![PathBuf::from(".")],
            cache_modules: true,
            cache_dir: Some(PathBuf::from("target/module_cache")),
        }
    }
}

impl Default for ModParser {
    fn default() -> Self {
        Self::new()
    }
}

