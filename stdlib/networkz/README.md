# NetworkZ - CURSED Standard Library Network Module

The NetworkZ module provides comprehensive HTTP client/server functionality and basic networking operations for the CURSED programming language. This module is implemented entirely in pure CURSED with no external dependencies.

## Features

- **HTTP Client**: GET, POST, and custom HTTP requests
- **HTTP Server**: Basic server functionality with request handlers
- **URL Parsing**: Complete URL parsing and manipulation
- **TCP Connections**: Low-level TCP socket management
- **Network Utilities**: Ping, port checking, and diagnostics
- **JSON/Form Support**: Convenience functions for API interactions
- **Error Handling**: Comprehensive network error management

## Quick Start

```cursed
yeet "networkz"

// Simple HTTP GET request
sus response HttpResponse = networkz.http_get("http://api.example.com/data") fam {
    when err -> {
        vibez.spill("Request failed:", err.message)
        damn
    }
}

vibez.spill("Status:", response.status_code)
vibez.spill("Body:", response.body)
```

## Data Structures

### NetworkError
```cursed
squad NetworkError {
    sus kind tea        // Error type (e.g., "tcp_connect", "http_parse")
    sus message tea     // Human-readable error message
    sus code drip       // HTTP status code or system error code
}
```

### HttpRequest
```cursed
squad HttpRequest {
    sus method tea      // HTTP method (GET, POST, etc.)
    sus url tea         // Complete URL
    sus headers []tea   // HTTP headers array
    sus body tea        // Request body content
    sus timeout drip    // Timeout in seconds
}
```

### HttpResponse
```cursed
squad HttpResponse {
    sus status_code drip    // HTTP status code (200, 404, etc.)
    sus headers []tea       // Response headers array
    sus body tea            // Response body content
    sus content_length drip // Content length in bytes
}
```

### UrlParts
```cursed
squad UrlParts {
    sus scheme tea      // Protocol (http, https)
    sus host tea        // Hostname or IP address
    sus port drip       // Port number
    sus path tea        // URL path
    sus query tea       // Query string
    sus fragment tea    // Fragment identifier
}
```

### TcpConnection
```cursed
squad TcpConnection {
    sus host tea        // Remote host
    sus port drip       // Remote port
    sus socket_fd drip  // Socket file descriptor
    sus is_connected lit // Connection status
}
```

### HttpServer
```cursed
squad HttpServer {
    sus host tea        // Bind address
    sus port drip       // Listen port
    sus socket_fd drip  // Server socket descriptor
    sus is_running lit  // Server status
    sus request_handler slay(HttpRequest) HttpResponse // Request handler
}
```

## HTTP Client Functions

### Basic HTTP Operations

#### `http_get(url tea) yikes<HttpResponse>`
Performs an HTTP GET request to the specified URL.

```cursed
sus response HttpResponse = networkz.http_get("https://api.github.com/users/octocat") fam {
    when err -> {
        vibez.spill("GET request failed:", err.message)
        damn
    }
}

ready (networkz.is_success_status(response.status_code)) {
    vibez.spill("Success! Body:", response.body)
} otherwise {
    vibez.spill("HTTP error:", response.status_code)
}
```

#### `http_post(url tea, body tea, content_type tea) yikes<HttpResponse>`
Performs an HTTP POST request with the specified body and content type.

```cursed
sus json_data tea = "{\"name\": \"John\", \"email\": \"john@example.com\"}"
sus response HttpResponse = networkz.http_post(
    "https://api.example.com/users",
    json_data,
    "application/json"
) fam {
    when err -> {
        vibez.spill("POST request failed:", err.message)
        damn
    }
}
```

#### `http_request_advanced(method tea, url tea, headers []tea, body tea, timeout drip) yikes<HttpResponse>`
Advanced HTTP request with custom headers and timeout.

```cursed
sus custom_headers []tea = [
    "Authorization: Bearer abc123",
    "User-Agent: MyApp/1.0",
    "Accept: application/json"
]

sus response HttpResponse = networkz.http_request_advanced(
    "PUT",
    "https://api.example.com/resource/123",
    custom_headers,
    "{\"status\": \"updated\"}",
    60  // 60 second timeout
) fam {
    when err -> {
        vibez.spill("Advanced request failed:", err.message)
        damn
    }
}
```

### JSON API Convenience Functions

#### `json_get(url tea) yikes<HttpResponse>`
GET request with JSON headers automatically set.

