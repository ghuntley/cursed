# web_vibez (HTTP Functionality)

## Overview
`web_vibez` provides comprehensive HTTP client and server functionality for CURSED web development. This module implements HTTP protocol handling, request/response processing, status code management, and basic web server capabilities. All functions are implemented in pure CURSED without external dependencies.

## HTTP Status Code Management

### Status Code Mapping
```cursed
slay status_code_text(code normie) tea
```
Maps HTTP status codes to their standard text descriptions.

**Parameters:**
- `code normie`: The HTTP status code

**Returns:**
- `tea`: Standard status text or "Unknown Status"

**Supported Status Codes:**
- `200` → "OK"
- `201` → "Created"
- `400` → "Bad Request"
- `401` → "Unauthorized"
- `403` → "Forbidden"
- `404` → "Not Found"
- `500` → "Internal Server Error"
- `502` → "Bad Gateway"
- `503` → "Service Unavailable"

**Examples:**
```cursed
sus ok_text := status_code_text(200)      # Returns "OK"
sus not_found := status_code_text(404)    # Returns "Not Found"
sus unknown := status_code_text(999)      # Returns "Unknown Status"
```

## HTTP Header Processing

### Header Parsing
```cursed
slay parse_headers(headers tea) lit
```
Validates and parses HTTP header format.

**Parameters:**
- `headers tea`: Raw header string to validate

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

**Validation Rules:**
- Non-empty header string
- Contains colon separator for key-value pairs
- Basic format validation

**Examples:**
```cursed
sus valid := parse_headers("Content-Type: application/json")  # Returns based
sus invalid := parse_headers("invalid header format")        # Returns cap
sus empty := parse_headers("")                               # Returns cap
```

## HTTP Client Operations

### GET Request
```cursed
slay http_get(url tea) tea
```
Performs HTTP GET request to specified URL.

**Parameters:**
- `url tea`: The target URL (must include protocol)

**Returns:**
- `tea`: HTTP response or error message

**Response Format:**
```
HTTP/1.1 200 OK\r\n
Content-Type: text/html\r\n
Content-Length: 13\r\n
\r\n
Hello, World!
```

### POST Request
```cursed
slay http_post(url tea, data tea) tea
```
Performs HTTP POST request with data payload.

**Parameters:**
- `url tea`: The target URL (must include protocol)
- `data tea`: The request body data

**Returns:**
- `tea`: HTTP response or error message

**Response Format:**
```
HTTP/1.1 201 Created\r\n
Content-Type: application/json\r\n
Content-Length: [data_length]\r\n
\r\n
[data]
```

**Examples:**
```cursed
sus response := http_get("https://api.example.com/data")
sus post_result := http_post("https://api.example.com/submit", "{\"key\":\"value\"}")

# Error cases
sus error1 := http_get("")                    # Returns "Error: Empty URL"
sus error2 := http_get("invalid-url")         # Returns "Error: Invalid URL protocol"
```

## HTTP Server Infrastructure

### Server Configuration
```cursed
be_like ServerConfig = lit

slay create_server() ServerConfig
```
Creates a new HTTP server configuration.

**Returns:**
- `ServerConfig`: Server configuration object

**Examples:**
```cursed
sus server := create_server()  # Initialize server configuration
```

## URL Processing

### URL Path Extraction
```cursed
slay parse_url_path(url tea) tea
```
Extracts the path component from a URL.

**Parameters:**
- `url tea`: The full URL to parse

**Returns:**
- `tea`: The path component (defaults to "/" if not found)

**URL Parsing Logic:**
1. Split by "://" to separate protocol
2. Extract host and path components
3. Return path portion or "/" as default

**Examples:**
```cursed
sus path1 := parse_url_path("https://example.com/api/v1/users")  # Returns "/api/v1/users"
sus path2 := parse_url_path("https://example.com")              # Returns "/"
sus path3 := parse_url_path("")                                 # Returns "/"
```

### Query Parameter Detection
```cursed
slay parse_query_params(url tea) lit
```
Detects presence of query parameters in URL.

**Parameters:**
- `url tea`: The URL to analyze

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

