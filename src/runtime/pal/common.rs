//! Common utilities and abstractions shared across all platform abstraction layers

use super::{PlatformError, Architecture, OperatingSystem};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
// Conditional sysinfo usage for cross-compilation compatibility
#[cfg(not(target_arch = "wasm32"))]
use sysinfo::System;

/// Common platform detection utilities
pub struct PlatformDetector;

impl PlatformDetector {
    /// Detect the current platform and return appropriate configuration
    pub fn detect() -> PlatformInfo {
        PlatformInfo {
            architecture: detect_architecture(),
            operating_system: detect_operating_system(),
            features: detect_platform_features(),
        }
    }
    
    /// Get optimal configuration for the detected platform
    pub fn get_optimal_config() -> PlatformConfig {
        let info = Self::detect();
        PlatformConfig::for_platform(info)
    }
}

/// Platform information structure
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub architecture: Architecture,
    pub operating_system: OperatingSystem,
    pub features: PlatformFeatures,
}

/// Platform-specific feature detection
#[derive(Debug, Clone)]
pub struct PlatformFeatures {
    pub vector_instructions: VectorInstructions,
    pub memory_features: MemoryFeatures,
    pub threading_features: ThreadingFeatures,
    pub crypto_acceleration: CryptoAcceleration,
}

#[derive(Debug, Clone)]
pub struct VectorInstructions {
    pub sse: bool,
    pub sse2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub avx512: bool,
    pub neon: bool,
    pub sve: bool,
    pub wasm_simd: bool,
}

#[derive(Debug, Clone)]
pub struct MemoryFeatures {
    pub large_pages: bool,
    pub memory_tagging: bool,
    pub memory_protection_keys: bool,
    pub page_size: usize,
    pub cache_line_size: usize,
}

#[derive(Debug, Clone)]
pub struct ThreadingFeatures {
    pub hyper_threading: bool,
    pub big_little_cores: bool,
    pub numa_topology: bool,
    pub hardware_concurrency: usize,
}

#[derive(Debug, Clone)]
pub struct CryptoAcceleration {
    pub aes_ni: bool,
    pub sha_extensions: bool,
    pub random_number_generator: bool,
}

/// Platform-specific configuration
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub default_stack_size: usize,
    pub max_goroutines: usize,
    pub gc_trigger_ratio: f32,
    pub memory_alignment: usize,
    pub scheduler_quantum: std::time::Duration,
}

impl PlatformConfig {
    pub fn for_platform(info: PlatformInfo) -> Self {
        match (info.architecture, info.operating_system) {
            (Architecture::Arm64, OperatingSystem::MacOS) => Self::arm64_macos(),
            (Architecture::Arm64, OperatingSystem::Linux) => Self::arm64_linux(),
            (Architecture::X86_64, OperatingSystem::MacOS) => Self::x86_64_macos(),
            (Architecture::X86_64, OperatingSystem::Linux) => Self::x86_64_linux(),
            (Architecture::X86_64, OperatingSystem::Windows) => Self::x86_64_windows(),
            (Architecture::Wasm32, _) => Self::wasm(),
            _ => Self::default_config(),
        }
    }
    
    fn arm64_macos() -> Self {
        Self {
            default_stack_size: 1024 * 1024,      // 1MB - ARM64 has large address space
            max_goroutines: 1_000_000,             // High limit on Apple Silicon
            gc_trigger_ratio: 0.8,                 // Aggressive GC for memory efficiency
            memory_alignment: 16,                   // ARM64 alignment requirement
            scheduler_quantum: std::time::Duration::from_millis(10),
        }
    }
    
    fn arm64_linux() -> Self {
        Self {
            default_stack_size: 512 * 1024,       // 512KB - More conservative
            max_goroutines: 500_000,
            gc_trigger_ratio: 0.75,
            memory_alignment: 16,
            scheduler_quantum: std::time::Duration::from_millis(10),
        }
    }
    
