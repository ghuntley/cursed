use std::process::{Command, Output};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::fs;
use crate::error::{CursedError, Result};

/// Execute a command with a timeout to prevent hanging
pub fn execute_with_timeout(
    mut cmd: Command,
    timeout_secs: u64,
    description: &str,
) -> Result<Output> {
    tracing::info!("Executing {}: {:?} (timeout: {}s)", description, cmd, timeout_secs);
    
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = cmd.output();
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(Ok(output)) => {
            tracing::info!("{} completed successfully", description);
            Ok(output)
        },
        Ok(Err(e)) => {
            tracing::error!("{} failed: {}", description, e);
            Err(CursedError::Io(format!("{} failed: {}", description, e)))
        },
        Err(_timeout) => {
            tracing::error!("{} timed out after {} seconds", description, timeout_secs);
            // Try to join the thread to clean up
            let _ = handle.join();
            Err(CursedError::CompilerError(format!(
                "{} timed out after {} seconds. This may be due to missing tools or system configuration issues.",
                description, timeout_secs
            )))
        }
    }
}

/// Check if a tool is available with timeout
pub fn check_tool_availability(tool_path: &str, timeout_secs: u64) -> bool {
    tracing::debug!("Checking availability of tool: {}", tool_path);
    
    let mut cmd = Command::new(tool_path);
    cmd.arg("--version");
    match execute_with_timeout(
        cmd,
        timeout_secs,
        &format!("tool check for {}", tool_path)
    ) {
        Ok(output) => {
            if output.status.success() {
                tracing::info!("Tool {} is available", tool_path);
                true
            } else {
                tracing::debug!("Tool {} check failed with status: {}", tool_path, output.status);
                false
            }
        },
        Err(e) => {
            tracing::debug!("Tool {} check failed: {}", tool_path, e);
            false
        }
    }
}

/// Progress reporter for long-running operations
pub struct ProgressReporter {
    operation: String,
    start_time: std::time::Instant,
}

impl ProgressReporter {
    pub fn new(operation: String) -> Self {
        tracing::info!("Starting operation: {}", operation);
        Self {
            operation,
            start_time: std::time::Instant::now(),
        }
    }
    
    pub fn report(&self, message: &str) {
        let elapsed = self.start_time.elapsed();
        tracing::info!("{} - {} (elapsed: {:.2}s)", self.operation, message, elapsed.as_secs_f64());
    }
    
    pub fn complete(&self) {
        let elapsed = self.start_time.elapsed();
        tracing::info!("{} completed in {:.2}s", self.operation, elapsed.as_secs_f64());
    }
}

/// Read a file with timeout to prevent hanging on slow I/O
pub fn read_file_with_timeout<P: AsRef<Path>>(
    path: P, 
    timeout_secs: u64
) -> Result<String> {
    let path = path.as_ref();
    tracing::debug!("Reading file with timeout: {} (timeout: {}s)", path.display(), timeout_secs);
    
    let (tx, rx) = mpsc::channel();
    let path_clone = path.to_path_buf();
    
    let handle = thread::spawn(move || {
        let result = fs::read_to_string(&path_clone);
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(Ok(content)) => {
            tracing::debug!("Successfully read file: {}", path.display());
            Ok(content)
        },
        Ok(Err(e)) => {
            tracing::error!("Failed to read file {}: {}", path.display(), e);
            Err(CursedError::Io(format!("Failed to read {}: {}", path.display(), e)))
        },
        Err(_timeout) => {
            tracing::error!("File read timed out after {} seconds: {}", timeout_secs, path.display());
            // Try to join the thread to clean up
            let _ = handle.join();
            Err(CursedError::Io(format!(
                "File read timed out after {} seconds: {}. This may be due to slow storage or network filesystem issues.",
                timeout_secs, path.display()
            )))
        }
    }
}

/// Check if a file exists with timeout to prevent hanging on slow filesystems
pub fn file_exists_with_timeout<P: AsRef<Path>>(
    path: P, 
    timeout_secs: u64
) -> bool {
    let path = path.as_ref();
    tracing::debug!("Checking file existence with timeout: {} (timeout: {}s)", path.display(), timeout_secs);
    
    let (tx, rx) = mpsc::channel();
    let path_clone = path.to_path_buf();
    
    let handle = thread::spawn(move || {
        let result = path_clone.exists();
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(exists) => {
            tracing::debug!("File existence check result: {} -> {}", path.display(), exists);
            exists
        },
        Err(_timeout) => {
            tracing::warn!("File existence check timed out after {} seconds: {}", timeout_secs, path.display());
            // Try to join the thread to clean up
            let _ = handle.join();
            false // Assume file doesn't exist if we can't check
        }
    }
}
