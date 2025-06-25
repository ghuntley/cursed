/// Comprehensive test suite for memory-mapped files module
/// 
/// Tests cover:
/// - Basic memory mapping operations
/// - Cross-platform compatibility
/// - Memory safety and error handling
/// - IPC shared memory functionality
/// - Performance and stress testing
/// - Thread safety and concurrent access

use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use tempfile::{NamedTempFile, TempDir};

// Import the mmap module from the CURSED stdlib
use cursed::stdlib::process::mmap::{
    MmapConfig, MmapManager, ProtectionFlags, MappingType, MemoryAdvice, SyncType,
    map_file, map_anonymous, create_shared_memory, get_mmap_manager,
};
use cursed::error::CursedError;

#[test]
fn test_anonymous_memory_mapping() {
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 4096,
        ..Default::default()
    };

    let handle = map_anonymous(config).expect("Failed to create anonymous mapping");
    
    assert_eq!(handle.len(), 4096);
    assert!(handle.can_read());
    assert!(handle.can_write());
    assert!(!handle.can_execute());
    
    // Test writing and reading from the mapping
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0xDE;
        slice[1] = 0xAD;
        slice[2] = 0xBE;
        slice[3] = 0xEF;
        slice[4095] = 0xFF;
    }
    
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice[0], 0xDE);
    assert_eq!(slice[1], 0xAD);
    assert_eq!(slice[2], 0xBE);
    assert_eq!(slice[3], 0xEF);
    assert_eq!(slice[4095], 0xFF);
}

#[test]
fn test_file_backed_mapping() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let test_data = b"This is test data for memory mapping validation! 🦀";
    temp_file.write_all(test_data).expect("Failed to write test data");
    temp_file.flush().expect("Failed to flush temp file");
    
    let file = temp_file.reopen().expect("Failed to reopen temp file");
    let config = MmapConfig {
        protection: ProtectionFlags::Read,
        mapping_type: MappingType::Private,
        length: test_data.len(),
        offset: 0,
        ..Default::default()
    };
    
    let handle = map_file(&file, config).expect("Failed to map file");
    
    assert_eq!(handle.len(), test_data.len());
    assert!(handle.can_read());
    assert!(!handle.can_write());
    
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice, test_data);
}

#[test]
fn test_large_file_mapping() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("large_test_file.dat");
    
    // Create a 1MB file with pattern data
    let file_size = 1024 * 1024; // 1MB
    let mut file = File::create(&file_path).expect("Failed to create large file");
    
    let pattern = b"DEADBEEF";
    for i in 0..(file_size / pattern.len()) {
        file.write_all(pattern).expect("Failed to write pattern");
    }
    file.flush().expect("Failed to flush large file");
    
    let file = File::open(&file_path).expect("Failed to open large file");
    let config = MmapConfig {
        protection: ProtectionFlags::Read,
        mapping_type: MappingType::Private,
        length: file_size,
        offset: 0,
        ..Default::default()
    };
    
    let handle = map_file(&file, config).expect("Failed to map large file");
    assert_eq!(handle.len(), file_size);
    
    let slice = handle.as_slice().expect("Failed to get large file slice");
    
    // Verify pattern at different offsets
    assert_eq!(&slice[0..8], pattern);
    assert_eq!(&slice[1024..1032], pattern);
    assert_eq!(&slice[file_size - 8..file_size], pattern);
}

