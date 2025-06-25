/// Comprehensive tests for the CURSED process runtime system
/// 
/// This test suite validates all aspects of process management including
/// spawning, control, IPC, shared memory, and signal handling.

use std::ffi::{CString, CStr};
use std::ptr;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::time::Duration;
use std::thread;

use cursed::runtime::process::*;
use cursed::runtime::{initialize_process_runtime, get_process_runtime};

/// Test process runtime initialization
#[test]
fn test_process_runtime_initialization() {
    initialize_process_runtime();
    
    let runtime = get_process_runtime();
    assert!(runtime.is_some());
    
    let runtime = runtime.unwrap();
    assert!(runtime.processes.read().unwrap().is_empty());
    assert!(runtime.ipc_channels.read().unwrap().is_empty());
}

/// Test basic process spawning
#[test] 
fn test_process_spawn_basic() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Test spawning a simple echo command
    let result = runtime.spawn_process("echo", &["hello".to_string(), "world".to_string()]);
    assert!(result.is_ok());
    
    let pid = result.unwrap();
    assert!(pid > 0);
    
    // Check process is tracked
    let processes = runtime.processes.read().unwrap();
    assert!(processes.contains_key(&pid));
    
    let process_info = processes.get(&pid).unwrap();
    assert_eq!(process_info.command, "echo");
    assert_eq!(process_info.status, ProcessStatus::Running);
}

/// Test process waiting and status
#[test]
fn test_process_wait_and_status() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Spawn a process that will exit quickly
    let pid_result = runtime.spawn_process("echo", &["test".to_string()]);
    assert!(pid_result.is_ok());
    let pid = pid_result.unwrap();
    
    // Wait for the process
    let wait_result = runtime.wait_process(pid);
    assert!(wait_result.is_ok());
    
    // Check final status
    let status_result = runtime.get_process_status(pid);
    assert!(status_result.is_ok());
    let status = status_result.unwrap();
    assert!(status == ProcessStatus::Exited as i32 || status == ProcessStatus::Killed as i32);
}

/// Test process information retrieval
#[test]
fn test_process_info_retrieval() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Spawn a process
    let pid_result = runtime.spawn_process("sleep", &["1".to_string()]);
    if pid_result.is_err() {
        // Skip test if sleep command not available
        return;
    }
    let pid = pid_result.unwrap();
    
    // Get process info
    let info_result = runtime.get_process_info(pid);
    assert!(info_result.is_ok());
    
    let info_ptr = info_result.unwrap();
    assert!(!info_ptr.is_null());
    
    let process_info = unsafe { &*info_ptr };
    assert_eq!(process_info.pid, pid);
    assert_eq!(process_info.command, "sleep");
    assert!(process_info.start_time > 0);
    
    // Clean up
    unsafe { Box::from_raw(info_ptr) };
}

/// Test IPC channel creation and operations
#[test]
fn test_ipc_channel_operations() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Create IPC configuration
    let config = IpcConfig {
        name: "test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Create a pipe channel
    let channel_result = runtime.create_ipc_channel(IpcChannelType::Pipe, &config);
    assert!(channel_result.is_ok());
    
    let channel_id = channel_result.unwrap();
    assert!(channel_id > 0);
    
    // Check channel is tracked
    let channels = runtime.ipc_channels.read().unwrap();
    assert!(channels.contains_key(&channel_id));
    
    let channel = channels.get(&channel_id).unwrap();
    assert_eq!(channel.channel_type, IpcChannelType::Pipe);
    assert!(channel.is_open);
}

/// Test IPC send and receive
#[test]
fn test_ipc_send_receive() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Create channel
    let config = IpcConfig {
        name: "send_recv_test".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_id = runtime.create_ipc_channel(IpcChannelType::MessageQueue, &config).unwrap();
    
    // Send data
    let test_data = b"Hello, IPC!";
    let send_result = runtime.ipc_send(channel_id, test_data);
    assert!(send_result.is_ok());
    
    // Receive data (with short timeout)
    let receive_result = runtime.ipc_receive(channel_id, 100);
    assert!(receive_result.is_ok());
    
    let data_ptr = receive_result.unwrap();
    assert!(!data_ptr.is_null());
    
    // Clean up
    unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
}

/// Test shared memory operations
#[test]
fn test_shared_memory_operations() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Create shared memory segment
    let shm_result = runtime.create_shared_memory("test_shm", 4096);
    assert!(shm_result.is_ok());
    
    let shm_ptr = shm_result.unwrap();
    assert!(!shm_ptr.is_null());
    
    // Check shared memory is tracked
    let shm_segments = runtime.shared_memory.read().unwrap();
    assert!(shm_segments.contains_key("test_shm"));
    
    let segment = shm_segments.get("test_shm").unwrap();
    assert_eq!(segment.name, "test_shm");
    assert_eq!(segment.size, 4096);
    assert_eq!(segment.ref_count, 1);
}

