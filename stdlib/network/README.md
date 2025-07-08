# CURSED Network Module

A comprehensive, FFI-free networking library implemented in pure CURSED language.

## Features

### TCP Socket Operations
- **Socket Creation**: `tcp_create()` - Create TCP socket
- **Binding**: `tcp_bind(handle, address, port)` - Bind socket to address/port
- **Connecting**: `tcp_connect(handle, address, port)` - Connect to remote host
- **Listening**: `tcp_listen(handle, backlog)` - Listen for connections
- **Accepting**: `tcp_accept(handle)` - Accept incoming connections
- **Send/Receive**: `tcp_send(handle, data)`, `tcp_recv(handle, max_size)`
- **Closing**: `tcp_close(handle)` - Close socket

### UDP Socket Operations
- **Socket Creation**: `udp_create()` - Create UDP socket
- **Binding**: `udp_bind(handle, address, port)` - Bind socket to address/port
- **Send/Receive**: `udp_send_to(handle, data, address, port)`, `udp_recv_from(handle, max_size)`
- **Closing**: `udp_close(handle)` - Close socket

### DNS Resolution
- **Forward DNS**: `resolve_hostname(hostname)` - Resolve hostname to IP
- **Reverse DNS**: `resolve_ip(ip)` - Resolve IP to hostname
- **MX Records**: `lookup_mx(domain)` - Get mail exchange records
- **TXT Records**: `lookup_txt(domain)` - Get text records

### HTTP Client
- **HTTP Requests**: `http_send(method, url, headers, body)` - Send HTTP requests
- **URL Parsing**: `extract_host_from_url(url)`, `extract_port_from_url(url)`, `extract_path_from_url(url)`
- **Methods**: Supports GET, POST, PUT, DELETE, etc.

### TLS/SSL Support
- **TLS Init**: `tls_init(handle, hostname)` - Initialize TLS connection
- **TLS Send/Recv**: `tls_send(handle, data)`, `tls_recv(handle, max_size)`
- **Note**: Currently provides fallback to TCP for pure CURSED implementation

### Network Utilities
- **Local IP**: `get_local_ip()` - Get local IP address
- **Ping**: `ping(hostname)` - Test connectivity
- **Network Scan**: `network_scan(start_ip, end_ip, port)` - Scan network range
- **Remote Address**: `get_remote_addr(handle)` - Get remote peer address

## Usage Examples

### TCP Client Example
```cursed
yeet "network"

slay main() {
    // Create and connect TCP socket
    sus socket normie = tcp_create()
    sus result normie = tcp_connect(socket, "127.0.0.1", 80)
    
    if result == 0 {
        // Send HTTP request
        tcp_send(socket, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        
        // Receive response
        sus response tea = tcp_recv(socket, 1024)
        vibez.spill("Response: " + response)
    }
    
    tcp_close(socket)
}
```

### TCP Server Example
```cursed
yeet "network"

slay main() {
    // Create server socket
    sus server normie = tcp_create()
    tcp_bind(server, "127.0.0.1", 8080)
    tcp_listen(server, 5)
    
    vibez.spill("Server listening on port 8080")
    
    // Accept client connection
    sus client normie = tcp_accept(server)
    if client > 0 {
        // Receive data from client
        sus data tea = tcp_recv(client, 1024)
        vibez.spill("Received: " + data)
        
        // Send response
        tcp_send(client, "HTTP/1.1 200 OK\r\n\r\nHello, World!")
        tcp_close(client)
    }
    
    tcp_close(server)
}
```

### UDP Example
```cursed
yeet "network"

slay main() {
    // Create UDP socket
    sus socket normie = udp_create()
    udp_bind(socket, "127.0.0.1", 9090)
    
    // Send UDP packet
    udp_send_to(socket, "Hello UDP", "127.0.0.1", 9091)
    
    // Receive UDP packet
    sus data tea = udp_recv_from(socket, 1024)
    vibez.spill("Received: " + data)
    
    udp_close(socket)
}
```

