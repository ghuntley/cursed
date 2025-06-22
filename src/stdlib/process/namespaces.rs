/// Process namespaces implementation for CURSED (Linux-specific)
/// 
/// Process namespaces are a fundamental Linux feature that enables:
/// - Container isolation and containerized applications
/// - Security sandboxing and privilege separation
/// - Resource isolation for multi-tenant systems
/// - Building custom runtime environments
/// - Process and network isolation
/// - Filesystem isolation and chroot alternatives
/// - User and group ID mapping for security
/// 
/// This module provides comprehensive namespace management for building
/// container-like isolation in CURSED applications.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::CString;

use crate::stdlib::process::error::{
    ProcessResult, ProcessError, process_not_found_pid, permission_denied_pid,
    invalid_state, execution_failed, timeout_error, system_error, platform_error
};

/// Linux namespace types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    /// Mount namespace (filesystem isolation)
    Mount,
    /// UTS namespace (hostname and domain name isolation) 
    UTS,
    /// IPC namespace (System V IPC isolation)
    IPC,
    /// PID namespace (process ID isolation)
    PID,
    /// Network namespace (network stack isolation)
    Network,
    /// User namespace (user and group ID isolation)
    User,
    /// Cgroup namespace (cgroup isolation)
    Cgroup,
    /// Time namespace (system time isolation)
    Time,
}

impl NamespaceType {
    /// Get the clone flag for this namespace type
    #[cfg(target_os = "linux")]
    pub fn clone_flag(&self) -> libc::c_int {
        match self {
            NamespaceType::Mount => libc::CLONE_NEWNS,
            NamespaceType::UTS => libc::CLONE_NEWUTS,
            NamespaceType::IPC => libc::CLONE_NEWIPC,
            NamespaceType::PID => libc::CLONE_NEWPID,
            NamespaceType::Network => libc::CLONE_NEWNET,
            NamespaceType::User => libc::CLONE_NEWUSER,
            NamespaceType::Cgroup => libc::CLONE_NEWCGROUP,
            _ => 0, // Time namespace might not be available on all kernels
        }
    }

    /// Get the unshare flag for this namespace type
    #[cfg(target_os = "linux")]
    pub fn unshare_flag(&self) -> libc::c_int {
        self.clone_flag()
    }

    /// Get the namespace name for /proc filesystem
    pub fn proc_name(&self) -> &'static str {
        match self {
            NamespaceType::Mount => "mnt",
            NamespaceType::UTS => "uts", 
            NamespaceType::IPC => "ipc",
            NamespaceType::PID => "pid",
            NamespaceType::Network => "net",
            NamespaceType::User => "user",
            NamespaceType::Cgroup => "cgroup",
            NamespaceType::Time => "time",
        }
    }
}

/// Namespace information
#[derive(Debug, Clone)]
pub struct NamespaceInfo {
    /// Namespace type
    pub namespace_type: NamespaceType,
    /// Namespace inode number
    pub inode: u64,
    /// Path to namespace file
    pub path: PathBuf,
    /// Device number
    pub device: u64,
}

/// Namespace manager for process isolation
pub struct NamespaceManager {
    /// Whether running on Linux (namespaces only supported on Linux)
    linux_supported: bool,
}

impl NamespaceManager {
    /// Create a new namespace manager
    pub fn new() -> Self {
        Self {
            linux_supported: cfg!(target_os = "linux"),
        }
    }

    /// Check if namespaces are supported on this platform
    pub fn is_supported(&self) -> bool {
        self.linux_supported
    }

    /// Create new namespaces for current process
    pub fn unshare_namespaces(&self, namespaces: &[NamespaceType]) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            if namespaces.is_empty() {
                return Ok(());
            }

            let mut flags = 0;
            for ns_type in namespaces {
                flags |= ns_type.unshare_flag();
            }

