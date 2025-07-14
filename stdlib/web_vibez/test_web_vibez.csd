# Comprehensive Tests for web_vibez HTTP Module
yeet "testz"
yeet "web_vibez"
yeet "stringz"
yeet "timez"

# ===============================
# Test HTTP Header Functions
# ===============================

slay test_http_headers() {
    test_start("HTTP headers manipulation")
    
    # Test header creation and manipulation
    sus headers HttpHeaders = web_vibez.init_headers()
    assert_eq_int(headers.count, 0)
    
    # Test adding headers
    headers = web_vibez.add_header(headers, "Content-Type", "application/json")
    headers = web_vibez.add_header(headers, "Authorization", "Bearer token123")
    assert_eq_int(headers.count, 2)
    
    # Test getting headers
    sus content_type tea = web_vibez.get_header(headers, "Content-Type")
    assert_eq_string(content_type, "application/json")
    
    sus auth tea = web_vibez.get_header(headers, "authorization")  # Test case insensitive
    assert_eq_string(auth, "Bearer token123")
    
    # Test non-existent header
    sus missing tea = web_vibez.get_header(headers, "Non-Existent")
    assert_eq_string(missing, "")
    
    # Test setting existing header
    headers = web_vibez.set_header(headers, "Content-Type", "text/html")
    sus updated_type tea = web_vibez.get_header(headers, "Content-Type")
    assert_eq_string(updated_type, "text/html")
    assert_eq_int(headers.count, 2)  # Count should remain same
    
    # Test removing header
    headers = web_vibez.remove_header(headers, "Authorization")
    assert_eq_int(headers.count, 1)
    sus removed_auth tea = web_vibez.get_header(headers, "Authorization")
    assert_eq_string(removed_auth, "")
    
    # Test headers to string
    sus headers_string tea = web_vibez.headers_to_string(headers)
    assert_contains(headers_string, "Content-Type: text/html")
    assert_contains(headers_string, "\r\n")
    
    test_end()
}

# ===============================
# Test HTTP URL Functions
# ===============================

slay test_http_url_parsing() {
    test_start("HTTP URL parsing and formatting")
    
    # Test HTTP URL parsing
    sus url1 HttpUrl = web_vibez.parse_url("http://example.com/path/to/resource")
    assert_eq_string(url1.scheme, "http")
    assert_eq_string(url1.host, "example.com")
    assert_eq_int(url1.port, 80)
    assert_eq_string(url1.path, "/path/to/resource")
    
    # Test HTTPS URL parsing
    sus url2 HttpUrl = web_vibez.parse_url("https://secure.example.com:8443/api/v1")
    assert_eq_string(url2.scheme, "https")
    assert_eq_string(url2.host, "secure.example.com")
    assert_eq_int(url2.port, 8443)
    assert_eq_string(url2.path, "/api/v1")
    
    # Test URL without protocol
    sus url3 HttpUrl = web_vibez.parse_url("localhost:3000/test")
    assert_eq_string(url3.scheme, "http")
    assert_eq_string(url3.host, "localhost")
    assert_eq_int(url3.port, 3000)
    assert_eq_string(url3.path, "/test")
    
    # Test URL to string conversion
    sus url_string tea = web_vibez.url_to_string(url1)
    assert_eq_string(url_string, "http://example.com/path/to/resource")
    
    sus secure_url_string tea = web_vibez.url_to_string(url2)
    assert_eq_string(secure_url_string, "https://secure.example.com:8443/api/v1")
    
    test_end()
}

# ===============================
# Test HTTP Request Functions
# ===============================