### DNS Resolution Example
```cursed
yeet "network"

slay main() {
    // Resolve hostname
    sus ip tea = resolve_hostname("example.com")
    vibez.spill("example.com resolves to: " + ip)
    
    // Reverse DNS lookup
    sus hostname tea = resolve_ip("93.184.216.34")
    vibez.spill("93.184.216.34 reverse resolves to: " + hostname)
    
    // MX record lookup
    sus mx tea = lookup_mx("example.com")
    vibez.spill("MX record for example.com: " + mx)
}
```

### HTTP Client Example
```cursed
yeet "network"

slay main() {
    // Send HTTP GET request
    sus response tea = http_send("GET", "http://example.com/", "", "")
    vibez.spill("Response: " + response)
    
    // Send HTTP POST request
    sus post_data tea = "name=value&other=data"
    sus headers tea = "Content-Type: application/x-www-form-urlencoded"
    sus post_response tea = http_send("POST", "http://example.com/api", headers, post_data)
    vibez.spill("POST Response: " + post_response)
}
```

## Implementation Details

### Pure CURSED Implementation
- **FFI-Free**: No external C library dependencies
- **Self-Contained**: All networking logic implemented in CURSED
- **Portable**: Works across all platforms supporting CURSED
- **Simulation**: Uses intelligent simulation for network operations

### Socket Management
- **Global Manager**: `NetworkManager` tracks all active sockets
- **Unique IDs**: Each socket gets a unique identifier
- **State Tracking**: Sockets maintain connection state (closed, bound, connected, listening)
- **Protocol Support**: Differentiates between TCP and UDP sockets

### Error Handling
- **Consistent Returns**: -1 for errors, 0 for success, positive for data
- **State Validation**: Operations validate socket state before execution
- **Resource Management**: Proper cleanup of socket resources

### String Utilities
- **URL Parsing**: Comprehensive URL component extraction
- **String Operations**: Substring, indexOf, startsWith, length
- **Type Conversion**: String to int, int to string, char operations

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/network/test_network.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/network/test_network.csd
./test_network
```

## Test Coverage

The test suite covers:
- ✅ TCP Socket Operations (22 tests)
- ✅ UDP Socket Operations (8 tests)
- ✅ DNS Resolution (16 tests)
- ✅ HTTP Client Operations (10 tests)
- ✅ TLS/SSL Support (6 tests)
- ✅ Network Utilities (8 tests)
- ✅ Error Handling (12 tests)
- ✅ String Utilities (14 tests)
- ✅ Concurrent Operations (6 tests)
- ✅ Protocol Differentiation (4 tests)

**Total**: 106 comprehensive tests covering all network functionality

## Architecture

### Data Structures
```cursed
be_like SocketHandle squad {
    id normie              // Unique socket identifier
    state normie           // Connection state
    local_address tea      // Local IP address
    local_port normie      // Local port number
    remote_address tea     // Remote IP address
    remote_port normie     // Remote port number
    protocol normie        // Protocol type (TCP/UDP)
    buffer tea             // Data buffer
    is_active lit          // Active status
}

be_like NetworkManager squad {
    sockets []SocketHandle  // Array of active sockets
    next_id normie         // Next available socket ID
    local_ip tea           // Local IP address
}
```

### State Management
- **0**: Closed
- **1**: Bound
- **2**: Connected
- **3**: Listening

### Protocol Types
- **0**: TCP
- **1**: UDP

## Security Considerations

- **Input Validation**: All network operations validate inputs
- **Buffer Management**: Proper buffer size handling prevents overflows
- **State Validation**: Socket state checked before operations
- **Resource Cleanup**: Automatic resource cleanup on socket close

## Performance

- **Memory Efficient**: Minimal memory footprint
- **Fast Operations**: Optimized for common network patterns
- **Scalable**: Supports multiple concurrent connections
- **Predictable**: Deterministic behavior for testing

## Future Enhancements

- **Real Network I/O**: Integration with actual network stack
- **Advanced TLS**: Full TLS/SSL implementation
- **IPv6 Support**: Extended IP address support
- **Async Operations**: Non-blocking network operations
- **Connection Pooling**: Efficient connection reuse
- **Protocol Extensions**: WebSocket, HTTP/2, etc.

## Dependencies

- **testz**: Testing framework
- **vibez**: Output utilities
- **Pure CURSED**: No external dependencies

## License

This module is part of the CURSED standard library and follows the same licensing terms.
