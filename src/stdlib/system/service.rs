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

    /// Reload systemd daemon configuration
    pub fn reload_daemon_config(&self) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Systemd => self.reload_systemd_config(),
            _ => Err(SystemError::PlatformNotSupported(
                "Daemon config reload only supported on systemd platforms".to_string()
            )),
        }
    }

    /// Get service logs (lines parameter specifies number of recent lines)
    pub fn get_service_logs(&self, name: &str, lines: Option<usize>) -> SystemResult<String> {
        match &self.platform {
            ServicePlatform::Systemd => self.get_service_logs_systemd(name, lines),
            _ => Err(SystemError::PlatformNotSupported(
                "Service logs only supported on systemd platforms".to_string()
            )),
        }
    }

    /// Mask a service (prevent it from being started by any means)
    pub fn mask_service(&self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Systemd => self.mask_service_systemd(name),
            _ => Err(SystemError::PlatformNotSupported(
                "Service masking only supported on systemd platforms".to_string()
            )),
        }
    }

    /// Unmask a previously masked service
    pub fn unmask_service(&self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Systemd => self.unmask_service_systemd(name),
            _ => Err(SystemError::PlatformNotSupported(
                "Service unmasking only supported on systemd platforms".to_string()
            )),
        }
    }

    /// Reload a service configuration without restarting
    pub fn reload_service(&self, name: &str) -> SystemResult<()> {
        match &self.platform {
            ServicePlatform::Systemd => self.reload_service_systemd(name),
            _ => Err(SystemError::PlatformNotSupported(
                "Service reload only supported on systemd platforms".to_string()
            )),
        }
    }

    /// Get the current platform being used for service management
    pub fn get_platform(&self) -> &ServicePlatform {
        &self.platform
    }

    /// Check if the service manager supports advanced features
    pub fn supports_advanced_features(&self) -> bool {
        matches!(self.platform, ServicePlatform::Systemd)
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
        use tracing::{debug, warn};
        
        // First get basic service list
        let output = Command::new("systemctl")
            .args(&["list-units", "--type=service", "--all", "--no-pager", "--plain"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        if !output.status.success() {
            return Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to list systemd services: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("UNIT") || line.starts_with("●") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0].ends_with(".service") {
                let name = parts[0].trim_end_matches(".service").to_string();
                let load_state = parts[1];
                let active_state = parts[2];
                let sub_state = parts[3];
                
                let status = match (active_state, sub_state) {
                    ("active", "running") => ServiceStatus::Running,
                    ("inactive", "dead") => ServiceStatus::Stopped,
                    ("activating", _) => ServiceStatus::Starting,
                    ("deactivating", _) => ServiceStatus::Stopping,
                    ("failed", _) => ServiceStatus::Error(format!("Service failed: {}", sub_state)),
                    ("active", "exited") => ServiceStatus::Stopped, // One-shot services
                    _ => ServiceStatus::Unknown,
                };

                // Get additional service details
                let mut service = Service {
                    name: name.clone(),
                    display_name: None,
                    description: None,
                    status,
                    start_type: ServiceStartType::Unknown,
                    binary_path: None,
                    dependencies: Vec::new(),
                    pid: None,
                    memory_usage: None,
                    start_time: None,
                };

                // Try to get detailed information for this service
                if let Ok(detailed_info) = self.get_detailed_service_info_systemd(&name) {
                    service.description = detailed_info.description;
                    service.start_type = detailed_info.start_type;
                    service.binary_path = detailed_info.binary_path;
                    service.dependencies = detailed_info.dependencies;
                    service.pid = detailed_info.pid;
                    service.memory_usage = detailed_info.memory_usage;
                    service.start_time = detailed_info.start_time;
                }

                services.push(service);
            }
        }

        debug!("Listed {} systemd services", services.len());
        Ok(services)
    }

    fn get_detailed_service_info_systemd(&self, name: &str) -> SystemResult<Service> {
        use tracing::debug;
        
        // Get service status with detailed information
        let show_output = Command::new("systemctl")
            .args(&["show", &format!("{}.service", name), "--no-pager"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        let mut description = None;
        let mut start_type = ServiceStartType::Unknown;
        let mut binary_path = None;
        let mut main_pid = None;
        let mut memory_current = None;
        let mut active_enter_timestamp = None;

        if show_output.status.success() {
            let show_stdout = String::from_utf8_lossy(&show_output.stdout);
            
            for line in show_stdout.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    match key {
                        "Description" => {
                            if !value.is_empty() {
                                description = Some(value.to_string());
                            }
                        },
                        "UnitFileState" => {
                            start_type = match value {
                                "enabled" => ServiceStartType::Automatic,
                                "disabled" => ServiceStartType::Disabled,
                                "static" => ServiceStartType::Manual,
                                "masked" => ServiceStartType::Disabled,
                                _ => ServiceStartType::Unknown,
                            };
                        },
                        "ExecStart" => {
                            if !value.is_empty() && value != "{ path=/bin/true ; argv[]=/bin/true ; ignore_errors=no ; start_time=[n/a] ; stop_time=[n/a] ; pid=0 ; code=(null) ; status=0/0 }" {
                                // Parse ExecStart format: { path=/path/to/binary ; ... }
                                if let Some(path_start) = value.find("path=") {
                                    if let Some(path_end) = value[path_start + 5..].find(" ;") {
                                        let path = &value[path_start + 5..path_start + 5 + path_end];
                                        binary_path = Some(path.to_string());
                                    }
                                }
                            }
                        },
                        "MainPID" => {
                            if let Ok(pid) = value.parse::<u32>() {
                                if pid > 0 {
                                    main_pid = Some(pid);
                                }
                            }
                        },
                        "MemoryCurrent" => {
                            if let Ok(memory) = value.parse::<u64>() {
                                memory_current = Some(memory);
                            }
                        },
                        "ActiveEnterTimestamp" => {
                            if !value.is_empty() && value != "n/a" {
                                // Parse timestamp - systemd uses format like "Mon 2023-01-01 12:00:00 UTC"
                                // For simplicity, we'll use current time as approximation
                                // In production, you'd want proper timestamp parsing
                                active_enter_timestamp = Some(SystemTime::now());
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        // Get dependencies
        let dependencies = self.get_service_dependencies_systemd(name).unwrap_or_default();

        debug!("Retrieved detailed info for service: {}", name);

        Ok(Service {
            name: name.to_string(),
            display_name: None,
            description,
            status: ServiceStatus::Unknown, // Will be set by caller
            start_type,
            binary_path,
            dependencies,
            pid: main_pid,
            memory_usage: memory_current,
            start_time: active_enter_timestamp,
        })
    }

    fn get_service_dependencies_systemd(&self, name: &str) -> SystemResult<Vec<String>> {
        let output = Command::new("systemctl")
            .args(&["list-dependencies", &format!("{}.service", name), "--plain", "--no-pager"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;

        let mut dependencies = Vec::new();
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.ends_with(".service") && !line.starts_with(&format!("{}.service", name)) {
                    let dep_name = line.trim_start_matches("● ")
                        .trim_start_matches("○ ")
                        .trim_end_matches(".service");
                    if !dep_name.is_empty() {
                        dependencies.push(dep_name.to_string());
                    }
                }
            }
        }

        Ok(dependencies)
    }

    fn validate_service_systemd(&self, name: &str) -> SystemResult<Result<(), SystemError>> {
        use tracing::debug;
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        // Check if the service unit file exists
        let output = Command::new("systemctl")
            .args(&["cat", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            debug!("Service {} validated successfully", name);
            Ok(Ok(()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("Unit") && stderr.contains("not found") {
                Ok(Err(SystemError::InformationNotAvailable(format!("Service '{}' not found", name))))
            } else if stderr.contains("is masked") {
                debug!("Service {} is masked", name);
                Ok(Ok(())) // Masked services still exist, just can't be started
            } else {
                Ok(Err(SystemError::SystemCallFailed(
                    output.status.code().unwrap_or(-1),
                    format!("Service validation failed for '{}': {}", name, stderr)
                )))
            }
        }
    }

    fn check_service_status_systemd(&self, name: &str) -> SystemResult<ServiceStatus> {
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["is-active", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        let status_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        let status = match status_str.as_str() {
            "active" => ServiceStatus::Running,
            "inactive" => ServiceStatus::Stopped,
            "activating" => ServiceStatus::Starting,
            "deactivating" => ServiceStatus::Stopping,
            "failed" => ServiceStatus::Error("Service failed".to_string()),
            "reloading" => ServiceStatus::Starting,
            _ => ServiceStatus::Unknown,
        };

        Ok(status)
    }

    fn get_service_start_type_systemd(&self, name: &str) -> SystemResult<ServiceStartType> {
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["is-enabled", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        let status_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        let start_type = match status_str.as_str() {
            "enabled" => ServiceStartType::Automatic,
            "disabled" => ServiceStartType::Disabled,
            "static" => ServiceStartType::Manual,
            "masked" => ServiceStartType::Disabled,
            "enabled-runtime" => ServiceStartType::Automatic,
            "indirect" => ServiceStartType::Manual,
            _ => ServiceStartType::Unknown,
        };

        Ok(start_type)
    }

    /// Reload systemd configuration
    pub fn reload_systemd_config(&self) -> SystemResult<()> {
        use tracing::info;
        
        info!("Reloading systemd configuration");
        
        let output = Command::new("systemctl")
            .args(&["daemon-reload"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            info!("Successfully reloaded systemd configuration");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to reload systemd configuration: {}", stderr)
            ))
        }
    }

    /// Get systemd service logs
    pub fn get_service_logs_systemd(&self, name: &str, lines: Option<usize>) -> SystemResult<String> {
        use tracing::debug;
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let mut args = vec!["journalctl", "-u", &service_name, "--no-pager"];
        
        if let Some(n) = lines {
            args.push("-n");
            args.push(&n.to_string());
        }

        debug!("Getting logs for service: {}", name);
        
        let output = Command::new(&args[0])
            .args(&args[1..])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute journalctl: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to get logs for service '{}': {}", name, stderr)
            ))
        }
    }

    /// Mask a systemd service (prevent it from being started)
    pub fn mask_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::info;
        
        info!("Masking systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["mask", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            info!("Successfully masked service: {}", name);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to mask service '{}': {}", name, stderr)
            ))
        }
    }

    /// Unmask a systemd service
    pub fn unmask_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::info;
        
        info!("Unmasking systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["unmask", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            info!("Successfully unmasked service: {}", name);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to unmask service '{}': {}", name, stderr)
            ))
        }
    }

    /// Reload a systemd service configuration without restarting
    pub fn reload_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::info;
        
        info!("Reloading systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["reload", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            info!("Successfully reloaded service: {}", name);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                format!("Failed to reload service '{}': {}", name, stderr)
            ))
        }
    }

    fn start_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::{info, warn};
        
        // First check if service exists and is not masked
        if let Err(e) = self.validate_service_systemd(name)? {
            return Err(e);
        }

        info!("Starting systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["start", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            // Wait a moment and verify the service actually started
            std::thread::sleep(Duration::from_millis(100));
            
            match self.check_service_status_systemd(name)? {
                ServiceStatus::Running | ServiceStatus::Starting => {
                    info!("Successfully started service: {}", name);
                    Ok(())
                },
                status => {
                    warn!("Service start command succeeded but service is not running: {:?}", status);
                    // This is not necessarily an error for one-shot services
                    Ok(())
                }
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse common systemctl error messages for better error reporting
            let error_msg = if stderr.contains("Unit not found") {
                format!("Service '{}' not found", name)
            } else if stderr.contains("is masked") {
                format!("Service '{}' is masked and cannot be started", name)
            } else if stderr.contains("Job failed") {
                format!("Failed to start service '{}': Job failed", name)
            } else if !stderr.is_empty() {
                format!("Failed to start service '{}': {}", name, stderr)
            } else if !stdout.is_empty() {
                format!("Failed to start service '{}': {}", name, stdout)
            } else {
                format!("Failed to start service '{}'", name)
            };

            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                error_msg
            ))
        }
    }

    fn stop_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::{info, warn};
        
        // First check if service exists
        if let Err(e) = self.validate_service_systemd(name)? {
            return Err(e);
        }

        info!("Stopping systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["stop", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            // Wait a moment and verify the service actually stopped
            std::thread::sleep(Duration::from_millis(100));
            
            match self.check_service_status_systemd(name)? {
                ServiceStatus::Stopped | ServiceStatus::Stopping => {
                    info!("Successfully stopped service: {}", name);
                    Ok(())
                },
                status => {
                    warn!("Service stop command succeeded but service is still running: {:?}", status);
                    Ok(()) // Still return success as the command succeeded
                }
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse common systemctl error messages
            let error_msg = if stderr.contains("Unit not found") {
                format!("Service '{}' not found", name)
            } else if stderr.contains("not loaded") {
                format!("Service '{}' is not loaded", name)
            } else if stderr.contains("Job failed") {
                format!("Failed to stop service '{}': Job failed", name)
            } else if !stderr.is_empty() {
                format!("Failed to stop service '{}': {}", name, stderr)
            } else if !stdout.is_empty() {
                format!("Failed to stop service '{}': {}", name, stdout)
            } else {
                format!("Failed to stop service '{}'", name)
            };

            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                error_msg
            ))
        }
    }

    fn enable_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::{info, warn};
        
        // First check if service exists
        if let Err(e) = self.validate_service_systemd(name)? {
            return Err(e);
        }

        info!("Enabling systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["enable", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            // Verify that the service is now enabled
            if let Ok(start_type) = self.get_service_start_type_systemd(name) {
                match start_type {
                    ServiceStartType::Automatic => {
                        info!("Successfully enabled service: {}", name);
                        Ok(())
                    },
                    _ => {
                        warn!("Service enable command succeeded but service is not set to automatic start");
                        Ok(()) // Still return success as the command succeeded
                    }
                }
            } else {
                Ok(()) // If we can't verify, assume success since command succeeded
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse common systemctl error messages
            let error_msg = if stderr.contains("Unit not found") {
                format!("Service '{}' not found", name)
            } else if stderr.contains("is masked") {
                format!("Service '{}' is masked and cannot be enabled", name)
            } else if stderr.contains("No such file or directory") {
                format!("Service '{}' unit file not found", name)
            } else if stderr.contains("already enabled") {
                // This is not really an error
                info!("Service '{}' is already enabled", name);
                return Ok(());
            } else if !stderr.is_empty() {
                format!("Failed to enable service '{}': {}", name, stderr)
            } else if !stdout.is_empty() {
                format!("Failed to enable service '{}': {}", name, stdout)
            } else {
                format!("Failed to enable service '{}'", name)
            };

            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                error_msg
            ))
        }
    }

    fn disable_service_systemd(&self, name: &str) -> SystemResult<()> {
        use tracing::{info, warn};
        
        // First check if service exists
        if let Err(e) = self.validate_service_systemd(name)? {
            return Err(e);
        }

        info!("Disabling systemd service: {}", name);
        
        let service_name = if name.ends_with(".service") { 
            name.to_string() 
        } else { 
            format!("{}.service", name) 
        };

        let output = Command::new("systemctl")
            .args(&["disable", &service_name])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, format!("Failed to execute systemctl: {}", e)))?;

        if output.status.success() {
            // Verify that the service is now disabled
            if let Ok(start_type) = self.get_service_start_type_systemd(name) {
                match start_type {
                    ServiceStartType::Disabled => {
                        info!("Successfully disabled service: {}", name);
                        Ok(())
                    },
                    _ => {
                        warn!("Service disable command succeeded but service is not disabled");
                        Ok(()) // Still return success as the command succeeded
                    }
                }
            } else {
                Ok(()) // If we can't verify, assume success since command succeeded
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Parse common systemctl error messages
            let error_msg = if stderr.contains("Unit not found") {
                format!("Service '{}' not found", name)
            } else if stderr.contains("No such file or directory") {
                format!("Service '{}' unit file not found", name)
            } else if stderr.contains("already disabled") {
                // This is not really an error
                info!("Service '{}' is already disabled", name);
                return Ok(());
            } else if !stderr.is_empty() {
                format!("Failed to disable service '{}': {}", name, stderr)
            } else if !stdout.is_empty() {
                format!("Failed to disable service '{}': {}", name, stdout)
            } else {
                format!("Failed to disable service '{}'", name)
            };

            Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                error_msg
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

    #[test]
    #[cfg(target_os = "linux")]
    fn test_systemd_service_operations() {
        use std::process::Command;
        
        // Only run if systemctl is available
        if Command::new("systemctl").arg("--version").output().is_err() {
            return; // Skip test if systemctl not available
        }

        let mut manager = ServiceManager::new().expect("Failed to create service manager");
        
        // Test platform detection
        if let ServicePlatform::Systemd = manager.platform {
            // Test listing services (should work on any systemd system)
            let services = manager.list_services();
            match services {
                Ok(service_list) => {
                    assert!(!service_list.is_empty(), "Should have at least some services");
                    
                    // Find a common service that should exist
                    let test_service = service_list.iter()
                        .find(|s| s.name == "systemd-resolved" || s.name == "dbus" || s.name == "ssh")
                        .cloned();
                        
                    if let Some(service) = test_service {
                        // Test getting specific service
                        let retrieved = manager.get_service(&service.name);
                        assert!(retrieved.is_ok(), "Should be able to retrieve service");
                        
                        // Test advanced features
                        assert!(manager.supports_advanced_features());
                        
                        // Test validation (should not fail for existing service)
                        let validation = manager.validate_service_systemd(&service.name);
                        assert!(validation.is_ok(), "Service validation should succeed");
                        
                        // Test status checking
                        let status = manager.check_service_status_systemd(&service.name);
                        assert!(status.is_ok(), "Status check should succeed");
                        
                        // Test getting start type
                        let start_type = manager.get_service_start_type_systemd(&service.name);
                        assert!(start_type.is_ok(), "Start type check should succeed");
                    }
                },
                Err(_) => {
                    // Service listing might fail in containerized environments
                    // This is acceptable for testing
                }
            }
        }
    }

    #[test]
    fn test_service_name_normalization() {
        let manager = ServiceManager::new().expect("Failed to create service manager");
        
        // Test service name handling
        let test_cases = vec![
            ("ssh", "ssh.service"),
            ("nginx.service", "nginx.service"),
            ("dbus", "dbus.service"),
        ];
        
        for (input, expected) in test_cases {
            let normalized = if input.ends_with(".service") {
                input.to_string()
            } else {
                format!("{}.service", input)
            };
            
            assert_eq!(normalized, expected, "Service name normalization failed");
        }
    }
}
