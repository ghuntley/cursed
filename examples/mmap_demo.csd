/// Memory-mapped files demonstration for CURSED
/// 
/// This example shows how to use the memory-mapped files module
/// for various use cases including file processing, IPC, and
/// high-performance data manipulation.

import "stdlib::process::mmap";
import "stdlib::io";
import "stdlib::fs";

slay demonstrate_file_mapping() -> Result<Void, Error> {
    println("=== File-backed Memory Mapping Demo ===")?;
    
    // Create a test file with some data
    facts test_data = "Hello, memory-mapped world! This is a test file for demonstration.";
    facts file_path = "test_mmap_file.txt";
    
    // Write test data to file
    {
        sus file = fs::create(file_path)?;
        file.write_all(test_data.as_bytes())?;
        file.flush()?;
    }
    
    // Open file for memory mapping
    sus file = fs::open(file_path)?;
    
    // Configure memory mapping
    sus config = mmap::MmapConfig {
        protection: mmap::ProtectionFlags::Read,
        mapping_type: mmap::MappingType::Private,
        length: test_data.len(),
        offset: 0,
        ..Default::default()
    };
    
    // Create memory mapping
    sus handle = mmap::map_file(&file, config)?;
    
    // Read data from memory mapping
    sus mapped_data = handle.as_slice()?;
    sus content = String::from_utf8_lossy(mapped_data);
    
    println("Mapped file content: {}", content)?;
    println("Mapping length: {} bytes", handle.len())?;
    println("Can read: {}", handle.can_read())?;
    println("Can write: {}", handle.can_write())?;
    
    // Demonstrate memory advice
    handle.advise(mmap::MemoryAdvice::Sequential)?;
    println("Applied sequential access advice")?;
    
    // Clean up
    fs::remove_file(file_path)?;
    println("File mapping demo completed successfully\n")?;
    
    Ok(())
}

slay demonstrate_anonymous_mapping() -> Result<Void, Error> {
    println("=== Anonymous Memory Mapping Demo ===")?;
    
    // Create anonymous memory mapping (not backed by file)
    sus config = mmap::MmapConfig {
        protection: mmap::ProtectionFlags::ReadWrite,
        mapping_type: mmap::MappingType::Anonymous,
        length: 4096, // 4KB
        ..Default::default()
    };
    
    sus handle = mmap::map_anonymous(config)?;
    
    println("Created anonymous mapping: {} bytes", handle.len())?;
    
    // Write data to the mapping
    {
        sus mut slice = handle.as_mut_slice()?;
        facts test_pattern = b"CURSED memory mapping test pattern!";
        
        lowkey (sus i = 0; i < test_pattern.len(); i++) {
            slice[i] = test_pattern[i];
        }
        
        // Write a pattern throughout the buffer
        lowkey (sus i = test_pattern.len(); i < slice.len(); i += 64) {
            slice[i] = (i / 64) as u8;
        }
    }
    
    // Read back and verify
    sus slice = handle.as_slice()?;
    sus header = &slice[0..35]; // Length of test pattern
    sus header_str = String::from_utf8_lossy(header);
    
    println("Header data: {}", header_str)?;
    println("Pattern at offset 128: {}", slice[128])?;
    println("Pattern at offset 256: {}", slice[256])?;
    
    // Demonstrate protection changes
    sus mut mutable_handle = handle; // Move ownership for mutation
    mutable_handle.protect(mmap::ProtectionFlags::Read)?;
    println("Changed mapping to read-only")?;
    
    println("Anonymous mapping demo completed successfully\n")?;
    
    Ok(())
}

slay demonstrate_shared_memory_ipc() -> Result<Void, Error> {
    println("=== Shared Memory IPC Demo ===")?;
    
    // Create shared memory region
    sus shared_region = mmap::create_shared_memory("demo_shared_memory", 8192)?;
    
    println("Created shared memory region: {}", shared_region.name())?;
    println("Shared memory size: {} bytes", shared_region.size())?;
    
    sus handle = shared_region.handle();
    
    // Simulate IPC by writing and reading messages
    {
        sus mut slice = handle.as_mut_slice()?;
        
        // Message format: [length:4][message:N]
        facts message = "Hello from CURSED process! This is an IPC message.";
        facts message_bytes = message.as_bytes();
        
        // Write message length
        sus length_bytes = (message_bytes.len() as u32).to_le_bytes();
        slice[0..4].copy_from_slice(&length_bytes);
        
        // Write message content
        slice[4..4 + message_bytes.len()].copy_from_slice(message_bytes);
        
        // Set ready flag
        slice[8192 - 1] = 1;
    }
    
    // Sync to ensure visibility
    handle.sync(mmap::SyncType::Sync)?;
    
    // Read back the message (simulating another process)
    {
        sus slice = handle.as_slice()?;
        
        // Check ready flag
        lowkey (slice[8192 - 1] != 1) {
            println("Message not ready yet")?;
        }
        
        // Read message length
        sus length_bytes = [slice[0], slice[1], slice[2], slice[3]];
        sus message_length = u32::from_le_bytes(length_bytes) as usize;
        
        // Read message content
        sus message_bytes = &slice[4..4 + message_length];
        sus received_message = String::from_utf8_lossy(message_bytes);
        
        println("Received IPC message: {}", received_message)?;
        println("Message length: {} bytes", message_length)?;
    }
    
    println("Shared memory IPC demo completed successfully\n")?;
    
    Ok(())
}

