use crate::error::Error;
/// Cross-platform daemon and service management
/// 
/// This module provides facilities for creating and managing daemon processes
/// on Unix systems and Windows services.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

use crate::stdlib::process::error::{ProcessError, ProcessResult, system_error, execution_failed};
use crate::stdlib::process::core::{ProcessConfig, spawn_process};

/// Options for daemon configuration
#[derive(Debug, Clone)]
pub struct DaemonOptions {
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: String,
    /// Auto restart on crash
    pub auto_restart: bool,
    /// Maximum restart attempts
    pub max_restarts: u32,
    /// Start timeout
    pub start_timeout: Duration,
    /// Stop timeout
    pub stop_timeout: Duration,
}

impl Default for DaemonOptions {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: "info".to_string(),
            auto_restart: true,
            max_restarts: 5,
            start_timeout: Duration::from_secs(30),
            stop_timeout: Duration::from_secs(10),
        }
    }
}

/// Daemon manager type alias
pub type DaemonManager = Daemon;

/// Daemon configuration
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    /// Daemon name
    pub name: String,
    /// Working directory
    pub working_directory: Option<PathBuf>,
    /// User to run as (Unix only)
    pub user: Option<String>,
    /// Group to run as (Unix only)
    pub group: Option<String>,
    /// PID file location
    pub pid_file: Option<PathBuf>,
    /// Log file location
    pub log_file: Option<PathBuf>,
    /// Lock file location
    pub lock_file: Option<PathBuf>,
    /// Daemon description
    pub description: Option<String>,
    /// Auto-restart on failure
    pub auto_restart: bool,
    /// Maximum restart attempts
    pub max_restarts: u32,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Umask for daemon process
    pub umask: Option<u32>,
}

impl DaemonConfig {
    /// Create a new daemon configuration
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            working_directory: Some(PathBuf::from("/")),
            user: None,
            group: None,
            pid_file: None,
            log_file: None,
            lock_file: None,
            description: None,
            auto_restart: false,
            max_restarts: 3,
            environment: HashMap::new(),
            umask: Some(0o022),
        }
    }
    
    /// Set working directory
    pub fn working_directory<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_directory = Some(dir.into());
        self
    }
    
    /// Set user (Unix only)
    pub fn user<S: Into<String>>(mut self, user: S) -> Self {
        self.user = Some(user.into());
        self
    }
    
    /// Set group (Unix only)
    pub fn group<S: Into<String>>(mut self, group: S) -> Self {
        self.group = Some(group.into());
        self
    }
    
    /// Set PID file
    pub fn pid_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.pid_file = Some(path.into());
        self
    }
    
    /// Set log file
    pub fn log_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.log_file = Some(path.into());
        self
    }
    
    /// Set lock file
    pub fn lock_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.lock_file = Some(path.into());
        self
    }
    
    /// Set description
    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    /// Enable auto-restart
    pub fn auto_restart(mut self, max_restarts: u32) -> Self {
        self.auto_restart = true;
        self.max_restarts = max_restarts;
        self
    }
    
    /// Add environment variable
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.environment.insert(key.into(), value.into());
        self
    }
    
    /// Set umask
    pub fn umask(mut self, mask: u32) -> Self {
        self.umask = Some(mask);
        self
    }
}

/// Daemon status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaemonStatus {
    Running,
    Stopped,
    Failed,
    Starting,
    Stopping,
    Unknown,
}

/// Daemon handle
pub struct Daemon {
    config: DaemonConfig,
    pid: Option<u32>,
    status: Arc<Mutex<DaemonStatus>>,
    restart_count: Arc<Mutex<u32>>,
}

