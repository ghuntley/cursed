//! System information and monitoring functionality
//!
//! This module provides system-level information and monitoring capabilities.

pub mod info;
pub mod service;
pub mod monitoring;
pub mod hardware;
pub mod environment;
pub mod platform;
pub mod error;

// Re-export all public items with explicit naming to avoid ambiguity
pub use info::{init_info, test_info, ModuleResult as InfoResult, ModuleHandler as InfoHandler};
pub use service::{init_service, test_service, ModuleResult as ServiceResult, ModuleHandler as ServiceHandler};
pub use monitoring::{init_monitoring, test_monitoring, ModuleResult as MonitoringResult, ModuleHandler as MonitoringHandler};
pub use hardware::{init_hardware, test_hardware, ModuleResult as HardwareResult, ModuleHandler as HardwareHandler};
pub use environment::{init_environment, test_environment, ModuleResult as EnvironmentResult, ModuleHandler as EnvironmentHandler};
pub use platform::{init_platform, test_platform, ModuleResult as PlatformResult, ModuleHandler as PlatformHandler};

// Re-export commonly used types (commented out until implemented)
// pub use info::{
//     get_system_info, get_os_info, get_kernel_info
// };

// pub use service::{
//     start_service, stop_service, get_service_status
// };

// pub use monitoring::{
//     monitor_system, get_resource_usage, get_performance_metrics
// };

// pub use hardware::{
//     get_hardware_info, get_cpu_info, get_memory_info, get_storage_info
// };

// pub use environment::{
//     get_environment_variable, set_environment_variable, get_system_paths
// };

/// System module initialization
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system monitoring
    // monitoring::init_monitoring()?;
    
    // Initialize service management
    // service::init_service_manager()?;
    
    // Initialize hardware detection
    // hardware::init_hardware_detection()?;
    
    Ok(())
}

/// System module cleanup
pub fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
    // Cleanup monitoring
    // monitoring::cleanup_monitoring()?;
    
    // Cleanup service management
    // service::cleanup_service_manager()?;
    
    // Cleanup hardware detection
    // hardware::cleanup_hardware_detection()?;
    
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
