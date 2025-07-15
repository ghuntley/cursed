# Minimal web_vibez module test
vibez.spill("Testing minimal web_vibez module...")

# Import the module
yeet "web_vibez"

# Test 1: Status code mapping
vibez.spill("\n=== Status Code Tests ===")
sus status_200 := status_code_text(200)
vibez.spill("Status 200: " + status_200)

sus status_404 := status_code_text(404)
vibez.spill("Status 404: " + status_404)

sus status_500 := status_code_text(500)
vibez.spill("Status 500: " + status_500)

# Test 2: HTTP method validation
vibez.spill("\n=== HTTP Method Tests ===")
sus get_valid := validate_method("GET")
vibez.spill("GET valid: " + get_valid.to_string())

sus post_valid := validate_method("POST")
vibez.spill("POST valid: " + post_valid.to_string())

sus invalid_method := validate_method("INVALID")
vibez.spill("INVALID valid: " + invalid_method.to_string())

# Test 3: Content type detection
vibez.spill("\n=== Content Type Tests ===")
sus json_type := detect_content_type("{\"key\":\"value\"}")
vibez.spill("JSON type: " + json_type)

sus html_type := detect_content_type("<html></html>")
vibez.spill("HTML type: " + html_type)

sus plain_type := detect_content_type("hello world")
vibez.spill("Plain type: " + plain_type)

# Test 4: URL processing
vibez.spill("\n=== URL Processing Tests ===")
sus path := parse_url_path("https://example.com/api/users")
vibez.spill("URL path: " + path)

sus query := parse_query_params("https://example.com?name=john&age=30")
vibez.spill("Query params: " + query)

# Test 5: Simple HTTP request
vibez.spill("\n=== HTTP Request Tests ===")
sus get_response := http_get("https://api.example.com")
vibez.spill("GET response (first 100 chars): " + get_response.substring(0, 100))

# Test 6: Response building
vibez.spill("\n=== Response Building Tests ===")
sus response := build_response(200, "Hello, World!")
vibez.spill("Response built successfully")

# Test 7: Error handling
vibez.spill("\n=== Error Handling Tests ===")
sus error_response := build_error_response(404, "Not Found")
vibez.spill("Error response built successfully")

vibez.spill("\n=== All Tests Complete ===")
