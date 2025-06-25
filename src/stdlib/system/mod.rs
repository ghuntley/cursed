/// System integration and management module for CURSED
/// 
/// This module provides comprehensive system integration capabilities including:
/// - System information gathering (OS, hardware, environment)
/// - Service management across platforms
/// - System monitoring and performance metrics
/// - Hardware detection and resource management
/// - System-wide configuration and settings

pub mod info;
pub mod service;
pub mod monitoring;
pub mod hardware;
pub mod environment;
pub mod platform;

pub use info::*;
pub use service::*;
pub use monitoring::*;
pub use hardware::*;
pub use environment::*;
pub use platform::*;

// Re-export commonly used types
pub use info::{
    SystemInfo, OsInfo, KernelInfo, 
    get_system_info, get_os_info, get_kernel_info
};

pub use service::{
    ServiceManager, Service, ServiceStatus, ServiceStartType,
    start_service, stop_service, get_service_status
};

pub use monitoring::{
    SystemMonitor, ResourceUsage, PerformanceMetrics,
    monitor_system, get_resource_usage, get_performance_metrics
};

pub use hardware::{
    HardwareInfo, CpuInfo, MemoryInfo, StorageInfo,
    get_hardware_info, get_cpu_info, get_memory_info, get_storage_info
};

pub use environment::{
    EnvironmentManager, SystemPath, Registry,
    get_environment_variable, set_environment_variable, get_system_paths
};

/// System module initialization
// pub fn init() -> crate::stdlib::system::info::SystemResult<()> {
    // Initialize system monitoring
    monitoring::init_monitoring()?;
    
    // Initialize service management
    service::init_service_manager()?;
    
    // Initialize hardware detection
    hardware::init_hardware_detection()?;
    
    Ok(())
}

/// System module cleanup
// pub fn cleanup() -> crate::stdlib::system::info::SystemResult<()> {
    // Cleanup monitoring
    monitoring::cleanup_monitoring()?;
    
    // Cleanup service management
    service::cleanup_service_manager()?;
    
    // Cleanup hardware detection
    hardware::cleanup_hardware_detection()?;
    
    Ok(())
}

/// Check if the system module is available on this platform
pub fn is_available() -> bool {
    #[cfg(any(unix, windows))]
    return true;
    
    #[cfg(not(any(unix, windows)))]
    return false;
}

/// Get the platform name for system integration
pub fn get_platform() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";
    
    #[cfg(target_os = "linux")]
    return "linux";
    
    #[cfg(target_os = "macos")]
    return "macos";
    
    #[cfg(target_os = "freebsd")]
    return "freebsd";
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux", 
        target_os = "macos",
        target_os = "freebsd"
    )))]
    return "unknown";
}

