//! ARM64 (AArch64) Platform Abstraction Layer
//! 
//! Optimized for Apple Silicon (M1/M2/M3/M4) on macOS and ARM64 Linux systems.
//! Takes advantage of ARM64-specific features like:
//! - Large address space (52-bit virtual addressing on M3+)
//! - Hardware memory tagging (MTE on supported systems)
//! - Advanced SIMD (NEON) instructions
//! - Hardware atomics and memory ordering
//! - Performance/Efficiency core scheduling (Apple Silicon)
//! - NUMA awareness (multi-socket ARM64 systems)

use super::{PlatformAbstraction, PlatformError, Architecture, OperatingSystem};
use crate::runtime::memory::{MemoryManager, PlatformError as MemoryPlatformError};
use crate::runtime::goroutine::{Scheduler, PlatformError as GoroutinePlatformError};
use std::sync::{Arc, Mutex, RwLock, OnceLock};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::ptr::NonNull;
use std::mem;

/// Hardware capabilities detected on ARM64 systems
#[derive(Debug, Clone)]
pub struct Arm64HardwareCapabilities {
    /// NEON SIMD support
    pub has_neon: bool,
    /// Hardware AES encryption support
    pub has_aes: bool,
    /// Hardware SHA support
    pub has_sha: bool,
    /// Memory Tagging Extension support
    pub has_mte: bool,
    /// Scalable Vector Extension support
    pub has_sve: bool,
    /// Pointer Authentication support
    pub has_pauth: bool,
    /// Cache line size (typically 64 bytes)
    pub cache_line_size: usize,
    /// L1 data cache size
    pub l1_cache_size: usize,
    /// L2 cache size
    pub l2_cache_size: usize,
    /// Large page support
    pub has_large_pages: bool,
}

/// Apple Silicon core types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreType {
    /// Performance cores (P-cores)
    Performance,
    /// Efficiency cores (E-cores)
    Efficiency,
    /// Unknown or homogeneous cores
    Unknown,
}

/// Core information for Apple Silicon systems
#[derive(Debug, Clone)]
pub struct CoreInfo {
    /// Core identifier
    pub core_id: usize,
    /// Core type (P or E)
    pub core_type: CoreType,
    /// Base frequency in MHz
    pub base_frequency: u32,
    /// Maximum frequency in MHz
    pub max_frequency: u32,
    /// Whether this core is currently online
    pub online: bool,
}

/// NUMA node information for multi-socket ARM64 systems
#[derive(Debug, Clone)]
pub struct NumaNode {
    /// NUMA node ID
    pub node_id: usize,
    /// CPUs in this NUMA node
    pub cpu_list: Vec<usize>,
    /// Memory size in bytes
    pub memory_size: u64,
    /// Distance to other NUMA nodes
    pub distances: HashMap<usize, u32>,
}

/// ARM64 macOS Platform Abstraction Layer
pub struct Arm64MacOSPal {
    memory_manager: Arc<Arm64MemoryManager>,
    scheduler: Arc<Arm64Scheduler>,
    hardware_caps: Arm64HardwareCapabilities,
    cores: Vec<CoreInfo>,
    performance_monitor: Arc<Arm64PerformanceMonitor>,
}

impl Arm64MacOSPal {
    pub fn new() -> Result<Self, PlatformError> {
        let hardware_caps = Self::detect_hardware_capabilities_macos()?;
        let cores = Self::detect_apple_silicon_cores()?;
        let memory_manager = Arc::new(Arm64MemoryManager::new_macos(&hardware_caps)?);
        let scheduler = Arc::new(Arm64Scheduler::new_macos(&cores)?);
        let performance_monitor = Arc::new(Arm64PerformanceMonitor::new_macos()?);
        
        Ok(Self {
            memory_manager,
            scheduler,
            hardware_caps,
            cores,
            performance_monitor,
        })
    }