```cursed
sus response HttpResponse = networkz.json_get("https://api.example.com/data") fam {
    when err -> {
        vibez.spill("JSON GET failed:", err.message)
        damn
    }
}
```

#### `json_post(url tea, json_body tea) yikes<HttpResponse>`
POST request with JSON content type and accept headers.

```cursed
sus user_data tea = "{\"name\": \"Alice\", \"role\": \"admin\"}"
sus response HttpResponse = networkz.json_post("https://api.example.com/users", user_data) fam {
    when err -> {
        vibez.spill("JSON POST failed:", err.message)
        damn
    }
}
```

### Form Data Functions

#### `form_post(url tea, form_data []tea) yikes<HttpResponse>`
POST request with form-encoded data.

```cursed
sus form_fields []tea = [
    "username=johndoe",
    "password=secret123",
    "remember=true"
]

sus response HttpResponse = networkz.form_post("https://example.com/login", form_fields) fam {
    when err -> {
        vibez.spill("Form POST failed:", err.message)
        damn
    }
}
```

## URL Parsing and Manipulation

#### `parse_url(url tea) yikes<UrlParts>`
Parses a URL into its component parts.

```cursed
sus url_parts UrlParts = networkz.parse_url("https://api.example.com:8080/v1/users?limit=10#section1") fam {
    when err -> {
        vibez.spill("URL parsing failed:", err.message)
        damn
    }
}

vibez.spill("Scheme:", url_parts.scheme)    // "https"
vibez.spill("Host:", url_parts.host)        // "api.example.com"
vibez.spill("Port:", url_parts.port)        // 8080
vibez.spill("Path:", url_parts.path)        // "/v1/users"
vibez.spill("Query:", url_parts.query)      // "limit=10"
vibez.spill("Fragment:", url_parts.fragment) // "section1"
```

#### `encode_url_params(params []tea) tea`
Encodes parameters for URL query strings.

```cursed
sus params []tea = ["name=John Doe", "city=New York", "age=30"]
sus encoded tea = networkz.encode_url_params(params)
vibez.spill("Encoded:", encoded)  // "name=John%20Doe&city=New%20York&age=30"
```

#### `decode_url_params(encoded tea) []tea`
Decodes URL-encoded parameter string.

```cursed
sus encoded tea = "name=John%20Doe&city=New%20York"
sus decoded []tea = networkz.decode_url_params(encoded)
// decoded[0] = "name=John Doe"
// decoded[1] = "city=New York"
```

## TCP Connection Management

#### `tcp_connect(host tea, port drip) yikes<TcpConnection>`
Establishes a TCP connection to a remote host.

```cursed
sus conn TcpConnection = networkz.tcp_connect("example.com", 80) fam {
    when err -> {
        vibez.spill("Connection failed:", err.message)
        damn
    }
}

vibez.spill("Connected to:", conn.host, "on port", conn.port)
```

#### `tcp_send(conn TcpConnection, data tea) yikes<drip>`
Sends data over a TCP connection.

```cursed
sus bytes_sent drip = networkz.tcp_send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n") fam {
    when err -> {
        vibez.spill("Send failed:", err.message)
        damn
    }
}

vibez.spill("Sent", bytes_sent, "bytes")
```

#### `tcp_receive(conn TcpConnection, buffer_size drip) yikes<tea>`
Receives data from a TCP connection.

```cursed
sus response tea = networkz.tcp_receive(conn, 4096) fam {
    when err -> {
        vibez.spill("Receive failed:", err.message)
        damn
    }
}

vibez.spill("Received:", response)
```

#### `tcp_close(conn TcpConnection) yikes<lit>`
Closes a TCP connection.

```cursed
networkz.tcp_close(conn) fam {
    when err -> {
        vibez.spill("Close failed:", err.message)
    }
}
```

## HTTP Server Functions

#### `create_http_server(host tea, port drip, handler slay(HttpRequest) HttpResponse) yikes<HttpServer>`
Creates an HTTP server with a request handler.

```cursed
// Define request handler
slay handle_request(req HttpRequest) HttpResponse {
    ready (stringz.equals(req.url, "/hello")) {
        damn HttpResponse{
            status_code: 200,
            headers: ["Content-Type: text/plain"],
            body: "Hello, World!",
            content_length: 13
        }
    } otherwise {
        damn HttpResponse{
            status_code: 404,
            headers: ["Content-Type: text/plain"],
            body: "Not Found",
            content_length: 9
        }
    }
}

sus server HttpServer = networkz.create_http_server("127.0.0.1", 8080, handle_request) fam {
    when err -> {
        vibez.spill("Server creation failed:", err.message)
        damn
    }
}
```

