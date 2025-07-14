yeet "testz"
yeet "web_vibez"

test_start("Simple web_vibez test")

# Test basic HTTP header creation
sus headers HttpHeaders = web_vibez.init_headers()
assert_eq_int(headers.count, 0)

headers = web_vibez.add_header(headers, "Content-Type", "application/json")
assert_eq_int(headers.count, 1)

sus content_type tea = web_vibez.get_header(headers, "Content-Type")
assert_eq_string(content_type, "application/json")

# Test HTTP request creation
sus request HttpRequest = web_vibez.create_request("GET", "http://example.com/api")
assert_eq_string(request.method, "GET")
assert_eq_string(request.url.host, "example.com")

# Test HTTP response creation
sus response HttpResponse = web_vibez.create_response(200)
assert_eq_int(response.status_code, 200)
assert_eq_string(response.status, "OK")

vibez.spill("Basic web_vibez functionality working!")

test_end()
print_test_summary()
