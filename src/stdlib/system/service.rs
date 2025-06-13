/// Service management for CURSED
/// 
/// This module provides cross-platform service management capabilities including
/// starting, stopping, and monitoring system services.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::process::Command;
use super::info::{SystemResult, SystemError};

/// Service manager for cross-platform service operations
#[derive(Debug)]
pub struct ServiceManager {
    platform: ServicePlatform,
    services_cache: HashMap<String, Service>,
    last_refresh: Option<SystemTime>,
}

/// Service information and control
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub status: ServiceStatus,
    pub start_type: ServiceStartType,
    pub binary_path: Option<String>,
    pub dependencies: Vec<String>,
    pub pid: Option<u32>,
    pub memory_usage: Option<u64>,
    pub start_time: Option<SystemTime>,
}

/// Service status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Paused,
    Unknown,
    Error(String),
}

/// Service start type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStartType {
    Automatic,
    Manual,
    Disabled,
    AutomaticDelayed,
    Unknown,
}

/// Platform-specific service management
#[derive(Debug, Clone)]
enum ServicePlatform {
    Windows,
    Systemd,      // Linux with systemd
    SysV,         // Traditional Unix/Linux
    Launchd,      // macOS
    OpenRC,       // Alpine Linux, Gentoo
    Upstart,      // Ubuntu (legacy)
    Unknown,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new() -> SystemResult<Self> {
        let platform = detect_service_platform()?;
        
        Ok(Self {
            platform,
            services_cache: HashMap::new(),
            last_refresh: None,
        })
    }

    /// List all services
    pub fn list_services(&mut self) -> SystemResult<Vec<Service>> {
        self.refresh_services()?;
        Ok(self.services_cache.values().cloned().collect())
    }

    /// Get a specific service by name
    pub fn get_service(&mut self, name: &str) -> SystemResult<Option<Service>> {
        self.refresh_services()?;
        Ok(self.services_cache.get(name).cloned())
    }

