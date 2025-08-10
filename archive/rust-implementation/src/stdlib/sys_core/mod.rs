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
    poll_fds, select_fds, epoll_operations
// };
pub use process_ops::{
    setpgid, setsid, getpid, getppid, getpgid, getsid
// };
pub use memory_ops::{
    get_page_size, get_memory_info
// };
pub use socket_ops::{
    send_data, recv_data, shutdown_socket
// };
pub use resource_limits::{
    get_resource_limit, set_resource_limit, get_process_limits
// };
pub use syscalls::{
    direct_syscall, safe_syscall
// };

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize system core operations
pub fn initialize() -> SysCoreResult<()> {
    INIT.call_once(|| {
        tracing::info!("SysCore module initialized");
    });
    Ok(())
/// Get system capabilities and features
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
    }
}

/// System capabilities information
#[derive(Debug, Clone)]
pub struct SystemInfo {
}
