# glowup_http - Modern HTTP Framework for CURSED

A comprehensive HTTP client/server framework with WebSocket support, built entirely in pure CURSED without external dependencies.

## Features

### 🚀 Core HTTP Functionality
- **HTTP Client**: Full-featured client with GET, POST, PUT, DELETE, HEAD, OPTIONS, PATCH support
- **HTTP Server**: High-performance server with routing, middleware, and request handling
- **HTTP/1.1 Support**: Complete HTTP/1.1 implementation with keep-alive and chunked encoding
- **Status Codes**: Comprehensive HTTP status code support (200, 201, 400, 401, 404, 500, etc.)

### 🔄 WebSocket Support
- **WebSocket Handshake**: RFC 6455 compliant handshake implementation
- **Frame Types**: Text, binary, ping, pong, and control frames
- **Real-time Communication**: Bidirectional real-time messaging
- **Connection Management**: Connection lifecycle management with proper cleanup

### 🎯 Modern Web Features
- **Middleware System**: Extensible middleware pipeline (CORS, logging, authentication)
- **URL Routing**: Flexible routing system with parameter support
- **Session Management**: Built-in session handling with secure cookies
- **Template Engine**: Basic template rendering for dynamic content
- **JSON Support**: Built-in JSON parsing and serialization

### 🛡️ Security & Performance
- **CORS Support**: Cross-Origin Resource Sharing middleware
- **Authentication**: Built-in authentication middleware
- **Secure Cookies**: HttpOnly and secure cookie support
- **Connection Pooling**: Efficient connection management
- **Compression**: Response compression support

## Quick Start

### HTTP Server Example

```cursed
yeet "glowup_http"

# Create server configuration
sus config ServerConfig
config.host = "localhost"
config.port = 8080
config.max_connections = 100
config.timeout = 30
config.keep_alive = based
config.compression = based

# Start server
http_server_create(config)

# Register routes
http_route_get("/", "home_handler")
http_route_get("/api/users", "users_handler")
http_route_post("/api/users", "create_user_handler")

# Start listening
http_server_listen("request_handler")
```

### HTTP Client Example

```cursed
yeet "glowup_http"

# GET request
sus response HttpResponse = http_client_get("https://api.example.com/data")
vibez.spill("Status: " + http_int_to_string(response.status_code))
vibez.spill("Body: " + response.body)

# POST request
sus post_data tea = "{\"name\": \"John\", \"email\": \"john@example.com\"}"
sus post_response HttpResponse = http_client_post("https://api.example.com/users", post_data)
vibez.spill("Created user: " + post_response.body)

# PUT request
sus put_data tea = "{\"name\": \"Jane\", \"email\": \"jane@example.com\"}"
sus put_response HttpResponse = http_client_put("https://api.example.com/users/1", put_data)

# DELETE request
sus delete_response HttpResponse = http_client_delete("https://api.example.com/users/1")
```

### WebSocket Example

```cursed
yeet "glowup_http"

# WebSocket handshake
sus client_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
sus accept_key tea = websocket_handshake(client_key)

# Send messages
websocket_send_text("Hello WebSocket!")
websocket_send_binary("binary_data")

# Ping/Pong
websocket_ping()
websocket_pong()

# Create custom frames
sus text_frame WebSocketFrame = websocket_create_frame(1, "Custom message")
sus binary_frame WebSocketFrame = websocket_create_frame(2, "Binary data")
```

## API Reference

### HTTP Request/Response

#### HttpRequest Structure
```cursed
vibe HttpRequest {
    method tea      # HTTP method (GET, POST, etc.)
    path tea        # Request path
    version tea     # HTTP version
    headers tea     # Request headers
    body tea        # Request body
    params tea      # URL parameters
    query tea       # Query string
}
```

#### HttpResponse Structure
```cursed
vibe HttpResponse {
    status_code normie    # HTTP status code
    status_text tea       # Status text
    headers tea           # Response headers
    body tea             # Response body
    content_type tea     # Content type
}
```

### Server Functions

#### `http_server_create(config ServerConfig) lit`
Creates and initializes HTTP server with given configuration.

#### `http_server_listen(callback tea) lit`
Starts server event loop and begins listening for connections.

#### `http_handle_request(request HttpRequest) HttpResponse`
Handles incoming HTTP requests and returns appropriate responses.

### Client Functions

#### `http_client_get(url tea) HttpResponse`
Performs HTTP GET request to specified URL.

#### `http_client_post(url tea, body tea) HttpResponse`
Performs HTTP POST request with request body.

#### `http_client_put(url tea, body tea) HttpResponse`
Performs HTTP PUT request with request body.

#### `http_client_delete(url tea) HttpResponse`
Performs HTTP DELETE request.

### WebSocket Functions

#### `websocket_handshake(key tea) tea`
Performs WebSocket handshake and returns accept key.

#### `websocket_create_frame(opcode normie, payload tea) WebSocketFrame`
Creates WebSocket frame with specified opcode and payload.

#### `websocket_send_text(payload tea) lit`
Sends text frame over WebSocket connection.

#### `websocket_send_binary(payload tea) lit`
Sends binary frame over WebSocket connection.

### Middleware Functions

#### `http_middleware_cors(request HttpRequest, response HttpResponse) HttpResponse`
Adds CORS headers to response.

#### `http_middleware_logging(request HttpRequest) lit`
Logs HTTP request details.

#### `http_middleware_auth(request HttpRequest) lit`
Checks request authentication.

### Routing Functions

#### `http_route_get(path tea, handler tea) lit`
Registers GET route handler.