            let result = unsafe { libc::unshare(flags) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "unshare_namespaces",
                    &format!("Failed to unshare namespaces: {:?}", namespaces)
                ));
            }

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Namespaces are only supported on Linux"))
        }
    }

    /// Enter an existing namespace
    pub fn enter_namespace(&self, namespace_path: &Path) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::io::AsRawFd;

            let file = fs::File::open(namespace_path)
                .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "enter_namespace", &format!("Failed to open namespace file: {}", e)))?;

            let result = unsafe { libc::setns(file.as_raw_fd(), 0) };
            if result != 0 {
                return Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "enter_namespace",
                    &format!("Failed to enter namespace: {}", namespace_path.display())
                ));
            }

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Namespaces are only supported on Linux"))
        }
    }

    /// Get current process namespaces
    pub fn get_current_namespaces(&self) -> ProcessResult<Vec<NamespaceInfo>> {
        self.get_process_namespaces(std::process::id())
    }

    /// Get namespaces for a specific process
    pub fn get_process_namespaces(&self, pid: u32) -> ProcessResult<Vec<NamespaceInfo>> {
        #[cfg(target_os = "linux")]
        {
            let ns_dir = format!("/proc/{}/ns", pid);
            let entries = fs::read_dir(&ns_dir)
                .map_err(|e| process_not_found_pid(pid, &format!("Failed to read namespace directory: {}", e)))?;

            let mut namespaces = Vec::new();

            for entry in entries {
                let entry = entry.map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "get_process_namespaces", &format!("Failed to read directory entry: {}", e)))?;
                
                let file_name = entry.file_name();
                let ns_name = file_name.to_string_lossy();
                
                let namespace_type = match ns_name.as_ref() {
                    "mnt" => NamespaceType::Mount,
                    "uts" => NamespaceType::UTS,
                    "ipc" => NamespaceType::IPC,
                    "pid" => NamespaceType::PID,
                    "net" => NamespaceType::Network,
                    "user" => NamespaceType::User,
                    "cgroup" => NamespaceType::Cgroup,
                    "time" => NamespaceType::Time,
                    _ => continue, // Skip unknown namespace types
                };

                let ns_path = entry.path();
                let metadata = fs::metadata(&ns_path)
                    .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "get_process_namespaces", &format!("Failed to get namespace metadata: {}", e)))?;

                let inode = metadata.ino();
                let device = metadata.dev();

                namespaces.push(NamespaceInfo {
                    namespace_type,
                    inode,
                    path: ns_path,
                    device,
                });
            }

            Ok(namespaces)
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Namespaces are only supported on Linux"))
        }
    }

    /// Create a new mount namespace and setup filesystem isolation
    pub fn create_mount_namespace(&self, new_root: Option<&Path>, old_root: Option<&Path>) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            // Unshare mount namespace
            self.unshare_namespaces(&[NamespaceType::Mount])?;

            if let Some(new_root_path) = new_root {
                // Change root if specified
                self.pivot_root(new_root_path, old_root)?;
            }

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Mount namespaces are only supported on Linux"))
        }
    }

    /// Change root directory using pivot_root
    #[cfg(target_os = "linux")]
    fn pivot_root(&self, new_root: &Path, old_root: Option<&Path>) -> ProcessResult<()> {
        let new_root_cstr = CString::new(new_root.to_string_lossy().as_bytes())
            .map_err(|_| invalid_state("pivot_root", "new_root", "Invalid path"))?;

        let old_root_path = old_root.unwrap_or_else(|| Path::new("/mnt"));
        let old_root_cstr = CString::new(old_root_path.to_string_lossy().as_bytes())
            .map_err(|_| invalid_state("pivot_root", "old_root", "Invalid path"))?;

        // Create old root directory if it doesn't exist
        let old_root_in_new = new_root.join(old_root_path.strip_prefix("/").unwrap_or(old_root_path));
        if !old_root_in_new.exists() {
            fs::create_dir_all(&old_root_in_new)
                .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "pivot_root", &format!("Failed to create old root directory: {}", e)))?;
        }

        let result = unsafe { libc::pivot_root(new_root_cstr.as_ptr(), old_root_cstr.as_ptr()) };
        if result != 0 {
            return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "pivot_root",
                &format!("Failed to pivot root from {} to {}", old_root_path.display(), new_root.display())
            ));
        }

        Ok(())
    }

    /// Create a new network namespace
    pub fn create_network_namespace(&self) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            self.unshare_namespaces(&[NamespaceType::Network])
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Network namespaces are only supported on Linux"))
        }
    }

    /// Create a new PID namespace
    pub fn create_pid_namespace(&self) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            self.unshare_namespaces(&[NamespaceType::PID])
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("PID namespaces are only supported on Linux"))
        }
    }

    /// Create a new user namespace with UID/GID mapping
    pub fn create_user_namespace(&self, uid_map: Option<&[(u32, u32, u32)]>, gid_map: Option<&[(u32, u32, u32)]>) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            // Unshare user namespace first
            self.unshare_namespaces(&[NamespaceType::User])?;

            // Set up UID mapping if provided
            if let Some(mappings) = uid_map {
                self.write_id_mapping("/proc/self/uid_map", mappings)?;
            }

            // Set up GID mapping if provided
            if let Some(mappings) = gid_map {
                // Disable setgroups for unprivileged user namespaces
                fs::write("/proc/self/setgroups", "deny")
                    .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "create_user_namespace", &format!("Failed to disable setgroups: {}", e)))?;
                
                self.write_id_mapping("/proc/self/gid_map", mappings)?;
            }

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("User namespaces are only supported on Linux"))
        }
    }

    #[cfg(target_os = "linux")]
    fn write_id_mapping(&self, map_file: &str, mappings: &[(u32, u32, u32)]) -> ProcessResult<()> {
        let mut map_content = String::new();
        for (inside_id, outside_id, length) in mappings {
            map_content.push_str(&format!("{} {} {}\n", inside_id, outside_id, length));
        }

        fs::write(map_file, map_content)
            .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "write_id_mapping", &format!("Failed to write ID mapping: {}", e)))?;

        Ok(())
    }

    /// Create an isolated container-like environment
    pub fn create_container_environment(&self, config: &ContainerConfig) -> ProcessResult<()> {
        #[cfg(target_os = "linux")]
        {
            // Create multiple namespaces for container isolation
            let mut namespaces = vec![
                NamespaceType::Mount,
                NamespaceType::UTS,
                NamespaceType::IPC,
                NamespaceType::PID,
            ];

            if config.network_isolation {
                namespaces.push(NamespaceType::Network);
            }

            if config.user_namespace {
                namespaces.push(NamespaceType::User);
            }

            self.unshare_namespaces(&namespaces)?;

            // Set hostname if specified
            if let Some(ref hostname) = config.hostname {
                self.set_hostname(hostname)?;
            }

            // Setup root filesystem if specified
            if let Some(ref root_path) = config.root_path {
                self.create_mount_namespace(Some(root_path), config.old_root_path.as_deref())?;
            }

            // Setup user mapping if in user namespace
            if config.user_namespace {
                if let Some(ref uid_map) = config.uid_mappings {
                    self.write_id_mapping("/proc/self/uid_map", uid_map)?;
                }
                if let Some(ref gid_map) = config.gid_mappings {
                    fs::write("/proc/self/setgroups", "deny")
                        .map_err(|e| system_error(e.raw_os_error().unwrap_or(-1), "create_container_environment", &format!("Failed to disable setgroups: {}", e)))?;
                    self.write_id_mapping("/proc/self/gid_map", gid_map)?;
                }
            }

            Ok(())
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(platform_error("Container environments are only supported on Linux"))
        }
    }

    #[cfg(target_os = "linux")]
    fn set_hostname(&self, hostname: &str) -> ProcessResult<()> {
        let hostname_cstr = CString::new(hostname)
            .map_err(|_| invalid_state("set_hostname", "hostname", "Invalid hostname"))?;

        let result = unsafe { libc::sethostname(hostname_cstr.as_ptr(), hostname.len()) };
        if result != 0 {
            return Err(system_error(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "set_hostname",
                &format!("Failed to set hostname: {}", hostname)
            ));
        }

        Ok(())
    }

    /// Compare namespaces between two processes
    pub fn compare_namespaces(&self, pid1: u32, pid2: u32) -> ProcessResult<NamespaceComparison> {
        let ns1 = self.get_process_namespaces(pid1)?;
        let ns2 = self.get_process_namespaces(pid2)?;

        let mut comparison = NamespaceComparison {
            same_namespaces: Vec::new(),
            different_namespaces: Vec::new(),
            only_in_first: Vec::new(),
            only_in_second: Vec::new(),
        };

        let mut ns1_map: HashMap<NamespaceType, &NamespaceInfo> = HashMap::new();
        let mut ns2_map: HashMap<NamespaceType, &NamespaceInfo> = HashMap::new();

        for ns in &ns1 {
            ns1_map.insert(ns.namespace_type, ns);
        }

        for ns in &ns2 {
            ns2_map.insert(ns.namespace_type, ns);
        }

        // Find common namespace types
        for (ns_type, ns1_info) in &ns1_map {
            if let Some(ns2_info) = ns2_map.get(ns_type) {
                if ns1_info.inode == ns2_info.inode {
                    comparison.same_namespaces.push(*ns_type);
                } else {
                    comparison.different_namespaces.push(*ns_type);
                }
            } else {
                comparison.only_in_first.push(*ns_type);
            }
        }

        // Find namespaces only in second process
        for (ns_type, _) in &ns2_map {
            if !ns1_map.contains_key(ns_type) {
                comparison.only_in_second.push(*ns_type);
            }
        }

        Ok(comparison)
    }

    /// Check if two processes share the same namespace
    pub fn shares_namespace(&self, pid1: u32, pid2: u32, namespace_type: NamespaceType) -> ProcessResult<bool> {
        let ns1 = self.get_process_namespaces(pid1)?;
        let ns2 = self.get_process_namespaces(pid2)?;

        let ns1_info = ns1.iter().find(|ns| ns.namespace_type == namespace_type);
        let ns2_info = ns2.iter().find(|ns| ns.namespace_type == namespace_type);

        match (ns1_info, ns2_info) {
            (Some(info1), Some(info2)) => Ok(info1.inode == info2.inode),
            _ => Ok(false),
        }
    }
}

