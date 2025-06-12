/// Platform-specific process management utilities
/// 
/// Process management is crucial for system integration in modern applications.
/// This module provides cross-platform process management capabilities that allow
/// CURSED programs to:
/// 
/// - Spawn and control external processes
/// - Monitor system resources and performance
/// - Implement process-based communication patterns
/// - Build system administration and automation tools
/// - Create microservice architectures with process isolation
/// - Implement distributed computing patterns
/// 
/// Key capabilities:
/// - Cross-platform process spawning and management
/// - Real-time process monitoring and health checks
/// - Resource usage tracking (CPU, memory, I/O)
/// - Signal handling and process control
/// - Platform-specific optimizations (Linux cgroups, Windows services, etc.)
/// 
/// This enables CURSED to be used for system programming, DevOps tooling,
/// container orchestration, and building robust distributed systems.
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use super::error::{ProcessError, ProcessResult};

/// Platform-specific process utilities
pub struct PlatformUtils;

/// Windows-specific process utilities
#[cfg(windows)]
pub mod windows {
    use super::*;
    use std::ptr;
    use winapi::um::processthreadsapi::*;
    use winapi::um::psapi::*;
    use winapi::um::winnt::*;
    use winapi::um::handleapi::*;
    use winapi::shared::minwindef::*;
    
    /// Get process command line on Windows
    pub fn get_process_command_line(pid: u32) -> ProcessResult<String> {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
            if handle == ptr::null_mut() {
                return Err(ProcessError::ProcessNotFound(pid));
            }
            
            // This is a simplified implementation
            // In practice, you'd use QueryFullProcessImageName or similar APIs
            CloseHandle(handle);
            Ok(format!("process_{}", pid))
        }
    }
    
    /// Get process environment variables on Windows
    pub fn get_process_environment(pid: u32) -> ProcessResult<HashMap<String, String>> {
        // Windows implementation would use ReadProcessMemory to read the PEB
        // This is a complex operation and requires elevated privileges
        Ok(HashMap::new())
    }
    
    /// Set process priority on Windows
    pub fn set_process_priority_class(pid: u32, priority_class: u32) -> ProcessResult<()> {
        unsafe {
            let handle = OpenProcess(PROCESS_SET_INFORMATION, FALSE, pid);
            if handle == ptr::null_mut() {
                return Err(ProcessError::ProcessNotFound(pid));
            }
            
            let result = SetPriorityClass(handle, priority_class);
            CloseHandle(handle);
            
            if result == 0 {
                Err(ProcessError::ExecutionFailed("Failed to set priority class".to_string()))
            } else {
                Ok(())
            }
        }
    }
    
    /// Get Windows service status
    pub fn get_service_status(service_name: &str) -> ProcessResult<ServiceStatus> {
        // Implementation would use OpenService and QueryServiceStatus
        Ok(ServiceStatus::Unknown)
    }
    
    /// Start Windows service
    pub fn start_service(service_name: &str) -> ProcessResult<()> {
        // Implementation would use StartService API
        Ok(())
    }
    
    /// Stop Windows service
    pub fn stop_service(service_name: &str) -> ProcessResult<()> {
        // Implementation would use ControlService with SERVICE_CONTROL_STOP
        Ok(())
    }
}

/// Unix-specific process utilities
#[cfg(unix)]
pub mod unix {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    
    /// Get process command line on Unix
    pub fn get_process_command_line(pid: u32) -> ProcessResult<Vec<String>> {
        let cmdline_path = format!("/proc/{}/cmdline", pid);
        let content = fs::read_to_string(&cmdline_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let args: Vec<String> = content
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        Ok(args)
    }
    
    /// Get process environment variables on Unix
    pub fn get_process_environment(pid: u32) -> ProcessResult<HashMap<String, String>> {
        let environ_path = format!("/proc/{}/environ", pid);
        let content = fs::read_to_string(&environ_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let mut env_vars = HashMap::new();
        
        for entry in content.split('\0') {
            if let Some(eq_pos) = entry.find('=') {
                let key = entry[..eq_pos].to_string();
                let value = entry[eq_pos + 1..].to_string();
                env_vars.insert(key, value);
            }
        }
        
        Ok(env_vars)
    }
    
    /// Get process file descriptors on Unix
    pub fn get_process_file_descriptors(pid: u32) -> ProcessResult<Vec<FileDescriptorInfo>> {
        let fd_dir = format!("/proc/{}/fd", pid);
        let entries = fs::read_dir(&fd_dir)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let mut descriptors = Vec::new();
        
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(fd_num) = file_name.parse::<u32>() {
                    let fd_path = entry.path();
                    let target = fs::read_link(&fd_path).unwrap_or_else(|_| fd_path.clone());
                    
                    descriptors.push(FileDescriptorInfo {
                        fd: fd_num,
                        path: target,
                        flags: get_fd_flags(pid, fd_num).unwrap_or(0),
                    });
                }
            }
        }
        
        Ok(descriptors)
    }
    
