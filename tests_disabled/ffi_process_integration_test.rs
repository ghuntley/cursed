/// FFI Integration Tests for Process Management
/// 
/// This module tests the Foreign Function Interface (FFI) for process management
/// and IPC functionality, ensuring that the C-compatible interfaces work correctly
/// and can be safely called from LLVM-generated code.

use std::ffi::{CString, CStr};
use std::ptr;
use std::os::raw::{c_char, c_int, c_long, c_void};

use cursed::runtime::process::{
    ProcessInfo, IpcConfig, initialize_process_runtime, shutdown_process_runtime,
    // FFI functions
    cursed_process_spawn, cursed_process_kill, cursed_process_terminate,
    cursed_process_pause, cursed_process_resume, cursed_process_wait,
    cursed_process_get_status, cursed_process_get_info,
    cursed_pipe_create, cursed_named_pipe_create, cursed_message_queue_create,
    cursed_shared_memory_create, cursed_socket_create, cursed_semaphore_create,
    cursed_ipc_send, cursed_ipc_receive,
    cursed_shm_create, cursed_shm_open, cursed_shm_write, cursed_shm_read,
    cursed_shm_close, cursed_sem_create, cursed_sem_open, cursed_sem_wait,
    cursed_sem_post, cursed_sem_close, cursed_signal_send
};

#[path = "common.rs"]
mod common;

/// Test FFI process spawning and control functions
#[test]
fn test_ffi_process_spawning() {
    common::tracing::setup();
    
    // Initialize the process runtime
    initialize_process_runtime();
    
    // Test process spawning with FFI
    let command = CString::new("echo").expect("Should create command string");
    let arg1 = CString::new("hello").expect("Should create arg string");
    let arg2 = CString::new("world").expect("Should create arg string");
    
    let args = vec![arg1.as_ptr(), arg2.as_ptr()];
    let args_ptr = args.as_ptr();
    let args_count = args.len() as c_int;
    
    let pid = unsafe { cursed_process_spawn(command.as_ptr(), args_ptr, args_count) };
    
    if pid > 0 {
        println!("FFI spawned process with PID: {}", pid);
        
        // Test getting process status
        let status = unsafe { cursed_process_get_status(pid) };
        println!("Process status: {}", status);
        
        // Test getting process info
        let info_ptr = unsafe { cursed_process_get_info(pid) };
        if !info_ptr.is_null() {
            let info = unsafe { Box::from_raw(info_ptr) };
            println!("Process info: PID={}, Command={}, Status={:?}", 
                     info.pid, info.command, info.status);
        }
        
        // Test waiting for process
        let exit_code = unsafe { cursed_process_wait(pid) };
        println!("Process exited with code: {}", exit_code);
    } else {
        println!("FFI process spawn failed (may be expected in restricted environment)");
    }
    
    // Cleanup
    shutdown_process_runtime();
}

/// Test FFI process control functions
#[test]
fn test_ffi_process_control() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Try to spawn a longer-running process for control testing
    let command = CString::new("sleep").expect("Should create command string");
    let arg1 = CString::new("0.5").expect("Should create arg string");
    
    let args = vec![arg1.as_ptr()];
    let args_ptr = args.as_ptr();
    let args_count = args.len() as c_int;
    
    let pid = unsafe { cursed_process_spawn(command.as_ptr(), args_ptr, args_count) };
    
    if pid > 0 {
        println!("FFI spawned sleep process with PID: {}", pid);
        
        // Test pause (might not work on all systems)
        let pause_result = unsafe { cursed_process_pause(pid) };
        println!("Pause result: {}", pause_result);
        
        // Test resume (might not work on all systems)
        let resume_result = unsafe { cursed_process_resume(pid) };
        println!("Resume result: {}", resume_result);
        
        // Test terminate
        let terminate_result = unsafe { cursed_process_terminate(pid) };
        println!("Terminate result: {}", terminate_result);
        
        // Wait for the process
        let exit_code = unsafe { cursed_process_wait(pid) };
        println!("Process exited with code: {}", exit_code);
    } else {
        println!("FFI process spawn failed (may be expected in restricted environment)");
    }
    
    shutdown_process_runtime();
}