/// Container configuration for creating isolated environments
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Enable network isolation
    pub network_isolation: bool,
    /// Enable user namespace
    pub user_namespace: bool,
    /// Custom hostname
    pub hostname: Option<String>,
    /// Root filesystem path
    pub root_path: Option<PathBuf>,
    /// Path where old root will be mounted
    pub old_root_path: Option<PathBuf>,
    /// UID mappings (inside_uid, outside_uid, length)
    pub uid_mappings: Option<Vec<(u32, u32, u32)>>,
    /// GID mappings (inside_gid, outside_gid, length)
    pub gid_mappings: Option<Vec<(u32, u32, u32)>>,
}

impl ContainerConfig {
    /// Create a new container configuration
    pub fn new() -> Self {
        Self {
            network_isolation: false,
            user_namespace: false,
            hostname: None,
            root_path: None,
            old_root_path: None,
            uid_mappings: None,
            gid_mappings: None,
        }
    }

    /// Enable network isolation
    pub fn with_network_isolation(mut self) -> Self {
        self.network_isolation = true;
        self
    }

    /// Enable user namespace with identity mapping
    pub fn with_user_namespace(mut self) -> Self {
        self.user_namespace = true;
        // Default identity mapping for current user
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        self.uid_mappings = Some(vec![(0, uid, 1)]);
        self.gid_mappings = Some(vec![(0, gid, 1)]);
        self
    }

