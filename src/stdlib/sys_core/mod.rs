use crate::error::CursedError;
/// System core operations module for CURSED
/// 
/// This module provides low-level system operations including:
/// - File descriptor management with platform-specific operations
/// - Low-level process management (fork, exec, wait)
/// - Memory mapping operations
/// - Network socket operations at syscall level
/// - Resource limits and system capabilities
/// - Direct system call access where needed

pub mod error;
pub mod fd_ops;
pub mod process_ops;
pub mod memory_ops;
pub mod socket_ops;
pub mod resource_limits;
pub mod syscalls;
pub mod platform;

// Re-export core functionality
pub use error::{SysCoreError, SysCoreResult};
pub use fd_ops::{
    FileDescriptor, FileDescriptorFlags, FileDescriptorTable,
    create_fd, close_fd, duplicate_fd, get_fd_flags, set_fd_flags,
    poll_fds, select_fds, epoll_operations
};
pub use process_ops::{
    ProcessId, ThreadId, ProcessGroup, SessionId,
    fork_process, exec_process, wait_process, kill_process,
    setpgid, setsid, getpid, getppid, getpgid, getsid
};
pub use memory_ops::{
    MemoryMap, MemoryProtection, MemoryFlags,
    mmap_memory, munmap_memory, mprotect_memory, mlock_memory, munlock_memory,
    get_page_size, get_memory_info
};
pub use socket_ops::{
    SocketDomain, SocketType, SocketProtocol, SocketAddress,
    create_socket, bind_socket, listen_socket, accept_socket, connect_socket,
    send_data, recv_data, shutdown_socket
};
pub use resource_limits::{
    ResourceType, ResourceLimit, ProcessLimits,
    get_resource_limit, set_resource_limit, get_process_limits
};
pub use syscalls::{
    SystemCall, SystemCallResult,
    direct_syscall, safe_syscall
};

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize system core operations
pub fn initialize() -> SysCoreResult<()> {
    INIT.call_once(|| {
        tracing::info!("SysCore module initialized");
    });
    Ok(())
}

/// Get system capabilities and features
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        platform: platform::get_platform_name(),
        page_size: memory_ops::get_page_size(),
        max_open_files: resource_limits::get_max_open_files(),
        supports_epoll: fd_ops::supports_epoll(),
        supports_fork: process_ops::supports_fork(),
        supports_mmap: memory_ops::supports_mmap(),
    }
}

/// System capabilities information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub platform: String,
    pub page_size: usize,
    pub max_open_files: u64,
    pub supports_epoll: bool,
    pub supports_fork: bool,
    pub supports_mmap: bool,
}
