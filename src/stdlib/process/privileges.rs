/// Privilege management and security for CURSED processes
/// 
/// Privilege management is critical for system security and enables:
/// - Secure daemon and service implementations
/// - Principle of least privilege enforcement
/// - Security sandboxing and isolation
/// - Safe handling of elevated permissions
/// - Protection against privilege escalation attacks
/// - Building secure system administration tools
/// - Container and virtualization security
/// 
/// This module provides comprehensive privilege management with proper
/// security checks and cross-platform support.

use std::collections::HashSet;
use std::ffi::CString;

use crate::stdlib::process::error::{
    ProcessResult, ProcessError, process_not_found_pid, permission_denied_pid,
    invalid_state, execution_failed, timeout_error, system_error, platform_error
};

/// User and group information
#[derive(Debug, Clone)]
pub struct UserInfo {
    /// User ID
    pub uid: u32,
    /// Group ID  
    pub gid: u32,
    /// Username
    pub username: String,
    /// Home directory
    pub home_dir: Option<String>,
    /// Login shell
    pub shell: Option<String>,
    /// Supplementary group IDs
    pub supplementary_groups: Vec<u32>,
}

/// Privilege levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    /// Root/Administrator privileges
    Root,
    /// Regular user privileges
    User,
    /// Restricted/sandboxed privileges
    Restricted,
    /// Unknown privilege level
    Unknown,
}

/// Linux capabilities (subset of the most important ones)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Override file read, write, and execute permission checks
    DacOverride,
    /// Override file owner ID checks
    DacReadSearch,
    /// Override file owner ID checks for deletion
    Fowner,
    /// Override permission checks for operations that normally require owner or group
    Fsetid,
    /// Send signals to any process
    Kill,
    /// Perform various network-related operations
    NetBindService,
    /// Use RAW and PACKET sockets
    NetRaw,
    /// Bypass permission checks for operations on files
    Setgid,
    /// Make arbitrary manipulations of process UIDs
    Setuid,
    /// Perform various system administration tasks
    SysAdmin,
    /// Raise process nice value and change scheduling
    SysNice,
    /// Perform I/O port operations and access PCI config
    SysRawio,
    /// Use reboot() system call
    SysReboot,
    /// Perform various privileged syslog operations
    Syslog,
    /// Bypass permission checks for sending signals
    SysChroot,
    /// Load and unload kernel modules
    SysModule,
    /// Perform VM86 mode operations
    SysTime,
    /// Bypass ptrace(2) permission checks
    SysPtrace,
    /// Bypass permission checks for accessing the audit log
    AuditWrite,
    /// Enable/disable kernel auditing
    AuditControl,
}

impl Capability {
    /// Get the Linux capability number
    #[cfg(target_os = "linux")]
    pub fn to_cap_value(&self) -> i32 {
        match self {
            Capability::DacOverride => 1,
            Capability::DacReadSearch => 2,
            Capability::Fowner => 3,
            Capability::Fsetid => 4,
            Capability::Kill => 5,
            Capability::Setgid => 6,
            Capability::Setuid => 7,
            Capability::NetBindService => 10,
            Capability::NetRaw => 13,
            Capability::SysChroot => 18,
            Capability::SysPtrace => 19,
            Capability::SysAdmin => 21,
            Capability::SysReboot => 22,
            Capability::SysNice => 23,
            Capability::SysRawio => 25,
            Capability::SysTime => 25,
            Capability::SysModule => 16,
            Capability::Syslog => 34,
            Capability::AuditWrite => 29,
            Capability::AuditControl => 30,
        }
    }

