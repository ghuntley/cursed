// Process management for CURSED
pub mod core;
pub mod ipc;
pub mod spawn;
pub mod async_process;

// Re-export key types
pub use core::{Process, ProcessId, ProcessHandle, ProcessManager};
pub use ipc::{IpcChannel, IpcMessage, IpcError};
pub use spawn::{ProcessSpawner, SpawnConfig};
pub use async_process::{AsyncProcess, AsyncProcessHandle};