    /// Get file descriptor flags
    fn get_fd_flags(pid: u32, fd: u32) -> ProcessResult<u32> {
        let fdinfo_path = format!("/proc/{}/fdinfo/{}", pid, fd);
        let content = fs::read_to_string(&fdinfo_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let mut flags = 0u32;
        
        for line in content.lines() {
            if line.starts_with("flags:") {
                if let Some(flags_str) = line.split_whitespace().nth(1) {
                    // Parse octal flags (e.g., "02000002")
                    if let Ok(parsed_flags) = u32::from_str_radix(flags_str, 8) {
                        flags = parsed_flags;
                        break;
                    }
                }
            }
        }
        
        Ok(flags)
    }
    
    /// Create daemon process on Unix
    pub fn create_daemon_process<F>(daemon_fn: F) -> ProcessResult<u32>
    where
        F: FnOnce() -> ProcessResult<()> + Send + 'static,
    {
        // Fork the process
        let pid = unsafe { libc::fork() };
        
        match pid {
            -1 => Err(ProcessError::SystemError(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to fork".to_string()
            )),
            0 => {
                // Child process
                
                // Create new session
                if unsafe { libc::setsid() } == -1 {
                    std::process::exit(1);
                }
                
                // Change working directory to root
                if unsafe { libc::chdir(b"/\0".as_ptr() as *const i8) } == -1 {
                    std::process::exit(1);
                }
                
                // Close standard file descriptors
                unsafe {
                    libc::close(0); // stdin
                    libc::close(1); // stdout
                    libc::close(2); // stderr
                }
                
                // Redirect to /dev/null
                let dev_null = std::ffi::CString::new("/dev/null").unwrap();
                unsafe {
                    let null_fd = libc::open(dev_null.as_ptr(), libc::O_RDWR);
                    if null_fd != -1 {
                        libc::dup2(null_fd, 0);
                        libc::dup2(null_fd, 1);
                        libc::dup2(null_fd, 2);
                        if null_fd > 2 {
                            libc::close(null_fd);
                        }
                    }
                }
                
                // Run daemon function
                if let Err(e) = daemon_fn() {
                    eprintln!("Daemon error: {}", e);
                    std::process::exit(1);
                }
                
                std::process::exit(0);
            }
            child_pid => Ok(child_pid as u32),
        }
    }
    
    /// Get system resource limits
    pub fn get_resource_limits() -> ProcessResult<ResourceLimits> {
        let mut limits = ResourceLimits::default();
        
        unsafe {
            let mut rlimit = libc::rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            };
            
            // Get maximum file descriptors
            if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlimit) == 0 {
                limits.max_file_descriptors = rlimit.rlim_cur;
            }
            
            // Get maximum processes
            if libc::getrlimit(libc::RLIMIT_NPROC, &mut rlimit) == 0 {
                limits.max_processes = rlimit.rlim_cur;
            }
            
            // Get maximum memory
            if libc::getrlimit(libc::RLIMIT_AS, &mut rlimit) == 0 {
                limits.max_virtual_memory = rlimit.rlim_cur;
            }
            
            // Get maximum core file size
            if libc::getrlimit(libc::RLIMIT_CORE, &mut rlimit) == 0 {
                limits.max_core_file_size = rlimit.rlim_cur;
            }
        }
        