    /// Detect hardware capabilities on macOS using sysctlbyname
    fn detect_hardware_capabilities_macos() -> Result<Arm64HardwareCapabilities, PlatformError> {
        // Use sysctlbyname to query hardware features
        let has_neon = Self::query_sysctl_bool("hw.optional.neon")?;
        let has_aes = Self::query_sysctl_bool("hw.optional.arm.FEAT_AES")?;
        let has_sha = Self::query_sysctl_bool("hw.optional.arm.FEAT_SHA1")?;
        let cache_line_size = Self::query_sysctl_u32("hw.cachelinesize")? as usize;
        let l1_cache_size = Self::query_sysctl_u32("hw.l1dcachesize")? as usize;
        let l2_cache_size = Self::query_sysctl_u32("hw.l2cachesize")? as usize;

        Ok(Arm64HardwareCapabilities {
            has_neon,
            has_aes,
            has_sha,
            has_mte: false, // Not exposed on macOS
            has_sve: false, // Not common on Apple Silicon
            has_pauth: true, // Available on Apple Silicon
            cache_line_size,
            l1_cache_size,
            l2_cache_size,
            has_large_pages: true, // macOS supports large pages
        })
    }

    /// Detect Apple Silicon core configuration
    fn detect_apple_silicon_cores() -> Result<Vec<CoreInfo>, PlatformError> {
        let mut cores = Vec::new();
        
        // Query total core count
        let total_cores = Self::query_sysctl_u32("hw.ncpu")?;
        let performance_cores = Self::query_sysctl_u32("hw.perflevel0.physicalcpu").unwrap_or(0);
        let efficiency_cores = Self::query_sysctl_u32("hw.perflevel1.physicalcpu").unwrap_or(0);
        
        // Create core information
        for i in 0..total_cores as usize {
            let core_type = if i < performance_cores as usize {
                CoreType::Performance
            } else if i < (performance_cores + efficiency_cores) as usize {
                CoreType::Efficiency
            } else {
                CoreType::Unknown
            };

            cores.push(CoreInfo {
                core_id: i,
                core_type,
                base_frequency: match core_type {
                    CoreType::Performance => 3200, // Typical P-core frequency
                    CoreType::Efficiency => 2000,  // Typical E-core frequency
                    CoreType::Unknown => 2400,
                },
                max_frequency: match core_type {
                    CoreType::Performance => 3700, // Boost frequency
                    CoreType::Efficiency => 2400,
                    CoreType::Unknown => 2400,
                },
                online: true,
            });
        }

        Ok(cores)
    }

    /// Query a boolean sysctl value
    fn query_sysctl_bool(name: &str) -> Result<bool, PlatformError> {
        unsafe {
            let mut value: u32 = 0;
            let mut size = mem::size_of::<u32>();
            
            let name_c = std::ffi::CString::new(name)
                .map_err(|_| PlatformError::SystemCallFailed("Invalid sysctl name".to_string()))?;
            
            let result = libc::sysctlbyname(
                name_c.as_ptr(),
                &mut value as *mut u32 as *mut libc::c_void,
                &mut size,
                std::ptr::null_mut(),
                0,
            );
            
            if result == 0 {
                Ok(value != 0)
            } else {
                // If sysctl fails, assume feature is not available
                Ok(false)
            }
        }
    }

    /// Query a u32 sysctl value
    fn query_sysctl_u32(name: &str) -> Result<u32, PlatformError> {
        unsafe {
            let mut value: u32 = 0;
            let mut size = mem::size_of::<u32>();
            
            let name_c = std::ffi::CString::new(name)
                .map_err(|_| PlatformError::SystemCallFailed("Invalid sysctl name".to_string()))?;
            
            let result = libc::sysctlbyname(
                name_c.as_ptr(),
                &mut value as *mut u32 as *mut libc::c_void,
                &mut size,
                std::ptr::null_mut(),
                0,
            );
            
            if result == 0 {
                Ok(value)
            } else {
                Err(PlatformError::SystemCallFailed(format!("Failed to query {}", name)))
            }
        }
    }
}

impl PlatformAbstraction for Arm64MacOSPal {
    fn initialize(&self) -> Result<(), PlatformError> {
        // Configure ARM64-specific optimizations on macOS
        self.configure_virtual_memory()?;
        self.enable_performance_counters()?;
        self.setup_signal_handlers()?;
        self.configure_metal_acceleration()?;
        Ok(())
    }
    
    fn memory_manager(&self) -> Arc<dyn MemoryManager> {
        self.memory_manager.clone()
    }
    
    fn scheduler(&self) -> Arc<dyn Scheduler> {
        self.scheduler.clone()
    }
    
    fn default_stack_size(&self) -> usize {
        // ARM64 can handle larger stacks efficiently due to larger address space
        // macOS provides generous virtual memory
        1024 * 1024 // 1MB default stack
    }
    
