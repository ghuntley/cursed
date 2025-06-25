use crate::error::CursedError;
/// Core types and functions for exec_vibez
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use lazy_static::lazy_static;

use super::error::{ExecResult, ExecError, execution_failed, invalid_arguments};
use super::context::VibeContext;

/// Global process tracking
static ACTIVE_PROCESSES: AtomicUsize = AtomicUsize::new(0);
static TOTAL_SPAWNED: AtomicU64 = AtomicU64::new(0);
static PROCESS_REGISTRY: RwLock<HashMap<u32, ProcessInfo>> = RwLock::new(HashMap::new());

/// Process information for tracking
#[derive(Debug, Clone)]
pub struct ProcessInfo {
/// Get active process count
pub fn get_active_process_count() -> usize {
    ACTIVE_PROCESSES.load(Ordering::Relaxed)
/// Get total spawned process count
pub fn get_total_spawned_count() -> u64 {
    TOTAL_SPAWNED.load(Ordering::Relaxed)
/// Register a new process
pub fn register_process(pid: u32, command: &str, args: &[String], working_dir: Option<PathBuf>) {
    ACTIVE_PROCESSES.fetch_add(1, Ordering::Relaxed);
    TOTAL_SPAWNED.fetch_add(1, Ordering::Relaxed);
    
    let info = ProcessInfo {
    
    if let Ok(mut registry) = PROCESS_REGISTRY.write() {
        registry.insert(pid, info);
    }
}

/// Unregister a process
pub fn unregister_process(pid: u32) {
    ACTIVE_PROCESSES.fetch_sub(1, Ordering::Relaxed);
    
    if let Ok(mut registry) = PROCESS_REGISTRY.write() {
        registry.remove(&pid);
    }
}

/// Get process information
pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    if let Ok(registry) = PROCESS_REGISTRY.read() {
        registry.get(&pid).cloned()
    } else {
        None
    }
}

/// Get all active process information
pub fn get_all_process_info() -> Vec<ProcessInfo> {
    if let Ok(registry) = PROCESS_REGISTRY.read() {
        registry.values().cloned().collect()
    } else {
        Vec::new()
    }
}

/// Core command creation functions following the spec API

/// Create a new Cmd instance to execute a given program
pub fn Command(name: &str, args: &[&str]) -> super::cmd::Cmd {
    super::cmd::Cmd::new(name, args)
/// Create a new Cmd with a context for timeout/cancellation
pub fn CommandContext(ctx: VibeContext, name: &str, args: &[&str]) -> super::cmd::Cmd {
    let mut cmd = super::cmd::Cmd::new(name, args);
    cmd.set_context(ctx);
    cmd
/// Look up the executable path for a named program
pub fn LookPath(file: &str) -> ExecResult<String> {
    use std::env;
    use std::path::Path;
    
    // If the file contains a path separator, check if it exists directly
    if file.contains('/') || file.contains('\\') {
        let path = Path::new(file);
        if path.is_file() && is_executable(path) {
            return Ok(path.to_string_lossy().to_string());
        }
        return Err(execution_failed(file, "File not found or not executable"));
    // Search in PATH
    if let Ok(path_var) = env::var("PATH") {
        for path_dir in env::split_paths(&path_var) {
            let full_path = path_dir.join(file);
            if full_path.is_file() && is_executable(&full_path) {
                return Ok(full_path.to_string_lossy().to_string());
            // On Windows, also try with common executable extensions
            #[cfg(windows)]
            {
                for ext in &[".exe", ".cmd", ".bat", ".com"] {
                    let exe_path = path_dir.join(format!("{}{}", file, ext));
                    if exe_path.is_file() && is_executable(&exe_path) {
                        return Ok(exe_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    Err(execution_failed(file, "Command not found in PATH"))
/// Check if a file is executable
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    
    #[cfg(unix)]
    {
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, if the file exists and has certain extensions, consider it executable
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            return matches!(ext.as_str(), "exe" | "cmd" | "bat" | "com" | "ps1");
        }
    }
    
    false
/// Process lifecycle management
pub struct ProcessLifecycle {
impl ProcessLifecycle {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn register_process(&self, pid: u32) {
        if let Ok(mut processes) = self.processes.lock() {
            processes.push(pid);
        }
    }
    
    pub fn unregister_process(&self, pid: u32) {
        if let Ok(mut processes) = self.processes.lock() {
            processes.retain(|&p| p != pid);
        // Run cleanup handlers
        if let Ok(handlers) = self.cleanup_handlers.lock() {
            for handler in handlers.iter() {
                handler(pid);
            }
        }
    pub fn add_cleanup_handler<F>(&self, handler: F)
    where
    {
        if let Ok(mut handlers) = self.cleanup_handlers.lock() {
            handlers.push(Box::new(handler));
        }
    }
    
    pub fn cleanup_all(&self) {
        if let Ok(processes) = self.processes.lock() {
            for &pid in processes.iter() {
                // Attempt graceful termination
                #[cfg(unix)]
                unsafe {
                    libc::kill(pid as i32, libc::SIGTERM);
                #[cfg(windows)]
                {
                    // Windows process termination would go here
                    // This is a simplified approach
                }
            }
        }
    }
// Global process lifecycle manager
lazy_static! {
    pub static ref PROCESS_LIFECYCLE: ProcessLifecycle = ProcessLifecycle::new();
/// Get the global process lifecycle manager
pub fn get_process_lifecycle() -> &'static ProcessLifecycle {
    &PROCESS_LIFECYCLE