/// Test signal handler registration
#[test]
fn test_signal_handler_registration() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Register signal handler for SIGTERM (15)
    let result = runtime.register_signal_handler(15, None);
    assert!(result.is_ok());
    
    // Check handler is registered
    let handlers = runtime.signal_handlers.read().unwrap();
    assert!(handlers.contains_key(&15));
    
    let handler = handlers.get(&15).unwrap();
    assert_eq!(handler.signal, 15);
    assert!(!handler.is_blocked);
}

/// Test signal sending
#[test]
fn test_signal_sending() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Spawn a long-running process
    let pid_result = runtime.spawn_process("sleep", &["10".to_string()]);
    if pid_result.is_err() {
        // Skip test if sleep command not available
        return;
    }
    let pid = pid_result.unwrap();
    
    // Send SIGTERM signal
    let signal_result = runtime.send_signal(pid, 15);
    assert!(signal_result.is_ok());
    
    // Process should be terminated
    thread::sleep(Duration::from_millis(100));
    let status_result = runtime.get_process_status(pid);
    assert!(status_result.is_ok());
}

/// Test FFI process spawn function
#[test]
fn test_ffi_process_spawn() {
    initialize_process_runtime();
    
    // Create C strings for command and arguments
    let command = CString::new("echo").unwrap();
    let arg1 = CString::new("hello").unwrap();
    let arg2 = CString::new("ffi").unwrap();
    
    let args: Vec<*const c_char> = vec![
        arg1.as_ptr(),
        arg2.as_ptr(),
    ];
    
    // Call FFI function
    let pid = unsafe {
        cursed_process_spawn(
            command.as_ptr(),
            args.as_ptr(),
            args.len() as c_int,
        )
    };
    
    assert!(pid > 0);
}

/// Test FFI process control functions
#[test]
fn test_ffi_process_control() {
    initialize_process_runtime();
    
    // Spawn process using FFI
    let command = CString::new("sleep").unwrap();
    let arg1 = CString::new("5").unwrap();
    let args: Vec<*const c_char> = vec![arg1.as_ptr()];
    
    let pid = unsafe {
        cursed_process_spawn(
            command.as_ptr(),
            args.as_ptr(),
            args.len() as c_int,
        )
    };
    
    if pid <= 0 {
        // Skip test if sleep command not available
        return;
    }
    
    // Test get status
    let status = unsafe { cursed_process_get_status(pid) };
    assert!(status >= 0);
    
    // Test kill
    let kill_result = unsafe { cursed_process_kill(pid) };
    assert_eq!(kill_result, 0);
}

/// Test FFI IPC functions
#[test]
fn test_ffi_ipc_functions() {
    initialize_process_runtime();
    
    // Create IPC config
    let config = IpcConfig {
        name: "ffi_test".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Test pipe creation
    let pipe_handle = unsafe { cursed_pipe_create(&config as *const IpcConfig) };
    assert!(!pipe_handle.is_null());
    
    // Test message queue creation
    let mq_handle = unsafe { cursed_message_queue_create(&config as *const IpcConfig) };
    assert!(!mq_handle.is_null());
    
    // Test IPC send (simplified)
    let test_data = b"test\0"; // Null-terminated for C compatibility
    let send_result = unsafe {
        cursed_ipc_send(pipe_handle, test_data.as_ptr() as *mut c_void)
    };
    assert_eq!(send_result, 0);
    
    // Test IPC receive
    let received_data = unsafe { cursed_ipc_receive(pipe_handle, 100) };
    assert!(!received_data.is_null());
    
    // Clean up
    if !received_data.is_null() {
        unsafe { Box::from_raw(received_data as *mut Vec<u8>) };
    }
}

/// Test FFI shared memory functions
#[test]
fn test_ffi_shared_memory_functions() {
    initialize_process_runtime();
    
    // Test create
    let shm_handle = unsafe { cursed_shm_create() };
    assert!(!shm_handle.is_null());
    
    // Test map
    let mapped_ptr = unsafe { cursed_shm_map(shm_handle) };
    assert!(!mapped_ptr.is_null());
    
    // Test write
    let test_data = b"shared memory test";
    let write_result = unsafe {
        cursed_shm_write(
            shm_handle,
            0,
            test_data.as_ptr() as *mut c_void,
            test_data.len() as c_long,
        )
    };
    assert_eq!(write_result, 0);
    
    // Test read
    let read_ptr = unsafe { cursed_shm_read(shm_handle, 0, test_data.len() as c_long) };
    assert!(!read_ptr.is_null());
    
    // Test sync
    let sync_result = unsafe { cursed_shm_sync(shm_handle) };
    assert_eq!(sync_result, 0);
    
    // Test lock/unlock
    let lock_result = unsafe { cursed_shm_lock(shm_handle) };
    assert_eq!(lock_result, 0);
    
    let unlock_result = unsafe { cursed_shm_unlock(shm_handle) };
    assert_eq!(unlock_result, 0);
    
    // Test unmap
    let unmap_result = unsafe { cursed_shm_unmap(shm_handle) };
    assert_eq!(unmap_result, 0);
}

/// Test FFI signal functions
#[test]
fn test_ffi_signal_functions() {
    initialize_process_runtime();
    
    // Test signal handler registration
    let register_result = unsafe { cursed_signal_register(15, ptr::null_mut()) };
    assert_eq!(register_result, 0);
    
    // Test signal blocking
    let block_result = unsafe { cursed_signal_block(15) };
    assert_eq!(block_result, 0);
    
    // Test signal unblocking
    let unblock_result = unsafe { cursed_signal_unblock(15) };
    assert_eq!(unblock_result, 0);
    
    // Test signal unregistration
    let unregister_result = unsafe { cursed_signal_unregister(15) };
    assert_eq!(unregister_result, 0);
    
    // Test signal wait (with short timeout)
    let wait_result = unsafe { cursed_signal_wait(15, 100) };
    assert_eq!(wait_result, 15); // Should return the signal number
}

/// Test concurrent process operations
#[test]
fn test_concurrent_process_operations() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    let mut handles = Vec::new();
    
    // Spawn multiple processes concurrently
    for i in 0..5 {
        let runtime_clone = runtime.clone();
        let handle = thread::spawn(move || {
            let result = runtime_clone.spawn_process("echo", &[format!("process_{}", i)]);
            assert!(result.is_ok());
            result.unwrap()
        });
        handles.push(handle);
    }
    
    // Collect PIDs
    let mut pids = Vec::new();
    for handle in handles {
        let pid = handle.join().unwrap();
        pids.push(pid);
    }
    
    // All processes should be tracked
    let processes = runtime.processes.read().unwrap();
    for pid in &pids {
        assert!(processes.contains_key(pid));
    }
    
    assert_eq!(processes.len(), pids.len());
}