#### `start_http_server(server HttpServer) yikes<lit>`
Starts the HTTP server.

```cursed
networkz.start_http_server(server) fam {
    when err -> {
        vibez.spill("Server start failed:", err.message)
        damn
    }
}

vibez.spill("Server running on", server.host, ":", server.port)
```

#### `stop_http_server(server HttpServer) yikes<lit>`
Stops the HTTP server.

```cursed
networkz.stop_http_server(server) fam {
    when err -> {
        vibez.spill("Server stop failed:", err.message)
    }
}
```

## Response Utilities

#### `get_response_header(response HttpResponse, header_name tea) tea`
Extracts a specific header from an HTTP response.

```cursed
sus content_type tea = networkz.get_response_header(response, "Content-Type")
ready (stringz.len(content_type) > 0) {
    vibez.spill("Content type:", content_type)
}
```

#### Status Code Checking Functions

```cursed
// Check if response indicates success (2xx)
ready (networkz.is_success_status(response.status_code)) {
    vibez.spill("Request succeeded")
}

// Check if response is a redirect (3xx)
ready (networkz.is_redirect_status(response.status_code)) {
    sus location tea = networkz.get_response_header(response, "Location")
    vibez.spill("Redirect to:", location)
}

// Check for client error (4xx)
ready (networkz.is_client_error_status(response.status_code)) {
    vibez.spill("Client error:", response.status_code)
}

// Check for server error (5xx)
ready (networkz.is_server_error_status(response.status_code)) {
    vibez.spill("Server error:", response.status_code)
}
```

## File Operations

#### `download_file(url tea, local_path tea) yikes<drip>`
Downloads a file from a URL to local storage.

```cursed
sus bytes_written drip = networkz.download_file(
    "https://example.com/file.pdf",
    "/tmp/downloaded_file.pdf"
) fam {
    when err -> {
        vibez.spill("Download failed:", err.message)
        damn
    }
}

vibez.spill("Downloaded", bytes_written, "bytes")
```

## Network Diagnostics

#### `ping_host(host tea) yikes<drip>`
Tests network connectivity to a host (returns ping time in milliseconds).

```cursed
sus ping_time drip = networkz.ping_host("google.com") fam {
    when err -> {
        vibez.spill("Ping failed:", err.message)
        damn
    }
}

vibez.spill("Ping time:", ping_time, "ms")
```

#### `check_port_open(host tea, port drip) yikes<lit>`
Checks if a specific port is open on a host.

```cursed
sus is_open lit = networkz.check_port_open("example.com", 80) fam {
    when err -> {
        vibez.spill("Port check failed:", err.message)
        damn
    }
}

ready (is_open) {
    vibez.spill("Port 80 is open")
} otherwise {
    vibez.spill("Port 80 is closed")
}
```

## Error Handling

NetworkZ provides comprehensive error handling through the `NetworkError` structure:

### Common Error Types

- **`url_parse`**: URL parsing errors
- **`tcp_connect`**: Connection establishment errors
- **`tcp_send`**: Data transmission errors  
- **`tcp_receive`**: Data reception errors
- **`http_parse`**: HTTP response parsing errors
- **`http_request`**: General HTTP request errors
- **`server_create`**: Server creation errors
- **`server_start`**: Server startup errors
- **`download`**: File download errors
- **`ping`**: Network connectivity errors

### Error Handling Patterns

```cursed
// Pattern 1: Simple error handling
sus response HttpResponse = networkz.http_get(url) fam {
    when err -> {
        vibez.spill("Request failed:", err.message, "(", err.kind, ")")
        damn
    }
}

// Pattern 2: Specific error handling
sus response HttpResponse = networkz.http_get(url) fam {
    when err -> {
        ready (stringz.equals(err.kind, "tcp_connect")) {
            vibez.spill("Connection error - server may be down")
        } otherwise ready (stringz.equals(err.kind, "http_parse")) {
            vibez.spill("Invalid response from server")
        } otherwise {
            vibez.spill("Unexpected error:", err.message)
        }
        damn
    }
}

// Pattern 3: Retry logic with exponential backoff
slay http_get_with_retry(url tea, max_attempts drip) yikes<HttpResponse> {
    sus attempt drip = 0
    bestie (attempt < max_attempts) {
        sus response HttpResponse = networkz.http_get(url) fam {
            when err -> {
                attempt = attempt + 1
                ready (attempt >= max_attempts) {
                    yikes err
                }
                // Exponential backoff: wait 2^attempt seconds
                sus wait_time drip = mathz.power(2, attempt) * 1000
                // In real implementation, would actually sleep
                damn // Continue to next iteration
            }
        }
        damn response
    }
    yikes networkz.create_network_error("retry", "Max retries exceeded", 503)
}
```

