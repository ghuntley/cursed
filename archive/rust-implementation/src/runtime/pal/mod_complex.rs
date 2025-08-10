//! Platform Abstraction Layer (PAL) for CURSED Runtime
//! 
//! This module provides platform-specific abstractions for:
//! - ARM64 (aarch64) - macOS and Linux
//! - x86_64 - macOS, Linux, and Windows  
//! - WebAssembly (WASM) - Browser and standalone runtimes
//!
//! The PAL handles:
//! - Memory management and allocation
//! - System calls and OS interfaces
//! - Thread synchronization primitives
//! - Platform-specific optimizations
//! - Architecture-specific register handling

pub mod arm64;
pub mod x86_64; 
pub mod wasm;
pub mod common;

use crate::runtime::memory::MemoryManager;
use crate::runtime::goroutine::Scheduler;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use common::{RuntimeStats, PerformanceMonitor, HardwareCapabilityDetector, MemoryManagementUtilities, SchedulerUtilities, TaskType, CoreType};

/// Enhanced platform abstraction trait for runtime operations
pub trait PlatformAbstraction: Send + Sync + HardwareCapabilityDetector + MemoryManagementUtilities + SchedulerUtilities {
    /// Initialize platform-specific runtime components
    fn initialize(&self) -> Result<(), PlatformError>;
    
    /// Get platform-specific memory manager
    fn memory_manager(&self) -> Arc<dyn MemoryManager>;
    
    /// Get platform-specific scheduler
    fn scheduler(&self) -> Arc<dyn Scheduler>;
    
    /// Platform-specific stack size for goroutines
    fn default_stack_size(&self) -> usize;
    
    /// Platform-specific memory page size
    fn page_size(&self) -> usize;
    
    /// Number of hardware threads available
    fn hardware_concurrency(&self) -> usize;
    
    /// Platform name for debugging
    fn platform_name(&self) -> &'static str;
    
    /// Architecture name
    fn architecture(&self) -> Architecture;
    
    /// Operating system
    fn operating_system(&self) -> OperatingSystem;

    // Enhanced platform capabilities
    
    /// Get comprehensive platform capabilities
    fn get_platform_capabilities(&self) -> PlatformCapabilities;
    
    /// Get runtime performance monitor
    fn performance_monitor(&self) -> Arc<PerformanceMonitor>;
    
    /// Get current runtime statistics
    fn get_runtime_stats(&self) -> RuntimeStats;
    
    /// Get memory usage statistics
    fn get_memory_stats(&self) -> MemoryStats;
    
    /// Get scheduler performance metrics
    fn get_scheduler_stats(&self) -> SchedulerStats;
    
    /// Get hardware utilization metrics
    fn get_hardware_utilization(&self) -> HardwareUtilization;
    
    /// Enable/disable platform features
    fn configure_feature(&self, feature: &str, enabled: bool) -> Result<(), PlatformError>;
    
    /// Get optimal configuration for current workload
    fn get_optimal_configuration(&self, workload_type: WorkloadType) -> PlatformConfiguration;
    
    /// Flush instruction cache (for JIT compilation)
    fn flush_instruction_cache(&self, addr: *mut u8, len: usize) -> Result<(), PlatformError>;
    
    /// Platform-specific memory barrier
    fn memory_barrier(&self);
    
    /// Platform-specific thread yield
    fn thread_yield(&self);
}

/// Enhanced platform capabilities structure
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    pub vector_instructions: VectorCapabilities,
    pub memory_features: MemoryCapabilities,
    pub threading_features: ThreadingCapabilities,
    pub crypto_acceleration: CryptoCapabilities,
    pub io_capabilities: IOCapabilities,
    pub debug_features: DebugCapabilities,
}