**Examples:**
```cursed
sus has_params := parse_query_params("https://example.com?key=value")  # Returns based
sus no_params := parse_query_params("https://example.com")             # Returns cap
```

## HTTP Method Validation

### Method Validation
```cursed
slay validate_method(method tea) lit
```
Validates HTTP method against supported methods.

**Parameters:**
- `method tea`: The HTTP method to validate

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

**Supported Methods:**
- GET, POST, PUT, DELETE, PATCH

**Examples:**
```cursed
sus valid_get := validate_method("GET")      # Returns based
sus valid_post := validate_method("POST")    # Returns based
sus invalid := validate_method("INVALID")    # Returns cap
```

## Content Type Detection

### Content Type Detection
```cursed
slay detect_content_type(data tea) tea
```
Automatically detects content type based on data format.

**Parameters:**
- `data tea`: The data to analyze

**Returns:**
- `tea`: Detected MIME type

**Detection Rules:**
- Starts with "{" and ends with "}" → "application/json"
- Starts with "<" and ends with ">" → "text/html"
- Default → "text/plain"

**Examples:**
```cursed
sus json_type := detect_content_type("{\"key\":\"value\"}")  # Returns "application/json"
sus html_type := detect_content_type("<html></html>")       # Returns "text/html"
sus text_type := detect_content_type("plain text")          # Returns "text/plain"
```

## HTTP Response Building

### Response Builder
```cursed
slay build_response(status normie, body tea) tea
```
Constructs complete HTTP response with headers.

**Parameters:**
- `status normie`: HTTP status code
- `body tea`: Response body content

**Returns:**
- `tea`: Complete HTTP response string

**Response Components:**
1. Status line with code and text
2. Content-Type header (auto-detected)
3. Content-Length header (calculated)
4. Empty line separator
5. Response body

### Error Response Builder
```cursed
slay build_error_response(status normie, message tea) tea
```
Builds JSON error response.

**Parameters:**
- `status normie`: HTTP error status code
- `message tea`: Error message

**Returns:**
- `tea`: HTTP error response with JSON body

**Examples:**
```cursed
sus success := build_response(200, "Hello, World!")
sus error := build_error_response(404, "Resource not found")

# success returns:
# HTTP/1.1 200 OK\r\n
# Content-Type: text/plain\r\n
# Content-Length: 13\r\n
# \r\n
# Hello, World!

# error returns:
# HTTP/1.1 404 Not Found\r\n
# Content-Type: application/json\r\n
# Content-Length: 38\r\n
# \r\n
# {"error": "Resource not found"}
```

## Request Validation

### Request Validation
```cursed
slay validate_request(method tea, url tea) lit
```
Validates HTTP request components.

**Parameters:**
- `method tea`: HTTP method
- `url tea`: Request URL

**Returns:**
- `lit`: `based` if valid, `cringe` otherwise

**Validation Checks:**
1. Method is supported
2. URL is not empty
3. Basic format validation

**Examples:**
```cursed
sus valid := validate_request("GET", "https://example.com")     # Returns based
sus invalid_method := validate_request("INVALID", "https://example.com")  # Returns cap
sus invalid_url := validate_request("GET", "")                 # Returns cap
```

## HTTP Logging

### Request Logging
```cursed
slay log_request(method tea, url tea, status normie)
```
Logs HTTP request details for debugging and monitoring.

**Parameters:**
- `method tea`: HTTP method
- `url tea`: Request URL
- `status normie`: Response status code

**Output Format:**
```
[HTTP] METHOD URL - STATUS_CODE
```

**Examples:**
```cursed
log_request("GET", "/api/users", 200)    # Outputs: [HTTP] GET /api/users - 200
log_request("POST", "/api/login", 401)   # Outputs: [HTTP] POST /api/login - 401
```

## Type Definitions

### HTTP Types
- `ServerConfig`: Server configuration type (alias for `lit`)
- `normie`: Integer type for status codes and content length
- `tea`: String type for URLs, headers, and content
- `lit`: Boolean type for validation results

## Error Handling

