yeet "testz"
yeet "web_vibez"

fr fr ========================================
fr fr CURSED Web Framework - Comprehensive Test Suite
fr fr Production-Grade HTTP Testing
fr fr ========================================

fr fr Test HTTP Status Code Mapping (Extended)
test_start("status_code_text extended tests")
assert_eq_string(status_code_text(200), "OK")
assert_eq_string(status_code_text(201), "Created")
assert_eq_string(status_code_text(204), "No Content")
assert_eq_string(status_code_text(301), "Moved Permanently")
assert_eq_string(status_code_text(302), "Found")
assert_eq_string(status_code_text(304), "Not Modified")
assert_eq_string(status_code_text(400), "Bad Request")
assert_eq_string(status_code_text(401), "Unauthorized")
assert_eq_string(status_code_text(403), "Forbidden")
assert_eq_string(status_code_text(404), "Not Found")
assert_eq_string(status_code_text(405), "Method Not Allowed")
assert_eq_string(status_code_text(409), "Conflict")
assert_eq_string(status_code_text(422), "Unprocessable Entity")
assert_eq_string(status_code_text(429), "Too Many Requests")
assert_eq_string(status_code_text(500), "Internal Server Error")
assert_eq_string(status_code_text(501), "Not Implemented")
assert_eq_string(status_code_text(502), "Bad Gateway")
assert_eq_string(status_code_text(503), "Service Unavailable")
assert_eq_string(status_code_text(504), "Gateway Timeout")
assert_eq_string(status_code_text(999), "Unknown Status")

fr fr Test Enhanced Header Parsing
test_start("parse_headers enhanced tests")
assert_false(parse_headers(""))
assert_true(parse_headers("Content-Type: application/json"))
assert_true(parse_headers("Accept: text/html"))
assert_true(parse_headers("Authorization: Bearer token123"))
assert_false(parse_headers("Invalid Header Format"))
assert_true(parse_headers("User-Agent: CURSED-Client"))

fr fr Test Multi-Header Parsing
test_start("parse_multi_headers tests")
assert_eq_int(parse_multi_headers(""), 0)
assert_eq_int(parse_multi_headers("Content-Type: application/json"), 1)
sus multi_headers tea = "Content-Type: application/json\nAuthorization: Bearer token\nAccept: text/html"
assert_eq_int(parse_multi_headers(multi_headers), 3)

fr fr Test Cookie Parsing
test_start("parse_cookies tests")
assert_eq_int(parse_cookies(""), 0)
assert_eq_int(parse_cookies("session=abc123"), 1)
assert_eq_int(parse_cookies("session=abc123; user=john; theme=dark"), 3)

fr fr Test Enhanced HTTP GET Requests
test_start("http_get enhanced tests")
sus get_response tea = http_get("https://example.com")
assert_true(get_response.contains("200 OK"))
assert_true(get_response.contains("CURSED-WebVibez"))
assert_true(get_response.contains("Cache-Control"))
assert_true(get_response.contains("Hello from CURSED!"))

sus invalid_get tea = http_get("")
assert_true(invalid_get.contains("Error: Empty URL"))

sus invalid_protocol tea = http_get("ftp://example.com")
assert_true(invalid_protocol.contains("Error: Invalid URL protocol"))

sus short_url tea = http_get("http://a")
assert_true(short_url.contains("Error: URL too short"))

fr fr Test Enhanced HTTP POST Requests
test_start("http_post enhanced tests")
sus post_data tea = "{\"name\": \"CURSED\", \"version\": \"1.0\"}"
sus post_response tea = http_post("https://api.example.com", post_data)
assert_true(post_response.contains("201 Created"))
assert_true(post_response.contains("Location:"))
assert_true(post_response.contains("CURSED-WebVibez"))
assert_true(post_response.contains(post_data))

fr fr Test HTTP PUT Requests
test_start("http_put tests")
sus put_data tea = "{\"updated\": \"data\"}"
sus put_response tea = http_put("https://api.example.com/resource", put_data)
assert_true(put_response.contains("200 OK"))
assert_true(put_response.contains(put_data))

fr fr Test HTTP DELETE Requests
test_start("http_delete tests")
sus delete_response tea = http_delete("https://api.example.com/resource")
assert_true(delete_response.contains("204 No Content"))
assert_true(delete_response.contains("CURSED-WebVibez"))