#[test]
fn test_memory_protection_changes() {
    let config = MmapConfig {
        protection: ProtectionFlags::Read,
        mapping_type: MappingType::Anonymous,
        length: 4096,
        ..Default::default()
    };
    
    let handle = map_anonymous(config).expect("Failed to create mapping");
    assert!(handle.can_read());
    assert!(!handle.can_write());
    
    // Change protection to read-write
    let mut mutable_handle = Arc::try_unwrap(handle).expect("Failed to unwrap handle");
    mutable_handle.protect(ProtectionFlags::ReadWrite).expect("Failed to change protection");
    
    assert!(mutable_handle.can_read());
    assert!(mutable_handle.can_write());
    
    // Test that we can now write
    {
        let slice = mutable_handle.as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0xAB;
        slice[100] = 0xCD;
    }
    
    let slice = mutable_handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice[0], 0xAB);
    assert_eq!(slice[100], 0xCD);
    
    // Change protection back to read-only
    mutable_handle.protect(ProtectionFlags::Read).expect("Failed to change protection back");
    assert!(mutable_handle.can_read());
    assert!(!mutable_handle.can_write());
}

#[test]
fn test_memory_advice_operations() {
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 8192,
        ..Default::default()
    };
    
    let handle = map_anonymous(config).expect("Failed to create mapping");
    
    // Test various memory advice operations
    handle.advise(MemoryAdvice::Sequential).expect("Failed to set sequential advice");
    handle.advise(MemoryAdvice::Random).expect("Failed to set random advice");
    handle.advise(MemoryAdvice::WillNeed).expect("Failed to set will-need advice");
    handle.advise(MemoryAdvice::DontNeed).expect("Failed to set don't-need advice");
    handle.advise(MemoryAdvice::Normal).expect("Failed to set normal advice");
    
    // Platform-specific advice (may be no-ops on some platforms)
    let _ = handle.advise(MemoryAdvice::DontDump);
    let _ = handle.advise(MemoryAdvice::DoDump);
}

#[test]
fn test_memory_locking() {
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 4096,
        ..Default::default()
    };
    
    let handle = map_anonymous(config).expect("Failed to create mapping");
    let mut mutable_handle = Arc::try_unwrap(handle).expect("Failed to unwrap handle");
    
    // Test locking pages in memory
    mutable_handle.lock_pages().expect("Failed to lock pages");
    
    // Write some data to locked pages
    {
        let slice = mutable_handle.as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0x12;
        slice[2048] = 0x34;
    }
    
    // Unlock pages
    mutable_handle.unlock_pages().expect("Failed to unlock pages");
    
    // Verify data is still there
    let slice = mutable_handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice[0], 0x12);
    assert_eq!(slice[2048], 0x34);
}

#[test]
fn test_sync_operations() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let initial_data = vec![0u8; 4096];
    temp_file.write_all(&initial_data).expect("Failed to write initial data");
    temp_file.flush().expect("Failed to flush temp file");
    
    let file = temp_file.reopen().expect("Failed to reopen temp file");
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Shared,
        length: 4096,
        offset: 0,
        ..Default::default()
    };
    
    let handle = map_file(&file, config).expect("Failed to map file");
    
    // Write some data
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0xFE;
        slice[1] = 0xED;
        slice[2] = 0xFA;
        slice[3] = 0xCE;
    }
    
    // Test sync operations
    handle.sync(SyncType::Async).expect("Failed to async sync");
    handle.sync(SyncType::Sync).expect("Failed to sync sync");
    handle.sync(SyncType::Invalidate).expect("Failed to invalidate sync");
}

#[test]
fn test_shared_memory_region() {
    let region = create_shared_memory("test_shared_region", 8192).expect("Failed to create shared memory");
    
    assert_eq!(region.name(), "test_shared_region");
    assert_eq!(region.size(), 8192);
    
    let handle = region.handle();
    assert!(handle.can_read());
    assert!(handle.can_write());
    assert_eq!(handle.len(), 8192);
    
    // Test writing and reading from shared memory
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0xBA;
        slice[1] = 0xBE;
        slice[8191] = 0xFF;
    }
    
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice[0], 0xBA);
    assert_eq!(slice[1], 0xBE);
    assert_eq!(slice[8191], 0xFF);
}