        Ok(limits)
    }
    
    /// Set resource limits
    pub fn set_resource_limit(resource: ResourceType, soft_limit: u64, hard_limit: u64) -> ProcessResult<()> {
        let resource_id = match resource {
            ResourceType::FileDescriptors => libc::RLIMIT_NOFILE,
            ResourceType::Processes => libc::RLIMIT_NPROC,
            ResourceType::VirtualMemory => libc::RLIMIT_AS,
            ResourceType::CoreFileSize => libc::RLIMIT_CORE,
            ResourceType::CpuTime => libc::RLIMIT_CPU,
        };
        
        let rlimit = libc::rlimit {
            rlim_cur: soft_limit,
            rlim_max: hard_limit,
        };
        
        let result = unsafe { libc::setrlimit(resource_id, &rlimit) };
        
        if result == 0 {
            Ok(())
        } else {
            Err(ProcessError::SystemError(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to set resource limit".to_string()
            ))
        }
    }
    
    /// Check if process is zombie
    pub fn is_zombie_process(pid: u32) -> ProcessResult<bool> {
        let stat_path = format!("/proc/{}/stat", pid);
        let content = fs::read_to_string(&stat_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let fields: Vec<&str> = content.split_whitespace().collect();
        if fields.len() > 2 {
            Ok(fields[2] == "Z")
        } else {
            Ok(false)
        }
    }
}

/// Linux-specific process utilities
#[cfg(target_os = "linux")]
pub mod linux {
    use super::*;
    use std::fs;
    
    /// Get process cgroup information
    pub fn get_process_cgroup(pid: u32) -> ProcessResult<Vec<CgroupInfo>> {
        let cgroup_path = format!("/proc/{}/cgroup", pid);
        let content = fs::read_to_string(&cgroup_path)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let mut cgroups = Vec::new();
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                cgroups.push(CgroupInfo {
                    hierarchy_id: parts[0].parse().unwrap_or(0),
                    subsystems: parts[1].split(',').map(|s| s.to_string()).collect(),
                    path: parts[2].to_string(),
                });
            }
        }
        
        Ok(cgroups)
    }
    
    /// Get process namespace information
    pub fn get_process_namespaces(pid: u32) -> ProcessResult<Vec<NamespaceInfo>> {
        let ns_dir = format!("/proc/{}/ns", pid);
        let entries = fs::read_dir(&ns_dir)
            .map_err(|_| ProcessError::ProcessNotFound(pid))?;
        
        let mut namespaces = Vec::new();
        
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let ns_path = entry.path();
                if let Ok(target) = fs::read_link(&ns_path) {
                    namespaces.push(NamespaceInfo {
                        ns_type: file_name,
                        inode: extract_namespace_inode(&target).unwrap_or(0),
                        path: target,
                    });
                }
            }
        }
        
        Ok(namespaces)
    }
    
    /// Extract namespace inode from symlink target
    fn extract_namespace_inode(target: &PathBuf) -> Option<u64> {
        let target_str = target.to_string_lossy();
        if target_str.starts_with('[') && target_str.ends_with(']') {
            let inode_str = &target_str[1..target_str.len()-1];
            inode_str.parse().ok()
        } else {
            None
        }
    }
    
    /// Get process security context (SELinux, AppArmor)
    pub fn get_process_security_context(pid: u32) -> ProcessResult<SecurityContext> {
        let attr_path = format!("/proc/{}/attr", pid);
        let mut context = SecurityContext::default();
        
        // Try to read SELinux context
        let selinux_path = format!("{}/current", attr_path);
        if let Ok(selinux_content) = fs::read_to_string(&selinux_path) {
            context.selinux = Some(selinux_content.trim().to_string());
        }
        
        // Try to read AppArmor context  
        let apparmor_path = format!("{}/apparmor/current", attr_path);
        if let Ok(apparmor_content) = fs::read_to_string(&apparmor_path) {
            context.apparmor = Some(apparmor_content.trim().to_string());
        }
        
        Ok(context)
    }
    
    /// Set process CPU affinity
    pub fn set_cpu_affinity(pid: u32, cpu_mask: u64) -> ProcessResult<()> {
        use std::mem;
        
        // Create CPU set from mask
        let mut cpu_set: libc::cpu_set_t = unsafe { mem::zeroed() };
        
        for cpu in 0..64 {
            if (cpu_mask & (1 << cpu)) != 0 {
                unsafe { libc::CPU_SET(cpu, &mut cpu_set); }
            }
        }
        
        let result = unsafe {
            libc::sched_setaffinity(
                pid as libc::pid_t,
                mem::size_of::<libc::cpu_set_t>(),
                &cpu_set
            )
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(ProcessError::SystemError(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to set CPU affinity".to_string()
            ))
        }
    }
    
    /// Get process CPU affinity
    pub fn get_cpu_affinity(pid: u32) -> ProcessResult<u64> {
        use std::mem;
        
        let mut cpu_set: libc::cpu_set_t = unsafe { mem::zeroed() };
        
        let result = unsafe {
            libc::sched_getaffinity(
                pid as libc::pid_t,
                mem::size_of::<libc::cpu_set_t>(),
                &mut cpu_set
            )
        };
        
        if result != 0 {
            return Err(ProcessError::SystemError(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to get CPU affinity".to_string()
            ));
        }
        
        let mut mask = 0u64;
        for cpu in 0..64 {
            if unsafe { libc::CPU_ISSET(cpu, &cpu_set) } {
                mask |= 1 << cpu;
            }
        }
        
        Ok(mask)
    }
}

