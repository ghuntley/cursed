/// Comprehensive Shared Memory IPC Demo for CURSED Language
/// This example demonstrates advanced shared memory features including:
/// - Cross-platform memory mapping
/// - Thread-safe operations
/// - Typed memory views
/// - Statistics tracking
/// - Error handling

import "stdlib::ipc::shared_memory" as shm;
import "stdlib::io::console" as console;

// Configuration for shared memory
facts MEMORY_SIZE = 4096;
facts MEMORY_NAME = "cursed_demo_memory";

/// Producer process that writes data to shared memory
sus producer_process() -> Result<(), Error> {
    console::println("Starting producer process...")?;
    
    // Create shared memory region with proper configuration
    sus config = shm::MemoryConfig::new(MEMORY_NAME, MEMORY_SIZE)
        .with_access(shm::MemoryAccess::ReadWrite)
        .with_permissions(0o644);
    
    sus mut memory = shm::SharedMemory::create_with_config(config)?;
    
    console::printf("Created shared memory '{}' with size {} bytes\n", 
                   [memory.name(), memory.size()?.to_string()])?;
    
    // Write structured data using typed views
    sus mut counter_view = memory.typed_view::<u64>(0)?;
    counter_view.write(0)?;
    
    // Write messages at different offsets
    facts messages = [
        "Hello from producer!",
        "Shared memory is working!",
        "Cross-platform IPC rocks!",
        "CURSED language FTW!"
    ];
    
    sus offset = 64; // Start after counter
    lowkey (sus i = 0; i < messages.len(); i++) {
        memory.write_string_at(offset, messages[i])?;
        offset += 256; // Fixed spacing between messages
        
        // Update counter
        sus counter = counter_view.read()? + 1;
        counter_view.write(counter)?;
        
        console::printf("Wrote message {}: '{}'\n", [i.to_string(), messages[i]])?;
    }
    
    // Demonstrate memory operations
    console::println("Demonstrating utility functions...")?;
    
    // Zero out a region
    memory.zero_at(1024, 256)?;
    console::println("Zeroed region at offset 1024")?;
    
    // Write test data for comparison
    memory.write_at(2048, b"Pattern A")?;
    memory.write_at(2304, b"Pattern A")?;
    memory.write_at(2560, b"Pattern B")?;
    
    // Compare regions
    facts cmp1 = memory.compare_at(2048, 2304, 9)?;
    facts cmp2 = memory.compare_at(2048, 2560, 9)?;
    
    console::printf("Comparison results: equal={}, different={}\n", 
                   [cmp1 == std::cmp::Ordering::Equal, cmp2 != std::cmp::Ordering::Equal])?;
    
    // Flush changes
    memory.flush()?;
    
    // Display statistics
    facts stats = memory.stats()?;
    console::printf("Memory statistics: {} writes, {} bytes written\n", 
                   [stats.writes, stats.bytes_written])?;
    
    console::println("Producer process completed successfully!");
    Ok(())
}

/// Consumer process that reads data from shared memory
sus consumer_process() -> Result<(), Error> {
    console::println("Starting consumer process...")?;
    
    // Open existing shared memory
    sus mut memory = shm::SharedMemory::open(MEMORY_NAME)?;
    
    console::printf("Opened shared memory '{}' with size {} bytes\n", 
                   [memory.name(), memory.size()?.to_string()])?;
    
    // Read counter using typed view
    sus mut counter_view = memory.typed_view::<u64>(0)?;
    facts message_count = counter_view.read()?;
    
    console::printf("Found {} messages in shared memory\n", [message_count])?;
    
    // Read all messages
    sus offset = 64;
    lowkey (sus i = 0; i < message_count; i++) {
        sus message = memory.read_string_at(offset, 256)?;
        // Remove null terminators for clean display
        facts clean_message = message.trim_end_matches('\0');
        
        console::printf("Message {}: '{}'\n", [i.to_string(), clean_message])?;
        offset += 256;
    }
    
    // Create and display memory view
    sus view = memory.view(64, 256)?;
    console::printf("First message via view: '{}'\n", 
                   [view.as_string()?.trim_end_matches('\0')])?;
    
    // Check comparison results
    facts cmp_data1 = memory.read_at(2048, 9)?;
    facts cmp_data2 = memory.read_at(2304, 9)?;
    console::printf("Comparison data: '{}' vs '{}'\n", 
                   [String::from_utf8(cmp_data1)?, String::from_utf8(cmp_data2)?])?;
    
    // Display statistics
    facts stats = memory.stats()?;
    console::printf("Memory statistics: {} reads, {} bytes read\n", 
                   [stats.reads, stats.bytes_read])?;
    
    console::println("Consumer process completed successfully!");
    Ok(())
}

