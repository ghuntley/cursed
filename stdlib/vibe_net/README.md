# vibe_net Module

Pure CURSED network operations module providing essential networking functionality for the compiler and package management system.

## Overview

The `vibe_net` module provides basic network operations including TCP connections, HTTP client functionality, hostname resolution, and network utilities. This module uses placeholder implementations that return predictable values for testing and development purposes.

## Functions

### TCP Operations

#### `tcp_connect(address tea, port normie) lit`
Establishes a TCP connection to the specified address and port.
- **Parameters**: 
  - `address`: Target IP address or hostname
  - `port`: Target port number (1-65535)
- **Returns**: `based` (true) if connection parameters are valid, `cap` (false) otherwise
- **Example**: `tcp_connect("localhost", 8080)`

#### `tcp_listen(address tea, port normie) lit`
Creates a TCP listener on the specified address and port.
- **Parameters**:
  - `address`: Local address to bind to
  - `port`: Local port number (1-65535)
- **Returns**: `based` (true) if listen parameters are valid, `cap` (false) otherwise
- **Example**: `tcp_listen("0.0.0.0", 3000)`

### HTTP Client Operations

#### `http_get(url tea) tea`
Performs an HTTP GET request to the specified URL.
- **Parameters**: 
  - `url`: Target URL for the GET request
- **Returns**: HTTP response string with headers and body
- **Example**: `http_get("http://example.com")`

#### `http_post(url tea, data tea) tea`
Performs an HTTP POST request with the provided data.
- **Parameters**:
  - `url`: Target URL for the POST request
  - `data`: Request body data
- **Returns**: HTTP response string with headers and body
- **Example**: `http_post("http://api.example.com/data", "payload")`

### Network Utilities

#### `network_available() lit`
Checks if network connectivity is available.
- **Returns**: `based` (true) if network is available, `cap` (false) otherwise
- **Example**: `network_available()`

#### `resolve_hostname(hostname tea) tea`
Resolves a hostname to an IP address.
- **Parameters**: 
  - `hostname`: Hostname to resolve
- **Returns**: IP address string or "0.0.0.0" for unknown hosts
- **Example**: `resolve_hostname("localhost")` returns `"127.0.0.1"`

#### `get_local_ip() tea`
Gets the local machine's IP address.
- **Returns**: Local IP address string
- **Example**: `get_local_ip()` returns `"192.168.1.100"`

#### `ping_host(hostname tea) lit`
Tests connectivity to a host.
- **Parameters**: 
  - `hostname`: Target hostname to ping
- **Returns**: `based` (true) if host is reachable, `cap` (false) otherwise
- **Example**: `ping_host("localhost")`

### HTTP Utilities

#### `parse_http_headers(response tea) tea`
Extracts headers from an HTTP response.
- **Parameters**: 
  - `response`: HTTP response string
- **Returns**: Parsed headers string
- **Example**: `parse_http_headers("HTTP/1.1 200 OK")`

#### `build_http_request(method tea, url tea, headers tea) tea`
Constructs an HTTP request string.
- **Parameters**:
  - `method`: HTTP method (GET, POST, etc.)
  - `url`: Request URL path
  - `headers`: Request headers
- **Returns**: Complete HTTP request string
- **Example**: `build_http_request("GET", "/api", "Host: example.com")`

## Usage Examples

```cursed
yeet "vibe_net"

# Check network availability
conditional vibe_net.network_available() {
    vibez.spill("Network is available")
} otherwise {
    vibez.spill("No network connection")
}

# Resolve hostname
sus ip tea = vibe_net.resolve_hostname("example.com")
vibez.spill("IP address: " + ip)

# Make HTTP GET request
sus response tea = vibe_net.http_get("http://example.com")
vibez.spill("Response: " + response)

# Test TCP connection
conditional vibe_net.tcp_connect("localhost", 8080) {
    vibez.spill("Can connect to localhost:8080")
} otherwise {
    vibez.spill("Cannot connect to localhost:8080")
}
```

## Package Management Integration

This module provides the networking foundation for the CURSED package manager:

- **Package Downloads**: HTTP GET operations for fetching packages
- **Registry Communication**: HTTP POST for package publishing
- **Dependency Resolution**: Hostname resolution for package registries
- **Health Checks**: Network availability and ping functionality

## Testing

Run the test suite to verify functionality:

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

- **Placeholder Implementation**: Current functions return mock data for testing purposes
- **Pure CURSED**: No FFI dependencies - entirely implemented in CURSED language
- **Error Handling**: Functions return predictable error states for invalid inputs
- **Future Enhancement**: API structure ready for real network implementation

## Development Status

- **API Complete**: All essential network functions implemented
- **Testing**: Comprehensive test coverage with both interpretation and compilation modes
- **Documentation**: Complete function documentation with examples
- **Integration Ready**: Prepared for package manager and compiler integration