    fn x86_64_macos() -> Self {
        Self {
            default_stack_size: 512 * 1024,       // 512KB
            max_goroutines: 1_000_000,
            gc_trigger_ratio: 0.7,
            memory_alignment: 16,
            scheduler_quantum: std::time::Duration::from_millis(15),
        }
    }
    
    fn x86_64_linux() -> Self {
        Self {
            default_stack_size: 8 * 1024 * 1024,  // 8MB - Linux default
            max_goroutines: 1_000_000,
            gc_trigger_ratio: 0.7,
            memory_alignment: 16,
            scheduler_quantum: std::time::Duration::from_millis(15),
        }
    }
    
    fn x86_64_windows() -> Self {
        Self {
            default_stack_size: 1024 * 1024,      // 1MB - Windows default
            max_goroutines: 500_000,               // Windows has thread limitations
            gc_trigger_ratio: 0.65,
            memory_alignment: 16,
            scheduler_quantum: std::time::Duration::from_millis(20),
        }
    }
    
    fn wasm() -> Self {
        Self {
            default_stack_size: 64 * 1024,        // 64KB - WASM stack constraints
            max_goroutines: 10_000,                // Limited by memory constraints
            gc_trigger_ratio: 0.9,                 // Conservative GC in WASM
            memory_alignment: 8,                    // WASM alignment
            scheduler_quantum: std::time::Duration::from_millis(5), // Cooperative scheduling
        }
    }
    
    fn default_config() -> Self {
        Self {
            default_stack_size: 256 * 1024,       // 256KB conservative default
            max_goroutines: 100_000,
            gc_trigger_ratio: 0.75,
            memory_alignment: 8,
            scheduler_quantum: std::time::Duration::from_millis(20),
        }
    }
}

/// Cross-platform atomic operations
pub struct AtomicOperations;

impl AtomicOperations {
    /// Platform-optimized memory fence
    pub fn memory_fence() {
        // For cross-compilation, use standard atomic fence
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Platform-optimized pause/yield instruction
    pub fn pause() {
        Self::platform_yield();
    }
    
    /// Cross-platform yield implementation
    fn platform_yield() {
        std::thread::yield_now();
    }
    
    /// Cross-platform memory barrier
    pub fn memory_barrier() {
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    }
}

/// Cross-platform cache management
pub struct CacheManager;

impl CacheManager {
    /// Flush instruction cache (needed after JIT compilation)
    pub fn flush_instruction_cache(addr: *mut u8, len: usize) -> Result<(), PlatformError> {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "aarch64")] {
                Self::flush_icache_arm64(addr, len)
            } else if #[cfg(target_arch = "x86_64")] {
                Self::flush_icache_x86_64(addr, len)
            } else if #[cfg(target_arch = "wasm32")] {
                // WASM doesn't need explicit cache flushing
                Ok(())
            } else {
                Err(PlatformError::UnsupportedPlatform {
                    arch: detect_architecture(),
                    os: detect_operating_system(),
                })
            }
        }
    }
    
    #[cfg(all(target_arch = "aarch64", not(target_os = "linux")))]
    fn flush_icache_arm64(addr: *mut u8, len: usize) -> Result<(), PlatformError> {
        // ARM64 requires explicit cache maintenance for executable code
        #[cfg(feature = "inline_asm")]
        unsafe {
            let end_addr = addr.add(len);
            let mut current = addr;
            
            // Clean data cache and invalidate instruction cache
            while current < end_addr {
                std::arch::asm!(
                    "dc cvau, {addr}",  // Clean data cache to PoU
                    "ic ivau, {addr}",  // Invalidate instruction cache
                    addr = in(reg) current,
                );
                current = current.add(64); // Cache line size
            }
            
            // Ensure instruction fetch sees the new instructions
            std::arch::asm!("dsb ish"); // Data Synchronization Barrier
            std::arch::asm!("isb");     // Instruction Synchronization Barrier
        }
        
        #[cfg(not(feature = "inline_asm"))]
        {
            // Fallback: Use platform-independent cache flush
            std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    fn flush_icache_arm64(_addr: *mut u8, _len: usize) -> Result<(), PlatformError> {
        // For cross-compilation, skip inline assembly
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
    
    #[cfg(target_arch = "x86_64")]
    fn flush_icache_x86_64(_addr: *mut u8, _len: usize) -> Result<(), PlatformError> {
        // x86_64 has coherent instruction cache, no explicit flushing needed
        // Use platform abstraction for memory barriers
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}

/// Cross-platform SIMD capability detection
pub fn detect_simd_capability(feature: &str) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            match feature {
                "sse" => is_x86_feature_detected!("sse"),
                "sse2" => is_x86_feature_detected!("sse2"),
                "avx" => is_x86_feature_detected!("avx"),
                "avx2" => is_x86_feature_detected!("avx2"),
                "avx512f" => is_x86_feature_detected!("avx512f"),
                _ => false,
            }
        } else if #[cfg(target_arch = "aarch64")] {
            match feature {
                "neon" => cfg!(target_feature = "neon"),
                "sve" => cfg!(target_feature = "sve"),
                _ => false,
            }
        } else if #[cfg(target_arch = "wasm32")] {
            match feature {
                "simd128" => cfg!(target_feature = "simd128"),
                _ => false,
            }
        } else {
            false
        }
    }
}