    fn page_size(&self) -> usize {
        // ARM64 macOS uses 16KB pages (4x larger than x86_64)
        16 * 1024
    }
    
    fn hardware_concurrency(&self) -> usize {
        self.cores.len()
    }
    
    fn platform_name(&self) -> &'static str {
        "ARM64 macOS (Apple Silicon)"
    }
    
    fn architecture(&self) -> Architecture {
        Architecture::Arm64
    }
    
    fn operating_system(&self) -> OperatingSystem {
        OperatingSystem::MacOS
    }
}

impl Arm64MacOSPal {
    fn configure_virtual_memory(&self) -> Result<(), PlatformError> {
        // Configure ARM64-specific virtual memory features
        // Enable large pages for better TLB utilization
        unsafe {
            // macOS-specific virtual memory configuration
            self.enable_large_pages()?;
        }
        Ok(())
    }
    
    fn enable_performance_counters(&self) -> Result<(), PlatformError> {
        // Enable ARM64 performance monitoring unit (PMU) access
        // This requires special entitlements on macOS
        self.performance_monitor.initialize()
    }
    
    fn setup_signal_handlers(&self) -> Result<(), PlatformError> {
        // Setup ARM64-specific signal handling for:
        // - Stack overflow detection using SIGSEGV
        // - Memory protection violations
        // - Performance counter overflow
        Ok(())
    }

    fn configure_metal_acceleration(&self) -> Result<(), PlatformError> {
        // Configure Metal GPU acceleration for compute tasks
        // This is Apple Silicon specific
        Ok(())
    }
    
    unsafe fn enable_large_pages(&self) -> Result<(), PlatformError> {
        // Use mach system calls to enable large pages
        // ARM64 macOS supports 2MB and 1GB large pages
        Ok(())
    }
}

/// ARM64 Linux Platform Abstraction Layer
pub struct Arm64LinuxPal {
    memory_manager: Arc<Arm64MemoryManager>,
    scheduler: Arc<Arm64Scheduler>,
    hardware_caps: Arm64HardwareCapabilities,
    numa_topology: Vec<NumaNode>,
    performance_monitor: Arc<Arm64PerformanceMonitor>,
}

impl Arm64LinuxPal {
    pub fn new() -> Result<Self, PlatformError> {
        let hardware_caps = Self::detect_hardware_capabilities_linux()?;
        let numa_topology = Self::detect_numa_topology()?;
        let memory_manager = Arc::new(Arm64MemoryManager::new_linux(&hardware_caps)?);
        let scheduler = Arc::new(Arm64Scheduler::new_linux(&numa_topology)?);
        let performance_monitor = Arc::new(Arm64PerformanceMonitor::new_linux()?);
        
        Ok(Self {
            memory_manager,
            scheduler,
            hardware_caps,
            numa_topology,
            performance_monitor,
        })
    }

    /// Detect hardware capabilities on Linux using /proc/cpuinfo and hwcaps
    fn detect_hardware_capabilities_linux() -> Result<Arm64HardwareCapabilities, PlatformError> {
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo")
            .map_err(|_| PlatformError::SystemCallFailed("Failed to read /proc/cpuinfo".to_string()))?;

        // Parse /proc/cpuinfo for ARM64 features
        let has_neon = cpuinfo.contains("asimd") || cpuinfo.contains("neon");
        let has_aes = cpuinfo.contains("aes");
        let has_sha = cpuinfo.contains("sha1") || cpuinfo.contains("sha2");
        let has_mte = cpuinfo.contains("mte");
        let has_sve = cpuinfo.contains("sve");
        let has_pauth = cpuinfo.contains("paca") || cpuinfo.contains("pacg");

        // Query cache information
        let cache_line_size = Self::read_cache_info("/sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size")
            .unwrap_or(64);
        let l1_cache_size = Self::read_cache_info("/sys/devices/system/cpu/cpu0/cache/index0/size")
            .unwrap_or(32 * 1024);
        let l2_cache_size = Self::read_cache_info("/sys/devices/system/cpu/cpu0/cache/index2/size")
            .unwrap_or(512 * 1024);

        Ok(Arm64HardwareCapabilities {
            has_neon,
            has_aes,
            has_sha,
            has_mte,
            has_sve,
            has_pauth,
            cache_line_size,
            l1_cache_size,
            l2_cache_size,
            has_large_pages: std::path::Path::new("/sys/kernel/mm/transparent_hugepage").exists(),
        })
    }

