# IPC - Inter-Process Communication Module

Pure CURSED implementation of comprehensive inter-process communication mechanisms without FFI dependencies.

## Overview

The IPC module provides a complete suite of inter-process communication primitives including:

- **Named Pipes (FIFOs)** - Unidirectional data channels with buffering
- **Message Queues** - Priority-based message passing with overflow protection
- **Shared Memory** - Key-value shared memory segments with process attachment
- **Semaphores** - Counting semaphores with wait/signal operations
- **Unix Sockets** - Connection-oriented and connectionless communication
- **Process Management** - Process registration and coordination

## Features

### ✅ **Pure CURSED Implementation**
- No external FFI dependencies
- Native CURSED data structures and algorithms
- Integration with signal_boost module for cleanup

### ✅ **Enterprise-Grade Functionality**
- Comprehensive error handling and resource management
- Priority-based message queuing with insertion sort
- Configurable limits and buffer sizes
- Process coordination and lifecycle management

### ✅ **Production-Ready Design**
- Resource cleanup and leak prevention
- Signal handling for graceful shutdown
- Statistics and monitoring capabilities
- Debugging and diagnostic tools

## Quick Start

```cursed
yeet "ipc"

# Initialize IPC subsystem
ipc.init_ipc()

# Create and use a message queue
ipc.create_message_queue("my_queue", 100)
ipc.send_message("my_queue", "Hello, World!", ipc.MSG_PRIORITY_NORMAL)
sus message map = ipc.receive_message("my_queue")
vibez.spill("Received: " + message.get("content"))

# Cleanup
ipc.cleanup_ipc()
```

## API Reference

### Initialization

#### `init_ipc() -> lit`
Initialize the IPC subsystem with default configuration.

```cursed
ipc.init_ipc()
```

#### `configure_ipc(config map) -> lit`
Configure IPC settings.

```cursed
sus config map = {
    "max_message_size": 32768,
    "max_queue_size": 500,
    "timeout_ms": 3000
}
ipc.configure_ipc(config)
```

#### `cleanup_ipc() -> lit`
Clean up all IPC resources and shutdown the subsystem.

### Named Pipes

#### `create_named_pipe(name tea, buffer_size normie) -> lit`
Create a named pipe with specified buffer size.

```cursed
ipc.create_named_pipe("my_pipe", 1024)
```

#### `write_to_pipe(name tea, data tea) -> lit`
Write data to a named pipe.

```cursed
ipc.write_to_pipe("my_pipe", "Hello, Pipe!")
```

#### `read_from_pipe(name tea) -> tea`
Read data from a named pipe (FIFO order).

```cursed
sus data tea = ipc.read_from_pipe("my_pipe")
```

#### `open_pipe_reader(name tea, reader_id tea) -> lit`
Register a reader for a named pipe.

```cursed
ipc.open_pipe_reader("my_pipe", "reader_process_1")
```

#### `open_pipe_writer(name tea, writer_id tea) -> lit`
Register a writer for a named pipe.

```cursed
ipc.open_pipe_writer("my_pipe", "writer_process_1")
```

### Message Queues

#### `create_message_queue(name tea, max_size normie) -> lit`
Create a message queue with priority support.

```cursed
ipc.create_message_queue("task_queue", 100)
```

#### `send_message(queue_name tea, message tea, priority normie) -> lit`
Send a prioritized message to a queue.

```cursed
ipc.send_message("task_queue", "High priority task", ipc.MSG_PRIORITY_HIGH)
ipc.send_message("task_queue", "Normal task", ipc.MSG_PRIORITY_NORMAL)
ipc.send_message("task_queue", "Background task", ipc.MSG_PRIORITY_LOW)
```

#### `receive_message(queue_name tea) -> map`
Receive the highest priority message from a queue.

```cursed
sus message map = ipc.receive_message("task_queue")
sus content tea = message.get("content")
sus priority normie = message.get("priority")
sus timestamp normie = message.get("timestamp")
```

### Priority Constants

```cursed
ipc.MSG_PRIORITY_URGENT = 15    # Highest priority
ipc.MSG_PRIORITY_HIGH = 10      # High priority  
ipc.MSG_PRIORITY_NORMAL = 5     # Normal priority
ipc.MSG_PRIORITY_LOW = 1        # Lowest priority
```

### Shared Memory

#### `create_shared_memory(name tea, size normie) -> lit`
Create a shared memory segment.

```cursed
ipc.create_shared_memory("shared_data", 4096)
```

#### `attach_shared_memory(name tea, process_id tea) -> lit`
Attach a process to a shared memory segment.

```cursed
ipc.attach_shared_memory("shared_data", "worker_process_1")
```