/// Cross-platform crypto capability detection
pub fn detect_crypto_capability(feature: &str) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            match feature {
                "aes" => is_x86_feature_detected!("aes"),
                "sha" => is_x86_feature_detected!("sha"),
                "rdrand" => is_x86_feature_detected!("rdrand"),
                _ => false,
            }
        } else if #[cfg(target_arch = "aarch64")] {
            match feature {
                "aes" => cfg!(target_feature = "aes"),
                "sha" => cfg!(target_feature = "sha2"),
                _ => false,
            }
        } else {
            false
        }
    }
}

/// Cross-platform hardware capability detection
pub fn detect_hardware_capability(feature: &str) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            match feature {
                "pku" => false, // PKU not supported in cross-compilation
                _ => false,
            }
        } else if #[cfg(target_arch = "aarch64")] {
            match feature {
                "mte" => false, // Would need runtime detection
                _ => false,
            }
        } else {
            false
        }
    }
}

/// Platform capability registry
pub struct CapabilityRegistry {
    capabilities: Arc<Mutex<HashMap<String, bool>>>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        let mut capabilities = HashMap::new();
        
        // Detect and register platform capabilities
        Self::register_vector_capabilities(&mut capabilities);
        Self::register_memory_capabilities(&mut capabilities);
        Self::register_crypto_capabilities(&mut capabilities);
        