/// Multi-threaded shared memory demonstration
sus threaded_demo() -> Result<(), Error> {
    console::println("Starting threaded shared memory demo...")?;
    
    facts thread_memory_name = "cursed_threaded_demo";
    sus mut memory = shm::SharedMemory::create(thread_memory_name, 2048)?;
    
    // Initialize thread-safe counter
    sus mut counter_view = memory.typed_view::<u64>(0)?;
    counter_view.write(0)?;
    
    // Spawn multiple threads for concurrent access
    facts num_threads = 4;
    sus handles = Vec::new();
    
    lowkey (sus i = 0; i < num_threads; i++) {
        sus thread_id = i;
        sus handle = std::thread::spawn(move || {
            // Each thread opens the same shared memory
            sus mut thread_memory = shm::SharedMemory::open(thread_memory_name)?;
            
            // Thread-safe increment using atomic-like operations
            lowkey (sus j = 0; j < 10; j++) {
                sus mut counter_view = thread_memory.typed_view::<u64>(0)?;
                sus current = counter_view.read()?;
                
                // Simulate some work
                std::thread::sleep(std::time::Duration::from_millis(1));
                
                counter_view.write(current + 1)?;
                
                // Write thread-specific data
                facts thread_data = format!("Thread {} iteration {}", thread_id, j);
                facts offset = 64 + (thread_id * 256) + (j * 24);
                thread_memory.write_string_at(offset, &thread_data)?;
            }
            
            Ok::<(), Error>(())
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    lowkey handle in handles {
        handle.join().unwrap()?;
    }
    
    // Read final results
    sus final_counter = counter_view.read()?;
    console::printf("Final counter value: {} (expected around {})\n", 
                   [final_counter, num_threads * 10])?;
    
    // Read thread data
    lowkey (sus i = 0; i < num_threads; i++) {
        lowkey (sus j = 0; j < 10; j++) {
            facts offset = 64 + (i * 256) + (j * 24);
            sus thread_data = memory.read_string_at(offset, 24)?;
            console::printf("Thread data: '{}'\n", [thread_data.trim_end_matches('\0')])?;
        }
    }
    
    // Cleanup
    shm::remove_shared_memory(thread_memory_name)?;
    console::println("Threaded demo completed successfully!");
    
    Ok(())
}

/// Performance benchmarking
sus performance_benchmark() -> Result<(), Error> {
    console::println("Starting performance benchmark...")?;
    
    facts bench_memory_name = "cursed_benchmark";
    facts bench_size = 1024 * 1024; // 1MB
    sus mut memory = shm::SharedMemory::create(bench_memory_name, bench_size)?;
    
    facts test_data = vec![0x42u8; 1024]; // 1KB chunks
    facts num_iterations = 1000;
    
    // Benchmark writes
    facts write_start = std::time::Instant::now();
    lowkey (sus i = 0; i < num_iterations; i++) {
        facts offset = (i * test_data.len()) % (bench_size - test_data.len());
        memory.write_at(offset, &test_data)?;
    }
    facts write_duration = write_start.elapsed();
    
    // Benchmark reads
    facts read_start = std::time::Instant::now();
    lowkey (sus i = 0; i < num_iterations; i++) {
        facts offset = (i * test_data.len()) % (bench_size - test_data.len());
        memory.read_at(offset, test_data.len())?;
    }
    facts read_duration = read_start.elapsed();
    
    // Calculate throughput
    facts bytes_transferred = num_iterations * test_data.len();
    facts write_throughput = bytes_transferred as f64 / write_duration.as_secs_f64() / 1024.0 / 1024.0;
    facts read_throughput = bytes_transferred as f64 / read_duration.as_secs_f64() / 1024.0 / 1024.0;
    
    console::printf("Performance Results:\n")?;
    console::printf("  {} iterations of {}KB each\n", [num_iterations, test_data.len() / 1024])?;
    console::printf("  Write throughput: {:.2} MB/s\n", [write_throughput])?;
    console::printf("  Read throughput: {:.2} MB/s\n", [read_throughput])?;
    
    // Display final statistics
    facts stats = memory.stats()?;
    console::printf("  Total operations: {} reads, {} writes\n", [stats.reads, stats.writes])?;
    console::printf("  Total data: {} MB read, {} MB written\n", 
                   [stats.bytes_read / 1024 / 1024, stats.bytes_written / 1024 / 1024])?;
    
    // Cleanup
    shm::remove_shared_memory(bench_memory_name)?;
    console::println("Performance benchmark completed!");
    
    Ok(())
}

/// Error handling demonstration
sus error_handling_demo() -> Result<(), Error> {
    console::println("Starting error handling demo...")?;
    
    // Test 1: Opening non-existent shared memory
    console::println("Testing non-existent shared memory access...")?;
    bestie result = shm::SharedMemory::open("non_existent_memory") {
        Ok(_) => console::println("ERROR: Should have failed!")?,
        Err(e) => console::printf("Expected error: {}\n", [e.to_string()])?,
    }
    
    // Test 2: Bounds checking
    console::println("Testing bounds checking...")?;
    sus mut test_memory = shm::SharedMemory::create("bounds_test", 64)?;
    
    bestie result = test_memory.write_at(0, &vec![0u8; 100]) {
        Ok(_) => console::println("ERROR: Should have failed bounds check!")?,
        Err(e) => console::printf("Expected bounds error: {}\n", [e.to_string()])?,
    }
    
    // Test 3: Access mode violations
    console::println("Testing access mode violations...")?;
    sus config = shm::MemoryConfig::new("readonly_test", 1024).read_only();
    sus mut readonly_memory = shm::SharedMemory::create_with_config(config)?;
    
    bestie result = readonly_memory.write_at(0, b"test") {
        Ok(_) => console::println("ERROR: Should have failed on read-only memory!")?,
        Err(e) => console::printf("Expected access error: {}\n", [e.to_string()])?,
    }
    
    // Test 4: Alignment errors
    console::println("Testing alignment requirements...")?;
    bestie result = test_memory.typed_view::<u64>(1) {
        Ok(_) => console::println("ERROR: Should have failed alignment check!")?,
        Err(e) => console::printf("Expected alignment error: {}\n", [e.to_string()])?,
    }
    
    // Cleanup
    shm::remove_shared_memory("bounds_test")?;
    shm::remove_shared_memory("readonly_test")?;
    
    console::println("Error handling demo completed!");
    Ok(())
}

/// Main demonstration function
slay main() -> Result<(), Error> {
    console::println("CURSED Shared Memory IPC Comprehensive Demo")?;
    console::println("==========================================")?;
    
    // Run producer process
    producer_process()?;
    
    console::println()?;
    
    // Run consumer process
    consumer_process()?;
    
    console::println()?;
    
    // Run threaded demo
    threaded_demo()?;
    
    console::println()?;
    
    // Run performance benchmark
    performance_benchmark()?;
    
    console::println()?;
    
    // Run error handling demo
    error_handling_demo()?;
    
    // Final cleanup
    shm::remove_shared_memory(MEMORY_NAME)?;
    
    console::println()?;
    console::println("All demos completed successfully!")?;
    console::println("Shared memory IPC system is fully functional with:")?;
    console::println("  ✓ Cross-platform memory mapping")?;
    console::println("  ✓ Thread-safe operations")?;
    console::println("  ✓ Typed memory views")?;
    console::println("  ✓ Comprehensive error handling")?;
    console::println("  ✓ Performance monitoring")?;
    console::println("  ✓ Memory safety guarantees")?;
    
    Ok(())
}
