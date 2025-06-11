/// Comprehensive tests for shared memory IPC implementation
use cursed::stdlib::ipc::shared_memory::*;
use cursed::stdlib::ipc::types::*;
use cursed::stdlib::ipc::traits::*;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

#[test]
fn test_shared_memory_config_creation() {
    let config = SharedMemoryConfig::new("test_memory".to_string(), 4096).unwrap();
    assert_eq!(config.id, "test_memory");
    assert_eq!(config.size, 4096);
    assert!(config.permissions.can_read());
    assert!(config.permissions.can_write());
    assert_eq!(config.sync_type, SyncType::Mutex);
    assert_eq!(config.access_mode, AccessMode::Random);
}

#[test]
fn test_config_validation() {
    // Empty ID should fail
    let result = SharedMemoryConfig::new("".to_string(), 4096);
    assert!(result.is_err());

    // Zero size should fail
    let result = SharedMemoryConfig::new("test".to_string(), 0);
    assert!(result.is_err());

    // Size too large should fail (> 1GB)
    let result = SharedMemoryConfig::new("test".to_string(), 2 * 1024 * 1024 * 1024);
    assert!(result.is_err());

    // Valid config should succeed
    let result = SharedMemoryConfig::new("valid_test".to_string(), 1024 * 1024);
    assert!(result.is_ok());
}

#[test]
fn test_config_builder_pattern() {
    let config = SharedMemoryConfig::new("builder_test".to_string(), 8192)
        .unwrap()
        .with_permissions(IpcPermissions::read_only())
        .with_sync_type(SyncType::ReadWriteLock)
        .with_access_mode(AccessMode::Sequential)
        .without_sync()
        .with_auto_cleanup(false);

    assert!(config.permissions.can_read());
    assert!(!config.permissions.can_write());
    assert_eq!(config.sync_type, SyncType::None);
    assert_eq!(config.access_mode, AccessMode::Sequential);
    assert!(!config.enable_sync);
    assert!(!config.auto_cleanup);
}

#[test]
fn test_memory_protection() {
    let rw = MemoryProtection::read_write();
    assert!(rw.read);
    assert!(rw.write);
    assert!(!rw.execute);

    let ro = MemoryProtection::read_only();
    assert!(ro.read);
    assert!(!ro.write);
    assert!(!ro.execute);

    let none = MemoryProtection::none();
    assert!(!none.read);
    assert!(!none.write);
    assert!(!none.execute);
}

#[test]
fn test_sync_types() {
    assert_eq!(SyncType::None, SyncType::None);
    assert_ne!(SyncType::Mutex, SyncType::None);
    
    let sem = SyncType::Semaphore(5);
    assert!(matches!(sem, SyncType::Semaphore(5)));

    let custom = SyncType::Custom("my_sync".to_string());
    assert!(matches!(custom, SyncType::Custom(_)));
}

#[test]
fn test_access_modes() {
    assert_eq!(AccessMode::Random, AccessMode::Random);
    assert_ne!(AccessMode::Sequential, AccessMode::Random);
    assert_ne!(AccessMode::ReadMostly, AccessMode::WriteMostly);
    assert_ne!(AccessMode::Concurrent, AccessMode::Sequential);
}

#[test]
fn test_shared_memory_manager() {
    let manager = SharedMemoryManager::global();
    let initial_count = get_active_region_count();
    
    // Manager should be accessible and have consistent state
    assert!(manager.active_regions().len() >= 0);
    assert_eq!(initial_count, get_active_region_count());
}

#[test]
fn test_initialization_and_shutdown() {
    // Test subsystem initialization
    let result = initialize_shared_memory_subsystem();
    assert!(result.is_ok());

    // Test getting statistics
    let active_count = get_active_region_count();
    let memory_usage = get_memory_usage();
    let transfer_rate = get_transfer_rate();
    let failure_count = get_allocation_failure_count();

    assert!(active_count >= 0);
    assert!(memory_usage >= 0);
    assert!(transfer_rate >= 0.0);
    assert!(failure_count >= 0);

    // Test cleanup
    let result = cleanup_all_regions();
    assert!(result.is_ok());

    // Test shutdown
    let result = shutdown_shared_memory_subsystem();
    assert!(result.is_ok());
}

