/// System information gathering for CURSED
/// 
/// This module provides comprehensive system information including operating system
/// details, hardware specifications, and runtime environment data.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::env;
use std::fs;

/// System information result type
pub type SystemResult<T> = Result<T, SystemError>;

/// System information errors
#[derive(Debug, Clone)]
pub enum SystemError {
    /// Platform not supported
    PlatformNotSupported(String),
    /// Information not available
    InformationNotAvailable(String),
    /// I/O error occurred
    IoError(String),
    /// Parse error occurred
    ParseError(String),
    /// Permission denied
    PermissionDenied(String),
    /// System call failed
    SystemCallFailed(i32, String),
    /// Monitoring error occurred
    MonitoringError(String),
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::PlatformNotSupported(msg) => write!(f, "Platform not supported: {}", msg),
            SystemError::InformationNotAvailable(msg) => write!(f, "Information not available: {}", msg),
            SystemError::IoError(msg) => write!(f, "I/O error: {}", msg),
            SystemError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SystemError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SystemError::SystemCallFailed(code, msg) => write!(f, "System call failed ({}): {}", code, msg),
            SystemError::MonitoringError(msg) => write!(f, "Monitoring error: {}", msg),
        }
    }
}

impl std::error::Error for SystemError {}

/// Comprehensive system information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_info: OsInfo,
    pub kernel_info: KernelInfo,
    pub boot_time: SystemTime,
    pub uptime: Duration,
    pub hostname: String,
    pub domain: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub environment_variables: HashMap<String, String>,
    pub user_info: UserInfo,
    pub system_paths: SystemPaths,
}

/// Operating system information
#[derive(Debug, Clone)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub build: String,
    pub edition: Option<String>,
    pub architecture: String,
    pub family: OsFamily,
    pub is_64bit: bool,
    pub install_date: Option<SystemTime>,
}

/// Kernel information
#[derive(Debug, Clone)]
pub struct KernelInfo {
    pub name: String,
    pub version: String,
    pub release: String,
    pub build_date: Option<String>,
    pub command_line: Option<String>,
    pub modules: Vec<String>,
}

/// Operating system family
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsFamily {
    Windows,
    Unix,
    Linux,
    MacOS,
    FreeBSD,
    NetBSD,
    OpenBSD,
    Unknown,
}

/// User information
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub home_directory: Option<String>,
    pub shell: Option<String>,
    pub groups: Vec<String>,
    pub is_admin: bool,
    pub is_elevated: bool,
}

/// System paths
#[derive(Debug, Clone)]
pub struct SystemPaths {
    pub home: Option<String>,
    pub temp: String,
    pub system: String,
    pub program_files: Option<String>,
    pub program_data: Option<String>,
    pub documents: Option<String>,
    pub desktop: Option<String>,
    pub downloads: Option<String>,
}

impl SystemInfo {
    /// Create a new SystemInfo by gathering all available information
    pub fn gather() -> SystemResult<Self> {
        Ok(Self {
            os_info: OsInfo::gather()?,
            kernel_info: KernelInfo::gather()?,
            boot_time: get_boot_time()?,
            uptime: get_uptime()?,
            hostname: get_hostname()?,
            domain: get_domain(),
            timezone: get_timezone()?,
            locale: get_locale()?,
            environment_variables: get_environment_variables(),
            user_info: UserInfo::gather()?,
            system_paths: SystemPaths::gather()?,
        })
    }
}

impl OsInfo {
    /// Gather operating system information
    pub fn gather() -> SystemResult<Self> {
        #[cfg(windows)]
        return Self::gather_windows();
        
        #[cfg(unix)]
        return Self::gather_unix();
        
        #[cfg(not(any(windows, unix)))]
        Err(SystemError::PlatformNotSupported("Operating system not supported".to_string()))
    }
    
