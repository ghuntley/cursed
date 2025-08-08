# CURSED Networking Standard Library (networkz)

The `networkz` module provides basic networking functionality for CURSED applications, including HTTP client operations, TCP socket management, URL parsing, and network utility functions.

**Status**: Basic implementation complete and working. This module demonstrates core networking patterns in pure CURSED without external dependencies.

## Features

### 🌐 HTTP Client Operations
- **HTTP Methods**: GET, POST, PUT, DELETE, PATCH support
- **Request Management**: Custom headers, body content, timeouts
- **Response Handling**: Status codes, headers, body content, error handling
- **Content Types**: JSON, form data, plain text support
- **Error Handling**: Comprehensive timeout and error management

### 🔌 TCP Socket Operations  
- **Connection Management**: Connect, send, receive, close operations
- **Error Handling**: Connection timeouts, refused connections
- **Data Transfer**: Reliable data sending and receiving
- **Status Monitoring**: Connection state tracking

### 🔗 URL Parsing and Validation
- **Full URL Parsing**: Scheme, host, port, path, query, fragment extraction
- **URL Building**: Reconstruct URLs from components
- **Validation**: Comprehensive URL format validation
- **Multiple Schemes**: HTTP, HTTPS, FTP, FTPS support

### 🛠 Network Utilities
- **IP Address Validation**: IPv4 address format checking
- **Hostname Validation**: Domain name format validation
- **Port Management**: Port range validation, well-known port detection
- **Protocol Mapping**: Default ports for common protocols

## Installation

```cursed
yeet "networkz"
```

## Quick Start Guide

### Basic HTTP Requests

```cursed
yeet "networkz"

fr fr Simple GET request
sus response HTTPResponse = http_get("https://api.example.com/users")
lowkey http_is_success(response) {
    vibez.spill("Success:", response.body)
} highkey {
    vibez.spill("Error:", response.error)
}

fr fr POST request with data
sus post_resp HTTPResponse = http_post("https://api.example.com/users", "name=John&email=john@example.com")
vibez.spill("Status:", http_status_text(post_resp.status_code))

fr fr JSON POST request
sus json_resp HTTPResponse = http_post_json("https://api.example.com/users", "{\"name\":\"John\",\"email\":\"john@example.com\"}")
```

### Custom HTTP Requests

```cursed
fr fr Create custom request with headers
sus request HTTPRequest = http_request_new("GET", "https://api.example.com/protected")
http_request_add_header(&request, "Authorization", "Bearer your-token-here")
http_request_add_header(&request, "User-Agent", "MyApp/1.0")
http_request_set_timeout(&request, 10000)  fr fr 10 second timeout

sus response HTTPResponse = http_send_request(request)
```

### TCP Socket Communication

```cursed
fr fr Connect to a TCP server
sus conn TCPConnection = tcp_connect("localhost", 8080)
lowkey tcp_is_connected(conn) {
    fr fr Send data
    sus bytes_sent normie = tcp_send(&conn, "Hello, Server!")
    vibez.spill("Sent", bytes_sent, "bytes")
    
    fr fr Receive response
    sus response tea = tcp_receive(&conn, 1024)
    vibez.spill("Received:", response)
    
    fr fr Close connection
    tcp_close(&conn)
} highkey {
    vibez.spill("Connection failed:", conn.error)
}
```

### URL Parsing

```cursed
fr fr Parse a complex URL
sus url_parts URLParts = parse_url("https://api.example.com:8443/v1/users?active=true&role=admin#results")

lowkey url_parts.is_valid {
    vibez.spill("Scheme:", url_parts.scheme)      fr fr "https"
    vibez.spill("Host:", url_parts.host)          fr fr "api.example.com"  
    vibez.spill("Port:", url_parts.port)          fr fr 8443
    vibez.spill("Path:", url_parts.path)          fr fr "/v1/users"
    vibez.spill("Query:", url_parts.query)        fr fr "active=true&role=admin"
    vibez.spill("Fragment:", url_parts.fragment)  fr fr "results"
} highkey {
    vibez.spill("Invalid URL:", url_parts.error)
}

fr fr Rebuild URL from parts
sus rebuilt_url tea = build_url(url_parts)
vibez.spill("Rebuilt:", rebuilt_url)
```

## API Reference

### HTTP Client Functions

#### `http_request_new(method tea, url tea) HTTPRequest`
Create a new HTTP request with the specified method and URL.

#### `http_request_add_header(request *HTTPRequest, key tea, value tea) lit`
Add a custom header to the HTTP request.