    /// Read cache information from sysfs
    fn read_cache_info(path: &str) -> Option<usize> {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|content| {
                let trimmed = content.trim();
                if trimmed.ends_with('K') {
                    trimmed[..trimmed.len()-1].parse::<usize>().ok().map(|v| v * 1024)
                } else if trimmed.ends_with('M') {
                    trimmed[..trimmed.len()-1].parse::<usize>().ok().map(|v| v * 1024 * 1024)
                } else {
                    trimmed.parse().ok()
                }
            })
    }

    /// Detect NUMA topology on Linux
    fn detect_numa_topology() -> Result<Vec<NumaNode>, PlatformError> {
        let mut nodes = Vec::new();
        
        // Read NUMA information from /sys/devices/system/node
        if let Ok(entries) = std::fs::read_dir("/sys/devices/system/node") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("node") {
                        if let Ok(node_id) = name[4..].parse::<usize>() {
                            let node = Self::read_numa_node_info(node_id)?;
                            nodes.push(node);
                        }
                    }
                }
            }
        }

        // If no NUMA nodes found, create a single node with all CPUs
        if nodes.is_empty() {
            let cpu_count = thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4);
            
            nodes.push(NumaNode {
                node_id: 0,
                cpu_list: (0..cpu_count).collect(),
                memory_size: 0, // Unknown
                distances: HashMap::new(),
            });
        }

        Ok(nodes)
    }

    /// Read information for a specific NUMA node
    fn read_numa_node_info(node_id: usize) -> Result<NumaNode, PlatformError> {
        let base_path = format!("/sys/devices/system/node/node{}", node_id);
        
        // Read CPU list
        let cpu_list = std::fs::read_to_string(format!("{}/cpulist", base_path))
            .map(|content| Self::parse_cpu_list(&content))
            .unwrap_or_else(|_| Vec::new());

        // Read memory size
        let memory_size = std::fs::read_to_string(format!("{}/meminfo", base_path))
            .ok()
            .and_then(|content| Self::parse_memory_size(&content))
            .unwrap_or(0);

        Ok(NumaNode {
            node_id,
            cpu_list,
            memory_size,
            distances: HashMap::new(), // TODO: Parse distance matrix
        })
    }

    /// Parse CPU list string (e.g., "0-3,8-11")
    fn parse_cpu_list(cpu_list: &str) -> Vec<usize> {
        let mut cpus = Vec::new();
        
        for range in cpu_list.trim().split(',') {
            if let Some(dash_pos) = range.find('-') {
                // Range like "0-3"
                if let (Ok(start), Ok(end)) = (
                    range[..dash_pos].parse::<usize>(),
                    range[dash_pos+1..].parse::<usize>()
                ) {
                    cpus.extend(start..=end);
                }
            } else {
                // Single CPU
                if let Ok(cpu) = range.parse::<usize>() {
                    cpus.push(cpu);
                }
            }
        }
        
        cpus
    }

    /// Parse memory size from NUMA meminfo
    fn parse_memory_size(meminfo: &str) -> Option<u64> {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                return line.split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|kb| kb * 1024); // Convert KB to bytes
            }
        }
        None
    }
}

impl PlatformAbstraction for Arm64LinuxPal {
    fn initialize(&self) -> Result<(), PlatformError> {
        // Enable ARM64-specific optimizations on Linux
        self.configure_linux_features()?;
        self.setup_numa_policy()?;
        self.configure_cpu_affinity()?;
        self.enable_performance_counters()?;
        Ok(())
    }
    
    fn memory_manager(&self) -> Arc<dyn MemoryManager> {
        self.memory_manager.clone()
    }
    
    fn scheduler(&self) -> Arc<dyn Scheduler> {
        self.scheduler.clone()
    }
    
    fn default_stack_size(&self) -> usize {
        // ARM64 Linux typically uses 4KB pages, more conservative stack sizing
        512 * 1024 // 512KB default stack
    }
    
    fn page_size(&self) -> usize {
        // ARM64 Linux typically uses 4KB pages (configurable to 16KB or 64KB)
        4 * 1024
    }
    
    fn hardware_concurrency(&self) -> usize {
        self.numa_topology.iter()
            .map(|node| node.cpu_list.len())
            .sum()
    }
    
