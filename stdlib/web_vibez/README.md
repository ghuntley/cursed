# web_vibez - CURSED HTTP Web Framework

A comprehensive, production-ready HTTP client and server framework implemented in pure CURSED. The web_vibez module provides enterprise-grade web development capabilities with zero external dependencies.

## Features

### Core HTTP Operations
- **Full HTTP Method Support**: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- **Advanced Request/Response Handling**: Complete HTTP/1.1 protocol support
- **Header Management**: Comprehensive header parsing and manipulation
- **Content Type Detection**: Automatic MIME type detection and handling
- **URL Processing**: Advanced URL parsing, encoding, and parameter extraction

### Server Infrastructure
- **Production-Ready Server**: Configurable HTTP server with port management
- **Advanced Routing System**: Pattern matching, wildcards, and parameter extraction
- **Middleware Support**: Composable middleware pipeline for request processing
- **Static File Serving**: Efficient static asset delivery with caching
- **WebSocket Support**: Protocol upgrade handling for real-time applications

### Security & Performance
- **CORS Support**: Cross-origin resource sharing configuration
- **Security Headers**: Comprehensive security header management
- **Rate Limiting**: Request throttling and abuse prevention
- **Session Management**: Secure session handling and validation
- **HTTP Compression**: Response compression for bandwidth optimization
- **Cache Control**: Flexible caching strategies for performance

### Enterprise Features
- **Health Checks**: Built-in health monitoring endpoints
- **Metrics Collection**: Performance and usage metrics
- **Request Logging**: Detailed request/response logging
- **Error Handling**: Comprehensive error response management
- **HTTP/2 Support**: Modern protocol support indication

## Quick Start

```cursed
yeet "web_vibez"

# Simple HTTP client
sus response := http_get("https://api.example.com/data")
vibez.spill(response)

# HTTP server setup
sus server := create_server(8080)
sus router := create_router()
add_route(router, "/api/users", "GET", "users_handler")

# Handle requests
sus result := handle_production_request("GET", "/api/users", "", "")
vibez.spill(result)
```

## HTTP Client API

### Basic HTTP Methods

```cursed
# GET request
sus response := http_get("https://api.example.com/users")

# POST request with data
sus post_data := "{\"name\": \"John\", \"email\": \"john@example.com\"}"
sus result := http_post("https://api.example.com/users", post_data)

# PUT request for updates
sus put_data := "{\"name\": \"Updated Name\"}"
sus update := http_put("https://api.example.com/users/1", put_data)

# DELETE request
sus delete_result := http_delete("https://api.example.com/users/1")

# PATCH request for partial updates
sus patch_data := "{\"status\": \"active\"}"
sus patch_result := http_patch("https://api.example.com/users/1", patch_data)
```

### Advanced HTTP Client

```cursed
# Generic HTTP request with custom headers
sus custom_response := http_request(
    "GET", 
    "https://api.example.com/data", 
    "", 
    "Authorization: Bearer token123\r\nAccept: application/json\r\n"
)
```

## HTTP Server API

### Server Configuration

```cursed
# Create server on specific port
sus server := create_server(3000)

# Create router for handling routes
sus router := create_router()

# Add routes with different methods
add_route(router, "/api/users", "GET", "get_users")
add_route(router, "/api/users", "POST", "create_user")
add_route(router, "/api/users/{id}", "PUT", "update_user")
add_route(router, "/api/users/{id}", "DELETE", "delete_user")
```

### Route Matching

```cursed
# Exact path matching
assert_true(match_route("/api/users", "/api/users"))

# Wildcard matching
assert_true(match_route("/api/users/123", "/api/users/*"))

# Parameter matching
assert_true(match_route("/api/users/123", "/api/users/{id}"))
```

### Request Handling

```cursed
# Production request handler
slay api_handler(method tea, path tea, body tea, headers tea) tea {
    lowkey path == "/api/users" && method == "GET" {
        sus users := "[{\"id\": 1, \"name\": \"John\"}]"
        damn build_json_response(200, users)
    } elif path == "/api/users" && method == "POST" {
        sus created := "{\"id\": 2, \"name\": \"Created\"}"
        damn build_json_response(201, created)
    } else {
        damn build_error_response(404, "Not found")
    }
}

# Handle incoming request
sus response := api_handler("GET", "/api/users", "", "")
```

## URL Processing

### URL Parsing

```cursed
# Extract path from URL
sus path := parse_url_path("https://example.com/api/users/123")
# Returns: "/api/users/123"

# Parse query parameters
sus query_string := parse_query_params("https://example.com/api?name=john&age=30")
# Returns: "name=john&age=30"

# Get specific parameter
sus name := get_query_param("https://example.com/api?name=john&age=30", "name")
# Returns: "john"
```

### URL Encoding

```cursed
# URL encode special characters
sus encoded := url_encode("hello world & more")
# Returns: "hello%20world%20%26%20more"

# URL decode
sus decoded := url_decode("hello%20world%20%26%20more")
# Returns: "hello world & more"
```

## Response Building

### Basic Responses

```cursed
# Simple response
sus response := build_response(200, "Hello, World!")

# JSON response
sus json_response := build_json_response(200, "{\"message\": \"success\"}")

# Error response
sus error := build_error_response(404, "Resource not found")
```

### Enhanced Responses

