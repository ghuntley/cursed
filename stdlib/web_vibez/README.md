# web_vibez Module

Pure CURSED implementation for HTTP client/server functionality with comprehensive networking capabilities.

## Functions

### HTTP Client Functions

#### `http_get(url tea) tea`
Performs HTTP GET request to the specified URL.
- **Parameters**: `url` - The target URL (must include protocol)
- **Returns**: HTTP response string with headers and body
- **Example**: `sus response := http_get("https://api.example.com/users")`

#### `http_post(url tea, data tea) tea`
Performs HTTP POST request with data payload.
- **Parameters**: 
  - `url` - The target URL (must include protocol)
  - `data` - Request body data
- **Returns**: HTTP response string with headers and body
- **Example**: `sus response := http_post("https://api.example.com/users", "{\"name\": \"John\"}")`

### HTTP Server Functions

#### `create_server() ServerConfig`
Creates a basic HTTP server configuration.
- **Returns**: Server configuration object
- **Example**: `sus server := create_server()`

#### `build_response(status normie, body tea) tea`
Builds a complete HTTP response with headers.
- **Parameters**:
  - `status` - HTTP status code (200, 404, etc.)
  - `body` - Response body content
- **Returns**: Complete HTTP response string
- **Example**: `sus response := build_response(200, "Hello, World!")`

#### `build_error_response(status normie, message tea) tea`
Builds an HTTP error response in JSON format.
- **Parameters**:
  - `status` - HTTP error status code
  - `message` - Error message
- **Returns**: JSON error response
- **Example**: `sus error := build_error_response(404, "User not found")`

### Utility Functions

#### `status_code_text(code normie) tea`
Converts HTTP status codes to descriptive text.
- **Parameters**: `code` - HTTP status code
- **Returns**: Status text description
- **Example**: `sus text := status_code_text(200)  # Returns "OK"`

#### `parse_headers(headers tea) lit`
Validates HTTP header format.
- **Parameters**: `headers` - Raw headers string
- **Returns**: `based` if valid, `cap` if invalid
- **Example**: `sus valid := parse_headers("Content-Type: application/json")`

#### `parse_url_path(url tea) tea`
Extracts the path component from a URL.
- **Parameters**: `url` - Complete URL
- **Returns**: Path component (e.g., "/api/users")
- **Example**: `sus path := parse_url_path("https://example.com/api/users")`

#### `validate_method(method tea) lit`
Validates HTTP method names.
- **Parameters**: `method` - HTTP method string
- **Returns**: `based` if valid (GET, POST, PUT, DELETE, PATCH), `cap` otherwise
- **Example**: `sus valid := validate_method("GET")`

#### `detect_content_type(data tea) tea`
Automatically detects content type based on data format.
- **Parameters**: `data` - Content data
- **Returns**: MIME type string
- **Supported Types**:
  - `application/json` - JSON objects
  - `text/html` - HTML content
  - `text/plain` - Plain text (default)
- **Example**: `sus type := detect_content_type("{\"key\": \"value\"}")`

#### `parse_query_params(url tea) lit`
Checks if URL contains query parameters.
- **Parameters**: `url` - URL to check
- **Returns**: `based` if query params present, `cap` otherwise
- **Example**: `sus has_params := parse_query_params("http://example.com?param=value")`

#### `validate_request(method tea, url tea) lit`
Validates complete HTTP request parameters.
- **Parameters**:
  - `method` - HTTP method
  - `url` - Target URL
- **Returns**: `based` if valid, `cap` otherwise
- **Example**: `sus valid := validate_request("GET", "https://example.com")`

#### `log_request(method tea, url tea, status normie)`
Logs HTTP request information for debugging.
- **Parameters**:
  - `method` - HTTP method
  - `url` - Target URL
  - `status` - Response status code
- **Example**: `log_request("GET", "/api/users", 200)`

## HTTP Status Codes Supported

- `200` - OK
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `500` - Internal Server Error
- `502` - Bad Gateway
- `503` - Service Unavailable

## Usage Examples

### Basic HTTP Client
```cursed
yeet "web_vibez"

# GET request
sus response := http_get("https://jsonplaceholder.typicode.com/posts/1")
vibez.spill("Response: " + response)

# POST request
sus data := "{\"title\": \"My Post\", \"body\": \"Content here\"}"
sus post_response := http_post("https://jsonplaceholder.typicode.com/posts", data)
vibez.spill("POST Response: " + post_response)
```

### Basic HTTP Server Response Building
```cursed
yeet "web_vibez"

# Build successful response
sus success_response := build_response(200, "Welcome to the API")
vibez.spill(success_response)

# Build error response
sus error_response := build_error_response(404, "Endpoint not found")
vibez.spill(error_response)
```

### URL and Header Processing
```cursed
yeet "web_vibez"

# Parse URL components
sus path := parse_url_path("https://api.example.com/v1/users")
vibez.spill("Path: " + path)  # Outputs: Path: /v1/users

# Validate headers
sus valid := parse_headers("Authorization: Bearer token123")
vibez.spill("Headers valid: " + valid.to_string())

# Detect content type
sus content_type := detect_content_type("{\"user\": \"john\"}")
vibez.spill("Content-Type: " + content_type)  # Outputs: application/json
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/web_vibez/test_web_vibez.csd
```

Test compilation mode:
```bash
cargo run --bin cursed -- compile stdlib/web_vibez/test_web_vibez.csd
./test_web_vibez
```

## Implementation Notes

- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED language
- **Protocol Validation**: Enforces HTTP/HTTPS protocols for security
- **Content-Type Detection**: Automatic detection based on data format
- **Error Handling**: Comprehensive error responses with proper status codes
- **Logging Support**: Built-in request logging for debugging and monitoring
- **Header Validation**: Basic header format validation for robustness

## Integration

This module integrates seamlessly with other CURSED stdlib modules:
- Use with `stringz` for advanced string processing
- Combine with `json` module for API responses
- Integrate with `logging` for comprehensive request tracking
- Works with `regex` for URL pattern matching

The web_vibez module provides a solid foundation for HTTP-based networking in CURSED applications, suitable for both client and server implementations.