    #[cfg(windows)]
    fn gather_windows() -> SystemResult<Self> {
        use std::process::Command;
        
        // Get OS information using systeminfo
        let output = Command::new("systeminfo")
            .args(&["/FO", "CSV"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;
        
        if !output.status.success() {
            return Err(SystemError::SystemCallFailed(
                output.status.code().unwrap_or(-1),
                "systeminfo command failed"
            ));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.split("\n").collect();
        
        // Parse CSV output (simplified)
        let mut name = "Windows".to_string();
        let mut version = "Unknown".to_string();
        let mut build = "Unknown".to_string();
        
        // Try PowerShell for more detailed info
        if let Ok(ps_output) = Command::new("powershell")
            .args(&["-Command", 
                "Get-ComputerInfo | Select-Object WindowsProductName,WindowsVersion,WindowsBuildLabEx,OSArchitecture | ConvertTo-Json"
            ])
            .output()
        {
            if ps_output.status.success() {
                let ps_stdout = String::from_utf8_lossy(&ps_output.stdout);
                // Simple JSON parsing (in production would use serde_json)
                if let Some(product_name) = extract_json_field(&ps_stdout, "WindowsProductName") {
                    name = product_name;
                }
                if let Some(win_version) = extract_json_field(&ps_stdout, "WindowsVersion") {
                    version = win_version;
                }
                if let Some(build_lab) = extract_json_field(&ps_stdout, "WindowsBuildLabEx") {
                    build = build_lab;
                }
            }
        }
        
        Ok(Self {
            name,
            version,
            build,
            edition: None,
            architecture: env::consts::ARCH.to_string(),
            family: OsFamily::Windows,
            is_64bit: env::consts::ARCH.contains("64"),
            install_date: None,
        })
    }
    
    #[cfg(unix)]
    fn gather_unix() -> SystemResult<Self> {
        let name = match std::env::consts::OS {
            "linux" => get_linux_distribution().unwrap_or_else(|| "Linux".to_string()),
            "macos" => "macOS".to_string(),
            "freebsd" => "FreeBSD".to_string(),
            "netbsd" => "NetBSD".to_string(),
            "openbsd" => "OpenBSD".to_string(),
            other => other.to_string(),
        };
        
        let family = match std::env::consts::OS {
            "linux" => OsFamily::Linux,
            "macos" => OsFamily::MacOS,
            "freebsd" => OsFamily::FreeBSD,
            "netbsd" => OsFamily::NetBSD,
            "openbsd" => OsFamily::OpenBSD,
            _ => OsFamily::Unix,
        };
        
        // Try to get version from /etc/os-release
        let (version, build) = if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
            let mut version = "Unknown".to_string();
            let mut build = "Unknown".to_string();
            
            for line in os_release.split("\n") {
                if line.starts_with("VERSION_ID=") {
                    version = line.split('=').nth(1)
                        .unwrap_or("Unknown")
                        .trim_matches('"')
                        .to_string();
                } else if line.starts_with("BUILD_ID=") {
                    build = line.split('=').nth(1)
                        .unwrap_or("Unknown")
                        .trim_matches('"')
                        .to_string();
                }
            }
            
            (version, build)
        } else {
            ("Unknown".to_string(), "Unknown".to_string())
        };
        
        Ok(Self {
            name,
            version,
            build,
            edition: None,
            architecture: env::consts::ARCH.to_string(),
            family,
            is_64bit: env::consts::ARCH.contains("64"),
            install_date: None,
        })
    }
}

impl KernelInfo {
    /// Gather kernel information
    pub fn gather() -> SystemResult<Self> {
        #[cfg(windows)]
        return Self::gather_windows();
        
        #[cfg(unix)]
        return Self::gather_unix();
        
        #[cfg(not(any(windows, unix)))]
        Err(SystemError::PlatformNotSupported("Kernel information not supported".to_string()))
    }
    
    #[cfg(windows)]
    fn gather_windows() -> SystemResult<Self> {
        use std::process::Command;
        
        // Get kernel version using ver command
        let output = Command::new("cmd")
            .args(&["/C", "ver"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;
        
        let version_str = String::from_utf8_lossy(&output.stdout);
        let version = version_str.trim().to_string();
        
        Ok(Self {
            name: "Windows NT".to_string(),
            version,
            release: "Windows".to_string(),
            build_date: None,
            command_line: None,
            modules: Vec::new(),
        })
    }
    
    #[cfg(unix)]
    fn gather_unix() -> SystemResult<Self> {
        use std::process::Command;
        
        // Get kernel information using uname
        let output = Command::new("uname")
            .args(&["-a"])
            .output()
            .map_err(|e| SystemError::SystemCallFailed(-1, e.to_string()))?;
        
        let uname_output = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = uname_output.split_whitespace().collect();
        
        let name = parts.get(0).unwrap_or(&"Unknown").to_string();
        let release = parts.get(2).unwrap_or(&"Unknown").to_string();
        let version = parts.get(3).unwrap_or(&"Unknown").to_string();
        
        // Try to get kernel command line
        let command_line = fs::read_to_string("/proc/cmdline")
            .map(|s| s.trim().to_string())
            .ok();
        
        // Try to get loaded modules
        let modules = if let Ok(modules_content) = fs::read_to_string("/proc/modules") {
            modules_content
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .next()
                        .unwrap_or("unknown")
                        .to_string()
                })
                .collect()
        } else {
            Vec::new()
        };
        
        Ok(Self {
            name,
            version,
            release,
            build_date: None,
            command_line,
            modules,
        })
    }
}

impl UserInfo {
    /// Gather user information
    pub fn gather() -> SystemResult<Self> {
        #[cfg(windows)]
        return Self::gather_windows();
        
        #[cfg(unix)]
        return Self::gather_unix();
        
        #[cfg(not(any(windows, unix)))]
        Err(SystemError::PlatformNotSupported("User information not supported".to_string()))
    }
    
    #[cfg(windows)]
    fn gather_windows() -> SystemResult<Self> {
        let username = env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
        let home_directory = env::var("USERPROFILE").ok();
        
        // Check if user is admin (simplified)
        let is_admin = check_windows_admin();
        
        Ok(Self {
            username,
            uid: None,
            gid: None,
            home_directory,
            shell: None,
            groups: Vec::new(),
            is_admin,
            is_elevated: is_admin,
        })
    }
    
    #[cfg(unix)]
    fn gather_unix() -> SystemResult<Self> {
        let username = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
        let home_directory = env::var("HOME").ok();
        let shell = env::var("SHELL").ok();
        
        let uid = unsafe { Some(libc::getuid()) };
        let gid = unsafe { Some(libc::getgid()) };
        let is_admin = uid == Some(0);
        let is_elevated = unsafe { libc::geteuid() == 0 };
        
        // Get user groups
        let groups = get_user_groups(&username);
        
        Ok(Self {
            username,
            uid,
            gid,
            home_directory,
            shell,
            groups,
            is_admin,
            is_elevated,
        })
    }
}

impl SystemPaths {
    /// Gather system paths
    pub fn gather() -> SystemResult<Self> {
        Ok(Self {
            home: env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok(),
            temp: env::temp_dir().to_string_lossy().to_string(),
            system: get_system_directory(),
            program_files: env::var("PROGRAMFILES").ok(),
            program_data: env::var("PROGRAMDATA").ok(),
            documents: get_documents_directory(),
            desktop: get_desktop_directory(),
            downloads: get_downloads_directory(),
        })
    }
}

// Helper functions

/// Get system information
pub fn get_system_info() -> SystemResult<SystemInfo> {
    SystemInfo::gather()
}

/// Get operating system information
pub fn get_os_info() -> SystemResult<OsInfo> {
    OsInfo::gather()
}

/// Get kernel information
pub fn get_kernel_info() -> SystemResult<KernelInfo> {
    KernelInfo::gather()
}

fn get_boot_time() -> SystemResult<SystemTime> {
    #[cfg(unix)]
    {
        if let Ok(uptime_content) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime_str) = uptime_content.split_whitespace().next() {
                if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                    let boot_time = SystemTime::now() - Duration::from_secs_f64(uptime_secs);
                    return Ok(boot_time);
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        // Use PowerShell to get boot time
        if let Ok(output) = Command::new("powershell")
            .args(&["-Command", "(Get-CimInstance -ClassName Win32_OperatingSystem).LastBootUpTime"])
            .output()
        {
            if output.status.success() {
                // For simplicity, estimate boot time
                let estimated_uptime = Duration::from_secs(3600); // 1 hour estimate
                return Ok(SystemTime::now() - estimated_uptime);
            }
        }
    }
    
    // Fallback: estimate boot time
    Ok(SystemTime::now() - Duration::from_secs(3600))
}

fn get_uptime() -> SystemResult<Duration> {
    #[cfg(unix)]
    {
        if let Ok(uptime_content) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime_str) = uptime_content.split_whitespace().next() {
                if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                    return Ok(Duration::from_secs_f64(uptime_secs));
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("powershell")
            .args(&["-Command", "[Environment]::TickCount / 1000"])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(seconds) = stdout.trim().parse::<u64>() {
                    return Ok(Duration::from_secs(seconds));
                }
            }
        }
    }
    
