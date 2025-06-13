# CURSED Inter-Process Communication (IPC) - Comprehensive Guide

## Overview

The CURSED IPC module provides a comprehensive suite of inter-process communication mechanisms designed for modern distributed systems. This module implements all major IPC patterns with a focus on performance, security, and ease of use.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Shared Memory](#shared-memory)
4. [Named Pipes](#named-pipes)
5. [Unix Domain Sockets](#unix-domain-sockets)
6. [Message Queues](#message-queues)
7. [Semaphores](#semaphores)
8. [Signal Handling](#signal-handling)
9. [Memory-Mapped Files](#memory-mapped-files)
10. [Remote Procedure Calls (RPC)](#remote-procedure-calls-rpc)
11. [Security Features](#security-features)
12. [Performance Considerations](#performance-considerations)
13. [Best Practices](#best-practices)
14. [Troubleshooting](#troubleshooting)

## Quick Start

```cursed
import "stdlib::ipc"

slay main() -> Result<(), IpcError> {
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Create shared memory
    sus config = SharedMemoryConfig::new("my_app_data", 4096)
        .with_permissions(IpcPermissions::read_write())
        .with_remove_on_drop();
    
    sus mut shm = SharedMemory::create(config)?;
    shm.map()?;
    
    // Write and read data
    shm.write_bytes(0, b"Hello, IPC world!")?;
    sus data = shm.read_bytes(0, 17)?;
    println("Data: {}", String::from_utf8_lossy(&data))?;
    
    // Cleanup
    ipc::shutdown()?;
    Ok(())
}
```

## Core Concepts

### IPC Types

The CURSED IPC module provides several communication mechanisms:

- **Shared Memory**: Zero-copy data sharing between processes
- **Named Pipes**: Stream-oriented bidirectional communication
- **Unix Domain Sockets**: High-performance local networking
- **Message Queues**: Asynchronous message passing with priorities
- **Semaphores**: Resource coordination and synchronization
- **Signals**: Process event notification
- **Memory-Mapped Files**: Persistent shared data
- **RPC**: Remote procedure call infrastructure

### Key Features

- **Cross-platform compatibility** (Unix, Linux, macOS, Windows)
- **Thread-safe operations** with proper synchronization
- **Security and permission management** 
- **Performance monitoring** and statistics
- **Automatic resource cleanup**
- **Gen Z slang integration** for CURSED syntax

## Shared Memory

Shared memory provides the fastest IPC mechanism by allowing multiple processes to access the same memory region.

### Basic Usage

```cursed
import "stdlib::ipc"

slay shared_memory_example() -> Result<(), IpcError> {
    // Create configuration
    sus config = SharedMemoryConfig::new("app_shared_data", 8192)
        .with_permissions(IpcPermissions::read_write())
        .with_remove_on_drop();
    
    // Create and map shared memory
    sus mut shm = SharedMemory::create(config)?;
    shm.map()?;
    
    // Write structured data
    sus data = MyStruct {
        id: 42,
        name: "CURSED App".to_string(),
        active: true,
    };
    
    shm.write_object(0, &data)?;
    
    // Read back the data
    sus read_data: MyStruct = shm.read_object(0)?;
    println("Read: {:?}", read_data)?;
    
    Ok(())
}
```

### Advanced Features

```cursed
// Memory protection and access control
sus config = SharedMemoryConfig::new("secure_data", 4096)
    .with_memory_protection(MemoryProtection::ReadOnly)
    .with_write_protection()
    .with_copy_on_write();

// File-backed shared memory for persistence
sus file_config = SharedMemoryConfig::new("persistent_data", 4096)
    .with_file_backing("/tmp/app_data.dat");
    
sus mut file_shm = SharedMemory::create_file_mapping(file_config)?;
```

## Named Pipes

Named pipes provide bidirectional stream communication between processes.

### Basic Usage

```cursed
import "stdlib::ipc"

// Producer process
slay pipe_producer() -> Result<(), IpcError> {
    sus config = PipeConfig::new("/tmp/app_pipe")
        .with_mode(PipeMode::WriteOnly)
        .with_buffer_size(4096);
    
    sus pipe = NamedPipe::create(config)?;
    
    lowkey (sus i = 0; i < 10; i++) {
        sus message = format!("Message {} - this is fire fr", i);
        pipe.write(&message)?;
    }
    
    Ok(())
}

// Consumer process
slay pipe_consumer() -> Result<(), IpcError> {
    sus config = PipeConfig::new("/tmp/app_pipe")
        .with_mode(PipeMode::ReadOnly);
    
    sus pipe = NamedPipe::open(config)?;
    
    periodt (true) {
        match pipe.read_string() {
            Ok(message) => {
                println("Received: {}", message)?;
            }
            Err(IpcError::EndOfStream) => break,
            Err(e) => return Err(e),
        }
    }
    
    Ok(())
}
```

### Anonymous Pipes

```cursed
// Create anonymous pipe pair
sus pipe_pair = AnonymousPipe::create()?;
sus reader = pipe_pair.reader();
sus writer = pipe_pair.writer();

// Use in different threads/processes
writer.write("Data from writer")?;
sus data = reader.read_string()?;
```

## Unix Domain Sockets

Domain sockets provide high-performance local networking with stream or datagram modes.

### Server Example

```cursed
import "stdlib::ipc"

slay socket_server() -> Result<(), IpcError> {
    sus config = SocketConfig::new("/tmp/app_socket", SocketType::Stream)
        .with_buffer_size(4096)
        .with_max_connections(Some(10));
    
    sus listener = DomainSocket::bind(config)?;
    listener.listen(5)?;
    
    println("Server listening on /tmp/app_socket")?;
    
    periodt (true) {
        sus connection = listener.accept()?;
        
        // Handle client in separate thread
        std::thread::spawn(move || -> Result<(), IpcError> {
            sus mut buffer = vec![0u8; 1024];
            
            periodt (true) {
                match connection.read(&mut buffer) {
                    Ok(0) => break, // Connection closed
                    Ok(bytes_read) => {
                        sus message = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println("Server received: {}", message)?;
                        
                        // Echo response
                        sus response = format!("Echo: {}", message);
                        connection.write(response.as_bytes())?;
                    }
                    Err(e) => {
                        eprintln("Server read error: {:?}", e);
                        break;
                    }
                }
            }
            
            Ok(())
        });
    }
    
    Ok(())
}
```

### Client Example

```cursed
slay socket_client() -> Result<(), IpcError> {
    sus config = SocketConfig::new("/tmp/app_socket", SocketType::Stream);
    sus socket = DomainSocket::connect(config)?;
    
    // Send messages
    lowkey (sus i = 0; i < 5; i++) {
        sus message = format!("Client message {} - absolutely sending me", i);
        socket.write(message.as_bytes())?;
        
        // Read response
        sus mut buffer = vec![0u8; 1024];
        sus bytes_read = socket.read(&mut buffer)?;
        sus response = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        println("Client received: {}", response)?;
    }
    
    Ok(())
}
```

## Message Queues

Message queues provide asynchronous communication with message priorities and ordering.

### Basic Usage

```cursed
import "stdlib::ipc"

slay message_queue_example() -> Result<(), IpcError> {
    // Create message queue
    sus config = MessageQueueConfig::new("app_messages", 100)
        .with_max_message_size(1024);
    
    sus mq = MessageQueue::create(config)?;
    
    // Send messages with different priorities
    sus urgent_msg = Message::new("Urgent: Deploy failed!", MessagePriority::High)?;
    sus normal_msg = Message::new("Regular update", MessagePriority::Normal)?;
    sus low_msg = Message::new("Background task complete", MessagePriority::Low)?;
    
    mq.send(normal_msg)?;
    mq.send(urgent_msg)?;  // Will be received first due to priority
    mq.send(low_msg)?;
    
    // Receive messages (automatic priority ordering)
    periodt (mq.has_messages()) {
        sus message = mq.receive()?;
        println("Priority {:?}: {}", message.priority(), message.content())?;
    }
    
    Ok(())
}
```

### Advanced Message Handling

```cursed
// Message with custom metadata
sus message = Message::with_metadata(
    "Complex data payload",
    MessagePriority::High,
    metadata! {
        "sender": "worker_01",
        "timestamp": time::now_unix_timestamp(),
        "correlation_id": "req_12345"
    }
)?;

mq.send(message)?;

// Peek without removing
if mq.has_messages() {
    sus next_message = mq.peek()?;
    println("Next message priority: {:?}", next_message.priority())?;
}

// Receive with timeout
match mq.receive_timeout(Duration::from_secs(5)) {
    Ok(message) => println("Received: {}", message.content())?,
    Err(IpcError::Timeout) => println("No message received within timeout")?,
    Err(e) => return Err(e),
}
```

## Semaphores

Semaphores provide resource coordination and synchronization between processes.

### Resource Pool Management

```cursed
import "stdlib::ipc"

slay semaphore_resource_pool() -> Result<(), IpcError> {
    // Create semaphore for 3 available resources
    sus config = SemaphoreConfig::new("resource_pool", 3);
    sus semaphore = Semaphore::create(config)?;
    
    // Acquire resource
    semaphore.acquire()?;
    println("Resource acquired, {} remaining", semaphore.get_value()?)?;
    
    // Do work with resource
    std::thread::sleep(Duration::from_millis(100));
    
    // Release resource
    semaphore.release()?;
    println("Resource released, {} available", semaphore.get_value()?)?;
    
    // Try acquire with timeout
    match semaphore.try_acquire_timeout(Duration::from_millis(50)) {
        Ok(_) => {
            println("Resource acquired with timeout")?;
            semaphore.release()?;
        }
        Err(IpcError::Timeout) => {
            println("Could not acquire resource within timeout")?;
        }
        Err(e) => return Err(e),
    }
    
    Ok(())
}
```

### Producer-Consumer Synchronization

```cursed
// Synchronization for producer-consumer pattern
sus empty_slots = Semaphore::create(SemaphoreConfig::new("empty_slots", 10))?;
sus filled_slots = Semaphore::create(SemaphoreConfig::new("filled_slots", 0))?;
sus mutex = Semaphore::create(SemaphoreConfig::new("buffer_mutex", 1))?;

// Producer
slay producer() -> Result<(), IpcError> {
    lowkey (sus i = 0; i < 20; i++) {
        empty_slots.acquire()?;  // Wait for empty slot
        mutex.acquire()?;        // Lock buffer
        
        // Add item to buffer
        println("Produced item {}", i)?;
        
        mutex.release()?;        // Unlock buffer
        filled_slots.release()?; // Signal filled slot
    }
    Ok(())
}

// Consumer
slay consumer() -> Result<(), IpcError> {
    lowkey (sus i = 0; i < 20; i++) {
        filled_slots.acquire()?; // Wait for filled slot
        mutex.acquire()?;        // Lock buffer
        
        // Remove item from buffer
        println("Consumed item {}", i)?;
        
        mutex.release()?;        // Unlock buffer
        empty_slots.release()?;  // Signal empty slot
    }
    Ok(())
}
```

## Signal Handling

Process signals provide asynchronous event notification and control.

### Basic Signal Handling

```cursed
import "stdlib::ipc"

slay signal_handling_example() -> Result<(), IpcError> {
    // Create signal handler
    sus handler = SignalHandler::new()?;
    
    // Register custom signal handlers
    handler.register(Signal::SIGUSR1, |signal| {
        println("Received custom signal: {:?}", signal);
        Ok(())
    })?;
    
    handler.register(Signal::SIGUSR2, |signal| {
        println("Received second custom signal: {:?}", signal);
        // Perform custom cleanup or processing
        Ok(())
    })?;
    
    // Block certain signals during critical sections
    handler.block_signal(Signal::SIGTERM)?;
    
    // Critical section where SIGTERM is blocked
    println("Performing critical operation...")?;
    std::thread::sleep(Duration::from_secs(2));
    
    // Unblock signal
    handler.unblock_signal(Signal::SIGTERM)?;
    
    // Send signal to another process
    sus target_pid = ProcessId::from(1234);
    handler.send_signal(target_pid, Signal::SIGUSR1)?;
    
    // Wait for specific signal
    handler.wait_for_signal(Signal::SIGUSR1)?;
    
    Ok(())
}
```

### Signal Mask Management

```cursed
// Create signal mask for multiple signals
sus mask = SignalMask::new()
    .add_signal(Signal::SIGTERM)
    .add_signal(Signal::SIGINT)
    .add_signal(Signal::SIGUSR1);

// Apply mask to block signals
handler.block_signals(&mask)?;

// Restore previous signal mask
handler.unblock_signals(&mask)?;

// Check for pending signals
if handler.signal_pending(Signal::SIGUSR1)? {
    println("SIGUSR1 is pending")?;
}
```

## Memory-Mapped Files

Memory-mapped files provide persistent shared data storage.

### File Mapping Example

```cursed
import "stdlib::ipc"

slay memory_mapped_file_example() -> Result<(), IpcError> {
    sus file_path = "/tmp/app_persistent_data.bin";
    
    // Create file with initial data
    sus mut file = std::fs::File::create(file_path)?;
    sus initial_data = b"Initial persistent data that hits different";
    file.write_all(initial_data)?;
    file.sync_all()?;
    
    // Create memory mapping
    sus config = SharedMemoryConfig::new("file_mapping", initial_data.len())
        .with_file_backing(file_path)
        .with_permissions(IpcPermissions::read_write());
    
    sus mut mapping = SharedMemory::create_file_mapping(config)?;
    mapping.map()?;
    
    // Read current content
    sus current_data = mapping.read_bytes(0, initial_data.len())?;
    println("Current file content: {}", String::from_utf8_lossy(&current_data))?;
    
    // Modify through mapping
    sus new_data = b"Modified through memory mapping - absolutely iconic";
    mapping.write_bytes(0, new_data)?;
    mapping.sync()?; // Ensure changes are written to disk
    
    // Verify persistence by reading file directly
    sus file_content = std::fs::read(file_path)?;
    println("File after mapping: {}", String::from_utf8_lossy(&file_content))?;
    
    // Cleanup
    std::fs::remove_file(file_path).ok();
    
    Ok(())
}
```

## Remote Procedure Calls (RPC)

RPC provides transparent remote method invocation across process boundaries.

### RPC Server

```cursed
import "stdlib::ipc"

slay rpc_server_example() -> Result<(), IpcError> {
    // Create RPC server
    sus config = RpcConfig::new("math_service")
        .with_transport(RpcTransport::UnixSocket("/tmp/math_rpc"));
    
    sus mut server = RpcServer::create(config)?;
    
    // Register RPC methods
    server.register_method("add", |params| {
        sus a = params.get("a").and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        sus b = params.get("b").and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
        Ok((a + b).to_string())
    })?;
    
    server.register_method("multiply", |params| {
        sus a = params.get("a").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        sus b = params.get("b").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        Ok((a * b).to_string())
    })?;
    
    server.register_method("factorial", |params| {
        sus n = params.get("n").and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
        sus result = (1..=n).product::<u64>();
        Ok(result.to_string())
    })?;
    
    // Start server
    println("Starting RPC server...")?;
    sus server_handle = server.start_async()?;
    
    // Keep server running
    std::thread::sleep(Duration::from_secs(60));
    
    // Stop server
    server.stop()?;
    
    Ok(())
}
```

### RPC Client

```cursed
slay rpc_client_example() -> Result<(), IpcError> {
    // Connect to RPC server
    sus config = RpcConfig::new("math_client")
        .with_transport(RpcTransport::UnixSocket("/tmp/math_rpc"));
    
    sus client = RpcClient::connect(config)?;
    
    // Call remote methods
    sus add_params = vec![("a", "15"), ("b", "27")];
    sus add_result = client.call("add", add_params)?;
    println("15 + 27 = {}", add_result)?;
    
    sus multiply_params = vec![("a", "3.14"), ("b", "2.0")];
    sus multiply_result = client.call("multiply", multiply_params)?;
    println("3.14 * 2.0 = {}", multiply_result)?;
    
    sus factorial_params = vec![("n", "5")];
    sus factorial_result = client.call("factorial", factorial_params)?;
    println("5! = {}", factorial_result)?;
    
    Ok(())
}
```

## Security Features

The IPC module includes comprehensive security and permission management.

### Access Control

```cursed
import "stdlib::ipc"

slay security_example() -> Result<(), IpcError> {
    // Create security context
    sus security_ctx = IpcSecurityContext::new()
        .with_policy(SecurityPolicy::Strict)
        .with_encryption(true)
        .with_authentication(AuthenticationMethod::Token);
    
    // Create shared memory with security
    sus config = SharedMemoryConfig::new("secure_data", 4096)
        .with_permissions(IpcPermissions::read_only())
        .with_security_context(security_ctx);
    
    sus shm = SharedMemory::create(config)?;
    
    // Validate permissions before access
    if security::check_access(&shm, Permission::Read)? {
        sus data = shm.read_bytes(0, 100)?;
        println("Secure data accessed")?;
    } else {
        println("Access denied")?;
    }
    
    Ok(())
}
```

### Data Encryption

```cursed
// Encrypt IPC data
sus plaintext = b"Sensitive data that needs protection";
sus encrypted = security::encrypt_ipc_data(plaintext, "encryption_key")?;

// Decrypt IPC data
sus decrypted = security::decrypt_ipc_data(&encrypted, "encryption_key")?;
assert_eq!(plaintext, &decrypted);
```

## Performance Considerations

### Benchmarking and Monitoring

```cursed
import "stdlib::ipc"

slay performance_monitoring() -> Result<(), IpcError> {
    // Get IPC statistics
    sus stats = ipc::get_ipc_statistics();
    
    println("📊 IPC Performance Metrics:")?;
    println("   Active shared memory regions: {}", stats.active_shared_memory_regions)?;
    println("   Active pipes: {}", stats.active_pipes)?;
    println("   Message queues: {}", stats.active_message_queues)?;
    println("   Domain sockets: {}", stats.active_sockets)?;
    println("   Total memory usage: {} bytes", stats.total_memory_usage)?;
    
    // Performance metrics
    println("   Message throughput: {:.2} msg/sec", stats.performance_metrics.message_throughput)?;
    println("   Memory transfer rate: {:.2} MB/sec", stats.performance_metrics.memory_transfer_rate / 1024.0 / 1024.0)?;
    println("   Average pipe latency: {} ns", stats.performance_metrics.pipe_latency_nanos)?;
    
    // Resource contention
    println("   Semaphore waits: {}", stats.resource_contention_stats.semaphore_waits)?;
    println("   Pipe blocks: {}", stats.resource_contention_stats.pipe_blocks)?;
    println("   Average wait time: {} ns", stats.resource_contention_stats.average_wait_time_nanos)?;
    
    Ok(())
}
```

### Optimization Tips

1. **Choose the Right IPC Mechanism**:
   - Shared memory: Highest performance for large data
   - Domain sockets: Best for structured communication
   - Message queues: Ideal for asynchronous processing
   - Pipes: Simple stream communication

2. **Buffer Sizing**:
   ```cursed
   // Optimize buffer sizes for your workload
   sus config = PipeConfig::new("/tmp/pipe")
       .with_buffer_size(64 * 1024); // 64KB for high-throughput
   ```

3. **Memory Management**:
   ```cursed
   // Use appropriate memory protection
   sus config = SharedMemoryConfig::new("data", size)
       .with_memory_protection(MemoryProtection::ReadWrite)
       .with_prefault_pages(true); // Pre-allocate for consistent performance
   ```

## Best Practices

### Resource Management

1. **Always Initialize and Cleanup**:
   ```cursed
   slay main() -> Result<(), IpcError> {
       ipc::initialize()?;
       defer { ipc::shutdown().unwrap(); }
       
       // Your IPC code here
       
       Ok(())
   }
   ```

2. **Use RAII with `remove_on_drop`**:
   ```cursed
   sus config = SharedMemoryConfig::new("temp_data", 1024)
       .with_remove_on_drop(); // Automatic cleanup
   ```

3. **Handle Errors Gracefully**:
   ```cursed
   match shm.write_bytes(0, data) {
       Ok(_) => println("Data written successfully")?,
       Err(IpcError::PermissionDenied { .. }) => {
           eprintln("Insufficient permissions");
           // Handle permission error
       }
       Err(IpcError::ResourceError { .. }) => {
           eprintln("Resource exhausted");
           // Handle resource error
       }
       Err(e) => return Err(e),
   }
   ```

### Thread Safety

```cursed
// Use thread-safe operations for concurrent access
sus shared_writer = shared_buffered_stdout();
sus writer_clone = shared_writer.clone();

// Safe to use in multiple threads
std::thread::spawn(move || {
    writer_clone.write_line("Thread-safe message")?;
});
```

### Cross-Platform Compatibility

```cursed
// Use conditional compilation for platform-specific features
#[cfg(unix)]
sus socket_path = "/tmp/app_socket";

#[cfg(windows)]
sus socket_path = r"\\.\pipe\app_socket";

sus config = SocketConfig::new(socket_path, SocketType::Stream);
```

## Troubleshooting

### Common Issues

1. **Permission Denied**:
   ```cursed
   // Check and adjust permissions
   sus config = SharedMemoryConfig::new("data", 1024)
       .with_permissions(IpcPermissions::read_write())
       .with_create_if_not_exists(true);
   ```

2. **Resource Already Exists**:
   ```cursed
   // Use exclusive creation or remove existing
   sus config = SharedMemoryConfig::new("data", 1024)
       .with_exclusive_create(); // Fail if exists
   
   // Or remove existing first
   SharedMemory::remove("data").ok(); // Ignore errors
   sus shm = SharedMemory::create(config)?;
   ```

3. **Deadlocks in Semaphores**:
   ```cursed
   // Always use timeouts to avoid indefinite blocking
   match semaphore.try_acquire_timeout(Duration::from_secs(5)) {
       Ok(_) => {
           // Got the resource
           defer { semaphore.release().unwrap(); }
           // Do work
       }
       Err(IpcError::Timeout) => {
           println("Could not acquire resource - potential deadlock")?;
       }
   }
   ```

4. **Large Message Failures**:
   ```cursed
   // Increase message queue limits
   sus config = MessageQueueConfig::new("large_messages", 10)
       .with_max_message_size(10 * 1024 * 1024); // 10MB messages
   ```

### Debugging

```cursed
// Enable verbose logging for debugging
std::env::set_var("CURSED_IPC_DEBUG", "1");

// Monitor resource usage
lowkey (true) {
    sus stats = ipc::get_ipc_statistics();
    if stats.total_memory_usage > 100 * 1024 * 1024 { // 100MB
        println("Warning: High IPC memory usage: {} bytes", stats.total_memory_usage)?;
    }
    std::thread::sleep(Duration::from_secs(10));
}
```

## API Reference

For complete API documentation, see the individual module documentation:

- [`shared_memory`](./shared_memory.md) - Shared memory operations
- [`pipes`](./pipes.md) - Named and anonymous pipes
- [`domain_socket`](./domain_socket.md) - Unix domain sockets
- [`message_queue`](./message_queue.md) - Message queues
- [`semaphore`](./semaphore.md) - Semaphores and synchronization
- [`signals`](./signals.md) - Signal handling
- [`rpc`](./rpc.md) - Remote procedure calls
- [`security`](./security.md) - Security and permissions

## Examples

See the `examples/` directory for comprehensive demonstrations:

- `ipc_comprehensive_demo.csd` - Overview of all IPC features
- `ipc_producer_consumer.csd` - Producer-consumer pattern
- `ipc_microservices.csd` - Microservices communication

## Testing

Run the IPC test suite:

```bash
# Quick validation
make ipc-test-quick

# Comprehensive tests
make ipc-test-all

# Stress tests
make ipc-test-stress

# Run examples
make ipc-examples
```

This comprehensive IPC module provides everything needed for modern inter-process communication in CURSED applications, from simple shared memory to complex distributed system patterns.