#[test]
fn test_handle_cloning_and_sharing() {
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 4096,
        ..Default::default()
    };
    
    let handle1 = map_anonymous(config).expect("Failed to create mapping");
    let handle2 = handle1.clone_handle();
    
    assert_eq!(handle1.as_ptr(), handle2.as_ptr());
    assert_eq!(handle1.len(), handle2.len());
    
    // Write data through first handle
    {
        let mut slice1 = Arc::try_unwrap(handle1).expect("Failed to unwrap handle1").as_mut_slice().expect("Failed to get mutable slice");
        slice1[0] = 0x99;
        slice1[1000] = 0x88;
    }
    
    // Read data through second handle
    let slice2 = handle2.as_slice().expect("Failed to get slice");
    assert_eq!(slice2[0], 0x99);
    assert_eq!(slice2[1000], 0x88);
}

#[test]
fn test_mmap_manager_statistics() {
    let manager = MmapManager::new();
    
    let stats_initial = manager.get_statistics();
    assert_eq!(stats_initial.total_mappings, 0);
    assert_eq!(stats_initial.total_size, 0);
    
    let config1 = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 4096,
        ..Default::default()
    };
    
    let config2 = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 8192,
        ..Default::default()
    };
    
    let _handle1 = manager.map_anonymous(config1).expect("Failed to create first mapping");
    let _handle2 = manager.map_anonymous(config2).expect("Failed to create second mapping");
    
    let stats_after = manager.get_statistics();
    assert_eq!(stats_after.total_mappings, 2);
    assert_eq!(stats_after.total_size, 4096 + 8192);
    assert_eq!(stats_after.active_handles, 2);
}

#[test]
fn test_error_conditions() {
    // Test zero-length mapping
    let config_zero = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: 0,
        ..Default::default()
    };
    
    let result = map_anonymous(config_zero);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CursedError::InvalidInput(_)));
    
    // Test invalid file mapping
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let file = temp_file.reopen().expect("Failed to reopen temp file");
    
    let config_too_large = MmapConfig {
        protection: ProtectionFlags::Read,
        mapping_type: MappingType::Private,
        length: 1024 * 1024 * 1024, // 1GB, likely larger than temp file
        offset: 0,
        ..Default::default()
    };
    
    let result = map_file(&file, config_too_large);
    // This may or may not fail depending on the system, but shouldn't panic
    let _ = result;
}

#[test]
fn test_concurrent_access() {
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Shared,
        length: 4096,
        ..Default::default()
    };
    
    let handle = map_anonymous(config).expect("Failed to create shared mapping");
    let barrier = Arc::new(Barrier::new(4));
    let mut threads = vec![];
    
    for thread_id in 0..4 {
        let handle_clone = handle.clone_handle();
        let barrier_clone = Arc::clone(&barrier);
        
        let thread = thread::spawn(move || {
            barrier_clone.wait();
            
            // Each thread writes to its own section
            let start_offset = thread_id * 1024;
            let mut slice = handle_clone.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
            
            for i in 0..1024 {
                slice[start_offset + i] = (thread_id as u8) * 16 + (i % 16) as u8;
            }
            
            thread_id
        });
        
        threads.push(thread);
    }
    
    // Wait for all threads to complete
    for thread in threads {
        let thread_id = thread.join().expect("Thread panicked");
        println!("Thread {} completed", thread_id);
    }
    
    // Verify that each thread wrote its data correctly
    let slice = handle.as_slice().expect("Failed to get slice");
    for thread_id in 0..4 {
        let start_offset = thread_id * 1024;
        for i in 0..1024 {
            let expected = (thread_id as u8) * 16 + (i % 16) as u8;
            assert_eq!(slice[start_offset + i], expected, 
                "Mismatch at thread {}, offset {}", thread_id, i);
        }
    }
}