        Self {
            capabilities: Arc::new(Mutex::new(capabilities)),
        }
    }
    
    pub fn has_capability(&self, name: &str) -> bool {
        self.capabilities
            .lock()
            .unwrap()
            .get(name)
            .copied()
            .unwrap_or(false)
    }
    
    fn register_vector_capabilities(caps: &mut HashMap<String, bool>) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                caps.insert("sse".to_string(), detect_simd_capability("sse"));
                caps.insert("sse2".to_string(), detect_simd_capability("sse2"));
                caps.insert("avx".to_string(), detect_simd_capability("avx"));
                caps.insert("avx2".to_string(), detect_simd_capability("avx2"));
                caps.insert("avx512f".to_string(), detect_simd_capability("avx512f"));
            } else if #[cfg(target_arch = "aarch64")] {
                caps.insert("neon".to_string(), cfg!(target_feature = "neon"));
                caps.insert("sve".to_string(), cfg!(target_feature = "sve"));
            } else if #[cfg(target_arch = "wasm32")] {
                caps.insert("simd128".to_string(), cfg!(target_feature = "simd128"));
            }
        }
    }
    
    fn register_memory_capabilities(caps: &mut HashMap<String, bool>) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                caps.insert("large_pages".to_string(), true);
                caps.insert("memory_protection_keys".to_string(), 
                detect_hardware_capability("pku"));
            } else if #[cfg(target_arch = "aarch64")] {
                caps.insert("large_pages".to_string(), true);
                caps.insert("memory_tagging".to_string(), false); // Detect MTE support
            } else if #[cfg(target_arch = "wasm32")] {
                caps.insert("bulk_memory".to_string(), cfg!(target_feature = "bulk-memory"));
                caps.insert("atomics".to_string(), cfg!(target_feature = "atomics"));
            }
        }
    }
    
    fn register_crypto_capabilities(caps: &mut HashMap<String, bool>) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                caps.insert("aes_ni".to_string(), detect_crypto_capability("aes"));
                caps.insert("sha_extensions".to_string(), detect_crypto_capability("sha"));
                caps.insert("rdrand".to_string(), detect_crypto_capability("rdrand"));
            } else if #[cfg(target_arch = "aarch64")] {
                caps.insert("aes".to_string(), cfg!(target_feature = "aes"));
                caps.insert("sha2".to_string(), cfg!(target_feature = "sha2"));
                caps.insert("sha3".to_string(), cfg!(target_feature = "sha3"));
            }
        }
    }
}

/// Common performance monitoring utilities
pub struct PerformanceMonitor {
    start_time: Instant,
    #[cfg(not(target_arch = "wasm32"))]
    system: Arc<Mutex<System>>,
    cpu_usage_history: Arc<RwLock<Vec<f32>>>,
    memory_usage_history: Arc<RwLock<Vec<u64>>>,
    gc_events: Arc<Mutex<Vec<GcEvent>>>,
    scheduler_metrics: Arc<Mutex<SchedulerMetrics>>,
}

#[derive(Debug, Clone)]
pub struct GcEvent {
    pub timestamp: SystemTime,
    pub duration: Duration,
    pub memory_freed: u64,
    pub gc_type: GcType,
}

#[derive(Debug, Clone, Copy)]
pub enum GcType {
    Minor,
    Major,
    Full,
    Concurrent,
}

#[derive(Debug, Default)]
pub struct SchedulerMetrics {
    pub goroutines_spawned: u64,
    pub goroutines_completed: u64,
    pub work_stealing_events: u64,
    pub context_switches: u64,
    pub total_runtime: Duration,
    pub idle_time: Duration,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            #[cfg(not(target_arch = "wasm32"))]
            system: Arc::new(Mutex::new(System::new())),
            cpu_usage_history: Arc::new(RwLock::new(Vec::new())),
            memory_usage_history: Arc::new(RwLock::new(Vec::new())),
            gc_events: Arc::new(Mutex::new(Vec::new())),
            scheduler_metrics: Arc::new(Mutex::new(SchedulerMetrics::default())),
        }
    }

    pub fn refresh_system_stats(&self) {
        // Simplified system monitoring without sysinfo complexity
        if let Ok(mut history) = self.cpu_usage_history.write() {
            history.push(0.0); // Placeholder CPU usage
            if history.len() > 1000 {
                history.drain(0..500);
            }
        }
        
        if let Ok(mut history) = self.memory_usage_history.write() {
            history.push(0); // Placeholder memory usage
            if history.len() > 1000 {
                history.drain(0..500);
            }
        }
    }

    pub fn record_gc_event(&self, duration: Duration, memory_freed: u64, gc_type: GcType) {
        let event = GcEvent {
            timestamp: SystemTime::now(),
            duration,
            memory_freed,
            gc_type,
        };
        
        if let Ok(mut events) = self.gc_events.lock() {
            events.push(event);
            if events.len() > 10000 {
                events.drain(0..5000); // Keep recent 5000 events
            }
        }
    }

    pub fn get_cpu_usage_avg(&self) -> f32 {
        if let Ok(history) = self.cpu_usage_history.read() {
            if history.is_empty() {
                0.0
            } else {
                history.iter().sum::<f32>() / history.len() as f32
            }
        } else {
            0.0
        }
    }

    pub fn get_memory_usage(&self) -> u64 {
        // Return placeholder value for now
        0
    }

    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn get_scheduler_metrics(&self) -> SchedulerMetrics {
        if let Ok(metrics) = self.scheduler_metrics.lock() {
            SchedulerMetrics {
                goroutines_spawned: metrics.goroutines_spawned,
                goroutines_completed: metrics.goroutines_completed,
                work_stealing_events: metrics.work_stealing_events,
                context_switches: metrics.context_switches,
                total_runtime: metrics.total_runtime,
                idle_time: metrics.idle_time,
            }
        } else {
            SchedulerMetrics::default()
        }
    }

    pub fn update_scheduler_metrics<F>(&self, updater: F) 
    where 
        F: FnOnce(&mut SchedulerMetrics)
    {
        if let Ok(mut metrics) = self.scheduler_metrics.lock() {
            updater(&mut *metrics);
        }
    }
}