fr fr Test HTTP PATCH Requests
test_start("http_patch tests")
sus patch_data tea = "{\"field\": \"updated_value\"}"
sus patch_response tea = http_patch("https://api.example.com/resource", patch_data)
assert_true(patch_response.contains("200 OK"))
assert_true(patch_response.contains(patch_data))

fr fr Test Advanced HTTP Request Method
test_start("http_request tests")
sus request_response tea = http_request("GET", "https://example.com", "", "")
assert_true(request_response.contains("200 OK"))

sus invalid_method tea = http_request("INVALID", "https://example.com", "", "")
assert_true(invalid_method.contains("Error: Invalid HTTP method"))

fr fr Test Enhanced Server Creation
test_start("create_server enhanced tests")
sus server_port := create_server(3000)
assert_eq_int(server_port, 3000)

sus invalid_port := create_server(70000)
assert_eq_int(invalid_port, 8080) fr fr Should default to 8080

sus default_port := create_server(0)
assert_eq_int(default_port, 8080)

fr fr Test Router System
test_start("router system tests")
sus router := create_router()
assert_eq_int(router, 0)

assert_true(add_route(router, "/api/users", "GET", "handler"))
assert_false(add_route(router, "", "GET", "handler"))
assert_false(add_route(router, "/api/users", "INVALID", "handler"))

fr fr Test Route Matching
test_start("route matching tests")
assert_true(match_route("/api/users", "/api/users"))
assert_false(match_route("/api/users", "/api/posts"))
assert_true(match_route("/api/users/123", "/api/users/*"))
assert_true(match_route("/api/users/123", "/api/users/{id}"))

fr fr Test Enhanced URL Path Parsing
test_start("parse_url_path enhanced tests")
assert_eq_string(parse_url_path(""), "/")
assert_eq_string(parse_url_path("https://example.com"), "/")
assert_eq_string(parse_url_path("https://example.com/api/users"), "/api/users")
assert_eq_string(parse_url_path("https://example.com/api/users/123"), "/api/users/123")
assert_eq_string(parse_url_path("/api/users"), "/api/users")

fr fr Test Enhanced Query Parameter Parsing
test_start("query parameter tests")
assert_eq_string(parse_query_params("https://example.com"), "")
assert_eq_string(parse_query_params("https://example.com?param=value"), "param=value")
assert_eq_string(parse_query_params("https://example.com?a=1&b=2"), "a=1&b=2")

assert_eq_string(get_query_param("https://example.com?name=john&age=30", "name"), "john")
assert_eq_string(get_query_param("https://example.com?name=john&age=30", "age"), "30")
assert_eq_string(get_query_param("https://example.com?name=john&age=30", "missing"), "")

fr fr Test Enhanced HTTP Method Validation
test_start("validate_method enhanced tests")
assert_true(validate_method("GET"))
assert_true(validate_method("POST"))
assert_true(validate_method("PUT"))
assert_true(validate_method("DELETE"))
assert_true(validate_method("PATCH"))
assert_true(validate_method("HEAD"))
assert_true(validate_method("OPTIONS"))
assert_false(validate_method("INVALID"))
assert_false(validate_method(""))

fr fr Test Enhanced Content Type Detection
test_start("detect_content_type enhanced tests")
assert_eq_string(detect_content_type("{\"key\": \"value\"}"), "application/json")
assert_eq_string(detect_content_type("[1, 2, 3]"), "application/json")
assert_eq_string(detect_content_type("<?xml version=\"1.0\"?>"), "application/xml")
assert_eq_string(detect_content_type("<!DOCTYPE html>"), "text/html")
assert_eq_string(detect_content_type("<html>"), "text/html")
assert_eq_string(detect_content_type("data:image/png;base64,"), "application/octet-stream")
assert_eq_string(detect_content_type("name=value&other=data"), "application/x-www-form-urlencoded")
assert_eq_string(detect_content_type("-----BEGIN CERTIFICATE-----"), "application/x-pem-file")
assert_eq_string(detect_content_type("plain text"), "text/plain")

fr fr Test MIME Type Registry
test_start("get_mime_type tests")
assert_eq_string(get_mime_type("html"), "text/html")
assert_eq_string(get_mime_type("css"), "text/css")
assert_eq_string(get_mime_type("js"), "application/javascript")
assert_eq_string(get_mime_type("json"), "application/json")
assert_eq_string(get_mime_type("png"), "image/png")
assert_eq_string(get_mime_type("jpg"), "image/jpeg")
assert_eq_string(get_mime_type("unknown"), "application/octet-stream")