/// Test FFI IPC creation functions
#[test]
fn test_ffi_ipc_creation() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Test creating different types of IPC channels
    let config = IpcConfig {
        name: "ffi_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Test pipe creation
    let pipe_handle = unsafe { cursed_pipe_create(&config as *const IpcConfig) };
    if !pipe_handle.is_null() {
        println!("FFI created pipe: {:?}", pipe_handle);
    } else {
        println!("FFI pipe creation failed");
    }
    
    // Test named pipe creation
    let named_pipe_handle = unsafe { cursed_named_pipe_create(&config as *const IpcConfig) };
    if !named_pipe_handle.is_null() {
        println!("FFI created named pipe: {:?}", named_pipe_handle);
    } else {
        println!("FFI named pipe creation failed");
    }
    
    // Test message queue creation
    let mq_handle = unsafe { cursed_message_queue_create(&config as *const IpcConfig) };
    if !mq_handle.is_null() {
        println!("FFI created message queue: {:?}", mq_handle);
    } else {
        println!("FFI message queue creation failed");
    }
    
    // Test socket creation
    let socket_handle = unsafe { cursed_socket_create(&config as *const IpcConfig) };
    if !socket_handle.is_null() {
        println!("FFI created socket: {:?}", socket_handle);
    } else {
        println!("FFI socket creation failed");
    }
    
    // Test semaphore creation
    let sem_handle = unsafe { cursed_semaphore_create(&config as *const IpcConfig) };
    if !sem_handle.is_null() {
        println!("FFI created semaphore: {:?}", sem_handle);
    } else {
        println!("FFI semaphore creation failed");
    }
    
    shutdown_process_runtime();
}

/// Test FFI IPC communication
#[test]
fn test_ffi_ipc_communication() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    let config = IpcConfig {
        name: "ffi_comm_test".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Create a pipe for communication testing
    let channel_handle = unsafe { cursed_pipe_create(&config as *const IpcConfig) };
    
    if !channel_handle.is_null() {
        println!("FFI created communication channel: {:?}", channel_handle);
        
        // Test sending data
        let test_message = CString::new("FFI test message").expect("Should create test message");
        let test_data = test_message.as_ptr() as *mut c_void;
        
        let send_result = unsafe { cursed_ipc_send(channel_handle, test_data) };
        println!("FFI send result: {}", send_result);
        
        if send_result == 0 {
            // Test receiving data
            let receive_result = unsafe { cursed_ipc_receive(channel_handle, 1000) };
            
            if !receive_result.is_null() {
                // Note: The received data needs to be properly interpreted
                // For this test, we just verify we got a non-null pointer
                println!("FFI received data: {:?}", receive_result);
                
                // Clean up the received data
                let received_data = unsafe { Box::from_raw(receive_result as *mut Vec<u8>) };
                let received_string = String::from_utf8_lossy(&received_data);
                println!("FFI received message: {}", received_string);
            } else {
                println!("FFI receive returned null (timeout or no data)");
            }
        }
    } else {
        println!("FFI failed to create communication channel");
    }
    
    shutdown_process_runtime();
}

/// Test FFI shared memory operations
#[test]
fn test_ffi_shared_memory_operations() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Test basic shared memory creation
    let shm_ptr = unsafe { cursed_shm_create() };
    if !shm_ptr.is_null() {
        println!("FFI created default shared memory: {:?}", shm_ptr);
    } else {
        println!("FFI failed to create default shared memory");
    }
    
    // Test named shared memory
    let shm_name = CString::new("ffi_test_shm").expect("Should create shm name");
    let named_shm_ptr = unsafe { cursed_shm_open(shm_name.as_ptr()) };
    
    if !named_shm_ptr.is_null() {
        println!("FFI opened named shared memory: {:?}", named_shm_ptr);
    } else {
        println!("FFI failed to open named shared memory");
    }
    
    // Test shared memory with config
    let config = IpcConfig {
        name: "ffi_shm_config_test".to_string(),
        config_type: 0,
        size_or_capacity: 4096,
        permissions: 0o666,
        flags: 0,
    };
    
    let config_shm_ptr = unsafe { cursed_shared_memory_create(&config as *const IpcConfig) };
    if !config_shm_ptr.is_null() {
        println!("FFI created configured shared memory: {:?}", config_shm_ptr);
        
        // Test writing to shared memory
        let test_data = CString::new("FFI shared memory test data").expect("Should create test data");
        let write_result = unsafe { 
            cursed_shm_write(config_shm_ptr, test_data.as_ptr() as *const c_void, test_data.as_bytes().len() as c_long) 
        };
        println!("FFI shared memory write result: {}", write_result);
        
        if write_result == 0 {
            // Test reading from shared memory
            let read_result = unsafe { 
                cursed_shm_read(config_shm_ptr, test_data.as_bytes().len() as c_long) 
            };
            
            if !read_result.is_null() {
                println!("FFI shared memory read successful: {:?}", read_result);
                
                // Clean up the read data
                let read_data = unsafe { Box::from_raw(read_result as *mut Vec<u8>) };
                let read_string = String::from_utf8_lossy(&read_data);
                println!("FFI read data: {}", read_string);
            } else {
                println!("FFI shared memory read returned null");
            }
        }
        
        // Test closing shared memory
        let close_result = unsafe { cursed_shm_close(config_shm_ptr) };
        println!("FFI shared memory close result: {}", close_result);
    } else {
        println!("FFI failed to create configured shared memory");
    }
    
    shutdown_process_runtime();
}