#### `write_shared_memory(name tea, key tea, value tea) -> lit`
Write key-value data to shared memory.

```cursed
ipc.write_shared_memory("shared_data", "counter", "42")
ipc.write_shared_memory("shared_data", "status", "running")
```

#### `read_shared_memory(name tea, key tea) -> tea`
Read data from shared memory by key.

```cursed
sus counter tea = ipc.read_shared_memory("shared_data", "counter")
sus status tea = ipc.read_shared_memory("shared_data", "status")
```

### Semaphores

#### `create_semaphore(name tea, initial_value normie) -> lit`
Create a counting semaphore.

```cursed
ipc.create_semaphore("resource_pool", 5)  # Allow 5 concurrent accesses
```

#### `semaphore_wait(name tea, process_id tea) -> lit`
Wait on a semaphore (P operation). Returns `cap` if would block.

```cursed
if ipc.semaphore_wait("resource_pool", "worker_1") {
    # Got access to resource
    vibez.spill("Acquired resource")
} else {
    # Would block - handle appropriately
    vibez.spill("Resource not available")
}
```

#### `semaphore_signal(name tea) -> lit`
Signal a semaphore (V operation).

```cursed
ipc.semaphore_signal("resource_pool")  # Release resource
```

### Unix Sockets

#### `create_unix_socket(name tea, socket_type tea) -> lit`
Create a Unix domain socket.

```cursed
ipc.create_unix_socket("app_socket", "stream")
```

#### `listen_unix_socket(name tea, server_process tea) -> lit`
Listen for connections on a socket.

```cursed
ipc.listen_unix_socket("app_socket", "server_process")
```

#### `connect_unix_socket(name tea, client_process tea) -> lit`
Connect to a listening socket.

```cursed
ipc.connect_unix_socket("app_socket", "client_process_1")
```

### Process Management

#### `register_process(process_id tea, process_name tea) -> lit`
Register a process with the IPC subsystem.

```cursed
ipc.register_process("worker_1", "Background Worker Process")
```

#### `unregister_process(process_id tea) -> lit`
Unregister a process from the IPC subsystem.

```cursed
ipc.unregister_process("worker_1")
```

#### `get_process_info(process_id tea) -> map`
Get information about a registered process.

```cursed
sus process_info map = ipc.get_process_info("worker_1")
sus name tea = process_info.get("name")
sus active lit = process_info.get("active")
```

#### `list_active_processes() -> [tea]`
Get a list of all active process IDs.

```cursed
sus active_processes [tea] = ipc.list_active_processes()
```

### Utilities and Diagnostics

#### `get_ipc_statistics() -> map`
Get comprehensive IPC usage statistics.

```cursed
sus stats map = ipc.get_ipc_statistics()
vibez.spill("Pipes created: " + core.tea(stats.get("pipes_created")))
vibez.spill("Messages sent: " + core.tea(stats.get("messages_sent")))
```

#### `list_ipc_resources() -> [tea]`
Get a list of all IPC resource names.

```cursed
sus resources [tea] = ipc.list_ipc_resources()
```

#### `get_ipc_resource_info(resource_name tea) -> map`
Get detailed information about a specific IPC resource.

```cursed
sus pipe_info map = ipc.get_ipc_resource_info("pipe_my_pipe")
sus type tea = pipe_info.get("type")
sus created_at normie = pipe_info.get("created_at")
```

#### `dump_ipc_state()`
Print comprehensive IPC system state for debugging.

```cursed
ipc.dump_ipc_state()
```

#### `test_ipc_connectivity() -> lit`
Run connectivity tests for all IPC mechanisms.

```cursed
if ipc.test_ipc_connectivity() {
    vibez.spill("All IPC mechanisms working correctly")
}
```

#### `reset_ipc()`
Reset the entire IPC system (useful for testing).

```cursed
ipc.reset_ipc()
```

## Configuration Options

The IPC module supports the following configuration parameters:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_message_size` | 65536 | Maximum message size in bytes (64KB) |
| `max_queue_size` | 1000 | Maximum number of messages per queue |
| `default_permissions` | 6 | Default permissions (rw-rw-rw-) |
| `pipe_buffer_size` | 8192 | Default pipe buffer size (8KB) |
| `timeout_ms` | 5000 | Default timeout for IPC operations |
| `max_shared_memory` | 1048576 | Maximum shared memory segment size (1MB) |
| `max_semaphore_value` | 32767 | Maximum semaphore value (POSIX SEM_VALUE_MAX) |

## Usage Patterns

### Producer-Consumer with Message Queue

```cursed
yeet "ipc"

# Producer process
slay producer() {
    ipc.create_message_queue("work_queue", 100)
    
    sus i normie = 0
    while i < 10 {
        sus task tea = "Task " + core.tea(i)
        ipc.send_message("work_queue", task, ipc.MSG_PRIORITY_NORMAL)
        i = i + 1
    }
}