fr fr Test Enhanced HTTP Response Builder
test_start("build_response enhanced tests")
sus response tea = build_response(200, "Success")
assert_true(response.contains("HTTP/1.1 200 OK"))
assert_true(response.contains("Content-Type: text/plain"))
assert_true(response.contains("Server: CURSED-WebVibez/1.0"))
assert_true(response.contains("Date:"))
assert_true(response.contains("Connection: close"))
assert_true(response.contains("Success"))

fr fr Test Response Builder with Custom Headers
test_start("build_response_with_headers tests")
sus custom_headers tea = "X-Custom-Header: value\r\nX-Another-Header: another\r\n"
sus custom_response tea = build_response_with_headers(200, "Custom", custom_headers)
assert_true(custom_response.contains("X-Custom-Header: value"))
assert_true(custom_response.contains("X-Another-Header: another"))

fr fr Test JSON Response Builder
test_start("build_json_response tests")
sus json_response tea = build_json_response(200, "success")
assert_true(json_response.contains("application/json"))
assert_true(json_response.contains("\"data\": \"success\""))

fr fr Test Enhanced Request Validation
test_start("validate_request enhanced tests")
assert_true(validate_request("GET", "https://example.com"))
assert_false(validate_request("INVALID", "https://example.com"))
assert_false(validate_request("GET", ""))

fr fr Test very long URL (should fail)
sus long_url tea = "https://example.com/" + "a".repeat(2000)
assert_false(validate_request("GET", long_url))

fr fr Test Enhanced Error Response Builder
test_start("build_error_response enhanced tests")
sus error_response tea = build_error_response(404, "Not Found")
assert_true(error_response.contains("HTTP/1.1 404 Not Found"))
assert_true(error_response.contains("application/json"))
assert_true(error_response.contains("\"error\": \"Not Found\""))
assert_true(error_response.contains("\"status\": 404"))

fr fr Test Request Logging
test_start("request logging tests")
log_request("GET", "/api/users", 200)
log_request("POST", "/api/users", 201)
log_request("DELETE", "/api/users/1", 204)
log_request_detailed("GET", "/api/users", 200, "CURSED-Client/1.0", "192.168.1.1")

fr fr Test Middleware System
test_start("middleware system tests")
sus middleware := create_middleware("auth")
assert_true(middleware)

sus empty_middleware := create_middleware("")
assert_false(empty_middleware)

sus processed_request tea = apply_middleware(middleware, "test request")
assert_true(processed_request.contains("Middleware processed"))

fr fr Test CORS Support
test_start("cors support tests")
sus base_response tea = build_response(200, "OK")
sus cors_response tea = add_cors_headers(base_response)
assert_true(cors_response.contains("Access-Control-Allow-Origin: *"))
assert_true(cors_response.contains("Access-Control-Allow-Methods"))
assert_true(cors_response.contains("Access-Control-Allow-Headers"))

fr fr Test Rate Limiting
test_start("rate limiting tests")
sus rate_limit := create_rate_limit(100)
assert_eq_int(rate_limit, 100)

sus default_rate_limit := create_rate_limit(0)
assert_eq_int(default_rate_limit, 60)

assert_true(check_rate_limit(rate_limit, "192.168.1.1"))
assert_false(check_rate_limit(rate_limit, ""))

fr fr Test URL Encoding/Decoding
test_start("url encoding tests")
assert_eq_string(url_encode("hello world"), "hello%20world")
assert_eq_string(url_encode("a&b=c?d#e"), "a%26b%3Dc%3Fd%23e")

assert_eq_string(url_decode("hello%20world"), "hello world")
assert_eq_string(url_decode("a%26b%3Dc%3Fd%23e"), "a&b=c?d#e")

fr fr Test Session Management
test_start("session management tests")
sus session := create_session("user123")
assert_true(session.contains("session_user123"))
assert_true(validate_session(session))

sus empty_session := create_session("")
assert_eq_string(empty_session, "")
assert_false(validate_session(""))
assert_false(validate_session("invalid"))

fr fr Test Security Headers
test_start("security headers tests")
sus secure_response tea = add_security_headers(base_response)
assert_true(secure_response.contains("X-Content-Type-Options: nosniff"))
assert_true(secure_response.contains("X-Frame-Options: DENY"))
assert_true(secure_response.contains("X-XSS-Protection"))
assert_true(secure_response.contains("Strict-Transport-Security"))

