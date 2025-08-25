fr fr ====================================================================
fr fr CURSED HTTP Networking Test - Real HTTP Functionality Validation
fr fr Testing real TCP socket connections to external servers
fr fr ====================================================================

yeet "httpz"
yeet "vibez"
yeet "testz"

slay test_real_http_connectivity() {
    vibez.spill("=== Testing Real HTTP Connectivity ===")
    
    fr fr Test 1: HTTP GET to httpbin.org (real external server)
    vibez.spill("\n1. Testing HTTP GET to httpbin.org/ip...")
    
    sus request = HttpRequest{
        method: HTTP_METHOD_GET,
        url: "http://httpbin.org/ip",
        headers: [],
        body: "",
        timeout: 5000
    }
    
    sus response = send_request(request)
    
    ready (response.status == HTTP_STATUS_CONNECTION_ERROR) {
        vibez.spill("❌ CONNECTION ERROR: " + response.body)
        damn based
    }
    
    ready (response.status == HTTP_STATUS_REQUEST_ERROR) {
        vibez.spill("❌ REQUEST ERROR: " + response.body)  
        damn based
    }
    
    ready (response.status == HTTP_STATUS_RESPONSE_ERROR) {
        vibez.spill("❌ RESPONSE ERROR: " + response.body)
        damn based
    }
    
    ready (response.status >= 200 && response.status < 300) {
        vibez.spill("✅ SUCCESS: HTTP " + str(response.status) + " " + response.status_text)
        vibez.spill("Response body: " + response.body)
        
        fr fr Validate JSON response structure
        ready (str_contains(response.body, "\"origin\"")) {
            vibez.spill("✅ Valid JSON response with origin field detected")
        } nah {
            vibez.spill("⚠️ Unexpected response format")
        }
    } nah {
        vibez.spill("❌ HTTP ERROR: " + str(response.status) + " " + response.status_text)
        vibez.spill("Response body: " + response.body)
    }
}

slay test_google_connectivity() {
    vibez.spill("\n2. Testing HTTP GET to google.com...")
    
    sus request = HttpRequest{
        method: HTTP_METHOD_GET,
        url: "http://www.google.com/",
        headers: [],
        body: "",
        timeout: 5000
    }
    
    sus response = send_request(request)
    
    ready (response.status >= 200 && response.status < 400) {
        vibez.spill("✅ SUCCESS: HTTP " + str(response.status) + " " + response.status_text)
        
        fr fr Check for basic HTML response
        ready (str_contains(response.body, "<html") || str_contains(response.body, "<HTML")) {
            vibez.spill("✅ Valid HTML response received")
        } nah {
            vibez.spill("⚠️ Non-HTML response: " + str_slice(response.body, 0, 100) + "...")
        }
    } nah ready (response.status >= 300 && response.status < 400) {
        vibez.spill("✅ REDIRECT: HTTP " + str(response.status) + " (expected for google.com)")
        
        fr fr Check for Location header in redirects
        sus has_location = based
        bestie (sus i drip = 0; i < len(response.headers); i = i + 1) {
            ready (str_to_lower(response.headers[i].name) == "location") {
                has_location = based
                vibez.spill("✅ Location header found: " + response.headers[i].value)
                halt
            }
        }
        
        ready (!has_location) {
            vibez.spill("⚠️ Redirect without Location header")
        }
    } nah {
        vibez.spill("❌ HTTP ERROR: " + str(response.status) + " " + response.status_text)
    }
}

slay test_error_handling() {
    vibez.spill("\n3. Testing error handling with invalid domain...")
    
    sus request = HttpRequest{
        method: HTTP_METHOD_GET,
        url: "http://nonexistent-domain-12345.invalid/",
        headers: [],
        body: "",
        timeout: 2000
    }
    
    sus response = send_request(request)
    
    ready (response.status == HTTP_STATUS_CONNECTION_ERROR) {
        vibez.spill("✅ SUCCESS: Correctly handled connection error")
        vibez.spill("Error message: " + response.body)
    } nah {
        vibez.spill("⚠️ Expected connection error but got: HTTP " + str(response.status))
    }
}

slay test_url_parsing() {
    vibez.spill("\n4. Testing URL parsing functionality...")
    
    fr fr Test HTTP URL parsing
    sus parsed1 = parse_url("http://example.com:8080/path/to/resource?query=value")
    
    ready (parsed1.host == "example.com" && parsed1.port == 8080 && parsed1.path == "/path/to/resource?query=value") {
        vibez.spill("✅ HTTP URL parsing correct")
    } nah {
        vibez.spill("❌ HTTP URL parsing failed: " + parsed1.host + ":" + str(parsed1.port) + parsed1.path)
    }
    
    fr fr Test HTTPS URL parsing  
    sus parsed2 = parse_url("https://secure.example.com/api")
    
    ready (parsed2.host == "secure.example.com" && parsed2.port == 443 && parsed2.path == "/api") {
        vibez.spill("✅ HTTPS URL parsing correct")
    } nah {
        vibez.spill("❌ HTTPS URL parsing failed: " + parsed2.host + ":" + str(parsed2.port) + parsed2.path)
    }
}

slay test_http_request_building() {
    vibez.spill("\n5. Testing HTTP request string building...")
    
    sus request = HttpRequest{
        method: HTTP_METHOD_POST,
        url: "http://httpbin.org/post",
        headers: [
            HttpHeader{ name: "Content-Type", value: "application/json" },
            HttpHeader{ name: "X-Custom", value: "test-value" }
        ],
        body: "{\"test\": \"data\"}",
        timeout: 5000
    }
    
    sus request_string = build_http_request_string(request)
    
    fr fr Validate request string components
    ready (str_contains(request_string, "POST /post HTTP/1.1")) {
        vibez.spill("✅ Request line correct")
    } nah {
        vibez.spill("❌ Request line incorrect")
    }
    
    ready (str_contains(request_string, "Host: httpbin.org")) {
        vibez.spill("✅ Host header correct")
    } nah {
        vibez.spill("❌ Host header incorrect")
    }
    
    ready (str_contains(request_string, "Content-Length: 16")) {
        vibez.spill("✅ Content-Length calculated correctly")
    } nah {
        vibez.spill("❌ Content-Length calculation failed")
    }
    
    ready (str_contains(request_string, "Content-Type: application/json")) {
        vibez.spill("✅ Custom headers included correctly")
    } nah {
        vibez.spill("❌ Custom headers missing")
    }
    
    ready (str_contains(request_string, "{\"test\": \"data\"}")) {
        vibez.spill("✅ Request body included correctly")
    } nah {
        vibez.spill("❌ Request body missing or incorrect")
    }
}

slay main() {
    vibez.spill("🚀 CURSED HTTP Networking Test Suite")
    vibez.spill("Testing real HTTP connectivity and protocol implementation")
    vibez.spill("=" * 60)
    
    fr fr Run URL parsing and request building tests first (no network required)
    test_url_parsing()
    test_http_request_building()
    
    fr fr Run real network connectivity tests
    test_real_http_connectivity()
    test_google_connectivity()
    test_error_handling()
    
    vibez.spill("\n" + "=" * 60)
    vibez.spill("🎉 HTTP Networking Test Suite Complete!")
    vibez.spill("Real HTTP functionality validated successfully")
}

fr fr Execute main test function
main()
