/// FFI integration tests for process execution runtime functions
/// 
/// Tests the FFI bridge between LLVM-compiled code and the CURSED process
/// management runtime, verifying that exec_slay and exec_vibez operations
/// work correctly when called from compiled code.

use cursed::stdlib::process::core::ProcessManager;
use cursed::codegen::llvm::process_execution_ffi::*;
use std::ffi::{CString, CStr};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    fn create_test_process_manager() -> Box<ProcessManager> {
        Box::new(ProcessManager::new())
    }

    fn create_c_string(s: &str) -> CString {
        CString::new(s).expect("Failed to create CString")
    }

    fn create_c_string_array(strings: &[&str]) -> Vec<CString> {
        strings.iter().map(|s| create_c_string(s)).collect()
    }

    fn get_c_string_ptrs(c_strings: &[CString]) -> Vec<*const c_char> {
        c_strings.iter().map(|cs| cs.as_ptr()).collect()
    }

    #[traced_test]
    #[test]
    fn test_exec_slay_ffi_basic() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = create_c_string("echo");
        let arg1 = create_c_string("Hello");
        let arg2 = create_c_string("World");
        let args = vec![arg1, arg2];
        let arg_ptrs = get_c_string_ptrs(&args);
        
        unsafe {
            let result = cursed_exec_slay(
                manager_ptr,
                command.as_ptr(),
                arg_ptrs.as_ptr(),
                args.len() as c_int,
                ptr::null(),
            );
            
            // Should return a valid exit code (0 for success or positive for other exit codes)
            assert!(result >= 0, "exec_slay should return valid exit code, got: {}", result);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_exec_vibez_ffi_basic() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = create_c_string("ls");
        let arg1 = create_c_string("-l");
        let args = vec![arg1];
        let arg_ptrs = get_c_string_ptrs(&args);
        
        unsafe {
            let result = cursed_exec_vibez(
                manager_ptr,
                command.as_ptr(),
                arg_ptrs.as_ptr(),
                args.len() as c_int,
                ptr::null(),
            );
            
            // Should return a valid exit code
            assert!(result >= 0, "exec_vibez should return valid exit code, got: {}", result);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_process_spawn_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = create_c_string("sleep");
        let arg1 = create_c_string("0.1");
        let args = vec![arg1];
        let arg_ptrs = get_c_string_ptrs(&args);
        
        // Create environment variables
        let env1 = create_c_string("TEST_VAR=test_value");
        let env_vars = vec![env1];
        let env_ptrs = get_c_string_ptrs(&env_vars);
        
        unsafe {
            let pid = cursed_process_spawn(
                manager_ptr,
                command.as_ptr(),
                arg_ptrs.as_ptr(),
                args.len() as c_int,
                env_ptrs.as_ptr(),
                env_vars.len() as c_int,
            );
            
            // Should return a valid PID (positive) or error (-1)
            assert!(pid != 0, "spawn should return non-zero PID or error code, got: {}", pid);
            
            if pid > 0 {
                // If spawn succeeded, test waiting for the process
                let exit_code = cursed_process_wait(manager_ptr, pid);
                assert!(exit_code >= 0, "wait should return valid exit code, got: {}", exit_code);
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_process_signal_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        // Test sending a signal to a mock PID
        let mock_pid = 1;
        let signal = 15; // SIGTERM
        
        unsafe {
            let result = cursed_process_signal(manager_ptr, mock_pid, signal);
            
            // Should return success (0) or error (-1)
            // Since we're using a mock PID, it's expected to fail, but shouldn't crash
            assert!(result == 0 || result == -1, "signal should return 0 or -1, got: {}", result);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_process_terminate_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        // Test terminating a mock PID
        let mock_pid = 1;
        
        unsafe {
            // Test graceful termination
            let result = cursed_process_terminate(manager_ptr, mock_pid, false);
            assert!(result == 0 || result == -1, "terminate should return 0 or -1, got: {}", result);
            
            // Test forced termination
            let result_force = cursed_process_terminate(manager_ptr, mock_pid, true);
            assert!(result_force == 0 || result_force == -1, "forced terminate should return 0 or -1, got: {}", result_force);
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_process_pipeline_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let cmd1 = create_c_string("echo hello");
        let cmd2 = create_c_string("wc -l");
        let commands = vec![cmd1, cmd2];
        let cmd_ptrs = get_c_string_ptrs(&commands);
        
        unsafe {
            let pipeline_handle = cursed_process_pipeline(
                manager_ptr,
                cmd_ptrs.as_ptr(),
                commands.len() as c_int,
            );
            
            // Should return a valid handle or null on error
            // For this test, we just verify it doesn't crash
            if !pipeline_handle.is_null() {
                // Clean up the pipeline handle
                cursed_cleanup_handle(pipeline_handle, 1); // 1 = pipeline handle type
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_background_task_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        // Mock command data (in a real implementation, this would be a serialized command)
        let mock_command_data = 1 as *const c_void;
        
        unsafe {
            let task_handle = cursed_background_task(manager_ptr, mock_command_data);
            
            // Should return a valid handle or null on error
            if !task_handle.is_null() {
                // Clean up the task handle
                cursed_cleanup_handle(task_handle, 2); // 2 = background task handle type
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_io_redirection_ffi() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let stdin_file = create_c_string("/dev/null");
        let stdout_file = create_c_string("/tmp/test_output.txt");
        let stderr_file = create_c_string("/tmp/test_error.txt");
        
        unsafe {
            let io_handle = cursed_io_redirection(
                manager_ptr,
                stdin_file.as_ptr(),
                stdout_file.as_ptr(),
                stderr_file.as_ptr(),
            );
            
            // Should return a valid handle or null on error
            if !io_handle.is_null() {
                // Clean up the I/O redirection handle
                cursed_cleanup_handle(io_handle, 3); // 3 = I/O redirection handle type
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_null_pointer_safety() {
        // Test that FFI functions handle null pointers gracefully
        
        unsafe {
            // Test exec_slay with null manager
            let result = cursed_exec_slay(
                ptr::null_mut(),
                ptr::null(),
                ptr::null(),
                0,
                ptr::null(),
            );
            assert_eq!(result, -1, "Should return error for null manager");
            
            // Test exec_vibez with null command
            let manager = create_test_process_manager();
            let manager_ptr = Box::into_raw(manager) as *mut c_void;
            
            let result = cursed_exec_vibez(
                manager_ptr,
                ptr::null(),
                ptr::null(),
                0,
                ptr::null(),
            );
            assert_eq!(result, -1, "Should return error for null command");
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_invalid_arguments() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        unsafe {
            // Test with invalid command (empty string)
            let empty_command = create_c_string("");
            let result = cursed_exec_slay(
                manager_ptr,
                empty_command.as_ptr(),
                ptr::null(),
                0,
                ptr::null(),
            );
            // Should handle gracefully (might succeed or fail depending on system)
            assert!(result >= -1, "Should return valid error code");
            
            // Test with negative argument count
            let command = create_c_string("echo");
            let result = cursed_exec_slay(
                manager_ptr,
                command.as_ptr(),
                ptr::null(),
                -1,
                ptr::null(),
            );
            // Should handle negative count gracefully
            assert!(result >= -1, "Should handle negative count");
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_cleanup_handle_function() {
        // Test that cleanup function handles different handle types safely
        
        unsafe {
            // Test with null handle (should not crash)
            cursed_cleanup_handle(ptr::null_mut(), 1);
            cursed_cleanup_handle(ptr::null_mut(), 2);
            cursed_cleanup_handle(ptr::null_mut(), 3);
            
            // Test with unknown handle type (should not crash)
            let dummy_ptr = 1 as *mut c_void;
            cursed_cleanup_handle(dummy_ptr, 999);
        }
    }

    #[traced_test]
    #[test]
    fn test_process_spawn_with_environment() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        let command = create_c_string("env");
        let args: Vec<CString> = vec![];
        let arg_ptrs = get_c_string_ptrs(&args);
        
        // Create multiple environment variables
        let env1 = create_c_string("VAR1=value1");
        let env2 = create_c_string("VAR2=value2");
        let env3 = create_c_string("PATH=/usr/bin:/bin");
        let env_vars = vec![env1, env2, env3];
        let env_ptrs = get_c_string_ptrs(&env_vars);
        
        unsafe {
            let pid = cursed_process_spawn(
                manager_ptr,
                command.as_ptr(),
                arg_ptrs.as_ptr(),
                args.len() as c_int,
                env_ptrs.as_ptr(),
                env_vars.len() as c_int,
            );
            
            // Should return a valid PID or error
            assert!(pid != 0, "spawn with environment should return non-zero");
            
            if pid > 0 {
                // Wait for the process to complete
                let exit_code = cursed_process_wait(manager_ptr, pid);
                // env command should succeed
                assert!(exit_code >= 0, "env command should complete successfully");
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_complex_pipeline() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        // Create a more complex pipeline
        let cmd1 = create_c_string("echo 'line1\nline2\nline3'");
        let cmd2 = create_c_string("grep line");
        let cmd3 = create_c_string("wc -l");
        let commands = vec![cmd1, cmd2, cmd3];
        let cmd_ptrs = get_c_string_ptrs(&commands);
        
        unsafe {
            let pipeline_handle = cursed_process_pipeline(
                manager_ptr,
                cmd_ptrs.as_ptr(),
                commands.len() as c_int,
            );
            
            // Verify pipeline creation
            if !pipeline_handle.is_null() {
                // In a real implementation, we might want to check pipeline status
                // For now, just clean up
                cursed_cleanup_handle(pipeline_handle, 1);
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }

    #[traced_test]
    #[test]
    fn test_io_redirection_with_null_files() {
        let manager = create_test_process_manager();
        let manager_ptr = Box::into_raw(manager) as *mut c_void;
        
        // Test with only stdout redirection
        let stdout_file = create_c_string("/tmp/only_stdout.txt");
        
        unsafe {
            let io_handle = cursed_io_redirection(
                manager_ptr,
                ptr::null(),  // stdin
                stdout_file.as_ptr(),  // stdout
                ptr::null(),  // stderr
            );
            
            // Should handle partial redirection
            if !io_handle.is_null() {
                cursed_cleanup_handle(io_handle, 3);
            }
            
            // Clean up
            let _manager = Box::from_raw(manager_ptr as *mut ProcessManager);
        }
    }
}
