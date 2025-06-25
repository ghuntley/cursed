/// Platform-Specific Operations Module for CURSED
/// 
/// Provides platform-specific functionality for Windows, macOS, and Linux
/// including system information, platform features, native API access,
/// and cross-platform abstractions for system-level operations.

use std::collections::HashMap;
use std::ffi::{OsString, OsStr};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::fs;
use std::io;

// =============================================================================
// PLATFORM DETECTION AND INFORMATION
// =============================================================================

/// Target platform enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Solaris,
    Android,
    iOS,
    Unknown,
}

impl Platform {
    /// Gets the current platform
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;
        
        #[cfg(target_os = "macos")]
        return Platform::MacOS;
        
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        
        #[cfg(target_os = "freebsd")]
        return Platform::FreeBSD;
        
        #[cfg(target_os = "openbsd")]
        return Platform::OpenBSD;
        
        #[cfg(target_os = "netbsd")]
        return Platform::NetBSD;
        
        #[cfg(target_os = "solaris")]
        return Platform::Solaris;
        
        #[cfg(target_os = "android")]
        return Platform::Android;
        
        #[cfg(target_os = "ios")]
        return Platform::iOS;
        
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos", 
            target_os = "linux",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "solaris",
            target_os = "android",
            target_os = "ios"
        )))]
        return Platform::Unknown;
    }
    
    /// Gets platform name as string
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::MacOS => "macOS",
            Platform::Linux => "Linux",
            Platform::FreeBSD => "FreeBSD",
            Platform::OpenBSD => "OpenBSD",
            Platform::NetBSD => "NetBSD",
            Platform::Solaris => "Solaris",
            Platform::Android => "Android",
            Platform::iOS => "iOS",
            Platform::Unknown => "Unknown",
        }
    }
    
    /// Checks if platform is Unix-like
    pub fn is_unix(&self) -> bool {
        matches!(
            self,
            Platform::MacOS
                | Platform::Linux
                | Platform::FreeBSD
                | Platform::OpenBSD
                | Platform::NetBSD
                | Platform::Solaris
                | Platform::Android
        )
    }
    
    /// Checks if platform is Windows
    pub fn is_windows(&self) -> bool {
        *self == Platform::Windows
    }
    
    /// Checks if platform supports POSIX
    pub fn is_posix(&self) -> bool {
        self.is_unix()
    }
    
    /// Gets the platform's path separator
    pub fn path_separator(&self) -> char {
        if self.is_windows() {
            '\\'
        } else {
            '/'
        }
    }
    
    /// Gets the platform's environment variable path separator
    pub fn env_path_separator(&self) -> char {
        if self.is_windows() {
            ';'
        } else {
            ':'
        }
    }
    
    /// Gets default shell for the platform
    pub fn default_shell(&self) -> &'static str {
        match self {
            Platform::Windows => "cmd.exe",
            Platform::MacOS | Platform::Linux => "/bin/bash",
            Platform::FreeBSD | Platform::OpenBSD | Platform::NetBSD => "/bin/sh",
            _ => "sh",
        }
    }
}

/// System information structure
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub platform: Platform,
    pub platform_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub architecture: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub uptime: u64,
    pub boot_time: u64,
}

impl SystemInfo {
    /// Gathers comprehensive system information
    pub fn gather() -> io::Result<Self> {
        let platform = Platform::current();
        
        Ok(Self {
            platform,
            platform_version: get_platform_version()?,
            kernel_version: get_kernel_version()?,
            hostname: get_hostname()?,
            architecture: get_architecture(),
            cpu_count: get_cpu_count(),
            total_memory: get_total_memory()?,
            available_memory: get_available_memory()?,
            uptime: get_uptime()?,
            boot_time: get_boot_time()?,
        })
    }
}

// =============================================================================
// SYSTEM INFORMATION FUNCTIONS
// =============================================================================