fr fr Test HTTP Compression
test_start("compression tests")
sus compressed_response tea = compress_response(base_response, "gzip")
assert_true(compressed_response.contains("Content-Encoding: gzip"))

fr fr Test Cache Control
test_start("cache control tests")
sus cached_response tea = add_cache_headers(base_response, 3600)
assert_true(cached_response.contains("Cache-Control: max-age=3600"))

fr fr Test Static File Serving
test_start("static file serving tests")
sus static_html tea = serve_static_file("index.html")
assert_true(static_html.contains("text/html"))
assert_true(static_html.contains("Cache-Control: max-age=3600"))

sus static_css tea = serve_static_file("styles.css")
assert_true(static_css.contains("text/css"))

sus static_js tea = serve_static_file("app.js")
assert_true(static_js.contains("application/javascript"))

fr fr Test WebSocket Support
test_start("websocket support tests")
sus ws_request tea = "GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: upgrade\r\n"
sus ws_response tea = handle_websocket_upgrade(ws_request)
assert_true(ws_response.contains("101 Switching Protocols"))
assert_true(ws_response.contains("Upgrade: websocket"))

sus invalid_ws tea = handle_websocket_upgrade("GET / HTTP/1.1\r\n")
assert_true(invalid_ws.contains("400 Bad Request"))

fr fr Test HTTP/2 Support
test_start("http2 support tests")
assert_true(supports_http2())

fr fr Test Health Check
test_start("health check tests")
sus health_response tea = health_check()
assert_true(health_response.contains("healthy"))
assert_true(health_response.contains("timestamp"))
assert_true(health_response.contains("version"))

fr fr Test Metrics Endpoint
test_start("metrics endpoint tests")
sus metrics_response tea = metrics_endpoint()
assert_true(metrics_response.contains("requests_total"))
assert_true(metrics_response.contains("response_time_avg"))
assert_true(metrics_response.contains("errors_total"))

fr fr Test Production Request Handler
test_start("production request handler tests")
sus home_response tea = handle_production_request("GET", "/", "", "")
assert_true(home_response.contains("Welcome to CURSED WebVibez!"))

sus health_prod tea = handle_production_request("GET", "/health", "", "")
assert_true(health_prod.contains("healthy"))

sus metrics_prod tea = handle_production_request("GET", "/metrics", "", "")
assert_true(metrics_prod.contains("requests_total"))

sus api_get tea = handle_production_request("GET", "/api/users", "", "")
assert_true(api_get.contains("API GET response"))

sus api_post tea = handle_production_request("POST", "/api/users", "{}", "")
assert_true(api_post.contains("API POST response"))

sus api_invalid tea = handle_production_request("PATCH", "/api/users", "", "")
assert_true(api_invalid.contains("Method not allowed"))

sus static_file tea = handle_production_request("GET", "/static/app.js", "", "")
assert_true(static_file.contains("application/javascript"))

sus not_found tea = handle_production_request("GET", "/nonexistent", "", "")
assert_true(not_found.contains("404 Not Found"))

sus invalid_request tea = handle_production_request("INVALID", "/", "", "")
assert_true(invalid_request.contains("400 Bad Request"))

fr fr Performance and Stress Tests
test_start("performance tests")
bestie i := 0; i < 10; i++ {
    sus perf_response tea = http_get("https://example.com/api/test")
    assert_true(perf_response.contains("200 OK"))
}

fr fr Test Large Request Handling
test_start("large request tests")
sus large_data tea = "data".repeat(1000)
sus large_response tea = http_post("https://example.com/api/large", large_data)
assert_true(large_response.contains("201 Created"))
assert_true(large_response.contains("4000")) fr fr Content-Length

fr fr Test Edge Cases
test_start("edge case tests")
sus edge_response tea = http_get("https://example.com/api/edge?param=value&other=data")
assert_true(edge_response.contains("200 OK"))

sus malformed_url tea = http_get("https://")
assert_true(malformed_url.contains("Error: URL too short"))

fr fr Integration Tests
test_start("integration tests")
sus integration_response tea = handle_production_request("POST", "/api/users", "{\"name\":\"test\"}", "Content-Type: application/json")
assert_true(integration_response.contains("201 Created"))

print_test_summary()
