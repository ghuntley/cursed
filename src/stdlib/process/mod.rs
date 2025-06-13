/// Process management and system interaction for CURSED
/// 
/// This module provides comprehensive process management functionality including:
/// - Process spawning and lifecycle management
/// - Environment variable handling and inheritance
/// - Process synchronization and waiting
/// - Working directory management
/// - Process termination and cleanup
/// - Cross-platform compatibility (Linux, macOS, Windows)
/// - Inter-process communication primitives
/// - Process monitoring and health checks
/// - Signal handling and process control
/// - Daemon and background process management

pub mod error;
pub mod core;
pub mod info;
pub mod control;
pub mod communication;
pub mod monitoring;
pub mod platform;
pub mod pipes;
pub mod signals;
pub mod daemon;
pub mod environment;
pub mod lifecycle;

// Re-export public API
pub use error::*;
pub use core::*;
pub use info::*;
pub use control::*;
pub use communication::*;
pub use monitoring::*;
pub use platform::*;
pub use pipes::*;
pub use signals::*;
pub use daemon::*;
pub use environment::*;
pub use lifecycle::*;
