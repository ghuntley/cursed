# glowup_http - CURSED HTTP Client/Server Module

A comprehensive HTTP client and server module implemented in pure CURSED without FFI dependencies. Provides essential HTTP functionality for web applications, APIs, and HTTP-based services.

## Features

### HTTP Client
- **GET, POST, PUT, DELETE** requests
- **URL parsing** with protocol, host, and path extraction
- **Header management** with utilities for common headers
- **Authentication** support (Basic Auth)
- **Request building** with proper HTTP/1.1 formatting

### HTTP Server
- **Route handling** with method and path matching
- **Request parsing** from raw HTTP requests
- **Response building** with proper status codes
- **Content-Type** utilities for common MIME types
- **Server configuration** management

### HTTP Utilities
- **Status code** validation and categorization
- **Cookie management** with security flags
- **Header parsing** and manipulation
- **Method validation** for HTTP verbs
- **Response status checking** (success, client error, server error)

## Usage Examples

### HTTP Client

```cursed
yeet "glowup_http"

# Simple GET request
sus response tea = http_get("http://api.example.com/users", "")
vibez.spill(response)

# POST request with JSON body
sus headers tea = add_header("", "Content-Type", "application/json")
sus post_response tea = http_post("http://api.example.com/users", headers, "{\"name\":\"John\",\"email\":\"john@example.com\"}")
vibez.spill(post_response)

# PUT request for updates
sus put_response tea = http_put("http://api.example.com/users/1", headers, "{\"name\":\"John Updated\"}")
vibez.spill(put_response)

# DELETE request
sus delete_response tea = http_delete("http://api.example.com/users/1", "")
vibez.spill(delete_response)
```

### HTTP Server

```cursed
yeet "glowup_http"

# Handle different routes
sus home_response tea = handle_route("GET", "/", "")
sus api_response tea = handle_route("GET", "/api/status", "")
sus echo_response tea = handle_route("POST", "/echo", "Hello World")

# Parse incoming requests
sus request tea = "GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n"
sus (method, path, body) = parse_request(request)
sus response tea = handle_route(method, path, body)
```

### URL Processing

```cursed
yeet "glowup_http"

# Parse URLs
sus (protocol, host, path) = parse_url("https://api.github.com/repos/user/project")
vibez.spill("Protocol: " + protocol)  # "https"
vibez.spill("Host: " + host)          # "api.github.com"
vibez.spill("Path: " + path)          # "/repos/user/project"
```

### Header Management

```cursed
yeet "glowup_http"

# Create and add headers
sus headers tea = create_basic_headers()
headers = add_header(headers, "Authorization", "Bearer token123")
headers = add_header(headers, "Accept", "application/json")

# Parse headers
sus (name, value) = parse_header("Content-Type: application/json")
vibez.spill("Header: " + name + " = " + value)
```

### Authentication

```cursed
yeet "glowup_http"

# Create Basic Auth header
sus auth tea = create_basic_auth("username", "password")
sus headers tea = add_header("", "Authorization", auth)
```

### Cookie Management

```cursed
yeet "glowup_http"

# Create session cookie
sus session_cookie tea = create_cookie("session_id", "abc123def456", 3600)
sus headers tea = add_header("", "Set-Cookie", session_cookie)
```

## API Reference

### HTTP Status Codes
- `status_ok() -> normie` - Returns 200
- `status_not_found() -> normie` - Returns 404
- `status_internal_error() -> normie` - Returns 500
- `status_bad_request() -> normie` - Returns 400

### HTTP Methods
- `is_valid_method(method tea) -> lit` - Validates HTTP method

### Request Building
- `build_request(method tea, path tea, headers tea, body tea) -> tea` - Builds HTTP request
- `build_response(status normie, headers tea, body tea) -> tea` - Builds HTTP response

### HTTP Client
- `http_get(url tea, headers tea) -> tea` - Performs GET request
- `http_post(url tea, headers tea, body tea) -> tea` - Performs POST request
- `http_put(url tea, headers tea, body tea) -> tea` - Performs PUT request
- `http_delete(url tea, headers tea) -> tea` - Performs DELETE request

### HTTP Server
- `handle_route(method tea, path tea, body tea) -> tea` - Handles HTTP routes
- `parse_request(request tea) -> (tea, tea, tea)` - Parses HTTP request

### URL Processing
- `parse_url(url tea) -> (tea, tea, tea)` - Parses URL into components

### Header Utilities
- `parse_header(header_line tea) -> (tea, tea)` - Parses header line
- `add_header(headers tea, name tea, value tea) -> tea` - Adds header
- `create_basic_headers() -> tea` - Creates basic headers

### Content Types
- `content_type_json() -> tea` - Returns "application/json"
- `content_type_html() -> tea` - Returns "text/html"
- `content_type_plain() -> tea` - Returns "text/plain"

### Status Checking
- `is_success_status(status normie) -> lit` - Checks if status is 2xx
- `is_client_error(status normie) -> lit` - Checks if status is 4xx
- `is_server_error(status normie) -> lit` - Checks if status is 5xx

### Authentication
- `create_basic_auth(username tea, password tea) -> tea` - Creates Basic Auth header

### Cookie Management
- `create_cookie(name tea, value tea, max_age normie) -> tea` - Creates cookie string

### Configuration
- `create_server_config(port normie, max_connections normie) -> tea` - Server config
- `create_client_config(timeout normie, max_redirects normie) -> tea` - Client config

## Implementation Notes

### Pure CURSED Implementation
This module is implemented entirely in CURSED without external FFI dependencies:
- **No network I/O**: For demonstration, client functions return mock responses
- **String processing**: All HTTP parsing and building uses CURSED string operations
- **Cross-platform**: Works consistently across all CURSED-supported platforms
- **Self-contained**: No external libraries or system dependencies

### Production Considerations
For production use, consider:
- **Network I/O**: Implement actual socket operations
- **Concurrency**: Add support for concurrent request handling
- **Security**: Implement proper TLS/SSL support
- **Performance**: Add connection pooling and keep-alive support
- **Standards**: Full HTTP/1.1 and HTTP/2 compliance

### Error Handling
- Invalid methods return `cap` from validation functions
- Malformed URLs return default values
- Missing headers return empty strings
- Unknown routes return 404 responses

## Testing

Run the test suite to verify functionality:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/glowup_http/test_glowup_http.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/glowup_http/test_glowup_http.csd
./test_glowup_http
```

## Integration Examples

### Web API Server
```cursed
yeet "glowup_http"

slay handle_api_request(request tea) tea {
    sus (method, path, body) = parse_request(request)
    
    skip method == "GET" && path == "/api/users" {
        damn build_response(200, content_type_json(), "[{\"id\":1,\"name\":\"John\"}]")
    } else skip method == "POST" && path == "/api/users" {
        damn build_response(201, content_type_json(), "{\"id\":2,\"name\":\"Created\"}")
    }
    
    damn handle_route(method, path, body)
}
```

### HTTP Client with Authentication
```cursed
yeet "glowup_http"

slay authenticated_request(url tea, token tea) tea {
    sus headers tea = create_basic_headers()
    headers = add_header(headers, "Authorization", "Bearer " + token)
    damn http_get(url, headers)
}
```

## Future Enhancements

- WebSocket support for real-time communication
- HTTP/2 protocol support
- Middleware system for request/response processing
- Request/response streaming for large payloads
- Built-in JSON serialization/deserialization
- Template engine integration for HTML responses
- Rate limiting and throttling capabilities
- Caching mechanisms for improved performance