# Consumer process
slay consumer() {
    while based {
        sus message map = ipc.receive_message("work_queue")
        if message.size() > 0 {
            sus task tea = message.get("content")
            vibez.spill("Processing: " + task)
        } else {
            break  # No more messages
        }
    }
}
```

### Resource Pool with Semaphores

```cursed
# Initialize resource pool (5 concurrent workers)
ipc.create_semaphore("worker_pool", 5)

slay acquire_worker(worker_id tea) lit {
    if ipc.semaphore_wait("worker_pool", worker_id) {
        vibez.spill("Worker " + worker_id + " acquired resource")
        damn based
    } else {
        vibez.spill("Worker " + worker_id + " waiting for resource")
        damn cap
    }
}

slay release_worker() {
    ipc.semaphore_signal("worker_pool")
    vibez.spill("Resource released")
}
```

### Shared Configuration with Shared Memory

```cursed
# Setup shared configuration
ipc.create_shared_memory("app_config", 4096)
ipc.write_shared_memory("app_config", "debug_mode", "true")
ipc.write_shared_memory("app_config", "max_connections", "100")
ipc.write_shared_memory("app_config", "timeout", "30")

# Read configuration from any process
slay get_config(key tea) tea {
    damn ipc.read_shared_memory("app_config", key)
}

sus debug_enabled tea = get_config("debug_mode")
sus max_conn tea = get_config("max_connections")
```

### Logging Pipeline with Named Pipes

```cursed
# Setup logging pipeline
ipc.create_named_pipe("app_logs", 8192)
ipc.open_pipe_writer("app_logs", "main_app")
ipc.open_pipe_reader("app_logs", "log_processor")

# Application logging
slay log_message(level tea, message tea) {
    sus log_entry tea = "[" + level + "] " + message
    ipc.write_to_pipe("app_logs", log_entry)
}

# Log processor
slay process_logs() {
    while based {
        sus log_entry tea = ipc.read_from_pipe("app_logs")
        if log_entry != "" {
            vibez.spill("LOG: " + log_entry)
        } else {
            break  # No more logs
        }
    }
}
```

## Error Handling

The IPC module uses return values to indicate success/failure:

- Functions returning `lit` use `based` for success, `cap` for failure
- Functions returning data use empty values (`""`, `{}`, `[]`) for failure
- Always check return values before proceeding with operations

```cursed
# Proper error handling
if !ipc.create_message_queue("my_queue", 100) {
    vibez.spill("Failed to create message queue")
    damn
}

if !ipc.send_message("my_queue", "test", ipc.MSG_PRIORITY_NORMAL) {
    vibez.spill("Failed to send message")
    damn
}

sus message map = ipc.receive_message("my_queue")
if message.size() == 0 {
    vibez.spill("No messages available")
    damn
}
```

## Integration with Signal Boost

The IPC module integrates with the signal_boost module for graceful shutdown:

```cursed
# Automatic cleanup on termination signals
signal_boost.register_signal_handler(signal_boost.SIGTERM, 
                                     signal_boost.HANDLER_CUSTOM, 
                                     "cleanup_ipc")
```

When SIGTERM or SIGINT is received, the IPC subsystem automatically:
1. Closes all named pipes
2. Clears all message queues  
3. Detaches all shared memory segments
4. Releases all semaphores
5. Closes all Unix sockets

## Testing

Comprehensive test suite available in `test_ipc.csd`:

```bash
# Run IPC tests
cargo run --bin cursed stdlib/ipc/test_ipc.csd

# Compile and run natively
cargo run --bin cursed -- compile stdlib/ipc/test_ipc.csd
./test_ipc
```

The test suite covers:
- All IPC mechanism creation and basic operations
- Error conditions and edge cases
- Resource limits and overflow handling
- Process management and coordination
- Performance with high message volumes
- Integration between different IPC types
- Cleanup and resource management

## Performance Characteristics

- **Message Queues**: O(n) insertion with priority ordering, O(1) removal
- **Named Pipes**: O(1) write/read operations with FIFO ordering
- **Shared Memory**: O(1) key-value access operations
- **Semaphores**: O(1) wait/signal operations
- **Memory Usage**: Configurable limits prevent unbounded growth
- **Scalability**: Tested with 1000+ messages and multiple concurrent processes

## Module Information

```cursed
sus info tea = ipc.get_module_info()
# Returns: "ipc v1.0 - Pure CURSED inter-process communication system"
```

## See Also

- `signal_boost` - Signal handling and graceful shutdown
- `core` - Core CURSED language utilities
- `testz` - Testing framework for validation