/// Gets platform version
fn get_platform_version() -> io::Result<String> {
    let platform = Platform::current();
    
    match platform {
        Platform::Windows => {
            // Use Windows version detection
            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                let output = Command::new("cmd")
                    .args(&["/c", "ver"])
                    .output()?;
                let version = String::from_utf8_lossy(&output.stdout);
                Ok(version.trim().to_string())
            }
            #[cfg(not(target_os = "windows"))]
            Ok("Windows (cross-compiled)".to_string())
        }
        Platform::MacOS => {
            #[cfg(target_os = "macos")]
            {
                let output = Command::new("sw_vers")
                    .arg("-productVersion")
                    .output()?;
                let version = String::from_utf8_lossy(&output.stdout);
                Ok(format!("macOS {}", version.trim()))
            }
            #[cfg(not(target_os = "macos"))]
            Ok("macOS (cross-compiled)".to_string())
        }
        Platform::Linux => {
            // Try to read from /etc/os-release
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                let mut name = "Linux";
                let mut version = "Unknown";
                
                for line in content.split("\n") {
                    if line.starts_with("NAME=") {
                        name = line.split('=').nth(1).unwrap_or("Linux").trim_matches('"');
                    } else if line.starts_with("VERSION=") {
                        version = line.split('=').nth(1).unwrap_or("Unknown").trim_matches('"');
                    }
                }
                
                Ok(format!("{} {}", name, version))
            } else {
                Ok("Linux".to_string())
            }
        }
        _ => Ok(platform.name().to_string()),
    }
}

/// Gets kernel version
fn get_kernel_version() -> io::Result<String> {
    let platform = Platform::current();
    
    if platform.is_unix() {
        let output = Command::new("uname")
            .arg("-r")
            .output()?;
        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    } else if platform.is_windows() {
        #[cfg(target_os = "windows")]
        {
            // Use Windows kernel version detection
            Ok("NT Kernel".to_string()) // Simplified
        }
        #[cfg(not(target_os = "windows"))]
        Ok("NT Kernel (cross-compiled)".to_string())
    } else {
        Ok("Unknown".to_string())
    }
}

/// Gets system hostname
fn get_hostname() -> io::Result<String> {
    let platform = Platform::current();
    
    if platform.is_unix() {
        let output = Command::new("hostname")
            .output()?;
        let hostname = String::from_utf8_lossy(&output.stdout);
        Ok(hostname.trim().to_string())
    } else if platform.is_windows() {
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("hostname")
                .output()?;
            let hostname = String::from_utf8_lossy(&output.stdout);
            Ok(hostname.trim().to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok("windows-host".to_string())
    } else {
        Ok("unknown-host".to_string())
    }
}

/// Gets system architecture
fn get_architecture() -> String {
    std::env::consts::ARCH.to_string()
}

/// Gets CPU count
fn get_cpu_count() -> usize {
    num_cpus::get()
}

/// Gets total system memory in bytes
fn get_total_memory() -> io::Result<u64> {
    let platform = Platform::current();
    
    match platform {
        Platform::Linux => {
            let content = fs::read_to_string("/proc/meminfo")?;
            for line in content.split("\n") {
                if line.starts_with("MemTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return Ok(kb * 1024); // Convert KB to bytes
                        }
                    }
                }
            }
            Ok(0)
        }
        Platform::MacOS => {
            #[cfg(target_os = "macos")]
            {
                let output = Command::new("sysctl")
                    .arg("-n")
                    .arg("hw.memsize")
                    .output()?;
                let mem_str = String::from_utf8_lossy(&output.stdout);
                mem_str.trim().parse().map_err(|_| {
                    std::io::Error::new(io::ErrorKind::InvalidData, "Failed to parse memory size")
                })
            }
            #[cfg(not(target_os = "macos"))]
            Ok(8 * 1024 * 1024 * 1024) // 8GB default
        }
        Platform::Windows => {
            #[cfg(target_os = "windows")]
            {
                // Use Windows memory detection
                Ok(8 * 1024 * 1024 * 1024) // Simplified: 8GB
            }
            #[cfg(not(target_os = "windows"))]
            Ok(8 * 1024 * 1024 * 1024)
        }
        _ => Ok(0),
    }
}

/// Gets available system memory in bytes
fn get_available_memory() -> io::Result<u64> {
    let platform = Platform::current();
    
    match platform {
        Platform::Linux => {
            let content = fs::read_to_string("/proc/meminfo")?;
            for line in content.split("\n") {
                if line.starts_with("MemAvailable:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return Ok(kb * 1024);
                        }
                    }
                }
            }
            Ok(0)
        }
        _ => {
            // For other platforms, estimate as 50% of total
            let total = get_total_memory()?;
            Ok(total / 2)
        }
    }
}

