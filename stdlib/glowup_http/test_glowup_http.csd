# glowup_http Test Suite
# Comprehensive tests for HTTP client/server functionality

yeet "testz"
yeet "glowup_http"

# Test HTTP Status Codes
test_start("HTTP Status Code Tests")
assert_eq_int(status_ok(), 200)
assert_eq_int(status_not_found(), 404)
assert_eq_int(status_internal_error(), 500)
assert_eq_int(status_bad_request(), 400)

# Test HTTP Method Validation
test_start("HTTP Method Validation")
assert_true(is_valid_method("GET"))
assert_true(is_valid_method("POST"))
assert_true(is_valid_method("PUT"))
assert_true(is_valid_method("DELETE"))
assert_false(is_valid_method("INVALID"))
assert_false(is_valid_method(""))

# Test HTTP Request Builder
test_start("HTTP Request Builder")
sus get_request tea = build_request("GET", "/test", "", "")
assert_true(len(get_request) > 0)
# Verify it contains essential components
assert_true(contains(get_request, "GET /test HTTP/1.1"))
assert_true(contains(get_request, "Host: localhost"))
assert_true(contains(get_request, "User-Agent: CURSED/1.0"))

sus post_request tea = build_request("POST", "/api/data", "", "{\"test\":\"data\"}")
assert_true(len(post_request) > 0)
assert_true(contains(post_request, "POST /api/data HTTP/1.1"))
assert_true(contains(post_request, "Content-Type: application/json"))

# Test HTTP Response Builder
test_start("HTTP Response Builder")
sus ok_response tea = build_response(200, "", "Hello World")
assert_true(contains(ok_response, "HTTP/1.1 200 OK"))
assert_true(contains(ok_response, "Server: CURSED/1.0"))
assert_true(contains(ok_response, "Hello World"))

sus not_found_response tea = build_response(404, "", "Not Found")
assert_true(contains(not_found_response, "HTTP/1.1 404 Not Found"))

# Test HTTP Header Parser
test_start("HTTP Header Parser")
sus (name, value) = parse_header("Content-Type: application/json")
assert_eq_string(name, "Content-Type")
assert_eq_string(value, "application/json")

sus (name2, value2) = parse_header("Authorization: Bearer token123")
assert_eq_string(name2, "Authorization")
assert_eq_string(value2, "Bearer token123")

# Test invalid header
sus (name3, value3) = parse_header("InvalidHeader")
assert_eq_string(name3, "")
assert_eq_string(value3, "")

# Test HTTP URL Parser
test_start("HTTP URL Parser")
sus (protocol, host, path) = parse_url("http://example.com/api/users")
assert_eq_string(protocol, "http")
assert_eq_string(host, "example.com")
assert_eq_string(path, "/api/users")

sus (protocol2, host2, path2) = parse_url("https://api.github.com/repos")
assert_eq_string(protocol2, "https")
assert_eq_string(host2, "api.github.com")
assert_eq_string(path2, "/repos")

# Test simple URL without path
sus (protocol3, host3, path3) = parse_url("http://localhost:8080")
assert_eq_string(protocol3, "http")
assert_eq_string(host3, "localhost:8080")
assert_eq_string(path3, "/")

# Test HTTP Client GET Request
test_start("HTTP Client GET Request")
sus get_response tea = http_get("http://example.com/test", "")
assert_true(len(get_response) > 0)
assert_true(contains(get_response, "HTTP/1.1 200 OK"))
assert_true(contains(get_response, "GET response from http://example.com/test"))

# Test HTTP Client POST Request
test_start("HTTP Client POST Request")
sus post_response tea = http_post("http://api.test.com/data", "", "{\"key\":\"value\"}")
assert_true(len(post_response) > 0)
assert_true(contains(post_response, "HTTP/1.1 200 OK"))
assert_true(contains(post_response, "POST response"))
assert_true(contains(post_response, "{\"key\":\"value\"}"))

# Test HTTP Client PUT Request
test_start("HTTP Client PUT Request")
sus put_response tea = http_put("http://api.test.com/update", "", "{\"updated\":true}")
assert_true(len(put_response) > 0)
assert_true(contains(put_response, "HTTP/1.1 200 OK"))
assert_true(contains(put_response, "PUT response"))

# Test HTTP Client DELETE Request
test_start("HTTP Client DELETE Request")
sus delete_response tea = http_delete("http://api.test.com/delete", "")
assert_true(len(delete_response) > 0)
assert_true(contains(delete_response, "HTTP/1.1 200 OK"))
assert_true(contains(delete_response, "DELETE response"))

# Test HTTP Server Route Handler
test_start("HTTP Server Route Handler")
sus home_response tea = handle_route("GET", "/", "")
assert_true(contains(home_response, "HTTP/1.1 200 OK"))
assert_true(contains(home_response, "Welcome to CURSED HTTP Server!"))