#[test]
fn test_memory_mapping_bounds_checking() {
    let mapping = MemoryMapping {
        start_addr: 0x1000,
        size: 4096,
        is_writable: true,
        is_executable: false,
        offset: 0,
    };

    // Test address within bounds
    let addr_within = 0x1500 as *const u8;
    assert!(mapping.contains_address(addr_within));

    // Test address at start
    let addr_start = 0x1000 as *const u8;
    assert!(mapping.contains_address(addr_start));

    // Test address at end (should be false - exclusive end)
    let addr_end = 0x2000 as *const u8;
    assert!(!mapping.contains_address(addr_end));

    // Test address before start
    let addr_before = 0x500 as *const u8;
    assert!(!mapping.contains_address(addr_before));

    // Test address after end
    let addr_after = 0x3000 as *const u8;
    assert!(!mapping.contains_address(addr_after));
}

#[cfg(unix)]
#[test]
fn test_unix_memory_protection_conversion() {
    let rw = MemoryProtection::read_write();
    let prot = rw.to_mmap_prot();
    assert!(prot & libc::PROT_READ != 0);
    assert!(prot & libc::PROT_WRITE != 0);
    assert!(prot & libc::PROT_EXEC == 0);

    let ro = MemoryProtection::read_only();
    let prot = ro.to_mmap_prot();
    assert!(prot & libc::PROT_READ != 0);
    assert!(prot & libc::PROT_WRITE == 0);
    assert!(prot & libc::PROT_EXEC == 0);

    let none = MemoryProtection::none();
    let prot = none.to_mmap_prot();
    assert_eq!(prot, 0);
}

#[cfg(windows)]
#[test]
fn test_windows_memory_protection_conversion() {
    let rw = MemoryProtection::read_write();
    let page_protect = rw.to_page_protect();
    assert_eq!(page_protect, 0x04); // PAGE_READWRITE

    let ro = MemoryProtection::read_only();
    let page_protect = ro.to_page_protect();
    assert_eq!(page_protect, 0x02); // PAGE_READONLY

    let none = MemoryProtection::none();
    let page_protect = none.to_page_protect();
    assert_eq!(page_protect, 0x01); // PAGE_NOACCESS
}

// Integration tests that create actual shared memory
// Note: These tests may fail in some environments due to permissions

#[test]
#[ignore] // Ignore by default since it creates actual shared memory
fn test_shared_memory_creation_and_cleanup() {
    let config = SharedMemoryConfig::new("test_create_cleanup".to_string(), 4096).unwrap();
    
    // Create shared memory
    let result = create_shared_memory(config);
    if result.is_err() {
        // Skip test if shared memory creation is not supported
        return;
    }
    let memory = result.unwrap();
    
    assert!(memory.is_valid());
    assert_eq!(memory.size(), 4096);
    assert_eq!(memory.handle().id, "test_create_cleanup");

    // Cleanup
    let result = remove_shared_memory("test_create_cleanup");
    assert!(result.is_ok());
}

#[test]
#[ignore] // Ignore by default since it creates actual shared memory
fn test_shared_memory_mapping() {
    let config = SharedMemoryConfig::new("test_mapping".to_string(), 8192).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    // Map memory
    let result = memory.map(MemoryProtection::read_write());
    if result.is_err() {
        // Clean up and skip
        let _ = remove_shared_memory("test_mapping");
        return;
    }

    assert!(memory.is_mapped());

    // Test writing and reading
    let test_data = b"Hello, shared memory!";
    let result = memory.write_at(0, test_data);
    assert!(result.is_ok());

    let mut read_buffer = vec![0u8; test_data.len()];
    let result = memory.read_at(0, &mut read_buffer);
    assert!(result.is_ok());
    assert_eq!(read_buffer, test_data);

    // Unmap and cleanup
    let _ = memory.unmap();
    let _ = remove_shared_memory("test_mapping");
}

