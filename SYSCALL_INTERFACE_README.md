# CURSED Real Syscall Interface

This document describes the implementation of a real syscall interface for the CURSED programming language, replacing mock operations with actual system calls.

## Overview

The CURSED syscall interface provides real system call access for:

- **File System Operations**: Real file I/O, directory management, and metadata access
- **Network Operations**: TCP/UDP sockets, HTTP client functionality
- **Process Management**: Process spawning, control, and environment management
- **System Integration**: Environment variables, working directory, system information

## Architecture

The syscall interface is implemented in several layers:

### 1. Zig Syscall Interface (`src-zig/syscall_interface.zig`)

Low-level Zig implementation that provides direct system call access:

- File operations: `cursed_file_open`, `cursed_file_read`, `cursed_file_write`, etc.
- Network operations: `cursed_socket_create`, `cursed_socket_connect`, `cursed_socket_send`, etc.
- Process operations: `cursed_process_spawn`, `cursed_process_wait`, `cursed_process_kill`, etc.
- Environment operations: `cursed_env_get`, `cursed_env_set`

### 2. Runtime Integration (`src-zig/runtime_syscall_integration.zig`)

LLVM IR generation for syscall functions that integrates with the CURSED runtime:

- Generates external function declarations for syscalls
- Provides helper functions for CURSED data type conversion
- Integrates with memory management and garbage collection
- Handles initialization and cleanup

### 3. CURSED Stdlib Modules

High-level CURSED language interfaces:

- `stdlib/fs_real/mod.csd`: Real filesystem operations
- `stdlib/net_real/mod.csd`: Real networking operations  
- `stdlib/process_real/mod.csd`: Real process management

## Features

### File System Operations

```cursed
yeet "fs_real"

// Write and read files
write_file("/tmp/test.txt", "Hello, CURSED!")
sus content tea = read_file("/tmp/test.txt")

// Directory operations
create_dir("/tmp/my_dir")
sus is_directory lit = is_dir("/tmp/my_dir")

// File metadata
sus metadata FileMetadata = get_file_metadata("/tmp/test.txt")
sus size thicc = get_file_size("/tmp/test.txt")
```

### Network Operations

```cursed
yeet "net_real"

// TCP client
sus socket TCPSocket = tcp_socket_create()
tcp_socket_connect(&socket, "127.0.0.1", 8080)
tcp_socket_send(&socket, "Hello, server!")
sus response tea = tcp_socket_recv(&socket, 1024)
tcp_socket_close(&socket)

// TCP server
sus listener TCPListener = tcp_listener_create("127.0.0.1", 8080, 10)
sus client TCPSocket = tcp_listener_accept(&listener)
sus message tea = tcp_socket_recv(&client, 1024)

// HTTP client
sus response HTTPResponse = http_get("http://example.com/api")
sus post_response HTTPResponse = http_post("http://api.example.com/data", "{\"key\":\"value\"}", "application/json")
```

### Process Management

```cursed
yeet "process_real"

// Spawn processes
sus args []tea = []tea{"ls", "-la"}
sus process Process = process_spawn("ls", args)
sus exit_code normie = process_wait(&process)

// Environment variables
sus path tea = env_get("PATH")
env_set("MY_VAR", "my_value")

// System information
sus pid normie = get_current_pid()
sus user tea = get_current_user()
sus hostname tea = get_hostname()
```

## Implementation Details

### Error Handling

All syscall operations return appropriate error codes:

- File operations return `false` or empty strings on failure
- Network operations return negative socket IDs or error codes
- Process operations return negative process IDs or non-zero exit codes

### Memory Management

The syscall interface integrates with CURSED's garbage collection:

- Temporary buffers are allocated through the CURSED allocator
- String conversions use GC-managed memory
- Resource cleanup is automatic through RAII patterns

### Platform Support

The syscall interface targets multiple platforms:

- **Linux**: Full support for all syscalls
- **macOS**: Compatible with Darwin syscalls
- **Windows**: Uses Windows API equivalents where needed

### Thread Safety

All syscall operations are thread-safe:

- Socket registry uses mutex protection
- File handle registry is synchronized
- Process registry handles concurrent access

## Usage Examples

### File System Example

```cursed
yeet "fs_real"
yeet "testz"

slay file_example() {
    // Create a test file
    sus filename tea = "/tmp/cursed_example.txt"
    sus content tea = "CURSED syscall interface demonstration"
    
    // Write file
    lowkey write_file(filename, content) {
        vibez.spill("File written successfully")
        
        // Read it back
        sus read_content tea = read_file(filename)
        vibez.spill("Read content: " + read_content)
        
        // Get file info
        sus size thicc = get_file_size(filename)
        vibez.spill("File size: " + thicc_to_string(size))
        
        // Clean up
        delete_file(filename)
    }
}
```

