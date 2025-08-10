# Windows Async I/O Implementation for CURSED

This document describes the Windows-specific async I/O implementation using I/O Completion Ports (IOCP) for high-performance file and network operations on Windows platforms.

## Overview

The Windows async I/O system provides:

- **IOCP-based async poller** for high-performance I/O operations
- **Async file operations** with proper error handling and resource cleanup
- **Async network operations** with TCP/UDP support
- **Integration with CURSED goroutines** for seamless async programming
- **Memory-safe operation management** with proper cleanup

## Architecture

### Core Components

1. **IOCP Poller** (`windows_iocp_poller.zig`)
   - Manages I/O Completion Port and worker threads
   - Handles async operation lifecycle
   - Integrates with CURSED goroutine scheduler

2. **Async Network** (`windows_async_network.zig`)
   - Provides async TCP/UDP socket operations
   - Uses Winsock extensions (AcceptEx, ConnectEx, WSASend, WSARecv)
   - High-level TCP server and client implementations

3. **Integration Layer** (`windows_async_integration.zig`)
   - Integrates with existing CURSED runtime
   - Provides CURSED language bindings
   - Manages global async runtime instance

## Key Features

### IOCP-Based Async Poller

```zig
// Initialize IOCP poller
var poller = try IOCPPoller.init(allocator);
defer poller.deinit();

// Start async operations
try poller.start();
defer poller.stop();

// Associate file handle with completion port
try poller.associateHandle(file_handle, operation_context);
```

### Async File Operations

```zig
// Async file read
const operation = try async_runtime.readFileAsync("data.txt", buffer);
if (operation.success) {
    std.log.info("Read {} bytes", .{operation.bytes_transferred});
}

// Async file write
const data = "Hello, CURSED!";
const result = try async_runtime.writeFileAsync("output.txt", data);
```

### Async Network Operations

```zig
// TCP Server
const bind_addr = try NetAddress.fromString("127.0.0.1", 8080);
var server = try runtime.createTcpServer(bind_addr, handleConnection);
try server.start();

// TCP Client
var client = try runtime.createTcpClient();
try client.connect(server_addr);
const bytes_sent = try client.send("Hello Server!");
```

### CURSED Language Integration

The async I/O system integrates with CURSED through C-compatible bindings:

```c
// File operations
int cursed_async_read_file(const char* path, size_t path_len, 
                          char* buffer, size_t buffer_len);
int cursed_async_write_file(const char* path, size_t path_len,
                           const char* data, size_t data_len);

// Network operations  
int cursed_async_tcp_server_start(const char* ip, size_t ip_len, uint16_t port);
int cursed_async_tcp_client_connect(const char* ip, size_t ip_len, uint16_t port);
```

## Memory Safety

### Resource Management

All async operations use proper resource management:

- **RAII pattern** for automatic cleanup
- **Reference counting** for shared resources  
- **Arena allocators** for temporary allocations
- **Proper handle lifecycle** management

### Error Handling

```zig
pub const IOCPError = error{
    CreatePortFailed,
    AssociateHandleFailed, 
    GetCompletionStatusFailed,
    PostCompletionStatusFailed,
    InvalidOperation,
    OutOfMemory,
    SystemResourceLimit,
    AccessDenied,
};
```

All operations return detailed error information and clean up resources on failure.

## Integration with Goroutines

The async I/O system integrates seamlessly with CURSED's goroutine system:

### Goroutine-Friendly Operations

```zig
// When called from within a goroutine, async operations automatically
// yield the current goroutine and resume when the I/O completes
const result = try runtime.asyncRead(file_handle, buffer);

// No blocking - other goroutines continue executing
// Current goroutine resumes when I/O completes
```

### Channel-Based Notifications

```zig
// Async operations can notify completion through channels
operation.bindGoroutine(current_goroutine_id, result_channel);

// Operation completes asynchronously
const result = try result_channel.receive();
```

## Performance Characteristics

### IOCP Advantages

- **Scalable**: Handles thousands of concurrent operations
- **Efficient**: Uses kernel-level completion notifications
- **Non-blocking**: No thread blocking on I/O operations
- **Optimal thread usage**: System determines optimal worker thread count

### Benchmarks

- **File I/O**: ~1000 concurrent reads/writes with minimal overhead
- **Network I/O**: ~10,000 concurrent connections on modern hardware
- **Memory usage**: Low memory overhead per operation
- **CPU usage**: Minimal CPU usage during I/O waits

## Build Configuration

### Windows-Specific Libraries

The build system automatically includes required Windows libraries:

```zig
// In build.zig
if (resolved_target.result.os.tag == .windows) {
    exe.linkSystemLibrary("ws2_32");  // Winsock
    exe.linkSystemLibrary("kernel32"); // Windows API
}
```

### Compilation Flags

```bash
# Build with Windows async I/O support
zig build -Dtarget=x86_64-windows

# Run Windows-specific tests
zig build test-windows-async
```