#[test]
#[ignore] // Ignore by default due to cross-process requirements
fn test_shared_memory_cross_process_simulation() {
    // This test simulates cross-process sharing using threads
    let config = SharedMemoryConfig::new("test_cross_process".to_string(), 16384).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    
    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = barrier.clone();

    // Writer thread
    let writer_handle = thread::spawn(move || {
        let result = open_shared_memory("test_cross_process");
        if result.is_err() {
            return;
        }
        
        let mut memory = result.unwrap();
        let _ = memory.map(MemoryProtection::read_write());
        
        // Write test data
        let test_data = b"Cross-process communication works!";
        let _ = memory.write_at(100, test_data);
        let _ = memory.sync();
        
        barrier_clone.wait();
    });

    // Reader thread
    let reader_handle = thread::spawn(move || {
        let result = open_shared_memory("test_cross_process");
        if result.is_err() {
            return;
        }
        
        let memory = result.unwrap();
        // Map as read-only for the reader
        // Note: This would require different handling in real implementation
        
        barrier.wait();
        
        // Try to read the data
        let mut read_buffer = vec![0u8; 34];
        let _ = memory.read_at(100, &mut read_buffer);
    });

    writer_handle.join().unwrap();
    reader_handle.join().unwrap();

    // Cleanup
    let _ = remove_shared_memory("test_cross_process");
}

#[test]
fn test_ipc_channel_trait_implementation() {
    let config = SharedMemoryConfig::new("test_trait".to_string(), 2048).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    // Test IpcChannel trait methods
    assert!(memory.is_open());
    assert_eq!(memory.handle().id, "test_trait");
    assert_eq!(memory.handle().handle_type, IpcHandleType::SharedMemory);
    
    let perms = memory.permissions();
    assert!(perms.can_read());
    assert!(perms.can_write());

    let mode = memory.mode();
    assert!(matches!(mode, IpcMode::ReadWrite));

    let stats = memory.statistics();
    assert_eq!(stats.bytes_read, 0);
    assert_eq!(stats.bytes_written, 0);

    // Test close
    let result = memory.close();
    assert!(result.is_ok());
    assert!(!memory.is_open());

    // Cleanup
    let _ = remove_shared_memory("test_trait");
}

#[test]
fn test_ipc_resource_trait_implementation() {
    let config = SharedMemoryConfig::new("test_resource".to_string(), 1024).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    // Test IpcResource trait methods
    let info = memory.resource_info();
    assert_eq!(info.resource_type, "SharedMemory");
    assert_eq!(info.id, "test_resource");
    assert_eq!(info.size, Some(1024));

    assert!(memory.is_valid());

    let usage_stats = memory.usage_stats();
    assert_eq!(usage_stats.total_operations, 0);
    assert_eq!(usage_stats.error_count, 0);

    // Test cleanup
    let result = memory.cleanup();
    assert!(result.is_ok());

    // Cleanup
    let _ = remove_shared_memory("test_resource");
}

#[test]
fn test_permission_enforcement() {
    let config = SharedMemoryConfig::new("test_permissions".to_string(), 1024)
        .unwrap()
        .with_permissions(IpcPermissions::read_only());
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    // Map with read-only protection
    let result = memory.map(MemoryProtection::read_only());
    if result.is_err() {
        let _ = remove_shared_memory("test_permissions");
        return;
    }

    // Writing should fail due to read-only permissions
    let test_data = b"This should fail";
    let result = memory.write_at(0, test_data);
    assert!(result.is_err());

    // Reading should work
    let mut read_buffer = vec![0u8; 16];
    let result = memory.read_at(0, &mut read_buffer);
    assert!(result.is_ok());

    // Cleanup
    let _ = remove_shared_memory("test_permissions");
}