#### `http_request_set_body(request *HTTPRequest, body tea, content_type tea) lit`
Set the request body and content type.

#### `http_request_set_timeout(request *HTTPRequest, timeout_ms normie) lit`
Set the request timeout in milliseconds.

#### `http_get(url tea) HTTPResponse`
Perform a GET request to the specified URL.

#### `http_post(url tea, data tea) HTTPResponse`
Perform a POST request with form data.

#### `http_post_json(url tea, json_data tea) HTTPResponse`
Perform a POST request with JSON data.

#### `http_put(url tea, data tea) HTTPResponse`
Perform a PUT request with data.

#### `http_delete(url tea) HTTPResponse`
Perform a DELETE request.

#### `http_patch(url tea, data tea) HTTPResponse`
Perform a PATCH request with JSON data.

#### `http_send_request(request HTTPRequest) HTTPResponse`
Send a custom HTTP request.

### HTTP Utility Functions

#### `http_is_success(response HTTPResponse) lit`
Check if the response indicates success (2xx status code).

#### `http_is_client_error(response HTTPResponse) lit`
Check if the response indicates a client error (4xx status code).

#### `http_is_server_error(response HTTPResponse) lit`
Check if the response indicates a server error (5xx status code).

#### `http_has_error(response HTTPResponse) lit`
Check if the response has any error condition.

#### `http_status_text(status_code normie) tea`
Get the text description for an HTTP status code.

#### `http_get_header(response HTTPResponse, header_name tea) tea`
Extract a specific header value from the response.

#### `http_get_content_type(response HTTPResponse) tea`
Get the content type from the response.

#### `http_get_content_length(response HTTPResponse) normie`
Get the content length from the response.

### TCP Socket Functions

#### `tcp_connect(host tea, port normie) TCPConnection`
Create a TCP connection to the specified host and port.

#### `tcp_send(conn *TCPConnection, data tea) normie`
Send data over the TCP connection. Returns bytes sent or -1 on error.

#### `tcp_receive(conn *TCPConnection, buffer_size normie) tea`
Receive data from the TCP connection up to buffer_size bytes.

#### `tcp_close(conn *TCPConnection) lit`
Close the TCP connection.

#### `tcp_is_connected(conn TCPConnection) lit`
Check if the TCP connection is active.

### URL Functions

#### `parse_url(url tea) URLParts`
Parse a URL into its component parts.

#### `is_valid_url(url tea) lit`
Check if a URL is valid.

#### `is_valid_scheme(scheme tea) lit`
Check if a URL scheme is supported.

#### `build_url(parts URLParts) tea`
Build a URL from its component parts.

### Network Utility Functions

#### `is_valid_ip(ip tea) lit`
Validate an IPv4 address format.

#### `is_valid_host(host tea) lit`
Validate a hostname or IP address.

#### `is_valid_port(port normie) lit`
Check if a port number is in valid range (1-65535).

#### `is_well_known_port(port normie) lit`
Check if a port is in the well-known range (1-1023).

#### `get_default_port(scheme tea) normie`
Get the default port for a URL scheme.

#### `get_scheme_from_port(port normie) tea`
Get the likely scheme for a port number.

## Data Structures

### HTTPRequest
```cursed
squad HTTPRequest {
    spill method tea          fr fr HTTP method (GET, POST, etc.)
    spill url tea            fr fr Request URL
    spill headers tea        fr fr Custom headers
    spill body tea           fr fr Request body
    spill content_type tea   fr fr Content type
    spill timeout_ms normie  fr fr Timeout in milliseconds
    spill user_agent tea     fr fr User agent string
}
```

### HTTPResponse
```cursed
squad HTTPResponse {
    spill status_code normie      fr fr HTTP status code
    spill headers tea            fr fr Response headers
    spill body tea               fr fr Response body
    spill error tea              fr fr Error message
    spill response_time_ms normie fr fr Response time
}
```

### TCPConnection
```cursed
squad TCPConnection {
    spill socket_id normie    fr fr Socket identifier
    spill host tea           fr fr Connected host
    spill port normie        fr fr Connected port
    spill is_connected lit   fr fr Connection status
    spill error tea          fr fr Error message
}
```

### URLParts
```cursed
squad URLParts {
    spill scheme tea     fr fr URL scheme (http, https)
    spill host tea       fr fr Hostname or IP
    spill port normie    fr fr Port number
    spill path tea       fr fr URL path
    spill query tea      fr fr Query string
    spill fragment tea   fr fr URL fragment
    spill is_valid lit   fr fr Validation status
    spill error tea      fr fr Error message
}
```

