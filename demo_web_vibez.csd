yeet "web_vibez"

# Demo of web_vibez functionality
vibez.spill("=== CURSED Web Framework Demo ===")

# Test status codes
vibez.spill("\n1. Status Code Mapping:")
vibez.spill("200 -> " + status_code_text(200))
vibez.spill("404 -> " + status_code_text(404))
vibez.spill("500 -> " + status_code_text(500))

# Test HTTP methods
vibez.spill("\n2. HTTP Method Validation:")
vibez.spill("GET valid: " + validate_method("GET").to_string())
vibez.spill("POST valid: " + validate_method("POST").to_string())
vibez.spill("INVALID valid: " + validate_method("INVALID").to_string())

# Test content type detection
vibez.spill("\n3. Content Type Detection:")
vibez.spill("JSON: " + detect_content_type("{\"key\":\"value\"}"))
vibez.spill("HTML: " + detect_content_type("<html></html>"))
vibez.spill("Plain: " + detect_content_type("hello world"))

# Test URL parsing
vibez.spill("\n4. URL Processing:")
vibez.spill("Path: " + parse_url_path("https://example.com/api/users"))
vibez.spill("Query: " + parse_query_params("https://example.com?name=john"))

# Test HTTP requests
vibez.spill("\n5. HTTP Requests:")
sus get_result := http_get("https://api.example.com")
vibez.spill("GET Response length: " + get_result.length().to_string())

sus post_result := http_post("https://api.example.com", "{\"data\":\"test\"}")
vibez.spill("POST Response length: " + post_result.length().to_string())

# Test server creation
vibez.spill("\n6. Server Operations:")
sus server := create_server(8080)
vibez.spill("Server created on port: " + server.to_string())

# Test response building
vibez.spill("\n7. Response Building:")
sus response := build_response(200, "Hello, World!")
vibez.spill("Response built successfully")

# Test production handler
vibez.spill("\n8. Production Handler:")
sus prod_response := handle_production_request("GET", "/", "", "")
vibez.spill("Production handler response length: " + prod_response.length().to_string())

vibez.spill("\n=== Demo Complete ===")