slay test_http_request_creation() {
    test_start("HTTP request creation and manipulation")
    
    # Test request creation
    sus request HttpRequest = web_vibez.create_request("GET", "http://api.example.com/users")
    assert_eq_string(request.method, "GET")
    assert_eq_string(request.url.host, "api.example.com")
    assert_eq_string(request.url.path, "/users")
    assert_eq_string(request.proto, "HTTP/1.1")
    assert_eq_int(request.proto_major, 1)
    assert_eq_int(request.proto_minor, 1)
    
    # Test default headers are added
    sus host_header tea = web_vibez.get_header(request.headers, "Host")
    assert_eq_string(host_header, "api.example.com")
    
    sus user_agent tea = web_vibez.get_header(request.headers, "User-Agent")
    assert_eq_string(user_agent, "CURSED-HTTP/1.0")
    
    # Test setting request body
    request = web_vibez.set_request_body(request, "test body content")
    assert_eq_string(request.body, "test body content")
    assert_eq_int(request.content_length, 17)  # Length of "test body content"
    
    sus content_length tea = web_vibez.get_header(request.headers, "Content-Length")
    assert_eq_string(content_length, "17")
    
    # Test setting JSON body
    sus json_request HttpRequest = web_vibez.create_request("POST", "http://api.test.com/data")
    json_request = web_vibez.set_request_json(json_request, '{"name":"test","value":42}')
    
    sus content_type tea = web_vibez.get_header(json_request.headers, "Content-Type")
    assert_eq_string(content_type, "application/json")
    assert_eq_string(json_request.body, '{"name":"test","value":42}')
    
    # Test adding custom headers
    request = web_vibez.add_request_header(request, "Authorization", "Bearer abc123")
    sus auth_header tea = web_vibez.get_header(request.headers, "Authorization")
    assert_eq_string(auth_header, "Bearer abc123")
    
    # Test request to string conversion
    sus request_string tea = web_vibez.request_to_string(request)
    assert_contains(request_string, "GET /users HTTP/1.1")
    assert_contains(request_string, "Host: api.example.com")
    assert_contains(request_string, "test body content")
    
    test_end()
}

# ===============================
# Test HTTP Response Functions
# ===============================

slay test_http_response_creation() {
    test_start("HTTP response creation and manipulation")
    
    # Test response creation
    sus response HttpResponse = web_vibez.create_response(200)
    assert_eq_int(response.status_code, 200)
    assert_eq_string(response.status, "OK")
    assert_eq_string(response.proto, "HTTP/1.1")
    
    # Test default headers
    sus connection tea = web_vibez.get_header(response.headers, "Connection")
    assert_eq_string(connection, "close")
    
    sus server tea = web_vibez.get_header(response.headers, "Server")
    assert_eq_string(server, "CURSED-HTTP/1.0")
    
    # Test setting response body
    response = web_vibez.set_response_body(response, "Hello, World!")
    assert_eq_string(response.body, "Hello, World!")
    assert_eq_int(response.content_length, 13)
    
    # Test setting JSON response
    sus json_response HttpResponse = web_vibez.create_response(201)
    json_response = web_vibez.set_response_json(json_response, '{"created":true,"id":123}')
    
    sus json_content_type tea = web_vibez.get_header(json_response.headers, "Content-Type")
    assert_eq_string(json_content_type, "application/json")
    assert_eq_string(json_response.body, '{"created":true,"id":123}')
    
    # Test setting HTML response
    sus html_response HttpResponse = web_vibez.create_response(200)
    html_response = web_vibez.set_response_html(html_response, "<h1>Welcome</h1>")
    
    sus html_content_type tea = web_vibez.get_header(html_response.headers, "Content-Type")
    assert_eq_string(html_content_type, "text/html")
    
    # Test response to string conversion
    sus response_string tea = web_vibez.response_to_string(response)
    assert_contains(response_string, "HTTP/1.1 200 OK")
    assert_contains(response_string, "Content-Length: 13")
    assert_contains(response_string, "Hello, World!")
    
    test_end()
}

# ===============================
# Test HTTP Status Functions
# ===============================