## Error Handling

The networking module provides comprehensive error handling:

### HTTP Errors
- **Connection Errors**: Network timeouts, DNS resolution failures
- **HTTP Errors**: 4xx client errors, 5xx server errors
- **Validation Errors**: Invalid URLs, malformed requests

### TCP Errors
- **Connection Failures**: Host unreachable, connection refused
- **Data Transfer Errors**: Send/receive failures
- **Validation Errors**: Invalid hosts, ports

### URL Parsing Errors
- **Format Errors**: Missing scheme, invalid characters
- **Component Errors**: Invalid ports, empty hosts
- **Scheme Errors**: Unsupported protocols

## Examples

### Complete HTTP Client Example

```cursed
yeet "networkz"

fr fr API client with error handling
slay make_api_request(endpoint tea, data tea) HTTPResponse {
    sus request HTTPRequest = http_request_new("POST", "https://api.example.com" + endpoint)
    http_request_add_header(&request, "Content-Type", "application/json")
    http_request_add_header(&request, "Authorization", "Bearer your-token")
    http_request_set_body(&request, data, "application/json")
    http_request_set_timeout(&request, 15000)
    
    damn http_send_request(request)
}

sus response HTTPResponse = make_api_request("/users", "{\"name\":\"John\",\"email\":\"john@example.com\"}")

lowkey http_is_success(response) {
    vibez.spill("User created successfully!")
    vibez.spill("Response:", response.body)
} highkey http_is_client_error(response) {
    vibez.spill("Client error:", http_status_text(response.status_code))
} highkey http_is_server_error(response) {
    vibez.spill("Server error:", http_status_text(response.status_code))
} highkey {
    vibez.spill("Network error:", response.error)
}
```

### TCP Server Communication

```cursed
yeet "networkz"

fr fr Simple TCP client
slay communicate_with_server(host tea, port normie, message tea) tea {
    sus conn TCPConnection = tcp_connect(host, port)
    
    check !tcp_is_connected(conn) {
        damn "Connection failed: " + conn.error
    }
    
    sus bytes_sent normie = tcp_send(&conn, message)
    check bytes_sent == -1 {
        tcp_close(&conn)
        damn "Send failed"
    }
    
    sus response tea = tcp_receive(&conn, 4096)
    tcp_close(&conn)
    
    damn response
}

sus server_response tea = communicate_with_server("localhost", 8080, "Hello Server!")
vibez.spill("Server replied:", server_response)
```

### URL Manipulation

```cursed
yeet "networkz"

fr fr URL builder and parser
slay build_api_url(base_url tea, endpoint tea, params []tea) tea {
    sus base URLParts = parse_url(base_url)
    check !base.is_valid {
        damn ""
    }
    
    base.path = endpoint
    
    lowkey len(params) > 0 {
        sus query tea = ""
        sus i normie = 0
        bestie i < len(params) {
            lowkey i > 0 {
                query = query + "&"
            }
            query = query + params[i]
            i = i + 1
        }
        base.query = query
    }
    
    damn build_url(base)
}

sus api_url tea = build_api_url("https://api.example.com", "/users", ["active=true", "limit=10"])
vibez.spill("API URL:", api_url)  fr fr "https://api.example.com/users?active=true&limit=10"
```

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed stdlib/networkz/test_networkz.csd
```

The test suite covers:
- HTTP client operations with various methods
- TCP socket connection and data transfer
- URL parsing and validation with edge cases
- Network utility functions
- Error handling scenarios
- Integration testing between components

## Performance Considerations

- **Connection Reuse**: For multiple requests to the same host, consider implementing connection pooling
- **Timeouts**: Set appropriate timeouts based on your application needs
- **Buffer Sizes**: Choose TCP receive buffer sizes based on expected data volumes
- **Error Handling**: Always check return values and error conditions

## Security Notes

- **HTTPS**: Use HTTPS for sensitive data transmission
- **Input Validation**: Always validate URLs and hostnames before use
- **Timeouts**: Set reasonable timeouts to prevent hanging connections
- **Headers**: Be careful with custom headers to avoid injection attacks

## Contributing

This module follows CURSED stdlib conventions:
- Pure CURSED implementation without external dependencies
- Comprehensive error handling with meaningful messages
- Extensive test coverage with edge cases
- Clear documentation with examples
- Performance-conscious design

## License

This module is part of the CURSED standard library and follows the same license terms.