/// macOS-specific process utilities
#[cfg(target_os = "macos")]
pub mod macos {
    use super::*;
    
    /// Get process information using sysctl
    pub fn get_process_info_sysctl(pid: u32) -> ProcessResult<MacOSProcessInfo> {
        // This would use sysctl(3) with CTL_KERN, KERN_PROC, KERN_PROC_PID
        // For now, return placeholder data
        Ok(MacOSProcessInfo {
            pid,
            name: format!("process_{}", pid),
            executable_path: None,
            parent_pid: None,
            process_group_id: None,
            session_id: None,
            controlling_terminal: None,
        })
    }
    
    /// Get process memory regions
    pub fn get_process_memory_regions(pid: u32) -> ProcessResult<Vec<MemoryRegion>> {
        // This would use vm_region_recurse_64 or similar
        Ok(Vec::new())
    }
    
    /// Get process Mach port information
    pub fn get_process_mach_ports(pid: u32) -> ProcessResult<Vec<MachPortInfo>> {
        // This would use mach_port_names or similar
        Ok(Vec::new())
    }
}

// Common data structures

#[derive(Debug, Clone)]
pub struct FileDescriptorInfo {
    pub fd: u32,
    pub path: PathBuf,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_file_descriptors: u64,
    pub max_processes: u64,
    pub max_virtual_memory: u64,
    pub max_core_file_size: u64,
    pub max_cpu_time: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_file_descriptors: 1024,
            max_processes: 32768,
            max_virtual_memory: u64::MAX,
            max_core_file_size: 0,
            max_cpu_time: u64::MAX,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    FileDescriptors,
    Processes,
    VirtualMemory,
    CoreFileSize,
    CpuTime,
}

#[cfg(windows)]
#[derive(Debug, Clone)]
pub enum ServiceStatus {
    Stopped,
    StartPending,
    StopPending,
    Running,
    ContinuePending,
    PausePending,
    Paused,
    Unknown,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone)]
pub struct CgroupInfo {
    pub hierarchy_id: u32,
    pub subsystems: Vec<String>,
    pub path: String,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone)]
pub struct NamespaceInfo {
    pub ns_type: String,
    pub inode: u64,
    pub path: PathBuf,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Default)]
pub struct SecurityContext {
    pub selinux: Option<String>,
    pub apparmor: Option<String>,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone)]
pub struct MacOSProcessInfo {
    pub pid: u32,
    pub name: String,
    pub executable_path: Option<PathBuf>,
    pub parent_pid: Option<u32>,
    pub process_group_id: Option<u32>,
    pub session_id: Option<u32>,
    pub controlling_terminal: Option<String>,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start_address: u64,
    pub size: u64,
    pub protection: u32,
    pub max_protection: u32,
    pub inheritance: u32,
    pub is_shared: bool,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone)]
pub struct MachPortInfo {
    pub name: u32,
    pub port_type: String,
    pub refs: u32,
}

impl PlatformUtils {
    /// Get platform-specific process information
    pub fn get_platform_info(pid: u32) -> ProcessResult<PlatformProcessInfo> {
        #[cfg(target_os = "linux")]
        {
            let cgroups = linux::get_process_cgroup(pid)?;
            let namespaces = linux::get_process_namespaces(pid)?;
            let security_context = linux::get_process_security_context(pid)?;
            
            Ok(PlatformProcessInfo::Linux {
                command_line: unix::get_process_command_line(pid)?,
                environment: unix::get_process_environment(pid)?,
                file_descriptors: unix::get_process_file_descriptors(pid)?,
                cgroups,
                namespaces,
                security_context,
            })
        }
        
        #[cfg(target_os = "macos")]
        {
            Ok(PlatformProcessInfo::MacOS {
                sysctl_info: macos::get_process_info_sysctl(pid)?,
                memory_regions: macos::get_process_memory_regions(pid)?,
                mach_ports: macos::get_process_mach_ports(pid)?,
            })
        }
        
        #[cfg(all(unix, not(target_os = "linux"), not(target_os = "macos")))]
        {
            Ok(PlatformProcessInfo::Unix {
                command_line: unix::get_process_command_line(pid)?,
                environment: unix::get_process_environment(pid)?,
                file_descriptors: unix::get_process_file_descriptors(pid)?,
            })
        }
        
        #[cfg(windows)]
        {
            Ok(PlatformProcessInfo::Windows {
                command_line: windows::get_process_command_line(pid)?,
                environment: windows::get_process_environment(pid)?,
            })
        }
        
        #[cfg(not(any(windows, unix)))]
        {
            Err(ProcessError::PlatformError("Unsupported platform".to_string()))
        }
    }
    
