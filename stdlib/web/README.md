# Web Module

The web module provides a comprehensive web framework with routing, middleware, session management, and WebSocket support for the CURSED language. It enables building modern web applications with clean, type-safe APIs.

## Features

- **HTTP Server**: Create and manage web servers
- **Routing**: Pattern-based URL routing with parameter extraction
- **Middleware**: Pluggable middleware system with priority support
- **Session Management**: Server-side session handling
- **Cookie Support**: HTTP cookie management
- **Template Rendering**: Template engine integration
- **Static File Serving**: Efficient static asset serving
- **CORS Support**: Cross-origin resource sharing
- **Security Headers**: CSP, HSTS, and other security features
- **WebSocket Support**: Real-time communication
- **Pure CURSED Implementation**: No external dependencies

## HTTP Methods

```cursed
HTTP_GET = 1
HTTP_POST = 2
HTTP_PUT = 3
HTTP_DELETE = 4
HTTP_HEAD = 5
HTTP_OPTIONS = 6
HTTP_PATCH = 7
```

## HTTP Status Codes

```cursed
HTTP_OK = 200
HTTP_CREATED = 201
HTTP_BAD_REQUEST = 400
HTTP_UNAUTHORIZED = 401
HTTP_FORBIDDEN = 403
HTTP_NOT_FOUND = 404
HTTP_INTERNAL_ERROR = 500
```

## Content Types

```cursed
CONTENT_TYPE_JSON = 1
CONTENT_TYPE_HTML = 2
CONTENT_TYPE_TEXT = 3
CONTENT_TYPE_XML = 4
```

## Basic Usage

### Server Setup

```cursed
yeet "web"

# Create and start server
sus server_id normie = web_server_create(8080)
web_server_listen(server_id, "127.0.0.1")
web_server_start(server_id)
```

### Routing

```cursed
# Add routes
web_route_add(server_id, HTTP_GET, "/", "home_handler")
web_route_add(server_id, HTTP_POST, "/api/users", "create_user_handler")
web_route_add(server_id, HTTP_GET, "/users/:id", "get_user_handler")

# Match routes
sus handler tea = web_route_match(server_id, HTTP_GET, "/users/123")
```

### Request Handling

```cursed
# Create request
sus request_id normie = web_request_create(HTTP_GET, "/api/users", "{}", "")

# Get request data
sus method smol = web_request_get_method(request_id)
sus path tea = web_request_get_path(request_id)
sus content_type tea = web_request_get_header(request_id, "Content-Type")
sus body tea = web_request_get_body(request_id)
sus user_id tea = web_request_get_param(request_id, "id")
```

### Response Handling

```cursed
# Create response
sus response_id normie = web_response_create(HTTP_OK, "{}", "Hello World")

# Modify response
web_response_set_status(response_id, HTTP_NOT_FOUND)
web_response_set_header(response_id, "Content-Type", "application/json")
web_response_set_body(response_id, "{\"message\": \"User not found\"}")
web_response_send(response_id)
```

### Middleware

```cursed
# Add middleware
web_middleware_add(server_id, "auth_middleware", 1)
web_middleware_add(server_id, "cors_middleware", 2)
web_middleware_add(server_id, "logging_middleware", 3)

# Execute middleware
web_middleware_execute(server_id, request_id, response_id)
```

### Session Management

```cursed
# Create session
web_session_create("user_session_123")

# Set session data
web_session_set("user_session_123", "user_id", "42")
web_session_set("user_session_123", "username", "john_doe")

# Get session data
sus user_id tea = web_session_get("user_session_123", "user_id")
sus username tea = web_session_get("user_session_123", "username")

# Destroy session
web_session_destroy("user_session_123")
```

### Cookie Support

```cursed
# Set cookie
web_cookie_set(response_id, "session", "abc123", "2024-12-31")

# Get cookie
sus session_id tea = web_cookie_get(request_id, "session")

# Delete cookie
web_cookie_delete(response_id, "session")
```

### Template Rendering

```cursed
# Load template
sus template_id normie = web_template_load("views/user.html")

# Render template
sus html tea = web_template_render(template_id, "{\"name\": \"John\", \"age\": 30}")

# Render string template
sus result tea = web_template_render_string("Hello {{name}}", "{\"name\": \"World\"}")
```

### Static File Serving

```cursed
# Serve static files
web_static_serve(server_id, "/static", "./public")
web_static_serve(server_id, "/assets", "./dist/assets")
```

### CORS Support

```cursed
# Enable CORS
web_cors_enable(server_id, "https://example.com,https://app.example.com")

# Set CORS headers
web_cors_set_headers(response_id, "GET,POST,PUT,DELETE", "Content-Type,Authorization")
```

### Security Headers

```cursed
# Set Content Security Policy
web_security_set_csp(response_id, "default-src 'self'; script-src 'self' 'unsafe-inline'")

# Set HSTS
web_security_set_hsts(response_id, 31536000)  # 1 year
```