sus health_response tea = handle_route("GET", "/health", "")
assert_true(contains(health_response, "HTTP/1.1 200 OK"))
assert_true(contains(health_response, "Server is healthy"))

sus echo_response tea = handle_route("POST", "/echo", "test message")
assert_true(contains(echo_response, "HTTP/1.1 200 OK"))
assert_true(contains(echo_response, "Echo: test message"))

sus api_response tea = handle_route("GET", "/api/status", "")
assert_true(contains(api_response, "HTTP/1.1 200 OK"))
assert_true(contains(api_response, "application/json"))
assert_true(contains(api_response, "{\"status\":\"ok\""))

sus not_found tea = handle_route("GET", "/nonexistent", "")
assert_true(contains(not_found, "HTTP/1.1 404 Not Found"))

# Test HTTP Request Parser
test_start("HTTP Request Parser")
sus sample_request tea = "GET /test HTTP/1.1\r\nHost: localhost\r\n\r\n"
sus (method, path, body) = parse_request(sample_request)
assert_eq_string(method, "GET")
assert_eq_string(path, "/test")

sus post_sample tea = "POST /api/data HTTP/1.1\r\nHost: localhost\r\n\r\n{\"test\":true}"
sus (method2, path2, body2) = parse_request(post_sample)
assert_eq_string(method2, "POST")
assert_eq_string(path2, "/api/data")

# Test Content-Type Utilities
test_start("Content-Type Utilities")
assert_eq_string(content_type_json(), "application/json")
assert_eq_string(content_type_html(), "text/html")
assert_eq_string(content_type_plain(), "text/plain")

# Test HTTP Response Status Checker
test_start("HTTP Response Status Checker")
assert_true(is_success_status(200))
assert_true(is_success_status(201))
assert_true(is_success_status(204))
assert_false(is_success_status(404))
assert_false(is_success_status(500))

assert_true(is_client_error(400))
assert_true(is_client_error(404))
assert_true(is_client_error(401))
assert_false(is_client_error(200))
assert_false(is_client_error(500))

assert_true(is_server_error(500))
assert_true(is_server_error(503))
assert_false(is_server_error(200))
assert_false(is_server_error(404))

# Test HTTP Header Utilities
test_start("HTTP Header Utilities")
sus headers tea = add_header("", "Content-Type", "application/json")
assert_eq_string(headers, "Content-Type: application/json")

sus more_headers tea = add_header(headers, "Authorization", "Bearer token")
assert_true(contains(more_headers, "Content-Type: application/json"))
assert_true(contains(more_headers, "Authorization: Bearer token"))

sus basic_headers tea = create_basic_headers()
assert_true(contains(basic_headers, "Cache-Control: no-cache"))
assert_true(contains(basic_headers, "Accept-Encoding: gzip, deflate"))

# Test HTTP Cookie Utilities
test_start("HTTP Cookie Utilities")
sus cookie tea = create_cookie("session", "abc123", 3600)
assert_true(contains(cookie, "session=abc123"))
assert_true(contains(cookie, "Max-Age=3600"))
assert_true(contains(cookie, "Path=/"))
assert_true(contains(cookie, "HttpOnly"))

sus simple_cookie tea = create_cookie("user", "john", 0)
assert_true(contains(simple_cookie, "user=john"))
assert_true(contains(simple_cookie, "Path=/"))

# Test HTTP Authentication
test_start("HTTP Authentication")
sus auth_header tea = create_basic_auth("user", "pass")
assert_true(contains(auth_header, "Basic user:pass"))

sus auth_header2 tea = create_basic_auth("admin", "secret123")
assert_true(contains(auth_header2, "Basic admin:secret123"))

# Test HTTP Server Configuration
test_start("HTTP Server Configuration")
sus server_config tea = create_server_config(8080, 100)
assert_true(contains(server_config, "Port: 8080"))
assert_true(contains(server_config, "Max Connections: 100"))

# Test HTTP Client Configuration
test_start("HTTP Client Configuration")
sus client_config tea = create_client_config(30, 5)
assert_true(contains(client_config, "Timeout: 30s"))
assert_true(contains(client_config, "Max Redirects: 5"))

# Helper function for string contains check
slay contains(text tea, substring tea) lit {
    skip len(substring) == 0 {
        damn based
    }
    
    skip len(text) < len(substring) {
        damn cap
    }
    
    bestie i := 0; i <= len(text) - len(substring); i++ {
        sus found lit = based
        bestie j := 0; j < len(substring); j++ {
            skip text[i + j] != substring[j] {
                found = cap
                ghosted
            }
        }
        skip found {
            damn based
        }
    }
    damn cap
}

# Test Results Summary
print_test_summary()