    /// Get capability name
    pub fn name(&self) -> &'static str {
        match self {
            Capability::DacOverride => "CAP_DAC_OVERRIDE",
            Capability::DacReadSearch => "CAP_DAC_READ_SEARCH", 
            Capability::Fowner => "CAP_FOWNER",
            Capability::Fsetid => "CAP_FSETID",
            Capability::Kill => "CAP_KILL",
            Capability::Setgid => "CAP_SETGID",
            Capability::Setuid => "CAP_SETUID",
            Capability::NetBindService => "CAP_NET_BIND_SERVICE",
            Capability::NetRaw => "CAP_NET_RAW",
            Capability::SysChroot => "CAP_SYS_CHROOT",
            Capability::SysPtrace => "CAP_SYS_PTRACE",
            Capability::SysAdmin => "CAP_SYS_ADMIN",
            Capability::SysReboot => "CAP_SYS_REBOOT",
            Capability::SysNice => "CAP_SYS_NICE",
            Capability::SysRawio => "CAP_SYS_RAWIO",
            Capability::SysTime => "CAP_SYS_TIME",
            Capability::SysModule => "CAP_SYS_MODULE",
            Capability::Syslog => "CAP_SYSLOG",
            Capability::AuditWrite => "CAP_AUDIT_WRITE",
            Capability::AuditControl => "CAP_AUDIT_CONTROL",
        }
    }

    /// Get capability description
    pub fn description(&self) -> &'static str {
        match self {
            Capability::DacOverride => "Override file read, write, and execute permission checks",
            Capability::DacReadSearch => "Override file read permission checks and directory search permission checks",
            Capability::Fowner => "Override permission checks on operations that normally require the filesystem UID of the process to match the UID of the file",
            Capability::Fsetid => "Don't clear set-user-ID and set-group-ID mode bits when a file is modified",
            Capability::Kill => "Bypass permission checks for sending signals",
            Capability::Setgid => "Make arbitrary manipulations of process GIDs and supplementary GID list",
            Capability::Setuid => "Make arbitrary manipulations of process UIDs",
            Capability::NetBindService => "Bind a socket to Internet domain privileged ports (port numbers less than 1024)",
            Capability::NetRaw => "Use RAW and PACKET sockets and bind to any address for transparent proxying",
            Capability::SysChroot => "Use chroot(2) and pivot_root(2)",
            Capability::SysPtrace => "Trace arbitrary processes using ptrace(2)",
            Capability::SysAdmin => "Perform a range of system administration operations",
            Capability::SysReboot => "Use reboot(2) and kexec_load(2)",
            Capability::SysNice => "Raise process nice value and change scheduling policy and priority",
            Capability::SysRawio => "Perform I/O port operations and access /proc/kcore",
            Capability::SysTime => "Set system clock and real-time hardware clock",
            Capability::SysModule => "Load and unload kernel modules",
            Capability::Syslog => "Perform privileged syslog(2) operations",
            Capability::AuditWrite => "Write records to kernel auditing log",
            Capability::AuditControl => "Enable and disable kernel auditing",
        }
    }
}

/// Privilege manager for security operations
pub struct PrivilegeManager {
    /// Current user information
    current_user: Option<UserInfo>,
    /// Whether running with elevated privileges
    elevated: bool,
}

impl PrivilegeManager {
    /// Create a new privilege manager
    pub fn new() -> ProcessResult<Self> {
        let current_user = Self::get_current_user_info()?;
        let elevated = Self::is_elevated();

        Ok(Self {
            current_user: Some(current_user),
            elevated,
        })
    }

    /// Check if running with elevated privileges
    pub fn is_elevated() -> bool {
        #[cfg(unix)]
        {
            unsafe { libc::geteuid() == 0 }
        }

        #[cfg(windows)]
        {
            // Windows privilege checking would require more complex WinAPI calls
            // For now, just check if we're running as administrator (simplified)
            std::env::var("USERNAME").map(|u| u.to_lowercase().contains("admin")).unwrap_or(false)
        }

        #[cfg(not(any(unix, windows)))]
        {
            false
        }
    }

    /// Get current user information
    pub fn get_current_user_info() -> ProcessResult<UserInfo> {
        #[cfg(unix)]
        {
            let uid = unsafe { libc::getuid() };
            let gid = unsafe { libc::getgid() };
            
            // Get username
            let username = std::env::var("USER")
                .or_else(|_| std::env::var("LOGNAME"))
                .unwrap_or_else(|_| uid.to_string());

            // Get home directory
            let home_dir = std::env::var("HOME").ok();

            // Get shell
            let shell = std::env::var("SHELL").ok();

            // Get supplementary groups
            let mut supplementary_groups = Vec::new();
            let mut groups = [0u32; 32]; // Max 32 groups
            let mut ngroups = groups.len() as i32;
            
            let result = unsafe { libc::getgroups(ngroups, groups.as_mut_ptr()) };
            if result >= 0 {
                supplementary_groups.extend_from_slice(&groups[..result as usize]);
            }

            Ok(UserInfo {
                uid,
                gid,
                username,
                home_dir,
                shell,
                supplementary_groups,
            })
        }

        #[cfg(windows)]
        {
            let username = std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());
            let home_dir = std::env::var("USERPROFILE").ok();

            Ok(UserInfo {
                uid: 0, // Windows doesn't have Unix UIDs
                gid: 0,
                username,
                home_dir,
                shell: None,
                supplementary_groups: Vec::new(),
            })
        }