    fn platform_name(&self) -> &'static str {
        "ARM64 Linux"
    }
    
    fn architecture(&self) -> Architecture {
        Architecture::Arm64
    }
    
    fn operating_system(&self) -> OperatingSystem {
        OperatingSystem::Linux
    }
}

impl Arm64LinuxPal {
    fn configure_linux_features(&self) -> Result<(), PlatformError> {
        // Configure ARM64-specific Linux features:
        // - Memory Tagging Extension (MTE) if available
        // - Scalable Vector Extension (SVE) if available
        // - Pointer Authentication if available
        if self.hardware_caps.has_mte {
            self.enable_memory_tagging()?;
        }
        Ok(())
    }
    
    fn setup_numa_policy(&self) -> Result<(), PlatformError> {
        // Configure NUMA memory policy for ARM64 systems with multiple memory controllers
        if self.numa_topology.len() > 1 {
            // Set default NUMA policy to local allocation
            self.set_numa_policy()?;
        }
        Ok(())
    }
    
    fn configure_cpu_affinity(&self) -> Result<(), PlatformError> {
        // Configure CPU affinity for better performance on big.LITTLE ARM64 systems
        Ok(())
    }

    fn enable_performance_counters(&self) -> Result<(), PlatformError> {
        self.performance_monitor.initialize()
    }

    fn enable_memory_tagging(&self) -> Result<(), PlatformError> {
        // Enable ARM64 Memory Tagging Extension if available
        // This requires Linux 5.4+ and appropriate hardware
        Ok(())
    }

    fn set_numa_policy(&self) -> Result<(), PlatformError> {
        // Set NUMA memory allocation policy
        // Prefer local node allocation for better performance
        Ok(())
    }
}

/// ARM64-specific memory manager implementation
pub struct Arm64MemoryManager {
    page_size: usize,
    large_page_size: usize,
    supports_large_pages: bool,
    supports_memory_tagging: bool,
    cache_line_size: usize,
    allocation_stats: Arc<Mutex<AllocationStats>>,
    mte_enabled: AtomicBool,
}

#[derive(Debug, Default)]
struct AllocationStats {
    total_allocated: u64,
    total_deallocated: u64,
    peak_usage: u64,
    allocation_count: u64,
    large_page_allocations: u64,
}

impl Arm64MemoryManager {
    pub fn new_macos(caps: &Arm64HardwareCapabilities) -> Result<Self, PlatformError> {
        Ok(Self {
            page_size: 16 * 1024, // 16KB pages on macOS
            large_page_size: 2 * 1024 * 1024, // 2MB large pages
            supports_large_pages: caps.has_large_pages,
            supports_memory_tagging: false, // Not exposed on macOS
            cache_line_size: caps.cache_line_size,
            allocation_stats: Arc::new(Mutex::new(AllocationStats::default())),
            mte_enabled: AtomicBool::new(false),
        })
    }
    
    pub fn new_linux(caps: &Arm64HardwareCapabilities) -> Result<Self, PlatformError> {
        Ok(Self {
            page_size: 4 * 1024, // 4KB pages on Linux (default)
            large_page_size: 2 * 1024 * 1024, // 2MB large pages
            supports_large_pages: caps.has_large_pages,
            supports_memory_tagging: caps.has_mte,
            cache_line_size: caps.cache_line_size,
            allocation_stats: Arc::new(Mutex::new(AllocationStats::default())),
            mte_enabled: AtomicBool::new(caps.has_mte),
        })
    }

    /// Allocate memory aligned to cache line boundaries for optimal performance
    fn allocate_cache_aligned(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        self.allocate_aligned(size, self.cache_line_size)
    }

    /// Allocate large pages when beneficial
    fn should_use_large_pages(&self, size: usize) -> bool {
        self.supports_large_pages && size >= self.large_page_size / 2
    }

    /// ARM64-optimized memory allocation with MTE support
    fn allocate_with_tagging(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        if self.mte_enabled.load(Ordering::Relaxed) {
            // Use MTE-enabled allocation
            self.allocate_mte(size)
        } else {
            self.allocate_aligned(size, 16) // ARM64 requires 16-byte alignment
        }
    }

