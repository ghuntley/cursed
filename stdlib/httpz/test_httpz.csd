yeet "testz"
yeet "httpz"

test_start("HTTP Module Tests")

fr fr Test HTTP GET request
test_start("http_get - successful request")
sus response HTTPResponse = http_get("http://example.com")
assert_eq_int(response.status_code, 200)
assert_true(response.error == "")
assert_true(str_contains(response.body, "Example"))
test_pass("HTTP GET request successful")

fr fr Test HTTP GET with invalid URL
test_start("http_get - invalid URL")
sus invalid_response HTTPResponse = http_get("")
assert_eq_int(invalid_response.status_code, 0)
assert_true(invalid_response.error != "")
test_pass("HTTP GET handles invalid URL")

fr fr Test HTTP GET with 404 response
test_start("http_get - 404 response")
sus not_found HTTPResponse = http_get("http://example.com/404")
assert_eq_int(not_found.status_code, 404)
assert_true(str_contains(not_found.body, "Not Found"))
test_pass("HTTP GET handles 404 response")

fr fr Test HTTP POST request
test_start("http_post - successful request")
sus post_response HTTPResponse = http_post("http://httpbin.org/post", "test=data")
assert_eq_int(post_response.status_code, 200)
assert_true(post_response.error == "")
assert_true(str_contains(post_response.body, "test=data"))
test_pass("HTTP POST request successful")

fr fr Test HTTP POST with empty data
test_start("http_post - empty data")
sus empty_post HTTPResponse = http_post("http://example.com", "")
assert_eq_int(empty_post.status_code, 400)
assert_true(empty_post.error != "")
test_pass("HTTP POST handles empty data")

fr fr Test HTTP POST with local server
test_start("http_post - local server")
sus local_post HTTPResponse = http_post("http://localhost:8080/api", "name=test&value=123")
assert_eq_int(local_post.status_code, 201)
assert_true(str_contains(local_post.body, "created"))
test_pass("HTTP POST to local server")

fr fr Test HTTP request creation
test_start("http_request_create")
sus request HTTPRequest = http_request_create("GET", "http://example.com")
assert_eq_string(request.method, "GET")
assert_eq_string(request.url, "http://example.com")
assert_eq_string(request.headers, "")
test_pass("HTTP request creation successful")

fr fr Test adding headers to request
test_start("http_request_add_header")
sus header_request HTTPRequest = http_request_create("GET", "http://example.com")
http_request_add_header(&header_request, "User-Agent", "CURSED-Client/1.0")
http_request_add_header(&header_request, "Accept", "application/json")
assert_true(str_contains(header_request.headers, "User-Agent: CURSED-Client/1.0"))
assert_true(str_contains(header_request.headers, "Accept: application/json"))
test_pass("HTTP header addition successful")

fr fr Test setting request body
test_start("http_request_set_body")
sus body_request HTTPRequest = http_request_create("POST", "http://example.com")
http_request_set_body(&body_request, "test body content")
assert_eq_string(body_request.body, "test body content")
test_pass("HTTP request body setting successful")

fr fr Test response success check
test_start("http_is_success")
sus success_response HTTPResponse = http_get("http://example.com")
assert_true(http_is_success(success_response))
sus error_response HTTPResponse = http_get("http://example.com/404")
assert_false(http_is_success(error_response))
test_pass("HTTP success check works correctly")

fr fr Test response error check
test_start("http_is_error")
sus good_response HTTPResponse = http_get("http://example.com")
assert_false(http_is_error(good_response))
sus bad_response HTTPResponse = http_get("http://example.com/404")
assert_true(http_is_error(bad_response))
test_pass("HTTP error check works correctly")

fr fr Test status text function
test_start("http_status_text")
assert_eq_string(http_status_text(200), "OK")
assert_eq_string(http_status_text(404), "Not Found")
assert_eq_string(http_status_text(500), "Internal Server Error")
test_pass("HTTP status text function works")

fr fr Test content type extraction
test_start("http_get_content_type")
sus typed_response HTTPResponse = http_get("http://httpbin.org/get")
sus content_type tea = http_get_content_type(typed_response)
assert_true(str_contains(content_type, "json"))
test_pass("HTTP content type extraction works")

fr fr Test URL validation
test_start("is_valid_url")
assert_true(is_valid_url("http://example.com"))
assert_true(is_valid_url("https://secure.example.com"))
assert_false(is_valid_url(""))
assert_false(is_valid_url("not-a-url"))
test_pass("URL validation works correctly")

fr fr Test timeout scenario
test_start("http_get - timeout")
sus timeout_response HTTPResponse = http_get("http://timeout.example.com")
assert_eq_int(timeout_response.status_code, 0)
assert_true(str_contains(timeout_response.error, "timeout"))
test_pass("HTTP timeout handling works")

print_test_summary()
