/// Integration tests for memory-mapped file IPC functionality
/// 
/// This test suite demonstrates how memory-mapped files can be used
/// for inter-process communication and shared memory scenarios.

use std::fs::{File, OpenOptions};
use std::io::{Write, Read, Seek, SeekFrom};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::{NamedTempFile, TempDir};

// Import the mmap module from the CURSED stdlib
use cursed::stdlib::process::mmap::{
    MmapConfig, ProtectionFlags, MappingType, MemoryAdvice, SyncType,
    map_file, map_anonymous, create_shared_memory, get_mmap_manager,
    MmapHandle, SharedMemoryRegion,
};
use cursed::error::CursedError;

/// Simulate shared memory communication between "processes" (threads)
#[test]
fn test_shared_memory_communication() {
    const BUFFER_SIZE: usize = 4096;
    const MESSAGE_SIZE: usize = 256;
    const NUM_MESSAGES: usize = 10;
    
    // Create shared memory region
    let shared_region = create_shared_memory("ipc_comm_test", BUFFER_SIZE)
        .expect("Failed to create shared memory region");
    
    let handle = shared_region.handle().clone();
    
    // Shared state for synchronization
    let producer_ready = Arc::new(Barrier::new(2));
    let consumer_ready = Arc::new(Barrier::new(2));
    let messages_sent = Arc::new(Mutex::new(0usize));
    let messages_received = Arc::new(Mutex::new(0usize));
    
    // Producer thread
    let producer_barrier = Arc::clone(&producer_ready);
    let consumer_barrier = Arc::clone(&consumer_ready);
    let sent_counter = Arc::clone(&messages_sent);
    let producer_handle = handle.clone_handle();
    
    let producer = thread::spawn(move || {
        producer_barrier.wait(); // Wait for consumer to be ready
        
        for i in 0..NUM_MESSAGES {
            let message = format!("Message #{:03} from producer - {}", i, "A".repeat(200));
            let message_bytes = message.as_bytes();
            
            // Write message to shared memory
            {
                let mut slice = producer_handle.as_ref().clone().as_mut_slice()
                    .expect("Failed to get mutable slice");
                
                // Write message length first (4 bytes)
                let len_bytes = (message_bytes.len() as u32).to_le_bytes();
                slice[0..4].copy_from_slice(&len_bytes);
                
                // Write message content
                slice[4..4 + message_bytes.len()].copy_from_slice(message_bytes);
                
                // Set ready flag
                slice[BUFFER_SIZE - 1] = 1;
            }
            
            // Sync to ensure visibility
            producer_handle.sync(SyncType::Sync).expect("Failed to sync");
            
            // Wait for consumer to read message
            loop {
                let slice = producer_handle.as_slice().expect("Failed to get slice");
                if slice[BUFFER_SIZE - 1] == 0 {
                    break; // Consumer has read the message
                }
                thread::sleep(Duration::from_millis(1));
            }
            
            let mut count = sent_counter.lock().unwrap();
            *count += 1;
        }
        
        consumer_barrier.wait(); // Signal completion
    });
    
    // Consumer thread
    let producer_barrier = Arc::clone(&producer_ready);
    let consumer_barrier = Arc::clone(&consumer_ready);
    let received_counter = Arc::clone(&messages_received);
    let consumer_handle = handle.clone_handle();
    
    let consumer = thread::spawn(move || {
        producer_barrier.wait(); // Signal ready to producer
        
        for expected_i in 0..NUM_MESSAGES {
            // Wait for message to be available
            loop {
                let slice = consumer_handle.as_slice().expect("Failed to get slice");
                if slice[BUFFER_SIZE - 1] == 1 {
                    break; // Message is ready
                }
                thread::sleep(Duration::from_millis(1));
            }
            
            // Read message from shared memory
            let message = {
                let slice = consumer_handle.as_slice().expect("Failed to get slice");
                
                // Read message length
                let len_bytes = [slice[0], slice[1], slice[2], slice[3]];
                let message_len = u32::from_le_bytes(len_bytes) as usize;
                
                // Read message content
                let message_bytes = &slice[4..4 + message_len];
                String::from_utf8_lossy(message_bytes).to_string()
            };
            
            // Verify message content
            assert!(message.starts_with(&format!("Message #{:03} from producer", expected_i)));
            assert!(message.contains(&"A".repeat(200)));
            
            // Clear ready flag to signal we've read the message
            {
                let mut slice = consumer_handle.as_ref().clone().as_mut_slice()
                    .expect("Failed to get mutable slice");
                slice[BUFFER_SIZE - 1] = 0;
            }
            
            consumer_handle.sync(SyncType::Sync).expect("Failed to sync");
            
            let mut count = received_counter.lock().unwrap();
            *count += 1;
        }
        
        consumer_barrier.wait(); // Signal completion
    });
    
    // Wait for both threads to complete
    producer.join().expect("Producer thread panicked");
    consumer.join().expect("Consumer thread panicked");
    
    // Verify all messages were sent and received
    let sent = *messages_sent.lock().unwrap();
    let received = *messages_received.lock().unwrap();
    
    assert_eq!(sent, NUM_MESSAGES);
    assert_eq!(received, NUM_MESSAGES);
    
    println!("Successfully communicated {} messages via shared memory", NUM_MESSAGES);
}