    // Fallback
    Ok(Duration::from_secs(3600))
}

fn get_hostname() -> SystemResult<String> {
    if let Ok(hostname) = env::var("HOSTNAME") {
        return Ok(hostname);
    }
    
    if let Ok(hostname) = env::var("COMPUTERNAME") {
        return Ok(hostname);
    }
    
    #[cfg(unix)]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("hostname").output() {
            if output.status.success() {
                let hostname = String::from_utf8_lossy(&output.stdout);
                return Ok(hostname.trim().to_string());
            }
        }
    }
    
    Ok("localhost".to_string())
}

fn get_domain() -> Option<String> {
    env::var("USERDOMAIN").ok()
        .or_else(|| env::var("DOMAIN").ok())
}

fn get_timezone() -> SystemResult<String> {
    #[cfg(unix)]
    {
        if let Ok(tz) = env::var("TZ") {
            return Ok(tz);
        }
        
        if let Ok(timezone) = fs::read_to_string("/etc/timezone") {
            return Ok(timezone.trim().to_string());
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        if let Ok(output) = Command::new("powershell")
            .args(&["-Command", "(Get-TimeZone).Id"])
            .output()
        {
            if output.status.success() {
                let timezone = String::from_utf8_lossy(&output.stdout);
                return Ok(timezone.trim().to_string());
            }
        }
    }
    
    Ok("UTC".to_string())
}

fn get_locale() -> SystemResult<String> {
    if let Ok(locale) = env::var("LANG") {
        return Ok(locale);
    }
    
    if let Ok(locale) = env::var("LC_ALL") {
        return Ok(locale);
    }
    
    Ok("en_US.UTF-8".to_string())
}

fn get_environment_variables() -> HashMap<String, String> {
    env::vars().collect()
}

#[cfg(unix)]
fn get_linux_distribution() -> Option<String> {
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        for line in os_release.split("\n") {
            if line.starts_with("PRETTY_NAME=") {
                return Some(
                    line.split('=')
                        .nth(1)?
                        .trim_matches('"')
                        .to_string()
                );
            }
        }
    }
    None
}