        #[cfg(not(any(unix, windows)))]
        {
            Ok(UserInfo {
                uid: 0,
                gid: 0,
                username: "unknown".to_string(),
                home_dir: None,
                shell: None,
                supplementary_groups: Vec::new(),
            })
        }
    }

    /// Drop privileges to a specific user
    pub fn drop_privileges(&mut self, target_uid: u32, target_gid: u32) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Drop supplementary groups first
            let result = unsafe { libc::setgroups(0, std::ptr::null()) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "drop_privileges",
                    "Failed to drop supplementary groups"
                ));
            }

            // Set GID
            let result = unsafe { libc::setgid(target_gid) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "drop_privileges",
                    &format!("Failed to set GID to {}", target_gid)
                ));
            }

            // Set UID (this should be done last as it's irreversible)
            let result = unsafe { libc::setuid(target_uid) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "drop_privileges",
                    &format!("Failed to set UID to {}", target_uid)
                ));
            }

            // Update internal state
            self.elevated = target_uid == 0;
            self.current_user = Some(Self::get_current_user_info()?);

            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err(platform_error("Privilege dropping not supported on this platform"))
        }
    }

    /// Drop privileges to a named user
    pub fn drop_privileges_to_user(&mut self, username: &str) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            let (uid, gid) = self.lookup_user(username)?;
            self.drop_privileges(uid, gid)
        }

        #[cfg(not(unix))]
        {
            Err(platform_error("User lookup not supported on this platform"))
        }
    }

    #[cfg(unix)]
    fn lookup_user(&self, username: &str) -> ProcessResult<(u32, u32)> {
        use std::ffi::CStr;
        use std::mem;
        use std::ptr;

        let username_cstr = CString::new(username)
            .map_err(|_| invalid_state("lookup_user", "username", "Invalid username"))?;

        // Try to get user info using getpwnam
        let pwd = unsafe { libc::getpwnam(username_cstr.as_ptr()) };
        if pwd.is_null() {
            return Err(process_not_found_pid(0, &format!("User '{}' not found", username)));
        }

        unsafe {
            let uid = (*pwd).pw_uid;
            let gid = (*pwd).pw_gid;
            Ok((uid, gid))
        }
    }

    /// Temporarily elevate privileges (if possible)
    pub fn elevate_privileges(&mut self) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // This only works if the process has the SETUID capability or is setuid
            let result = unsafe { libc::seteuid(0) };
            if result != 0 {
                return Err(permission_denied_pid(
                    std::process::id(),
                    "Failed to elevate privileges - insufficient permissions"
                ));
            }

            self.elevated = true;
            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err(platform_error("Privilege elevation not supported on this platform"))
        }
    }

    /// Get current privilege level
    pub fn get_privilege_level(&self) -> PrivilegeLevel {
        if self.elevated {
            PrivilegeLevel::Root
        } else if let Some(ref user) = self.current_user {
            if user.uid == 0 {
                PrivilegeLevel::Root
            } else {
                PrivilegeLevel::User
            }
        } else {
            PrivilegeLevel::Unknown
        }
    }

    /// Check if process has a specific capability (Linux only)
    pub fn has_capability(&self, capability: Capability) -> ProcessResult<bool> {
        #[cfg(target_os = "linux")]
        {
            // Read capabilities from /proc/self/status
            let status_content = std::fs::read_to_string("/proc/self/status")
                .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "has_capability", "Failed to read process status"))?;

            // Look for CapEff line (effective capabilities)
            for line in status_content.split("\n") {
                if line.starts_with("CapEff:") {
                    if let Some(cap_hex) = line.split_whitespace().nth(1) {
                        if let Ok(cap_mask) = u64::from_str_radix(cap_hex, 16) {
                            let cap_bit = 1u64 << capability.to_cap_value();
                            return Ok((cap_mask & cap_bit) != 0);
                        }
                    }
                }
            }

            Ok(false)
        }

        #[cfg(not(target_os = "linux"))]
        {
            // On non-Linux systems, just check if we're root for privileged capabilities
            match capability {
                Capability::DacOverride | Capability::Fowner | Capability::Kill |
                Capability::Setuid | Capability::Setgid | Capability::SysAdmin => {
                    Ok(self.elevated)
                }
                _ => Ok(false),
            }
        }
    }

    /// Drop specific capabilities (Linux only)
    pub fn drop_capabilities(&self, capabilities: &[Capability]) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            for capability in capabilities {
                let result = unsafe { libc::prctl(libc::PR_CAPBSET_DROP, capability.to_cap_value(), 0, 0, 0) };
                if result != 0 {
                    return Err(system_error(
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                        "drop_capabilities",
                        &format!("Failed to drop capability: {}", capability.name())
                    ));
                }
            }
            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Capability management not supported on this platform"))
        }
    }

    /// Get all current capabilities (Linux only)
    pub fn get_current_capabilities(&self) -> ProcessResult<HashSet<Capability>> {
        #[cfg(target_os = "linux")]
        {
            let mut capabilities = HashSet::new();
            
            // Check each capability we know about
            let all_caps = [
                Capability::DacOverride, Capability::DacReadSearch, Capability::Fowner,
                Capability::Fsetid, Capability::Kill, Capability::Setgid, Capability::Setuid,
                Capability::NetBindService, Capability::NetRaw, Capability::SysChroot,
                Capability::SysPtrace, Capability::SysAdmin, Capability::SysReboot,
                Capability::SysNice, Capability::SysRawio, Capability::SysTime,
                Capability::SysModule, Capability::Syslog, Capability::AuditWrite,
                Capability::AuditControl,
            ];

            for cap in &all_caps {
                if self.has_capability(*cap)? {
                    capabilities.insert(*cap);
                }
            }

            Ok(capabilities)
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Return empty set on non-Linux platforms
            Ok(HashSet::new())
        }
    }

    /// Create a secure execution environment
    pub fn create_secure_environment(&self) -> ProcessResult<SecureEnvironment> {
        let user_info = self.current_user.clone()
            .ok_or_else(|| invalid_state("create_secure_environment", "user_info", "No user information available"))?;

        Ok(SecureEnvironment {
            target_uid: if self.elevated { 65534 } else { user_info.uid }, // nobody user if root
            target_gid: if self.elevated { 65534 } else { user_info.gid },
            allowed_capabilities: HashSet::new(),
            chroot_path: None,
            no_new_privs: true,
        })
    }

    /// Get current user information
    pub fn current_user(&self) -> Option<&UserInfo> {
        self.current_user.as_ref()
    }

    /// Check if current user is in a specific group
    pub fn is_in_group(&self, gid: u32) -> bool {
        if let Some(ref user) = self.current_user {
            user.gid == gid || user.supplementary_groups.contains(&gid)
        } else {
            false
        }
    }

    /// Enable no_new_privs for current process (Linux only)
    pub fn set_no_new_privs(&self) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            let result = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "set_no_new_privs",
                    "Failed to set no_new_privs"
                ));
            }
            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("no_new_privs not supported on this platform"))
        }
    }
}

