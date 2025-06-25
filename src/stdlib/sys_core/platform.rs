/// Platform-specific operations and information
// use crate::stdlib::sys_core::error::{SysCoreResult, platform_error};

/// Get platform name
pub fn get_platform_name() -> String {
    #[cfg(target_os = "linux")]
    return "Linux".to_string();
    
    #[cfg(target_os = "macos")]
    return "macOS".to_string();
    
    #[cfg(target_os = "windows")]
    return "Windows".to_string();
    
    #[cfg(target_os = "freebsd")]
    return "FreeBSD".to_string();
    
    #[cfg(target_os = "openbsd")]
    return "OpenBSD".to_string();
    
    #[cfg(target_os = "netbsd")]
    return "NetBSD".to_string();
    
                  target_os = "freebsd", target_os = "openbsd", target_os = "netbsd")))]
    return "Unknown".to_string();
/// Get platform-specific configuration
pub fn get_platform_config() -> PlatformConfig {
    PlatformConfig {
        supports_mmap: true, // Available on most platforms
        path_separator: if cfg!(windows) { '\\' } else { '/' },
    }
}

/// Platform configuration information
#[derive(Debug, Clone)]
pub struct PlatformConfig {
/// Platform-specific system calls
pub trait PlatformSyscalls {
    fn get_current_user_id(&self) -> SysCoreResult<u32>;
    fn get_current_group_id(&self) -> SysCoreResult<u32>;
    fn get_process_id(&self) -> u32;
    fn get_parent_process_id(&self) -> SysCoreResult<u32>;
    fn get_thread_id(&self) -> u64;
/// Unix platform implementation
#[cfg(unix)]
pub struct UnixPlatform;

#[cfg(unix)]
impl PlatformSyscalls for UnixPlatform {
    fn get_current_user_id(&self) -> SysCoreResult<u32> {
        Ok(unsafe { libc::getuid() })
    fn get_current_group_id(&self) -> SysCoreResult<u32> {
        Ok(unsafe { libc::getgid() })
    fn get_process_id(&self) -> u32 {
        unsafe { libc::getpid() as u32 }
    }
    
    fn get_parent_process_id(&self) -> SysCoreResult<u32> {
        Ok(unsafe { libc::getppid() as u32 })
    fn get_thread_id(&self) -> u64 {
        #[cfg(target_os = "linux")]
        {
            unsafe { libc::syscall(libc::SYS_gettid) as u64 }
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            std::thread::current().id().as_u64().get()
        }
    }
/// Windows platform implementation
#[cfg(windows)]
pub struct WindowsPlatform;

#[cfg(windows)]
impl PlatformSyscalls for WindowsPlatform {
    fn get_current_user_id(&self) -> SysCoreResult<u32> {
        // Windows doesn't have UIDs in the same way as Unix
        // This is a simplified implementation
        Ok(0)
    fn get_current_group_id(&self) -> SysCoreResult<u32> {
        // Windows doesn't have GIDs in the same way as Unix
        Ok(0)
    fn get_process_id(&self) -> u32 {
        std::process::id()
    fn get_parent_process_id(&self) -> SysCoreResult<u32> {
        // Windows implementation would require additional APIs
        Err(platform_error("Parent process ID not implemented on Windows", None))
    fn get_thread_id(&self) -> u64 {
        std::thread::current().id().as_u64().get()
    }
}

/// Get platform-specific syscall implementation
pub fn get_platform_syscalls() -> Box<dyn PlatformSyscalls> {
    #[cfg(unix)]
    return Box::new(UnixPlatform);
    
    #[cfg(windows)]
    return Box::new(WindowsPlatform);
/// Get system endianness
pub fn get_endianness() -> Endianness {
    #[cfg(target_endian = "little")]
    return Endianness::Little;
    
    #[cfg(target_endian = "big")]
    return Endianness::Big;
/// System endianness
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endianness {
/// Get system word size
pub fn get_word_size() -> usize {
    std::mem::size_of::<usize>()
/// Get system pointer size
pub fn get_pointer_size() -> usize {
    std::mem::size_of::<*const ()>()
/// Check if running in a container
pub fn is_running_in_container() -> bool {
    #[cfg(target_os = "linux")]
    {
        // Check for common container indicators
        std::path::Path::new("/.dockerenv").exists() ||
        std::fs::read_to_string("/proc/1/cgroup")
            .map(|content| content.contains("docker") || content.contains("containerd"))
            .unwrap_or(false)
    #[cfg(not(target_os = "linux"))]
    {
        false // Simplified for other platforms
    }
}

/// Get system timezone
pub fn get_system_timezone() -> String {
    #[cfg(unix)]
    {
        std::env::var("TZ").unwrap_or_else(|_| {
            std::fs::read_link("/etc/localtime")
                .ok()
                .and_then(|path| {
                    path.to_str()
                        .and_then(|s| s.split("/zoneinfo/").nth(1))
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "UTC".to_string())
        })
    #[cfg(windows)]
    {
        // Windows timezone detection would require additional APIs
        "UTC".to_string()
    }
}

/// Get system locale
pub fn get_system_locale() -> String {
    std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .unwrap_or_else(|_| "C".to_string())
}