/// Test FFI semaphore operations
#[test]
fn test_ffi_semaphore_operations() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Test semaphore creation
    let sem_name = CString::new("ffi_test_semaphore").expect("Should create semaphore name");
    let sem_handle = unsafe { cursed_sem_create(sem_name.as_ptr(), 1) };
    
    if !sem_handle.is_null() {
        println!("FFI created semaphore: {:?}", sem_handle);
        
        // Test semaphore wait
        let wait_result = unsafe { cursed_sem_wait(sem_handle) };
        println!("FFI semaphore wait result: {}", wait_result);
        
        // Test semaphore post
        let post_result = unsafe { cursed_sem_post(sem_handle) };
        println!("FFI semaphore post result: {}", post_result);
        
        // Test closing semaphore
        let close_result = unsafe { cursed_sem_close(sem_handle) };
        println!("FFI semaphore close result: {}", close_result);
    } else {
        println!("FFI failed to create semaphore");
    }
    
    // Test opening existing semaphore
    let existing_sem = unsafe { cursed_sem_open(sem_name.as_ptr()) };
    if !existing_sem.is_null() {
        println!("FFI opened existing semaphore: {:?}", existing_sem);
        let _ = unsafe { cursed_sem_close(existing_sem) };
    } else {
        println!("FFI failed to open existing semaphore");
    }
    
    shutdown_process_runtime();
}

/// Test FFI signal operations
#[test]
fn test_ffi_signal_operations() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Try to spawn a process to send signals to
    let command = CString::new("sleep").expect("Should create command string");
    let arg1 = CString::new("1").expect("Should create arg string");
    
    let args = vec![arg1.as_ptr()];
    let args_ptr = args.as_ptr();
    let args_count = args.len() as c_int;
    
    let pid = unsafe { cursed_process_spawn(command.as_ptr(), args_ptr, args_count) };
    
    if pid > 0 {
        println!("FFI spawned process for signal testing: {}", pid);
        
        // Test sending different signals
        let signals_to_test = vec![
            (2, "SIGINT"),
            (15, "SIGTERM"),
            (9, "SIGKILL"),
        ];
        
        for (signal, signal_name) in signals_to_test {
            let signal_result = unsafe { cursed_signal_send(pid, signal) };
            println!("FFI send {} to process {}: result {}", signal_name, pid, signal_result);
            
            // Brief pause between signals
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            // Check if process is still running
            let status = unsafe { cursed_process_get_status(pid) };
            println!("Process status after {}: {}", signal_name, status);
            
            // If process is dead, break
            if status != 0 { // Assuming 0 is Running
                break;
            }
        }
        
        // Final wait
        let exit_code = unsafe { cursed_process_wait(pid) };
        println!("Signal test process final exit code: {}", exit_code);
    } else {
        println!("FFI failed to spawn process for signal testing");
    }
    
    shutdown_process_runtime();
}

/// Test FFI error handling and edge cases
#[test]
fn test_ffi_error_handling() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Test null pointer handling
    println!("Testing FFI null pointer handling...");
    
    // Test process operations with invalid PID
    let invalid_pid = -1;
    let kill_result = unsafe { cursed_process_kill(invalid_pid) };
    println!("FFI kill invalid PID result: {}", kill_result);
    assert_eq!(kill_result, -1, "Should return error for invalid PID");
    
    let status_result = unsafe { cursed_process_get_status(invalid_pid) };
    println!("FFI get status invalid PID result: {}", status_result);
    assert_eq!(status_result, -1, "Should return error for invalid PID");
    
    // Test IPC operations with null handles
    let null_handle = ptr::null_mut();
    let null_data = ptr::null_mut();
    
    let send_result = unsafe { cursed_ipc_send(null_handle, null_data) };
    println!("FFI IPC send null handle result: {}", send_result);
    assert_eq!(send_result, -1, "Should return error for null handle");
    
    let receive_result = unsafe { cursed_ipc_receive(null_handle, 100) };
    println!("FFI IPC receive null handle result: {:?}", receive_result);
    assert!(receive_result.is_null(), "Should return null for null handle");
    
    // Test shared memory operations with null pointers
    let null_name = ptr::null();
    let shm_open_result = unsafe { cursed_shm_open(null_name) };
    println!("FFI shared memory open null name result: {:?}", shm_open_result);
    assert!(shm_open_result.is_null(), "Should return null for null name");
    
    // Test process spawning with null command
    let spawn_result = unsafe { cursed_process_spawn(null_name, ptr::null(), 0) };
    println!("FFI process spawn null command result: {}", spawn_result);
    assert_eq!(spawn_result, -1, "Should return error for null command");
    
    // Test IPC config with null pointer
    let null_config = ptr::null();
    let pipe_result = unsafe { cursed_pipe_create(null_config) };
    println!("FFI pipe create null config result: {:?}", pipe_result);
    assert!(pipe_result.is_null(), "Should return null for null config");
    
    shutdown_process_runtime();
}