#[derive(Debug, Clone)]
pub struct VectorCapabilities {
    pub simd_width: usize,
    pub max_vector_size: usize,
    pub supported_types: Vec<String>,
    pub has_gather_scatter: bool,
    pub has_predication: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryCapabilities {
    pub max_addressable_memory: u64,
    pub page_sizes: Vec<usize>,
    pub numa_nodes: usize,
    pub cache_levels: usize,
    pub prefetcher_streams: usize,
    pub memory_encryption: bool,
}

#[derive(Debug, Clone)]
pub struct ThreadingCapabilities {
    pub max_threads: usize,
    pub thread_local_storage_slots: usize,
    pub atomic_operations: Vec<String>,
    pub lock_free_structures: Vec<String>,
    pub core_types: Vec<CoreType>,
}

#[derive(Debug, Clone)]
pub struct CryptoCapabilities {
    pub hash_algorithms: Vec<String>,
    pub encryption_algorithms: Vec<String>,
    pub key_exchange_algorithms: Vec<String>,
    pub random_number_generators: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IOCapabilities {
    pub async_io: bool,
    pub vectored_io: bool,
    pub memory_mapped_io: bool,
    pub direct_io: bool,
    pub zero_copy_networking: bool,
}

#[derive(Debug, Clone)]
pub struct DebugCapabilities {
    pub hardware_breakpoints: usize,
    pub watchpoints: usize,
    pub performance_counters: usize,
    pub instruction_tracing: bool,
    pub branch_tracing: bool,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: u64,
    pub currently_used: u64,
    pub peak_usage: u64,
    pub fragmentation_ratio: f32,
    pub large_page_usage: u64,
    pub cache_hit_ratio: f32,
    pub gc_overhead: f32,
}

/// Scheduler performance statistics
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub goroutines_created: u64,
    pub goroutines_destroyed: u64,
    pub context_switches: u64,
    pub work_stealing_attempts: u64,
    pub work_stealing_successes: u64,
    pub load_balancing_events: u64,
    pub avg_queue_depth: f32,
    pub core_utilization: Vec<f32>,
}

/// Hardware utilization metrics
#[derive(Debug, Clone)]
pub struct HardwareUtilization {
    pub cpu_usage_per_core: Vec<f32>,
    pub memory_bandwidth_usage: f32,
    pub cache_miss_rates: Vec<f32>,
    pub instruction_throughput: f32,
    pub thermal_state: ThermalState,
    pub power_consumption: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum ThermalState {
    Normal,
    Warm,
    Hot,
    Critical,
}

/// Workload type for optimization
#[derive(Debug, Clone, Copy)]
pub enum WorkloadType {
    Compute,
    Memory,
    IO,
    Network,
    Mixed,
    Latency,
    Throughput,
}

/// Platform-specific configuration
#[derive(Debug, Clone)]
pub struct PlatformConfiguration {
    pub memory_allocator: AllocatorType,
    pub scheduler_policy: SchedulerPolicy,
    pub gc_configuration: GcConfiguration,
    pub optimization_flags: Vec<String>,
    pub performance_hints: Vec<PerformanceHint>,
}

#[derive(Debug, Clone, Copy)]
pub enum AllocatorType {
    Default,
    NUMA,
    LowLatency,
    HighThroughput,
    LowFragmentation,
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulerPolicy {
    RoundRobin,
    WorkStealing,
    NUMA,
    PowerAware,
    Deadline,
}

#[derive(Debug, Clone)]
pub struct GcConfiguration {
    pub algorithm: GcAlgorithm,
    pub heap_size_hint: Option<usize>,
    pub collection_frequency: GcFrequency,
    pub concurrent_marking: bool,
    pub incremental_collection: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum GcAlgorithm {
    MarkAndSweep,
    Generational,
    Concurrent,
    Incremental,
    RegionBased,
}

#[derive(Debug, Clone, Copy)]
pub enum GcFrequency {
    Aggressive,
    Balanced,
    Conservative,
    Manual,
}

#[derive(Debug, Clone)]
pub struct PerformanceHint {
    pub category: String,
    pub suggestion: String,
    pub impact: Impact,
}

#[derive(Debug, Clone, Copy)]
pub enum Impact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm64,
    X86_64,
    Wasm32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    MacOS,
    Linux,
    Windows,
    Browser,
    WasmRuntime,
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("Platform initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Unsupported platform: {arch:?} on {os:?}")]
    UnsupportedPlatform { arch: Architecture, os: OperatingSystem },
    
    #[error("Memory allocation failed: {0}")]
    MemoryAllocationFailed(String),
    
