# IPC (Inter-Process Communication) Module

Pure CURSED implementation of inter-process communication without FFI dependencies. Provides native messaging, coordination, and communication patterns using CURSED's language features.

## Overview

The IPC module provides comprehensive inter-process communication mechanisms including:

- **Channels**: Message passing between processes
- **Process Coordination**: Process registration and signaling
- **Shared Memory**: Memory sharing simulation
- **Message Queues**: FIFO message queuing
- **Semaphores**: Synchronization primitives
- **Named Pipes**: File-based communication
- **Utilities**: System monitoring and cleanup

## Core Features

### 🔄 IPC Channels
- Create communication channels between processes
- Send/receive messages with priority and timestamps
- Channel lifecycle management (open/close)

### 👥 Process Coordination
- Process registration and status tracking
- Process signaling (terminate, suspend, resume)
- Process health monitoring

### 💾 Shared Memory
- Create and manage shared memory segments
- Read/write operations with bounds checking
- Permission-based access control

### 📬 Message Queues
- FIFO message queuing system
- Queue capacity management
- Message priority handling

### 🔒 Semaphores
- Synchronization primitive implementation
- Acquire/release operations
- Waiting process management

### 📁 Named Pipes
- File-based communication channels
- Read/write operations
- Permission management

## API Reference

### IPC Channels

```cursed
# Create a new IPC channel
sus channel := ipc_create_channel("my_channel", 1024)

# Send message through channel
sus success := ipc_send_message(channel, 1, "Hello", 1)

# Receive message from channel
sus message := ipc_receive_message(channel)

# Close channel
sus closed_channel := ipc_close_channel(channel)
```

### Process Coordination

```cursed
# Register a process
sus process := ipc_register_process(1234, "worker_process")

# Check if process is alive
sus alive := ipc_process_alive(process)

# Send signal to process
sus success := ipc_signal_process(process, "terminate")
```

### Shared Memory

```cursed
# Create shared memory segment
sus memory := ipc_create_shared_memory("shared_data", 4096)

# Write to shared memory
sus success := ipc_write_shared_memory(memory, 0, "data")

# Read from shared memory
sus data := ipc_read_shared_memory(memory, 0, 100)
```

### Message Queues

```cursed
# Create message queue
sus queue := ipc_create_message_queue("task_queue", 10)

# Create and push message
sus message := IpcMessage(1, "task_data", 1, 1640995200)
sus updated_queue := ipc_queue_push(queue, message)

# Pop message from queue
sus (popped_message, new_queue) := ipc_queue_pop(updated_queue)
```

### Semaphores

```cursed
# Create semaphore
sus semaphore := ipc_create_semaphore("resource_lock", 1)

# Acquire semaphore
sus acquired_sem := ipc_semaphore_acquire(semaphore)

# Release semaphore
sus released_sem := ipc_semaphore_release(acquired_sem)
```

### Named Pipes

```cursed
# Create named pipe
sus pipe := ipc_create_named_pipe("/tmp/my_pipe", 644)

# Write to pipe
sus success := ipc_pipe_write(pipe, "pipe_data")

# Read from pipe
sus data := ipc_pipe_read(pipe)
```

### Utilities

```cursed
# Get IPC system statistics
sus stats := ipc_get_stats()

# Perform health check
sus healthy := ipc_health_check()

# Cleanup all IPC resources
sus cleaned := ipc_cleanup()
```

## Data Structures

### IpcMessage
```cursed
IpcMessage(id normie, content tea, priority normie, timestamp thicc)
```

### IpcChannel
```cursed
IpcChannel(name tea, buffer_size normie, is_open lit)
```

### ProcessInfo
```cursed
ProcessInfo(pid normie, name tea, status tea, created_at thicc)
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/ipc/test_ipc.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/ipc/test_ipc.csd
./test_ipc
```

### Test Coverage

The test suite includes:
- ✅ IPC Channel operations (create, send, receive, close)
- ✅ Process coordination (register, signal, health check)
- ✅ Shared memory operations (create, read, write, bounds checking)
- ✅ Message queue operations (create, push, pop, capacity management)
- ✅ Semaphore operations (create, acquire, release, waiting)
- ✅ Named pipe operations (create, read, write, permissions)
- ✅ Complex IPC scenarios (multi-process coordination)
- ✅ Edge cases and error conditions