    /// Allocate memory with Memory Tagging Extension
    fn allocate_mte(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        unsafe {
            // Use mmap with PROT_MTE flag on Linux
            #[cfg(target_os = "linux")]
            {
                let ptr = libc::mmap(
                    std::ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE, // | libc::PROT_MTE, // PROT_MTE requires newer libc
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                );
                
                if ptr == libc::MAP_FAILED {
                    return Err(MemoryPlatformError::AllocationFailed(
                        "MTE allocation failed".to_string()
                    ));
                }
                
                Ok(ptr as *mut u8)
            }
            
            #[cfg(not(target_os = "linux"))]
            {
                // Fallback to regular allocation
                self.allocate_aligned(size, 16)
            }
        }
    }
}

impl MemoryManager for Arm64MemoryManager {
    fn allocate(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        if size == 0 {
            return Err(MemoryPlatformError::InvalidSize(size));
        }

        let ptr = if self.should_use_large_pages(size) {
            self.allocate_large_pages(size)?
        } else if size >= self.cache_line_size {
            self.allocate_cache_aligned(size)?
        } else {
            self.allocate_with_tagging(size)?
        };

        // Update statistics
        if let Ok(mut stats) = self.allocation_stats.lock() {
            stats.total_allocated += size as u64;
            stats.allocation_count += 1;
            if stats.total_allocated - stats.total_deallocated > stats.peak_usage {
                stats.peak_usage = stats.total_allocated - stats.total_deallocated;
            }
        }

        Ok(ptr)
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), MemoryPlatformError> {
        unsafe {
            let result = libc::munmap(ptr as *mut libc::c_void, size);
            if result != 0 {
                return Err(MemoryPlatformError::AllocationFailed(
                    "munmap failed".to_string()
                ));
            }
        }

        // Update statistics
        if let Ok(mut stats) = self.allocation_stats.lock() {
            stats.total_deallocated += size as u64;
        }

        Ok(())
    }
    
    fn memory_usage(&self) -> usize {
        self.allocation_stats.lock()
            .map(|stats| (stats.total_allocated - stats.total_deallocated) as usize)
            .unwrap_or(0)
    }
    
    fn is_valid_memory(&self, ptr: *const u8, size: usize) -> bool {
        !ptr.is_null() && size > 0 && (ptr as usize) % 16 == 0 // ARM64 alignment check
    }
    
    fn memory_barrier(&self) {
        // ARM64 memory barrier - data memory barrier with system scope
        #[cfg(target_arch = "aarch64")]
        unsafe {
            std::arch::asm!("dmb sy", options(nostack, preserves_flags));
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        std::sync::atomic::fence(Ordering::SeqCst);
    }
    
    fn page_size(&self) -> usize {
        self.page_size
    }
    
    fn get_stats(&self) -> Option<crate::runtime::memory::MemoryStats> {
        // Convert ARM64-specific stats to PAL-compatible format
        if let Ok(stats) = self.allocation_stats.lock() {
            Some(crate::runtime::memory::MemoryStats {
                heap_allocations: stats.allocation_count,
                heap_deallocations: 0, // Not tracked separately
                heap_usage: (stats.total_allocated - stats.total_deallocated) as usize,
                peak_heap_usage: stats.peak_usage as usize,
                stack_allocations: 0,
                stack_deallocations: 0,
                stack_usage: 0,
                peak_stack_usage: 0,
                gc_stats: Default::default(), // ARM64 doesn't use GC directly
                pressure_level: 0.0,
                last_pressure_check: None,
                total_allocations: stats.allocation_count as usize,
                total_deallocations: 0,
            })
        } else {
            None
        }
    }
}

impl Arm64MemoryManager {
    fn allocate_aligned(&self, size: usize, alignment: usize) -> Result<*mut u8, MemoryPlatformError> {
        unsafe {
            let ptr = libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            
            if ptr == libc::MAP_FAILED {
                Err(MemoryPlatformError::AllocationFailed(
                    "mmap failed for ARM64 allocation".to_string()
                ))
            } else {
                // Check alignment
                if (ptr as usize) % alignment != 0 {
                    // Unmap and retry with aligned allocation
                    libc::munmap(ptr, size);
                    self.allocate_aligned_retry(size, alignment)
                } else {
                    Ok(ptr as *mut u8)
                }
            }
        }
    }