### WebSocket Support

```cursed
# Upgrade to WebSocket
web_websocket_upgrade(request_id, response_id)

# Send message
web_websocket_send(connection_id, "Hello WebSocket!")

# Receive message
sus message tea = web_websocket_receive(connection_id)

# Close connection
web_websocket_close(connection_id)
```

### URL Utilities

```cursed
# Parse URL
sus parsed tea = web_url_parse("https://example.com/path?param=value")

# Encode URL
sus encoded tea = web_url_encode("hello world")

# Decode URL
sus decoded tea = web_url_decode("hello%20world")
```

## Functions

### Server Functions
- `web_server_create(port normie) normie` - Create web server
- `web_server_start(server_id normie) lit` - Start server
- `web_server_stop(server_id normie) lit` - Stop server  
- `web_server_listen(server_id normie, address tea) lit` - Bind server to address

### Routing Functions
- `web_route_add(server_id normie, method smol, path tea, handler_name tea) lit` - Add route
- `web_route_remove(server_id normie, method smol, path tea) lit` - Remove route
- `web_route_match(server_id normie, method smol, path tea) tea` - Match route

### Request Functions
- `web_request_create(method smol, path tea, headers tea, body tea) normie` - Create request
- `web_request_get_method(request_id normie) smol` - Get request method
- `web_request_get_path(request_id normie) tea` - Get request path
- `web_request_get_header(request_id normie, header_name tea) tea` - Get header
- `web_request_get_body(request_id normie) tea` - Get request body
- `web_request_get_param(request_id normie, param_name tea) tea` - Get parameter

### Response Functions
- `web_response_create(status_code smol, headers tea, body tea) normie` - Create response
- `web_response_set_status(response_id normie, status_code smol) lit` - Set status
- `web_response_set_header(response_id normie, header_name tea, header_value tea) lit` - Set header
- `web_response_set_body(response_id normie, body tea) lit` - Set body
- `web_response_send(response_id normie) lit` - Send response

### Middleware Functions
- `web_middleware_add(server_id normie, middleware_name tea, priority normie) lit` - Add middleware
- `web_middleware_remove(server_id normie, middleware_name tea) lit` - Remove middleware
- `web_middleware_execute(server_id normie, request_id normie, response_id normie) lit` - Execute middleware

### Session Functions
- `web_session_create(session_id tea) lit` - Create session
- `web_session_get(session_id tea, key tea) tea` - Get session value
- `web_session_set(session_id tea, key tea, value tea) lit` - Set session value
- `web_session_destroy(session_id tea) lit` - Destroy session

### Cookie Functions
- `web_cookie_set(response_id normie, name tea, value tea, expires tea) lit` - Set cookie
- `web_cookie_get(request_id normie, name tea) tea` - Get cookie
- `web_cookie_delete(response_id normie, name tea) lit` - Delete cookie

### Template Functions
- `web_template_load(template_file tea) normie` - Load template
- `web_template_render(template_id normie, data tea) tea` - Render template
- `web_template_render_string(template_string tea, data tea) tea` - Render string template

### Static File Functions
- `web_static_serve(server_id normie, path tea, directory tea) lit` - Serve static files

### URL Functions
- `web_url_parse(url tea) tea` - Parse URL
- `web_url_encode(text tea) tea` - Encode URL
- `web_url_decode(encoded_text tea) tea` - Decode URL

### CORS Functions
- `web_cors_enable(server_id normie, origins tea) lit` - Enable CORS
- `web_cors_set_headers(response_id normie, methods tea, headers tea) lit` - Set CORS headers

### Security Functions
- `web_security_set_csp(response_id normie, policy tea) lit` - Set CSP header
- `web_security_set_hsts(response_id normie, max_age normie) lit` - Set HSTS header

### WebSocket Functions
- `web_websocket_upgrade(request_id normie, response_id normie) lit` - Upgrade to WebSocket
- `web_websocket_send(connection_id normie, message tea) lit` - Send WebSocket message
- `web_websocket_receive(connection_id normie) tea` - Receive WebSocket message
- `web_websocket_close(connection_id normie) lit` - Close WebSocket connection

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/web/test_web.csd
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/web/test_web.csd
cargo run --bin cursed -- compile stdlib/web/test_web.csd
./test_web
```

## Error Handling

All functions return appropriate error values:
- Boolean functions return `cap` (false) on error
- Integer functions return -1 on error
- String functions return empty string on error

## Security

- Input validation on all parameters
- XSS prevention through proper escaping
- CSRF protection support
- Secure cookie handling
- Security headers integration

## Performance

- Efficient routing with pattern matching
- Optimized middleware execution
- Memory-efficient session management
- Fast static file serving
- Optimized for both interpretation and compilation modes

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures
- `json` - JSON handling
- `net` - Network operations

## License

Part of the CURSED language standard library.