/// Hardware capability detection utilities
pub trait HardwareCapabilityDetector {
    fn supports_feature(&self, feature: &str) -> bool;
    fn get_cache_line_size(&self) -> usize;
    fn get_page_size(&self) -> usize;
    fn get_numa_node_count(&self) -> usize;
    fn get_cpu_frequency(&self) -> u32;
    fn has_vector_instructions(&self) -> bool;
    fn has_crypto_acceleration(&self) -> bool;
}

/// Common memory management utilities
pub trait MemoryManagementUtilities {
    fn prefault_memory(&self, ptr: *mut u8, size: usize) -> Result<(), PlatformError>;
    fn suggest_memory_layout(&self, allocation_size: usize) -> MemoryLayout;
    fn get_memory_pressure(&self) -> MemoryPressure;
    fn optimize_for_numa(&self, node_preference: Option<usize>) -> Result<(), PlatformError>;
}

#[derive(Debug, Clone)]
pub struct MemoryLayout {
    pub alignment: usize,
    pub use_large_pages: bool,
    pub preferred_numa_node: Option<usize>,
    pub allocation_strategy: AllocationStrategy,
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationStrategy {
    Sequential,
    Interleaved,
    LocalFirst,
    Scattered,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryPressure {
    Low,
    Medium,
    High,
    Critical,
}

/// Common scheduler interface utilities
pub trait SchedulerUtilities {
    fn suggest_worker_count(&self) -> usize;
    fn get_preferred_core_type(&self, task_type: TaskType) -> CoreType;
    fn should_migrate_task(&self, current_load: f32, target_load: f32) -> bool;
    fn get_load_balancing_strategy(&self) -> LoadBalancingStrategy;
}

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Compute,
    IO,
    Memory,
    Network,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum CoreType {
    Performance,
    Efficiency,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastLoaded,
    WorkStealing,
    AffinityAware,
}

/// Platform runtime statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub uptime: Duration,
    pub memory_usage: u64,
    pub memory_peak: u64,
    pub cpu_usage_avg: f32,
    pub gc_collections: u64,
    pub gc_time_total: Duration,
    pub goroutines_active: u64,
    pub goroutines_total: u64,
    pub scheduler_context_switches: u64,
    pub platform_features: Vec<String>,
    pub performance_counters: HashMap<String, u64>,
}

impl RuntimeStats {
    pub fn new() -> Self {
        Self {
            uptime: Duration::new(0, 0),
            memory_usage: 0,
            memory_peak: 0,
            cpu_usage_avg: 0.0,
            gc_collections: 0,
            gc_time_total: Duration::new(0, 0),
            goroutines_active: 0,
            goroutines_total: 0,
            scheduler_context_switches: 0,
            platform_features: Vec::new(),
            performance_counters: HashMap::new(),
        }
    }

