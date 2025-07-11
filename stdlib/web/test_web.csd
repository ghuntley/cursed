yeet "testz"
yeet "web"

# Web Module Comprehensive Test Suite
# Testing all web framework functionality

test_start("Web Server Management Tests")

# Test server creation
assert_eq_int(web_server_create(8080), 1)
assert_eq_int(web_server_create(0), -1)
assert_eq_int(web_server_create(70000), -1)

# Test server operations
assert_true(web_server_start(1))
assert_true(web_server_listen(1, "127.0.0.1"))
assert_true(web_server_stop(1))
assert_false(web_server_start(-1))
assert_false(web_server_listen(-1, "127.0.0.1"))
assert_false(web_server_listen(1, ""))

print_test_summary()

test_start("Routing Functionality Tests")

# Test route management
assert_true(web_route_add(1, HTTP_GET, "/test", "test_handler"))
assert_true(web_route_add(1, HTTP_POST, "/api/users", "user_handler"))
assert_false(web_route_add(-1, HTTP_GET, "/test", "handler"))
assert_false(web_route_add(1, 0, "/test", "handler"))
assert_false(web_route_add(1, HTTP_GET, "", "handler"))
assert_false(web_route_add(1, HTTP_GET, "/test", ""))

# Test route matching
assert_eq_string(web_route_match(1, HTTP_GET, "/test"), "default_handler")
assert_eq_string(web_route_match(-1, HTTP_GET, "/test"), "")
assert_eq_string(web_route_match(1, 0, "/test"), "")
assert_eq_string(web_route_match(1, HTTP_GET, ""), "")

# Test route removal
assert_true(web_route_remove(1, HTTP_GET, "/test"))
assert_false(web_route_remove(-1, HTTP_GET, "/test"))
assert_false(web_route_remove(1, 0, "/test"))
assert_false(web_route_remove(1, HTTP_GET, ""))

print_test_summary()

test_start("Request Handling Tests")

# Test request creation
assert_eq_int(web_request_create(HTTP_GET, "/test", "{}", ""), 1)
assert_eq_int(web_request_create(0, "/test", "{}", ""), -1)
assert_eq_int(web_request_create(HTTP_GET, "", "{}", ""), -1)

# Test request data retrieval
assert_eq_int(web_request_get_method(1), HTTP_GET)
assert_eq_int(web_request_get_method(-1), -1)
assert_eq_string(web_request_get_path(1), "/test")
assert_eq_string(web_request_get_path(-1), "")
assert_eq_string(web_request_get_header(1, "Content-Type"), "header_value")
assert_eq_string(web_request_get_header(-1, "Content-Type"), "")
assert_eq_string(web_request_get_header(1, ""), "")
assert_eq_string(web_request_get_body(1), "request_body")
assert_eq_string(web_request_get_body(-1), "")
assert_eq_string(web_request_get_param(1, "id"), "param_value")
assert_eq_string(web_request_get_param(-1, "id"), "")
assert_eq_string(web_request_get_param(1, ""), "")

print_test_summary()

test_start("Response Handling Tests")

# Test response creation
assert_eq_int(web_response_create(HTTP_OK, "{}", "test body"), 1)
assert_eq_int(web_response_create(50, "{}", "test body"), -1)
assert_eq_int(web_response_create(700, "{}", "test body"), -1)

# Test response modifications
assert_true(web_response_set_status(1, HTTP_NOT_FOUND))
assert_false(web_response_set_status(-1, HTTP_OK))
assert_false(web_response_set_status(1, 50))
assert_true(web_response_set_header(1, "Content-Type", "application/json"))
assert_false(web_response_set_header(-1, "Content-Type", "application/json"))
assert_false(web_response_set_header(1, "", "application/json"))
assert_false(web_response_set_header(1, "Content-Type", ""))
assert_true(web_response_set_body(1, "new body"))
assert_false(web_response_set_body(-1, "new body"))
assert_true(web_response_send(1))
assert_false(web_response_send(-1))

print_test_summary()

test_start("Middleware Support Tests")

# Test middleware management
assert_true(web_middleware_add(1, "auth_middleware", 1))
assert_true(web_middleware_add(1, "cors_middleware", 2))
assert_false(web_middleware_add(-1, "auth_middleware", 1))
assert_false(web_middleware_add(1, "", 1))
assert_false(web_middleware_add(1, "auth_middleware", -1))

# Test middleware execution
assert_true(web_middleware_execute(1, 1, 1))
assert_false(web_middleware_execute(-1, 1, 1))
assert_false(web_middleware_execute(1, -1, 1))
assert_false(web_middleware_execute(1, 1, -1))

# Test middleware removal
assert_true(web_middleware_remove(1, "auth_middleware"))
assert_false(web_middleware_remove(-1, "auth_middleware"))
assert_false(web_middleware_remove(1, ""))

print_test_summary()

test_start("Session Management Tests")