```cursed
# Response with custom headers
sus custom_headers := "X-Custom-Header: value\r\nX-API-Version: 1.0\r\n"
sus enhanced := build_response_with_headers(200, "Custom response", custom_headers)

# Response with CORS headers
sus cors_response := add_cors_headers(build_response(200, "CORS enabled"))

# Response with security headers
sus secure_response := add_security_headers(build_response(200, "Secure"))

# Response with caching
sus cached := add_cache_headers(build_response(200, "Cached content"), 3600)
```

## Middleware System

### Creating Middleware

```cursed
# Create middleware
sus auth_middleware := create_middleware("auth")

# Apply middleware to request
sus processed := apply_middleware(auth_middleware, "incoming request")
```

### Built-in Middleware

```cursed
# CORS middleware
sus cors_enabled := add_cors_headers(response)

# Security middleware
sus secure := add_security_headers(response)

# Compression middleware
sus compressed := compress_response(response, "gzip")
```

## Session Management

```cursed
# Create session
sus session := create_session("user123")

# Validate session
lowkey validate_session(session) {
    vibez.spill("Valid session")
} else {
    vibez.spill("Invalid session")
}
```

## Static File Serving

```cursed
# Serve static files
sus html_file := serve_static_file("index.html")
sus css_file := serve_static_file("styles.css")
sus js_file := serve_static_file("app.js")

# Automatic MIME type detection
sus mime_type := get_mime_type("png")  # Returns: "image/png"
```

## WebSocket Support

```cursed
# Handle WebSocket upgrade
sus ws_request := "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: upgrade\r\n"
sus ws_response := handle_websocket_upgrade(ws_request)
```

## Health and Monitoring

```cursed
# Health check endpoint
sus health := health_check()
# Returns JSON with health status

# Metrics endpoint
sus metrics := metrics_endpoint()
# Returns JSON with performance metrics
```

## Production Request Handler

```cursed
# Complete production handler
sus prod_response := handle_production_request(
    "GET",                    # HTTP method
    "/api/users",            # Request path
    "",                      # Request body
    "Accept: application/json" # Headers
)
```

## Content Type Detection

```cursed
# Automatic content type detection
sus json_type := detect_content_type("{\"key\": \"value\"}")  # "application/json"
sus html_type := detect_content_type("<html></html>")         # "text/html"
sus xml_type := detect_content_type("<?xml version=\"1.0\"?>") # "application/xml"
sus form_type := detect_content_type("name=value&age=30")     # "application/x-www-form-urlencoded"
```

## Error Handling

All functions return appropriate error messages for invalid inputs:

```cursed
# Error examples
sus error1 := http_get("")                    # "Error: Empty URL"
sus error2 := http_get("ftp://example.com")   # "Error: Invalid URL protocol"
sus error3 := http_get("http://a")            # "Error: URL too short"
```

## Rate Limiting

```cursed
# Create rate limiter
sus rate_limit := create_rate_limit(100)  # 100 requests per minute

# Check rate limit
lowkey check_rate_limit(rate_limit, "192.168.1.1") {
    vibez.spill("Request allowed")
} else {
    vibez.spill("Rate limit exceeded")
}
```

## Logging

```cursed
# Basic request logging
log_request("GET", "/api/users", 200)

# Detailed logging with client info
log_request_detailed("GET", "/api/users", 200, "Mozilla/5.0", "192.168.1.1")
```

## Testing

The module includes comprehensive tests covering all functionality:

```bash
# Run tests in interpretation mode
cargo run --bin cursed stdlib/web_vibez/test_web_vibez.💀

# Run tests in compilation mode
cargo run --bin cursed -- compile stdlib/web_vibez/test_web_vibez.💀
./test_web_vibez
```

## Performance Characteristics

### Time Complexity
- URL parsing: O(n) where n is URL length
- Header parsing: O(n) where n is header count
- Route matching: O(log n) for pattern matching
- Response building: O(n) where n is content length

### Memory Usage
- Minimal memory footprint with efficient string operations
- No dynamic memory allocation for simple operations
- Immutable string handling prevents memory leaks

## Security Features

### Built-in Security
- URL validation and sanitization
- Header format validation
- Request size limits
- Security headers (XSS protection, CSRF protection, etc.)
- Session validation and management

### Security Headers
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`

## Production Deployment

### Configuration
```cursed
# Production server setup
sus server := create_server(80)  # HTTP port
sus router := create_router()

# Add production routes
add_route(router, "/", "GET", "home_handler")
add_route(router, "/api/*", "GET", "api_handler")
add_route(router, "/health", "GET", "health_handler")
add_route(router, "/metrics", "GET", "metrics_handler")
```

### Performance Optimizations
- Efficient string operations
- Minimal memory allocation
- Fast route matching
- Header caching
- Response compression

## Compatibility

- **HTTP Standards**: Full HTTP/1.1 compliance
- **MIME Types**: Comprehensive MIME type support
- **WebSocket**: Protocol upgrade support
- **HTTP/2**: Ready for HTTP/2 implementation
- **Cross-Platform**: Works on all CURSED-supported platforms

## Dependencies

- `testz`: Testing framework for comprehensive validation
- `vibez`: Output operations for logging and debugging
- **No external dependencies**: Pure CURSED implementation

## Version

web_vibez 1.0 - Production-ready HTTP framework for CURSED

## License

Part of the CURSED programming language standard library.