## Implementation Details

### Pure CURSED Design
- **No FFI Dependencies**: Implemented entirely in CURSED without external library calls
- **Memory Safe**: All operations use CURSED's built-in memory management
- **Type Safe**: Strong typing with tuple-based data structures
- **Cross-Platform**: Works identically in interpretation and compilation modes

### Performance Characteristics
- **Lightweight**: Minimal overhead with pure CURSED implementation
- **Scalable**: Efficient tuple-based data structures
- **Deterministic**: Predictable behavior across all platforms

### Security Features
- **Bounds Checking**: All memory operations include bounds validation
- **Permission Management**: Access control for shared resources
- **Process Isolation**: Secure process coordination mechanisms

## Usage Examples

### Simple Message Passing
```cursed
# Create communication channel
sus channel := ipc_create_channel("worker_comm", 2048)

# Process A sends message
sus success := ipc_send_message(channel, 1, "task_complete", 1)

# Process B receives message  
sus message := ipc_receive_message(channel)
sus (id, content, priority, timestamp) := message
vibez.spill("Received: " + content)
```

### Process Coordination
```cursed
# Register processes
sus worker1 := ipc_register_process(1001, "data_processor")
sus worker2 := ipc_register_process(1002, "file_handler")

# Coordinate processes
sus alive1 := ipc_process_alive(worker1)
stan alive1 {
    sus result := ipc_signal_process(worker1, "suspend")
}
```

### Shared Data Exchange
```cursed
# Setup shared memory
sus shared_mem := ipc_create_shared_memory("data_exchange", 8192)

# Process A writes data
sus write_ok := ipc_write_shared_memory(shared_mem, 0, "shared_data")

# Process B reads data
sus data := ipc_read_shared_memory(shared_mem, 0, 100)
```

## Production Deployment

### System Requirements
- CURSED runtime environment
- Sufficient memory for channel buffers and shared segments
- File system access for named pipes (when applicable)

### Configuration
- Adjust buffer sizes based on message volume
- Set appropriate permissions for shared resources
- Configure timeouts for blocking operations

### Monitoring
- Use `ipc_get_stats()` for system monitoring
- Implement `ipc_health_check()` in health monitoring systems
- Regular `ipc_cleanup()` for resource management

## Migration from FFI

This module replaces the previous FFI-based implementation:

### Replaced Components
- ❌ `libc::signal()` calls → ✅ Pure CURSED signaling
- ❌ POSIX shared memory → ✅ CURSED memory simulation
- ❌ Unix domain sockets → ✅ CURSED channel system
- ❌ System semaphores → ✅ CURSED synchronization primitives

### Benefits of Pure CURSED Implementation
- **Portability**: Works across all platforms without system dependencies
- **Simplicity**: Easier to understand and maintain
- **Security**: Reduced attack surface with no external dependencies
- **Consistency**: Identical behavior in interpretation and compilation modes

## Error Handling

The module uses CURSED's boolean return values and empty string/zero values to indicate errors:

- Channel operations return `cap` (false) on failure
- Memory operations return empty strings on bounds errors
- Process operations return `cap` on invalid states
- All functions include appropriate bounds and state checking

## Performance Considerations

- **Channel Buffer Sizes**: Larger buffers improve throughput but increase memory usage
- **Message Queues**: Configure appropriate queue sizes for your workload
- **Shared Memory**: Use appropriate segment sizes for your data
- **Cleanup**: Regular cleanup prevents resource leaks

## Contributing

When extending the IPC module:
1. Maintain pure CURSED implementation (no FFI)
2. Add comprehensive tests for new functionality
3. Update documentation with examples
4. Ensure both interpretation and compilation mode compatibility
5. Follow CURSED naming conventions and patterns

## Version History

- **v1.0.0**: Initial pure CURSED implementation
- Complete FFI elimination
- Comprehensive test suite
- Production-ready functionality