slay test_http_status_codes() {
    test_start("HTTP status code handling")
    
    # Test common status codes
    assert_eq_string(web_vibez.get_status_text(200), "OK")
    assert_eq_string(web_vibez.get_status_text(201), "Created")
    assert_eq_string(web_vibez.get_status_text(400), "Bad Request")
    assert_eq_string(web_vibez.get_status_text(401), "Unauthorized")
    assert_eq_string(web_vibez.get_status_text(404), "Not Found")
    assert_eq_string(web_vibez.get_status_text(500), "Internal Server Error")
    
    # Test redirect status codes
    assert_eq_string(web_vibez.get_status_text(301), "Moved Permanently")
    assert_eq_string(web_vibez.get_status_text(302), "Found")
    
    # Test unknown status code
    assert_eq_string(web_vibez.get_status_text(999), "Unknown Status")
    
    # Test using status_text public API
    assert_eq_string(web_vibez.status_text(204), "No Content")
    assert_eq_string(web_vibez.status_text(503), "Service Unavailable")
    
    test_end()
}

# ===============================
# Test HTTP Client Functions
# ===============================

slay test_http_client() {
    test_start("HTTP client functionality")
    
    # Test client creation
    sus client HttpClient = web_vibez.create_client()
    assert_eq_int(client.timeout, 30000000000)  # 30 seconds
    assert_eq_string(client.user_agent, "CURSED-HTTP/1.0")
    assert_eq_int(client.max_redirects, 10)
    
    # Test client GET request
    sus get_response HttpResponse = web_vibez.client_get(client, "http://httpbin.org/get")
    assert_eq_int(get_response.status_code, 200)
    assert_eq_string(get_response.status, "OK")
    assert_contains(get_response.body, "Success")
    
    # Test client POST request
    sus post_response HttpResponse = web_vibez.client_post(client, "http://httpbin.org/post", "test data")
    assert_eq_int(post_response.status_code, 200)
    
    # Test client JSON POST request
    sus json_post_response HttpResponse = web_vibez.client_post_json(client, "http://api.test.com/users", '{"name":"John","age":30}')
    assert_eq_int(json_post_response.status_code, 200)
    
    # Test client PUT request
    sus put_response HttpResponse = web_vibez.client_put(client, "http://httpbin.org/put", "updated data")
    assert_eq_int(put_response.status_code, 200)
    
    # Test client DELETE request
    sus delete_response HttpResponse = web_vibez.client_delete(client, "http://httpbin.org/delete")
    assert_eq_int(delete_response.status_code, 200)
    
    # Test convenience functions
    sus simple_get HttpResponse = web_vibez.http_get("http://example.com")
    assert_eq_int(simple_get.status_code, 200)
    
    sus simple_post HttpResponse = web_vibez.http_post("http://example.com/api", "payload")
    assert_eq_int(simple_post.status_code, 200)
    
    sus simple_json_post HttpResponse = web_vibez.http_post_json("http://example.com/api", '{"test":true}')
    assert_eq_int(simple_json_post.status_code, 200)
    
    test_end()
}

# ===============================
# Test HTTP Server Functions
# ===============================

slay test_http_server() {
    test_start("HTTP server functionality")
    
    # Test server creation
    sus server HttpServer = web_vibez.create_server("localhost", 8080)
    assert_eq_string(server.addr, "localhost")
    assert_eq_int(server.port, 8080)
    assert_eq_int(server.timeout, 30000000000)
    assert_eq_int(server.max_connections, 100)
    assert_false(server.running)
    
    # Test handler registration
    web_vibez.server_handle_func("/", "home_handler")
    web_vibez.server_handle_get("/users", "get_users_handler")
    web_vibez.server_handle_post("/users", "create_user_handler")
    web_vibez.server_handle_put("/users/:id", "update_user_handler")
    web_vibez.server_handle_delete("/users/:id", "delete_user_handler")
    
    # Test convenience functions
    web_vibez.handle_func("/api", "api_handler")
    web_vibez.handle_get("/health", "health_check_handler")
    web_vibez.handle_post("/webhook", "webhook_handler")
    
    # Test server start/stop
    sus start_result lit = web_vibez.server_start(server)
    assert_true(start_result)
    assert_true(server.running)
    
    sus stop_result lit = web_vibez.server_stop(server)
    assert_true(stop_result)
    assert_false(server.running)
    
    # Test listen and serve convenience function
    # Note: This would normally block, but our implementation simulates it
    sus listen_result lit = web_vibez.listen_and_serve("localhost", 3000)
    assert_true(listen_result)
    
    test_end()
}