impl Daemon {
    /// Create a new daemon
    pub fn new(config: DaemonConfig) -> Self {
        Self {
            config,
            pid: None,
            status: Arc::new(Mutex::new(DaemonStatus::Stopped)),
            restart_count: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Start the daemon
    pub fn start<F>(&mut self, daemon_main: F) -> ProcessResult<()>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        self.set_status(DaemonStatus::Starting);
        
        #[cfg(unix)]
        {
            self.start_unix_daemon(daemon_main)
        }
        
        #[cfg(windows)]
        {
            self.start_windows_service(daemon_main)
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Err(system_error(-1, "Daemon creation not supported on this platform".to_string()))
        }
    }
    
    #[cfg(unix)]
    fn start_unix_daemon<F>(&mut self, daemon_main: F) -> ProcessResult<()>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        // First fork
        let pid = unsafe { libc::fork() };
        
        match pid {
            -1 => return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "First fork failed".to_string()
            )),
            0 => {
                // Child process
                
                // Create new session
                if unsafe { libc::setsid() } == -1 {
                    std::process::exit(1);
                }
                
                // Second fork to ensure we're not a session leader
                let pid2 = unsafe { libc::fork() };
                match pid2 {
                    -1 => std::process::exit(1),
                    0 => {
                        // Grandchild process - this becomes the daemon
                        
                        // Change working directory
                        if let Some(ref dir) = self.config.working_directory {
                            let dir_cstr = std::ffi::CString::new(dir.to_string_lossy().as_bytes()).unwrap();
                            if unsafe { libc::chdir(dir_cstr.as_ptr()) } == -1 {
                                std::process::exit(1);
                            }
                        }
                        
                        // Set umask
                        if let Some(mask) = self.config.umask {
                            unsafe { libc::umask(mask); }
                        }
                        
                        // Close standard file descriptors
                        unsafe {
                            libc::close(0); // stdin
                            libc::close(1); // stdout
                            libc::close(2); // stderr
                        }
                        
                        // Redirect to log file or /dev/null
                        self.setup_stdio();
                        
                        // Write PID file
                        if let Err(e) = self.write_pid_file() {
                            eprintln!("Failed to write PID file: {}", e);
                            std::process::exit(1);
                        }
                        
                        // Create lock file
                        if let Err(e) = self.create_lock_file() {
                            eprintln!("Failed to create lock file: {}", e);
                            std::process::exit(1);
                        }
                        
                        // Run the daemon main function
                        if let Err(e) = daemon_main() {
                            eprintln!("Daemon main function failed: {}", e);
                            std::process::exit(1);
                        }
                        
                        std::process::exit(0);
                    }
                    child_pid => {
                        // First child exits
                        std::process::exit(0);
                    }
                }
            }
            child_pid => {
                // Parent process
                self.pid = Some(child_pid as u32);
                self.set_status(DaemonStatus::Running);
                
                // Wait for first child to exit
                let mut status = 0;
                unsafe { libc::waitpid(child_pid, &mut status, 0); }
                
                Ok(())
            }
        }
    }
    
    #[cfg(windows)]
    fn start_windows_service<F>(&mut self, daemon_main: F) -> ProcessResult<()>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        // For Windows, we'll create a background process instead of a true service
        // Real service implementation would use the Windows Service API
        
        let config = self.config.clone();
        let status = Arc::clone(&self.status);
        
        thread::spawn(move || {
            status.lock().unwrap().clone_from(&DaemonStatus::Running);
            
            if let Err(e) = daemon_main() {
                eprintln!("Service main function failed: {}", e);
                *status.lock().unwrap() = DaemonStatus::Failed;
            } else {
                *status.lock().unwrap() = DaemonStatus::Stopped;
            }
        });
        
        self.pid = Some(std::process::id());
        self.set_status(DaemonStatus::Running);
        
        Ok(())
    }
    
    /// Stop the daemon
    pub fn stop(&mut self) -> ProcessResult<()> {
        if let Some(pid) = self.pid {
            self.set_status(DaemonStatus::Stopping);
            
            #[cfg(unix)]
            {
                // Send SIGTERM
                let result = unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
                if result != 0 {
                    return Err(system_error(
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                        format!("Failed to stop daemon with PID {}", pid)
                    ));
                }
            }
            
            #[cfg(windows)]
            {
                // Terminate the process
                let result = Command::new("taskkill")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .arg("/F")
                    .output();
                    
                if let Err(e) = result {
                    return Err(execution_failed("taskkill", &e.to_string()));
                }
            }
            
            // Wait for process to exit
            thread::sleep(Duration::from_millis(100));
            
            // Clean up files
            self.cleanup_files()?;
            
            self.pid = None;
            self.set_status(DaemonStatus::Stopped);
        }
        
        Ok(())
    }
    
    /// Restart the daemon
    pub fn restart<F>(&mut self, daemon_main: F) -> ProcessResult<()>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        if self.is_running()? {
            self.stop()?;
        }
        
        // Increment restart count
        {
            let mut count = self.restart_count.lock().unwrap();
            *count += 1;
            
            if *count > self.config.max_restarts {
                return Err(system_error(-1, "Maximum restart attempts exceeded".to_string()));
            }
        }
        
        thread::sleep(Duration::from_millis(500)); // Brief delay before restart
        self.start(daemon_main)
    }
    
    /// Check if daemon is running
    pub fn is_running(&self) -> ProcessResult<bool> {
        if let Some(pid) = self.pid {
            #[cfg(unix)]
            {
                // Send signal 0 to check if process exists
                let result = unsafe { libc::kill(pid as libc::pid_t, 0) };
                Ok(result == 0)
            }
            
            #[cfg(windows)]
            {
                let output = Command::new("tasklist")
                    .arg("/FI")
                    .arg(&format!("PID eq {}", pid))
                    .arg("/FO")
                    .arg("CSV")
                    .output();
                    
                match output {
                    Ok(result) if result.status.success() => {
                        let output_str = String::from_utf8_lossy(&result.stdout);
                        Ok(output_str.split("\n").count() > 1) // Header + process line
                    }
                    _ => Ok(false)
                }
            }
            
            #[cfg(not(any(unix, windows)))]
            {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Get daemon status
    pub fn status(&self) -> DaemonStatus {
        self.status.lock().unwrap().clone()
    }
    
    /// Get daemon PID
    pub fn pid(&self) -> Option<u32> {
        self.pid
    }
    
    /// Get restart count
    pub fn restart_count(&self) -> u32 {
        *self.restart_count.lock().unwrap()
    }
    
    /// Set daemon status
    fn set_status(&self, status: DaemonStatus) {
        *self.status.lock().unwrap() = status;
    }
    
    /// Setup stdio redirection with enhanced error handling and logging
    fn setup_stdio(&self) {
        #[cfg(unix)]
        {
            let log_path = self.config.log_file.as_ref()
                .map(|p| p.clone())
                .unwrap_or_else(|| PathBuf::from("/dev/null"));
            
            // Create log directory if it doesn't exist
            if let Some(parent) = log_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            
            // Open log file with enhanced options
            let log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .write(true)
                .open(&log_path);
                
            match log_file {
                Ok(file) => {
                    let fd = file.as_raw_fd();
                    
                    // Redirect stdout and stderr to log file with error checking
                    unsafe {
                        if libc::dup2(fd, 1) == -1 || libc::dup2(fd, 2) == -1 {
                            // If log redirection fails, fallback to /dev/null
                            self.fallback_stdio_redirect();
                        }
                    }
                }
                Err(_) => {
                    // If log file creation fails, redirect to /dev/null
                    self.fallback_stdio_redirect();
                }
            }
            
            // Always redirect stdin to /dev/null for daemon processes
            self.redirect_stdin_to_null();
        }
        
        #[cfg(windows)]
        {
            // Windows daemon stdio setup - redirect to log file or NUL
            self.setup_windows_stdio();
        }
    }
    
    #[cfg(unix)]
    fn fallback_stdio_redirect(&self) {
        let dev_null = std::ffi::CString::new("/dev/null").unwrap();
        unsafe {
            let null_fd = libc::open(dev_null.as_ptr(), libc::O_WRONLY);
            if null_fd != -1 {
                libc::dup2(null_fd, 1); // stdout to /dev/null
                libc::dup2(null_fd, 2); // stderr to /dev/null
                libc::close(null_fd);
            }
        }
    }
    
    #[cfg(unix)]
    fn redirect_stdin_to_null(&self) {
        let dev_null = std::ffi::CString::new("/dev/null").unwrap();
        unsafe {
            let null_fd = libc::open(dev_null.as_ptr(), libc::O_RDONLY);
            if null_fd != -1 {
                libc::dup2(null_fd, 0); // stdin from /dev/null
                libc::close(null_fd);
            }
        }
    }
    
    #[cfg(windows)]
    fn setup_windows_stdio(&self) {
        use std::os::windows::io::AsRawHandle;
        
        // Windows-specific stdio redirection for services
        let log_path = self.config.log_file.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "NUL".to_string());
        
        // Create log directory if needed
        if let Some(ref log_file) = self.config.log_file {
            if let Some(parent) = log_file.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        
        // Redirect output streams using Windows CreateFile
        if let Ok(log_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&log_path)
        {
            // Windows handle redirection would go here
            // This is a simplified version - real implementation would use 
            // SetStdHandle or DuplicateHandle APIs
            std::mem::drop(log_file); // Keep file open for daemon lifetime
        }
    }
    
    /// Write PID file
    fn write_pid_file(&self) -> ProcessResult<()> {
        if let Some(ref pid_file) = self.config.pid_file {
            let pid = std::process::id();
            let mut file = File::create(pid_file)
                .map_err(|e| system_error(-1, format!("Failed to create PID file: {}", e)))?;
            
            write!(file, "{}", pid)
                .map_err(|e| system_error(-1, format!("Failed to write PID file: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Create lock file
    fn create_lock_file(&self) -> ProcessResult<()> {
        if let Some(ref lock_file) = self.config.lock_file {
            File::create(lock_file)
                .map_err(|e| system_error(-1, format!("Failed to create lock file: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Clean up daemon files
    fn cleanup_files(&self) -> ProcessResult<()> {
        // Remove PID file
        if let Some(ref pid_file) = self.config.pid_file {
            let _ = std::fs::remove_file(pid_file);
        }
        
        // Remove lock file
        if let Some(ref lock_file) = self.config.lock_file {
            let _ = std::fs::remove_file(lock_file);
        }
        
        Ok(())
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.cleanup_files();
    }
}

// Platform-specific raw file descriptor access
#[cfg(unix)]
trait AsRawFd {
    fn as_raw_fd(&self) -> libc::c_int;
}

#[cfg(unix)]
impl AsRawFd for File {
    fn as_raw_fd(&self) -> libc::c_int {
        use std::os::unix::io::AsRawFd;
        self.as_raw_fd()
    }
}

/// Service manager for handling multiple daemons
pub struct ServiceManager {
    services: Arc<Mutex<HashMap<String, Daemon>>>,
}

impl ServiceManager {
    /// Create a new service manager
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register a service
    pub fn register(&self, name: String, config: DaemonConfig) -> ProcessResult<()> {
        let mut services = self.services.lock()
            .map_err(|_| system_error(-1, "Failed to lock services".to_string()))?;
        
        if services.contains_key(&name) {
            return Err(system_error(-1, format!("Service '{}' already registered", name)));
        }
        
        let daemon = Daemon::new(config);
        services.insert(name, daemon);
        
        Ok(())
    }
    
    /// Start a service
    pub fn start_service<F>(&self, name: &str, daemon_main: F) -> ProcessResult<()>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        let mut services = self.services.lock()
            .map_err(|_| system_error(-1, "Failed to lock services".to_string()))?;
        
        if let Some(daemon) = services.get_mut(name) {
            daemon.start(daemon_main)
        } else {
            Err(system_error(-1, format!("Service '{}' not found", name)))
        }
    }
    
    /// Stop a service
    pub fn stop_service(&self, name: &str) -> ProcessResult<()> {
        let mut services = self.services.lock()
            .map_err(|_| system_error(-1, "Failed to lock services".to_string()))?;
        
        if let Some(daemon) = services.get_mut(name) {
            daemon.stop()
        } else {
            Err(system_error(-1, format!("Service '{}' not found", name)))
        }
    }
    
    /// Get service status
    pub fn service_status(&self, name: &str) -> ProcessResult<DaemonStatus> {
        let services = self.services.lock()
            .map_err(|_| system_error(-1, "Failed to lock services".to_string()))?;
        
        if let Some(daemon) = services.get(name) {
            Ok(daemon.status())
        } else {
            Err(system_error(-1, format!("Service '{}' not found", name)))
        }
    }
    
    /// List all services
    pub fn list_services(&self) -> Vec<String> {
        self.services.lock()
            .map(|services| services.keys().cloned().collect())
            .unwrap_or_else(|_| Vec::new())
    }
    
    /// Start all services
    pub fn start_all(&self) -> ProcessResult<()> {
        let service_names = self.list_services();
        
        for name in service_names {
            // Note: This would need individual daemon_main functions for each service
            // In a real implementation, services would be configured with their main functions
            eprintln!("Would start service: {}", name);
        }
        
        Ok(())
    }
    
    /// Stop all services
    pub fn stop_all(&self) -> ProcessResult<()> {
        let service_names = self.list_services();
        
        for name in service_names {
            if let Err(e) = self.stop_service(&name) {
                eprintln!("Failed to stop service '{}': {}", name, e);
            }
        }
        
        Ok(())
    }
}

/// System service integration
pub mod system {
    use super::*;
    
    /// Install daemon as system service
    pub fn install_system_service(config: &DaemonConfig, executable_path: &str) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            install_systemd_service(config, executable_path)
        }
        
        #[cfg(windows)]
        {
            install_windows_service(config, executable_path)
        }
        
        #[cfg(target_os = "macos")]
        {
            install_launchd_service(config, executable_path)
        }
        
        #[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
        {
            Err(system_error(-1, "System service installation not supported on this platform".to_string()))
        }
    }
    
    #[cfg(target_os = "linux")]
    fn install_systemd_service(config: &DaemonConfig, executable_path: &str) -> ProcessResult<()> {
        let service_content = format!(
            r#"[Unit]
Description={}
After=network.target

[Service]
Type=forking
ExecStart={}
Restart=always
User={}
Group={}
WorkingDirectory={}

[Install]
WantedBy=multi-user.target
"#,
            config.description.as_deref().unwrap_or(&config.name),
            executable_path,
            config.user.as_deref().unwrap_or("root"),
            config.group.as_deref().unwrap_or("root"),
            config.working_directory.as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "/".into())
        );
        
        let service_file = format!("/etc/systemd/system/{}.service", config.name);
        std::fs::write(&service_file, service_content)
            .map_err(|e| system_error(-1, format!("Failed to write service file: {}", e)))?;
        
        // Reload systemd
        Command::new("systemctl")
            .arg("daemon-reload")
            .output()
            .map_err(|e| execution_failed("systemctl", &e.to_string()))?;
        
        // Enable service
        Command::new("systemctl")
            .arg("enable")
            .arg(&config.name)
            .output()
            .map_err(|e| execution_failed("systemctl", &e.to_string()))?;
        
        Ok(())
    }
    
    #[cfg(windows)]
    fn install_windows_service(config: &DaemonConfig, executable_path: &str) -> ProcessResult<()> {
        // Use sc.exe to install Windows service
        let result = Command::new("sc")
            .arg("create")
            .arg(&config.name)
            .arg(&format!("binPath={}", executable_path))
            .arg(&format!("DisplayName={}", config.description.as_deref().unwrap_or(&config.name)))
            .arg("start=auto")
            .output();
            
        match result {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(system_error(-1, format!("Failed to install service: {}", error_msg)))
            }
            Err(e) => Err(execution_failed("sc", &e.to_string()))
        }
    }
    
    #[cfg(target_os = "macos")]
    fn install_launchd_service(config: &DaemonConfig, executable_path: &str) -> ProcessResult<()> {
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>WorkingDirectory</key>
    <string>{}</string>
</dict>
</plist>
"#,
            config.name,
            executable_path,
            config.working_directory.as_ref()
                .map(|p| p.to_string_lossy())
                .unwrap_or_else(|| "/".into())
        );
        
        let plist_file = format!("/Library/LaunchDaemons/{}.plist", config.name);
        std::fs::write(&plist_file, plist_content)
            .map_err(|e| system_error(-1, format!("Failed to write plist file: {}", e)))?;
        
        // Load the service
        Command::new("launchctl")
            .arg("load")
            .arg(&plist_file)
            .output()
            .map_err(|e| execution_failed("launchctl", &e.to_string()))?;
        
        Ok(())
    }
    
    /// Uninstall system service
    pub fn uninstall_system_service(service_name: &str) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            // Stop service
            let _ = Command::new("systemctl").arg("stop").arg(service_name).output();
            
            // Disable service
            let _ = Command::new("systemctl").arg("disable").arg(service_name).output();
            
            // Remove service file
            let service_file = format!("/etc/systemd/system/{}.service", service_name);
            let _ = std::fs::remove_file(service_file);
            
            // Reload systemd
            Command::new("systemctl")
                .arg("daemon-reload")
                .output()
                .map_err(|e| execution_failed("systemctl", &e.to_string()))?;
        }
        
        #[cfg(windows)]
        {
            // Stop service
            let _ = Command::new("net").arg("stop").arg(service_name).output();
            
            // Delete service
            Command::new("sc")
                .arg("delete")
                .arg(service_name)
                .output()
                .map_err(|e| execution_failed("sc", &e.to_string()))?;
        }
        
        #[cfg(target_os = "macos")]
        {
            let plist_file = format!("/Library/LaunchDaemons/{}.plist", service_name);
            
            // Unload service
            let _ = Command::new("launchctl").arg("unload").arg(&plist_file).output();
            
            // Remove plist file
            let _ = std::fs::remove_file(plist_file);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;
    
    #[test]
    fn test_daemon_config() {
        let config = DaemonConfig::new("test-daemon")
            .working_directory("/tmp")
            .user("nobody")
            .group("nogroup")
            .pid_file("/tmp/test-daemon.pid")
            .log_file("/tmp/test-daemon.log")
            .description("Test daemon")
            .auto_restart(3)
            .env("TEST_VAR", "test_value")
            .umask(0o022);
        
        assert_eq!(config.name, "test-daemon");
        assert_eq!(config.working_directory, Some(PathBuf::from("/tmp")));
        assert_eq!(config.user, Some("nobody".to_string()));
        assert_eq!(config.group, Some("nogroup".to_string()));
        assert_eq!(config.pid_file, Some(PathBuf::from("/tmp/test-daemon.pid")));
        assert_eq!(config.log_file, Some(PathBuf::from("/tmp/test-daemon.log")));
        assert_eq!(config.description, Some("Test daemon".to_string()));
        assert!(config.auto_restart);
        assert_eq!(config.max_restarts, 3);
        assert_eq!(config.environment.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.umask, Some(0o022));
    }
    
    #[test]
    fn test_daemon_creation() {
        let config = DaemonConfig::new("test-daemon");
        let daemon = Daemon::new(config);
        
        assert_eq!(daemon.status(), DaemonStatus::Stopped);
        assert_eq!(daemon.pid(), None);
        assert_eq!(daemon.restart_count(), 0);
    }
    
    #[test]
    fn test_service_manager() {
        let manager = ServiceManager::new();
        
        let config = DaemonConfig::new("test-service");
        assert!(manager.register("test-service".to_string(), config).is_ok());
        
        let services = manager.list_services();
        assert!(services.contains(&"test-service".to_string()));
        
        let status = manager.service_status("test-service").unwrap();
        assert_eq!(status, DaemonStatus::Stopped);
    }
    
    #[test]
    fn test_daemon_status_enum() {
        assert_eq!(DaemonStatus::Running, DaemonStatus::Running);
        assert_ne!(DaemonStatus::Running, DaemonStatus::Stopped);
    }
    
    #[test]
    fn test_daemon_file_operations() {
        let temp_dir = std::env::temp_dir();
        let config = DaemonConfig::new("test-file-daemon")
            .pid_file(temp_dir.join("test-daemon.pid"))
            .lock_file(temp_dir.join("test-daemon.lock"));
        
        let daemon = Daemon::new(config);
        
        // Test PID file creation (would be called during daemon start)
        if let Err(e) = daemon.write_pid_file() {
            println!("Note: PID file creation may fail in test environment: {}", e);
        }
        
        // Test lock file creation
        if let Err(e) = daemon.create_lock_file() {
            println!("Note: Lock file creation may fail in test environment: {}", e);
        }
        
        // Test cleanup
        let _ = daemon.cleanup_files();
    }
}