    #[error("System call failed: {0}")]
    SystemCallFailed(String),
}

/// Enhanced factory function to create the appropriate PAL for the current platform
pub fn create_platform_abstraction() -> Result<Arc<dyn PlatformAbstraction>, PlatformError> {
    // Detect platform capabilities first
    let platform_info = common::PlatformDetector::detect();
    
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            let pal = Arc::new(arm64::Arm64MacOSPal::new()?);
            log::info!("Initialized ARM64 macOS PAL with {} cores, {} features", 
                      pal.hardware_concurrency(), 
                      pal.get_platform_capabilities().vector_instructions.supported_types.len());
            Ok(pal)
        } else if #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
            let pal = Arc::new(arm64::Arm64LinuxPal::new()?);
            log::info!("Initialized ARM64 Linux PAL with {} cores, NUMA nodes: {}", 
                      pal.hardware_concurrency(),
                      pal.get_platform_capabilities().memory_features.numa_nodes);
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "macos"))] {
            let pal = Arc::new(x86_64::X86_64MacOSPal::new()?);
            log::info!("Initialized x86_64 macOS PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
            let pal = Arc::new(x86_64::X86_64LinuxPal::new()?);
            log::info!("Initialized x86_64 Linux PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(all(target_arch = "x86_64", target_os = "windows"))] {
            let pal = Arc::new(x86_64::X86_64WindowsPal::new()?);
            log::info!("Initialized x86_64 Windows PAL with {} cores", pal.hardware_concurrency());
            Ok(pal)
        } else if #[cfg(target_arch = "wasm32")] {
            let pal = Arc::new(wasm::WasmPal::new()?);
            log::info!("Initialized WebAssembly PAL with {} memory pages", 
                      pal.get_platform_capabilities().memory_features.page_sizes.len());
            Ok(pal)
        } else {
            Err(PlatformError::UnsupportedPlatform {
                arch: platform_info.architecture,
                os: platform_info.operating_system,
            })
        }
    }
}

/// Initialize the platform abstraction layer with comprehensive setup
pub fn initialize_pal() -> Result<Arc<dyn PlatformAbstraction>, PlatformError> {
    let pal = create_platform_abstraction()?;
    
    // Initialize platform-specific components
    pal.initialize()?;
    
    // Log platform information
    let capabilities = pal.get_platform_capabilities();
    log::info!("Platform initialized: {}", pal.platform_name());
    log::debug!("Vector capabilities: {} instructions supported", 
               capabilities.vector_instructions.supported_types.len());
    log::debug!("Memory features: {} page sizes, {} NUMA nodes", 
               capabilities.memory_features.page_sizes.len(),
               capabilities.memory_features.numa_nodes);
    log::debug!("Threading: {} max threads, {} core types", 
               capabilities.threading_features.max_threads,
               capabilities.threading_features.core_types.len());
    
    Ok(pal)
}

/// Get platform version information
pub fn get_platform_version_info(verbose: bool) -> String {
    common::VersionReporter::generate_version_report(verbose)
}

/// Get current platform statistics
pub fn get_platform_stats() -> Result<RuntimeStats, PlatformError> {
    let pal = create_platform_abstraction()?;
    Ok(pal.get_runtime_stats())
}

fn detect_architecture() -> Architecture {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            Architecture::Arm64
        } else if #[cfg(target_arch = "x86_64")] {
            Architecture::X86_64
        } else if #[cfg(target_arch = "wasm32")] {
            Architecture::Wasm32
        } else {
            // Fallback detection
            Architecture::X86_64
        }
    }
}

fn detect_operating_system() -> OperatingSystem {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            OperatingSystem::MacOS
        } else if #[cfg(target_os = "linux")] {
            OperatingSystem::Linux
        } else if #[cfg(target_os = "windows")] {
            OperatingSystem::Windows
        } else if #[cfg(target_arch = "wasm32")] {
            // Distinguish between browser and standalone WASM runtime
            if cfg!(target_feature = "atomics") {
                OperatingSystem::WasmRuntime
            } else {
                OperatingSystem::Browser
            }
        } else {
            OperatingSystem::Linux
        }
    }
}