/// Test error handling for invalid operations
#[test]
fn test_error_handling() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Test operations on non-existent process
    let invalid_pid = 99999;
    
    let kill_result = runtime.kill_process(invalid_pid);
    assert!(kill_result.is_err());
    
    let status_result = runtime.get_process_status(invalid_pid);
    assert!(status_result.is_err());
    
    let wait_result = runtime.wait_process(invalid_pid);
    assert!(wait_result.is_err());
    
    // Test operations on non-existent IPC channel
    let invalid_channel = 99999;
    
    let send_result = runtime.ipc_send(invalid_channel, b"test");
    assert!(send_result.is_err());
    
    let receive_result = runtime.ipc_receive(invalid_channel, 100);
    assert!(receive_result.is_err());
}

/// Test memory safety and cleanup
#[test]
fn test_memory_safety_and_cleanup() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Test process info cleanup
    let pid_result = runtime.spawn_process("echo", &["cleanup_test".to_string()]);
    assert!(pid_result.is_ok());
    let pid = pid_result.unwrap();
    
    // Get process info and ensure it's properly managed
    let info_result = runtime.get_process_info(pid);
    assert!(info_result.is_ok());
    let info_ptr = info_result.unwrap();
    assert!(!info_ptr.is_null());
    
    // Properly clean up the allocated memory
    unsafe { Box::from_raw(info_ptr) };
    
    // Test shared memory cleanup
    let shm_result = runtime.create_shared_memory("cleanup_shm", 1024);
    assert!(shm_result.is_ok());
    
    // Verify shared memory is tracked
    {
        let shm_segments = runtime.shared_memory.read().unwrap();
        assert!(shm_segments.contains_key("cleanup_shm"));
    }
    
    // Test IPC channel cleanup
    let config = IpcConfig {
        name: "cleanup_channel".to_string(),
        config_type: 0,
        size_or_capacity: 512,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_result = runtime.create_ipc_channel(IpcChannelType::Socket, &config);
    assert!(channel_result.is_ok());
    let channel_id = channel_result.unwrap();
    
    // Verify channel is tracked
    {
        let channels = runtime.ipc_channels.read().unwrap();
        assert!(channels.contains_key(&channel_id));
    }
}

/// Test runtime statistics and monitoring
#[test]
fn test_runtime_statistics() {
    initialize_process_runtime();
    let runtime = get_process_runtime().unwrap();
    
    // Initial state should be empty
    assert_eq!(runtime.processes.read().unwrap().len(), 0);
    assert_eq!(runtime.ipc_channels.read().unwrap().len(), 0);
    assert_eq!(runtime.shared_memory.read().unwrap().len(), 0);
    
    // Create some resources
    let _pid = runtime.spawn_process("echo", &["stats_test".to_string()]).unwrap();
    
    let config = IpcConfig {
        name: "stats_channel".to_string(),
        config_type: 0,
        size_or_capacity: 256,
        permissions: 0o666,
        flags: 0,
    };
    let _channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config).unwrap();
    let _shm_ptr = runtime.create_shared_memory("stats_shm", 2048).unwrap();
    
    // Verify counters are updated
    assert_eq!(runtime.processes.read().unwrap().len(), 1);
    assert_eq!(runtime.ipc_channels.read().unwrap().len(), 1);
    assert_eq!(runtime.shared_memory.read().unwrap().len(), 1);
}
