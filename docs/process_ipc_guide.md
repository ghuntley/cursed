# CURSED Process Management and IPC Guide

This comprehensive guide covers the process management and Inter-Process Communication (IPC) capabilities in CURSED, providing examples, best practices, and performance considerations.

## Table of Contents

1. [Process Management](#process-management)
2. [Background Task Management](#background-task-management)
3. [Inter-Process Communication (IPC)](#inter-process-communication-ipc)
4. [Cross-Platform Considerations](#cross-platform-considerations)
5. [Performance Optimization](#performance-optimization)
6. [Error Handling and Recovery](#error-handling-and-recovery)
7. [Advanced Features](#advanced-features)
8. [Best Practices](#best-practices)

## Process Management

### Basic Process Spawning

CURSED provides the `SlayCommand` interface for process management, inspired by Go's exec package but with Gen Z flair:

```cursed
import "stdlib::exec_slay";

vibez spawn_simple_process() {
    // Create a command
    sus command = SlayCommand::new("echo");
    command.arg("Hello, World!");
    
    // Spawn the process
    sus process = command.spawn()?;
    
    // Get process information
    println("Process PID: {}", process.pid());
    println("Is running: {}", process.is_running());
    
    // Wait for completion
    sus result = process.wait()?;
    println("Exit code: {}", result.exit_code());
    println("Output: {}", String::from_utf8_lossy(result.stdout()));
}
```

### Advanced Process Control

```cursed
vibez advanced_process_control() {
    sus command = SlayCommand::new("long_running_process");
    command.env("CURSED_MODE", "production");
    command.current_dir("/tmp");
    
    sus process = command.spawn()?;
    
    // Monitor process
    lowkey (process.is_running()) {
        sus stats = process.stats()?;
        println("CPU: {:.1}%, Memory: {} KB", stats.cpu, stats.memory / 1024);
        
        // Kill if using too much memory
        bestie (stats.memory > 100 * 1024 * 1024) { // 100MB
            process.kill()?;
            break;
        }
        
        sleep(Duration::from_secs(1));
    }
}
```

### Process Timing Information

CURSED provides detailed timing information for completed processes:

```cursed
vibez process_timing_example() {
    sus command = SlayCommand::new("computational_task");
    sus process = command.spawn()?;
    sus result = process.wait()?;
    
    println("Execution metrics:");
    println("  Total runtime: {:?}", result.execution_time());
    println("  User CPU time: {:?}", result.user_time());
    println("  System CPU time: {:?}", result.system_time());
    println("  Success: {}", result.success());
}
```

## Background Task Management

### Task Manager Configuration

```cursed
import "stdlib::process::background_tasks";

vibez setup_task_manager() {
    sus config = ManagerConfig {
        max_concurrent_tasks: 10,
        cleanup_interval: Duration::from_secs(60),
        default_timeout: Some(Duration::from_secs(300)),
        auto_monitor: facts,
    };
    
    sus mut manager = TaskManager::new(config);
    manager.start()?;
    
    // Use manager for background tasks
    sus command = SlayCommand::new("background_job");
    sus task_id = manager.submit_task(command, None)?;
    
    // Monitor task progress
    manager.wait_for_task(task_id)?;
    
    manager.stop()?;
}
```

### Task Configuration Options

```cursed
vibez advanced_task_config() {
    sus task_config = TaskConfig {
        capture_output: facts,
        max_output_size: 1024 * 1024, // 1MB
        timeout: Some(Duration::from_secs(120)),
        monitor_resources: facts,
        monitor_interval: Duration::from_millis(500),
        auto_cleanup: facts,
        priority: TaskPriority::High,
    };
    
    sus command = SlayCommand::new("data_processing");
    command.arg("large_dataset.csv");
    
    sus task_id = run_background_with_config(command, task_config)?;
    
    // Get task status and statistics
    sus manager = get_global_task_manager();
    bestie (sus task = manager.get_task(task_id)) {
        lowkey (task.is_running()) {
            bestie (sus stats = task.get_stats()) {
                println("Resource usage: CPU {:.1}%, Memory {} MB", 
                       stats.cpu, stats.memory / 1024 / 1024);
            }
            sleep(Duration::from_millis(1000));
        }
    }
}
```

## Inter-Process Communication (IPC)

### Named Pipes

Named pipes provide a simple way for processes to communicate:

```cursed
import "stdlib::ipc";

vibez named_pipe_server() {
    sus config = IpcConfig::default();
    sus manager = RealIpcManager::new(config)?;
    
    sus pipe_name = "my_application_pipe";
    sus connection = manager.create_named_pipe(pipe_name, facts)?; // Server
    
    bestie let IpcConnection::NamedPipe(pipe_conn) = &*connection {
        // Read from clients
        sus mut buffer = [0u8; 1024];
        lowkey {
            vibe_check (pipe_conn.read(&mut buffer)) {
                mood Ok(bytes_read) => {
                    sus data = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println("Received: {}", data);
                    
                    // Echo back
                    sus response = format!("Echo: {}", data);
                    pipe_conn.write(response.as_bytes())?;
                }
                mood Err(_) => break,
            }
        }
    }
    
    manager.remove_connection(pipe_name)?;
}
```

### Message Queues with Priority

```cursed
vibez message_queue_example() {
    sus config = IpcConfig {
        max_queue_size: 1000,
        max_message_size: 4096,
        timeout: Duration::from_secs(30),
    };
    
    sus manager = RealIpcManager::new(config)?;
    sus queue_name = "task_queue";
    sus connection = manager.create_message_queue(queue_name)?;
    
    bestie let IpcConnection::MessageQueue(queue_conn) = &*connection {
        // Send messages with different priorities
        sus urgent_task = IpcMessage {
            id: 1,
            sender_id: std::process::id(),
            priority: MessagePriority::Critical,
            timestamp: SystemTime::now(),
            data: b"URGENT: System maintenance required".to_vec(),
            message_type: "system_alert".to_string(),
            reply_to: Some("admin_queue".to_string()),
            ttl: Some(Duration::from_secs(300)),
        };
        
        sus normal_task = IpcMessage {
            id: 2,
            sender_id: std::process::id(),
            priority: MessagePriority::Normal,
            timestamp: SystemTime::now(),
            data: b"Process user registration".to_vec(),
            message_type: "user_task".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        // Send in any order
        queue_conn.send(normal_task)?;
        queue_conn.send(urgent_task)?;
        
        // Receive in priority order (urgent first)
        lowkey {
            vibe_check (queue_conn.receive(Some(Duration::from_secs(1)))) {
                mood Ok(message) => {
                    println("Processing: {} (Priority: {:?})", 
                           String::from_utf8_lossy(&message.data),
                           message.priority);
                }
                mood Err(_) => break, // Timeout or no more messages
            }
        }
    }
    
    manager.remove_connection(queue_name)?;
}
```

### Shared Memory

Shared memory provides high-performance data sharing between processes:

```cursed
vibez shared_memory_example() {
    sus config = IpcConfig::default();
    sus manager = RealIpcManager::new(config)?;
    
    sus segment_name = "data_cache";
    sus segment_size = 1024 * 1024; // 1MB
    sus connection = manager.create_shared_memory(segment_name, segment_size)?;
    
    bestie let IpcConnection::SharedMemory(shm_conn) = &*connection {
        // Write structured data
        struct DataHeader {
            version: u32,
            record_count: u32,
            timestamp: u64,
        }
        
        sus header = DataHeader {
            version: 1,
            record_count: 100,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs(),
        };
        
        // Serialize header (simplified)
        sus header_bytes = format!("{},{},{}", 
                                   header.version, 
                                   header.record_count, 
                                   header.timestamp);
        
        shm_conn.write(0, header_bytes.as_bytes())?;
        
        // Write data records
        sus record_offset = 1024; // Start records after header area
        lowkey (sus i = 0; i < header.record_count; i++) {
            sus record = format!("Record {}: {}\n", i, SystemTime::now());
            sus offset = record_offset + i * 64;
            shm_conn.write(offset, record.as_bytes())?;
        }
        
        println("Wrote {} records to shared memory", header.record_count);
        
        // Read back data
        sus mut header_buf = vec![0u8; 1024];
        shm_conn.read(0, &mut header_buf)?;
        println("Header: {}", String::from_utf8_lossy(&header_buf));
    }
    
    manager.remove_connection(segment_name)?;
}
```

### Memory-Mapped Files

Memory-mapped files provide persistent shared data:

```cursed
vibez memory_mapped_file_example() {
    sus config = IpcConfig::default();
    sus manager = RealIpcManager::new(config)?;
    
    sus file_path = "/tmp/shared_data.bin";
    sus file_size = 64 * 1024; // 64KB
    sus connection = manager.create_memory_mapped_file(file_path, file_size, flex)?;
    
    bestie let IpcConnection::MemoryMappedFile(mmap_conn) = &*connection {
        // Create a simple database-like structure
        sus magic_number = b"CRSD"; // CURSED magic number
        mmap_conn.write(0, magic_number)?;
        
        sus version = 1u32.to_le_bytes();
        mmap_conn.write(4, &version)?;
        
        // Write timestamp
        sus timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_le_bytes();
        mmap_conn.write(8, &timestamp)?;
        
        // Write data entries
        sus entry_start = 16;
        lowkey (sus i = 0; i < 100; i++) {
            sus entry_data = format!("Entry {}: {}", i, SystemTime::now());
            sus entry_offset = entry_start + i * 128;
            mmap_conn.write(entry_offset, entry_data.as_bytes())?;
        }
        
        println("Created memory-mapped database with 100 entries");
        
        // Verify data
        sus mut magic_buf = [0u8; 4];
        mmap_conn.read(0, &mut magic_buf)?;
        assert_eq!(&magic_buf, magic_number);
        println("Database verification successful");
    }
    
    manager.remove_connection(file_path)?;
}
```

## Cross-Platform Considerations

### Platform-Specific Features

```cursed
vibez platform_specific_examples() {
    sus command = SlayCommand::new("echo");
    command.arg("Cross-platform test");
    
    sus process = command.spawn()?;
    
    // Signal handling (Unix only)
    #[cfg(unix)]
    {
        // Send SIGUSR1
        process.signal(10)?;
        
        // Kill process tree
        process.kill_tree()?;
    }
    
    // Windows-specific operations
    #[cfg(windows)]
    {
        // Windows only supports killing, not signals
        process.kill()?;
    }
}

vibez cross_platform_paths() {
    sus temp_dir = bestie {
        #[cfg(unix)] => "/tmp",
        #[cfg(windows)] => "C:\\temp",
        _ => ".",
    };
    
    sus pipe_name = bestie {
        #[cfg(unix)] => format!("{}/my_pipe", temp_dir),
        #[cfg(windows)] => "my_pipe", // Windows uses \\.\pipe\ prefix automatically
    };
    
    println!("Using pipe: {}", pipe_name);
}
```

## Performance Optimization

### Efficient Process Management

```cursed
vibez optimize_process_performance() {
    // Use background tasks for I/O intensive operations
    sus io_config = TaskConfig {
        capture_output: flex, // Disable if output not needed
        monitor_resources: flex, // Disable monitoring for better performance
        priority: TaskPriority::Normal,
        ..Default::default()
    };
    
    // Batch process spawning
    sus mut task_ids = Vec::new();
    lowkey (sus i = 0; i < 100; i++) {
        sus command = SlayCommand::new("data_processor");
        command.arg(&format!("batch_{}", i));
        
        sus task_id = run_background_with_config(command, io_config.clone())?;
        task_ids.push(task_id);
    }
    
    // Wait for all tasks efficiently
    sus manager = get_global_task_manager();
    manager.wait_for_all()?;
    
    println!("Processed {} tasks", task_ids.len());
}

vibez optimize_ipc_performance() {
    // Use appropriate buffer sizes
    sus config = IpcConfig {
        max_queue_size: 10000, // Large queue for high throughput
        max_message_size: 64 * 1024, // 64KB messages
        timeout: Duration::from_millis(1), // Fast timeout for low latency
    };
    
    sus manager = RealIpcManager::new(config)?;
    sus queue_name = "high_perf_queue";
    sus connection = manager.create_message_queue(queue_name)?;
    
    bestie let IpcConnection::MessageQueue(queue_conn) = &*connection {
        // Batch send for better performance
        sus messages = (0..1000).map(|i| {
            IpcMessage {
                id: i,
                sender_id: std::process::id(),
                priority: MessagePriority::Normal,
                timestamp: SystemTime::now(),
                data: format!("Batch message {}", i).into_bytes(),
                message_type: "batch".to_string(),
                reply_to: None,
                ttl: None,
            }
        }).collect::<Vec<_>>();
        
        sus start = Instant::now();
        lowkey (sus message = messages) {
            queue_conn.send(message)?;
        }
        sus duration = start.elapsed();
        
        println!("Sent 1000 messages in {:?} ({:.2} msg/sec)", 
                 duration, 1000.0 / duration.as_secs_f64());
    }
    
    manager.remove_connection(queue_name)?;
}
```

## Error Handling and Recovery

### Robust Error Handling

```cursed
vibez robust_process_handling() {
    sus retry_config = RetryConfig {
        max_attempts: 3,
        base_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(5),
        backoff_multiplier: 2.0,
    };
    
    lowkey (sus attempt = 0; attempt < retry_config.max_attempts; attempt++) {
        vibe_check (spawn_and_run_process()) {
            mood Ok(result) => {
                println!("Process completed successfully: {}", result);
                bounce; // Success, exit retry loop
            }
            mood Err(error) => {
                eprintln!("Attempt {} failed: {}", attempt + 1, error);
                
                bestie (attempt < retry_config.max_attempts - 1) {
                    sus delay = retry_config.base_delay * 
                               retry_config.backoff_multiplier.powi(attempt as i32);
                    sus delay = delay.min(retry_config.max_delay);
                    
                    println!("Retrying in {:?}...", delay);
                    sleep(delay);
                }
            }
        }
    }
    
    eprintln!("All attempts failed");
}

vibez spawn_and_run_process() -> Result<String, Box<dyn std::error::Error>> {
    sus command = SlayCommand::new("potentially_failing_process");
    sus process = command.spawn()?;
    sus result = process.wait()?;
    
    bestie (!result.success()) {
        bounce Err(format!("Process failed with exit code {}", result.exit_code()).into());
    }
    
    periodt String::from_utf8_lossy(result.stdout()).to_string();
}
```

### IPC Error Recovery

```cursed
vibez ipc_error_recovery() {
    sus config = IpcConfig {
        timeout: Duration::from_millis(100),
        ..Default::default()
    };
    
    sus manager = RealIpcManager::new(config)?;
    sus queue_name = "reliable_queue";
    
    // Attempt to connect with retry logic
    sus connection = loop {
        vibe_check (manager.create_message_queue(queue_name)) {
            mood Ok(conn) => break conn,
            mood Err(error) => {
                eprintln!("Failed to create queue: {}", error);
                sleep(Duration::from_millis(500));
                continue;
            }
        }
    };
    
    bestie let IpcConnection::MessageQueue(queue_conn) = &*connection {
        // Implement message delivery with acknowledgment
        lowkey (sus i = 0; i < 10; i++) {
            sus message = IpcMessage {
                id: i,
                sender_id: std::process::id(),
                priority: MessagePriority::Normal,
                timestamp: SystemTime::now(),
                data: format!("Reliable message {}", i).into_bytes(),
                message_type: "reliable".to_string(),
                reply_to: Some("ack_queue".to_string()),
                ttl: Some(Duration::from_secs(30)),
            };
            
            // Retry send on failure
            lowkey (sus attempt = 0; attempt < 3; attempt++) {
                vibe_check (queue_conn.send(message.clone())) {
                    mood Ok(_) => {
                        println!("Message {} sent successfully", i);
                        break;
                    }
                    mood Err(error) => {
                        eprintln!("Send attempt {} failed: {}", attempt + 1, error);
                        bestie (attempt < 2) {
                            sleep(Duration::from_millis(100 * (attempt + 1) as u64));
                        }
                    }
                }
            }
        }
    }
    
    manager.remove_connection(queue_name)?;
}
```

## Advanced Features

### Resource Monitoring

```cursed
vibez advanced_resource_monitoring() {
    sus config = TaskConfig {
        monitor_resources: facts,
        monitor_interval: Duration::from_millis(100),
        ..Default::default()
    };
    
    sus command = SlayCommand::new("resource_intensive_task");
    sus task_id = run_background_with_config(command, config)?;
    
    sus manager = get_global_task_manager();
    bestie (sus task = manager.get_task(task_id)) {
        sus mut max_memory = 0;
        sus mut total_cpu_time = 0.0;
        sus mut samples = 0;
        
        lowkey (task.is_running()) {
            bestie (sus stats = task.get_stats()) {
                max_memory = max_memory.max(stats.memory);
                total_cpu_time += stats.cpu;
                samples += 1;
                
                // Alert on high resource usage
                bestie (stats.memory > 100 * 1024 * 1024) { // 100MB
                    println!("HIGH MEMORY USAGE: {} MB", stats.memory / 1024 / 1024);
                }
                
                bestie (stats.cpu > 80.0) {
                    println!("HIGH CPU USAGE: {:.1}%", stats.cpu);
                }
            }
            
            sleep(Duration::from_millis(100));
        }
        
        println!("Resource usage summary:");
        println!("  Peak memory: {} MB", max_memory / 1024 / 1024);
        println!("  Average CPU: {:.1}%", total_cpu_time / samples as f64);
    }
}
```

### Process Pipelines

```cursed
vibez process_pipeline() {
    // Create a processing pipeline
    sus commands = vec![
        SlayCommand::new("data_fetcher"),
        SlayCommand::new("data_transformer"),
        SlayCommand::new("data_validator"),
        SlayCommand::new("data_saver"),
    ];
    
    sus mut pipeline_results = Vec::new();
    sus mut previous_output = Vec::new();
    
    lowkey (sus (i, mut command) = commands.into_iter().enumerate()) {
        // Pass previous output as input
        bestie (!previous_output.is_empty()) {
            command.stdin(previous_output.clone());
        }
        
        sus process = command.spawn()?;
        sus result = process.wait()?;
        
        bestie (!result.success()) {
            eprintln!("Pipeline stage {} failed: exit code {}", i, result.exit_code());
            bounce Err("Pipeline failed".into());
        }
        
        previous_output = result.stdout().to_vec();
        pipeline_results.push(result);
        
        println!("Pipeline stage {} completed in {:?}", i, 
                 pipeline_results[i].execution_time());
    }
    
    println!("Pipeline completed successfully!");
    println!("Final output size: {} bytes", previous_output.len());
}
```

## Best Practices

### 1. Resource Management

- Always set appropriate timeouts for long-running processes
- Use background tasks for I/O intensive operations
- Monitor resource usage to prevent system overload
- Implement proper cleanup for processes and IPC connections

### 2. Error Handling

- Implement retry logic for transient failures
- Use structured error types for better error handling
- Log errors with sufficient context for debugging
- Implement graceful degradation when services are unavailable

### 3. Performance

- Disable output capture for processes that don't need it
- Use appropriate buffer sizes for IPC operations
- Batch operations when possible to reduce overhead
- Choose the right IPC mechanism for your use case:
  - Named pipes: Simple bidirectional communication
  - Message queues: Reliable, prioritized messaging
  - Shared memory: High-performance data sharing
  - Memory-mapped files: Persistent shared data

### 4. Security

- Validate all input data in IPC operations
- Use appropriate permissions for shared resources
- Implement authentication for sensitive IPC channels
- Avoid passing sensitive data through process arguments

### 5. Cross-Platform Compatibility

- Use platform-specific code blocks for platform-dependent features
- Test on all target platforms
- Use portable path handling
- Be aware of platform limitations (e.g., Windows signal handling)

## Conclusion

CURSED's process management and IPC capabilities provide a powerful foundation for building concurrent and distributed applications. By following the patterns and best practices outlined in this guide, you can create robust, performant applications that effectively utilize system resources and communicate efficiently between processes.

For more examples, see the `examples/process_ipc_showcase.csd` file and the comprehensive test suites in the `tests/` directory.