    /// Check if running with elevated privileges
    pub fn is_elevated() -> bool {
        #[cfg(windows)]
        {
            // Would check for admin privileges on Windows
            false
        }
        
        #[cfg(unix)]
        {
            unsafe { libc::geteuid() == 0 }
        }
        
        #[cfg(not(any(windows, unix)))]
        {
            false
        }
    }
    
    /// Get current user information
    pub fn get_current_user() -> ProcessResult<UserInfo> {
        #[cfg(unix)]
        {
            let uid = unsafe { libc::getuid() };
            let gid = unsafe { libc::getgid() };
            
            Ok(UserInfo {
                uid: Some(uid),
                gid: Some(gid),
                username: std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
                home_directory: std::env::var("HOME").ok().map(PathBuf::from),
            })
        }
        
        #[cfg(windows)]
        {
            Ok(UserInfo {
                uid: None,
                gid: None,
                username: std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string()),
                home_directory: std::env::var("USERPROFILE").ok().map(PathBuf::from),
            })
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Err(ProcessError::PlatformError("Unsupported platform".to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlatformProcessInfo {
    #[cfg(windows)]
    Windows {
        command_line: String,
        environment: HashMap<String, String>,
    },
    
    #[cfg(unix)]
    Unix {
        command_line: Vec<String>,
        environment: HashMap<String, String>,
        file_descriptors: Vec<FileDescriptorInfo>,
    },
    
    #[cfg(target_os = "linux")]
    Linux {
        command_line: Vec<String>,
        environment: HashMap<String, String>,
        file_descriptors: Vec<FileDescriptorInfo>,
        cgroups: Vec<CgroupInfo>,
        namespaces: Vec<NamespaceInfo>,
        security_context: SecurityContext,
    },
    
    #[cfg(target_os = "macos")]
    MacOS {
        sysctl_info: MacOSProcessInfo,
        memory_regions: Vec<MemoryRegion>,
        mach_ports: Vec<MachPortInfo>,
    },
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub username: String,
    pub home_directory: Option<PathBuf>,
}

/// Get platform name
pub fn get_platform_name() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";
    
    #[cfg(target_os = "linux")]
    return "linux";
    
    #[cfg(target_os = "macos")]
    return "macos";
    
    #[cfg(target_os = "freebsd")]
    return "freebsd";
    
    #[cfg(target_os = "openbsd")]
    return "openbsd";
    
    #[cfg(target_os = "netbsd")]
    return "netbsd";
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux", 
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )))]
    return "unknown";
}

/// Check if platform supports feature
pub fn supports_feature(feature: PlatformFeature) -> bool {
    match feature {
        PlatformFeature::Signals => cfg!(unix),
        PlatformFeature::ProcessGroups => cfg!(unix),
        PlatformFeature::ResourceLimits => cfg!(unix),
        PlatformFeature::FileDescriptors => cfg!(unix),
        PlatformFeature::WindowsServices => cfg!(windows),
        PlatformFeature::Cgroups => cfg!(target_os = "linux"),
        PlatformFeature::Namespaces => cfg!(target_os = "linux"),
        PlatformFeature::SELinux => cfg!(target_os = "linux"),
        PlatformFeature::AppArmor => cfg!(target_os = "linux"),
        PlatformFeature::MachPorts => cfg!(target_os = "macos"),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlatformFeature {
    Signals,
    ProcessGroups,
    ResourceLimits,
    FileDescriptors,
    WindowsServices,
    Cgroups,
    Namespaces,
    SELinux,
    AppArmor,
    MachPorts,
}