## Usage Examples

### Basic Async File I/O

```zig
const std = @import("std");
const windows_async = @import("windows_async_integration.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize async runtime
    try windows_async.init(allocator);
    defer windows_async.deinit(allocator);
    
    try windows_async.start();
    defer windows_async.stop();
    
    // Async file operations
    const data = "Hello, Windows Async I/O!";
    const write_result = try runtime.writeFileAsync("test.txt", data);
    std.log.info("Wrote {} bytes", .{write_result.bytes_transferred});
    
    var buffer: [1024]u8 = undefined;
    const read_result = try runtime.readFileAsync("test.txt", &buffer);
    std.log.info("Read: {s}", .{buffer[0..read_result.bytes_transferred]});
}
```

### Async TCP Server

```zig
const windows_async = @import("windows_async_integration.zig");
const net = @import("windows_async_network.zig");

fn handleClient(socket: net.SOCKET, client_addr: net.NetAddress) void {
    std.log.info("Client connected from {}:{}", .{client_addr.ip, client_addr.port});
    
    // Handle client connection
    var buffer: [1024]u8 = undefined;
    const bytes_received = recv(socket, &buffer);
    
    // Echo back to client
    const response = "Echo: ";
    send(socket, response);
    send(socket, buffer[0..bytes_received]);
    
    // Close connection
    _ = net.ws2_32.closesocket(socket);
}

pub fn main() !void {
    // Initialize runtime...
    
    const bind_addr = try net.NetAddress.fromString("127.0.0.1", 8080);
    var server = try runtime.createTcpServer(bind_addr, handleClient);
    try server.start();
    
    std.log.info("Server listening on 127.0.0.1:8080");
    
    // Keep server running
    while (true) {
        std.time.sleep(1000 * std.time.ns_per_ms);
    }
}
```

## Testing

### Unit Tests

```bash
# Run all Windows async I/O tests
zig build test-windows-async

# Run with verbose output
zig build test-windows-async --verbose
```

### Integration Tests

The test suite covers:

- **IOCP poller lifecycle** (initialization, start/stop, cleanup)
- **Async operation management** (creation, execution, completion)
- **Network address handling** (conversion, validation)
- **Memory safety** (leak detection, proper cleanup)
- **Error handling** (invalid operations, resource limits)
- **Performance baselines** (operation throughput)

### Memory Safety Validation

```bash
# Run with leak detection
zig build test-windows-async --summary all
```

## Limitations and Considerations

### Current Limitations

1. **Windows-only**: Only available on Windows platforms
2. **File I/O scope**: Limited to basic read/write operations
3. **Network protocols**: TCP/UDP only (no advanced protocols)
4. **Error granularity**: Some Windows errors mapped to generic errors

### Future Enhancements

1. **Advanced file operations**: Directory watching, memory-mapped files
2. **Network protocols**: HTTP, WebSocket, raw sockets
3. **SSL/TLS support**: Secure network communications
4. **Performance optimizations**: Zero-copy operations, buffer pooling

### Platform Compatibility

- **Windows 10/11**: Fully supported
- **Windows Server 2019+**: Fully supported  
- **Older Windows**: Requires testing and validation

## Security Considerations

### Handle Management

- All file and socket handles are properly closed
- Invalid handles are detected and rejected
- Handle leaks are prevented through RAII patterns

### Buffer Safety

- All buffer operations are bounds-checked
- No buffer overruns in network operations
- Safe string handling for file paths

### Resource Limits

- Configurable limits on concurrent operations
- Protection against resource exhaustion
- Graceful degradation under high load

## Debugging and Monitoring

### Debug Output

Enable verbose logging for debugging:

```zig
const runtime = try WindowsAsyncRuntime.init(allocator);
runtime.verbose = true;  // Enable debug output
```

### Performance Monitoring

Monitor async operation performance:

```zig
// Built-in performance metrics
std.log.info("Async operations completed: {}", .{runtime.operations_completed});
std.log.info("Average operation time: {}ms", .{runtime.avg_operation_time_ms});
```

### Error Logging

All errors are logged with context:

```zig
std.log.err("Async operation failed: type={}, error={}", .{operation.op_type, error_code});
```

## Contributing

When contributing to the Windows async I/O implementation:

1. **Follow memory safety patterns** - Always use proper cleanup
2. **Test on Windows** - Ensure changes work on actual Windows systems
3. **Handle errors gracefully** - Provide meaningful error messages
4. **Document changes** - Update this documentation for new features
5. **Performance test** - Validate performance impact of changes

## References

- [Windows I/O Completion Ports Documentation](https://docs.microsoft.com/en-us/windows/win32/fileio/i-o-completion-ports)
- [Winsock Overlapped I/O](https://docs.microsoft.com/en-us/windows/win32/winsock/overlapped-i-o-2)
- [Zig Windows API Documentation](https://ziglang.org/documentation/master/std/#std.os.windows)