### Error Conditions
- Empty URL: "Error: Empty URL"
- Invalid protocol: "Error: Invalid URL protocol"
- Malformed requests handled gracefully
- All errors returned as strings, not exceptions

### Error Response Format
```json
{"error": "Error message description"}
```

## Performance Characteristics

### Time Complexity
- URL parsing: O(n) where n is URL length
- Header validation: O(n) where n is header length
- Response building: O(n) where n is content length
- Method validation: O(1) with fixed method set

### Memory Usage
- String operations create new strings (immutable)
- No dynamic memory allocation for simple operations
- Efficient string concatenation for response building

## Usage Patterns

### Simple HTTP Client
```cursed
yeet "web_vibez"

# Basic HTTP client functionality
slay fetch_data(url tea) tea {
    ready !validate_method("GET") {
        damn "Error: Invalid method"
    }
    
    sus response := http_get(url)
    log_request("GET", url, 200)
    damn response
}

# POST data to API
slay post_data(url tea, data tea) tea {
    ready !validate_request("POST", url) {
        damn "Error: Invalid request"
    }
    
    sus response := http_post(url, data)
    log_request("POST", url, 201)
    damn response
}
```

### Basic HTTP Server
```cursed
# Simple request handler
slay handle_request(method tea, path tea, body tea) tea {
    ready !validate_method(method) {
        damn build_error_response(405, "Method not allowed")
    }
    
    ready path == "/" {
        damn build_response(200, "Welcome to CURSED Web Server!")
    } elif path == "/api/health" {
        damn build_response(200, "{\"status\":\"healthy\"}")
    } else {
        damn build_error_response(404, "Not found")
    }
}

# Route processing
slay process_route(url tea) tea {
    sus path := parse_url_path(url)
    sus has_params := parse_query_params(url)
    
    ready has_params {
        damn handle_request("GET", path + "?params", "")
    } else {
        damn handle_request("GET", path, "")
    }
}
```

### Content Negotiation
```cursed
# Automatic content type handling
slay serve_content(data tea) tea {
    sus content_type := detect_content_type(data)
    
    ready content_type == "application/json" {
        damn build_response(200, data)
    } elif content_type == "text/html" {
        damn build_response(200, data)
    } else {
        damn build_response(200, data)
    }
}
```

## Implementation Notes

### HTTP Protocol Compliance
- HTTP/1.1 response format
- Standard status codes and messages
- Proper header formatting
- Basic protocol validation

### Pure CURSED Implementation
- No external HTTP library dependencies
- Compatible with both interpretation and compilation modes
- Self-contained request/response handling

### Extension Points
- Easily extensible for additional HTTP methods
- Configurable content type detection
- Customizable error response formats

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "web_vibez"

# Test status code mapping
test_start("status codes")
assert_eq_string(status_code_text(200), "OK")
assert_eq_string(status_code_text(404), "Not Found")

# Test HTTP methods
test_start("method validation")
assert_true(validate_method("GET"))
assert_true(validate_method("POST"))
assert_false(validate_method("INVALID"))

# Test URL processing
test_start("URL parsing")
assert_eq_string(parse_url_path("https://example.com/test"), "/test")
assert_true(parse_query_params("https://example.com?param=value"))

print_test_summary()
```

### Integration Tests
- End-to-end HTTP request/response cycles
- Error condition handling
- Large payload processing
- Performance benchmarks

## Dependencies

- `testz`: Testing framework for module validation
- `vibez`: Output operations for logging
- No external HTTP libraries or networking dependencies

## Security Considerations

- URL validation prevents malformed requests
- Input sanitization for headers and content
- Safe string operations prevent buffer overflows
- No execution of arbitrary content

## Thread Safety

- All HTTP functions are pure and thread-safe
- No shared state or global variables
- Safe for concurrent request processing
- Immutable string operations only

## Compatibility

### HTTP Standards
- HTTP/1.1 protocol compliance
- Standard MIME types and status codes
- Common HTTP header formats
- RESTful API conventions

### Platform Support
- Consistent behavior across all platforms
- No platform-specific networking code
- Portable pure CURSED implementation