    pub fn format_human_readable(&self) -> String {
        format!(
            "Runtime Statistics:\n\
            Uptime: {:?}\n\
            Memory Usage: {} MB (Peak: {} MB)\n\
            CPU Usage: {:.1}%\n\
            GC Collections: {} (Total Time: {:?})\n\
            Goroutines: {} active, {} total\n\
            Context Switches: {}\n\
            Features: {}\n",
            self.uptime,
            self.memory_usage / 1024 / 1024,
            self.memory_peak / 1024 / 1024,
            self.cpu_usage_avg,
            self.gc_collections,
            self.gc_time_total,
            self.goroutines_active,
            self.goroutines_total,
            self.scheduler_context_switches,
            self.platform_features.join(", ")
        )
    }
}

/// Version and feature reporting utilities
pub struct VersionReporter;

impl VersionReporter {
    pub fn get_runtime_version() -> String {
        format!("CURSED Runtime v{}", env!("CARGO_PKG_VERSION"))
    }

    pub fn get_platform_info() -> PlatformInfo {
        PlatformDetector::detect()
    }

    pub fn get_compiler_version() -> String {
        format!("CURSED Compiler v{} (LLVM {})", 
                env!("CARGO_PKG_VERSION"),
                Self::get_llvm_version())
    }

    pub fn get_llvm_version() -> String {
        // Would query LLVM version at runtime
        "18.1".to_string()
    }

    pub fn generate_version_report(verbose: bool) -> String {
        let platform_info = Self::get_platform_info();
        let runtime_version = Self::get_runtime_version();
        let compiler_version = Self::get_compiler_version();

        if verbose {
            format!(
                "{}\n\
                {}\n\
                Platform: {} on {}\n\
                Architecture: {:?}\n\
                OS: {:?}\n\
                Hardware Features:\n\
                  - Vector Instructions: NEON: {}, SSE: {}, AVX: {}, AVX2: {}\n\
                  - Crypto Acceleration: AES: {}, SHA: {}\n\
                  - Memory Features: Large Pages: {}, MTE: {}\n\
                  - Threading: Hardware Concurrency: {}, NUMA: {}\n\
                  - Cache: Line Size: {} bytes, Page Size: {} KB\n\
                Build Configuration:\n\
                  - Target: {}-{}\n\
                  - Debug: {}\n\
                  - Features: {}\n",
                runtime_version,
                compiler_version,
                platform_info.architecture.platform_name(),
                platform_info.operating_system.name(),
                platform_info.architecture,
                platform_info.operating_system,
                platform_info.features.vector_instructions.neon,
                platform_info.features.vector_instructions.sse,
                platform_info.features.vector_instructions.avx,
                platform_info.features.vector_instructions.avx2,
                platform_info.features.crypto_acceleration.aes_ni,
                platform_info.features.crypto_acceleration.sha_extensions,
                platform_info.features.memory_features.large_pages,
                platform_info.features.memory_features.memory_tagging,
                platform_info.features.threading_features.hardware_concurrency,
                platform_info.features.threading_features.numa_topology,
                platform_info.features.memory_features.cache_line_size,
                platform_info.features.memory_features.page_size / 1024,
                cfg!(target_arch = "aarch64").then(|| "aarch64").unwrap_or("x86_64"),
                cfg!(target_os = "macos").then(|| "apple-darwin")
                    .or_else(|| cfg!(target_os = "linux").then(|| "unknown-linux-gnu"))
                    .or_else(|| cfg!(target_os = "windows").then(|| "pc-windows-msvc"))
                    .unwrap_or("unknown"),
                cfg!(debug_assertions),
                Self::get_enabled_features().join(", ")
            )
        } else {
            format!("{} on {} ({})", 
                    runtime_version,
                    platform_info.architecture.platform_name(),
                    platform_info.operating_system.name())
        }
    }