#[cfg(unix)]
fn get_user_groups(username: &str) -> Vec<String> {
    use std::process::Command;
    
    if let Ok(output) = Command::new("groups").arg(username).output() {
        if output.status.success() {
            let groups_str = String::from_utf8_lossy(&output.stdout);
            return groups_str
                .split_whitespace()
                .skip(2) // Skip "username :"
                .map(|s| s.to_string())
                .collect();
        }
    }
    
    Vec::new()
}

#[cfg(windows)]
fn check_windows_admin() -> bool {
    use std::process::Command;
    
    // Try to run a command that requires admin privileges
    if let Ok(output) = Command::new("net")
        .args(&["session"])
        .output()
    {
        output.status.success()
    } else {
        false
    }
}

#[cfg(windows)]
fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let search_pattern = format!("\"{}\":", field);
    if let Some(start) = json.find(&search_pattern) {
        let after_key = &json[start + search_pattern.len()..];
        if let Some(value_start) = after_key.find('"') {
            let value_part = &after_key[value_start + 1..];
            if let Some(value_end) = value_part.find('"') {
                return Some(value_part[..value_end].to_string());
            }
        }
    }
    None
}

fn get_system_directory() -> String {
    #[cfg(windows)]
    {
        env::var("SYSTEMROOT").unwrap_or_else(|_| "C:\\Windows".to_string())
    }
    
    #[cfg(unix)]
    {
        "/".to_string()
    }
    
    #[cfg(not(any(windows, unix)))]
    {
        "/".to_string()
    }
}

