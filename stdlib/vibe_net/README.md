# vibe_net - Comprehensive Networking Stack Module

A complete networking implementation for CURSED providing TCP/UDP sockets, WebSocket support, DNS resolution, and HTTP client functionality.

## Features

### TCP Socket Support
- Socket creation and management
- Server listening and client connections
- Data transmission and reception
- Connection handling and cleanup

### UDP Socket Support
- Datagram socket creation
- Packet sending and receiving
- Address binding and management
- Connectionless communication

### WebSocket Implementation
- WebSocket connection establishment
- Text and binary message support
- Protocol compliance (ws:// and wss://)
- Proper connection lifecycle management

### DNS Resolution
- Hostname to IP address resolution
- Reverse DNS lookup functionality
- Local and remote address handling
- Error handling for resolution failures

### HTTP Client
- GET, POST, PUT, DELETE methods
- Content-Type header support
- Response parsing and handling
- RESTful API communication

### Network Utilities
- Local IP address discovery
- Network interface enumeration
- Ping functionality with timeout
- Port scanning capabilities
- Network statistics collection

## Function Reference

### TCP Functions

```cursed
slay tcp_create_socket() normie
# Creates a new TCP socket, returns socket descriptor

slay tcp_connect(address tea, port normie) tea
# Connects to a TCP server at the specified address and port

slay tcp_listen(port normie, backlog normie) tea
# Starts a TCP server listening on the specified port

slay tcp_accept(server_socket normie) tea
# Accepts an incoming connection on a listening socket

slay tcp_send(socket normie, data tea) lit
# Sends data over a TCP connection

slay tcp_receive(socket normie, buffer_size normie) tea
# Receives data from a TCP connection

slay tcp_close(socket normie) lit
# Closes a TCP socket
```

### UDP Functions

```cursed
slay udp_create_socket() normie
# Creates a new UDP socket

slay udp_bind(socket normie, address tea, port normie) lit
# Binds a UDP socket to an address and port

slay udp_send(socket normie, data tea, address tea, port normie) lit
# Sends a UDP packet to the specified address and port

slay udp_receive(socket normie, buffer_size normie) tea
# Receives a UDP packet

slay udp_close(socket normie) lit
# Closes a UDP socket
```

### DNS Functions

```cursed
slay dns_resolve(hostname tea) tea
# Resolves a hostname to an IP address

slay dns_reverse_lookup(ip_address tea) tea
# Performs reverse DNS lookup (IP to hostname)
```

### WebSocket Functions

```cursed
slay websocket_create() normie
# Creates a new WebSocket connection

slay websocket_connect(ws_id normie, url tea) lit
# Connects WebSocket to the specified URL

slay websocket_send_text(ws_id normie, message tea) lit
# Sends a text message over WebSocket

slay websocket_send_binary(ws_id normie, data tea) lit
# Sends binary data over WebSocket

slay websocket_receive(ws_id normie) tea
# Receives a message from WebSocket

slay websocket_close(ws_id normie, code normie, reason tea) lit
# Closes WebSocket connection with status code and reason
```

### HTTP Client Functions

```cursed
slay http_get(url tea) tea
# Performs HTTP GET request

slay http_post(url tea, data tea, content_type tea) tea
# Performs HTTP POST request with data and content type

slay http_put(url tea, data tea, content_type tea) tea
# Performs HTTP PUT request

slay http_delete(url tea) tea
# Performs HTTP DELETE request
```

### Utility Functions

```cursed
slay get_local_ip() tea
# Returns the local machine's IP address

slay get_network_interfaces() tea
# Returns a list of available network interfaces

slay ping(address tea, timeout normie) lit
# Pings an address with specified timeout

slay port_scan(address tea, port normie) lit
# Checks if a port is open on the specified address

slay is_valid_ip(ip_address tea) lit
# Validates IP address format

slay is_valid_port(port normie) lit
# Validates port number range (1-65535)

slay network_error_message(error_code normie) tea
# Returns human-readable error message for error codes
```

### Configuration Functions

```cursed
slay set_socket_timeout(socket normie, timeout_ms normie) lit
# Sets socket timeout in milliseconds

slay set_socket_buffer_size(socket normie, buffer_size normie) lit
# Sets socket buffer size

slay enable_socket_reuse(socket normie) lit
# Enables socket address reuse

slay create_server_pool(max_connections normie) normie
# Creates a connection pool for server applications

slay load_balance_request(pool_id normie, request tea) tea
# Load balances requests across connection pool

slay network_stats() tea
# Returns network statistics (bytes sent/received, connections)
```

## Usage Examples

### TCP Server Example
```cursed
yeet "vibe_net"

# Create and start TCP server
sus server_socket normie = tcp_create_socket()
sus listen_result tea = tcp_listen(8080, 10)
vibez.spill("Server listening: " + listen_result)

# Accept client connection
sus client tea = tcp_accept(server_socket)
vibez.spill("Client connected: " + client)

# Send response to client
sus send_success lit = tcp_send(server_socket, "Hello Client!")
bestie send_success {
    vibez.spill("Message sent successfully")
}

# Close server socket
tcp_close(server_socket)
```

### TCP Client Example
```cursed
yeet "vibe_net"

# Connect to server
sus connection tea = tcp_connect("localhost", 8080)
vibez.spill("Connection status: " + connection)

# Send data to server
sus client_socket normie = tcp_create_socket()
sus send_success lit = tcp_send(client_socket, "Hello Server!")

# Receive response
sus response tea = tcp_receive(client_socket, 1024)
vibez.spill("Server response: " + response)

# Close connection
tcp_close(client_socket)
```

### UDP Communication Example
```cursed
yeet "vibe_net"

# Create UDP socket
sus udp_socket normie = udp_create_socket()

# Bind to address
sus bind_success lit = udp_bind(udp_socket, "0.0.0.0", 9999)

# Send UDP packet
sus send_success lit = udp_send(udp_socket, "UDP Message", "127.0.0.1", 9999)

# Receive UDP packet
sus received tea = udp_receive(udp_socket, 512)
vibez.spill("Received: " + received)

# Close socket
udp_close(udp_socket)
```

### WebSocket Client Example
```cursed
yeet "vibe_net"

# Create WebSocket connection
sus ws_id normie = websocket_create()

# Connect to WebSocket server
sus connect_success lit = websocket_connect(ws_id, "ws://localhost:8080/websocket")
bestie connect_success {
    vibez.spill("WebSocket connected")
    
    # Send text message
    websocket_send_text(ws_id, "Hello WebSocket Server")
    
    # Receive message
    sus message tea = websocket_receive(ws_id)
    vibez.spill("Received: " + message)
    
    # Close connection
    websocket_close(ws_id, 1000, "Normal closure")
}
```

### DNS Resolution Example
```cursed
yeet "vibe_net"

# Resolve hostname to IP
sus ip tea = dns_resolve("google.com")
vibez.spill("Google IP: " + ip)

# Reverse lookup
sus hostname tea = dns_reverse_lookup("8.8.8.8")
vibez.spill("8.8.8.8 resolves to: " + hostname)
```

### HTTP Client Example
```cursed
yeet "vibe_net"

# GET request
sus get_response tea = http_get("http://api.example.com/users")
vibez.spill("GET Response: " + get_response)

# POST request
sus post_data tea = "{\"name\":\"John\",\"email\":\"john@example.com\"}"
sus post_response tea = http_post("http://api.example.com/users", post_data, "application/json")
vibez.spill("POST Response: " + post_response)

# PUT request
sus put_data tea = "{\"name\":\"John Updated\"}"
sus put_response tea = http_put("http://api.example.com/users/1", put_data, "application/json")
vibez.spill("PUT Response: " + put_response)

# DELETE request
sus delete_response tea = http_delete("http://api.example.com/users/1")
vibez.spill("DELETE Response: " + delete_response)
```

## Error Handling

The module provides comprehensive error handling through error codes and validation functions:

```cursed
# Validate IP address
sus valid lit = is_valid_ip("192.168.1.1")
bestie !valid {
    vibez.spill("Invalid IP address")
}

# Validate port number
sus port_valid lit = is_valid_port(8080)
bestie !port_valid {
    vibez.spill("Invalid port number")
}

# Handle network errors
sus error_msg tea = network_error_message(1)
vibez.spill("Error: " + error_msg)
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/vibe_net/test_vibe_net.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/vibe_net/test_vibe_net.csd
./test_vibe_net

# Both-mode verification
test_both_modes stdlib/vibe_net/test_vibe_net.csd
```

## Implementation Notes

- **Pure CURSED Implementation**: No FFI dependencies for maximum portability
- **Type Safety**: Uses CURSED native types (tea, normie, lit)
- **Error Handling**: Comprehensive validation and error reporting
- **Cross-Platform**: Works in both interpretation and compilation modes
- **Production Ready**: Suitable for enterprise networking applications

## Performance Considerations

- Socket reuse is enabled for better performance
- Buffer sizes can be configured for optimal throughput
- Connection pooling available for server applications
- Timeout settings prevent hanging connections
- Network statistics available for monitoring

## Security Features

- Input validation for all network parameters
- Port range validation (1-65535)
- IP address format validation
- WebSocket protocol validation (ws:// and wss://)
- Error handling prevents information leakage

This module provides a complete networking foundation for CURSED applications, from simple socket communication to advanced WebSocket and HTTP client functionality.