/// Secure execution environment configuration
#[derive(Debug, Clone)]
pub struct SecureEnvironment {
    /// Target user ID to run as
    pub target_uid: u32,
    /// Target group ID to run as
    pub target_gid: u32,
    /// Allowed capabilities to retain
    pub allowed_capabilities: HashSet<Capability>,
    /// Optional chroot path
    pub chroot_path: Option<String>,
    /// Enable no_new_privs
    pub no_new_privs: bool,
}

impl SecureEnvironment {
    /// Create a minimal security environment
    pub fn minimal() -> Self {
        Self {
            target_uid: 65534, // nobody
            target_gid: 65534, // nogroup
            allowed_capabilities: HashSet::new(),
            chroot_path: None,
            no_new_privs: true,
        }
    }

    /// Create a network service environment
    pub fn network_service() -> Self {
        let mut caps = HashSet::new();
        caps.insert(Capability::NetBindService);
        
        Self {
            target_uid: 65534,
            target_gid: 65534,
            allowed_capabilities: caps,
            chroot_path: None,
            no_new_privs: true,
        }
    }

    /// Apply this secure environment to current process
    pub fn apply(&self) -> ProcessResult<()> {
        let mut manager = PrivilegeManager::new()?;

        // Set no_new_privs first
        if self.no_new_privs {
            manager.set_no_new_privs()?;
        }

        // Apply chroot if specified
        if let Some(ref chroot_path) = self.chroot_path {
            #[cfg(unix)]
            {
                let path_cstr = CString::new(chroot_path.as_bytes())
                    .map_err(|_| invalid_state("apply", "chroot_path", "Invalid chroot path"))?;
                
                let result = unsafe { libc::chroot(path_cstr.as_ptr()) };
                if result != 0 {
                    return Err(system_error(
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                        "apply",
                        &format!("Failed to chroot to {}", chroot_path)
                    ));
                }

                // Change to root directory after chroot
                let root_cstr = CString::new("/").unwrap();
                unsafe { libc::chdir(root_cstr.as_ptr()) };
            }

            #[cfg(not(unix))]
            {
                return Err(platform_error("chroot not supported on this platform"));
            }
        }

        // Drop capabilities we don't need
        #[cfg(target_os = "linux")]
        {
            let current_caps = manager.get_current_capabilities()?;
            let caps_to_drop: Vec<_> = current_caps.difference(&self.allowed_capabilities).cloned().collect();
            if !caps_to_drop.is_empty() {
                manager.drop_capabilities(&caps_to_drop)?;
            }
        }

        // Drop privileges last (irreversible)
        manager.drop_privileges(self.target_uid, self.target_gid)?;

        Ok(())
    }