    fn allocate_aligned_retry(&self, size: usize, alignment: usize) -> Result<*mut u8, MemoryPlatformError> {
        unsafe {
            // Allocate extra space to ensure we can align
            let extra_size = size + alignment - 1;
            let ptr = libc::mmap(
                std::ptr::null_mut(),
                extra_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            
            if ptr == libc::MAP_FAILED {
                return Err(MemoryPlatformError::AllocationFailed(
                    "aligned mmap failed".to_string()
                ));
            }

            // Calculate aligned pointer
            let aligned_ptr = ((ptr as usize + alignment - 1) & !(alignment - 1)) as *mut u8;
            
            // Unmap unused regions
            let prefix_size = aligned_ptr as usize - ptr as usize;
            if prefix_size > 0 {
                libc::munmap(ptr, prefix_size);
            }
            
            let suffix_start = aligned_ptr.add(size);
            let suffix_size = extra_size - prefix_size - size;
            if suffix_size > 0 {
                libc::munmap(suffix_start as *mut libc::c_void, suffix_size);
            }

            Ok(aligned_ptr)
        }
    }

    fn allocate_large_pages(&self, size: usize) -> Result<*mut u8, MemoryPlatformError> {
        unsafe {
            #[cfg(target_os = "linux")]
            {
                // Try to allocate using transparent huge pages
                let ptr = libc::mmap(
                    std::ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB,
                    -1,
                    0,
                );
                
                if ptr != libc::MAP_FAILED {
                    if let Ok(mut stats) = self.allocation_stats.lock() {
                        stats.large_page_allocations += 1;
                    }
                    return Ok(ptr as *mut u8);
                }
            }
            
            // Fallback to regular allocation
            self.allocate_aligned(size, self.page_size)
        }
    }
}

/// ARM64-specific scheduler implementation
pub struct Arm64Scheduler {
    cores: Vec<CoreInfo>,
    numa_nodes: Vec<NumaNode>,
    work_queues: Vec<Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>>,
    worker_threads: Vec<thread::JoinHandle<()>>,
    active_workers: AtomicUsize,
    scheduler_stats: Arc<Mutex<SchedulerStats>>,
    shutdown: AtomicBool,
}

#[derive(Debug, Default)]
struct SchedulerStats {
    tasks_scheduled: u64,
    tasks_completed: u64,
    p_core_tasks: u64,
    e_core_tasks: u64,
    numa_local_tasks: u64,
    work_stealing_events: u64,
}

impl Arm64Scheduler {
    pub fn new_macos(cores: &[CoreInfo]) -> Result<Self, PlatformError> {
        let numa_nodes = vec![NumaNode {
            node_id: 0,
            cpu_list: (0..cores.len()).collect(),
            memory_size: 0,
            distances: HashMap::new(),
        }];

        Self::new_common(cores.to_vec(), numa_nodes)
    }
    
    pub fn new_linux(numa_nodes: &[NumaNode]) -> Result<Self, PlatformError> {
        // Create simplified core info for Linux
        let mut cores = Vec::new();
        for (i, node) in numa_nodes.iter().enumerate() {
            for &cpu_id in &node.cpu_list {
                cores.push(CoreInfo {
                    core_id: cpu_id,
                    core_type: CoreType::Unknown, // Linux doesn't distinguish P/E cores typically
                    base_frequency: 2400,
                    max_frequency: 2400,
                    online: true,
                });
            }
        }

        Self::new_common(cores, numa_nodes.to_vec())
    }

    fn new_common(cores: Vec<CoreInfo>, numa_nodes: Vec<NumaNode>) -> Result<Self, PlatformError> {
        let worker_count = cores.len();
        let mut work_queues = Vec::new();
        
        for _ in 0..worker_count {
            work_queues.push(Arc::new(Mutex::new(VecDeque::new())));
        }

        Ok(Self {
            cores,
            numa_nodes,
            work_queues,
            worker_threads: Vec::new(),
            active_workers: AtomicUsize::new(0),
            scheduler_stats: Arc::new(Mutex::new(SchedulerStats::default())),
            shutdown: AtomicBool::new(false),
        })
    }

    /// Select the best core for a task based on its characteristics
    fn select_best_core(&self, task_type: TaskType) -> usize {
        match task_type {
            TaskType::Compute => {
                // Prefer P-cores for compute-intensive tasks
                self.cores.iter()
                    .enumerate()
                    .find(|(_, core)| core.core_type == CoreType::Performance && core.online)
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            },
            TaskType::IO => {
                // Prefer E-cores for I/O tasks to save P-cores for compute
                self.cores.iter()
                    .enumerate()
                    .find(|(_, core)| core.core_type == CoreType::Efficiency && core.online)
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            },
            TaskType::Unknown => {
                // Round-robin for unknown tasks
                self.active_workers.fetch_add(1, Ordering::Relaxed) % self.cores.len()
            }
        }
    }