/// Test shared memory ring buffer implementation
#[test]
fn test_shared_memory_ring_buffer() {
    const BUFFER_SIZE: usize = 8192;
    const ITEM_SIZE: usize = 64;
    const NUM_ITEMS: usize = BUFFER_SIZE / ITEM_SIZE;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 2;
    const ITEMS_PER_PRODUCER: usize = 50;
    
    // Create shared memory for ring buffer
    let shared_region = create_shared_memory("ring_buffer_test", BUFFER_SIZE)
        .expect("Failed to create shared memory region");
    let handle = shared_region.handle().clone();
    
    // Initialize ring buffer structure
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        // Clear the buffer
        slice.fill(0);
        
        // Ring buffer header:
        // [0..4]: head index (producer)
        // [4..8]: tail index (consumer)
        // [8..12]: item count
        // [12..ITEM_SIZE]: padding
        // [ITEM_SIZE..]: ring buffer data
    }
    
    let all_ready = Arc::new(Barrier::new(NUM_PRODUCERS + NUM_CONSUMERS));
    let completion_barrier = Arc::new(Barrier::new(NUM_PRODUCERS + NUM_CONSUMERS));
    let produced_items = Arc::new(Mutex::new(Vec::new()));
    let consumed_items = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = vec![];
    
    // Spawn producer threads
    for producer_id in 0..NUM_PRODUCERS {
        let handle_clone = handle.clone_handle();
        let ready_barrier = Arc::clone(&all_ready);
        let completion_barrier = Arc::clone(&completion_barrier);
        let produced_items = Arc::clone(&produced_items);
        
        let producer = thread::spawn(move || {
            ready_barrier.wait(); // Wait for all threads to be ready
            
            for item_num in 0..ITEMS_PER_PRODUCER {
                let item_data = format!("P{:02}-{:03}", producer_id, item_num);
                let item_id = producer_id * 1000 + item_num;
                
                // Find a slot in the ring buffer
                loop {
                    let (head, tail, count) = {
                        let slice = handle_clone.as_slice().expect("Failed to get slice");
                        let head = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as usize;
                        let tail = u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]) as usize;
                        let count = u32::from_le_bytes([slice[8], slice[9], slice[10], slice[11]]) as usize;
                        (head, tail, count)
                    };
                    
                    if count < NUM_ITEMS {
                        // We can insert an item
                        let slot_offset = ITEM_SIZE + (head % NUM_ITEMS) * ITEM_SIZE;
                        
                        {
                            let mut slice = handle_clone.as_ref().clone().as_mut_slice()
                                .expect("Failed to get mutable slice");
                            
                            // Write item ID (4 bytes) and data
                            let id_bytes = (item_id as u32).to_le_bytes();
                            slice[slot_offset..slot_offset + 4].copy_from_slice(&id_bytes);
                            let data_bytes = item_data.as_bytes();
                            let copy_len = std::cmp::min(data_bytes.len(), ITEM_SIZE - 4);
                            slice[slot_offset + 4..slot_offset + 4 + copy_len].copy_from_slice(&data_bytes[..copy_len]);
                            
                            // Update head and count atomically (simplified)
                            let new_head = (head + 1) % (NUM_ITEMS * 2); // Prevent overflow
                            let new_count = count + 1;
                            
                            let head_bytes = (new_head as u32).to_le_bytes();
                            let count_bytes = (new_count as u32).to_le_bytes();
                            slice[0..4].copy_from_slice(&head_bytes);
                            slice[8..12].copy_from_slice(&count_bytes);
                        }
                        
                        handle_clone.sync(SyncType::Sync).expect("Failed to sync");
                        
                        // Record produced item
                        {
                            let mut produced = produced_items.lock().unwrap();
                            produced.push(item_id);
                        }
                        
                        break;
                    } else {
                        // Buffer is full, wait a bit
                        thread::sleep(Duration::from_micros(100));
                    }
                }
            }
            
            completion_barrier.wait();
        });
        
        handles.push(producer);
    }
    
    // Spawn consumer threads
    for consumer_id in 0..NUM_CONSUMERS {
        let handle_clone = handle.clone_handle();
        let ready_barrier = Arc::clone(&all_ready);
        let completion_barrier = Arc::clone(&completion_barrier);
        let consumed_items = Arc::clone(&consumed_items);
        
        let consumer = thread::spawn(move || {
            ready_barrier.wait(); // Wait for all threads to be ready
            
            let items_per_consumer = (NUM_PRODUCERS * ITEMS_PER_PRODUCER) / NUM_CONSUMERS;
            let mut consumed_count = 0;
            
            while consumed_count < items_per_consumer {
                let (head, tail, count) = {
                    let slice = handle_clone.as_slice().expect("Failed to get slice");
                    let head = u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]) as usize;
                    let tail = u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]) as usize;
                    let count = u32::from_le_bytes([slice[8], slice[9], slice[10], slice[11]]) as usize;
                    (head, tail, count)
                };
                
                if count > 0 {
                    // We can consume an item
                    let slot_offset = ITEM_SIZE + (tail % NUM_ITEMS) * ITEM_SIZE;
                    
                    let (item_id, _item_data) = {
                        let slice = handle_clone.as_slice().expect("Failed to get slice");
                        let id_bytes = [slice[slot_offset], slice[slot_offset + 1], slice[slot_offset + 2], slice[slot_offset + 3]];
                        let item_id = u32::from_le_bytes(id_bytes) as usize;
                        
                        let data_slice = &slice[slot_offset + 4..slot_offset + ITEM_SIZE];
                        let end = data_slice.iter().position(|&b| b == 0).unwrap_or(data_slice.len());
                        let item_data = String::from_utf8_lossy(&data_slice[..end]).to_string();
                        
                        (item_id, item_data)
                    };
                    
                    {
                        let mut slice = handle_clone.as_ref().clone().as_mut_slice()
                            .expect("Failed to get mutable slice");
                        
                        // Update tail and count atomically (simplified)
                        let new_tail = (tail + 1) % (NUM_ITEMS * 2); // Prevent overflow
                        let new_count = count - 1;
                        
                        let tail_bytes = (new_tail as u32).to_le_bytes();
                        let count_bytes = (new_count as u32).to_le_bytes();
                        slice[4..8].copy_from_slice(&tail_bytes);
                        slice[8..12].copy_from_slice(&count_bytes);
                    }
                    
                    handle_clone.sync(SyncType::Sync).expect("Failed to sync");
                    
                    // Record consumed item
                    {
                        let mut consumed = consumed_items.lock().unwrap();
                        consumed.push(item_id);
                    }
                    
                    consumed_count += 1;
                } else {
                    // Buffer is empty, wait a bit
                    thread::sleep(Duration::from_micros(100));
                }
            }
            
            completion_barrier.wait();
        });
        
        handles.push(consumer);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Verify all items were produced and consumed
    let produced = produced_items.lock().unwrap();
    let consumed = consumed_items.lock().unwrap();
    
    let total_expected = NUM_PRODUCERS * ITEMS_PER_PRODUCER;
    assert_eq!(produced.len(), total_expected);
    assert_eq!(consumed.len(), total_expected);
    
    // Verify no items were lost or duplicated
    let mut produced_sorted = produced.clone();
    let mut consumed_sorted = consumed.clone();
    produced_sorted.sort();
    consumed_sorted.sort();
    
    assert_eq!(produced_sorted, consumed_sorted);
    
    println!("Ring buffer test: {} producers, {} consumers, {} items total", 
             NUM_PRODUCERS, NUM_CONSUMERS, total_expected);
    println!("All items successfully passed through ring buffer");
}