# ===============================
# Test HTTP Middleware Functions
# ===============================

slay test_http_middleware() {
    test_start("HTTP middleware functionality")
    
    # Test adding middleware
    web_vibez.add_middleware("custom_auth")
    web_vibez.add_middleware("request_logger")
    web_vibez.add_middleware("rate_limiter")
    
    # Test removing middleware
    web_vibez.remove_middleware("request_logger")
    
    # Test built-in middleware enablers
    web_vibez.enable_logging_middleware()
    web_vibez.enable_cors_middleware()
    web_vibez.enable_compression_middleware()
    web_vibez.enable_rate_limit_middleware()
    
    test_end()
}

# ===============================
# Test HTTP Cookie Functions
# ===============================

slay test_http_cookies() {
    test_start("HTTP cookie functionality")
    
    # Test cookie creation
    sus cookie HttpCookie = web_vibez.create_cookie("session_id", "abc123xyz")
    assert_eq_string(cookie.name, "session_id")
    assert_eq_string(cookie.value, "abc123xyz")
    assert_eq_string(cookie.path, "/")
    assert_false(cookie.secure)
    assert_false(cookie.http_only)
    
    # Test cookie to string conversion
    sus cookie_string tea = web_vibez.cookie_to_string(cookie)
    assert_contains(cookie_string, "session_id=abc123xyz")
    assert_contains(cookie_string, "Path=/")
    
    # Test secure cookie
    cookie.secure = based
    cookie.http_only = based
    cookie.domain = "example.com"
    cookie.expires = 1640995200  # Some timestamp
    
    sus secure_cookie_string tea = web_vibez.cookie_to_string(cookie)
    assert_contains(secure_cookie_string, "Domain=example.com")
    assert_contains(secure_cookie_string, "Expires=1640995200")
    assert_contains(secure_cookie_string, "Secure")
    assert_contains(secure_cookie_string, "HttpOnly")
    
    test_end()
}

# ===============================
# Test HTTP Routing Functions
# ===============================

slay test_http_routing() {
    test_start("HTTP routing functionality")
    
    # Test adding routes
    web_vibez.add_route("GET", "/", "home_handler")
    web_vibez.add_route("POST", "/api/users", "create_user")
    web_vibez.add_route("GET", "/api/users/:id", "get_user")
    web_vibez.add_route("PUT", "/api/users/:id", "update_user")
    web_vibez.add_route("DELETE", "/api/users/:id", "delete_user")
    
    # Test finding routes
    sus route_index normie = web_vibez.find_route("GET", "/")
    assert_ge_int(route_index, 0)
    
    sus post_route_index normie = web_vibez.find_route("POST", "/api/users")
    assert_ge_int(post_route_index, 0)
    
    sus missing_route_index normie = web_vibez.find_route("PATCH", "/nonexistent")
    assert_eq_int(missing_route_index, -1)
    
    test_end()
}

# ===============================
# Test HTTP Utilities
# ===============================

slay test_http_utilities() {
    test_start("HTTP utility functions")
    
    # Test form data encoding
    sus encoded tea = web_vibez.encode_form_data("name=John Doe&age=30")
    assert_contains(encoded, "%20")  # Space should be encoded
    
    # Test form data decoding
    sus decoded tea = web_vibez.decode_form_data("name=John%20Doe%26Company")
    assert_contains(decoded, " ")    # %20 should be decoded to space
    assert_contains(decoded, "&")    # %26 should be decoded to &
    
    # Test JSON response creation
    sus json_response tea = web_vibez.create_json_response("test data")
    assert_contains(json_response, '"data":"test data"')
    assert_contains(json_response, '"status":"success"')
    assert_contains(json_response, '"timestamp"')
    
    # Test error response creation
    sus error_response tea = web_vibez.create_error_response("Not found", 404)
    assert_contains(error_response, '"error":"Not found"')
    assert_contains(error_response, '"code":404')
    
    test_end()
}

# ===============================
# Test WebSocket Support
# ===============================