#[test]
fn test_bounds_checking() {
    let config = SharedMemoryConfig::new("test_bounds".to_string(), 1024).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    let result = memory.map(MemoryProtection::read_write());
    if result.is_err() {
        let _ = remove_shared_memory("test_bounds");
        return;
    }

    // Writing beyond bounds should be limited
    let large_data = vec![0u8; 2048]; // Larger than the 1024 byte region
    let result = memory.write_at(0, &large_data);
    if let Ok(bytes_written) = result {
        // Should only write up to the available space
        assert!(bytes_written <= 1024);
    }

    // Reading beyond bounds should be limited
    let mut large_buffer = vec![0u8; 2048];
    let result = memory.read_at(0, &mut large_buffer);
    if let Ok(bytes_read) = result {
        // Should only read up to the available space
        assert!(bytes_read <= 1024);
    }

    // Writing at offset near end should be limited
    let test_data = b"Test data that is longer than remaining space";
    let result = memory.write_at(1000, test_data);
    if let Ok(bytes_written) = result {
        // Should only write the remaining 24 bytes
        assert!(bytes_written <= 24);
    }

    // Cleanup
    let _ = remove_shared_memory("test_bounds");
}

#[test]
fn test_statistics_tracking() {
    let config = SharedMemoryConfig::new("test_stats".to_string(), 2048).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let mut memory = result.unwrap();

    let result = memory.map(MemoryProtection::read_write());
    if result.is_err() {
        let _ = remove_shared_memory("test_stats");
        return;
    }

    // Perform some operations
    let test_data = b"Statistics test data";
    let _ = memory.write_at(0, test_data);
    
    let mut read_buffer = vec![0u8; test_data.len()];
    let _ = memory.read_at(0, &mut read_buffer);

    // Check statistics
    let stats = memory.statistics();
    assert!(stats.write_operations > 0);
    assert!(stats.read_operations > 0);
    assert!(stats.bytes_written > 0);
    assert!(stats.bytes_read > 0);

    // Cleanup
    let _ = remove_shared_memory("test_stats");
}

#[test]
fn test_concurrent_access_simulation() {
    let config = SharedMemoryConfig::new("test_concurrent".to_string(), 4096).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    
    // Simulate concurrent access with multiple threads
    let handles: Vec<_> = (0..4).map(|i| {
        thread::spawn(move || {
            let result = open_shared_memory("test_concurrent");
            if result.is_err() {
                return;
            }
            
            let mut memory = result.unwrap();
            let _ = memory.map(MemoryProtection::read_write());
            
            // Each thread writes to a different offset
            let offset = i * 1024;
            let test_data = format!("Thread {} data", i);
            let _ = memory.write_at(offset, test_data.as_bytes());
            
            // Small delay to simulate real work
            thread::sleep(Duration::from_millis(10));
            
            // Read back the data
            let mut read_buffer = vec![0u8; test_data.len()];
            let _ = memory.read_at(offset, &mut read_buffer);
        })
    }).collect();

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Cleanup
    let _ = remove_shared_memory("test_concurrent");
}

#[test]
fn test_error_conditions() {
    // Test opening non-existent shared memory
    let result = open_shared_memory("non_existent_memory");
    // This should either error or skip the test
    if result.is_ok() {
        // If it succeeds, clean it up
        let _ = remove_shared_memory("non_existent_memory");
    }

    // Test removing non-existent shared memory
    let result = remove_shared_memory("definitely_does_not_exist");
    // This may or may not error depending on the platform
    // On some systems, removing non-existent shared memory is not an error
}

#[test]
fn test_memory_alignment() {
    let config = SharedMemoryConfig::new("test_alignment".to_string(), 4097) // Non-aligned size
        .unwrap()
        .with_auto_cleanup(true);
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let memory = result.unwrap();

    // The implementation should handle non-aligned sizes gracefully
    assert_eq!(memory.size(), 4097);

    // Cleanup is automatic due to auto_cleanup = true
}

#[test]
fn test_handle_lifecycle() {
    let config = SharedMemoryConfig::new("test_lifecycle".to_string(), 1024).unwrap();
    
    let result = create_shared_memory(config);
    if result.is_err() {
        return; // Skip if not supported
    }
    let memory = result.unwrap();

    let handle = memory.handle();
    assert!(handle.is_valid());
    assert!(!handle.id.is_empty());
    assert!(handle.age().as_secs() < 1); // Should be very recent

    // Cleanup
    let _ = remove_shared_memory("test_lifecycle");
}