    fn get_enabled_features() -> Vec<String> {
        let mut features = Vec::new();
        
        if cfg!(feature = "concurrent_gc") {
            features.push("concurrent_gc".to_string());
        }
        if cfg!(feature = "enhanced_dynamic_dispatch") {
            features.push("enhanced_dynamic_dispatch".to_string());
        }
        if cfg!(feature = "crypto") {
            features.push("crypto".to_string());
        }
        if cfg!(feature = "async") {
            features.push("async".to_string());
        }
        
        if features.is_empty() {
            features.push("default".to_string());
        }
        
        features
    }
}

/// Extension methods for Architecture and OperatingSystem
impl Architecture {
    pub fn platform_name(&self) -> &'static str {
        match self {
            Architecture::Arm64 => "ARM64 (AArch64)",
            Architecture::X86_64 => "x86_64 (Intel/AMD)",
            Architecture::Wasm32 => "WebAssembly 32-bit",
        }
    }

    pub fn default_page_size(&self) -> usize {
        match self {
            Architecture::Arm64 => {
                if cfg!(target_os = "macos") {
                    16 * 1024 // 16KB on macOS
                } else {
                    4 * 1024  // 4KB on Linux
                }
            },
            Architecture::X86_64 => 4 * 1024, // 4KB
            Architecture::Wasm32 => 64 * 1024, // 64KB
        }
    }

    pub fn default_cache_line_size(&self) -> usize {
        match self {
            Architecture::Arm64 => 64,
            Architecture::X86_64 => 64,
            Architecture::Wasm32 => 64, // Virtual
        }
    }
}

impl OperatingSystem {
    pub fn name(&self) -> &'static str {
        match self {
            OperatingSystem::MacOS => "macOS",
            OperatingSystem::Linux => "Linux",
            OperatingSystem::Windows => "Windows",
            OperatingSystem::Browser => "Browser",
            OperatingSystem::WasmRuntime => "WASM Runtime",
        }
    }

    pub fn supports_large_pages(&self) -> bool {
        match self {
            OperatingSystem::MacOS => true,
            OperatingSystem::Linux => true,
            OperatingSystem::Windows => true,
            OperatingSystem::Browser => false,
            OperatingSystem::WasmRuntime => false,
        }
    }

    pub fn default_stack_size(&self) -> usize {
        match self {
            OperatingSystem::MacOS => 8 * 1024 * 1024,    // 8MB
            OperatingSystem::Linux => 8 * 1024 * 1024,    // 8MB  
            OperatingSystem::Windows => 1024 * 1024,      // 1MB
            OperatingSystem::Browser => 64 * 1024,        // 64KB
            OperatingSystem::WasmRuntime => 256 * 1024,   // 256KB
        }
    }
}

// Helper functions for platform detection
fn detect_architecture() -> Architecture {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            Architecture::Arm64
        } else if #[cfg(target_arch = "x86_64")] {
            Architecture::X86_64
        } else if #[cfg(target_arch = "wasm32")] {
            Architecture::Wasm32
        } else {
            Architecture::X86_64 // Default fallback
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
            OperatingSystem::Browser
        } else {
            OperatingSystem::Linux // Default fallback
        }
    }
}

fn detect_platform_features() -> PlatformFeatures {
    PlatformFeatures {
        vector_instructions: detect_vector_instructions(),
        memory_features: detect_memory_features(),
        threading_features: detect_threading_features(),
        crypto_acceleration: detect_crypto_acceleration(),
    }
}

fn detect_vector_instructions() -> VectorInstructions {
    VectorInstructions {
        sse: detect_sse_support(),
        sse2: detect_sse2_support(),
        avx: detect_avx_support(),
        avx2: detect_avx2_support(),
        avx512: detect_avx512_support(),
        neon: cfg!(target_feature = "neon"),
        sve: cfg!(target_feature = "sve"),
        wasm_simd: cfg!(target_feature = "simd128"),
    }
}

