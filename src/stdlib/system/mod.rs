/// System utilities module for CURSED
/// 
/// This module provides system-level functionality including process management,
/// signal handling, and system information retrieval.

pub mod process;
pub mod error;

// Re-export process management functionality
pub use process::{
    // Process spawning
    spawn_process, spawn_with_args, spawn_with_env, spawn_with_config,
    
    // Process control
    kill_process, wait_for_process, process_exists, terminate_process, interrupt_process,
    
    // Process information
    get_pid, get_ppid, get_process_info, list_processes, get_current_pid, get_parent_pid,
    
    // Signal handling
    send_signal, handle_signals, install_signal_handler, remove_signal_handler,
    
    // Exit code management
    exit_with_code, get_exit_code, ExitCode,
    
    // Command execution helpers
    execute_command, execute_command_with_output, pipe_commands, run_shell_command,
    
    // Process structures and types
    Process, ProcessInfo, ProcessConfig, Signal, ProcessStatus, ProcessHandle,
    
    // Async operations
    spawn_async, wait_async, ProcessFuture,
};

// Re-export error types
pub use error::{
    ProcessError, ProcessResult, ProcessErrorKind,
    spawn_error, wait_error, signal_error, permission_error,
};