    /// Start a service
    pub fn start_service(&mut self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Windows => self.start_service_windows(name),
            ServicePlatform::Systemd => self.start_service_systemd(name),
            ServicePlatform::SysV => self.start_service_sysv(name),
            ServicePlatform::Launchd => self.start_service_launchd(name),
            ServicePlatform::OpenRC => self.start_service_openrc(name),
            ServicePlatform::Upstart => self.start_service_upstart(name),
            ServicePlatform::Unknown => Err(SystemError::PlatformNotSupported(
                "Service management not supported on this platform".to_string()
            )),
        }
    }

    /// Stop a service
    pub fn stop_service(&mut self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Windows => self.stop_service_windows(name),
            ServicePlatform::Systemd => self.stop_service_systemd(name),
            ServicePlatform::SysV => self.stop_service_sysv(name),
            ServicePlatform::Launchd => self.stop_service_launchd(name),
            ServicePlatform::OpenRC => self.stop_service_openrc(name),
            ServicePlatform::Upstart => self.stop_service_upstart(name),
            ServicePlatform::Unknown => Err(SystemError::PlatformNotSupported(
                "Service management not supported on this platform".to_string()
            )),
        }
    }

    /// Restart a service
    pub fn restart_service(&mut self, name: &str) -> SystemResult<()> {
        self.stop_service(name)?;
        
        // Wait a moment for the service to fully stop
        std::thread::sleep(Duration::from_millis(500));
        
        self.start_service(name)
    }

    /// Enable a service (set to start automatically)
    pub fn enable_service(&mut self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Windows => self.enable_service_windows(name),
            ServicePlatform::Systemd => self.enable_service_systemd(name),
            ServicePlatform::SysV => self.enable_service_sysv(name),
            ServicePlatform::Launchd => self.enable_service_launchd(name),
            ServicePlatform::OpenRC => self.enable_service_openrc(name),
            ServicePlatform::Upstart => self.enable_service_upstart(name),
            ServicePlatform::Unknown => Err(SystemError::PlatformNotSupported(
                "Service management not supported on this platform".to_string()
            )),
        }
    }

    /// Disable a service (prevent automatic startup)
    pub fn disable_service(&mut self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Windows => self.disable_service_windows(name),
            ServicePlatform::Systemd => self.disable_service_systemd(name),
            ServicePlatform::SysV => self.disable_service_sysv(name),
            ServicePlatform::Launchd => self.disable_service_launchd(name),
            ServicePlatform::OpenRC => self.disable_service_openrc(name),
            ServicePlatform::Upstart => self.disable_service_upstart(name),
            ServicePlatform::Unknown => Err(SystemError::PlatformNotSupported(
                "Service management not supported on this platform".to_string()
            )),
        }
    }

    /// Refresh the services cache
    fn refresh_services(&mut self) -> SystemResult<()> {
        // Only refresh if cache is older than 30 seconds
        if let Some(last_refresh) = self.last_refresh {
            if last_refresh.elapsed().unwrap_or(Duration::from_secs(0)) < Duration::from_secs(30) {
                return Ok(());
            }
        }

        self.services_cache.clear();
        
        let services = match &self.platform {
            ServicePlatform::Windows => self.list_services_windows()?,
            ServicePlatform::Systemd => self.list_services_systemd()?,
            ServicePlatform::SysV => self.list_services_sysv()?,
            ServicePlatform::Launchd => self.list_services_launchd()?,
            ServicePlatform::OpenRC => self.list_services_openrc()?,
            ServicePlatform::Upstart => self.list_services_upstart()?,
            ServicePlatform::Unknown => return Err(SystemError::PlatformNotSupported(
                "Service listing not supported on this platform".to_string()
            )),
        };

        for service in services {
            self.services_cache.insert(service.name.clone(), service);
        }

        self.last_refresh = Some(SystemTime::now());
        Ok(())
    }

    // Windows service management
    fn list_services_windows(&self) -> SystemResult<Vec<Service>> {
        let output = Command::new("sc")
            .args(&["query", "type=service", "state=all"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if !output.status.success() {
            return Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                "Failed to query Windows services".to_string()
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();
        let mut current_service: Option<Service> = None;

        for line in stdout.lines() {
            let line = line.trim();
            
            if line.starts_with("SERVICE_NAME:") {
                if let Some(service) = current_service.take() {
                    services.push(service);
                }
                
                let name = line.split(':').nth(1).unwrap_or("").trim().to_string();
                current_service = Some(Service {
                    name,
                    display_name: None,
                    description: None,
                    status: ServiceStatus::Unknown,
                    start_type: ServiceStartType::Unknown,
                    binary_path: None,
                    dependencies: Vec::new(),
                    pid: None,
                    memory_usage: None,
                    start_time: None,
                });
            } else if line.starts_with("DISPLAY_NAME:") {
                if let Some(ref mut service) = current_service {
                    service.display_name = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
                }
            } else if line.starts_with("STATE") {
                if let Some(ref mut service) = current_service {
                    if line.contains("RUNNING") {
                        service.status = ServiceStatus::Running;
                    } else if line.contains("STOPPED") {
                        service.status = ServiceStatus::Stopped;
                    } else if line.contains("START_PENDING") {
                        service.status = ServiceStatus::Starting;
                    } else if line.contains("STOP_PENDING") {
                        service.status = ServiceStatus::Stopping;
                    } else if line.contains("PAUSED") {
                        service.status = ServiceStatus::Paused;
                    }
                }
            }
        }

        if let Some(service) = current_service {
            services.push(service);
        }

        Ok(services)
    }

    fn start_service_windows(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("net")
            .args(&["start", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}': {}", name, stderr)
            ))
        }
    }

    fn stop_service_windows(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("net")
            .args(&["stop", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}': {}", name, stderr)
            ))
        }
    }

    fn enable_service_windows(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("sc")
            .args(&["config", name, "start=auto"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to enable service '{}': {}", name, stderr)
            ))
        }
    }

    fn disable_service_windows(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("sc")
            .args(&["config", name, "start=disabled"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to disable service '{}': {}", name, stderr)
            ))
        }
    }

    // Systemd service management
    fn list_services_systemd(&self) -> SystemResult<Vec<Service>> {
        let output = Command::new("systemctl")
            .args(&["list-units", "--type=service", "--all", "--no-pager"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if !output.status.success() {
            return Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                "Failed to list systemd services".to_string()
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();

        for line in stdout.lines().skip(1) { // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let name = parts[0].trim_end_matches(".service").to_string();
                let status = match parts[2] {
                    "active" => ServiceStatus::Running,
                    "inactive" => ServiceStatus::Stopped,
                    "activating" => ServiceStatus::Starting,
                    "deactivating" => ServiceStatus::Stopping,
                    "failed" => ServiceStatus::Error("Service failed".to_string()),
                    _ => ServiceStatus::Unknown,
                };

                services.push(Service {
                    name,
                    display_name: None,
                    description: None,
                    status,
                    start_type: ServiceStartType::Unknown,
                    binary_path: None,
                    dependencies: Vec::new(),
                    pid: None,
                    memory_usage: None,
                    start_time: None,
                });
            }
        }

        Ok(services)
    }

    fn start_service_systemd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("systemctl")
            .args(&["start", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}': {}", name, stderr)
            ))
        }
    }

    fn stop_service_systemd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("systemctl")
            .args(&["stop", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}': {}", name, stderr)
            ))
        }
    }

    fn enable_service_systemd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("systemctl")
            .args(&["enable", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to enable service '{}': {}", name, stderr)
            ))
        }
    }

    fn disable_service_systemd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("systemctl")
            .args(&["disable", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to disable service '{}': {}", name, stderr)
            ))
        }
    }

    // Placeholder implementations for other platforms
    fn list_services_sysv(&self) -> SystemResult<Vec<Service>> {
        // SysV init services are typically in /etc/init.d/
        Ok(Vec::new()) // Simplified implementation
    }

    fn start_service_sysv(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("service")
            .args(&[name, "start"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}'", name)
            ))
        }
    }

    fn stop_service_sysv(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("service")
            .args(&[name, "stop"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}'", name)
            ))
        }
    }

    fn enable_service_sysv(&self, _name: &str) -> SystemResult<()> {
        // SysV enable/disable varies by distribution
        Err(SystemError::InformationNotAvailable("SysV service enable not implemented".to_string()))
    }

    fn disable_service_sysv(&self, _name: &str) -> SystemResult<()> {
        // SysV enable/disable varies by distribution
        Err(SystemError::InformationNotAvailable("SysV service disable not implemented".to_string()))
    }

    // macOS Launchd placeholder implementations
    fn list_services_launchd(&self) -> SystemResult<Vec<Service>> {
        Ok(Vec::new())
    }

    fn start_service_launchd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("launchctl")
            .args(&["start", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}'", name)
            ))
        }
    }

    fn stop_service_launchd(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("launchctl")
            .args(&["stop", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}'", name)
            ))
        }
    }

    fn enable_service_launchd(&self, _name: &str) -> SystemResult<()> {
        Err(SystemError::InformationNotAvailable("Launchd service enable not implemented".to_string()))
    }

    fn disable_service_launchd(&self, _name: &str) -> SystemResult<()> {
        Err(SystemError::InformationNotAvailable("Launchd service disable not implemented".to_string()))
    }

    // OpenRC placeholder implementations
    fn list_services_openrc(&self) -> SystemResult<Vec<Service>> {
        Ok(Vec::new())
    }

    fn start_service_openrc(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("rc-service")
            .args(&[name, "start"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}'", name)
            ))
        }
    }

    fn stop_service_openrc(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("rc-service")
            .args(&[name, "stop"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}'", name)
            ))
        }
    }

    fn enable_service_openrc(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("rc-update")
            .args(&["add", name, "default"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to enable service '{}'", name)
            ))
        }
    }

    fn disable_service_openrc(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("rc-update")
            .args(&["del", name, "default"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to disable service '{}'", name)
            ))
        }
    }

    // Upstart placeholder implementations
    fn list_services_upstart(&self) -> SystemResult<Vec<Service>> {
        Ok(Vec::new())
    }

    fn start_service_upstart(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("initctl")
            .args(&["start", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to start service '{}'", name)
            ))
        }
    }

    fn stop_service_upstart(&self, name: &str) -> SystemResult<()> {
        let output = Command::new("initctl")
            .args(&["stop", name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to stop service '{}'", name)
            ))
        }
    }

    fn enable_service_upstart(&self, _name: &str) -> SystemResult<()> {
        Err(SystemError::InformationNotAvailable("Upstart service enable not implemented".to_string()))
    }

    fn disable_service_upstart(&self, _name: &str) -> SystemResult<()> {
        Err(SystemError::InformationNotAvailable("Upstart service disable not implemented".to_string()))
    }
}