    /// Set custom hostname
    pub fn with_hostname<S: Into<String>>(mut self, hostname: S) -> Self {
        self.hostname = Some(hostname.into());
        self
    }

    /// Set root filesystem path
    pub fn with_root_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.root_path = Some(path.into());
        self
    }

    /// Set custom UID/GID mappings
    pub fn with_id_mappings(mut self, uid_map: Vec<(u32, u32, u32)>, gid_map: Vec<(u32, u32, u32)>) -> Self {
        self.uid_mappings = Some(uid_map);
        self.gid_mappings = Some(gid_map);
        self
    }

    /// Create a sandboxed configuration
    pub fn sandboxed() -> Self {
        Self::new()
            .with_network_isolation()
            .with_user_namespace()
            .with_hostname("sandbox".to_string())
    }
}

/// Result of comparing namespaces between processes
#[derive(Debug, Clone)]
pub struct NamespaceComparison {
    /// Namespace types that are the same between processes
    pub same_namespaces: Vec<NamespaceType>,
    /// Namespace types that are different between processes
    pub different_namespaces: Vec<NamespaceType>,
    /// Namespace types only present in first process
    pub only_in_first: Vec<NamespaceType>,
    /// Namespace types only present in second process
    pub only_in_second: Vec<NamespaceType>,
}