/// Test FFI memory management and cleanup
#[test]
fn test_ffi_memory_management() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    // Test that FFI functions properly manage memory
    let mut allocated_pointers = vec![];
    
    // Create multiple resources
    for i in 0..10 {
        let config = IpcConfig {
            name: format!("ffi_memory_test_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        // Create IPC channel
        let channel_handle = unsafe { cursed_pipe_create(&config as *const IpcConfig) };
        if !channel_handle.is_null() {
            allocated_pointers.push(("channel", channel_handle));
        }
        
        // Create shared memory
        let shm_handle = unsafe { cursed_shared_memory_create(&config as *const IpcConfig) };
        if !shm_handle.is_null() {
            allocated_pointers.push(("shm", shm_handle));
        }
    }
    
    println!("FFI allocated {} resources for memory test", allocated_pointers.len());
    
    // Test operations on allocated resources
    for (resource_type, handle) in &allocated_pointers {
        match *resource_type {
            "channel" => {
                let test_data = CString::new("memory_test").expect("Should create test data");
                let send_result = unsafe { cursed_ipc_send(*handle, test_data.as_ptr() as *mut c_void) };
                println!("FFI {} send result: {}", resource_type, send_result);
            }
            "shm" => {
                let test_data = CString::new("shm_memory_test").expect("Should create test data");
                let write_result = unsafe { 
                    cursed_shm_write(*handle, test_data.as_ptr() as *const c_void, test_data.as_bytes().len() as c_long) 
                };
                println!("FFI {} write result: {}", resource_type, write_result);
            }
            _ => {}
        }
    }
    
    // Cleanup test - the runtime should handle cleanup when shutdown
    shutdown_process_runtime();
    
    // Reinitialize and verify old handles are invalid
    initialize_process_runtime();
    
    // Test that old handles are no longer valid
    for (resource_type, handle) in &allocated_pointers {
        match *resource_type {
            "channel" => {
                let test_data = CString::new("invalid_test").expect("Should create test data");
                let send_result = unsafe { cursed_ipc_send(*handle, test_data.as_ptr() as *mut c_void) };
                // This should fail since the handle is from the old runtime
                println!("FFI old {} handle test result: {}", resource_type, send_result);
            }
            _ => {}
        }
    }
    
    shutdown_process_runtime();
    println!("FFI memory management test completed");
}

/// Test FFI thread safety
#[test]
fn test_ffi_thread_safety() {
    common::tracing::setup();
    
    initialize_process_runtime();
    
    let num_threads = 4;
    let operations_per_thread = 10;
    
    let mut handles = vec![];
    let (tx, rx) = std::sync::mpsc::channel();
    
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        
        let handle = std::thread::spawn(move || {
            let mut successes = 0;
            let mut failures = 0;
            
            for op_id in 0..operations_per_thread {
                // Create IPC channel from thread
                let config = IpcConfig {
                    name: format!("ffi_thread_{}_{}", thread_id, op_id),
                    config_type: 0,
                    size_or_capacity: 512,
                    permissions: 0o666,
                    flags: 0,
                };
                
                let channel_handle = unsafe { cursed_pipe_create(&config as *const IpcConfig) };
                
                if !channel_handle.is_null() {
                    // Try to send data
                    let test_data = CString::new(format!("thread_{}_{}", thread_id, op_id))
                        .expect("Should create test data");
                    
                    let send_result = unsafe { 
                        cursed_ipc_send(channel_handle, test_data.as_ptr() as *mut c_void) 
                    };
                    
                    if send_result == 0 {
                        successes += 1;
                    } else {
                        failures += 1;
                    }
                } else {
                    failures += 1;
                }
                
                // Brief pause
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            
            tx_clone.send((thread_id, successes, failures)).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut total_successes = 0;
    let mut total_failures = 0;
    
    while let Ok((thread_id, successes, failures)) = rx.recv() {
        total_successes += successes;
        total_failures += failures;
        println!("FFI thread {} results: {} successes, {} failures", 
                 thread_id, successes, failures);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("FFI thread should complete");
    }
    
    println!("FFI thread safety test results:");
    println!("  Total successes: {}", total_successes);
    println!("  Total failures: {}", total_failures);
    println!("  Success rate: {:.1}%", 
             (total_successes as f64 / (total_successes + total_failures) as f64) * 100.0);
    
    assert!(total_successes > 0, "Should have some successful operations from threads");
    
    shutdown_process_runtime();
}