/// Gets system uptime in seconds
fn get_uptime() -> io::Result<u64> {
    let platform = Platform::current();
    
    match platform {
        Platform::Linux => {
            let content = fs::read_to_string("/proc/uptime")?;
            let uptime_str = content.split_whitespace().next().unwrap_or("0");
            let uptime_f: f64 = uptime_str.parse().map_err(|_| {
                std::io::Error::new(io::ErrorKind::InvalidData, "Failed to parse uptime")
            })?;
            Ok(uptime_f as u64)
        }
        Platform::MacOS => {
            #[cfg(target_os = "macos")]
            {
                let output = Command::new("sysctl")
                    .arg("-n")
                    .arg("kern.boottime")
                    .output()?;
                // Parse boot time and calculate uptime
                // Simplified implementation
                Ok(3600) // 1 hour placeholder
            }
            #[cfg(not(target_os = "macos"))]
            Ok(3600)
        }
        _ => Ok(0),
    }
}

/// Gets system boot time as Unix timestamp
fn get_boot_time() -> io::Result<u64> {
    let uptime = get_uptime()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Ok(now - uptime)
}

// =============================================================================
// PLATFORM-SPECIFIC FEATURES
// =============================================================================

/// Platform-specific feature flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformFeature {
    // File system features
    CaseSensitiveFileSystem,
    HardLinks,
    SymbolicLinks,
    ExtendedAttributes,
    FilePermissions,
    
    // Process features
    ProcessPriorities,
    SignalHandling,
    ForkSupport,
    ThreadSupport,
    
    // Network features
    UnixDomainSockets,
    RawSockets,
    NetworkNamespaces,
    
    // Security features
    Chroot,
    Setuid,
    Capabilities,
    Sandboxing,
    
    // System features
    VirtualMemory,
    SwapFiles,
    SystemCalls,
    KernelModules,
    
    // Desktop features
    SystemTray,
    WindowManager,
    DesktopNotifications,
    ClipboardAccess,
}

impl PlatformFeature {
    /// Checks if the current platform supports this feature
    pub fn is_supported(&self) -> bool {
        let platform = Platform::current();
        self.is_supported_on(platform)
    }
    
    /// Checks if a specific platform supports this feature
    pub fn is_supported_on(&self, platform: Platform) -> bool {
        match self {
            PlatformFeature::CaseSensitiveFileSystem => !platform.is_windows(),
            PlatformFeature::HardLinks => true,
            PlatformFeature::SymbolicLinks => true,
            PlatformFeature::ExtendedAttributes => platform.is_unix(),
            PlatformFeature::FilePermissions => platform.is_unix(),
            
            PlatformFeature::ProcessPriorities => true,
            PlatformFeature::SignalHandling => platform.is_unix(),
            PlatformFeature::ForkSupport => platform.is_unix(),
            PlatformFeature::ThreadSupport => true,
            
            PlatformFeature::UnixDomainSockets => platform.is_unix(),
            PlatformFeature::RawSockets => platform.is_unix(),
            PlatformFeature::NetworkNamespaces => platform == Platform::Linux,
            
            PlatformFeature::Chroot => platform.is_unix(),
            PlatformFeature::Setuid => platform.is_unix(),
            PlatformFeature::Capabilities => platform == Platform::Linux,
            PlatformFeature::Sandboxing => {
                matches!(platform, Platform::Linux | Platform::MacOS | Platform::Windows)
            }
            
            PlatformFeature::VirtualMemory => true,
            PlatformFeature::SwapFiles => !matches!(platform, Platform::iOS | Platform::Android),
            PlatformFeature::SystemCalls => platform.is_unix(),
            PlatformFeature::KernelModules => matches!(platform, Platform::Linux | Platform::FreeBSD),
            
            PlatformFeature::SystemTray => {
                matches!(platform, Platform::Windows | Platform::MacOS | Platform::Linux)
            }
            PlatformFeature::WindowManager => {
                matches!(platform, Platform::Linux | Platform::FreeBSD | Platform::OpenBSD)
            }
            PlatformFeature::DesktopNotifications => {
                matches!(platform, Platform::Windows | Platform::MacOS | Platform::Linux)
            }
            PlatformFeature::ClipboardAccess => {
                matches!(platform, Platform::Windows | Platform::MacOS | Platform::Linux)
            }
        }
    }
    