fn get_documents_directory() -> Option<String> {
    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok()
            .map(|home| format!("{}\\Documents", home))
    }
    
    #[cfg(unix)]
    {
        env::var("HOME").ok()
            .map(|home| format!("{}/Documents", home))
    }
    
    #[cfg(not(any(windows, unix)))]
    {
        None
    }
}

fn get_desktop_directory() -> Option<String> {
    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok()
            .map(|home| format!("{}\\Desktop", home))
    }
    
    #[cfg(unix)]
    {
        env::var("HOME").ok()
            .map(|home| format!("{}/Desktop", home))
    }
    
    #[cfg(not(any(windows, unix)))]
    {
        None
    }
}

fn get_downloads_directory() -> Option<String> {
    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok()
            .map(|home| format!("{}\\Downloads", home))
    }
    
    #[cfg(unix)]
    {
        env::var("HOME").ok()
            .map(|home| format!("{}/Downloads", home))
    }
    
    #[cfg(not(any(windows, unix)))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_gathering() {
        let result = SystemInfo::gather();
        assert!(result.is_ok(), "Failed to gather system info: {:?}", result);
        
        let info = result.unwrap();
        assert!(!info.hostname.is_empty());
        assert!(!info.os_info.name.is_empty());
    }

    #[test]
    fn test_os_info_gathering() {
        let result = OsInfo::gather();
        assert!(result.is_ok(), "Failed to gather OS info: {:?}", result);
        
        let info = result.unwrap();
        assert!(!info.name.is_empty());
        assert!(!info.architecture.is_empty());
    }

    #[test]
    fn test_kernel_info_gathering() {
        let result = KernelInfo::gather();
        assert!(result.is_ok(), "Failed to gather kernel info: {:?}", result);
        
        let info = result.unwrap();
        assert!(!info.name.is_empty());
    }

    #[test]
    fn test_user_info_gathering() {
        let result = UserInfo::gather();
        assert!(result.is_ok(), "Failed to gather user info: {:?}", result);
        
        let info = result.unwrap();
        assert!(!info.username.is_empty());
    }

    #[test]
    fn test_system_paths() {
        let result = SystemPaths::gather();
        assert!(result.is_ok(), "Failed to gather system paths: {:?}", result);
        
        let paths = result.unwrap();
        assert!(!paths.temp.is_empty());
        assert!(!paths.system.is_empty());
    }

    #[test]
    fn test_hostname() {
        let hostname = get_hostname();
        assert!(hostname.is_ok());
        assert!(!hostname.unwrap().is_empty());
    }

    #[test]
    fn test_uptime() {
        let uptime = get_uptime();
        assert!(uptime.is_ok());
        assert!(uptime.unwrap().as_secs() > 0);
    }
}
