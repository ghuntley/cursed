# vibe_net - Comprehensive Networking Stack Module

A complete, production-ready networking implementation for CURSED following the vibe_net specification. This module provides TCP/UDP sockets, WebSocket support, DNS resolution, HTTP client functionality, and advanced networking features including connection pooling, circuit breakers, and rate limiting.

## Features

### Core Networking
- **TCP/UDP Sockets**: Full socket API with connection management
- **WebSocket Support**: WebSocket and secure WebSocket (wss://) protocols
- **DNS Resolution**: Comprehensive DNS queries (A, AAAA, MX, NS, TXT, SRV records)
- **HTTP Client**: RESTful HTTP client with GET, POST, PUT, DELETE methods
- **IPv6 Support**: Complete IPv6 implementation with dual-stack operation

### Advanced Features
- **Connection Pooling**: Efficient connection reuse for high-performance applications
- **Circuit Breaker**: Fault tolerance with automatic failure recovery
- **Rate Limiting**: Traffic control with configurable rate limits
- **Network Interfaces**: System network interface discovery and management
- **Protocol Adapters**: WebSocket, HTTP/2, and MQTT protocol support

### Production-Ready
- **Pure CURSED Implementation**: No FFI dependencies for maximum portability
- **Type Safety**: Uses CURSED native types (tea, normie, lit)
- **Error Handling**: Comprehensive validation and error reporting
- **Cross-Platform**: Works in both interpretation and compilation modes
- **Specification Compliant**: Follows vibe_net specification exactly

## Core Types

### IP Address Management

```cursed
# IP address representation
be_like IPVibe squad {
    address tea
    version normie
    zone tea
}

# Create IP address from string
sus ip IPVibe = ParseIP("192.168.1.1")

# Create IPv4 address from octets
sus ipv4 IPVibe = IPv4(192, 168, 1, 1)

# IP address methods
vibez.spill(ip.String())         # "192.168.1.1"
vibez.spill(ip.IsIPv4())         # true
vibez.spill(ip.IsIPv6())         # false
vibez.spill(ip.IsLoopback())     # false
vibez.spill(ip.IsPrivate())      # true
vibez.spill(ip.IsMulticast())    # false
```

### Address Resolution

```cursed
# TCP address resolution
sus tcp_addr TCPAddrVibe = ResolveTCPAddr("tcp", "localhost:8080")
vibez.spill(tcp_addr.Network())  # "tcp"
vibez.spill(tcp_addr.String())   # "localhost:8080"
vibez.spill(tcp_addr.Port())     # 8080

# UDP address resolution
sus udp_addr UDPAddrVibe = ResolveUDPAddr("udp", "localhost:8081")
vibez.spill(udp_addr.Network())  # "udp"
vibez.spill(udp_addr.String())   # "localhost:8081"
```

### Connection Management

```cursed
# Connection interface
be_like ConnVibe squad {
    id normie
    network tea
    local_addr tea
    remote_addr tea
    state normie
    read_timeout normie
    write_timeout normie
    keep_alive lit
}

# Connection methods
sus data tea = conn.Read(1024)
sus bytes_written normie = conn.Write("Hello")
sus close_success lit = conn.Close()
vibez.spill(conn.LocalAddr())
vibez.spill(conn.RemoteAddr())
```

## API Reference

### High-Level Functions

```cursed
# Connect to remote address
sus conn ConnVibe = Dial("tcp", "localhost:8080")

# Connect with timeout
sus conn ConnVibe = DialTimeout("tcp", "localhost:8080", 5000)

# Listen for connections
sus listener TCPListenerVibe = Listen("tcp", "0.0.0.0:8080")

# Accept connections
sus client_conn ConnVibe = listener.Accept()
```

### TCP Operations

```cursed
# TCP connection
sus local_addr TCPAddrVibe = ResolveTCPAddr("tcp", "0.0.0.0:0")
sus remote_addr TCPAddrVibe = ResolveTCPAddr("tcp", "localhost:8080")
sus tcp_conn TCPConnVibe = DialTCP("tcp", local_addr, remote_addr)

# TCP connection configuration
tcp_conn.SetKeepAlive(based)
tcp_conn.SetNoDelay(based)
tcp_conn.SetReadBuffer(16384)
tcp_conn.SetWriteBuffer(16384)
tcp_conn.SetKeepAlivePeriod(30000)
tcp_conn.SetLinger(10)

# TCP listener
sus listen_addr TCPAddrVibe = ResolveTCPAddr("tcp", "0.0.0.0:9090")
sus listener TCPListenerVibe = ListenTCP("tcp", listen_addr)
sus accepted_conn ConnVibe = listener.Accept()
sus accepted_tcp TCPConnVibe = listener.AcceptTCP()
```

### UDP Operations

```cursed
# UDP connection
sus local_addr UDPAddrVibe = ResolveUDPAddr("udp", "0.0.0.0:0")
sus remote_addr UDPAddrVibe = ResolveUDPAddr("udp", "localhost:8081")
sus udp_conn UDPConnVibe = DialUDP("udp", local_addr, remote_addr)

# UDP packet operations
sus n normie
sus from_addr UDPAddrVibe  
sus err tea
n, from_addr, err = udp_conn.ReadFromUDP(1024)

sus target_addr UDPAddrVibe = ResolveUDPAddr("udp", "127.0.0.1:8082")
sus bytes_sent normie = udp_conn.WriteToUDP("UDP data", target_addr)
```

### DNS Resolution

```cursed
# DNS resolver
sus resolver DNSResolverVibe = NewDNSResolver()

# Host resolution
sus addrs []tea = resolver.LookupHost("google.com")
sus ips []IPVibe = resolver.LookupIP("google.com")
sus hostnames []tea = resolver.LookupAddr("8.8.8.8")

# DNS records
sus mx_records []MXVibe = resolver.LookupMX("gmail.com")
sus ns_records []NSVibe = resolver.LookupNS("example.com")
sus txt_records []tea = resolver.LookupTXT("google.com")

# SRV records
sus cname tea
sus srv_records []SRVVibe
cname, srv_records = resolver.LookupSRV("http", "tcp", "example.com")

# Global DNS functions
sus global_addrs []tea = LookupHost("localhost")
sus global_ips []IPVibe = LookupIP("localhost")
sus global_mx []MXVibe = LookupMX("gmail.com")
```

### WebSocket Support

```cursed
# WebSocket connection
sus base_conn ConnVibe = Dial("tcp", "localhost:8080")
sus ws_conn WebSocketConnVibe = NewWebSocketConn(base_conn, "ws")

# WebSocket operations
sus msg_type normie
sus msg_data tea
msg_type, msg_data = ws_conn.ReadMessage()

sus write_success lit = ws_conn.WriteMessage(1, "Hello WebSocket")
ws_conn.Close()
```

### HTTP/2 Support

```cursed
# HTTP/2 connection
sus base_conn ConnVibe = Dial("tcp", "localhost:8080")
sus h2_conn HTTP2ConnVibe = NewHTTP2Conn(base_conn)

# HTTP/2 streams
sus stream HTTP2StreamVibe = h2_conn.CreateStream()
vibez.spill(stream.id)     # Stream ID
vibez.spill(stream.state)  # Stream state

h2_conn.Close()
```

### Connection Pooling

```cursed
# Connection pool
sus pool ConnPoolVibe = NewConnPool("tcp", "localhost:8080", 10)

# Get connection from pool
sus conn ConnVibe = pool.Get()

# Use connection
sus data tea = conn.Read(1024)
conn.Write("Hello")

# Return connection to pool
pool.Put(conn)

# Pool statistics
sus stats ConnPoolStats = pool.Stats()
vibez.spill(stats.ActiveConns)
vibez.spill(stats.IdleConns)
vibez.spill(stats.TotalAcquired)

# Close pool
pool.Close()
```

### Circuit Breaker

```cursed
# Circuit breaker for fault tolerance
sus cb CircuitBreakerVibe = NewCircuitBreaker(3, 60000)

# Execute operations through circuit breaker
sus result tea = cb.Execute("operation_name")

# Circuit breaker control
cb.Reset()  # Manual reset
```

### Rate Limiting

```cursed
# Rate limiter
sus rl RateLimiterVibe = NewRateLimiter(10, 1000)  # 10 requests per second

# Check if operation is allowed
sus allowed lit = rl.Allow()
bestie allowed {
    # Perform operation
    vibez.spill("Operation allowed")
} norly {
    vibez.spill("Rate limit exceeded")
}
```

### Network Interfaces

```cursed
# Get all network interfaces
sus interfaces []InterfaceVibe = Interfaces()

bestie i := 0; i < interfaces.length(); i++ {
    sus intf InterfaceVibe = interfaces[i]
    vibez.spill("Interface: " + intf.Name)
    vibez.spill("  Index: " + intf.Index.(tea))
    vibez.spill("  MTU: " + intf.MTU.(tea))
    vibez.spill("  Hardware: " + intf.HardwareAddr)
    
    # Get interface addresses
    sus addrs []tea = intf.Addrs()
    bestie j := 0; j < addrs.length(); j++ {
        vibez.spill("  Address: " + addrs[j])
    }
}

# Get specific interface
sus eth0 InterfaceVibe = InterfaceByName("eth0")
```

### IPv6 Support

```cursed
# IPv6 functions
sus ipv6_enabled lit = IsIPv6Enabled()
sus prefer_ipv6 lit = PreferIPv6()
SetPreferIPv6(based)

# IPv6 addresses
sus ipv6_addrs []IPVibe = IPv6InterfaceAddrs()
bestie i := 0; i < ipv6_addrs.length(); i++ {
    vibez.spill("IPv6 Address: " + ipv6_addrs[i].String())
}
```

## Advanced Usage Examples

### TCP Server with Connection Pool

```cursed
yeet "vibe_net"

# Create connection pool for handling multiple clients
sus pool ConnPoolVibe = NewConnPool("tcp", "client_backend:8080", 50)

# Create TCP server
sus listener TCPListenerVibe = Listen("tcp", "0.0.0.0:8080")
vibez.spill("Server listening on " + listener.Addr())

# Handle client connections
slay handle_client(client_conn ConnVibe) {
    # Read client request
    sus request tea = client_conn.Read(1024)
    vibez.spill("Client request: " + request)
    
    # Get backend connection from pool
    sus backend_conn ConnVibe = pool.Get()
    
    # Forward request to backend
    backend_conn.Write(request)
    sus response tea = backend_conn.Read(1024)
    
    # Send response to client
    client_conn.Write(response)
    
    # Return connections
    pool.Put(backend_conn)
    client_conn.Close()
}

# Accept client connections
bestie based {
    sus client_conn ConnVibe = listener.Accept()
    yolo handle_client(client_conn)  # Handle in goroutine
}
```

### HTTP Client with Circuit Breaker

```cursed
yeet "vibe_net"

# Create circuit breaker for API calls
sus cb CircuitBreakerVibe = NewCircuitBreaker(5, 30000)

slay make_api_request(endpoint tea) tea {
    # Execute API call through circuit breaker
    sus result tea = cb.Execute(endpoint)
    
    bestie result.contains("success") {
        # Make actual HTTP request
        sus response tea = http_get("https://api.example.com/" + endpoint)
        damn response
    }
    
    damn "Circuit breaker open - request failed"
}

# Use API with fault tolerance
sus user_data tea = make_api_request("users/123")
vibez.spill("User data: " + user_data)
```

### WebSocket Client with Rate Limiting

```cursed
yeet "vibe_net"

# Create rate limiter for WebSocket messages
sus rl RateLimiterVibe = NewRateLimiter(10, 1000)

# Connect to WebSocket server
sus conn ConnVibe = Dial("tcp", "localhost:8080")
sus ws_conn WebSocketConnVibe = NewWebSocketConn(conn, "ws")

# Send messages with rate limiting
slay send_message(message tea) lit {
    bestie rl.Allow() {
        damn ws_conn.WriteMessage(1, message)
    }
    vibez.spill("Rate limit exceeded - message dropped")
    damn cap
}

# Example usage
send_message("Hello WebSocket")
send_message("Another message")
```

### DNS Resolution with Caching

```cursed
yeet "vibe_net"

# DNS resolver with custom configuration
sus resolver DNSResolverVibe = NewDNSResolver()

# Resolve multiple record types
slay resolve_domain(domain tea) {
    vibez.spill("Resolving " + domain + ":")
    
    # A records
    sus ips []IPVibe = resolver.LookupIP(domain)
    bestie i := 0; i < ips.length(); i++ {
        vibez.spill("  A: " + ips[i].String())
    }
    
    # MX records
    sus mx_records []MXVibe = resolver.LookupMX(domain)
    bestie i := 0; i < mx_records.length(); i++ {
        vibez.spill("  MX: " + mx_records[i].Host + " (priority: " + mx_records[i].Pref.(tea) + ")")
    }
    
    # NS records
    sus ns_records []NSVibe = resolver.LookupNS(domain)
    bestie i := 0; i < ns_records.length(); i++ {
        vibez.spill("  NS: " + ns_records[i].Host)
    }
    
    # TXT records
    sus txt_records []tea = resolver.LookupTXT(domain)
    bestie i := 0; i < txt_records.length(); i++ {
        vibez.spill("  TXT: " + txt_records[i])
    }
}

# Resolve domains
resolve_domain("google.com")
resolve_domain("github.com")
```

## Legacy Compatibility

The module provides full backward compatibility with the existing vibe_net API:

```cursed
# Legacy TCP functions
sus socket normie = tcp_create_socket()
sus connection tea = tcp_connect("localhost", 8080)
sus listen_result tea = tcp_listen(9090, 10)
sus client tea = tcp_accept(socket)
sus send_success lit = tcp_send(socket, "Hello")
sus received tea = tcp_receive(socket, 1024)
tcp_close(socket)

# Legacy UDP functions
sus udp_socket normie = udp_create_socket()
udp_bind(udp_socket, "0.0.0.0", 8888)
udp_send(udp_socket, "Hello UDP", "127.0.0.1", 8888)
sus udp_data tea = udp_receive(udp_socket, 512)
udp_close(udp_socket)

# Legacy DNS functions
sus ip tea = dns_resolve("google.com")
sus hostname tea = dns_reverse_lookup("8.8.8.8")

# Legacy WebSocket functions
sus ws_id normie = websocket_create()
websocket_connect(ws_id, "ws://localhost:8080")
websocket_send_text(ws_id, "Hello")
sus message tea = websocket_receive(ws_id)
websocket_close(ws_id, 1000, "Normal closure")

# Legacy HTTP functions
sus get_response tea = http_get("http://api.example.com")
sus post_response tea = http_post("http://api.example.com", "{\"data\":\"test\"}", "application/json")
sus put_response tea = http_put("http://api.example.com/1", "{\"data\":\"updated\"}", "application/json")
sus delete_response tea = http_delete("http://api.example.com/1")

# Legacy utility functions
sus local_ip tea = get_local_ip()
sus interfaces tea = get_network_interfaces()
sus ping_result lit = ping("8.8.8.8", 5000)
sus port_open lit = port_scan("127.0.0.1", 80)
```

## Error Handling

Comprehensive error handling with validation functions:

```cursed
# IP address validation
sus valid_ip lit = is_valid_ip("192.168.1.1")
bestie !valid_ip {
    vibez.spill("Invalid IP address")
}

# Port validation
sus valid_port lit = is_valid_port(8080)
bestie !valid_port {
    vibez.spill("Invalid port number")
}

# Network error messages
sus error_msg tea = network_error_message(1)  # "Connection refused"
vibez.spill("Network error: " + error_msg)

# Socket configuration
sus timeout_set lit = set_socket_timeout(socket, 5000)
sus buffer_set lit = set_socket_buffer_size(socket, 16384)
sus reuse_enabled lit = enable_socket_reuse(socket)
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
test_both_modes() {
    local program=$1
    cargo run --bin cursed "$program" > interp_output.txt
    cargo run --bin cursed -- compile "$program"
    ./"$(basename "$program" .csd)" > comp_output.txt
    diff interp_output.txt comp_output.txt
}
test_both_modes "stdlib/vibe_net/test_vibe_net.csd"
```

## Performance Considerations

- **Connection Pooling**: Use connection pools for high-concurrency applications
- **Circuit Breakers**: Implement fault tolerance to prevent cascade failures
- **Rate Limiting**: Control traffic to prevent resource exhaustion
- **Socket Configuration**: Tune buffer sizes and timeouts for your workload
- **IPv6**: Enable IPv6 for modern network stack performance
- **Keep-Alive**: Use TCP keep-alive for long-lived connections

## Security Features

- **Input Validation**: All network parameters are validated
- **Port Range Validation**: Ports restricted to valid range (1-65535)
- **IP Address Validation**: Proper IP address format checking
- **Protocol Validation**: WebSocket and HTTP protocol validation
- **Error Handling**: Prevents information leakage through errors
- **Secure Defaults**: Safe default configurations for all components

## Implementation Notes

- **Pure CURSED**: No FFI dependencies for maximum portability
- **Specification Compliant**: Follows vibe_net specification exactly
- **Type Safety**: Uses CURSED native types throughout
- **Memory Safe**: Proper resource management and cleanup
- **Cross-Platform**: Consistent behavior across all platforms
- **Production Ready**: Suitable for enterprise networking applications

This module provides a complete, production-ready networking foundation for CURSED applications, from simple socket communication to advanced distributed system patterns with fault tolerance and performance optimization.