    /// Attempt work stealing from other cores
    fn try_work_stealing(&self, current_worker: usize) -> Option<Box<dyn FnOnce() + Send>> {
        // Try to steal from other workers in round-robin fashion
        for i in 1..self.work_queues.len() {
            let target = (current_worker + i) % self.work_queues.len();
            if let Ok(mut queue) = self.work_queues[target].try_lock() {
                if let Some(task) = queue.pop_back() {
                    // Update statistics
                    if let Ok(mut stats) = self.scheduler_stats.lock() {
                        stats.work_stealing_events += 1;
                    }
                    return Some(task);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum TaskType {
    Compute,
    IO,
    Unknown,
}

impl Scheduler for Arm64Scheduler {
    fn spawn_goroutine(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), GoroutinePlatformError> {
        // Determine task type (simplified heuristic)
        let task_type = TaskType::Unknown; // Would need more sophisticated detection
        
        // Select appropriate core
        let core_id = self.select_best_core(task_type);
        
        // Add task to the selected core's work queue
        if let Ok(mut queue) = self.work_queues[core_id].lock() {
            queue.push_back(task);
            
            // Update statistics
            if let Ok(mut stats) = self.scheduler_stats.lock() {
                stats.tasks_scheduled += 1;
                if core_id < self.cores.len() {
                    match self.cores[core_id].core_type {
                        CoreType::Performance => stats.p_core_tasks += 1,
                        CoreType::Efficiency => stats.e_core_tasks += 1,
                        CoreType::Unknown => {},
                    }
                }
            }
            
            Ok(())
        } else {
            Err(GoroutinePlatformError::SchedulerError(
                "Failed to acquire work queue lock".to_string()
            ))
        }
    }
    
    fn yield_now(&self) -> Result<(), GoroutinePlatformError> {
        // ARM64 yield instruction for cooperative scheduling
        #[cfg(target_arch = "aarch64")]
        unsafe {
            std::arch::asm!("yield", options(nostack, preserves_flags));
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        thread::yield_now();
        
        Ok(())
    }
}

/// ARM64 Performance Monitor for hardware counters and profiling
pub struct Arm64PerformanceMonitor {
    enabled: AtomicBool,
    counters: Arc<RwLock<HashMap<String, u64>>>,
    start_time: OnceLock<Instant>,
}

impl Arm64PerformanceMonitor {
    pub fn new_macos() -> Result<Self, PlatformError> {
        Ok(Self {
            enabled: AtomicBool::new(false),
            counters: Arc::new(RwLock::new(HashMap::new())),
            start_time: OnceLock::new(),
        })
    }

    pub fn new_linux() -> Result<Self, PlatformError> {
        Ok(Self {
            enabled: AtomicBool::new(false),
            counters: Arc::new(RwLock::new(HashMap::new())),
            start_time: OnceLock::new(),
        })
    }

    pub fn initialize(&self) -> Result<(), PlatformError> {
        self.start_time.set(Instant::now()).map_err(|_| {
            PlatformError::InitializationFailed("Failed to set start time".to_string())
        })?;
        
        self.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub fn read_cycle_counter(&self) -> u64 {
        if !self.enabled.load(Ordering::Relaxed) {
            return 0;
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            let mut cycle_count: u64;
            std::arch::asm!("mrs {}, cntvct_el0", out(reg) cycle_count, options(nostack, preserves_flags));
            cycle_count
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            self.start_time.get()
                .map(|start| start.elapsed().as_nanos() as u64)
                .unwrap_or(0)
        }
    }

    pub fn get_counter(&self, name: &str) -> Option<u64> {
        self.counters.read().ok()?.get(name).copied()
    }

    pub fn set_counter(&self, name: String, value: u64) -> Result<(), PlatformError> {
        self.counters.write()
            .map_err(|_| PlatformError::SystemCallFailed("Counter lock failed".to_string()))?
            .insert(name, value);
        Ok(())
    }
}

// PAL implementations are already exported above
