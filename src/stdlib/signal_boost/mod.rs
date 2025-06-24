use crate::error::Error;
// Module error types
pub type Error = ModuleError;

#[derive(Debug, Clone)]
pub enum ModuleError {
    InvalidInput,
    InvalidOperation,
    OperationFailed(String),
    ConfigurationError(String),
    RuntimeError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::InvalidInput => write!(f, "Invalid input"),
            ModuleError::InvalidOperation => write!(f, "Invalid operation"),
            ModuleError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            ModuleError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            ModuleError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}
/// SignalBoost - Enhanced signal handling for CURSED
/// Provides comprehensive OS signal management with graceful shutdown patterns
pub mod core;
pub mod handler;
pub mod graceful;
pub mod multiplexer;
pub mod actions;
pub mod process;
pub mod filtering;
pub mod genZ;
pub mod error;
pub mod context;

// Re-export all public APIs
pub use core::{
    BoostSignal, NotifyHandle,
    // Common signals
    SIGINT, SIGTERM, SIGHUP, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGBUS, SIGFPE,
    SIGKILL, SIGSEGV, SIGPIPE, SIGALRM, SIGCHLD, SIGCONT, SIGSTOP, SIGTSTP,
    SIGTTIN, SIGTTOU, SIGUSR1, SIGUSR2, SIGWINCH,
    // Core functions
    notify, notify_context, stop, reset, ignored
};

pub use handler::{SignalHandler, SignalHandlerConfig};

pub use graceful::{
    GracefulShutdown, ShutdownOptions, ShutdownStatus, ShutdownTask, ShutdownTaskGroup
};

pub use multiplexer::{SignalMultiplexer, MultiplexerHandle};

pub use actions::{
    SignalAction, ignore_action, exit_action, exit_with_code_action,
    log_action, shook_action, chain_actions
};

pub use process::{
    signal_process, signal_group, broadcast, get_targets
};

pub use filtering::{
    filter_signals, throttle_signals, debounce_signals
};

pub use genZ::{
    VibeChecker, vibe_check, yeet_on_signal, no_cap_reload_config
};

pub use error::{SignalBoostError, SignalBoostResult};

// Module initialization
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the SignalBoost module
pub fn initialize() -> SignalBoostResult<()> {
    INIT.call_once(|| {
        // Initialize platform-specific signal handling
        #[cfg(unix)]
        unsafe {
            // Set up signal mask for proper handling
            let mut set = std::mem::zeroed();
            libc::sigemptyset(&mut set);
            libc::pthread_sigmask(libc::SIG_SETMASK, &set, std::ptr::null_mut());
        }
        
        tracing::info!("SignalBoost module initialized");
    });
    
    Ok(())
}

/// Get module statistics and status
pub fn get_statistics() -> ModuleStatistics {
    ModuleStatistics {
        active_handlers: handler::get_active_count(),
        active_multiplexers: multiplexer::get_active_count(),
        graceful_shutdowns: graceful::get_active_count(),
        signals_processed: core::get_signals_processed(),
    }
}

/// Module statistics
#[derive(Debug, Clone)]
pub struct ModuleStatistics {
    pub active_handlers: usize,
    pub active_multiplexers: usize,
    pub graceful_shutdowns: usize,
    pub signals_processed: u64,
}
