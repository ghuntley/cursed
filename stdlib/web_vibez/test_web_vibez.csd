yeet "testz"
yeet "web_vibez"

# Test HTTP Status Code Mapping
test_start("status_code_text tests")
assert_eq_string(status_code_text(200), "OK")
assert_eq_string(status_code_text(404), "Not Found")
assert_eq_string(status_code_text(500), "Internal Server Error")
assert_eq_string(status_code_text(999), "Unknown Status")

# Test Header Parsing
test_start("parse_headers tests")
assert_false(parse_headers(""))
assert_true(parse_headers("Content-Type: text/html"))
assert_false(parse_headers("Invalid Header"))

# Test HTTP GET Requests
test_start("http_get tests")
sus get_response tea = http_get("http://example.com")
assert_true(get_response.contains("200 OK"))
assert_true(get_response.contains("Hello, World!"))

sus invalid_get tea = http_get("")
assert_true(invalid_get.contains("Error: Empty URL"))

sus invalid_protocol tea = http_get("ftp://example.com")
assert_true(invalid_protocol.contains("Error: Invalid URL protocol"))

# Test HTTP POST Requests
test_start("http_post tests")
sus post_data tea = "{\"name\": \"test\"}"
sus post_response tea = http_post("https://api.example.com", post_data)
assert_true(post_response.contains("201 Created"))
assert_true(post_response.contains(post_data))

sus invalid_post tea = http_post("", "data")
assert_true(invalid_post.contains("Error: Empty URL"))

sus invalid_post_protocol tea = http_post("ftp://example.com", "data")
assert_true(invalid_post_protocol.contains("Error: Invalid URL protocol"))

# Test Server Creation
test_start("create_server tests")
sus server_config := create_server()
assert_true(server_config)

# Test URL Path Parsing
test_start("parse_url_path tests")
assert_eq_string(parse_url_path(""), "/")
assert_eq_string(parse_url_path("http://example.com"), "/")
assert_eq_string(parse_url_path("https://example.com/api/users"), "/api/users")

# Test HTTP Method Validation
test_start("validate_method tests")
assert_true(validate_method("GET"))
assert_true(validate_method("POST"))
assert_true(validate_method("PUT"))
assert_true(validate_method("DELETE"))
assert_false(validate_method("INVALID"))
assert_false(validate_method(""))

# Test Content Type Detection
test_start("detect_content_type tests")
assert_eq_string(detect_content_type("{\"key\": \"value\"}"), "application/json")
assert_eq_string(detect_content_type("<html></html>"), "text/html")
assert_eq_string(detect_content_type("plain text"), "text/plain")

# Test HTTP Response Builder
test_start("build_response tests")
sus response tea = build_response(200, "Success")
assert_true(response.contains("HTTP/1.1 200 OK"))
assert_true(response.contains("Content-Type: text/plain"))
assert_true(response.contains("Success"))

# Test Query Parameter Detection
test_start("parse_query_params tests")
assert_true(parse_query_params("http://example.com?param=value"))
assert_false(parse_query_params("http://example.com"))

# Test Request Validation
test_start("validate_request tests")
assert_true(validate_request("GET", "http://example.com"))
assert_false(validate_request("INVALID", "http://example.com"))
assert_false(validate_request("GET", ""))

# Test Error Response Builder
test_start("build_error_response tests")
sus error_response tea = build_error_response(404, "Not Found")
assert_true(error_response.contains("HTTP/1.1 404 Not Found"))
assert_true(error_response.contains("application/json"))
assert_true(error_response.contains("Not Found"))

# Test Request Logging (visual verification)
test_start("log_request tests")
log_request("GET", "/api/users", 200)
log_request("POST", "/api/users", 201)
log_request("DELETE", "/api/users/1", 204)

print_test_summary()