# Test session operations
assert_true(web_session_create("session123"))
assert_false(web_session_create(""))
assert_eq_string(web_session_get("session123", "user_id"), "session_value")
assert_eq_string(web_session_get("", "user_id"), "")
assert_eq_string(web_session_get("session123", ""), "")
assert_true(web_session_set("session123", "user_id", "42"))
assert_false(web_session_set("", "user_id", "42"))
assert_false(web_session_set("session123", "", "42"))
assert_true(web_session_destroy("session123"))
assert_false(web_session_destroy(""))

print_test_summary()

test_start("Cookie Support Tests")

# Test cookie operations
assert_true(web_cookie_set(1, "session", "abc123", "2024-12-31"))
assert_false(web_cookie_set(-1, "session", "abc123", "2024-12-31"))
assert_false(web_cookie_set(1, "", "abc123", "2024-12-31"))
assert_eq_string(web_cookie_get(1, "session"), "cookie_value")
assert_eq_string(web_cookie_get(-1, "session"), "")
assert_eq_string(web_cookie_get(1, ""), "")
assert_true(web_cookie_delete(1, "session"))
assert_false(web_cookie_delete(-1, "session"))
assert_false(web_cookie_delete(1, ""))

print_test_summary()

test_start("Template Rendering Tests")

# Test template operations
assert_eq_int(web_template_load("template.html"), 1)
assert_eq_int(web_template_load(""), -1)
assert_eq_string(web_template_render(1, "{}"), "<html><body>Rendered Template</body></html>")
assert_eq_string(web_template_render(-1, "{}"), "")
assert_eq_string(web_template_render_string("Hello {{name}}", "{\"name\": \"World\"}"), "Rendered: Hello {{name}}")
assert_eq_string(web_template_render_string("", "{}"), "")

print_test_summary()

test_start("Static File Serving Tests")

# Test static file operations
assert_true(web_static_serve(1, "/static", "./public"))
assert_false(web_static_serve(-1, "/static", "./public"))
assert_false(web_static_serve(1, "", "./public"))
assert_false(web_static_serve(1, "/static", ""))

print_test_summary()

test_start("URL Utilities Tests")

# Test URL operations
assert_eq_string(web_url_parse("https://example.com/test"), "{\"scheme\": \"https\", \"host\": \"example.com\", \"path\": \"/test\"}")
assert_eq_string(web_url_parse(""), "")
assert_eq_string(web_url_encode("hello world"), "hello world")
assert_eq_string(web_url_encode(""), "")
assert_eq_string(web_url_decode("hello%20world"), "hello%20world")
assert_eq_string(web_url_decode(""), "")

print_test_summary()

test_start("CORS Support Tests")

# Test CORS operations
assert_true(web_cors_enable(1, "https://example.com"))
assert_false(web_cors_enable(-1, "https://example.com"))
assert_true(web_cors_set_headers(1, "GET,POST", "Content-Type,Authorization"))
assert_false(web_cors_set_headers(-1, "GET,POST", "Content-Type,Authorization"))

print_test_summary()

test_start("Security Headers Tests")

# Test security header operations
assert_true(web_security_set_csp(1, "default-src 'self'"))
assert_false(web_security_set_csp(-1, "default-src 'self'"))
assert_false(web_security_set_csp(1, ""))
assert_true(web_security_set_hsts(1, 31536000))
assert_false(web_security_set_hsts(-1, 31536000))
assert_false(web_security_set_hsts(1, -1))

print_test_summary()

test_start("WebSocket Support Tests")

# Test WebSocket operations
assert_true(web_websocket_upgrade(1, 1))
assert_false(web_websocket_upgrade(-1, 1))
assert_false(web_websocket_upgrade(1, -1))
assert_true(web_websocket_send(1, "test message"))
assert_false(web_websocket_send(-1, "test message"))
assert_eq_string(web_websocket_receive(1), "websocket_message")
assert_eq_string(web_websocket_receive(-1), "")
assert_true(web_websocket_close(1))
assert_false(web_websocket_close(-1))

print_test_summary()

test_start("HTTP Constants Tests")

# Test HTTP method constants
assert_eq_int(HTTP_GET, 1)
assert_eq_int(HTTP_POST, 2)
assert_eq_int(HTTP_PUT, 3)
assert_eq_int(HTTP_DELETE, 4)
assert_eq_int(HTTP_HEAD, 5)
assert_eq_int(HTTP_OPTIONS, 6)
assert_eq_int(HTTP_PATCH, 7)

# Test HTTP status constants
assert_eq_int(HTTP_OK, 200)
assert_eq_int(HTTP_CREATED, 201)
assert_eq_int(HTTP_BAD_REQUEST, 400)
assert_eq_int(HTTP_UNAUTHORIZED, 401)
assert_eq_int(HTTP_FORBIDDEN, 403)
assert_eq_int(HTTP_NOT_FOUND, 404)
assert_eq_int(HTTP_INTERNAL_ERROR, 500)

# Test content type constants
assert_eq_int(CONTENT_TYPE_JSON, 1)
assert_eq_int(CONTENT_TYPE_HTML, 2)
assert_eq_int(CONTENT_TYPE_TEXT, 3)
assert_eq_int(CONTENT_TYPE_XML, 4)

print_test_summary()