/// Detect the service platform
fn detect_service_platform() -> SystemResult<ServicePlatform> {
    #[cfg(windows)]
    return Ok(ServicePlatform::Windows);
    
    #[cfg(unix)]
    {
        // Check for systemd
        if Command::new("systemctl").arg("--version").output().is_ok() {
            return Ok(ServicePlatform::Systemd);
        }
        
        // Check for launchd (macOS)
        if Command::new("launchctl").arg("version").output().is_ok() {
            return Ok(ServicePlatform::Launchd);
        }
        
        // Check for OpenRC
        if Command::new("rc-service").arg("--version").output().is_ok() {
            return Ok(ServicePlatform::OpenRC);
        }
        
        // Check for Upstart
        if Command::new("initctl").arg("version").output().is_ok() {
            return Ok(ServicePlatform::Upstart);
        }
        
        // Default to SysV
        return Ok(ServicePlatform::SysV);
    }
    
    #[cfg(not(any(windows, unix)))]
    Ok(ServicePlatform::Unknown)
}

// Global service manager instance
static mut GLOBAL_SERVICE_MANAGER: Option<ServiceManager> = None;
static INIT_LOCK: std::sync::Once = std::sync::Once::new();

/// Initialize the global service manager
pub fn init_service_manager() -> SystemResult<()> {
    unsafe {
        INIT_LOCK.call_once(|| {
            if let Ok(manager) = ServiceManager::new() {
                GLOBAL_SERVICE_MANAGER = Some(manager);
            }
        });
    }
    Ok(())
}