slay test_websocket_support() {
    test_start("WebSocket support functionality")
    
    # Test WebSocket upgrade detection
    sus ws_request HttpRequest = web_vibez.create_request("GET", "ws://localhost:8080/ws")
    ws_request.headers = web_vibez.add_header(ws_request.headers, "Connection", "Upgrade")
    ws_request.headers = web_vibez.add_header(ws_request.headers, "Upgrade", "websocket")
    ws_request.headers = web_vibez.add_header(ws_request.headers, "Sec-WebSocket-Key", "test-key")
    
    sus is_upgrade lit = web_vibez.websocket_upgrade(ws_request)
    assert_true(is_upgrade)
    
    # Test WebSocket accept response
    sus ws_response HttpResponse = web_vibez.websocket_accept(ws_request)
    assert_eq_int(ws_response.status_code, 101)
    assert_eq_string(ws_response.status, "Switching Protocols")
    
    sus upgrade_header tea = web_vibez.get_header(ws_response.headers, "Upgrade")
    assert_eq_string(upgrade_header, "websocket")
    
    sus connection_header tea = web_vibez.get_header(ws_response.headers, "Connection")
    assert_eq_string(connection_header, "Upgrade")
    
    # Test non-WebSocket request
    sus regular_request HttpRequest = web_vibez.create_request("GET", "http://localhost:8080/")
    sus not_upgrade lit = web_vibez.websocket_upgrade(regular_request)
    assert_false(not_upgrade)
    
    test_end()
}

# ===============================
# Test Security Functions
# ===============================

slay test_security_functions() {
    test_start("HTTP security functions")
    
    # Test header value sanitization
    sus clean_value tea = web_vibez.sanitize_header_value("normal value")
    assert_eq_string(clean_value, "normal value")
    
    sus malicious_value tea = web_vibez.sanitize_header_value("malicious\r\nSet-Cookie: evil=true")
    assert_not_contains(malicious_value, "\r")
    assert_not_contains(malicious_value, "\n")
    
    # Test method validation
    assert_true(web_vibez.validate_method("GET"))
    assert_true(web_vibez.validate_method("POST"))
    assert_true(web_vibez.validate_method("PUT"))
    assert_true(web_vibez.validate_method("DELETE"))
    assert_true(web_vibez.validate_method("PATCH"))
    assert_true(web_vibez.validate_method("HEAD"))
    assert_true(web_vibez.validate_method("OPTIONS"))
    assert_false(web_vibez.validate_method("INVALID"))
    assert_false(web_vibez.validate_method("TRACE"))
    
    # Test security headers
    sus response HttpResponse = web_vibez.create_response(200)
    response = web_vibez.add_security_headers(response)
    
    sus xss_protection tea = web_vibez.get_header(response.headers, "X-XSS-Protection")
    assert_eq_string(xss_protection, "1; mode=block")
    
    sus frame_options tea = web_vibez.get_header(response.headers, "X-Frame-Options")
    assert_eq_string(frame_options, "DENY")
    
    sus content_type_options tea = web_vibez.get_header(response.headers, "X-Content-Type-Options")
    assert_eq_string(content_type_options, "nosniff")
    
    sus hsts tea = web_vibez.get_header(response.headers, "Strict-Transport-Security")
    assert_eq_string(hsts, "max-age=31536000")
    
    test_end()
}

# ===============================
# Test Performance Monitoring
# ===============================

slay test_performance_monitoring() {
    test_start("HTTP performance monitoring")
    
    # Test metrics initialization
    sus metrics HttpMetrics = web_vibez.get_metrics()
    assert_eq_int(metrics.total_requests, 0)
    assert_eq_int(metrics.successful_requests, 0)
    assert_eq_int(metrics.failed_requests, 0)
    
    # Test recording successful request
    web_vibez.record_request(based, 150, 1024, 2048)
    metrics = web_vibez.get_metrics()
    assert_eq_int(metrics.total_requests, 1)
    assert_eq_int(metrics.successful_requests, 1)
    assert_eq_int(metrics.failed_requests, 0)
    assert_eq_int(metrics.average_response_time, 150)
    assert_eq_int(metrics.bytes_sent, 1024)
    assert_eq_int(metrics.bytes_received, 2048)
    
    # Test recording failed request
    web_vibez.record_request(cap, 300, 512, 1024)
    metrics = web_vibez.get_metrics()
    assert_eq_int(metrics.total_requests, 2)
    assert_eq_int(metrics.successful_requests, 1)
    assert_eq_int(metrics.failed_requests, 1)
    assert_eq_int(metrics.average_response_time, 225)  # (150 + 300) / 2
    assert_eq_int(metrics.bytes_sent, 1536)  # 1024 + 512
    assert_eq_int(metrics.bytes_received, 3072)  # 2048 + 1024
    
    test_end()
}