    /// Gets feature description
    pub fn description(&self) -> &'static str {
        match self {
            PlatformFeature::CaseSensitiveFileSystem => "Case-sensitive file system",
            PlatformFeature::HardLinks => "Hard link support",
            PlatformFeature::SymbolicLinks => "Symbolic link support",
            PlatformFeature::ExtendedAttributes => "Extended file attributes",
            PlatformFeature::FilePermissions => "POSIX file permissions",
            
            PlatformFeature::ProcessPriorities => "Process priority control",
            PlatformFeature::SignalHandling => "POSIX signal handling",
            PlatformFeature::ForkSupport => "Process forking",
            PlatformFeature::ThreadSupport => "Thread support",
            
            PlatformFeature::UnixDomainSockets => "Unix domain sockets",
            PlatformFeature::RawSockets => "Raw socket access",
            PlatformFeature::NetworkNamespaces => "Network namespaces",
            
            PlatformFeature::Chroot => "Chroot jail support",
            PlatformFeature::Setuid => "Setuid/setgid support",
            PlatformFeature::Capabilities => "Linux capabilities",
            PlatformFeature::Sandboxing => "Application sandboxing",
            
            PlatformFeature::VirtualMemory => "Virtual memory",
            PlatformFeature::SwapFiles => "Swap file support",
            PlatformFeature::SystemCalls => "System call interface",
            PlatformFeature::KernelModules => "Loadable kernel modules",
            
            PlatformFeature::SystemTray => "System tray integration",
            PlatformFeature::WindowManager => "Window manager",
            PlatformFeature::DesktopNotifications => "Desktop notifications",
            PlatformFeature::ClipboardAccess => "Clipboard access",
        }
    }
}

// =============================================================================
// PLATFORM-SPECIFIC OPERATIONS
// =============================================================================

/// Platform-specific file operations
pub mod file_ops {
    use super::*;
    
    /// Sets file permissions (Unix only)
    #[cfg(unix)]
    pub fn set_permissions<P: AsRef<Path>>(path: P, mode: u32) -> io::Result<()> {
        use std::os::unix::fs::PermissionsExt;
        let permissions = std::fs::Permissions::from_mode(mode);
        std::fs::set_permissions(path, permissions)
    }
    
    #[cfg(not(unix))]
    pub fn set_permissions<P: AsRef<Path>>(_path: P, _mode: u32) -> io::Result<()> {
        Err(std::io::Error::new(
            io::ErrorKind::Unsupported,
            "File permissions not supported on this platform",
        ))
    }
    
    /// Gets file permissions (Unix only)
    #[cfg(unix)]
    pub fn get_permissions<P: AsRef<Path>>(path: P) -> io::Result<u32> {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.permissions().mode())
    }
    
    #[cfg(not(unix))]
    pub fn get_permissions<P: AsRef<Path>>(_path: P) -> io::Result<u32> {
        Err(std::io::Error::new(
            io::ErrorKind::Unsupported,
            "File permissions not supported on this platform",
        ))
    }
    
    /// Creates a hard link
    pub fn create_hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        std::fs::hard_link(src, dst)
    }
    
    /// Creates a symbolic link
    #[cfg(unix)]
    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        std::os::unix::fs::symlink(src, dst)
    }
    
    #[cfg(windows)]
    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        std::os::windows::fs::symlink_file(src, dst)
    }
    
    #[cfg(not(any(unix, windows)))]
    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(_src: P, _dst: Q) -> io::Result<()> {
        Err(std::io::Error::new(
            io::ErrorKind::Unsupported,
            "Symbolic links not supported on this platform",
        ))
    }
    
    /// Reads a symbolic link
    pub fn read_symlink<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        std::fs::read_link(path)
    }
}

/// Platform-specific process operations
pub mod process_ops {
    use super::*;
    
    /// Gets the current process ID
    pub fn current_pid() -> u32 {
        std::process::id()
    }
    
    /// Gets the parent process ID (Unix only)
    #[cfg(unix)]
    pub fn parent_pid() -> Option<u32> {
        unsafe {
            Some(libc::getppid() as u32)
        }
    }
    
    #[cfg(not(unix))]
    pub fn parent_pid() -> Option<u32> {
        None // Not easily available on Windows
    }
    
    /// Sets process priority
    #[cfg(unix)]
    pub fn set_priority(pid: u32, priority: i32) -> io::Result<()> {
        unsafe {
            if libc::setpriority(libc::PRIO_PROCESS, pid, priority) == -1 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }
    
    #[cfg(not(unix))]
    pub fn set_priority(_pid: u32, _priority: i32) -> io::Result<()> {
        Err(std::io::Error::new(
            io::ErrorKind::Unsupported,
            "Process priority setting not implemented on this platform",
        ))
    }
    
    /// Gets process priority
    #[cfg(unix)]
    pub fn get_priority(pid: u32) -> io::Result<i32> {
        unsafe {
            let priority = libc::getpriority(libc::PRIO_PROCESS, pid);
            if priority == -1 && std::io::Error::last_os_error().kind() != io::ErrorKind::NotFound {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(priority)
            }
        }
    }
    
    #[cfg(not(unix))]
    pub fn get_priority(_pid: u32) -> io::Result<i32> {
        Err(std::io::Error::new(
            io::ErrorKind::Unsupported,
            "Process priority getting not implemented on this platform",
        ))
    }
}

/// Platform-specific network operations
pub mod network_ops {
    use super::*;
    