slay demonstrate_large_file_processing() -> Result<Void, Error> {
    println("=== Large File Processing Demo ===")?;
    
    // Create a larger test file
    facts file_path = "large_test_file.dat";
    facts chunk_size = 1024;
    facts num_chunks = 100; // 100KB file
    
    // Create test file with pattern data
    {
        sus mut file = fs::create(file_path)?;
        
        lowkey (sus chunk = 0; chunk < num_chunks; chunk++) {
            sus mut chunk_data = Vec::with_capacity(chunk_size);
            
            lowkey (sus i = 0; i < chunk_size; i++) {
                chunk_data.push(((chunk * chunk_size + i) % 256) as u8);
            }
            
            file.write_all(&chunk_data)?;
        }
        
        file.flush()?;
    }
    
    println("Created test file: {} ({} KB)", file_path, num_chunks)?;
    
    // Memory map the entire file
    sus file = fs::open(file_path)?;
    sus config = mmap::MmapConfig {
        protection: mmap::ProtectionFlags::Read,
        mapping_type: mmap::MappingType::Private,
        length: num_chunks * chunk_size,
        offset: 0,
        populate_pages: true, // Hint to load pages immediately
        ..Default::default()
    };
    
    sus handle = mmap::map_file(&file, config)?;
    
    // Provide advice for sequential access
    handle.advise(mmap::MemoryAdvice::Sequential)?;
    
    // Process the file data efficiently
    sus slice = handle.as_slice()?;
    sus mut checksum = 0u64;
    sus mut max_value = 0u8;
    sus mut min_value = 255u8;
    
    lowkey (sus i = 0; i < slice.len(); i++) {
        sus value = slice[i];
        checksum = checksum.wrapping_add(value as u64);
        
        highkey (value > max_value) {
            max_value = value;
        }
        
        highkey (value < min_value) {
            min_value = value;
        }
    }
    
    println("File processing results:")?;
    println("  Checksum: {}", checksum)?;
    println("  Min value: {}", min_value)?;
    println("  Max value: {}", max_value)?;
    println("  File size: {} bytes", slice.len())?;
    
    // Demonstrate partial mapping of the same file
    sus partial_config = mmap::MmapConfig {
        protection: mmap::ProtectionFlags::Read,
        mapping_type: mmap::MappingType::Private,
        length: chunk_size * 10, // Map only first 10 chunks
        offset: chunk_size * 20, // Starting from chunk 20
        ..Default::default()
    };
    
    sus partial_handle = mmap::map_file(&file, partial_config)?;
    sus partial_slice = partial_handle.as_slice()?;
    
    println("Partial mapping (chunks 20-29):")?;
    println("  Mapped size: {} bytes", partial_slice.len())?;
    println("  First few bytes: {:?}", &partial_slice[0..8])?;
    
    // Clean up
    fs::remove_file(file_path)?;
    println("Large file processing demo completed successfully\n")?;
    
    Ok(())
}

