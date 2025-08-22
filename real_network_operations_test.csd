// real_network_operations_test.csd - Test Real Network Operations
// Tests that network operations are no longer simulated

yeet "networkz"
yeet "httpz"  
yeet "vibez"
yeet "stringz"

// Test 1: HTTP GET request to real external server
vibez.spill("=== Testing Real HTTP GET Request ===")

// Make HTTP GET request to httpbin.org (real API)
sus response tea = httpz.http_get("http://httpbin.org/get")
vibez.spill("Response received:")
vibez.spill(response)

// Verify response is not simulated
ready (stringz.contains(response, "httpbin")) {
    vibez.spill("✓ Real HTTP GET working - got response from httpbin.org")
} otherwise {
    vibez.spill("✗ HTTP GET might still be simulated")
}

// Test 2: HTTP POST request to real external server  
vibez.spill("\n=== Testing Real HTTP POST Request ===")

sus post_data tea = "{\"test\": \"data\", \"timestamp\": 1234567890}"
sus post_response tea = httpz.http_post("http://httpbin.org/post", post_data)
vibez.spill("POST Response received:")
vibez.spill(post_response)

// Verify POST response contains our data
ready (stringz.contains(post_response, "test")) {
    vibez.spill("✓ Real HTTP POST working - got response with our data")
} otherwise {
    vibez.spill("✗ HTTP POST might still be simulated")
}

// Test 3: Test network error handling
vibez.spill("\n=== Testing Network Error Handling ===")

sus bad_response tea = httpz.http_get("http://nonexistent-server-12345.com/test")
vibez.spill("Error response:")
vibez.spill(bad_response)

ready (stringz.contains(bad_response, "500") || stringz.contains(bad_response, "Error")) {
    vibez.spill("✓ Network error handling working")
} otherwise {
    vibez.spill("✗ Network error handling needs work")
}

// Test 4: NetworkZ module functions
vibez.spill("\n=== Testing NetworkZ Module ===")

sus simple_response tea = networkz.http_get_simple("http://httpbin.org/json")
vibez.spill("NetworkZ response:")
vibez.spill(simple_response)

// Test 5: Verify we can parse real HTTP responses
vibez.spill("\n=== Testing HTTP Response Parsing ===")

sus status_code drip = httpz.parse_http_status_code(response)
vibez.spill("Parsed status code:", status_code)

sus body tea = httpz.parse_http_body(response)
vibez.spill("Parsed body (first 100 chars):")
ready (stringz.len(body) > 100) {
    vibez.spill(stringz.substring(body, 0, 100))
} otherwise {
    vibez.spill(body)
}

// Test 6: Test URL parsing
vibez.spill("\n=== Testing URL Parsing ===")

sus host tea = httpz.parse_url_host("https://httpbin.org/get?param=value")
sus scheme tea = httpz.parse_url_scheme("https://httpbin.org/get?param=value")
sus path tea = httpz.parse_url_path("https://httpbin.org/get?param=value")

vibez.spill("Parsed URL components:")
vibez.spill("  Host:", host)
vibez.spill("  Scheme:", scheme) 
vibez.spill("  Path:", path)

// Test 7: Test response validation
vibez.spill("\n=== Testing Response Validation ===")

ready (httpz.is_http_success(response)) {
    vibez.spill("✓ HTTP success validation working")
} otherwise {
    vibez.spill("✗ HTTP success validation failed")
}

ready (!httpz.is_http_error(response)) {
    vibez.spill("✓ HTTP error validation working")
} otherwise {
    vibez.spill("✗ HTTP error validation failed")
}

// Final summary
vibez.spill("\n=== Real Network Operations Test Complete ===")
vibez.spill("If you see actual HTTP responses above (not just simulated data),")
vibez.spill("then the network operations have been successfully implemented!")