#[test]
fn test_thread_safety_stress() {
    let manager = Arc::new(MmapManager::new());
    let barrier = Arc::new(Barrier::new(8));
    let mut threads = vec![];
    
    for thread_id in 0..8 {
        let manager_clone = Arc::clone(&manager);
        let barrier_clone = Arc::clone(&barrier);
        
        let thread = thread::spawn(move || {
            barrier_clone.wait();
            
            // Create multiple mappings per thread
            let mut handles = vec![];
            for i in 0..10 {
                let config = MmapConfig {
                    protection: ProtectionFlags::ReadWrite,
                    mapping_type: MappingType::Anonymous,
                    length: (thread_id + 1) * 1024 + i * 512,
                    ..Default::default()
                };
                
                let handle = manager_clone.map_anonymous(config).expect("Failed to create mapping");
                
                // Write thread-specific data
                {
                    let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
                    slice[0] = thread_id as u8;
                    slice[slice.len() - 1] = (thread_id + i) as u8;
                }
                
                handles.push(handle);
            }
            
            // Verify data integrity
            for (i, handle) in handles.iter().enumerate() {
                let slice = handle.as_slice().expect("Failed to get slice");
                assert_eq!(slice[0], thread_id as u8);
                assert_eq!(slice[slice.len() - 1], (thread_id + i) as u8);
            }
            
            thread_id
        });
        
        threads.push(thread);
    }
    
    // Wait for all threads to complete
    for thread in threads {
        let thread_id = thread.join().expect("Thread panicked");
        println!("Stress test thread {} completed", thread_id);
    }
    
    let stats = manager.get_statistics();
    assert_eq!(stats.total_mappings, 8 * 10); // 8 threads × 10 mappings each
    println!("Final statistics: {:?}", stats);
}

#[test]
fn test_performance_large_mappings() {
    use std::time::Instant;
    
    let sizes = vec![
        1024 * 1024,         // 1MB
        16 * 1024 * 1024,    // 16MB
        64 * 1024 * 1024,    // 64MB
    ];
    
    for size in sizes {
        let start = Instant::now();
        
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Anonymous,
            length: size,
            ..Default::default()
        };
        
        let handle = map_anonymous(config).expect("Failed to create large mapping");
        let creation_time = start.elapsed();
        
        // Write test pattern
        let write_start = Instant::now();
        {
            let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
            for chunk in slice.chunks_mut(4096) {
                chunk[0] = 0xAA;
                if chunk.len() > 1 {
                    chunk[chunk.len() - 1] = 0xBB;
                }
            }
        }
        let write_time = write_start.elapsed();
        
        // Read test pattern
        let read_start = Instant::now();
        let slice = handle.as_slice().expect("Failed to get slice");
        let mut checksum = 0u64;
        for chunk in slice.chunks(4096) {
            checksum = checksum.wrapping_add(chunk[0] as u64);
            if chunk.len() > 1 {
                checksum = checksum.wrapping_add(chunk[chunk.len() - 1] as u64);
            }
        }
        let read_time = read_start.elapsed();
        
        println!("Size: {}MB, Creation: {:?}, Write: {:?}, Read: {:?}, Checksum: {}",
                 size / (1024 * 1024), creation_time, write_time, read_time, checksum);
        
        // Performance assertions (these may need adjustment based on system)
        assert!(creation_time.as_millis() < 100, "Creation took too long: {:?}", creation_time);
        assert!(write_time.as_millis() < 1000, "Writing took too long: {:?}", write_time);
        assert!(read_time.as_millis() < 500, "Reading took too long: {:?}", read_time);
    }
}

#[test]
fn test_global_manager_singleton() {
    let manager1 = get_mmap_manager();
    let manager2 = get_mmap_manager();
    
    // Should be the same instance
    assert!(std::ptr::eq(manager1, manager2));
    
    // Test that it works across multiple threads
    let barrier = Arc::new(Barrier::new(4));
    let mut threads = vec![];
    
    for _ in 0..4 {
        let barrier_clone = Arc::clone(&barrier);
        let thread = thread::spawn(move || {
            barrier_clone.wait();
            let manager = get_mmap_manager();
            std::ptr::addr_of!(*manager) as usize
        });
        threads.push(thread);
    }
    
    let addresses: Vec<usize> = threads.into_iter()
        .map(|t| t.join().expect("Thread panicked"))
        .collect();
    
    // All threads should get the same manager instance
    for addr in &addresses[1..] {
        assert_eq!(addresses[0], *addr, "Manager instances differ across threads");
    }
}