/// Test memory-mapped file based IPC with persistence
#[test]
fn test_file_based_ipc() {
    const MESSAGE_COUNT: usize = 20;
    const MESSAGE_SIZE: usize = 128;
    const FILE_SIZE: usize = MESSAGE_COUNT * MESSAGE_SIZE + 1024; // Extra space for metadata
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let ipc_file_path = temp_dir.path().join("ipc_messages.dat");
    
    // Initialize IPC file
    {
        let mut file = File::create(&ipc_file_path).expect("Failed to create IPC file");
        let initial_data = vec![0u8; FILE_SIZE];
        file.write_all(&initial_data).expect("Failed to write initial data");
        file.flush().expect("Failed to flush IPC file");
    }
    
    let completion_barrier = Arc::new(Barrier::new(3)); // Writer, reader, and main thread
    let messages_written = Arc::new(Mutex::new(Vec::new()));
    let messages_read = Arc::new(Mutex::new(Vec::new()));
    
    // Writer thread
    let writer_file_path = ipc_file_path.clone();
    let writer_barrier = Arc::clone(&completion_barrier);
    let written_messages = Arc::clone(&messages_written);
    
    let writer = thread::spawn(move || {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&writer_file_path)
            .expect("Failed to open IPC file for writing");
        
        let config = MmapConfig {
            protection: ProtectionFlags::ReadWrite,
            mapping_type: MappingType::Shared,
            length: FILE_SIZE,
            offset: 0,
            ..Default::default()
        };
        
        let handle = map_file(&file, config).expect("Failed to map IPC file");
        
        for i in 0..MESSAGE_COUNT {
            let message = format!("IPC Message #{:03} - {}", i, "X".repeat(80));
            let message_bytes = message.as_bytes();
            let offset = 4 + i * MESSAGE_SIZE; // Skip 4-byte counter at start
            
            {
                let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
                
                // Write message length and content
                let len_bytes = (message_bytes.len() as u32).to_le_bytes();
                slice[offset..offset + 4].copy_from_slice(&len_bytes);
                slice[offset + 4..offset + 4 + message_bytes.len()].copy_from_slice(message_bytes);
                
                // Update message counter
                let count_bytes = ((i + 1) as u32).to_le_bytes();
                slice[0..4].copy_from_slice(&count_bytes);
            }
            
            handle.sync(SyncType::Sync).expect("Failed to sync");
            
            {
                let mut written = written_messages.lock().unwrap();
                written.push(message);
            }
            
            thread::sleep(Duration::from_millis(10)); // Simulate processing time
        }
        
        writer_barrier.wait();
    });
    
    // Reader thread
    let reader_file_path = ipc_file_path.clone();
    let reader_barrier = Arc::clone(&completion_barrier);
    let read_messages = Arc::clone(&messages_read);
    
    let reader = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Give writer a head start
        
        let file = File::open(&reader_file_path).expect("Failed to open IPC file for reading");
        
        let config = MmapConfig {
            protection: ProtectionFlags::Read,
            mapping_type: MappingType::Shared,
            length: FILE_SIZE,
            offset: 0,
            ..Default::default()
        };
        
        let handle = map_file(&file, config).expect("Failed to map IPC file");
        
        let mut last_count = 0;
        let start_time = Instant::now();
        
        while last_count < MESSAGE_COUNT && start_time.elapsed() < Duration::from_secs(10) {
            let slice = handle.as_slice().expect("Failed to get slice");
            let count_bytes = [slice[0], slice[1], slice[2], slice[3]];
            let current_count = u32::from_le_bytes(count_bytes) as usize;
            
            if current_count > last_count {
                // Read new messages
                for i in last_count..current_count {
                    let offset = 4 + i * MESSAGE_SIZE;
                    
                    let len_bytes = [slice[offset], slice[offset + 1], slice[offset + 2], slice[offset + 3]];
                    let message_len = u32::from_le_bytes(len_bytes) as usize;
                    
                    let message_bytes = &slice[offset + 4..offset + 4 + message_len];
                    let message = String::from_utf8_lossy(message_bytes).to_string();
                    
                    {
                        let mut read = read_messages.lock().unwrap();
                        read.push(message);
                    }
                }
                
                last_count = current_count;
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        }
        
        reader_barrier.wait();
    });
    
    // Wait for both threads to complete
    writer.join().expect("Writer thread panicked");
    reader.join().expect("Reader thread panicked");
    completion_barrier.wait(); // Wait for main thread
    
    // Verify all messages were written and read correctly
    let written = messages_written.lock().unwrap();
    let read = messages_read.lock().unwrap();
    
    assert_eq!(written.len(), MESSAGE_COUNT);
    assert_eq!(read.len(), MESSAGE_COUNT);
    assert_eq!(*written, *read);
    
    // Verify file persistence by reading the file again
    {
        let file = File::open(&ipc_file_path).expect("Failed to reopen IPC file");
        let config = MmapConfig {
            protection: ProtectionFlags::Read,
            mapping_type: MappingType::Private,
            length: FILE_SIZE,
            offset: 0,
            ..Default::default()
        };
        
        let handle = map_file(&file, config).expect("Failed to remap IPC file");
        let slice = handle.as_slice().expect("Failed to get slice");
        
        let count_bytes = [slice[0], slice[1], slice[2], slice[3]];
        let final_count = u32::from_le_bytes(count_bytes) as usize;
        assert_eq!(final_count, MESSAGE_COUNT);
        
        // Verify a few messages
        for i in 0..3 {
            let offset = 4 + i * MESSAGE_SIZE;
            let len_bytes = [slice[offset], slice[offset + 1], slice[offset + 2], slice[offset + 3]];
            let message_len = u32::from_le_bytes(len_bytes) as usize;
            let message_bytes = &slice[offset + 4..offset + 4 + message_len];
            let message = String::from_utf8_lossy(message_bytes).to_string();
            
            assert!(message.starts_with(&format!("IPC Message #{:03}", i)));
        }
    }
    
    println!("File-based IPC test completed: {} messages persisted successfully", MESSAGE_COUNT);
}