    /// Lists network interfaces
    pub fn list_network_interfaces() -> io::Result<Vec<String>> {
        let platform = Platform::current();
        
        match platform {
            Platform::Linux | Platform::MacOS => {
                let output = Command::new("ip")
                    .args(&["link", "show"])
                    .output()
                    .or_else(|_| Command::new("ifconfig").output())?;
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut interfaces = Vec::new();
                
                for line in output_str.split("\n") {
                    // Parse interface names (simplified)
                    if line.contains(":") && !line.starts_with(' ') {
                        if let Some(name) = line.split(':').next() {
                            if let Some(iface) = name.split_whitespace().last() {
                                interfaces.push(iface.to_string());
                            }
                        }
                    }
                }
                
                Ok(interfaces)
            }
            Platform::Windows => {
                #[cfg(target_os = "windows")]
                {
                    let output = Command::new("ipconfig")
                        .output()?;
                    // Parse Windows ipconfig output
                    Ok(vec!["eth0".to_string()]) // Simplified
                }
                #[cfg(not(target_os = "windows"))]
                Ok(vec!["eth0".to_string()])
            }
            _ => Ok(Vec::new()),
        }
    }
    
    /// Gets default gateway
    pub fn get_default_gateway() -> io::Result<String> {
        let platform = Platform::current();
        
        match platform {
            Platform::Linux => {
                let output = Command::new("ip")
                    .args(&["route", "show", "default"])
                    .output()?;
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                for line in output_str.split("\n") {
                    if line.contains("default via") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            return Ok(parts[2].to_string());
                        }
                    }
                }
                
                Ok("0.0.0.0".to_string())
            }
            Platform::MacOS => {
                let output = Command::new("route")
                    .args(&["get", "default"])
                    .output()?;
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                for line in output_str.split("\n") {
                    if line.trim().starts_with("gateway:") {
                        if let Some(gateway) = line.split(':').nth(1) {
                            return Ok(gateway.trim().to_string());
                        }
                    }
                }
                
                Ok("0.0.0.0".to_string())
            }
            _ => Ok("0.0.0.0".to_string()),
        }
    }
}

// =============================================================================
// PLATFORM UTILITIES
// =============================================================================

/// Gets all supported features for current platform
pub fn get_supported_features() -> Vec<PlatformFeature> {
    use PlatformFeature::*;
    
    let all_features = [
        CaseSensitiveFileSystem, HardLinks, SymbolicLinks, ExtendedAttributes, FilePermissions,
        ProcessPriorities, SignalHandling, ForkSupport, ThreadSupport,
        UnixDomainSockets, RawSockets, NetworkNamespaces,
        Chroot, Setuid, Capabilities, Sandboxing,
        VirtualMemory, SwapFiles, SystemCalls, KernelModules,
        SystemTray, WindowManager, DesktopNotifications, ClipboardAccess,
    ];
    
    all_features
        .iter()
        .filter(|feature| feature.is_supported())
        .copied()
        .collect()
}

/// Gets platform capabilities as a map
pub fn get_platform_capabilities() -> HashMap<String, bool> {
    let mut capabilities = HashMap::new();
    
    for feature in get_supported_features() {
        capabilities.insert(feature.description().to_string(), true);
    }
    
    capabilities
}

/// Executes a platform-appropriate command
pub fn execute_platform_command(command: &str, args: &[&str]) -> io::Result<Output> {
    let platform = Platform::current();
    
    if platform.is_windows() {
        Command::new("cmd")
            .arg("/c")
            .arg(command)
            .args(args)
            .output()
    } else {
        Command::new(command)
            .args(args)
            .output()
    }
}

/// Gets platform-specific temporary directory
pub fn get_temp_directory() -> PathBuf {
    std::env::temp_dir()
}

/// Gets platform-specific home directory
pub fn get_home_directory() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Gets platform-specific config directory
pub fn get_config_directory() -> Option<PathBuf> {
    dirs::config_dir()
}

/// Gets platform-specific data directory
pub fn get_data_directory() -> Option<PathBuf> {
    dirs::data_dir()
}

/// Gets platform-specific cache directory
pub fn get_cache_directory() -> Option<PathBuf> {
    dirs::cache_dir()
}

// Include external crate for CPU count
extern crate num_cpus;

// Include external crate for directories
extern crate dirs;