# ===============================
# Test HTTP/2 Support
# ===============================

slay test_http2_support() {
    test_start("HTTP/2 support framework")
    
    # Test HTTP/2 settings creation
    sus settings Http2Settings = web_vibez.create_http2_settings()
    assert_eq_int(settings.header_table_size, 4096)
    assert_true(settings.enable_push)
    assert_eq_int(settings.max_concurrent_streams, 100)
    assert_eq_int(settings.initial_window_size, 65535)
    assert_eq_int(settings.max_frame_size, 16384)
    assert_eq_int(settings.max_header_list_size, 8192)
    
    test_end()
}

# ===============================
# Test Public API Functions
# ===============================

slay test_public_api() {
    test_start("Public API convenience functions")
    
    # Test request creation API
    sus request HttpRequest = web_vibez.new_request("POST", "http://api.example.com/data")
    assert_eq_string(request.method, "POST")
    assert_eq_string(request.url.host, "api.example.com")
    assert_eq_string(request.url.path, "/data")
    
    # Test response creation API
    sus response HttpResponse = web_vibez.new_response(201)
    assert_eq_int(response.status_code, 201)
    assert_eq_string(response.status, "Created")
    
    test_end()
}

# ===============================
# Integration Tests
# ===============================

slay test_integration_client_server() {
    test_start("Integration: Client-Server communication simulation")
    
    # Start a simulated server
    sus server HttpServer = web_vibez.create_server("localhost", 8080)
    web_vibez.server_handle_get("/api/test", "test_handler")
    web_vibez.enable_logging_middleware()
    web_vibez.enable_cors_middleware()
    
    sus start_success lit = web_vibez.server_start(server)
    assert_true(start_success)
    
    # Make a client request
    sus client HttpClient = web_vibez.create_client()
    sus response HttpResponse = web_vibez.client_get(client, "http://localhost:8080/api/test")
    
    # Verify response
    assert_eq_int(response.status_code, 200)
    assert_contains(response.body, "Success")
    
    # Stop server
    sus stop_success lit = web_vibez.server_stop(server)
    assert_true(stop_success)
    
    test_end()
}

slay test_integration_full_http_cycle() {
    test_start("Integration: Full HTTP request-response cycle")
    
    # Create and configure a request
    sus request HttpRequest = web_vibez.create_request("POST", "https://api.example.com:8443/v1/users")
    request = web_vibez.set_request_json(request, '{"name":"Alice","email":"alice@example.com"}')
    request = web_vibez.add_request_header(request, "Authorization", "Bearer token123")
    
    # Verify request is properly formed
    assert_eq_string(request.method, "POST")
    assert_eq_string(request.url.scheme, "https")
    assert_eq_int(request.url.port, 8443)
    assert_eq_string(request.url.path, "/v1/users")
    
    sus auth_header tea = web_vibez.get_header(request.headers, "Authorization")
    assert_eq_string(auth_header, "Bearer token123")
    
    sus content_type tea = web_vibez.get_header(request.headers, "Content-Type")
    assert_eq_string(content_type, "application/json")
    
    # Send request and get response
    sus response HttpResponse = web_vibez.send_request(request)
    
    # Verify response
    assert_eq_int(response.status_code, 200)
    assert_contains(response.body, "Success")
    assert_contains(response.body, "timestamp")
    
    # Add security headers to response
    response = web_vibez.add_security_headers(response)
    
    # Verify security headers were added
    sus xss_protection tea = web_vibez.get_header(response.headers, "X-XSS-Protection")
    assert_not_empty_string(xss_protection)
    
    test_end()
}