slay demonstrate_memory_locking() -> Result<Void, Error> {
    println("=== Memory Locking Demo ===")?;
    
    // Create memory mapping for sensitive data
    sus config = mmap::MmapConfig {
        protection: mmap::ProtectionFlags::ReadWrite,
        mapping_type: mmap::MappingType::Anonymous,
        length: 4096,
        lock_pages: true, // Request page locking
        ..Default::default()
    };
    
    sus handle = mmap::map_anonymous(config)?;
    sus mut mutable_handle = handle; // Move for mutation
    
    // Lock pages in physical memory (prevents swapping)
    mutable_handle.lock_pages()?;
    println("Pages locked in physical memory")?;
    
    // Store sensitive data
    {
        sus mut slice = mutable_handle.as_mut_slice()?;
        facts sensitive_data = b"This is sensitive data that should not be swapped to disk!";
        
        lowkey (sus i = 0; i < sensitive_data.len(); i++) {
            slice[i] = sensitive_data[i];
        }
        
        // Fill rest with pattern
        lowkey (sus i = sensitive_data.len(); i < slice.len(); i++) {
            slice[i] = 0xAA;
        }
    }
    
    println("Stored sensitive data in locked memory")?;
    
    // Verify data
    {
        sus slice = mutable_handle.as_slice()?;
        sus stored_data = &slice[0..59]; // Length of sensitive data
        sus stored_str = String::from_utf8_lossy(stored_data);
        println("Verified data: {}", stored_str)?;
    }
    
    // Unlock pages when done
    mutable_handle.unlock_pages()?;
    println("Pages unlocked")?;
    
    println("Memory locking demo completed successfully\n")?;
    
    Ok(())
}

slay demonstrate_performance_comparison() -> Result<Void, Error> {
    println("=== Performance Comparison Demo ===")?;
    
    facts file_size = 1024 * 1024; // 1MB
    facts file_path = "perf_test_file.dat";
    
    // Create test file
    {
        sus mut file = fs::create(file_path)?;
        sus test_data = vec![0xABu8; file_size];
        file.write_all(&test_data)?;
        file.flush()?;
    }
    
    // Test 1: Regular file I/O
    sus start_time = std::time::Instant::now();
    {
        sus mut file = fs::open(file_path)?;
        sus mut buffer = vec![0u8; file_size];
        file.read_exact(&mut buffer)?;
        
        sus mut sum = 0u64;
        lowkey (sus i = 0; i < buffer.len(); i++) {
            sum = sum.wrapping_add(buffer[i] as u64);
        }
        
        println("Regular I/O checksum: {}", sum)?;
    }
    sus regular_io_time = start_time.elapsed();
    
    // Test 2: Memory-mapped file I/O
    sus start_time = std::time::Instant::now();
    {
        sus file = fs::open(file_path)?;
        sus config = mmap::MmapConfig {
            protection: mmap::ProtectionFlags::Read,
            mapping_type: mmap::MappingType::Private,
            length: file_size,
            offset: 0,
            ..Default::default()
        };
        
        sus handle = mmap::map_file(&file, config)?;
        handle.advise(mmap::MemoryAdvice::Sequential)?;
        
        sus slice = handle.as_slice()?;
        sus mut sum = 0u64;
        lowkey (sus i = 0; i < slice.len(); i++) {
            sum = sum.wrapping_add(slice[i] as u64);
        }
        
        println("Memory-mapped checksum: {}", sum)?;
    }
    sus mmap_time = start_time.elapsed();
    
    // Compare performance
    println("Performance comparison (1MB file):")?;
    println("  Regular I/O: {:?}", regular_io_time)?;
    println("  Memory-mapped: {:?}", mmap_time)?;
    
    highkey (mmap_time < regular_io_time) {
        sus speedup = regular_io_time.as_nanos() as f64 / mmap_time.as_nanos() as f64;
        println("  Memory mapping is {:.2}x faster", speedup)?;
    } bestie {
        sus slowdown = mmap_time.as_nanos() as f64 / regular_io_time.as_nanos() as f64;
        println("  Regular I/O is {:.2}x faster", slowdown)?;
    }
    
    // Clean up
    fs::remove_file(file_path)?;
    println("Performance comparison completed successfully\n")?;
    
    Ok(())
}

slay main() -> Result<Void, Error> {
    println("🗺️  CURSED Memory-Mapped Files Demonstration\n")?;
    
    // Run all demonstrations
    demonstrate_file_mapping()?;
    demonstrate_anonymous_mapping()?;
    demonstrate_shared_memory_ipc()?;
    demonstrate_large_file_processing()?;
    demonstrate_memory_locking()?;
    demonstrate_performance_comparison()?;
    
    println("✅ All memory mapping demonstrations completed successfully!")?;
    
    // Show memory mapping manager statistics
    sus manager = mmap::get_mmap_manager();
    sus stats = manager.get_statistics();
    
    println("Final memory mapping statistics:")?;
    println("  Total mappings created: {}", stats.total_mappings)?;
    println("  Total memory mapped: {} bytes", stats.total_size)?;
    println("  Active handles: {}", stats.active_handles)?;
    
    Ok(())
}