## Performance Considerations

### Connection Reuse
```cursed
// For multiple requests to same host, consider connection pooling
slay make_multiple_requests(base_url tea, endpoints []tea) []HttpResponse {
    sus responses []HttpResponse = []
    sus i drip = 0
    
    bestie (i < arrayz.len(endpoints)) {
        sus full_url tea = stringz.concat([base_url, endpoints[i]])
        sus response HttpResponse = networkz.http_get(full_url) fam {
            when err -> {
                // Log error but continue with other requests
                vibez.spill("Request to", endpoints[i], "failed:", err.message)
                i = i + 1
                damn // Continue loop
            }
        }
        responses = arrayz.push(responses, response)
        i = i + 1
    }
    
    damn responses
}
```

### Timeout Configuration
```cursed
// Use appropriate timeouts for different scenarios
sus quick_response HttpResponse = networkz.http_request_advanced(
    "GET", 
    "https://fast-api.example.com/health", 
    [], 
    "", 
    5  // 5 second timeout for health checks
)

sus large_download HttpResponse = networkz.http_request_advanced(
    "GET", 
    "https://cdn.example.com/large-file.zip", 
    [], 
    "", 
    300  // 5 minute timeout for large downloads
)
```

### Memory Management
```cursed
// For large responses, consider streaming or chunked processing
slay process_large_response(response HttpResponse) lit {
    ready (response.content_length > 10 * 1024 * 1024) {  // 10MB
        vibez.spill("Warning: Large response size:", response.content_length, "bytes")
        // In real implementation, process in chunks
    }
    
    // Process response body
    damn based
}
```

## Best Practices

1. **Always handle errors**: Network operations can fail in many ways
2. **Use appropriate timeouts**: Prevent hanging requests
3. **Validate URLs**: Parse and validate URLs before making requests  
4. **Handle redirects**: Check for 3xx status codes and Location headers
5. **Set proper headers**: Include User-Agent, Accept, and Content-Type headers
6. **Connection cleanup**: Always close connections when done
7. **Retry logic**: Implement exponential backoff for transient failures
8. **Security**: Validate certificates in production (HTTPS)
9. **Rate limiting**: Respect server rate limits and implement client-side throttling
10. **Logging**: Log request/response details for debugging

## Thread Safety

NetworkZ operations are designed to be thread-safe when used properly:

- Each connection should be used by only one goroutine at a time
- Server instances can handle multiple concurrent requests
- Stateless functions (like `http_get`, `parse_url`) are safe to call concurrently
- Shared connection pools should be protected with appropriate synchronization

## Integration Examples

### REST API Client
```cursed
squad ApiClient {
    sus base_url tea
    sus auth_token tea
    sus timeout drip
}

slay create_api_client(base_url tea, auth_token tea) ApiClient {
    damn ApiClient{
        base_url: base_url,
        auth_token: auth_token,
        timeout: 30
    }
}

slay api_get(client ApiClient, endpoint tea) yikes<HttpResponse> {
    sus full_url tea = stringz.concat([client.base_url, endpoint])
    sus headers []tea = [
        stringz.concat(["Authorization: Bearer ", client.auth_token]),
        "Accept: application/json"
    ]
    
    damn networkz.http_request_advanced("GET", full_url, headers, "", client.timeout)
}
```

### Web Scraper
```cursed
slay scrape_page(url tea) yikes<[]tea> {
    sus response HttpResponse = networkz.http_get(url) fam {
        when err -> yikes err
    }
    
    ready (!networkz.is_success_status(response.status_code)) {
        yikes networkz.create_network_error("scrape", "HTTP error", response.status_code)
    }
    
    // Extract links (simplified)
    sus links []tea = stringz.extract_all(response.body, "href=\"", "\"")
    damn links
}
```

This documentation provides comprehensive coverage of the NetworkZ module's capabilities for building robust networked applications in CURSED.