/// Cleanup the global service manager
pub fn cleanup_service_manager() -> SystemResult<()> {
    unsafe {
        GLOBAL_SERVICE_MANAGER = None;
    }
    Ok(())
}

/// Start a service using the global service manager
pub fn start_service(name: &str) -> SystemResult<()> {
    unsafe {
        if let Some(ref mut manager) = GLOBAL_SERVICE_MANAGER {
            manager.start_service(name)
        } else {
            Err(SystemError::InformationNotAvailable("Service manager not initialized".to_string()))
        }
    }
}

/// Stop a service using the global service manager
pub fn stop_service(name: &str) -> SystemResult<()> {
    unsafe {
        if let Some(ref mut manager) = GLOBAL_SERVICE_MANAGER {
            manager.stop_service(name)
        } else {
            Err(SystemError::InformationNotAvailable("Service manager not initialized".to_string()))
        }
    }
}

/// Get service status using the global service manager
pub fn get_service_status(name: &str) -> SystemResult<ServiceStatus> {
    unsafe {
        if let Some(ref mut manager) = GLOBAL_SERVICE_MANAGER {
            if let Some(service) = manager.get_service(name)? {
                Ok(service.status)
            } else {
                Err(SystemError::InformationNotAvailable(format!("Service '{}' not found", name)))
            }
        } else {
            Err(SystemError::InformationNotAvailable("Service manager not initialized".to_string()))
        }
    }
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Running => write!(f, "Running"),
            ServiceStatus::Stopped => write!(f, "Stopped"),
            ServiceStatus::Starting => write!(f, "Starting"),
            ServiceStatus::Stopping => write!(f, "Stopping"),
            ServiceStatus::Paused => write!(f, "Paused"),
            ServiceStatus::Unknown => write!(f, "Unknown"),
            ServiceStatus::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::fmt::Display for ServiceStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStartType::Automatic => write!(f, "Automatic"),
            ServiceStartType::Manual => write!(f, "Manual"),
            ServiceStartType::Disabled => write!(f, "Disabled"),
            ServiceStartType::AutomaticDelayed => write!(f, "Automatic (Delayed)"),
            ServiceStartType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager_creation() {
        let result = ServiceManager::new();
        assert!(result.is_ok(), "Failed to create service manager: {:?}", result);
    }

    #[test]
    fn test_platform_detection() {
        let platform = detect_service_platform();
        assert!(platform.is_ok(), "Failed to detect service platform: {:?}", platform);
    }

    #[test]
    fn test_service_status_display() {
        assert_eq!(format!("{}", ServiceStatus::Running), "Running");
        assert_eq!(format!("{}", ServiceStatus::Stopped), "Stopped");
        assert_eq!(format!("{}", ServiceStatus::Error("test".to_string())), "Error: test");
    }

    #[test]
    fn test_service_start_type_display() {
        assert_eq!(format!("{}", ServiceStartType::Automatic), "Automatic");
        assert_eq!(format!("{}", ServiceStartType::Manual), "Manual");
        assert_eq!(format!("{}", ServiceStartType::Disabled), "Disabled");
    }

    #[test]
    fn test_global_service_manager() {
        init_service_manager().expect("Failed to initialize service manager");
        
        // Test cleanup
        cleanup_service_manager().expect("Failed to cleanup service manager");
    }
}
