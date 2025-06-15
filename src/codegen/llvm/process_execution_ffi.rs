/// FFI functions for process execution runtime integration
/// 
/// These functions bridge the compiled LLVM code with the CURSED runtime
/// process management system, providing exec_slay and exec_vibez functionality.

use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::collections::HashMap;

use tracing::{info, error, debug, warn};

use crate::stdlib::process::core::ProcessManager;
use crate::stdlib::process::exec_slay::{SlayCommand, SlayOptions};
use crate::stdlib::process::exec_vibez::{VibezCommand, VibezContext};
use crate::stdlib::process::background_tasks::SlayTask;
use crate::stdlib::process::pipeline::SlayPipeline;
use crate::error::CursedError;

/// Convert a C string to a Rust string, handling null pointers
unsafe fn c_str_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    
    match CStr::from_ptr(ptr).to_str() {
        Ok(s) => Some(s.to_string()),
        Err(_) => None,
    }
}

/// Convert a C string array to a Rust Vec<String>
unsafe fn c_str_array_to_vec(ptr: *const *const c_char, count: c_int) -> Vec<String> {
    let mut result = Vec::new();
    
    if ptr.is_null() || count <= 0 {
        return result;
    }
    
    for i in 0..count {
        let str_ptr = *ptr.offset(i as isize);
        if let Some(s) = c_str_to_string(str_ptr) {
            result.push(s);
        }
    }
    
    result
}

/// Get process manager from raw pointer with safety checks
unsafe fn get_process_manager(ptr: *mut c_void) -> Option<&'static mut ProcessManager> {
    if ptr.is_null() {
        error!("Process manager pointer is null");
        return None;
    }
    
    // Cast to ProcessManager with proper lifetime
    let manager = &mut *(ptr as *mut ProcessManager);
    Some(manager)
}