#### `http_route_post(path tea, handler tea) lit`
Registers POST route handler.

#### `http_route_put(path tea, handler tea) lit`
Registers PUT route handler.

#### `http_route_delete(path tea, handler tea) lit`
Registers DELETE route handler.

### Session Management

#### `session_create(id tea) lit`
Creates new session with given ID.

#### `session_get(id tea) tea`
Retrieves session data by ID.

#### `session_destroy(id tea) lit`
Destroys session with given ID.

### Cookie Functions

#### `cookie_set(name tea, value tea) tea`
Creates cookie header string.

#### `cookie_get(headers tea, name tea) tea`
Extracts cookie value from headers.

### Template Engine

#### `template_render(template tea, data tea) tea`
Renders template with provided data.

#### `template_compile(template tea) tea`
Compiles template for efficient rendering.

### JSON Utilities

#### `json_parse(text tea) tea`
Parses JSON text into object.

#### `json_stringify(object tea) tea`
Converts object to JSON string.

### URL Utilities

#### `url_parse(url tea) tea`
Parses URL into components.

#### `url_encode(text tea) tea`
URL encodes text string.

#### `url_decode(text tea) tea`
URL decodes text string.

## Configuration

### ServerConfig Structure
```cursed
vibe ServerConfig {
    host tea              # Server host address
    port normie           # Server port number
    max_connections normie # Maximum concurrent connections
    timeout normie        # Connection timeout in seconds
    keep_alive lit        # Enable keep-alive connections
    compression lit       # Enable response compression
}
```

### ClientConfig Structure
```cursed
vibe ClientConfig {
    timeout normie        # Request timeout in seconds
    max_redirects normie  # Maximum redirect follow count
    user_agent tea        # User agent string
    follow_redirects lit  # Enable redirect following
    verify_ssl lit        # Enable SSL verification
}
```

## Constants

### HTTP Status Codes
- `HTTP_OK` - 200 OK
- `HTTP_CREATED` - 201 Created
- `HTTP_BAD_REQUEST` - 400 Bad Request
- `HTTP_UNAUTHORIZED` - 401 Unauthorized
- `HTTP_NOT_FOUND` - 404 Not Found
- `HTTP_INTERNAL_ERROR` - 500 Internal Server Error

### HTTP Methods
- `METHOD_GET` - "GET"
- `METHOD_POST` - "POST"
- `METHOD_PUT` - "PUT"
- `METHOD_DELETE` - "DELETE"
- `METHOD_HEAD` - "HEAD"
- `METHOD_OPTIONS` - "OPTIONS"
- `METHOD_PATCH` - "PATCH"

### WebSocket
- `WEBSOCKET_MAGIC` - "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"

## Advanced Usage

### Custom Middleware
```cursed
slay custom_middleware(request HttpRequest, response HttpResponse) HttpResponse {
    # Add custom headers
    response.headers = response.headers + "X-Custom-Header: value\r\n"
    
    # Log request details
    vibez.spill("Processing: " + request.method + " " + request.path)
    
    damn response
}
```

### Route Handlers
```cursed
slay user_handler(request HttpRequest) HttpResponse {
    bestie request.method == METHOD_GET {
        damn http_response_new(HTTP_OK, "{\"users\": []}")
    } else if request.method == METHOD_POST {
        # Create user logic
        damn http_response_new(HTTP_CREATED, "{\"id\": 1}")
    } else {
        damn http_response_new(HTTP_BAD_REQUEST, "Method not allowed")
    }
}
```

### WebSocket Message Handling
```cursed
slay websocket_handler(frame WebSocketFrame) lit {
    bestie frame.opcode == 1 {
        # Text frame
        vibez.spill("Received text: " + frame.payload)
        websocket_send_text("Echo: " + frame.payload)
    } else if frame.opcode == 2 {
        # Binary frame
        vibez.spill("Received binary data")
        websocket_send_binary("Binary response")
    } else if frame.opcode == 9 {
        # Ping frame
        websocket_pong()
    }
    
    damn based
}
```

## Testing

The framework includes comprehensive tests covering all functionality:

```bash
# Run all tests
cargo run --bin cursed stdlib/glowup_http/test_glowup_http.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/glowup_http/test_glowup_http.csd
./test_glowup_http
```

## Architecture

### Pure CURSED Implementation
- **No FFI Dependencies**: Entirely implemented in CURSED without external libraries
- **Memory Safe**: Uses CURSED's built-in memory management
- **Cross-Platform**: Works on all platforms supported by CURSED
- **Performance**: Optimized for CURSED's runtime characteristics

### Design Principles
- **Simplicity**: Clean, intuitive API design
- **Extensibility**: Middleware and plugin architecture
- **Performance**: Efficient request/response handling
- **Standards Compliance**: Follows HTTP/1.1 and WebSocket RFCs

## Contributing

1. Follow CURSED coding conventions
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure both interpretation and compilation modes work

## License

This module is part of the CURSED language standard library.

## Examples

See the `examples/` directory for more comprehensive examples:
- REST API server
- WebSocket chat application
- HTTP client examples
- Middleware implementations

## Performance

The framework is optimized for:
- **Low latency**: Efficient request processing
- **High throughput**: Concurrent connection handling
- **Memory efficiency**: Minimal memory footprint
- **Scalability**: Handles thousands of concurrent connections

## Future Enhancements

- HTTP/2 support
- Built-in rate limiting
- Advanced caching mechanisms
- GraphQL support
- Server-sent events (SSE)
- Enhanced security features