    /// Add an allowed capability
    pub fn allow_capability(&mut self, capability: Capability) {
        self.allowed_capabilities.insert(capability);
    }

    /// Set chroot path
    pub fn with_chroot<S: Into<String>>(mut self, path: S) -> Self {
        self.chroot_path = Some(path.into());
        self
    }

    /// Set target user and group
    pub fn with_user_group(mut self, uid: u32, gid: u32) -> Self {
        self.target_uid = uid;
        self.target_gid = gid;
        self
    }
}

/// Convenience functions for common privilege operations

/// Check if running as root/administrator
pub fn is_root() -> bool {
    PrivilegeManager::is_elevated()
}

/// Get current user ID
pub fn current_uid() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getuid() }
    }

    #[cfg(not(unix))]
    {
        0
    }
}

/// Get current group ID
pub fn current_gid() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getgid() }
    }

    #[cfg(not(unix))]
    {
        0
    }
}

/// Safely drop privileges to nobody user
pub fn drop_to_nobody() -> ProcessResult<()> {
    let mut manager = PrivilegeManager::new()?;
    manager.drop_privileges(65534, 65534) // nobody:nogroup
}

/// Create and apply a secure sandboxed environment
pub fn create_sandbox() -> ProcessResult<()> {
    let env = SecureEnvironment::minimal();
    env.apply()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_methods() {
        let cap = Capability::NetBindService;
        assert_eq!(cap.name(), "CAP_NET_BIND_SERVICE");
        assert!(cap.description().contains("privileged ports"));
    }

    #[test]
    fn test_privilege_level() {
        let level = PrivilegeLevel::User;
        assert_eq!(level, PrivilegeLevel::User);
        assert_ne!(level, PrivilegeLevel::Root);
    }

    #[test]
    fn test_secure_environment_creation() {
        let env = SecureEnvironment::minimal();
        assert_eq!(env.target_uid, 65534);
        assert_eq!(env.target_gid, 65534);
        assert!(env.no_new_privs);
        assert!(env.allowed_capabilities.is_empty());
    }

    #[test]
    fn test_network_service_environment() {
        let env = SecureEnvironment::network_service();
        assert!(env.allowed_capabilities.contains(&Capability::NetBindService));
        assert_eq!(env.allowed_capabilities.len(), 1);
    }

    #[test]
    fn test_user_info_creation() {
        let user_info = PrivilegeManager::get_current_user_info();
        assert!(user_info.is_ok());
        
        let user_info = user_info.unwrap();
        assert!(!user_info.username.is_empty());
    }

    #[test]
    fn test_privilege_manager_creation() {
        let manager = PrivilegeManager::new();
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert!(manager.current_user().is_some());
    }

    #[cfg(unix)]
    #[test]
    fn test_current_uid_gid() {
        let uid = current_uid();
        let gid = current_gid();
        
        // Should return valid IDs
        assert!(uid >= 0);
        assert!(gid >= 0);
        
        // Should match what libc returns
        assert_eq!(uid, unsafe { libc::getuid() });
        assert_eq!(gid, unsafe { libc::getgid() });
    }

    #[test]
    fn test_is_root() {
        let is_root = is_root();
        // Just ensure it returns a boolean without panicking
        assert!(is_root == true || is_root == false);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_capability_checking() {
        let manager = PrivilegeManager::new().unwrap();
        
        // Test capability checking (should not panic)
        let has_kill = manager.has_capability(Capability::Kill);
        assert!(has_kill.is_ok());
        
        let caps = manager.get_current_capabilities();
        assert!(caps.is_ok());
    }
}