### Network Example

```cursed
yeet "net_real"

slay network_example() {
    // Create HTTP GET request
    sus response HTTPResponse = http_get("http://httpbin.org/get")
    
    lowkey response.status_code == 200 {
        vibez.spill("HTTP request successful")
        vibez.spill("Response body: " + response.body)
    } nah {
        vibez.spill("HTTP request failed with status: " + normie_to_string(response.status_code))
    }
}
```

### Process Example

```cursed
yeet "process_real"

slay process_example() {
    // Get system information
    vibez.spill("Current PID: " + normie_to_string(get_current_pid()))
    vibez.spill("Current user: " + get_current_user())
    vibez.spill("Home directory: " + get_home_dir())
    
    // Execute a command
    sus exit_code normie = execute_shell_command("echo 'Hello from shell!'")
    vibez.spill("Command exit code: " + normie_to_string(exit_code))
}
```

## Testing

Comprehensive test suites are provided:

- `stdlib/fs_real/test_fs_real.csd`: File system operations
- `stdlib/net_real/test_net_real.csd`: Network operations
- `stdlib/process_real/test_process_real.csd`: Process management

Run tests with:

```bash
zig build && ./zig-out/bin/cursed-zig stdlib/fs_real/test_fs_real.csd
zig build && ./zig-out/bin/cursed-zig stdlib/net_real/test_net_real.csd
zig build && ./zig-out/bin/cursed-zig stdlib/process_real/test_process_real.csd
```

## Integration with CURSED Runtime

The syscall interface integrates seamlessly with the CURSED runtime:

1. **Initialization**: Called during runtime startup
2. **Memory Management**: Uses CURSED allocator and GC
3. **Error Handling**: Integrates with CURSED error system
4. **Concurrency**: Thread-safe for goroutine usage
5. **Cleanup**: Automatic resource cleanup on shutdown

## Performance Considerations

- Syscalls are optimized for performance with minimal overhead
- Buffer reuse reduces memory allocations
- Registry management uses efficient hash maps
- String conversions are minimized

## Security

- All file operations validate paths and permissions
- Network operations respect system firewall rules
- Process spawning follows system security policies
- Environment variable access is sandboxed

## Future Enhancements

Planned improvements include:

- Asynchronous I/O support for better concurrency
- Memory-mapped file support for large files
- Advanced networking features (SSL/TLS, HTTP/2)
- Process monitoring and resource limits
- Cross-platform path handling improvements

## Dependencies

The syscall interface depends on:

- Zig standard library for system calls
- LLVM for IR generation
- CURSED runtime system for memory management
- Platform-specific system libraries

## Troubleshooting

Common issues and solutions:

1. **Permission Denied**: Ensure proper file/directory permissions
2. **Network Connection Failed**: Check firewall and network connectivity
3. **Process Spawn Failed**: Verify command exists and is executable
4. **Memory Allocation Failed**: Check available system memory

## API Reference

### File System API

- `read_file(path: tea) -> tea`
- `write_file(path: tea, content: tea) -> lit`
- `append_file(path: tea, content: tea) -> lit`
- `delete_file(path: tea) -> lit`
- `file_exists(path: tea) -> lit`
- `get_file_size(path: tea) -> thicc`
- `create_dir(path: tea) -> lit`
- `remove_dir(path: tea) -> lit`
- `is_dir(path: tea) -> lit`
- `get_file_metadata(path: tea) -> FileMetadata`

### Network API

- `tcp_socket_create() -> TCPSocket`
- `tcp_socket_connect(socket: *TCPSocket, addr: tea, port: normie) -> lit`
- `tcp_socket_send(socket: *TCPSocket, data: tea) -> normie`
- `tcp_socket_recv(socket: *TCPSocket, size: normie) -> tea`
- `tcp_socket_close(socket: *TCPSocket) -> lit`
- `http_get(url: tea) -> HTTPResponse`
- `http_post(url: tea, data: tea, content_type: tea) -> HTTPResponse`

### Process API

- `process_spawn(command: tea, args: []tea) -> Process`
- `process_wait(process: *Process) -> normie`
- `process_kill(process: *Process, signal: normie) -> lit`
- `env_get(name: tea) -> tea`
- `env_set(name: tea, value: tea) -> lit`
- `get_current_pid() -> normie`
- `get_current_user() -> tea`
- `execute_command(command: tea) -> normie`

This syscall interface transforms CURSED from a language with simulated operations to one with full system integration capabilities.