# ===============================
# Performance and Stress Tests
# ===============================

slay test_performance_multiple_requests() {
    test_start("Performance: Multiple HTTP requests")
    
    sus client HttpClient = web_vibez.create_client()
    sus start_time normie = timez.now_nanos()
    
    # Simulate multiple requests
    sus i normie = 0
    bestie i = 0; i < 10; i = i + 1 {
        sus response HttpResponse = web_vibez.client_get(client, "http://example.com/api/data")
        assert_eq_int(response.status_code, 200)
        
        # Record metrics
        web_vibez.record_request(based, 100 + i * 10, 1024, 2048)
    }
    
    sus end_time normie = timez.now_nanos()
    sus total_time normie = end_time - start_time
    
    # Verify metrics
    sus metrics HttpMetrics = web_vibez.get_metrics()
    assert_ge_int(metrics.total_requests, 10)
    assert_ge_int(metrics.successful_requests, 10)
    assert_eq_int(metrics.failed_requests, 0)
    
    vibez.spill("Performance test completed in " + tea(total_time) + " nanoseconds")
    vibez.spill("Average response time: " + tea(metrics.average_response_time) + "ns")
    
    test_end()
}

# ===============================
# Error Handling Tests
# ===============================

slay test_error_handling() {
    test_start("HTTP error handling and edge cases")
    
    # Test invalid URL parsing
    sus invalid_url HttpUrl = web_vibez.parse_url("")
    assert_eq_string(invalid_url.host, "localhost")  # Should fallback to default
    
    # Test oversized headers (should not crash)
    sus headers HttpHeaders = web_vibez.init_headers()
    sus i normie = 0
    bestie i = 0; i < 100; i = i + 1 {  # Try to add more than the limit
        headers = web_vibez.add_header(headers, "Header" + tea(i), "Value" + tea(i))
    }
    assert_le_int(headers.count, 50)  # Should not exceed the limit
    
    # Test malformed header sanitization
    sus clean_header tea = web_vibez.sanitize_header_value("normal\r\nmalicious\nheader")
    assert_not_contains(clean_header, "\r")
    assert_not_contains(clean_header, "\n")
    
    # Test server start when already running
    sus server HttpServer = web_vibez.create_server("localhost", 9090)
    web_vibez.server_start(server)
    sus duplicate_start lit = web_vibez.server_start(server)
    assert_false(duplicate_start)  # Should fail when already running
    
    web_vibez.server_stop(server)
    
    test_end()
}

# ===============================
# Main Test Runner
# ===============================

slay run_all_tests() {
    vibez.spill("🚀 Starting comprehensive web_vibez HTTP module tests")
    vibez.spill("====================================================")
    
    # Core functionality tests
    test_http_headers()
    test_http_url_parsing()
    test_http_request_creation()
    test_http_response_creation()
    test_http_status_codes()
    
    # Client and server tests
    test_http_client()
    test_http_server()
    test_http_middleware()
    test_http_cookies()
    test_http_routing()
    
    # Utility and advanced features
    test_http_utilities()
    test_websocket_support()
    test_security_functions()
    test_performance_monitoring()
    test_http2_support()
    test_public_api()
    
    # Integration tests
    test_integration_client_server()
    test_integration_full_http_cycle()
    
    # Performance and error handling
    test_performance_multiple_requests()
    test_error_handling()
    
    print_test_summary()
    
    fr fr all_tests_passed() {
        vibez.spill("🎉 All web_vibez HTTP module tests passed!")
        vibez.spill("✅ HTTP client and server functionality verified")
        vibez.spill("✅ WebSocket support tested")
        vibez.spill("✅ Security features validated")
        vibez.spill("✅ Performance monitoring working")
        vibez.spill("✅ Integration tests successful")
    } else {
        vibez.spill("❌ Some tests failed - check implementation")
    }
}

# Run all tests
run_all_tests()