/// Test high-performance shared memory data structures
#[test]
fn test_shared_memory_data_structures() {
    const SHARED_SIZE: usize = 64 * 1024; // 64KB shared memory
    const NUM_WRITERS: usize = 4;
    const NUM_READERS: usize = 2;
    const OPERATIONS_PER_WRITER: usize = 1000;
    
    // Create shared memory region
    let shared_region = create_shared_memory("data_structures_test", SHARED_SIZE)
        .expect("Failed to create shared memory region");
    let handle = shared_region.handle().clone();
    
    // Initialize shared data structures
    {
        let mut slice = handle.as_ref().clone().as_mut_slice().expect("Failed to get mutable slice");
        slice.fill(0);
        
        // Layout:
        // [0..8]: write counter (u64)
        // [8..16]: read counter (u64)
        // [16..24]: checksum (u64)
        // [24..]: circular buffer for data
    }
    
    let start_barrier = Arc::new(Barrier::new(NUM_WRITERS + NUM_READERS));
    let completion_barrier = Arc::new(Barrier::new(NUM_WRITERS + NUM_READERS));
    let operation_results = Arc::new(Mutex::new(Vec::new()));
    
    let mut thread_handles = vec![];
    
    // Spawn writer threads
    for writer_id in 0..NUM_WRITERS {
        let handle_clone = handle.clone_handle();
        let start_barrier = Arc::clone(&start_barrier);
        let completion_barrier = Arc::clone(&completion_barrier);
        let results = Arc::clone(&operation_results);
        
        let writer = thread::spawn(move || {
            start_barrier.wait();
            let start_time = Instant::now();
            
            for op in 0..OPERATIONS_PER_WRITER {
                let data_value = (writer_id as u64) * 10000 + (op as u64);
                
                // Write operation
                {
                    let mut slice = handle_clone.as_ref().clone().as_mut_slice()
                        .expect("Failed to get mutable slice");
                    
                    // Read current write counter
                    let write_counter_bytes = [slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7]];
                    let write_counter = u64::from_le_bytes(write_counter_bytes);
                    
                    // Write data at circular buffer position
                    let buffer_offset = 24 + ((write_counter % ((SHARED_SIZE - 24) / 8)) * 8) as usize;
                    let data_bytes = data_value.to_le_bytes();
                    slice[buffer_offset..buffer_offset + 8].copy_from_slice(&data_bytes);
                    
                    // Update write counter
                    let new_counter = write_counter + 1;
                    let counter_bytes = new_counter.to_le_bytes();
                    slice[0..8].copy_from_slice(&counter_bytes);
                    
                    // Update checksum
                    let checksum_bytes = [slice[16], slice[17], slice[18], slice[19], slice[20], slice[21], slice[22], slice[23]];
                    let current_checksum = u64::from_le_bytes(checksum_bytes);
                    let new_checksum = current_checksum.wrapping_add(data_value);
                    let new_checksum_bytes = new_checksum.to_le_bytes();
                    slice[16..24].copy_from_slice(&new_checksum_bytes);
                }
                
                handle_clone.sync(SyncType::Async).expect("Failed to sync");
                
                if op % 100 == 0 {
                    thread::sleep(Duration::from_micros(10)); // Brief pause every 100 ops
                }
            }
            
            let elapsed = start_time.elapsed();
            {
                let mut results = operation_results.lock().unwrap();
                results.push(format!("Writer {}: {} ops in {:?}", writer_id, OPERATIONS_PER_WRITER, elapsed));
            }
            
            completion_barrier.wait();
        });
        
        thread_handles.push(writer);
    }
    
    // Spawn reader threads
    for reader_id in 0..NUM_READERS {
        let handle_clone = handle.clone_handle();
        let start_barrier = Arc::clone(&start_barrier);
        let completion_barrier = Arc::clone(&completion_barrier);
        let results = Arc::clone(&operation_results);
        
        let reader = thread::spawn(move || {
            start_barrier.wait();
            let start_time = Instant::now();
            
            let mut last_read_counter = 0u64;
            let mut reads_performed = 0;
            let target_reads = (NUM_WRITERS * OPERATIONS_PER_WRITER) / NUM_READERS;
            
            while reads_performed < target_reads && start_time.elapsed() < Duration::from_secs(30) {
                let (write_counter, read_counter) = {
                    let slice = handle_clone.as_slice().expect("Failed to get slice");
                    let write_bytes = [slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7]];
                    let read_bytes = [slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14], slice[15]];
                    (u64::from_le_bytes(write_bytes), u64::from_le_bytes(read_bytes))
                };
                
                if write_counter > read_counter {
                    // There's data to read
                    let items_to_read = std::cmp::min(write_counter - read_counter, 10); // Read up to 10 items at once
                    
                    for _ in 0..items_to_read {
                        let buffer_offset = 24 + ((read_counter % ((SHARED_SIZE - 24) / 8)) * 8) as usize;
                        
                        let _data_value = {
                            let slice = handle_clone.as_slice().expect("Failed to get slice");
                            let data_bytes = [slice[buffer_offset], slice[buffer_offset + 1], slice[buffer_offset + 2], slice[buffer_offset + 3],
                                            slice[buffer_offset + 4], slice[buffer_offset + 5], slice[buffer_offset + 6], slice[buffer_offset + 7]];
                            u64::from_le_bytes(data_bytes)
                        };
                        
                        // Update read counter
                        {
                            let mut slice = handle_clone.as_ref().clone().as_mut_slice()
                                .expect("Failed to get mutable slice");
                            let new_read_counter = read_counter + 1;
                            let counter_bytes = new_read_counter.to_le_bytes();
                            slice[8..16].copy_from_slice(&counter_bytes);
                        }
                        
                        reads_performed += 1;
                        last_read_counter = read_counter + 1;
                        
                        if reads_performed >= target_reads {
                            break;
                        }
                    }
                } else {
                    thread::sleep(Duration::from_micros(50));
                }
            }
            
            let elapsed = start_time.elapsed();
            {
                let mut results = operation_results.lock().unwrap();
                results.push(format!("Reader {}: {} reads in {:?}", reader_id, reads_performed, elapsed));
            }
            
            completion_barrier.wait();
        });
        
        thread_handles.push(reader);
    }
    
    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().expect("Thread panicked");
    }
    
    // Verify final state
    let (final_write_counter, final_read_counter, final_checksum) = {
        let slice = handle.as_slice().expect("Failed to get slice");
        let write_bytes = [slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7]];
        let read_bytes = [slice[8], slice[9], slice[10], slice[11], slice[12], slice[13], slice[14], slice[15]];
        let checksum_bytes = [slice[16], slice[17], slice[18], slice[19], slice[20], slice[21], slice[22], slice[23]];
        (u64::from_le_bytes(write_bytes), u64::from_le_bytes(read_bytes), u64::from_le_bytes(checksum_bytes))
    };
    
    let expected_total_writes = (NUM_WRITERS * OPERATIONS_PER_WRITER) as u64;
    assert_eq!(final_write_counter, expected_total_writes);
    assert_eq!(final_read_counter, expected_total_writes);
    
    // Print results
    let results = operation_results.lock().unwrap();
    for result in results.iter() {
        println!("{}", result);
    }
    
    println!("Shared memory data structures test completed:");
    println!("  Total writes: {}", final_write_counter);
    println!("  Total reads: {}", final_read_counter);
    println!("  Final checksum: {}", final_checksum);
}