fn detect_sse_support() -> bool {
    cfg_if::cfg_if! { 
        if #[cfg(target_arch = "x86_64")] { 
            is_x86_feature_detected!("sse") 
        } else { 
            false 
        } 
    }
}

fn detect_sse2_support() -> bool {
    cfg_if::cfg_if! { 
        if #[cfg(target_arch = "x86_64")] { 
            is_x86_feature_detected!("sse2") 
        } else { 
            false 
        } 
    }
}

fn detect_avx_support() -> bool {
    cfg_if::cfg_if! { 
        if #[cfg(target_arch = "x86_64")] { 
            is_x86_feature_detected!("avx") 
        } else { 
            false 
        } 
    }
}

fn detect_avx2_support() -> bool {
    cfg_if::cfg_if! { 
        if #[cfg(target_arch = "x86_64")] { 
            is_x86_feature_detected!("avx2") 
        } else { 
            false 
        } 
    }
}

fn detect_avx512_support() -> bool {
    cfg_if::cfg_if! { 
        if #[cfg(target_arch = "x86_64")] { 
            is_x86_feature_detected!("avx512f") 
        } else { 
            false 
        } 
    }
}

fn detect_memory_features() -> MemoryFeatures {
    MemoryFeatures {
        large_pages: true, // Most modern platforms support large pages
        memory_tagging: cfg!(target_feature = "mte"), // ARM64 MTE
        memory_protection_keys: detect_pku_support(),
        page_size: get_page_size(),
        cache_line_size: get_cache_line_size(),
    }
}

fn detect_threading_features() -> ThreadingFeatures {
    ThreadingFeatures {
        hyper_threading: detect_ht_support(),
        big_little_cores: cfg!(target_arch = "aarch64"), // ARM64 often has big.LITTLE
        numa_topology: detect_numa_topology(),
        hardware_concurrency: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1),
    }
}

fn detect_crypto_acceleration() -> CryptoAcceleration {
    CryptoAcceleration {
        aes_ni: detect_aes_support(),
        sha_extensions: detect_sha_support(),
        random_number_generator: detect_rdrand_support(),
    }
}

fn detect_aes_support() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            is_x86_feature_detected!("aes")
        } else if #[cfg(target_arch = "aarch64")] {
            cfg!(target_feature = "aes")
        } else {
            false
        }
    }
}

fn detect_sha_support() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            is_x86_feature_detected!("sha")
        } else if #[cfg(target_arch = "aarch64")] {
            cfg!(target_feature = "sha2") || cfg!(target_feature = "sha3")
        } else {
            false
        }
    }
}

fn detect_rdrand_support() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            is_x86_feature_detected!("rdrand")
        } else {
            false
        }
    }
}

fn detect_pku_support() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            false // PKU not supported in cross-compilation
        } else {
            false
        }
    }
}

fn detect_ht_support() -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            false // HT detection not supported in cross-compilation
        } else {
            false
        }
    }
}

fn get_page_size() -> usize {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            16 * 1024 // 16KB on ARM64 macOS
        } else if #[cfg(target_arch = "aarch64")] {
            4 * 1024  // 4KB on ARM64 Linux (typically)
        } else if #[cfg(target_arch = "wasm32")] {
            64 * 1024 // 64KB WASM page size
        } else {
            4 * 1024  // 4KB default
        }
    }
}

fn get_cache_line_size() -> usize {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            64 // ARM64 typically uses 64-byte cache lines
        } else if #[cfg(target_arch = "x86_64")] {
            64 // x86_64 typically uses 64-byte cache lines
        } else {
            64 // Safe default
        }
    }
}

fn detect_numa_topology() -> bool {
    // Simplified NUMA detection - in practice would check /proc/cpuinfo or similar
    std::thread::available_parallelism()
        .map(|n| n.get() > 16) // Assume NUMA if more than 16 cores
        .unwrap_or(false)
}