impl NamespaceComparison {
    /// Check if processes share all namespaces
    pub fn shares_all_namespaces(&self) -> bool {
        self.different_namespaces.is_empty() && 
        self.only_in_first.is_empty() && 
        self.only_in_second.is_empty()
    }

    /// Check if processes are completely isolated
    pub fn completely_isolated(&self) -> bool {
        self.same_namespaces.is_empty()
    }
}

// Additional trait for working with namespaces
use std::os::unix::fs::MetadataExt;

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_namespace_manager_creation() {
        let manager = NamespaceManager::new();
        // Should work on any platform, but only be supported on Linux
        #[cfg(target_os = "linux")]
        assert!(manager.is_supported());
        
        #[cfg(not(target_os = "linux"))]
        assert!(!manager.is_supported());
    }

    #[test]
    fn test_namespace_type_methods() {
        let ns_type = NamespaceType::Mount;
        assert_eq!(ns_type.proc_name(), "mnt");
        
        let ns_type = NamespaceType::Network;
        assert_eq!(ns_type.proc_name(), "net");
    }

    #[test]
    fn test_container_config() {
        let config = ContainerConfig::new()
            .with_network_isolation()
            .with_hostname("test-container");
        
        assert!(config.network_isolation);
        assert_eq!(config.hostname.as_deref(), Some("test-container"));
    }

    #[test]
    fn test_sandboxed_config() {
        let config = ContainerConfig::sandboxed();
        assert!(config.network_isolation);
        assert!(config.user_namespace);
        assert_eq!(config.hostname.as_deref(), Some("sandbox"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_current_namespaces() {
        let manager = NamespaceManager::new();
        let namespaces = manager.get_current_namespaces();
        
        // Should be able to read our own namespaces
        assert!(namespaces.is_ok());
        let namespaces = namespaces.unwrap();
        
        // Should have at least some namespaces
        assert!(!namespaces.is_empty());
        
        // Should include common namespace types
        let ns_types: Vec<_> = namespaces.iter().map(|ns| ns.namespace_type).collect();
        assert!(ns_types.contains(&NamespaceType::Mount));
        assert!(ns_types.contains(&NamespaceType::PID));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_namespace_comparison() {
        let manager = NamespaceManager::new();
        let current_pid = std::process::id();
        
        // Compare process with itself - should share all namespaces
        let comparison = manager.compare_namespaces(current_pid, current_pid);
        assert!(comparison.is_ok());
        
        let comparison = comparison.unwrap();
        assert!(comparison.shares_all_namespaces());
        assert!(!comparison.completely_isolated());
    }
}