#[test]
fn test_mapping_offset_and_partial_files() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let full_data = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    temp_file.write_all(full_data).expect("Failed to write full data");
    temp_file.flush().expect("Failed to flush temp file");
    
    let file = temp_file.reopen().expect("Failed to reopen temp file");
    
    // Map middle portion of the file
    let config = MmapConfig {
        protection: ProtectionFlags::Read,
        mapping_type: MappingType::Private,
        length: 10,  // Map 10 bytes
        offset: 10,  // Starting from offset 10
        ..Default::default()
    };
    
    let handle = map_file(&file, config).expect("Failed to map file with offset");
    let slice = handle.as_slice().expect("Failed to get slice");
    
    assert_eq!(slice.len(), 10);
    assert_eq!(slice, b"ABCDEFGHIJ"); // Should be bytes 10-19 from the original data
}

#[test]
fn test_memory_mapping_edge_cases() {
    // Test mapping at page boundaries
    let page_size = 4096; // Assume 4KB pages
    
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Anonymous,
        length: page_size * 3, // 3 pages
        ..Default::default()
    };
    
    let handle = map_anonymous(config).expect("Failed to create multi-page mapping");
    
    // Write to each page boundary
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = 0x11;                    // First byte of page 1
        slice[page_size - 1] = 0x22;       // Last byte of page 1
        slice[page_size] = 0x33;           // First byte of page 2
        slice[page_size * 2 - 1] = 0x44;   // Last byte of page 2
        slice[page_size * 2] = 0x55;       // First byte of page 3
        slice[page_size * 3 - 1] = 0x66;   // Last byte of page 3
    }
    
    // Verify all writes
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice[0], 0x11);
    assert_eq!(slice[page_size - 1], 0x22);
    assert_eq!(slice[page_size], 0x33);
    assert_eq!(slice[page_size * 2 - 1], 0x44);
    assert_eq!(slice[page_size * 2], 0x55);
    assert_eq!(slice[page_size * 3 - 1], 0x66);
}

#[test]
fn test_copy_on_write_mapping() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let original_data = b"Original data that should not be modified in file";
    temp_file.write_all(original_data).expect("Failed to write original data");
    temp_file.flush().expect("Failed to flush temp file");
    
    let file = temp_file.reopen().expect("Failed to reopen temp file");
    let config = MmapConfig {
        protection: ProtectionFlags::ReadWrite,
        mapping_type: MappingType::Private, // Copy-on-write
        length: original_data.len(),
        offset: 0,
        ..Default::default()
    };
    
    let handle = map_file(&file, config).expect("Failed to map file");
    
    // Read original data
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(slice, original_data);
    
    // Modify the mapping (should not affect the file due to COW)
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice[0] = b'M';
        slice[1] = b'o';
        slice[2] = b'd';
        slice[3] = b'i';
        slice[4] = b'f';
        slice[5] = b'i';
        slice[6] = b'e';
        slice[7] = b'd';
    }
    
    // Verify the mapping shows modified data
    let slice = handle.as_slice().expect("Failed to get slice");
    assert_eq!(&slice[0..8], b"Modified");
    
    // Verify the original file is unchanged by reading it again
    drop(handle); // Ensure mapping is closed
    
    let mut file_content = Vec::new();
    let mut file = temp_file.reopen().expect("Failed to reopen temp file");
    use std::io::Read;
    file.read_to_end(&mut file_content).expect("Failed to read file");
    assert_eq!(file_content, original_data); // File should be unchanged
}