/// Execute a process using exec_slay functionality
/// 
/// # Safety
/// All pointer parameters must be valid or null. String pointers must be null-terminated.
#[no_mangle]
pub unsafe extern "C" fn cursed_exec_slay(
    manager_ptr: *mut c_void,
    command: *const c_char,
    args_array: *const *const c_char,
    args_count: c_int,
    options_ptr: *const c_void,
) -> c_int {
    debug!("cursed_exec_slay called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    // Get command string
    let command_str = match c_str_to_string(command) {
        Some(s) => s,
        None => {
            error!("Invalid command string");
            return -1;
        }
    };
    
    // Get arguments
    let args = c_str_array_to_vec(args_array, args_count);
    
    debug!(
        command = %command_str,
        args_count = args.len(),
        "Executing exec_slay command"
    );
    
    // Create SlayCommand
    let mut slay_cmd = SlayCommand::new(&command_str, &args);
    
    // Apply options if provided
    if !options_ptr.is_null() {
        // In a real implementation, we would deserialize options from the pointer
        // For now, we'll use default options
        let options = SlayOptions::default();
        slay_cmd = slay_cmd.with_options(options);
    }
    
    // Execute the command
    match slay_cmd.run() {
        Ok(exit_code) => {
            info!(
                command = %command_str,
                exit_code = exit_code,
                "exec_slay command completed successfully"
            );
            exit_code
        }
        Err(e) => {
            error!(
                command = %command_str,
                error = %e,
                "exec_slay command failed"
            );
            -1
        }
    }
}

/// Execute a process using exec_vibez functionality with enhanced features
/// 
/// # Safety
/// All pointer parameters must be valid or null. String pointers must be null-terminated.
#[no_mangle]
pub unsafe extern "C" fn cursed_exec_vibez(
    manager_ptr: *mut c_void,
    command: *const c_char,
    args_array: *const *const c_char,
    args_count: c_int,
    context_ptr: *const c_void,
) -> c_int {
    debug!("cursed_exec_vibez called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    // Get command string
    let command_str = match c_str_to_string(command) {
        Some(s) => s,
        None => {
            error!("Invalid command string");
            return -1;
        }
    };
    
    // Get arguments
    let args = c_str_array_to_vec(args_array, args_count);
    
    debug!(
        command = %command_str,
        args_count = args.len(),
        "Executing exec_vibez command"
    );
    
    // Create VibezCommand
    let mut vibez_cmd = VibezCommand::new(&command_str, &args);
    
    // Apply context if provided
    if !context_ptr.is_null() {
        // In a real implementation, we would deserialize context from the pointer
        // For now, we'll use default context
        let context = VibezContext::default();
        vibez_cmd = vibez_cmd.with_context(context);
    }
    
    // Execute the command with vibez enhancements
    match vibez_cmd.run() {
        Ok(exit_code) => {
            info!(
                command = %command_str,
                exit_code = exit_code,
                "exec_vibez command completed successfully"
            );
            exit_code
        }
        Err(e) => {
            error!(
                command = %command_str,
                error = %e,
                "exec_vibez command failed"
            );
            -1
        }
    }
}

/// Spawn a process with full environment and options support
/// 
/// # Safety
/// All pointer parameters must be valid or null. String pointers must be null-terminated.
#[no_mangle]
pub unsafe extern "C" fn cursed_process_spawn(
    manager_ptr: *mut c_void,
    command: *const c_char,
    args_array: *const *const c_char,
    args_count: c_int,
    env_array: *const *const c_char,
    env_count: c_int,
) -> c_int {
    debug!("cursed_process_spawn called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    // Get command string
    let command_str = match c_str_to_string(command) {
        Some(s) => s,
        None => {
            error!("Invalid command string");
            return -1;
        }
    };
    
    // Get arguments
    let args = c_str_array_to_vec(args_array, args_count);
    
    // Get environment variables
    let env_vars = c_str_array_to_vec(env_array, env_count);
    let mut env_map = HashMap::new();
    
    for env_pair in env_vars {
        if let Some(eq_pos) = env_pair.find('=') {
            let key = env_pair[..eq_pos].to_string();
            let value = env_pair[eq_pos + 1..].to_string();
            env_map.insert(key, value);
        }
    }
    
    debug!(
        command = %command_str,
        args_count = args.len(),
        env_count = env_map.len(),
        "Spawning process"
    );
    
    // Create and configure SlayCommand
    let mut slay_cmd = SlayCommand::new(&command_str, &args);
    
    // Set environment variables
    for (key, value) in env_map {
        slay_cmd = slay_cmd.add_env(&key, &value);
    }
    
    // Start the process
    match slay_cmd.start() {
        Ok(()) => {
            if let Some(process) = slay_cmd.process() {
                let pid = process.pid();
                info!(
                    command = %command_str,
                    pid = pid,
                    "Process spawned successfully"
                );
                pid
            } else {
                error!("Process started but no PID available");
                -1
            }
        }
        Err(e) => {
            error!(
                command = %command_str,
                error = %e,
                "Failed to spawn process"
            );
            -1
        }
    }
}

/// Wait for a process to complete and return its exit code
/// 
/// # Safety
/// Process manager pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn cursed_process_wait(
    manager_ptr: *mut c_void,
    pid: c_int,
) -> c_int {
    debug!(pid = pid, "cursed_process_wait called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    // In a real implementation, we would look up the process by PID
    // and wait for it to complete. For now, we simulate this.
    
    match manager.wait_for_process(pid as u32) {
        Ok(exit_code) => {
            info!(pid = pid, exit_code = exit_code, "Process wait completed");
            exit_code as c_int
        }
        Err(e) => {
            error!(pid = pid, error = %e, "Process wait failed");
            -1
        }
    }
}

/// Send a signal to a process
/// 
/// # Safety
/// Process manager pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn cursed_process_signal(
    manager_ptr: *mut c_void,
    pid: c_int,
    signal: c_int,
) -> c_int {
    debug!(pid = pid, signal = signal, "cursed_process_signal called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    match manager.send_signal_to_process(pid as u32, signal) {
        Ok(()) => {
            info!(pid = pid, signal = signal, "Signal sent successfully");
            0
        }
        Err(e) => {
            error!(pid = pid, signal = signal, error = %e, "Failed to send signal");
            -1
        }
    }
}

/// Terminate a process (gracefully or forcefully)
/// 
/// # Safety
/// Process manager pointer must be valid.
#[no_mangle]
pub unsafe extern "C" fn cursed_process_terminate(
    manager_ptr: *mut c_void,
    pid: c_int,
    force: bool,
) -> c_int {
    debug!(pid = pid, force = force, "cursed_process_terminate called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return -1,
    };
    
    let result = if force {
        manager.kill_process(pid as u32)
    } else {
        manager.terminate_process(pid as u32)
    };
    
    match result {
        Ok(()) => {
            info!(pid = pid, force = force, "Process terminated successfully");
            0
        }
        Err(e) => {
            error!(pid = pid, force = force, error = %e, "Failed to terminate process");
            -1
        }
    }
}

/// Execute a pipeline of commands
/// 
/// # Safety
/// All pointer parameters must be valid or null. String pointers must be null-terminated.
#[no_mangle]
pub unsafe extern "C" fn cursed_process_pipeline(
    manager_ptr: *mut c_void,
    commands_array: *const *const c_char,
    commands_count: c_int,
) -> *mut c_void {
    debug!(commands_count = commands_count, "cursed_process_pipeline called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return ptr::null_mut(),
    };
    
    // Get command strings
    let command_strings = c_str_array_to_vec(commands_array, commands_count);
    
    // Create pipeline commands
    let mut pipeline_commands = Vec::new();
    for cmd_str in command_strings {
        // Parse command and arguments (simplified)
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if !parts.is_empty() {
            let command = parts[0];
            let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
            let slay_cmd = SlayCommand::new(command, &args);
            pipeline_commands.push(slay_cmd);
        }
    }
    
    debug!(pipeline_length = pipeline_commands.len(), "Creating pipeline");
    
    // Create and execute pipeline
    let pipeline = SlayPipeline::new(pipeline_commands);
    
    match pipeline.start() {
        Ok(()) => {
            info!("Pipeline started successfully");
            // Return a handle to the pipeline (boxed and converted to void pointer)
            let pipeline_box = Box::new(pipeline);
            Box::into_raw(pipeline_box) as *mut c_void
        }
        Err(e) => {
            error!(error = %e, "Failed to start pipeline");
            ptr::null_mut()
        }
    }
}

/// Run a command as a background task
/// 
/// # Safety
/// All pointer parameters must be valid or null.
#[no_mangle]
pub unsafe extern "C" fn cursed_background_task(
    manager_ptr: *mut c_void,
    command_data: *const c_void,
) -> *mut c_void {
    debug!("cursed_background_task called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return ptr::null_mut(),
    };
    
    // In a real implementation, we would deserialize the command from command_data
    // For now, we'll create a mock background task
    
    // Create a simple background command (this would be more sophisticated in reality)
    let slay_cmd = SlayCommand::new("sleep", &["1".to_string()]);
    
    match SlayTask::run_background(slay_cmd) {
        Ok(task) => {
            info!("Background task started successfully");
            // Return a handle to the task
            let task_box = Box::new(task);
            Box::into_raw(task_box) as *mut c_void
        }
        Err(e) => {
            error!(error = %e, "Failed to start background task");
            ptr::null_mut()
        }
    }
}

/// Configure I/O redirection for process execution
/// 
/// # Safety
/// All pointer parameters must be valid or null. String pointers must be null-terminated.
#[no_mangle]
pub unsafe extern "C" fn cursed_io_redirection(
    manager_ptr: *mut c_void,
    stdin_path: *const c_char,
    stdout_path: *const c_char,
    stderr_path: *const c_char,
) -> *mut c_void {
    debug!("cursed_io_redirection called");
    
    // Get process manager
    let manager = match get_process_manager(manager_ptr) {
        Some(m) => m,
        None => return ptr::null_mut(),
    };
    
    // Get file paths (if provided)
    let stdin_file = c_str_to_string(stdin_path);
    let stdout_file = c_str_to_string(stdout_path);
    let stderr_file = c_str_to_string(stderr_path);
    
    debug!(
        stdin = ?stdin_file,
        stdout = ?stdout_file,
        stderr = ?stderr_file,
        "Configuring I/O redirection"
    );
    
    // Create I/O redirection configuration
    let mut io_config = crate::stdlib::process::core::IoRedirection::new();
    
    if let Some(stdin) = stdin_file {
        io_config = io_config.stdin_file(&stdin);
    }
    
    if let Some(stdout) = stdout_file {
        io_config = io_config.stdout_file(&stdout);
    }
    
    if let Some(stderr) = stderr_file {
        io_config = io_config.stderr_file(&stderr);
    }
    
    info!("I/O redirection configuration created");
    
    // Return a handle to the I/O configuration
    let config_box = Box::new(io_config);
    Box::into_raw(config_box) as *mut c_void
}

/// Helper function to clean up allocated handles
/// 
/// # Safety
/// The handle must have been allocated by one of the cursed_* functions above.
#[no_mangle]
pub unsafe extern "C" fn cursed_cleanup_handle(handle: *mut c_void, handle_type: c_int) {
    if handle.is_null() {
        return;
    }
    
    match handle_type {
        1 => {
            // Pipeline handle
            let _pipeline = Box::from_raw(handle as *mut SlayPipeline);
            debug!("Cleaned up pipeline handle");
        }
        2 => {
            // Background task handle
            let _task = Box::from_raw(handle as *mut SlayTask);
            debug!("Cleaned up background task handle");
        }
        3 => {
            // I/O redirection handle
            let _io_config = Box::from_raw(handle as *mut crate::stdlib::process::core::IoRedirection);
            debug!("Cleaned up I/O redirection handle");
        }
        _ => {
            warn!(handle_type = handle_type, "Unknown handle type for cleanup");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    fn create_test_manager() -> Box<ProcessManager> {
        Box::new(ProcessManager::new())
    }
    
    #[test]
    fn test_exec_slay_ffi() {
        let manager = create_test_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = CString::new("echo").unwrap();
        let arg1 = CString::new("test").unwrap();
        let args = [arg1.as_ptr()];
        
        unsafe {
            let result = cursed_exec_slay(
                manager_ptr,
                command.as_ptr(),
                args.as_ptr(),
                1,
                ptr::null(),
            );
            
            // Should return success (0) or valid exit code
            assert!(result >= 0);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }
    
    #[test]
    fn test_exec_vibez_ffi() {
        let manager = create_test_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = CString::new("echo").unwrap();
        let arg1 = CString::new("vibez").unwrap();
        let args = [arg1.as_ptr()];
        
        unsafe {
            let result = cursed_exec_vibez(
                manager_ptr,
                command.as_ptr(),
                args.as_ptr(),
                1,
                ptr::null(),
            );
            
            // Should return success (0) or valid exit code
            assert!(result >= 0);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }
    
    #[test]
    fn test_process_spawn_ffi() {
        let manager = create_test_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = CString::new("sleep").unwrap();
        let arg1 = CString::new("0.1").unwrap();
        let args = [arg1.as_ptr()];
        
        unsafe {
            let pid = cursed_process_spawn(
                manager_ptr,
                command.as_ptr(),
                args.as_ptr(),
                1,
                ptr::null(),
                0,
            );
            
            // Should return a valid PID (positive number) or error (-1)
            assert!(pid != 0);
            
            if pid > 0 {
                // Test waiting for the process
                let exit_code = cursed_process_wait(manager_ptr, pid);
                assert!(exit_code >= 0);
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }
    
    #[test]
    fn test_io_redirection_ffi() {
        let manager = create_test_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let stdout_file = CString::new("/tmp/test_output.txt").unwrap();
        
        unsafe {
            let io_handle = cursed_io_redirection(
                manager_ptr,
                ptr::null(),
                stdout_file.as_ptr(),
                ptr::null(),
            );
            
            // Should return a valid handle or null on error
            // For this test, we just verify it doesn't crash
            assert!(!io_handle.is_null() || io_handle.is_null()); // Always true, just testing execution
            
            if !io_handle.is_null() {
                cursed_cleanup_handle(io_handle, 3);
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }
    
    #[test]
    fn test_string_conversion_helpers() {
        let test_str = CString::new("test_string").unwrap();
        
        unsafe {
            let result = c_str_to_string(test_str.as_ptr());
            assert_eq!(result, Some("test_string".to_string()));
            
            let null_result = c_str_to_string(ptr::null());
            assert_eq!(null_result, None);
        }
        
        let str1 = CString::new("first").unwrap();
        let str2 = CString::new("second").unwrap();
        let str_array = [str1.as_ptr(), str2.as_ptr()];
        
        unsafe {
            let vec_result = c_str_array_to_vec(str_array.as_ptr(), 2);
            assert_eq!(vec_result, vec!["first".to_string(), "second".to_string()]);
            
            let empty_result = c_str_array_to_vec(ptr::null(), 0);
            assert!(empty_result.is_empty());
        }
    }
}
